//! 🔣️ Literal Block2d JSON words retain all IEEE states independently of native scalar printing.
use crate::Block2dSnapshot;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::Number;
use semio_framework_value::ToValue;

fn word(value: f64) -> DslValue {
    semio_framework_value::DslValue::Object(vec![("bits".into(), DslValue::String(format!("{:016x}", value.to_bits())))])
}
fn members(value: &mut DslValue) -> Result<&mut Vec<(String, DslValue)>, String> {
    match value {
        semio_framework_value::DslValue::Object(value) => Ok(value),
        _ => Err("Block2d JSON record is not an object".into()),
    }
}
fn field<'a>(value: &'a mut DslValue, key: &str) -> Result<&'a mut DslValue, String> {
    members(value)?.iter_mut().find(|(name, _)| name == key).map(|(_, value)| value).ok_or_else(|| format!("Block2d JSON field {key} is missing"))
}
fn put(value: &mut DslValue, key: &str, owned: DslValue) {
    *field(value, key).expect("authored Block2d field") = owned;
}
fn decode_word(value: &mut DslValue) -> Result<(), String> {
    let semio_framework_value::DslValue::Object(entries) = value else { return Err("Block2d JSON word must be a closed bits object".into()) };
    if entries.len() != 1 || entries[0].0 != "bits" {
        return Err("Block2d JSON word must contain only bits".into());
    }
    let semio_framework_value::DslValue::String(bits) = &entries[0].1 else { return Err("Block2d JSON bits must be text".into()) };
    if bits.len() != 16 || !bits.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err("Block2d JSON bits must be sixteen lowercase hexadecimal digits".into());
    }
    let bits = u64::from_str_radix(bits, 16).map_err(|_| "Block2d JSON bits exceed binary64")?;
    *value = semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(f64::from_bits(bits)));
    Ok(())
}

/// 📤️ Emits every declared floating field as its exact closed binary64 word.
pub(crate) fn to_json_text(source: &Block2dSnapshot) -> String {
    let mut value = source.to_value();
    let presentation = field(&mut value, "presentation").unwrap();
    for (key, number) in [("radius", source.presentation.radius), ("width", source.presentation.width), ("height", source.presentation.height)] {
        if let Some(number) = number {
            put(presentation, key, word(number));
        }
    }
    let semio_framework_value::DslValue::Array(handles) = field(&mut value, "handles").unwrap() else { unreachable!("authored Block2d handles") };
    for (handle, source) in handles.iter_mut().zip(&source.handles) {
        put(handle, "angle", word(source.angle));
        put(handle, "radius", word(source.radius));
    }
    let camera = field(&mut value, "camera2d").unwrap();
    put(camera, "x", word(source.camera2d.x));
    put(camera, "y", word(source.camera2d.y));
    put(camera, "zoom", word(source.camera2d.zoom));
    semio_framework_pack_json::to_json_string(&value)
}

/// 📥️ Reconstructs only the declared closed binary64 transport fields.
pub(crate) fn from_json_text(text: &str) -> Result<Block2dSnapshot, String> {
    let mut value: DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    if let Some((_, presentation)) = members(&mut value)?.iter_mut().find(|(key, _)| key == "presentation") {
        for (key, number) in members(presentation)? {
            if matches!(key.as_str(), "radius" | "width" | "height") {
                decode_word(number)?;
            }
        }
    }
    if let Some((_, handles)) = members(&mut value)?.iter_mut().find(|(key, _)| key == "handles") {
        let semio_framework_value::DslValue::Array(handles) = handles else { return Err("Block2d JSON handles must be an array".into()) };
        for handle in handles {
            decode_word(field(handle, "angle")?)?;
            decode_word(field(handle, "radius")?)?;
        }
    }
    if let Some((_, camera)) = members(&mut value)?.iter_mut().find(|(key, _)| key == "camera2d") {
        for key in ["x", "y", "zoom"] {
            decode_word(field(camera, key)?)?;
        }
    }
    Block2dSnapshot::from_value(value).map_err(|error| error.to_string())
}
