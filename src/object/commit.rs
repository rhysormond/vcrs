use regex::RegexBuilder;
use std::error::Error;

#[derive(Debug)]
pub struct Commit {
    tree: String,
    pub parent: Option<String>,
    author: String,
    committer: String,
    gpgsig: Option<String>,
    pub message: String,
}

impl Commit {
    pub fn serialize(&self) -> Vec<u8> {
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
        .into_bytes()
    }

    pub fn deserialize(body: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let content = String::from_utf8(body)?;
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
        let captures = regex.captures(&*content).unwrap();

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
