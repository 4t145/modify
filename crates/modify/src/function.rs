use std::marker::PhantomData;

use crate::{Modification, ModificationLayer};

#[derive(Debug, Clone)]
pub struct Call<F>(pub F);

impl<F, T: ?Sized> Modification<T> for Call<F>
where
    F: FnOnce(&mut T),
{
    fn modify(self, value: &mut T) {
        (self.0)(value);
    }
}

// pub struct Call<T: ?Sized>(PhantomData<*const fn() -> fn(&mut T)>);

// impl<T: ?Sized> Default for Call<T> {
//     fn default() -> Self {
//         Call(PhantomData)
//     }
// }

// impl<F, T: ?Sized> ModificationLayer<F> for Call<T>
// where
//     Closure<F>: Modification<T>,
// {
//     type Modification = Closure<F>;

//     fn layer(self, inner: F) -> Self::Modification {
//         Closure(inner)
//     }
// }
