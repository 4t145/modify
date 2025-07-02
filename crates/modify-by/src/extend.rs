use crate::Modification;

#[derive(Debug, Clone)]
pub struct Extend<E>(pub E);

impl<T, E> Modification<T> for Extend<E>
where
    E: IntoIterator,
    T: ?Sized + std::iter::Extend<E::Item>,
{
    fn modify(self, value: &mut T) {
        value.extend(self.0);
    }
}

pub fn extend<E>(iter: E) -> Extend<E> {
    Extend(iter)
}