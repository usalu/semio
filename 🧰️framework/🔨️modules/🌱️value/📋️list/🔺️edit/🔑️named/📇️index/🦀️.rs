//! 📇️ One original key admission publishes immutable sorted ranks for repeated sparse planning.
use super::{NamedEditPlan,NamedEditSource,NamedEditLane,NamedEditStep,NamedEditFault,NamedRankedSource,invalid};
use crate::{IndexedEditShape,RetirementDemand,ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,RetirementStep},shared_issuance::{SealedShared,SharedIssuer},retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
#[path="♻️transfer/🦀️.rs"]mod transfer;
pub use transfer::KeyPreservingPersistentSet;
#[path="🌳️root/🦀️.rs"]mod root;
pub use root::{NamedPersistentRoot,NamedPersistentRootPreparation,NamedRecordKey,NamedPersistentRootReceiver};
fn fixed(bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
fn paid(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
/// 🔐️ Private immutable coordinates retain their genuine original key root and sorted directory issuer.
pub struct AdmittedNamedKeyIndex{original:ManuallyDrop<Option<RetainedCloneBinding>>,directory:ManuallyDrop<Option<SealedShared<Vec<usize>>>>,retirement:RetainedCloneClose,len:usize,projection:usize,closing:bool}
impl AdmittedNamedKeyIndex{
 pub fn len(&self)->usize{self.len}
 pub fn is_empty(&self)->bool{self.len==0}
 pub fn ready(&self)->bool{!self.closing&&self.original.is_some()&&self.directory.is_some()}
 pub(super) fn order(&self)->&[usize]{self.directory.as_ref().expect("admitted original named key directory").get()}
 pub(super) fn same_original(&self,other:&Self)->bool{self.ready()&&other.ready()&&self.len==other.len&&self.projection==other.projection&&self.directory.as_ref().unwrap().identity()==other.directory.as_ref().unwrap().identity()}
 /// 🪪️ Equal content does not authorize a different original root or different key projection selector.
 pub fn check_source(&self,source:&impl NamedRankedSource)->Result<(),ValueError>{if !self.ready()||self.len!=source.len()||self.projection!=source.key_projection()||!source.original().check_binding(&self.original)?{return Err(invalid());}Ok(())}
 /// 🎟️ Reuse duplicates two fixed original authorities and performs no base scan or allocation.
 pub fn project_demand(&self,source:&impl NamedRankedSource)->Result<RetirementDemand,ValueError>{self.check_source(source)?;Ok(fixed(size_of::<Self>()*2+source.original().binding_copy_bytes()+size_of::<SealedShared<Vec<usize>>>()))}
 pub fn project(&self,source:&impl NamedRankedSource,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let d=self.project_demand(source)?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let mut original=None;source.original().bind(&mut original,grant)?.ok_or_else(invalid)?;let directory=self.directory.as_ref().unwrap().try_duplicate(grant)?.0;Ok(Some((Self{original:ManuallyDrop::new(original),directory:ManuallyDrop::new(Some(directory)),retirement:Default::default(),len:self.len,projection:self.projection,closing:false},paid(d))))}
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if self.directory.is_some()||!self.retirement.is_empty(){return self.retirement.next_owner_demands::<SealedShared<Vec<usize>>>(self.directory.is_some(),&None,body);}RetainedCloneBinding::close_demand(&self.original,body)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.directory,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}RetainedCloneBinding::close_one(&mut self.original,grant)}
 pub fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.directory.is_none()&&self.retirement.is_empty()}
}
impl Drop for AdmittedNamedKeyIndex{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"immutable named key index requires original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.directory);}}}}
struct IndexRetirement{index:AdmittedNamedKeyIndex}
impl RetireOwned for AdmittedNamedKeyIndex{fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(IndexRetirement{index:self})}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<IndexRetirement>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for IndexRetirement{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.index.close_demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.index.close_step(grant){Ok(s)=>RetirementStep::Progress(s.progress()),Err(e)=>RetirementStep::Failure(e)}}
 fn terminal_is_empty(&self)->bool{self.index.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
struct AdmissionSource<'a,S:NamedRankedSource>{source:&'a S}
impl<S:NamedRankedSource> NamedEditSource for AdmissionSource<'_,S>{
 fn shape(&self)->IndexedEditShape{IndexedEditShape{base_len:self.source.len(),removed:0,modified:0,added:0}}
 fn key_demand(&self,lane:NamedEditLane,ordinal:usize)->Result<RetirementDemand,ValueError>{if lane!=NamedEditLane::Base{return Err(invalid());}self.source.rank_demand(ordinal)}
 fn observe_key(&self,lane:NamedEditLane,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(&str,RetainedCloneProgress)>,ValueError>{if lane!=NamedEditLane::Base{return Err(invalid());}Ok(self.source.observe_key(ordinal,grant)?.map(|(key,p)|(key.get(),p)))}
 fn added_index(&self,_:usize)->usize{unreachable!("key admission declares no added coordinates")}
}
/// 📥️ Admission reuses the genuine named planner and closes it before sealing its sorted original directory.
pub struct NamedKeyIndexPreparation{original:ManuallyDrop<Option<RetainedCloneBinding>>,order:ManuallyDrop<Option<Vec<usize>>>,directory:ManuallyDrop<Option<SealedShared<Vec<usize>>>>,plan:Option<NamedEditPlan>,retirement:RetainedCloneClose,len:usize,projection:usize,phase:u8,fault:Option<NamedEditFault>,closing:bool}
impl NamedKeyIndexPreparation{
 pub fn constructor_demand()->RetirementDemand{fixed(size_of::<Self>()*2)}
 pub fn new(len:usize,grant:RetainedCloneGrant)->Result<Option<Self>,ValueError>{if !retained_clone_grant_admits(grant,Self::constructor_demand()){return Ok(None);}Ok(Some(Self{original:ManuallyDrop::new(None),order:ManuallyDrop::new(None),directory:ManuallyDrop::new(None),plan:None,retirement:Default::default(),len,projection:0,phase:0,fault:None,closing:false}))}
 fn validate(&self,source:&impl NamedRankedSource)->Result<(),ValueError>{if self.closing||self.phase>6||self.fault.is_some()||source.len()!=self.len{return Err(invalid());}if self.original.is_some()&&(self.projection!=source.key_projection()||!source.original().check_binding(&self.original)?){return Err(invalid());}Ok(())}
 pub fn demand(&self,source:&impl NamedRankedSource,body:usize)->Result<RetirementDemand,ValueError>{self.validate(source)?;match self.phase{0=>Ok(fixed(size_of::<Self>()+source.original().binding_copy_bytes())),1=>Ok(fixed(size_of::<NamedEditPlan>()*2)),2=>self.plan.as_ref().ok_or_else(invalid)?.demand(&AdmissionSource{source},body),3=>self.plan.as_ref().ok_or_else(invalid)?.base_take_demand(),4=>if self.plan.as_ref().unwrap().terminal_is_empty(){Ok(fixed(size_of::<NamedEditPlan>()*2))}else{self.plan.as_ref().unwrap().close_demand(body)},5=>Ok(RetirementDemand{copy_bytes:size_of::<Vec<usize>>()*3+size_of::<SealedShared<Vec<usize>>>()*2,capacity_bytes:SealedShared::<Vec<usize>>::birth_bytes(),depth:1,..Default::default()}),6=>Ok(Default::default()),_=>Err(invalid())}}
 pub fn advance(&mut self,source:&impl NamedRankedSource,grant:RetainedCloneGrant)->Result<NamedEditStep,ValueError>{let d=self.demand(source,grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(NamedEditStep::Progress(Default::default()));}let mut p=paid(d);match self.phase{
  0=>{source.original().bind(&mut self.original,grant)?.ok_or_else(invalid)?;self.projection=source.key_projection();self.phase=1;},
  1=>{self.plan=Some(NamedEditPlan::new(AdmissionSource{source}.shape()));self.phase=2;},
  2=>{let step=self.plan.as_mut().unwrap().advance(&AdmissionSource{source},grant)?;p=step.progress();match step{NamedEditStep::Admitted(_)=>self.phase=3,NamedEditStep::Refused(fault,_)=>{self.fault=Some(fault);return Ok(NamedEditStep::Refused(fault,p));},_=>{}}},
  3=>{*self.order=Some(self.plan.as_mut().unwrap().take_base_order(grant)?.ok_or_else(invalid)?.0);self.plan.as_mut().unwrap().begin_close();self.phase=4;},
  4=>{if self.plan.as_ref().unwrap().terminal_is_empty(){self.plan=None;self.phase=5;}else{p=self.plan.as_mut().unwrap().close_step(grant)?.progress();}},
  5=>{let rows=self.order.take().ok_or_else(invalid)?;match SealedShared::admit(rows,SharedIssuer::owned(),grant){Ok((directory,birth))=>{*self.directory=Some(directory);p.retained_capacity_bytes=birth.retained_capacity_bytes;self.phase=6;},Err((e,rows))=>{*self.order=Some(rows);return Err(e);}}},
  6=>return Ok(NamedEditStep::Admitted(Default::default())),_=>return Err(invalid()),
 }Ok(if self.phase==6{NamedEditStep::Admitted(p)}else{NamedEditStep::Progress(p)})}
 pub fn output_ready(&self)->bool{!self.closing&&self.phase==6&&self.fault.is_none()&&self.directory.is_some()}
 pub fn fault(&self)->Option<NamedEditFault>{self.fault}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid());}Ok(fixed(size_of::<Self>()+size_of::<AdmittedNamedKeyIndex>()*2))}
 pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(AdmittedNamedKeyIndex,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(grant,d){return Ok(None);}let index=AdmittedNamedKeyIndex{original:ManuallyDrop::new(self.original.take()),directory:ManuallyDrop::new(self.directory.take()),retirement:Default::default(),len:self.len,projection:self.projection,closing:false};self.phase=7;Ok(Some((index,paid(d))))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(plan)=self.plan.as_mut(){plan.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if let Some(plan)=self.plan.as_ref(){return if plan.terminal_is_empty(){Ok(fixed(size_of::<NamedEditPlan>()*2))}else{plan.close_demand(body)};}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Vec<usize>>(false,&None,body);}if self.order.is_some(){return self.retirement.next_owner_demands::<Vec<usize>>(true,&None,body);}if self.directory.is_some(){return self.retirement.next_owner_demands::<SealedShared<Vec<usize>>>(true,&None,body);}RetainedCloneBinding::close_demand(&self.original,body)}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if let Some(plan)=self.plan.as_mut(){if plan.terminal_is_empty(){self.plan=None;return Ok(RetainedCloneStep::Progress(paid(d)));}return plan.close_step(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.order,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.directory,grant)?{return Ok(RetainedCloneStep::Progress(s.progress()));}RetainedCloneBinding::close_one(&mut self.original,grant)}
 pub fn terminal_is_empty(&self)->bool{self.plan.is_none()&&self.original.is_none()&&self.order.is_none()&&self.directory.is_none()&&self.retirement.is_empty()}
}
impl Drop for NamedKeyIndexPreparation{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"named key admission requires original funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.order);ManuallyDrop::drop(&mut self.directory);}}}}
/// 🧭️ The same public named planner receives either original Vec ranks or immutable page ranks.
pub struct IndexedNamedSource<'a,S:NamedRankedSource,E:NamedEditSource>{source:&'a S,edits:&'a E,index:&'a AdmittedNamedKeyIndex}
impl<'a,S:NamedRankedSource,E:NamedEditSource> IndexedNamedSource<'a,S,E>{pub fn new(source:&'a S,edits:&'a E,index:&'a AdmittedNamedKeyIndex)->Result<Self,ValueError>{index.check_source(source)?;if edits.shape().base_len!=source.len(){return Err(invalid());}Ok(Self{source,edits,index})}}
impl<S:NamedRankedSource,E:NamedEditSource> NamedEditSource for IndexedNamedSource<'_,S,E>{
 fn shape(&self)->IndexedEditShape{self.edits.shape()}
 fn key_demand(&self,lane:NamedEditLane,ordinal:usize)->Result<RetirementDemand,ValueError>{self.index.check_source(self.source)?;if lane==NamedEditLane::Base{self.source.rank_demand(ordinal)}else{self.edits.key_demand(lane,ordinal)}}
 fn observe_key(&self,lane:NamedEditLane,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(&str,RetainedCloneProgress)>,ValueError>{self.index.check_source(self.source)?;if lane==NamedEditLane::Base{Ok(self.source.observe_key(ordinal,grant)?.map(|(key,p)|(key.get(),p)))}else{self.edits.observe_key(lane,ordinal,grant)}}
 fn added_index(&self,ordinal:usize)->usize{self.edits.added_index(ordinal)}
 fn admitted_base(&self)->Option<&AdmittedNamedKeyIndex>{Some(self.index)}
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
