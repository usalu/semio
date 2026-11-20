//! 🌉️ Decodes the host's drawing actions into the editor command schema.

use super::DrawingCommand;
use semio_framework_value::{DslValue, FromValue, Number, ToValue};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_plugin::{Fault, FaultCode, FaultOrigin};

fn invalid_args(action: &str, detail: impl std::fmt::Display) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("Drawing action '{action}': {detail}"))
}

fn snake_case(key: &str) -> String {
    let mut name = String::with_capacity(key.len());
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            name.push('_');
        }
        name.push(ch.to_ascii_lowercase());
    }
    name
}

fn normalize_numbers(value: DslValue) -> DslValue {
    match value {
        DslValue::Number(Number::Float(number)) if number.fract() == 0.0 && number.abs() < 9_007_199_254_740_992.0 => {
            DslValue::Number(if number >= 0.0 { Number::UInt(number as u64) } else { Number::Int(number as i64) })
        }
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(normalize_numbers).collect()),
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, normalize_numbers(value))).collect()),
        value => value,
    }
}

fn override_edit(action: &str, edit: &mut DslValue, value: DslValue) -> Result<(), Fault> {
    let kind = edit.get("kind").and_then(DslValue::as_str);
    let (field, value) = match (action, kind, value) {
        ("editFill", Some("color"), DslValue::String(color)) => {
            let alpha = match edit.get("value") {
                Some(DslValue::Array(channels)) => channels.get(3).and_then(DslValue::as_f64).unwrap_or(1.0),
                _ => 1.0,
            };
            let mut accepted = |_| true;
            let mut control = NativeDecodeControl::new(64, &mut accepted);
            let rgba = semio_s_artifact_draw_drawing_io::text::color::decode_color_text(&color, alpha, &mut control)
                .map_err(|error| invalid_args(action, error))?;
            ("value", rgba.to_value())
        }
        ("editPath", Some("coordinate"), value) | ("editFill", Some("coordinate" | "alpha" | "offset" | "addStop"), value) => {
            let field = if kind == Some("addStop") { "offset" } else { "value" };
            let number = match value {
                DslValue::String(text) => text.trim().parse::<f64>().ok(),
                value => value.as_f64(),
            }.filter(|number| number.is_finite()).ok_or_else(|| invalid_args(action, "expected a finite edit value"))?;
            (field, number.to_value())
        }
        ("editFill", Some("color" | "type"), value) => ("value", value),
        _ => return Err(invalid_args(action, "the edit does not accept a control value")),
    };
    let DslValue::Object(entries) = edit else {
        return Err(invalid_args(action, "expected an edit object"));
    };
    entries.retain(|(key, _)| key != field);
    entries.push((field.into(), value));
    Ok(())
}

/// 🎯️ Resolves a manifest action and decodes its arguments using the generated command enum.
pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<DrawingCommand, Fault> {
    if !DrawingCommand::TOOL_JOB_IDS.contains(&action) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("The drawing editor has no command for action '{action}'")));
    }
    let mut entries = std::collections::BTreeMap::new();
    match args {
        Some(DslValue::Object(args)) => {
            for (key, value) in args {
                if entries.insert(snake_case(key), normalize_numbers(value.clone())).is_some() {
                    return Err(invalid_args(action, format!("duplicate argument '{key}'")));
                }
            }
        }
        None | Some(DslValue::Null) => {}
        _ => return Err(invalid_args(action, "expected an argument object")),
    }

    let default_text = |value: &str| DslValue::String(value.into());
    match action {
        "addLayer" | "dropLayerKind" => { entries.entry("kind".into()).or_insert_with(|| default_text("path")); }
        "exportDocument" => { entries.entry("format".into()).or_insert_with(|| default_text(super::export_document::DEFAULT_EXPORT_FORMAT)); }
        "combineBoolean" => { entries.entry("operation".into()).or_insert_with(|| default_text("union")); }
        "patchLayer" | "patchLayers" | "loadDocumentJson" => {
            let field = if action == "loadDocumentJson" { "json" } else { "value" };
            if let Some(value) = entries.get_mut(field) {
                if !matches!(value, DslValue::String(_)) {
                    *value = DslValue::String(semio_framework_pack_json::to_json_string(value));
                }
            }
        }
        "editPath" | "editFill" => {
            let edit = entries.remove("edit").ok_or_else(|| invalid_args(action, "missing edit"))?;
            let mut edit = match edit {
                DslValue::String(json) => semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map(normalize_numbers)
                    .map_err(|error| invalid_args(action, error))?,
                edit => edit,
            };
            if let Some(value) = entries.remove("value") {
                override_edit(action, &mut edit, value)?;
            }
            entries.insert("edit".into(), edit);
        }
        _ => {}
    }
    let defaults = match action {
        "editSelection" | "combineBoolean" => Some(("ids", DslValue::Array(Vec::new()))),
        "patchLayers" => Some(("layer_ids", DslValue::Array(Vec::new()))),
        "dropLayerKind" | "moveLayer" => Some(("drop_position", default_text("after"))),
        _ => None,
    };
    if let Some((field, value)) = defaults {
        entries.entry(field.into()).or_insert(value);
    }
    let mut variant = action.to_owned();
    variant[..1].make_ascii_uppercase();
    let payload = DslValue::Object(entries.into_iter().collect());
    DrawingCommand::from_value(DslValue::object([(variant, payload)])).map_err(|error| invalid_args(action, error))
}
