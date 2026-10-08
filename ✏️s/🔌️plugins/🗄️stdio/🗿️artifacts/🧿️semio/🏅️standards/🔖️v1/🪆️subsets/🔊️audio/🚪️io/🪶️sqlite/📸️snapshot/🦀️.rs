//! 🔊️ Ordered audio channels, exact IEEE754 scalar samples and provenance tags.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::audio::schema::snapshot::{SemioAudioSnapshot,SemioAudioFormat,SemioAudioChannel,SemioAudioTag};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="🧮️semantic/🦀️.rs"]
mod semantic;

/// 🎼️ Emits the original five-column raw-word sample rows through one authored writer.
pub(crate)fn visit_rows(snapshot:&SemioAudioSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 out.insert_key("semio_audio_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(i64::from(snapshot.sample_rate)),Cell::Text(format(snapshot.format))])?;
 for(ordinal,channel)in snapshot.channels.iter().enumerate(){let id=out.insert("semio_audio_channel",&[Cell::Integer(1),Cell::Integer(integer(ordinal)?)])?;for(ordinal,sample)in channel.samples.iter().enumerate(){out.insert("semio_audio_sample",&[Cell::Integer(id),Cell::Integer(integer(ordinal)?),if sample.is_nan(){Cell::Null}else{Cell::Real(f64::from(*sample))},Cell::Integer(i64::from(sample.to_bits()))])?;}}
 for(ordinal,tag)in snapshot.tags.iter().enumerate(){out.insert("semio_audio_tag",&[Cell::Integer(1),Cell::Integer(integer(ordinal)?),Cell::Text(&tag.key),Cell::Text(&tag.value)])?;}Ok(())
}
/// 🫳️ Visits all actual typed cells before native forecast or output allocation.
pub(crate)fn admit_values(snapshot:&SemioAudioSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn integer(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn format(value:SemioAudioFormat)->&'static str{match value{SemioAudioFormat::Pcm8=>"pcm8",SemioAudioFormat::Pcm16=>"pcm16",SemioAudioFormat::Pcm24=>"pcm24",SemioAudioFormat::Pcm32=>"pcm32",SemioAudioFormat::Float32=>"f32",SemioAudioFormat::Float64=>"f64"}}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio row identity or columns"))}else{Ok(())}}
impl ArtifactSqliteSnapshot for SemioAudioSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::audio::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::audio::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="audio"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_audio_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioAudioSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{artifact::RowIndex,transfer::reserve};
 use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
 control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let document=database.table("semio_audio_document")?.single_row()?;identity(document,4)?;if document.rowid!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio document identifier"))}
 let sample_rate=u32::try_from(document.integer(2)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;
 let format=match document.text(3)?{"pcm8"=>SemioAudioFormat::Pcm8,"pcm16"=>SemioAudioFormat::Pcm16,"pcm24"=>SemioAudioFormat::Pcm24,"pcm32"=>SemioAudioFormat::Pcm32,"f32"=>SemioAudioFormat::Float32,"f64"=>SemioAudioFormat::Float64,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio audio sample format"))};
 let channels=RowIndex::new(database,"semio_audio_channel",3,&[],control,"invalid Semio audio channel owner or identifier")?;
 let samples=RowIndex::new(database,"semio_audio_sample",5,&[],control,"invalid Semio sample channel or identifier")?;
 let tags=RowIndex::new(database,"semio_audio_tag",5,&[],control,"invalid Semio audio tag owner or identifier")?;
 let order=channels.ordered(2,control,"Semio audio channel ordinals must be contiguous")?;let tag_order=tags.ordered(2,control,"Semio audio tag ordinals must be contiguous")?;
 for(count,&index)in channels.indices().iter().enumerate(){if channels.row(index)?.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio channel owner or identifier"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,channels.len())?;}
 for(count,&index)in samples.indices().iter().enumerate(){let row=samples.row(index)?;if channels.get(row.integer(1)?,control)?.is_none(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio sample channel or identifier"))}row.integer(2)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count+1,samples.len())?;}
 let grouped=samples.grouped_by(2,control,"Semio audio sample ordinals must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut snapshot=Owned::new(Self{schema:String::new(),sample_rate,format,channels:Vec::new(),tags:Vec::new()});snapshot.get_mut().channels=reserve(order.len(),control)?;snapshot.get_mut().tags=reserve(tag_order.len(),control)?;
 let mut completed=0;
 for index in order{
  let row=channels.row(index)?;let range=samples.range_by(&grouped,(0,Some(row.rowid)),control,|row|Ok((0,Some(row.integer(1)?))))?;
  let mut channel=Owned::new(SemioAudioChannel{samples:Vec::new()});channel.get_mut().samples=reserve(range.len(),control)?;
  for position in range{
   let row=samples.row(grouped[position])?;let bits=u32::try_from(row.integer(4)?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let value=f32::from_bits(bits);
   if value.is_nan(){if row.values[3]!=SqliteValue::Null{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"NaN Semio sample must have NULL numeric value"))}}else if row.real(3)?!=f64::from(value){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio sample value disagrees with IEEE754 bits"))}
   channel.get_mut().samples.push(value);completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  snapshot.get_mut().channels.push(channel.take());
 }
 for index in tag_order{
  let row=tags.row(index)?;if row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio audio tag owner or identifier"))}
  let mut tag=Owned::new(SemioAudioTag{key:String::new(),value:String::new()});tag.get_mut().key=reconstruct_text(control,row.text(3)?)?;tag.get_mut().value=reconstruct_text(control,row.text(4)?)?;snapshot.get_mut().tags.push(tag.take());completed+=1;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
 }
 snapshot.get_mut().schema=reconstruct_text(control,document.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,completed)?;Ok(snapshot.take())
}
}

impl SemioAudioSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.scalars(2)?;b.entities(self.channels.len())?;for channel in &self.channels{b.entities(channel.samples.len())?;b.scalars(channel.samples.len())?;}b.entities(self.tags.len())?;for tag in &self.tags{b.text(&tag.key)?;b.text(&tag.value)?;}Ok(())}
}

impl SemioAudioSnapshot{
/// 🧮️ Projects owned semantic rows under the caller's typed resource control.
pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()
}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
