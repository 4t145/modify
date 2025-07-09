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

#[derive(Debug, Clone)]
pub struct OptionalSetModification<T>(pub T);

impl<T> Modification<T> for OptionalSetModification<Option<T>> {
    fn modify(self, value: &mut T) {
        if let Some(v) = self.0 {
            *value = v;
        }
    }
}

pub const fn optional_set<T>(value: T) -> OptionalSetModification<T> {
    OptionalSetModification(value)
}
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionalSet;

impl<T> ModificationLayer<T> for OptionalSet {
    type Modification = OptionalSetModification<T>;

    fn layer(self, inner: T) -> Self::Modification {
        OptionalSetModification(inner)
    }
}
