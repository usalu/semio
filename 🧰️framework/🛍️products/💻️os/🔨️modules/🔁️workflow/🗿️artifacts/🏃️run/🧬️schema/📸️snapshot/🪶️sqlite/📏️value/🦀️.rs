//! 📏️ Borrowed Run literal leaves retain semantic byte authority before native ownership.
use super::{RunArtifact,RunTrigger};
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
fn add(total:&mut usize,bytes:usize,maximum:usize)->Result<(),ValueError>{
 *total=total.checked_add(bytes).filter(|count|*count<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Run native literal values exceed semantic byte limit"))?;Ok(())
}
fn field(value:&semio_framework_dsl_record::FieldValue,total:&mut usize,maximum:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 native.scoped_depth(32,|native|->Result<(),ValueError>{
  use semio_framework_dsl_record::FieldValue;
  match value{
   FieldValue::Bool(_)=>add(total,1,maximum)?,
   FieldValue::Int(_)|FieldValue::UInt(_)|FieldValue::Float(_)=>add(total,8,maximum)?,
   FieldValue::Enum(_)=>add(total,4,maximum)?,
   FieldValue::Text(text)=>add(total,text.len(),maximum)?,
   FieldValue::Absent=>{},
   FieldValue::List(values)=>{for value in values{field(value,total,maximum,native)?;}},
   FieldValue::Record(record)=>{for value in record.fields.values(){field(value,total,maximum,native)?;}},
   FieldValue::Block(value)=>field(value,total,maximum,native)?,
   _=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run native field is outside its authored literal domain")),
  }
  native.step()?;Ok(())
 })
}
pub(super) fn record(record:&semio_framework_dsl_record::RecordValue,maximum:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),ValueError>{
 native.scoped_stage(|native|{native.begin_stage(0)?;let mut total=0;for value in record.fields.values(){field(value,&mut total,maximum,native)?;}native.checkpoint()})
}
struct Census<'a,'b>{total:usize,completed:usize,rows:usize,control:&'a mut SqliteSnapshotControl<'b>}
impl Census<'_,'_>{
 fn bytes(&mut self,bytes:usize)->Result<(),ValueError>{add(&mut self.total,bytes,self.control.limits().max_value_bytes)}
 fn text(&mut self,text:&str)->Result<(),ValueError>{self.bytes(text.len())}
 fn step(&mut self)->Result<(),ValueError>{self.completed=self.completed.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Run native semantic census workload overflow"))?;if self.completed%256==0||self.completed==self.rows{self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,self.completed,self.rows)?;}Ok(())}
}
pub(super) fn owner(value:&RunArtifact,rows:usize,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,rows)?;
 let mut census=Census{total:0,completed:0,rows,control};
 for text in[&value.schema,&value.workflow_ref,&value.workflow_checkpoint_id,&value.input_collection_ref,&value.input_snapshot_id,&value.output_collection_ref,&value.started_at]{census.text(text)?;}
 if let Some(text)=&value.finished_at{census.text(text)?;}census.bytes(5)?;census.step()?;
 match &value.trigger{
  RunTrigger::Manual{actor}=>{census.text("manual")?;census.text(actor)?;},
  RunTrigger::Automation{automation_ref,event_fingerprint}=>{census.text("automation")?;census.text(automation_ref)?;census.text(event_fingerprint)?;},
 }census.step()?;
 for parameter in&value.parameter_values{census.text(&parameter.parameter_id)?;census.text(&parameter.value)?;census.step()?;}
 for node in&value.node_records{
  for text in[&node.node_id,&node.document_fingerprint,&node.config_fingerprint]{census.text(text)?;}census.bytes(12)?;
  for values in[&node.input_fingerprints,&node.output_fingerprints]{for value in values{census.text(&value.port_id)?;census.text(&value.fingerprint)?;census.step()?;}}
  for output in&node.outputs{for text in[&output.port_id,&output.artifact_id,&output.path]{census.text(text)?;}census.step()?;}
  census.step()?;
 }
 for log in&value.logs{for text in[&log.node_id,&log.level,&log.message,&log.at]{census.text(text)?;}census.step()?;}Ok(())
}

