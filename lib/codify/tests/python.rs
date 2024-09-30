use codify::python::Type;
use codify::{rust, ToRust};

#[test]
fn python_to_rust_and_back() {
    let type_map: [(Type, rust::Type); 7] = [
        (Type::NoneType, rust::Type::Unit),
        (Type::Bool, rust::Type::Bool),
        (Type::Int, rust::Type::I64),
        (Type::Float, rust::Type::F64),
        (Type::Str, rust::Type::String),
        (
            Type::List(Box::new(Type::Bool)),
            rust::Type::Vec(Box::new(rust::Type::Bool)),
        ),
        (
            Type::Dict(Box::new(Type::Bool), Box::new(Type::Int)),
            rust::Type::Map(Box::new(rust::Type::Bool), Box::new(rust::Type::I64)),
        ),
    ];

    for (py_type, rust_type) in type_map {
        assert_eq!(Some(rust_type), py_type.to_rust());
        assert_eq!(Ok(py_type.clone()), codify::python::Type::try_from(py_type));
    }
}

#[test]
fn python_format() {
    assert_eq!("NoneType", Type::NoneType.to_string());
    assert_eq!("bool", Type::Bool.to_string());
    assert_eq!("int", Type::Int.to_string());
    assert_eq!("float", Type::Float.to_string());
    assert_eq!("complex", Type::Complex.to_string());
    assert_eq!("str", Type::Str.to_string());
    assert_eq!("list[bool]", Type::List(Box::new(Type::Bool)).to_string());
    assert_eq!(
        "dict[bool, int]",
        Type::Dict(Box::new(Type::Bool), Box::new(Type::Int)).to_string()
    );
    assert_eq!(
        "POINTER(c_uint)",
        Type::FFI(codify::c::Type::PtrMut(Box::new(codify::c::Type::UInt))).to_string()
    );
    assert_eq!(
        "c_uint * 15",
        Type::FFI(codify::c::Type::Array(
            Box::new(codify::c::Type::UInt),
            Some(15)
        ))
        .to_string()
    );
}

#[test]
fn python_parse() {
    assert_eq!(Ok(Type::NoneType), "NoneType".parse());
    assert_eq!(Ok(Type::Bool), "bool".parse());
    assert_eq!(Ok(Type::Int), "int".parse());
    assert_eq!(Ok(Type::Float), "float".parse());
    assert_eq!(Ok(Type::Complex), "complex".parse());
    assert_eq!(Ok(Type::Str), "str".parse());
    assert_eq!(Ok(Type::List(Box::new(Type::Bool))), "list[bool]".parse());
    assert_eq!(
        Ok(Type::List(Box::new(Type::Bool))),
        "list  [    bool  ]".parse()
    );
    assert_eq!(
        Ok(Type::Dict(Box::new(Type::Bool), Box::new(Type::Int))),
        "dict   [   bool , int ]".parse()
    );
    assert_eq!(
        Ok(Type::FFI(codify::c::Type::PtrMut(Box::new(
            codify::c::Type::Size_t
        )))),
        "POINTER(c_size_t)".parse()
    );
    assert_eq!(
        Ok(Type::FFI(codify::c::Type::PtrMut(Box::new(
            codify::c::Type::Int
        )))),
        "POINTER  (  c_int  )".parse()
    );
    assert_eq!(
        Ok(Type::FFI(codify::c::Type::Array(
            Box::new(codify::c::Type::UInt),
            Some(15)
        ))),
        "c_uint*15".parse()
    );
    assert_eq!(
        Ok(Type::FFI(codify::c::Type::Array(
            Box::new(codify::c::Type::UInt),
            Some(15)
        ))),
        "c_uint * 15".parse()
    );
}

#[test]
fn python_malformated() {
    assert_eq!(Err(()), "".parse::<Type>());
    assert_eq!(Err(()), "[]".parse::<Type>());
    assert_eq!(Err(()), "bogus[]".parse::<Type>());
    assert_eq!(Err(()), "[bool".parse::<Type>());
    assert_eq!(Err(()), "[bool]".parse::<Type>());
    assert_eq!(Err(()), "[bool, int".parse::<Type>());
    assert_eq!(Err(()), "[bool, int]".parse::<Type>());
    assert_eq!(Err(()), "list".parse::<Type>());
    assert_eq!(Err(()), "list[".parse::<Type>());
    assert_eq!(Err(()), "list[bool".parse::<Type>());
    assert_eq!(Err(()), "list[bool, int".parse::<Type>());
    assert_eq!(Err(()), "list[bool, int]".parse::<Type>());
    assert_eq!(Err(()), "dict[bool".parse::<Type>());
    assert_eq!(Err(()), "dict[bool]".parse::<Type>());
    assert_eq!(Err(()), "dict[bool, int".parse::<Type>());
    assert_eq!(Err(()), "dict[bool, ]".parse::<Type>());
    assert_eq!(Err(()), "dict[bool, ,]".parse::<Type>());
    assert_eq!(Err(()), "POINTER".parse::<Type>());
    assert_eq!(Err(()), "POINTER(".parse::<Type>());
    assert_eq!(Err(()), "POINTER(x".parse::<Type>());
    assert_eq!(Err(()), "POINTER(bogus".parse::<Type>());
    assert_eq!(Err(()), "POINTER(c_size_t".parse::<Type>());
}
