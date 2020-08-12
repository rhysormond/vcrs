#[derive(Debug)]
pub enum Object {
    Commit(String),
    Tree(String),
    Blob(String),
}
