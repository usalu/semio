//! 📥️ Original owned rows become immutable fixed-fanout pages through separately funded carry turns.
use super::{PersistentList,PERSISTENT_PAGE_FANOUT,fits};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},shared_issuance::SealedShared};
use std::mem::{ManuallyDrop,size_of};
const LEVELS:usize=usize::BITS as usize/4;
type Rows<T>=[Option<SealedShared<T>>;PERSISTENT_PAGE_FANOUT];
type Roots<T>=[Option<PersistentList<T>>;PERSISTENT_PAGE_FANOUT];
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Wait,Leaf,Carry,Branch,Scan,Ready,Spent}
/// 🌳️ A native address-space bound limits original construction custody independently of stream length.
pub struct PersistentListBuilder<T:crate::retirement::RetireOwned+Send+Sync+'static>{
 rows:ManuallyDrop<Rows<T>>,levels:ManuallyDrop<[Option<Box<Roots<T>>>;LEVELS]>,carry:ManuallyDrop<Option<PersistentList<T>>>,output:ManuallyDrop<Option<PersistentList<T>>>,
 retirement:RetainedCloneClose,counts:[u8;LEVELS],expected:usize,adopted:usize,row_count:usize,level:usize,close_index:usize,phase:Phase,finishing:bool,closing:bool,
}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> PersistentListBuilder<T>{
 /// 🔍️ A cold owner checks every retained row and page before releasing any original.
 pub fn cold_is_exclusive(&self)->bool where T:crate::shared_issuance::ColdRetire{
  self.retirement.is_empty()
   && self.rows.iter().flatten().all(|value|value.original_is_exclusive()&&value.get().cold_exclusive())
   && self.levels.iter().flatten().all(|level|level.iter().flatten().all(|list|list.cold_is_exclusive()))
   && self.carry.as_ref().is_none_or(|list|list.cold_is_exclusive())
   && self.output.as_ref().is_none_or(|list|list.cold_is_exclusive())
 }
 /// ♻️ Returns false without mutation when an original page, row or nested payload has aliases.
 pub fn cold_dismantle(&mut self)->bool where T:crate::shared_issuance::ColdRetire{
  if !self.cold_is_exclusive(){return false}
  for slot in self.rows.iter_mut(){if let Some(value)=slot.take(){super::cold_take(value).cold_release();}}
  for level in self.levels.iter_mut(){if let Some(mut level)=level.take(){for list in level.iter_mut().flatten(){list.cold_dismantle_prechecked();}}}
  if let Some(mut list)=self.carry.take(){list.cold_dismantle_prechecked();}
  if let Some(mut list)=self.output.take(){list.cold_dismantle_prechecked();}
  true
 }
 /// 🌱️ Initializes only the fixed empty frontier without allocating or probing original input.
 pub fn new(expected:usize)->Self{Self{rows:ManuallyDrop::new(std::array::from_fn(|_|None)),levels:ManuallyDrop::new(std::array::from_fn(|_|None)),carry:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),counts:[0;LEVELS],expected,adopted:0,row_count:0,level:0,close_index:0,phase:Phase::Wait,finishing:false,closing:false}}
 /// 🚪️ Only the current original stream position may supply another owned row header.
 pub fn accepts_row(&self)->bool{!self.closing&&!self.finishing&&self.phase==Phase::Wait}
 /// 🧭️ Observes only the actual retained finishing declaration without spending or issuing authority.
 pub fn finishing(&self)->bool{self.finishing}
 /// 📏️ Adoption moves one original row header without copying its retained payload.
 pub fn adopt_demand(&self)->Result<RetirementDemand,ValueError>{if !self.accepts_row()||self.adopted>=self.expected{return Err(refusal("original persistent row is outside its admitted stream"));}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// 📦️ A denied adoption preserves the exact receiving slot; actual row movement reports its own work.
 pub fn adopt(&mut self,row:&mut Option<SealedShared<T>>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>>{let d=match self.adopt_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if !fits(d,grant){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}if row.is_none(){return crate::RetainedWorkOutcome{result:Err(refusal("original persistent row receiving slot is empty").with_retained_progress(receipt(d))),work:1}}self.rows[self.row_count]=row.take();self.row_count+=1;self.adopted+=1;if self.row_count==PERSISTENT_PAGE_FANOUT{self.phase=Phase::Leaf;}crate::RetainedWorkOutcome{result:Ok(Some(receipt(d))),work:1}}
 /// 🏁️ The exact authored cardinality precedes the retained finishing declaration.
 pub fn finish_demand(&self)->Result<RetirementDemand,ValueError>{if self.closing||self.finishing||self.adopted!=self.expected||self.phase==Phase::Spent{return Err(refusal("original persistent stream has not supplied its exact cardinality"));}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// 🧵️ Finishing records one paid fixed control transition before later page preparation.
 pub fn finish(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>>{if self.finishing&&!self.closing{return crate::RetainedWorkOutcome{result:Ok(Some(Default::default())),work:0}}let d=match self.finish_demand(){Ok(d)=>d,Err(error)=>return crate::RetainedWorkOutcome{result:Err(error),work:0}};if !fits(d,grant){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}self.finishing=true;if self.phase==Phase::Wait{self.level=0;self.phase=if self.row_count>0{Phase::Leaf}else{Phase::Scan};}crate::RetainedWorkOutcome{result:Ok(Some(receipt(d))),work:1}}
 /// 🎟️ Actual page or level backing births remain separate from completed-child publication.
 pub fn demand(&self)->Result<RetirementDemand,ValueError>{if self.closing{return Err(refusal("original persistent construction is closing"));}match self.phase{Phase::Leaf|Phase::Branch=>Ok(if self.carry.is_some(){RetirementDemand{depth:1,..Default::default()}}else{PersistentList::<T>::page_birth_demand()}),Phase::Carry=>{if self.level>=LEVELS{return Err(refusal("original persistent carry exceeds ranked address space"))}Ok(RetirementDemand{capacity_bytes:if self.levels[self.level].is_none(){size_of::<Roots<T>>()}else{0},depth:1,..Default::default()})},Phase::Scan=>Ok(RetirementDemand{depth:1,..Default::default()}),_=>Err(refusal("original persistent builder has no pending preparation turn"))}}
 /// 🪜️ Each genuine allocator, row carry, rank scan or final publication reports its actual bounded work.
 pub fn advance(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{if self.output_ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.demand()?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}work=1;let p=receipt(d);match self.phase{
 Phase::Leaf=>{if self.carry.is_some(){self.row_count=0;self.level=0;self.phase=Phase::Carry;}else{let rows=std::mem::replace(&mut*self.rows,std::array::from_fn(|_|None));let turn=PersistentList::admit_leaf(rows,self.row_count,grant);work=turn.work;return match turn.result{Ok((root,p))=>{*self.carry=Some(root);Ok(RetainedCloneStep::Progress(p))},Err((error,rows))=>{*self.rows=rows;Err(error)}}}},
 Phase::Carry=>{let slot=usize::from(*self.counts.get(self.level).ok_or_else(||refusal("original persistent carry exceeds ranked address space").with_retained_progress(p))?);if slot>=PERSISTENT_PAGE_FANOUT||self.carry.is_none(){return Err(refusal("original persistent carry has no vacant original page slot").with_retained_progress(p))}if self.levels[self.level].is_none(){let layout=std::alloc::Layout::new::<Roots<T>>();let raw=unsafe{std::alloc::alloc(layout)}as*mut Roots<T>;if raw.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original persistent level allocation failed"))}unsafe{raw.write(std::array::from_fn(|_|None));self.levels[self.level]=Some(Box::from_raw(raw));}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()}))}self.levels[self.level].as_mut().ok_or_else(||ValueError::invariant("original funded level table"))?[slot]=self.carry.take();self.counts[self.level]+=1;self.phase=if usize::from(self.counts[self.level])==PERSISTENT_PAGE_FANOUT{Phase::Branch}else if self.finishing{self.level=0;Phase::Scan}else{Phase::Wait};},
 Phase::Branch=>{if self.carry.is_some(){self.counts[self.level]=0;self.level+=1;self.phase=Phase::Carry;}else{if self.level>=LEVELS||self.counts[self.level]==0{return Err(refusal("original persistent branch has no children").with_retained_progress(p))}let table=self.levels[self.level].as_mut().ok_or_else(||refusal("original persistent branch has no owned table").with_retained_progress(p))?;let roots=std::mem::replace(&mut**table,std::array::from_fn(|_|None));let turn=PersistentList::admit_branch(roots,grant);work=turn.work;return match turn.result{Ok((root,p))=>{*self.carry=Some(root);Ok(RetainedCloneStep::Progress(p))},Err((error,roots))=>{**table=roots;Err(error)}}}},
 Phase::Scan=>{if self.expected==0{*self.output=Some(PersistentList::default());self.phase=Phase::Ready;}else if self.level>=LEVELS{return Err(refusal("original persistent stream lost its complete page roots").with_retained_progress(p))}else if self.counts[self.level]==0{self.level+=1;}else if self.counts[self.level]==1&&self.counts[self.level+1..].iter().all(|count|*count==0){let root=self.levels[self.level].as_mut().ok_or_else(||refusal("original final root has no owned table").with_retained_progress(p))?[0].take().ok_or_else(||refusal("original final root slot is empty").with_retained_progress(p))?;*self.output=Some(root);self.counts[self.level]=0;self.phase=Phase::Ready;}else{self.phase=Phase::Branch;}},_=>return Err(refusal("quoted original persistent preparation phase").with_retained_progress(p))}
 Ok(if self.output_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})})();crate::RetainedWorkOutcome{result,work}}
 /// 👁️ Readiness requires the genuine final owned root, including an authored empty collection.
 pub fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 /// 📤️ Final publication transfers only a fixed root header and leaves no duplicate source custody.
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(refusal("original persistent candidate is not ready"));}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// 🔑️ A denied final transfer retains the original final root for another funded turn or cancellation.
 pub fn take(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<Option<(PersistentList<T>,RetainedCloneProgress)>>{let mut work=0;let result=(||{let d=self.take_demand()?;if !fits(d,grant){return Ok(None);}work=1;let output=self.output.take().ok_or_else(||ValueError::invariant("ready original persistent root"))?;self.phase=Phase::Spent;Ok(Some((output,receipt(d))))})();crate::RetainedWorkOutcome{result,work}}
 /// 🛑️ Every receiving frontier keeps its genuine row and page issuers until funded closure.
 pub fn begin_close(&mut self){self.closing=true;}
 /// ♻️ Cleanup quotes the actual child or a distinct scalar frontier scan.
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.terminal_is_empty(){return Ok(Default::default())}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<PersistentList<T>>(false,&None,body)}
  let present=if self.close_index<PERSISTENT_PAGE_FANOUT{self.rows[self.close_index].is_some()}else if self.close_index<PERSISTENT_PAGE_FANOUT+LEVELS{self.levels[self.close_index-PERSISTENT_PAGE_FANOUT].is_some()}else if self.close_index==PERSISTENT_PAGE_FANOUT+LEVELS{self.carry.is_some()}else if self.close_index==PERSISTENT_PAGE_FANOUT+LEVELS+1{self.output.is_some()}else{return Err(refusal("original persistent close frontier lost its retained owner"))};
  if !present{return Ok(RetirementDemand{depth:1,..Default::default()})}if self.close_index<PERSISTENT_PAGE_FANOUT{self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body)}else if self.close_index<PERSISTENT_PAGE_FANOUT+LEVELS{self.retirement.next_owner_demands::<Box<Roots<T>>>(true,&None,body)}else{self.retirement.next_owner_demands::<PersistentList<T>>(true,&None,body)}
 }
 /// 🚚️ Each genuine child outcome precedes separate frontier metadata work.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementOutcome{let mut work=0;let result=(|| -> Result<crate::RetainedCloneStep,semio_framework_value_control_contract::OriginalRetirementFailure>{
  if !self.closing{return Err(refusal("original persistent closure has not started").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(grant.maximum_copy_bytes)?;if !fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.retirement.is_empty(){let turn=self.retirement.step_granted(grant);work=turn.work;return turn.result.map(|s|RetainedCloneStep::Progress(s.progress()))}
  let present=if self.close_index<PERSISTENT_PAGE_FANOUT{self.rows[self.close_index].is_some()}else if self.close_index<PERSISTENT_PAGE_FANOUT+LEVELS{self.levels[self.close_index-PERSISTENT_PAGE_FANOUT].is_some()}else if self.close_index==PERSISTENT_PAGE_FANOUT+LEVELS{self.carry.is_some()}else{self.output.is_some()};if !present{work=1;self.close_index+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  let turn=if self.close_index<PERSISTENT_PAGE_FANOUT{self.retirement.begin_granted(&mut self.rows[self.close_index],grant)}else if self.close_index<PERSISTENT_PAGE_FANOUT+LEVELS{self.retirement.begin_granted(&mut self.levels[self.close_index-PERSISTENT_PAGE_FANOUT],grant)}else if self.close_index==PERSISTENT_PAGE_FANOUT+LEVELS{self.retirement.begin_granted(&mut self.carry,grant)}else{self.retirement.begin_granted(&mut self.output,grant)};work=turn.work;turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default())))
 })();crate::RetainedWorkOutcome{result,work}}
 /// 🧹️ Terminal owned custody is determined from real slots and the actual native cleanup frontier.
 pub fn terminal_is_empty(&self)->bool{self.rows.iter().all(Option::is_none)&&self.levels.iter().all(Option::is_none)&&self.carry.is_none()&&self.output.is_none()&&self.retirement.is_empty()}
}
impl<T:crate::retirement::RetireOwned+Send+Sync+'static> Drop for PersistentListBuilder<T>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"original persistent builder retains owned rows and roots until funded closure");if empty{unsafe{ManuallyDrop::drop(&mut self.rows);ManuallyDrop::drop(&mut self.levels);ManuallyDrop::drop(&mut self.carry);ManuallyDrop::drop(&mut self.output);}}}}

#[cfg(test)]#[path="♻️close/🧪️tests/🦀️.rs"]mod close_work_tests;

#[cfg(test)]
#[path="🎟️work/🧪️tests/🦀️.rs"]
mod normal_work_tests;
