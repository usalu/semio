//! 🧾️ Actual original Source refusals retain complete outcome headers and their selected payload.
use semio_framework_value_native_decoding::*;
use semio_framework_value_control_contract::*;
use semio_framework_value_retirement::retirement::{self,RetireOwned,RetirementCursor};
use std::{any::TypeId,mem::size_of,sync::Arc};
#[path="../../../🛬️decode/🏭️construction/🧪️tests/source-bank.rs"]mod source_bank;
use source_bank::Bank;
fn law()->serde_json::Value{serde_json::from_str(include_str!("owner.json")).unwrap()}
type Outcome=RetainedWorkOutcome<u64,u64>;
struct Context<'a>{turn:&'a OriginalSourceTurn,original:&'a mut Option<Outcome>,cursor:&'a mut Option<Box<dyn RetirementCursor>>}
struct Admission{source:Arc<OriginalSourceTurnIssuer>}
impl OriginalEffectProducer for Admission{
 type Output=RetainedCloneStep;type Failure=ValueError;type Context<'turn,'source>=Context<'turn>where 'source:'turn;
 fn validate_context<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{self.source.validate(c.turn).map_err(Into::into)}
 fn source_turn<'turn,'source>(&self,c:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(c.turn.identity())}
 fn receive<'reply,'turn,'source>(&mut self,c:&mut Self::Context<'turn,'source>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{let turn=retirement::admit_retirement_cursor(c.original,effect);Ok(reply.complete(turn.result.map(|admitted|match admitted{Some((cursor,progress))=>{*c.cursor=Some(cursor);RetainedCloneStep::Progress(progress)},None=>RetainedCloneStep::Progress(Default::default())})))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>where 'source:'turn{Ok(reply.pending())}
}
#[test]fn original_outcome_header_and_receipt_survive_actual_unfunded_cursor_birth(){
 let law=law();assert_eq!(TypeId::of::<<Outcome as RetireOwned>::RetirementCursor>(),TypeId::of::<<Result<u64,u64>as RetireOwned>::RetirementCursor>());
 for variant in law["variants"].as_array().unwrap(){let mut bank=Bank::from_stock(&law["stock"],false);let mut observer=|_|true;let mut native=NativeDecodeControl::new(1<<20,&mut observer);let mut frame=NativeReceivingEffect::default();let mut producer=OriginalEffectAuthority::new(Admission{source:bank.source.clone()});let value=law["value"].as_u64().unwrap();let expected=if variant=="Ok"{Ok(value)}else{Err(value)};let metadata=law["workMetadata"].as_str().unwrap().parse().unwrap();let mut original=Some(Outcome{result:expected,work:metadata});let mut cursor=None;let demand=retirement::retirement_cursor_birth_demand(original.as_ref().unwrap()).unwrap();assert_eq!(demand.copy_bytes,size_of::<Option<Outcome>>()+size_of::<<Result<u64,u64>as RetireOwned>::RetirementCursor>());assert!(demand.copy_bytes>size_of::<Option<Result<u64,u64>>>());
  for denial in law["denials"].as_array().unwrap(){let(d,work)=match denial.as_str().unwrap(){"work"=>(demand,0),"copy"=>(RetirementDemand{copy_bytes:demand.copy_bytes-1,..demand},4),"capacity"=>(RetirementDemand{capacity_bytes:demand.capacity_bytes-1,..demand},4),_=>unreachable!()};let before=bank.wallet.lock().unwrap().fuel;let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run_with_turn(&mut native,|turn,native|{let mut context=Context{turn,original:&mut original,cursor:&mut cursor};native.receive_original_effect(d,work,&mut frame,&mut producer,|_,effect,producer,action|producer.drive(&mut context,effect,action))}).unwrap());let NativeReceivingEffectPoll::Complete(turn)=turn else{panic!("actual finite birth refusal must settle")};assert_eq!(heap,(0,0));assert_eq!(turn.work,law["deniedWork"].as_u64().unwrap()as u128);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(original.as_ref().unwrap().result,expected);assert_eq!(original.as_ref().unwrap().work,metadata);assert!(cursor.is_none());assert_eq!(bank.wallet.lock().unwrap().fuel,before);}
  assert!(producer.into_producer().is_ok());assert_eq!(native.actual_work(),0);
 }
}
struct Readiness(bool);
impl RetireOwned for Readiness{type RetirementCursor=retirement::Leaf<bool>;fn retirement_ready(&self)->bool{self.0}fn retirement_seed(self)->Self::RetirementCursor{retirement::leaf(self.0)}fn controlled_retirement_supported()->bool{true}}
struct Unsupported;
impl RetireOwned for Unsupported{type RetirementCursor=retirement::Leaf<()>;fn retirement_seed(self)->Self::RetirementCursor{retirement::leaf(())}fn controlled_retirement_supported()->bool{false}}
#[test]fn outcome_retirement_propagates_selected_readiness_and_both_type_capabilities(){
 let law=law();let rejected=RetainedWorkOutcome::<u64,Readiness>{result:Err(Readiness(false)),work:u128::MAX};assert_eq!(rejected.retirement_ready(),law["ready"].as_bool().unwrap());assert!(retirement::retirement_cursor_birth_demand(&rejected).is_err());let accepted=RetainedWorkOutcome::<u64,Readiness>{result:Ok(73),work:u128::MAX};assert!(accepted.retirement_ready());assert!(retirement::retirement_cursor_birth_demand(&accepted).is_ok());assert_eq!(RetainedWorkOutcome::<u64,Unsupported>::controlled_retirement_supported(),law["unsupported"].as_bool().unwrap());assert_eq!(RetainedWorkOutcome::<Unsupported,u64>::controlled_retirement_supported(),law["unsupported"].as_bool().unwrap());
}
