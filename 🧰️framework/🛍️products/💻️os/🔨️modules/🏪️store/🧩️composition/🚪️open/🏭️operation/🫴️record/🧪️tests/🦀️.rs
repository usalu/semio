//! 🧪️ Actual Drawing opens from the original framed member pages and retires every partial native owner.
use super::*;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_framework_os_store_owned_schema_decode::{OwnedSchemaDecodePages,OwnedSchemaDecodeCredits,OwnedSchemaDecodePage};
use semio_framework_job::{OriginalJobDecodeOwner,OperationId,Generation,StepBudget,root_cancel_token};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use pack::snapshot::ArtifactPack;
fn policy()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:256}}
fn request(bytes:&[u8])->MemberOpenRequest{
 let mut frame=Vec::new();let mut n=bytes.len();loop{let b=(n&127)as u8;n>>=7;frame.push(b|if n==0{0}else{128});if n==0{break}}frame.extend_from_slice(bytes);frame.push(0);
 let mut pages=OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits{maximum_pages:frame.len().div_ceil(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES),maximum_bytes:frame.len()}).unwrap();for page in frame.chunks(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES){pages.admit_page(OwnedSchemaDecodePage::try_from_slice(page).unwrap()).unwrap()}pages.seal().unwrap();
 let expected=semio_framework_artifact_reference::ArtifactRef{artifact_id:"drawing-original".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.draw.drawing".into(),standard:"1".into(),subset:"*".into()}};
 MemberOpenRequest::new(OperationId(91),Generation(7),1000,expected,None,pages,semio_framework_os_spr::ActorId("author".into())).admit(1).unwrap_or_else(|_|panic!("original Drawing request"))
}
fn close<T:RetireOwned>(value:T){let mut c=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("typed Drawing law close"));for _ in 0..100000{if c.terminal_is_empty(){return}c.step(policy()).unwrap();}panic!("typed Drawing closure stalled")}
#[test]
fn genuine_original_drawing_member_receives_closes_and_funds_exact_handoff(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️.json")).unwrap();let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(law["cases"].as_array().unwrap().len(),5);
 let expected:DrawingSnapshot=serde_json::from_value(corpus["base"].clone()).unwrap();let bytes=expected.encode_pack();let grant=policy();
 for cut in [0,1,12,50,200,100000]{
  let mut open=OriginalRecordMemberSnapshotOpen::<DrawingSnapshot>::begin(request(&bytes)).unwrap_or_else(|_|panic!("actual original Drawing open"));let cancel=root_cancel_token();let mut native=OriginalJobDecodeOwner::empty();let mut sequence=0;let mut ready=false;
  for _ in 0..cut{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(91),Generation(7),StepBudget::new(1<<24,999,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt).with_original_decoder(&mut native);
   let d=open.opening_demand(&cx,grant.maximum_copy_bytes).unwrap();for axis in 0..5{let mut denied=grant;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=0,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=0,3 if d.release_bytes>0=>denied.maximum_release_bytes=0,4 if d.depth>0=>denied.maximum_depth=0,_=>continue}let before=open.phase;let(step,heap)=observe_retirement_allocations(||open.step(&mut cx,denied).unwrap());assert!(matches!(step,MemberSnapshotOpenStep::Pending(_)));assert_eq!(open.last_receipt(),Default::default());assert_eq!(heap,(0,0));assert!(open.phase==before);assert_eq!(open.opening_demand(&cx,grant.maximum_copy_bytes).unwrap(),d);}
   let(step,heap)=observe_retirement_allocations(||open.step(&mut cx,grant).unwrap());let p=open.last_receipt();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(cx.retained_progress(),p);assert!(p.fits(grant));if matches!(step,MemberSnapshotOpenStep::Ready){ready=true;break}
  }
  if cut==100000{assert!(ready)}
  if ready{assert!(open.receiving.as_ref().unwrap().terminal_is_empty());assert!(open.parser.as_ref().unwrap().terminal_is_empty());let mut receipt=Default::default();let denied=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let mut cx=StepContext::new(OperationId(91),Generation(7),StepBudget::new(1<<24,999,denied),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=observe_retirement_allocations(||open.take_ready(&mut cx).unwrap());assert!(result.is_none());assert_eq!(heap,(0,0));assert!(open.snapshot.is_some());drop(cx);
   let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(91),Generation(7),StepBudget::new(1<<24,999,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);let(result,heap)=observe_retirement_allocations(||open.take_ready(&mut cx).unwrap());assert_eq!(heap,(0,0));let(snapshot,original)=result.unwrap();assert_eq!(snapshot,expected);assert!(open.terminal_is_empty());assert!(cx.retained_progress().copied_bytes>0);close(snapshot);let mut original=original;while !original.terminal_is_empty(){original.close_step(grant).unwrap();}
  }
  for _ in 0..100000{if open.terminal_is_empty(){break}let d=open.close_demand(grant.maximum_copy_bytes).unwrap();let denied=RetainedCloneGrant{maximum_items:0,..grant};let(step,heap)=observe_retirement_allocations(||open.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(open.close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||open.close_step(grant).unwrap());let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));}
  assert!(open.terminal_is_empty());let(_,heap)=observe_retirement_allocations(||drop(open));assert_eq!(heap,(0,0));while !native.terminal_is_empty(){native.close_step(grant).unwrap();}
 }
 close(expected);
}
#[test]
fn foreign_declared_snapshot_envelope_retains_original_request_and_error_receipt(){
 let grant=policy();let mut open=OriginalRecordMemberSnapshotOpen::<DrawingSnapshot>::begin(request(b"foreign-envelope")).unwrap_or_else(|_|panic!("original foreign request"));let cancel=root_cancel_token();let mut sequence=0;let mut refused=false;
 for _ in 0..32{let mut receipt=Default::default();let mut cx=StepContext::new(OperationId(91),Generation(7),StepBudget::new(1<<24,999,grant),cancel.clone(),||Some(1),&mut sequence,&mut receipt);match open.step(&mut cx,grant){Ok(_)=>{},Err(failure)=>{assert_eq!(cx.retained_progress(),failure.retained_progress());assert!(open.request.is_some());assert!(open.parser.is_none());refused=true;break}}}
 assert!(refused);for _ in 0..100000{if open.terminal_is_empty(){break}open.close_step(grant).unwrap();}assert!(open.terminal_is_empty());
}
