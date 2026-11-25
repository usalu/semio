//! ⏸️ A host abort that meets a press, gesture or typing slot lent to a retained read is kept by its ledger and ends the
//! slot once the read returns (law `🔣️.json`, schema `📐️.json`).
use super::*;
use serde_json::{json,Value};

fn law()->Value{serde_json::from_str(include_str!("🔣️.json")).unwrap()}
fn reason(value:&Value)->ToolAbortReason{ToolAbortReason::parse(value.as_str().unwrap()).unwrap()}
fn clock(ms:u64)->HybridLogicalTimestamp{HybridLogicalTimestamp{actor:0,physical_ms:ms,logical:1}}
fn ended<M>(window:&str,step:ToolStep<M>)->Option<Value>{match step{ToolStep::Idle=>None,ToolStep::Aborted(_,reason)=>Some(json!({"window":window,"reason":reason.as_str()})),ToolStep::Committed(..)=>Some(json!({"window":window,"committed":true})),other=>panic!("a deferred abort reported {}",other.kind())}}
fn written<T,E>(result:Result<T,E>,expected:&Value){assert_eq!(result.is_ok(),expected=="accepted","write {expected}");}

/// 🎛️ One fixture action on a scrub ledger.
fn scrub_act(ledger:&mut ScrubLedger<u64>,window:&str,action:&Value)->Vec<Value>{
 if let Some(all)=action.get("abortAll"){return ledger.abort_all(reason(all)).into_iter().filter_map(|step|ended(window,step)).collect()}
 if let Some(abort)=action.get("abort"){return ended(window,ledger.abort(window,action["press"].as_str(),reason(abort))).into_iter().collect()}
 if let Some(write)=action.get("write"){let mut input=Some(ScrubInput::Tick{gesture:"press".into(),leaves:vec![1]});written(ledger.send(window,"test#press",&ActorId("native-source".into()),"base",&mut input,clock(20)),write);return Vec::new()}
 assert_eq!(action["settleDeferred"],json!(true),"a scrub action");ledger.settle_deferred_aborts().into_iter().filter_map(|step|ended(window,step)).collect()
}

/// 🎛️ One fixture action on a typing ledger.
fn typing_act(ledger:&mut TypingLedger<u64>,window:&str,action:&Value)->Vec<Value>{
 if let Some(all)=action.get("abortAll"){return ledger.abort_all(reason(all)).into_iter().filter_map(|(window,step)|ended(&window,step)).collect()}
 if let Some(abort)=action.get("abort"){return ended(window,ledger.abort(window,reason(abort))).into_iter().collect()}
 if let Some(ms)=action.get("lapse"){return ledger.lapse(clock(ms.as_u64().unwrap())).into_iter().filter_map(|(window,step)|ended(&window,step.unwrap())).collect()}
 if let Some(write)=action.get("write"){written(ledger.commit(window,TypingCommit::Enter,clock(20)).map(|step|assert!(matches!(step,ToolStep::Idle),"a deferred run publishes nothing")),write);return Vec::new()}
 assert_eq!(action["settleDeferred"],json!(true),"a typing action");ledger.settle_deferred_aborts().into_iter().filter_map(|(window,step)|ended(&window,step)).collect()
}

/// 🎛️ One fixture action on a gesture ledger.
fn gesture_act(ledger:&mut GestureLedger<u64>,window:&str,action:&Value)->Vec<Value>{
 if let Some(event)=action.get("host"){return ended(window,ledger.host_event(window,GestureHostEvent::parse(event.as_str().unwrap()).unwrap())).into_iter().collect()}
 if let Some(event)=action.get("hostAll"){return ledger.host_event_all(GestureHostEvent::parse(event.as_str().unwrap()).unwrap()).into_iter().filter_map(|(window,step)|ended(&window,step)).collect()}
 if let Some(keep)=action.get("retain"){let keep:Vec<&str>=keep.as_array().unwrap().iter().map(|window|window.as_str().unwrap()).collect();return ledger.retain_windows(|window|keep.contains(&window)).into_iter().filter_map(|(window,step)|ended(&window,step)).collect()}
 if let Some(write)=action.get("write"){written(ledger.settle(window,None),write);return Vec::new()}
 assert_eq!(action["settleDeferred"],json!(true),"a gesture action");ledger.settle_deferred_aborts().into_iter().filter_map(|(window,step)|ended(&window,step)).collect()
}

/// 🔁️ Holds the window's retained read across the case's `held` actions, returns it, then runs its `returned` actions.
macro_rules! deferred_case{($ledger:expr,$ledger_ty:ty,$state:ty,$case:expr,$window:expr,$act:expr,$closed:expr)=>{{
 let(case,window)=($case,$window);let mut ledger=$ledger;let mut bank=effect_harness::bank();let revision=ledger.source_revision();let key=ledger.retained_read_key(window).unwrap();
 let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<$state>>=Arc::new(OwnedValueRetirementFactory::<$state>::default());
 for step in 0..65536{if ledger.retained_read(window,key).is_ok(){break}let g=funded(ledger.retained_read_demands(window,key,4096).unwrap());effect_harness::paid(&mut bank,g,|e,r,recover|if recover{ledger.recover_advance_retained_read_into(window,key,&factory,e,r)}else{ledger.advance_retained_read_into(window,key,&factory,e,r)});assert!(step<65535);}
 assert!(ledger.retained_read_pending(window));
 let outcome=|ledger:&mut $ledger_ty,actions:&Value|->Value{let aborted:Vec<Value>=actions.as_array().unwrap().iter().flat_map(|action|$act(&mut *ledger,window,action)).collect();let ledger:&$ledger_ty=ledger;json!({"aborted":aborted,"open":ledger.open(window).is_some(),"provisional":ledger.provisional().next().is_some(),"revisionMoved":ledger.source_revision()!=revision,"closed":$closed(ledger,window)})};
 assert_eq!(outcome(&mut ledger,&case["held"]),case["heldExpected"],"{} while the read is held",case["name"]);
 assert!(ledger.retained_read_pending(window),"{}: the read is still held",case["name"]);
 let mut returned=None;for step in 0..65536{if !ledger.retained_read_pending(window){break}let g=funded(ledger.retained_read_return_demands(window,key,4096).unwrap());effect_harness::paid(&mut bank,g,|e,r,recover|if recover{ledger.recover_return_retained_read_into(window,key,&mut returned,e,r)}else{ledger.return_retained_read_into(window,key,&mut returned,e,r)});assert!(step<65535);}
 assert!(Arc::ptr_eq(&factory,&returned.unwrap()));
 assert_eq!(outcome(&mut ledger,&case["returned"]),case["returnedExpected"],"{} once the read returned",case["name"]);
}}}

/// ⏸️ LAW: a blur, a freeze, a retired window or a moved base that meets a slot lent to a retained read ends it for every
/// reader at once (its press closes now, the first fact wins), leaves the source revision the read pinned untouched, and
/// ends it with zero trace once the read returns — by the ledger's settlement or by the next writer of the slot, which
/// then sees it already ended; a lapse waits on the read and fires after it.
#[test]
fn every_host_abort_during_a_retained_read_ends_its_slot_once_the_read_returns(){
 let law=law();assert_eq!(law["contract"],"semio.tool.deferred-abort.v1");let window=law["window"].as_str().unwrap();assert_eq!(window,"source");
 let cases=law["cases"].as_array().unwrap();
 for role in ["scrub","typing","gesture"]{assert!(cases.iter().any(|case|case["role"]==role),"the law covers {role}");}
 for case in cases{match case["role"].as_str().unwrap(){
  "scrub"=>deferred_case!(scrub(),ScrubLedger<u64>,ScrubState<u64>,case,window,scrub_act,|ledger:&ScrubLedger<u64>,window:&str|json!(ledger.closed(window))),
  "typing"=>deferred_case!(typing(),TypingLedger<u64>,TypingState<u64>,case,window,typing_act,|_:&TypingLedger<u64>,_:&str|Value::Null),
  "gesture"=>deferred_case!(gesture(),GestureLedger<u64>,GestureState<u64>,case,window,gesture_act,|ledger:&GestureLedger<u64>,window:&str|json!(ledger.closed(window))),
  role=>panic!("unknown role {role}"),
 }}
}
