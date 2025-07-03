use crate::Modification;
#[derive(Debug, Clone)]
pub struct Map<F, M> {
    pub map: F,
    pub modification: M,
}
impl<F, M> Map<F, M> {
    pub fn new(map: F, modification: M) -> Self {
        Self { map, modification }
    }
}
impl<T: ?Sized, U: ?Sized, F, M> Modification<T> for Map<F, M>
where
    F: FnOnce(&mut T) -> &mut U,
    M: Modification<U>,
{
    fn modify(self, value: &mut T) {
        self.modification.modify((self.map)(value));
    }
}

pub fn map<F, M>(map: F, modification: M) -> Map<F, M> {
    Map::new(map, modification)
}