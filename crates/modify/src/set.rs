use crate::{Modification, ModificationLayer};

#[derive(Debug, Clone)]
pub struct SetModification<T>(pub T);

impl<T> Modification<T> for SetModification<T> {
    fn modify(self, value: &mut T) {
        *value = self.0;
    }
}

pub const fn set<T>(value: T) -> SetModification<T> {
    SetModification(value)
}
#[derive(Debug, Clone, Copy, Default)]
pub struct Set;

impl<T> ModificationLayer<T> for Set {
    type Modification = SetModification<T>;

    fn layer(self, inner: T) -> Self::Modification {
        SetModification(inner)
    }
}
