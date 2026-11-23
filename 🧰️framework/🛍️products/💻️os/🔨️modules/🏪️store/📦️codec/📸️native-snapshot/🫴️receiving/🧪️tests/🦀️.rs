//! 🧫️ Actual original receiving turns reconstruct the independent serde owner from complete Pack framing.
use super::*;
use crate::original_close::{funded,funded_invoked,retire};
use semio_framework_value::RetireOwned;
use protocol::PackLimits;
#[derive(Debug,PartialEq,serde::Deserialize,semio_framework_dsl_record_derive::DslRecord,semio_framework_dsl_record_derive::DslReceiving,semio_framework_value::RetireOwned)]
struct OriginalRecord{text:String,items:Vec<String>}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:64<<20,maximum_capacity_bytes:64<<20,maximum_release_bytes:64<<20,maximum_depth:128}}
fn wire(expected:&OriginalRecord)->Vec<u8>{let body=semio_os_pack::record::encode_document(&OriginalRecord::__dsl_spec(),&expected.__dsl_to_record(),&Default::default()).unwrap();let envelope=semio_os_semio::SemioEnvelope::from_envelope_id("test.original",semio_os_semio::Component::Pack,1).unwrap();semio_os_semio::wrap_binary(&envelope,&body)}
fn step_once(owner:&mut OriginalSnapshotRecordReceiving<OriginalRecord>,source:&[u8],native:&mut NativeDecodeControl<'_>)->(RetainedCloneProgress,usize,usize){
 let(turn,born,released)=funded_invoked(stock(),|effect|semio_framework_trace_memory_testing::observe_backing(||owner.step(source,native,4096,effect)));
 (turn.result.unwrap(),born,released)
}
fn close_once(owner:&mut OriginalSnapshotRecordReceiving<OriginalRecord>)->(RetainedCloneProgress,usize,usize){
 let(poll,born,released)=funded_invoked(stock(),|effect|semio_framework_trace_memory_testing::observe_backing(||owner.close_poll(4096,effect)));
 let progress=match poll{RetainedWorkPoll::Complete(turn)=>turn.result.unwrap().progress(),RetainedWorkPoll::Pending{..}=>Default::default()};
 (progress,born,released)
}
#[test]
fn original_snapshot_pack_receiving_preserves_same_source_control_and_paid_close(){
 let neutral=include_str!("../../../../../../../../🔨️modules/🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🧫️fixtures/🔣️.json");let row:serde_json::Value=serde_json::from_str(neutral).unwrap();let expected:OriginalRecord=serde_json::from_value(serde_json::json!({"text":row["text"],"items":row["items"]})).unwrap();let source=wire(&expected);let pointer=source.as_ptr();
 let mut observer=|_|true;let mut native=NativeDecodeControl::new(64<<20,&mut observer);
 let(mut owner,_)=OriginalSnapshotRecordReceiving::<OriginalRecord>::admit(&source,"test.original",PackLimits::default(),&native,stock()).unwrap().unwrap();let mut turns=0;
 while !owner.is_ready(){let(progress,born,released)=step_once(&mut owner,&source,&mut native);assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(source.as_ptr(),pointer);turns+=1;assert!(turns<100000);}
 let mut output=None;owner.take_into(&mut output,stock()).unwrap();assert_eq!(output.as_ref(),Some(&expected));
 while !owner.terminal_is_empty(){let(progress,born,released)=close_once(&mut owner);assert_eq!((born,released),(progress.retained_capacity_bytes,progress.released_bytes));}
 retire(output.take().unwrap(),stock());drop(owner);
}

#[test]
fn original_snapshot_record_prefixes_retire_through_the_same_original_snapshot_recipient(){
 let expected=OriginalRecord{text:"original retained record 雪".repeat(40),items:vec!["one".into(),"two".into()]};let source=wire(&expected);
 for prefix in[0,1,8,64,512,4096,usize::MAX]{
  let mut observer=|_|true;let mut native=NativeDecodeControl::new(64<<20,&mut observer);
  let(mut owner,_)=OriginalSnapshotRecordReceiving::<OriginalRecord>::admit(&source,"test.original",PackLimits::default(),&native,stock()).unwrap().unwrap();let mut turns=0;
  while turns<prefix&&!owner.is_ready(){step_once(&mut owner,&source,&mut native);turns+=1;assert!(turns<100000);}
  retire(owner,stock());
 }
}
