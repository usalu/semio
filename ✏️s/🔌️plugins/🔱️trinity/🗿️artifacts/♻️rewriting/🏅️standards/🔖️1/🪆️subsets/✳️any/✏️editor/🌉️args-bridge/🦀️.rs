//! 🌉️ Rewriting action bridge: folds the shell `{action, args}` vocabulary (camelCase keys, JSON floats, control values, the node-graph host operations array) into the typed `TrinityRewritingCommand`.

//#region 🔖️ActionBridge
use super::TrinityRewritingCommand;
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::{DslValue, Fault, FaultCode, FaultOrigin};

fn invalid(action: &str, detail: impl std::fmt::Display) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("rewriting action '{action}' arguments do not decode: {detail}"))
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
        other => semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(other)),
    })
}

fn required(action: &str, args: Option<&DslValue>, keys: &[&str]) -> Result<String, Fault> {
    text(args, keys).ok_or_else(|| invalid(action, format!("missing {}", keys[0])))
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

pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<TrinityRewritingCommand, Fault> {
    const SURFACE: &[&str] = &["surfaceId", "surface_id"];
    Ok(match action {
        "nodeGraphEdit" => TrinityRewritingCommand::NodeGraphEdit { surface_id: text(args, SURFACE).unwrap_or_default(), operations_json: required(action, args, &["operationsJson", "operations_json", "operations"])? },
        "setLhsJson" => TrinityRewritingCommand::SetLhsJson { value: required(action, args, &["value", "json"])? },
        "setRhsJson" => TrinityRewritingCommand::SetRhsJson { value: required(action, args, &["value", "json"])? },
        "setParameter" => TrinityRewritingCommand::SetParameter { name: required(action, args, &["name"])?, value: text(args, &["value"]).unwrap_or_default() },
        "addRuleClause" => TrinityRewritingCommand::AddRuleClause { kind: required(action, args, &["kind", "value"])? },
        "resetRule" => TrinityRewritingCommand::ResetRule,
        "setActiveExample" => TrinityRewritingCommand::SetActiveExample { example_id: required(action, args, &["exampleId", "example_id", "value", "id"])? },
        "patchNodes" => TrinityRewritingCommand::PatchNodes { node_ids: ids(args), field: text(args, &["field"]).unwrap_or_else(|| "name".into()), value: required(action, args, &["value"])? },
        "nodeGraphViewport" => TrinityRewritingCommand::SetViewport { surface_id: text(args, SURFACE), viewport: viewport(action, args)? },
        "reorganize" => TrinityRewritingCommand::Reorganize,
        "setLodMode" => TrinityRewritingCommand::SetLodMode { value: required(action, args, &["value", "mode"])? },
        "addWorkingNode" => {
            let coordinate = |key: &str| field(args, &[key]).and_then(DslValue::as_f64).ok_or_else(|| invalid(action, format!("missing numeric {key}")));
            TrinityRewritingCommand::AddWorkingNode { kind: text(args, &["kind"]), name: text(args, &["name"]), x: coordinate("x")?, y: coordinate("y")? }
        }
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the trinity rewriting editor has no command for action '{action}'"))),
    })
}
//#endregion 🔖️ActionBridge
