use crate::Modification;

pub struct Extend<E>(pub E);

impl<T, E> Modification<T> for Extend<E>
where
    E: IntoIterator,
    T: std::iter::Extend<E::Item>,
{
    fn modify(self, value: &mut T) {
        value.extend(self.0);
    }
}
