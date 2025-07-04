use crate::{Modification, ModificationLayer};

#[derive(Debug, Clone)]
pub struct ExtendModification<E>(pub E);

impl<T, E> Modification<T> for ExtendModification<E>
where
    E: IntoIterator,
    T: ?Sized + std::iter::Extend<E::Item>,
{
    fn modify(self, value: &mut T) {
        value.extend(self.0);
    }
}

pub fn extend<E>(e: E) -> ExtendModification<E> {
    ExtendModification(e)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Extend;

impl<Inner> ModificationLayer<Inner> for Extend {
    type Modification = ExtendModification<Inner>;

    fn layer(self, inner: Inner) -> Self::Modification {
        ExtendModification(inner)
    }
}
