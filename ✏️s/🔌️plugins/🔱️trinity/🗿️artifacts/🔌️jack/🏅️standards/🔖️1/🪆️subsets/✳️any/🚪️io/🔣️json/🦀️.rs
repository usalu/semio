//! 🔌️ Declared Jack JSON camera fields own closed exact binary64 words; decoding also admits a plain number, as the artifact
//! schema's `Binary64Transport` does.
use semio_framework_value::DslValue;
use semio_framework_value::{ValueError, ValueRefusalKind};
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }
pub(crate) fn convert(mut value: DslValue, decode: bool) -> Result<DslValue, ValueError> {
    let semio_framework_value::DslValue::Object(fields) = &mut value else { return Err(invalid("Jack JSON object required")) };
    if decode {
        if fields.iter().any(|(key, _)| !["schema", "name", "manifestId", "manifest", "camera", "content", "rootNodeId", "query"].contains(&key.as_str())) { return Err(invalid("Jack JSON parent has unknown fields")); }
        for key in ["schema", "name", "manifest", "camera", "content", "query"] { if !fields.iter().any(|(name, _)| name == key) { return Err(invalid(format!("Jack JSON parent requires {key}"))); } }
    }
    if let Some((_, camera)) = fields.iter_mut().find(|(key, _)| key == "camera") {
        let semio_framework_value::DslValue::Object(axes) = camera else { return Err(invalid("Jack JSON camera object required")) };
        for key in ["x", "y", "zoom"] {
            if let Some((_, axis)) = axes.iter_mut().find(|(name, _)| name == key) {
                if decode && axis.as_f64().is_some() {
                    continue;
                }
                if decode {
                    let semio_framework_value::DslValue::Object(word) = axis else { return Err(invalid("Jack JSON camera requires a closed word or a plain number")) };
                    if word.len() != 1 || word[0].0 != "bits" {
                        return Err(invalid("Jack JSON camera word fields"));
                    }
                    let bits = word[0].1.as_str().ok_or_else(|| invalid("Jack JSON camera bits require text"))?;
                    if bits.len() != 16 || !bits.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
                        return Err(invalid("Jack JSON camera bits require lowercase hex64"));
                    }
                    *axis = semio_framework_value::DslValue::float(f64::from_bits(u64::from_str_radix(bits, 16).map_err(|error| invalid(error.to_string()))?));
                } else {
                    let bits = axis.as_f64().ok_or_else(|| invalid("Jack owned camera scalar required"))?.to_bits();
                    *axis = semio_framework_value::DslValue::object([("bits".into(), semio_framework_value::DslValue::String(format!("{bits:016x}")))]);
                }
            }
        }
    }
    Ok(value)
}
