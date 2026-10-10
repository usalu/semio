//! ✉️ Original envelope and selected child share the real caller ledger and retained recipient.
use super::*;
use store::sqlite_snapshot::{artifact::reconstruct_text_into,transfer::{SchemaValidationStorage,validate_database_into}};
use semio_framework_value::native_decoding::NativeDecodeRetirementRecipient;
#[derive(semio_framework_value::RetireOwned)]
struct Prefix{snapshot:Option<SemioSnapshot>,validation:SchemaValidationStorage,child:NativeDecodeRetirementRecipient}
fn empty()->Prefix{Prefix{snapshot:None,validation:SchemaValidationStorage::empty(),child:NativeDecodeRetirementRecipient::new()}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
/// 🪢️ Installs the whole envelope before schema validation or selected domain field production.
pub(super)fn reconstruct(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<SemioSnapshot,ValueError>{native_decoding::workspace::reconstruct_projected(c,empty,|prefix,c|{
 let phase=SqliteSnapshotPhase::ReconstructSnapshot;c.check_database(database,phase)?;validate_database_into(&mut prefix.validation,database,SQL,phase,c)?;let row=database.table("semio_base_document")?.single_row()?;if row.rowid!=1||row.integer(0)?!=1||row.values.len()!=21{return Err(invalid("invalid Semio union document identity or columns"))}let tag=row.text(2)?;let(column,selected)=selection(tag)?;
 for i in 3..21{if i==column{if row.integer(i)?!=1{return Err(invalid("Semio union selected document FK must be one"))}}else if !matches!(row.values[i],SqliteValue::Null){return Err(invalid("Semio union requires one selected document"))}}
 for(index,table)in database.tables.iter().enumerate(){if table.name!="semio_base_document"&&!table.name.starts_with(selected)&&!table.rows.is_empty(){return Err(invalid("unselected Semio subset rows must be empty"))}c.checkpoint(phase,index,database.tables.len())?;}
 let subset=c.with_retirement_child(&mut prefix.child,|c|Ok::<_,ValueError>(match tag{
  "brep"=>SemioSubsetSnapshot::Brep(SemioBrepSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "mesh"=>SemioSubsetSnapshot::Mesh(SemioMeshSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "model"=>SemioSubsetSnapshot::Model(SemioModelSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "value"=>SemioSubsetSnapshot::Value(SemioValueSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "document"=>SemioSubsetSnapshot::Document(SemioDocumentSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "cad"=>SemioSubsetSnapshot::Cad(SemioCadSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "drawing"=>SemioSubsetSnapshot::Drawing(SemioDrawingSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "image"=>SemioSubsetSnapshot::Image(SemioImageSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "video"=>SemioSubsetSnapshot::Video(SemioVideoSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "audio"=>SemioSubsetSnapshot::Audio(SemioAudioSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "animation"=>SemioSubsetSnapshot::Animation(SemioAnimationSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "presentation"=>SemioSubsetSnapshot::Presentation(SemioPresentationSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "flow"=>SemioSubsetSnapshot::Flow(SemioFlowSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "text"=>SemioSubsetSnapshot::Text(SemioTextSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "table"=>SemioSubsetSnapshot::Table(SemioTableSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "graph"=>SemioSubsetSnapshot::Graph(SemioGraphSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "object"=>SemioSubsetSnapshot::Object(SemioObjectSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  "kit"=>SemioSubsetSnapshot::Kit(SemioKitSnapshot::reconstruct_sqlite_database(database,c,SQL)?),
  _=>return Err(invalid("unknown Semio union discriminator"))
 }))?;
 prefix.snapshot=Some(SemioSnapshot{schema:String::new(),subset});reconstruct_text_into(&mut prefix.snapshot.as_mut().unwrap().schema,c,row.text(1)?)?;c.checkpoint(phase,1,1)
},|prefix|prefix.snapshot.take().unwrap())}
