use crate::{Modification, ModificationLayer};
#[derive(Debug, Clone)]
pub struct MapModification<F, M> {
    pub map: F,
    pub modification: M,
}
impl<F, M> MapModification<F, M> {
    pub fn new(map: F, modification: M) -> Self {
        Self { map, modification }
    }
}
impl<T: ?Sized, U: ?Sized, F, M> Modification<T> for MapModification<F, M>
where
    F: FnOnce(&mut T) -> &mut U,
    M: Modification<U>,
{
    fn modify(self, value: &mut T) {
        self.modification.modify((self.map)(value));
    }
}

pub fn map<F>(map: F) -> Map<F> {
    Map::new(map)
}

pub struct Map<F> {
    pub map: F,
}

impl<M> Map<M> {
    pub fn new(map: M) -> Self {
        Self { map }
    }
}

impl<F, I> ModificationLayer<I> for Map<F> {
    type Modification = MapModification<F, I>;

    fn layer(self, inner: I) -> Self::Modification {
        MapModification::new(self.map, inner)
    }
}
