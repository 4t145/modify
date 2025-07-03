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

#[derive(Debug, Clone)]
pub struct Then<M>(pub M);

impl<A, B> ModificationLayer<A> for Then<B> {
    type Modification = ThenModification<A, B>;

    fn layer(self, inner: A) -> Self::Modification {
        ThenModification {
            first: inner,
            then: self.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThenModification<A, B> {
    pub first: A,
    pub then: B,
}

impl<T: ?Sized, A, B> Modification<T> for ThenModification<A, B>
where
    A: Modification<T>,
    B: Modification<T>,
{
    fn modify(self, value: &mut T) {
        self.first.modify(value);
        self.then.modify(value);
    }
}
