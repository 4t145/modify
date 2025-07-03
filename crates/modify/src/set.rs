use crate::Modification;

#[derive(Debug, Clone)]
pub struct Set<T>(pub T);

impl<T> Modification<T> for Set<T> {
    fn modify(self, value: &mut T) {
        *value = self.0;
    }
}

pub const fn set<T>(value: T) -> Set<T> {
    Set(value)
}
