use codify::typescript::Type;
use codify::{rust, ToRust};

#[test]
fn typescript_to_rust_and_back() {
    let type_map: [(Type, rust::Type); 7] = [
        (Type::Any, rust::Type::Any),
        (Type::Boolean, rust::Type::Bool),
        (Type::Number, rust::Type::F64),
        (Type::BigInt, rust::Type::I128),
        (Type::String, rust::Type::String),
        (
            Type::Array(Box::new(Type::BigInt)),
            rust::Type::Vec(Box::new(rust::Type::I128)),
        ),
        (
            Type::Map(Box::new(Type::String), Box::new(Type::Number)),
            rust::Type::Map(Box::new(rust::Type::String), Box::new(rust::Type::F64)),
        ),
    ];

    for (ts_type, rust_type) in type_map {
        assert_eq!(Some(rust_type), ts_type.to_rust());
        assert_eq!(
            Ok(ts_type.clone()),
            codify::typescript::Type::try_from(ts_type)
        );
    }
}

#[test]
fn typescript_format() {
    assert_eq!("any", Type::Any.to_string());
    assert_eq!("undefined", Type::Undefined.to_string());
    assert_eq!("boolean", Type::Boolean.to_string());
    assert_eq!("number", Type::Number.to_string());
    assert_eq!("bigint", Type::BigInt.to_string());
    assert_eq!("string", Type::String.to_string());
    assert_eq!(
        "Array<number>",
        Type::Array(Box::new(Type::Number)).to_string()
    );
    assert_eq!(
        "Map<number, string>",
        Type::Map(Box::new(Type::Number), Box::new(Type::String)).to_string()
    );
}

#[test]
fn typescript_parse() {
    assert_eq!(Ok(Type::Any), "any".parse());
    assert_eq!(Ok(Type::Undefined), "undefined".parse());
    assert_eq!(Ok(Type::Undefined), "Undefined".parse());
    assert_eq!(Ok(Type::Boolean), "boolean".parse());
    assert_eq!(Ok(Type::Number), "Number".parse());
    assert_eq!(Ok(Type::BigInt), "bigint".parse());
    assert_eq!(Ok(Type::String), "String".parse());
    assert_eq!(Ok(Type::Array(Box::new(Type::Number))), "Number[]".parse());
    assert_eq!(
        Ok(Type::Array(Box::new(Type::Number))),
        "Number  []".parse()
    );
    assert_eq!(
        Ok(Type::Array(Box::new(Type::Number))),
        "Array<number>".parse()
    );
    assert_eq!(
        Ok(Type::Array(Box::new(Type::Number))),
        "Array    <   number  >".parse()
    );
    assert_eq!(
        Ok(Type::Map(Box::new(Type::Number), Box::new(Type::String))),
        "Map<number, string>".parse()
    );
    assert_eq!(
        Ok(Type::Map(Box::new(Type::Number), Box::new(Type::String))),
        "Map    <   number  ,   string  >".parse()
    );
}
