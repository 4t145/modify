use jsonpath_rust::query::{QueryPath, queryable::Queryable};

use modify::{ApplySet as _, Modification, ModificationLayer, ModificationLayerExt, Set};
use serde_json::json;

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
        if let Some(v) = value.reference_mut(self.path) { self.modify.modify(v); }
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


#[test]
fn test() {
    let mut json = json!({
        "items": [
            {"name": "item1", "value": 10},
            {"name": "item2", "value": 20}
        ]
    });
    json.reference("$.items[0].value").unwrap();
    on_json_path("$.items[0].value")
        .set(json!(10.5))
        .modify(&mut json);
    assert_eq!(
        json,
        json!({
            "items": [
                {"name": "item1", "value": 10.5},
                {"name": "item2", "value": 20}
            ]
        })
    );
}

// on_json_path("items/0").layer(set),
// pub struct MyModification {
    // #[modify(on_json_path($.items).then(Serde))]
    // items: Extend<Vec<String>>
// }