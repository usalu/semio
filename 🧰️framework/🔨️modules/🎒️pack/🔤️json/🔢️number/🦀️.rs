//! 🔢️ JSON number: the integer and float number carrier with its comparisons and numeric conversions. See [`super`].

//#region 🔖️Number
/// 👁️ A JSON number, keeping the writer's-eye distinction JSON itself does not: an integer
/// literal (`UInt`/`Int`) round-trips without a decimal point, a `Float` always carries one (or an
/// exponent) so `42` and `42.0` are never confused on the wire.
#[derive(Clone, Copy, Debug)]
pub enum Number {
    UInt(u64),
    Int(i64),
    Float(f64),
}

impl Number {
    /// 🔎️ Widens to `f64` regardless of variant — lossy for `u64`/`i64` magnitudes beyond 2^53.
    pub fn as_f64(&self) -> f64 {
        match *self {
            Number::UInt(v) => v as f64,
            Number::Int(v) => v as f64,
            Number::Float(v) => v,
        }
    }

    /// 🎯️ Exact `i64`, only for the `Int` variant and `UInt` values that fit.
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Number::Int(v) => Some(v),
            Number::UInt(v) => i64::try_from(v).ok(),
            Number::Float(_) => None,
        }
    }

    /// ❎️ Exact `u64`, only for the `UInt` variant and non-negative `Int` values.
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Number::UInt(v) => Some(v),
            Number::Int(v) => u64::try_from(v).ok(),
            Number::Float(_) => None,
        }
    }

    /// 📍️ Whether this literal was written without a decimal point or exponent.
    pub fn is_integer(&self) -> bool {
        !matches!(self, Number::Float(_))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Number::UInt(a), Number::UInt(b)) => a == b,
            (Number::Int(a), Number::Int(b)) => a == b,
            (Number::Float(a), Number::Float(b)) => a == b,
            (Number::UInt(a), Number::Int(b)) | (Number::Int(b), Number::UInt(a)) => i64::try_from(a).is_ok_and(|a| a == b),
            _ => false,
        }
    }
}

impl From<u64> for Number {
    fn from(v: u64) -> Self {
        Number::UInt(v)
    }
}

impl From<i64> for Number {
    fn from(v: i64) -> Self {
        Number::Int(v)
    }
}

impl From<f64> for Number {
    fn from(v: f64) -> Self {
        Number::Float(v)
    }
}

impl From<u32> for Number {
    fn from(v: u32) -> Self {
        Number::UInt(v as u64)
    }
}

impl From<i32> for Number {
    fn from(v: i32) -> Self {
        Number::Int(v as i64)
    }
}

impl From<usize> for Number {
    fn from(v: usize) -> Self {
        Number::UInt(v as u64)
    }
}

impl From<i8> for Number {
    fn from(v: i8) -> Self {
        Number::Int(v as i64)
    }
}
//#endregion 🔖️Number
