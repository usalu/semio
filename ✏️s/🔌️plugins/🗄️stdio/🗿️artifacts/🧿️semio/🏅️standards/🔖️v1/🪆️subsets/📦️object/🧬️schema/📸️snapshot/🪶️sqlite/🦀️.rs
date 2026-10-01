//! 📦️ Spatial placement and independently typed persisted child handles.
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,insert_ieee754,insert_key_ieee754};
use super::{SemioObjectSnapshot,SemioTransform,SemioBrepSnapshot,SemioMeshSnapshot,SemioValueSnapshot};
use crate::standards::v1::subsets::base::schema::{geometry::{SemioPoint3,SemioQuaternion},child::validate_semio_child_identity};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::BTreeMap;
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,columns:usize)->Result<(),String>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err("invalid Semio object row identity or columns".into())}else{Ok(())}}
fn project_child<S>(child:&store::ArtifactChild<S>,table:&str,p:&mut Projection<'_,'_>)->Result<(),String>{let target=&child.target;let reference=p.insert_float("semio_object_reference",&[Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])?;p.insert_float(table,&[Cell::Integer(1),Cell::Text(&child.child_id),Cell::Integer(reference)])?;Ok(())}
fn child<S>(database:&SqliteDatabase,table:&str,subset:&str,references:&mut BTreeMap<i64,store::os_io::ArtifactRef>,control:&mut SqliteSnapshotControl<'_>)->Result<Option<store::ArtifactChild<S>>,String>{let rows=database.table(table)?;if rows.rows.is_empty(){return Ok(None);}let row=SqliteRow::new(rows.single_row()?,float_columns(table))?;identity(row,4)?;if row.integer(1)?!=1{return Err("invalid Semio object child owner".into());}let target=references.remove(&row.integer(3)?).ok_or("dangling or multiply owned Semio object reference")?;validate_semio_child_identity(row.text(2)?,&target,subset)?;Ok(Some(store::ArtifactChild::new(reconstruct_text(control,row.text(2)?)?,target)))}
impl ArtifactSqliteSnapshot for SemioObjectSnapshot{
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="object"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_object_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
let mut p=Projection::new(Self::SQLITE_SCHEMA,control)?;let t=self.transform;p.insert_key_float("semio_object_document",1,&[Cell::Text(&self.schema),Cell::Real(t.translation.x),Cell::Real(t.translation.y),Cell::Real(t.translation.z),Cell::Real(t.rotation.x),Cell::Real(t.rotation.y),Cell::Real(t.rotation.z),Cell::Real(t.rotation.w),Cell::Real(t.scale.x),Cell::Real(t.scale.y),Cell::Real(t.scale.z)])?;
if let Some(child)=&self.brep{project_child(child,"semio_object_brep_child",&mut p)?;}if let Some(child)=&self.mesh{project_child(child,"semio_object_mesh_child",&mut p)?;}if let Some(child)=&self.properties{project_child(child,"semio_object_value_child",&mut p)?;}p.checkpoint()?;self.validate()?;p.finish()
}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioObjectSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,String> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits()).map_err(|error|error.to_string())?;let document=single_float_row(database,"semio_object_document")?;identity(document,12)?;if document.rowid!=1{return Err("invalid Semio object document identifier".into());}
let mut references=BTreeMap::new();for(count,row)in float_rows(database,"semio_object_reference",control)?.into_iter().enumerate(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;identity(row,5)?;let target=store::os_io::ArtifactRef{artifact_id:reconstruct_text(control,row.text(1)?)?,dialect:store::os_io::ArtifactDialect{artifact_kind:reconstruct_text(control,row.text(2)?)?,standard:reconstruct_text(control,row.text(3)?)?,subset:reconstruct_text(control,row.text(4)?)?}};if references.insert(row.rowid,target).is_some(){return Err("duplicate Semio object reference identity".into());}}
let transform=SemioTransform{translation:SemioPoint3{x:document.real(2)?,y:document.real(3)?,z:document.real(4)?},rotation:SemioQuaternion{x:document.real(5)?,y:document.real(6)?,z:document.real(7)?,w:document.real(8)?},scale:SemioPoint3{x:document.real(9)?,y:document.real(10)?,z:document.real(11)?}};let brep=child::<SemioBrepSnapshot>(database,"semio_object_brep_child","brep",&mut references,control)?;let mesh=child::<SemioMeshSnapshot>(database,"semio_object_mesh_child","mesh",&mut references,control)?;let properties=child::<SemioValueSnapshot>(database,"semio_object_value_child","value",&mut references,control)?;if !references.is_empty(){return Err("unowned Semio object reference".into());}let result=Self{schema:reconstruct_text(control,document.text(1)?)?,transform,brep,mesh,properties};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;result.validate()?;Ok(result)
    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_object_document"=>&[FloatColumn::Binary64(2),FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9),FloatColumn::Binary64(10),FloatColumn::Binary64(11)],_=>&[]}}
trait FloatProjection { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,String>; fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),String>; }
impl FloatProjection for Projection<'_,'_> { fn insert_float(&mut self,table:&str,cells:&[Cell<'_>])->Result<i64,String>{insert_ieee754(self,table,cells,float_columns(table))} fn insert_key_float(&mut self,table:&str,key:i64,cells:&[Cell<'_>])->Result<(),String>{insert_key_ieee754(self,table,key,cells,float_columns(table))} }
fn float_rows<'a>(db:&'a SqliteDatabase,table:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,String>{let table_rows=db.table(table)?;control.check_rows(table_rows.rows.len())?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,table_rows.rows.len())?;let mut result=Vec::new();for(count,row)in table_rows.rows.iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,table_rows.rows.len())?;}}Ok(result)}
fn ordered_float_rows<'a>(db:&'a SqliteDatabase,table:&str,ordinal:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<SqliteRow<'a>>,String>{let mut result=Vec::new();for(count,row)in semio_framework_os_kernel::sqlite_snapshot::artifact::ordered_row_refs(db.table(table)?,ordinal,control)?.into_iter().enumerate(){result.push(SqliteRow::new(row,float_columns(table))?);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,0)?;}}Ok(result)}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,String>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioObjectSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),String>{b.text(&self.schema)?;b.scalars(10)?;if let Some(child)=&self.brep{native_child(child,b)?;}if let Some(child)=&self.mesh{native_child(child,b)?;}if let Some(child)=&self.properties{native_child(child,b)?;}Ok(())}
}

/// 🧭️ Bounds the actual typed ArtifactRef fields in an owned Semio relationship.
pub fn native_reference(reference:&store::os_io::ArtifactRef,b:&mut Bound<'_, '_>)->Result<(),String>{b.text(&reference.artifact_id)?;b.text(&reference.dialect.artifact_kind)?;b.text(&reference.dialect.standard)?;b.text(&reference.dialect.subset)?;Ok(())}
/// 🪆️ Bounds a typed durable ArtifactChild identity and target without materializing its cache.
pub fn native_child<S>(child:&store::ArtifactChild<S>,b:&mut Bound<'_, '_>)->Result<(),String>{b.entities(1)?;b.text(&child.child_id)?;native_reference(&child.target,b)}
