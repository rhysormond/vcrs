use crate::object::Object;
use crate::reference::Reference;
use crate::repository::Repository;
use std::error::Error;

pub fn log(object: String) -> Result<(), Box<dyn Error>> {
    let reference = Reference::from_name(object.as_str())?;
    let repo = Repository::for_working_directory()?;
    let mut maybe_commit = Some(repo.find_commit(&reference)?);
    loop {
        maybe_commit = match &maybe_commit {
            Some(commit) => {
                let obj = repo.read_object(commit).expect(&*format!(
                    "Object {} was either not a commit or was packed",
                    commit
                ));
                match obj {
                    Object::Commit(commit_obj) => {
                        println!("{} {}", commit, commit_obj.message);
                        commit_obj.parent
                    }
                    _ => panic!("Commit parent was not a commit."),
                }
            }
            None => return Ok(()),
        };
    }
}
