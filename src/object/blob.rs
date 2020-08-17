use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Blob {
    pub content: String,
}

impl Blob {
    pub fn serialize(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }

    pub fn deserialize(bytes: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            content: String::from_utf8(bytes)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::object::blob::Blob;

    #[test]
    fn can_roundtrip_tags() {
        let serialized = "blob";
        let deserialized = Blob {
            content: serialized.to_string(),
        };
        let blob = Blob::deserialize(Vec::from(serialized)).unwrap();
        assert_eq!(blob, deserialized);
        assert_eq!(String::from_utf8(blob.serialize()).unwrap(), serialized)
    }
}
