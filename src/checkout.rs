use crate::object::Object;
use crate::reference::Reference;
use crate::repository::Repository;

pub fn checkout(object: String) -> Result<(), Box<dyn std::error::Error>> {
    let reference = Reference::from_name(object.as_str())?;
    let repo = Repository::for_working_directory()?;
    // TODO[Rhys] add more relaxed safeguards here
    assert!(repo.is_empty()?);

    let hash = repo.find_commit(&reference)?;

    let tree_hash = match repo.read_object(hash.as_str())? {
        Object::Commit(data) => data.tree,
        _ => panic!("Object was not a commit."),
    };

    let tree = match repo.read_object(tree_hash.as_str())? {
        Object::Tree(data) => data,
        _ => panic!(),
    };

    repo.checkout_tree(tree, &repo.work_tree)?;
    repo.set_head(&reference)?;

    Ok(())
}
