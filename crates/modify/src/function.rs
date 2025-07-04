use crate::{Modification, ModificationLayer};

#[derive(Debug, Clone)]
pub struct CallModification<F>(pub F);

pub fn call<F>(f: F) -> CallModification<F> {
    CallModification(f)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Call;

impl<F, T: ?Sized> Modification<T> for CallModification<F>
where
    F: FnOnce(&mut T),
{
    fn modify(self, value: &mut T) {
        (self.0)(value);
    }
}

impl<F> ModificationLayer<F> for Call {
    type Modification = CallModification<F>;

    fn layer(self, inner: F) -> Self::Modification {
        CallModification(inner)
    }
}
