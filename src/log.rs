use crate::object::Object;
use crate::reference::Reference;
use crate::repository::Repository;

pub fn log(object: Option<String>) {
    let head = object.unwrap_or("HEAD".to_string());
    let reference = Reference::from_name(head.as_str());
    let repo = Repository::for_working_directory();
    let mut maybe_commit = Some(repo.find_commit(&reference));
    loop {
        maybe_commit = match &maybe_commit {
            Some(commit) => {
                let obj = repo.read_object(commit);
                match obj {
                    Object::Commit(commit_obj) => {
                        println!("{} {}", commit, commit_obj.message);
                        commit_obj.parent
                    }
                    _ => panic!("Commit parent was not a commit."),
                }
            }
            None => return (),
        };
    }
}
