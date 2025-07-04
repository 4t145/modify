use modify::{Modification, ModificationLayerExt, apply, call, extend, index};

pub struct MyState {
    pub data: Vec<String>,
}
#[test]
fn test_vec() {
    let mut data = vec!["value1", "value2"];
    apply(extend(vec!["value3", "value4"]))
        .then_apply(call(|x: &mut Vec<&'static str>| {
            x.pop();
        }))
        .modify(&mut data);
    assert_eq!(data, vec!["value1", "value2", "value3"]);
}

#[test]
fn test_unsized() {
    let mut data = String::from("hello world");
    let x = &mut data[0..5];
    call(str::make_ascii_uppercase).modify(x);
    assert_eq!(x, "HELLO");
    assert_eq!(data, "HELLO world");
}

#[test]
fn test_indexed() {
    let mut data = String::from("hello world");
    index(0..5)
        .then_apply(call(str::make_ascii_uppercase))
        .modify(&mut data);
    assert_eq!(data, "HELLO world");
}
