use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Error, Read};
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::object::Object;

const GIT_DIR: &str = ".git";
const OBJECT_DIR: &str = "objects";
const HEADS_DIR: &str = "heads";
const HEAD_FILE: &str = "HEAD";

#[derive(Debug)]
pub struct Repository {
    pub work_tree: PathBuf,
    pub root: PathBuf,
    pub objects: PathBuf,
    pub heads: PathBuf,
    pub head: PathBuf,
}

impl Repository {
    pub fn new(work_tree: PathBuf) -> Self {
        let root = work_tree.join(GIT_DIR);
        let objects = root.join(Path::new(OBJECT_DIR));
        let heads = root.join(Path::new(HEADS_DIR));
        let head = root.join(Path::new(HEAD_FILE));
        Self {
            work_tree,
            root,
            objects,
            heads,
            head,
        }
    }

    fn read_zlib(path: PathBuf) -> Result<String, Error> {
        let file = File::open(path)?;
        let mut decoder = ZlibDecoder::new(BufReader::new(&file));
        let mut contents = String::new();
        decoder.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn write_zlib(path: PathBuf, contents: &str) -> Result<(), Error> {
        let file = File::create(path)?;
        let mut encoder = ZlibEncoder::new(BufWriter::new(&file), Compression::default());
        encoder.write_all(contents.as_ref())?;
        encoder.finish()?;
        Ok(())
    }

    pub fn read_object(&self, hash: &str) -> Result<Object, Box<dyn std::error::Error>> {
        let (dir, file) = hash.split_at(2);
        let relative_path = format!("{}/{}", dir, file);
        let path = self.objects.join(Path::new(&relative_path));
        let contents = Repository::read_zlib(path)?;

        // TODO[Rhys] this could use some much fancier parsing
        // TODO[Rhys] i don't think we need the whole contents here
        // TODO[Rhys] we could be doing size validation on these objects
        match contents.split_whitespace().next() {
            Some("commit") => Ok(Object::Commit(contents)),
            Some("tree") => Ok(Object::Tree(contents)),
            Some("blob") => Ok(Object::Blob(contents)),
            other => panic!("invalid object type {:?}", other)
        }
    }
}
