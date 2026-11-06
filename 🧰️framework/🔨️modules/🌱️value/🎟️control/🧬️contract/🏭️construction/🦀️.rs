//! 🏭️ Original construction grants and receipts preserve their producer-recorded replies.
use crate::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress};
use std::sync::atomic::{AtomicU64,Ordering};
static NEXT:AtomicU64=AtomicU64::new(1);
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original construction action or reply changed")}
/// 🪪️ Names the actual construction issuer without exposing its representation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalConstructionIssuerIdentity(u64);
/// 🪪️ Names one construction issuer and one of its actual actions.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalConstructionActionIdentity{issuer:OriginalConstructionIssuerIdentity,action:u64}
/// 🚦️ Retains the actual producer boundary without manufacturing its answer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalConstructionPhase{Fresh,GrantStarted,Granted,GrantRefused,AcceptStarted,Settled}
/// 🧳️ Owns the original request and both producer reply slots across interruptions.
pub struct OriginalConstructionAction{identity:OriginalConstructionActionIdentity,source_turn:Option<crate::OriginalSourceTurnIdentity>,callback_invoked:bool,demand:RetirementDemand,phase:OriginalConstructionPhase,grant:Option<RetainedCloneGrant>,progress:Option<RetainedCloneProgress>,actual:Option<usize>,grant_reply:Option<Result<RetainedCloneGrant,ValueError>>,accept_reply:Option<Result<(),ValueError>>}
impl OriginalConstructionAction{pub fn identity(&self)->OriginalConstructionActionIdentity{self.identity}pub fn phase(&self)->OriginalConstructionPhase{self.phase}pub fn demand(&self)->RetirementDemand{self.demand}pub fn grant(&self)->Option<RetainedCloneGrant>{self.grant}pub fn progress(&self)->Option<RetainedCloneProgress>{self.progress}pub fn actual(&self)->Option<usize>{self.actual}}
impl Drop for OriginalConstructionAction{fn drop(&mut self){assert!(std::thread::panicking()||matches!(self.phase,OriginalConstructionPhase::GrantRefused|OriginalConstructionPhase::Settled),"original construction action remains unsettled")}}
/// 🎟️ Borrows the original grant reply until its producer records the actual verdict.
pub struct OriginalConstructionGrant<'a>{identity:OriginalConstructionActionIdentity,demand:RetirementDemand,entered:&'a mut bool,reply:&'a mut Option<Result<RetainedCloneGrant,ValueError>>}
/// 🧾️ Proves that the actual grant producer deposited its reply.
pub struct OriginalConstructionGrantComplete<'a>{_reply:&'a mut Option<Result<RetainedCloneGrant,ValueError>>}
impl<'a> OriginalConstructionGrant<'a>{pub fn identity(&self)->OriginalConstructionActionIdentity{self.identity}pub fn demand(&self)->RetirementDemand{self.demand}pub fn complete(self,result:Result<RetainedCloneGrant,ValueError>)->OriginalConstructionGrantComplete<'a>{assert!(self.reply.is_none(),"construction grant already answered");*self.reply=Some(result);OriginalConstructionGrantComplete{_reply:self.reply}}}
/// 📨️ Borrows the actual grant, progress, and original receipt reply.
pub struct OriginalConstructionAccept<'a>{identity:OriginalConstructionActionIdentity,grant:RetainedCloneGrant,progress:RetainedCloneProgress,actual:Option<usize>,entered:&'a mut bool,reply:&'a mut Option<Result<(),ValueError>>}
/// 🧾️ Proves that the actual receipt producer deposited its reply.
pub struct OriginalConstructionAcceptComplete<'a>{_reply:&'a mut Option<Result<(),ValueError>>}
impl<'a> OriginalConstructionAccept<'a>{pub fn identity(&self)->OriginalConstructionActionIdentity{self.identity}pub fn grant(&self)->RetainedCloneGrant{self.grant}pub fn progress(&self)->RetainedCloneProgress{self.progress}pub fn actual(&self)->Option<usize>{self.actual}pub fn complete(self,result:Result<(),ValueError>)->OriginalConstructionAcceptComplete<'a>{assert!(self.reply.is_none(),"construction receipt already answered");*self.reply=Some(result);OriginalConstructionAcceptComplete{_reply:self.reply}}}
/// 🫴️ The defining producer deposits each reply before returning or unwinding.
pub trait OriginalConstructionRecipient:Send{
 type Context<'turn,'source> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn;
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn;
 fn grant<'a,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a> where 'source:'turn;
 fn accept<'a,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a> where 'source:'turn;
 fn submitted<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->RetainedCloneProgress where 'source:'turn;
}
trait OriginalConstructionDispatch{
 fn validate(&self)->Result<(),ValueError>;
 fn grant<'a>(&mut self,quote:OriginalConstructionGrant<'a>)->Result<OriginalConstructionGrantComplete<'a>,ValueError>;
 fn accept<'a>(&mut self,receipt:OriginalConstructionAccept<'a>)->Result<OriginalConstructionAcceptComplete<'a>,ValueError>;
 fn submitted(&self)->RetainedCloneProgress;
}
struct ConstructionDispatch<'borrow,'turn,'source,P:OriginalConstructionRecipient> where 'source:'turn{producer:&'borrow mut P,context:&'borrow mut P::Context<'turn,'source>,source_turn:Option<crate::OriginalSourceTurnIdentity>}
impl<'borrow,'turn,'source,P:OriginalConstructionRecipient> OriginalConstructionDispatch for ConstructionDispatch<'borrow,'turn,'source,P> where 'source:'turn{
 fn validate(&self)->Result<(),ValueError>{self.producer.validate_context(self.context)?;if self.producer.source_turn(self.context)!=self.source_turn{return Err(invalid())}Ok(())}
 fn grant<'a>(&mut self,quote:OriginalConstructionGrant<'a>)->Result<OriginalConstructionGrantComplete<'a>,ValueError>{self.producer.validate_context(self.context)?;if self.producer.source_turn(self.context)!=self.source_turn{return Err(invalid())}*quote.entered=true;Ok(self.producer.grant(self.context,quote))}
 fn accept<'a>(&mut self,receipt:OriginalConstructionAccept<'a>)->Result<OriginalConstructionAcceptComplete<'a>,ValueError>{self.producer.validate_context(self.context)?;if self.producer.source_turn(self.context)!=self.source_turn{return Err(invalid())}*receipt.entered=true;Ok(self.producer.accept(self.context,receipt))}
 fn submitted(&self)->RetainedCloneProgress{self.producer.submitted(self.context)}
}
/// 🪪️ Issues original actions and lends the same actual construction recipient.
struct OriginalConstructionIssuer{identity:OriginalConstructionIssuerIdentity,next:u64,outstanding:usize}
impl OriginalConstructionIssuer{fn new()->Self{Self{identity:OriginalConstructionIssuerIdentity(NEXT.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|n|n.checked_add(1)).expect("construction issuer identity exhausted")),next:1,outstanding:0}}fn loan<'a>(&'a mut self,recipient:&'a mut dyn OriginalConstructionDispatch,source_turn:Option<crate::OriginalSourceTurnIdentity>)->OriginalConstructionLoan<'a>{OriginalConstructionLoan{issuer:self,recipient,source_turn}}}
impl Default for OriginalConstructionIssuer{fn default()->Self{Self::new()}}
impl Drop for OriginalConstructionIssuer{fn drop(&mut self){assert!(std::thread::panicking()||self.outstanding==0,"original construction issuer retains actions")}}
/// 🏠️ Owns the actual original recipient and its unique issuer across borrowed turns.
pub struct OriginalConstructionAuthority<P:OriginalConstructionRecipient>{issuer:OriginalConstructionIssuer,recipient:P}
impl<P:OriginalConstructionRecipient> OriginalConstructionAuthority<P>{
 /// 🆕️ Binds the actual producer to its newly issued authority.
 pub fn new(recipient:P)->Self{Self{issuer:OriginalConstructionIssuer::new(),recipient}}
 /// 🎟️ Lends the same resident issuer and producer.
 pub fn with_loan<'turn,'source,R>(&mut self,context:&mut P::Context<'turn,'source>,operation:impl FnOnce(&mut OriginalConstructionLoan<'_>)->R)->Result<R,ValueError> where 'source:'turn{self.recipient.validate_context(context)?;let source_turn=self.recipient.source_turn(context);let mut dispatch=ConstructionDispatch{producer:&mut self.recipient,context,source_turn};let mut loan=self.issuer.loan(&mut dispatch,source_turn);Ok(operation(&mut loan))}
 /// 👁️ Borrows an immutable projection of the actual resident producer.
 pub fn inspect<R>(&self,query:impl FnOnce(&P)->R)->R{query(&self.recipient)}
 /// 🫴️ Returns the original producer only after all its actions have returned.
 pub fn into_recipient(self)->Result<P,Self>{if self.issuer.outstanding!=0{return Err(self)}Ok(self.recipient)}
}
/// 🧵️ Borrows one exact issuer and producer for each action turn.
pub struct OriginalConstructionLoan<'a>{issuer:&'a mut OriginalConstructionIssuer,recipient:&'a mut dyn OriginalConstructionDispatch,source_turn:Option<crate::OriginalSourceTurnIdentity>}
impl OriginalConstructionLoan<'_>{
 pub fn source_turn(&self)->Option<crate::OriginalSourceTurnIdentity>{self.source_turn}
 pub fn validate_context(&self)->Result<(),ValueError>{self.recipient.validate()}
 /// 🪪️ Observes this same actual construction issuer.
 pub fn identity(&self)->OriginalConstructionIssuerIdentity{self.issuer.identity}
 /// ⏳️ Observes whether this actual issuer retains any live action.
 pub fn has_live_actions(&self)->bool{self.issuer.outstanding!=0}
 pub fn reborrow(&mut self)->OriginalConstructionLoan<'_>{OriginalConstructionLoan{issuer:&mut *self.issuer,recipient:&mut *self.recipient,source_turn:self.source_turn}}
 pub fn submitted(&self)->Result<RetainedCloneProgress,ValueError>{self.validate_context()?;Ok(self.recipient.submitted())}
 fn matches(&self,action:&OriginalConstructionAction)->bool{action.identity.issuer==self.issuer.identity&&action.source_turn==self.source_turn}
 pub fn begin(&mut self,demand:RetirementDemand,slot:&mut Option<OriginalConstructionAction>)->Result<(),ValueError>{self.validate_context()?;if slot.is_some(){return Err(invalid())}let next=self.issuer.next.checked_add(1).ok_or_else(invalid)?;let outstanding=self.issuer.outstanding.checked_add(1).ok_or_else(invalid)?;let identity=OriginalConstructionActionIdentity{issuer:self.issuer.identity,action:self.issuer.next};self.issuer.next=next;self.issuer.outstanding=outstanding;*slot=Some(OriginalConstructionAction{identity,source_turn:self.source_turn,callback_invoked:false,demand,phase:OriginalConstructionPhase::Fresh,grant:None,progress:None,actual:None,grant_reply:None,accept_reply:None});Ok(())}
 pub fn quote(&mut self,action:&mut OriginalConstructionAction)->Result<(),ValueError>{self.validate_context()?;if !self.matches(action)||action.phase!=OriginalConstructionPhase::Fresh{return Err(invalid())}action.phase=OriginalConstructionPhase::GrantStarted;action.callback_invoked=false;let quote=OriginalConstructionGrant{identity:action.identity,demand:action.demand,entered:&mut action.callback_invoked,reply:&mut action.grant_reply};match self.recipient.grant(quote){Ok(complete)=>drop(complete),Err(error)=>{action.callback_invoked=false;return Err(error)}}Ok(())}
 pub fn recover_grant(&mut self,action:&mut OriginalConstructionAction)->Result<Option<Result<RetainedCloneGrant,ValueError>>,ValueError>{self.validate_context()?;if !self.matches(action)||action.phase!=OriginalConstructionPhase::GrantStarted{return Err(invalid())}if !action.callback_invoked{action.callback_invoked=false;let quote=OriginalConstructionGrant{identity:action.identity,demand:action.demand,entered:&mut action.callback_invoked,reply:&mut action.grant_reply};match self.recipient.grant(quote){Ok(complete)=>drop(complete),Err(error)=>{action.callback_invoked=false;return Err(error)}}}let reply=action.grant_reply.take();if let Some(result)=reply.as_ref(){match result{Ok(grant)=>{action.grant=Some(*grant);action.phase=OriginalConstructionPhase::Granted},Err(_)=>action.phase=OriginalConstructionPhase::GrantRefused}}Ok(reply)}
 pub fn post(&mut self,action:&mut OriginalConstructionAction,progress:RetainedCloneProgress,actual:Option<usize>)->Result<(),ValueError>{self.validate_context()?;if !self.matches(action)||action.phase!=OriginalConstructionPhase::Granted||action.progress.is_some()||actual.map_or(progress.retained_capacity_bytes!=0,|bytes|bytes!=progress.retained_capacity_bytes){return Err(invalid())}action.actual=actual;let grant=action.grant.ok_or_else(invalid)?;action.progress=Some(progress);action.phase=OriginalConstructionPhase::AcceptStarted;action.callback_invoked=false;let receipt=OriginalConstructionAccept{identity:action.identity,grant,progress,actual,entered:&mut action.callback_invoked,reply:&mut action.accept_reply};match self.recipient.accept(receipt){Ok(complete)=>drop(complete),Err(error)=>{action.callback_invoked=false;return Err(error)}}Ok(())}
 pub fn recover_accept(&mut self,action:&mut OriginalConstructionAction)->Result<Option<Result<(),ValueError>>,ValueError>{self.validate_context()?;if !self.matches(action)||action.phase!=OriginalConstructionPhase::AcceptStarted{return Err(invalid())}if !action.callback_invoked{let grant=action.grant.ok_or_else(invalid)?;let progress=action.progress.ok_or_else(invalid)?;action.callback_invoked=false;let receipt=OriginalConstructionAccept{identity:action.identity,grant,progress,actual:action.actual,entered:&mut action.callback_invoked,reply:&mut action.accept_reply};match self.recipient.accept(receipt){Ok(complete)=>drop(complete),Err(error)=>{action.callback_invoked=false;return Err(error)}}}let reply=action.accept_reply.take();if reply.is_some(){action.phase=OriginalConstructionPhase::Settled}Ok(reply)}
 /// ↩️ Cancels an accepted grant before any physical effect starts.
 pub fn cancel(&mut self,action:&mut OriginalConstructionAction,performed:bool,progress:RetainedCloneProgress,actual:Option<usize>)->Result<(),ValueError>{self.validate_context()?;if !self.matches(action)||!matches!(action.phase,OriginalConstructionPhase::Fresh|OriginalConstructionPhase::Granted)||action.progress.is_some()||performed||progress!=RetainedCloneProgress::default()||actual.is_some(){return Err(invalid())}action.phase=OriginalConstructionPhase::Settled;Ok(())}
 pub fn finish(&mut self,slot:&mut Option<OriginalConstructionAction>)->Result<(),ValueError>{self.validate_context()?;let action=slot.as_ref().ok_or_else(invalid)?;if !self.matches(action)||!matches!(action.phase,OriginalConstructionPhase::GrantRefused|OriginalConstructionPhase::Settled)||action.grant_reply.is_some()||action.accept_reply.is_some()||self.issuer.outstanding==0{return Err(invalid())}self.issuer.outstanding-=1;drop(slot.take());Ok(())}
}

#[cfg(test)]
mod tests{
 use super::*;
 struct Original{binding:std::sync::Arc<u8>,interrupt:bool,interrupt_accept:bool}
 impl OriginalConstructionRecipient for Original{
  type Context<'turn,'source>=&'turn u8 where 'source:'turn;
  fn validate_context<'turn,'source>(&self,context:&&'turn u8)->Result<(),ValueError> where 'source:'turn{if std::ptr::eq(self.binding.as_ref(),*context){Ok(())}else{Err(invalid())}}
  fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{None}
  fn grant<'a,'turn,'source>(&mut self,_:&mut &'turn u8,quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a> where 'source:'turn{let completed=quote.complete(Ok(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}));if self.interrupt{self.interrupt=false;panic!("original producer interrupted after grant reply")}completed}
  fn accept<'a,'turn,'source>(&mut self,_:&mut &'turn u8,receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a> where 'source:'turn{assert_eq!(receipt.actual(),None);let reply=receipt.complete(Err(ValueError::literal(ValueRefusalKind::Canceled,"actual original receipt refusal")));if self.interrupt_accept{self.interrupt_accept=false;panic!("original producer interrupted after receipt reply")}reply}
  fn submitted<'turn,'source>(&self,_:&&'turn u8)->RetainedCloneProgress where 'source:'turn{RetainedCloneProgress::default()}
 }
 #[test]
 fn recorded_grant_and_receipt_refusal_return_only_to_the_original_issuer(){
  let binding=std::sync::Arc::new(7u8);let foreign_binding=std::sync::Arc::new(8u8);let mut original=OriginalConstructionAuthority::new(Original{binding:binding.clone(),interrupt:true,interrupt_accept:true});let mut foreign=OriginalConstructionAuthority::new(Original{binding:foreign_binding.clone(),interrupt:false,interrupt_accept:false});let mut context=binding.as_ref();let mut foreign_context=foreign_binding.as_ref();let mut action=None;assert!(original.with_loan(&mut foreign_context,|_|()).is_err());
  original.with_loan(&mut context,|loan|{loan.begin(RetirementDemand::default(),&mut action).unwrap();let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||loan.quote(action.as_mut().unwrap())));assert!(interrupted.is_err());assert_eq!(action.as_ref().unwrap().phase(),OriginalConstructionPhase::GrantStarted);}).unwrap();
  foreign.with_loan(&mut foreign_context,|wrong|assert!(wrong.recover_grant(action.as_mut().unwrap()).is_err())).unwrap();
  original.with_loan(&mut context,|loan|{let grant=loan.recover_grant(action.as_mut().unwrap()).unwrap().unwrap().unwrap();assert_eq!(grant.maximum_items,1);assert!(loan.recover_grant(action.as_mut().unwrap()).is_err());let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||loan.post(action.as_mut().unwrap(),RetainedCloneProgress::default(),None)));assert!(interrupted.is_err());let refusal=loan.recover_accept(action.as_mut().unwrap()).unwrap().unwrap().unwrap_err();assert_eq!(refusal.message.as_ref(),"actual original receipt refusal");loan.finish(&mut action).unwrap();assert!(action.is_none());}).unwrap();
 }
}

#[cfg(test)]
#[test]
fn original_construction_retains_uninvoked_actual_receipt_until_the_same_source_returns(){
 use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
 struct Turn<'a>{token:&'a crate::OriginalSourceTurn,live:&'a AtomicBool,panic_next:&'a AtomicBool,panic_after:&'a AtomicUsize}
 struct Producer{source:crate::OriginalSourceTurnIssuer,posted:usize,actual:Option<usize>}
 impl OriginalConstructionRecipient for Producer{type Context<'turn,'source>=Turn<'turn> where 'source:'turn;fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{self.source.validate(turn.token).map_err(ValueError::from)?;if turn.panic_next.swap(false,Ordering::SeqCst){panic!("original Source validation interrupted")}if turn.panic_after.load(Ordering::SeqCst)>0&&turn.panic_after.fetch_sub(1,Ordering::SeqCst)==1{panic!("original Source dispatch validation interrupted")}if turn.live.load(Ordering::SeqCst){Ok(())}else{Err(ValueError::literal(ValueRefusalKind::Canceled,"original Source turn paused"))}}fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{Some(turn.token.identity())}fn submitted<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->RetainedCloneProgress where 'source:'turn{Default::default()}fn grant<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,quote:OriginalConstructionGrant<'reply>)->OriginalConstructionGrantComplete<'reply> where 'source:'turn{quote.complete(Ok(RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:8,maximum_depth:1,..Default::default()}))}fn accept<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,receipt:OriginalConstructionAccept<'reply>)->OriginalConstructionAcceptComplete<'reply> where 'source:'turn{self.posted+=1;self.actual=receipt.actual();receipt.complete(Ok(()))}}
 let source=crate::OriginalSourceTurnIssuer::new();let token=source.issue().unwrap();let live=AtomicBool::new(true);let panic_next=AtomicBool::new(false);let panic_after=AtomicUsize::new(0);let mut turn=Turn{token:&token,live:&live,panic_next:&panic_next,panic_after:&panic_after};let mut authority=OriginalConstructionAuthority::new(Producer{source,posted:0,actual:None});let layout=std::alloc::Layout::array::<u8>(8).unwrap();let pointer=unsafe{std::alloc::alloc(layout)};assert!(!pointer.is_null());let mut action=None;authority.with_loan(&mut turn,|loan|{loan.begin(RetirementDemand{capacity_bytes:8,depth:1,..Default::default()},&mut action).unwrap();loan.quote(action.as_mut().unwrap()).unwrap();assert!(loan.recover_grant(action.as_mut().unwrap()).unwrap().unwrap().is_ok());let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()};live.store(false,Ordering::SeqCst);assert_eq!(loan.post(action.as_mut().unwrap(),progress,Some(layout.size())).unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(action.as_ref().unwrap().actual(),None);live.store(true,Ordering::SeqCst);panic_next.store(true,Ordering::SeqCst);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||loan.post(action.as_mut().unwrap(),progress,Some(layout.size())))).is_err());assert_eq!(action.as_ref().unwrap().actual(),None);panic_after.store(2,Ordering::SeqCst);assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||loan.post(action.as_mut().unwrap(),progress,Some(layout.size())))).is_err());assert_eq!(action.as_ref().unwrap().actual(),Some(layout.size()));assert!(loan.recover_accept(action.as_mut().unwrap()).unwrap().unwrap().is_ok());loan.finish(&mut action).unwrap();}).unwrap();assert_eq!(authority.inspect(|producer|(producer.posted,producer.actual)),(1,Some(layout.size())));unsafe{std::alloc::dealloc(pointer,layout)};assert!(authority.into_recipient().is_ok());
}
