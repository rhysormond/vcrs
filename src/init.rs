use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::{fs, io};

use crate::repository::Repository;

pub fn init() -> Result<(), Box<dyn Error>> {
    let repo = Repository::new(std::env::current_dir()?);

    if !repo.work_tree.read_dir()?.next().is_none() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("{:?} is not empty", repo.work_tree),
        )));
    }

    fs::create_dir_all(repo.objects)?;
    fs::create_dir_all(repo.heads)?;
    let mut file = File::create(repo.head)?;
    file.write_all(b"ref: refs/heads/master\n")?;

    Ok(())
}
