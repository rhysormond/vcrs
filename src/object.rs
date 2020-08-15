use std::{error::Error, fmt};

pub mod blob;
pub mod commit;
pub mod constant;
pub mod tag;
pub mod tree;
mod util;

use crate::object::util::take_string;
use constant::*;

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
            other => panic!(format!("Object type {} is not valid.", other)),
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
