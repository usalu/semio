//! 🕸️ Borrowed framework DAG projection into explicitly named semantic entities.
use crate::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:&str)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}
use graph::manifest::PropertyValue;
use semio_framework_value::{DslValue, Number};
use store::sqlite_snapshot::{
    artifact::{insert_ieee754, Cell, FloatColumn, Projection},
    SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase,
};
const SQL: &str = include_str!("../🗄️.sql");
const NODE: &[FloatColumn] = &[FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10)];
const SLIDER: &[FloatColumn] = &[FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5)];
const NUMBER: &[FloatColumn] = &[FloatColumn::Binary64(2)];
fn order(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| invalid("DAG ordinal exceeds signed64"))
}
fn boolean(value: bool) -> Cell<'static> {
    Cell::Integer(i64::from(value))
}
fn optional(value: Option<&str>) -> Cell<'_> {
    value.map(Cell::Text).unwrap_or(Cell::Null)
}
fn forecast_push<T>(stack:&mut Vec<T>,value:T,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if stack.len()==stack.capacity(){let count=stack.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"DAG forecast frontier overflow"))?;let mut next=store::sqlite_snapshot::transfer::reserve(count,control)?;for(index,_)in stack.iter().enumerate(){if index%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index,stack.len())?;}}next.extend(std::mem::take(stack));*stack=next;}stack.push(value);Ok(())
}
struct Forecast<'a, 'b> {
    control: &'a mut SqliteSnapshotControl<'b>,
    rows: usize,
    work: usize,
}
impl Forecast<'_, '_> {
    fn add(&mut self, count: usize) -> Result<(), ValueError> {
        self.rows = self.rows.checked_add(count).ok_or_else(||work("DAG row overflow"))?;
        self.control.check_rows(self.rows)?;
        self.work = self.work.checked_add(1).ok_or_else(||work("DAG work overflow"))?;
        if self.work % 256 == 0 {
            self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.work, 0)?;
        }
        Ok(())
    }
    fn property(&mut self, value: &PropertyValue) -> Result<(), ValueError> {
        let mut stack=store::sqlite_snapshot::transfer::reserve(1,self.control)?;
        stack.push(value);
        while let Some(value) = stack.pop() {
            match value {
                PropertyValue::Null => self.add(1)?,
                PropertyValue::Bool(_) | PropertyValue::Number(_) | PropertyValue::String(_) => self.add(2)?,
                PropertyValue::Array(values) => {
                    self.add(1usize.checked_add(values.len()).ok_or_else(||work("DAG array overflow"))?)?;
                    for value in values {
                        self.add(0)?;
                        forecast_push(&mut stack,value,self.control)?;
                    }
                }
                PropertyValue::Object(values) => {
                    self.add(1usize.checked_add(values.len()).ok_or_else(||work("DAG object overflow"))?)?;
                    for value in values.values() {
                        self.add(0)?;
                        forecast_push(&mut stack,value,self.control)?;
                    }
                }
            }
        }
        Ok(())
    }
    fn intrinsic(&mut self, value: &DslValue) -> Result<(), ValueError> {
        let mut stack=store::sqlite_snapshot::transfer::reserve(1,self.control)?;
        stack.push(value);
        while let Some(value) = stack.pop() {
            match value {
                DslValue::Null => self.add(1)?,
                DslValue::Bool(_) | DslValue::Number(_) | DslValue::String(_) | DslValue::Bytes(_) => self.add(2)?,
                DslValue::Array(values) => {
                    self.add(1usize.checked_add(values.len()).ok_or_else(||work("DAG array overflow"))?)?;
                    for value in values {
                        self.add(0)?;
                        forecast_push(&mut stack,value,self.control)?;
                    }
                }
                DslValue::Object(values) => {
                    self.add(1usize.checked_add(values.len()).ok_or_else(||work("DAG object overflow"))?)?;
                    for (_, value) in values {
                        self.add(0)?;
                        forecast_push(&mut stack,value,self.control)?;
                    }
                }
            }
        }
        Ok(())
    }
    fn port(&mut self, port: &IoPortSpec) -> Result<(), ValueError> {
        self.add(1)?;
        for value in [&port.default, &port.value].into_iter().flatten() {
            self.add(1)?;
            self.intrinsic(value)?;
        }
        Ok(())
    }
}
pub(super) fn forecast(snapshot: &DagSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, ValueError> {
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
    if control.limits().max_tables < 40 || control.limits().max_columns < 21 || SQL.len() > control.limits().max_schema_bytes {
        return Err(work("DAG schema admission"));
    }
    let mut count = Forecast { control, rows: 0, work: 0 };
    count.add(1)?;
    for node in &snapshot.nodes {
        count.add(2)?;
        for property in node.properties.values() {
            count.add(1)?;
            count.property(property)?;
        }
        match &node.kind {
            DagNodeKind::Computation { inputs, outputs, .. } | DagNodeKind::Cluster { inputs, outputs } | DagNodeKind::AppInstance { inputs, outputs, .. } => {
                for port in inputs.iter().chain(outputs) {
                    count.port(port)?;
                }
            }
            DagNodeKind::Slider { output, .. } | DagNodeKind::Note { output, .. } | DagNodeKind::Image { output, .. } => count.port(output)?,
            DagNodeKind::Select { options, output, .. } => {
                count.add(options.len())?;
                count.port(output)?;
            }
            DagNodeKind::Screen { media, input } => {
                count.add(usize::from(media.is_some()))?;
                count.port(input)?;
            }
            DagNodeKind::Preview { content, expanded, input } => {
                count.add(expanded.len())?;
                match content {
                    DagPreviewContent::Empty => {}
                    DagPreviewContent::Scalar { .. } | DagPreviewContent::Image { .. } => count.add(1)?,
                    DagPreviewContent::Tree { json } => {
                        count.add(1)?;
                        count.intrinsic(json)?;
                    }
                }
                count.port(input)?;
            }
            DagNodeKind::Action { input, .. } | DagNodeKind::Export { input, .. } => count.port(input)?,
        }
    }
    for edge in &snapshot.edges {
        count.add(1)?;
        for property in edge.properties.values() {
            count.add(1)?;
            count.property(property)?;
        }
    }
    count.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, count.work, count.work)?;
    Ok(count.rows)
}
enum Edge<'a> {
    Array { parent: i64, ordinal: usize },
    Object { parent: i64, ordinal: usize, key: &'a str },
}
fn property(value: &PropertyValue, p: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    let mut stack=p.allocate_frontier(1)?;
    stack.push((None,value));
    let mut root = None;
    while let Some((edge, value)) = stack.pop() {
        let kind = match value {
            PropertyValue::Null => "null",
            PropertyValue::Bool(_) => "bool",
            PropertyValue::Number(_) => "number",
            PropertyValue::String(_) => "string",
            PropertyValue::Array(_) => "array",
            PropertyValue::Object(_) => "object",
        };
        let id = p.insert("dag_property_value", &[Cell::Text(kind)])?;
        match edge {
            None => root = Some(id),
            Some(Edge::Array { parent, ordinal }) => {
                p.insert("dag_property_array_element", &[Cell::Integer(parent), Cell::Integer(order(ordinal)?), Cell::Integer(id)])?;
            }
            Some(Edge::Object { parent, ordinal, key }) => {
                p.insert("dag_property_object_member", &[Cell::Integer(parent), Cell::Integer(order(ordinal)?), Cell::Text(key), Cell::Integer(id)])?;
            }
        }
        match value {
            PropertyValue::Null => {}
            PropertyValue::Bool(value) => {
                p.insert("dag_property_boolean", &[Cell::Integer(id), boolean(*value)])?;
            }
            PropertyValue::Number(value) => {
                insert_ieee754(p, "dag_property_number", &[Cell::Integer(id), Cell::Real(*value)], NUMBER)?;
            }
            PropertyValue::String(value) => {
                p.insert("dag_property_string", &[Cell::Integer(id), Cell::Text(value)])?;
            }
            PropertyValue::Array(values) => {
                for (ordinal, value) in values.iter().enumerate().rev() {
                    p.checkpoint()?;
                    p.push_frontier(&mut stack,(Some(Edge::Array { parent:id,ordinal }),value))?;
                }
            }
            PropertyValue::Object(values) => {
                for (ordinal, (key, value)) in values.iter().enumerate().rev() {
                    p.checkpoint()?;
                    p.push_frontier(&mut stack,(Some(Edge::Object { parent:id,ordinal,key }),value))?;
                }
            }
        }
        p.checkpoint()?;
    }
    root.ok_or_else(||invalid("DAG empty property root"))
}
fn intrinsic(value: &DslValue, p: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    let mut stack=p.allocate_frontier(1)?;
    stack.push((None,value));
    let mut root = None;
    while let Some((edge, value)) = stack.pop() {
        let kind = match value {
            DslValue::Null => "null",
            DslValue::Bool(_) => "bool",
            DslValue::Number(Number::UInt(_)) => "uint",
            DslValue::Number(Number::Int(_)) => "int",
            DslValue::Number(Number::Float(_)) => "float",
            DslValue::String(_) => "string",
            DslValue::Bytes(_) => "bytes",
            DslValue::Array(_) => "array",
            DslValue::Object(_) => "object",
        };
        let id = p.insert("dag_intrinsic_value", &[Cell::Text(kind)])?;
        match edge {
            None => root = Some(id),
            Some(Edge::Array { parent, ordinal }) => {
                p.insert("dag_intrinsic_array_element", &[Cell::Integer(parent), Cell::Integer(order(ordinal)?), Cell::Integer(id)])?;
            }
            Some(Edge::Object { parent, ordinal, key }) => {
                p.insert("dag_intrinsic_object_member", &[Cell::Integer(parent), Cell::Integer(order(ordinal)?), Cell::Text(key), Cell::Integer(id)])?;
            }
        }
        match value {
            DslValue::Null => {}
            DslValue::Bool(value) => {
                p.insert("dag_intrinsic_boolean", &[Cell::Integer(id), boolean(*value)])?;
            }
            DslValue::Number(Number::UInt(value)) => {
                p.insert(
                    "dag_intrinsic_unsigned",
                    &[Cell::Integer(id), Cell::Integer(i64::from(u32::try_from(value >> 32).map_err(|_|invalid("DAG unsigned high32"))?)), Cell::Integer(i64::from(u32::try_from(value & 0xffff_ffff).map_err(|_|invalid("DAG unsigned low32"))?))],
                )?;
            }
            DslValue::Number(Number::Int(value)) => {
                p.insert("dag_intrinsic_signed", &[Cell::Integer(id), Cell::Integer(*value)])?;
            }
            DslValue::Number(Number::Float(value)) => {
                insert_ieee754(p, "dag_intrinsic_float", &[Cell::Integer(id), Cell::Real(*value)], NUMBER)?;
            }
            DslValue::String(value) => {
                p.insert("dag_intrinsic_string", &[Cell::Integer(id), Cell::Text(value)])?;
            }
            DslValue::Bytes(value) => {
                p.insert("dag_intrinsic_bytes", &[Cell::Integer(id), Cell::Blob(value)])?;
            }
            DslValue::Array(values) => {
                for (ordinal, value) in values.iter().enumerate().rev() {
                    p.checkpoint()?;
                    p.push_frontier(&mut stack,(Some(Edge::Array { parent:id,ordinal }),value))?;
                }
            }
            DslValue::Object(values) => {
                for (ordinal, (key, value)) in values.iter().enumerate().rev() {
                    p.checkpoint()?;
                    p.push_frontier(&mut stack,(Some(Edge::Object { parent:id,ordinal,key }),value))?;
                }
            }
        }
        p.checkpoint()?;
    }
    root.ok_or_else(||invalid("DAG empty intrinsic root"))
}
fn port(value: &IoPortSpec, node: i64, side: &str, ordinal: usize, p: &mut Projection<'_, '_>) -> Result<(), ValueError> {
    let id = p.insert(
        "dag_io_port",
        &[
            Cell::Integer(node),
            Cell::Text(side),
            Cell::Integer(order(ordinal)?),
            Cell::Text(&value.id),
            Cell::Text(&value.label),
            Cell::Text(&value.code),
            Cell::Text(&value.abbreviation),
            Cell::Text(&value.full_name),
            optional(value.value_type.as_deref()),
            value.connected.map(boolean).unwrap_or(Cell::Null),
            optional(value.artifact_kind.as_deref()),
            Cell::Text(&value.cardinality),
            Cell::Text(match value.shape {
                PortShape::Semicircle => "semicircle",
                PortShape::Triangle => "triangle",
            }),
            boolean(value.visible),
            value.resolved.map(boolean).unwrap_or(Cell::Null),
        ],
    )?;
    for (table, value) in [("dag_port_default", &value.default), ("dag_port_value", &value.value)] {
        if let Some(value) = value {
            let value = intrinsic(value, p)?;
            p.insert(table, &[Cell::Integer(id), Cell::Integer(value)])?;
        }
    }
    Ok(())
}
fn ports(values: &[IoPortSpec], node: i64, side: &str, p: &mut Projection<'_, '_>) -> Result<(), ValueError> {
    for (ordinal, value) in values.iter().enumerate() {
        port(value, node, side, ordinal, p)?;
    }
    Ok(())
}
pub(super) fn project(snapshot: &DagSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    let total = forecast(snapshot, control)?;
    let mut p = Projection::new(SQL, control)?;
    let document = p.insert("dag_document", &[Cell::Text(&snapshot.schema)])?;
    for (ordinal, node) in snapshot.nodes.iter().enumerate() {
        let id = insert_ieee754(
            &mut p,
            "dag_node",
            &[
                Cell::Integer(document),
                Cell::Integer(order(ordinal)?),
                Cell::Text(&node.id),
                Cell::Text(&node.name),
                Cell::Text(&node.abbreviation),
                Cell::Text(&node.icon),
                Cell::Real(node.x),
                Cell::Real(node.y),
                Cell::Real(node.width),
                Cell::Real(node.height),
                optional(node.operator_kind.as_deref()),
                Cell::Text(dag_node_kind_tag(&node.kind)),
            ],
            NODE,
        )?;
        for (key, value) in &node.properties {
            let value = property(value, &mut p)?;
            p.insert("dag_node_property", &[Cell::Integer(id), Cell::Text(key), Cell::Integer(value)])?;
        }
        match &node.kind {
            DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => {
                p.insert("dag_computation", &[Cell::Integer(id), boolean(*variadic_inputs), boolean(*variadic_outputs)])?;
                ports(inputs, id, "input", &mut p)?;
                ports(outputs, id, "output", &mut p)?;
            }
            DagNodeKind::Slider { min, max, step, value, output } => {
                insert_ieee754(&mut p, "dag_slider", &[Cell::Integer(id), Cell::Real(*min), Cell::Real(*max), Cell::Real(*step), Cell::Real(*value)], SLIDER)?;
                port(output, id, "output", 0, &mut p)?;
            }
            DagNodeKind::Select { options, selected, output } => {
                let select = p.insert(
                    "dag_select",
                    &[Cell::Integer(id), Cell::Integer(i64::from(u32::try_from(selected >> 32).map_err(|_|invalid("DAG selection high32"))?)), Cell::Integer(i64::from(u32::try_from(selected & 0xffff_ffff).map_err(|_|invalid("DAG selection low32"))?))],
                )?;
                for (ordinal, value) in options.iter().enumerate() {
                    p.insert("dag_select_option", &[Cell::Integer(select), Cell::Integer(order(ordinal)?), Cell::Text(value)])?;
                }
                port(output, id, "output", 0, &mut p)?;
            }
            DagNodeKind::Screen { media, input } => {
                let screen = p.insert("dag_screen", &[Cell::Integer(id)])?;
                if let Some(media) = media {
                    p.insert(
                        "dag_screen_media",
                        &[
                            Cell::Integer(screen),
                            Cell::Text(match media.kind {
                                DagMediaKind::Image => "image",
                                DagMediaKind::Svg => "svg",
                                DagMediaKind::Pdf => "pdf",
                                DagMediaKind::Video => "video",
                            }),
                            Cell::Text(&media.src),
                        ],
                    )?;
                }
                port(input, id, "input", 0, &mut p)?;
            }
            DagNodeKind::Note { text, output } => {
                p.insert("dag_note", &[Cell::Integer(id), Cell::Text(text)])?;
                port(output, id, "output", 0, &mut p)?;
            }
            DagNodeKind::Image { src, output } => {
                p.insert("dag_image", &[Cell::Integer(id), Cell::Text(src)])?;
                port(output, id, "output", 0, &mut p)?;
            }
            DagNodeKind::Preview { content, expanded, input } => {
                let preview = p.insert(
                    "dag_preview",
                    &[
                        Cell::Integer(id),
                        Cell::Text(match content {
                            DagPreviewContent::Empty => "empty",
                            DagPreviewContent::Scalar { .. } => "scalar",
                            DagPreviewContent::Image { .. } => "image",
                            DagPreviewContent::Tree { .. } => "tree",
                        }),
                    ],
                )?;
                match content {
                    DagPreviewContent::Empty => {}
                    DagPreviewContent::Scalar { text } => {
                        p.insert("dag_preview_scalar", &[Cell::Integer(preview), Cell::Text(text)])?;
                    }
                    DagPreviewContent::Image { src } => {
                        p.insert("dag_preview_image", &[Cell::Integer(preview), Cell::Text(src)])?;
                    }
                    DagPreviewContent::Tree { json } => {
                        let value = intrinsic(json, &mut p)?;
                        p.insert("dag_preview_tree", &[Cell::Integer(preview), Cell::Integer(value)])?;
                    }
                }
                for (ordinal, path) in expanded.iter().enumerate() {
                    p.insert("dag_preview_expanded", &[Cell::Integer(preview), Cell::Integer(order(ordinal)?), Cell::Text(path)])?;
                }
                port(input, id, "input", 0, &mut p)?;
            }
            DagNodeKind::Action { label, input } => {
                p.insert("dag_action", &[Cell::Integer(id), Cell::Text(label)])?;
                port(input, id, "input", 0, &mut p)?;
            }
            DagNodeKind::Export { label, format, input } => {
                p.insert("dag_export", &[Cell::Integer(id), Cell::Text(label), Cell::Text(format)])?;
                port(input, id, "input", 0, &mut p)?;
            }
            DagNodeKind::Cluster { inputs, outputs } => {
                p.insert("dag_cluster", &[Cell::Integer(id)])?;
                ports(inputs, id, "input", &mut p)?;
                ports(outputs, id, "output", &mut p)?;
            }
            DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs, outputs } => {
                p.insert("dag_app_instance", &[Cell::Integer(id), Cell::Text(instance_id), Cell::Text(plugin_id), Cell::Text(app_id), Cell::Text(icon)])?;
                ports(inputs, id, "input", &mut p)?;
                ports(outputs, id, "output", &mut p)?;
            }
        }
        p.checkpoint_total(total)?;
    }
    for (ordinal, edge) in snapshot.edges.iter().enumerate() {
        let id = p.insert(
            "dag_edge",
            &[
                Cell::Integer(document),
                Cell::Integer(order(ordinal)?),
                Cell::Text(&edge.id),
                Cell::Text(&edge.source),
                Cell::Text(&edge.target),
                Cell::Text(match edge.route_style {
                    EdgeRouteStyle::Bezier => "bezier",
                    EdgeRouteStyle::SharpSz => "sharpSz",
                }),
            ],
        )?;
        for (key, value) in &edge.properties {
            let value = property(value, &mut p)?;
            p.insert("dag_edge_property", &[Cell::Integer(id), Cell::Text(key), Cell::Integer(value)])?;
        }
        p.checkpoint_total(total)?;
    }
    p.checkpoint_total(total)?;
    p.finish()
}
