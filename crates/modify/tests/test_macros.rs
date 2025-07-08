use std::time::SystemTime;

use modify::Modification;

pub struct MyData {
    items: Vec<String>,
    name: String,
    index: u32,
    latest_update: SystemTime,
}

#[derive(Modification)]
#[modify(target = "MyData")]
pub struct MyUpdate {
    #[modify(by = Extend, on = items)]
    new_items: Vec<String>,
    #[modify(by = Set, on = latest_update)]
    update_time: SystemTime,
}
// impl modify::Modification<MyData> for MyUpdate {
//     fn modify(self, target: &mut MyData) {
//         use modify::*;
//         // (map(|target: &mut MyData| &mut target.items).then(Extend))
//         //     .finally(self.new_items)
//         //     .modify(target);
//         Set.finally(self.update_time)
//             .modify(&mut target.latest_update);
//     }
// }
#[test]
fn test_modification() {
    let mut data = MyData {
        items: vec!["item1".to_string(), "item2".to_string()],
        name: "Test".to_string(),
        index: 1,
        latest_update: SystemTime::now(),
    };

    let update = MyUpdate {
        new_items: vec!["item3".to_string(), "item4".to_string()],
        update_time: SystemTime::now(),
    };

    update.modify(&mut data);

    assert_eq!(data.items, vec!["item1", "item2", "item3", "item4"]);
}
