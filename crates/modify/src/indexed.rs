use std::ops::IndexMut;

use crate::{Modification, ModificationLayer};

pub struct Index<I>(pub I);

impl<I, M> ModificationLayer<M> for Index<I> {
    type Modification = IndexModification<I, M>;

    fn layer(self, inner: M) -> Self::Modification {
        IndexModification {
            index: self.0,
            modification: inner,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndexModification<I, M> {
    pub index: I,
    pub modification: M,
}

impl<I, M> IndexModification<I, M> {
    pub fn new(index: I, modification: M) -> Self {
        IndexModification {
            index,
            modification,
        }
    }
}

impl<T, I, M> Modification<T> for IndexModification<I, M>
where
    T: IndexMut<I> + ?Sized,
    M: Modification<T::Output>,
{
    fn modify(self, value: &mut T) {
        let item = value.index_mut(self.index);
        self.modification.modify(item);
    }
}

pub fn index<I>(index: I) -> Index<I> {
    Index(index)
}