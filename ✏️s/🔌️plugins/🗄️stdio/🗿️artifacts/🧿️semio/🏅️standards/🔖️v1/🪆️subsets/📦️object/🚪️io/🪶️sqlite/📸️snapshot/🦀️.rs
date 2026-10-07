//! 📦️ Spatial placement and independently typed persisted child handles.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow};
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::{brep::schema::snapshot::SemioBrepSnapshot,mesh::schema::snapshot::SemioMeshSnapshot,value::schema::snapshot::SemioValueSnapshot};
use crate::standards::v1::subsets::base::schema::{geometry::{SemioPoint3,SemioQuaternion},child::validate_semio_child_identity};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 📦️ Emits placement companions and each independent persisted child through one writer.
pub(crate)fn visit_rows(snapshot:&SemioObjectSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 snapshot.validate().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;let t=snapshot.transform;
 out.insert_key_float("semio_object_document",1,&[Cell::Text(&snapshot.schema),Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)],float_columns("semio_object_document"))?;
 if let Some(child)=&snapshot.brep{project_child(child,"semio_object_brep_child",out)?;}if let Some(child)=&snapshot.mesh{project_child(child,"semio_object_mesh_child",out)?;}if let Some(child)=&snapshot.properties{project_child(child,"semio_object_value_child",out)?;}Ok(())
}
/// 🫳️ Admits every typed relational cell before native forecast or allocation.
pub(crate)fn admit_values(snapshot:&SemioObjectSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio object row identity or columns"))}else{Ok(())}}
fn project_child<S>(child:&store::ArtifactChild<S>,table:&str,p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let target=&child.target;let reference=p.insert("semio_object_reference",&[Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])?;p.insert(table,&[Cell::Integer(1),Cell::Text(&child.child_id),Cell::Integer(reference)])?;Ok(())}
fn child<S:Send+'static>(database:&SqliteDatabase,table:&str,subset:&str,references:&mut semio_framework_os_kernel::sqlite_snapshot::artifact::RowIndex<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<Option<store::ArtifactChild<S>>,ValueError>{
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 let rows=database.table(table)?;if rows.rows.is_empty(){return Ok(None)}let row=SqliteRow::new(rows.single_row()?,float_columns(table))?;identity(row,4)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio object child owner"))}
 let reference=references.take(row.integer(3)?,control)?.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"dangling or multiply owned Semio object reference"))?;
 let mut child=Owned::new(store::ArtifactChild::<S>::new(String::new(),semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}));
 child.get_mut().target.artifact_id=reconstruct_text(control,reference.text(1)?)?;child.get_mut().target.dialect.artifact_kind=reconstruct_text(control,reference.text(2)?)?;child.get_mut().target.dialect.standard=reconstruct_text(control,reference.text(3)?)?;child.get_mut().target.dialect.subset=reconstruct_text(control,reference.text(4)?)?;child.get_mut().child_id=reconstruct_text(control,row.text(2)?)?;
 let view=child.get_mut();validate_semio_child_identity(&view.child_id,&view.target,subset).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;Ok(Some(child.take()))
}
impl ArtifactSqliteSnapshot for SemioObjectSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::object::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="object"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_object_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioObjectSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::artifact::RowIndex;
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let document=single_float_row(database,"semio_object_document")?;identity(document,12)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio object document identifier"))}
 let mut references=RowIndex::new(database,"semio_object_reference",5,float_columns("semio_object_reference"),control,"duplicate Semio object reference identity")?;
 for(count,&index)in references.indices().iter().enumerate(){let row=references.row(index)?;for column in 1..5{row.text(column)?;}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,references.len())?;}
 let transform=SemioTransform{translation:SemioPoint3{x:document.real(2)?,y:document.real(3)?,z:document.real(4)?},rotation:SemioQuaternion{x:document.real(5)?,y:document.real(6)?,z:document.real(7)?,w:document.real(8)?},scale:SemioPoint3{x:document.real(9)?,y:document.real(10)?,z:document.real(11)?}};
 let mut snapshot=Owned::new(Self{schema:String::new(),transform,brep:None,mesh:None,properties:None});
 snapshot.get_mut().brep=child::<SemioBrepSnapshot>(database,"semio_object_brep_child","brep",&mut references,control)?;
 snapshot.get_mut().mesh=child::<SemioMeshSnapshot>(database,"semio_object_mesh_child","mesh",&mut references,control)?;
 snapshot.get_mut().properties=child::<SemioValueSnapshot>(database,"semio_object_value_child","value",&mut references,control)?;
 if references.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unowned Semio object reference"))}
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;snapshot.get_mut().validate().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;Ok(snapshot.take())
}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_object_document"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11)],_=>&[]}}

fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioObjectSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.scalars(10)?;if let Some(child)=&self.brep{native_child(child,b)?;}if let Some(child)=&self.mesh{native_child(child,b)?;}if let Some(child)=&self.properties{native_child(child,b)?;}Ok(())}
}

/// 🧭️ Bounds the actual typed ArtifactRef fields in an owned Semio relationship.
pub fn native_reference(reference:&semio_framework_artifact_reference::ArtifactRef,b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.text(&reference.artifact_id)?;b.text(&reference.dialect.artifact_kind)?;b.text(&reference.dialect.standard)?;b.text(&reference.dialect.subset)?;Ok(())}
/// 🪆️ Bounds a typed durable ArtifactChild identity and target without materializing its cache.
pub fn native_child<S>(child:&store::ArtifactChild<S>,b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(1)?;b.text(&child.child_id)?;native_reference(&child.target,b)}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
