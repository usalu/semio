//! 🚦️ ZIP native ownership and literal semantic row admission under one caller control.
use crate::standards::v2_0::subsets::base::schema::snapshot::{ZipSnapshot,ZipExtraField};
use semio_framework_os_kernel as store;
use store::ArtifactSqliteSnapshot as _;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn error(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
const TABLES:[(&str,usize);12]=[("zip_archive",4),("zip_central_extra_field",4),("zip_central_extra_field_byte",4),("zip_central_header",11),("zip_central_legacy_comment_byte",4),("zip_central_legacy_name_byte",4),("zip_entry",6),("zip_entry_byte",4),("zip_local_extra_field",4),("zip_local_extra_field_byte",4),("zip_local_header",6),("zip_local_legacy_name_byte",4)];
fn extent(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{let mut bytes=0usize;for statement in ZipSnapshot::SQLITE_SCHEMA.split(';'){bytes=bytes.checked_add(statement.trim().len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP schema bytes overflow"))?;}for(name,_)in TABLES{bytes=bytes.checked_add(name.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP schema names overflow"))?;}if bytes.max(ZipSnapshot::SQLITE_SCHEMA.len())>limits.max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP authored schema exceeds caller bytes"))}if limits.max_tables<TABLES.len()||TABLES.iter().any(|(_,width)|*width>limits.max_columns)||limits.max_rows<1{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"ZIP authored relational extent exceeds caller limits"))}Ok(())}
struct Borrowed{limits:store::sqlite_snapshot::SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Borrowed{fn add(&mut self,rows:usize,bytes:usize,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{let total=self.rows.checked_add(rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"ZIP semantic rows overflow"))?;let cells=self.bytes.checked_add(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP semantic cells overflow"))?;if total>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"ZIP semantic rows exceed caller limit"))}if cells>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP semantic cells exceed caller bytes"))}self.rows=total;self.bytes=cells;let mut remaining=rows;while remaining>0{let count=remaining.min(256);n.advance(count)?;remaining-=count;}Ok(())}}
fn field(value:&RecordValue,id:u16)->Result<&FieldValue,ValueError>{value.get(id).ok_or_else(||error("ZIP required native field is missing"))}
fn list(value:Option<&FieldValue>)->Result<&[FieldValue],ValueError>{match value{Some(FieldValue::List(values))=>Ok(values),_=>Err(error("ZIP requires an ordered native collection"))}}
fn record(value:&FieldValue,count:usize)->Result<&RecordValue,ValueError>{let FieldValue::Record(value)=value else{return Err(error("ZIP requires its declared native metadata record"))};if value.fields.len()!=count||value.fields.keys().any(|id|usize::from(*id)>=count){return Err(error("ZIP native record has undeclared fields"))}Ok(value)}
fn uint(value:&FieldValue,maximum:u64)->Result<(),ValueError>{if matches!(value,FieldValue::UInt(value)if *value<=maximum){Ok(())}else{Err(error("ZIP native unsigned field exceeds its declared width"))}}
fn boolean(value:&FieldValue)->Result<(),ValueError>{if matches!(value,FieldValue::Bool(_)){Ok(())}else{Err(error("ZIP native flag requires Bool"))}}
fn text(value:&FieldValue,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<usize,ValueError>{let FieldValue::Text(value)=value else{return Err(error("ZIP native lexical field requires Text"))};n.scoped_stage(|n|{n.begin_stage(value.len())?;for part in value.as_bytes().chunks(256){n.advance(part.len())?;}Ok::<_,ValueError>(())})?;Ok(value.len())}
fn add(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP semantic cell extent overflow"))}
fn octets(value:Option<&FieldValue>,c:&mut Borrowed,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{let Some(FieldValue::Bytes64(value))=value else{return Err(error("ZIP data requires intrinsic base64 octets"))};c.add(value.len(),value.len().checked_mul(32).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP octet rows overflow"))?,n)}
fn legacy(value:Option<&FieldValue>,c:&mut Borrowed,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{if matches!(value,Some(FieldValue::Absent)){return Ok(())}for value in list(value)?{uint(value,u64::from(u8::MAX))?;c.add(1,32,n)?;}Ok(())}
fn extras(value:Option<&FieldValue>,c:&mut Borrowed,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{for value in list(value)?{let value=record(value,2)?;uint(field(value,0)?,u64::from(u16::MAX))?;c.add(1,32,n)?;octets(value.get(1),c,n)?;}Ok(())}
fn admit_record(value:&RecordValue,limits:store::sqlite_snapshot::SqliteDatabaseLimits,n:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{extent(limits)?;n.scoped_stage(|n|{n.begin_stage(0)?;if value.fields.len()!=4||value.fields.keys().any(|id|*id>=4){return Err(error("ZIP root has undeclared fields"))}let mut c=Borrowed{limits,rows:0,bytes:0};let schema=text(field(value,0)?,n)?;let comment=text(field(value,2)?,n)?;boolean(field(value,3)?)?;c.add(1,add(add(16,schema)?,comment)?,n)?;
 for value in list(value.get(1))?{let value=record(value,3)?;let name=text(field(value,0)?,n)?;c.add(1,add(40,name)?,n)?;octets(value.get(1),&mut c,n)?;let metadata=record(field(value,2)?,4)?;uint(field(metadata,0)?,u64::from(u16::MAX))?;boolean(field(metadata,3)?)?;let local=record(field(metadata,1)?,6)?;for id in 0..4{uint(field(local,id)?,u64::from(u16::MAX))?;}c.add(1,48,n)?;legacy(local.get(5),&mut c,n)?;extras(local.get(4),&mut c,n)?;let central=record(field(metadata,2)?,11)?;for id in[0,1,2,3,4,9]{uint(field(central,id)?,u64::from(u16::MAX))?;}uint(field(central,10)?,u64::from(u32::MAX))?;let comment=text(field(central,7)?,n)?;c.add(1,add(80,comment)?,n)?;legacy(central.get(6),&mut c,n)?;legacy(central.get(8),&mut c,n)?;extras(central.get(5),&mut c,n)?;}n.checkpoint()})}
struct Census<'c,'p>{control:&'c mut SqliteSnapshotControl<'p>,rows:usize,bytes:usize,phase:SqliteSnapshotPhase}
impl Census<'_,'_>{
 fn add(&mut self,rows:usize,bytes:usize)->Result<(),ValueError>{
  let count=self.rows.checked_add(rows).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"ZIP semantic row count overflow"))?;self.control.check_rows(count)?;
  let total=self.bytes.checked_add(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP semantic value byte count overflow"))?;self.control.check_value_bytes(total)?;
  let mut checkpoint=(self.rows/256).checked_add(1).and_then(|value|value.checked_mul(256)).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"ZIP semantic progress overflow"))?;
  while checkpoint<=count{self.control.checkpoint(self.phase,checkpoint,0)?;checkpoint=checkpoint.checked_add(256).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"ZIP semantic progress overflow"))?;}
  self.rows=count;self.bytes=total;Ok(())
 }
 fn row(&mut self,scalars:usize,texts:&[&str])->Result<(),ValueError>{let bytes=texts.iter().try_fold(scalars.checked_mul(8).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP semantic scalar extent overflow"))?,|sum,value|sum.checked_add(value.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP semantic text extent overflow")))?;self.add(1,bytes)}
 fn octets(&mut self,count:usize)->Result<(),ValueError>{self.add(count,count.checked_mul(32).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP semantic byte entity extent overflow"))?)}
 fn fields(&mut self,values:&[ZipExtraField])->Result<(),ValueError>{for field in values{self.row(4,&[])?;self.octets(field.data.len())?;}Ok(())}
}
fn admit_snapshot(value:&ZipSnapshot,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{
 extent(control.limits())?;control.checkpoint(phase,0,0)?;let mut census=Census{control,rows:0,bytes:0,phase};
 census.row(2,&[&value.schema,&value.comment])?;
 for entry in &value.entries{
  census.row(5,&[&entry.name])?;census.octets(entry.data.len())?;let local=&entry.metadata.local;let central=&entry.metadata.central;
  census.row(6,&[])?;if let Some(bytes)=&local.unicode_path_legacy_name{census.octets(bytes.len())?;}census.fields(&local.extra_fields)?;
  census.row(10,&[&central.comment])?;if let Some(bytes)=&central.unicode_path_legacy_name{census.octets(bytes.len())?;}if let Some(bytes)=&central.unicode_comment_legacy{census.octets(bytes.len())?;}census.fields(&central.extra_fields)?;
 }
 census.control.checkpoint(phase,census.rows,census.rows)
}
pub(super) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<ZipSnapshot,ValueError>{
 extent(control.limits())?;let limits=control.limits();
 let snapshot=store::decode_sqlite_snapshot_record_native(payload,"stdio.zip",ZipSnapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {admit_record(record,limits,native)?;ZipSnapshot::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)?;admit_snapshot(&snapshot,control,SqliteSnapshotPhase::DecodeNative)?;Ok(snapshot)
}
pub(super) fn encode(snapshot:&ZipSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{
 admit_snapshot(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;
 store::encode_sqlite_snapshot_record_native(encoding,"stdio.zip",ZipSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control,native_owner)
}

pub(super) fn preflight(snapshot:&ZipSnapshot,_encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{admit_snapshot(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;let mut bound=store::sqlite_snapshot::artifact::NativeEncodingBound::file_only(control)?;bound.add(4096)?;bound.repeated(snapshot.schema.len(),6)?;bound.repeated(snapshot.comment.len(),6)?;for entry in &snapshot.entries{bound.add(1024)?;bound.repeated(entry.name.len(),6)?;bound.repeated(entry.data.len(),2)?;let local=&entry.metadata.local;let central=&entry.metadata.central;bound.repeated(central.comment.len(),6)?;for bytes in[local.unicode_path_legacy_name.as_deref(),central.unicode_path_legacy_name.as_deref(),central.unicode_comment_legacy.as_deref()].into_iter().flatten(){bound.repeated(bytes.len(),16)?;}for field in local.extra_fields.iter().chain(&central.extra_fields){bound.add(128)?;bound.repeated(field.data.len(),2)?;}}bound.finish()}
