use semio_framework_value::{ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use protocol::codec::{HistoryReadLimits, RetainedWireByteSource};
use semio_framework_os_store_member_open::receiving::MemberHistoryReceiving;
use semio_framework_value::native_decoding::allocation::NativeForwardedDecodeReceipt;
use semio_framework_os_spr::history::RetainedHistorySourceStep;
use semio_framework_os_store_testing_doubles::OriginalMemberCaller;
use semio_framework_os_store_owned_schema_decode::{OwnedSchemaDecodePages, OwnedSchemaDecodePage, OwnedSchemaDecodeCredits};
use protocol::codec::OriginalWireSourceIdentity;

struct OriginalPages<'a>{pages:&'a OwnedSchemaDecodePages,identity:OriginalWireSourceIdentity}
impl RetainedWireByteSource for OriginalPages<'_>{fn original_source_identity(&self)->OriginalWireSourceIdentity{self.identity}fn byte_at(&self,index:usize)->Option<u8>{self.pages.byte_at(index)}}
fn hex(value:&str)->Vec<u8>{value.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect()}
async fn original_bytes(law:&serde_json::Value)->Vec<u8>{let options=protocol::format::WriteOptions{required_flags:protocol::REQUIRED_HASH_CHAIN,optional_flags:protocol::OPTIONAL_CANONICAL};let mut writer=protocol::format::SprWriter::begin(Vec::new(),&options).await.unwrap();for row in law["records"].as_array().unwrap(){writer.write_record(row["kind"].as_u64().unwrap()as u8,row["flags"].as_u64().unwrap()&2!=0,&hex(row["payloadHex"].as_str().unwrap()),protocol::codec::ids::CodecId(0)).await.unwrap();}writer.commit().await.unwrap();writer.into_sink().await}
fn measure<T>(run:impl FnOnce()->T)->(T,usize,usize){semio_framework_trace_memory_testing::observe_backing(run)}
fn exact(grant:RetainedCloneGrant,receipt:RetainedCloneProgress,born:usize,released:usize){assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(born,released))}

#[test]
fn original_member_history_receiving_reborrows_actual_caller_retains_pages_and_pays_cancelled_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let bytes=semio_framework_async::poll::resolve_ready(original_bytes(&law));
 let limits:HistoryReadLimits=serde_json::from_value(law["limits"].clone()).unwrap();
 let maximum=law["control"]["maximumAllocationBytes"].as_u64().unwrap()as usize;
 let receiving_law:serde_json::Value=serde_json::from_str(include_str!("../../../📜️history/🏭️factory/🧪️tests/🧫️receiving/🔣️.json")).unwrap();
 for cut in receiving_law["sourceReceiving"]["cuts"].as_array().unwrap().iter().map(|cut|cut.as_u64().unwrap()as usize){
  let mut caller=OriginalMemberCaller::new();let grant=caller.policy();
  let page_bytes=semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES;
  assert_eq!(page_bytes,receiving_law["sourceReceiving"]["originalPageBytes"].as_u64().unwrap()as usize);
  let(mut pages,input_born,input_released)=measure(||{let mut pages=OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits{maximum_pages:bytes.len().div_ceil(page_bytes),maximum_bytes:bytes.len()}).unwrap();for chunk in bytes.chunks(page_bytes){pages.admit_page(OwnedSchemaDecodePage::try_from_slice(chunk).unwrap()).unwrap()}pages.seal().unwrap();pages});
  assert_eq!(input_released,0);
  let source=OriginalPages{identity:OriginalWireSourceIdentity{address:pages.slots.as_ptr()as usize,extent:pages.byte_count(),authority:Some((1,1))},pages:&pages};
  let original=source.original_source_identity();
  let((loan,receipt),born,released)=measure(||caller.run(grant,|parent,cx|parent.admit_loan(cx).unwrap().unwrap()));exact(grant,receipt,born,released);assert_eq!((born,released),(0,0));let mut loan=loan;
  let mut receiving=MemberHistoryReceiving::new((0,bytes.len()),limits,maximum);let mut history=None;
  let(mut births,mut releases)=(0,0);
  for _ in 0..cut{
   if receiving.terminal_is_empty(){break}
   let demand=receiving.demands(grant.maximum_copy_bytes).unwrap();
   for(axis,denied)in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}].into_iter().enumerate(){
    if axis==1&&demand.copy_bytes==0||axis==2&&demand.capacity_bytes==0||axis==3&&demand.release_bytes==0{continue}
    let before=receiving.receipt.as_ref().map_or(0,NativeForwardedDecodeReceipt::owned_bytes);
    let((step,receipt),born,released)=measure(||caller.run(denied,|_,cx|{loan.validate_context(cx).unwrap();receiving.step(&source,cx,denied,&mut history).unwrap()}));
    assert_eq!(step,RetainedHistorySourceStep::Blocked,"cut{cut} axis{axis} demand{demand:?}");exact(denied,receipt,born,released);assert_eq!((born,released),(0,0));assert_eq!(before,receiving.receipt.as_ref().map_or(0,NativeForwardedDecodeReceipt::owned_bytes));
   }
   let((step,receipt),born,released)=measure(||caller.run(grant,|_,cx|{loan.validate_context(cx).unwrap();receiving.step(&source,cx,grant,&mut history).unwrap()}));
   assert!(!matches!(step,RetainedHistorySourceStep::Blocked|RetainedHistorySourceStep::Fault),"actual original Member decode must advance at cut{cut}");exact(grant,receipt,born,released);births+=born;releases+=released;assert_eq!(source.original_source_identity(),original);
  }
  if cut==65536{assert!(receiving.terminal_is_empty());let decoded=history.as_ref().unwrap();assert_eq!(decoded.doc_id,law["expected"]["docId"].as_str().unwrap());assert_eq!(decoded.schema,law["expected"]["schema"].as_str().unwrap());assert_eq!(decoded.edits.len(),2);assert_eq!(decoded.viewer_line.as_deref(),law["expected"]["viewerLine"].as_str());assert_eq!(decoded.viewer_checkpoint.as_deref(),law["expected"]["viewerCheckpoint"].as_str());}
  else{let(((),receipt),born,released)=measure(||caller.run(grant,|_,cx|cx.original_cancel_token().cancel_now()));exact(grant,receipt,born,released);assert_eq!((born,released),(0,0));}
  for _ in 0..caller.maximum_turns(){if receiving.terminal_is_empty(){break}let((step,receipt),born,released)=measure(||caller.run(grant,|_,cx|{loan.validate_context(cx).unwrap();receiving.close_step(&source,cx,grant).unwrap()}));exact(grant,receipt,born,released);assert_eq!(step.progress(),receipt);assert!(receipt.copied_items>0||matches!(step,RetainedCloneStep::Complete(_)));births+=born;releases+=released;assert_eq!(source.original_source_identity(),original);}
  assert!(receiving.terminal_is_empty());let(_,born,released)=measure(||drop(receiving));assert_eq!((born,released),(0,0));
  let mut active:Option<Box<dyn ErasedSnapshotRetirement>>=None;
  for _ in 0..caller.maximum_turns(){if history.is_none()&&active.is_none(){break}let((step,receipt),born,released)=measure(||caller.run(grant,|_,cx|{loan.validate_context(cx).unwrap();let raw=if active.is_some(){semio_framework_os_store_read::artifact_retirement_box_close_step(&mut active,grant)}else{semio_framework_os_store_read::artifact_retirement_admit_owned(&mut history,&mut active,grant)};semio_framework_os_store_member_open::receive_member_close(cx,raw).unwrap()}));exact(grant,receipt,born,released);assert_eq!(step.progress(),receipt);births+=born;releases+=released;}
  assert!(history.is_none()&&active.is_none());assert_eq!(births,releases,"actual Member receiving custody cut{cut}");
  let((step,receipt),born,released)=measure(||caller.run(grant,|parent,cx|loan.close_step(parent,cx).unwrap()));exact(grant,receipt,born,released);assert_eq!(step.progress(),receipt);assert!(loan.terminal_is_empty());assert_eq!((born,released),(0,0));drop(loan);
  let mut input_releases=0;for _ in 0..caller.maximum_turns(){if pages.terminal_is_empty(){break}let((step,receipt),born,released)=measure(||caller.run(grant,|_,cx|{let raw=if pages.close_take_page().is_some(){Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{pages.close_backing_step(grant)};semio_framework_os_store_member_open::receive_member_close(cx,raw).unwrap()}));exact(grant,receipt,born,released);assert_eq!(born,0);assert_eq!(step.progress(),receipt);input_releases+=released;}
  assert!(pages.terminal_is_empty());assert_eq!(input_born,input_releases);caller.close();
  eprintln!("[DEBUG] Actual Member History cut{cut} originalPages={input_born} decodeBorn={births} decodeReleased={releases} caller1/4096/65536/65536/64 sameOwnedLoan=true originalDeadlineFuel=true terminalDrop0=true");
 }
}
