use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::{DslValue,retirement::RetireOwned,owned::NativeReceiver};
use crate::driver::{self as base,Trace};
use semio_framework_value_testing_admission::{EffectBank,Wallet};
pub(crate) const BODY:usize=4096;
pub(crate) fn fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../🔗️sealed-shared-receiver/🧪️tests/🧫️fixtures/📐️.json")).unwrap()}
pub(crate) fn bank()->EffectBank{EffectBank::new(Wallet::from_profile(&fixture()["stock"]))}
/// 🔁️ Receives at every prefix cut: each turn is denied and funded, a foreign Source is refused once bound, the output is checked and every owner closes to terminal-empty. Returns the turn count at which the output was first ready.
pub(crate) fn prefixes<T:NativeReceive<DslValue>+RetireOwned>(bound:u64,input:impl Fn()->DslValue,each:impl FnMut(u64,&<T as NativeReceive<DslValue>>::Receiver,&EffectBank,&Trace),check:impl Fn(&T))->u64 where <T as NativeReceive<DslValue>>::Receiver:NativeReceiver<DslValue,T>{sampled_prefixes::<T>(bound,1,BODY,input,each,check)}
/// 🔁️ Like [`prefixes`], but replays only the first 64 cuts and every `stride`-th cut afterwards, quoting at most `ceiling` copy bytes per turn except the final publication that moves the whole value: each cut replays its whole prefix, so exhaustive cuts of a wide record cost quadratic turns.
pub(crate) fn sampled_prefixes<T:NativeReceive<DslValue>+RetireOwned>(bound:u64,stride:u64,ceiling:usize,input:impl Fn()->DslValue,mut each:impl FnMut(u64,&<T as NativeReceive<DslValue>>::Receiver,&EffectBank,&Trace),check:impl Fn(&T))->u64 where <T as NativeReceive<DslValue>>::Receiver:NativeReceiver<DslValue,T>{
 let mut finished=0;
 for cut in (0..bound).filter(|cut|*cut<64||cut%stride==0){
  let mut bank=bank();let original=base::admit(&mut bank,input());let foreign=base::admit(&mut bank,input());let mut receiver=T::receiver();let mut trace=Trace::default();let mut turns=0;let mut history=std::collections::VecDeque::new();
  while !receiver.output_ready()&&turns<cut{let d=receiver.demands(original.borrow(),BODY,1<<20).unwrap();history.push_back(d.copy_bytes);if history.len()>12{history.pop_front();}assert!(d.copy_bytes<=ceiling.max(size_of::<T>()+2*size_of::<Option<T>>()+size_of::<bool>()+size_of::<usize>()+size_of::<u8>()),"turn {turns} quotes {d:?} after copy bytes {history:?}");base::none(base::receive_traced(&mut receiver,original.borrow(),&mut bank,BODY,Some(1),None,&mut trace)).unwrap();each(turns,&receiver,&bank,&trace);turns+=1;if turns>1&&!receiver.output_ready(){assert!(receiver.demands(foreign.borrow(),BODY,1<<20).is_err())}}
  if receiver.output_ready(){finished=turns;let value=base::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");check(&value);base::retire(value,&mut bank);}
  base::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);base::close_source(original,&mut bank);base::close_source(foreign,&mut bank);if finished>0&&cut>finished{break}
 }
 assert!(finished>0);finished
}
type Reply<'r,E>=OriginalEffectReply<'r,RetainedCloneStep,E>;
type Done<'r,E>=Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>;
/// 🧭️ Adapts the record matcher cursor to the receiver contract; the published byte encodes the match (0 missing, 255 ambiguous, field index plus one).
pub(crate) struct MatcherReceiver{pub(crate) inner:crate::native_decoding::owned::NativeRecordMatcher<DslValue>,taken:bool,closing:bool}
impl MatcherReceiver{pub(crate) fn new(inner:crate::native_decoding::owned::NativeRecordMatcher<DslValue>)->Self{Self{inner,taken:false,closing:false}}}
impl NativeReceiver<DslValue,u8> for MatcherReceiver{
 fn demands(&self,i:RetainedCloneRef<'_,DslValue>,b:usize,m:u64)->Result<RetirementDemand,ValueError>{self.inner.demands(i,b,m)}
 fn select_advance(&self,i:RetainedCloneRef<'_,DslValue>,b:usize,m:u64)->Result<crate::owned::NativeAdvanceSelection,ValueError>{self.inner.select_advance(i,b,m)}
 fn advance_nested(&mut self,i:RetainedCloneRef<'_,DslValue>,c:&mut NativeDecodeControl<'_>,w:OriginalSourceTurnWitness<'_>,b:usize,m:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,crate::native_decoding::NativeReceivingFailure>{self.inner.advance_nested(i,c,w,b,m,g)}
 fn advance_local<'r>(&mut self,i:RetainedCloneRef<'_,DslValue>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{self.inner.advance_local(i,b,e,r)}
 fn advance_pending(&self)->bool{self.inner.advance_pending()}
 fn output_ready(&self)->bool{self.inner.result().is_some()&&!self.taken&&!self.closing}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{depth:1,..Default::default()})}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn take<'r>(&mut self,o:&mut Option<u8>,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalReceivingPrimary>)->Done<'r,OriginalReceivingPrimary>{use crate::native_decoding::owned::NativeFieldMatch as M;if !retained_clone_grant_admits(e.grant(),self.take_demand()?)||e.maximum_work_units()==0{return Ok(r.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !e.performed(1)?{return Ok(r.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"matcher take exceeded its original work").into())))}*o=Some(match self.inner.result().ok_or_else(||ValueError::invariant("matcher has no result"))?{M::Missing=>0,M::Ambiguous=>255,M::Field(n)=>n as u8+1});self.taken=true;Ok(r.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
 fn is_closing(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(self.inner.begin_close_demand())}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(self.inner.begin_close_work_demand())}
 fn begin_close_into<'r>(&mut self,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{let turn=self.inner.begin_close_paid(e);if turn.result.is_ok(){self.closing=true}Ok(r.complete(turn.result.map_err(Into::into)))}
 fn recover_begin_close_into<'r>(&mut self,_:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{Ok(r.pending())}
 fn close_demands(&self,b:usize)->Result<RetirementDemand,ValueError>{self.inner.close_demands(b)}
 fn close_work_demand(&self,b:usize)->Result<u64,ValueError>{self.inner.close_work_demand(b)}
 fn close_step<'r>(&mut self,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.close_into(b,e,r)}
 fn recover_close_step<'r>(&mut self,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.recover_close_into(b,e,r)}
 fn close_original_step<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.close_original_into(o,b,e,r)}
 fn recover_close_original_step<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,b:usize,e:&mut RetainedAllocationEffect<'_>,r:Reply<'r,OriginalRetirementFailure>)->Done<'r,OriginalRetirementFailure>{self.inner.recover_close_original_into(o,b,e,r)}
 fn close_original_birth_step(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->crate::OriginalRetirementPoll{self.inner.close_original_birth_step(o,g)}
 fn requires_original_control(&self)->bool{self.inner.requires_original_control()}
 fn requires_original_birth(&self)->bool{self.inner.requires_original_birth()}
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()}
}
