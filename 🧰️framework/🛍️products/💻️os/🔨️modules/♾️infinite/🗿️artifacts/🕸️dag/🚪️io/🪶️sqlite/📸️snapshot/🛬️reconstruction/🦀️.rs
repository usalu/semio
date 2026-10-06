//! 🕸️ Validated borrowed framework DAG entity indexes and explicit owned scalar reconstruction.
use crate::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
use semio_framework_value::{DslValue, FromValue};
use super::values::{property,intrinsic};
use store::sqlite_snapshot::{
    artifact::{FloatColumn, FloatRow, Reconstruction},
    SqliteDatabase, SqliteRow, SqliteSnapshotControl, SqliteValue,
};
pub(super) const SQL: &str = include_str!("../🗄️.sql");
pub(super) const WIDTHS: &[(&str, usize)] = &[
    ("dag_document", 2),
    ("dag_node", 21),
    ("dag_edge", 7),
    ("dag_io_port", 16),
    ("dag_computation", 4),
    ("dag_slider", 14),
    ("dag_select", 4),
    ("dag_select_option", 4),
    ("dag_screen", 2),
    ("dag_screen_media", 4),
    ("dag_note", 3),
    ("dag_image", 3),
    ("dag_preview", 3),
    ("dag_preview_scalar", 3),
    ("dag_preview_image", 3),
    ("dag_preview_tree", 3),
    ("dag_preview_expanded", 4),
    ("dag_action", 3),
    ("dag_export", 4),
    ("dag_cluster", 2),
    ("dag_app_instance", 6),
    ("dag_node_property", 4),
    ("dag_edge_property", 4),
    ("dag_property_value", 2),
    ("dag_property_boolean", 3),
    ("dag_property_number", 5),
    ("dag_property_string", 3),
    ("dag_property_array_element", 4),
    ("dag_property_object_member", 5),
    ("dag_intrinsic_value", 2),
    ("dag_intrinsic_boolean", 3),
    ("dag_intrinsic_unsigned", 4),
    ("dag_intrinsic_signed", 3),
    ("dag_intrinsic_float", 5),
    ("dag_intrinsic_string", 3),
    ("dag_intrinsic_bytes", 3),
    ("dag_intrinsic_array_element", 4),
    ("dag_intrinsic_object_member", 5),
    ("dag_port_default", 3),
    ("dag_port_value", 3),
];
use super::rows::Read;
impl Read<'_, '_, '_>{
    pub(super) fn text(&mut self, row: &SqliteRow, column: usize) -> Result<String, ValueError> {
        Reconstruction::new(self.control)?.text(row.text(column)?)
    }
    pub(super) fn optional_text(&mut self, row: &SqliteRow, column: usize) -> Result<Option<String>, ValueError> {
        row.optional_text(column)?.map(|value| Reconstruction::new(self.control)?.text(value)).transpose()
    }
    pub(super) fn boolean(&mut self, row: &SqliteRow, column: usize) -> Result<bool, ValueError> {
        self.step()?;
        match row.integer(column)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(invalid("DAG boolean is not zero or one")),
        }
    }
    pub(super) fn optional_boolean(&mut self, row: &SqliteRow, column: usize) -> Result<Option<bool>, ValueError> {
        match row.values.get(column) {
            Some(SqliteValue::Null) => Ok(None),
            Some(SqliteValue::Integer(_)) => self.boolean(row, column).map(Some),
            _ => Err(invalid("DAG optional boolean storage differs")),
        }
    }
    pub(super) fn unsigned(&mut self, row: &SqliteRow, column: usize) -> Result<u64, ValueError> {
        self.step()?;
        let high = u32::try_from(row.integer(column)?).map_err(|_|invalid("DAG unsigned high word exceeds32bits"))?;
        let low = u32::try_from(row.integer(column + 1)?).map_err(|_|invalid("DAG unsigned low word exceeds32bits"))?;
        Ok((u64::from(high) << 32) | u64::from(low))
    }
}
const NODE: &[FloatColumn] = &[FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10)];
const SLIDER: &[FloatColumn] = &[FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5)];
fn retire_ports(values: Vec<IoPortSpec>) {
    for mut value in values {
        if let Some(value) = value.default.take() {
            DslValue::retire_decoded(value)
        }
        if let Some(value) = value.value.take() {
            DslValue::retire_decoded(value)
        }
    }
}
struct Ports {
    inputs: Vec<IoPortSpec>,
    outputs: Vec<IoPortSpec>,
}
impl Drop for Ports {
    fn drop(&mut self) {
        retire_ports(std::mem::take(&mut self.inputs));
        retire_ports(std::mem::take(&mut self.outputs));
    }
}
impl Ports {
    fn one(mut self, input: bool) -> Result<IoPortSpec, ValueError> {
        let (actual, other) = if input { (&mut self.inputs, &self.outputs) } else { (&mut self.outputs, &self.inputs) };
        if actual.len() != 1 || !other.is_empty() {
            return Err(invalid("DAG scalar node port cardinality differs"));
        }
        Ok(actual.pop().unwrap())
    }
}
fn ports(node:i64,read:&mut Read<'_,'_,'_>)->Result<Ports,ValueError>{
 let mut rows=read.relations("dag_io_port",1,node)?;
 for row in &rows{if !matches!(row.text(2)?,"input"|"output"){return Err(invalid("DAG port side differs"))}if row.integer(3)?<0{return Err(invalid("DAG port ordinal is negative"))}read.step()?;}
 super::rows::sort(&mut rows,&mut read.work,read.control,|left,right|Ok((left.text(2)?,left.integer(3)?).cmp(&(right.text(2)?,right.integer(3)?))))?;
 let inputs=rows.iter().take_while(|row|row.text(2).unwrap()=="input").count();
 let outputs=rows.len()-inputs;
 let mut result=Ports{inputs:super::values::reserve(inputs,read.control)?,outputs:super::values::reserve(outputs,read.control)?};
 for row in rows{
  let side=row.text(2)?;let ordinal=usize::try_from(row.integer(3)?).map_err(|_|invalid("DAG port ordinal is negative"))?;
  let values=if side=="input"{&mut result.inputs}else{&mut result.outputs};
  if ordinal!=values.len(){return Err(invalid("DAG port ordinals must be unique and contiguous per side"))}
  values.push(IoPortSpec{id:String::new(),label:String::new(),code:String::new(),abbreviation:String::new(),full_name:String::new(),value_type:None,default:None,value:None,connected:None,artifact_kind:None,cardinality:String::new(),shape:PortShape::Semicircle,visible:true,resolved:None});
  let value=values.last_mut().unwrap();
  value.id=read.text(row,4)?;value.label=read.text(row,5)?;value.code=read.text(row,6)?;value.abbreviation=read.text(row,7)?;value.full_name=read.text(row,8)?;value.value_type=read.optional_text(row,9)?;value.connected=read.optional_boolean(row,10)?;value.artifact_kind=read.optional_text(row,11)?;value.cardinality=read.text(row,12)?;
  value.shape=match row.text(13)?{"semicircle"=>PortShape::Semicircle,"triangle"=>PortShape::Triangle,_=>return Err(invalid("DAG port shape differs"))};value.visible=read.boolean(row,14)?;value.resolved=read.optional_boolean(row,15)?;
  if let Some(row)=read.optional_child("dag_port_default",row.rowid)?{value.default=Some(intrinsic(row.integer(2)?,read)?);}
  if let Some(row)=read.optional_child("dag_port_value",row.rowid)?{value.value=Some(intrinsic(row.integer(2)?,read)?);}
 }
 Ok(result)
}
fn bag(table:&'static str,owner:i64,read:&mut Read<'_,'_,'_>)->Result<graph::manifest::PropertyBag,ValueError>{
 let rows=read.relations(table,1,owner)?;
 let mut result=semio_framework_value::DecodedValue::new(graph::manifest::PropertyBag::from_admitted(super::values::reserve(rows.len(),read.control)?),<graph::manifest::PropertyBag as FromValue>::retire_decoded);
 for row in rows{let key=read.text(row,2)?;if result.get_mut().contains_key(&key){return Err(invalid("DAG property bag keys must be unique"))}let value=property(row.integer(3)?,read)?;result.get_mut().insert_controlled(key,value,&mut||read.step())?;read.step()?;}
 Ok(result.take())
}
struct PreviewOwner(Option<DagPreviewContent>);
impl Drop for PreviewOwner {
    fn drop(&mut self) {
        if let Some(DagPreviewContent::Tree { json }) = self.0.take() {
            DslValue::retire_decoded(json)
        }
    }
}
fn kind(node: i64, tag: &str, read: &mut Read<'_, '_, '_>) -> Result<DagNodeKind, ValueError> {
    Ok(match tag {
        "computation" => {
            let row = read.child("dag_computation", node)?;
            let variadic_inputs = read.boolean(row, 2)?;
            let variadic_outputs = read.boolean(row, 3)?;
            let mut ports = ports(node, read)?;
            DagNodeKind::Computation { inputs: std::mem::take(&mut ports.inputs), outputs: std::mem::take(&mut ports.outputs), variadic_inputs, variadic_outputs }
        }
        "slider" => {
            let row = FloatRow::new(read.child("dag_slider", node)?, SLIDER)?;
            let min = row.real(2)?;
            let max = row.real(3)?;
            let step = row.real(4)?;
            let value = row.real(5)?;
            DagNodeKind::Slider { min, max, step, value, output: ports(node, read)?.one(false)? }
        }
        "select" => {
            let row = read.child("dag_select", node)?;
            let selected = read.unsigned(row, 2)?;
            let rows = read.ordered("dag_select_option", 1, row.rowid, 2)?;
            let mut options = super::values::reserve(rows.len(),read.control)?;
            for row in rows {
                options.push(read.text(row, 3)?);
                read.step()?;
            }
            DagNodeKind::Select { selected, options, output: ports(node, read)?.one(false)? }
        }
        "screen" => {
            let screen = read.child("dag_screen", node)?;
            let media = if let Some(row) = read.optional_child("dag_screen_media", screen.rowid)? {
                let kind = match row.text(2)? {
                    "image" => DagMediaKind::Image,
                    "svg" => DagMediaKind::Svg,
                    "pdf" => DagMediaKind::Pdf,
                    "video" => DagMediaKind::Video,
                    _ => return Err(invalid("DAG media kind differs")),
                };
                Some(DagMedia { kind, src: read.text(row, 3)? })
            } else {
                None
            };
            DagNodeKind::Screen { media, input: ports(node, read)?.one(true)? }
        }
        "note" => {
            let row = read.child("dag_note", node)?;
            let text = read.text(row, 2)?;
            DagNodeKind::Note { text, output: ports(node, read)?.one(false)? }
        }
        "image" => {
            let row = read.child("dag_image", node)?;
            let src = read.text(row, 2)?;
            DagNodeKind::Image { src, output: ports(node, read)?.one(false)? }
        }
        "preview" => {
            let row = read.child("dag_preview", node)?;
            let content = match row.text(2)? {
                "empty" => DagPreviewContent::Empty,
                "scalar" => {
                    let row = read.child("dag_preview_scalar", row.rowid)?;
                    DagPreviewContent::Scalar { text: read.text(row, 2)? }
                }
                "image" => {
                    let row = read.child("dag_preview_image", row.rowid)?;
                    DagPreviewContent::Image { src: read.text(row, 2)? }
                }
                "tree" => {
                    let row = read.child("dag_preview_tree", row.rowid)?;
                    DagPreviewContent::Tree { json: intrinsic(row.integer(2)?, read)? }
                }
                _ => return Err(invalid("DAG preview variant differs")),
            };
            let mut content = PreviewOwner(Some(content));
            let rows=read.ordered("dag_preview_expanded",1,row.rowid,2)?;
            let mut expanded=DagExpandedPaths::from_admitted(super::values::reserve(rows.len(),read.control)?);
            for row in rows {
                let key=read.text(row,3)?;
                if !expanded.insert_controlled(key,&mut||read.step())? {
                    return Err(invalid("DAG preview expanded paths must be unique"));
                }
                read.step()?;
            }
            let input = ports(node, read)?.one(true)?;
            DagNodeKind::Preview { content: content.0.take().unwrap(), expanded, input }
        }
        "action" => {
            let row = read.child("dag_action", node)?;
            let label = read.text(row, 2)?;
            DagNodeKind::Action { label, input: ports(node, read)?.one(true)? }
        }
        "export" => {
            let row = read.child("dag_export", node)?;
            let label = read.text(row, 2)?;
            let format = read.text(row, 3)?;
            DagNodeKind::Export { label, format, input: ports(node, read)?.one(true)? }
        }
        "cluster" => {
            read.child("dag_cluster", node)?;
            let mut ports = ports(node, read)?;
            DagNodeKind::Cluster { inputs: std::mem::take(&mut ports.inputs), outputs: std::mem::take(&mut ports.outputs) }
        }
        "appInstance" => {
            let row = read.child("dag_app_instance", node)?;
            let instance_id = read.text(row, 2)?;
            let plugin_id = read.text(row, 3)?;
            let app_id = read.text(row, 4)?;
            let icon = read.text(row, 5)?;
            let mut ports = ports(node, read)?;
            DagNodeKind::AppInstance { instance_id, plugin_id, app_id, icon, inputs: std::mem::take(&mut ports.inputs), outputs: std::mem::take(&mut ports.outputs) }
        }
        _ => return Err(invalid("DAG node kind differs")),
    })
}
struct SnapshotOwner(Option<DagSnapshot>);
impl Drop for SnapshotOwner {
    fn drop(&mut self) {
        if let Some(snapshot) = self.0.take() {
            <DagSnapshot as semio_framework_dsl_record::DslField>::retire_decoded(snapshot);
        }
    }
}
/// 📸️ Reconstructs the actual persisted root and every declared node, edge, port and payload entity.
pub(super) fn reconstruct(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<DagSnapshot, ValueError> {
    let mut read = Read::new(database, control)?;
    let document = database.table("dag_document")?.single_row()?;
    let document = read.take("dag_document", document.rowid)?;
    let mut result = SnapshotOwner(Some(DagSnapshot { schema: read.text(document, 1)?, nodes: Vec::new(), edges: Vec::new() }));
    let nodes=read.ordered("dag_node",1,document.rowid,2)?;
    let edges=read.ordered("dag_edge",1,document.rowid,2)?;
    result.0.as_mut().unwrap().nodes=super::values::reserve(nodes.len(),read.control)?;
    result.0.as_mut().unwrap().edges=super::values::reserve(edges.len(),read.control)?;
    for row in nodes {
        result.0.as_mut().unwrap().nodes.push(DagNodeSpec::default());
        let value = result.0.as_mut().unwrap().nodes.last_mut().unwrap();
        value.id = read.text(row, 3)?;
        value.name = read.text(row, 4)?;
        value.abbreviation = read.text(row, 5)?;
        value.icon = read.text(row, 6)?;
        let floats = FloatRow::new(row, NODE)?;
        value.x = floats.real(7)?;
        value.y = floats.real(8)?;
        value.width = floats.real(9)?;
        value.height = floats.real(10)?;
        value.operator_kind = read.optional_text(row, 11)?;
        value.kind = kind(row.rowid, row.text(12)?, &mut read)?;
        value.properties = bag("dag_node_property", row.rowid, &mut read)?;
    }
    for row in edges {
        let route_style = match row.text(6)? {
            "bezier" => EdgeRouteStyle::Bezier,
            "sharpSz" => EdgeRouteStyle::SharpSz,
            _ => return Err(invalid("DAG edge route differs")),
        };
        let edge = DagHostSnapshotEdge { id: read.text(row, 3)?, source: read.text(row, 4)?, target: read.text(row, 5)?, route_style, properties: bag("dag_edge_property", row.rowid, &mut read)? };
        result.0.as_mut().unwrap().edges.push(edge);
    }
    read.finish()?;
    Ok(result.0.take().unwrap())
}
