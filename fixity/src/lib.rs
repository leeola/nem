pub type Result<T> = std::result::Result<T, ()>;

pub trait Fixity {
    fn new() -> Id;
    fn push<T>(content: T, id: Option<Id>) -> Result<Commit>;
    fn clone() -> ();
}

pub struct Id(Vec<u8>);
pub struct Hash(Vec<u8>);

pub struct Commit {
    pub id: Id,
    pub content: Hash,
    pub prev_commit: Option<Hash>,
}
