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

    pub fn find_object(kind: String, hash: String) -> Result<String, Error> {
        Ok(hash)
    }

    pub fn hash(bytes: &[u8]) -> String {
        let mut hasher = Sha1::new();
        hasher.input(bytes);
        hasher.result_str()
    }

    fn hash_to_path(hash: &str) -> PathBuf {
        let (dir, file) = hash.split_at(2);
        Path::new(&format!("{}/{}", dir, file)).into()
    }

    fn read_zlib(path: PathBuf) -> Result<Vec<u8>, Error> {
        let file = File::open(path)?;
        let mut decoder = ZlibDecoder::new(BufReader::new(&file));
        let mut bytes = vec![];
        decoder
            .read_to_end(&mut bytes)
            .expect("Unable to read file.");
        Ok(bytes)
    }

    fn write_zlib(path: PathBuf, bytes: &Vec<u8>) -> Result<(), Error> {
        fs::create_dir_all(path.parent().unwrap())?;
        let file = File::create(path)?;
        let mut encoder = ZlibEncoder::new(BufWriter::new(&file), Compression::default());
        encoder.write_all(bytes)?;
        encoder.finish()?;
        Ok(())
    }

    pub fn write_object(&self, obj: Object) -> Result<String, Error> {
        let content = obj.serialize();
        let hash = Repository::hash(&content);
        let relative_path = Repository::hash_to_path(&*hash);
        let path = self.objects.join(relative_path);
        Repository::write_zlib(path, &content).map(|_ok| hash)
    }

    pub fn read_object(&self, hash: &str) -> Result<Object, Box<dyn std::error::Error>> {
        let relative_path = Repository::hash_to_path(hash);
        let path = self.objects.join(relative_path);
        let bytes = Repository::read_zlib(path)?;
        Object::deserialize(bytes.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::Repository;
    use std::path::Path;

    #[test]
    fn hashes() {
        let data = "test";
        let expected = "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3";
        let hash = Repository::hash(data.as_bytes());
        assert_eq!(hash, expected)
    }

    #[test]
    fn converts_hash_to_path() {
        let hash = "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3";
        let expected = Path::new("a9/4a8fe5ccb19ba61c4c0873d391e987982fbbd3");
        let path = Repository::hash_to_path(hash);
        assert_eq!(path, expected)
    }
}
