# Partial
Partial representations of types.

## Usage
```rs
use partial::{Partial, IntoPartial, PartialOps};

#[derive(Debug, Default, Clone, Partial)]
#[partial(derive(Debug, Clone))]
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

Other types implementing `IntoPartial` can be used as their `Partial` representation instead of `Option`:
```rs
#[derive(Debug, Default, Partial)]
struct Inner {
    valid: bool,
    id: u32,
    name: String,
}

#[derive(Debug, Default, Partial)]
struct Outer {
    #[partial(flatten)]
    inner: Inner,
    primitive: i32,
}

let mut value = Outer::default();
let update = Partial::<Outer> {
    inner: Partial::<Inner> {
        valid: Some(true),
        ..Partial::<Inner>::empty()
    },
    primitive: None,
};
value.set(update);
```
