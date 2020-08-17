use std::error::Error;

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Reference {
    Ref(String),
    Commit(String),
}

impl Reference {
    pub fn serialize(&self) -> String {
        match self {
            Reference::Ref(body) => format!("ref: {}\n", body),
            Reference::Commit(body) => format!("{}\n", body),
        }
    }

    pub fn deserialize(body: &str) -> Result<Self, Box<dyn Error>> {
        let ref_regex = Regex::new(r"^ref: (.*)\n$")?;
        let commit_regex = Regex::new(r"^([a-z0-9]*)\n$")?;

        let maybe_match = ref_regex.captures(body).map(|c| c.get(1)).flatten();

        let reference = match maybe_match {
            Some(n) => Reference::Ref(n.as_str().to_string()),
            None => {
                // TODO[Rhys] look at some other way to parse these
                let hash = commit_regex
                    .captures(body)
                    .map(|c| c.get(1))
                    .flatten()
                    .expect("Reference couldn't be parsed.");
                Reference::Commit(hash.as_str().to_string())
            }
        };
        Ok(reference)
    }
}

#[cfg(test)]
mod tests {
    use crate::reference::Reference;

    #[test]
    fn round_trips_commits() {
        let data = "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3\n";
        let expected = Reference::Commit("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3".to_string());
        let deserialized = Reference::deserialize(data).unwrap();
        assert_eq!(deserialized, expected);
        assert_eq!(deserialized.serialize(), data)
    }

    #[test]
    fn round_trips_refs() {
        let data = "ref: refs/heads/master";
        let expected = Reference::Ref("refs/heads/master".to_string());
        let deserialized = Reference::deserialize(data).unwrap();
        assert_eq!(deserialized, expected);
        assert_eq!(deserialized.serialize(), data)
    }
}
