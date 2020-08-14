use std::{error::Error, fmt};

use regex::RegexBuilder;

const OBJECT_KIND_SEP: char = 0x20_u8 as char;
const OBJECT_SIZE_SEP: char = 0x00_u8 as char;

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
    Commit(Commit),
    Tree(String),
    Blob(String),
    Tag(String),
}

#[derive(Debug)]
pub struct Commit {
    tree: String,
    parent: Option<String>,
    author: String,
    committer: String,
    gpgsig: Option<String>,
    message: String,
}

impl Commit {
    pub fn serialize(&self) -> String {
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
        )
    }

    pub fn deserialize(body: String) -> Result<Self, Box<dyn Error>> {
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
        let captures = regex.captures(&*body).unwrap();

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

impl Object {
    pub fn new(kind: &str, content: String) -> Result<Self, Box<dyn Error>> {
        match kind {
            "blob" => Ok(Object::Blob(content)),
            "commit" => Ok(Object::Commit(Commit::deserialize(content)?)),
            "tag" => Ok(Object::Tag(content)),
            "tree" => Ok(Object::Tree(content)),
            other => Err(Box::new(DeserializationError {
                thing: content,
                reason: format!("Unsupported object type {}.", other),
            })),
        }
    }
    pub fn serialize(&self) -> String {
        // TODO[Rhys] figure out how to deduplicate this with the deserialization match
        let (kind, content) = match self {
            Self::Blob(content) => ("blob", content.clone()),
            Self::Commit(content) => ("commit", content.serialize()),
            Self::Tag(content) => ("tag", content.clone()),
            Self::Tree(content) => ("tree", content.clone()),
        };
        let size = content.len();
        format!(
            "{}{}{}{}{}",
            kind, OBJECT_KIND_SEP, size, OBJECT_SIZE_SEP, content
        )
    }

    pub fn deserialize(body: String) -> Result<Self, Box<dyn Error>> {
        // TODO[Rhys] this could use some much fancier parsing
        let mut kind_splitter = body.splitn(2, OBJECT_KIND_SEP);
        let kind = kind_splitter.next().unwrap();
        let mut content_splitter = kind_splitter.next().unwrap().splitn(2, OBJECT_SIZE_SEP);
        let size = content_splitter.next().unwrap();
        let content: String = content_splitter.next().unwrap().parse()?;

        assert_eq!(
            size.parse::<usize>()? as usize,
            content.len(),
            "Content length was not equal to the encoded size."
        );

        Self::new(kind, content)
    }
}
