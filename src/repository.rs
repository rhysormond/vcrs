use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Error, Read};
use std::path::{Path, PathBuf};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crypto::digest::Digest;
use crypto::sha1::Sha1;

use crate::object::Object;
use std::fs;

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
    pub fn for_working_directory() -> Result<Self, Error> {
        std::env::current_dir().map(Repository::new)
    }

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

    pub fn find_object(kind: String, hash: String) -> Result<String, Error>{
        Ok(hash)
    }

    fn hash_to_path(hash: &str) -> PathBuf {
        let (dir, file) = hash.split_at(2);
        Path::new(&format!("{}/{}", dir, file)).into()
    }

    fn read_zlib(path: PathBuf) -> Result<String, Error> {
        let file = File::open(path)?;
        let mut decoder = ZlibDecoder::new(BufReader::new(&file));
        let mut contents = String::new();
        decoder.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn write_zlib(path: PathBuf, contents: &str) -> Result<(), Error> {
        fs::create_dir_all(path.parent().unwrap())?;
        let file = File::create(path)?;
        let mut encoder = ZlibEncoder::new(BufWriter::new(&file), Compression::default());
        encoder.write_all(contents.as_ref())?;
        encoder.finish()?;
        Ok(())
    }

    pub fn write_object(&self, obj: Object) -> Result<String, Error> {
        let contents = obj.serialize();
        let mut hasher = Sha1::new();
        hasher.input_str(&*contents);
        let hash = hasher.result_str();
        let relative_path = Repository::hash_to_path(&*hash);
        let path = self.objects.join(relative_path);
        Repository::write_zlib(path, &*contents).map(|_ok| hash)
    }

    pub fn read_object(&self, hash: &str) -> Result<Object, Box<dyn std::error::Error>> {
        let relative_path = Repository::hash_to_path(hash);
        let path = self.objects.join(relative_path);
        let contents = Repository::read_zlib(path)?;
        Object::deserialize(contents)
    }
}
