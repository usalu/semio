//! 🎚️ Finite record decisions retain one paid typed field frontier and the original immutable source pair.
use super::RetainedFieldChange;
use crate::field::{Done,Flight,Reply,slot,native,owe,unit};
use semio_framework_value::{OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneBinding, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
use semio_framework_value_persistent::retained_equality::RetainedEquality;
use std::mem::{size_of,ManuallyDrop};
const META:usize=size_of::<usize>()*4+2;
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=META;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=META;p}
/// 🧬️ A finite schema quotes an allocation-free empty result and source-free typed child construction within its declared work; failed construction retains no child allocation.
pub trait RetainedRecordChange<R:Sync+'static>:RetireOwned+Send+'static{
 const FIELD_COUNT:usize;
 const EMPTY_WORK:u64;
 fn empty_result()->Self;
 fn field_bytes(index:usize)->Result<usize,ValueError>;
 fn field_work(index:usize)->Result<u64,ValueError>;
 fn field(index:usize,work:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<Option<(Box<dyn RetainedRecordChangeField<R,Self>>,RetainedCloneProgress)>>;
}
/// 🪡️ Typed children never retain borrowed pointers; all original aliases and partial fields close explicitly.
pub trait RetainedRecordChangeField<R:Sync+'static,P>:Send+'static{
 fn demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn work_demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn advance_into<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>;
 fn recover_advance_into<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>;
 fn finished(&self)->bool;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn requires_original_birth(&self)->bool;
 fn requires_original_control(&self)->bool;
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll;
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>;
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>;
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>;
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 🎯️ Implementations borrow a declared field and install its header without allocation, traversal, destruction or panic, within their quoted work.
pub trait RetainedRecordFieldProjection<R:Sync+'static,P,T:RetainedEquality+RetainedClone>:Send+'static{
 const PROJECT_WORK:u64;
 const INSTALL_WORK:u64;
 fn project(source:RetainedCloneRef<'_,R>)->RetainedCloneRef<'_,T>;
 fn install(patch:&mut P,value:Option<T>);
}
/// 🪡️ Named projections retain no borrowed pointers and install only the funded result.
pub struct RetainedRecordFieldChange<R:Sync+'static,P,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>>{field:RetainedFieldChange<T>,finished:bool,projection:std::marker::PhantomData<fn(R,P,F)>}
impl<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>> RetainedRecordFieldChange<R,P,T,F>{
 pub fn new()->Self{Self{field:Default::default(),finished:false,projection:Default::default()}}
 fn advance_reply<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
  let g=effect.grant();let projections=F::PROJECT_WORK.checked_mul(2).ok_or_else(||invalid("record projection work overflow"))?;
  if recover{return self.field.recover_decide_into(F::project(a),F::project(o),body,maximum_work_units.saturating_sub(projections),effect,reply.progress())}
  if effect.actual().is_some(){return Ok(reply.complete(Err(invalid("record field needs an empty allocation marker").into())))}
  let work=match RetainedRecordChangeField::<R,P>::work_demand(self,a,o,body,maximum_work_units){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
  if work==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let demand=match RetainedRecordChangeField::<R,P>::demand(self,a,o,body,maximum_work_units){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
  if !retained_clone_grant_admits(g,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if !self.field.ready(){
   charge!(effect,reply,projections,"record projection exceeded original Work");
   return self.field.decide_into(F::project(a),F::project(o),body,maximum_work_units-projections,effect,reply.progress())
  }
  let fixed=size_of::<Option<T>>()*2+size_of::<bool>()*2;let mut value=None;
  let mut inner=effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,..g});
  let taken=native(|slot|self.field.take_change_into(&mut value,&mut inner,slot))?;drop(inner);
  Ok(reply.complete(match taken{
   Ok(RetainedCloneStep::Complete(mut progress))=>{F::install(p,value.take());self.finished=true;progress.copied_bytes+=fixed;match unit(effect,F::INSTALL_WORK+1,"record field install exceeded original Work"){Ok(())=>Ok(RetainedCloneStep::Complete(progress)),Err(error)=>Err(error.into())}},
   Ok(RetainedCloneStep::Progress(_))=>Ok(RetainedCloneStep::Progress(Default::default())),
   Err(error)=>Err(error.into()),
  }))
 }
}
impl<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>> RetainedRecordChangeField<R,P> for RetainedRecordFieldChange<R,P,T,F>{

 fn demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.finished{return Err(invalid("record field already transferred"))}if self.field.ready(){let mut d=self.field.change_demand()?;d.copy_bytes+=size_of::<Option<T>>()*2+size_of::<bool>()*2;return Ok(d)}let projections=F::PROJECT_WORK.checked_mul(2).ok_or_else(||invalid("record projection work overflow"))?;self.field.decision_demand(F::project(a),F::project(o),body,maximum_work_units.saturating_sub(projections))}
 fn work_demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if self.finished{return Err(invalid("record field already transferred"))}let work=if self.field.ready(){self.field.take_change_work_demand()?.checked_add(F::INSTALL_WORK).and_then(|v|v.checked_add(1))}else{let projections=F::PROJECT_WORK.checked_mul(2).ok_or_else(||invalid("record projection work overflow"))?;let child=self.field.decision_work_demand(F::project(a),F::project(o),body,maximum_work_units.saturating_sub(projections))?;if child==0{return Ok(0)}child.checked_add(projections)}.ok_or_else(||invalid("record field work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
 fn advance_into<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(a,o,p,body,maximum_work_units,effect,reply,false)}
 fn recover_advance_into<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(a,o,p,body,maximum_work_units,effect,reply,true)}
 fn finished(&self)->bool{self.finished}
 fn begin_close(&mut self){self.field.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.field.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.field.close_work_demand(body)}
 fn requires_original_birth(&self)->bool{self.field.requires_original_birth()}
 fn requires_original_control(&self)->bool{self.field.requires_original_control()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{self.field.close_original_birth_step(original,grant)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.field.close_into(body,effect,reply)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.field.recover_close_into(body,effect,reply)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.field.close_original_into(original,body,effect,reply)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.field.recover_close_original_into(original,body,effect,reply)}
 fn terminal_is_empty(&self)->bool{self.field.terminal_is_empty()}
}
/// 🏗️ The record field birth prices its typed equality cursor, two lazy clone preparations and direct owner fields.
struct RecordFieldBirth<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>>(std::marker::PhantomData<fn(R,P,T,F)>);
unsafe impl<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>> semio_framework_value_persistent::retained_equality::RetainedEqualityBirth for RecordFieldBirth<R,P,T,F>{type Output=RetainedRecordFieldChange<R,P,T,F>;fn work()->Result<u64,ValueError>{RetainedFieldChange::<T>::birth_work()?.checked_add(3).ok_or_else(||invalid("record field birth work overflow"))}fn birth()->Self::Output{RetainedRecordFieldChange::new()}}
/// ⛽️ Returns the constructor work and the independently admitted allocation turn.
pub fn boxed_record_field_work<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>>()->Result<u64,ValueError>{semio_framework_value_persistent::retained_equality::equality_box_work::<RecordFieldBirth<R,P,T,F>>()}
/// 🧳️ Constructs no child before its real work, physical grant and allocation marker have been admitted.
pub fn boxed_record_field<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>>(maximum_work_units:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<Option<(Box<dyn RetainedRecordChangeField<R,P>>,RetainedCloneProgress)>>{let turn=semio_framework_value_persistent::retained_equality::equality_box::<RecordFieldBirth<R,P,T,F>>(maximum_work_units,grant,actual);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(field,progress)|(field as Box<dyn RetainedRecordChangeField<R,P>>,progress)))}}
/// 🧳️ Record aliases remain immutable through every field and all partial original result retirement.
pub struct RetainedRecordChangeOwner<R:Sync+'static,P:RetainedRecordChange<R>>{authored:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<P>>,active:Option<Box<dyn RetainedRecordChangeField<R,P>>>,retirement:RetainedCloneClose,flight:Flight,index:usize,bytes:usize,phase:u8,closing:bool}
impl<R:Sync+'static,P:RetainedRecordChange<R>> RetainedRecordChangeOwner<R,P>{
 pub fn new()->Self{Self{authored:None,original:None,output:ManuallyDrop::new(None),active:None,retirement:Default::default(),flight:Default::default(),index:0,bytes:0,phase:0,closing:false}}
 fn field(&self)->Result<&dyn RetainedRecordChangeField<R,P>,ValueError>{self.active.as_deref().ok_or_else(||invalid("record active field is absent"))}
 /// 🧭️ Later original inverses must keep the same immutable record authority.
 pub fn verify_original(&self,o:RetainedCloneRef<'_,R>)->Result<(),ValueError>{o.check_binding(&self.original)?;Ok(())}
 pub fn ready(&self)->bool{self.phase==8&&!self.closing}
 fn binding_work(&self)->u64{let authored=if self.authored.is_some(){RetainedCloneRef::<R>::check_binding_work(&self.authored)}else{0};let original=if self.original.is_some(){RetainedCloneRef::<R>::check_binding_work(&self.original)}else{0};authored+original}
 fn phase_work(&self)->u64{match self.phase{2=>2,3 if self.index!=P::FIELD_COUNT=>3,4=>3,5=>2,6=>5,_=>1}}
 pub fn demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>8{return Err(invalid("record decision settled or closing"))}let work=maximum_work_units.saturating_sub(self.binding_work());Ok(match self.phase{0|1=>nested(scalar(RetainedCloneBinding::alias_copy_bytes())),2=>scalar(META+size_of::<Option<P>>()*2),3=>if self.index==P::FIELD_COUNT{scalar(META)}else{let bytes=P::field_bytes(self.index)?;RetirementDemand{capacity_bytes:bytes,..scalar(META+bytes*2+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)}},4=>nested(self.field()?.demand(a,o,body.saturating_sub(META),work.saturating_sub(self.phase_work()))?),5=>if self.field()?.terminal_is_empty(){scalar(META)}else{nested(self.field()?.close_demand(body.saturating_sub(META))?)},6=>RetirementDemand{release_bytes:self.bytes,..scalar(META+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)},8=>self.take_demand()?,_=>return Err(invalid("record decision phase is invalid"))})}
 pub fn work_demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if self.closing||self.phase>8{return Err(invalid("record decision settled or closing"))}if maximum_work_units==0||self.phase==8{return Ok(0)}let binding=self.binding_work();if maximum_work_units<=binding{return Ok(0)}let available=maximum_work_units-binding;let child=match self.phase{0|1=>RetainedCloneBinding::alias_work_demand(),6=>0,2=>P::EMPTY_WORK,3=>if self.index==P::FIELD_COUNT{0}else{P::field_work(self.index)?},4=>{let child=self.field()?.work_demand(a,o,body.saturating_sub(META),available.saturating_sub(self.phase_work()))?;if child==0{return Ok(0)}child},5=>if self.field()?.terminal_is_empty(){0}else{self.field()?.close_work_demand(body.saturating_sub(META))?},_=>return Err(invalid("record decision phase is invalid"))};let work=child.checked_add(self.phase_work()).and_then(|v|v.checked_add(binding)).ok_or_else(||invalid("record decision work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
/// 📥️ Records one decision turn of the record owner or its active field in the caller's reply.
 pub fn advance_into<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(a,o,body,maximum_work_units,effect,reply,false)}
 /// 🔁️ Recovers the same active field action.
 pub fn recover_advance_into<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Done<'r>,ValueError>{self.advance_reply(a,o,body,maximum_work_units,effect,reply,true)}
 fn field_maximum(&self,maximum_work_units:u64)->u64{maximum_work_units.saturating_sub(self.binding_work()).saturating_sub(self.phase_work())}
 fn advance_reply<'r>(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{ let mut child_step=None;
  let g=effect.grant();
  let route=if recover{self.flight.resume(effect,body,maximum_work_units,None)?}else{
   if !self.flight.idle(){return Err(invalid("record decision requires recovery"))}
   if effect.actual().is_some(){return Ok(reply.complete(Err(invalid("record decision requires an empty allocation marker").into())))}
   if self.ready(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   if maximum_work_units==0||g.maximum_items==0||g.maximum_depth==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   let work=match self.work_demand(a,o,body,maximum_work_units){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if work==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   let d=match self.demand(a,o,body,maximum_work_units){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if !retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if self.authored.is_some(){match a.check_binding_paid(&self.authored,effect).result{Ok(Some(true))=>{},Ok(_)=>return Ok(reply.complete(Err(invalid("record authored binding changed").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}}
   if self.original.is_some(){match o.check_binding_paid(&self.original,effect).result{Ok(Some(true))=>{},Ok(_)=>return Ok(reply.complete(Err(invalid("record original binding changed").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}}
   charge!(effect,reply,1,"record decision exceeded original Work");
   if self.phase!=4&&!(self.phase==5&&!self.field()?.terminal_is_empty()){
    let maximum_work_units=maximum_work_units-self.binding_work();let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
    let settled:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
     match self.phase{
      0=>{let turn=a.bind(&mut self.authored,effect);p=turn.result?.ok_or_else(||invalid("record authored binding produced no receipt"))?;self.phase=1;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;},
      1=>{let turn=o.bind(&mut self.original,effect);p=turn.result?.ok_or_else(||invalid("record original binding produced no receipt"))?;self.phase=2;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;},
      2=>{*self.output=Some(P::empty_result());unit(effect,P::EMPTY_WORK+1,"record empty result exceeded original Work")?;self.phase=3;p.copied_bytes=size_of::<Option<P>>()*2+size_of::<u8>()*2;},
      3=>{if self.index==P::FIELD_COUNT{self.phase=8;}else{let bytes=P::field_bytes(self.index)?;let base=effect.performed_work();let remaining=maximum_work_units.saturating_sub(self.phase_work());let turn=P::field(self.index,remaining,g,effect.actual());owe(effect,base,turn.work)?;let Some((field,born))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};let reported=born.retained_capacity_bytes;self.bytes=effect.actual().unwrap_or(0);self.active=Some(field);self.phase=4;unit(effect,2,"record field birth exceeded original Work")?;p.copied_bytes=born.copied_bytes+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2+size_of::<usize>()*2+size_of::<u8>()*2;p.retained_capacity_bytes=self.bytes;let marker=if reported==0{None}else{Some(reported)};if *effect.actual()!=marker||reported!=bytes{return Err(OriginalRetirementFailure::from_primary(invalid("record field allocation receipt disagrees with its admission")).with_retained_progress(p))}}},
      5=>{p=Default::default();self.phase=6;unit(effect,1,"record field close exceeded original Work")?;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;},
      6=>{drop(self.active.take());p.released_bytes=self.bytes;self.bytes=0;self.index+=1;self.phase=3;unit(effect,4,"record field release exceeded original Work")?;},
      _=>return Err(invalid("record decision phase is invalid").into())
     }Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
    })();
    return Ok(reply.complete(settled))
   }
   let route=if self.phase==4{1}else{2};self.flight.start(effect,route,body,maximum_work_units,None)?;route
  };
  let nested=body.saturating_sub(META);let rest=self.field_maximum(maximum_work_units);let mut inner=effect.nested(child(g));
  let field=self.active.as_mut().ok_or_else(||invalid("record active field is absent"))?;
  match(route,recover){
   (1,false)=>{let done=field.advance_into(a,o,self.output.as_mut().ok_or_else(||invalid("record output is absent"))?,nested,rest,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,true)=>{let done=field.recover_advance_into(a,o,self.output.as_mut().ok_or_else(||invalid("record output is absent"))?,nested,rest,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,false)=>{let done=field.close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,true)=>{let done=field.recover_close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   _=>return Err(invalid("record decision route changed")),
  }
  drop(inner);
  let Some(result)=self.flight.settle(&mut child_step)else{return Ok(reply.pending())};
  let settled:Result<RetainedCloneStep,OriginalRetirementFailure>=(||{
   let step=result?;let mut p=step.progress();
   if route==1{let field=self.active.as_mut().ok_or_else(||invalid("record active field is absent"))?;if field.finished(){field.begin_close();self.phase=5;unit(effect,2,"record field finish exceeded original Work")?;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;}}
   else if self.field()?.terminal_is_empty(){self.phase=6;unit(effect,1,"record field close exceeded original Work")?;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;}
   Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();
  Ok(reply.complete(settled))
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("record original sparse result pending"));}Ok(scalar(META+size_of::<Option<P>>()*2))}
/// 📤️ Moves the finished sparse result into the caller slot.
 pub fn take_into<'r>(&mut self,output:&mut Option<P>,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r,RetainedCloneStep,ValueError>)->Result<Done<'r,RetainedCloneStep,ValueError>,ValueError>{
  if output.is_some(){return Err(invalid("record result output is occupied"))}
  let d=match self.take_demand(){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error)))};
  let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error)))};
  if remaining<1||!retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"record result exceeded original Work")))),Err(error)=>return Ok(reply.complete(Err(error)))}
  *output=Some(match self.output.take(){Some(value)=>value,None=>return Ok(reply.complete(Err(invalid("record sparse result is absent"))))});self.phase=9;
  Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(field)=self.active.as_mut(){field.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("record original close not begun"));}if let Some(field)=self.active.as_ref(){return if field.terminal_is_empty(){Ok(RetirementDemand{release_bytes:self.bytes,..scalar(META+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)})}else{Ok(nested(field.close_demand(body.saturating_sub(META))?))};}if !self.retirement.is_empty()||self.output.is_some(){return Ok(nested(self.retirement.next_owner_demands::<P>(self.output.is_some(),&None,body.saturating_sub(META))?));}if self.original.is_some()||self.authored.is_some(){return Ok(nested(RetainedCloneBinding::close_demand(if self.original.is_some(){&self.original}else{&self.authored},body.saturating_sub(META))?));}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.closing{return Err(invalid("record original close not begun"))}if self.terminal_is_empty(){return Ok(0)}let body=body.saturating_sub(META);
  let child=if let Some(field)=self.active.as_ref(){if field.terminal_is_empty(){0}else{field.close_work_demand(body)?}}else if !self.retirement.is_empty()||self.output.is_some(){self.retirement.next_owner_work::<P>(self.output.is_some(),&None,body)?}else{RetainedCloneBinding::close_work_demand(if self.original.is_some(){&self.original}else{&self.authored},body)?};
  child.checked_add(1).ok_or_else(||invalid("record close work overflow"))
 }
fn selected_binding(&mut self)->&mut Option<RetainedCloneBinding>{if self.original.is_some(){&mut self.original}else{&mut self.authored}}
 /// 👶️ Selects the child whose original Birth must precede an ordinary effect.
 pub fn requires_original_birth(&self)->bool{if !self.closing||!self.flight.idle(){return false}if let Some(field)=self.active.as_ref(){return !field.terminal_is_empty()&&field.requires_original_birth()}if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}if self.output.is_some(){return false}RetainedCloneBinding::requires_original_birth(if self.original.is_some(){&self.original}else{&self.authored})}
 /// 🪢️ Reports whether the selected child close runs under its original controller.
 pub fn requires_original_control(&self)->bool{if let Some(loaned)=self.flight.loaned(){return loaned}if !self.closing{return false}if let Some(field)=self.active.as_ref(){return !field.terminal_is_empty()&&field.requires_original_control()}if !self.retirement.is_empty(){return self.retirement.requires_original_control()}if self.output.is_some(){return false}RetainedCloneBinding::requires_original_control(if self.original.is_some(){&self.original}else{&self.authored})}
 /// 👶️ Continues the selected child Birth on the original controller.
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{if !self.requires_original_birth(){return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invalid("record original Birth is not selected").into()),work:0})}if let Some(field)=self.active.as_mut(){return field.close_original_birth_step(original,grant)}if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}RetainedCloneBinding::close_original_birth_step(self.selected_binding(),original,grant)}
 fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{ let mut child_step=None;
  if !self.closing{return Err(invalid("record original close not begun"))}
  let g=effect.grant();let nested=body.saturating_sub(META);
  let route=if recover{self.flight.resume(effect,body,0,original.as_deref_mut())?}else{
   if !self.flight.idle(){return Err(invalid("record close requires recovery"))}
   if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   if self.requires_original_birth(){return Err(invalid("record Birth must settle before effect"))}
   if self.requires_original_control()&&original.is_none(){return Err(invalid("record close requires original controller"))}
   let d=match self.close_demand(body){Ok(demand)=>demand,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   let work=match self.close_work_demand(body){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   let remaining=match effect.remaining_work_units(){Ok(work)=>work,Err(error)=>return Ok(reply.complete(Err(error.into())))};
   if remaining<work||!retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   charge!(effect,reply,1,"record close exceeded original Work");
   let refuse=|error:OriginalRetirementFailure|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)};
   if let Some(field)=self.active.as_ref(){if field.terminal_is_empty(){drop(self.active.take());self.bytes=0;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))))}}
   let route=if self.active.is_some(){1}else if !self.retirement.is_empty(){2}else{
    let mut inner=effect.nested(child(g));let turn=self.retirement.begin_granted(&mut self.output,&mut inner);
    match turn.result{Ok(Some(step))=>return Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(step.progress()))))),Ok(None)=>{},Err(error)=>return Ok(reply.complete(Err(refuse(error.into()))))}
    3};
   self.flight.start(effect,route,body,0,original.as_deref_mut())?;route
  };
  let mut inner=effect.nested(child(g));
  match(route,recover,original.as_deref_mut()){
   (1,false,None)=>{let done=self.active.as_mut().ok_or_else(||invalid("record active field is absent"))?.close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,true,None)=>{let done=self.active.as_mut().ok_or_else(||invalid("record active field is absent"))?.recover_close_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,false,Some(loan))=>{let done=self.active.as_mut().ok_or_else(||invalid("record active field is absent"))?.close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (1,true,Some(loan))=>{let done=self.active.as_mut().ok_or_else(||invalid("record active field is absent"))?.recover_close_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,false,None)=>{let done=self.retirement.step_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,true,None)=>{let done=self.retirement.recover_step_into(nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,false,Some(loan))=>{let done=self.retirement.step_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (2,true,Some(loan))=>{let done=self.retirement.recover_step_original_into(loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,false,None)=>{let binding=if self.original.is_some(){&mut self.original}else{&mut self.authored};let done=RetainedCloneBinding::close_into(binding,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,true,None)=>{let binding=if self.original.is_some(){&mut self.original}else{&mut self.authored};let done=RetainedCloneBinding::recover_close_into(binding,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,false,Some(loan))=>{let binding=if self.original.is_some(){&mut self.original}else{&mut self.authored};let done=RetainedCloneBinding::close_original_into(binding,loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   (3,true,Some(loan))=>{let binding=if self.original.is_some(){&mut self.original}else{&mut self.authored};let done=RetainedCloneBinding::recover_close_original_into(binding,loan,nested,&mut inner,slot(&mut child_step)?)?;drop(done)},
   _=>return Err(invalid("record close route changed")),
  }
  drop(inner);
  match self.flight.settle(&mut child_step){None=>Ok(reply.pending()),Some(result)=>{let settled=result.map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)}).map(|step|{let progress=receipt(step.progress());if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)}});Ok(reply.complete(settled))}}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.flight.idle()&&self.active.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.authored.is_none()&&self.original.is_none()}
}
retained_close!([R:Sync+'static,P:RetainedRecordChange<R>] RetainedRecordChangeOwner<R,P>,close_reply);
impl<R:Sync+'static,P:RetainedRecordChange<R>> Drop for RetainedRecordChangeOwner<R,P>{fn drop(&mut self){let empty=self.active.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.authored.is_none()&&self.original.is_none();assert!(std::thread::panicking()||empty,"original record result/child/source custody requires explicit closure");if empty{unsafe{ManuallyDrop::drop(&mut self.output);}}}}
#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;
