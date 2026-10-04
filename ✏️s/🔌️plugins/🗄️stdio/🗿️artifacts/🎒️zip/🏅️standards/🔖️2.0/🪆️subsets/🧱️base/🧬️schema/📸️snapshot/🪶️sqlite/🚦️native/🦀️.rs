//! 🚦️ ZIP native ownership and literal semantic row admission under one caller control.
use super::super::{ZipSnapshot,ZipExtraField};
use semio_framework_os_kernel as store;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
use semio_framework_dsl_record::{FieldValue,RecordValue};
use semio_framework_diagnostic::{TextError,TextSpan};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn error(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
struct Rows{count:usize,maximum:usize}
impl Rows{fn new(maximum:usize)->Result<Self,ValueError>{let mut result=Self{count:0,maximum};result.add(1)?;Ok(result)}fn add(&mut self,count:usize)->Result<(),ValueError>{self.count=self.count.checked_add(count).filter(|n|*n<=self.maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"ZIP native semantic rows exceed caller limit"))?;Ok(())}}
fn list(value:Option<&FieldValue>)->Result<&[FieldValue],ValueError>{match value{None|Some(FieldValue::Absent)=>Ok(&[]),Some(FieldValue::List(values))=>Ok(values),_=>Err(error("ZIP requires an ordered native collection"))}}
fn record(value:Option<&FieldValue>)->Result<&RecordValue,ValueError>{match value{Some(FieldValue::Record(value))=>Ok(value),_=>Err(error("ZIP requires its declared native metadata record"))}}
fn octets(value:Option<&FieldValue>)->Result<usize,ValueError>{match value{None|Some(FieldValue::Absent)=>Ok(0),Some(FieldValue::Bytes64(value))=>Ok(value.len()),_=>Err(error("ZIP requires intrinsic base64 octets"))}}
fn legacy(value:Option<&FieldValue>)->Result<usize,ValueError>{list(value).map(<[FieldValue]>::len)}
fn native_fields(values:&[FieldValue],rows:&mut Rows,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(values.len())?;for value in values{let value=record(Some(value))?;rows.add(1)?;rows.add(octets(value.fields.get(&1))?)?;native.step()?;}Ok(())})
}
fn admit_record(value:&RecordValue,maximum:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 let mut rows=Rows::new(maximum)?;let entries=list(value.fields.get(&1))?;
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(entries.len())?;for entry in entries{
  let entry=record(Some(entry))?;rows.add(3)?;rows.add(octets(entry.fields.get(&1))?)?;
  if matches!(entry.fields.get(&2),None|Some(FieldValue::Absent)){native.step()?;continue;}
  let metadata=record(entry.fields.get(&2))?;let local=record(metadata.fields.get(&1))?;let central=record(metadata.fields.get(&2))?;
  native_fields(list(local.fields.get(&4))?,&mut rows,native)?;native_fields(list(central.fields.get(&5))?,&mut rows,native)?;
  rows.add(legacy(local.fields.get(&5))?)?;rows.add(legacy(central.fields.get(&6))?)?;rows.add(legacy(central.fields.get(&8))?)?;
  native.step()?;
 }Ok(())})
}
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
 control.checkpoint(phase,0,0)?;let mut census=Census{control,rows:0,bytes:0,phase};
 census.row(2,&[&value.schema,&value.comment])?;
 for entry in &value.entries{
  census.row(5,&[&entry.name])?;census.octets(entry.data.len())?;let local=&entry.metadata.local;let central=&entry.metadata.central;
  census.row(6,&[])?;if let Some(bytes)=&local.unicode_path_legacy_name{census.octets(bytes.len())?;}census.fields(&local.extra_fields)?;
  census.row(10,&[&central.comment])?;if let Some(bytes)=&central.unicode_path_legacy_name{census.octets(bytes.len())?;}if let Some(bytes)=&central.unicode_comment_legacy{census.octets(bytes.len())?;}census.fields(&central.extra_fields)?;
 }
 census.control.checkpoint(phase,census.rows,census.rows)
}
pub(super) fn decode(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<ZipSnapshot,ValueError>{
 control.check_rows(1)?;let maximum=control.limits().max_rows;
 let snapshot=store::decode_sqlite_snapshot_record_native(payload,"stdio.zip",ZipSnapshot::__dsl_spec_producer(),|record,native|{admit_record(record,maximum,native)?;ZipSnapshot::__dsl_from_record_controlled(record,native)},control)?;admit_snapshot(&snapshot,control,SqliteSnapshotPhase::DecodeNative)?;Ok(snapshot)
}
pub(super) fn encode(snapshot:&ZipSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
 admit_snapshot(snapshot,control,SqliteSnapshotPhase::EncodeNative)?;
 store::encode_sqlite_snapshot_record_native(encoding,"stdio.zip",ZipSnapshot::__dsl_spec_producer(),|native|snapshot.__dsl_to_record_controlled(native),control)
}
