use modify::{
    ApplyFn, Call, ExtendModification, Index, IndexModification, Modification, ModificationExt,
    ModificationLayer, ModificationLayerExt, extend,
};

pub struct MyState {
    pub data: Vec<String>,
}
#[test]
fn test_vec() {
    let mut data = vec!["value1", "value2"];
    extend(vec!["value3", "value4"])
        .and_then(Call(|x: &mut Vec<_>| {
            x.pop();
        }))
        .modify(&mut data);
    assert_eq!(data, vec!["value1", "value2", "value3"]);
}

#[test]
fn test_unsized() {
    let mut data = String::from("hello world");
    let x = &mut data[0..5];
    ().call(str::make_ascii_uppercase).modify(x);
    assert_eq!(x, "HELLO");
    assert_eq!(data, "HELLO world");
}

#[test]
fn test_indexed() {
    let mut data = String::from("hello world");
    Index(0..5)
        .call(str::make_ascii_uppercase)
        .into_dyn()
        .modify(&mut data);
    assert_eq!(data, "HELLO world");
}
