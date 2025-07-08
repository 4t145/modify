use jsonpath_rust::query::{QueryPath, queryable::Queryable};

use modify::{Modification, ModificationLayer};

pub struct OnJsonPathModification<M> {
    path: QueryPath,
    modify: M,
}

pub struct OnJsonPath {
    path: QueryPath,
}

impl OnJsonPath {
    pub fn new(path: impl Into<QueryPath>) -> Self {
        Self { path: path.into() }
    }
}

impl<M> OnJsonPathModification<M> {
    pub fn new(path: impl Into<QueryPath>, modify: M) -> Self {
        Self {
            path: path.into(),
            modify,
        }
    }
}

impl<M> Modification<serde_json::Value> for OnJsonPathModification<M>
where
    M: Modification<serde_json::Value>,
{
    fn modify(self, value: &mut serde_json::Value) {
        if let Some(v) = value.reference_mut(self.path) {
            self.modify.modify(v);
        }
    }
}

impl<M> ModificationLayer<M> for OnJsonPath {
    type Modification = OnJsonPathModification<M>;

    fn layer(self, inner: M) -> Self::Modification {
        OnJsonPathModification::new(self.path, inner)
    }
}

pub fn on_json_path<P: Into<QueryPath>>(path: P) -> OnJsonPath {
    OnJsonPath::new(path)
}

#[cfg(test)]
mod test {
    use modify::{ModificationLayerExt, index, set};
    use serde_json::json;

    use crate::ensure::number;

    use super::*;
    #[test]
    fn test() {
        let mut json = json!({
            "items": [
                {"name": "item1", "value": 10},
                {"name": "item2", }
            ]
        });

        json.reference("$.items[0].value").unwrap();
        on_json_path("$.items[0].value")
            .then_apply(set(json!(10.5)))
            .then_apply(
                index("items")
                    .then(index(1))
                    .then(index("value").then_apply(number()))
                    .finally(set(serde_json::json!(20.5)))
                    .modify(&mut json),
            )
            .modify(&mut json);

        assert_eq!(
            json,
            json!({
                "items": [
                    {"name": "item1", "value": 10.5},
                    {"name": "item2", "value": 20.5}
                ]
            })
        );
    }
}

// on_json_path("items/0").layer(set),
// pub struct MyModification {
//     #[modify(index("items").then_apply(ensure_array()).then(Extend))]
//     items: Vec<String>,
//
// }
