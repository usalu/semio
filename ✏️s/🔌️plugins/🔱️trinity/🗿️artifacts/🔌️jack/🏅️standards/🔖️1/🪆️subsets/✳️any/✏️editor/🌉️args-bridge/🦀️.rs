//! 🌉️ Jack action bridge: folds the shell `{action, args}` vocabulary into the typed `TrinityJackCommand`.

//#region 🔖️ActionBridge
use super::TrinityJackCommand;
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::{DslValue, Fault, FaultCode, FaultOrigin};

fn invalid(action: &str, detail: impl std::fmt::Display) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("jack action '{action}' arguments do not decode: {detail}"))
}

fn field<'a>(args: Option<&'a DslValue>, keys: &[&str]) -> Option<&'a DslValue> {
    keys.iter().find_map(|key| args.and_then(|args| args.get(key))).filter(|value| !matches!(value, DslValue::Null))
}

/// 📝️ Prints a host control value into the `String` field a verb carries (`"512"`, `"true"`, or the text itself).
fn text(args: Option<&DslValue>, keys: &[&str]) -> Option<String> {
    field(args, keys).map(|value| match value {
        DslValue::String(text) => text.clone(),
        DslValue::Number(number) => {
            let float = number.as_f64();
            if float.is_finite() && float.fract() == 0.0 { format!("{}", float as i64) } else { format!("{float}") }
        }
        DslValue::Bool(flag) => flag.to_string(),
        other => semio_framework_pack_json::to_json_string(other),
    })
}

fn required(action: &str, args: Option<&DslValue>, keys: &[&str]) -> Result<String, Fault> {
    text(args, keys).ok_or_else(|| invalid(action, format!("missing {}", keys[0])))
}

fn offset(action: &str, args: Option<&DslValue>, key: &str) -> Result<u64, Fault> {
    match field(args, &[key]) {
        Some(DslValue::Number(number)) if number.as_f64().is_finite() && number.as_f64() >= 0.0 && number.as_f64().fract() == 0.0 => Ok(number.as_f64() as u64),
        Some(DslValue::String(text)) => text.parse().map_err(|error| invalid(action, format!("{key}: {error}"))),
        _ => Err(invalid(action, format!("missing {key}"))),
    }
}

/// 🔡️ `nodeIds` arrives as a list, a JSON-array string, or the rail's text field listing ids
/// separated by commas or whitespace. An empty field means "the current node selection".
fn ids(args: Option<&DslValue>) -> Vec<String> {
    match field(args, &["nodeIds", "node_ids", "ids"]) {
        Some(DslValue::Array(items)) => items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect(),
        Some(DslValue::String(text)) if text.trim_start().starts_with('[') => semio_framework_pack_json::from_json_str::<Vec<String>>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default(),
        Some(DslValue::String(text)) => text.split(|character: char| character == ',' || character.is_whitespace()).filter(|id| !id.is_empty()).map(str::to_string).collect(),
        _ => Vec::new(),
    }
}

/// 📷️ The node-graph host nests its pose under `viewport`; a flat `{x, y, zoom}` payload is the pose itself.
fn viewport(action: &str, args: Option<&DslValue>) -> Result<Viewport2d, Fault> {
    let coordinate = |value: Option<&DslValue>, fallback: f64| value.and_then(DslValue::as_f64).unwrap_or(fallback);
    let pose = field(args, &["viewport"]).or(args).ok_or_else(|| invalid(action, "missing viewport"))?;
    let viewport = Viewport2d { x: coordinate(pose.get("x"), 0.0), y: coordinate(pose.get("y"), 0.0), zoom: coordinate(pose.get("zoom"), 1.0) };
    viewport.validate().map_err(|error| invalid(action, format!("{error:?}")))?;
    Ok(viewport)
}

pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<TrinityJackCommand, Fault> {
    const RESULTS: &[&str] = &["resultsWindowId", "results_window_id"];
    Ok(match action {
        "loadDocumentJson" => TrinityJackCommand::LoadDocumentJson { json: required(action, args, &["json", "value"])? },
        "deleteSelection" => TrinityJackCommand::DeleteSelection,
        "patchNodes" => TrinityJackCommand::PatchNodes { node_ids: ids(args), field: text(args, &["field"]).unwrap_or_else(|| "name".into()), value: required(action, args, &["value"])? },
        "runQuery" => TrinityJackCommand::RunQuery { query: text(args, &["query"]).filter(|query| !query.trim().is_empty()), results_window_id: text(args, RESULTS).unwrap_or_default() },
        "loadExampleQuery" => TrinityJackCommand::LoadExampleQuery { query: required(action, args, &["query", "value"])?, results_window_id: text(args, RESULTS).unwrap_or_default() },
        "setActiveExample" => TrinityJackCommand::SetActiveExample { example_id: required(action, args, &["exampleId", "example_id", "value", "id"])? },
        "nodeGraphViewport" => TrinityJackCommand::SetViewport { surface_id: text(args, &["surfaceId", "surface_id"]), viewport: viewport(action, args)? },
        "textEdit" => TrinityJackCommand::TextEdit { text: text(args, &["text", "value"]).unwrap_or_default() },
        "textSelect" => TrinityJackCommand::TextSelect { start: offset(action, args, "start")?, end: offset(action, args, "end")? },
        "formatDocument" => TrinityJackCommand::FormatDocument,
        "setLodMode" => TrinityJackCommand::SetLodMode { value: required(action, args, &["value", "mode"])? },
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the trinity jack editor has no command for action '{action}'"))),
    })
}
//#endregion 🔖️ActionBridge
