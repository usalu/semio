//! 🔊️ Ordered audio channels, exact IEEE754 scalar samples and provenance tags.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::schema::snapshot::sqlite::native::Bound;
use super::{SemioAudioSnapshot,SemioAudioFormat,SemioAudioChannel,SemioAudioTag};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
use std::collections::{BTreeMap,BTreeSet};
fn integer(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn format(value:SemioAudioFormat)->&'static str{match value{SemioAudioFormat::Pcm8=>"pcm8",SemioAudioFormat::Pcm16=>"pcm16",SemioAudioFormat::Pcm24=>"pcm24",SemioAudioFormat::Pcm32=>"pcm32",SemioAudioFormat::Float32=>"f32",SemioAudioFormat::Float64=>"f64"}}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio row identity or columns"))}else{Ok(())}}
impl ArtifactSqliteSnapshot for SemioAudioSnapshot{
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{super::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{super::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{let mut b=Bound::new("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&store::os_io::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="audio"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_audio_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioAudioSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {

control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,declared_schema,control.limits())?;
let document=database.table("semio_audio_document")?.single_row()?;identity(document,4)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio document identifier"));}let sample_rate=u32::try_from(document.integer(2)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let format=match document.text(3)?{"pcm8"=>SemioAudioFormat::Pcm8,"pcm16"=>SemioAudioFormat::Pcm16,"pcm24"=>SemioAudioFormat::Pcm24,"pcm32"=>SemioAudioFormat::Pcm32,"f32"=>SemioAudioFormat::Float32,"f64"=>SemioAudioFormat::Float64,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio audio sample format"))};
let channels=database.table("semio_audio_channel")?.ordered_rows(2)?;let mut channel_ids=BTreeSet::new();for row in &channels{identity(row,3)?;if row.integer(1)?!=1||!channel_ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio channel owner or identifier"));}}
let mut samples=BTreeMap::<i64,Vec<&SqliteRow>>::new();let mut ids=BTreeSet::new();let mut completed=0usize;
for row in &database.table("semio_audio_sample")?.rows{identity(row,5)?;if !channel_ids.contains(&row.integer(1)?)||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio sample channel or identifier"));}row.integer(2)?;samples.entry(row.integer(1)?).or_default().push(row);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
let mut native_channels=Vec::new();for row in channels{let mut ordered=samples.remove(&row.rowid).unwrap_or_default();ordered.sort_by_key(|row|row.integer(2).unwrap_or(-1));let mut native_samples=Vec::new();for(ordinal,sample)in ordered.into_iter().enumerate(){if sample.integer(2)?!=integer(ordinal)?{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio audio sample ordinals must be contiguous"));}let bits=u32::try_from(sample.integer(4)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let value=f32::from_bits(bits);if value.is_nan(){if sample.values[3]!=SqliteValue::Null{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN Semio sample must have NULL numeric value"));}}else if sample.real(3)?!=f64::from(value){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio sample value disagrees with IEEE754 bits"));}native_samples.push(value);completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}native_channels.push(SemioAudioChannel{samples:native_samples});}
let mut tags=Vec::new();let mut ids=BTreeSet::new();for row in database.table("semio_audio_tag")?.ordered_rows(2)?{identity(row,5)?;if row.integer(1)?!=1||!ids.insert(row.rowid){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio tag owner or identifier"));}tags.push(SemioAudioTag{key:reconstruct_text(control,row.text(3)?)?,value:reconstruct_text(control,row.text(4)?)?});completed+=1;if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;}}
control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(Self{schema:reconstruct_text(control,document.text(1)?)?,sample_rate,format,channels:native_channels,tags})
    }
}

impl SemioAudioSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.scalars(2)?;b.entities(self.channels.len())?;for channel in &self.channels{b.entities(channel.samples.len())?;b.scalars(channel.samples.len())?;}b.entities(self.tags.len())?;for tag in &self.tags{b.text(&tag.key)?;b.text(&tag.value)?;}Ok(())}
}

impl SemioAudioSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;projection.insert_key("semio_audio_document",1,&[Cell::Text(&self.schema),Cell::Integer(i64::from(self.sample_rate)),Cell::Text(format(self.format))])?;
for(ordinal,channel)in self.channels.iter().enumerate(){let id=projection.insert("semio_audio_channel",&[Cell::Integer(1),Cell::Integer(integer(ordinal)?)])?;for(ordinal,sample)in channel.samples.iter().enumerate(){projection.insert("semio_audio_sample",&[Cell::Integer(id),Cell::Integer(integer(ordinal)?),if sample.is_nan(){Cell::Null}else{Cell::Real(f64::from(*sample))},Cell::Integer(i64::from(sample.to_bits()))])?;}}
for(ordinal,tag)in self.tags.iter().enumerate(){projection.insert("semio_audio_tag",&[Cell::Integer(1),Cell::Integer(integer(ordinal)?),Cell::Text(&tag.key),Cell::Text(&tag.value)])?;}projection.finish()
}
}
