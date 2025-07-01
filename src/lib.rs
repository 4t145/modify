pub mod merge;
pub mod set;
pub mod extend;

pub use merge::Mergeable as MergeTrait;

pub trait Modification<T>: Sized {
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
