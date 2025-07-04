use std::collections::HashMap;

use modify::{
    DynModification, Modification, ModificationLayerExt, apply, call, extend, filter_map, index,
    set,
};
use serde_json::Value;
#[test]
fn test_extend_array() {
    let add_hello = index("items").then_apply(extend_array(Some(serde_json::json!("hello"))));
    let mut object = serde_json::json!({});
    add_hello.modify(&mut object);
    assert!(object["items"].as_array().unwrap().len() == 1);
}

#[test]
fn test_many_index() {
    let set_name = index("items")
        .then(index(1))
        .then(index("value"))
        .finally(set(serde_json::json!("bob")));

    let mut object = serde_json::json!({
        "items": [
            {"value": "hello"},
            {"value": "world"}
        ]
    });
    set_name.modify(&mut object);
    assert!(object["items"][1]["value"].as_str() == Some("bob"));
}

#[test]
fn test_merge_object() {
    let mut object_1 = serde_json::json!({
        "items": ["world"]
    });
    let object_2 = serde_json::json!({
        "items": ["hello"]
    });
    extend_object(object_2).modify(&mut object_1);
    assert!(object_1["items"].as_array().unwrap().len() == 1);
}

pub fn extend_array<E>(iter: E) -> impl Modification<Value>
where
    E: IntoIterator<Item = Value>,
{
    fn set_empty_array_if_null(value: &mut Value) {
        if value.is_null() {
            *value = serde_json::json!([]);
        }
    }
    apply(call(set_empty_array_if_null))
        .then(filter_map(Value::as_array_mut))
        .finally(extend(iter))
}

pub fn extend_object(object: Value) -> impl Modification<Value> {
    fn set_empty_object_if_null(value: &mut Value) {
        if value.is_null() {
            *value = serde_json::json!({});
        }
    }
    apply(call(set_empty_object_if_null))
        .then(filter_map(Value::as_object_mut))
        .finally(call(|x: &mut serde_json::Map<String, Value>| {
            if let Value::Object(obj) = object {
                for (k, v) in obj {
                    x.insert(k, v);
                }
            }
        }))
}

pub struct JsonValueModify(pub HashMap<String, DynModification<Value>>);

impl Modification<Value> for JsonValueModify {
    fn modify(self, target: &mut Value) {
        if let Value::Object(map) = target {
            for (key, modification) in self.0 {
                if let Some(value) = map.get_mut(&key) {
                    modification.modify(value);
                } else {
                    let mut value = Value::Null;
                    modification.modify(&mut value);
                    map.insert(key, value);
                }
            }
        }
    }
}
