use num::bigint::ToBigInt;
use num::ToPrimitive;
use crate::objects::value::typecast::typecast;
use crate::objects::value::utils::OprError;
use crate::objects::value::Value;
use crate::Type;

macro_rules! typecast_mul {
    ($e:ident, $t:ident, $s:literal, $x:ident, $y:ident) => {
        Ok(Value::$e($x * typecast(&$y, Value::Type(Type::from_str($s)))?.$t().unwrap()))
    };
}

fn mul_str(x: String, y: Value) -> Result<Value, OprError> { // TODO check usize::MAX to see if need to split, refactor too
    match y {
        Value::I8(y) => if y < 0 {Err(OprError::TypecastError(Type::from_str("str")))}
        else {Ok(Value::Str(x.repeat(y as usize)))},
        Value::I16(y) => if y < 0 {Err(OprError::TypecastError(Type::from_str("str")))}
        else {Ok(Value::Str(x.repeat(y as usize)))},
        Value::I32(y) => if y < 0 {Err(OprError::TypecastError(Type::from_str("str")))}
        else {Ok(Value::Str(x.repeat(y as usize)))},
        Value::I64(y) => if y < 0 {Err(OprError::TypecastError(Type::from_str("str")))}
        else {Ok(Value::Str(x.repeat(y as usize)))},
        Value::I128(y) => if y < 0 {Err(OprError::TypecastError(Type::from_str("str")))}
        else {Ok(Value::Str(x.repeat(y as usize)))},
        Value::Isize(y) => if y < 0 {Err(OprError::TypecastError(Type::from_str("str")))}
        else {Ok(Value::Str(x.repeat(y as usize)))},
        Value::Ibig(y) => if y < 0.to_bigint().unwrap() {Err(OprError::TypecastError(Type::from_str("str")))}
            else {Ok(Value::Str(x.repeat(y.to_usize().unwrap())))},
        Value::U8(y) => Ok(Value::Str(x.repeat(y as usize))),
        Value::U16(y) => Ok(Value::Str(x.repeat(y as usize))),
        Value::U32(y) => Ok(Value::Str(x.repeat(y as usize))),
        Value::U64(y) => Ok(Value::Str(x.repeat(y as usize))),
        Value::U128(y) => Ok(Value::Str(x.repeat(y as usize))),
        Value::Usize(y) => Ok(Value::Str(x.repeat(y as usize))),
        Value::Ubig(y) => Ok(Value::Str(x.repeat(y.to_usize().unwrap()))),
        _ => Err(OprError::NoImplForOpr),
    }
}

pub fn mul(x: &Value, y: Value) -> Result<Value, OprError> {
    if let Value::Str(x) = x {
        return mul_str(x.clone(), y);
    } else if let Value::Str(y) = y {
        return mul_str(y, x.clone());
    }
    match x {
        Value::I8(x) => typecast_mul!(I8, as_i8, "i8", x, y),
        Value::I16(x) => typecast_mul!(I16, as_i16, "i16", x, y),
        Value::I32(x) => typecast_mul!(I32, as_i32, "i32", x, y),
        Value::I64(x) => typecast_mul!(I64, as_i64, "i64", x, y),
        Value::I128(x) => typecast_mul!(I128, as_i128, "i128", x, y),
        Value::Isize(x) => typecast_mul!(Isize, as_isize, "isize", x, y),
        Value::Ibig(x) => typecast_mul!(Ibig, as_ibig, "ibig", x, y),
        Value::U8(x) => typecast_mul!(U8, as_u8, "u8", x, y),
        Value::U16(x) => typecast_mul!(U16, as_u16, "u16", x, y),
        Value::U32(x) => typecast_mul!(U32, as_u32, "u32", x, y),
        Value::U64(x) => typecast_mul!(U64, as_u64, "u64", x, y),
        Value::U128(x) => typecast_mul!(U128, as_u128, "u128", x, y),
        Value::Usize(x) => typecast_mul!(Usize, as_usize, "usize", x, y),
        Value::Ubig(x) => typecast_mul!(Ubig, as_ubig, "ubig", x, y),
        Value::F16(x) => typecast_mul!(F16, as_f16, "f16", x, y),
        Value::F32(x) => typecast_mul!(F32, as_f32, "f32", x, y),
        Value::F64(x) => typecast_mul!(F64, as_f64, "f64", x, y),
        _ => Err(OprError::NoImplForOpr)
    }
}