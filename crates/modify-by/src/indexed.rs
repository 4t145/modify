use std::ops::IndexMut;

use crate::Modification;

#[derive(Debug, Clone)]
pub struct Indexed<I, M> {
    pub index: I,
    pub modification: M,
}

impl<I, M> Indexed<I, M> {
    pub fn new(index: I, modification: M) -> Self {
        Indexed {
            index,
            modification,
        }
    }
}

impl<T, I, M> Modification<T> for Indexed<I, M>
where
    T: IndexMut<I> + ?Sized,
    M: Modification<T::Output>,
{
    fn modify(self, value: &mut T) {
        let item = value.index_mut(self.index);
        self.modification.modify(item);
    }
}
