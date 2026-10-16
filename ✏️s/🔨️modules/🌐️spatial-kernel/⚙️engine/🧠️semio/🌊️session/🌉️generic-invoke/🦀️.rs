//! 🌉️ Generic invoke: the `brep_invoke` argument readers and result writers.

use super::*;

// #region 🔖️GenericInvoke
/// 🌉️ `brep_invoke` argument/result JSON shape: `{"error": "..."}` on failure, otherwise one of
/// `{"handle": "..."}` / `{"handles": [...]}` / `{"value": ...}` / a raw `MeshTransfer` object /
/// `{"vertices": [...], "edges": [...], "faces": [...], "shells": [...]}` for `deconstruct`.
pub(super) fn invoke_args(args_json: &str) -> Result<semio_framework_pack_json::Value, BrepModuleError> {
    semio_framework_pack_json::parse(args_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| BrepModuleError::InvalidArgs(error.to_string()))
}

pub(super) fn arg_f64(args: &semio_framework_pack_json::Value, key: &str) -> Result<f64, BrepModuleError> {
    args.get(key).and_then(|value| value.as_f64()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing number {key}")))
}

pub(super) fn arg_f64_or(args: &semio_framework_pack_json::Value, key: &str, fallback: f64) -> f64 {
    args.get(key).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

pub(super) fn arg_usize(args: &semio_framework_pack_json::Value, key: &str) -> Result<usize, BrepModuleError> {
    args.get(key).and_then(|value| value.as_u64()).map(|value| value as usize).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing integer {key}")))
}

pub(super) fn arg_bool_or(args: &semio_framework_pack_json::Value, key: &str, fallback: bool) -> bool {
    args.get(key).and_then(|value| value.as_bool()).unwrap_or(fallback)
}

fn arg_string(args: &semio_framework_pack_json::Value, key: &str) -> Result<String, BrepModuleError> {
    args.get(key).and_then(|value| value.as_str()).map(str::to_string).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing string {key}")))
}

fn value_vec3(value: &semio_framework_pack_json::Value) -> Result<Vec3, BrepModuleError> {
    let items = value.as_array().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()))?;
    if items.len() != 3 {
        return Err(BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    }
    let axis = |index: usize| items[index].as_f64().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    Ok([axis(0)?, axis(1)?, axis(2)?])
}

pub(super) fn arg_vec3(args: &semio_framework_pack_json::Value, key: &str) -> Result<Vec3, BrepModuleError> {
    let value = args.get(key).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point {key}")))?;
    value_vec3(value)
}

pub(super) fn arg_points(args: &semio_framework_pack_json::Value, key: &str) -> Result<Vec<Vec3>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point array {key}")))?;
    items.iter().map(value_vec3).collect()
}

pub(super) fn arg_handle(args: &semio_framework_pack_json::Value, key: &str) -> Result<GeometryHandle, BrepModuleError> {
    arg_string(args, key).map(GeometryHandle)
}

pub(super) fn arg_handles(args: &semio_framework_pack_json::Value, key: &str) -> Result<Vec<GeometryHandle>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing handle array {key}")))?;
    items.iter().map(|item| item.as_str().map(|text| GeometryHandle(text.to_string())).ok_or_else(|| BrepModuleError::InvalidArgs(format!("{key} entries must be strings")))).collect()
}

pub(super) fn handle_result(handle: GeometryHandle) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("handle".to_string(), semio_framework_pack_json::Value::String(handle.0))])
}

fn handles_result(handles: Vec<GeometryHandle>) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("handles".to_string(), semio_framework_pack_json::array(handles.into_iter().map(|handle| semio_framework_pack_json::Value::String(handle.0))))])
}

pub(super) fn number_result(value: f64) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("value".to_string(), semio_framework_pack_json::Value::Number(semio_framework_pack_json::Number::Float(value)))])
}

pub(super) fn vec3_result(value: Vec3) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([(
        "value".to_string(),
        semio_framework_pack_json::array(value.into_iter().map(semio_framework_pack_json::Number::Float).map(semio_framework_pack_json::Value::Number)),
    )])
}

fn string_result(value: String) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("value".to_string(), semio_framework_pack_json::Value::String(value))])
}

pub(super) fn unit_result() -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([])
}

pub(super) fn topology_result(topology: semio_framework_3d_brep_engine::BrepTopology) -> semio_framework_pack_json::Value {
    let handle_array = |handles: Vec<GeometryHandle>| semio_framework_pack_json::array(handles.into_iter().map(|handle| semio_framework_pack_json::Value::String(handle.0)));
    semio_framework_pack_json::object([
        ("vertices".to_string(), handle_array(topology.vertices)),
        ("edges".to_string(), handle_array(topology.edges)),
        ("faces".to_string(), handle_array(topology.faces)),
        ("shells".to_string(), handle_array(topology.shells)),
    ])
}

pub(super) fn mesh_result(mesh: &semio_framework_3d_brep_representation::contract::MeshTransfer) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(mesh))
}

// #endregion 🔖️GenericInvoke
