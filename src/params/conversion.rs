use super::*;
use crate::message::Error;
use crate::signature;

//
//
// Base TO
//
//

impl<'a> std::convert::From<&Base<'a>> for signature::Base {
    fn from(b: &Base) -> crate::signature::Base {
        match b {
            Base::Boolean(_) => signature::Base::Boolean,
            Base::Byte(_) => signature::Base::Byte,
            Base::Double(_) => signature::Base::Double,
            Base::Int16(_) => signature::Base::Int16,
            Base::Int32(_) => signature::Base::Int32,
            Base::Int64(_) => signature::Base::Int64,
            Base::Uint16(_) => signature::Base::Uint16,
            Base::Uint32(_) => signature::Base::Uint32,
            Base::Uint64(_) => signature::Base::Uint64,
            Base::ObjectPath(_) => signature::Base::ObjectPath,
            Base::Signature(_) => signature::Base::Signature,
            Base::String(_) => signature::Base::String,
            Base::UnixFd(_) => signature::Base::UnixFd,
            Base::BooleanRef(_) => signature::Base::Boolean,
            Base::ByteRef(_) => signature::Base::Byte,
            Base::DoubleRef(_) => signature::Base::Double,
            Base::Int16Ref(_) => signature::Base::Int16,
            Base::Int32Ref(_) => signature::Base::Int32,
            Base::Int64Ref(_) => signature::Base::Int64,
            Base::Uint16Ref(_) => signature::Base::Uint16,
            Base::Uint32Ref(_) => signature::Base::Uint32,
            Base::Uint64Ref(_) => signature::Base::Uint64,
            Base::ObjectPathRef(_) => signature::Base::ObjectPath,
            Base::SignatureRef(_) => signature::Base::Signature,
            Base::StringRef(_) => signature::Base::String,
            Base::UnixFdRef(_) => signature::Base::UnixFd,
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for bool {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<bool, Error> {
        if let Base::Boolean(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for String {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<String, Error> {
        if let Base::String(value) = b {
            Ok(value.clone())
        } else {
            Err(Error::InvalidType)
        }
    }
}
impl<'a> std::convert::TryFrom<&Base<'a>> for &'a str {
    type Error = Error;
    fn try_from(b: &Base<'a>) -> std::result::Result<&'a str, Error> {
        if let Base::StringRef(value) = b {
            Ok(value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for u8 {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<u8, Error> {
        if let Base::Byte(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for u16 {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<u16, Error> {
        if let Base::Uint16(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for u32 {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<u32, Error> {
        if let Base::Uint32(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for u64 {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<u64, Error> {
        if let Base::Uint64(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for i16 {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<i16, Error> {
        if let Base::Int16(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for i32 {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<i32, Error> {
        if let Base::Int32(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}

impl<'a> std::convert::TryFrom<&Base<'a>> for i64 {
    type Error = Error;
    fn try_from(b: &Base) -> std::result::Result<i64, Error> {
        if let Base::Int64(value) = b {
            Ok(*value)
        } else {
            Err(Error::InvalidType)
        }
    }
}



//
//
// Param TO
//
//

impl<'a, 'e> std::convert::From<&Param<'a, 'e>> for signature::Type {
    fn from(p: &Param<'a, 'e>) -> crate::signature::Type {
        match p {
            Param::Base(b) => signature::Type::Base(b.into()),
            Param::Container(c) => signature::Type::Container(c.into()),
        }
    }
}

//
//
// Param FROM
//
//

impl<'a, 'e> std::convert::From<Base<'a>> for Param<'a, 'e> {
    fn from(s: Base<'a>) -> Self {
        Param::Base(s)
    }
}
impl<'a, 'e> std::convert::From<Container<'a, 'e>> for Param<'a, 'e> {
    fn from(s: Container<'a, 'e>) -> Self {
        Param::Container(s)
    }
}

impl<'a, 'e> std::convert::From<bool> for Param<'a, 'e> {
    fn from(s: bool) -> Self {
        Param::Base(Base::Boolean(s))
    }
}
impl<'a, 'e> std::convert::From<String> for Param<'a, 'e> {
    fn from(s: String) -> Self {
        Param::Base(Base::String(s))
    }
}
impl<'a, 'e> std::convert::From<u8> for Param<'a, 'e> {
    fn from(s: u8) -> Self {
        Param::Base(Base::Byte(s))
    }
}
impl<'a, 'e> std::convert::From<u16> for Param<'a, 'e> {
    fn from(s: u16) -> Self {
        Param::Base(Base::Uint16(s))
    }
}
impl<'a, 'e> std::convert::From<u32> for Param<'a, 'e> {
    fn from(s: u32) -> Self {
        Param::Base(Base::Uint32(s))
    }
}
impl<'a, 'e> std::convert::From<u64> for Param<'a, 'e> {
    fn from(s: u64) -> Self {
        Param::Base(Base::Uint64(s))
    }
}
impl<'a, 'e> std::convert::From<i16> for Param<'a, 'e> {
    fn from(s: i16) -> Self {
        Param::Base(Base::Int16(s))
    }
}
impl<'a, 'e> std::convert::From<i32> for Param<'a, 'e> {
    fn from(s: i32) -> Self {
        Param::Base(Base::Int32(s))
    }
}
impl<'a, 'e> std::convert::From<i64> for Param<'a, 'e> {
    fn from(s: i64) -> Self {
        Param::Base(Base::Int64(s))
    }
}

//
//
// Container FROM
//
//

impl<'a, 'e> std::convert::TryFrom<(signature::Type, Vec<Param<'a, 'e>>)> for Container<'a, 'e> {
    type Error = Error;
    fn try_from(
        parts: (signature::Type, Vec<Param<'a, 'e>>),
    ) -> std::result::Result<Container<'a, 'e>, Error> {
        let arr = Array {
            element_sig: parts.0,
            values: parts.1,
        };
        validate_array(&arr)?;
        Ok(Container::Array(arr))
    }
}
impl<'a, 'e> std::convert::TryFrom<Vec<Param<'a, 'e>>> for Container<'a, 'e> {
    type Error = Error;
    fn try_from(elems: Vec<Param<'a, 'e>>) -> std::result::Result<Container<'a, 'e>, Error> {
        if elems.is_empty() {
            return Err(Error::EmptyArray);
        }
        Container::try_from((elems[0].sig(), elems))
    }
}

impl<'a, 'e> std::convert::TryFrom<(signature::Base, signature::Type, DictMap<'a, 'e>)>
    for Container<'a, 'e>
{
    type Error = Error;
    fn try_from(
        parts: (signature::Base, signature::Type, DictMap<'a, 'e>),
    ) -> std::result::Result<Container<'a, 'e>, Error> {
        let dict = Dict {
            key_sig: parts.0,
            value_sig: parts.1,
            map: parts.2,
        };
        validate_dict(&dict)?;
        Ok(Container::Dict(dict))
    }
}
impl<'a, 'e> std::convert::TryFrom<DictMap<'a, 'e>> for Container<'a, 'e> {
    type Error = Error;
    fn try_from(elems: DictMap<'a, 'e>) -> std::result::Result<Container<'a, 'e>, Error> {
        if elems.is_empty() {
            return Err(Error::EmptyDict);
        }
        let key_sig = elems.keys().nth(0).unwrap().sig();
        let value_sig = elems.values().nth(0).unwrap().sig();

        if let signature::Type::Base(key_sig) = key_sig {
            Container::try_from((key_sig, value_sig, elems))
        } else {
            Err(Error::InvalidSignatureShouldBeBase)
        }
    }
}

//
//
// Base FROM
//
//

impl<'a> std::convert::From<bool> for Base<'a> {
    fn from(s: bool) -> Self {
        Base::Boolean(s)
    }
}
impl<'a> std::convert::From<String> for Base<'a> {
    fn from(s: String) -> Self {
        Base::String(s)
    }
}
impl<'a> std::convert::From<&'a str> for Base<'a> {
    fn from(s: &'a str) -> Self {
        Base::StringRef(s)
    }
}
impl<'a> std::convert::From<u8> for Base<'a> {
    fn from(s: u8) -> Self {
        Base::Byte(s)
    }
}
impl<'a> std::convert::From<u16> for Base<'a> {
    fn from(s: u16) -> Self {
        Base::Uint16(s)
    }
}
impl<'a> std::convert::From<u32> for Base<'a> {
    fn from(s: u32) -> Self {
        Base::Uint32(s)
    }
}
impl<'a> std::convert::From<u64> for Base<'a> {
    fn from(s: u64) -> Self {
        Base::Uint64(s)
    }
}
impl<'a> std::convert::From<i16> for Base<'a> {
    fn from(s: i16) -> Self {
        Base::Int16(s)
    }
}
impl<'a> std::convert::From<i32> for Base<'a> {
    fn from(s: i32) -> Self {
        Base::Int32(s)
    }
}
impl<'a> std::convert::From<i64> for Base<'a> {
    fn from(s: i64) -> Self {
        Base::Int64(s)
    }
}

//
//
// Container TO
//
//

impl<'a, 'e> std::convert::From<&Container<'a, 'e>> for signature::Container {
    fn from(c: &Container<'a, 'e>) -> crate::signature::Container {
        match c {
            Container::Array(arr) => signature::Container::Array(Box::new(arr.element_sig.clone())),
            Container::Dict(dict) => {
                signature::Container::Dict(dict.key_sig, Box::new(dict.value_sig.clone()))
            }
            Container::Struct(params) => {
                signature::Container::Struct(params.iter().map(|param| param.into()).collect())
            }
            Container::Variant(_) => signature::Container::Variant,
            Container::ArrayRef(arr) => {
                signature::Container::Array(Box::new(arr.element_sig.clone()))
            }
            Container::DictRef(dict) => {
                signature::Container::Dict(dict.key_sig, Box::new(dict.value_sig.clone()))
            }
            Container::StructRef(params) => {
                signature::Container::Struct(params.iter().map(|param| param.into()).collect())
            }
            Container::VariantRef(_) => signature::Container::Variant,
        }
    }
}