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
