//! 📏️ Law: every wire decodes to the pinned value, refusal and work units within the pinned control-turn ceiling; a batching may lower the turns and must keep everything else.
use super::*;

fn bytes(row:&serde_json::Value)->Vec<u8>{match row["hex"].as_str(){Some(hex)=>(0..hex.len()).step_by(2).map(|at|u8::from_str_radix(&hex[at..at+2],16).unwrap()).collect(),None=>row["text"].as_str().unwrap().as_bytes().to_vec()}}

fn cause(failure:&NativeReceivingFailure)->String{
 let kind=|error:Option<&ValueError>|error.map(|error|format!("{:?}",error.kind));
 match failure{
  NativeReceivingFailure::Primary(primary)=>format!("primary:{:?}",kind(primary.primary.as_ref().or(primary.quote_refusal.as_ref()).or(primary.allocation_refusal.as_ref()))),
  NativeReceivingFailure::ValueEffect(error)=>format!("value-effect:{:?}",kind(error.child.as_ref().or(error.producer_refusal.as_ref()).or(error.work_refusal.as_ref()))),
  NativeReceivingFailure::RetirementEffect(_)=>"retirement-effect".into(),
  NativeReceivingFailure::ReceivingEffect(_)=>"receiving-effect".into(),
 }
}

struct Run{accepted:bool,refusal:Option<String>,front:u64,work:u64,cleanup:u64,turns:u64,value:Option<Projection>}

fn run(wire:&[u8],corpus:&serde_json::Value)->Run{
 let stock=&corpus["stock"];let g=RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize};
 let limits=JsonReadLimits{maximum_bytes:corpus["limits"]["maximumBytes"].as_u64().unwrap(),maximum_allocation_bytes:g.maximum_capacity_bytes,maximum_depth:corpus["limits"]["maximumDepth"].as_u64().unwrap()as usize,maximum_items:corpus["limits"]["maximumItems"].as_u64().unwrap()};
 let fuel=stock["fuel"].as_u64().unwrap();let mut caller=OriginalJobFixtureCaller::admit(OperationId(944),Generation(1),g,clock,100_000_000,fuel);caller.retain_dispatch().unwrap();
 let mut state=OriginalJsonOutputSource::<Projection>::new(OriginalNativeDecodePolicy{maximum_owned_bytes:g.maximum_capacity_bytes},limits);
 let(mut captured,mut failed,mut closes,mut turns,mut refusal,mut front)=(None,false,Vec::new(),0u64,None,0u64);
 for at in 0..100000{
  turns+=1;let before=caller.fuel_remaining();let typed=matches!(state.receiver().map(|receiver|receiver.phase()),Some("receive"|"take"|"closeReceiver"|"closeParser"|"ready"));let outcome=funded(&mut caller,g,|cx|if state.ready(){state.take_step(&mut captured,cx)}else{state.receive_step(wire,cx)});
  if !typed{front+=before-caller.fuel_remaining()}
  if let Some(outcome)=outcome{assert!(outcome.receiving_refusal.is_none());failed=outcome.value.is_err();
   if let Err(error)=&outcome.value{refusal=Some(match error{OriginalJsonOutputFailure::Refused(error)=>format!("refused:{:?}",error.kind),OriginalJsonOutputFailure::Stored(_)=>match state.receiver().and_then(|receiver|receiver.syntax_error()){Some(syntax)=>format!("syntax:{syntax:?}"),None=>format!("typed:{}",state.failure().map_or("none".to_string(),cause))}})}
   closes.push(ControlledRetirement::retain(outcome));drain(&mut caller,g,&mut state,&mut closes);if failed{break}}
  if captured.is_some(){break}assert!(at<99999);
 }
 let work=fuel-caller.fuel_remaining();
 caller.cancel().cancel_now();drain(&mut caller,g,&mut state,&mut closes);
 let value=captured.take();
 for at in 0..100000{if state.terminal_is_empty(){break}drain(&mut caller,g,&mut state,&mut closes);match funded(&mut caller,g,|cx|state.close_step(cx)){OriginalJsonOutputClose::Producer(Some(outcome))=>{assert!(outcome.receiving_refusal.is_none());outcome.value.unwrap();},_=>{}}assert!(at<99999)}
 assert!(closes.is_empty());assert!(state.terminal_is_empty());let cleanup=fuel-caller.fuel_remaining()-work;caller.retire(g);
 Run{accepted:!failed&&value.is_some(),refusal,front,work,cleanup,turns,value}
}

#[test]
fn decoded_value_refusal_and_total_work_stay_identical_while_control_turns_shrink(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let name=row["name"].as_str().unwrap();let wire=bytes(row);let oracle=serde_json::from_slice::<Projection>(&wire);let ran=run(&wire,&corpus);
  assert_eq!(ran.accepted,row["accepted"].as_bool().unwrap(),"{name}");
  assert_eq!(oracle.is_ok(),ran.accepted,"{name}: serde_json decides the same acceptance");
  if let(Some(value),Ok(expected))=(ran.value.as_ref(),oracle.as_ref()){assert_eq!(value,expected,"{name}")}
  assert_eq!(ran.refusal.as_deref(),row["refusal"].as_str(),"{name}");
  assert_eq!(ran.front,row["frontWork"].as_u64().unwrap(),"{name}: parse and source work units");
  assert_eq!(ran.work,row["work"].as_u64().unwrap(),"{name}: decode work units");assert_eq!(ran.cleanup,row["cleanupWork"].as_u64().unwrap(),"{name}: cleanup work units");
  assert!(ran.turns<=row["maxTurns"].as_u64().unwrap(),"{name}: {} control turns exceed the pinned ceiling",ran.turns);
 }
}
