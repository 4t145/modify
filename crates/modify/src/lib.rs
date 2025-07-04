//! - Modification: Data -> Data
//! - ModificationLayer: Modification -> Modification
//! 
//! 
mod extend;
pub use extend::*;
mod function;
pub use function::*;
mod merge;
pub use merge::*;
mod set;
pub use set::*;
mod then;
pub use then::*;
mod sequence;
mod tuple;
pub use sequence::*;
mod indexed;
pub use indexed::*;
mod conditional;
pub use conditional::*;
mod map;
pub use map::*;

mod macros;

pub trait ModificationLayer<M> {
    type Modification;
    fn layer(self, inner: M) -> Self::Modification;
}

pub trait ModificationLayerExt {
    fn then<Next>(self, next: Next) -> ThenLayer<Self, Next>
    where
        Self: Sized,
    {
        ThenLayer {
            first: self,
            second: next,
        }
    }
    fn then_apply<M>(self, modification: M) -> ThenLayer<Self, Apply<M>>
    where
        Self: Sized + ModificationLayer<M>,
    {
        self.then(Apply { modification })
    }
    fn finally<M>(self, modification: M) -> Self::Modification
    where
        Self: Sized + ModificationLayer<M>,
    {
        self.layer(modification)
    }
    fn finish(self) -> Self::Modification
    where
        Self: Sized + ModificationLayer<()>,
    {
        self.finally(())
    }
}

impl<T> ModificationLayerExt for T {}

pub trait Modification<T: ?Sized> {
    fn modify(self, value: &mut T);
}

pub struct DynModification<T: ?Sized> {
    pub inner: Box<dyn FnOnce(&mut T)>,
}

impl<T: ?Sized> DynModification<T> {
    fn new<M: Modification<T> + 'static>(modification: M) -> Self {
        Self {
            inner: Box::new(move |value| modification.modify(value)),
        }
    }
}

impl<T: ?Sized> Modification<T> for DynModification<T> {
    fn modify(self, value: &mut T) {
        (self.inner)(value);
    }
}

pub trait ModificationExt<T: ?Sized>: Modification<T> {
    fn layer<L: ModificationLayer<Self>>(self, layer: L) -> L::Modification
    where
        Self: Sized,
    {
        layer.layer(self)
    }
    fn into_dyn(self) -> DynModification<T>
    where
        Self: Sized + 'static,
    {
        DynModification::new(self)
    }
}

impl<M, T: ?Sized> ModificationExt<T> for M where M: Modification<T> {}

impl<T: ?Sized + 'static> std::ops::Mul<DynModification<T>> for DynModification<T> {
    type Output = DynModification<T>;
    fn mul(self, rhs: DynModification<T>) -> Self::Output {
        apply(self).then_apply(rhs).into_dyn()
    }
}
