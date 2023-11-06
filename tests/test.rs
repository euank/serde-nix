use std::collections::HashMap;

use quickcheck_macros::quickcheck;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_nix::ser::Error;

#[test]
fn test_write_bool() {
    assert_eq!(serde_nix::to_string(&true).unwrap(), "true".to_string());
    assert_eq!(serde_nix::to_string(&false).unwrap(), "false".to_string());
}

#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug, Default)]
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
        r#"{ "foo.bar\${baz}" = 1; }"#.to_string(),
    );

    // unusual keys that don't need quoting
    let mut m = HashMap::new();
    m.insert("a-'_b", 1);
    assert_eq!(
        serde_nix::to_string(&m).unwrap(),
        r#"{ a-'_b = 1; }"#.to_string(),
    );

    // newline value
    let mut m = HashMap::new();
    m.insert("a", "\n");
    assert_eq!(
        "{ a = \"\\n\"; }".to_string(),
        serde_nix::to_string(&m).unwrap(),
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
        r#"{ "$" = 1; }"#.to_string(),
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

#[cfg(test)]
fn round_trip<'de, T>(v: T) -> Result<(), Error>
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let nix_str = serde_nix::to_string(&v)?;
    println!("nix str: {}", nix_str);
    // evaluate with the nix interpreter, convert to json, and parse that json. :|
    let json = std::process::Command::new("nix-instantiate")
        .args(&["--eval", "--json", "-E", &nix_str])
        .output()
        .expect("could not run nix-instantiate");

    let json_str = String::from_utf8(json.stdout).unwrap();
    let obj: T = serde_json::from_str(&json_str).unwrap();

    assert_eq!(v, obj);

    Ok(())
}

#[test]
fn test_round_trip_through_nix() {
    round_trip(1).unwrap();
    round_trip(vec![1, 2, 3, 4]).unwrap();
    round_trip("foo".to_string()).unwrap();
    round_trip(Person {
        age: 10,
        name: "foo".to_string(),
    })
    .unwrap();
    round_trip((1, 2, 3)).unwrap();
}

#[quickcheck]
fn quickcheck_hashmap_keys(m: HashMap<String, String>) -> bool {
    // nix can't do null keys, don't bother
    if m.keys().chain(m.values()).any(|k| {
        k.chars().any(|c| match c {
            // nix doesn't deal with null, form feed, or backspace well
            '\0' | '\u{c}' | '\u{8}' => true,
            _ => false,
        })
    }) {
        return true;
    }
    println!("Debug: {:?}", m);
    round_trip(m).unwrap();
    true
}
