use rust_codegen::*;

#[test]
fn single_struct() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .field("one", "usize")
        .field("two", "String");

    let expect = r#"
struct Foo {
    one: usize,
    two: String,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_pushed_field() {
    let mut scope = Scope::new();
    let mut struct_ = Struct::new("Foo");
    let field = Field::new("one", "usize");
    struct_.push_field(field);
    scope.push_struct(struct_);

    let expect = r#"
struct Foo {
    one: usize,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_attribute() {
    let mut scope = Scope::new();
    let mut struct_ = Struct::new("Foo");
    let field = Field::new("one", "usize");
    struct_.push_field(field);
    struct_.attr("#[test]");
    scope.push_struct(struct_);

    let expect = r#"
#[test]
struct Foo {
    one: usize,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_multiple_attributes() {
    let mut scope = Scope::new();
    let mut struct_ = Struct::new("Foo");
    let field = Field::new("one", "usize");
    struct_.push_field(field);
    struct_.attr("#[test]");
    struct_.attr("#[cfg(target_os = \"linux\")]");
    scope.push_struct(struct_);

    let expect = r#"
#[test]
#[cfg(target_os = "linux")]
struct Foo {
    one: usize,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn single_struct_documented_field() {
    let mut scope = Scope::new();

    let doc = vec!["Field's documentation", "Second line"];

    let mut struct_ = Struct::new("Foo");

    let mut field1 = Field::new("one", "usize");
    field1.doc(doc.clone());
    struct_.push_field(field1);

    let mut field2 = Field::new("two", "usize");
    field2.annotation(vec![r#"#[serde(rename = "bar")]"#]);
    struct_.push_field(field2);

    let mut field3 = Field::new("three", "usize");
    field3.doc(doc).annotation(vec![
        r#"#[serde(skip_serializing)]"#,
        r#"#[serde(skip_deserializing)]"#,
    ]);
    struct_.push_field(field3);

    scope.push_struct(struct_);

    let expect = r#"
struct Foo {
    /// Field's documentation
    /// Second line
    one: usize,
    #[serde(rename = "bar")]
    two: usize,
    /// Field's documentation
    /// Second line
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    three: usize,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn empty_struct() {
    let mut scope = Scope::new();

    scope.new_struct("Foo");

    let expect = r#"
struct Foo;"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn two_structs() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .field("one", "usize")
        .field("two", "String");

    scope.new_struct("Bar").field("hello", "World");

    let expect = r#"
struct Foo {
    one: usize,
    two: String,
}

struct Bar {
    hello: World,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_derive() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .derive("Debug")
        .derive("Clone")
        .field("one", "usize")
        .field("two", "String");

    let expect = r#"
#[derive(Debug, Clone)]
struct Foo {
    one: usize,
    two: String,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_repr() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .repr("C")
        .field("one", "u8")
        .field("two", "u8");

    let expect = r#"
#[repr(C)]
struct Foo {
    one: u8,
    two: u8,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_allow() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .allow("dead_code")
        .field("one", "u8")
        .field("two", "u8");

    let expect = r#"
#[allow(dead_code)]
struct Foo {
    one: u8,
    two: u8,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_1() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .generic("T")
        .generic("U")
        .field("one", "T")
        .field("two", "U");

    let expect = r#"
struct Foo<T, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_2() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .generic("T, U")
        .field("one", "T")
        .field("two", "U");

    let expect = r#"
struct Foo<T, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_3() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .generic("T: Win, U")
        .field("one", "T")
        .field("two", "U");

    let expect = r#"
struct Foo<T: Win, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_where_clause_1() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .generic("T")
        .bound("T", "Foo")
        .field("one", "T");

    let expect = r#"
struct Foo<T>
where T: Foo,
{
    one: T,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_where_clause_2() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .generic("T, U")
        .bound("T", "Foo")
        .bound("U", "Baz")
        .field("one", "T")
        .field("two", "U");

    let expect = r#"
struct Foo<T, U>
where T: Foo,
      U: Baz,
{
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_doc() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .doc(
            "Hello, this is a doc string\n\
              that continues on another line.",
        )
        .field("one", "T");

    let expect = r#"
/// Hello, this is a doc string
/// that continues on another line.
struct Foo {
    one: T,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_in_mod() {
    let mut scope = Scope::new();

    {
        let module = scope.new_module("foo");
        module
            .new_struct("Foo")
            .doc("Hello some docs")
            .derive("Debug")
            .generic("T, U")
            .bound("T", "SomeBound")
            .bound("U", "SomeOtherBound")
            .field("one", "T")
            .field("two", "U");
    }

    let expect = r#"
mod foo {
    /// Hello some docs
    #[derive(Debug)]
    struct Foo<T, U>
    where T: SomeBound,
          U: SomeOtherBound,
    {
        one: T,
        two: U,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_mod_import() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .import("bar", "Bar")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expect = r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_multiple_allow() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .allow("dead_code")
        .allow("clippy::all")
        .field("one", "u8")
        .field("two", "u8");

    let expect = r#"
#[allow(dead_code)]
#[allow(clippy::all)]
struct Foo {
    one: u8,
    two: u8,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}