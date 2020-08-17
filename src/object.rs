use std::error::Error;

use nom::{bytes::complete::take_while, combinator::map_res, sequence::tuple, IResult};

use constant::*;

pub mod blob;
pub mod commit;
pub mod constant;
pub mod tag;
pub mod tree;

#[derive(Debug, PartialEq)]
pub enum Object {
    Blob(blob::Blob),
    Commit(commit::Commit),
    Tag(tag::Tag),
    Tree(tree::Tree),
}

impl Object {
    pub fn new(kind: String, content: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        match kind.as_str() {
            NAME_BLOB => Ok(Object::Blob(blob::Blob::deserialize(content)?)),
            NAME_COMMIT => Ok(Object::Commit(commit::Commit::deserialize(content)?)),
            NAME_TAG => Ok(Object::Tag(tag::Tag::deserialize(content)?)),
            NAME_TREE => Ok(Object::Tree(tree::Tree::deserialize(content)?)),
            other => panic!(format!("Object type {} is not valid.", other)),
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        // TODO[Rhys] figure out how to deduplicate this with the deserialization match
        let (kind, content) = match self {
            Self::Blob(content) => (NAME_BLOB, content.serialize()),
            Self::Commit(content) => (NAME_COMMIT, content.serialize()),
            Self::Tag(content) => (NAME_TAG, content.serialize()),
            Self::Tree(content) => (NAME_TREE, content.serialize()),
        };
        let size = content.len().to_string();
        [
            kind.as_bytes().to_vec(),
            vec![ASCII_SPACE],
            size.as_bytes().to_vec(),
            vec![ASCII_NULL],
            content.to_vec(),
        ]
        .concat()
    }

    // TODO[Rhys] deduplicate this with the tree parsing logic
    fn parse_kind(input: &[u8]) -> IResult<&[u8], String> {
        map_res(take_while(|c| c != ASCII_SPACE), |c: &[u8]| {
            String::from_utf8(c.to_vec())
        })(input)
        .map(
            // Note[Rhys] we slice remainder here to drop the space delimiter
            |(remainder, path)| (&remainder[1..], path),
        )
    }

    fn parse_size(input: &[u8]) -> IResult<&[u8], usize> {
        // TODO[Rhys] this &[u8] -> usize conversion is pretty sloppy
        map_res(take_while(|c| c != ASCII_NULL), |c: &[u8]| {
            String::from_utf8(c.to_vec()).unwrap().parse()
        })(input)
        .map(
            // Note[Rhys] we slice remainder here to drop the null delimiter
            |(remainder, path)| (&remainder[1..], path),
        )
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        // TODO[Rhys] this shouldn't unwrap but lifetimes for bytes gets weird
        let (remainder, (kind, size)) = tuple((Object::parse_kind, Object::parse_size))(bytes).unwrap();

        assert_eq!(
            size,
            remainder.len(),
            "Content length was not equal to the encoded size."
        );

        Self::new(kind, remainder.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::object::constant::{ASCII_NULL, ASCII_SPACE};
    use crate::object::{blob, Object, Object::Blob};

    #[test]
    fn parses_kind() {
        let expected_remainder = 48;
        let raw = [116, 114, 101, 101, ASCII_SPACE, expected_remainder];
        let (remainder, kind) = Object::parse_kind(&raw).unwrap();
        assert_eq!(kind, "tree");
        assert_eq!(remainder, [expected_remainder]);
    }

    #[test]
    fn parses_size() {
        let expected_remainder = 48;
        let raw = [49, 50, 51, ASCII_NULL, expected_remainder];
        let (remainder, size) = Object::parse_size(&raw).unwrap();
        assert_eq!(size, 123);
        assert_eq!(remainder, [expected_remainder]);
    }

    #[test]
    fn serializes() {
        let blob = Blob(blob::Blob {
            content: "some blob".to_string(),
        });
        let serialized = String::from_utf8(blob.serialize()).unwrap();
        let expected = "blob 9\u{0}some blob";
        assert_eq!(serialized, expected)
    }

    #[test]
    fn deserializes() {
        let serialized = "blob 9\u{0}some blob";
        let expected = Blob(blob::Blob {
            content: "some blob".to_string(),
        });
        let blob = Object::deserialize(serialized.as_bytes()).unwrap();
        assert_eq!(blob, expected)
    }

    #[test]
    #[should_panic]
    fn panics_when_size_is_incorrect() {
        let serialized = "blob 8\u{0}some blob";
        Object::deserialize(serialized.as_bytes()).unwrap();
    }
}
