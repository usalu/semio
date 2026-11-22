//! 🧪️ Actual Drawing opens from the original framed member pages under a fixture Source and retires every partial native owner.
use super::*;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_os_store_owned_schema_decode::{OwnedSchemaDecodePages,OwnedSchemaDecodeCredits,OwnedSchemaDecodePage};
use semio_framework_job::{OriginalJobDecodeOwner,OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_value::{RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use pack::snapshot::ArtifactPack;
fn policy()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<20,maximum_depth:4096}}
/// 🏦️ Admits the fixture Source that owns the request's operation, with a pinned copy stock and business fuel, and keeps its Source dispatch across every later turn as the production owned caller does across Pending.
fn caller(copy:usize,fuel:u64)->OriginalJobFixtureCaller{let mut caller=OriginalJobFixtureCaller::admit(OperationId(91),Generation(7),RetainedCloneGrant{maximum_items:1<<40,maximum_copy_bytes:copy,maximum_capacity_bytes:1<<40,maximum_release_bytes:1<<40,maximum_depth:256},||Some(1),u64::MAX,fuel);caller.retain_dispatch().unwrap();caller}
fn request(bytes:&[u8])->MemberOpenRequest{
 let mut frame=Vec::new();let mut n=bytes.len();loop{let b=(n&127)as u8;n>>=7;frame.push(b|if n==0{0}else{128});if n==0{break}}frame.extend_from_slice(bytes);frame.push(0);
 let mut pages=OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits{maximum_pages:frame.len().div_ceil(semio_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES),maximum_bytes:frame.len()}).unwrap();for page in frame.chunks(semio_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES){pages.admit_page(OwnedSchemaDecodePage::try_from_slice(page).unwrap()).unwrap()}pages.seal().unwrap();
 let expected=semio_framework_artifact_reference::ArtifactRef{artifact_id:"drawing-original".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.draw.drawing".into(),standard:"1".into(),subset:"*".into()}};
 MemberOpenRequest::new(OperationId(91),Generation(7),1000,expected,None,pages,semio_os_spr::ActorId("author".into())).admit(1).unwrap_or_else(|_|panic!("original Drawing request"))
}
/// ➖️ The physical receipt one turn added to the Source's cumulative spend.
fn since(now:RetainedCloneProgress,start:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:now.copied_items-start.copied_items,copied_bytes:now.copied_bytes-start.copied_bytes,retained_capacity_bytes:now.retained_capacity_bytes-start.retained_capacity_bytes,released_bytes:now.released_bytes-start.released_bytes}}
/// ⌛️ Runs one synchronous retirement poll in an original effect of the fixture Source and observes only its own heap; `None` when the Source cannot fund the effect.
fn polled<T,E>(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->Option<(RetainedWorkOutcome<T,E>,(usize,usize))>{match caller.with_effect(grant,|effect|observe_retirement_allocations(||run(effect))){Ok((RetainedWorkPoll::Complete(turn),heap))=>Some((turn,heap)),Ok((RetainedWorkPoll::Pending{..},_))=>panic!("synchronous retirement stayed pending"),Err(_)=>None}}
fn close<T:RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T){let mut c=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("typed Drawing law close"));for _ in 0..100000{if c.terminal_is_empty(){return}polled(caller,policy(),|effect|c.step(policy().maximum_copy_bytes,effect)).expect("funded typed Drawing close").0.result.unwrap_or_else(|_|panic!("typed Drawing close refused"));}panic!("typed Drawing closure stalled")}
/// 🧹️ Runs one member open close poll; an unfunded effect reports no progress and no heap.
fn close_open(caller:&mut OriginalJobFixtureCaller,open:&mut OriginalRecordMemberSnapshotOpen<DrawingSnapshot>,grant:RetainedCloneGrant)->(RetainedCloneProgress,(usize,usize)){polled(caller,grant,|effect|open.close_step(grant.maximum_copy_bytes,effect)).map_or((Default::default(),(0,0)),|(turn,heap)|(turn.result.unwrap_or_else(|_|panic!("original member open close refused")).progress(),heap))}
/// ♻️ Returns the decoder to its Source and then retires the fixture Source itself.
fn retire(mut caller:OriginalJobFixtureCaller,mut decoder:OriginalJobDecodeOwner){for _ in 0..100000{if decoder.terminal_is_empty(){break}caller.turn(policy(),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})).unwrap();}assert!(decoder.terminal_is_empty());caller.retire(policy());}
#[test]
fn genuine_original_drawing_member_receives_closes_and_funds_exact_handoff(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️.json")).unwrap();let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(law["cases"].as_array().unwrap().len(),5);
 let expected:DrawingSnapshot=serde_json::from_value(corpus["base"].clone()).unwrap();let bytes=expected.encode_pack();let grant=policy();let mut caller=caller(usize::MAX>>1,1<<40);
 for cut in [0,1,12,50,200,100000]{
  let mut open=OriginalRecordMemberSnapshotOpen::<DrawingSnapshot>::begin(request(&bytes)).unwrap_or_else(|_|panic!("actual original Drawing open"));let mut native=caller.admitted_decoder();let mut ready=false;let mut table_admitted=false;
  for _ in 0..cut{
   let step=caller.turn_with_decoder(grant,&mut native,|cx|{
    let d=open.opening_demand(cx,grant.maximum_copy_bytes).unwrap();for axis in 0..5{let mut denied=grant;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=0,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=0,3 if d.release_bytes>0=>denied.maximum_release_bytes=0,4 if d.depth>0=>denied.maximum_depth=0,_=>continue}let before=open.phase;let(step,heap)=observe_retirement_allocations(||open.step(cx,denied).unwrap());assert!(matches!(step,MemberSnapshotOpenStep::Pending(_)));assert_eq!(open.last_receipt(),Default::default());if heap!=(0,0){assert!(!table_admitted,"a denied step allocated after the Source's one-time return table");table_admitted=true}assert!(open.phase==before);assert_eq!(open.opening_demand(cx,grant.maximum_copy_bytes).unwrap(),d);}let start=cx.retained_progress();let(step,heap)=observe_retirement_allocations(||open.step(cx,grant).unwrap());let p=open.last_receipt();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(since(cx.retained_progress(),start),p);assert!(p.fits(grant));
    Ok(step)
   }).unwrap();
   if matches!(step,MemberSnapshotOpenStep::Ready){ready=true;break}
  }
  if cut==100000{assert!(ready)}
  if ready{assert!(open.receiving.as_ref().unwrap().terminal_is_empty());assert!(open.parser.as_ref().unwrap().terminal_is_empty());let denied=RetainedCloneGrant{maximum_copy_bytes:0,..grant};let(result,heap)=caller.turn(denied,|cx|Ok(observe_retirement_allocations(||open.take_ready(cx).unwrap()))).unwrap();assert!(result.is_none());assert_eq!(heap,(0,0));assert!(open.snapshot.is_some());
   let(result,heap,spent)=caller.turn(grant,|cx|{let start=cx.retained_progress();let(result,heap)=observe_retirement_allocations(||open.take_ready(cx).unwrap());Ok((result,heap,since(cx.retained_progress(),start)))}).unwrap();assert_eq!(heap,(0,0));let(snapshot,original)=result.unwrap();assert_eq!(snapshot,expected);assert!(open.terminal_is_empty());assert!(spent.copied_bytes>0);close(&mut caller,snapshot);let mut original=original;while !original.terminal_is_empty(){original.close_step(grant).unwrap();}
  }
  for _ in 0..100000{if open.terminal_is_empty(){break}let d=open.close_demand(grant.maximum_copy_bytes).unwrap();let denied=RetainedCloneGrant{maximum_items:0,..grant};let(p,heap)=close_open(&mut caller,&mut open,denied);assert_eq!(p,Default::default());assert_eq!(heap,(0,0));assert_eq!(open.close_demand(grant.maximum_copy_bytes).unwrap(),d);let(p,heap)=close_open(&mut caller,&mut open,grant);assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));}
  assert!(open.terminal_is_empty());let(_,heap)=observe_retirement_allocations(||drop(open));assert_eq!(heap,(0,0));for _ in 0..100000{if native.terminal_is_empty(){break}caller.turn(grant,|cx|cx.with_original_retirement(|cx|{native.close_on_source(cx)?;Ok(())})).unwrap();}assert!(native.terminal_is_empty());
 }
 close(&mut caller,expected);caller.retire(policy());
}
#[test]
fn foreign_declared_snapshot_envelope_retains_original_request_and_error_receipt(){
 let grant=policy();let mut caller=caller(usize::MAX>>1,1<<40);let mut open=OriginalRecordMemberSnapshotOpen::<DrawingSnapshot>::begin(request(b"foreign-envelope")).unwrap_or_else(|_|panic!("original foreign request"));let mut refused=false;
 for _ in 0..32{let failed=caller.turn(grant,|cx|{let start=cx.retained_progress();Ok(match open.step(cx,grant){Ok(_)=>None,Err(failure)=>Some((since(cx.retained_progress(),start),failure.retained_progress()))})}).unwrap();if let Some((spent,receipt))=failed{assert_eq!(spent,receipt);assert!(open.request.is_some());assert!(open.parser.is_none());refused=true;break}}
 assert!(refused);for _ in 0..100000{if open.terminal_is_empty(){break}close_open(&mut caller,&mut open,grant);}assert!(open.terminal_is_empty());caller.retire(policy());
}
fn budget()->serde_json::Value{serde_json::from_str(include_str!("🪙️budget.json")).unwrap()}
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../../../../../../../../../../✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️.json")).unwrap()}
/// 🧮️ Opens one original Drawing on turns drawn from a pinned fixture Source stock and returns its turns to Ready, the copy stock left and the funded handoff.
fn opened(bytes:&[u8],turn:RetainedCloneGrant,fuel:u64,stock:usize,maximum:u64)->(u64,usize,DrawingSnapshot){
 let mut caller=caller(stock,fuel);let mut native=caller.admitted_decoder();let mut open=OriginalRecordMemberSnapshotOpen::<DrawingSnapshot>::begin(request(bytes)).unwrap_or_else(|_|panic!("original Drawing open"));let mut turns=0u64;
 loop{
  assert!(turns<maximum,"original Drawing open needed more than {maximum} turns");turns+=1;let grant=caller.within_stock(turn);
  let step=caller.turn_with_decoder(grant,&mut native,|cx|open.step(cx,grant)).unwrap();
  if matches!(step,MemberSnapshotOpenStep::Ready){break}
 }
 let left=caller.remaining_grant().maximum_copy_bytes;
 let(snapshot,mut original)=caller.turn(policy(),|cx|open.take_ready(cx)).unwrap().expect("funded original Drawing handoff");
 while !original.terminal_is_empty(){original.close_step(policy()).unwrap();}for _ in 0..100000{if open.terminal_is_empty(){break}close_open(&mut caller,&mut open,policy());}assert!(open.terminal_is_empty());drop(open);retire(caller,native);
 (turns,left,snapshot)
}
#[test]
fn a_large_original_drawing_settles_in_turns_proportional_to_its_size_over_the_grant(){
 let law=budget();let law=&law["batching"];let n=|key:&str|law[key].as_u64().unwrap();let corpus=corpus();
 let pattern=law["titlePattern"].as_str().unwrap();let title:String=pattern.chars().cycle().take(n("titleBytes")as usize).collect();assert_eq!(title.len()as u64,n("titleBytes"));
 let mut document=corpus["base"].clone();document["title"]=serde_json::Value::String(title);
 let base:DrawingSnapshot=serde_json::from_value(corpus["base"].clone()).unwrap();let large:DrawingSnapshot=serde_json::from_value(document).unwrap();
 let turn=RetainedCloneGrant{maximum_copy_bytes:n("copyBytes")as usize,..policy()};let fuel=n("fuel");
 let(base_turns,_,opened_base)=opened(&base.encode_pack(),turn,fuel,usize::MAX>>1,n("maximumTurns"));assert_eq!(opened_base,base);
 let(large_turns,_,opened_large)=opened(&large.encode_pack(),turn,fuel,usize::MAX>>1,n("maximumTurns"));assert_eq!(opened_large,large);
 let share=(n("copyBytes")/2).min(n("scopeWorkCeiling").min(fuel/2)/n("actionsPerByteCeiling"));let bound=n("turnsPerShare")*n("titleBytes").div_ceil(share);
 assert!(bound<n("titleBytes"),"the batching bound must stay far below one turn per decoded byte");
 assert!(large_turns<=base_turns+bound,"{} title bytes took {large_turns} turns against {base_turns} base turns and a bound of {bound}",n("titleBytes"));
 let mut caller=caller(usize::MAX>>1,1<<40);close(&mut caller,opened_base);close(&mut caller,opened_large);close(&mut caller,base);close(&mut caller,large);caller.retire(policy());
}
#[test]
fn a_receiver_between_one_and_two_half_shares_reaches_ready_without_draining_its_pinned_stock(){
 let law=budget();let law=&law["floor"];let n=|key:&str|law[key].as_u64().unwrap();let ratio=&law["turnCopyOverReceiver"];
 let receiver=size_of::<<DrawingSnapshot as NativeReceive<RecordValue>>::Receiver>();let copy=receiver*ratio["numerator"].as_u64().unwrap()as usize/ratio["denominator"].as_u64().unwrap()as usize;assert!(receiver<copy);
 let expected:DrawingSnapshot=serde_json::from_value(corpus()["base"].clone()).unwrap();let stock=n("stockCopyBytes")as usize;
 let(_,left,snapshot)=opened(&expected.encode_pack(),RetainedCloneGrant{maximum_copy_bytes:copy,..policy()},n("fuel"),stock,n("maximumTurns"));
 assert_eq!(snapshot,expected);assert!(left>=copy,"the open drained its pinned stock to {left} of {stock}");
 let mut caller=caller(usize::MAX>>1,1<<40);close(&mut caller,snapshot);close(&mut caller,expected);caller.retire(policy());
}
