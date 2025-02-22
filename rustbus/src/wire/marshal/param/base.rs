//! Marshal base params into raw bytes

use crate::params;
use crate::wire::errors::MarshalError;
use crate::wire::marshal::MarshalContext;
use crate::wire::util::*;
use crate::ByteOrder;

fn marshal_boolean(b: bool, byteorder: ByteOrder, buf: &mut Vec<u8>) {
    if b {
        write_u32(1, byteorder, buf);
    } else {
        write_u32(0, byteorder, buf);
    }
}

fn marshal_byte(i: u8, buf: &mut Vec<u8>) {
    buf.push(i);
}

fn marshal_i16(i: i16, byteorder: ByteOrder, buf: &mut Vec<u8>) {
    write_u16(i as u16, byteorder, buf);
}

fn marshal_u16(i: u16, byteorder: ByteOrder, buf: &mut Vec<u8>) {
    write_u16(i, byteorder, buf);
}
fn marshal_i32(i: i32, byteorder: ByteOrder, buf: &mut Vec<u8>) {
    write_u32(i as u32, byteorder, buf);
}

fn marshal_u32(i: u32, byteorder: ByteOrder, buf: &mut Vec<u8>) {
    write_u32(i, byteorder, buf);
}
fn marshal_i64(i: i64, byteorder: ByteOrder, buf: &mut Vec<u8>) {
    write_u64(i as u64, byteorder, buf);
}

fn marshal_u64(i: u64, byteorder: ByteOrder, buf: &mut Vec<u8>) {
    write_u64(i, byteorder, buf);
}

fn marshal_string(s: &str, byteorder: ByteOrder, buf: &mut Vec<u8>) -> Result<(), MarshalError> {
    if s.contains('\0') {
        Err(params::validation::Error::StringContainsNullByte.into())
    } else {
        write_string(s, byteorder, buf);
        Ok(())
    }
}
fn marshal_objectpath(
    s: &str,
    byteorder: ByteOrder,
    buf: &mut Vec<u8>,
) -> Result<(), MarshalError> {
    params::validate_object_path(s)?;
    write_string(s, byteorder, buf);
    Ok(())
}
pub(super) fn marshal_signature(s: &str, buf: &mut Vec<u8>) -> Result<(), MarshalError> {
    params::validate_signature(s)?;
    write_signature(s, buf);
    Ok(())
}

pub fn marshal_base_param(p: &params::Base, ctx: &mut MarshalContext) -> Result<(), MarshalError> {
    pad_to_align(p.sig().get_alignment(), ctx.buf);

    match p {
        params::Base::Boolean(b) => marshal_boolean(*b, ctx.byteorder, ctx.buf),
        params::Base::BooleanRef(b) => marshal_boolean(**b, ctx.byteorder, ctx.buf),
        params::Base::Byte(i) => marshal_byte(*i, ctx.buf),
        params::Base::ByteRef(i) => marshal_byte(**i, ctx.buf),
        params::Base::Int16(i) => marshal_i16(*i, ctx.byteorder, ctx.buf),
        params::Base::Int16Ref(i) => marshal_i16(**i, ctx.byteorder, ctx.buf),
        params::Base::Uint16(i) => marshal_u16(*i, ctx.byteorder, ctx.buf),
        params::Base::Uint16Ref(i) => marshal_u16(**i, ctx.byteorder, ctx.buf),
        params::Base::Int32(i) => marshal_i32(*i, ctx.byteorder, ctx.buf),
        params::Base::Int32Ref(i) => marshal_i32(**i, ctx.byteorder, ctx.buf),
        params::Base::Uint32(i) => marshal_u32(*i, ctx.byteorder, ctx.buf),
        params::Base::Uint32Ref(i) => marshal_u32(**i, ctx.byteorder, ctx.buf),
        params::Base::Int64(i) => marshal_i64(*i, ctx.byteorder, ctx.buf),
        params::Base::Int64Ref(i) => marshal_i64(**i, ctx.byteorder, ctx.buf),
        params::Base::Uint64(i) => marshal_u64(*i, ctx.byteorder, ctx.buf),
        params::Base::Uint64Ref(i) => marshal_u64(**i, ctx.byteorder, ctx.buf),
        params::Base::Double(i) => marshal_u64(*i, ctx.byteorder, ctx.buf),
        params::Base::DoubleRef(i) => marshal_u64(**i, ctx.byteorder, ctx.buf),
        params::Base::StringRef(s) => marshal_string(s, ctx.byteorder, ctx.buf)?,
        params::Base::String(s) => marshal_string(s, ctx.byteorder, ctx.buf)?,
        params::Base::Signature(s) => marshal_signature(s, ctx.buf)?,
        params::Base::SignatureRef(s) => marshal_signature(s, ctx.buf)?,
        params::Base::ObjectPath(s) => marshal_objectpath(s, ctx.byteorder, ctx.buf)?,
        params::Base::ObjectPathRef(s) => marshal_objectpath(s, ctx.byteorder, ctx.buf)?,
        params::Base::UnixFd(i) => marshal_unixfd(i, ctx)?,
        params::Base::UnixFdRef(i) => marshal_unixfd(i, ctx)?,
    }
    Ok(())
}
