use modify::{Extend, Fn, Indexed, Modification, ModificationExt};

pub struct MyState {
    pub data: Vec<String>,
}
#[test]
fn test_vec() {
    let mut data = vec!["value1", "value2"];
    Extend(vec!["value3", "value4"])
        .then(Fn(|x: &mut Vec<_>| {
            x.pop();
        }))
        .into_dyn()
        .modify(&mut data);
    assert_eq!(data, vec!["value1", "value2", "value3"]);
}

#[test]
fn test_unsized() {
    let mut data = String::from("hello world");
    let mut x = &mut data[0..5];
    Fn(|x: &mut str| {
        x.make_ascii_uppercase();
    })
    .modify(&mut x);
    assert_eq!(x, "HELLO");
    assert_eq!(data, "HELLO world");
}

#[test]
fn test_indexed() {
    let mut data = String::from("hello world");
    Indexed::new(0..5, Fn(str::make_ascii_uppercase))
        .into_dyn()
        .modify(&mut data);
    assert_eq!(data, "HELLO world");
}
