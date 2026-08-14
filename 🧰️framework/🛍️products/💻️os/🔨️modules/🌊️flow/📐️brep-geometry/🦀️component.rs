//! 📐️ Flow brep geometry session — in-process kernel side APIs.
//!
//! Hosts (procedural3d, playbook, flow wasm exports) call these without depending on the
//! packaged brep operator extension. Operator crates import the same session so handles match
//! when linked into one native/wasm image.

//! 🔷️ Flow brep module: native geometry operators.

use base64::Engine;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{block_on, Brep, BrepKernel, GeometryHandle, GeometryKind};
use semio_framework_3d::engine::{ParamDomain, PointClassification, Vec3};
use neural_engine::{channel_output, Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType};
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock, RwLock};

pub static KERNEL: OnceLock<RwLock<Box<dyn BrepKernel + Send + Sync>>> = OnceLock::new();
pub static MESH_CACHE: OnceLock<Mutex<HashMap<(String, u64), semio_framework::MeshData>>> = OnceLock::new();

pub fn kernel() -> &'static RwLock<Box<dyn BrepKernel + Send + Sync>> {
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
pub fn with_kernel<T>(f: impl FnOnce(&mut dyn BrepKernel) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut guard = kernel().write().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    f(&mut **guard)
}

/// 🔓️ Read-only kernel access — lets concurrent queries (tessellate, volume, closest-point, …)
/// proceed in parallel with each other while still serializing against mutating operations.
pub fn with_kernel_read<T>(f: impl FnOnce(&dyn BrepKernel) -> Result<T, EvalError>) -> Result<T, EvalError> {
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

pub fn geometry_dict(kernel: &dyn BrepKernel, handle: &GeometryHandle) -> Result<Dictionary, EvalError> {
    let kind = block_on(kernel.kind(handle)).map_err(map_kernel_error)?;
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

pub fn read_xyz_dict(dict: &Dictionary) -> Result<Vec3, EvalError> {
    Ok([
        dict.get("x").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
        dict.get("y").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
        dict.get("z").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()).unwrap_or(0.0),
    ])
}

pub fn read_xyz(input: &Dictionary, key: &str) -> Result<Vec3, EvalError> {
    let dict = input.get(key).and_then(|value| value.as_dictionary()).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    read_xyz_dict(dict)
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
            read_xyz_dict(dict)
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
                    read_xyz_dict(dict)
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

pub fn wire_from_points(kernel: &mut dyn BrepKernel, points: &[Vec3]) -> Result<GeometryHandle, EvalError> {
    if points.len() >= 2 {
        block_on(kernel.polyline_wire(points)).map_err(map_kernel_error)
    } else if let Some(point) = points.first() {
        block_on(kernel.vertex(*point)).map_err(map_kernel_error)
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
            let topology = block_on(kernel.deconstruct(&shape)).map_err(map_kernel_error)?;
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
#[derive(Debug, thiserror::Error)]
pub enum BrepModuleError {
    #[error("brep kernel lock poisoned")]
    LockPoisoned,
    #[error(transparent)]
    Kernel(#[from] semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::BrepError),
    #[error(transparent)]
    Codec(#[from] neural_engine::EvalError),
    #[error("{0}")]
    Mesh(String),
    #[error("unsupported solid export format: {0}")]
    UnsupportedExportFormat(String),
    #[error("unsupported solid import format: {0}")]
    UnsupportedImportFormat(String),
}
// #endregion ⚠️ Errors

// #region 🔖️Tessellation
/// 🧹️ Retains only geometry handles referenced by the current evaluation outputs.
pub fn retain_geometry_handles(live: &[String]) {
    let live_set: HashSet<String> = live.iter().cloned().collect();
    if let Ok(mut guard) = kernel().write() {
        block_on(guard.retain(&live_set));
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
        block_on(guard.tessellate(&geometry, tolerance)).map_err(|error| error.to_string())?
    };
    let data = semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::mesh_data_from_mesh_transfer(&mesh);
    if let Ok(mut cache) = mesh_cache().lock() {
        cache.insert(key, data.clone());
    }
    Ok(data)
}

pub fn tessellate_geometry_json_for_wasm(handle: &str, tolerance: f64) -> String {
    match tessellate_geometry(handle, tolerance) {
        Ok(mesh) => serde_json::to_string(&mesh).unwrap_or_else(|_| serde_json::json!({ "error": "mesh encode failed" }).to_string()),
        Err(error) => serde_json::json!({ "error": error }).to_string(),
    }
}

/// 🗑️ Disposes a geometry handle owned by the in-process brep kernel.
pub fn dispose_geometry(handle: &str) {
    evict_mesh_cache_for_handle(handle);
    if let Ok(mut kernel) = kernel().write() {
        block_on(kernel.dispose(&GeometryHandle(handle.to_string())));
    }
}
// #endregion 🔖️Tessellation

// #region 🔖️MediaExport
/// 📤️ Exports geometry handles owned by the in-process brep kernel to a solid/mesh interchange format. STEP/OBJ/STL go through the kernel's native codecs; GLB bridges through tessellation (`tessellate` → merged `MeshData` → `GlbExporter`) since the kernel has no native GLB writer wired here. Binary formats are base64-encoded. Returns `{"data","binary","format"}` or `{"error"}` JSON.
pub fn export_solid_json(handles: &[String], format: &str, deflection: f64) -> String {
    let shapes: Vec<GeometryHandle> = handles.iter().cloned().map(GeometryHandle).collect();
    let outcome: Result<(String, bool), BrepModuleError> = kernel().read().map_err(|_| BrepModuleError::LockPoisoned).and_then(|guard| {
        let guard = &**guard;
        match format {
            "step" => block_on(guard.export_step(&shapes)).map(|text| (text, false)).map_err(BrepModuleError::from),
            "obj" => block_on(guard.export_obj(&shapes, deflection)).map(|text| (text, false)).map_err(BrepModuleError::from),
            "stl" => block_on(guard.export_stl(&shapes, deflection)).map(|data| (encode_base64(&data), true)).map_err(BrepModuleError::from),
            "glb" => export_glb_via_tessellation(guard, &shapes, deflection).map(|data| (encode_base64(&data), true)),
            other => Err(BrepModuleError::UnsupportedExportFormat(other.to_string())),
        }
    });
    match outcome {
        Ok((data, binary)) => serde_json::json!({ "data": data, "binary": binary, "format": format }).to_string(),
        Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
    }
}

/// 🧊️ Bridges GLB export through mesh tessellation: tessellates every shape, merges the resulting triangle soup into one `MeshData`, and encodes it with the shared `GlbExporter` mesh codec (the same codec every other app uses for GLB).
pub fn export_glb_via_tessellation(kernel: &dyn BrepKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepModuleError> {
    use semio_framework::MeshExporter;
    let mut merged = semio_framework::MeshData::default();
    for shape in shapes {
        let transfer = block_on(kernel.tessellate(shape, deflection))?;
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
            "step" => block_on(guard.import_step(data)).map(|handles| handles.into_iter().map(|handle| handle.0).collect()).map_err(BrepModuleError::from),
            "obj" => block_on(guard.import_obj(data, tolerance)).map(|handle| vec![handle.0]).map_err(BrepModuleError::from),
            "stl" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| block_on(guard.import_stl(&bytes, tolerance)).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)),
            "glb" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| import_glb_via_tessellation(guard, &bytes, tolerance)),
            other => Err(BrepModuleError::UnsupportedImportFormat(other.to_string())),
        }
    });
    match outcome {
        Ok(handles) => serde_json::json!({ "handles": handles }).to_string(),
        Err(error) => serde_json::json!({ "error": error.to_string() }).to_string(),
    }
}

/// 🧊️ Bridges GLB import through the mesh codec: decodes GLB bytes to `MeshData` via `GlbImporter`, re-encodes it as OBJ text, and ingests that through the kernel's own OBJ importer.
pub fn import_glb_via_tessellation(kernel: &mut dyn BrepKernel, bytes: &[u8], tolerance: f64) -> Result<Vec<String>, BrepModuleError> {
    use semio_framework::MeshImporter;
    let mesh = semio_framework::GlbImporter.import(bytes).map_err(BrepModuleError::Mesh)?;
    let obj_text = semio_framework::mesh_to_obj(&mesh, "glb-import");
    block_on(kernel.import_obj(&obj_text, tolerance)).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)
}
// #endregion 🔖️MediaExport
