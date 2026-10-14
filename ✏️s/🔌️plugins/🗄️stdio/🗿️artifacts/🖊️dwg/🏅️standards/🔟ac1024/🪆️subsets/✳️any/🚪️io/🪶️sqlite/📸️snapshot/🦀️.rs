//! 🖊️ DWG's explicit domain relational schema and typed ownership modules.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError,ValueRefusalKind};
#[path="📏️encoding/🦀️.rs"]
pub mod encoding;
use semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding;
use semio_framework_os_kernel::sqlite_snapshot::composition::{sqlite_schema_bytes, sqlite_schema_len, sqlite_schema_text};
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_dwg_snapshot_objects::standards::v_ac1024::subsets::any::schema::snapshot::objects::DwgLogicalObjectBody;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::number;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::reader;
#[path="📄️document/🦀️.rs"]
pub mod document;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::header;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::drawing;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::tables;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::records;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::entities;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::objects;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::associativity;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::evaluation;
use semio_s_artifact_stdio_dwg_sqlite_objects::standards::v_ac1024::subsets::any::io::sqlite::snapshot::blocks;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::actions;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::visual_style;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::material;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::table_style;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::layout;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::mleader_style;
use semio_s_artifact_stdio_dwg_sqlite_drawing::standards::v_ac1024::subsets::any::io::sqlite::snapshot::constraints;

use semio_framework_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl}};
#[path="📏️encoding/🛬️admission/🦀️.rs"]
pub mod native_admission;

impl ArtifactSqliteSnapshotHost for DwgSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{admit_native_rows(self,semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative,control)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)}
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let limits=control.limits();let snapshot=store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {construct_native_record(record,native,limits)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)?;admit_native_rows(&snapshot,semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::DecodeNative,control)?;Ok(snapshot)}
    fn preflight_sqlite_snapshot_encoding(&self,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{encoding::preflight(self,control)}

 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
  control.checkpoint(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
  if dialect.artifact_kind!="s.stdio.dwg"||!matches!(dialect.standard.as_str(),"ac1018"|"ac1024")||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG owned SQLite dialect must be an exact AC1018 or AC1024 full snapshot"));}
  let candidate=Self::reconstruct_sqlite_database(database,control)?;
  if candidate.project_sqlite_database(control)?!=self.project_sqlite_database(control)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG owned state differs from its semantic projection"));}
  Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))
 })().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}
 
 
 
}
/// 🧵️ The DDL each projection module owns, in table-creation order.
const SQLITE_SCHEMA_SEGMENTS: &[&str] = &[document::SQL, "\n", header::SQL, "\n", drawing::SQL, "\n", tables::SQL, "\n", entities::SQL, "\n", objects::SQL, "\n", associativity::SQL, "\n", evaluation::SQL, "\n", blocks::SQL, "\n", actions::SQL, "\n", visual_style::SQL, "\n", material::SQL, "\n", table_style::SQL, "\n", layout::SQL, "\n", mleader_style::SQL, "\n", constraints::SQL];
/// 🗄️ The composed relational schema text of every owned DWG table.
const SQLITE_SCHEMA_BYTES: [u8; sqlite_schema_len(SQLITE_SCHEMA_SEGMENTS)] = sqlite_schema_bytes(SQLITE_SCHEMA_SEGMENTS);
impl pack::snapshot::ArtifactSqliteSnapshot for DwgSnapshot {
const SQLITE_SCHEMA:&'static str=sqlite_schema_text(&SQLITE_SCHEMA_BYTES);
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Self::reconstruct_sqlite_database(database,control)}
}
impl DwgSnapshot {
/// 🗄️ Projects the owned DWG relational fields with typed refusals.
fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let mut projection=number::Projection::new(Self::SQLITE_SCHEMA,control)?;
  document::project(&mut projection,self)?;header::project(&mut projection,&self.header)?;
  drawing::project(&mut projection,&self.drawing,project_body)?;
  projection.finish()
 }
/// 🫳️ Reconstructs the same owned relational snapshot with typed refusals.
fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let mut reader=reader::Reader::new(database,Self::SQLITE_SCHEMA,control)?;
  let document=document::reconstruct(&mut reader)?;let header=header::reconstruct(&mut reader)?;
  let drawing=drawing::reconstruct(&mut reader,reconstruct_body)?;reader.finish()?;
  Ok(Self{schema:document.schema,version:document.version,maintenance_version:document.maintenance_version,codepage:document.codepage,drawing,header,classes:document.classes,dependencies:document.dependencies,summary:document.summary,application:document.application,template:document.template,auxiliary_header:document.auxiliary_header,revision_history:document.revision_history,preview:document.preview,application_history:document.application_history})
 }
}
pub fn construct_native_record(record:&semio_framework_dsl_record::RecordValue,native:&mut semio_framework_value::NativeDecodeControl<'_>,limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits)->Result<DwgSnapshot,ValueError>{native_admission::root(record,native,limits)?;DwgSnapshot::__dsl_from_record_controlled(record,native)}
#[cfg(test)]
pub fn forecast_native_rows(record:&semio_framework_dsl_record::RecordValue,native:&mut semio_framework_value::NativeDecodeControl<'_>,limits:semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits)->Result<usize,ValueError>{native_admission::root(record,native,limits)}
fn admit_native_rows(snapshot:&DwgSnapshot,phase:semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<usize,ValueError>{
    let mut projection=number::Projection::admission(DwgSnapshot::SQLITE_SCHEMA,phase,control)?;
    document::project(&mut projection,snapshot)?;header::project(&mut projection,&snapshot.header)?;
    drawing::project(&mut projection,&snapshot.drawing,project_body)?;projection.finish_admission()
}
fn project_body(p:&mut number::Projection<'_,'_>,id:i64,value:&DwgLogicalObjectBody)->Result<(),ValueError>{match value{
 DwgLogicalObjectBody::Dictionary(v)=>tables::project_dictionary(p,id,v),
 DwgLogicalObjectBody::TableControl(v)=>tables::project_control(p,id,v),
 DwgLogicalObjectBody::TableRecord(v)=>records::project_record(p,id,v),
 DwgLogicalObjectBody::XRecord(v)=>tables::project_xrecord(p,id,v),
 DwgLogicalObjectBody::Entity(v)=>entities::project(p,id,v),
 DwgLogicalObjectBody::AssociativeDependency(v)=>associativity::project_dependency(p,id,v),
 DwgLogicalObjectBody::AssociativeValueDependency(v)=>associativity::project_value_dependency(p,id,v),
 DwgLogicalObjectBody::AssociativeGeometryDependency(v)=>associativity::project_geometry_dependency(p,id,v),
 DwgLogicalObjectBody::BlockGripLocationComponent(v)=>evaluation::project_grip_location(p,id,v),
 DwgLogicalObjectBody::DynamicBlockProxyNode(v)=>evaluation::project_proxy(p,id,v),
 DwgLogicalObjectBody::AssociativeVariable(v)=>associativity::project_variable(p,id,v),
 DwgLogicalObjectBody::AssociativeDimensionDependencyBody(v)=>objects::project_dimension_dependency_body(p,id,v),
 DwgLogicalObjectBody::VisualStyle(v)=>visual_style::project(p,id,v),
 DwgLogicalObjectBody::BlockParameterDependencyBody(v)=>objects::project_block_parameter_dependency_body(p,id,v),
 DwgLogicalObjectBody::BlockRepresentationData(v)=>objects::project_representation_data(p,id,v),
 DwgLogicalObjectBody::DynamicBlockPurgePreventer(v)=>objects::project_purge_preventer(p,id,v),
 DwgLogicalObjectBody::EvaluationGraph(v)=>objects::project_graph(p,id,v),
 DwgLogicalObjectBody::BlockFlipParameter(v)=>blocks::project_flip_parameter(p,id,v),
 DwgLogicalObjectBody::BlockVisibilityParameter(v)=>blocks::project_visibility_parameter(p,id,v),
 DwgLogicalObjectBody::Placeholder(v)=>objects::project_placeholder(p,id,v),
 DwgLogicalObjectBody::DictionaryVariable(v)=>objects::project_dictionary_variable(p,id,v),
 DwgLogicalObjectBody::AnnotationScale(v)=>objects::project_annotation_scale(p,id,v),
 DwgLogicalObjectBody::SortEntitiesTable(v)=>objects::project_sort_entities_table(p,id,v),
 DwgLogicalObjectBody::TableStyle(v)=>table_style::project(p,id,v),
 DwgLogicalObjectBody::MlineStyle(v)=>material::project_mline(p,id,v),
 DwgLogicalObjectBody::MLeaderStyle(v)=>mleader_style::project(p,id,v),
 DwgLogicalObjectBody::Material(v)=>material::project_material(p,id,v),
 DwgLogicalObjectBody::BlockMoveAction(v)=>actions::project_move(p,id,v),
 DwgLogicalObjectBody::AssocNetwork(v)=>associativity::project_network(p,id,v),
 DwgLogicalObjectBody::Assoc2dConstraintGroup(v)=>constraints::project(p,id,v),
 DwgLogicalObjectBody::BlockLinearParameter(v)=>blocks::project_linear_parameter(p,id,v),
 DwgLogicalObjectBody::BlockLinearGrip(v)=>blocks::project_linear_grip(p,id,v),
 DwgLogicalObjectBody::BlockFlipGrip(v)=>blocks::project_flip_grip(p,id,v),
 DwgLogicalObjectBody::BlockVisibilityGrip(v)=>blocks::project_visibility_grip(p,id,v),
 DwgLogicalObjectBody::BlockAlignmentParameter(v)=>blocks::project_alignment_parameter(p,id,v),
 DwgLogicalObjectBody::BlockAlignmentGrip(v)=>blocks::project_alignment_grip(p,id,v),
 DwgLogicalObjectBody::BlockStretchAction(v)=>actions::project_stretch(p,id,v),
 DwgLogicalObjectBody::BlockScaleAction(v)=>actions::project_scale(p,id,v),
 DwgLogicalObjectBody::BlockFlipAction(v)=>actions::project_flip(p,id,v),
 DwgLogicalObjectBody::BlockBasePointParameter(v)=>blocks::project_base_point(p,id,v),
 DwgLogicalObjectBody::BlockVerticalConstraintParameter(v)=>blocks::project_constraint_parameter(p,id,v,"vertical"),
 DwgLogicalObjectBody::BlockHorizontalConstraintParameter(v)=>blocks::project_constraint_parameter(p,id,v,"horizontal"),
 DwgLogicalObjectBody::Layout(v)=>layout::project(p,id,v)
}}
fn reconstruct_body(r:&mut reader::Reader<'_,'_,'_>,id:i64,kind:&str)->Result<DwgLogicalObjectBody,ValueError>{Ok(match kind{
 "dictionary"=>DwgLogicalObjectBody::Dictionary(tables::reconstruct_dictionary(r,id)?),
 "table_control"=>DwgLogicalObjectBody::TableControl(tables::reconstruct_control(r,id)?),
 "table_record"=>DwgLogicalObjectBody::TableRecord(records::reconstruct_record(r,id)?),
 "xrecord"=>DwgLogicalObjectBody::XRecord(tables::reconstruct_xrecord(r,id)?),
 "entity"=>DwgLogicalObjectBody::Entity(entities::reconstruct(r,id)?),
 "associative_dependency"=>DwgLogicalObjectBody::AssociativeDependency(associativity::reconstruct_dependency(r,id)?),
 "associative_value_dependency"=>DwgLogicalObjectBody::AssociativeValueDependency(associativity::reconstruct_value_dependency(r,id)?),
 "associative_geometry_dependency"=>DwgLogicalObjectBody::AssociativeGeometryDependency(associativity::reconstruct_geometry_dependency(r,id)?),
 "block_grip_location_component"=>DwgLogicalObjectBody::BlockGripLocationComponent(evaluation::reconstruct_grip_location(r,id)?),
 "dynamic_block_proxy_node"=>DwgLogicalObjectBody::DynamicBlockProxyNode(evaluation::reconstruct_proxy(r,id)?),
 "associative_variable"=>DwgLogicalObjectBody::AssociativeVariable(associativity::reconstruct_variable(r,id)?),
 "associative_dimension_dependency_body"=>DwgLogicalObjectBody::AssociativeDimensionDependencyBody(objects::reconstruct_dimension_dependency_body(r,id)?),
 "visual_style"=>DwgLogicalObjectBody::VisualStyle(visual_style::reconstruct(r,id)?),
 "block_parameter_dependency_body"=>DwgLogicalObjectBody::BlockParameterDependencyBody(objects::reconstruct_block_parameter_dependency_body(r,id)?),
 "block_representation_data"=>DwgLogicalObjectBody::BlockRepresentationData(objects::reconstruct_representation_data(r,id)?),
 "dynamic_block_purge_preventer"=>DwgLogicalObjectBody::DynamicBlockPurgePreventer(objects::reconstruct_purge_preventer(r,id)?),
 "evaluation_graph"=>DwgLogicalObjectBody::EvaluationGraph(objects::reconstruct_graph(r,id)?),
 "block_flip_parameter"=>DwgLogicalObjectBody::BlockFlipParameter(blocks::reconstruct_flip_parameter(r,id)?),
 "block_visibility_parameter"=>DwgLogicalObjectBody::BlockVisibilityParameter(blocks::reconstruct_visibility_parameter(r,id)?),
 "placeholder"=>DwgLogicalObjectBody::Placeholder(objects::reconstruct_placeholder(r,id)?),
 "dictionary_variable"=>DwgLogicalObjectBody::DictionaryVariable(objects::reconstruct_dictionary_variable(r,id)?),
 "annotation_scale"=>DwgLogicalObjectBody::AnnotationScale(objects::reconstruct_annotation_scale(r,id)?),
 "sort_entities_table"=>DwgLogicalObjectBody::SortEntitiesTable(objects::reconstruct_sort_entities_table(r,id)?),
 "table_style"=>DwgLogicalObjectBody::TableStyle(Box::new(table_style::reconstruct(r,id)?)),
 "mline_style"=>DwgLogicalObjectBody::MlineStyle(material::reconstruct_mline(r,id)?),
 "mleader_style"=>DwgLogicalObjectBody::MLeaderStyle(mleader_style::reconstruct(r,id)?),
 "material"=>DwgLogicalObjectBody::Material(material::reconstruct_material(r,id)?),
 "block_move_action"=>DwgLogicalObjectBody::BlockMoveAction(actions::reconstruct_move(r,id)?),
 "assoc_network"=>DwgLogicalObjectBody::AssocNetwork(associativity::reconstruct_network(r,id)?),
 "assoc_2d_constraint_group"=>DwgLogicalObjectBody::Assoc2dConstraintGroup(constraints::reconstruct(r,id)?),
 "block_linear_parameter"=>DwgLogicalObjectBody::BlockLinearParameter(blocks::reconstruct_linear_parameter(r,id)?),
 "block_linear_grip"=>DwgLogicalObjectBody::BlockLinearGrip(blocks::reconstruct_linear_grip(r,id)?),
 "block_flip_grip"=>DwgLogicalObjectBody::BlockFlipGrip(blocks::reconstruct_flip_grip(r,id)?),
 "block_visibility_grip"=>DwgLogicalObjectBody::BlockVisibilityGrip(blocks::reconstruct_visibility_grip(r,id)?),
 "block_alignment_parameter"=>DwgLogicalObjectBody::BlockAlignmentParameter(blocks::reconstruct_alignment_parameter(r,id)?),
 "block_alignment_grip"=>DwgLogicalObjectBody::BlockAlignmentGrip(blocks::reconstruct_alignment_grip(r,id)?),
 "block_stretch_action"=>DwgLogicalObjectBody::BlockStretchAction(actions::reconstruct_stretch(r,id)?),
 "block_scale_action"=>DwgLogicalObjectBody::BlockScaleAction(actions::reconstruct_scale(r,id)?),
 "block_flip_action"=>DwgLogicalObjectBody::BlockFlipAction(actions::reconstruct_flip(r,id)?),
 "block_base_point_parameter"=>DwgLogicalObjectBody::BlockBasePointParameter(blocks::reconstruct_base_point(r,id)?),
 "block_vertical_constraint_parameter"=>DwgLogicalObjectBody::BlockVerticalConstraintParameter(blocks::reconstruct_constraint_parameter(r,id,"vertical")?),
 "block_horizontal_constraint_parameter"=>DwgLogicalObjectBody::BlockHorizontalConstraintParameter(blocks::reconstruct_constraint_parameter(r,id,"horizontal")?),
 "layout"=>DwgLogicalObjectBody::Layout(layout::reconstruct(r,id)?),
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"DWG logical object body kind is unknown"))
})}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
