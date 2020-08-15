use std::error::Error;

pub mod blob;
pub mod commit;
pub mod constant;
pub mod tag;
pub mod tree;
mod util;

use crate::object::util::take_string;
use constant::*;

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

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut iter = body.iter();
        let kind = take_string(&mut iter, ASCII_SPACE)?;
        let size: usize = take_string(&mut iter, ASCII_NULL)?.parse()?;
        let content: Vec<u8> = iter.cloned().collect();

        assert_eq!(
            size,
            content.len(),
            "Content length was not equal to the encoded size."
        );

        Self::new(kind, content)
    }
}

#[cfg(test)]
mod tests {
    use crate::object::{blob, Object, Object::Blob};

    #[test]
    fn serializes() {
        let blob = Blob(blob::Blob {
            content: String::from("some blob"),
        });
        let serialized = String::from_utf8(blob.serialize()).unwrap();
        let expected = "blob 9\u{0}some blob";
        assert_eq!(serialized, expected)
    }

    #[test]
    fn deserializes() {
        let serialized = "blob 9\u{0}some blob";
        let expected = Blob(blob::Blob {
            content: String::from("some blob"),
        });
        let blob = Object::deserialize(serialized.into()).unwrap();
        assert_eq!(blob, expected)
    }

    #[test]
    #[should_panic]
    fn panics_when_size_is_incorrect() {
        let serialized = "blob 8\u{0}some blob";
        Object::deserialize(serialized.into()).unwrap();
    }
}
