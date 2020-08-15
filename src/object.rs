use std::{error::Error, fmt};

use regex::RegexBuilder;

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
    Blob(Blob),
    Commit(Commit),
    Tag(Tag),
    Tree(Tree),
}

#[derive(Debug)]
pub struct Blob {
    content: String,
}

impl Blob {
    pub fn serialize(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
            content: String::from_utf8(body)?,
        }
        )
    }
}

#[derive(Debug)]
pub struct Commit {
    tree: String,
    pub parent: Option<String>,
    author: String,
    committer: String,
    gpgsig: Option<String>,
    pub message: String,
}

impl Commit {
    pub fn serialize(&self) -> Vec<u8> {
        let maybe_parent = self
            .parent
            .as_ref()
            .map(|sig| format!("parent {}\n", sig))
            .unwrap_or("".into());
        let maybe_gpgsig = self
            .gpgsig
            .as_ref()
            .map(|sig| format!("gpgsig {}\n", sig))
            .unwrap_or("".into());
        format!(
            "tree {}\n{}author {}\ncommitter {}\n{}\n{}\n",
            self.tree, maybe_parent, self.author, self.committer, maybe_gpgsig, self.message,
        ).into_bytes()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let content = String::from_utf8(body)?;
        // TODO[Rhys] this could use some much cleverer parsing
        let regex = RegexBuilder::new(
            r"(?x)
            tree\ (?P<tree>[a-zA-Z0-9]*)\n
            (parent\ (?P<parent>[a-zA-Z0-9]*)\n)?
            author\ (?P<author>.*)\n
            committer\ (?P<committer>.*)\n
            (gpgsig\ (?P<gpgsig>
                -----BEGIN\ PGP\ SIGNATURE-----[\w\d\s+/=]*-----END\ PGP\ SIGNATURE-----
            )\n)?\n
            (?P<message>.*)
        ",
        )
        .multi_line(true)
        .build()
        .unwrap();
        let captures = regex.captures(&*content).unwrap();

        Ok(Self {
            tree: captures.name("tree").unwrap().as_str().into(),
            parent: captures.name("parent").map(|cap| cap.as_str().into()),
            author: captures.name("author").unwrap().as_str().into(),
            committer: captures.name("committer").unwrap().as_str().into(),
            gpgsig: captures.name("gpgsig").map(|cap| cap.as_str().into()),
            message: captures.name("message").unwrap().as_str().into(),
        })
    }
}

#[derive(Debug)]
pub struct Tag {
    content: String,
}

impl Tag {
    pub fn serialize(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            content: String::from_utf8(body)?,
        })
    }
}

#[derive(Debug)]
pub struct Tree {
    content: String,
}

impl Tree {
    pub fn serialize(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            content: String::from_utf8(body)?,
        })
    }
}

impl Object {
    pub fn new(kind: String, content: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        match kind.as_str() {
            "blob" => Ok(Object::Blob(Blob::deserialize(content)?)),
            "commit" => Ok(Object::Commit(Commit::deserialize(content)?)),
            "tag" => Ok(Object::Tag(Tag::deserialize(content)?)),
            "tree" => Ok(Object::Tree(Tree::deserialize(content)?)),
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
        [kind.as_bytes().to_vec(), vec![OBJECT_KIND_SEP, size, OBJECT_SIZE_SEP], content.to_vec()].concat()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut iter = body.iter();
        let kind_raw: Vec<u8> = iter.by_ref().take_while(|&b| *b != OBJECT_KIND_SEP).cloned().collect();
        let kind = String::from_utf8(kind_raw)?;
        let size_raw: Vec<u8> = iter.by_ref().take_while(|&b| *b != OBJECT_SIZE_SEP).cloned().collect();
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
