//! 🔁️ Blocking carriers drive native bounded receivers to completion for synchronous text and pack contracts whose owners have no direct wire constructor.
use super::{NativeReceive,NativeReceiver};
use crate::{DslValue,NativeDecodeConstructionRecipient,NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneSource,RetainedCloneStep,retained_clone_grant_admits},retirement::{RetireOwned,controlled::ControlledRetirement}};

const TURN_BODY:usize=4096;
const TURN_LIMIT:usize=1<<24;

fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

/// 🎟️ One exact single-item grant per admitted turn, bounded by the original turn body.
fn exact(demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{
 if demand.copy_bytes>TURN_BODY{return Err(fault("a native receiving turn exceeds its original body"));}
 Ok(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth})
}

struct Construction{grant:RetainedCloneGrant,issued:bool}
impl NativeDecodeConstructionRecipient for Construction{
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{
  if self.issued||!retained_clone_grant_admits(self.grant,demand){return Err(fault("a native receiving turn exceeded its single physical birth"));}
  self.issued=true;Ok(self.grant)
 }
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{
  if !self.issued||grant!=self.grant||!progress.fits(grant){return Err(fault("a native receiving turn reported an unfunded birth"));}
  Ok(())
 }
}

fn admit<T:RetireOwned+Sync>(value:T)->Result<RetainedCloneSource<T>,ValueError>{
 let demand=RetainedCloneSource::<T>::owned_constructor_demand::<()>();
 let grant=RetainedCloneGrant{maximum_copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),..exact(RetirementDemand{capacity_bytes:demand.capacity_bytes,depth:demand.depth,..Default::default()})?};
 RetainedCloneSource::admit_owned(value,(),grant).map(|(source,_)|source).map_err(|_|fault("the original source was refused admission"))
}

fn advance<T,R:NativeReceiver<DslValue,T>>(receiver:&mut R,original:RetainedCloneRef<'_,DslValue>,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
 let mut recipient=Construction{grant,issued:false};
 control.with_construction_recipient(&mut recipient,|control|receiver.advance(original,control,TURN_BODY,grant))
}

fn run<T,R:NativeReceiver<DslValue,T>>(receiver:&mut R,original:&RetainedCloneSource<DslValue>,control:&mut NativeDecodeControl<'_>)->Result<T,ValueError>{
 for _ in 0..TURN_LIMIT{
  if receiver.output_ready(){return receiver.take(exact(receiver.take_demand()?)?)?.map(|(output,_)|output).ok_or_else(||fault("a ready native receiver yielded no output"));}
  let demand=receiver.demands(original.borrow(),TURN_BODY)?;
  control.admit_turn_capacity(demand.capacity_bytes)?;
  advance::<T,R>(receiver,original.borrow(),control,exact(demand)?)?;
 }
 Err(fault("a native receiver did not settle"))
}

fn close<T,R:NativeReceiver<DslValue,T>>(receiver:&mut R)->Result<(),ValueError>{
 receiver.begin_close();
 for _ in 0..TURN_LIMIT{
  if receiver.terminal_is_empty(){return Ok(());}
  receiver.close_step(exact(receiver.close_demands(TURN_BODY)?)?)?;
 }
 Err(fault("a native receiver did not close"))
}

/// ♻️ Retires one owner through its funded controlled retirement.
pub fn retire<T:RetireOwned>(value:T)->Result<(),ValueError>{
 let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error)?;
 for _ in 0..TURN_LIMIT{
  if owner.terminal_is_empty(){return Ok(());}
  owner.step(exact(owner.next_demand(TURN_BODY)?)?)?;
 }
 Err(fault("a native owner did not retire"))
}

fn receive_from<T:NativeReceive<DslValue>>(original:&RetainedCloneSource<DslValue>)->Result<T,ValueError>{
 let (mut receiver,_)=T::begin_receive(exact(T::receive_birth_demand())?)?;
 let mut callback=|_|true;
 let mut control=NativeDecodeControl::new_retained(&mut callback);
 let outcome=run::<T,_>(&mut receiver,original,&mut control);
 let closed=close::<T,_>(&mut receiver);
 match(outcome,closed){
  (Ok(output),Ok(()))=>Ok(output),
  (Ok(output),Err(error))=>{retire(output)?;Err(error)}
  (Err(error),_)=>Err(error),
 }
}

/// 🛬️ Receives one native owner from an owned wire value under unbounded funding, closing every partial owner on refusal.
pub fn receive_blocking<T:NativeReceive<DslValue>>(value:DslValue)->Result<T,ValueError>{
 let original=admit(value)?;
 let outcome=receive_from::<T>(&original);
 let retired=retire(original);
 match(outcome,retired){
  (Ok(output),Ok(()))=>Ok(output),
  (Ok(output),Err(error))=>{retire(output)?;Err(error)}
  (Err(error),_)=>Err(error),
 }
}
