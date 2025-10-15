#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Uuid(u128);

impl Uuid {
    pub fn new() -> Self {
        let id = rand::random();
        Self(id)
    }
}
