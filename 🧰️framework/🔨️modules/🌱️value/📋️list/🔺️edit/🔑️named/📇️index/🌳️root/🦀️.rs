//! 🌳️ Independent immutable named roots retain their original row source and admitted sorted directory.
use super::*;
use crate::{list::persistent::PersistentList,named_edit::PersistentRankedKeys,retained_clone::RetainedCloneSource};
#[path="🛬️receive/🦀️.rs"]mod receiving;
pub use receiving::{NamedRecordKey,NamedPersistentRootReceiver};
/// 🪢️ Immutable payload contains no mutable directory retirement or projection-binding cursor.
pub struct NamedPersistentRoot<T:RetireOwned+Send+Sync+'static>{source:ManuallyDrop<Option<RetainedCloneSource<PersistentList<T>>>>,directory:ManuallyDrop<Option<SealedShared<Vec<usize>>>>,len:usize,key:fn(&T)->&str}
impl<T:RetireOwned+Send+Sync+'static> NamedPersistentRoot<T>{
 pub fn constructor_demand()->RetirementDemand{fixed(size_of::<NamedPersistentRootPreparation<T>>()*2)}
 pub fn admit_parts(source:&mut Option<RetainedCloneSource<PersistentList<T>>>,index:&mut Option<AdmittedNamedKeyIndex>,key:fn(&T)->&str,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(NamedPersistentRootPreparation<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=Self::constructor_demand();if !retained_clone_grant_admits(grant,d){return Ok(None);}index.as_ref().ok_or_else(invalid)?.check_source(&PersistentRankedKeys{original:source.as_ref().ok_or_else(invalid)?.try_borrow()?,key})?;work+=1;Ok(Some((NamedPersistentRootPreparation{source:ManuallyDrop::new(source.take()),index:ManuallyDrop::new(index.take()),key,closing:false,ready:false},paid(d))))})();crate::RetainedWorkOutcome{result,work}}
 pub fn ranked(&self)->Result<PersistentRankedKeys<'_,T>,ValueError>{Ok(PersistentRankedKeys{original:self.source.as_ref().ok_or_else(invalid)?.try_borrow()?,key:self.key})}
 /// 📖️ An independently retained row reader keeps the original source and projection after named-root closure.
 pub fn project_rows(&self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(crate::retained_clone::RetainedOwnedProjection<PersistentList<T>>,RetainedCloneProgress)>{match self.source.as_ref(){Some(source)=>source.retain_original(grant),None=>crate::RetainedWorkOutcome{result:Err(invalid()),work:0}}}
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
 pub fn project_index_demand(&self)->Result<RetirementDemand,ValueError>{let original=self.ranked()?.original;Ok(fixed(size_of::<AdmittedNamedKeyIndex>()*2+original.binding_copy_bytes()+size_of::<SealedShared<Vec<usize>>>()))}
 pub fn project_index(&self,output:&mut Option<AdmittedNamedKeyIndex>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{
 let d=self.project_index_demand()?;let ranked=self.ranked()?;let directory=self.directory.as_ref().ok_or_else(invalid)?;
 if let Some(owner)=output.as_ref(){if owner.closing||owner.len!=self.len||owner.projection!=self.key as usize||!ranked.original.check_binding(&owner.original)?||owner.directory.as_ref().is_some_and(|actual|actual.identity()!=directory.identity()){return Err(invalid())}if owner.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}}
 if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
 if output.is_none(){*output=Some(AdmittedNamedKeyIndex{original:ManuallyDrop::new(None),directory:ManuallyDrop::new(None),retirement:Default::default(),len:self.len,projection:self.key as usize,closing:false});work+=1;}
 let owner=output.as_mut().ok_or_else(||ValueError::invariant("output absent"))?;if owner.original.is_none(){take_work!(work,ranked.original.bind(&mut owner.original,grant)).ok_or_else(invalid)?;return Ok(RetainedCloneStep::Progress(paid(d)))}
 let(alias,_)=take_work!(work,directory.try_duplicate(grant));*owner.directory=Some(alias);Ok(RetainedCloneStep::Complete(paid(d)))
 })();crate::RetainedWorkOutcome{result,work}}

 fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.directory.is_none()}
}
impl<T:RetireOwned+Send+Sync+'static> Drop for NamedPersistentRoot<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"independent named root requires its original owned retirement cursor");if empty{unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.directory);}}}}
/// 📥️ Source validation and paid binding closure precede immutable root publication.
pub struct NamedPersistentRootPreparation<T:RetireOwned+Send+Sync+'static>{source:ManuallyDrop<Option<RetainedCloneSource<PersistentList<T>>>>,index:ManuallyDrop<Option<AdmittedNamedKeyIndex>>,key:fn(&T)->&str,closing:bool,ready:bool}
impl<T:RetireOwned+Send+Sync+'static> NamedPersistentRootPreparation<T>{
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.ready{return Err(invalid());}RetainedCloneBinding::close_demand(&self.index.as_ref().ok_or_else(invalid)?.original,body)}
 pub fn advance(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{let d=self.demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}let index=self.index.as_mut().ok_or_else(invalid)?;let step=take_work!(work,RetainedCloneBinding::close_one(&mut index.original,grant));if index.original.is_none(){self.ready=true;work+=1;}Ok(if self.ready{RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})})();crate::RetainedWorkOutcome{result,work}}
 pub fn output_ready(&self)->bool{!self.closing&&self.ready&&self.source.is_some()&&self.index.is_some()}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(size_of::<NamedPersistentRoot<T>>()*2+size_of::<AdmittedNamedKeyIndex>()))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(NamedPersistentRoot<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let index=self.index.as_mut().ok_or_else(invalid)?;work+=1;let root=NamedPersistentRoot{source:ManuallyDrop::new(self.source.take()),directory:ManuallyDrop::new(index.directory.take()),len:index.len,key:self.key};*self.index=None;Ok(Some((root,paid(d))))})();crate::RetainedWorkOutcome{result,work}}
 pub fn begin_close(&mut self){self.closing=true;if let Some(index)=self.index.as_mut(){index.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if let Some(index)=self.index.as_ref(){return if index.terminal_is_empty(){Ok(fixed(size_of::<AdmittedNamedKeyIndex>()*2))}else{index.close_demand(body)};}if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed(size_of::<RetainedCloneSource<PersistentList<T>>>()*2))}else{source.next_close_demand(body)};}Ok(Default::default())}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{
 if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
 let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
 if let Some(index)=self.index.as_mut(){if index.terminal_is_empty(){*self.index=None;work+=1;return Ok(RetainedCloneStep::Progress(paid(d)))}let step=take_work!(work,index.close_step(grant));return Ok(RetainedCloneStep::Progress(step.progress()))}
 if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){*self.source=None;work+=1;return Ok(RetainedCloneStep::Progress(paid(d)))}let step=take_work!(work,source.close_step(grant));return Ok(RetainedCloneStep::Progress(step.progress()))}
 Ok(RetainedCloneStep::Complete(Default::default()))
 })();crate::RetainedWorkOutcome{result,work}}

 pub fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.index.is_none()}
}
impl<T:RetireOwned+Send+Sync+'static> Drop for NamedPersistentRootPreparation<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"independent named root preparation requires funded original closure");if empty{unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.index);}}}}
struct RootRetirement<T:RetireOwned+Send+Sync+'static>{root:NamedPersistentRoot<T>,retirement:RetainedCloneClose}
impl<T:RetireOwned+Send+Sync+'static> RetireOwned for NamedPersistentRoot<T>{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(RootRetirement{root:self,retirement:Default::default()})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<RootRetirement<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:RetireOwned+Send+Sync+'static> RetirementCursor for RootRetirement<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.retirement.is_empty()||self.root.directory.is_some(){return self.retirement.next_owner_demands::<SealedShared<Vec<usize>>>(self.root.directory.is_some(),&None,body);}if let Some(source)=self.root.source.as_ref(){return if source.terminal_is_empty(){Ok(fixed(size_of::<RetainedCloneSource<PersistentList<T>>>()*2))}else{source.next_close_demand(body)};}Ok(Default::default())}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{let mut work=0;let result=(||{
 if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
 let d=self.next_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
 if !self.retirement.is_empty(){let step=take_work!(work,self.retirement.step_granted(g));return Ok(RetainedCloneStep::Progress(step.progress()))}
 if let Some(step)=take_work!(work,self.retirement.begin_granted(&mut self.root.directory,g)){return Ok(RetainedCloneStep::Progress(step.progress()))}
 if let Some(source)=self.root.source.as_mut(){if source.terminal_is_empty(){*self.root.source=None;work+=1;return Ok(RetainedCloneStep::Progress(paid(d)))}let step=take_work!(work,source.close_step(g));return Ok(RetainedCloneStep::Progress(step.progress()))}
 Ok(RetainedCloneStep::Complete(Default::default()))
 })();match result{Ok(RetainedCloneStep::Complete(p))if p==Default::default()=>RetirementStep::Complete(work),Ok(step)=>RetirementStep::Progress(step.progress(),work),Err(error)=>RetirementStep::Failure(error,work)}}

 fn terminal_is_empty(&self)->bool{self.root.terminal_is_empty()&&self.retirement.is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
