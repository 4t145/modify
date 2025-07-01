use crate::Modification;

pub struct Set<T>(pub T);

impl<T> Modification<T> for Set<T> {
    fn modify(self, value: &mut T) {
        *value = self.0;
    }
}
