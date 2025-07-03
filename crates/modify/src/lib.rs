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
mod step;
pub use step::*;
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
    fn apply<M>(self, modification: M) -> Self::Modification
    where
        Self: Sized + ModificationLayer<M>,
    {
        self.layer(modification)
    }
}
pub trait ApplyFn<F>: ModificationLayer<Call<F>> {
    fn call(self, function: F) -> Self::Modification
    where
        Self: Sized,
    {
        self.layer(Call(function))
    }
}

pub trait ApplySet<T>: ModificationLayer<Set<T>> {
    fn set(self, value: T) -> Self::Modification
    where
        Self: Sized,
    {
        self.layer(Set(value))
    }
}
impl<T> ModificationLayerExt for T {}
impl<T, F> ApplyFn<F> for T where T: ModificationLayer<Call<F>> {}
impl<T, V> ApplySet<V> for T where T: ModificationLayer<Set<V>> {}

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
    fn and_then<M2: Modification<T>>(self, then: M2) -> StepModification<Self, M2>
    where
        Self: Sized,
    {
        self.layer(Then(then))
    }
    // fn on_index<I>(self, index: I) -> IndexedModification<I, Self>
    // where
    //     Self: Sized,
    // {
    //     IndexedModification {
    //         index,
    //         modification: self,
    //     }
    // }
    // fn filter<C, M>(self, condition: C, modification: M) -> ThenModification<Self, Filter<C, M>>
    // where
    //     C: FnOnce(&T) -> bool,
    //     M: Modification<T>,
    //     Self: Sized,
    // {
    //     self.then(Filter {
    //         condition,
    //         modification,
    //     })
    // }
    // fn map<F, M, U>(self, map: F, modification: M) -> ThenModification<Self, Map<F, M>>
    // where
    //     F: FnOnce(&mut T) -> &mut U,
    //     M: Modification<U>,
    //     U: ?Sized,
    //     Self: Sized,
    // {
    //     self.then(Map::new(map, modification))
    // }
    // fn filter_map<C, M, U>(
    //     self,
    //     filter_map: C,
    //     modification: M,
    // ) -> ThenModification<Self, FilterMap<C, M>>
    // where
    //     C: FnOnce(&mut T) -> Option<&mut U>,
    //     M: Modification<U>,
    //     U: ?Sized,
    //     Self: Sized,
    // {
    //     self.then(FilterMap::new(filter_map, modification))
    // }
    fn into_dyn(self) -> DynModification<T>
    where
        Self: Sized + 'static,
    {
        DynModification::new(self)
    }
}

impl<M, T: ?Sized> ModificationExt<T> for M where M: Modification<T> {}
