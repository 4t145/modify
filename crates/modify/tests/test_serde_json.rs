use std::collections::HashMap;

use modify::{DynModification, Call, Modification, ModificationExt, extend, set};
use serde_json::Value;
#[test]
fn test_extend_array() {
    let add_hello = extend_array(Some(serde_json::json!("hello")))
        .on_index("items")
        .into_dyn();
    let mut object = serde_json::json!({});
    add_hello.modify(&mut object);
    assert!(object["items"].as_array().unwrap().len() == 1);
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
    ().filter(Value::is_null, set(serde_json::json!([])))
        .filter_map(Value::as_array_mut, extend(iter))
}

pub fn extend_object(object: Value) -> impl Modification<Value> {
    ().filter(Value::is_null, set(serde_json::json!({})))
        .filter_map(
            Value::as_object_mut,
            Call(|x: &mut serde_json::Map<String, Value>| {
                if let Value::Object(obj) = object {
                    for (k, v) in obj {
                        x.insert(k, v);
                    }
                }
            }),
        )
}

pub struct JsonValueModify(pub HashMap<String, DynModification<Value>>);

impl Modification<Value> for JsonValueModify {
    fn modify(self, target: &mut Value) {
        let x = (|x: usize| x)(1);
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
