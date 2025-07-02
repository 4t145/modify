use crate::Modification;

#[derive(Debug, Clone)]
pub struct Then<A, B> {
    pub first: A,
    pub then: B,
}

impl<T: ?Sized, A, B> Modification<T> for Then<A, B>
where
    A: Modification<T>,
    B: Modification<T>,
{
    fn modify(self, value: &mut T) {
        self.first.modify(value);
        self.then.modify(value);
    }
}
