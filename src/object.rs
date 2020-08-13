use std::{error::Error, fmt};

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
    Commit(String),
    Tree(String),
    Blob(String),
    Tag(String),
}

impl Object {
    pub fn serialize(&self) -> String {
        match self {
            Self::Commit(body) => body,
            Self::Tree(body) => body,
            Self::Blob(body) => body,
            Self::Tag(body) => body,
        }
        .to_string()
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

        match kind {
            "commit" => Ok(Object::Commit(content)),
            "tree" => Ok(Object::Tree(content)),
            "blob" => Ok(Object::Blob(content)),
            "tag" => Ok(Object::Tag(content)),
            other => Err(Box::new(DeserializationError {
                thing: content,
                reason: format!("Unsupported object type {}.", other),
            })),
        }
    }
}
