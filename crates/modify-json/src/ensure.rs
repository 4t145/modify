use modify::{Filter, Not, SetModification, ensure};
use serde_json::Value;

pub type JsonEnsure = Filter<Not<fn(&Value) -> bool>, SetModification<Value>>;
pub const fn ensure_array() -> JsonEnsure {
    ensure(Value::is_array as fn(&Value) -> bool, Value::Array(vec![]))
}

pub fn ensure_object() -> JsonEnsure {
    ensure(
        Value::is_object as fn(&Value) -> bool,
        Value::Object(serde_json::Map::new()),
    )
}

pub const fn ensure_string() -> JsonEnsure {
    ensure(
        Value::is_string as fn(&Value) -> bool,
        Value::String(String::new()),
    )
}

pub fn ensure_number() -> JsonEnsure {
    ensure(
        Value::is_number as fn(&Value) -> bool,
        Value::Number(serde_json::Number::from(0)),
    )
}

pub const fn ensure_boolean() -> JsonEnsure {
    ensure(Value::is_boolean as fn(&Value) -> bool, Value::Bool(false))
}

pub const fn ensure_null() -> JsonEnsure {
    ensure(Value::is_null as fn(&Value) -> bool, Value::Null)
}
