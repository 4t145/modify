use std::ops::IndexMut;

use crate::{Modification, ModificationLayer};

pub struct Indexed<I>(pub I);

impl<I, M> ModificationLayer<M> for Indexed<I> {
    type Modification = IndexedModification<I, M>;

    fn layer(self, inner: M) -> Self::Modification {
        IndexedModification {
            index: self.0,
            modification: inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexedModification<I, M> {
    pub index: I,
    pub modification: M,
}

impl<I, M> IndexedModification<I, M> {
    pub fn new(index: I, modification: M) -> Self {
        IndexedModification {
            index,
            modification,
        }
    }
}

impl<T, I, M> Modification<T> for IndexedModification<I, M>
where
    T: IndexMut<I> + ?Sized,
    M: Modification<T::Output>,
{
    fn modify(self, value: &mut T) {
        let item = value.index_mut(self.index);
        self.modification.modify(item);
    }
}
