//! 🖼️ Image dimensions, frame pixel buffers, nullable ICC profiles and ordered metadata.
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use super::{SemioImageSnapshot,SemioImageFrame,SemioImageMetadataEntry,SemioColorspace};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text,reconstruct_blob},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::BTreeSet;
fn number(value:usize)->Result<i64,String>{i64::try_from(value).map_err(|error|error.to_string())}
fn identity(row:&SqliteRow,columns:usize)->Result<(),String>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err("invalid Semio image row identity or columns".into())}else{Ok(())}}
fn colorspace(value:SemioColorspace)->&'static str{match value{SemioColorspace::Rgb=>"rgb",SemioColorspace::Rgba=>"rgba",SemioColorspace::Grayscale=>"grayscale",SemioColorspace::GrayscaleAlpha=>"grayscale_alpha",SemioColorspace::Indexed=>"indexed"}}
fn pixels(width:u32,height:u32)->Result<usize,String>{usize::try_from(width).map_err(|error|error.to_string())?.checked_mul(usize::try_from(height).map_err(|error|error.to_string())?).and_then(|count|count.checked_mul(4)).ok_or_else(||"Semio image pixel size overflow".into())}
impl ArtifactSqliteSnapshot for SemioImageSnapshot{
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="image"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_image_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key("semio_image_document",1,&[Cell::Text(&self.schema),Cell::Integer(i64::from(self.width)),Cell::Integer(i64::from(self.height)),Cell::Text(colorspace(self.colorspace)),Cell::Integer(i64::from(self.bit_depth)),match &self.icc{None=>Cell::Null,Some(value)=>Cell::Blob(value)}])?;
for(ordinal,frame)in self.frames.iter().enumerate(){if frame.rgba8.len()!=pixels(self.width,self.height)?{return Err("Semio image RGBA8 size disagrees with dimensions".into());}projection.insert("semio_image_frame",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Integer(i64::from(frame.delay_ms)),Cell::Blob(&frame.rgba8)])?;}
for(ordinal,metadata)in self.metadata.iter().enumerate(){projection.insert("semio_image_metadata",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&metadata.key),Cell::Text(&metadata.value)])?;}projection.finish()
}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioImageSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,String> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits()).map_err(|error|error.to_string())?;let document=database.table("semio_image_document")?.single_row()?;identity(document,7)?;if document.rowid!=1{return Err("invalid Semio image document identifier".into());}let width=u32::try_from(document.integer(2)?).map_err(|error|error.to_string())?;let height=u32::try_from(document.integer(3)?).map_err(|error|error.to_string())?;let bit_depth=u8::try_from(document.integer(5)?).map_err(|error|error.to_string())?;let colorspace=match document.text(4)?{"rgb"=>SemioColorspace::Rgb,"rgba"=>SemioColorspace::Rgba,"grayscale"=>SemioColorspace::Grayscale,"grayscale_alpha"=>SemioColorspace::GrayscaleAlpha,"indexed"=>SemioColorspace::Indexed,_=>return Err("unknown Semio image colorspace".into())};let icc=match &document.values[6]{SqliteValue::Null=>None,SqliteValue::Blob(value)=>Some(reconstruct_blob(control,value)?),_=>return Err("invalid Semio image ICC storage class".into())};
let mut frames=Vec::new();let mut ids=BTreeSet::new();let mut completed=0usize;for row in database.table("semio_image_frame")?.ordered_rows(2)?{identity(row,5)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err("invalid Semio image frame ownership or identity".into());}let rgba8=row.blob(4)?;if rgba8.len()!=pixels(width,height)?{return Err("Semio image RGBA8 size disagrees with dimensions".into());}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;frames.push(SemioImageFrame{delay_ms:u32::try_from(row.integer(3)?).map_err(|error|error.to_string())?,rgba8:reconstruct_blob(control,rgba8)?});completed+=1;}
let mut metadata=Vec::new();let mut ids=BTreeSet::new();for row in database.table("semio_image_metadata")?.ordered_rows(2)?{identity(row,5)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err("invalid Semio image metadata ownership or identity".into());}metadata.push(SemioImageMetadataEntry{key:reconstruct_text(control,row.text(3)?)?,value:reconstruct_text(control,row.text(4)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,width,height,colorspace,bit_depth,frames,icc,metadata})
    }
}

impl SemioImageSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),String>{b.text(&self.schema)?;b.scalars(4)?;b.entities(self.frames.len())?;for frame in &self.frames{b.scalars(1)?;b.bytes(&frame.rgba8)?;}if let Some(icc)=&self.icc{b.bytes(icc)?;}b.entities(self.metadata.len())?;for entry in &self.metadata{b.text(&entry.key)?;b.text(&entry.value)?;}Ok(())}
}
