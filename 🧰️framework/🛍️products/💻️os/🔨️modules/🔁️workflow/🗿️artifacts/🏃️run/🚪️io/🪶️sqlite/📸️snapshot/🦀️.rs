//! 🏃️ Exact persisted Run ownership through seven handcrafted relational tables.
use crate::{RunArtifact,RunTrigger,RunStatus,RunNodeStatus,RunParameterValue,RunNodeRecord,RunOutputArtifact,PortFingerprint,RunLogLine};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_diagnostic::{TextError,TextSpan};

use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{Projection,Cell,FloatColumn,insert_ieee754,read_binary64}}};
#[path="./💰️backing/🦀️.rs"]mod backing;
#[path="./📏️value/🦀️.rs"]mod value_bytes;
const DURATION:&[FloatColumn]=&[FloatColumn::Binary64(7)];
fn add(total:usize,count:usize)->Result<usize,ValueError>{total.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Run semantic row count overflow"))}
fn workload(value:&RunArtifact,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{
 control.checkpoint(phase,0,value.node_records.len())?;let mut count=add(add(add(2,value.parameter_values.len())?,value.node_records.len())?,value.logs.len())?;
 for(index,node)in value.node_records.iter().enumerate(){count=add(add(add(count,node.input_fingerprints.len())?,node.output_fingerprints.len())?,node.outputs.len())?;control.check_rows(count)?;if(index+1)%256==0{control.checkpoint(phase,index+1,value.node_records.len())?}}
 control.check_rows(count)?;Ok(count)
}
fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Run ordinal exceeds INTEGER width"))}
fn run_status(value:i64)->Result<RunStatus,ValueError>{match value{0=>Ok(RunStatus::Pending),1=>Ok(RunStatus::Running),2=>Ok(RunStatus::Succeeded),3=>Ok(RunStatus::Failed),4=>Ok(RunStatus::Canceled),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run status is undeclared"))}}
fn node_status(value:i64)->Result<RunNodeStatus,ValueError>{match value{0=>Ok(RunNodeStatus::Computed),1=>Ok(RunNodeStatus::CacheHit),2=>Ok(RunNodeStatus::Failed),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run node status is undeclared"))}}
fn list(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match value{Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run native collection requires a literal list"))}}
fn native_rows(record:&semio_framework_dsl_record::RecordValue,maximum:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 let check=|count|if count>maximum{Err(ValueError::new(ValueRefusalKind::WorkLimit,"Run native snapshot exceeds semantic row limit"))}else{Ok(())};
 let mut count=2usize;for index in[5,9,10]{count=add(count,list(record.get(index))?.len())?}check(count)?;
 let nodes=list(record.get(9))?;native.scoped_stage(|native|->Result<_,ValueError>{native.begin_stage(nodes.len())?;for node in nodes{let record=match node{semio_framework_dsl_record::FieldValue::Record(record)=>record,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run node requires a literal record"))};for index in[4,5,6]{count=add(count,list(record.get(index))?.len())?}check(count)?;native.step()?}Ok(())})
}
impl ArtifactSqliteSnapshot for RunArtifact{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->Result<Self,ValueError>{
  control.check_rows(2)?;control.check_value_bytes(40)?;let maximum=control.limits().max_rows;let maximum_value_bytes=control.limits().max_value_bytes;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {native_rows(record,maximum,native)?;value_bytes::record(record,maximum_value_bytes,native)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_owner)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut store::NativeSnapshotEncodeOwner<'_,'_>)->Result<store::io_schema::IoPayload,ValueError>{
  let rows=workload(self,control,SqliteSnapshotPhase::EncodeNative)?;value_bytes::owner(self,rows,control)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
 }
 fn preflight_sqlite_snapshot_encoding(&self,_:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let rows=workload(self,control,SqliteSnapshotPhase::EncodeNative)?;value_bytes::owner(self,rows,control)}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let count=workload(self,control,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=Projection::new(Self::SQLITE_SCHEMA,control)?;
  out.insert_key("run_document",1,&[Cell::Text(&self.schema),Cell::Text(&self.workflow_ref),Cell::Text(&self.workflow_checkpoint_id),Cell::Text(&self.input_collection_ref),Cell::Text(&self.input_snapshot_id),Cell::Text(&self.output_collection_ref),Cell::Integer(i64::from(crate::snapshot::run_status_ordinal(self.status))),Cell::Text(&self.started_at),self.finished_at.as_deref().map_or(Cell::Null,Cell::Text),Cell::Integer(i64::from(self.sealed))])?;
  match &self.trigger{RunTrigger::Manual{actor}=>{out.insert("run_trigger",&[Cell::Integer(1),Cell::Text("manual"),Cell::Text(actor),Cell::Null,Cell::Null])?},RunTrigger::Automation{automation_ref,event_fingerprint}=>{out.insert("run_trigger",&[Cell::Integer(1),Cell::Text("automation"),Cell::Null,Cell::Text(automation_ref),Cell::Text(event_fingerprint)])?}};
  for(index,value)in self.parameter_values.iter().enumerate(){out.insert("run_parameter_value",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Text(&value.parameter_id),Cell::Text(&value.value)])?;if(index+1)%256==0{out.checkpoint_total(count)?}}
  for(index,node)in self.node_records.iter().enumerate(){
   let id=insert_ieee754(&mut out,"run_node",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Text(&node.node_id),Cell::Integer(i64::from(crate::snapshot::run_node_status_ordinal(node.status))),Cell::Text(&node.document_fingerprint),Cell::Text(&node.config_fingerprint),Cell::Real(node.duration_ms)],DURATION)?;
   for(direction,values)in[("input",&node.input_fingerprints),("output",&node.output_fingerprints)]{for(index,value)in values.iter().enumerate(){out.insert("run_port_fingerprint",&[Cell::Integer(id),Cell::Text(direction),Cell::Integer(ordinal(index)?),Cell::Text(&value.port_id),Cell::Text(&value.fingerprint)])?;if(index+1)%256==0{out.checkpoint_total(count)?}}}
   for(index,value)in node.outputs.iter().enumerate(){out.insert("run_output_artifact",&[Cell::Integer(id),Cell::Integer(ordinal(index)?),Cell::Text(&value.port_id),Cell::Text(&value.artifact_id),Cell::Text(&value.path)])?;if(index+1)%256==0{out.checkpoint_total(count)?}}
   if(index+1)%256==0{out.checkpoint_total(count)?}
  }
  for(index,value)in self.logs.iter().enumerate(){out.insert("run_log",&[Cell::Integer(1),Cell::Integer(ordinal(index)?),Cell::Text(&value.node_id),Cell::Text(&value.level),Cell::Text(&value.message),Cell::Text(&value.at)])?;if(index+1)%256==0{out.checkpoint_total(count)?}}out.checkpoint_total(count)?;out.finish()
 }
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{backing::reconstruct(database,control)}
}
