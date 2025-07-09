use modify::{Modification, ModificationLayer};
use serde::Serialize;
#[derive(Debug, Clone, Copy)]
pub struct SetJson;
pub struct SetJsonModification<T>(pub T);

impl<T> ModificationLayer<T> for SetJson
where
    T: Serialize,
{
    type Modification = SetJsonModification<T>;

    fn layer(self, value: T) -> Self::Modification {
        SetJsonModification(value)
    }
}

impl<T: Serialize> Modification<serde_json::Value> for SetJsonModification<T> {
    fn modify(self, value: &mut serde_json::Value) {
        *value = serde_json::to_value(self.0).expect("Failed to serialize value");
    }
}

pub fn set_json<T: Serialize>(value: T) -> SetJsonModification<T> {
    SetJsonModification(value)
}
#[derive(Debug, Clone, Copy)]
pub struct ExtendJsonArray;

pub struct ExtendJsonArrayModification<I>(pub I);

impl<I, T> ModificationLayer<I> for ExtendJsonArray
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    type Modification = ExtendJsonArrayModification<I>;

    fn layer(self, items: I) -> Self::Modification {
        ExtendJsonArrayModification(items)
    }
}

impl<I, T> Modification<Vec<serde_json::Value>> for ExtendJsonArrayModification<I>
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    fn modify(self, value: &mut Vec<serde_json::Value>) {
        for item in self.0 {
            let serialized_item = serde_json::to_value(item).expect("Failed to serialize item");
            value.push(serialized_item);
        }
    }
}

pub fn extend_json_array<I, T>(items: I) -> ExtendJsonArrayModification<I>
where
    I: IntoIterator<Item = T>,
    T: Serialize,
{
    ExtendJsonArrayModification(items)
}
