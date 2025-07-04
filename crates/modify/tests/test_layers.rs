use modify::{Modification, ModificationLayerExt, Set, call, index};

#[test]
fn test_layers() {
    let modification = (
        index(0..5)
            .then_apply(call(str::make_ascii_uppercase))
            .then(index(0..2))
            .then_apply(call(str::make_ascii_lowercase)),
        index(6..11)
            .then_apply(call(str::make_ascii_lowercase))
            .then(index(0..2))
            .then_apply(call(str::make_ascii_uppercase)),
    );
    let mut data = String::from("Hello World");
    modification.modify(&mut data);
    println!("Modified data: {}", data);
    assert_eq!(data, "heLLO WOrld");
}

#[test]
fn test_static_layers() {
    let modification = index(0).then(Set).finally(10);

    let mut data = [1, 3, 5, 7];
    modification.modify(&mut data);
    println!("Modified data: {:?}", data);
    assert_eq!(data, [10, 3, 5, 7]);
}
