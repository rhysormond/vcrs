#[derive(Debug, PartialEq)]
pub struct Tag {
    // TODO[Rhys] figure out how tags are actually structured
    content: String,
}

impl Tag {
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
    use crate::object::tag::Tag;

    #[test]
    fn can_roundtrip_tags() {
        let serialized = "tag";
        let deserialized = Tag {
            content: serialized.to_string(),
        };
        let tag = Tag::deserialize(Vec::from(serialized));
        assert_eq!(tag, deserialized);
        assert_eq!(String::from_utf8(tag.serialize()).unwrap(), serialized)
    }
}
