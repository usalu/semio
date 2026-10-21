//! 🎚️ Finite record decisions retain one paid typed field frontier and the original immutable source pair.
use super::RetainedFieldChange;
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retirement::RetireOwned, retained_clone::{RetainedClone, RetainedCloneRef, RetainedCloneBinding, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits}};
use semio_framework_value_persistent::retained_equality::RetainedEquality;
use std::mem::{size_of,ManuallyDrop};
const META:usize=size_of::<usize>()*4+2;
fn invalid(text:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,text)}
fn scalar(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=META;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-META,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=META;p}
/// 🧬️ A finite schema creates an allocation-free empty result and exactly the declared source-free typed child body.
pub trait RetainedRecordChange<R:Sync+'static>:RetireOwned+Send+'static{
 const FIELD_COUNT:usize;
 fn empty_result()->Self;
 fn field_bytes(index:usize)->Result<usize,ValueError>;
 fn field(index:usize)->Result<Box<dyn RetainedRecordChangeField<R,Self>>,ValueError>;
}
/// 🪡️ Typed children never retain borrowed pointers; all original aliases and partial fields close explicitly.
pub trait RetainedRecordChangeField<R:Sync+'static,P>:Send+'static{
 fn demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn finished(&self)->bool;
 fn begin_close(&mut self);
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}
/// 🎯️ Field selectors borrow actual native members; setters move only the admitted field header into its declared sparse slot.
pub struct RetainedRecordFieldChange<R:Sync+'static,P,T:RetainedEquality+RetainedClone>{field:RetainedFieldChange<T>,get:for<'a>fn(RetainedCloneRef<'a,R>)->RetainedCloneRef<'a,T>,set:fn(&mut P,Option<T>),finished:bool}
impl<R:Sync+'static,P,T:RetainedEquality+RetainedClone> RetainedRecordFieldChange<R,P,T>{pub fn new(get:for<'a>fn(RetainedCloneRef<'a,R>)->RetainedCloneRef<'a,T>,set:fn(&mut P,Option<T>))->Self{Self{field:Default::default(),get,set,finished:false}}}
impl<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone> RetainedRecordChangeField<R,P> for RetainedRecordFieldChange<R,P,T>{
 fn demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize)->Result<RetirementDemand,ValueError>{if self.finished{return Err(invalid("record field already transferred"));}if self.field.ready(){let mut d=self.field.change_demand()?;d.copy_bytes+=size_of::<Option<T>>()*2+size_of::<bool>();return Ok(d);}self.field.decision_demand((self.get)(a),(self.get)(o),body)}
 fn advance(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,p:&mut P,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(a,o,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.field.ready(){return self.field.decide((self.get)(a),(self.get)(o),body,g).map(|s|RetainedCloneStep::Progress(s.progress()));}let fixed=size_of::<Option<T>>()*2+size_of::<bool>();let(value,mut progress)=self.field.take_change(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,..g})?.unwrap();(self.set)(p,value);self.finished=true;progress.copied_bytes+=fixed;Ok(RetainedCloneStep::Complete(progress))}
 fn finished(&self)->bool{self.finished}
 fn begin_close(&mut self){self.field.begin_close();}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.field.close_demand(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.field.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.field.terminal_is_empty()}
}
/// 🏗️ Allocates only an admitted concrete cursor body; the original record owner quotes its exact native layout first.
pub fn boxed_record_field<R:Sync+'static,P:'static,T:RetainedEquality+RetainedClone>(field:RetainedRecordFieldChange<R,P,T>)->Result<Box<dyn RetainedRecordChangeField<R,P>>,ValueError>{semio_framework_value_persistent::retained_equality::equality_box(field).map(|field|field as Box<dyn RetainedRecordChangeField<R,P>>)}
/// 🧳️ Record aliases remain immutable through every field and all partial original result retirement.
pub struct RetainedRecordChangeOwner<R:Sync+'static,P:RetainedRecordChange<R>>{authored:Option<RetainedCloneBinding>,original:Option<RetainedCloneBinding>,output:ManuallyDrop<Option<P>>,active:Option<Box<dyn RetainedRecordChangeField<R,P>>>,retirement:RetainedCloneClose,index:usize,bytes:usize,phase:u8,closing:bool}
impl<R:Sync+'static,P:RetainedRecordChange<R>> RetainedRecordChangeOwner<R,P>{
 pub fn new()->Self{Self{authored:None,original:None,output:ManuallyDrop::new(None),active:None,retirement:Default::default(),index:0,bytes:0,phase:0,closing:false}}
 /// 🧭️ Later original inverses must keep the same immutable record authority.
 pub fn verify_original(&self,o:RetainedCloneRef<'_,R>)->Result<(),ValueError>{o.check_binding(&self.original)?;Ok(())}
 pub fn ready(&self)->bool{self.phase==8&&!self.closing}
 pub fn demand(&self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>8{return Err(invalid("record decision settled or closing"));}a.check_binding(&self.authored)?;o.check_binding(&self.original)?;Ok(match self.phase{0=>nested(scalar(a.binding_copy_bytes())),1=>nested(scalar(o.binding_copy_bytes())),2=>scalar(META+size_of::<Option<P>>()*2),3=>if self.index==P::FIELD_COUNT{scalar(META)}else{let bytes=P::field_bytes(self.index)?;RetirementDemand{capacity_bytes:bytes,..scalar(META+bytes*2+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)}},4=>nested(self.active.as_ref().unwrap().demand(a,o,body)?),5=>nested(self.active.as_ref().unwrap().close_demand(body.saturating_sub(META))?),6=>RetirementDemand{release_bytes:self.bytes,..scalar(META+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)},8=>self.take_demand()?,_=>return Err(invalid("record decision phase is invalid"))})}
 pub fn advance(&mut self,a:RetainedCloneRef<'_,R>,o:RetainedCloneRef<'_,R>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(a,o,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{0=>{p=receipt(a.bind(&mut self.authored,child(g))?.unwrap());self.phase=1;},1=>{p=receipt(o.bind(&mut self.original,child(g))?.unwrap());self.phase=2;},2=>{*self.output=Some(P::empty_result());self.phase=3;},3=>{if self.index==P::FIELD_COUNT{self.phase=8;}else{self.bytes=P::field_bytes(self.index)?;self.active=Some(P::field(self.index)?);p.retained_capacity_bytes=self.bytes;self.phase=4;}},4=>{p=receipt(self.active.as_mut().unwrap().advance(a,o,self.output.as_mut().unwrap(),body,child(g))?.progress());if self.active.as_ref().unwrap().finished(){self.active.as_mut().unwrap().begin_close();self.phase=5;}},5=>{p=receipt(self.active.as_mut().unwrap().close_step(child(g))?.progress());if self.active.as_ref().unwrap().terminal_is_empty(){self.phase=6;}},6=>{drop(self.active.take());p.released_bytes=self.bytes;self.bytes=0;self.index+=1;self.phase=3;},8=>return Ok(RetainedCloneStep::Complete(Default::default())),_=>unreachable!()}Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid("record original sparse result pending"));}Ok(scalar(META+size_of::<Option<P>>()*2))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(P,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}self.phase=9;Ok(Some((self.output.take().unwrap(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(field)=self.active.as_mut(){field.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid("record original close not begun"));}if let Some(field)=self.active.as_ref(){return if field.terminal_is_empty(){Ok(RetirementDemand{release_bytes:self.bytes,..scalar(META+size_of::<Option<Box<dyn RetainedRecordChangeField<R,P>>>>()*2)})}else{Ok(nested(field.close_demand(body.saturating_sub(META))?))};}if !self.retirement.is_empty()||self.output.is_some(){return Ok(nested(self.retirement.next_owner_demands::<P>(self.output.is_some(),&None,body.saturating_sub(META))?));}if self.original.is_some()||self.authored.is_some(){return Ok(nested(RetainedCloneBinding::close_demand(if self.original.is_some(){&self.original}else{&self.authored},body.saturating_sub(META))?));}Ok(Default::default())}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(field)=self.active.as_mut(){if field.terminal_is_empty(){drop(self.active.take());self.bytes=0;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}));}return field.close_step(child(g)).map(|s|RetainedCloneStep::Progress(receipt(s.progress())));}if !self.retirement.is_empty(){return self.retirement.step_granted(child(g)).map(|s|RetainedCloneStep::Progress(receipt(s.progress())));}if let Some(step)=self.retirement.begin_granted(&mut self.output,child(g))?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())));}let step=RetainedCloneBinding::close_one(if self.original.is_some(){&mut self.original}else{&mut self.authored},child(g))?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(receipt(step.progress()))}else{RetainedCloneStep::Progress(receipt(step.progress()))})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.active.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.authored.is_none()&&self.original.is_none()}
}
impl<R:Sync+'static,P:RetainedRecordChange<R>> Drop for RetainedRecordChangeOwner<R,P>{fn drop(&mut self){let empty=self.active.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.authored.is_none()&&self.original.is_none();assert!(std::thread::panicking()||empty,"original record result/child/source custody requires explicit closure");if empty{unsafe{ManuallyDrop::drop(&mut self.output);}}}}
#[cfg(test)] #[path="🧪️tests/🦀️.rs"] mod tests;
