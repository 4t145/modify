#[cfg(feature = "serde")]
mod serde_impl;
use merge::Merge as MergeTrait;
use std::ops::{Deref, DerefMut};

pub trait Modification<T> {
    fn modify(self, value: &mut T);
}

pub struct ModificationFn<F>(pub F);

impl<F, T> Modification<T> for ModificationFn<F>
where
    F: FnOnce(&mut T),
{
    fn modify(self, value: &mut T) {
        (self.0)(value);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Merge<M>(pub M);

pub trait MergeAnnotation<T> {
    fn merge_value(&self, raw: &mut T, input: T);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Annotated<T, A = Replace> {
    value: T,
    annotation: A,
}

impl<T, A> std::fmt::Display for Annotated<T, A>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T, A> Deref for Annotated<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T, A> DerefMut for Annotated<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T, A> Annotated<T, A> {
    pub fn new(value: T, annotation: A) -> Self {
        Self { value, annotation }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn annotation(&self) -> &A {
        &self.annotation
    }

    pub fn annotation_mut(&mut self) -> &mut A {
        &mut self.annotation
    }

    pub fn map_annotation<A2>(self, f: impl FnOnce(A) -> A2) -> Annotated<T, A2> {
        Annotated {
            value: self.value,
            annotation: f(self.annotation),
        }
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Annotated<U, A> {
        Annotated {
            value: f(self.value),
            annotation: self.annotation,
        }
    }
}

impl<T, A> MergeTrait for Annotated<T, A>
where
    A: MergeAnnotation<T>,
{
    fn merge(&mut self, input: Self) {
        self.annotation.merge_value(&mut self.value, input.value);
    }
}

impl<T> MergeAnnotation<T> for fn(&mut T, T) {
    fn merge_value(&self, raw: &mut T, input: T) {
        self(raw, input);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Replace;

impl<T> MergeAnnotation<T> for Replace {
    fn merge_value(&self, raw: &mut T, input: T) {
        *raw = input;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Remain;

impl<T> MergeAnnotation<T> for Remain {
    fn merge_value(&self, _raw: &mut T, _input: T) {}
}
