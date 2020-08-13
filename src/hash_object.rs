use crate::object::Object;
use crate::repository::Repository;
use std::error::Error;
use std::fs;

pub fn hash_object(kind: String, file: String, write: bool) -> Result<(), Box<dyn Error>> {
    let repo = Repository::for_working_directory()?;
    let obj = {
        let content = fs::read_to_string(file)?;
        Object::new(&*kind, content)?
    };
    if write {
        repo.write_object(obj)?;
    } else {
        println!("{}", Repository::hash(&*obj.serialize()));
    }
    Ok(())
}
