//! 🧪️ Actual failed source effects retain their initialized original page prefix.
use super::*;
use semio_framework_value::RetainedWorkOutcome;
use semio_framework_job_payload::{JobPayloadWriteProgress,JobPayloadWriteFailure};
struct PartialSource<'a>{bytes:&'a[u8],written:usize}
impl JobPayloadStagedSource for PartialSource<'_>{
 fn byte_len(&self)->usize{self.bytes.len()}
 fn copy_byte_demand(&self)->Result<usize,JobPayloadAdmissionFault>{Ok(self.bytes.len())}
 fn depth_demand(&self)->Result<usize,JobPayloadAdmissionFault>{Ok(1)}
 fn write_bytes(&self,target:&mut[MaybeUninit<u8>])->RetainedWorkOutcome<JobPayloadWriteProgress,JobPayloadWriteFailure>{
  for(slot,byte)in target.iter_mut().zip(self.bytes).take(self.written){slot.write(*byte);}
  RetainedWorkOutcome{result:Err(JobPayloadWriteFailure{fault:JobPayloadAdmissionFault::StreamBytes,initialized_bytes:self.written,progress:RetainedCloneProgress{copied_items:1,copied_bytes:self.written,..Default::default()}}),work:1}
 }
}
#[test]
fn failed_original_writer_source_keeps_actual_prefix_work_and_same_bank_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("writer-actual.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("writer-actual.schema.json")).unwrap();for(key,value)in law.as_object().unwrap(){assert_eq!(&schema["properties"][key]["const"],value);}
 let bytes:Vec<u8>=serde_json::from_value(law["source"].clone()).unwrap();let expected:Vec<u8>=serde_json::from_value(law["expectedPublishedPrefix"].clone()).unwrap();let source=PartialSource{bytes:&bytes,written:law["failedInitializedBytes"].as_u64().unwrap()as usize};let mut original=OriginalFixture::new();
 let mut writer=RetainedPayloadBuilder::new(JobPayloadStream::CheckpointState);while !writer.is_initialized(){let d=writer.initialization_demand();original.turn(exact(d),|cx|{let turn=writer.advance_initialization(cx);receive_original_retained_work(cx,turn)});}
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if writer.staged_page_len().is_some(){break}let d=writer.staged_page_birth_demands().unwrap();original.turn(exact(d),|cx|{let turn=writer.begin_staged_page_funded(cx);receive_original_retained_work(cx,turn)});}
 assert_eq!(writer.staged_page_len(),Some(0));let pointer=writer.staged_backing_identity();let d=writer.staged_source_demands(&source).unwrap();let(failure,heap)=observe(||original.turn(exact(d),|cx|{let turn=writer.write_staged_source_funded(cx,&source);Ok(receive_original_retained_work(cx,turn))}));assert!(failure.is_err());assert_eq!(heap,(0,0));assert_eq!(writer.staged_page_len(),Some(source.written));assert_eq!(writer.staged_backing_identity(),pointer);assert_eq!(original.caller.receipt().copied_bytes,source.written);assert_eq!(original.caller.receipt().copied_items,1);
 let d=writer.staged_commit_demands().unwrap();let step=original.turn(exact(d),|cx|{let turn=writer.commit_staged_page_funded(cx);receive_original_retained_work(cx,turn)});assert!(matches!(step,RetainedCloneStep::Complete(_)));let mut payload=finish_writer(&mut original,&mut writer);assert_eq!(payload.page(0),Some(expected.as_slice()));original.close_payload(&mut payload);original.finish();
}
