use crate::{Modification, ModificationLayer};

// m.layer(a).layer(b) == m.layer(a.then(b))
#[derive(Debug, Clone)]
pub struct ThenLayer<First, Second> {
    pub first: First,
    pub second: Second,
}

impl<First, Second, Inner> ModificationLayer<Inner> for ThenLayer<First, Second>
where
    First: ModificationLayer<<Second as ModificationLayer<Inner>>::Modification>,
    Second: ModificationLayer<Inner>,
{
    type Modification = <First as ModificationLayer<
        <Second as ModificationLayer<Inner>>::Modification,
    >>::Modification;

    fn layer(self, inner: Inner) -> Self::Modification {
        self.first.layer(self.second.layer(inner))
    }
}

impl<T: ?Sized, First, Second> Modification<T> for ThenLayer<First, Second>
where
    Self: ModificationLayer<()>,
    <Self as ModificationLayer<()>>::Modification: Modification<T>,
{
    fn modify(self, value: &mut T) {
        self.layer(()).modify(value);
    }
}

#[derive(Debug, Clone)]
pub struct Then<M>(pub M);

impl<A, B> ModificationLayer<A> for Then<B> {
    type Modification = ApplyModification<A, B>;

    fn layer(self, inner: A) -> Self::Modification {
        ApplyModification {
            current: inner,
            then: self.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApplyModification<C, M> {
    pub current: C,
    pub then: M,
}

impl<T: ?Sized, C, M> Modification<T> for ApplyModification<C, M>
where
    C: Modification<T>,
    M: Modification<T>,
{
    fn modify(self, value: &mut T) {
        self.current.modify(value);
        self.then.modify(value);
    }
}

pub struct Apply<M> {
    pub modification: M,
}

impl<M> Apply<M> {
    pub fn new(modification: M) -> Self {
        Apply { modification }
    }
}

pub fn apply<M>(modification: M) -> Apply<M> {
    Apply::new(modification)
}

impl<Current, Then> ModificationLayer<Then> for Apply<Current> {
    type Modification = ApplyModification<Current, Then>;

    fn layer(self, inner: Then) -> Self::Modification {
        ApplyModification {
            current: self.modification,
            then: inner,
        }
    }
}

impl<T: ?Sized, M: Modification<T>> Modification<T> for Apply<M> {
    fn modify(self, value: &mut T) {
        self.modification.modify(value);
    }
}
