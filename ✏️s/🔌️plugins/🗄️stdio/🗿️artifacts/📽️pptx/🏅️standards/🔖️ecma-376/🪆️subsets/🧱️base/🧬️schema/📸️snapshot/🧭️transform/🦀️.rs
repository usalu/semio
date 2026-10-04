//! 📐️ Exact signed64 transform scalar boundary.
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
use semio_framework_value::{DslValue, NativeDecodeControl, NativeEncodeControl, ValueError};
use semio_framework_value::ValueRefusalKind;
fn parse(text: &str) -> Result<i64, String> {
    if text.len() > 20 {
        return Err("PPTX transform exceeds signed64 decimal width".into());
    }
    let value = text.parse::<i64>().map_err(|_| "PPTX transform requires signed64 decimal text")?;
    if value.to_string() != text {
        return Err("PPTX transform requires canonical signed64 decimal text".into());
    }
    Ok(value)
}
pub fn to_value(value: &i64) -> DslValue {
    DslValue::String(value.to_string())
}
pub fn from_value(value: DslValue) -> Result<i64, ValueError> {
    match value {
        DslValue::String(text) => parse(&text).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error)),
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "PPTX transform requires decimal text")),
    }
}
pub fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<i64, ValueError> {
    control.checkpoint()?;
    match value {
        DslValue::String(text) => parse(text).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, error)),
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "PPTX transform requires decimal text")),
    }
}
pub fn to_value_controlled(value: &i64, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    let mut digits = [0u8; 20];
    let mut magnitude = value.unsigned_abs();
    let mut at = 20;
    loop {
        at -= 1;
        digits[at] = b'0' + (magnitude % 10) as u8;
        magnitude /= 10;
        if magnitude == 0 {
            break;
        }
    }
    if *value < 0 {
        at -= 1;
        digits[at] = b'-';
    }
    control.copy_text(std::str::from_utf8(&digits[at..]).unwrap()).map(DslValue::String)
}
