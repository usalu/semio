//! 🌐️ First-party Semio geometry session with explicit instance ownership.
use semio_framework_os_flow::mesh::*;
use neural_engine::{Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, ValueType, VALUE_TYPE_GEOMETRY, VALUE_TYPE_NUMBER, VALUE_TYPE_POINT, VALUE_TYPE_VECTOR};
use semio_framework_3d::brep::engine::{Brep, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3};
use semio_framework_3d::brep::queries::tessellation::{TessellationJob, TessellationStep};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use semio_framework_3d::brep::engine::retirement::{PayloadRetirement, NativeRetirementStep, RetirementFrontier};
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex, RwLock};

// 🔀️ dedyn-fw-os-misc, O1/R11 case 3: `BrepKernel` has exactly one impl (`Brep`, in `🗄️stdio`) —
// every `dyn BrepKernel` call site was already handing this module a concrete `Brep`, so the trait
// object was a no-op coercion, not a real seam. Deleting it also clears an existing O1 violation:
// every `BrepKernel` method is `async fn`, which is not dyn-compatible (E0038) — `dyn BrepKernel`
// could not have compiled as-is.





// #region 🔖️Helpers

/// 🔓️ Read-only kernel access — lets concurrent queries (tessellate, volume, closest-point, …)
/// proceed in parallel with each other while still serializing against mutating operations.

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
    let kind = kernel.kind(handle).map_err(|error| map_kernel_error(&error))?;
    Ok(Dictionary::with_schema("geometry").insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))).insert("kind", Value::Atom(Atom::String(kind_label(kind).into()))))
}

pub fn number_dictionary(value: f64) -> Dictionary {
    Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value)))
}

pub fn point_dictionary(point: Vec3) -> Dictionary {
    Dictionary::with_schema("point").insert("x", Value::Atom(Atom::Decimal(point[0]))).insert("y", Value::Atom(Atom::Decimal(point[1]))).insert("z", Value::Atom(Atom::Decimal(point[2])))
}

/// 🧭️ A three-axis input. `read_xyz` only asks for `x`/`y`/`z`, so a `point` satisfies a `vector`
/// channel and the declaration says so — the accepted set, not one nominal type, is what the
/// port-compatibility oracle intersects.
pub fn vector_channel(id: &str, operator_id: &str, default: Vec3) -> ChannelSpec {
    ChannelSpec::requires(id, &["math.vector", operator_id]).with_value_types(&[VALUE_TYPE_VECTOR, VALUE_TYPE_POINT]).with_default(Value::Dictionary(vector_dictionary(default)))
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
        kernel.polyline_wire(points).map_err(|error| map_kernel_error(&error))
    } else if let Some(point) = points.first() {
        kernel.vertex(*point).map_err(|error| map_kernel_error(&error))
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



pub fn map_kernel_error(error: &semio_framework_3d::brep::engine::BrepError) -> EvalError {
    EvalError::InvalidInput(error.to_string())
}

pub fn number_channel(id: &str, operator_id: &str, default: f64) -> ChannelSpec {
    ChannelSpec::number_default(id, default, &[operator_id])
}

/// 🔷️ A brep kernel handle input — `read_geometry` reads `handle` off a `geometry` dictionary, so
/// every wire, face, surface, solid and compound is the SAME port type.
pub fn geometry_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id]).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn list_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::list(id, &[operator_id])
}

/// 📍️ A three-axis input read through `read_xyz` — see [`vector_channel`] for why both schemas count.
pub fn point_channel(id: &str, operator_id: &str) -> ChannelSpec {
    ChannelSpec::requires(id, &[operator_id]).with_value_types(&[VALUE_TYPE_POINT, VALUE_TYPE_VECTOR])
}

pub fn out_solid(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Sld", "solid", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_wire(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("W", "Wre", "wire", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_curve(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("C", "Crv", "curve", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_face(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("F", "Fce", "face", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

/// 📤️ The `face` an operator MAKES, for the operators that are also GIVEN a `face` — one operator's
/// input ids and output ids are disjoint, because `"{nodeId}@{portId}"` is the only public name a
/// wire endpoint has.
///
/// @see `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs` — `produced_channel_id`
pub fn out_face_result(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("F", "Fce", neural_engine::produced_channel_id("face"), full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_surface(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("S", "Srf", "surface", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_geometry(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("G", "Geo", "geometry", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

/// 📤️ The `geometry` an operator MAKES — see [`out_face_result`].
pub fn out_geometry_result(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("G", "Geo", neural_engine::produced_channel_id("geometry"), full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_compound(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("C", "Cmp", "compound", full_name).with_value_types(&[VALUE_TYPE_GEOMETRY])
}

pub fn out_point(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("P", "Pnt", "point", full_name).with_value_types(&[VALUE_TYPE_POINT])
}

/// 📤️ The `point` an operator FINDS, for the operators that are also GIVEN a `point` — see [`out_face_result`].
pub fn out_point_result(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("P", "Pnt", neural_engine::produced_channel_id("point"), full_name).with_value_types(&[VALUE_TYPE_POINT])
}

pub fn out_normal(full_name: &str) -> ChannelSpec {
    ChannelSpec::named("N", "Nrm", "normal", full_name).with_value_types(&[VALUE_TYPE_VECTOR])
}

pub fn out_span() -> ChannelSpec {
    ChannelSpec::named("S", "Spn", "span", "DomainSpan").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_curvature() -> ChannelSpec {
    ChannelSpec::named("K", "Cur", "curvature", "CurveCurvature").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_volume() -> ChannelSpec {
    ChannelSpec::named("V", "Vol", "volume", "MeasuredVolume").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_area() -> ChannelSpec {
    ChannelSpec::named("A", "Are", "area", "MeasuredArea").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_length() -> ChannelSpec {
    ChannelSpec::named("L", "Len", "length", "MeasuredLength").with_value_types(&[VALUE_TYPE_NUMBER])
}

pub fn out_center() -> ChannelSpec {
    ChannelSpec::named("P", "CoM", "center", "CenterOfMass").with_value_types(&["point"])
}

pub fn out_box() -> ChannelSpec {
    ChannelSpec::named("B", "Box", "box", "BoundingBox").with_value_types(&["geometry"])
}

pub fn out_distance() -> ChannelSpec {
    ChannelSpec::named("D", "Dst", "distance", "MeasuredDistance").with_value_types(&["number"])
}

pub fn out_classification() -> ChannelSpec {
    ChannelSpec::named("C", "Cls", "classification", "PointClassification").with_value_types(&["number"])
}

pub fn out_report() -> ChannelSpec {
    ChannelSpec::named("R", "Rpt", "report", "ValidationReport").with_value_types(&["text"])
}

pub fn out_vertex() -> ChannelSpec {
    ChannelSpec::named("V", "Vtx", "vertex", "Vertex").with_value_types(&["geometry"])
}


pub fn out_stl() -> ChannelSpec {
    ChannelSpec::named("L", "Stl", "stl", "StlExport").with_value_types(&["text"])
}

pub fn out_obj() -> ChannelSpec {
    ChannelSpec::named("O", "Obj", "obj", "ObjExport").with_value_types(&["text"])
}


/// 🪪️ Expands only ambiguous channel shorthand to its full semantic identifier.
fn distinct_channels(mut channels: Vec<ChannelSpec>) -> Vec<ChannelSpec> {
    let codes: Vec<_> = channels.iter().map(|channel| channel.code.clone()).collect();
    let abbreviations: Vec<_> = channels.iter().map(|channel| channel.abbreviation.clone()).collect();
    for channel in &mut channels {
        if codes.iter().filter(|code| **code == channel.code).count() > 1 { channel.code = channel.name.to_uppercase(); }
        if abbreviations.iter().filter(|abbreviation| **abbreviation == channel.abbreviation).count() > 1 { channel.abbreviation = channel.name.clone(); }
    }
    channels
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
        inputs: distinct_channels(inputs),
        outputs: distinct_channels(outputs),
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

pub struct BrepDeconstruct(pub SessionCapture);

impl Operator for BrepDeconstruct {
    fn retirement_is_empty(&self) -> bool { self.0.terminal_is_empty() }
    fn retire_step(&mut self, items:usize, bytes:usize, _: &mut neural_engine::ValueRetirement) -> Result<neural_engine::ValueRetirementStep,&'static str> {
        self.0.close_step(items,bytes).map_err(|_| "brep.geometry-capture-retirement-failed")
    }
    fn retire_cold(mut self:Box<Self>) { self.0.retire_cold(); }
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let shape = read_geometry(input, "brep")?;
            let topology = kernel.deconstruct(&shape).map_err(|error| map_kernel_error(&error))?;
            Ok(Dictionary::new()
                .insert(neural_engine::produced_channel_id("brep"), Value::Dictionary(geometry_dict(kernel, &shape)?))
                .insert("vertex", Value::Dictionary(topology_list("vertex", topology.vertices)))
                .insert("edge", Value::Dictionary(topology_list("edge", topology.edges)))
                .insert("face", Value::Dictionary(topology_list("face", topology.faces)))
                .insert("errors", Value::Dictionary(Dictionary::with_schema("list"))))
        })
    }
}

pub fn topology_output(code: &str, abbreviation: &str, name: &str, schema: &str) -> ChannelSpec {
    ChannelSpec::named(code, abbreviation, name, name).with_operators(vec![schema.to_string()]).with_value_types(&["list"]).with_cardinality(Cardinality::ZeroOrMore)
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
    Kernel(semio_framework_3d::brep::engine::BrepError),
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

impl From<semio_framework_3d::brep::engine::BrepError> for BrepModuleError {
    fn from(error: semio_framework_3d::brep::engine::BrepError) -> Self {
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

/// 🩺️ One blocking finding from the pre-tessellation validate gate, in the typed shape the preview
/// status object carries — never a re-parsed prose string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewDiagnostic {
    pub entity: String,
    pub code: String,
    pub message: String,
}

/// ⏱️ What one budgeted [`tessellate_step`] call achieved.
#[derive(Clone, Debug, PartialEq)]
pub enum TessellationStepOutcome {
    /// 🔁 Budget spent, work remains — step again next turn.
    Working { units_done: usize, units_total: usize, faces_done: usize, faces_total: usize, phase: &'static str },
    /// ✅ The mesh is ready (freshly tessellated, or served from the LOD cache).
    Ready { mesh: semio_framework::MeshData, units_total: usize, faces_total: usize },
    /// 🛑 A cancel retired the job; nothing is produced and the slot is free.
    Cancelled,
    /// 🩺️ The validate gate rejected the topology before any triangle was produced.
    Invalid { issues: Vec<PreviewDiagnostic> },
    /// 💥 The kernel refused the handle or a unit faulted.
    Failed { message: String },
}

/// ⏱️ Retained resumable tessellations keyed by `(handle, tolerance bits)`. Bounded: a new job past
/// the ceiling evicts the least recently stepped one rather than growing without limit.
const TESSELLATION_JOB_CAPACITY: usize = 32;

struct RetainedTessellation {
    job: TessellationJob,
    last_step: u64,
}

#[derive(Default)]
struct TessellationJobRegistry {
    jobs: ManuallyDrop<BTreeMap<(String, u64), RetainedTessellation>>,
    clock: u64,
}



/// 🧹️ Drops retained tessellations whose handle is no longer live. An empty `live` clears them all.

/// 🛑️ Retires the in-flight tessellation of `handle` at `tolerance`. Returns true when a job was
/// actually retired — a cancel for an already-finished or never-started job is a no-op, not a fault.

/// 🛑️ Retires every in-flight tessellation — the explicit "cancel preview evaluation" gesture.

/// 📈️ Progress of the in-flight tessellation of `handle` at `tolerance`, if one is retained.

/// 🎚️ The cached mesh for `handle` at `tolerance` OR at any FINER tolerance already computed — a
/// finer mesh is a valid, better-than-requested answer, so switching the LOD mode from fine to
/// coarse serves the existing mesh instead of retessellating.

/// ⏱️ Advances the tessellation of `handle` at `tolerance` by at most `budget` units and reports
/// what happened. This is the ONLY tessellation entry point that respects an interactive step
/// ceiling: it never runs more than `budget` face/edge units before returning, so a host can drive
/// it from inside its own maintenance budget, paint progress, and cancel it.
///
/// The validate gate runs once, on first admission of a `(handle, tolerance)` pair — a solid with a
/// blocking (`code` not prefixed `warning-`) validation issue never reaches the tessellator.

/// 🧊️ Tessellates a geometry handle owned by the in-process brep kernel into preview `MeshData`.
/// Unbudgeted convenience over [`tessellate_step`] for callers with no interactive ceiling (export
/// bridges, schema tests) — the incremental path is still the only algorithm underneath.

/// 🌐️ One budgeted tessellate ROUND TRIP as the extension-boundary JSON envelope: progress/phase
/// always, plus one base64 `pack` mesh-body chunk per continuation once the mesh is ready. `chunk`
/// selects which chunk to ship; `chunks` tells the caller how many there are in total.
///
/// ⏱️ `budget` bounds ONE step in face/edge units — the granularity at which a cancel can land —
/// and `wall_micros` bounds the whole round trip in wall time: the call keeps stepping while the
/// job is still working AND the deadline has not passed. Both bounds are needed because units are
/// not time: the cheapest step measured on this kernel is 29 µs and the dearest 5.9 s, so a unit
/// budget alone either wastes a whole interactive round trip on microseconds of work or overruns
/// any ceiling on a single face (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).

// 🚫️async: E1 pure codec helper (no I/O), consumed from sync envelope call sites — see R9
fn failed_envelope_json(code: &str, message: &str) -> String {
    use semio_framework_os_flow::os_pack::json::{object, Value};
    semio_framework_os_flow::os_pack::json::to_string(&object([
        ("done".to_string(), Value::Bool(true)),
        ("cancellable".to_string(), Value::Bool(false)),
        ("phase".to_string(), Value::String("failed".to_string())),
        ("error".to_string(), Value::String(message.to_string())),
        ("errorCode".to_string(), Value::String(code.to_string())),
    ]))
}


/// 🗑️ Disposes a geometry handle owned by the in-process brep kernel.
// #endregion 🔖️Tessellation





// #region 🔖️MediaExport
/// 📤️ Exports geometry handles owned by the in-process brep kernel to a solid/mesh interchange format. STEP/OBJ/STL go through the kernel's native codecs; GLB bridges through tessellation (`tessellate` → merged `MeshData` → `GlbExporter`) since the kernel has no native GLB writer wired here. Binary formats are base64-encoded. Returns `{"data","binary","format"}` or `{"error"}` JSON.

/// 🧊️ Bridges GLB export through mesh tessellation: tessellates every shape, merges the resulting triangle soup into one `MeshData`, and encodes it with the shared `GlbExporter` mesh codec (the same codec every other app uses for GLB).
pub fn export_glb_via_tessellation(kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepModuleError> {
    use semio_framework::MeshExporter;
    let mut merged = semio_framework::MeshData::default();
    for shape in shapes {
        let transfer = kernel.tessellate(shape, deflection)?;
        let mesh = semio_framework_3d::brep::engine::mesh_data_from_mesh_transfer(&transfer);
        let offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(mesh.positions);
        merged.normals.extend(mesh.normals);
        merged.indices.extend(mesh.indices.into_iter().map(|index| index + offset));
    }
    semio_framework::GlbExporter.export(&merged).map_err(BrepModuleError::Mesh)
}

/// 📥️ Imports STEP/OBJ/STL solid data (or GLB mesh data, bridged through the kernel's OBJ ingestion since it has no raw-mesh entry point) into the in-process kernel. STEP/OBJ expect UTF-8 text in `data`; STL/GLB expect base64-encoded bytes. Returns `{"handles":[...]}` or `{"error"}` JSON.

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
fn invoke_args(args_json: &str) -> Result<semio_framework_os_flow::os_pack::json::Value, BrepModuleError> {
    semio_framework_os_flow::os_pack::json::parse(args_json).map_err(|error| BrepModuleError::InvalidArgs(error.to_string()))
}

fn arg_f64(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<f64, BrepModuleError> {
    args.get(key).and_then(|value| value.as_f64()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing number {key}")))
}

fn arg_f64_or(args: &semio_framework_os_flow::os_pack::json::Value, key: &str, fallback: f64) -> f64 {
    args.get(key).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

fn arg_usize(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<usize, BrepModuleError> {
    args.get(key).and_then(|value| value.as_u64()).map(|value| value as usize).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing integer {key}")))
}

fn arg_bool_or(args: &semio_framework_os_flow::os_pack::json::Value, key: &str, fallback: bool) -> bool {
    args.get(key).and_then(|value| value.as_bool()).unwrap_or(fallback)
}

fn arg_string(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<String, BrepModuleError> {
    args.get(key).and_then(|value| value.as_str()).map(str::to_string).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing string {key}")))
}

fn value_vec3(value: &semio_framework_os_flow::os_pack::json::Value) -> Result<Vec3, BrepModuleError> {
    let items = value.as_array().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()))?;
    if items.len() != 3 {
        return Err(BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    }
    let axis = |index: usize| items[index].as_f64().ok_or_else(|| BrepModuleError::InvalidArgs("expected a 3-number array".to_string()));
    Ok([axis(0)?, axis(1)?, axis(2)?])
}

fn arg_vec3(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<Vec3, BrepModuleError> {
    let value = args.get(key).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point {key}")))?;
    value_vec3(value)
}

fn arg_points(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<Vec<Vec3>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point array {key}")))?;
    items.iter().map(value_vec3).collect()
}

fn arg_handle(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<GeometryHandle, BrepModuleError> {
    arg_string(args, key).map(GeometryHandle)
}

fn arg_handles(args: &semio_framework_os_flow::os_pack::json::Value, key: &str) -> Result<Vec<GeometryHandle>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing handle array {key}")))?;
    items.iter().map(|item| item.as_str().map(|text| GeometryHandle(text.to_string())).ok_or_else(|| BrepModuleError::InvalidArgs(format!("{key} entries must be strings")))).collect()
}

fn handle_result(handle: GeometryHandle) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("handle".to_string(), semio_framework_os_flow::os_pack::json::Value::String(handle.0))])
}

fn handles_result(handles: Vec<GeometryHandle>) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("handles".to_string(), semio_framework_os_flow::os_pack::json::array(handles.into_iter().map(|handle| semio_framework_os_flow::os_pack::json::Value::String(handle.0))))])
}

fn number_result(value: f64) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("value".to_string(), semio_framework_os_flow::os_pack::json::Value::Number(semio_framework_os_flow::os_pack::json::Number::Float(value)))])
}

fn vec3_result(value: Vec3) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([(
        "value".to_string(),
        semio_framework_os_flow::os_pack::json::array(value.into_iter().map(semio_framework_os_flow::os_pack::json::Number::Float).map(semio_framework_os_flow::os_pack::json::Value::Number)),
    )])
}

fn string_result(value: String) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([("value".to_string(), semio_framework_os_flow::os_pack::json::Value::String(value))])
}

fn unit_result() -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::object([])
}

fn topology_result(topology: semio_framework_3d::brep::engine::BrepTopology) -> semio_framework_os_flow::os_pack::json::Value {
    let handle_array = |handles: Vec<GeometryHandle>| semio_framework_os_flow::os_pack::json::array(handles.into_iter().map(|handle| semio_framework_os_flow::os_pack::json::Value::String(handle.0)));
    semio_framework_os_flow::os_pack::json::object([
        ("vertices".to_string(), handle_array(topology.vertices)),
        ("edges".to_string(), handle_array(topology.edges)),
        ("faces".to_string(), handle_array(topology.faces)),
        ("shells".to_string(), handle_array(topology.shells)),
    ])
}

fn mesh_result(mesh: &semio_framework_3d::brep::engine::MeshTransfer) -> semio_framework_os_flow::os_pack::json::Value {
    semio_framework_os_flow::os_pack::json::from_dsl_value(&semio_framework_os_flow::os_dsl::ToValue::to_value(mesh))
}

/// 🌉️ Dispatches one `BrepKernel` method by name over `os_pack::json` args (see `handle_result`
/// and friends above for the response shapes); the sole bridge every `SemioBrepKernel` TS method
/// (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts`) calls into. Every arm is declared, argument for
/// argument, by the verb catalog `🔣️.json` beside this file (schema `🧬️schema/🔣️.json`); the bridge law
/// `🌊️flow/🧪️tests/📐️brep-invoke` holds the two to each other.

/// 🌐️ `brep_invoke` implementation shared by the wasm export and native callers/tests: returns
/// the result JSON or `{"error": "..."}` — never panics on malformed input.
// #endregion 🔖️GenericInvoke

/// 🧾️ Records real heap layouts; cloned handles clear the synchronous consuming-call receipt.
#[derive(Default)]
struct ShellAllocator {
    layout_bytes: std::sync::atomic::AtomicUsize,
    receipt: std::sync::atomic::AtomicPtr<std::sync::atomic::AtomicUsize>,
}
impl Clone for ShellAllocator {
    fn clone(&self) -> Self { Self { layout_bytes:std::sync::atomic::AtomicUsize::new(self.layout_bytes.load(std::sync::atomic::Ordering::Relaxed)),receipt:std::sync::atomic::AtomicPtr::default() } }
}
unsafe impl std::alloc::Allocator for ShellAllocator {
    fn allocate(&self,layout:std::alloc::Layout) -> Result<std::ptr::NonNull<[u8]>,std::alloc::AllocError> {
        let allocation = std::alloc::Allocator::allocate(&std::alloc::Global,layout)?;
        self.layout_bytes.store(layout.size(),std::sync::atomic::Ordering::Relaxed);
        Ok(allocation)
    }
    unsafe fn deallocate(&self,pointer:std::ptr::NonNull<u8>,layout:std::alloc::Layout) {
        unsafe { std::alloc::Allocator::deallocate(&std::alloc::Global,pointer,layout); }
        let receipt = self.receipt.load(std::sync::atomic::Ordering::Relaxed);
        if !receipt.is_null() { unsafe { (*receipt).fetch_add(layout.size(),std::sync::atomic::Ordering::Relaxed); } }
    }
}
type ShellArc<T> = Arc<T,ShellAllocator>;
fn shell<T>(value:T) -> ShellArc<T> { Arc::new_in(value,ShellAllocator::default()) }
fn shell_bytes<T>(owner:&ShellArc<T>) -> usize { Arc::allocator(owner).layout_bytes.load(std::sync::atomic::Ordering::Relaxed) }
fn record_shell<T>(owner:&ShellArc<T>,receipt:&std::sync::atomic::AtomicUsize) { Arc::allocator(owner).receipt.store(std::ptr::from_ref(receipt).cast_mut(),std::sync::atomic::Ordering::Relaxed); }

#[derive(Clone)]
pub struct Session { state: ShellArc<SessionState> }

/// 🎟️ Owns one exact authority reader until reference release or real final-family retirement.
#[must_use = "session captures require explicit retirement"]
pub struct SessionCapture { session: ManuallyDrop<Option<Session>>, exclusive: bool }
impl std::ops::Deref for SessionCapture {
    type Target = Session;
    fn deref(&self) -> &Session { self.session.as_ref().expect("open geometry capture") }
}
impl SessionCapture {
    pub fn terminal_is_empty(&self) -> bool { self.session.is_none() }
    /// 🧾️ Returns actual possible allocator-layout bytes for the next shallow shell release.
    pub fn shell_byte_requirement(&self) -> usize {
        let Some(session) = self.session.as_ref() else { return 0 };
        let state = &session.state;
        shell_bytes(state) + if self.exclusive { shell_bytes(&state.kernel) + shell_bytes(&state.mesh_cache) + shell_bytes(&state.claims) + shell_bytes(&state.next_authority) + shell_bytes(&state.retirement) } else { 0 }
    }
    /// 🧹️ Releases a non-final reader atomically; the final reader retains the actual family cursor.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<neural_engine::ValueRetirementStep,String> {
        use neural_engine::ValueRetirementStep as Step;
        if self.terminal_is_empty() { return Ok(Step::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if !self.exclusive {
            if maximum_bytes < self.shell_byte_requirement() { return Ok(Step::Blocked); }
            let receipt = std::sync::atomic::AtomicUsize::new(0);
            let Session { state } = self.session.take().expect("owned geometry capture");
            record_shell(&state,&receipt);
            match Arc::into_inner(state) {
                None => {},
                Some(state) => { *self.session = Some(Session { state:shell(state) }); self.exclusive = true; }
            }
            return Ok(Step::Pending { released_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed) });
        }
        let session = self.session.as_ref().expect("exclusive geometry capture");
        if session.terminal_is_empty() {
            if maximum_bytes < self.shell_byte_requirement() { return Ok(Step::Blocked); }
            let receipt = std::sync::atomic::AtomicUsize::new(0);
            let Session { state } = self.session.take().expect("terminal geometry capture");
            record_shell(&state,&receipt);
            if let Some(state) = Arc::into_inner(state) {
                record_shell(&state.kernel,&receipt); record_shell(&state.mesh_cache,&receipt);
                record_shell(&state.claims,&receipt); record_shell(&state.next_authority,&receipt); record_shell(&state.retirement,&receipt);
                drop(state);
            }
            return Ok(Step::Pending { released_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed) });
        }
        Ok(match session.close_step(maximum_items,maximum_bytes)? {
            Step::Complete => Step::Pending { released_items:0,released_bytes:0 },
            step => step,
        })
    }
    /// 🧊️ Drains a capture at an explicit cold boundary; retained registries supply individual grants.
    pub fn retire_cold(&mut self) {
        while !self.terminal_is_empty() {
            let step = self.close_step(1,4096.max(self.shell_byte_requirement())).expect("cold geometry capture retirement");
            assert!(!matches!(step,neural_engine::ValueRetirementStep::Blocked),"cold geometry capture is paused");
            if matches!(step,neural_engine::ValueRetirementStep::Pending { released_items:0,released_bytes:0 }) {
                assert!(self.session.as_ref().is_some_and(Session::terminal_is_empty),"cold geometry capture waits on a live family authority");
            }
        }
    }
}
impl Drop for SessionCapture {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(),"geometry capture requires explicit retirement before drop"); } }
}

/// 🔌️ Owner-supplied geometry operations; the retained session owns all authority and retirement.
pub trait GeometryOperations: Send + Sync {
    fn export(&self, _: &Brep, format: &str, _: &[GeometryHandle], _: f64) -> Result<(String,bool),BrepModuleError> { Err(BrepModuleError::UnsupportedExportFormat(format.into())) }
    fn import(&self, _: &mut Brep, format: &str, _: &str, _: f64) -> Result<Vec<GeometryHandle>,BrepModuleError> { Err(BrepModuleError::UnsupportedImportFormat(format.into())) }
    fn invoke(&self, _: &mut Brep, method: &str, _: &semio_framework_os_flow::os_pack::json::Value) -> Result<semio_framework_os_flow::os_pack::json::Value,BrepModuleError> { Err(BrepModuleError::UnknownMethod(method.into())) }
}
impl GeometryOperations for () {}

struct SessionState {
    operations: &'static dyn GeometryOperations,
    kernel: ShellArc<RwLock<ManuallyDrop<Brep>>>,
    mesh_cache: ShellArc<Mutex<ManuallyDrop<BTreeMap<(String, u64), semio_framework::MeshData>>>>,
    claims: ShellArc<Mutex<BTreeMap<u64, BTreeSet<String>>>>,
    next_authority: ShellArc<std::sync::atomic::AtomicU64>,
    authority: u64,
    closed: std::sync::atomic::AtomicBool,
    jobs: Mutex<TessellationJobRegistry>,
    retirement: ShellArc<Mutex<SessionRetirement>>,
}
#[derive(Default)]
struct SessionRetirement { payloads:PayloadRetirement, extracted:bool, paused:bool }
struct RetiredClaims(BTreeSet<String>);
impl RetirementFrontier for RetiredClaims {
    fn advance(&mut self,payloads:&mut PayloadRetirement) -> bool { if let Some(handle) = self.0.pop_first() { payloads.text(handle); } self.0.is_empty() }
}
struct RetiredJobs(BTreeMap<(String,u64),RetainedTessellation>);
impl RetirementFrontier for RetiredJobs {
    fn advance(&mut self,payloads:&mut PayloadRetirement) -> bool {
        if let Some(((handle,_),retained)) = self.0.pop_first() { payloads.text(handle); retained.job.detach_retirement(payloads); }
        self.0.is_empty()
    }
}
struct RetiredMeshes(BTreeMap<(String,u64),semio_framework::MeshData>);
impl RetirementFrontier for RetiredMeshes {
    fn advance(&mut self,payloads:&mut PayloadRetirement) -> bool {
        if let Some(((handle,_),mesh)) = self.0.pop_first() {
            payloads.text(handle); payloads.pod(mesh.positions); payloads.pod(mesh.normals); payloads.pod(mesh.colors); payloads.pod(mesh.indices); payloads.pod(mesh.uvs); payloads.pod(mesh.face_ids); payloads.pod(mesh.vertex_ids); payloads.pod(mesh.edge_positions); payloads.pod(mesh.edge_ids); payloads.pod(mesh.edge_uvs); payloads.pod(mesh.edge_is_seam);
            if let Some(texture) = mesh.paint_texture_base64 { payloads.text(texture); }
        }
        self.0.is_empty()
    }
}
impl Drop for SessionState {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(self.closed.load(std::sync::atomic::Ordering::Acquire),"geometry authority requires explicit close before drop");
            assert!(self.jobs.get_mut().expect("geometry jobs").jobs.is_empty(),"geometry jobs require explicit retirement transfer");
        }
    }
}
impl Drop for SessionRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.extracted && self.payloads.terminal_is_empty(),"geometry family requires terminal-empty retirement before drop"); } }
}
impl Default for Session { fn default() -> Self { Self::new() } }
impl Session {
    /// 🔗️ Captures this exact authority for a supplied operator or retained composition reader.
    pub fn capture(&self) -> SessionCapture { SessionCapture { session:ManuallyDrop::new(Some(self.clone())),exclusive:false } }
    /// 🔌️ Supplies one separately admitted retained-job authority over this kernel family.
    pub fn port(&self) -> Box<dyn semio_framework_os_flow::geometry::GeometryPort> {
        let mut claims = self.state.claims.lock().expect("geometry claims");
        let authority = self.state.next_authority.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let closed = self.is_closed();
        if !closed { claims.insert(authority,BTreeSet::new()); }
        let authority = Session { state: shell(SessionState {
            operations:self.state.operations, kernel: self.state.kernel.clone(), mesh_cache: self.state.mesh_cache.clone(),
            claims: self.state.claims.clone(), next_authority: self.state.next_authority.clone(), authority,
            closed: std::sync::atomic::AtomicBool::new(closed), jobs: Mutex::new(TessellationJobRegistry::default()), retirement:self.state.retirement.clone(),
        }) };
        Box::new(SessionPort { authority:ManuallyDrop::new(Some(authority)),anchor:self.capture() })
    }
    /// 🚪️ Seals this authority and transfers its claims and jobs to persistent retirement.
    pub fn begin_close(&self) {
        let mut claims = self.state.claims.lock().expect("geometry claims");
        if self.state.closed.swap(true, std::sync::atomic::Ordering::AcqRel) { return; }
        let own_claims = claims.remove(&self.state.authority).unwrap_or_default();
        let jobs = std::mem::take(&mut *self.state.jobs.lock().expect("geometry jobs").jobs);
        let mut retirement = self.state.retirement.lock().expect("geometry retirement");
        retirement.payloads.frontier(RetiredClaims(own_claims));
        retirement.payloads.frontier(RetiredJobs(jobs));
    }
    /// 🛑️ Pauses retirement without reopening authority or dropping owned resources.
    pub fn cancel_close(&self) { self.state.retirement.lock().expect("geometry retirement").paused = true; }
    /// ▶️ Resumes the retained close cursor after cancellation.
    pub fn resume_close(&self) { self.state.retirement.lock().expect("geometry retirement").paused = false; }
    /// 🎟️ Drains one structural frontier and at most the supplied allocation-byte credit.
    pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> Result<neural_engine::ValueRetirementStep,String> {
        use neural_engine::ValueRetirementStep as Step;
        {
            let retirement = self.state.retirement.lock().map_err(|_| "geometry retirement lock poisoned")?;
            if retirement.extracted && retirement.payloads.terminal_is_empty() { return Ok(Step::Complete); }
            if maximum_items == 0 || maximum_bytes == 0 || retirement.paused { return Ok(Step::Blocked); }
        }
        self.begin_close();
        let claims = self.state.claims.lock().map_err(|_| "geometry claims lock poisoned")?;
        let mut retirement = self.state.retirement.lock().map_err(|_| "geometry retirement lock poisoned")?;
        if retirement.extracted && retirement.payloads.terminal_is_empty() { return Ok(Step::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 || retirement.paused { return Ok(Step::Blocked); }
        if !retirement.extracted && claims.is_empty() {
            let Ok(mut kernel) = self.state.kernel.try_write() else { return Ok(Step::Pending { released_items:0,released_bytes:0 }) };
            let Ok(mut cache) = self.state.mesh_cache.try_lock() else { return Ok(Step::Pending { released_items:0,released_bytes:0 }) };
            kernel.detach_retirement(&mut retirement.payloads);
            retirement.payloads.frontier(RetiredMeshes(std::mem::take(&mut **cache)));
            retirement.extracted = true;
            return Ok(Step::Pending { released_items:1,released_bytes:0 });
        }
        Ok(match retirement.payloads.close_step(maximum_items,maximum_bytes) {
            NativeRetirementStep::Blocked => Step::Blocked,
            NativeRetirementStep::Pending { released_items,released_bytes } => Step::Pending { released_items,released_bytes },
            NativeRetirementStep::Complete => Step::Pending { released_items:0,released_bytes:0 },
        })
    }
    pub fn terminal_is_empty(&self) -> bool {
        let retirement = self.state.retirement.lock().expect("geometry retirement");
        self.is_closed() && retirement.extracted && retirement.payloads.terminal_is_empty()
    }
    /// 🧊️ Synchronous authority release for cold callers; retained owners use close_step.
    pub fn close(&self) {
        self.begin_close();
        loop {
            match self.close_step(1,4096) {
                Ok(neural_engine::ValueRetirementStep::Pending { released_items:0,released_bytes:0 }) | Ok(neural_engine::ValueRetirementStep::Complete) | Ok(neural_engine::ValueRetirementStep::Blocked) | Err(_) => return,
                Ok(neural_engine::ValueRetirementStep::Pending { .. }) => {},
            }
        }
    }
    pub fn is_closed(&self) -> bool { self.state.closed.load(std::sync::atomic::Ordering::Acquire) }
    pub fn new() -> Self { Self::with_operations(&()) }
    /// 🔌️ Admits an explicit static owner operation table without retaining extra payload state.
    pub fn with_operations(operations: &'static dyn GeometryOperations) -> Self {
        Self { state: shell(SessionState { operations, kernel: shell(RwLock::new(ManuallyDrop::new(Brep::new()))), mesh_cache: shell(Mutex::new(ManuallyDrop::new(BTreeMap::new()))), claims: shell(Mutex::new(BTreeMap::from([(1,BTreeSet::new())]))), next_authority: shell(std::sync::atomic::AtomicU64::new(2)), authority:1,closed:std::sync::atomic::AtomicBool::new(false),jobs:Mutex::new(TessellationJobRegistry::default()),retirement:shell(Mutex::new(SessionRetirement::default())) }) }
    }
fn kernel(&self) -> &RwLock<ManuallyDrop<Brep>> { &self.state.kernel }
fn mesh_cache(&self) -> &Mutex<ManuallyDrop<BTreeMap<(String, u64), semio_framework::MeshData>>> { &self.state.mesh_cache }
pub fn evict_mesh_cache_for_handles(&self, handles: &[String]) {
    if self.is_closed() { return; }
    if handles.is_empty() {
        if let Ok(mut cache) = self.mesh_cache().lock() {
            cache.clear();
        }
        return;
    }
    let live: HashSet<&str> = handles.iter().map(String::as_str).collect();
    if let Ok(mut cache) = self.mesh_cache().lock() {
        cache.retain(|(handle, _), _| live.contains(handle.as_str()));
    }
}
pub fn evict_mesh_cache_for_handle(&self, handle: &str) {
    if self.is_closed() { return; }
    if let Ok(mut cache) = self.mesh_cache().lock() {
        cache.retain(|(cached_handle, _), _| cached_handle != handle);
    }
}
pub fn with_kernel<T>(&self, f: impl FnOnce(&mut Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut claims = self.state.claims.lock().map_err(|_| EvalError::InvalidInput("geometry claims lock poisoned".into()))?;
    let mut guard = self.kernel().write().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    let before = guard.live_handles();
    let result = f(&mut guard);
    claims.entry(self.state.authority).or_default().extend(guard.live_handles().difference(&before).cloned());
    result
}
pub fn with_kernel_read<T>(&self, f: impl FnOnce(&Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let guard = self.kernel().read().map_err(|_| EvalError::InvalidInput("brep kernel lock poisoned".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    f(&guard)
}
pub fn retain_geometry_handles(&self, live: &[String]) {
    let mut claims = self.state.claims.lock().expect("geometry claims");
    if self.is_closed() { return; }
    claims.insert(self.state.authority, live.iter().cloned().collect());
    let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
    if let Ok(mut guard) = self.kernel().write() { guard.retain(&merged); }
    self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
    self.retain_tessellation_jobs(live);
}
fn tessellation_jobs(&self) -> &Mutex<TessellationJobRegistry> { &self.state.jobs }
pub fn retain_tessellation_jobs(&self, live: &[String]) {
    if self.is_closed() { return; }
    let Ok(mut registry) = self.tessellation_jobs().lock() else { return };
    if live.is_empty() {
        registry.jobs.clear();
        return;
    }
    let live_set: HashSet<&str> = live.iter().map(String::as_str).collect();
    registry.jobs.retain(|(handle, _), _| live_set.contains(handle.as_str()));
}
pub fn cancel_tessellation(&self, handle: &str, tolerance: f64) -> bool {
    let Ok(mut registry) = self.tessellation_jobs().lock() else { return false };
    match registry.jobs.remove(&(handle.to_string(), tolerance.to_bits())) {
        Some(mut retained) => {
            retained.job.cancel();
            true
        }
        None => false,
    }
}
pub fn cancel_all_tessellations(&self) -> usize {
    let Ok(mut registry) = self.tessellation_jobs().lock() else { return 0 };
    let count = registry.jobs.len();
    for (_, retained) in registry.jobs.iter_mut() {
        retained.job.cancel();
    }
    registry.jobs.clear();
    count
}
pub fn tessellation_progress(&self, handle: &str, tolerance: f64) -> Option<(usize, usize, &'static str)> {
    let registry = self.tessellation_jobs().lock().ok()?;
    let retained = registry.jobs.get(&(handle.to_string(), tolerance.to_bits()))?;
    let progress = retained.job.progress();
    Some((progress.units_done, progress.units_total, progress.phase.tag()))
}
pub fn cached_mesh_at_or_finer(&self, handle: &str, tolerance: f64) -> Option<semio_framework::MeshData> {
    if self.is_closed() { return None; }
    let cache = self.mesh_cache().lock().ok()?;
    if let Some(exact) = cache.get(&(handle.to_string(), tolerance.to_bits())) {
        return Some(exact.clone());
    }
    let mut best: Option<(f64, &semio_framework::MeshData)> = None;
    for ((cached_handle, bits), mesh) in cache.iter() {
        if cached_handle != handle {
            continue;
        }
        let cached_tolerance = f64::from_bits(*bits);
        if cached_tolerance > tolerance {
            continue;
        }
        if best.is_none_or(|(current, _)| cached_tolerance > current) {
            best = Some((cached_tolerance, mesh));
        }
    }
    best.map(|(_, mesh)| mesh.clone())
}
pub fn tessellate_step(&self, handle: &str, tolerance: f64, budget: usize) -> TessellationStepOutcome {
    if self.is_closed() { return TessellationStepOutcome::Failed { message: "geometry.session-closed".into() }; }
    let mut claims = self.state.claims.lock().expect("geometry claims");
    if self.is_closed() { return TessellationStepOutcome::Failed { message: "geometry.session-closed".into() }; }
    claims.entry(self.state.authority).or_default().insert(handle.into());
    if let Some(mesh) = self.cached_mesh_at_or_finer(handle, tolerance) {
        return TessellationStepOutcome::Ready { units_total: 0, faces_total: 0, mesh };
    }
    let key = (handle.to_string(), tolerance.to_bits());
    let Ok(guard) = self.kernel().read() else {
        return TessellationStepOutcome::Failed { message: "brep kernel lock poisoned".to_string() };
    };
    let geometry = GeometryHandle(handle.to_string());
    let Ok(mut registry) = self.tessellation_jobs().lock() else {
        return TessellationStepOutcome::Failed { message: "tessellation job registry lock poisoned".to_string() };
    };
    if !registry.jobs.contains_key(&key) {
        if let Err(issues) = guard.validate_gate_sync(&geometry) {
            return TessellationStepOutcome::Invalid { issues: issues.into_iter().map(|issue| PreviewDiagnostic { entity: issue.entity, code: issue.code.to_string(), message: issue.message }).collect() };
        }
        let job = match guard.tessellate_job_sync(&geometry, tolerance) {
            Ok(job) => job,
            Err(error) => return TessellationStepOutcome::Failed { message: error.to_string() },
        };
        if registry.jobs.len() >= TESSELLATION_JOB_CAPACITY {
            if let Some(oldest) = registry.jobs.iter().min_by_key(|(_, retained)| retained.last_step).map(|(key, _)| key.clone()) {
                registry.jobs.remove(&oldest);
            }
        }
        let clock = registry.clock;
        registry.jobs.insert(key.clone(), RetainedTessellation { job, last_step: clock });
    }
    registry.clock += 1;
    let clock = registry.clock;
    let Some(retained) = registry.jobs.get_mut(&key) else {
        return TessellationStepOutcome::Failed { message: "tessellation job vanished between admission and step".to_string() };
    };
    retained.last_step = clock;
    let step = retained.job.step(guard.tessellation_body(), budget);
    match step {
        Err(error) => {
            registry.jobs.remove(&key);
            TessellationStepOutcome::Failed { message: error.to_string() }
        }
        Ok(TessellationStep::Working(progress)) => TessellationStepOutcome::Working { units_done: progress.units_done, units_total: progress.units_total, faces_done: progress.faces_done, faces_total: progress.faces_total, phase: progress.phase.tag() },
        Ok(TessellationStep::Cancelled(_)) => {
            registry.jobs.remove(&key);
            TessellationStepOutcome::Cancelled
        }
        Ok(TessellationStep::Done(progress)) => {
            let Some(retained) = registry.jobs.remove(&key) else {
                return TessellationStepOutcome::Failed { message: "finished tessellation job vanished".to_string() };
            };
            let Some((transfer, _report)) = retained.job.into_mesh() else {
                return TessellationStepOutcome::Failed { message: "finished tessellation job produced no mesh".to_string() };
            };
            let mesh = semio_framework_3d::brep::engine::mesh_data_from_mesh_transfer(&transfer);
            if let Ok(mut cache) = self.mesh_cache().lock() {
                cache.insert(key, mesh.clone());
            }
            TessellationStepOutcome::Ready { mesh, units_total: progress.units_total, faces_total: progress.faces_total }
        }
    }
}
pub fn tessellate_geometry(&self, handle: &str, tolerance: f64) -> Result<semio_framework::MeshData, String> {
    loop {
        match self.tessellate_step(handle, tolerance, usize::MAX) {
            TessellationStepOutcome::Ready { mesh, .. } => return Ok(mesh),
            TessellationStepOutcome::Working { .. } => continue,
            TessellationStepOutcome::Cancelled => return Err("tessellation cancelled".to_string()),
            TessellationStepOutcome::Failed { message } => return Err(message),
            TessellationStepOutcome::Invalid { issues } => {
                let joined = issues.iter().map(|issue| format!("[{}] {}: {}", issue.code, issue.entity, issue.message)).collect::<Vec<_>>().join("; ");
                return Err(format!("validation rejected the solid before tessellation: {joined}"));
            }
        }
    }
}
pub fn tessellate_step_envelope_json(&self, handle: &str, tolerance: f64, budget: usize, wall_micros: u64, chunk: usize) -> String {
    use semio_framework_os_flow::os_pack::json::{object, Value};
    let deadline = semio_framework_job::default_now_us().map(|now| now.saturating_add(wall_micros));
    let mut outcome = self.tessellate_step(handle, tolerance, budget);
    while matches!(outcome, TessellationStepOutcome::Working { .. }) {
        let Some(deadline) = deadline else { break };
        if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
            break;
        }
        outcome = self.tessellate_step(handle, tolerance, budget);
    }
    let envelope = match outcome {
        TessellationStepOutcome::Working { units_done, units_total, faces_done, faces_total, phase } => object([
            ("done".to_string(), Value::Bool(false)),
            ("cancellable".to_string(), Value::Bool(true)),
            ("phase".to_string(), Value::String(phase.to_string())),
            ("unitsDone".to_string(), Value::from(units_done as u64)),
            ("unitsTotal".to_string(), Value::from(units_total as u64)),
            ("facesDone".to_string(), Value::from(faces_done as u64)),
            ("facesTotal".to_string(), Value::from(faces_total as u64)),
        ]),
        TessellationStepOutcome::Ready { mesh, units_total, faces_total } => {
            let body = match encode_mesh_pack(&mesh) {
                Ok(bytes) => bytes,
                Err(message) => return failed_envelope_json("tessellate.encode", &message),
            };
            let chunks = chunk_mesh_base64(&encode_base64(&body));
            let index = chunk.min(chunks.len().saturating_sub(1));
            object([
                ("done".to_string(), Value::Bool(true)),
                ("cancellable".to_string(), Value::Bool(false)),
                ("phase".to_string(), Value::String("complete".to_string())),
                ("unitsDone".to_string(), Value::from(units_total as u64)),
                ("unitsTotal".to_string(), Value::from(units_total as u64)),
                ("facesDone".to_string(), Value::from(faces_total as u64)),
                ("facesTotal".to_string(), Value::from(faces_total as u64)),
                ("chunk".to_string(), Value::from(index as u64)),
                ("chunks".to_string(), Value::from(chunks.len() as u64)),
                ("packBytes".to_string(), Value::from(body.len() as u64)),
                ("meshPack".to_string(), Value::String(chunks[index].clone())),
            ])
        }
        TessellationStepOutcome::Cancelled => object([("done".to_string(), Value::Bool(true)), ("cancellable".to_string(), Value::Bool(false)), ("phase".to_string(), Value::String("cancelled".to_string()))]),
        TessellationStepOutcome::Invalid { issues } => object([
            ("done".to_string(), Value::Bool(true)),
            ("cancellable".to_string(), Value::Bool(false)),
            ("phase".to_string(), Value::String("invalid".to_string())),
            ("diagnostics".to_string(), Value::Array(issues.iter().map(|issue| object([("entity".to_string(), Value::String(issue.entity.clone())), ("code".to_string(), Value::String(issue.code.clone())), ("message".to_string(), Value::String(issue.message.clone()))])).collect())),
        ]),
        TessellationStepOutcome::Failed { message } => return failed_envelope_json("tessellate.failed", &message),
    };
    semio_framework_os_flow::os_pack::json::to_string(&envelope)
}
pub fn tessellate_geometry_json_for_wasm(&self, handle: &str, tolerance: f64) -> String {
    match self.tessellate_geometry(handle, tolerance) {
        Ok(mesh) => semio_framework_os_flow::os_pack::json::to_json_string(&mesh),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error))])),
    }
}
pub fn dispose_geometry(&self, handle: &str) -> Result<(), String> {
    let mut claims = self.state.claims.lock().map_err(|_| "geometry claims lock poisoned")?;
    if self.is_closed() { return Err("geometry.session-closed".into()); }
    if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(handle)) { return Err("geometry.handle-retained-by-other-authority".into()); }
    let mut kernel = self.kernel().write().map_err(|_| "geometry kernel lock poisoned")?;
    kernel.dispose(&GeometryHandle(handle.into()));
    if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(handle); }
    self.evict_mesh_cache_for_handle(handle);
    if let Ok(mut jobs) = self.tessellation_jobs().lock() { jobs.jobs.retain(|(job_handle, _), _| job_handle != handle); }
    Ok(())
}
pub fn export_solid_json(&self, handles: &[String], format: &str, deflection: f64) -> String {
    let shapes: Vec<GeometryHandle> = handles.iter().cloned().map(GeometryHandle).collect();
    let outcome: Result<(String, bool), BrepModuleError> = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned).and_then(|guard| {
        if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
        let guard = &**guard;
        match format {

            "obj" => guard.export_obj(&shapes, deflection).map(|text| (text, false)).map_err(BrepModuleError::from),
            "stl" => guard.export_stl(&shapes, deflection).map(|data| (encode_base64(&data), true)).map_err(BrepModuleError::from),
            "glb" => export_glb_via_tessellation(guard, &shapes, deflection).map(|data| (encode_base64(&data), true)),
            other => self.state.operations.export(guard,other,&shapes,deflection),
        }
    });
    match outcome {
        Ok((data, binary)) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([
            ("data".to_string(), semio_framework_os_flow::os_pack::json::Value::String(data)),
            ("binary".to_string(), semio_framework_os_flow::os_pack::json::Value::Bool(binary)),
            ("format".to_string(), semio_framework_os_flow::os_pack::json::Value::String(format.to_string())),
        ])),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error.to_string()))])),
    }
}
pub fn import_solid_json(&self, format: &str, data: &str, tolerance: f64) -> String {
    let mut claims = self.state.claims.lock().expect("geometry claims");
    let outcome: Result<Vec<String>, BrepModuleError> = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned).and_then(|mut guard| {
        if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
        let guard = &mut **guard;
        match format {

            "obj" => guard.import_obj(data, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from),
            "stl" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| guard.import_stl(&bytes, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)),
            "glb" => decode_base64(data).map_err(BrepModuleError::from).and_then(|bytes| import_glb_via_tessellation(guard, &bytes, tolerance)),
            other => self.state.operations.import(guard,other,data,tolerance).map(|handles| handles.into_iter().map(|handle| handle.0).collect()),
        }
    });
    if let Ok(handles) = &outcome { claims.entry(self.state.authority).or_default().extend(handles.iter().cloned()); }
    match outcome {
        Ok(handles) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("handles".to_string(), semio_framework_os_flow::os_pack::json::from_dsl_value(&semio_framework_os_flow::os_dsl::ToValue::to_value(&handles)))])),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error.to_string()))])),
    }
}
fn brep_invoke_inner(&self, method: &str, args_json: &str) -> Result<semio_framework_os_flow::os_pack::json::Value, BrepModuleError> {
    let mut claims = self.state.claims.lock().map_err(|_| BrepModuleError::LockPoisoned)?;
    if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
    let args = invoke_args(args_json)?;
    let result = match method {
        "box" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.box_prim(arg_f64(&args, "width")?, arg_f64(&args, "depth")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sphere" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sphere_prim(arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cylinder" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cylinder_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cone" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cone_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "lineCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.line_curve(arg_vec3(&args, "start")?, arg_vec3(&args, "end")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "circleCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.circle_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "arcCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .arc_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?, arg_f64(&args, "startAngle")?, arg_f64(&args, "endAngle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "ellipseCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .ellipse_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "semiMajor")?, arg_f64(&args, "semiMinor")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "interpolateCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.interpolate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "approximateCurve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .approximate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?, arg_usize(&args, "controlPoints")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "polylineWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.polyline_wire(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rectangleWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.rectangle_wire(arg_f64(&args, "width")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromPoints" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.planar_face_from_points(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.planar_face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrudeWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.extrude_wire(&arg_handle(&args, "wire")?, arg_vec3(&args, "vector")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrude" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.extrude(&arg_handle(&args, "face")?, arg_vec3(&args, "direction")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "revolve" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .revolve(&arg_handle(&args, "face")?, arg_vec3(&args, "axisOrigin")?, arg_vec3(&args, "axisDirection")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "loft" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.loft(&arg_handles(&args, "profiles")?, arg_bool_or(&args, "smooth", false)).map(handle_result).map_err(BrepModuleError::from)
        }
        "sweep" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sweep(&arg_handle(&args, "profile")?, &arg_handle(&args, "path")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "thickenFace" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.thicken_face(&arg_handle(&args, "face")?, arg_f64(&args, "thickness")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "offsetFace" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.offset_face(&arg_handle(&args, "face")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "fuse" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.fuse(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cut" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.cut(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "intersect" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.intersect(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "translate" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.translate(&arg_handle(&args, "shape")?, arg_vec3(&args, "offset")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotate" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.rotate(&arg_handle(&args, "shape")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotateAbout" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard
                .rotate_about(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "scale" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.scale(&arg_handle(&args, "shape")?, arg_f64(&args, "factor")?, arg_vec3(&args, "center")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "mirror" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.mirror(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "normal")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sewFaces" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.sew_faces(&arg_handles(&args, "faces")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "faceFromWire" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "healSolid" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.heal_solid(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "volume" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.volume(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "area" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.area(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "length" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.length(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "centerOfMass" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.center_of_mass(&arg_handle(&args, "shape")?).map(vec3_result).map_err(BrepModuleError::from)
        }
        "distance" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.distance(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(number_result).map_err(BrepModuleError::from)
        }
        "deconstruct" => {
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.deconstruct(&arg_handle(&args, "shape")?).map(topology_result).map_err(BrepModuleError::from)
        }
        "tessellate" => {
            let guard = self.kernel().read().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.tessellate(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-3)).map(|mesh| mesh_result(&mesh)).map_err(BrepModuleError::from)
        }
        "dispose" => {
            let handle = arg_handle(&args, "handle")?;
            if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(&handle.0)) { return Err(BrepModuleError::InvalidArgs("geometry.handle-retained-by-other-authority".into())); }
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.dispose(&handle);
            if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(&handle.0); }
            self.evict_mesh_cache_for_handle(&handle.0);
            if let Ok(mut jobs) = self.tessellation_jobs().lock() { jobs.jobs.retain(|(job_handle, _), _| job_handle != &handle.0); }
            Ok(unit_result())
        }
        "retain" => {
            let live = arg_handles(&args, "handles")?.into_iter().map(|handle| handle.0).collect::<Vec<_>>();
            claims.insert(self.state.authority, live.iter().cloned().collect());
            let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
            let mut guard = self.kernel().write().map_err(|_| BrepModuleError::LockPoisoned)?;
            guard.retain(&merged);
            self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
            self.retain_tessellation_jobs(&live);
            Ok(unit_result())
        }
        other => {
            let mut guard = self.kernel().write().map_err(|_|BrepModuleError::LockPoisoned)?;
            self.state.operations.invoke(&mut guard,other,&args)
        },
    };
    if let Ok(value) = &result {
        fn gather(value: &semio_framework_os_flow::os_pack::json::Value, handles: &mut BTreeSet<String>) {
            if let Some(handle) = value.get("handle").and_then(|value| value.as_str()) { handles.insert(handle.into()); }
            for key in ["handles", "vertices", "edges", "faces", "shells"] {
                if let Some(values) = value.get(key).and_then(|value| value.as_array()) { handles.extend(values.iter().filter_map(|value| value.as_str().map(str::to_string))); }
            }
        }
        gather(value, claims.entry(self.state.authority).or_default());
    }
    result
}
pub fn brep_invoke_json(&self, method: &str, args_json: &str) -> String {
    match self.brep_invoke_inner(method, args_json) {
        Ok(value) => semio_framework_os_flow::os_pack::json::to_string(&value),
        Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".to_string(), semio_framework_os_flow::os_pack::json::Value::String(error.to_string()))])),
    }
}
}

/// ⚓️ Retains the producing family while retiring a separately admitted job authority.
struct SessionPort { authority:ManuallyDrop<Option<Session>>, anchor:SessionCapture }
impl Drop for SessionPort {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.authority.is_none() && self.anchor.terminal_is_empty(),"geometry port requires explicit retirement before drop"); } }
}
impl semio_framework_os_flow::geometry::GeometryPort for SessionPort {
    fn retain(&self, handles: &[String]) { if let Some(authority) = self.authority.as_ref() { authority.retain_geometry_handles(handles); } }
    fn tessellate_step(&self, handle: &str, tolerance: f64, units: usize) -> semio_framework_os_flow::geometry::GeometryStep {
        use semio_framework_os_flow::geometry::GeometryStep;
        let Some(authority) = self.authority.as_ref() else { return GeometryStep::Failed("geometry session is closed".into()) };
        match authority.tessellate_step(handle, tolerance, units) {
            TessellationStepOutcome::Working { units_done, units_total, phase, .. } => GeometryStep::Working { units_done, units_total, phase: phase.into() },
            TessellationStepOutcome::Ready { mesh, .. } => GeometryStep::Ready(mesh),
            TessellationStepOutcome::Cancelled => GeometryStep::Cancelled,
            TessellationStepOutcome::Failed { message } => GeometryStep::Failed(message),
            TessellationStepOutcome::Invalid { issues } => GeometryStep::Failed(issues.into_iter().map(|issue| issue.message).collect::<Vec<_>>().join("; ")),
        }
    }
    fn dispose(&self, handle: &str) -> Result<(), String> { self.authority.as_ref().ok_or("geometry session is closed")?.dispose_geometry(handle) }
    fn cancel(&self) -> usize { self.authority.as_ref().map_or(0,Session::cancel_all_tessellations) }
    fn begin_close(&self) { if let Some(authority) = self.authority.as_ref() { authority.begin_close(); } }
    fn terminal_is_empty(&self) -> bool { self.authority.is_none() && self.anchor.terminal_is_empty() }
    fn next_close_byte_demand(&self) -> usize { self.authority.as_ref().map_or_else(|| self.anchor.shell_byte_requirement(),|authority| shell_bytes(&authority.state)) }
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize) -> Result<neural_engine::ValueRetirementStep,String> {
        use neural_engine::ValueRetirementStep as Step;
        if self.authority.is_none() && self.anchor.terminal_is_empty() { return Ok(Step::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if self.anchor.session.as_ref().is_some_and(|session| session.state.retirement.lock().expect("geometry retirement").paused) { return Ok(Step::Blocked); }
        if let Some(authority) = self.authority.as_ref() {
            if maximum_bytes < shell_bytes(&authority.state) { return Ok(Step::Blocked); }
            authority.begin_close();
            let receipt = std::sync::atomic::AtomicUsize::new(0);
            let Session { state } = self.authority.take().expect("geometry port authority");
            record_shell(&state,&receipt);
            if let Some(state) = Arc::into_inner(state) { drop(state); }
            return Ok(Step::Pending { released_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed) });
        }
        self.anchor.close_step(maximum_items,maximum_bytes)
    }
}
#[cfg(all(target_arch = "wasm32", not(target_env = "p2"), feature = "browser-publication"))]
mod browser {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub struct BrowserSession { session: super::Session }
    #[wasm_bindgen]
    impl BrowserSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self { Self { session: super::Session::new() } }
        pub fn brep_invoke(&self, method: &str, arguments: &str) -> String { self.session.brep_invoke_json(method, arguments) }
        pub fn tessellate(&self, handle: &str, tolerance: f64) -> String { self.session.tessellate_geometry_json_for_wasm(handle, tolerance) }
        pub fn dispose(&self, handle: &str) -> Result<(), JsValue> { self.session.dispose_geometry(handle).map_err(|error| JsValue::from_str(&error)) }
        pub fn begin_close(&self) { self.session.begin_close(); }
        pub fn cancel_close(&self) { self.session.cancel_close(); }
        pub fn resume_close(&self) { self.session.resume_close(); }
        pub fn terminal_is_empty(&self) -> bool { self.session.terminal_is_empty() }
        pub fn close_step(&self, maximum_items: usize, maximum_bytes: usize) -> String {
            match self.session.close_step(maximum_items,maximum_bytes) {
                Ok(neural_engine::ValueRetirementStep::Blocked) => "{\"phase\":\"blocked\",\"items\":0,\"bytes\":0}".into(),
                Ok(neural_engine::ValueRetirementStep::Complete) => "{\"phase\":\"complete\",\"items\":0,\"bytes\":0}".into(),
                Ok(neural_engine::ValueRetirementStep::Pending { released_items,released_bytes }) => format!("{{\"phase\":\"pending\",\"items\":{released_items},\"bytes\":{released_bytes}}}"),
                Err(error) => semio_framework_os_flow::os_pack::json::to_string(&semio_framework_os_flow::os_pack::json::object([("error".into(),semio_framework_os_flow::os_pack::json::Value::String(error))])),
            }
        }
    }
}
