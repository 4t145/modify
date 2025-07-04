use crate::{Modification, ModificationLayer};

pub trait Condition<A: ?Sized> {
    fn check(self, args: &A) -> bool;
}

impl<F: FnOnce(&A) -> bool, A: ?Sized> Condition<A> for F {
    fn check(self, args: &A) -> bool {
        self(args)
    }
}

pub struct Not<T>(T);

impl<A: ?Sized, T: Condition<A>> Condition<A> for Not<T> {
    fn check(self, args: &A) -> bool {
        !self.0.check(args)
    }
}

#[derive(Debug, Clone)]
pub struct Filter<C, M> {
    pub condition: C,
    pub modification: M,
}

impl<T: ?Sized, C, M> Modification<T> for Filter<C, M>
where
    C: Condition<T>,
    M: Modification<T>,
{
    fn modify(self, value: &mut T) {
        if self.condition.check(value) {
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

pub const fn ensure<C, T>(if_not: C, default: T) -> Filter<Not<C>, crate::SetModification<T>>
where
    C: FnOnce(&T) -> bool,
    T: Clone,
{
    Filter {
        condition: Not(if_not),
        modification: crate::set(default),
    }
}

pub const fn not<T>(condition: T) -> Not<T> {
    Not(condition)
}
