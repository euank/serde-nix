use serde::Serialize;

#[test]
fn test_write_bool() {
    assert_eq!(serde_nix::to_string(&true).unwrap(), "true".to_string());
    assert_eq!(serde_nix::to_string(&false).unwrap(), "false".to_string());
}

#[derive(Serialize)]
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
