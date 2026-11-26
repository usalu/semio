//! 📥️ Declared Puzzle2d JSON restores words only at its typed floating-point fields.
use crate::Puzzle2dSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::parse_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::JsonValue;

pub fn register() {}

fn duplicate_member(value: &JsonValue) -> Option<&str> {
    match value {
        JsonValue::Object { members } => {
            let mut names = std::collections::HashSet::with_capacity(members.len());
            members.iter().find_map(|member| {
                if names.insert(member.key.as_str()) { duplicate_member(&member.value) } else { Some(member.key.as_str()) }
            })
        }
        JsonValue::Array { items } => items.iter().find_map(duplicate_member),
        _ => None,
    }
}

pub fn deserialize(from: &JsonSnapshot) -> Result<Puzzle2dSnapshot, semio_framework_diagnostic::TextError> {
    if let Some(name) = duplicate_member(&from.value) {
        return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("puzzle2d<-json: repeated field {name}"), semio_framework_diagnostic::TextSpan::at(1, 1)));
    }
    let raw = semio_framework_pack_json::to_dsl_value(&from.to_pack_value());
    let raw = crate::standards::v1::subsets::any::io::puzzle2d_json::convert(raw, true)
        .map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("puzzle2d<-json: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    semio_framework_value::FromValue::from_value(raw)
        .map_err(|error| semio_framework_diagnostic::TextError::new(error.kind, format!("puzzle2d<-json: {error}"), semio_framework_diagnostic::TextSpan::at(1, 1)))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Puzzle2dSnapshot, semio_framework_diagnostic::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
