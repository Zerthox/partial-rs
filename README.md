# Partial
Partial representations of types.

## Usage
```rs
use partial::{Partial, IntoPartial, PartialOps};

#[derive(Debug, Default, Clone, Partial)]
struct MyStruct {
    valid: bool,
    id: u32,
    name: String,
}

let mut first = MyStruct {
    valid: true,
    id: 123,
    name: "foo".into(),
};
let second = MyStruct {
    valid: false,
    id: 456,
    name: "bar".into(),
};

let update = second.into_partial().and(Partial::<MyStruct> {
    id: Some(456), // only change id
    ..Partial::<MyStruct>::empty()
});
value.set(update);
```
