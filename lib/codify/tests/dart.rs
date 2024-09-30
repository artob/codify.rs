use codify::dart::Type;
use codify::{rust, ToRust};

#[test]
fn dart_to_rust_and_back() {
    let type_map: [(Type, rust::Type); 8] = [
        (Type::Void, rust::Type::Unit),
        (Type::Object, rust::Type::Any),
        (Type::Bool, rust::Type::Bool),
        (Type::Double, rust::Type::F64),
        (Type::Int, rust::Type::Isize),
        (Type::String, rust::Type::String),
        (
            Type::List(Box::new(Type::Double)),
            rust::Type::Vec(Box::new(rust::Type::F64)),
        ),
        (
            Type::Map(Box::new(Type::String), Box::new(Type::Int)),
            rust::Type::Map(Box::new(rust::Type::String), Box::new(rust::Type::Isize)),
        ),
    ];

    for (cs_type, rust_type) in type_map {
        assert_eq!(Some(rust_type), cs_type.to_rust());
        assert_eq!(Ok(cs_type.clone()), Type::try_from(cs_type));
    }
}

#[test]
fn dart_format() {
    assert_eq!("void", Type::Void.to_string());
    assert_eq!(
        "Object?",
        Type::Nullable(Box::new(Type::Object)).to_string()
    );
    assert_eq!(
        "List<BigInt>",
        Type::List(Box::new(Type::BigInt)).to_string()
    );
    assert_eq!(
        "Map<String, double>",
        Type::Map(Box::new(Type::String), Box::new(Type::Double)).to_string()
    );
}

#[test]
fn dart_parse() {
    assert_eq!(Ok(Type::Void), "void".parse());
    assert_eq!(
        Ok(Type::Nullable(Box::new(Type::Object))),
        "Object?".parse()
    );
    assert_eq!(
        Ok(Type::Nullable(Box::new(Type::Object))),
        "Object   ?".parse()
    );
    assert_eq!(
        Ok(Type::List(Box::new(Type::BigInt))),
        "List<BigInt>".parse()
    );
    assert_eq!(
        Ok(Type::List(Box::new(Type::BigInt))),
        "List<   BigInt >".parse()
    );
    assert_eq!(
        Ok(Type::Map(Box::new(Type::String), Box::new(Type::Double))),
        "Map<String,double>".parse()
    );
    assert_eq!(
        Ok(Type::Map(Box::new(Type::String), Box::new(Type::Double))),
        "Map<String, double>".parse()
    );
    assert_eq!(
        Ok(Type::Map(Box::new(Type::String), Box::new(Type::Double))),
        "  Map  <  String, double  >".parse()
    );
    assert_eq!(Err(()), "".parse::<Type>());
    assert_eq!(Err(()), "?".parse::<Type>());
    assert_eq!(Err(()), "bogus?".parse::<Type>());
    assert_eq!(Err(()), "List".parse::<Type>());
    assert_eq!(Err(()), "List?".parse::<Type>());
    assert_eq!(Err(()), "Map".parse::<Type>());
    assert_eq!(Err(()), "Map<".parse::<Type>());
    assert_eq!(Err(()), "Map<String".parse::<Type>());
    assert_eq!(Err(()), "Map<String,".parse::<Type>());
    assert_eq!(Err(()), "Map<String,bogus".parse::<Type>());
    assert_eq!(Err(()), "Map<String,double".parse::<Type>());
}

#[test]
fn dart_malformated() {
    assert_eq!(Err(()), "".parse::<Type>());
    assert_eq!(Err(()), "?".parse::<Type>());
    assert_eq!(Err(()), "bogus?".parse::<Type>());
    assert_eq!(Err(()), "List?".parse::<Type>());
    assert_eq!(Err(()), "List<".parse::<Type>());
    assert_eq!(Err(()), "List><".parse::<Type>());
    assert_eq!(Err(()), "List<,".parse::<Type>());
    assert_eq!(Err(()), "List   <   ,  ".parse::<Type>());
    assert_eq!(Err(()), "Map?".parse::<Type>());
    assert_eq!(Err(()), "Map<?".parse::<Type>());
    assert_eq!(Err(()), "Map<?,".parse::<Type>());
    assert_eq!(Err(()), "Map<?,bogus".parse::<Type>());
    assert_eq!(Err(()), "Map<?,double".parse::<Type>());
    assert_eq!(Err(()), "Map<?,double>".parse::<Type>());
    assert_eq!(Err(()), "Map<String?".parse::<Type>());
    assert_eq!(Err(()), "Map<String?,".parse::<Type>());
    assert_eq!(Err(()), "Map<String?,bogus".parse::<Type>());
    assert_eq!(Err(()), "Map<String?,double".parse::<Type>());
    assert_eq!(Err(()), "Map<String,bogus".parse::<Type>());
    assert_eq!(Err(()), "Map<String,double?,".parse::<Type>());
    assert_eq!(Err(()), "Map<String,double,bogus".parse::<Type>());
    assert_eq!(Err(()), "Map<String,double,bogus>".parse::<Type>());
    assert_eq!(Err(()), "Map<String,double,double".parse::<Type>());
    assert_eq!(Err(()), "Map<String,double,double>".parse::<Type>());
    assert_eq!(Err(()), "Map<String,bogus,?".parse::<Type>());
}
