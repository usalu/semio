//! 🧭️ Canonical unsigned64 XML doctype position scalar.
use semio_framework_value::{DslValue, NativeDecodeControl, NativeEncodeControl, ValueError, ValueRefusalKind};
pub fn parse(text: &str) -> Result<u64, String> {
    if text.is_empty() || text.len() > 1 && text.starts_with('0') || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("XML position requires canonical decimal text".into());
    }
    text.parse::<u64>().map_err(|_| "XML position requires unsigned64 decimal text".into())
}
pub fn digits(value: u64) -> usize {
    let mut value = value;
    let mut count = 1;
    while value >= 10 {
        value /= 10;
        count += 1;
    }
    count
}
fn decimal(value: u64, bytes: &mut [u8; 20]) -> &str {
    let mut value = value;
    let mut start = 20;
    loop {
        start -= 1;
        bytes[start] = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
    std::str::from_utf8(&bytes[start..]).unwrap()
}
pub fn to_value(value: &u64) -> DslValue {
    DslValue::String(value.to_string())
}
pub fn from_value(value: DslValue) -> Result<u64, ValueError> {
    match value {
        DslValue::String(text) => parse(&text).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message)),
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "XML position requires decimal text")),
    }
}
pub fn to_value_controlled(value: &u64, control: &mut NativeEncodeControl<'_>) -> Result<DslValue, ValueError> {
    let mut bytes = [0u8; 20];
    control.copy_text(decimal(*value, &mut bytes)).map(DslValue::String)
}
pub fn from_value_controlled(value: &DslValue, control: &mut NativeDecodeControl<'_>) -> Result<u64, ValueError> {
    control.checkpoint()?;
    match value {
        DslValue::String(text) => parse(text).map_err(|message| ValueError::new(ValueRefusalKind::InvalidValue, message)),
        _ => Err(ValueError::new(ValueRefusalKind::InvalidValue, "XML position requires decimal text")),
    }
}
