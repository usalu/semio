//! 🌐️ First-party Semio geometry session with explicit instance ownership.
use semio_framework_os_flow::mesh::*;
use neural_engine::{Atom, Cardinality, ChannelSpec, Dictionary, EvalError, FieldSpec, Operator, OperatorImpl, OperatorInfo, Registry, Schema, Value, VALUE_TYPE_GEOMETRY, VALUE_TYPE_NUMBER, VALUE_TYPE_POINT, VALUE_TYPE_VECTOR};
use semio_framework_value::{ValueType,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::retirement::controlled::{ControlledRetirement,RetainedOwnerGate};
use semio_framework_mesh_engine::HistoryFoldIndex;
use semio_framework_3d::brep::engine::{Brep, BrepKernel, GeometryHandle, GeometryKind, ParamDomain, PointClassification, Vec3};
use semio_framework_3d::brep::queries::tessellation::{TessellationJob, TessellationStep};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::mem::ManuallyDrop;
use std::sync::{Arc,Weak};

// 🔀️ dedyn-fw-os-misc, O1/R11 case 3: `BrepKernel` has exactly one impl (`Brep`, in `🗄️stdio`) —
// every `dyn BrepKernel` call site was already handing this module a concrete `Brep`, so the trait
// object was a no-op coercion, not a real seam. Deleting it also clears an existing O1 violation:
// every `BrepKernel` method is `async fn`, which is not dyn-compatible (E0038) — `dyn BrepKernel`
// could not have compiled as-is.





// #region 🔖️Helpers

/// 🔓️ Read-only kernel access borrows the original retained owner and refuses contention without parking.

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
        summary: "Construct, deconstruct, or modify a brep from vertices, edges, faces, and shells".into(),
        fields: vec![
            FieldSpec::new("vertex", ValueType::List(Box::new(ValueType::Schema("vertex".into())))).with_default(empty_list_value()),
            FieldSpec::new("edge", ValueType::List(Box::new(ValueType::Schema("edge".into())))).with_default(empty_list_value()),
            FieldSpec::new("face", ValueType::List(Box::new(ValueType::Schema("face".into())))).with_default(empty_list_value()),
            FieldSpec::new("shell", ValueType::List(Box::new(ValueType::Schema("shell".into())))).with_default(empty_list_value()),
        ],
    }
}

pub fn topology_list(schema: &str, handles: Vec<GeometryHandle>) -> Dictionary {
    handles
        .into_iter()
        .enumerate()
        .fold(Dictionary::with_schema("list"), |list, (index, handle)| list.insert(index.to_string(), Value::Dictionary(Dictionary::with_schema(schema).insert("handle", Value::Atom(Atom::String(handle.as_str().to_string()))))))
}

/// 🎯️ Resolves exact decimal labels exclusively within one deconstructed source domain.
fn selected_topology(kernel: &Brep, input: &Dictionary, key: &str, handles: &[GeometryHandle]) -> Result<Vec<GeometryHandle>, EvalError> {
    let value = input.get(key).ok_or_else(|| EvalError::MissingInput(key.into()))?;
    let text = value.as_dictionary().and_then(|value| value.get("value")).and_then(Value::as_atom).and_then(Atom::as_str).ok_or_else(|| EvalError::InvalidInput(format!("{key} must contain a text label array")))?;
    if text.len() > 16_000_000 { return Err(EvalError::InvalidInput("component label selection exceeds its byte limit".into())); }
    let value = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| EvalError::InvalidInput(format!("{key} must contain a label array")))?;
    let values = value.as_array().filter(|values| values.len() <= 600_000).ok_or_else(|| EvalError::InvalidInput(format!("{key} must contain a bounded label array")))?;
    let mut labels = BTreeSet::new();
    for value in values {
        let label = value.as_str().filter(|label| !label.is_empty() && !label.starts_with('0') && label.bytes().all(|byte| byte.is_ascii_digit())).and_then(|label| label.parse::<u64>().ok()).ok_or_else(|| EvalError::InvalidInput("component labels must be exact positive uint64 decimal text".into()))?;
        labels.insert(label);
    }
    let mut found = BTreeMap::new();
    for handle in handles {
        let label = kernel.label(handle).ok_or_else(|| EvalError::InvalidInput("source component has no persistent label".into()))?;
        if labels.contains(&label) && found.insert(label,handle.clone()).is_some() { return Err(EvalError::InvalidInput("component label is ambiguous within its source topology".into())); }
    }
    labels.into_iter().map(|label| found.remove(&label).ok_or_else(|| EvalError::InvalidInput(format!("component label {label} no longer exists in the selected source topology")))).collect()
}

pub struct BrepDeconstruct(pub SessionCapture);

/// 🪪️ Resolves one current collection leaf by its captured handle and reports its current index.
fn scoped_brep_source(input: &Dictionary) -> Result<(GeometryHandle, u32), EvalError> {
    let captured = read_text(input, "sourceHandle")?;
    if !captured.is_empty() && (captured.len() != 64 || !captured.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))) { return Err(EvalError::InvalidInput("sourceHandle must be an exact geometry handle".into())); }
    let source = input.get("brep").and_then(Value::as_dictionary).ok_or_else(|| EvalError::MissingInput("brep".into()))?;
    if source.schema() != Some("list") {
        let shape = read_geometry(input, "brep")?;
        if !captured.is_empty() && shape.as_str() != captured { return Err(EvalError::InvalidInput("selected source geometry has changed".into())); }
        return Ok((shape, 0));
    }
    if captured.is_empty() || source.len() > 600_001 { return Err(EvalError::InvalidInput("a bounded source collection requires a captured geometry handle".into())); }
    let mut found = None;
    for (key, value) in source.iter() {
        if key == neural_engine::SCHEMA_KEY { continue; }
        let index = key.parse::<u32>().ok().filter(|index| index.to_string() == *key).ok_or_else(|| EvalError::InvalidInput("source collection indices must be canonical integers".into()))?;
        let handle = value.as_dictionary().and_then(|value| value.get("handle")).and_then(Value::as_atom).and_then(Atom::as_str).ok_or_else(|| EvalError::InvalidInput("source collection must contain geometry leaves".into()))?;
        if handle == captured && found.replace(index).is_some() { return Err(EvalError::InvalidInput("selected source geometry is ambiguous in its collection".into())); }
    }
    found.map(|index| (GeometryHandle(captured), index)).ok_or_else(|| EvalError::InvalidInput("selected source geometry no longer exists in its collection".into()))
}

impl Operator for BrepDeconstruct {
    fn retirement_is_empty(&self) -> bool { self.0.terminal_is_empty() }
    fn next_retire_copy_byte_demand(&self)->Result<usize,ValueError> {self.0.next_close_copy_byte_demand()}
    fn next_retire_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.0.next_close_capacity_byte_demand(copy)}
    fn next_retire_release_byte_demand(&self)->Result<usize,ValueError> {self.0.next_close_release_byte_demand()}
    fn next_retire_depth_demand(&self)->Result<usize,ValueError> {self.0.next_close_depth_demand()}
    fn retire_step(&mut self,grant:RetainedCloneGrant,_:&mut neural_engine::ValueRetirement)->Result<RetainedCloneStep,ValueError> {self.0.close_step(grant)}
    fn retire_cold(mut self:Box<Self>) { self.0.retire_cold(); }
    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        self.0.with_kernel(|kernel| {
            let (shape, source_index) = scoped_brep_source(input)?;
            let topology = kernel.deconstruct(&shape).map_err(|error| map_kernel_error(&error))?;
            let selected_edges = selected_topology(kernel,input,"edgeLabels",&topology.edges)?;
            let selected_faces = selected_topology(kernel,input,"faceLabels",&topology.faces)?;
            Ok(Dictionary::new()
                .insert(neural_engine::produced_channel_id("brep"), Value::Dictionary(geometry_dict(kernel, &shape)?))
                .insert("vertex", Value::Dictionary(topology_list("vertex", topology.vertices)))
                .insert("edge", Value::Dictionary(topology_list("edge", topology.edges)))
                .insert("face", Value::Dictionary(topology_list("face", topology.faces)))
                .insert("shell", Value::Dictionary(topology_list("shell", topology.shells)))
                .insert("selectedEdges", Value::Dictionary(topology_list("edge", selected_edges)))
                .insert("selectedFaces", Value::Dictionary(topology_list("face", selected_faces)))
                .insert("sourceIndex", Value::Dictionary(number_dictionary(f64::from(source_index))))
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
    OwnerBusy,
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
            Self::OwnerBusy => formatter.write_str("brep kernel owner busy"),
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
    validation_units:usize,
    validated:bool,
    job:TessellationJob,
    last_step:u64,
    retired:bool,
    output:Option<semio_framework_3d::brep::engine::MeshTransfer>,
    previous_mesh:Option<CachedMesh>,
}
semio_framework_value::artifact_retire_struct!(RetainedTessellation {validation_units,validated,job,last_step,retired,output,previous_mesh});
#[derive(Default)]
struct TessellationJobRegistry {
    jobs:HistoryFoldIndex<(String,u64),RetainedTessellation>,
    clock:u64,
    retire_slot:usize,
    closing:Option<ControlledRetirement<((String,u64),RetainedTessellation)>>,
}
semio_framework_value::artifact_retire_struct!(TessellationJobRegistry {jobs,clock,retire_slot,closing});
struct CachedMesh {mesh:semio_framework::MeshData,valid:bool}
semio_framework_value::artifact_retire_struct!(CachedMesh {mesh,valid});
#[derive(Default)]
struct AuthorityClaims {handles:HistoryFoldIndex<String,bool>,active:bool}
impl AuthorityClaims {
    fn open()->Self {Self {active:true,..Self::default()}}
    fn insert(&mut self,handle:String) {self.handles.insert(handle,true);}
    fn contains(&self,handle:&str)->bool {self.active && self.handles.get(handle)==Some(&true)}
    fn remove(&mut self,handle:&str) {if let Some(live)=self.handles.get_mut(handle) {*live=false;}}
    fn retain(&mut self,handles:&[String]) {for (handle,live) in self.handles.slot_entries_mut() {*live=handles.iter().any(|item|item==handle);}for handle in handles {if self.handles.get(handle)==Some(&true) {continue;}self.insert(handle.clone());}}
    fn extend(&mut self,handles:impl IntoIterator<Item=String>) {for handle in handles {self.insert(handle);}}
    fn iter(&self)->impl Iterator<Item=&String> {self.handles.iter().filter(|(_,live)|self.active && **live).map(|(handle,_)|handle)}
}
impl FromIterator<String> for AuthorityClaims {fn from_iter<T:IntoIterator<Item=String>>(handles:T)->Self {let mut claims=Self::open();claims.extend(handles);claims}}
semio_framework_value::artifact_retire_struct!(AuthorityClaims {handles,active});
type MeshCache=HistoryFoldIndex<(String,u64),CachedMesh>;
type FamilyClaims=HistoryFoldIndex<u64,AuthorityClaims>;
type FamilyRetirement=ControlledRetirement<(Brep,MeshCache,FamilyClaims)>;



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
    use semio_framework_pack_json::{object, Value};
    semio_framework_pack_json::to_string(&object([
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
    use semio_framework::mesh_io::MeshExporter;
    let mut merged = semio_framework::MeshData::default();
    for shape in shapes {
        let transfer = kernel.tessellate(shape, deflection)?;
        let mesh = semio_framework_3d::brep::engine::mesh_data_from_mesh_transfer(&transfer)?;
        let offset = (merged.positions.len() / 3) as u32;
        merged.positions.extend(mesh.positions);
        merged.normals.extend(mesh.normals);
        merged.indices.extend(mesh.indices.into_iter().map(|index| index + offset));
    }
    semio_framework::mesh_io::GlbExporter.export(&merged).map_err(BrepModuleError::Mesh)
}

/// 📥️ Imports STEP/OBJ/STL solid data (or GLB mesh data, bridged through the kernel's OBJ ingestion since it has no raw-mesh entry point) into the in-process kernel. STEP/OBJ expect UTF-8 text in `data`; STL/GLB expect base64-encoded bytes. Returns `{"handles":[...]}` or `{"error"}` JSON.

/// 🧊️ Bridges GLB import through the mesh codec: decodes GLB bytes to `MeshData` via `GlbImporter`, re-encodes it as OBJ text, and ingests that through the kernel's own OBJ importer.
pub fn import_glb_via_tessellation(kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<String>, BrepModuleError> {
    use semio_framework::mesh_io::MeshImporter;
    let mesh = semio_framework::mesh_io::GlbImporter.import(bytes).map_err(BrepModuleError::Mesh)?;
    let obj_text = semio_framework::mesh_io::text::mesh_to_obj(&mesh, "glb-import");
    kernel.import_obj(&obj_text, tolerance).map(|handle| vec![handle.0]).map_err(BrepModuleError::from)
}
// #endregion 🔖️MediaExport

// #region 🔖️GenericInvoke
/// 🌉️ `brep_invoke` argument/result JSON shape: `{"error": "..."}` on failure, otherwise one of
/// `{"handle": "..."}` / `{"handles": [...]}` / `{"value": ...}` / a raw `MeshTransfer` object /
/// `{"vertices": [...], "edges": [...], "faces": [...], "shells": [...]}` for `deconstruct`.
fn invoke_args(args_json: &str) -> Result<semio_framework_pack_json::Value, BrepModuleError> {
    semio_framework_pack_json::parse(args_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| BrepModuleError::InvalidArgs(error.to_string()))
}

fn arg_f64(args: &semio_framework_pack_json::Value, key: &str) -> Result<f64, BrepModuleError> {
    args.get(key).and_then(|value| value.as_f64()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing number {key}")))
}

fn arg_f64_or(args: &semio_framework_pack_json::Value, key: &str, fallback: f64) -> f64 {
    args.get(key).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

fn arg_usize(args: &semio_framework_pack_json::Value, key: &str) -> Result<usize, BrepModuleError> {
    args.get(key).and_then(|value| value.as_u64()).map(|value| value as usize).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing integer {key}")))
}

fn arg_bool_or(args: &semio_framework_pack_json::Value, key: &str, fallback: bool) -> bool {
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

fn arg_vec3(args: &semio_framework_pack_json::Value, key: &str) -> Result<Vec3, BrepModuleError> {
    let value = args.get(key).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point {key}")))?;
    value_vec3(value)
}

fn arg_points(args: &semio_framework_pack_json::Value, key: &str) -> Result<Vec<Vec3>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing point array {key}")))?;
    items.iter().map(value_vec3).collect()
}

fn arg_handle(args: &semio_framework_pack_json::Value, key: &str) -> Result<GeometryHandle, BrepModuleError> {
    arg_string(args, key).map(GeometryHandle)
}

fn arg_handles(args: &semio_framework_pack_json::Value, key: &str) -> Result<Vec<GeometryHandle>, BrepModuleError> {
    let items = args.get(key).and_then(|value| value.as_array()).ok_or_else(|| BrepModuleError::InvalidArgs(format!("missing handle array {key}")))?;
    items.iter().map(|item| item.as_str().map(|text| GeometryHandle(text.to_string())).ok_or_else(|| BrepModuleError::InvalidArgs(format!("{key} entries must be strings")))).collect()
}

fn handle_result(handle: GeometryHandle) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("handle".to_string(), semio_framework_pack_json::Value::String(handle.0))])
}

fn handles_result(handles: Vec<GeometryHandle>) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("handles".to_string(), semio_framework_pack_json::array(handles.into_iter().map(|handle| semio_framework_pack_json::Value::String(handle.0))))])
}

fn number_result(value: f64) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("value".to_string(), semio_framework_pack_json::Value::Number(semio_framework_pack_json::Number::Float(value)))])
}

fn vec3_result(value: Vec3) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([(
        "value".to_string(),
        semio_framework_pack_json::array(value.into_iter().map(semio_framework_pack_json::Number::Float).map(semio_framework_pack_json::Value::Number)),
    )])
}

fn string_result(value: String) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([("value".to_string(), semio_framework_pack_json::Value::String(value))])
}

fn unit_result() -> semio_framework_pack_json::Value {
    semio_framework_pack_json::object([])
}

fn topology_result(topology: semio_framework_3d::brep::engine::BrepTopology) -> semio_framework_pack_json::Value {
    let handle_array = |handles: Vec<GeometryHandle>| semio_framework_pack_json::array(handles.into_iter().map(|handle| semio_framework_pack_json::Value::String(handle.0)));
    semio_framework_pack_json::object([
        ("vertices".to_string(), handle_array(topology.vertices)),
        ("edges".to_string(), handle_array(topology.edges)),
        ("faces".to_string(), handle_array(topology.faces)),
        ("shells".to_string(), handle_array(topology.shells)),
    ])
}

fn mesh_result(mesh: &semio_framework_3d::brep::engine::MeshTransfer) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(mesh))
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
pub struct SessionCapture {
    session:ManuallyDrop<Option<Session>>,
    orphan:ManuallyDrop<Option<SessionState>>,
    allocation:ManuallyDrop<Option<Weak<SessionState,ShellAllocator>>>,
    allocation_bytes:usize,
}
impl std::ops::Deref for SessionCapture {
    type Target=Session;
    fn deref(&self)->&Session {self.session.as_ref().expect("open geometry capture")}
}
impl SessionCapture {
    fn owned(session:Session)->Self {Self {session:ManuallyDrop::new(Some(session)),orphan:ManuallyDrop::new(None),allocation:ManuallyDrop::new(None),allocation_bytes:0}}
    pub fn terminal_is_empty(&self)->bool {self.session.is_none() && self.orphan.is_none() && self.allocation.is_none()}
    fn retained_state(&self)->Option<&SessionState> {self.session.as_ref().map(|session|&*session.state).or_else(||self.orphan.as_ref())}
    pub fn begin_close(&self) {if let Some(state)=self.retained_state() {state.begin_close();}}
    pub fn cancel_close(&self) {if let Some(state)=self.retained_state() {state.retirement.try_lock().expect("geometry retirement").paused=true;}}
    pub fn resume_close(&self) {if let Some(state)=self.retained_state() {state.retirement.try_lock().expect("geometry retirement").paused=false;}}

    pub fn is_closed(&self)->bool {self.session.as_ref().map_or_else(||self.orphan.as_ref().is_none_or(|state|state.closed.load(std::sync::atomic::Ordering::Acquire)),Session::is_closed)}
    pub fn native_terminal_is_empty(&self)->bool {self.session.as_ref().map_or_else(||self.orphan.as_ref().is_none_or(SessionState::terminal_is_empty),Session::terminal_is_empty)}
    pub fn shell_byte_requirement(&self)->usize {
        if let Some(state)=self.orphan.as_ref() {shell_bytes(&state.kernel)+shell_bytes(&state.mesh_cache)+shell_bytes(&state.claims)+shell_bytes(&state.next_authority)+shell_bytes(&state.retirement)} else if self.allocation.is_some() {self.allocation_bytes} else {0}
    }
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_copy_byte_demand(),_=>Ok(0)}}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_capacity_byte_demand(copy),_=>Ok(0)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_release_byte_demand(),_=>Ok(self.shell_byte_requirement())}}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError> {match self.orphan.as_ref() {Some(state) if !state.terminal_is_empty()=>state.next_depth_demand(),_=>Ok(usize::from(!self.terminal_is_empty()))}}
    /// ⚓️ Pins the actual allocation while the final original state transfers inline without rebirth.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
        let state=self.session.as_ref().map(|session|&*session.state).or_else(||self.orphan.as_ref());
        if state.is_some_and(|state|state.retirement.try_lock().map(|owner|owner.paused).unwrap_or(true)) {return Ok(ownership_progress());}
        if let Some(session)=self.session.as_ref() {
            let allocation=Arc::downgrade(&session.state);
            self.allocation_bytes=shell_bytes(&session.state);
            *self.allocation=Some(allocation);
            let Session {state}=self.session.take().unwrap();
            *self.orphan=Arc::into_inner(state);
            return Ok(ownership_advance());
        }
        if let Some(state)=self.orphan.as_ref() {
            if !state.terminal_is_empty() {return state.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
            if grant.maximum_release_bytes<self.shell_byte_requirement() {return Ok(ownership_progress());}
            let receipt=std::sync::atomic::AtomicUsize::new(0);
            record_shell(&state.kernel,&receipt);record_shell(&state.mesh_cache,&receipt);record_shell(&state.claims,&receipt);record_shell(&state.next_authority,&receipt);record_shell(&state.retirement,&receipt);
            drop(self.orphan.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed),..Default::default()}));
        }
        if grant.maximum_release_bytes<self.allocation_bytes {return Ok(ownership_progress());}
        let receipt=std::sync::atomic::AtomicUsize::new(0);
        let allocation=self.allocation.take().unwrap();
        Weak::allocator(&allocation).receipt.store(std::ptr::from_ref(&receipt).cast_mut(),std::sync::atomic::Ordering::Relaxed);
        drop(allocation);
        self.allocation_bytes=0;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,released_bytes:receipt.load(std::sync::atomic::Ordering::Relaxed),..Default::default()}))
    }
    /// 🧊️ Grants each current original currency exactly at an explicit cold boundary.
    pub fn retire_cold(&mut self) {
        while !self.terminal_is_empty() {
            let copy=self.next_close_copy_byte_demand().expect("cold capture copy demand");
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).expect("cold capture capacity demand"),maximum_release_bytes:self.next_close_release_byte_demand().expect("cold capture release demand"),maximum_depth:self.next_close_depth_demand().expect("cold capture depth demand")};
            let step=self.close_step(grant).expect("cold geometry capture retirement");
            assert!(step.progress()!=RetainedCloneProgress::default() || self.terminal_is_empty(),"cold geometry capture waits on an original owner");
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
    fn invoke(&self, _: &mut Brep, method: &str, _: &semio_framework_pack_json::Value) -> Result<semio_framework_pack_json::Value,BrepModuleError> { Err(BrepModuleError::UnknownMethod(method.into())) }
}
impl GeometryOperations for () {}

struct SessionState {
    operations:&'static dyn GeometryOperations,
    kernel:ShellArc<RetainedOwnerGate<ManuallyDrop<Brep>>>,
    mesh_cache:ShellArc<RetainedOwnerGate<ManuallyDrop<MeshCache>>>,
    claims:ShellArc<RetainedOwnerGate<ManuallyDrop<FamilyClaims>>>,
    next_authority:ShellArc<std::sync::atomic::AtomicU64>,
    authority:u64,
    closed:std::sync::atomic::AtomicBool,
    jobs:RetainedOwnerGate<ControlledRetirement<TessellationJobRegistry>>,
    retirement:ShellArc<RetainedOwnerGate<SessionRetirement>>,
}
struct SessionRetirement {family:Option<FamilyRetirement>,active:usize,paused:bool}
impl Default for SessionRetirement {fn default()->Self {Self {family:None,active:1,paused:false}}}
fn ownership_busy()->ValueError {ValueError::literal(ValueRefusalKind::WorkLimit,"geometry owner is busy")}
fn ownership_progress()->RetainedCloneStep {RetainedCloneStep::Progress(RetainedCloneProgress::default())}
fn ownership_advance()->RetainedCloneStep {RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})}
fn controlled<T:semio_framework_value::retirement::RetireOwned>(value:T)->ControlledRetirement<T> {ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original geometry owner has no retirement authority"))}
impl SessionState {
    fn begin_close(&self) {
        let mut claims=self.claims.try_lock().expect("geometry claims owner");
        let mut retirement=self.retirement.try_lock().expect("geometry retirement owner");
        if self.closed.swap(true,std::sync::atomic::Ordering::AcqRel) {return;}
        if let Some(claim)=claims.get_mut(&self.authority) {claim.active=false;retirement.active-=1;}
    }
    fn terminal_is_empty(&self)->bool {
        let Ok(jobs)=self.jobs.try_lock() else {return false};
        let Ok(retirement)=self.retirement.try_lock() else {return false};
        self.closed.load(std::sync::atomic::Ordering::Acquire) && jobs.terminal_is_empty() && (retirement.active!=0 || Arc::strong_count(&self.retirement)>1 || retirement.family.as_ref().is_some_and(ControlledRetirement::terminal_is_empty))
    }
    fn close_step(&self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
        if self.retirement.try_lock().map_err(|_|ownership_busy())?.paused {return Ok(ownership_progress());}
        self.begin_close();
        let mut jobs=self.jobs.try_lock().map_err(|_|ownership_busy())?;
        if !jobs.terminal_is_empty() {return jobs.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        drop(jobs);
        let mut retirement=self.retirement.try_lock().map_err(|_|ownership_busy())?;
        if retirement.active!=0 || Arc::strong_count(&self.retirement)>1 {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if retirement.family.is_none() {
            let mut kernel=self.kernel.try_lock().map_err(|_|ownership_busy())?;
            let mut cache=self.mesh_cache.try_lock().map_err(|_|ownership_busy())?;
            let mut claims=self.claims.try_lock().map_err(|_|ownership_busy())?;
            let original=(std::mem::replace(&mut **kernel,Brep::new()),std::mem::take(&mut **cache),std::mem::take(&mut **claims));
            retirement.family=Some(controlled(original));
            return Ok(ownership_advance());
        }
        let family=retirement.family.as_mut().unwrap();
        if family.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        family.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))
    }
    fn demand<R>(&self,query:impl FnOnce(&dyn semio_framework_value::ErasedSnapshotRetirement)->Result<R,ValueError>,transition:R)->Result<R,ValueError> {
        let jobs=self.jobs.try_lock().map_err(|_|ownership_busy())?;
        if !jobs.terminal_is_empty() {return query(&*jobs);}
        let retirement=self.retirement.try_lock().map_err(|_|ownership_busy())?;
        match retirement.family.as_ref() {Some(family) if retirement.active==0=>query(family),_=>Ok(transition)}
    }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.demand(|owner|owner.next_copy_byte_demand(),0)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.demand(|owner|owner.next_capacity_byte_demand(copy),0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.demand(|owner|owner.next_release_byte_demand(),0)}
    fn next_depth_demand(&self)->Result<usize,ValueError> {self.demand(|owner|owner.next_depth_demand(),usize::from(!self.terminal_is_empty()))}
}
impl Drop for SessionState {fn drop(&mut self) {if !std::thread::panicking() {assert!(self.closed.load(std::sync::atomic::Ordering::Acquire),"geometry authority requires explicit close before drop");assert!(self.jobs.get_mut().terminal_is_empty(),"geometry jobs require terminal-empty retirement");}}}
impl Drop for SessionRetirement {fn drop(&mut self) {if !std::thread::panicking() {assert!(self.active==0 && self.family.as_ref().is_some_and(ControlledRetirement::terminal_is_empty),"geometry family requires terminal-empty retirement");}}}
impl Default for Session { fn default() -> Self { Self::new() } }
impl Session {
    /// 🔗️ Captures this exact authority for a supplied operator or retained composition reader.
    pub fn capture(&self) -> SessionCapture { SessionCapture::owned(self.clone()) }
    /// 🔌️ Supplies one separately admitted retained-job authority over this kernel family.
    pub fn port(&self) -> Box<dyn semio_framework_os_flow::geometry::GeometryPort> {
        let mut claims = self.state.claims.try_lock().expect("geometry claims");
        let authority = self.state.next_authority.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let closed = self.is_closed();
        if !closed {claims.insert(authority,AuthorityClaims::open());self.state.retirement.try_lock().expect("geometry retirement").active+=1;}
        let authority = Session { state: shell(SessionState {
            operations:self.state.operations, kernel: self.state.kernel.clone(), mesh_cache: self.state.mesh_cache.clone(),
            claims: self.state.claims.clone(), next_authority: self.state.next_authority.clone(), authority,
            closed: std::sync::atomic::AtomicBool::new(closed), jobs:RetainedOwnerGate::new(controlled(TessellationJobRegistry::default())), retirement:self.state.retirement.clone(),
        }) };
        Box::new(SessionPort {authority:SessionCapture::owned(authority),anchor:self.capture()})
    }
    /// 🚪️ Seals this original authority without extracting claims, jobs or family resources.
    pub fn begin_close(&self) {self.state.begin_close();}
    pub fn cancel_close(&self) {self.state.retirement.try_lock().expect("geometry retirement").paused=true;}
    pub fn resume_close(&self) {self.state.retirement.try_lock().expect("geometry retirement").paused=false;}
    /// 🎟️ Forwards all independent currencies to the actual retained authority.
    pub fn close_step(&self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {self.state.close_step(grant)}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {self.state.next_copy_byte_demand()}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.state.next_capacity_byte_demand(copy)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {self.state.next_release_byte_demand()}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError> {self.state.next_depth_demand()}
    pub fn terminal_is_empty(&self)->bool {self.state.terminal_is_empty() && self.state.retirement.try_lock().is_ok_and(|owner|owner.active==0 && owner.family.as_ref().is_some_and(ControlledRetirement::terminal_is_empty))}
    /// 🧊️ Explicit cold callers admit the next exact original demands independently.
    pub fn close(&self) {
        self.begin_close();
        while !self.terminal_is_empty() {
            let copy=self.next_close_copy_byte_demand().expect("cold geometry copy demand");
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_close_capacity_byte_demand(copy).expect("cold geometry capacity demand"),maximum_release_bytes:self.next_close_release_byte_demand().expect("cold geometry release demand"),maximum_depth:self.next_close_depth_demand().expect("cold geometry depth demand")};
            let progress=self.close_step(grant).expect("cold geometry retirement").progress();
            if progress==RetainedCloneProgress::default() {return;}
        }
    }
    pub fn is_closed(&self) -> bool { self.state.closed.load(std::sync::atomic::Ordering::Acquire) }
    pub fn new() -> Self { Self::with_operations(&()) }
    /// 🔌️ Admits an explicit static owner operation table without retaining extra payload state.
    pub fn with_operations(operations: &'static dyn GeometryOperations) -> Self {
        Self { state: shell(SessionState { operations, kernel:shell(RetainedOwnerGate::new(ManuallyDrop::new(Brep::new()))), mesh_cache:shell(RetainedOwnerGate::new(ManuallyDrop::new(MeshCache::new()))), claims:shell(RetainedOwnerGate::new(ManuallyDrop::new(FamilyClaims::from_iter([(1,AuthorityClaims::open())])))), next_authority: shell(std::sync::atomic::AtomicU64::new(2)), authority:1,closed:std::sync::atomic::AtomicBool::new(false),jobs:RetainedOwnerGate::new(controlled(TessellationJobRegistry::default())),retirement:shell(RetainedOwnerGate::new(SessionRetirement::default())) }) }
    }
fn kernel(&self) -> &RetainedOwnerGate<ManuallyDrop<Brep>> { &self.state.kernel }
fn mesh_cache(&self) -> &RetainedOwnerGate<ManuallyDrop<MeshCache>> { &self.state.mesh_cache }
pub fn evict_mesh_cache_for_handles(&self,handles:&[String]) {
    if self.is_closed() {return;}
    if let Ok(mut cache)=self.mesh_cache().try_lock() {for (key,cached) in cache.slot_entries_mut() {if !handles.iter().any(|handle|handle==&key.0) {cached.valid=false;}}}
}
pub fn evict_mesh_cache_for_handle(&self,handle:&str) {
    if self.is_closed() {return;}
    if let Ok(mut cache)=self.mesh_cache().try_lock() {for (key,cached) in cache.slot_entries_mut() {if key.0==handle {cached.valid=false;}}}
}
pub fn with_kernel<T>(&self, f: impl FnOnce(&mut Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let mut claims = self.state.claims.try_lock().map_err(|_| EvalError::InvalidInput("geometry claims owner busy".into()))?;
    let mut guard = self.kernel().try_lock().map_err(|_| EvalError::InvalidInput("brep kernel owner busy".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    let before = guard.live_handles();
    let result = f(&mut guard);
    claims.entry(self.state.authority).or_default().extend(guard.live_handles().difference(&before).cloned());
    result
}
pub fn with_kernel_read<T>(&self, f: impl FnOnce(&Brep) -> Result<T, EvalError>) -> Result<T, EvalError> {
    let guard = self.kernel().try_lock().map_err(|_| EvalError::InvalidInput("brep kernel owner busy".into()))?;
    if self.is_closed() { return Err(EvalError::InvalidInput("geometry.session-closed".into())); }
    f(&guard)
}
/// 📦 Advances an owned mesh import and admits exactly its final published handle.
pub fn step_mesh_import(&self,cursor:&mut semio_framework_3d::brep::engine::MeshImportCursor,budget:usize)->Result<Option<GeometryHandle>,EvalError> {
    self.close_mesh_import(cursor,budget,4096)
}
/// 🎟️ Uses exact retained import and rollback byte credit within this authority.
pub fn close_mesh_import(&self,cursor:&mut semio_framework_3d::brep::engine::MeshImportCursor,budget:usize,bytes:usize)->Result<Option<GeometryHandle>,EvalError> {
    let mut claims=self.state.claims.try_lock().map_err(|_|EvalError::InvalidInput("geometry claims owner busy".into()))?;
    let mut guard=self.kernel().try_lock().map_err(|_|EvalError::InvalidInput("brep kernel owner busy".into()))?;
    if self.is_closed() {return Err(EvalError::InvalidInput("geometry.session-closed".into()));}
    let result=guard.close_mesh_import_sync(cursor,budget,bytes).map_err(|error|map_kernel_error(&error))?;
    if let Some(handle)=&result {claims.entry(self.state.authority).or_default().insert(handle.as_str().to_string());}
    Ok(result)
}
pub fn retain_geometry_handles(&self, live: &[String]) {
    let mut claims = self.state.claims.try_lock().expect("geometry claims");
    if self.is_closed() { return; }
    claims.entry(self.state.authority).or_default().retain(live);
    let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
    if let Ok(mut guard) = self.kernel().try_lock() { guard.retain(&merged); }
    self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
    self.retain_tessellation_jobs(live);
}
fn tessellation_jobs(&self)->&RetainedOwnerGate<ControlledRetirement<TessellationJobRegistry>> {&self.state.jobs}
pub fn retain_tessellation_jobs(&self,live:&[String]) {
    if self.is_closed() {return;}
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return};
    let Some(registry)=owner.original_mut() else {return};
    for (key,retained) in registry.jobs.slot_entries_mut() {if !retained.retired && !live.iter().any(|handle|handle==&key.0) {retained.job.cancel();retained.retired=true;}}
    registry.retire_slot=0;
}
pub fn cancel_tessellation(&self,handle:&str,tolerance:f64)->bool {
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return false};
    let Some(registry)=owner.original_mut() else {return false};
    for (key,retained) in registry.jobs.slot_entries_mut() {if key.0==handle && key.1==tolerance.to_bits() && !retained.retired {retained.job.cancel();retained.retired=true;registry.retire_slot=0;return true;}}
    false
}
pub fn cancel_all_tessellations(&self)->usize {
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return 0};
    let Some(registry)=owner.original_mut() else {return 0};
    let mut count=0;
    for retained in registry.jobs.slot_values_mut() {if !retained.retired {retained.job.cancel();retained.retired=true;count+=1;}}
    registry.retire_slot=0;count
}
fn retire_jobs_for_handle(&self,handle:&str) {
    let Ok(mut owner)=self.tessellation_jobs().try_lock() else {return};
    let Some(registry)=owner.original_mut() else {return};
    for (key,retained) in registry.jobs.slot_entries_mut() {if key.0==handle && !retained.retired {retained.job.cancel();retained.retired=true;}}
    registry.retire_slot=0;
}
/// 🎟️ Transfers one original cancelled row inline, then pays its exact typed retirement demands.
pub fn close_retired_tessellation_step(&self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
    let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
    let Some(registry)=owner.original_mut() else {return Err(ownership_busy())};
    if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(ownership_progress());}
    if let Some(closing)=registry.closing.as_mut() {
        if closing.terminal_is_empty() {registry.closing=None;return Ok(ownership_advance());}
        return closing.step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));
    }
    if registry.retire_slot>=registry.jobs.slot_count() {return Ok(RetainedCloneStep::Complete(Default::default()));}
    let slot=registry.retire_slot;registry.retire_slot+=1;
    if let Some(original)=registry.jobs.extract_slot_if(slot,|_,row|!row.retired) {registry.closing=Some(controlled(original));}
    Ok(ownership_advance())
}
fn retired_tessellation_demand<R>(&self,query:impl FnOnce(&dyn semio_framework_value::ErasedSnapshotRetirement)->Result<R,ValueError>,scan:R)->Result<R,ValueError> {
    let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
    let Some(registry)=owner.original() else {return Err(ownership_busy())};
    match registry.closing.as_ref() {Some(closing)=>query(closing),None=>Ok(scan)}
}
pub fn next_retired_tessellation_copy_byte_demand(&self)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_copy_byte_demand(),0)}
pub fn next_retired_tessellation_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_capacity_byte_demand(copy),0)}
pub fn next_retired_tessellation_release_byte_demand(&self)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_release_byte_demand(),0)}
pub fn next_retired_tessellation_depth_demand(&self)->Result<usize,ValueError> {self.retired_tessellation_demand(|owner|owner.next_depth_demand(),1)}
fn retire_tessellations_cold(&self)->Result<(),String> {
    loop {
        let copy=self.next_retired_tessellation_copy_byte_demand().map_err(|error|error.to_string())?;
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_retired_tessellation_capacity_byte_demand(copy).map_err(|error|error.to_string())?,maximum_release_bytes:self.next_retired_tessellation_release_byte_demand().map_err(|error|error.to_string())?,maximum_depth:self.next_retired_tessellation_depth_demand().map_err(|error|error.to_string())?};
        if matches!(self.close_retired_tessellation_step(grant).map_err(|error|error.to_string())?,RetainedCloneStep::Complete(_)) {return Ok(());}
    }
}
pub fn tessellation_progress(&self,handle:&str,tolerance:f64)->Option<(usize,usize,&'static str)> {
    let owner=self.tessellation_jobs().try_lock().ok()?;
    let registry=owner.original()?;
    let retained=registry.jobs.get(&(handle.to_string(),tolerance.to_bits()))?;
    if retained.retired {return None;}
    let progress=retained.job.progress();
    Some((progress.units_done.saturating_add(if retained.validated {retained.validation_units}else {0}),progress.units_total.saturating_add(retained.validation_units),if retained.validated {progress.phase.tag()}else {"validating"}))
}
pub fn cached_mesh_at_or_finer(&self, handle: &str, tolerance: f64) -> Option<semio_framework::MeshData> {
    if self.is_closed() { return None; }
    let cache = self.mesh_cache().try_lock().ok()?;
    if let Some(exact) = cache.get(&(handle.to_string(), tolerance.to_bits())) {
        if exact.valid {return Some(exact.mesh.clone());}
    }
    let mut best: Option<(f64, &semio_framework::MeshData)> = None;
    for ((cached_handle,bits),cached) in cache.iter() {
        if !cached.valid {continue;}
        let mesh=&cached.mesh;
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
    let mut claims = self.state.claims.try_lock().expect("geometry claims");
    if self.is_closed() { return TessellationStepOutcome::Failed { message: "geometry.session-closed".into() }; }
    claims.entry(self.state.authority).or_default().insert(handle.into());
    if let Some(mesh) = self.cached_mesh_at_or_finer(handle, tolerance) {
        return TessellationStepOutcome::Ready { units_total: 0, faces_total: 0, mesh };
    }
    let key = (handle.to_string(), tolerance.to_bits());
    let Ok(guard) = self.kernel().try_lock() else {
        return TessellationStepOutcome::Failed { message: "brep kernel owner busy".to_string() };
    };
    let geometry = GeometryHandle(handle.to_string());
    let Ok(mut job_owner) = self.tessellation_jobs().try_lock() else {
        return TessellationStepOutcome::Failed { message: "tessellation job registry owner busy".to_string() };
    };
    let Some(registry)=job_owner.original_mut() else {return TessellationStepOutcome::Cancelled};
    if registry.jobs.get(&key).is_some_and(|row|row.retired) {return TessellationStepOutcome::Cancelled;}
    if !registry.jobs.contains_key(&key) {
        let job = match guard.tessellate_job_sync(&geometry, tolerance) {
            Ok(job) => job,
            Err(error) => return TessellationStepOutcome::Failed { message: error.to_string() },
        };
        if registry.jobs.len()>=TESSELLATION_JOB_CAPACITY {return TessellationStepOutcome::Failed {message:"tessellation retirement admission is required before another retained job".into()};}
        let clock = registry.clock;
        let validation_units=usize::from(matches!(guard.kind_sync(&geometry),Ok(GeometryKind::Solid|GeometryKind::Shell|GeometryKind::Compound)));
        registry.jobs.insert(key.clone(),RetainedTessellation {validation_units,validated:validation_units==0,job,last_step:clock,retired:false,output:None,previous_mesh:None});
    }
    registry.clock += 1;
    let clock = registry.clock;
    let Some(retained) = registry.jobs.get_mut(&key) else {
        return TessellationStepOutcome::Failed { message: "tessellation job vanished between admission and step".to_string() };
    };
    retained.last_step = clock;
    let mut remaining=budget;
    if !retained.validated {
        if remaining==0 {return TessellationStepOutcome::Working {units_done:0,units_total:retained.job.progress().units_total.saturating_add(1),faces_done:0,faces_total:0,phase:"validating"};}
        if let Err(issues)=guard.validate_gate_sync(&geometry) {
            retained.job.cancel();retained.retired=true;registry.retire_slot=0;
            return TessellationStepOutcome::Invalid {issues:issues.into_iter().map(|issue|PreviewDiagnostic {entity:issue.entity,code:issue.code.to_string(),message:issue.message}).collect()};
        }
        retained.validated=true;remaining-=1;
    }
    let validation_units=retained.validation_units;
    let step = guard.step_tessellation_job_sync(&geometry,&mut retained.job,remaining);
    match step {
        Err(error) => {
            retained.job.cancel();retained.retired=true;registry.retire_slot=0;
            TessellationStepOutcome::Failed { message: error.to_string() }
        }
        Ok(TessellationStep::Working(progress)) => TessellationStepOutcome::Working { units_done:progress.units_done.saturating_add(validation_units),units_total:progress.units_total.saturating_add(validation_units), faces_done: progress.faces_done, faces_total: progress.faces_total, phase: progress.phase.tag() },
        Ok(TessellationStep::Cancelled(_)) => {
            retained.job.cancel();retained.retired=true;registry.retire_slot=0;
            TessellationStepOutcome::Cancelled
        }
        Ok(TessellationStep::Done(progress)) => {
            retained.retired=true;registry.retire_slot=0;
            let Some((transfer,_report))=retained.job.take_mesh() else {return TessellationStepOutcome::Failed {message:"finished tessellation job produced no mesh".into()};};
            retained.output=Some(transfer);
            let mesh=match semio_framework_3d::brep::engine::mesh_data_from_mesh_transfer(retained.output.as_ref().unwrap()) {Ok(mesh)=>mesh,Err(error)=>return TessellationStepOutcome::Failed {message:error.to_string()}};
            if let Ok(mut cache)=self.mesh_cache().try_lock() {retained.previous_mesh=cache.insert(key,CachedMesh {mesh:mesh.clone(),valid:true});}
            TessellationStepOutcome::Ready { mesh, units_total:progress.units_total.saturating_add(validation_units),faces_total:progress.faces_total }
        }
    }
}
pub fn tessellate_geometry(&self, handle: &str, tolerance: f64) -> Result<semio_framework::MeshData, String> {
    self.retire_tessellations_cold()?;
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
    use semio_framework_pack_json::{object, Value};
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
    semio_framework_pack_json::to_string(&envelope)
}
pub fn tessellate_geometry_json_for_wasm(&self, handle: &str, tolerance: f64) -> String {
    match self.tessellate_geometry(handle, tolerance) {
        Ok(mesh) => semio_framework_pack_json::to_json_string(&mesh),
        Err(error) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(error))])),
    }
}
pub fn dispose_geometry(&self, handle: &str) -> Result<(), String> {
    let mut claims = self.state.claims.try_lock().map_err(|_| "geometry claims owner busy")?;
    if self.is_closed() { return Err("geometry.session-closed".into()); }
    if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(handle)) { return Err("geometry.handle-retained-by-other-authority".into()); }
    let mut kernel = self.kernel().try_lock().map_err(|_| "geometry kernel owner busy")?;
    kernel.dispose(&GeometryHandle(handle.into()));
    if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(handle); }
    self.evict_mesh_cache_for_handle(handle);
    self.retire_jobs_for_handle(handle);
    Ok(())
}
pub fn export_solid_json(&self, handles: &[String], format: &str, deflection: f64) -> String {
    let shapes: Vec<GeometryHandle> = handles.iter().cloned().map(GeometryHandle).collect();
    let outcome: Result<(String, bool), BrepModuleError> = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy).and_then(|guard| {
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
        Ok((data, binary)) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([
            ("data".to_string(), semio_framework_pack_json::Value::String(data)),
            ("binary".to_string(), semio_framework_pack_json::Value::Bool(binary)),
            ("format".to_string(), semio_framework_pack_json::Value::String(format.to_string())),
        ])),
        Err(error) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(error.to_string()))])),
    }
}
pub fn import_solid_json(&self, format: &str, data: &str, tolerance: f64) -> String {
    let mut claims = self.state.claims.try_lock().expect("geometry claims");
    let outcome: Result<Vec<String>, BrepModuleError> = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy).and_then(|mut guard| {
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
        Ok(handles) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("handles".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&handles)))])),
        Err(error) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(error.to_string()))])),
    }
}
fn brep_invoke_inner(&self, method: &str, args_json: &str) -> Result<semio_framework_pack_json::Value, BrepModuleError> {
    let mut claims = self.state.claims.try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
    if self.is_closed() { return Err(BrepModuleError::InvalidArgs("geometry.session-closed".into())); }
    let args = invoke_args(args_json)?;
    let result = match method {
        "box" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.box_prim(arg_f64(&args, "width")?, arg_f64(&args, "depth")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sphere" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.sphere_prim(arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cylinder" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.cylinder_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cone" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.cone_prim(arg_f64(&args, "radius")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "lineCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.line_curve(arg_vec3(&args, "start")?, arg_vec3(&args, "end")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "circleCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.circle_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "arcCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .arc_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "radius")?, arg_f64(&args, "startAngle")?, arg_f64(&args, "endAngle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "ellipseCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .ellipse_curve(arg_vec3(&args, "center")?, arg_vec3(&args, "normal")?, arg_f64(&args, "semiMajor")?, arg_f64(&args, "semiMinor")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "interpolateCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.interpolate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "approximateCurve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .approximate_curve(&arg_points(&args, "points")?, arg_usize(&args, "degree")?, arg_usize(&args, "controlPoints")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "polylineWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.polyline_wire(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rectangleWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.rectangle_wire(arg_f64(&args, "width")?, arg_f64(&args, "height")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromPoints" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.planar_face_from_points(&arg_points(&args, "points")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "planarFaceFromWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.planar_face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrudeWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.extrude_wire(&arg_handle(&args, "wire")?, arg_vec3(&args, "vector")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "extrude" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.extrude(&arg_handle(&args, "face")?, arg_vec3(&args, "direction")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "revolve" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .revolve(&arg_handle(&args, "face")?, arg_vec3(&args, "axisOrigin")?, arg_vec3(&args, "axisDirection")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "loft" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.loft(&arg_handles(&args, "profiles")?, arg_bool_or(&args, "smooth", false)).map(handle_result).map_err(BrepModuleError::from)
        }
        "sweep" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.sweep(&arg_handle(&args, "profile")?, &arg_handle(&args, "path")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "thickenFace" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.thicken_face(&arg_handle(&args, "face")?, arg_f64(&args, "thickness")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "offsetFace" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.offset_face(&arg_handle(&args, "face")?, arg_f64(&args, "distance")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "fuse" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.fuse(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "cut" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.cut(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "intersect" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.intersect(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "translate" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.translate(&arg_handle(&args, "shape")?, arg_vec3(&args, "offset")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotate" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.rotate(&arg_handle(&args, "shape")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "rotateAbout" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard
                .rotate_about(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "axis")?, arg_f64(&args, "angle")?)
                .map(handle_result)
                .map_err(BrepModuleError::from)
        }
        "scale" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.scale(&arg_handle(&args, "shape")?, arg_f64(&args, "factor")?, arg_vec3(&args, "center")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "mirror" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.mirror(&arg_handle(&args, "shape")?, arg_vec3(&args, "origin")?, arg_vec3(&args, "normal")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "sewFaces" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.sew_faces(&arg_handles(&args, "faces")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "faceFromWire" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.face_from_wire(&arg_handle(&args, "wire")?).map(handle_result).map_err(BrepModuleError::from)
        }
        "healSolid" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.heal_solid(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-6)).map(handle_result).map_err(BrepModuleError::from)
        }
        "volume" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.volume(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "area" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.area(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "length" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.length(&arg_handle(&args, "shape")?).map(number_result).map_err(BrepModuleError::from)
        }
        "centerOfMass" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.center_of_mass(&arg_handle(&args, "shape")?).map(vec3_result).map_err(BrepModuleError::from)
        }
        "distance" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.distance(&arg_handle(&args, "a")?, &arg_handle(&args, "b")?).map(number_result).map_err(BrepModuleError::from)
        }
        "deconstruct" => {
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.deconstruct(&arg_handle(&args, "shape")?).map(topology_result).map_err(BrepModuleError::from)
        }
        "tessellate" => {
            let guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.tessellate(&arg_handle(&args, "shape")?, arg_f64_or(&args, "tolerance", 1e-3)).map(|mesh| mesh_result(&mesh)).map_err(BrepModuleError::from)
        }
        "dispose" => {
            let handle = arg_handle(&args, "handle")?;
            if claims.iter().any(|(authority, handles)| *authority != self.state.authority && handles.contains(&handle.0)) { return Err(BrepModuleError::InvalidArgs("geometry.handle-retained-by-other-authority".into())); }
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.dispose(&handle);
            if let Some(own) = claims.get_mut(&self.state.authority) { own.remove(&handle.0); }
            self.evict_mesh_cache_for_handle(&handle.0);
            self.retire_jobs_for_handle(&handle.0);
            Ok(unit_result())
        }
        "retain" => {
            let live = arg_handles(&args, "handles")?.into_iter().map(|handle| handle.0).collect::<Vec<_>>();
            claims.entry(self.state.authority).or_default().retain(&live);
            let merged = claims.values().flat_map(|handles| handles.iter().cloned()).collect::<HashSet<_>>();
            let mut guard = self.kernel().try_lock().map_err(|_| BrepModuleError::OwnerBusy)?;
            guard.retain(&merged);
            self.evict_mesh_cache_for_handles(&merged.into_iter().collect::<Vec<_>>());
            self.retain_tessellation_jobs(&live);
            Ok(unit_result())
        }
        other => {
            let mut guard = self.kernel().try_lock().map_err(|_|BrepModuleError::OwnerBusy)?;
            self.state.operations.invoke(&mut guard,other,&args)
        },
    };
    if let Ok(value) = &result {
        fn gather(value: &semio_framework_pack_json::Value, handles:&mut AuthorityClaims) {
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
        Ok(value) => semio_framework_pack_json::to_string(&value),
        Err(error) => semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".to_string(), semio_framework_pack_json::Value::String(error.to_string()))])),
    }
}
}

/// ⚓️ Retains the producing family while retiring a separately admitted job authority.
struct SessionPort {authority:SessionCapture,anchor:SessionCapture}
impl Drop for SessionPort {fn drop(&mut self) {if !std::thread::panicking() {assert!(self.authority.terminal_is_empty() && self.anchor.terminal_is_empty(),"geometry port requires terminal-empty retirement");}}}
impl semio_framework_os_flow::geometry::GeometryPort for SessionPort {
    fn retain(&self,handles:&[String]) {if let Some(authority)=self.authority.session.as_ref() {authority.retain_geometry_handles(handles);}}
    fn tessellate_step(&self,handle:&str,tolerance:f64,units:usize)->semio_framework_os_flow::geometry::GeometryStep {
        use semio_framework_os_flow::geometry::GeometryStep;
        let Some(authority)=self.authority.session.as_ref() else {return GeometryStep::Failed("geometry session is closed".into())};
        match authority.tessellate_step(handle,tolerance,units) {
            TessellationStepOutcome::Working {units_done,units_total,phase,..}=>GeometryStep::Working {units_done,units_total,phase:phase.into()},
            TessellationStepOutcome::Ready {mesh,..}=>GeometryStep::Ready(mesh),TessellationStepOutcome::Cancelled=>GeometryStep::Cancelled,
            TessellationStepOutcome::Failed {message}=>GeometryStep::Failed(message),
            TessellationStepOutcome::Invalid {issues}=>GeometryStep::Failed(issues.into_iter().map(|issue|issue.message).collect::<Vec<_>>().join("; ")),
        }
    }
    fn dispose(&self,handle:&str)->Result<(),String> {self.authority.session.as_ref().ok_or("geometry session is closed")?.dispose_geometry(handle)}
    fn cancel(&self)->usize {self.authority.session.as_ref().map_or(0,Session::cancel_all_tessellations)}
    fn begin_close(&self) {if let Some(authority)=self.authority.session.as_ref() {authority.begin_close();}}
    fn terminal_is_empty(&self)->bool {self.authority.terminal_is_empty() && self.anchor.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if self.authority.terminal_is_empty() {self.anchor.next_close_copy_byte_demand()}else {self.authority.next_close_copy_byte_demand()}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if self.authority.terminal_is_empty() {self.anchor.next_close_capacity_byte_demand(copy)}else {self.authority.next_close_capacity_byte_demand(copy)}}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {if self.authority.terminal_is_empty() {self.anchor.next_close_release_byte_demand()}else {self.authority.next_close_release_byte_demand()}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {if self.authority.terminal_is_empty() {self.anchor.next_close_depth_demand()}else {self.authority.next_close_depth_demand()}}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.authority.terminal_is_empty() {return self.authority.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        self.anchor.close_step(grant)
    }
}
#[cfg(all(target_arch = "wasm32", not(target_env = "p2"), feature = "browser-publication"))]
mod browser {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub struct BrowserSession {session:super::SessionCapture}
    #[wasm_bindgen]
    impl BrowserSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self { Self {session:super::SessionCapture::owned(super::Session::new())} }
        pub fn brep_invoke(&self, method: &str, arguments: &str) -> String { self.session.session.as_ref().map_or_else(||"{\"error\":\"geometry.session-closed\"}".into(),|session|session.brep_invoke_json(method,arguments)) }
        pub fn tessellate(&self, handle: &str, tolerance: f64) -> String { self.session.session.as_ref().map_or_else(||"{\"error\":\"geometry.session-closed\"}".into(),|session|session.tessellate_geometry_json_for_wasm(handle,tolerance)) }
        pub fn dispose(&self, handle: &str) -> Result<(), JsValue> { self.session.session.as_ref().ok_or_else(||JsValue::from_str("geometry.session-closed"))?.dispose_geometry(handle).map_err(|error|JsValue::from_str(&error)) }
        pub fn begin_close(&self) { self.session.begin_close(); }
        pub fn cancel_close(&self) { self.session.cancel_close(); }
        pub fn resume_close(&self) { self.session.resume_close(); }
        pub fn terminal_is_empty(&self) -> bool { self.session.terminal_is_empty() }
        pub fn next_close_copy_byte_demand(&self)->Result<usize,JsValue> {self.session.next_close_copy_byte_demand().map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,JsValue> {self.session.next_close_capacity_byte_demand(copy).map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn next_close_release_byte_demand(&self)->Result<usize,JsValue> {self.session.next_close_release_byte_demand().map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn next_close_depth_demand(&self)->Result<usize,JsValue> {self.session.next_close_depth_demand().map_err(|error|JsValue::from_str(&error.to_string()))}
        pub fn close_step(&mut self,maximum_items:usize,maximum_copy_bytes:usize,maximum_capacity_bytes:usize,maximum_release_bytes:usize,maximum_depth:usize)->String {
            match self.session.close_step(super::RetainedCloneGrant {maximum_items,maximum_copy_bytes,maximum_capacity_bytes,maximum_release_bytes,maximum_depth}) {
                Ok(step)=>{let progress=step.progress();let phase=if matches!(step,super::RetainedCloneStep::Complete(_)) {"complete"}else {"pending"};format!("{{\"phase\":\"{phase}\",\"items\":{},\"copyBytes\":{},\"capacityBytes\":{},\"releaseBytes\":{}}}",progress.copied_items,progress.copied_bytes,progress.retained_capacity_bytes,progress.released_bytes)},
                Err(error)=>semio_framework_pack_json::to_string(&semio_framework_pack_json::object([("error".into(),semio_framework_pack_json::Value::String(error.to_string()))])),
            }
        }
    }
}


/// 🧹️ Original analytic metadata keeps its exact typed owner until independently admitted release.
#[cfg(test)]
#[test]
fn retired_analytic_mesh_metadata_obeys_exact_byte_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧫️fixtures/🎯️component-picking/🔣️.json")).unwrap();
    let grants=&fixture["retirement"];let count=grants["labels"].as_u64().unwrap() as usize;let label_bytes=grants["labelBytes"].as_u64().unwrap() as usize;
    let mesh=semio_framework::MeshData {component_references:HistoryFoldIndex::from_iter([(String::from("face"),vec!["x".repeat(label_bytes);count])]),..Default::default()};
    let mut owner=controlled(mesh);let mut released=0;let mut turns=0;
    while !owner.terminal_is_empty() {
        let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let depth=owner.next_depth_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:release,maximum_depth:depth};
        if release>8 {assert_eq!(owner.step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap().progress(),RetainedCloneProgress::default());}
        let progress=owner.step(grant).unwrap().progress();assert!(progress.fits(grant));released+=progress.released_bytes;turns+=1;assert!(turns<1000000);
    }
    assert!(released>=count*label_bytes,"metadata labels require admitted physical release");
    println!("[DEBUG] Analytic metadata labels={count} repeatedEightByteRefusal=true releasedBytes={released} turns={turns}");
}

#[cfg(test)]
mod physical_session_custody {
    use super::*;
    use std::alloc::{GlobalAlloc,Layout,System};
    thread_local! {static EVENTS:std::cell::Cell<(bool,usize,usize)>=const {std::cell::Cell::new((false,0,0))};}
    struct Observer;
    fn event(birth:usize,release:usize) {let _=EVENTS.try_with(|events|{let(active,born,freed)=events.get();if active {events.set((true,born+birth,freed+release));}});}
    unsafe impl GlobalAlloc for Observer {
        unsafe fn alloc(&self,layout:Layout)->*mut u8 {let pointer=unsafe {System.alloc(layout)};if !pointer.is_null() {event(layout.size(),0);}pointer}
        unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout) {event(0,layout.size());unsafe {System.dealloc(pointer,layout)}}
        unsafe fn realloc(&self,pointer:*mut u8,layout:Layout,size:usize)->*mut u8 {let result=unsafe {System.realloc(pointer,layout,size)};if !result.is_null() {event(size,layout.size());}result}
    }
    #[global_allocator]
    static SYSTEM:Observer=Observer;
    fn observe<T>(work:impl FnOnce()->T)->(T,(usize,usize)) {EVENTS.with(|events|events.set((true,0,0)));let value=work();let counts=EVENTS.with(|events|{let(_,born,freed)=events.replace((false,0,0));(born,freed)});(value,counts)}
    #[test]
    fn original_session_capture_keeps_same_allocation_until_full_typed_grants() {
        let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
        let row=&fixture["box"];let custody=&fixture["physicalCustody"];
        let (mut capture,factory)=observe(|| {
            let source=Session::new();let handle=source.with_kernel(|kernel|kernel.box_prim_sync(row["width"].as_f64().unwrap(),row["depth"].as_f64().unwrap(),row["height"].as_f64().unwrap()).map_err(|error|map_kernel_error(&error))).unwrap();
            assert!(matches!(source.tessellate_step(handle.as_str(),0.05,0),TessellationStepOutcome::Working {..}));
            let mut warmed=false;
            for _ in 0..custody["maximumWarmTurns"].as_u64().unwrap() {
                match source.tessellate_step(handle.as_str(),0.05,1) {TessellationStepOutcome::Working {faces_done,..}=>{if faces_done>=custody["warmFaces"].as_u64().unwrap() as usize {warmed=true;break;}},other=>panic!("original partial tessellation did not remain owned: {other:?}")}
            }
            assert!(warmed,"original job must retain sampled edges and a partial mesh");
            let capture=source.capture();drop(handle);drop(source);capture
        });
        let retained=factory.0.checked_sub(factory.1).unwrap();let original=Arc::as_ptr(&capture.session.as_ref().unwrap().state);
        let (count,cancel)=observe(||capture.cancel_all_tessellations());assert_eq!(count,1);
        assert_eq!(cancel,(custody["cancelCapacityBytes"].as_u64().unwrap() as usize,custody["cancelReleaseBytes"].as_u64().unwrap() as usize));
        let handoff=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
        let (step,heap)=observe(||capture.close_step(handoff).unwrap());assert!(step.progress().fits(handoff));
        assert_eq!(heap,(custody["handoffCapacityBytes"].as_u64().unwrap() as usize,custody["handoffReleaseBytes"].as_u64().unwrap() as usize));
        assert_eq!(capture.allocation.as_ref().unwrap().as_ptr(),original);
        let (mut born,mut freed,mut refusals,mut turns,mut final_shell)=(0,0,0,0,0);
        while !capture.terminal_is_empty() {
            let copy=capture.next_close_copy_byte_demand().unwrap();let capacity=capture.next_close_capacity_byte_demand(copy).unwrap();let release=capture.next_close_release_byte_demand().unwrap();let depth=capture.next_close_depth_demand().unwrap();
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
            if release>custody["insufficientReleaseBytes"].as_u64().unwrap() as usize {
                for _ in 0..2 {let (step,heap)=observe(||capture.close_step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(capture.next_close_release_byte_demand().unwrap(),release);refusals+=1;}
            }
            let parent=capture.orphan.is_none() && capture.allocation.is_some();
            let (step,heap)=observe(||capture.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
            if parent && capture.terminal_is_empty() {assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(heap.1,release);final_shell+=1;}
        }
        assert_eq!(retained+born,freed);assert_eq!(final_shell,1);assert!(refusals>0);
        println!("[DEBUG] Original Session actualWeakPin=true warmedPartialMesh=true retainedSource={retained} admittedBirth={born} physicalRelease={freed} repeatedEightByteRefusals={refusals} turns={turns} separateFinalShell={final_shell}");
    }
}
