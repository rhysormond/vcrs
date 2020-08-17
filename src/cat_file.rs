use crate::reference::Reference;
use crate::repository::Repository;

pub fn cat_file(object: String) {
    let reference = Reference::from_name(object.as_str());
    let repo = Repository::for_working_directory();
    let obj = repo.read_object(repo.find_commit(&reference).as_str());
    println!("{:#?}", obj);
}
