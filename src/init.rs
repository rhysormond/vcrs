use std::fs::File;
use std::io::prelude::*;
use std::{fs, io};

use crate::repository::Repository;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::for_working_directory()?;

    if !repo.work_tree.read_dir()?.next().is_none() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("{:?} is not empty", repo.work_tree),
        )));
    }

    // TODO[Rhys] we need to create things like config, description, tags, etc.
    fs::create_dir_all(repo.objects)?;
    fs::create_dir_all(repo.heads)?;
    let mut file = File::create(repo.head)?;
    file.write_all(b"ref: refs/heads/master\n")?;

    Ok(())
}
