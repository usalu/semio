//! 📐️ Flow brep geometry session — in-process kernel side APIs.
//!
//! Hosts (procedural3d, playbook, flow wasm exports) call these without depending on the
//! packaged brep operator extension. Operator crates import the same session so handles match
//! when linked into one native/wasm image.

//! 🔷️ Flow brep module: native geometry operators.

use base64::Engine;
use neural_engine::{Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock, RwLock};

// 🔀️ dedyn-fw-os-misc, O1/R11 case 3: `BrepKernel` has exactly one impl (`Brep`, in `🗄️stdio`) —
// every `dyn BrepKernel` call site was already handing this module a concrete `Brep`, so the trait
// object was a no-op coercion, not a real seam. Deleting it also clears an existing O1 violation:
// every `BrepKernel` method is `async fn`, which is not dyn-compatible (E0038) — `dyn BrepKernel`
// could not have compiled as-is.
pub static KERNEL: OnceLock<RwLock<Box<Brep>>> = OnceLock::new();
pub static MESH_CACHE: OnceLock<Mutex<HashMap<(String, u64), semio_framework::MeshData>>> = OnceLock::new();

pub fn kernel() -> &'static RwLock<Box<Brep>> {
    KERNEL.get_or_init(|| RwLock::new(Box::new(Brep::new())))
}

pub fn mesh_cache() -> &'static Mutex<HashMap<(String, u64), semio_framework::MeshData>> {
    MESH_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn evict_mesh_cache_for_handles(handles: &[String]) {
    if handles.is_empty() {
        if let Ok(mut cache) = mesh_cache().lock() {
            cache.clear();
        }
        return;
    }
    let live: HashSet<&str> = handles.iter().map(String::as_str).collect();
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.retain(|(handle, _), _| live.contains(handle.as_str()));
    }
}

pub fn evict_mesh_cache_for_handle(handle: &str) {
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.retain(|(cached_handle, _), _| cached_handle != handle);
    }
}

// #region 🔖️Helpers
pub fn with_kernel<T>(f: impl FnOnce(&mut Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = kernel().write().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    f(&mut **guard)
}

/// 🔓️ Read-only kernel access — lets concurrent queries (tessellate, volume, closest-point, …)
/// proceed in parallel with each other while still serializing against mutating operations.
pub fn with_kernel_read<T>(f: impl FnOnce(&Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let guard = kernel().read().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    f(&**guard)
}

pub fn kind_label(kind: GeometryKind) -> &'static str {
    match kind {
        GeometryKind::Vertex => "vertex",
        GeometryKind::Edge => "edge",
        GeometryKind::Wire => "wire",
        GeometryKind::Face => "face",
        GeometryKind::Shell => "shell",
        GeometryKind::Solid => "solid",
        GeometryKind::Compound => "compound",
        GeometryKind::Curve => "curve",
        GeometryKind::Surface => "surface",
    }
}

pub fn geometry_dict(kernel: &Brep, handle: &GeometryHandle) -> Result<Dictionary, EvalError> {
    let kind = kernel.kind(handle).map_err(map_kernel_error)?;
    Ok(Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))).insert("kind", Value::Atom(Atom::String(kind_label(kind).into()))))
}

pub fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

pub fn point_dictionary(point: Vec3) -> Dictionary {
    Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(point[0]))).insert("y", Value::Atom(Atom::Decimal(point[1]))).insert("z", Value::Atom(Atom::Decimal(point[2])))
}

pub fn vector_channel(id: &str, operator_id: &str, default: Vec3) -> ChannelSpec {
    ChannelSpec::requires(id, &["math.vector", operator_id]).with_default(Value::Dictionary(vector_dictionary(default)))
}

pub fn vector_dictionary(vector: Vec3) -> Dictionary {
    Dictionary::with_schema("vector").insert("x", Value::Atom(Atom::Decimal(vector[0]))).insert("y", Value::Atom(Atom::Decimal(vector[1]))).insert("z", Value::Atom(Atom::Decimal(vector[2])))
}

pub fn text_dictionary(value: impl Into<String>) -> Dictionary {
    Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(value.into())))
}

pub fn read_channel_number(input: &Dictionary, key: &str) -> Result<f64, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()).ok_or_else(|| EvalError::MissingInput(key.into()))
}

pub fn read_text(input: &Dictionary, key: &str) -> Result<String, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    dict.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(str::to_string).ok_or_else(|| EvalError::MissingInput(key.into()))
}

pub fn read_geometry(input: &Dictionary, key: &str) -> Result<GeometryHandle, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let handle = dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).ok_or_else(|| EvalError::MissingInput(format!("{key}.handle")))?;
    Ok(GeometryHandle(handle.to_string()))
}

pub fn read_optional_geometry(input: &Dictionary, key: &str) -> Option<GeometryHandle> {
    input.get(key).and_then(|value| value.as_dictionary()).and_then(|dict| dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|handle| GeometryHandle(handle.to_string())))
}

/// 🚫️ Requires all three axes present and numeric — a missing/malformed `x`/`y`/`z` is a real
/// caller error, never a silent `0.0` (audit §13.2: silent defaults hide bad input as valid
/// geometry). `label` names the offending field in the error.
pub fn read_xyz_dict(dict: &Dictionary, label: &str) -> Result<Vec3, EvalError> {
    let axis = |name: &str| dict.get(name).and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).ok_or_else(|| EvalError::MissingInput(format!("{label}.{name}")));
    Ok([axis("x")?, axis("y")?, axis("z")?])
}

pub fn read_xyz(input: &Dictionary, key: &str) -> Result<Vec3, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    read_xyz_dict(dict, key)
}

pub fn read_list(input: &Dictionary, key: &str) -> Result<Dictionary, EvalError> {
    input.get(key).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).cloned().ok_or_else(|| EvalError::MissingInput(key.into()))
}

pub fn list_indices(list: &Dictionary) -> Vec<usize> {
    let mut indices: Vec<usize> = list.keys().filter_map(|key| key.parse::<usize>().ok()).collect();
    indices.sort_unstable();
    indices
}

pub fn read_point_list(input: &Dictionary, key: &str) -> Result<Vec<Vec3>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point")))?;
            read_xyz_dict(dict, &format!("{key}[{index}]"))
        })
        .collect()
}

pub fn read_geometry_list(input: &Dictionary, key: &str) -> Result<Vec<GeometryHandle>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let dict = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be geometry")))?;
            dict.get("handle").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()).map(|handle| GeometryHandle(handle.to_string())).ok_or_else(|| EvalError::MissingInput(format!("{key}[{index}].handle")))
        })
        .collect()
}

/// 🕳️ Like [`read_geometry_list`] but treats a genuinely ABSENT `key` as an empty list — for
/// optional list inputs only. A present-but-malformed value (wrong schema, a non-geometry entry)
/// still propagates its `EvalError` instead of silently becoming empty, unlike a bare
/// `.unwrap_or_default()` on the strict reader would (audit §13.2).
pub fn read_geometry_list_or_empty(input: &Dictionary, key: &str) -> Result<Vec<GeometryHandle>, EvalError> {
    if input.get(key).is_none() {
        return Ok(Vec::new());
    }
    read_geometry_list(input, key)
}

pub fn read_nested_point_lists(input: &Dictionary, key: &str) -> Result<Vec<Vec<Vec3>>, EvalError> {
    let list = read_list(input, key)?;
    list_indices(&list)
        .into_iter()
        .map(|index| {
            let sub = list.get(&index.to_string()).and_then(|value| value.as_dictionary()).filter(|dict| dict.schema() == Some("list")).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}] must be a point list")))?;
            list_indices(sub)
                .into_iter()
                .map(|sub_index| {
                    let dict = sub.get(&sub_index.to_string()).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::InvalidInput(format!("{key}[{index}][{sub_index}] must be a point")))?;
                    read_xyz_dict(dict, &format!("{key}[{index}][{sub_index}]"))
                })
                .collect()
        })
        .collect()
}

pub fn points_to_grid(points: &[Vec3], rows: usize) -> Result<Vec<Vec<Vec3>>, EvalError> {
    if rows == 0 {
        return Err(EvalError::InvalidInput("rows must be positive".into()));
    }
    if !points.len().is_multiple_of(rows) {
        return Err(EvalError::InvalidInput("points length must divide evenly by rows".into()));
    }
    let cols = points.len() / rows;
    Ok((0..rows).map(|row| (0..cols).map(|col| points[row * cols + col]).collect()).collect())
}

pub fn wire_from_points(kernel: &mut Brep, points: &[Vec3]) -> Result<GeometryHandle, EvalError> {
    if points.len() >= 2 {
        kernel.polyline_wire(points).map_err(map_kernel_error)
    } else if let Some(point) = points.first() {
        kernel.vertex(*point).map_err(map_kernel_error)
    } else {
        Err(EvalError::InvalidInput("no intersection".into()))
    }
}

pub fn domain_span(domain: ParamDomain) -> f64 {
    domain.max - domain.min
}

pub fn classify_number(classification: PointClassification) -> f64 {
    match classification {
        PointClassification::Inside => 0.0,
        PointClassification::Outside => 1.0,
        PointClassification::OnBoundary => 2.0,
    }
}

pub fn decode_base64(text: &str) -> Result<Vec<u8>, EvalError> {
    base64::engine::general_purpose::STANDARD.decode(text.trim()).map_err(|error| EvalError::InvalidInput(format!("invalid base64: {error}")))
}

pub fn encode_base64(data: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(data)
}

pub fn map_kernel_error(error: semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::BrepError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

pub fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

pub fn geometry_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id])
}

pub fn list_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::list(id, &[operator_id])
}

pub fn point_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id])
}

pub fn out_solid(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Sld", "solid", full_name)
}

pub fn out_wire(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("W", "Wre", "wire", full_name)
}

pub fn out_curve(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("C", "Crv", "curve", full_name)
}

pub fn out_face(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("F", "Fce", "face", full_name)
}

pub fn out_surface(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Srf", "surface", full_name)
}

pub fn out_geometry(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("G", "Geo", "geometry", full_name)
}

pub fn out_compound(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("C", "Cmp", "compound", full_name)
}

pub fn out_point(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("P", "Pnt", "point", full_name)
}

pub fn out_normal(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("N", "Nrm", "normal", full_name)
}

pub fn out_span() -> ChannelSpec {
    ChannelSpec::named("S", "Spn", "span", "DomainSpan")
}

pub fn out_curvature() -> ChannelSpec {
    ChannelSpec::named("K", "Cur", "curvature", "CurveCurvature")
}

pub fn out_volume() -> ChannelSpec {
    ChannelSpec::named("V", "Vol", "volume", "MeasuredVolume")
}

pub fn out_area() -> ChannelSpec {
    ChannelSpec::named("A", "Are", "area", "MeasuredArea")
}

pub fn out_length() -> ChannelSpec {
    ChannelSpec::named("L", "Len", "length", "MeasuredLength")
}

pub fn out_center() -> ChannelSpec {
    ChannelSpec::named("P", "CoM", "center", "CenterOfMass")
}

pub fn out_box() -> ChannelSpec {
    ChannelSpec::named("B", "Box", "box", "BoundingBox")
}

pub fn out_distance() -> ChannelSpec {
    ChannelSpec::named("D", "Dst", "distance", "MeasuredDistance")
}

pub fn out_classification() -> ChannelSpec {
    ChannelSpec::named("C", "Cls", "classification", "PointClassification")
}

pub fn out_report() -> ChannelSpec {
    ChannelSpec::named("R", "Rpt", "report", "ValidationReport")
}

pub fn out_vertex() -> ChannelSpec {
    ChannelSpec::named("V", "Vtx", "vertex", "Vertex")
}

pub fn out_step() -> ChannelSpec {
    ChannelSpec::named("S", "Stp", "step", "StepExport")
}

pub fn out_stl() -> ChannelSpec {
    ChannelSpec::named("L", "Stl", "stl", "StlExport")
}

pub fn out_obj() -> ChannelSpec {
    ChannelSpec::named("O", "Obj", "obj", "ObjExport")
}

pub fn out_dwg() -> ChannelSpec {
    ChannelSpec::named("D", "Dwg", "dwg", "DwgExport")
}

#[allow(
    clippy::too_many_arguments,
    reason = "positional operator-metadata builder mirroring this file's registration table shape (id/name/abbreviation/icon/summary/inputs/outputs/group columns); ~20 call sites, restructuring into a params struct would only churn call sites with no behavior change"
)]
pub fn operator_info_with_outputs(id: &str, name: &str, abbreviation: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, outputs: Vec<ChannelSpec>, group: &[&str]) -> OperatorInfo {
    OperatorInfo {
        id: id.into(),
        extension: "brep".into(),
        name: name.into(),
        abbreviation: abbreviation.into(),
        icon: icon.into(),
        summary: summary.into(),
        inputs,
        outputs,
        group: group.iter().map(|entry| (*entry).to_string()).collect(),
        ..Default::default()
    }
}

pub fn register_untyped(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operator>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: operation }], produces);
}

pub fn register_typed(registry: &mut Registry, info: OperatorInfo, operation: Box<dyn Operator>, produces: &[&str]) {
    registry.register_operator(info, vec![OperatorImpl { schemas: vec![], operator: operation }], produces);
}

#[allow(clippy::too_many_arguments, reason = "positional geometry-operator registration helper; ~68 call sites forming this file's operator table, restructuring into a params struct would only churn call sites with no behavior change")]
pub fn reg_geo(registry: &mut Registry, id: &str, name: &str, abbr: &str, icon: &str, summary: &str, inputs: Vec<ChannelSpec>, output: ChannelSpec, group: &[&str], operation: Box<dyn Operator>) {
    register_untyped(registry, operator_info_with_outputs(id, name, abbr, icon, summary, inputs, vec![output], group), operation, &["geometry"]);
}

pub fn geometry_schema() -> Schema {
    Schema {
        id: "geometry".into(),
        module: "brep".into(),
        name: "Geometry".into(),
        icon: "emoji:🔷️".into(),
        summary: "Opaque brep geometry handle".into(),
        fields: vec![FieldSpec::new("handle", ValueType::Text), FieldSpec::new("kind", ValueType::Text).with_default(Value::Atom(Atom::String("solid".into())))],
    }
}

pub fn empty_list_value() -> Value {
    Value::Dictionary(Dictionary::with_schema("list"))
}

pub fn topology_element_schema(id: &str, name: &str, icon: &str) -> Schema {
    Schema { id: id.into(), module: "brep".into(), name: name.into(), icon: icon.into(), summary: format!("{name} topology element"), fields: vec![FieldSpec::new("handle", ValueType::Text)] }
}

pub fn brep_schema() -> Schema {
    Schema {
        id: "brep".into(),
        module: "brep".into(),
        name: "Brep".into(),
        icon: "emoji:🧊️".into(),
        summary: "Construct, deconstruct, or modify a brep from vertices, edges, and faces".into(),
        fields: vec![
            FieldSpec::new("vertex", ValueType::List(Box::new(ValueType::Schema("vertex".into())))).with_default(empty_list_value()),
            FieldSpec::new("edge", ValueType::List(Box::new(ValueType::Schema("edge".into())))).with_default(empty_list_value()),
            FieldSpec::new("face", ValueType::List(Box::new(ValueType::Schema("face".into())))).with_default(empty_list_value()),
        ],
    }
}

pub fn topology_list(schema: &str, handles: Vec<GeometryHandle>) -> Dictionary {
    handles
        .into_iter()
        .enumerate()
        .fold(Dictionary::with_schema("list"), |list, (index, handle)| list.insert(index.to_string(), Value::Dictionary(Dictionary::with_schema(schema).insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))))))
}

pub struct BrepDeconstruct;

impl Operator for BrepDeconstruct {
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        with_kernel(|kernel| {
            let shape = read_geometry(input, "brep")?;
            let topology = kernel.deconstruct(&shape).map_err(map_kernel_error)?;
            Ok(Dictionary::new()
                .insert("brep", Value::Dictionary(geometry_dict(kernel, &shape)?))
                .insert("vertex", Value::Dictionary(topology_list("vertex", topology.vertices)))
                .insert("edge", Value::Dictionary(topology_list("edge", topology.edges)))
                .insert("face", Value::Dictionary(topology_list("face", topology.faces)))
                .insert("errors", Value::Dictionary(Dictionary::with_schema("list"))))
        })
    }
}

pub fn topology_output(code: &str, abbreviation: &str, name: &str, schema: &str) -> ChannelSpec {
    ChannelSpec::named(code, abbreviation, name, name).with_operators(vec![schema.to_string()]).with_cardinality(Cardinality::ZeroOrMore)
}

pub fn text_schema() -> Schema {
    Schema { id: "text".into(), module: "brep".into(), name: "Text".into(), icon: "emoji:📝️".into(), summary: "Text payload".into(), fields: vec![FieldSpec::new("value", ValueType::Text)] }
}

// #endregion 🔖️Helpers

// #region ⚠️ Errors
/// 🧯️ Internal error type for the brep module's media import/export bridging helpers (`export_solid_json`/`import_solid_json` still surface it flattened to JSON `{"error"}` strings, matching prior behaviour byte-for-byte).
#[derive(Debug)]
pub enum BrepModuleError {
    LockPoisoned,
    Kernel(semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::BrepError),
    Codec(EvalError),
    Mesh(String),
    UnsupportedExportFormat(String),
    UnsupportedImportFormat(String),
    InvalidArgs(String),
    UnknownMethod(String),
}

impl std::fmt::Display for BrepModuleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LockPoisoned => formatter.write_str("brep kernel lock poisoned"),
            Self::Kernel(error) => std::fmt::Display::fmt(error, formatter),
            Self::Codec(error) => std::fmt::Display::fmt(error, formatter),
            Self::Mesh(detail) => formatter.write_str(detail),
            Self::UnsupportedExportFormat(format) => write!(formatter, "unsupported solid export format: {format}"),
            Self::UnsupportedImportFormat(format) => write!(formatter, "unsupported solid import format: {format}"),
            Self::InvalidArgs(detail) => write!(formatter, "invalid brep_invoke args: {detail}"),
            Self::UnknownMethod(method) => write!(formatter, "unknown brep_invoke method: {method}"),
        }
    }
}

impl std::error::Error for BrepModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Kernel(error) => Some(error),
            Self::Codec(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::BrepError> for BrepModuleError {
    fn from(error: semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::BrepError) -> Self {
        Self::Kernel(error)
    }
}

impl From<EvalError> for BrepModuleError {
    fn from(error: EvalError) -> Self {
        Self::Codec(error)
    }
}
// #endregion ⚠️ Errors

// #region 🔖️Tessellation
/// 🧹️ Retains only geometry handles referenced by the current evaluation outputs.
pub fn retain_geometry_handles(live: &[String]) {
    let live_set: HashSet<String> = live.iter().cloned().collect();
    if let Ok(mut guard) = kernel().write() {
        guard.retain(&live_set);
    }
    evict_mesh_cache_for_handles(live);
}

/// 🧊️ Tessellates a geometry handle owned by the in-process brep kernel into preview `MeshData`.
pub fn tessellate_geometry(handle: &str, tolerance: f64) -> Result<semio_framework::MeshData, String> {
    let key = (handle.to_string(), tolerance.to_bits());
    if let Ok(cache) = mesh_cache().lock() {
        if let Some(cached) = cache.get(&key) {
            return Ok(cached.clone());
        }
    }
    let guard = kernel().read().map_err(|_| "brep kernel lock poisoned".to_string())?;
    let mesh = {
        let geometry = GeometryHandle(handle.to_string());
        guard.tessellate(&geometry, tolerance).map_err(|error| error.to_string())?
    };
    let data = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::mesh_data_from_mesh_transfer(&mesh);
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.insert(key, data.clone());
    }
    Ok(data)
}

pub fn tessellate_geometry_json_for_wasm(handle: &str, tolerance: f64) -> String {
    match tessellate_geometry(handle, tolerance) {
        Ok(mesh) => crate::os_pack::json::to_json_string(&mesh),
        Err(error) => crate::os_pack::json::to_string(&crate::os_pack::json::object([("error".to_string(), crate::os_pack::json::Value::String(error))])),
    }
}

/// 🗑️ Disposes a geometry handle owned by the in-process brep kernel.
pub fn dispose_geometry(handle: &str) {
    evict_mesh_cache_for_handle(handle);
    if let Ok(mut kernel) = kernel().write() {
        kernel.dispose(&GeometryHandle(handle.to_string()));
    }
}
// #endregion 🔖️Tessellation

// #region 🔖️WasmTessellationBridge
/// 🌐️ Direct JS-callable wasm-bindgen exports for the flow-core brep tessellation bridge
/// (see `🧰️framework/🔨️modules/🧊️3d/🟦️.ts`'s `ensureBrepWasmLoaded`), distinct from the
/// byte-oriented `flow_bridge_*` ABI used by the `FlowActionState` job dispatch above.
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; these are direct JS-callable
// wasm-bindgen exports for the browser flow-core bundle, so this is narrowed to exclude the
// WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_bridge {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn tessellate(handle: &str, tolerance: f64) -> String {
        super::tessellate_geometry_json_for_wasm(handle, tolerance)
    }

    #[wasm_bindgen]
    pub fn dispose(handle: &str) {
        super::dispose_geometry(handle);
    }

    /// 🌐️ Generic JSON-RPC bridge for the CAD `SpatialKernel` (see `🧠️semio/🟦️.ts`): dispatches
    /// one `BrepKernel` method by name over JSON args, sharing the same in-process `kernel()` the
    /// `tessellate`/`dispose` exports above use so handles stay valid across calls.
    #[wasm_bindgen]
    pub fn brep_invoke(method: &str, args_json: &str) -> String {
        super::brep_invoke_json(method, args_json)
    }
}
// #endregion 🔖️WasmTessellationBridge

// #region 🔖️MediaExport
/// 📤️ Exports geometry handles owned by the in-process brep kernel to a solid/mesh interchange format. STEP/OBJ/STL go through the kernel's native codecs; GLB bridges through tessellation (`tessellate` → merged `MeshData` → `GlbExporter`) since the kernel has no native GLB writer wired here. Binary formats are base64-encoded. Returns `{"data","binary","format"}` or `{"error"}` JSON.
pub fn export_solid_json(handles: &[String], format: &str, deflection: f64) -> String {
    let shapes: Vec<GeometryHandle> = handles.iter().cloned().map(GeometryHandle).collect();
    let outcome: Result<(String, bool), BrepModuleError> = kernel().read().map_err(|_| BrepModuleError::LockPoisoned).and_then(|guard| {
        let guard = &**guard;
        match format {
            "step" => guard.export_step(&shapes).map(|text| (text, false)).map_err(BrepModuleError::from),
            "obj" => guard.export_obj(&shapes, deflection).map(|text| (text, false)).map_err(BrepModuleError::from),
            "stl" => guard.export_stl(&shapes, deflection).map(|data| (encode_base64(&data), true)).map_err(BrepModuleError::from),
            "glb" => export_glb_via_tessellation(guard, &shapes, deflection).map(|data| (encode_base64(&data), true)),
            other => Err(BrepModuleError::UnsupportedExportFormat(other.to_string())),
        }
    });
    match outcome {
        Ok((data, binary)) => crate::os_pack::json::to_string(&crate::os_pack::json::object([
            ("data".to_string(), crate::os_pack::json::Value::String(data)),
            ("binary".to_string(), crate::os_pack::json::Value::Bool(binary)),
            ("format".to_string(), crate::os_pack::json::Value::String(format.to_string())),
        ])),
        Err(error) => crate::os_pack::json::to_string(&crate::os_pack::json::object([("error".to_string(), crate::os_pack::json::Value::String(error.to_string()))])),
    }
}

/// 🧊️ Bridges GLB export through mesh tessellation: tessellates every shape, merges the resulting triangle soup into one `MeshData`, and encodes it with the shared `GlbExporter` mesh codec (the same codec every other app uses for GLB).
pub fn export_glb_via_tessellation(kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepModuleError> {
    use semio_framework::MeshExporter;
    let mut merged = semio_framework::MeshData::default();
    for shape in shapes {
        let transfer = kernel.tessellate(shape, deflection)?;
        let mesh = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::mesh_data_from_mesh_transfer(&transfer);
        let offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(mesh.positions);
        merged.normals.extend(mesh.normals);
        merged.indices.extend(mesh.indices.into_iter().map(|index| index + offset));
    }
    semio_framework::GlbExporter.export(&merged).map_err(BrepModuleError::Mesh)
}

/// 📥️ Imports STEP/OBJ/STL solid data (or GLB mesh data, bridged through the kernel's OBJ ingestion since it has no raw-mesh entry point) into the in-process kernel. STEP/OBJ expect UTF-8 text in `data`; STL/GLB expect base64-encoded bytes. Returns `{"handles":[...]}` or `{"error"}` JSON.
pub fn import_solid_json(format: &str, data: &str, tolerance: f64) -> String {
    let outcome: Result<Vec<String>, BrepModuleError> = kernel().write().map_err(|_| BrepModuleError::LockPoisoned).and_then(|mut guard| {
        let guard = &mut **guard;
        match format {
            "step" => guard.import_step(data).map(|handles| handles.into_iter().map(|handle| handle.0).collect()).map_err(BrepModuleError::from),
            "obj" => guard.import_obj(data, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from),
            "stl" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| guard.import_stl(&bytes, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)),
            "glb" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| import_glb_via_tessellation(guard, &bytes, tolerance)),
            other => Err(BrepModuleError::UnsupportedImportFormat(other.to_string())),
        }
    });
    match outcome {
        Ok(handles) => crate::os_pack::json::to_string(&crate::os_pack::json::object([("handles".to_string(), crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(&handles)))])),
        Err(error) => crate::os_pack::json::to_string(&crate::os_pack::json::object([("error".to_string(), crate::os_pack::json::Value::String(error.to_string()))])),
    }
}

/// 🧊️ Bridges GLB import through the mesh codec: decodes GLB bytes to `MeshData` via `GlbImporter`, re-encodes it as OBJ text, and ingests that through the kernel's own OBJ importer.
pub fn import_glb_via_tessellation(kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<String>, BrepModuleError> {
    use semio_framework::MeshImporter;
    let mesh = semio_framework::GlbImporter.import(bytes).map_err(BrepModuleError::Mesh)?;
    let obj_text = semio_framework::mesh_to_obj(&mesh, "glb-import");
    kernel.import_obj(&obj_text, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)
}
// #endregion 🔖️MediaExport

// #region 🔖️GenericInvoke
/// 🌉️ `brep_invoke` argument/result JSON shape: `{"error": "..."}` on failure, otherwise one of
/// `{"handle": "..."}` / `{"handles": [...]}` / `{"value": ...}` / a raw `MeshTransfer` object /
/// `{"vertices": [...], "edges": [...], "faces": [...], "shells": [...]}` for `deconstruct`.
fn invoke_args(args_json: &str) -> Result<crate::os_pack::json::Value, BrepModuleError> {
    crate::os_pack::json::parse(args_json).map_err(|error| BrepModuleError::InvalidArgs(error.to_string()))
}

fn arg_f64(args: &crate::os_pack::json::Value, key: &str) -> Result<f64, BrepModuleError> {
    args.get(key).and_then(|value| value.as_f64()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing number {key}")))
}

fn arg_f64_or(args: &crate::os_pack::json::Value, key: &str, fallback: f64) -> f64 {
    args.get(key).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

fn arg_usize(args: &crate::os_pack::json::Value, key: &str) -> Result<usize, BrepModuleError> {
    args.get(key).and_then(|value| value.as_u64()).map(|value| value as usize).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing integer {key}")))
}

fn arg_bool_or(args: &crate::os_pack::json::Value, key: &str, fallback: bool) -> bool {
    args.get(key).and_then(|value| value.as_bool()).unwrap_or(fallback)
}

fn arg_string(args: &crate::os_pack::json::Value, key: &str) -> Result<String, BrepModuleError> {
    args.get(key).and_then(|value| value.as_str()).map(str::to_string).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing string {key}")))
}

fn value_vec3(value: &crate::os_pack::json::Value) -> Result<Vec3, BrepModuleError> {
    let items = value.as_array().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()))?;
    if items.len() != 3 {
        return Err(BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    }
    let axis = |index: usize| items[index].as_f64().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    Ok([axis(0)?, axis(1)?, axis(2)?])
}

fn arg_vec3(args: &crate::os_pack::json::Value, key: &str) -> Result<Vec3, BrepModuleError> {
    let value = args.get(key).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point {key}")))?;
    value_vec3(value)
}

fn arg_points(args: &crate::os_pack::json::Value, key: &str) -> Result<Vec<Vec3>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point array {key}")))?;
    items.iter().map(value_vec3).collect()
}

fn arg_handle(args: &crate::os_pack::json::Value, key: &str) -> Result<GeometryHandle, BrepModuleError> {
    arg_string(args, key).map(GeometryHandle)
}

fn arg_handles(args: &crate::os_pack::json::Value, key: &str) -> Result<Vec<GeometryHandle>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing handle array {key}")))?;
    items.iter().map(|item| item.as_str().map(|text| GeometryHandle(text.to_string())).ok_or_else(|| BrepModuleError::InvalidArgs(format!("{key} entries must be strings")))).collect()
}

fn handle_result(handle: GeometryHandle) -> crate::os_pack::json::Value {
    crate::os_pack::json::object([("handle".to_string(), crate::os_pack::json::Value::String(handle.0))])
}

fn handles_result(handles: Vec<GeometryHandle>) -> crate::os_pack::json::Value {
    crate::os_pack::json::object([("handles".to_string(), crate::os_pack::json::array(handles.into_iter().map(|handle| crate::os_pack::json::Value::String(handle.0))))])
}

fn number_result(value: f64) -> crate::os_pack::json::Value {
    crate::os_pack::json::object([("value".to_string(), crate::os_pack::json::Value::Number(crate::os_pack::json::Number::Float(value)))])
}

fn vec3_result(value: Vec3) -> crate::os_pack::json::Value {
    crate::os_pack::json::object([(
        "value".to_string(),
        crate::os_pack::json::array(value.into_iter().map(crate::os_pack::json::Number::Float).map(crate::os_pack::json::Value::Number)),
    )])
}

fn string_result(value: String) -> crate::os_pack::json::Value {
    crate::os_pack::json::object([("value".to_string(), crate::os_pack::json::Value::String(value))])
}

fn unit_result() -> crate::os_pack::json::Value {
    crate::os_pack::json::object([])
}

fn topology_result(topology: semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::BrepTopology) -> crate::os_pack::json::Value {
    let handle_array = |handles: Vec<GeometryHandle>| crate::os_pack::json::array(handles.into_iter().map(|handle| crate::os_pack::json::Value::String(handle.0)));
    crate::os_pack::json::object([
        ("vertices".to_string(), handle_array(topology.vertices)),
        ("edges".to_string(), handle_array(topology.edges)),
        ("faces".to_string(), handle_array(topology.faces)),
        ("shells".to_string(), handle_array(topology.shells)),
    ])
}

fn mesh_result(mesh: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::MeshTransfer) -> crate::os_pack::json::Value {
    crate::os_pack::json::from_dsl_value(&crate::os_dsl::ToValue::to_value(mesh))
}

/// 🌉️ Dispatches one `BrepKernel` method by name over `os_pack::json` args (see `handle_result`
/// and friends above for the response shapes); the sole bridge every `SemioBrepKernel` TS method
/// (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts`) calls into.
fn brep_invoke_inner(method: &str, args_json: &str) -> Result<crate::os_pack::json::Value, BrepModuleError> {
    let args = invoke_args(args_json)?;
    match method {
        "box" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.box_prim(arg_f64(&args, "width")?, arg_f64(&args, "depth")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sphere" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sphere_prim(arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cylinder" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cylinder_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cone" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cone_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "lineCurve" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.line_curve(arg_vec3(&args, "start")?, arg_vec3(&args, "end")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "circleCurve" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.circle_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "arcCurve" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .arc_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?, arg_f64(&args, "startAngle")?, arg_f64(&args, "endAngle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "ellipseCurve" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .ellipse_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "semiMajor")?, arg_f64(&args, "semiMinor")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "interpolateCurve" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.interpolate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "approximateCurve" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .approximate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?, arg_usize(&args, "controlPoints")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "polylineWire" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.polyline_wire(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rectangleWire" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.rectangle_wire(arg_f64(&args, "width")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromPoints" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.planar_face_from_points(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromWire" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.planar_face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrudeWire" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.extrude_wire(&arg_handle(&args, "wire")?, arg_vec3(&args, "vector")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrude" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.extrude(&arg_handle(&args, "face")?, arg_vec3(&args, "direction")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "revolve" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .revolve(&arg_handle(&args, "face")?, arg_vec3(&args, "axisOrigin")?, arg_vec3(&args, "axisDirection")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "loft" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.loft(&arg_handles(&args, "profiles")?, arg_bool_or(&args, "smooth", false)).map(handle_result).map_err(BrepModuleError::from)
        }
        "sweep" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sweep(&arg_handle(&args, "profile")?, &arg_handle(&args, "path")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "thickenFace" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.thicken_face(&arg_handle(&args, "face")?, arg_f64(&args, "thickness")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "offsetFace" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.offset_face(&arg_handle(&args, "face")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "fuse" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.fuse(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cut" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cut(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "intersect" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.intersect(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sewFaces" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sew_faces(&arg_handles(&args, "faces")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "faceFromWire" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "healSolid" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.heal_solid(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "volume" => {
            let guard = kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.volume(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "area" => {
            let guard = kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.area(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "length" => {
            let guard = kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.length(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "centerOfMass" => {
            let guard = kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.center_of_mass(&arg_handle(&args, "shape")?).map(vec3_result).map_err(BrepModuleError::from)
        }
        "distance" => {
            let guard = kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.distance(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(number_result).map_err(BrepModuleError::from)
        }
        "deconstruct" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.deconstruct(&arg_handle(&args, "shape")?).map(topology_result).map_err(BrepModuleError::from)
        }
        "tessellate" => {
            let guard = kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.tessellate(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-3)).map(|mesh| mesh_result(&mesh)).map_err(BrepModuleError::from)
        }
        "exportStep" => {
            let guard = kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.export_step(&arg_handles(&args, "shapes")?).map(string_result).map_err(BrepModuleError::from)
        }
        "importStep" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.import_step(&arg_string(&args, "data")?).map(handles_result).map_err(BrepModuleError::from)
        }
        "dispose" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.dispose(&arg_handle(&args, "handle")?);
            Ok(unit_result())
        }
        "retain" => {
            let mut guard = kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            let live: HashSet<String> = arg_handles(&args, "handles")?.into_iter().map(|handle| handle.0).collect();
            guard.retain(&live);
            Ok(unit_result())
        }
        other => Err(BrepModuleError::UnknownMethod(other.to_string())),
    }
}

/// 🌐️ `brep_invoke` implementation shared by the wasm export and native callers/tests: returns
/// the result JSON or `{"error": "..."}` — never panics on malformed input.
pub fn brep_invoke_json(method: &str, args_json: &str) -> String {
    match brep_invoke_inner(method, args_json) {
        Ok(value) => crate::os_pack::json::to_string(&value),
        Err(error) => crate::os_pack::json::to_string(&crate::os_pack::json::object([("error".to_string(), crate::os_pack::json::Value::String(error.to_string()))])),
    }
}
// #endregion 🔖️GenericInvoke
