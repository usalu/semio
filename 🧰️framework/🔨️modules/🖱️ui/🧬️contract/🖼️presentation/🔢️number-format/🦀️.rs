/// 🔢️ Twelve significant decimal digits with the same display thresholds as React's formatNumber.
pub fn format_ui_number(value: f64) -> String {
    if !value.is_finite() { return String::new() }
    if value == 0.0 { return "0".into() }
    let scientific = format!("{:.11e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').expect("scientific numeric format");
    let exponent: i32 = exponent.parse().expect("scientific decimal exponent");
    let mut rounded_digits = mantissa.replace('.', "").into_bytes();
    if scientific.parse::<f64>().is_ok_and(|rounded| rounded < value.abs()) && exact_decimal_half(value.abs(), exponent - 11) {
        if let Some(last) = rounded_digits.last_mut() { *last += 1; }
    }
    let rounded_digits = String::from_utf8(rounded_digits).expect("decimal digits");
    let rounded = format!("{rounded_digits}e{}", exponent - 11).parse::<f64>().expect("rounded decimal");
    let shortest = rounded.to_string();
    let (coefficient, power) = shortest.split_once('e').map_or((shortest.as_str(), 0), |(coefficient, power)| (coefficient, power.parse::<i32>().expect("shortest decimal exponent")));
    let point = coefficient.find('.').unwrap_or(coefficient.len());
    let digits = coefficient.replace('.', "");
    let leading = digits.bytes().position(|digit| digit != b'0').expect("nonzero rounded value");
    let exponent = point as i32 - leading as i32 - 1 + power;
    let digits = digits[leading..].trim_end_matches('0');
    let sign = if value.is_sign_negative() { "-" } else { "" };
    if !(-6..21).contains(&exponent) {
        let tail = if digits.len() > 1 { format!(".{}", &digits[1..]) } else { String::new() };
        return format!("{sign}{}{tail}e{}{exponent}", &digits[..1], if exponent >= 0 { "+" } else { "" });
    }
    let point = exponent + 1;
    if point <= 0 { return format!("{sign}0.{}{digits}", "0".repeat((-point) as usize)) }
    let point = point as usize;
    if point >= digits.len() { return format!("{sign}{digits}{}", "0".repeat(point - digits.len())) }
    format!("{sign}{}.{}", &digits[..point], &digits[point..])
}

/// ⚖️ An exact binary value lies halfway between two decimal multiples only at this exponent.
fn exact_decimal_half(value: f64, decimal_power: i32) -> bool {
    let bits = value.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let mantissa = (bits & ((1_u64 << 52) - 1)) | if exponent == 0 { 0 } else { 1_u64 << 52 };
    let trailing = mantissa.trailing_zeros();
    let binary_power = if exponent == 0 { -1074 } else { exponent - 1075 } + trailing as i32;
    if binary_power != decimal_power - 1 { return false }
    decimal_power <= 0 || (decimal_power <= 22 && (mantissa >> trailing) % 5_u64.pow(decimal_power as u32) == 0)
}

/// 🎯️ `value` with exactly `precision` fraction digits, ties away from zero — the twin of JavaScript's
/// `toFixed`, so a number field reads identically in every renderer. Negative zero prints unsigned and a
/// magnitude `toFixed` itself would print in exponent form falls back to [`format_ui_number`].
pub fn format_ui_number_fixed(value: f64, precision: u16) -> String {
    if !value.is_finite() { return String::new() }
    if value.abs() >= 1e21 { return format_ui_number(value) }
    let digits = usize::from(precision.min(crate::UI_NUMBER_PRECISION_MAX));
    let magnitude = value.abs();
    let text = if exact_decimal_half(magnitude, -(digits as i32)) {
        let exact = format!("{magnitude:.*}", digits + 1);
        decimal_increment(exact[..exact.len() - 1].trim_end_matches('.'))
    } else {
        format!("{magnitude:.digits$}")
    };
    if value.is_sign_negative() && text.bytes().any(|byte| byte.is_ascii_digit() && byte != b'0') { format!("-{text}") } else { text }
}

/// ➕️ A non-negative decimal string plus one unit in its last place, carrying through nines — the exact half-up
/// step of a tie, done on the digits because a unit below one ulp would vanish in `f64`.
fn decimal_increment(text: &str) -> String {
    let mut digits = text.as_bytes().to_vec();
    for byte in digits.iter_mut().rev().filter(|byte| byte.is_ascii_digit()) {
        if *byte == b'9' {
            *byte = b'0';
        } else {
            *byte += 1;
            return String::from_utf8(digits).expect("ascii decimal");
        }
    }
    format!("1{}", String::from_utf8(digits).expect("ascii decimal"))
}

/// 🧮️ `value` rounded to `precision` fraction digits by the same law [`format_ui_number_fixed`] prints.
pub fn round_ui_number(value: f64, precision: u16) -> f64 {
    if !value.is_finite() || value.abs() >= 1e21 { return value }
    format_ui_number_fixed(value, precision).parse().unwrap_or(value)
}
