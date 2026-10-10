//! 🪆️ Original object reconstruction retains all child identities before cancellable field copies.
use super::{SemioObjectSnapshot,SemioBrepSnapshot,SemioMeshSnapshot,SemioValueSnapshot,SemioTransform,SemioPoint3,SemioQuaternion,identity,float_columns,validate_semio_child_identity};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase,artifact::{FloatRow,RowIndex,RowIndexStorage,reconstruct_text_into},transfer::{SchemaValidationStorage,validate_database_into,validate_component_into}}};
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(semio_framework_value::RetireOwned)]
struct Prefix{snapshot:Option<SemioObjectSnapshot>,validation:SchemaValidationStorage,references:RowIndexStorage}
fn empty()->Prefix{Prefix{snapshot:Some(super::native_decoding::empty()),validation:SchemaValidationStorage::empty(),references:RowIndexStorage::empty()}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
/// 🧒️ Fills the original caller child slot without an independently returned field owner.
pub(crate)fn child_into<S:Send+'static>(destination:&mut Option<store::ArtifactChild<S>>,database:&SqliteDatabase,table:&str,subset:&str,references:&mut RowIndex<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let rows=database.table(table)?;if rows.rows.is_empty(){return Ok(())}let row=FloatRow::new(rows.single_row()?,float_columns(table))?;identity(row,4)?;if row.integer(1)?!=1{return Err(invalid("invalid Semio object child owner"))}
 let reference=references.take(row.integer(3)?,control)?.ok_or_else(||invalid("dangling or multiply owned Semio object reference"))?;
 *destination=Some(store::ArtifactChild::new(String::new(),semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}));let child=destination.as_mut().unwrap();
 reconstruct_text_into(&mut child.target.artifact_id,control,reference.text(1)?)?;reconstruct_text_into(&mut child.target.dialect.artifact_kind,control,reference.text(2)?)?;reconstruct_text_into(&mut child.target.dialect.standard,control,reference.text(3)?)?;reconstruct_text_into(&mut child.target.dialect.subset,control,reference.text(4)?)?;reconstruct_text_into(&mut child.child_id,control,row.text(2)?)?;
 validate_semio_child_identity(&child.child_id,&child.target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))
}
/// 🪢️ Installs one complete original prefix before validating or reconstructing any source field.
pub(super)fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared:&str)->Result<SemioObjectSnapshot,ValueError>{crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::workspace::reconstruct_projected(control,empty,|prefix,control|{
 let phase=SqliteSnapshotPhase::ReconstructSnapshot;control.check_database(database,phase)?;validate_database_into(&mut prefix.validation,database,declared,phase,control)?;validate_component_into(&mut prefix.validation,database,SemioObjectSnapshot::SQLITE_SCHEMA,phase,control)?;
 let document=FloatRow::new(database.table("semio_object_document")?.single_row()?,float_columns("semio_object_document"))?;identity(document,12)?;if document.rowid!=1{return Err(invalid("invalid Semio object document identifier"))}
 let mut references=RowIndex::new(&mut prefix.references,database,"semio_object_reference",5,&[],control,"duplicate Semio object reference identity")?;for(count,&index)in references.indices().iter().enumerate(){let row=references.row(index)?;for column in 1..5{row.text(column)?;}control.checkpoint(phase,count+1,references.len())?;}
 let snapshot=prefix.snapshot.as_mut().unwrap();snapshot.transform=SemioTransform{translation:SemioPoint3{x:document.real(2)?,y:document.real(3)?,z:document.real(4)?},rotation:SemioQuaternion{x:document.real(5)?,y:document.real(6)?,z:document.real(7)?,w:document.real(8)?},scale:SemioPoint3{x:document.real(9)?,y:document.real(10)?,z:document.real(11)?}};
 child_into::<SemioBrepSnapshot>(&mut snapshot.brep,database,"semio_object_brep_child","brep",&mut references,control)?;child_into::<SemioMeshSnapshot>(&mut snapshot.mesh,database,"semio_object_mesh_child","mesh",&mut references,control)?;child_into::<SemioValueSnapshot>(&mut snapshot.properties,database,"semio_object_value_child","value",&mut references,control)?;
 if references.remaining()!=0{return Err(invalid("unowned Semio object reference"))}reconstruct_text_into(&mut snapshot.schema,control,document.text(1)?)?;snapshot.validate().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;control.checkpoint(phase,0,0)
},|prefix|prefix.snapshot.take().unwrap())}
