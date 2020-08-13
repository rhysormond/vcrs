use std::{error::Error, fmt};

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
}

impl Object {
    pub fn serialize(&self) -> String {
        match self {
            Self::Commit(body) => body,
            Self::Tree(body) => body,
            Self::Blob(body) => body,
        }
        .to_string()
    }

    pub fn deserialize(body: String) -> Result<Self, Box<dyn Error>> {
        // TODO[Rhys] this could use some much fancier parsing
        // TODO[Rhys] i don't think we need the whole contents here
        // TODO[Rhys] we could be doing size validation on these objects
        // TODO[Rhys] this encoding is incorrect as we need to strip the object type and size
        let clone = body.clone();
        match body.split_whitespace().next() {
            Some("commit") => Ok(Object::Commit(body)),
            Some("tree") => Ok(Object::Tree(body)),
            Some("blob") => Ok(Object::Blob(body)),
            Some(other) => Err(Box::new(DeserializationError {
                thing: clone,
                reason: format!("unsupported type {}", other),
            })),
            None => Err(Box::new(DeserializationError {
                thing: clone,
                reason: String::from("type could not be inferred"),
            })),
        }
    }
}
