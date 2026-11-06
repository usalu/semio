//! 🎚️ Finite record decisions retain one paid typed field frontier and the original immutable source pair.
use super::RetainedFieldChange;
use semio_framework_value::{RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneBinding, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
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
 fn demand(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn advance(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome;
 fn finished(&self)->bool;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn close_step(&mut self,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome;
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
impl<R:Sync+'static,P,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>> RetainedRecordFieldChange<R,P,T,F>{pub fn new()->Self{Self{field:Default::default(),finished:false,projection:Default::default()}}}
impl<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>> RetainedRecordChangeField<R,P> for RetainedRecordFieldChange<R,P,T,F>{
 fn demand(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.finished{return Err(invalid("record field already transferred"))}if self.field.ready(){let mut d=self.field.change_demand()?;d.copy_bytes+=size_of::<Option<T>>()*2+size_of::<bool>()*2;return Ok(d)}let projections=F::PROJECT_WORK.checked_mul(2).ok_or_else(||invalid("record projection work overflow"))?;self.field.decision_demand(body,maximum_work_units.saturating_sub(projections))}
 fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if self.finished{return Err(invalid("record field already transferred"))}let work=if self.field.ready(){self.field.take_change_work_demand()?.checked_add(F::INSTALL_WORK).and_then(|v|v.checked_add(1))}else{let projections=F::PROJECT_WORK.checked_mul(2).ok_or_else(||invalid("record projection work overflow"))?;let child=self.field.decision_work_demand(body,maximum_work_units.saturating_sub(projections))?;if child==0{return Ok(0)}child.checked_add(projections)}.ok_or_else(||invalid("record field work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
 fn advance(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result=(||{if actual.is_some(){return Err(invalid("record field needs an empty allocation marker").into())}if self.work_demand(body,maximum_work_units)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let d=self.demand(body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.field.ready(){let projections=F::PROJECT_WORK.checked_mul(2).ok_or_else(||invalid("record projection work overflow"))?;let left=F::project(a);let right=F::project(o);work=projections;let turn=self.field.decide(left,right,body,maximum_work_units-projections,g,actual);work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}let fixed=size_of::<Option<T>>()*2+size_of::<bool>()*2;let turn=self.field.take_change(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,..g});work=turn.work;let Some((value,mut progress))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};F::install(p,value);work+=F::INSTALL_WORK;self.finished=true;work+=1;progress.copied_bytes+=fixed;Ok(RetainedCloneStep::Complete(progress))})();semio_framework_value::RetainedWorkOutcome{result,work}}
 fn finished(&self)->bool{self.finished}
 fn begin_close(&mut self){self.field.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.field.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.field.close_work_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{self.field.close_step(g,actual)}
 fn terminal_is_empty(&self)->bool{self.field.terminal_is_empty()}
}
/// 🏗️ The record field birth prices its typed equality cursor, two lazy clone preparations and direct owner fields.
struct RecordFieldBirth<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>>(std::marker::PhantomData<fn(R,P,T,F)>);
impl<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>> semio_framework_value_persistent::retained_equality::RetainedEqualityBirth for RecordFieldBirth<R,P,T,F>{type Output=RetainedRecordFieldChange<R,P,T,F>;fn work()->Result<u64,ValueError>{RetainedFieldChange::<T>::birth_work()?.checked_add(3).ok_or_else(||invalid("record field birth work overflow"))}fn birth()->Self::Output{RetainedRecordFieldChange::new()}}
/// ⛽️ Returns the constructor work and the independently admitted allocation turn.
pub fn boxed_record_field_work<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>>()->Result<u64,ValueError>{semio_framework_value_persistent::retained_equality::equality_box_work::<RecordFieldBirth<R,P,T,F>>()}
/// 🧳️ Constructs no child before its real work, physical grant and allocation marker have been admitted.
pub fn boxed_record_field<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone,F:RetainedRecordFieldProjection<R,P,T>>(maximum_work_units:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<Option<(Box<dyn RetainedRecordChangeField<R,P>>,RetainedCloneProgress)>>{let turn=semio_framework_value_persistent::retained_equality::equality_box::<RecordFieldBirth<R,P,T,F>>(maximum_work_units,grant,actual);RetainedWorkOutcome{work:turn.work,result:turn.result.map(|row|row.map(|(field,progress)|(field as Box<dyn RetainedRecordChangeField<R,P>>,progress)))}}
/// 🧳️ Record aliases remain immutable through every field and all partial original result retirement.
pub struct RetainedRecordChangeOwner<R:Sync+'static,P:RetainedRecordChange<R>>{authored:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<P>>,active:Option<Box<dyn RetainedRecordChangeField<R,P>>>,retirement:RetainedCloneClose,index:usize,bytes:usize,phase:u8,closing:bool}
impl<R:Sync+'static,P:RetainedRecordChange<R>> RetainedRecordChangeOwner<R,P>{
 pub fn new()->Self{Self{authored:None,original:None,output:ManuallyDrop::new(None),active:None,retirement:Default::default(),index:0,bytes:0,phase:0,closing:false}}
 /// 🧭️ Later original inverses must keep the same immutable record authority.
 pub fn verify_original(&self,o:RetainedCloneRef<'_,R>)->Result<(),ValueError>{o.check_binding(&self.original)?;Ok(())}
 pub fn ready(&self)->bool{self.phase==8&&!self.closing}
 fn binding_work(&self)->u64{let authored=if self.authored.is_some(){RetainedCloneRef::<R>::check_binding_work(&self.authored)}else{0};let original=if self.original.is_some(){RetainedCloneRef::<R>::check_binding_work(&self.original)}else{0};authored+original}
 fn phase_work(&self)->u64{match self.phase{2=>2,3 if self.index!=P::FIELD_COUNT=>3,4=>3,5=>2,6=>5,_=>1}}
 pub fn demand(&self,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>8{return Err(invalid("record decision settled or closing"))}let work=maximum_work_units.saturating_sub(self.binding_work());Ok(match self.phase{0|1=>nested(scalar(RetainedCloneBinding::alias_copy_bytes())),2=>scalar(META+size_of::<Option<P>>()*2),3=>if self.index==P::FIELD_COUNT{scalar(META)}else{let bytes=P::field_bytes(self.index)?;RetirementDemand{capacity_bytes:bytes,..scalar(META+bytes*2+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)}},4=>nested(self.active.as_ref().unwrap().demand(body.saturating_sub(META),work.saturating_sub(self.phase_work()))?),5=>if self.active.as_ref().unwrap().terminal_is_empty(){scalar(META)}else{nested(self.active.as_ref().unwrap().close_demand(body.saturating_sub(META))?)},6=>RetirementDemand{release_bytes:self.bytes,..scalar(META+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)},8=>self.take_demand()?,_=>return Err(invalid("record decision phase is invalid"))})}
 pub fn work_demand(&self,body:usize,maximum_work_units:u64)->Result<u64,ValueError>{if self.closing||self.phase>8{return Err(invalid("record decision settled or closing"))}if maximum_work_units==0||self.phase==8{return Ok(0)}let binding=self.binding_work();if maximum_work_units<=binding{return Ok(0)}let available=maximum_work_units-binding;let child=match self.phase{0|1=>RetainedCloneBinding::alias_work_demand(),6=>0,2=>P::EMPTY_WORK,3=>if self.index==P::FIELD_COUNT{0}else{P::field_work(self.index)?},4=>{let child=self.active.as_ref().unwrap().work_demand(body.saturating_sub(META),available.saturating_sub(self.phase_work()))?;if child==0{return Ok(0)}child},5=>if self.active.as_ref().unwrap().terminal_is_empty(){0}else{self.active.as_ref().unwrap().close_work_demand(body.saturating_sub(META))?},_=>return Err(invalid("record decision phase is invalid"))};let work=child.checked_add(self.phase_work()).and_then(|v|v.checked_add(binding)).ok_or_else(||invalid("record decision work overflow"))?;Ok(if work<=maximum_work_units{work}else{0})}
 pub fn advance(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{
  let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
   if actual.is_some(){return Err(invalid("record decision requires an empty allocation marker").into())}if self.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}if maximum_work_units==0||g.maximum_items==0||g.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.work_demand(body,maximum_work_units)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}
   let d=self.demand(body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let binding=self.binding_work();if self.authored.is_some(){let checked=a.check_binding_paid(&self.authored,maximum_work_units-work,g);work+=checked.work;if checked.result?!=Some(true){return Err(invalid("record authored binding changed").into())}}if self.original.is_some(){let checked=o.check_binding_paid(&self.original,maximum_work_units-work,g);work+=checked.work;if checked.result?!=Some(true){return Err(invalid("record original binding changed").into())}}let maximum_work_units=maximum_work_units-binding;work+=1;
   match self.phase{
    0=>{let turn=a.bind(&mut self.authored,child(g));work+=turn.work;p=turn.result?.unwrap();self.phase=1;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;},
    1=>{let turn=o.bind(&mut self.original,child(g));work+=turn.work;p=turn.result?.unwrap();self.phase=2;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;},
    2=>{*self.output=Some(P::empty_result());work+=P::EMPTY_WORK+1;self.phase=3;p.copied_bytes=size_of::<Option<P>>()*2+size_of::<u8>()*2;},
    3=>{if self.index==P::FIELD_COUNT{self.phase=8;}else{let bytes=P::field_bytes(self.index)?;let turn=P::field(self.index,maximum_work_units-self.phase_work(),g,actual);work+=turn.work;let Some((field,born))=turn.result? else{return Ok(RetainedCloneStep::Progress(Default::default()))};let reported=born.retained_capacity_bytes;self.bytes=actual.unwrap_or(0);self.active=Some(field);self.phase=4;work+=2;p.copied_bytes=born.copied_bytes+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2+size_of::<usize>()*2+size_of::<u8>()*2;p.retained_capacity_bytes=self.bytes;let marker=if reported==0{None}else{Some(reported)};if *actual!=marker||reported!=bytes{return Err(semio_framework_value::OriginalRetirementFailure::from_primary(invalid("record field allocation receipt disagrees with its admission")).with_retained_progress(p))}}},
    4=>{let phase_work=self.phase_work();let field=self.active.as_mut().ok_or_else(||invalid("active absent"))?;let turn=field.advance(a,o,self.output.as_mut().ok_or_else(||invalid("output absent"))?,body.saturating_sub(META),maximum_work_units-phase_work,child(g),actual);work+=turn.work;let step=turn.result?;p=step.progress();if self.active.as_ref().unwrap().finished(){self.active.as_mut().unwrap().begin_close();self.phase=5;work+=2;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;}},
    5=>{p=Default::default();if !self.active.as_ref().unwrap().terminal_is_empty(){let turn=self.active.as_mut().unwrap().close_step(child(g),actual);work+=turn.work;p=turn.result?.progress();}if self.active.as_ref().unwrap().terminal_is_empty(){self.phase=6;work+=1;p.copied_items=1;p.copied_bytes+=size_of::<u8>()*2;}},
    6=>{drop(self.active.take());p.released_bytes=self.bytes;self.bytes=0;self.index+=1;self.phase=3;work+=4;},
    _=>return Err(invalid("record decision phase is invalid").into())
   }Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
  })();semio_framework_value::RetainedWorkOutcome{result,work}
 }
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("record original sparse result pending"));}Ok(scalar(META+size_of::<Option<P>>()*2))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(P,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}self.phase=9;Ok(Some((self.output.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(field)=self.active.as_mut(){field.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("record original close not begun"));}if let Some(field)=self.active.as_ref(){return if field.terminal_is_empty(){Ok(RetirementDemand{release_bytes:self.bytes,..scalar(META+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)})}else{Ok(nested(field.close_demand(body.saturating_sub(META))?))};}if !self.retirement.is_empty()||self.output.is_some(){return Ok(nested(self.retirement.next_owner_demands::<P>(self.output.is_some(),&None,body.saturating_sub(META))?));}if self.original.is_some()||self.authored.is_some(){return Ok(nested(RetainedCloneBinding::close_demand(if self.original.is_some(){&self.original}else{&self.authored},body.saturating_sub(META))?));}Ok(Default::default())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if !self.closing{return Err(invalid("record original close not begun"))}if self.terminal_is_empty(){return Ok(0)}let body=body.saturating_sub(META);
  let child=if let Some(field)=self.active.as_ref(){if field.terminal_is_empty(){0}else{field.close_work_demand(body)?}}else if !self.retirement.is_empty()||self.output.is_some(){self.retirement.next_owner_work::<P>(self.output.is_some(),&None,body)?}else{RetainedCloneBinding::close_work_demand(if self.original.is_some(){&self.original}else{&self.authored},body)?};
  child.checked_add(1).ok_or_else(||invalid("record close work overflow"))
 }
 pub fn close_step(&mut self,g:RetainedCloneGrant,actual:&mut Option<usize>)->semio_framework_value::OriginalRetirementOutcome{
  let mut work=0;let result:Result<_,semio_framework_value::OriginalRetirementFailure>=(||{
   let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}work=1;
   if let Some(field)=self.active.as_mut(){if field.terminal_is_empty(){drop(self.active.take());self.bytes=0;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}let turn=field.close_step(child(g),actual);work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(receipt(s.progress()))).map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)})}
   if !self.retirement.is_empty(){let turn=self.retirement.step_granted(child(g));work+=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(receipt(s.progress()))).map_err(|error|{let progress=receipt(error.retained_progress());error.with_retained_progress(progress)})}
   let turn=self.retirement.begin_granted(&mut self.output,child(g));work+=turn.work;if let Some(step)=turn.result?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())))}
   let turn=RetainedCloneBinding::close_one(if self.original.is_some(){&mut self.original}else{&mut self.authored},child(g));work+=turn.work;let step=turn.result?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(receipt(step.progress()))}else{RetainedCloneStep::Progress(receipt(step.progress()))})
  })();semio_framework_value::RetainedWorkOutcome{result,work}
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.active.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.authored.is_none()&&self.original.is_none()}
}
impl<R:Sync+'static,P:RetainedRecordChange<R>> Drop for RetainedRecordChangeOwner<R,P>{fn drop(&mut self){let empty=self.active.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.authored.is_none()&&self.original.is_none();assert!(std::thread::panicking()||empty,"original record result/child/source custody requires explicit closure");if empty{unsafe{ManuallyDrop::drop(&mut self.output);}}}}
#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;
