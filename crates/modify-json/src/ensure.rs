use modify::{Filter, Modification, ModificationLayer, Not, SetModification, ensure};
use serde_json::Value;

use crate::JsonObject;

pub type JsonEnsure = Filter<Not<fn(&Value) -> bool>, SetModification<Value>>;
pub const fn array() -> JsonEnsure {
    ensure(Value::is_array as fn(&Value) -> bool, Value::Array(vec![]))
}

pub fn ensure_object() -> JsonEnsure {
    ensure(
        Value::is_object as fn(&Value) -> bool,
        Value::Object(serde_json::Map::new()),
    )
}

pub const fn string() -> JsonEnsure {
    ensure(
        Value::is_string as fn(&Value) -> bool,
        Value::String(String::new()),
    )
}

pub fn number() -> JsonEnsure {
    ensure(
        Value::is_number as fn(&Value) -> bool,
        Value::Number(serde_json::Number::from(0)),
    )
}

pub const fn boolean() -> JsonEnsure {
    ensure(Value::is_boolean as fn(&Value) -> bool, Value::Bool(false))
}

pub const fn null() -> JsonEnsure {
    ensure(Value::is_null as fn(&Value) -> bool, Value::Null)
}

#[derive(Debug)]
pub struct EnsureObjectFieldIsAndThen<'k, M, T> {
    key: &'k str,
    then: M,
    as_fn: fn(&mut Value) -> Option<&mut T>,
    default_fn: fn() -> Value,
}

impl<'k, M, T> Clone for EnsureObjectFieldIsAndThen<'k, M, T>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        EnsureObjectFieldIsAndThen {
            key: self.key,
            then: self.then.clone(),
            as_fn: self.as_fn,
            default_fn: self.default_fn,
        }
    }
}

impl<'k, M, T> Modification<JsonObject> for EnsureObjectFieldIsAndThen<'k, M, T>
where
    M: Modification<T>,
{
    fn modify(self, value: &mut JsonObject) {
        let key = self.key;
        let field = value.entry(key).or_insert((self.default_fn)());
        if !field.is_array() {
            *field = (self.default_fn)()
        }
        if let Some(array) = (self.as_fn)(field) {
            self.then.modify(array);
        }
    }
}

pub struct EnsureObjectFieldIs<'k, T> {
    key: &'k str,
    as_fn: fn(&mut Value) -> Option<&mut T>,
    default_fn: fn() -> Value,
}

impl<'a, M, T> ModificationLayer<M> for EnsureObjectFieldIs<'a, T> {
    type Modification = EnsureObjectFieldIsAndThen<'a, M, T>;

    fn layer(self, layer: M) -> Self::Modification {
        EnsureObjectFieldIsAndThen {
            key: self.key,
            then: layer,
            as_fn: self.as_fn,
            default_fn: self.default_fn,
        }
    }
}

pub fn array_field(key: &str) -> EnsureObjectFieldIs<'_, Vec<Value>> {
    fn default_fn() -> Value {
        Value::Array(vec![])
    }
    EnsureObjectFieldIs {
        key,
        as_fn: Value::as_array_mut,
        default_fn,
    }
}

pub fn object_field(key: &str) -> EnsureObjectFieldIs<'_, JsonObject> {
    fn default_fn() -> Value {
        Value::Object(serde_json::Map::new())
    }
    EnsureObjectFieldIs {
        key,
        as_fn: Value::as_object_mut,
        default_fn,
    }
}

pub fn number_field(key: &str) -> EnsureObjectFieldIs<'_, serde_json::Number> {
    fn default_fn() -> Value {
        serde_json::json!(0.0)
    }
    fn as_number_mut(value: &mut Value) -> Option<&mut serde_json::Number> {
        match value {
            Value::Number(num) => Some(num),
            _ => None,
        }
    }
    EnsureObjectFieldIs {
        key,
        as_fn: as_number_mut,
        default_fn,
    }
}

pub fn string_field(key: &str) -> EnsureObjectFieldIs<'_, String> {
    fn default_fn() -> Value {
        Value::String(String::new())
    }
    fn as_string_mut(value: &mut Value) -> Option<&mut String> {
        match value {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    EnsureObjectFieldIs {
        key,
        as_fn: as_string_mut,
        default_fn,
    }
}

pub fn boolean_field(key: &str) -> EnsureObjectFieldIs<'_, bool> {
    fn default_fn() -> Value {
        Value::Bool(false)
    }
    fn as_bool_mut(value: &mut Value) -> Option<&mut bool> {
        match value {
            Value::Bool(b) => Some(b),
            _ => None,
        }
    }
    EnsureObjectFieldIs {
        key,
        as_fn: as_bool_mut,
        default_fn,
    }
}

pub fn field(key: &str) -> EnsureObjectFieldIs<'_, Value> {
    fn default_fn() -> Value {
        Value::Null
    }
    fn some(value: &mut Value) -> Option<&mut Value> {
        Some(value)
    }
    EnsureObjectFieldIs {
        key,
        as_fn: some,
        default_fn,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::JsonObject;
    use modify::{Modification, ModificationLayerExt, extend};
    macro_rules! object {
        ($($tt:tt)*) => {
            match serde_json::json!({ $($tt)* }) {
                Value::Object(map) => map,
                _ => {
                    unreachable!("This macro should only be used to create an object")
                }
            }
        };
    }
    #[derive(Modification)]
    #[modify(target = "JsonObject")]
    pub struct ObjectUpdate {
        #[modify(by = array_field("items").then(Extend))]
        items: Vec<Value>,
        #[modify(by = object_field("data").then(string_field("name")).then(Set))]
        name: String,
        // #[modify(by = extend, on = strings)]
        // strings: Vec<String>,
    }

    #[test]
    fn test_object_array_field() {
        let mut object = object! {};
        array_field("items")
            .then(modify::Extend)
            .finally([serde_json::json!(1)])
            .modify(&mut object);
        assert_eq!(
            object,
            object! {
                "items": [1]
            }
        );
    }

    #[test]
    fn test_derived() {
        let mut object = object! {};
        ObjectUpdate {
            items: vec![serde_json::json!(1)],
            name: "Alice".to_string()
        }
        .modify(&mut object);
        assert_eq!(
            object,
            object! {
                "items": [1],
                "data": {
                    "name": "Alice"
                }
            }
        );
    }
}
