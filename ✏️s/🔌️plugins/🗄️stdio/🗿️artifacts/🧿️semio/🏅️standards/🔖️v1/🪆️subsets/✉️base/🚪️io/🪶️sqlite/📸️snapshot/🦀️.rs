//! ✉️ The explicit eighteen-way relational Semio union preserves each selected typed entity schema.
#[path="🧮️semantic/🦀️.rs"]
mod semantic;

use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::{native, native_decoding};

use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot;
use semio_s_artifact_stdio_semio_audio::schema::snapshot::SemioAudioSnapshot;
use semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot;
use semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot;
use semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot;
use semio_s_artifact_stdio_semio_table::schema::snapshot::SemioTableSnapshot;
use semio_s_artifact_stdio_semio_text::schema::snapshot::SemioTextSnapshot;
use semio_s_artifact_stdio_semio_value::schema::snapshot::SemioValueSnapshot;
use semio_s_artifact_stdio_semio_video::schema::snapshot::SemioVideoSnapshot;
use semio_framework_os_kernel::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{artifact::{Cell,RowWriter},SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
const SQL:&str=concat!(include_str!("🗄️.sql"),include_str!("../../../../🧊️brep/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔺️mesh/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🏛️model/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔢️value/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📑️document/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📐️cad/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🖊️drawing/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🖼️image/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🎬️video/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔊️audio/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🎞️animation/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📽️presentation/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📊️table/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🕸️graph/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../📦️object/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"),include_str!("../../../../🧰️kit/🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql"));
fn selection(tag:&str)->Result<(usize,&'static str),ValueError>{match tag{"brep"=>Ok((3,"semio_brep_")),"mesh"=>Ok((4,"semio_mesh_")),"model"=>Ok((5,"semio_model_")),"value"=>Ok((6,"semio_value_")),"document"=>Ok((7,"semio_document_")),"cad"=>Ok((8,"semio_cad_")),"drawing"=>Ok((9,"semio_drawing_")),"image"=>Ok((10,"semio_image_")),"video"=>Ok((11,"semio_video_")),"audio"=>Ok((12,"semio_audio_")),"animation"=>Ok((13,"semio_animation_")),"presentation"=>Ok((14,"semio_presentation_")),"flow"=>Ok((15,"semio_flow_")),"text"=>Ok((16,"semio_text_")),"table"=>Ok((17,"semio_table_")),"graph"=>Ok((18,"semio_graph_")),"object"=>Ok((19,"semio_object_")),"kit"=>Ok((20,"semio_kit_")),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio subset discriminator"))}}
impl ArtifactSqliteSnapshotHost for SemioSnapshot{
fn retire_sqlite_snapshot(self){drop(native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{union_encoding::encode_snapshot(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{union_decoding::decode_snapshot(payload,control,native_control.native())}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;let mut b=native::Bound::file_only(&self.schema,control)?;match &self.subset{
SemioSubsetSnapshot::Brep(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Mesh(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Model(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Value(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Document(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Cad(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Drawing(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Image(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Video(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Audio(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Animation(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Presentation(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Flow(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Text(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Table(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Graph(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Object(value)=>value.native_fields(&mut b)?,SemioSubsetSnapshot::Kit(value)=>value.native_fields(&mut b)?}b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_base_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
let tag=match &self.subset{SemioSubsetSnapshot::Brep(_)=>"brep",SemioSubsetSnapshot::Mesh(_)=>"mesh",SemioSubsetSnapshot::Model(_)=>"model",SemioSubsetSnapshot::Value(_)=>"value",SemioSubsetSnapshot::Document(_)=>"document",SemioSubsetSnapshot::Cad(_)=>"cad",SemioSubsetSnapshot::Drawing(_)=>"drawing",SemioSubsetSnapshot::Image(_)=>"image",SemioSubsetSnapshot::Video(_)=>"video",SemioSubsetSnapshot::Audio(_)=>"audio",SemioSubsetSnapshot::Animation(_)=>"animation",SemioSubsetSnapshot::Presentation(_)=>"presentation",SemioSubsetSnapshot::Flow(_)=>"flow",SemioSubsetSnapshot::Text(_)=>"text",SemioSubsetSnapshot::Table(_)=>"table",SemioSubsetSnapshot::Graph(_)=>"graph",SemioSubsetSnapshot::Object(_)=>"object",SemioSubsetSnapshot::Kit(_)=>"kit"};if row.text(2)?!=tag{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio union owned variant differs from selected relational subset"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}




}
impl pack::snapshot::ArtifactSqliteSnapshot for SemioSnapshot {
const SQLITE_SCHEMA:&'static str=SQL;
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(c)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{reconstruction::reconstruct(database,control)}
}

impl SemioSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{semantic::layout(c.limits())?;let mut out=RowWriter::new(SQL,c)?;visit_rows(self,&mut out)?;out.finish()}
/// 🧾️ Counts exact root and selected child cells without materializing a child database.
pub(crate)fn admit_sqlite_values(&self,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{semantic::layout(c.limits())?;let mut out=RowWriter::borrowed(c,phase)?;visit_rows(self,&mut out)?;out.finish_borrowed()}
}

fn visit_rows(snapshot:&SemioSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let tag=subset_tag(&snapshot.subset);let(column,_)=selection(tag)?;
 let mut cells=[Cell::Null;20];cells[0]=Cell::Text(&snapshot.schema);cells[1]=Cell::Text(tag);cells[column-1]=Cell::Integer(1);out.insert_key("semio_base_document",1,&cells)?;
 match &snapshot.subset{SemioSubsetSnapshot::Brep(value)=>semio_s_artifact_stdio_semio_brep::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Mesh(value)=>semio_s_artifact_stdio_semio_mesh::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Model(value)=>semio_s_artifact_stdio_semio_model::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Value(value)=>semio_s_artifact_stdio_semio_value::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Document(value)=>semio_s_artifact_stdio_semio_document::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Cad(value)=>semio_s_artifact_stdio_semio_cad::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Drawing(value)=>semio_s_artifact_stdio_semio_drawing::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Image(value)=>semio_s_artifact_stdio_semio_image::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Video(value)=>semio_s_artifact_stdio_semio_video::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Audio(value)=>semio_s_artifact_stdio_semio_audio::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Animation(value)=>semio_s_artifact_stdio_semio_animation::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Presentation(value)=>semio_s_artifact_stdio_semio_presentation::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Flow(value)=>semio_s_artifact_stdio_semio_flow::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Text(value)=>semio_s_artifact_stdio_semio_text::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Table(value)=>semio_s_artifact_stdio_semio_table::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Graph(value)=>semio_s_artifact_stdio_semio_graph::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Object(value)=>semio_s_artifact_stdio_semio_object::io::sqlite::snapshot::visit_rows(value,out),SemioSubsetSnapshot::Kit(value)=>semio_s_artifact_stdio_semio_kit::io::sqlite::snapshot::visit_rows(value,out)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;




#[path="💰️reconstruction/🦀️.rs"]mod reconstruction;

/// 🛬️ Decodes the union envelope's native payload by dispatching on its subset tag to that subset's own decoder.
mod union_decoding {
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_decoding::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteDatabaseLimits};
pub(crate) fn decode_snapshot(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>)->Result<crate::schema::snapshot::SemioSnapshot,ValueError>{super::semantic::layout(control.limits())?;decode(payload,crate::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA,control,binary_snapshot,text_snapshot,native_control)}
fn binary_snapshot(body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<crate::schema::snapshot::SemioSnapshot,ValueError>{
 use crate::schema::snapshot::SemioSubsetSnapshot as S;
 let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio envelope native format"))}let ordinal=reader.read_u8().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;let schema=control.borrow_text(bytes(&mut reader)?)?;let limits=super::semantic::subset_limits(limits,schema.len(),super::semantic::tag(ordinal)?)?;let schema=Owned::new(control.copy_text(schema)?);let payload=reader.read_bytes(reader.remaining()).map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?;
 let id=match ordinal{0=>"stdio.semio.brep",1=>"stdio.semio.mesh",2=>"stdio.semio.model",3=>"stdio.semio.value",4=>"s.stdio.semio.document",5=>"stdio.semio.cad",6=>"stdio.semio.drawing",7=>"s.stdio.semio.image",8=>"stdio.semio.video",9=>"stdio.semio.audio",10=>"s.stdio.semio.animation",11=>"s.stdio.semio.presentation",12=>"stdio.semio.flow",13=>"s.stdio.semio.text",14=>"s.stdio.semio.table",15=>"s.stdio.semio.graph",16=>"stdio.semio.object",17=>"stdio.semio.kit",_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native envelope subset ordinal"))};
 let body=store::semio_format::unwrap_binary_controlled(payload,id,store::semio_format::Component::Pack,1,control)?;
 let subset=Owned::new(match ordinal{
 0=>S::Brep(semio_s_artifact_stdio_semio_brep::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 1=>S::Mesh(semio_s_artifact_stdio_semio_mesh::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 2=>S::Model(semio_s_artifact_stdio_semio_model::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 3=>{let body=control.borrow_text(body)?;S::Value(semio_s_artifact_stdio_semio_value::io::sqlite::snapshot::native_decoding::document(body,control,limits)?)},
 4=>S::Document(semio_s_artifact_stdio_semio_document::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 5=>S::Cad(semio_s_artifact_stdio_semio_cad::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 6=>S::Drawing(semio_s_artifact_stdio_semio_drawing::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 7=>S::Image(semio_s_artifact_stdio_semio_image::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 8=>S::Video(semio_s_artifact_stdio_semio_video::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 9=>S::Audio(semio_s_artifact_stdio_semio_audio::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 10=>S::Animation(semio_s_artifact_stdio_semio_animation::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 11=>S::Presentation(semio_s_artifact_stdio_semio_presentation::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 12=>S::Flow(semio_s_artifact_stdio_semio_flow::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 13=>S::Text(semio_s_artifact_stdio_semio_text::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 14=>S::Table(semio_s_artifact_stdio_semio_table::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 15=>S::Graph(semio_s_artifact_stdio_semio_graph::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 16=>S::Object(semio_s_artifact_stdio_semio_object::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 17=>S::Kit(semio_s_artifact_stdio_semio_kit::io::sqlite::snapshot::native_decoding::binary(body,control,limits)?),
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native envelope subset ordinal"))
 });let value=Owned::new(crate::schema::snapshot::SemioSnapshot{schema:schema.take(),subset:subset.take()});control.checkpoint()?;Ok(value.take())
}
fn text_snapshot(body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<crate::schema::snapshot::SemioSnapshot,ValueError>{
 use crate::schema::snapshot::SemioSubsetSnapshot as S;
 control.scoped_stage(|control|{control.begin_stage(body.len())?;for _ in body.bytes(){control.step()?}Ok::<_,ValueError>(())})?;
 let mut fields=body.splitn(3,'\n');let tag=fields.next().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio envelope subset"))?.trim().strip_prefix("subset=").ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio envelope requires subset header"))?;let schema=fields.next().ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"missing Semio envelope schema"))?.trim().strip_prefix("schema=").ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio envelope requires schema header"))?;let body=fields.next().unwrap_or("");if !schema.len().is_multiple_of(2){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native odd hex length"))}let limits=super::semantic::subset_limits(limits,schema.len()/2,tag)?;let schema=Owned::new(hex_text(schema,control)?);
 let subset=Owned::new(match tag{
 "brep"=>S::Brep(semio_s_artifact_stdio_semio_brep::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "mesh"=>S::Mesh(semio_s_artifact_stdio_semio_mesh::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "model"=>S::Model(semio_s_artifact_stdio_semio_model::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "value"=>S::Value(semio_s_artifact_stdio_semio_value::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "document"=>S::Document(semio_s_artifact_stdio_semio_document::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "cad"=>S::Cad(semio_s_artifact_stdio_semio_cad::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "drawing"=>S::Drawing(semio_s_artifact_stdio_semio_drawing::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "image"=>S::Image(semio_s_artifact_stdio_semio_image::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "video"=>S::Video(semio_s_artifact_stdio_semio_video::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "audio"=>S::Audio(semio_s_artifact_stdio_semio_audio::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "animation"=>S::Animation(semio_s_artifact_stdio_semio_animation::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "presentation"=>S::Presentation(semio_s_artifact_stdio_semio_presentation::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "flow"=>S::Flow(semio_s_artifact_stdio_semio_flow::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "text"=>S::Text(semio_s_artifact_stdio_semio_text::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "table"=>S::Table(semio_s_artifact_stdio_semio_table::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "graph"=>S::Graph(semio_s_artifact_stdio_semio_graph::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "object"=>S::Object(semio_s_artifact_stdio_semio_object::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 "kit"=>S::Kit(semio_s_artifact_stdio_semio_kit::io::sqlite::snapshot::native_decoding::document(body,control,limits)?),
 _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native envelope subset tag"))
 });let value=Owned::new(crate::schema::snapshot::SemioSnapshot{schema:schema.take(),subset:subset.take()});control.checkpoint()?;Ok(value.take())
}
}

/// 🛫️ Encodes the union envelope's native payload by dispatching to the selected subset's own field encoder.
mod union_encoding {
use semio_s_artifact_stdio_semio_base::io::sqlite::snapshot::native_encoding::*;
use semio_framework_value::ValueError;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
pub(crate) fn encode_snapshot(value:&crate::schema::snapshot::SemioSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{encode_admitted(encoding,"semio stdio.semio.dsl v1\n","stdio.semio.pack v1",control,|control|value.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative),|w,e|{w.entities(1)?;if e==SnapshotEncoding::Binary{w.byte(1)?;w.byte(crate::schema::snapshot::subset_ordinal(&value.subset))?;w.string(&value.schema,e)?}else{w.bytes(b"subset=")?;w.bytes(crate::schema::snapshot::subset_tag(&value.subset).as_bytes())?;w.bytes(b"\nschema=")?;w.string(&value.schema,e)?;w.byte(b'\n')?}subset(&value.subset,w,e)},native_owner)}
fn subset(value:&crate::schema::snapshot::SemioSubsetSnapshot,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),ValueError>{
 use crate::schema::snapshot::SemioSubsetSnapshot as S;
 let token=match value{S::Brep(_)=>"stdio.semio.brep.pack v1",S::Mesh(_)=>"stdio.semio.mesh.pack v1",S::Model(_)=>"stdio.semio.model.pack v1",S::Value(_)=>"stdio.semio.value.pack v1",S::Document(_)=>"s.stdio.semio.document.pack v1",S::Cad(_)=>"stdio.semio.cad.pack v1",S::Drawing(_)=>"stdio.semio.drawing.pack v1",S::Image(_)=>"s.stdio.semio.image.pack v1",S::Video(_)=>"stdio.semio.video.pack v1",S::Audio(_)=>"stdio.semio.audio.pack v1",S::Animation(_)=>"s.stdio.semio.animation.pack v1",S::Presentation(_)=>"s.stdio.semio.presentation.pack v1",S::Flow(_)=>"stdio.semio.flow.pack v1",S::Text(_)=>"s.stdio.semio.text.pack v1",S::Table(_)=>"s.stdio.semio.table.pack v1",S::Graph(_)=>"s.stdio.semio.graph.pack v1",S::Object(_)=>"stdio.semio.object.pack v1",S::Kit(_)=>"stdio.semio.kit.pack v1"};
 if e==SnapshotEncoding::Binary{w.binary_envelope(token)?}match value{
 S::Brep(v)=>semio_s_artifact_stdio_semio_brep::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Mesh(v)=>semio_s_artifact_stdio_semio_mesh::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Model(v)=>semio_s_artifact_stdio_semio_model::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Value(v)=>semio_s_artifact_stdio_semio_value::io::sqlite::snapshot::native_encoding::fields(v,w),
 S::Document(v)=>semio_s_artifact_stdio_semio_document::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Cad(v)=>semio_s_artifact_stdio_semio_cad::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Drawing(v)=>semio_s_artifact_stdio_semio_drawing::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Image(v)=>semio_s_artifact_stdio_semio_image::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Video(v)=>semio_s_artifact_stdio_semio_video::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Audio(v)=>semio_s_artifact_stdio_semio_audio::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Animation(v)=>semio_s_artifact_stdio_semio_animation::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Presentation(v)=>semio_s_artifact_stdio_semio_presentation::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Flow(v)=>semio_s_artifact_stdio_semio_flow::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Text(v)=>semio_s_artifact_stdio_semio_text::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Table(v)=>semio_s_artifact_stdio_semio_table::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Graph(v)=>semio_s_artifact_stdio_semio_graph::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Object(v)=>semio_s_artifact_stdio_semio_object::io::sqlite::snapshot::native_encoding::fields(v,w,e),
 S::Kit(v)=>semio_s_artifact_stdio_semio_kit::io::sqlite::snapshot::native_encoding::fields(v,w,e)
 }
}
}
