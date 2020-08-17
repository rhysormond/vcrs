use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::repository::Repository;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::for_working_directory()?;
    assert!(repo.is_empty()?);

    // TODO[Rhys] we need to create things like config, description, tags, etc.
    fs::create_dir_all(repo.objects)?;
    fs::create_dir_all(repo.refs)?;
    let mut file = File::create(repo.head)?;
    file.write_all(b"ref: refs/heads/master\n")?;

    Ok(())
}
