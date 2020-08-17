use crate::repository::Repository;
use std::error::Error;

pub fn cat_file(object: String) -> Result<(), Box<dyn Error>> {
    let repo = Repository::for_working_directory()?;
    let obj = repo.read_object(repo.find_object(object)?.as_str())?;
    println!("{:#?}", obj);
    Ok(())
}
