use crate::object::Object;
use crate::repository::Repository;

pub fn checkout(commit: String) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::for_working_directory()?;
    // TODO[Rhys] add more relaxed safeguards here
    assert!(repo.is_empty()?);

    let tree_hash = match repo.read_object(commit.as_str())? {
        Object::Commit(data) => data.tree,
        _ => panic!("Object was not a commit."),
    };

    let tree = match repo.read_object(tree_hash.as_str())? {
        Object::Tree(data) => data,
        _ => panic!(),
    };

    repo.checkout_tree(tree, &repo.work_tree)?;

    Ok(())
}
