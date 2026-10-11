//! 🎬️ Ordered elementary streams and timestamped, field-level compressed samples.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use crate::standards::v1::subsets::video::schema::snapshot::{SemioVideoSnapshot,SemioVideoStream,SemioVideoStreamKind,SemioVideoSample,SemioRational};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,reconstruct_text,reconstruct_blob},validate_sqlite_database_schema,SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase}};
#[path="💰️reconstruction/🦀️.rs"]
mod reconstruction;
#[path="🧮️semantic/🦀️.rs"]
mod semantic;
/// 🔢️ Formats canonical unsigned timestamp cells in fixed borrowed stack storage.
fn decimal(mut value:u64,buffer:&mut[u8;20])->&str{let mut at=buffer.len();loop{at-=1;buffer[at]=b'0'+(value%10)as u8;value/=10;if value==0{break}}std::str::from_utf8(&buffer[at..]).expect("decimal ASCII")}
/// 🎬️ Emits every authored stream and sample cell through one controlled row writer.
pub(crate)fn visit_rows(snapshot:&SemioVideoSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
out.insert_key("semio_video_document",1,&[Cell::Text(&snapshot.schema)])?;
for(ordinal,stream)in snapshot.streams.iter().enumerate(){let id=out.insert("semio_video_stream",&[Cell::Integer(1),Cell::Integer(integer(ordinal)?),Cell::Text(kind(stream.kind)),Cell::Text(&stream.codec),Cell::Integer(i64::from(stream.width)),Cell::Integer(i64::from(stream.height)),Cell::Integer(stream.rate.num),Cell::Integer(stream.rate.den)])?;for(ordinal,sample)in stream.samples.iter().enumerate(){let mut digits=[0u8;20];let pts=decimal(sample.pts,&mut digits);out.insert("semio_video_sample",&[Cell::Integer(id),Cell::Integer(integer(ordinal)?),Cell::Text(pts),Cell::Integer(i64::from(sample.key)),Cell::Blob(&sample.data)])?;}}Ok(())
}
/// 🫳️ Admits all actual typed cells before native forecast or output allocation.
pub(crate)fn admit_values(snapshot:&SemioVideoSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}
fn integer(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|error|ValueError::new(ValueRefusalKind::WorkLimit,error.to_string()))}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=columns{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video row identity or columns"))}else{Ok(())}}
fn kind(value:SemioVideoStreamKind)->&'static str{match value{SemioVideoStreamKind::Video=>"video",SemioVideoStreamKind::Audio=>"audio",SemioVideoStreamKind::Subtitle=>"subtitle"}}
impl ArtifactSqliteSnapshot for SemioVideoSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}
fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::video::io::sqlite::snapshot::native_encoding::encode(self,encoding,control,native_owner)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{crate::standards::v1::subsets::video::io::sqlite::snapshot::native_decoding::decode(payload,control,native_control.native())}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="video"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_video_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(database, control, Self::SQLITE_SCHEMA)}
}

impl SemioVideoSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;crate::standards::v1::subsets::base::io::sqlite::snapshot::projection::project_rows_owned(Self::SQLITE_SCHEMA,control,|out|visit_rows(self,out))
}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,declared_schema:&str)->Result<Self,ValueError>{reconstruction::reconstruct(database,control,declared_schema)}
}

impl SemioVideoSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.streams.len())?;for stream in &self.streams{b.scalars(5)?;b.text(&stream.codec)?;b.entities(stream.samples.len())?;for sample in &stream.samples{b.scalars(2)?;b.bytes(&sample.data)?;}}Ok(())}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
