use crate::Modification;

pub trait Mergeable {
    fn merge(&mut self, input: Self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Merge<M>(pub M);

impl<T: Mergeable> Modification<T> for Merge<T> {
    fn modify(self, value: &mut T) {
        value.merge(self.0);
    }
}
