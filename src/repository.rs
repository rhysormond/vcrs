use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::object::tree::Tree;
use crate::object::Object;
use crate::reference::Reference;

const GIT_DIR: &str = ".git";
const OBJECT_DIR: &str = "objects";
const REFS_DIR: &str = "refs";
const HEAD_FILE: &str = "HEAD";

#[derive(Debug)]
pub struct Repository {
    pub work_tree: PathBuf,
    pub root: PathBuf,
    pub objects: PathBuf,
    pub refs: PathBuf,
    pub head: PathBuf,
}

impl Repository {
    pub fn for_working_directory() -> Self {
        std::env::current_dir().map(Repository::new).unwrap()
    }

    pub fn new(work_tree: PathBuf) -> Self {
        let root = work_tree.join(GIT_DIR);
        let objects = root.join(Path::new(OBJECT_DIR));
        let refs = root.join(Path::new(REFS_DIR));
        let head = root.join(Path::new(HEAD_FILE));
        Self {
            work_tree,
            root,
            objects,
            refs,
            head,
        }
    }

    pub fn is_empty(&self) -> bool {
        let mut files = self.work_tree.read_dir().unwrap();
        files.all(|f| f.unwrap().file_name() == GIT_DIR)
    }

    pub fn checkout_tree(&self, tree: Tree, path: &PathBuf) {
        // TODO[Rhys] this is pretty sloppy
        tree.leaves.iter().for_each(|l| {
            let child_path = path.join(PathBuf::from(&l.path));
            match self.read_object(l.hash.as_str()) {
                Object::Tree(data) => {
                    // TODO[Rhys] don't check out the tree if the hashes are the same
                    self.checkout_tree(data, &child_path);
                }
                Object::Blob(data) => {
                    fs::create_dir_all(child_path.parent().unwrap()).unwrap();
                    let mut file = File::create(child_path).unwrap();
                    file.write_all(data.content.as_bytes()).unwrap();
                }
                _ => panic!("Object was not a tree or a blob."),
            }
        });
    }

    // TODO[Rhys] consider typing this more strongly to return a Commit
    pub fn find_commit(&self, reference: &Reference) -> String {
        match reference {
            Reference::Head => {
                let content = fs::read_to_string(&self.head).unwrap();
                self.find_commit(&Reference::from_file(content.as_str()))
            }
            Reference::Ref(path) => {
                let file = &self.root.join(Path::new(&path));
                let content = fs::read_to_string(file).unwrap();
                self.find_commit(&Reference::from_file(content.as_str()))
            }
            Reference::Commit(hash) => hash.to_string(),
        }
    }

    pub fn set_head(&self, head: &Reference) {
        // Note[Rhys] serializing the content early means we can fail before blanking the HEAD file
        let content = head.serialize();
        File::create(&self.head).unwrap().write_all(content.as_bytes()).unwrap();
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

    fn read_zlib(path: PathBuf) -> Vec<u8> {
        let file = File::open(path).unwrap();
        let mut decoder = ZlibDecoder::new(BufReader::new(&file));
        let mut bytes = vec![];
        decoder
            .read_to_end(&mut bytes)
            .expect("Unable to read file.");
        bytes
    }

    fn write_zlib(path: PathBuf, bytes: &Vec<u8>) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let file = File::create(path).unwrap();
        let mut encoder = ZlibEncoder::new(BufWriter::new(&file), Compression::default());
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap();
    }

    pub fn write_object(&self, obj: Object) -> String {
        let content = obj.serialize();
        let hash = Repository::hash(&content);
        let relative_path = Repository::hash_to_path(&*hash);
        let path = self.objects.join(relative_path);
        Repository::write_zlib(path, &content);
        hash
    }

    pub fn read_object(&self, hash: &str) -> Object {
        let relative_path = Repository::hash_to_path(hash);
        let path = self.objects.join(relative_path);
        let bytes = Repository::read_zlib(path);
        Object::deserialize(bytes.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::repository::Repository;

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
