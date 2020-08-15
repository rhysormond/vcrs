pub mod blob;
pub mod commit;
pub mod tag;
pub mod tree;

use std::{error::Error, fmt};

const OBJECT_KIND_SEP: u8 = 0x20;
const OBJECT_SIZE_SEP: u8 = 0x00;

#[derive(Debug)]
struct DeserializationError {
    thing: String,
    reason: String,
}

impl Error for DeserializationError {}

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not deserialize \n {} \n because \n {}",
            self.thing, self.reason
        )
    }
}

#[derive(Debug)]
pub enum Object {
    Blob(blob::Blob),
    Commit(commit::Commit),
    Tag(tag::Tag),
    Tree(tree::Tree),
}

impl Object {
    pub fn new(kind: String, content: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        match kind.as_str() {
            "blob" => Ok(Object::Blob(blob::Blob::deserialize(content)?)),
            "commit" => Ok(Object::Commit(commit::Commit::deserialize(content)?)),
            "tag" => Ok(Object::Tag(tag::Tag::deserialize(content)?)),
            "tree" => Ok(Object::Tree(tree::Tree::deserialize(content)?)),
            other => Err(Box::new(DeserializationError {
                thing: String::from_utf8(content)?,
                reason: format!("Unsupported object type {}.", other),
            })),
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        // TODO[Rhys] figure out how to deduplicate this with the deserialization match
        let (kind, content) = match self {
            Self::Blob(content) => ("blob", content.serialize()),
            Self::Commit(content) => ("commit", content.serialize()),
            Self::Tag(content) => ("tag", content.serialize()),
            Self::Tree(content) => ("tree", content.serialize()),
        };
        let size: u8 = content.len() as u8;
        [
            kind.as_bytes().to_vec(),
            vec![OBJECT_KIND_SEP, size, OBJECT_SIZE_SEP],
            content.to_vec(),
        ]
        .concat()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut iter = body.iter();
        let kind_raw: Vec<u8> = iter
            .by_ref()
            .take_while(|&b| *b != OBJECT_KIND_SEP)
            .cloned()
            .collect();
        let kind = String::from_utf8(kind_raw)?;
        let size_raw: Vec<u8> = iter
            .by_ref()
            .take_while(|&b| *b != OBJECT_SIZE_SEP)
            .cloned()
            .collect();
        let size: usize = String::from_utf8(size_raw)?.parse()?;
        let content: Vec<u8> = iter.cloned().collect();

        assert_eq!(
            size,
            content.len(),
            "Content length was not equal to the encoded size."
        );

        Self::new(kind, content)
    }
}
