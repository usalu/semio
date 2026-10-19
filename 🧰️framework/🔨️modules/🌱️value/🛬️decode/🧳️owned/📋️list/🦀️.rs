//! 🧳️ Original typed list children become persistent native rows through mandatory receiving and funded ownership transfers.
use super::{NativeReceive,NativeReceiver};
use crate::{DslValue,NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,list::persistent::{PersistentList,PersistentListBuilder},retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits},shared_issuance::{SealedShared,SharedIssuer}};
use std::mem::{ManuallyDrop,size_of};
/// 👓️ Existing original native array owners lend constant-cost cardinality and ranked typed child references.
pub trait NativeListSource:Sync+'static{type Row:Sync+'static;fn row_count(&self)->Result<usize,ValueError>;fn row(&self,index:usize)->Result<&Self::Row,ValueError>;}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native persistent receiving requires the exact original ranked typed list")}
fn phase_error()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native persistent receiving has closed or transferred its required owner")}
impl<I:Sync+'static> NativeListSource for Vec<I>{type Row=I;fn row_count(&self)->Result<usize,ValueError>{Ok(self.len())}fn row(&self,index:usize)->Result<&I,ValueError>{self.get(index).ok_or_else(invalid)}}
impl NativeListSource for DslValue{type Row=Self;fn row_count(&self)->Result<usize,ValueError>{match self{Self::Array(rows)=>Ok(rows.len()),_=>Err(invalid())}}fn row(&self,index:usize)->Result<&Self,ValueError>{match self{Self::Array(rows)=>rows.get(index).ok_or_else(invalid),_=>Err(invalid())}}}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Count,Initialize,RowBirth,RowAdvance,RowTake,RowClose,RowSeal,RowAdopt,Build,Finish,FinalTake,Ready,Spent}
/// 🌳️ One required child receiver and a fixed-height page builder retain every original partial owner.
pub struct NativePersistentListReceiver<I:NativeListSource,T:NativeReceive<I::Row>+Sync>{
 binding:Option<RetainedCloneBinding>,builder:ManuallyDrop<Option<Box<PersistentListBuilder<T>>>>,child:ManuallyDrop<Option<T::Receiver>>,raw:ManuallyDrop<Option<T>>,sealed:ManuallyDrop<Option<SealedShared<T>>>,output:ManuallyDrop<Option<PersistentList<T>>>,retirement:RetainedCloneClose,count:usize,index:usize,phase:Phase,closing:bool,input:std::marker::PhantomData<fn()->I>,
}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(phase_error)?;d.depth=d.depth.checked_add(1).ok_or_else(phase_error)?;Ok(d)}
fn child_grant(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g}}
fn enlarge(mut p:RetainedCloneProgress,extra:usize)->Result<RetainedCloneProgress,ValueError>{p.copied_bytes=p.copied_bytes.checked_add(extra).ok_or_else(phase_error)?;p.copied_items=p.copied_items.max(usize::from(extra>0));Ok(p)}
fn retained_error(error:ValueError,extra:usize)->ValueError{let p=error.retained_progress();match enlarge(p,extra){Ok(p)=>error.with_retained_progress(p),Err(error)=>error.with_retained_progress(p)}}
impl<I:NativeListSource,T:NativeReceive<I::Row>+Sync> NativePersistentListReceiver<I,T>{
 fn metadata()->usize{size_of::<usize>()*4+size_of::<Phase>()}
 fn row_source<'a>(&self,original:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,I::Row>,ValueError>{original.get().row(self.index)?;let index=self.index;Ok(original.project(index,move|input|input.row(index).expect("original validated native list child")))}
 fn builder(&self)->Result<&PersistentListBuilder<T>,ValueError>{self.builder.as_deref().ok_or_else(phase_error)}
 fn child(&self)->Result<&T::Receiver,ValueError>{self.child.as_ref().ok_or_else(phase_error)}
 fn next_row_phase(&self)->Result<Phase,ValueError>{Ok(if !self.builder()?.accepts_row(){Phase::Build}else if self.index<self.count{Phase::RowBirth}else{Phase::Finish})}
}
impl<I:NativeListSource,T:NativeReceive<I::Row>+Sync> NativeReceive<I> for PersistentList<T>{
 type Receiver=NativePersistentListReceiver<I,T>;
 fn receive_birth_demand()->RetirementDemand{fixed(size_of::<Self::Receiver>()*2)}
 fn begin_receive(g:RetainedCloneGrant)->Result<(Self::Receiver,RetainedCloneProgress),ValueError>{let d=<Self as NativeReceive<I>>::receive_birth_demand();if !retained_clone_grant_admits(g,d){return Err(phase_error());}Ok((NativePersistentListReceiver{binding:None,builder:ManuallyDrop::new(None),child:ManuallyDrop::new(None),raw:ManuallyDrop::new(None),sealed:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),count:0,index:0,phase:Phase::Bind,closing:false,input:std::marker::PhantomData},receipt(d)))}
}
impl<I:NativeListSource,T:NativeReceive<I::Row>+Sync> NativeReceiver<I,PersistentList<T>> for NativePersistentListReceiver<I,T>{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize)->Result<RetirementDemand,ValueError>{
  if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(phase_error());}original.check_binding(&self.binding)?;let m=Self::metadata();
  match self.phase{
   Phase::Bind=>Ok(fixed(original.binding_copy_bytes()+m)),Phase::Count=>Ok(fixed(m)),
   Phase::Initialize=>{let mut d=PersistentListBuilder::<T>::constructor_demand();d.capacity_bytes=size_of::<PersistentListBuilder<T>>();nested(d,size_of::<Option<Box<PersistentListBuilder<T>>>>()*2+m)},
   Phase::RowBirth=>nested(T::receive_birth_demand(),size_of::<Option<T::Receiver>>()*2+m),
   Phase::RowAdvance=>nested(self.child()?.demands(self.row_source(original)?,body)?,m),
   Phase::RowTake=>nested(self.child()?.take_demand()?,size_of::<Option<T>>()*2+m),
   Phase::RowClose=>if self.child()?.terminal_is_empty(){Ok(fixed(size_of::<Option<T::Receiver>>()*2+m))}else{nested(self.child()?.close_demands(body.saturating_sub(m))?,m)},
   Phase::RowSeal=>nested(crate::native_decoding::NativeOwnedDecodeControl::shared_birth_demand::<T>(),m),
   Phase::RowAdopt=>nested(self.builder()?.adopt_demand()?,size_of::<Option<SealedShared<T>>>()+m),
   Phase::Build=>nested(self.builder()?.demand()?,m),Phase::Finish=>nested(self.builder()?.finish_demand()?,m),
   Phase::FinalTake=>nested(self.builder()?.take_demand()?,size_of::<Option<PersistentList<T>>>()*2+m),_=>Err(phase_error()),
  }
 }
 fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.demands(original,body)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}control.checkpoint()?;let m=Self::metadata();let p=match self.phase{
   Phase::Bind=>{let p=original.bind(&mut self.binding,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-m,..g})?.ok_or_else(phase_error)?;self.phase=Phase::Count;enlarge(p,m)?},
   Phase::Count=>{self.count=original.get().row_count().map_err(|e|e.with_retained_progress(receipt(d)))?;self.phase=Phase::Initialize;receipt(d)},
   Phase::Initialize=>{let extra=size_of::<Option<Box<PersistentListBuilder<T>>>>()*2+m;let builder=PersistentListBuilder::new(self.count,child_grant(g,extra))?.ok_or_else(phase_error)?;control.charge(d.capacity_bytes)?;*self.builder=Some(Box::new(builder));self.phase=if self.count==0{Phase::Finish}else{Phase::RowBirth};receipt(d)},
   Phase::RowBirth=>{let extra=size_of::<Option<T::Receiver>>()*2+m;control.charge(d.capacity_bytes)?;let(child,p)=T::begin_receive(child_grant(g,extra))?;*self.child=Some(child);self.phase=Phase::RowAdvance;enlarge(p,extra)?},
   Phase::RowAdvance=>{let source=self.row_source(original)?;let child=self.child.as_mut().ok_or_else(phase_error)?;let p=child.advance(source,control,body,child_grant(g,m)).map_err(|e|retained_error(e,m))?.progress();if child.output_ready(){self.phase=Phase::RowTake;}enlarge(p,m)?},
   Phase::RowTake=>{let extra=size_of::<Option<T>>()*2+m;control.charge(d.capacity_bytes)?;let child=self.child.as_mut().ok_or_else(phase_error)?;let(row,p)=child.take(child_grant(g,extra))?.ok_or_else(phase_error)?;*self.raw=Some(row);child.begin_close();self.phase=Phase::RowClose;enlarge(p,extra)?},
   Phase::RowClose=>{let child=self.child.as_mut().ok_or_else(phase_error)?;if child.terminal_is_empty(){self.child.take();self.phase=Phase::RowSeal;receipt(d)}else{control.charge(d.capacity_bytes)?;let p=child.close_step(child_grant(g,m)).map_err(|e|retained_error(e,m))?.progress();enlarge(p,m)?}},
   Phase::RowSeal=>{let result=control.receive_shared(&mut self.raw,&mut self.sealed,SharedIssuer::owned());if self.sealed.is_some(){self.phase=Phase::RowAdopt;}if result.is_err()&&self.sealed.is_some(){self.begin_close();}let mut p=result.map_err(|e|retained_error(e,m))?;p.copied_bytes=d.copy_bytes;p},
   Phase::RowAdopt=>{let extra=size_of::<Option<SealedShared<T>>>()+m;let p=self.builder.as_mut().ok_or_else(phase_error)?.adopt(&mut self.sealed,child_grant(g,extra))?.ok_or_else(phase_error)?;self.index+=1;self.phase=self.next_row_phase()?;enlarge(p,extra)?},
   Phase::Build=>{control.charge(d.capacity_bytes)?;let builder=self.builder.as_mut().ok_or_else(phase_error)?;let p=builder.advance(child_grant(g,m)).map_err(|e|retained_error(e,m))?.progress();if builder.output_ready(){self.phase=Phase::FinalTake;}else if builder.accepts_row(){self.phase=if self.index<self.count{Phase::RowBirth}else{Phase::Finish};}enlarge(p,m)?},
   Phase::Finish=>{let p=self.builder.as_mut().ok_or_else(phase_error)?.finish(child_grant(g,m))?.ok_or_else(phase_error)?;self.phase=Phase::Build;enlarge(p,m)?},
   Phase::FinalTake=>{let extra=size_of::<Option<PersistentList<T>>>()*2+m;let(root,p)=self.builder.as_mut().ok_or_else(phase_error)?.take(child_grant(g,extra))?.ok_or_else(phase_error)?;*self.output=Some(root);self.phase=Phase::Ready;enlarge(p,extra)?},_=>return Err(phase_error()),
  };crate::retained_clone::admit_retained_clone_progress(g,p,"original persistent native receiving")?;Ok(if self.output_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(phase_error());}Ok(fixed(size_of::<Option<PersistentList<T>>>()*2+size_of::<Phase>()))}
 fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(PersistentList<T>,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok(None);}let output=self.output.take().ok_or_else(phase_error)?;self.phase=Phase::Spent;Ok(Some((output,receipt(d))))}
 fn begin_close(&mut self){self.closing=true;if let Some(child)=self.child.as_mut(){child.begin_close();}if let Some(builder)=self.builder.as_mut(){builder.begin_close();}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.closing{return Err(phase_error());}let m=Self::metadata();let body=body.saturating_sub(m);
  if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(fixed(size_of::<Option<T::Receiver>>()*2+m))}else{nested(child.close_demands(body)?,m)};}
  if let Some(builder)=self.builder.as_ref(){return if builder.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<Box<PersistentListBuilder<T>>>>()*2+m,release_bytes:size_of::<PersistentListBuilder<T>>(),depth:1,..Default::default()})}else{nested(builder.close_demand(body)?,m)};}
  if !self.retirement.is_empty(){return nested(self.retirement.next_owner_demands::<T>(false,&None,body)?,m);}
  if self.raw.is_some(){return nested(self.retirement.next_owner_demands::<T>(true,&None,body)?,m);}
  if self.sealed.is_some(){return nested(self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body)?,m);}
  if self.output.is_some(){return nested(self.retirement.next_owner_demands::<PersistentList<T>>(true,&None,body)?,m);}
  if self.binding.is_some(){return nested(RetainedCloneBinding::close_demand(&self.binding,body)?,m);}Ok(Default::default())
 }
 fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  let d=self.close_demands(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let m=Self::metadata();
  if let Some(child)=self.child.as_mut(){if child.terminal_is_empty(){self.child.take();return Ok(RetainedCloneStep::Progress(receipt(d)));}return child.close_step(child_grant(g,m)).and_then(|s|enlarge(s.progress(),m)).map(RetainedCloneStep::Progress);}
  if let Some(builder)=self.builder.as_mut(){if builder.terminal_is_empty(){self.builder.take();return Ok(RetainedCloneStep::Progress(receipt(d)));}return builder.close_step(child_grant(g,m)).and_then(|s|enlarge(s.progress(),m)).map(RetainedCloneStep::Progress);}
  let child=child_grant(g,m);let p=if !self.retirement.is_empty(){self.retirement.step_granted(child)?.progress()}else if let Some(s)=self.retirement.begin_granted(&mut self.raw,child)?{s.progress()}else if let Some(s)=self.retirement.begin_granted(&mut self.sealed,child)?{s.progress()}else if let Some(s)=self.retirement.begin_granted(&mut self.output,child)?{s.progress()}else{RetainedCloneBinding::close_one(&mut self.binding,child)?.progress()};let p=enlarge(p,m)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
 }
 fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.builder.is_none()&&self.raw.is_none()&&self.sealed.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:NativeListSource,T:NativeReceive<I::Row>+Sync> Drop for NativePersistentListReceiver<I,T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native persistent receiving retains original physical owners until funded closure");}}
