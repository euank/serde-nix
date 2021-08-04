use serde_nix::ser::Error;

use serde::Serialize;
use std::collections::HashMap;

#[test]
fn test_write_bool() {
    assert_eq!(serde_nix::to_string(&true).unwrap(), "true".to_string());
    assert_eq!(serde_nix::to_string(&false).unwrap(), "false".to_string());
}

#[derive(Serialize, Hash, PartialEq, Eq, Debug, Default)]
struct Person {
    name: String,
    age: i32,
}

#[test]
fn test_object() {
    let p = Person {
        name: "foo".to_string(),
        age: 20,
    };
    assert_eq!(
        serde_nix::to_string(&p).unwrap(),
        r#"{ name = "foo"; age = 20; }"#.to_string(),
    );
}

#[test]
fn test_hashmaps() {
    // default
    let mut m = HashMap::new();
    m.insert("foo", "bar");
    m.insert("bar", "baz");
    let s = serde_nix::to_string(&m).unwrap();
    let correct = vec![
        r#"{ foo = "bar"; bar = "baz"; }"#.to_string(),
        r#"{ bar = "baz"; foo = "bar"; }"#.to_string(),
    ];
    assert!(correct.contains(&s));

    // bool val
    let mut m = HashMap::new();
    m.insert("foo", true);
    assert_eq!(
        serde_nix::to_string(&m).unwrap(),
        r#"{ foo = true; }"#.to_string(),
    );

    // key that needs quoting & escaping
    let mut m = HashMap::new();
    m.insert("foo.bar${baz}", 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap(),
        r#"{ "foo.bar''${baz}" = 1; }"#.to_string(),
    );

    // char key
    let mut m = HashMap::new();
    m.insert('c', 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap(),
        r#"{ c = 1; }"#.to_string(),
    );
    let mut m = HashMap::new();
    m.insert('$', 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap(),
        r#"{ "''$" = 1; }"#.to_string(),
    );

    // All invalid keys
    let mut m = HashMap::new();
    m.insert(1, 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap_err().to_string(),
        Error::MapKeyMustBeAString.to_string(),
    );
    let mut m = HashMap::new();
    m.insert(true, 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap_err().to_string(),
        Error::MapKeyMustBeAString.to_string(),
    );
    let mut m = HashMap::new();
    m.insert(Person::default(), 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap_err().to_string(),
        Error::MapKeyMustBeAString.to_string(),
    );
    let mut m = HashMap::new();
    m.insert(("foo", "bar"), 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap_err().to_string(),
        Error::MapKeyMustBeAString.to_string(),
    );
}

#[test]
fn test_arrays() {
    assert_eq!(
        &serde_nix::to_string(&vec!["foo", "bar"]).unwrap(),
        r#"[ "foo" "bar" ]"#,
    );
}
