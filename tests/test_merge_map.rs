use modify_by::{Modification, extend::Extend};

pub struct MyState {
    pub data: Vec<String>,
}
#[test]
fn test_merge_json_value() {
    let mut data = vec!["value1", "value2"];
    Extend(vec!["value3", "value4"]).modify(&mut data);
    assert_eq!(data, vec!["value1", "value2", "value3", "value4"]);
}
