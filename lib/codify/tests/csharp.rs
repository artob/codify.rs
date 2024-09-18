use codify::csharp::Type;
use codify::{rust, ToRust};

#[test]
fn csharp_to_rust_and_back() {
    let type_map: [(Type, rust::Type); 10] = [
        (Type::Void, rust::Type::Unit),
        (Type::Object, rust::Type::Any),
        (Type::Bool, rust::Type::Bool),
        (Type::Float, rust::Type::F32),
        (Type::Double, rust::Type::F64),
        (Type::NInt, rust::Type::Isize),
        (Type::NUInt, rust::Type::Usize),
        (Type::String, rust::Type::String),
        (
            Type::List(Box::new(Type::UShort)),
            rust::Type::Vec(Box::new(rust::Type::U16)),
        ),
        (
            Type::Dictionary(Box::new(Type::Long), Box::new(Type::String)),
            rust::Type::Map(Box::new(rust::Type::I64), Box::new(rust::Type::String)),
        ),
    ];

    for (cs_type, rust_type) in type_map {
        assert_eq!(Some(rust_type), cs_type.to_rust());
        assert_eq!(Ok(cs_type.clone()), codify::csharp::Type::try_from(cs_type));
    }
}

#[test]
fn csharp_format() {
    assert_eq!("void", Type::Void.to_string());
    assert_eq!("int?", Type::Nullable(Box::new(Type::Int)).to_string());
    assert_eq!("float[]", Type::Array(Box::new(Type::Float)).to_string());
    assert_eq!("ref char", Type::RefMut(Box::new(Type::Char)).to_string());
    assert_eq!("out string", Type::Out(Box::new(Type::String)).to_string());
    assert_eq!("const byte*", Type::Ptr(Box::new(Type::Byte)).to_string());
    assert_eq!("byte*", Type::PtrMut(Box::new(Type::Byte)).to_string());
    assert_eq!(
        "System.Collections.Generic.List<ushort>",
        Type::List(Box::new(Type::UShort)).to_string()
    );
    assert_eq!(
        "System.Collections.Generic.Dictionary<long, string>",
        Type::Dictionary(Box::new(Type::Long), Box::new(Type::String)).to_string()
    )
}

#[test]
fn csharp_parse() {
    assert_eq!(Ok(Type::Void), "void".parse());
    assert_eq!(Ok(Type::Nullable(Box::new(Type::Int))), "int?".parse());
    assert_eq!(Ok(Type::Nullable(Box::new(Type::Int))), "int   ?".parse());
    assert_eq!(Ok(Type::Array(Box::new(Type::Float))), "float[]".parse());
    assert_eq!(Ok(Type::Array(Box::new(Type::Float))), "float   []".parse());
    assert_eq!(Ok(Type::RefMut(Box::new(Type::Char))), "ref char".parse());
    assert_eq!(Ok(Type::RefMut(Box::new(Type::Char))), "ref   char".parse());
    assert_eq!(Ok(Type::Out(Box::new(Type::String))), "out string".parse());
    assert_eq!(Ok(Type::Out(Box::new(Type::String))), "out  string".parse());
    assert_eq!(Ok(Type::Ptr(Box::new(Type::Byte))), "const byte*".parse());
    assert_eq!(Ok(Type::Ptr(Box::new(Type::Byte))), "const   byte*".parse());
    assert_eq!(Ok(Type::Ptr(Box::new(Type::Byte))), "const  byte *".parse());
    assert_eq!(Ok(Type::PtrMut(Box::new(Type::Byte))), "byte*".parse());
    assert_eq!(Ok(Type::PtrMut(Box::new(Type::Byte))), "byte *".parse());
    assert_eq!(
        Ok(Type::List(Box::new(Type::UShort))),
        "System.Collections.Generic.List<ushort>".parse()
    );
    assert_eq!(
        Ok(Type::List(Box::new(Type::UShort))),
        "System.Collections.Generic.List<   ushort >".parse()
    );
    assert_eq!(
        Ok(Type::Dictionary(
            Box::new(Type::Long),
            Box::new(Type::String)
        )),
        "System.Collections.Generic.Dictionary<long,string>".parse()
    );
    assert_eq!(
        Ok(Type::Dictionary(
            Box::new(Type::Long),
            Box::new(Type::String)
        )),
        "System.Collections.Generic.Dictionary<long, string>".parse()
    );
    assert_eq!(
        Ok(Type::Dictionary(
            Box::new(Type::Long),
            Box::new(Type::String)
        )),
        "System.Collections.Generic.Dictionary<   long  , string  >".parse()
    );
}

#[test]
fn csharp_malformated() {
    assert_eq!(Err(()), "".parse::<Type>());
    assert_eq!(Err(()), "?".parse::<Type>());
    assert_eq!(Err(()), "bogus?".parse::<Type>());
    assert_eq!(Err(()), "]".parse::<Type>());
    assert_eq!(Err(()), "b]".parse::<Type>());
    assert_eq!(Err(()), "bogus]".parse::<Type>());
    assert_eq!(Err(()), "[]".parse::<Type>());
    assert_eq!(Err(()), "ref".parse::<Type>());
    assert_eq!(Err(()), "ref ".parse::<Type>());
    assert_eq!(Err(()), "out".parse::<Type>());
    assert_eq!(Err(()), "out ".parse::<Type>());
    assert_eq!(Err(()), "const".parse::<Type>());
    assert_eq!(Err(()), "const ".parse::<Type>());
    assert_eq!(Err(()), "const byte".parse::<Type>());
    assert_eq!(Err(()), "const *".parse::<Type>());
    assert_eq!(Err(()), "const b*".parse::<Type>());
    assert_eq!(Err(()), "const bogus*".parse::<Type>());
    assert_eq!(Err(()), "*".parse::<Type>());
    assert_eq!(Err(()), "b*".parse::<Type>());
    assert_eq!(Err(()), "bogus*".parse::<Type>());
    assert_eq!(Err(()), "System.Collections.Generic".parse::<Type>());
    assert_eq!(Err(()), "System.Collections.Generic.".parse::<Type>());
    assert_eq!(Err(()), "System.Collections.Generic.List".parse::<Type>());
    assert_eq!(Err(()), "System.Collections.Generic.List<".parse::<Type>());
    assert_eq!(
        Err(()),
        "System.Collections.Generic.List<bogus".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.List<uint".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.List<    bogus".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.List<    uint".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<long".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<   long".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<,".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<,>".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<,long".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<,long>".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<long,string".parse::<Type>()
    );
    assert_eq!(
        Err(()),
        "System.Collections.Generic.Dictionary<long   ,string".parse::<Type>()
    );
}
