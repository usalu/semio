//! 📝️ Float writer: the scalar text sink, the f64 and f32 spellings and the exact-arithmetic digit generator. See [`super`].

use std::cmp::Ordering;
use std::fmt;

//#region 🔖️Writer
pub struct ScalarText { pub bytes: [u8; 128], pub length: usize }

impl ScalarText {
    pub fn new() -> Self { Self { bytes: [0; 128], length: 0 } }
    pub fn text(&self) -> &str { std::str::from_utf8(&self.bytes[..self.length]).expect("formatted scalar UTF-8") }
}

impl fmt::Write for ScalarText {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.length.checked_add(text.len()).filter(|end| *end <= self.bytes.len()).ok_or(fmt::Error)?;
        self.bytes[self.length..end].copy_from_slice(text.as_bytes()); self.length = end; Ok(())
    }
}

/// 💾️ Writes canonical binary64 JSON directly into admitted first-party or system storage.
pub fn write_float_to(value: f64, out: &mut impl fmt::Write) -> fmt::Result {
    use fmt::Write as _;
    if !value.is_finite() {
        return out.write_str("null");
    }
    if value == 0.0 {
        return out.write_str(if value.is_sign_negative() { "-0.0" } else { "0.0" });
    }
    let negative = value.is_sign_negative();
    let magnitude = value.abs();
    let mut scientific = ScalarText::new();
    write!(scientific, "{magnitude:e}")?;
    let (mantissa_text, exponent_text) = scientific.text().split_once('e').expect("LowerExp always emits an exponent");
    let mut exponent: i32 = exponent_text.parse().expect("LowerExp exponent is always a plain integer");
    let digit_count = mantissa_text.bytes().filter(|byte| *byte != b'.').count();
    let (mut digits, exponent_adjust) = float_format::correctly_rounded_digits(magnitude, exponent, digit_count);
    exponent += exponent_adjust;
    if exponent_adjust != 0 {
        digits.truncate(digit_count);
    }
    write_decimal_digits(negative, &digits, exponent, -5..=15, out)
}

/// 🔢️ Preserves binary32 shortest digits and its canonical JSON decimal range without promoting its precision.
pub fn write_float32_to(value: f32, out: &mut impl fmt::Write) -> fmt::Result {
    use fmt::Write as _;
    if !value.is_finite() { return out.write_str("null"); }
    if value == 0.0 { return out.write_str(if value.is_sign_negative() { "-0.0" } else { "0.0" }); }
    let mut scientific = ScalarText::new();
    write!(scientific, "{:e}", value.abs())?;
    let (mantissa, exponent) = scientific.text().split_once('e').expect("LowerExp always emits an exponent");
    let mut exponent: i32 = exponent.parse().expect("LowerExp exponent is a plain integer");
    let count = mantissa.bytes().filter(|byte| *byte != b'.').count();
    let (mut digits, adjustment) = float_format::correctly_rounded_digits(value.abs() as f64, exponent, count);
    exponent += adjustment;
    if adjustment != 0 { digits.truncate(count); }
    write_decimal_digits(value.is_sign_negative(), &digits, exponent, -6..=12, out)
}

fn write_decimal_digits(negative: bool, digits: &[u8], exponent: i32, fixed: std::ops::RangeInclusive<i32>, out: &mut impl fmt::Write) -> fmt::Result {
    let digit_count = digits.len() as i32;
    if negative {
        out.write_char('-')?;
    }
    if fixed.contains(&exponent) {
        if exponent >= digit_count - 1 {
            out.write_str(std::str::from_utf8(digits).expect("decimal digits are ASCII"))?;
            for _ in 0..(exponent - (digit_count - 1)) {
                out.write_char('0')?;
            }
            out.write_str(".0")?;
        } else if exponent >= 0 {
            let integer_len = (exponent + 1) as usize;
            out.write_str(std::str::from_utf8(&digits[..integer_len]).expect("decimal digits are ASCII"))?;
            out.write_char('.')?;
            out.write_str(std::str::from_utf8(&digits[integer_len..]).expect("decimal digits are ASCII"))?;
        } else {
            out.write_str("0.")?;
            for _ in 0..(-exponent - 1) {
                out.write_char('0')?;
            }
            out.write_str(std::str::from_utf8(digits).expect("decimal digits are ASCII"))?;
        }
    } else {
        out.write_char(digits[0] as char)?;
        if digits.len() > 1 {
            out.write_char('.')?;
            out.write_str(std::str::from_utf8(&digits[1..]).expect("decimal digits are ASCII"))?;
        }
        out.write_char('e')?;
        if exponent >= 0 {
            out.write_char('+')?;
        }
        fmt::Write::write_fmt(out, format_args!("{exponent}"))?;
    }
    Ok(())
}

/// ✍️ [`write_float`] as a standalone string, for callers that write one bare JSON number
/// directly (no [`Value`] tree) — `🧵️canonical-edit::ScalarBytes` is exactly this shape:
/// a fixed-size scalar buffer, not a document.
pub fn format_f64(value: f64) -> String {
    let mut out = String::new();
    let _ = write_float_to(value, &mut out);
    out
}
//#endregion 🔖️Writer

//#region 🔖️FloatFormat
/// 🎯️ Exact-arithmetic correctly-rounded decimal digit generation for `f64`, used to reconcile
/// [`write_float`]'s digit string with `serde_json`'s (`zmij`'s) round-half-to-even tie-break at
/// the last significant digit — Rust's own shortest-round-trip `{:e}` formatter picks the
/// mathematically-correct-length, correctly-magnituded digit string, but at an exact
/// halfway-point tie (the true binary value sits precisely between two adjacent same-length
/// decimal strings, both of which round-trip) it does not consistently pick the even one the way
/// IEEE 754 and `zmij`'s Schubfach-based writer do. A "does a neighboring digit also round-trip"
/// probe was tried first and found unsound for large-magnitude values, where the round-trip basin
/// can hold more than two adjacent minimal-length candidates — this module instead recomputes the
/// digits directly from the value's exact rational form, `mantissa * 2^binary_exponent`, using a
/// small first-party big unsigned integer (no external bignum crate).
mod float_format {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Limbs { pub values: [u32; 80], pub length: usize }
    impl Limbs {
        fn zeroes(length: usize) -> Self { assert!(length <= 80); Self { values: [0; 80], length } }
        fn push(&mut self, value: u32) { assert!(self.length < self.values.len()); self.values[self.length] = value; self.length += 1; }
        fn pop(&mut self) -> Option<u32> { if self.length == 0 { None } else { self.length -= 1; Some(self.values[self.length]) } }
    }
    impl std::ops::Deref for Limbs { type Target = [u32]; fn deref(&self) -> &[u32] { &self.values[..self.length] } }
    impl std::ops::DerefMut for Limbs { fn deref_mut(&mut self) -> &mut [u32] { &mut self.values[..self.length] } }
    pub(super) struct Digits { pub values: [u8; 32], pub length: usize }
    impl Digits {
        fn new() -> Self { Self { values: [0; 32], length: 0 } }
        fn push(&mut self, value: u8) { assert!(self.length < self.values.len()); self.values[self.length] = value; self.length += 1; }
        fn pop(&mut self) { assert!(self.length > 0); self.length -= 1; }
        fn prepend_zero(&mut self) { assert!(self.length < self.values.len()); self.values.copy_within(0..self.length, 1); self.values[0] = b'0'; self.length += 1; }
        pub(super) fn truncate(&mut self, length: usize) { self.length = self.length.min(length); }
    }
    impl std::ops::Deref for Digits { type Target = [u8]; fn deref(&self) -> &[u8] { &self.values[..self.length] } }
    impl std::ops::DerefMut for Digits { fn deref_mut(&mut self) -> &mut [u8] { &mut self.values[..self.length] } }
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Big(Limbs);

    impl Big {
        /// 🌱️ A single 64-bit value as a two-limb (or trimmed one-limb) big unsigned integer.
        fn from_u64(value: u64) -> Self {
            let mut limbs = Limbs::zeroes(2); limbs[0] = value as u32; limbs[1] = (value >> 32) as u32;
            Self::trim(&mut limbs);
            Big(limbs)
        }

        fn trim(limbs: &mut Limbs) {
            while limbs.len() > 1 && *limbs.last().expect("non-empty by loop condition") == 0 {
                limbs.pop();
            }
        }

        fn is_zero(&self) -> bool {
            self.0.len() == 1 && self.0[0] == 0
        }

        /// ✳️ In-place multiply by a small (`u32`-sized) factor.
        fn mul_small(&mut self, factor: u32) {
            let mut carry: u64 = 0;
            for limb in self.0.iter_mut() {
                let product = (*limb as u64) * (factor as u64) + carry;
                *limb = product as u32;
                carry = product >> 32;
            }
            if carry > 0 {
                self.0.push(carry as u32);
            }
            Self::trim(&mut self.0);
        }

        /// ✖️ In-place multiply by `5^exponent`, chunked so every factor still fits a `u32`.
        fn mul_pow5(&mut self, mut exponent: u32) {
            while exponent > 0 {
                let chunk = exponent.min(13);
                self.mul_small(5u32.pow(chunk));
                exponent -= chunk;
            }
        }

        /// ⬅️️ In-place multiply by `2^bits`.
        fn shl(&mut self, bits: u32) {
            if bits == 0 {
                return;
            }
            let limb_shift = (bits / 32) as usize;
            let bit_shift = bits % 32;
            let mut result = Limbs::zeroes(self.0.len() + limb_shift + 1);
            for (index, &limb) in self.0.iter().enumerate() {
                let value = limb as u64;
                if bit_shift == 0 {
                    result[index + limb_shift] |= value as u32;
                } else {
                    let shifted = value << bit_shift;
                    result[index + limb_shift] |= shifted as u32;
                    result[index + limb_shift + 1] |= (shifted >> 32) as u32;
                }
            }
            Self::trim(&mut result);
            self.0 = result;
        }

        fn bit_length(&self) -> u32 {
            let top = *self.0.last().expect("non-empty by construction");
            if top == 0 {
                0
            } else {
                (self.0.len() as u32 - 1) * 32 + (32 - top.leading_zeros())
            }
        }

        fn get_bit(&self, index: u32) -> bool {
            let limb = (index / 32) as usize;
            let bit = index % 32;
            if limb >= self.0.len() {
                false
            } else {
                (self.0[limb] >> bit) & 1 == 1
            }
        }

        fn cmp(&self, other: &Big) -> std::cmp::Ordering {
            if self.0.len() != other.0.len() {
                return self.0.len().cmp(&other.0.len());
            }
            for index in (0..self.0.len()).rev() {
                if self.0[index] != other.0[index] {
                    return self.0[index].cmp(&other.0[index]);
                }
            }
            std::cmp::Ordering::Equal
        }

        /// ➖️ In-place `self -= other`, requiring `self >= other`.
        fn sub_assign(&mut self, other: &Big) {
            let mut borrow: i64 = 0;
            for index in 0..self.0.len() {
                let lhs = self.0[index] as i64;
                let rhs = if index < other.0.len() { other.0[index] as i64 } else { 0 };
                let mut diff = lhs - rhs - borrow;
                if diff < 0 {
                    diff += 1 << 32;
                    borrow = 1;
                } else {
                    borrow = 0;
                }
                self.0[index] = diff as u32;
            }
            Self::trim(&mut self.0);
        }

        fn shl1(&mut self) {
            let mut carry = 0u32;
            for limb in self.0.iter_mut() {
                let next_carry = *limb >> 31;
                *limb = (*limb << 1) | carry;
                carry = next_carry;
            }
            if carry != 0 {
                self.0.push(carry);
            }
        }

        fn add_one(&mut self) {
            let mut carry = 1u64;
            for limb in self.0.iter_mut() {
                let sum = *limb as u64 + carry;
                *limb = sum as u32;
                carry = sum >> 32;
                if carry == 0 {
                    return;
                }
            }
            if carry > 0 {
                self.0.push(carry as u32);
            }
        }

        /// ➗️ Schoolbook binary long division: `self / other`, returning `(quotient, remainder)`.
        fn div_rem(&self, other: &Big) -> (Big, Big) {
            let bits = self.bit_length();
            let mut quotient = Big(Limbs::zeroes((bits / 32) as usize + 1));
            let mut remainder = Big(Limbs::zeroes(1));
            for index in (0..bits).rev() {
                remainder.shl1();
                if self.get_bit(index) {
                    remainder.0[0] |= 1;
                }
                if remainder.cmp(other) != std::cmp::Ordering::Less {
                    remainder.sub_assign(other);
                    let limb = (index / 32) as usize;
                    quotient.0[limb] |= 1 << (index % 32);
                }
            }
            Self::trim(&mut quotient.0);
            Self::trim(&mut remainder.0);
            (quotient, remainder)
        }

        fn double(&self) -> Big {
            let mut doubled = self.clone();
            doubled.shl1();
            doubled
        }

        /// 🔟️ Base-10 textual expansion, most-significant digit first, no leading zero (unless
        /// the value itself is zero).
        fn to_decimal_digits(&self) -> Digits {
            if self.is_zero() {
                let mut digits = Digits::new(); digits.push(b'0'); return digits;
            }
            let mut little_endian_digits = Digits::new();
            let mut remaining = self.clone();
            let billion = Big::from_u64(1_000_000_000);
            while !remaining.is_zero() {
                let (quotient, remainder) = remaining.div_rem(&billion);
                let mut chunk = remainder.0.iter().rev().fold(0u64, |accumulator, &limb| (accumulator << 32) | limb as u64);
                for _ in 0..9 {
                    little_endian_digits.push(b'0' + (chunk % 10) as u8);
                    chunk /= 10;
                }
                remaining = quotient;
            }
            while little_endian_digits.len() > 1 && *little_endian_digits.last().expect("non-empty by loop condition") == b'0' {
                little_endian_digits.pop();
            }
            little_endian_digits.reverse(); little_endian_digits
        }
    }

    /// ❎️ Decomposes a finite, non-zero, non-negative `f64` into its exact
    /// `mantissa * 2^binary_exponent` form (the mantissa carries the implicit leading bit for
    /// normal numbers; subnormals keep the raw 52-bit significand at the fixed minimum exponent).
    fn decompose(magnitude: f64) -> (u64, i32) {
        let bits = magnitude.to_bits();
        let raw_exponent = ((bits >> 52) & 0x7FF) as i32;
        let raw_mantissa = bits & 0x000F_FFFF_FFFF_FFFF;
        if raw_exponent == 0 {
            (raw_mantissa, 1 - 1023 - 52)
        } else {
            (raw_mantissa | (1u64 << 52), raw_exponent - 1023 - 52)
        }
    }

    /// ✅️ The correctly-rounded (round-half-to-even) `digit_count`-digit decimal significand for
    /// `magnitude`, given the leading-digit decimal exponent `decimal_exponent` — both already
    /// established by Rust's own shortest-round-trip `{:e}` formatter, which is trusted for LENGTH
    /// and MAGNITUDE (every case in this module's own differential sweep confirms both are already
    /// correct); only the exact DIGITS at that fixed precision are recomputed here. Returns
    /// `(digits, exponent_adjust)`, where `exponent_adjust` is `1` when rounding carries all the
    /// way through (e.g. `"999"` rounds up to a truncated `"100"` with the exponent bumped), `0`
    /// otherwise.
    pub(super) fn correctly_rounded_digits(magnitude: f64, decimal_exponent: i32, digit_count: usize) -> (Digits, i32) {
        let (mantissa, binary_exponent) = decompose(magnitude);
        let scale = decimal_exponent - (digit_count as i32 - 1);
        let power_of_two = binary_exponent - scale;
        let power_of_five = -scale;

        let mut numerator = Big::from_u64(mantissa);
        if power_of_five > 0 {
            numerator.mul_pow5(power_of_five as u32);
        }
        if power_of_two > 0 {
            numerator.shl(power_of_two as u32);
        }
        let mut denominator = Big::from_u64(1);
        if power_of_five < 0 {
            denominator.mul_pow5((-power_of_five) as u32);
        }
        if power_of_two < 0 {
            denominator.shl((-power_of_two) as u32);
        }

        let (mut quotient, remainder) = numerator.div_rem(&denominator);
        let comparison = remainder.double().cmp(&denominator);
        let quotient_is_odd = quotient.0[0] & 1 == 1;
        let round_up = match comparison {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => quotient_is_odd,
        };
        if round_up {
            quotient.add_one();
        }

        let mut digit_string = quotient.to_decimal_digits();
        let mut exponent_adjust = 0;
        if digit_string.len() > digit_count {
            debug_assert_eq!(digit_string.len(), digit_count + 1, "rounding carries at most one extra digit");
            debug_assert_eq!(digit_string.last(), Some(&b'0'), "a carry past the digit budget must land on a power of ten");
            digit_string.pop();
            exponent_adjust = 1;
        }
        while digit_string.len() < digit_count {
            digit_string.prepend_zero();
        }
        (digit_string, exponent_adjust)
    }
}
//#endregion 🔖️FloatFormat
