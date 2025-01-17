use crate::object::Object;
use crate::repository::Repository;
use std::fs;

pub fn hash_object(kind: String, file: String, write: bool) {
    let repo = Repository::for_working_directory();
    let obj = {
        let content = fs::read_to_string(file).unwrap();
        Object::new(kind, content.as_bytes().to_vec())
    };

    if write {
        repo.write_object(obj);
    } else {
        println!("{}", Repository::hash(&obj.serialize()));
    };
}
