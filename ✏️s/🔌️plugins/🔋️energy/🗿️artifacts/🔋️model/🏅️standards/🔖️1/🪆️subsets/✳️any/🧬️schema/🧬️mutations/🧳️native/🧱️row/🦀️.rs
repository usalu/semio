//! 🧱️ A prepared original row replaces one persistent page path before its sealed model branch is published.
use crate::{snapshot::{original::EnergyModelSnapshot,state::fields::RetainedBranchProjection},diff::original::EnergyModelDiff,scalar::{ScalarRowPath,CursorClose,cursor_demand,close_cursor,birth,invalid,scalar,nested,child,add,tag,receipt}};
use semio_framework_value::{RetirementDemand,ValueError,shared_issuance::SealedShared,retirement::RetireOwned,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_persistent::list::persistent::{PersistentList,PersistentSet,PersistentSplice,PersistentSpliceKind};
use semio_framework_value_snapshot_projection::snapshot_projection::SnapshotProjection;
use protocol::mutation::retained::retained_mutation_demand_fits;
use std::mem::{size_of,ManuallyDrop};
#[path="📤️output/🦀️.rs"]mod output;pub use output::{BranchOutput,DirectBranchOutput,NestedBranchPath,NestedBranchOutput,ScheduleBranchOutput,SiteBranchOutput,GroundBranchOutput,RunPeriodBranchOutput};
#[path="📋️change/🦀️.rs"]mod change;pub use change::RowChange;pub(crate) use change::RowListPreparation;
/// 🧭️ The authored row path supplies the exact persistent collection and owning sealed branch.
pub trait RowBranchPath:Send+Sync+'static{
 type Row:RetireOwned+Send+Sync;
 type Branch:SnapshotProjection<Output=Box<Self::Branch>>+Send+Sync;
 fn rows(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>;
 fn branch(original:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>;
 fn rows_mut(branch:&mut Self::Branch)->&mut PersistentList<Self::Row>;
 type Output:crate::row::BranchOutput<Self::Branch>;
 fn output()->Self::Output;
}
/// 🔢️ Existing scalar schema paths use the same original row publication owner.
pub struct ScalarBranch<S>(std::marker::PhantomData<fn()->S>);
impl<S:ScalarRowPath> RowBranchPath for ScalarBranch<S>{type Row=S::Row;type Branch=S::Branch;type Output=S::Output;fn rows(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,PersistentList<Self::Row>>{S::rows(o)}fn branch(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Branch>{S::branch(o)}fn rows_mut(b:&mut Self::Branch)->&mut PersistentList<Self::Row>{S::rows_mut(b)}fn output()->Self::Output{S::output()}}
/// 🧳️ Original row, document bindings and displaced branch paths remain separately closeable.
pub struct RowBranchPreparation<P:RowBranchPath>{
 rank:usize,kind:RowChange,phase:u8,closing:bool,original:Option<RetainedCloneBinding>,row:Option<RetainedCloneBinding>,
 output:Option<Box<P::Output>>,branch_value:ManuallyDrop<Option<SealedShared<P::Branch>>>,set:Option<Box<RowListPreparation<P::Row>>>,branch:Option<Box<RetainedBranchProjection<P::Branch>>>,
 next:ManuallyDrop<Option<PersistentList<P::Row>>>,previous:ManuallyDrop<Option<PersistentList<P::Row>>>,decision:ManuallyDrop<Option<EnergyModelDiff>>,retirement:RetainedCloneClose,
}
impl<P:RowBranchPath> RowBranchPreparation<P>{
 pub fn new(rank:usize)->Self{Self{rank,kind:RowChange::Set,phase:0,closing:false,original:None,row:None,output:None,branch_value:ManuallyDrop::new(None),set:None,branch:None,next:ManuallyDrop::new(None),previous:ManuallyDrop::new(None),decision:ManuallyDrop::new(None),retirement:Default::default()}}
 /// 🌱️ A ranked insertion retains the same original authored row through every page split.
 pub fn insert(rank:usize)->Self{let mut value=Self::new(rank);value.kind=RowChange::Insert;value}
 /// ✂️ A ranked removal rebuilds its actual original collection without a replacement row.
 pub fn remove(rank:usize)->Self{let mut value=Self::new(rank);value.kind=RowChange::Remove;value}
 pub fn ready(&self)->bool{!self.closing&&self.phase==16}
 fn check(&self,row:Option<RetainedCloneRef<'_,SealedShared<P::Row>>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>)->Result<(),ValueError>{if self.kind!=RowChange::Remove{row.ok_or_else(invalid)?.check_binding(&self.row)?;}original.check_binding(&self.original)?;Ok(())}
 fn demand_original(&self,row:Option<RetainedCloneRef<'_,SealedShared<P::Row>>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.phase>=16{return Err(invalid());}self.check(row,original)?;match self.phase{
  0=>nested(scalar(original.binding_copy_bytes()),1),1=>if self.kind==RowChange::Remove{Ok(scalar(1))}else{nested(scalar(row.ok_or_else(invalid)?.binding_copy_bytes()),1)},2=>Ok(birth::<RowListPreparation<P::Row>>()),
  3=>nested(self.set.as_ref().ok_or_else(invalid)?.demand(P::rows(original),row)?,1),
  4=>nested(self.set.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<PersistentList<P::Row>>>()+1),5=>cursor_demand(&self.set,body),
  6=>Ok(birth::<RetainedBranchProjection<P::Branch>>()),
  7|9=>nested(self.branch.as_ref().ok_or_else(invalid)?.demand(P::branch(original),body.saturating_sub(1))?,1),
  8=>nested(RetainedBranchProjection::<P::Branch>::exchange_demand::<PersistentList<P::Row>>(),1),
  10=>nested(self.branch.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<SealedShared<P::Branch>>>()+1),11=>cursor_demand(&self.branch,body),
  12=>Ok(birth::<P::Output>()),13=>nested(self.output.as_ref().ok_or_else(invalid)?.demand(&self.branch_value,original,body.saturating_sub(1))?,1),14=>nested(self.output.as_ref().ok_or_else(invalid)?.take_demand()?,2*size_of::<Option<EnergyModelDiff>>()+1),15=>cursor_demand(&self.output,body),_=>Err(invalid())}}
 fn advance_original(&mut self,row:Option<RetainedCloneRef<'_,SealedShared<P::Row>>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand_original(row,original,body)?;if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}let mut p=receipt(d);match self.phase{
  0=>{p=add(original.bind(&mut self.original,child(g,1))?.ok_or_else(invalid)?,1)?;self.phase=1;},1=>{if self.kind!=RowChange::Remove{p=add(row.ok_or_else(invalid)?.bind(&mut self.row,child(g,1))?.ok_or_else(invalid)?,1)?;}self.phase=2;},
  2=>{self.set=Some(Box::new(RowListPreparation::new(self.kind,self.rank)));self.phase=3;},
  3=>{let set=self.set.as_mut().ok_or_else(invalid)?;p=add(set.advance(P::rows(original),row,child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if set.output_ready(){self.phase=4;}},
  4=>{let extra=2*size_of::<Option<PersistentList<P::Row>>>()+1;let set=self.set.as_mut().ok_or_else(invalid)?;let(next,progress)=set.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.next=Some(next);set.begin_close();self.phase=5;p=add(progress,extra)?;},
  5=>{p=close_cursor(&mut self.set,g)?;if self.set.is_none(){self.phase=6;}},6=>{self.branch=Some(Box::new(RetainedBranchProjection::new()));self.phase=7;},
  7|9=>{let stage=self.branch.as_mut().ok_or_else(invalid)?;p=add(stage.advance(P::branch(original),body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if self.phase==7&&stage.raw_ready(){self.phase=8;}else if self.phase==9&&stage.ready(){self.phase=10;}},
  8=>{let stage=self.branch.as_mut().ok_or_else(invalid)?;let(_,progress)=stage.exchange(P::rows_mut,&mut self.next,&mut self.previous,child(g,1)).map_err(|e|tag(e,1))?;stage.begin_seal()?;self.phase=9;p=add(progress,1)?;},
  10=>{let extra=2*size_of::<Option<SealedShared<P::Branch>>>()+1;let stage=self.branch.as_mut().ok_or_else(invalid)?;let(branch,progress)=stage.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.branch_value=Some(branch);stage.begin_close();self.phase=11;p=add(progress,extra)?;},
  11=>{p=close_cursor(&mut self.branch,g)?;if self.branch.is_none(){self.phase=12;}},12=>{self.output=Some(Box::new(P::output()));self.phase=13;},
  13=>{let output=self.output.as_mut().ok_or_else(invalid)?;p=add(output.advance(&mut self.branch_value,original,body.saturating_sub(1),child(g,1)).map_err(|e|tag(e,1))?.progress(),1)?;if output.ready(){self.phase=14;}},
  14=>{let extra=2*size_of::<Option<EnergyModelDiff>>()+1;let output=self.output.as_mut().ok_or_else(invalid)?;let(diff,progress)=output.take(child(g,extra)).map_err(|e|tag(e,extra))?.ok_or_else(invalid)?;*self.decision=Some(diff);output.begin_close();self.phase=15;p=add(progress,extra)?;},15=>{p=close_cursor(&mut self.output,g)?;if self.output.is_none(){self.phase=16;}},_=>return Err(invalid())}semio_framework_value::retained_clone::admit_retained_clone_progress(g,p,"original Energy row branch publication")?;Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 /// 📏️ Replacement and insertion quote the actual authored row as well as the original collection.
 pub fn demand(&self,row:RetainedCloneRef<'_,SealedShared<P::Row>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{self.demand_original(Some(row),original,body)}
 /// 🧵️ An authored immutable row supplies exactly the existing replacement or insertion source.
 pub fn advance(&mut self,row:RetainedCloneRef<'_,SealedShared<P::Row>>,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.advance_original(Some(row),original,body,g)}
 /// ✂️ Removal borrows only its actual original collection and retained rank.
 pub fn demand_remove(&self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize)->Result<RetirementDemand,ValueError>{if self.kind!=RowChange::Remove{return Err(invalid());}self.demand_original(None,original,body)}
 /// 🧵️ No replacement row is invented or sealed for a removal.
 pub fn advance_remove(&mut self,original:RetainedCloneRef<'_,EnergyModelSnapshot>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.kind!=RowChange::Remove{return Err(invalid());}self.advance_original(None,original,body,g)}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready()||self.decision.is_none(){return Err(invalid());}Ok(scalar(2*size_of::<Option<EnergyModelDiff>>()+1))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<Option<(EnergyModelDiff,RetainedCloneProgress)>,ValueError>{let d=self.take_demand()?;if !retained_mutation_demand_fits(d,g){return Ok(None);}self.phase=17;Ok(Some((self.decision.take().ok_or_else(invalid)?,receipt(d))))}
 pub fn begin_close(&mut self){self.closing=true;if let Some(c)=self.output.as_mut(){c.begin_close();}if let Some(c)=self.set.as_mut(){c.begin_close();}if let Some(c)=self.branch.as_mut(){c.begin_close();}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid());}if self.output.is_some(){return cursor_demand(&self.output,body);}if self.branch.is_some(){return cursor_demand(&self.branch,body);}if self.set.is_some(){return cursor_demand(&self.set,body);}if !self.retirement.is_empty()||self.decision.is_some(){return self.retirement.next_owner_demands::<EnergyModelDiff>(self.decision.is_some(),&None,body);}if self.branch_value.is_some(){return self.retirement.next_owner_demands::<SealedShared<P::Branch>>(true,&None,body);}if self.next.is_some()||self.previous.is_some(){return self.retirement.next_owner_demands::<PersistentList<P::Row>>(true,&None,body);}RetainedCloneBinding::close_demand(if self.original.is_some(){&self.original}else{&self.row},body)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.close_demand(g.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if !retained_mutation_demand_fits(d,g){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.output.is_some(){return close_cursor(&mut self.output,g).map(RetainedCloneStep::Progress);}if self.branch.is_some(){return close_cursor(&mut self.branch,g).map(RetainedCloneStep::Progress);}if self.set.is_some(){return close_cursor(&mut self.set,g).map(RetainedCloneStep::Progress);}if !self.retirement.is_empty(){return self.retirement.step_granted(g).map(|s|RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.decision,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.branch_value,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.next,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}if let Some(s)=self.retirement.begin_granted(&mut self.previous,g)?{return Ok(RetainedCloneStep::Progress(s.progress()));}let s=RetainedCloneBinding::close_one(if self.original.is_some(){&mut self.original}else{&mut self.row},g)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(s.progress())}else{RetainedCloneStep::Progress(s.progress())})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.branch_value.is_none()&&self.set.is_none()&&self.branch.is_none()&&self.next.is_none()&&self.previous.is_none()&&self.decision.is_none()&&self.retirement.is_empty()&&self.original.is_none()&&self.row.is_none()}
}
impl<P:RowBranchPath> CursorClose for RowBranchPreparation<P>{fn close_demand(&self,b:usize)->Result<RetirementDemand,ValueError>{Self::close_demand(self,b)}fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Self::close_step(self,g)}fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}}
impl<P:RowBranchPath> Drop for RowBranchPreparation<P>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"original row branch publication retains path custody");if empty{unsafe{ManuallyDrop::drop(&mut self.branch_value);ManuallyDrop::drop(&mut self.next);ManuallyDrop::drop(&mut self.previous);ManuallyDrop::drop(&mut self.decision);}}}}

/// 🚦️ Row policies report authored semantic outcomes without creating a branch for refused changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum RowOutcome{Changed,Unchanged,Invalid,InvalidReference,InvalidContext,Duplicate,Missing,Referenced}
impl RowOutcome{pub fn as_str(self)->&'static str{match self{Self::Changed=>"changed",Self::Unchanged=>"no-op",Self::Invalid=>"invalid",Self::InvalidReference=>"invalid-reference",Self::InvalidContext=>"invalid-context",Self::Duplicate=>"duplicate",Self::Missing=>"missing",Self::Referenced=>"referenced"}}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="🌱️admission/🦀️.rs"]mod admission;pub use admission::{RowAdmission,RowAdmissionScan};
