use partial::{IntoPartial, Partial, PartialOps};

#[test]
fn simple() {
    #[derive(Debug, Default, Clone, PartialEq, Partial)]
    struct Test {
        valid: bool,
        id: u32,
        name: String,
    }

    let mut test = Test::default();
    let partial = Partial::<Test> {
        valid: Some(true),
        id: Some(123),
        name: None,
    };
    test.set(partial);
    assert_eq!(
        test,
        Test {
            valid: true,
            id: 123,
            name: String::new(),
        }
    );

    assert!(Partial::<Test>::empty().is_empty());
}

#[test]
fn generic() {
    #[derive(Debug, Clone, PartialEq, Partial)]
    struct Generic<'a, S, T>
    where
        S: Clone + Into<String>,
    {
        name: S,
        value: &'a T,
    }

    let generic = Generic::<&str, u32> {
        name: "foo",
        value: &123,
    };
    let mut partial = generic.into_partial();
    assert_eq!(partial.name, Some("foo"));
    assert_eq!(partial.value, Some(&123));

    partial.set_and(PartialGeneric::<&str, u32> {
        value: Some(&456),
        ..PartialGeneric::empty()
    });
    assert_eq!(partial.name, None);
    assert_eq!(partial.value, Some(&456));
}

#[test]
fn tuple() {
    #[derive(Debug, Default, Clone, PartialEq, Partial)]
    struct Tuple<T>(i32, T)
    where
        T: std::ops::Add<i32>;

    let partial = Tuple::<i32>::default().into_partial();
    assert_eq!(partial.0, Some(0));
    assert_eq!(partial.1, Some(0));

    let partial = partial.and(PartialTuple::<i32>(Some(123), None));
    assert_eq!(partial.0, Some(123));
    assert_eq!(partial.1, None);
}

#[test]
fn attributes() {
    #[derive(Debug, Default, Clone, Partial)]
    #[partial(name = "MyPartial")]
    #[partial(derive(Debug, Clone, PartialEq))]
    #[allow(unused)]
    struct MyStruct {
        private: i32,
        pub public: i32,
        pub(crate) restricted: i32,
    }

    let partial = MyPartial::default();
    let clone = partial.clone();
    assert_eq!(partial, clone);
}
