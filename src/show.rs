use crate::repository::Repository;
use std::error::Error;

pub fn show(kind: String, object: String) -> Result<(), Box<dyn Error>> {
    let repo = Repository::for_working_directory()?;
    let obj = repo.read_object(&*Repository::find_object(kind, object)?)?;
    println!("{:?}", obj);
    println!("{}", String::from_utf8(obj.serialize())?);
    Ok(())
}
