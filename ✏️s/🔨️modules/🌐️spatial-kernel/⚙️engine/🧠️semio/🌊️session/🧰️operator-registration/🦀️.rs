//! 🧰️ Operator helpers: dictionaries, channel specs, schemas, registration and the `BrepDeconstruct` operator.

use super::*;

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



pub fn map_kernel_error(error: &semio_framework_3d_brep_engine::BrepError) -> EvalError {
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
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)> {neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError> {Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_maximum_copy_bytes:usize)->Result<usize,ValueError> {Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError> {Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,ValueError> {Ok(1)}
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
