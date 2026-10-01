//! 🎬️ Ordered elementary streams and timestamped, field-level compressed samples.
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use super::{SemioVideoSnapshot,SemioVideoStream,SemioVideoStreamKind,SemioVideoSample,SemioRational};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text,reconstruct_blob},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
fn integer(value:usize)->Result<i64,String>{i64::try_from(value).map_err(|error|error.to_string())}
fn identity(row:&SqliteRow,columns:usize)->Result<(),String>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err("invalid Semio video row identity or columns".into())}else{Ok(())}}
fn kind(value:SemioVideoStreamKind)->&'static str{match value{SemioVideoStreamKind::Video=>"video",SemioVideoStreamKind::Audio=>"audio",SemioVideoStreamKind::Subtitle=>"subtitle"}}
impl ArtifactSqliteSnapshot for SemioVideoSnapshot{
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),String>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="video"){return Err(String::from("Semio owned snapshot dialect differs from its dedicated semantic subset").into());}
let row=database.table("semio_video_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(String::from("Semio owned document identity differs from projected semantic fields").into());}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key("semio_video_document",1,&[Cell::Text(&self.schema)])?;
for(ordinal,stream)in self.streams.iter().enumerate(){if stream.rate.den==0{return Err("Semio video rational denominator cannot be zero".into());}let id=projection.insert("semio_video_stream",&[Cell::Integer(1),Cell::Integer(integer(ordinal)?),Cell::Text(kind(stream.kind)),Cell::Text(&stream.codec),Cell::Integer(i64::from(stream.width)),Cell::Integer(i64::from(stream.height)),Cell::Integer(stream.rate.num),Cell::Integer(stream.rate.den)])?;for(ordinal,sample)in stream.samples.iter().enumerate(){projection.insert("semio_video_sample",&[Cell::Integer(id),Cell::Integer(integer(ordinal)?),Cell::Text(&sample.pts.to_string()),Cell::Integer(i64::from(sample.key)),Cell::Blob(&sample.data)])?;}}projection.finish()
}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String> { Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) }
}

impl SemioVideoSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,String> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits()).map_err(|error|error.to_string())?;let document=database.table("semio_video_document")?.single_row()?;identity(document,2)?;if document.rowid!=1{return Err("invalid Semio video document identifier".into());}
let ordered=database.table("semio_video_stream")?.ordered_rows(2)?;let mut ids=BTreeSet::new();for row in &ordered{identity(row,9)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err("invalid Semio video stream ownership or identity".into());}}
let mut samples=BTreeMap::<i64,Vec<&SqliteRow>>::new();let mut sample_ids=BTreeSet::new();let mut completed=0usize;for row in &database.table("semio_video_sample")?.rows{identity(row,6)?;if !ids.contains(&row.integer(1)?)||!sample_ids.insert(row.rowid){return Err("invalid Semio video sample ownership or identity".into());}row.integer(2)?;samples.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut streams=Vec::new();for row in ordered{let kind=match row.text(3)?{"video"=>SemioVideoStreamKind::Video,"audio"=>SemioVideoStreamKind::Audio,"subtitle"=>SemioVideoStreamKind::Subtitle,_=>return Err("unknown Semio video stream kind".into())};let rate=SemioRational{num:row.integer(7)?,den:row.integer(8)?};if rate.den==0{return Err("Semio video rational denominator cannot be zero".into());}let mut ordered_samples=samples.remove(&row.rowid).unwrap_or_default();ordered_samples.sort_by_key(|row|row.integer(2).unwrap_or(-1));let mut native_samples=Vec::new();for(ordinal,sample)in ordered_samples.into_iter().enumerate(){if sample.integer(2)?!=integer(ordinal)?{return Err("Semio video sample ordinals must be contiguous".into());}let pts=sample.text(3)?.parse::<u64>().map_err(|error|error.to_string())?;if pts.to_string()!=sample.text(3)?{return Err("noncanonical Semio video timestamp".into());}let key=match sample.integer(4)?{0=>false,1=>true,_=>return Err("invalid Semio video key-frame flag".into())};native_samples.push(SemioVideoSample{pts,key,data:reconstruct_blob(control,sample.blob(5)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
streams.push(SemioVideoStream{kind,codec:reconstruct_text(control,row.text(4)?)?,width:u32::try_from(row.integer(5)?).map_err(|error|error.to_string())?,height:u32::try_from(row.integer(6)?).map_err(|error|error.to_string())?,rate,samples:native_samples});}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,streams})
    }
}

impl SemioVideoSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),String>{b.text(&self.schema)?;b.entities(self.streams.len())?;for stream in &self.streams{b.scalars(5)?;b.text(&stream.codec)?;b.entities(stream.samples.len())?;for sample in &stream.samples{b.scalars(2)?;b.bytes(&sample.data)?;}}Ok(())}
}
