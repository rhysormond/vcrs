use crate::repository::Repository;
use std::error::Error;

pub fn show(hash: String) -> Result<(), Box<dyn Error>> {
    let repo = Repository::new(std::env::current_dir()?);
    let contents = repo.read_object(&hash)?;
    println!("{:?}", contents);
    Ok(())
}
