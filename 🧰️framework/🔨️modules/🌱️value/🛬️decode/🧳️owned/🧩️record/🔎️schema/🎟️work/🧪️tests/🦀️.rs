//! 🧭️ The shared matcher driver walks flattened original names through genuine Source, Work and construction authorities.
use semio_framework_value_native_owned::retained_clone::{RetainedCloneRef,RetainedCloneSource};
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use semio_framework_value_control_contract::*;
use semio_framework_value_native_owned::owned::{NativeFieldMatch,NativeRecordKey,NativeRecordMatcher,NativeRecordSchema,NativeSchemaFields,NativeSchemaKeySource,NativeSchemaLeaf,NativeSourceKeyValue,tree::{invalid,NativeScalarQuery,NativeSourceScalar,NativeTreeKind,NativeValueSource}};
use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,NativeReceivingEffect,NativeReceivingEffectPoll,OriginalEffectAuthority,OriginalReceivingPrimary};
use semio_framework_value_native_decoding::{OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value_native_receiver_protocol::NativeAdvanceSelection;
use semio_framework_value_retirement::OriginalRetirementFailure;
use semio_framework_value_native_schema_engine::source::{NativeSourceKeyReply,NativeSourceKeyReplyComplete};
#[path="../../../../../🏭️construction/🧪️tests/source-bank.rs"]mod source_bank;
use source_bank::Bank;

fn nested_schema(index:usize)->Option<NativeRecordSchema>{match index{0=>Some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name:"dup",ordinal:0})),1=>Some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name:"beta",ordinal:1})),2=>Some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name:"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaz",ordinal:2})),_=>None}}
fn root_schema(index:usize)->Option<NativeRecordSchema>{match index{0=>Some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name:"alpha",ordinal:0})),1=>Some(NativeRecordSchema::Flatten{fields:unsafe{NativeSchemaFields::native(nested_schema)},count:3}),2=>Some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name:"dup",ordinal:2})),3=>Some(NativeRecordSchema::Union{fields:unsafe{NativeSchemaFields::native(union_schema)},count:2}),_=>None}}
fn union_schema(index:usize)->Option<NativeRecordSchema>{if index<2{Some(NativeRecordSchema::Leaf(NativeSchemaLeaf{name:"union",ordinal:7}))}else{None}}

#[derive(semio_framework_value_native_owned::RetireOwned)]#[value(crate="semio_framework_value_native_owned")]struct KeySource{name:String,ordinal:Option<u16>,refusal:Option<String>}
impl NativeValueSource for KeySource{
 fn query_schema_key<'reply,'source>(&'source self,query:NativeSchemaKeySource,effect:&mut RetainedAllocationEffect<'_>,reply:NativeSourceKeyReply<'reply,'source>)->Result<NativeSourceKeyReplyComplete<'reply>,ValueError>{
  if effect.remaining_work_units()?<1{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema key query lacks original Work"))}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema key query exceeded original Work"))))}
  if let Some(message)=&self.refusal{return Ok(reply.complete(Err(ValueError::new(ValueRefusalKind::InvalidValue,message.clone()))))}
  Ok(reply.complete(match query{NativeSchemaKeySource::Record(0)|NativeSchemaKeySource::RecordName(0)=>Ok(match self.ordinal{Some(n)=>NativeSourceKeyValue::Ordinal(u32::from(n)),None=>NativeSourceKeyValue::Name(&self.name)}),_=>Err(invalid())}))
 }
 fn query_scalar<'reply>(&self,_:NativeScalarQuery,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(invalid())))}Ok(reply.complete(Err(invalid())))}
 fn recover_scalar<'reply>(&self,_:NativeScalarQuery,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,NativeSourceScalar,ValueError>)->Result<OriginalEffectReplyComplete<'reply,NativeSourceScalar,ValueError>,ValueError>{Ok(reply.pending())}
 fn as_source(&self)->&dyn NativeValueSource{self}
 fn tree_kind(&self)->Option<NativeTreeKind>{Some(NativeTreeKind::Object)}
 fn record_key(&self,index:usize)->Result<NativeRecordKey<'_>,ValueError>{if index!=0{return Err(invalid())}Ok(match self.ordinal{Some(n)=>NativeRecordKey::Ordinal(n),None=>NativeRecordKey::Name(&self.name)})}
}

type Matcher=NativeRecordMatcher<KeySource>;
struct Local;
impl OriginalEffectProducer for Local{
 type Output=RetainedCloneStep;type Failure=OriginalReceivingPrimary;type Context<'a,'b>=(&'a mut Matcher,RetainedCloneRef<'b,KeySource>)where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{c.0.advance_local(c.1,4096,effect,reply)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{Ok(reply.pending())}
}


struct Close{turn:OriginalSourceTurnIdentity}
impl OriginalEffectProducer for Close{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=(&'a mut Matcher,bool)where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(self.turn)}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{if c.1{let turn=c.0.begin_close_paid(effect);Ok(reply.complete(turn.result.map_err(Into::into)))}else{c.0.close_into(4096,effect,reply)}}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'b:'a{if c.1{Ok(reply.pending())}else{c.0.recover_close_into(4096,effect,reply)}}
}
fn close(bank:&mut Bank,native:&mut NativeDecodeControl<'_>,matcher:&mut Matcher,maximum:u64,trace:&mut Trace)->Result<(),ValueError>{
 for _ in 0..maximum{
  if matcher.terminal_is_empty(){return Ok(())}
  let begin=!matcher.close_started();
  let(demand,work)=if begin{(matcher.begin_close_demand(),matcher.begin_close_work_demand())}else{(matcher.close_demands(4096)?,matcher.close_work_demand(4096)?)};
  trace.steps.push(format!("close begin={begin} work={work} control={} birth={}",matcher.requires_original_control(),matcher.requires_original_birth()));
  let mut frame=NativeReceivingEffect::<OriginalRetirementFailure>::default();let mut authority=OriginalEffectAuthority::new(Close{turn:bank.turn.identity()});
  let mut outcome=Err(ValueError::invariant("matcher close stayed pending"));
  for _ in 0..8{let polled=bank.run(native,|control|control.receive_original_effect(demand,work,&mut frame,&mut authority,|_,effect,owner,action|owner.drive(&mut (&mut *matcher,begin),effect,action)));
   match polled{Ok(NativeReceivingEffectPoll::Complete(turn))=>{outcome=turn.result.map(|_|()).map_err(|error|ValueError::invariant(Box::leak(format!("matcher close refused {error:?}").into_boxed_str())));break},Ok(NativeReceivingEffectPoll::Pending{..})=>{trace.steps.push("close pending".into())},Err(error)=>{outcome=Err(error);break}}}
  if let Err(rest)=authority.into_producer(){std::mem::forget(rest)}if outcome.is_err(){std::mem::forget(frame)}outcome?;
 }
 Err(ValueError::invariant("matcher close exhausted its turns"))
}
#[derive(Debug,Default)]struct Trace{steps:Vec<String>}

/// 🥀️ Denies the acknowledgement grant at the Source stock and requires the key frame to stay retryable at the same selection.
fn starve_ack(bank:&mut Bank,native:&mut NativeDecodeControl<'_>,matcher:&mut Matcher,input:RetainedCloneRef<'_,KeySource>,fuel:u64,trace:&mut Trace)->Result<(),ValueError>{
 let grant=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:16};
 bank.wallet.lock().unwrap().deny=true;
 let polled=bank.run_with_turn(native,|turn,native|Ok(matcher.advance_nested(input,native,turn.witness(),4096,fuel,grant)));
 bank.wallet.lock().unwrap().deny=false;
 if let RetainedWorkPoll::Complete(outcome)=polled?{outcome.result.map_err(|_|ValueError::invariant("starved acknowledgement refused"))?;}
 assert_eq!(format!("{:?}",matcher.selection()?),"KeyAcknowledgement","starved acknowledgement must stay retryable");trace.steps.push("starved acknowledgement retryable".into());
 Ok(())
}
fn turn(bank:&mut Bank,native:&mut NativeDecodeControl<'_>,matcher:&mut Matcher,input:RetainedCloneRef<'_,KeySource>,fuel:u64,trace:&mut Trace)->Result<(),ValueError>{
 let demand=matcher.demands(input,4096,fuel)?;trace.steps.push(format!("{:?}",matcher.selection()?));if matcher.key_action_result_ready(){trace.steps.push("KeySettlement outcome resident".into());}
 match matcher.select_advance(input,4096,fuel)?{
  NativeAdvanceSelection::Local{work}=>{
   let mut frame=NativeReceivingEffect::<OriginalReceivingPrimary>::default();let mut authority=OriginalEffectAuthority::new(Local);
   let polled=bank.run(native,|control|control.receive_original_effect(demand,work,&mut frame,&mut authority,|_,effect,owner,action|owner.drive(&mut (&mut *matcher,input),effect,action)));
   let outcome=match polled{Ok(NativeReceivingEffectPoll::Complete(turn))=>turn.result.map(|_|()).map_err(|error|ValueError::invariant(Box::leak(format!("matcher local action refused {error:?}").into_boxed_str()))),Ok(NativeReceivingEffectPoll::Pending{..})=>Err(ValueError::invariant("matcher local action stayed pending")),Err(error)=>Err(error)};
   if let Err(rest)=authority.into_producer(){std::mem::forget(rest)}if outcome.is_err(){std::mem::forget(frame)}outcome?;
  },
  NativeAdvanceSelection::Nested=>{
   let grant=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:16};
   let polled=bank.run_with_turn(native,|turn,native|Ok(matcher.advance_nested(input,native,turn.witness(),4096,fuel,grant)))?;
   if let RetainedWorkPoll::Complete(outcome)=polled{outcome.result.map_err(|_|ValueError::invariant("matcher nested action refused"))?;}
  },
 }
 Ok(())
}


#[test]
fn actual_native_schema_matcher_driver_visits_flattened_original_names(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
 let maximum=law["stock"]["maximumTurns"].as_u64().unwrap();
 for case in law["cases"].as_array().unwrap(){
  let cuts:Vec<Option<usize>>=if case["kind"]=="cut"{(0..=case["turns"].as_u64().unwrap() as usize).map(Some).collect()}else{vec![None]};let mut visited=std::collections::BTreeSet::<String>::new();
  for cut in cuts{
  let mut bank=Bank::from_stock(&law["stock"],false);
  let refused=case["kind"]=="refused";let input=KeySource{name:case["key"].as_str().unwrap_or("").into(),ordinal:case["ordinal"].as_u64().map(|n|n as u16),refusal:case["message"].as_str().map(str::to_owned)};
  let mut source=RetainedCloneSource::<KeySource>::from_owner(input);let original=source.borrow();
  let mut matcher=Matcher::new(unsafe{NativeSchemaFields::native(root_schema)},4,NativeSchemaKeySource::Record(0));
  let mut observer=|_|true;let mut native=NativeDecodeControl::new(law["stock"]["capacityBytes"].as_u64().unwrap() as usize,&mut observer);
  let mut trace=Trace::default();
  let mut failure=None;
  let mut count=0usize;
  let mut starved=!case["starvedAck"].as_bool().unwrap_or(false);
  for _ in 0..maximum{if matcher.result().is_some()||cut==Some(count){break}if !starved&&format!("{:?}",matcher.selection().unwrap())=="KeyAcknowledgement"{starved=true;starve_ack(&mut bank,&mut native,&mut matcher,original,law["stock"]["fuel"].as_u64().unwrap(),&mut trace).unwrap_or_else(|error|panic!("starved acknowledgement {error:?}"));trace.steps.push("KeyAcknowledgement starved".into());}if let Err(error)=turn(&mut bank,&mut native,&mut matcher,original,law["stock"]["fuel"].as_u64().unwrap(),&mut trace){failure=Some(error);break}count+=1;}
  if cut.is_some(){
   assert!(failure.is_none()&&(matcher.result().is_none()||cut==Some(count)),"{failure:?} {:?}",trace.steps.iter().rev().take(6).collect::<Vec<_>>());
  }else if refused{
   let error=failure.expect("refused key must fail the driver");assert_eq!(trace.steps.last().map(String::as_str),Some("KeyError"));assert!(format!("{error:?}").contains(case["message"].as_str().unwrap()),"{error:?}");assert!(matcher.result().is_none());
  }else{
   assert!(failure.is_none(),"{failure:?} after {:?}",trace.steps.iter().rev().take(6).collect::<Vec<_>>());
   let actual=match matcher.result().unwrap_or_else(||panic!("matcher result absent after {:?}",trace.steps.iter().rev().take(8).collect::<Vec<_>>())){NativeFieldMatch::Missing=>serde_json::json!({"kind":"missing","field":null}),NativeFieldMatch::Field(field)=>serde_json::json!({"kind":"field","field":field}),NativeFieldMatch::Ambiguous=>serde_json::json!({"kind":"ambiguous","field":null})};
   assert_eq!(actual,serde_json::json!({"kind":case["kind"],"field":case["field"]}));
   if let Some(turns)=case["turns"].as_u64(){assert_eq!(count as u64,turns,"turns for {}",case["key"]);}
  }
  close(&mut bank,&mut native,&mut matcher,maximum,&mut trace).unwrap_or_else(|error|panic!("close {error:?} after {:?}",trace.steps.iter().rev().take(8).collect::<Vec<_>>()));
  visited.extend(trace.steps.iter().cloned());
  assert!(matcher.terminal_is_empty());drop(matcher);drop(source);
  }
  if case["starvedAck"].as_bool().unwrap_or(false){assert!(visited.contains("starved acknowledgement retryable"),"starved acknowledgement never reached in {visited:?}")}
  for forbidden in law["forbidden"][case["kind"].as_str().filter(|kind|matches!(*kind,"refused"|"cut")).unwrap_or("matched")].as_array().unwrap(){let forbidden=forbidden.as_str().unwrap();assert!(!visited.iter().any(|step|step.starts_with(forbidden)),"{forbidden} visited in {visited:?}");}
  for required in law["visits"][case["kind"].as_str().filter(|kind|matches!(*kind,"refused"|"cut")).unwrap_or("matched")].as_array().unwrap(){let required=required.as_str().unwrap();assert!(visited.contains(required),"{required} absent from {visited:?}");}
 }
}
