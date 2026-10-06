//! 💰️ Run reconstruction settles concrete indexes and direct typed field ownership.
use super::{RunArtifact,RunTrigger,RunParameterValue,RunNodeRecord,RunOutputArtifact,PortFingerprint,RunLogLine,DURATION,run_status,node_status};
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema_controlled,artifact::read_binary64};
#[path="./🔍️rows/🦀️.rs"]mod rows;
fn retire(value:RunArtifact){semio_framework_dsl_record::DslField::retire_decoded(value)}
fn fingerprints(rows:&[&SqliteRow],native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Vec<PortFingerprint>,ValueError>{
 native.scoped_stage(|native|{native.begin_stage(rows.len())?;let mut values=native.allocate_vec(rows.len())?;for row in rows{values.push(PortFingerprint{port_id:native.copy_text(row.text(4)?)?,fingerprint:native.copy_text(row.text(5)?)?});native.step()?;}Ok(values)})
}
fn construct(database:&SqliteDatabase,rows:&rows::Rows<'_>,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<semio_framework_dsl_record::__rt::DecodedFieldOwner<RunArtifact>,ValueError>{
 let count=2usize.checked_add(rows.parameters.len()).and_then(|n|n.checked_add(rows.nodes.len())).and_then(|n|n.checked_add(rows.logs.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Run native binding workload overflow"))?;
 native.begin_stage(count)?;
 let document=database.table("run_document")?.single_row()?;let trigger=database.table("run_trigger")?.single_row()?;
 let mut owner=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(RunArtifact{schema:String::new(),workflow_ref:String::new(),workflow_checkpoint_id:String::new(),input_collection_ref:String::new(),input_snapshot_id:String::new(),parameter_values:Vec::new(),output_collection_ref:String::new(),status:run_status(document.integer(7)?)?,trigger:RunTrigger::Manual{actor:String::new()},node_records:Vec::new(),logs:Vec::new(),started_at:String::new(),finished_at:None,sealed:match document.integer(10)?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run sealed flag must be zero or one"))}},retire);
 let value=owner.as_mut();value.schema=native.copy_text(document.text(1)?)?;value.workflow_ref=native.copy_text(document.text(2)?)?;value.workflow_checkpoint_id=native.copy_text(document.text(3)?)?;value.input_collection_ref=native.copy_text(document.text(4)?)?;value.input_snapshot_id=native.copy_text(document.text(5)?)?;value.output_collection_ref=native.copy_text(document.text(6)?)?;value.started_at=native.copy_text(document.text(8)?)?;value.finished_at=document.optional_text(9)?.map(|text|native.copy_text(text)).transpose()?;native.step()?;
 value.trigger=match trigger.text(2)?{"manual" if trigger.values[4]==SqliteValue::Null&&trigger.values[5]==SqliteValue::Null=>RunTrigger::Manual{actor:native.copy_text(trigger.text(3)?)?},"automation" if trigger.values[3]==SqliteValue::Null=>RunTrigger::Automation{automation_ref:native.copy_text(trigger.text(4)?)?,event_fingerprint:native.copy_text(trigger.text(5)?)?},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run trigger fields violate their declared branch"))};native.step()?;
 value.parameter_values=native.allocate_vec(rows.parameters.len())?;for row in&rows.parameters{value.parameter_values.push(RunParameterValue{parameter_id:native.copy_text(row.text(3)?)?,value:native.copy_text(row.text(4)?)?});native.step()?;}
 value.node_records=native.allocate_vec(rows.nodes.len())?;
 for row in&rows.nodes{
  let input=fingerprints(rows.fingerprints(row.rowid,true)?,native)?;let output=fingerprints(rows.fingerprints(row.rowid,false)?,native)?;let artifacts=rows.outputs(row.rowid)?;
  let outputs=native.scoped_stage(|native|{native.begin_stage(artifacts.len())?;let mut values=native.allocate_vec(artifacts.len())?;for row in artifacts{values.push(RunOutputArtifact{port_id:native.copy_text(row.text(3)?)?,artifact_id:native.copy_text(row.text(4)?)?,path:native.copy_text(row.text(5)?)?});native.step()?;}Ok::<_,ValueError>(values)})?;
  value.node_records.push(RunNodeRecord{node_id:native.copy_text(row.text(3)?)?,status:node_status(row.integer(4)?)?,document_fingerprint:native.copy_text(row.text(5)?)?,config_fingerprint:native.copy_text(row.text(6)?)?,input_fingerprints:input,output_fingerprints:output,outputs,duration_ms:read_binary64(row,7,DURATION)?});native.step()?;
 }
 value.logs=native.allocate_vec(rows.logs.len())?;for row in&rows.logs{value.logs.push(RunLogLine{node_id:native.copy_text(row.text(3)?)?,level:native.copy_text(row.text(4)?)?,message:native.copy_text(row.text(5)?)?,at:native.copy_text(row.text(6)?)?});native.step()?;}native.checkpoint()?;Ok(owner)
}
pub(super) fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<RunArtifact,ValueError>{
 let phase=SqliteSnapshotPhase::ReconstructSnapshot;validate_sqlite_database_schema_controlled(database,<RunArtifact as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA,phase,control)?;control.check_database(database,phase)?;
 let document=database.table("run_document")?.single_row()?;let trigger=database.table("run_trigger")?.single_row()?;
 if document.rowid!=1||document.values.len()!=11||document.integer(0)?!=1||trigger.rowid<=0||trigger.values.len()!=6||trigger.integer(0)?!=trigger.rowid||trigger.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Run requires complete aliased document and trigger ownership"))}
 let rows=rows::Rows::new(database,control)?;
 let owner=control.allocation_stage(phase,|remaining,progress|{let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|progress(event.completed,event.total);let mut native=semio_framework_value::NativeDecodeControl::new(remaining,&mut callback);let result=construct(database,&rows,&mut native);(result,native.owned_bytes())})??;
 control.checkpoint(phase,rows.nodes.len(),rows.nodes.len())?;Ok(owner.take())
}
