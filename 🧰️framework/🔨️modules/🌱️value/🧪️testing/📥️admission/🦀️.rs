//! 📥️ Test-only source admission drives each real stage from the caller's existing finite wallet.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_native_decoding::{native_decoding,native_decoding::NativeDecodeControl,native_decoding::OriginalReceivingPrimary};
pub use semio_framework_value_retained_clone::retained_clone;
pub use semio_framework_value_retirement::retirement;
use crate::{retained_clone::RetainedCloneSource,retirement::RetireOwned};
pub use semio_framework_value_testing_effect_bank::EffectBank;
pub fn capture<T:RetireOwned+Sync,W>(value:T,wallet:&mut W,loan:impl Fn(&W)->RetainedCloneGrant,available_work:impl Fn(&W)->u64,mut post:impl FnMut(&mut W,RetainedCloneProgress,u64,(usize,usize),RetainedCloneGrant))->RetainedCloneSource<T>{
 let mut owner=RetainedCloneSource::admission_owned(value,());let mut actual=None;
 for _ in 0..10000{
  if actual.is_some(){let g=loan(wallet);let d=RetirementDemand{copy_bytes:std::mem::size_of::<Option<usize>>(),depth:1,..Default::default()};assert!(retained_clone::retained_clone_grant_admits(g,d));assert!(available_work(wallet)>=1);actual=None;post(wallet,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},1,(0,0),g);continue}
  if owner.phase()=="ready"{break}let g=loan(wallet);let maximum=available_work(wallet);let quoted=owner.advance_work_demand().unwrap();assert!(quoted<=maximum,"source fixture lacks its actual finite caller work");assert!(retained_clone::retained_clone_grant_admits(g,owner.next_demand().unwrap()));
  let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.advance(g,&mut actual));assert!(turn.work<=quoted);let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));if let Some(bytes)=actual{assert_eq!(bytes,heap.0)}else{assert_eq!(heap.0,0)}post(wallet,p,turn.work,heap,g);turn.result.unwrap();
 }
 assert!(actual.is_none());assert_eq!(owner.phase(),"ready","source fixture failed at its actual demand {:?}",owner.next_demand());let mut output=None;let g=loan(wallet);assert!(available_work(wallet)>=1);let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.take_output(&mut output,g));assert!(turn.work<=1);let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(|e|e.retained_progress());assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));post(wallet,p,turn.work,heap,g);turn.result.unwrap();assert!(owner.terminal_is_empty());output.expect("paid source output remains in its actual caller slot")
}

struct ConstructionLoan{current:RetainedCloneGrant,issued:bool,accepted:bool,submitted:RetainedCloneProgress}
impl crate::native_decoding::NativeDecodeConstructionRecipient for ConstructionLoan{fn submitted(&self)->RetainedCloneProgress{self.submitted}fn grant(&mut self,d:crate::RetirementDemand)->Result<RetainedCloneGrant,crate::ValueError>{assert!(!self.issued);assert!(crate::retained_clone::retained_clone_grant_admits(self.current,d));self.issued=true;Ok(self.current)}fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),crate::ValueError>{self.submitted=self.submitted.checked_add(p)?;assert!(!self.accepted);assert!(p.fits(g));assert!(p.fits(self.current));assert!(g.maximum_items<=self.current.maximum_items&&g.maximum_copy_bytes<=self.current.maximum_copy_bytes&&g.maximum_capacity_bytes<=self.current.maximum_capacity_bytes&&g.maximum_release_bytes<=self.current.maximum_release_bytes&&g.maximum_depth<=self.current.maximum_depth);self.accepted=true;Ok(())}}
pub struct WorkLoan<'a>{remaining:&'a u64,maximum:u64}
impl<'a> WorkLoan<'a>{pub fn new(remaining:&'a u64,maximum:u64)->Self{Self{remaining,maximum}}}
impl crate::native_decoding::NativeDecodeWorkRecipient for WorkLoan<'_>{fn available_work(&mut self,observed:u64)->Result<u64,crate::ValueError>{let limit:u64=(*self.remaining).min(self.maximum);limit.checked_sub(observed).ok_or_else(||crate::ValueError::invariant("native observed work exceeds its actual finite scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,crate::ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=(*self.remaining).min(self.maximum)))}}
pub fn receive<T,E:From<crate::ValueError>>(control:&mut crate::NativeDecodeControl<'_>,g:RetainedCloneGrant,mut work_loan:WorkLoan<'_>,apply:impl FnOnce(&mut crate::NativeDecodeControl<'_>)->crate::RetainedWorkOutcome<T,E>)->crate::RetainedWorkOutcome<T,E>{let mut loan=ConstructionLoan{current:g,issued:false,accepted:false,submitted:Default::default()};let mut construction_issuer=crate::native_decoding::NativeConstructionRecipientIssuer::new();let mut construction_loan=construction_issuer.loan(&mut loan);let mut work_issuer=crate::native_decoding::NativeDecodeWorkIssuer::new();let mut native_work_loan=work_issuer.loan(&mut work_loan);let mut work=0;let result=control.with_work_recipient(&mut native_work_loan,|control|control.with_construction_recipient(&mut construction_loan,|control|{let turn=apply(control);work=turn.work;turn.result}));crate::RetainedWorkOutcome{result,work}}


pub struct Wallet{left:RetainedCloneGrant,work:u64}
impl Wallet{
 pub fn remaining(&self)->RetainedCloneGrant{self.left}
 pub fn work_remaining(&self)->u64{self.work}
 pub fn work_loan(&self,maximum:u64)->WorkLoan<'_>{WorkLoan::new(&self.work,maximum)}
 pub fn from_profile(profile:&serde_json::Value)->Self{Self{left:RetainedCloneGrant{maximum_items:profile["items"].as_u64().unwrap().try_into().unwrap(),maximum_copy_bytes:profile["copyBytes"].as_u64().unwrap().try_into().unwrap(),maximum_capacity_bytes:profile["capacityBytes"].as_u64().unwrap().try_into().unwrap(),maximum_release_bytes:profile["releaseBytes"].as_u64().unwrap().try_into().unwrap(),maximum_depth:profile["depth"].as_u64().unwrap().try_into().unwrap()},work:profile["fuel"].as_u64().unwrap()}}
 pub fn loan(&self,items:usize,copy:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:if self.work==0{0}else{self.left.maximum_items.min(items)},maximum_copy_bytes:self.left.maximum_copy_bytes.min(copy),..self.left}}
 pub fn post(&mut self,p:RetainedCloneProgress,work:u64,g:RetainedCloneGrant,heap:(usize,usize)){assert!(p.fits(g));assert!(p.fits(self.left));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));self.left.maximum_items=self.left.maximum_items.checked_sub(p.copied_items).unwrap();self.left.maximum_copy_bytes=self.left.maximum_copy_bytes.checked_sub(p.copied_bytes).unwrap();self.left.maximum_capacity_bytes=self.left.maximum_capacity_bytes.checked_sub(p.retained_capacity_bytes).unwrap();self.left.maximum_release_bytes=self.left.maximum_release_bytes.checked_sub(p.released_bytes).unwrap();self.work=self.work.checked_sub(work).expect("native fixture depleted its explicit CPU stock");}
}

pub fn post_step<E>(wallet:&mut Wallet,turn:crate::RetainedWorkOutcome<crate::RetainedCloneStep,E>,grant:RetainedCloneGrant,heap:(usize,usize),receipt:impl FnOnce(&E)->RetainedCloneProgress)->Result<crate::RetainedCloneStep,E>{let p=turn.result.as_ref().map(|s|s.progress()).unwrap_or_else(receipt);assert!(turn.work<=wallet.work_remaining());wallet.post(p,turn.work,grant,heap);turn.result}
/// ⛽️ Posts only work covered by the exact original caller loan.
pub fn post_step_with_work<E>(wallet:&mut Wallet,turn:crate::RetainedWorkOutcome<crate::RetainedCloneStep,E>,grant:RetainedCloneGrant,maximum:u64,heap:(usize,usize),receipt:impl FnOnce(&E)->RetainedCloneProgress)->Result<crate::RetainedCloneStep,E>{let p=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(receipt);assert!(turn.work<=maximum);wallet.post(p,turn.work,grant,heap);turn.result}
pub fn denials(g:RetainedCloneGrant,d:crate::RetirementDemand)->impl Iterator<Item=RetainedCloneGrant>{[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(yes,g)|yes.then_some(g))}
pub fn close_owned<T:RetireOwned>(value:T,wallet:&mut Wallet,body:usize,limit:usize){
 let(mut owner,h)=semio_framework_value_testing_allocator::observe_retirement_allocations(||crate::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native fixture supports paid original closure")));assert_eq!(h,(0,0));let mut actual=None;
 for _ in 0..limit{
  if actual.is_some(){let d=RetirementDemand{copy_bytes:std::mem::size_of::<Option<usize>>(),depth:1,..Default::default()};let g=wallet.loan(1,body);assert!(retained_clone::retained_clone_grant_admits(g,d));assert!(wallet.work_remaining()>=1);actual=None;wallet.post(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},1,g,(0,0));continue}
  if owner.terminal_is_empty(){return}
  let d=owner.next_demand(body).unwrap();let quoted=owner.next_computational_work(body).unwrap();assert!(quoted<=wallet.work_remaining());assert!(d.copy_bytes<=body);let g=wallet.loan(1,body);
  let(result,h)=semio_framework_value_testing_allocator::observe_retirement_allocations(||owner.step(g,&mut actual));let work=owner.last_turn_work();assert!(work<=quoted);if let Some(bytes)=actual{assert_eq!(bytes,h.0)}else{assert_eq!(h.0,0)}
  post_step(wallet,crate::RetainedWorkOutcome{result,work},g,h,crate::OriginalRetirementFailure::retained_progress).unwrap();
 }
 panic!("native fixture did not close in its original turn bound")
}

/// 🧷️ A test retirement step keeps the actual allocator marker until its separately funded metadata clear.
pub fn close_step(actual:&mut Option<usize>,grant:RetainedCloneGrant,available_work:u64,demand:Result<RetirementDemand,ValueError>,quoted:Result<u64,ValueError>,apply:impl FnOnce(RetainedCloneGrant,&mut Option<usize>)->semio_framework_value_retirement::OriginalRetirementOutcome)->semio_framework_value_retirement::OriginalRetirementOutcome{
 if actual.is_some(){let demand=RetirementDemand{copy_bytes:std::mem::size_of::<Option<usize>>(),depth:1,..Default::default()};if available_work<1||!retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}*actual=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}
 let(demand,quoted)=match demand.and_then(|demand|quoted.map(|quoted|(demand,quoted))){Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
 if quoted>available_work||!retained_clone::retained_clone_grant_admits(grant,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
 let turn=apply(grant,actual);assert!(turn.work<=quoted);let progress=turn.result.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());if let Some(bytes)=*actual{assert_eq!(bytes,progress.retained_capacity_bytes)}else{assert_eq!(progress.retained_capacity_bytes,0)}turn
}

/// 🧾️ Quotes the actual pending marker clear before inspecting the child close frontier.
pub fn close_demands(actual:&Option<usize>,child:Result<RetirementDemand,ValueError>)->Result<RetirementDemand,ValueError>{if actual.is_some(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<usize>>(),depth:1,..Default::default()})}else{child}}
/// ⛽️ Selects the real marker-clear work before the next child action.
pub fn close_work_demand(actual:&Option<usize>,child:Result<u64,ValueError>)->Result<u64,ValueError>{if actual.is_some(){Ok(1)}else{child}}
