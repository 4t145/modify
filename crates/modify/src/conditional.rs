use crate::{Modification, ModificationLayer};
#[derive(Debug, Clone)]
pub struct Filter<C, M> {
    pub condition: C,
    pub modification: M,
}

impl<T: ?Sized, C, M> Modification<T> for Filter<C, M>
where
    C: FnOnce(&T) -> bool,
    M: Modification<T>,
{
    fn modify(self, value: &mut T) {
        if (self.condition)(value) {
            self.modification.modify(value);
        }
    }
}

impl<C, M> Filter<C, M> {
    pub fn new(condition: C, modification: M) -> Self {
        Self {
            condition,
            modification,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterMapModification<C, M> {
    pub filter_map: C,
    pub modification: M,
}

impl<C, M> FilterMapModification<C, M> {
    pub fn new(filter_map: C, modification: M) -> Self {
        Self {
            filter_map,
            modification,
        }
    }
}

impl<T: ?Sized, U: ?Sized, C, M> Modification<T> for FilterMapModification<C, M>
where
    C: FnOnce(&mut T) -> Option<&mut U>,
    M: Modification<U>,
{
    fn modify(self, value: &mut T) {
        if let Some(target) = (self.filter_map)(value) {
            self.modification.modify(target);
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterMap<C> {
    pub filter_map: C,
}

impl<C> FilterMap<C> {
    pub fn new(filter_map: C) -> Self {
        Self { filter_map }
    }
}

impl<C, I> ModificationLayer<I> for FilterMap<C> {
    type Modification = FilterMapModification<C, I>;

    fn layer(self, inner: I) -> Self::Modification {
        FilterMapModification {
            filter_map: self.filter_map,
            modification: inner,
        }
    }
}

pub fn filter_map<C>(filter_map: C) -> FilterMap<C> {
    FilterMap::new(filter_map)
}
