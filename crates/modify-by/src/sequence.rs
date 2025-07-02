use crate::Modification;

#[derive(Debug, Clone)]
pub struct Sequence<I>(pub I);

impl<I> Sequence<I> {
    pub fn from_iter<T: IntoIterator<IntoIter = I>>(iter: T) -> Self {
        Sequence(iter.into_iter())
    }
}

impl<T: ?Sized, I> Modification<T> for Sequence<I>
where
    I: Iterator,
    I::Item: Modification<T>,
{
    fn modify(self, value: &mut T) {
        for item in self.0 {
            item.modify(value);
        }
    }
}

impl<const N: usize, T: ?Sized, M> Modification<T> for [M; N]
where
    M: Modification<T>,
{
    fn modify(self, value: &mut T) {
        Sequence::from_iter(self).modify(value);
    }
}

impl<T: ?Sized, M> Modification<T> for Vec<M>
where
    M: Modification<T>,
{
    fn modify(self, value: &mut T) {
        Sequence::from_iter(self).modify(value);
    }
}
