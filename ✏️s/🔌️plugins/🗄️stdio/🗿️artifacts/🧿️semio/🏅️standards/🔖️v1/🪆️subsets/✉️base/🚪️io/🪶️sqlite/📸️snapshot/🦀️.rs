//! ✉️ The explicit eighteen-way relational Semio union preserves each selected typed entity schema.
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
#[path = "📏️native/🦀️.rs"]
pub mod native;

use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use crate::standards::v1::subsets::base::schema::snapshot::*;
use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use crate::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use crate::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot;
use crate::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;
use crate::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot;
use crate::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use crate::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot;
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,Reconstruction},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
const SQL:&str=concat!(include_str!("🗄️.sql"),include_str!("../../../../🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📑️document/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📊️table/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📦️object/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"));
fn selection(tag:&str)->Result<(usize,&'static str),ValueError>{match tag{"brep"=>Ok((3,"semio_brep_")),"mesh"=>Ok((4,"semio_mesh_")),"model"=>Ok((5,"semio_model_")),"value"=>Ok((6,"semio_value_")),"document"=>Ok((7,"semio_document_")),"cad"=>Ok((8,"semio_cad_")),"drawing"=>Ok((9,"semio_drawing_")),"image"=>Ok((10,"semio_image_")),"video"=>Ok((11,"semio_video_")),"audio"=>Ok((12,"semio_audio_")),"animation"=>Ok((13,"semio_animation_")),"presentation"=>Ok((14,"semio_presentation_")),"flow"=>Ok((15,"semio_flow_")),"text"=>Ok((16,"semio_text_")),"table"=>Ok((17,"semio_table_")),"graph"=>Ok((18,"semio_graph_")),"object"=>Ok((19,"semio_object_")),"kit"=>Ok((20,"semio_kit_")),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio subset discriminator"))}}
impl ArtifactSqliteSnapshot for SemioSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::base::io::sqlite::snapshot::native_encoding::encode_snapshot(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::decode_snapshot(payload,control,native_control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut b=native::Bound::file_only(&self.schema,control)?;match &self.subset{
SemioSubsetSnapshot::Brep(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Mesh(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Model(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Value(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Document(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Cad(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Drawing(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Image(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Video(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Audio(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Animation(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Presentation(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Flow(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Text(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Table(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Graph(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Object(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Kit(value)=>value.native_fields(&mut b)?}b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_base_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
let tag=match &self.subset{SemioSubsetSnapshot::Brep(_)=>"brep",SemioSubsetSnapshot::Mesh(_)=>"mesh",SemioSubsetSnapshot::Model(_)=>"model",SemioSubsetSnapshot::Value(_)=>"value",SemioSubsetSnapshot::Document(_)=>"document",SemioSubsetSnapshot::Cad(_)=>"cad",SemioSubsetSnapshot::Drawing(_)=>"drawing",SemioSubsetSnapshot::Image(_)=>"image",SemioSubsetSnapshot::Video(_)=>"video",SemioSubsetSnapshot::Audio(_)=>"audio",SemioSubsetSnapshot::Animation(_)=>"animation",SemioSubsetSnapshot::Presentation(_)=>"presentation",SemioSubsetSnapshot::Flow(_)=>"flow",SemioSubsetSnapshot::Text(_)=>"text",SemioSubsetSnapshot::Table(_)=>"table",SemioSubsetSnapshot::Graph(_)=>"graph",SemioSubsetSnapshot::Object(_)=>"object",SemioSubsetSnapshot::Kit(_)=>"kit"};if row.text(2)?!=tag{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio union owned variant differs from selected relational subset"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=SQL;
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(c)}
fn from_sqlite_database(db:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{let result=(||->Result<Self,ValueError>{c.check_database(db,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(db,SQL,SqliteSnapshotPhase::ReconstructSnapshot,c)?;let row=db.table("semio_base_document")?.single_row()?;if row.rowid!=1||row.integer(0)?!=1||row.values.len()!=21{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio union document identity or columns"));}let tag=row.text(2)?;let(column,prefix)=selection(tag)?;for i in 3..21{if i==column{if row.integer(i)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio union selected document FK must be one"));}}else if row.values[i]!=SqliteValue::Null{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio union requires one selected document"));}}for table in &db.tables{if table.name!="semio_base_document"&&!table.name.starts_with(prefix)&&!table.rows.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unselected Semio subset rows must be empty"));}}
let subset=Owned::new(match tag{"brep"=>SemioSubsetSnapshot::Brep(SemioBrepSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"mesh"=>SemioSubsetSnapshot::Mesh(SemioMeshSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"model"=>SemioSubsetSnapshot::Model(SemioModelSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"value"=>SemioSubsetSnapshot::Value(SemioValueSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"document"=>SemioSubsetSnapshot::Document(SemioDocumentSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"cad"=>SemioSubsetSnapshot::Cad(SemioCadSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"drawing"=>SemioSubsetSnapshot::Drawing(SemioDrawingSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"image"=>SemioSubsetSnapshot::Image(SemioImageSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"video"=>SemioSubsetSnapshot::Video(SemioVideoSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"audio"=>SemioSubsetSnapshot::Audio(SemioAudioSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"animation"=>SemioSubsetSnapshot::Animation(SemioAnimationSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"presentation"=>SemioSubsetSnapshot::Presentation(SemioPresentationSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"flow"=>SemioSubsetSnapshot::Flow(SemioFlowSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"text"=>SemioSubsetSnapshot::Text(SemioTextSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"table"=>SemioSubsetSnapshot::Table(SemioTableSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"graph"=>SemioSubsetSnapshot::Graph(SemioGraphSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"object"=>SemioSubsetSnapshot::Object(SemioObjectSnapshot::reconstruct_sqlite_database(db,c,SQL)?),"kit"=>SemioSubsetSnapshot::Kit(SemioKitSnapshot::reconstruct_sqlite_database(db,c,SQL)?),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio union discriminator"))});let mut r=Reconstruction::new(c)?;let value=Owned::new(Self{schema:r.text(row.text(1)?)?,subset:subset.take()});r.checkpoint()?;Ok(value.take())})();result}
}

impl SemioSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::layout(c.limits())?;let mut out=RowWriter::new(SQL,c)?;visit_rows(self,&mut out)?;out.finish()}
/// 🧾️ Counts exact root and selected child cells without materializing a child database.
pub(crate)fn admit_sqlite_values(&self,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{semantic::layout(c.limits())?;let mut out=RowWriter::borrowed(c,phase)?;visit_rows(self,&mut out)?;out.finish_borrowed()}
}

fn visit_rows(snapshot:&SemioSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 use crate::standards::v1::subsets as owners;
 let tag=crate::standards::v1::subsets::base::schema::snapshot::subset_tag(&snapshot.subset);let(column,_)=selection(tag)?;
 let mut cells=[Cell::Null;20];cells[0]=Cell::Text(&snapshot.schema);cells[1]=Cell::Text(tag);cells[column-1]=Cell::Integer(1);out.insert_key("semio_base_document",1,&cells)?;
 match &snapshot.subset{SemioSubsetSnapshot::Brep(value)=>owners::brep::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Mesh(value)=>owners::mesh::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Model(value)=>owners::model::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Value(value)=>owners::value::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Document(value)=>owners::document::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Cad(value)=>owners::cad::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Drawing(value)=>owners::drawing::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Image(value)=>owners::image::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Video(value)=>owners::video::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Audio(value)=>owners::audio::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Animation(value)=>owners::animation::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Presentation(value)=>owners::presentation::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Flow(value)=>owners::flow::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Text(value)=>owners::text::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Table(value)=>owners::table::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Graph(value)=>owners::graph::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Object(value)=>owners::object::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Kit(value)=>owners::kit::io::sqlite::snapshot::visit_rows(value,out)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
