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
    fn then<M2: Modification<T>>(self, then: M2) -> Then<Self, M2>
    where
        Self: Sized,
    {
        Then { first: self, then }
    }
    fn on_index<I>(self, index: I) -> Indexed<I, Self>
    where
        Self: Sized,
    {
        Indexed {
            index,
            modification: self,
        }
    }
    fn filter<C, M>(self, condition: C, modification: M) -> Then<Self, Filter<C, M>>
    where
        C: FnOnce(&T) -> bool,
        M: Modification<T>,
        Self: Sized,
    {
        self.then(Filter {
            condition,
            modification,
        })
    }
    fn map<F, M, U>(self, map: F, modification: M) -> Then<Self, Map<F, M>>
    where
        F: FnOnce(&mut T) -> &mut U,
        M: Modification<U>,
        U: ?Sized,
        Self: Sized,
    {
        self.then(Map::new(map, modification))
    }
    fn filter_map<C, M, U>(self, filter_map: C, modification: M) -> Then<Self, FilterMap<C, M>>
    where
        C: FnOnce(&mut T) -> Option<&mut U>,
        M: Modification<U>,
        U: ?Sized,
        Self: Sized,
    {
        self.then(FilterMap::new(filter_map, modification))
    }
    fn into_dyn(self) -> DynModification<T>
    where
        Self: Sized + 'static,
    {
        DynModification::new(self)
    }
}

impl<M, T: ?Sized> ModificationExt<T> for M where M: Modification<T> {}
