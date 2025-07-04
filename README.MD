# Modify: fn(&mut Data)

A functional toolkit liberary to modify your data. 

This liberary allows you pass your modification between different block of your code.

## Define a Modification
```rust
let modification = index(0..5)
    .then_apply(call(str::make_ascii_uppercase)).into_dyn();

// somewhere else
let mut data = String::from("hello world");

// apply the modification
modification.modify(&mut data);
```

## Create a typed Modification

```rust
#[derive(Modification, Serialize, Deserialize)]
#[modify(target = "MyData<T>")]
pub struct MyUpdate {
    #[modify(with = extend)]
    items: Vec<T>,
    #[modify(with = set, field = updated_at)]
    time: Dtu
}
```