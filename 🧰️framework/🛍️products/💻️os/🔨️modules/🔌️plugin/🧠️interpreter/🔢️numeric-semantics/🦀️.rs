//! 🔢️ Numeric semantics: the canonical nans, the ties-even rounding, the wasm minimum and maximum and the truncating and saturating conversions. See [`super`].

use super::*;

//#region 🔢️NumericSemantics
pub(super) fn canonical_f32(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(0x7fc0_0000)
    } else {
        value
    }
}

pub(super) fn canonical_f64(value: f64) -> f64 {
    if value.is_nan() {
        f64::from_bits(0x7ff8_0000_0000_0000)
    } else {
        value
    }
}

pub(super) fn round_ties_even_f32(value: f32) -> f32 {
    value.round_ties_even()
}

pub(super) fn round_ties_even_f64(value: f64) -> f64 {
    value.round_ties_even()
}

pub(super) fn wasm_min_f32(left: f32, right: f32) -> f32 {
    if left.is_nan() || right.is_nan() {
        return f32::NAN;
    }
    if left == right {
        if left == 0.0 && (left.is_sign_negative() || right.is_sign_negative()) {
            -0.0
        } else {
            left
        }
    } else if left < right {
        left
    } else {
        right
    }
}

pub(super) fn wasm_max_f32(left: f32, right: f32) -> f32 {
    if left.is_nan() || right.is_nan() {
        return f32::NAN;
    }
    if left == right {
        if left == 0.0 && (!left.is_sign_negative() || !right.is_sign_negative()) {
            0.0
        } else {
            left
        }
    } else if left > right {
        left
    } else {
        right
    }
}

pub(super) fn wasm_min_f64(left: f64, right: f64) -> f64 {
    if left.is_nan() || right.is_nan() {
        return f64::NAN;
    }
    if left == right {
        if left == 0.0 && (left.is_sign_negative() || right.is_sign_negative()) {
            -0.0
        } else {
            left
        }
    } else if left < right {
        left
    } else {
        right
    }
}

pub(super) fn wasm_max_f64(left: f64, right: f64) -> f64 {
    if left.is_nan() || right.is_nan() {
        return f64::NAN;
    }
    if left == right {
        if left == 0.0 && (!left.is_sign_negative() || !right.is_sign_negative()) {
            0.0
        } else {
            left
        }
    } else if left > right {
        left
    } else {
        right
    }
}

pub(super) fn trunc_i32_from_f32(value: f32, signed: bool) -> Result<i32, CoreError> {
    trunc_i32_from_f64(f64::from(value), signed)
}

pub(super) fn trunc_i32_from_f64(value: f64, signed: bool) -> Result<i32, CoreError> {
    if value.is_nan() {
        return Err(CoreError::Trap("invalid conversion to integer".into()));
    }
    let value = value.trunc();
    if signed {
        if !(-2_147_483_648.0..2_147_483_648.0).contains(&value) {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as i32)
    } else {
        if !(0.0..4_294_967_296.0).contains(&value) {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as u32 as i32)
    }
}

pub(super) fn trunc_i64_from_f32(value: f32, signed: bool) -> Result<i64, CoreError> {
    trunc_i64_from_f64(f64::from(value), signed)
}

pub(super) fn trunc_i64_from_f64(value: f64, signed: bool) -> Result<i64, CoreError> {
    if value.is_nan() {
        return Err(CoreError::Trap("invalid conversion to integer".into()));
    }
    let value = value.trunc();
    if signed {
        if value < -9_223_372_036_854_775_808.0 || value >= 9_223_372_036_854_775_808.0 {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as i64)
    } else {
        if value < 0.0 || value >= 18_446_744_073_709_551_616.0 {
            return Err(CoreError::Trap("integer overflow".into()));
        }
        Ok(value as u64 as i64)
    }
}

pub(super) fn saturating_i32_from_f32(value: f32, signed: bool) -> i32 {
    saturating_i32_from_f64(f64::from(value), signed)
}

pub(super) fn saturating_i32_from_f64(value: f64, signed: bool) -> i32 {
    if value.is_nan() {
        return 0;
    }
    if signed {
        value.trunc().clamp(i32::MIN as f64, i32::MAX as f64) as i32
    } else {
        value.trunc().clamp(0.0, u32::MAX as f64) as u32 as i32
    }
}

pub(super) fn saturating_i64_from_f32(value: f32, signed: bool) -> i64 {
    saturating_i64_from_f64(f64::from(value), signed)
}

pub(super) fn saturating_i64_from_f64(value: f64, signed: bool) -> i64 {
    if value.is_nan() {
        return 0;
    }
    if signed {
        value.trunc().clamp(i64::MIN as f64, i64::MAX as f64) as i64
    } else {
        value.trunc().clamp(0.0, u64::MAX as f64) as u64 as i64
    }
}
//#endregion 🔢️NumericSemantics
