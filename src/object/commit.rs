use regex::RegexBuilder;

#[derive(Debug, PartialEq)]
pub struct Commit {
    pub tree: String,
    // TODO[Rhys] this should actually be a vec as commits can have zero to many parents
    pub parent: Option<String>,
    author: String,
    committer: String,
    gpgsig: Option<String>,
    pub message: String,
}

impl Commit {
    fn serialize_field(field_name: &str, obj: &str) -> String {
        format!("{} {}\n", field_name, obj)
    }

    fn serialize_optional_field(field_name: &str, obj: &Option<String>) -> String {
        obj.as_ref()
            .map(|sig| format!("{} {}\n", field_name, sig))
            .unwrap_or("".into())
    }

    pub fn serialize(&self) -> Vec<u8> {
        // TODO[Rhys] find a way to get rid of the unnecessary cloning here
        let fields = vec![
            Self::serialize_field("tree", &self.tree),
            Self::serialize_optional_field("parent", &self.parent),
            Self::serialize_field("author", &self.author),
            Self::serialize_field("committer", &self.committer),
            Self::serialize_optional_field("gpgsig", &self.gpgsig),
            "\n".to_string(),
            self.message.clone(),
        ];
        fields
            .iter()
            .flat_map(|f| f.clone().into_bytes().into_iter())
            .collect()
    }

    pub fn deserialize(bytes: Vec<u8>) -> Self {
        let content = String::from_utf8(bytes).unwrap();
        // TODO[Rhys] this could use some much cleverer parsing
        let regex = RegexBuilder::new(
            r"(?x)
            tree\ (?P<tree>[a-zA-Z0-9]*)\n
            (parent\ (?P<parent>[a-zA-Z0-9]*)\n)?
            author\ (?P<author>.*)\n
            committer\ (?P<committer>.*)\n
            (gpgsig\ (?P<gpgsig>
                -----BEGIN\ PGP\ SIGNATURE-----[\w\d\s+/=]*-----END\ PGP\ SIGNATURE-----
            )\n)?
            \n
            (?P<message>.*)
        ",
        )
        .multi_line(true)
        .build()
        .unwrap();
        let captures = regex.captures(&*content).unwrap();

        Self {
            tree: captures.name("tree").unwrap().as_str().into(),
            parent: captures.name("parent").map(|cap| cap.as_str().into()),
            author: captures.name("author").unwrap().as_str().into(),
            committer: captures.name("committer").unwrap().as_str().into(),
            gpgsig: captures.name("gpgsig").map(|cap| cap.as_str().into()),
            message: captures.name("message").unwrap().as_str().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::commit::Commit;

    #[test]
    fn serializes_fields() {
        assert_eq!(Commit::serialize_field(&"field", "value"), "field value\n");
    }

    #[test]
    fn serializes_optional_field_when_defined() {
        assert_eq!(
            Commit::serialize_optional_field(&"field", &Some("value".to_string())),
            "field value\n"
        );
    }

    #[test]
    fn doesnt_serialize_optional_field_when_not_defined() {
        assert_eq!(Commit::serialize_optional_field(&"field", &None), "");
    }

    #[test]
    fn round_trips_basic_commits() {
        let serialized = "\
            tree 2b5bfdf7798569e0b59b16eb9602d5fa572d6038\n\
            author Linus Torvalds <torvalds@ppc970.osdl.org> 1112911993 -0700\n\
            committer Linus Torvalds <torvalds@ppc970.osdl.org> 1112911993 -0700\n\
            \n\
            Initial revision of \"git\", the information manager from hell\
            ";
        let deserialized = Commit {
            tree: "2b5bfdf7798569e0b59b16eb9602d5fa572d6038".to_string(),
            parent: None,
            author: "Linus Torvalds <torvalds@ppc970.osdl.org> 1112911993 -0700".to_string(),
            committer: "Linus Torvalds <torvalds@ppc970.osdl.org> 1112911993 -0700".to_string(),
            gpgsig: None,
            message: "Initial revision of \"git\", the information manager from hell".to_string(),
        };
        let commit = Commit::deserialize(Vec::from(serialized));
        assert_eq!(commit, deserialized);
        assert_eq!(String::from_utf8(commit.serialize()).unwrap(), serialized)
    }

    #[test]
    fn round_trips_commits_with_all_fields() {
        let serialized = "\
            tree c171921c5c0f2e02f7243c13d331e96f149fd653\n\
            parent 4478b9c55808657544198529c58e29888d31e677\n\
            author rhysormond <email> 1597275816 -0700\n\
            committer rhysormond <email> 1597275816 -0700\n\
            gpgsig -----BEGIN PGP SIGNATURE-----\n\
            \n\
             iQIzBAABCAAdFiEEdnvMMujyElTR0B8vmoIqIpWBpgYFAl80frQACgkQmoIqIpWB\n\
             pgaghQ/+NLPMK0UjuZM0Spp2W5t7yqAczySyYQJOG1gAnkpgiPKeTmXrBFEKBWO4\n\
             JaAZlOp5Ds9fRjro/rYG6eTwBXnE09UKYZY6kBTWmIi8JInQnA/9eJdcnxR0z2aK\n\
             pkdptU41BZZyLoYnDfN0hOJD4V2mGqRxWY1HXlpz3KIPBlhfYbaMss9z+5c2U4gf\n\
             HLPbB2wsAt5uIWfXufdpsJm94wniQBprl6MtWjdnjRwbLQXLh61vXaKgghAwJZO+\n\
             LqIAF5tUzrAqXTxRCZ7dL0gcuuV76FFEubrJwb+sGHWqtlLB4f9XWAnuVRql0EAJ\n\
             1n3OgGP3cnNeznGdZZJcEoZtJAsUnZ18yO9CeTcZ+EavtiaomPWOlCmzoBRV3HZu\n\
             B2nYKAWhNbu645iaZ7x73xBMxR3AmGmOOrE5TT41Kjfhw3JQ7risr5YfjFj88h3r\n\
             tCHtHig8f8foNR5ClQJoryjqDQCR4DluJQbeOU4PXRsJwuJZ8FdyHvc6wIdcrneQ\n\
             6P5L2ktGH740m22/bCf0M3zXl3A79Jz5FxmN0Oh+VUpMh5r4Q56csc8tg/0PSPKO\n\
             K1Eb+gTBqNlHvQSNizQR0rP5MLSp+o0YE364uvYF4imGCLOSQCTb8hNbIy31t1ah\n\
             IOTSJoM985ubIYxonwcFDUfJ3jZGZxqulu3fSbeYa31ZRVwsCbM=\n\
             =By2v\n\
             -----END PGP SIGNATURE-----\n\
            \n\
            refactor: clean up init and add todos\
        ";

        let deserialized = Commit {
            tree: "c171921c5c0f2e02f7243c13d331e96f149fd653".to_string(),
            parent: Some("4478b9c55808657544198529c58e29888d31e677".to_string()),
            author: "rhysormond <email> 1597275816 -0700".to_string(),
            committer: "rhysormond <email> 1597275816 -0700".to_string(),
            gpgsig: Some(
                "\
                -----BEGIN PGP SIGNATURE-----\n\
                \n\
                 iQIzBAABCAAdFiEEdnvMMujyElTR0B8vmoIqIpWBpgYFAl80frQACgkQmoIqIpWB\n\
                 pgaghQ/+NLPMK0UjuZM0Spp2W5t7yqAczySyYQJOG1gAnkpgiPKeTmXrBFEKBWO4\n\
                 JaAZlOp5Ds9fRjro/rYG6eTwBXnE09UKYZY6kBTWmIi8JInQnA/9eJdcnxR0z2aK\n\
                 pkdptU41BZZyLoYnDfN0hOJD4V2mGqRxWY1HXlpz3KIPBlhfYbaMss9z+5c2U4gf\n\
                 HLPbB2wsAt5uIWfXufdpsJm94wniQBprl6MtWjdnjRwbLQXLh61vXaKgghAwJZO+\n\
                 LqIAF5tUzrAqXTxRCZ7dL0gcuuV76FFEubrJwb+sGHWqtlLB4f9XWAnuVRql0EAJ\n\
                 1n3OgGP3cnNeznGdZZJcEoZtJAsUnZ18yO9CeTcZ+EavtiaomPWOlCmzoBRV3HZu\n\
                 B2nYKAWhNbu645iaZ7x73xBMxR3AmGmOOrE5TT41Kjfhw3JQ7risr5YfjFj88h3r\n\
                 tCHtHig8f8foNR5ClQJoryjqDQCR4DluJQbeOU4PXRsJwuJZ8FdyHvc6wIdcrneQ\n\
                 6P5L2ktGH740m22/bCf0M3zXl3A79Jz5FxmN0Oh+VUpMh5r4Q56csc8tg/0PSPKO\n\
                 K1Eb+gTBqNlHvQSNizQR0rP5MLSp+o0YE364uvYF4imGCLOSQCTb8hNbIy31t1ah\n\
                 IOTSJoM985ubIYxonwcFDUfJ3jZGZxqulu3fSbeYa31ZRVwsCbM=\n\
                 =By2v\n\
                 -----END PGP SIGNATURE-----\
                "
                .to_string(),
            ),
            message: "refactor: clean up init and add todos".to_string(),
        };
        let commit = Commit::deserialize(Vec::from(serialized));
        assert_eq!(commit, deserialized);
        assert_eq!(String::from_utf8(commit.serialize()).unwrap(), serialized)
    }
}
