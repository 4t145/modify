use crate::Modification;
#[derive(Debug, Clone)]
pub struct Filter<C, M> {
    pub condition: C,
    pub modification: M,
}

impl<T: ?Sized, C, M> Modification<T> for Filter<C, M>
where
    C: FnOnce(&T) -> bool,
    M: Modification<T>,
{
    fn modify(self, value: &mut T) {
        if (self.condition)(value) {
            self.modification.modify(value);
        }
    }
}

impl<C, M> Filter<C, M> {
    pub fn new(condition: C, modification: M) -> Self {
        Self { condition, modification }
    }
}

#[derive(Debug, Clone)]
pub struct FilterMap<C, M> {
    pub filter_map: C,
    pub modification: M,
}

impl<C, M> FilterMap<C, M> {
    pub fn new(filter_map: C, modification: M) -> Self {
        Self { filter_map, modification }
    }
}

impl<T: ?Sized, U: ?Sized, C, M> Modification<T> for FilterMap<C, M>
where
    C: FnOnce(&mut T) -> Option<&mut U>,
    M: Modification<U>,
{
    fn modify(self, value: &mut T) {
        if let Some(target) = (self.filter_map)(value) {
            self.modification.modify(target);
        }
    }
}
