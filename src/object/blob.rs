#[derive(Debug, PartialEq)]
pub struct Blob {
    pub content: String,
}

impl Blob {
    pub fn serialize(&self) -> Vec<u8> {
        self.content.clone().into_bytes()
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        Self {
            content: String::from_utf8(bytes).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::blob::Blob;

    #[test]
    fn round_trips_tags() {
        let serialized = "blob";
        let deserialized = Blob {
            content: serialized.to_string(),
        };
        let blob = Blob::deserialize(Vec::from(serialized));
        assert_eq!(blob, deserialized);
        assert_eq!(String::from_utf8(blob.serialize()).unwrap(), serialized)
    }
}
