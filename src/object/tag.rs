use std::error::Error;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::object::tag::Tag;

    #[test]
    fn can_roundtrip_tags() {
        let serialized = "tag";
        let deserialized = Tag {
            content: String::from(serialized),
        };
        let tag = Tag::deserialize(Vec::from(serialized)).unwrap();
        assert_eq!(tag, deserialized);
        assert_eq!(String::from_utf8(tag.serialize()).unwrap(), serialized)
    }
}
