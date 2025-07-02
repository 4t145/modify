use crate::Modification;

#[derive(Debug, Clone)]
pub struct Fn<F>(pub F);

impl<F, T: ?Sized> Modification<T> for Fn<F>
where
    F: FnOnce(&mut T),
{
    fn modify(self, value: &mut T) {
        (self.0)(value);
    }
}
