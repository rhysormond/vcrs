use std::error::Error;

#[derive(Debug)]
pub struct Blob {
    content: String,
}

impl Blob {
    pub fn serialize(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            content: String::from_utf8(body)?,
        })
    }
}