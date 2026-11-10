//! 🎮️ Moves ordered native list slots one admitted step at a time.

use super::{PagedList,PagedListAllocationError,PagedListError,PagedListProgress,PagedListRefusalKind,PagedListReservationFailure,PagedListReservationOutcome};
use semio_framework_value_control_contract::{RetainedAllocationEffect,RetainedCloneGrant,ValueError,ValueRefusalKind};
/// 🩻 Keeps original edit, allocation and Work refusals independent.
#[derive(Debug)]
pub enum PagedListEditFailure{Edit(PagedListError),Allocation{error:PagedListAllocationError,copied_bytes:usize},Work(ValueError)}
impl From<PagedListError> for PagedListEditFailure{fn from(error:PagedListError)->Self{Self::Edit(error)}}
impl From<PagedListReservationFailure> for PagedListEditFailure{fn from(error:PagedListReservationFailure)->Self{match error{PagedListReservationFailure::Allocation{error,copied_bytes}=>Self::Allocation{error,copied_bytes},PagedListReservationFailure::Work(error)=>Self::Work(error)}}}
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PagedListEditStep {
    pub complete: bool,
    pub moved_items: usize,
    pub progress: PagedListProgress,
}

/// 🛸 Distinguishes an unadmitted edit from a completed original edit step.
#[derive(Debug)]
pub enum PagedListEditOutcome{Denied,Ready(PagedListEditStep)}

pub struct PagedListEditCursor {
    insertion: bool,
    target: usize,
    position: usize,
    expected_length: usize,
    owner: Option<usize>,
    started: bool,
    complete: bool,
}

impl PagedListEditCursor {
    pub fn insert(index: usize, original_length: usize) -> Self {
        Self { insertion: true, target: index, position: original_length, expected_length: original_length, owner: None, started: false, complete: false }
    }

    pub fn remove(index: usize, original_length: usize) -> Self {
        Self { insertion: false, target: index, position: index, expected_length: original_length, owner: None, started: false, complete: false }
    }

    pub fn is_finished(&self) -> bool { self.complete }

    pub fn step_demand<T,const N:usize>(&self,owner:&PagedList<T,N>,item:&Option<T>)->Result<semio_framework_value_control_contract::RetirementDemand,PagedListEditFailure>{use semio_framework_value_control_contract::RetirementDemand;let fail=|reason|PagedListEditFailure::Edit(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason});if self.owner.is_some_and(|address|address!=owner as *const _ as usize)||owner.len()!=self.expected_length{return Err(fail("ordered list edit owner or length changed"))}if self.complete{return Ok(Default::default())}if self.target>owner.len()||(!self.insertion&&self.target==owner.len()){return Err(fail("ordered list edit index is outside its owner"))}if self.owner.is_none(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<usize>>(),depth:1,..Default::default()})}if self.insertion&&!self.started{if item.is_none(){return Err(fail("ordered list insertion lost its source owner"))}if !owner.has_reserved_slot(){return Ok(RetirementDemand{copy_bytes:owner.next_reserve_copy_byte_demand()?,capacity_bytes:owner.next_allocation_bytes()?,depth:owner.next_reserve_depth_demand()?,..Default::default()})}return Ok(RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<T>()+2*size_of::<usize>()+2*size_of::<bool>(),depth:owner.next_push_depth_demand()?,..Default::default()})}if !self.insertion&&item.is_some(){return Err(fail("ordered list removal destination already owns an item"))}let shifting=if self.insertion{self.position>self.target}else{self.position+1<owner.len()};if shifting{return Ok(RetirementDemand{copy_bytes:3*size_of::<T>()+size_of::<usize>()+2*size_of::<bool>(),depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<T>()+2*size_of::<usize>()+size_of::<bool>(),depth:owner.next_pop_depth_demand()?,..Default::default()})}

    pub fn step_work_demand<T,const N:usize>(&self,owner:&PagedList<T,N>,item:&Option<T>)->Result<u64,PagedListEditFailure>{self.step_demand(owner,item)?;Ok(u64::from(!self.complete))}

    pub fn step<T,const N:usize>(&mut self,owner:&mut PagedList<T,N>,item:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkPoll<PagedListEditOutcome,PagedListEditFailure>{
        if effect.maximum_work_units()>1{if let Some(turn)=self.funded_source_step(owner,item,effect){return semio_framework_value_control_contract::RetainedWorkPoll::Complete(turn)}}
        use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedWorkPoll};
        let demand=match self.step_demand(owner,item){Ok(d)=>d,Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work:0})};if self.complete{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:true,..Default::default()})),work:0})}let grant=effect.grant();if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand)||effect.maximum_work_units()==0{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(PagedListEditOutcome::Denied),work:0})}if self.owner.is_some()&&self.insertion&&!self.started&&!owner.has_reserved_slot(){let turn=owner.reserve_one(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|match step{PagedListReservationOutcome::Denied=>PagedListEditOutcome::Denied,PagedListReservationOutcome::Ready(progress)=>PagedListEditOutcome::Ready(PagedListEditStep{moved_items:usize::from(progress.progressed),progress,complete:false})}).map_err(Into::into),work:turn.work})}match effect.performed(1){Ok(true)=>{},Ok(false)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(ValueError::literal(ValueRefusalKind::WorkLimit,"ordered list edit exceeded its work authority"))),work:1}),Err(error)=>return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(error)),work:1})}let result=(||{if self.owner.is_none(){self.owner=Some(owner as *const _ as usize);return Ok(PagedListEditOutcome::Ready(PagedListEditStep{progress:PagedListProgress{work:1,progressed:true,placed_bytes:demand.copy_bytes,..Default::default()},..Default::default()}))}if self.insertion&&!self.started{let mut progress=owner.place_reserved(item,grant.maximum_copy_bytes)?;if progress.progressed{self.expected_length+=1;self.started=true;self.complete=self.position==self.target;progress.work=1;progress.placed_bytes=demand.copy_bytes}return Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:self.complete,moved_items:usize::from(progress.progressed),progress}))}let shifting=if self.insertion{self.position>self.target}else{self.position+1<owner.len()};if shifting{if self.insertion{owner.swap(self.position-1,self.position);self.position-=1}else{owner.swap(self.position,self.position+1);self.position+=1}self.started=true;self.complete=self.insertion&&self.position==self.target;return Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:self.complete,moved_items:1,progress:PagedListProgress{work:1,progressed:true,placed_bytes:demand.copy_bytes,..Default::default()}}))}*item=owner.pop();self.expected_length-=1;self.complete=true;Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:true,moved_items:1,progress:PagedListProgress{work:1,progressed:true,placed_bytes:demand.copy_bytes,..Default::default()}}))        })();RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:1})
    }

    /// 📜 Restores source-funded ordered moves when the caller granted more than one work unit.
    fn funded_source_step<T,const N:usize>(&mut self,owner:&mut PagedList<T,N>,item:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->Option<semio_framework_value_control_contract::RetainedWorkOutcome<PagedListEditOutcome,PagedListEditFailure>>{
        use semio_framework_value_control_contract::{RetainedWorkOutcome,RetirementDemand,retained_clone_grant_admits};
        if self.complete||self.owner.is_none()||self.owner.is_some_and(|address|address!=owner as *const _ as usize)||owner.len()!=self.expected_length{return None}
        if self.target>owner.len()||(!self.insertion&&self.target==owner.len())||(self.insertion&&!self.started&&item.is_none())||(!self.insertion&&item.is_some()){return None}
        if self.insertion&&!self.started&&!owner.has_reserved_slot(){return None}
        let shifting=if self.insertion{self.started&&self.position>self.target}else{self.position+1<owner.len()};
        let state_bytes=if self.insertion&&!self.started{size_of::<usize>()+2*size_of::<bool>()}else if shifting{size_of::<usize>()+2*size_of::<bool>()}else{size_of::<usize>()+size_of::<bool>()};
        let (demand,work)=if self.insertion&&!self.started{(owner.adoption_demand(item).ok()?,1u64)}else if shifting{let left=if self.insertion{self.position-1}else{self.position};(owner.swap_source_demand(left,left+1).ok()?,owner.swap_source_work_demand(left,left+1).ok()?)}else{(owner.pop_source_demand().ok()?,owner.pop_source_work_demand().ok()?)};
        let total_copy=demand.copy_bytes.checked_add(state_bytes)?;
        let required=RetirementDemand{copy_bytes:total_copy,depth:demand.depth.max(1),..demand};
        let grant=effect.grant();
        let available=effect.remaining_work_units().ok()?;
        let total_work=work.checked_add(1)?;
        if !retained_clone_grant_admits(grant,required)||available<total_work||grant.maximum_copy_bytes<state_bytes{return None}
        let boundary=effect.performed_boundary();
        let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-state_bytes,..grant};
        let refuse=|reason|PagedListEditFailure::Edit(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason});
        let (mut progress,child_work)=if self.insertion&&!self.started{
            let turn={let mut child=effect.reborrow(child_grant,work).ok()?;owner.adopt_reserved(item,&mut child)};
            let recorded=turn.work;
            match turn.result{Ok(progress) if progress.progressed=>(progress,recorded),Ok(_) if recorded==0=>return None,Ok(_)=>return Some(RetainedWorkOutcome{result:Ok(PagedListEditOutcome::Denied),work:recorded}),Err(error)=>return Some(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(error)),work:recorded})}
        }else if shifting{
            let left=if self.insertion{self.position-1}else{self.position};
            let turn={let mut child=effect.reborrow(child_grant,work).ok()?;owner.swap_source_step(left,left+1,&mut child)};
            let recorded=turn.work;
            match turn.result{Ok(true)=>(PagedListProgress{progressed:true,placed_bytes:demand.copy_bytes,..Default::default()},recorded),Ok(false) if recorded==0=>return None,Ok(false)=>return Some(RetainedWorkOutcome{result:Ok(PagedListEditOutcome::Denied),work:recorded}),Err(error)=>return Some(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(error)),work:recorded})}
        }else{
            let pop_demand=owner.pop_source_demand().ok()?;
            let quoted=owner.pop_source_work_demand().ok()?;
            let moved={let mut child=effect.reborrow(child_grant,work).ok()?;let child_boundary=child.performed_boundary();let admitted=child.grant().maximum_items>0&&retained_clone_grant_admits(child.grant(),pop_demand)&&child.remaining_work_units().ok().is_some_and(|units|units>=quoted);if !admitted{None}else{let performed=child.performed(quoted);let charged=child.performed_since(child_boundary).unwrap_or(0);Some((performed,charged))}};
            let Some((performed,charged))=moved else{return None};
            match performed{Ok(true)=>{},Ok(false)=>return Some(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(ValueError::literal(ValueRefusalKind::WorkLimit,"original paged tail exceeded its Work authority"))),work:charged}),Err(error)=>return Some(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(error)),work:charged})}
            let Some(value)=owner.pop() else{return Some(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(ValueError::invariant("original tail leaf is absent"))),work:charged})};
            *item=Some(value);
            (PagedListProgress{progressed:true,placed_bytes:pop_demand.copy_bytes,..Default::default()},charged)
        };
        match effect.performed(1){Ok(true)=>{},Ok(false)=>return Some(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(ValueError::literal(ValueRefusalKind::WorkLimit,"ordered edit state exceeded its original work"))),work:effect.performed_since(boundary).unwrap_or(0)}),Err(error)=>return Some(RetainedWorkOutcome{result:Err(PagedListEditFailure::Work(error)),work:effect.performed_since(boundary).unwrap_or(0)})}
        if self.insertion&&!self.started{self.expected_length+=1;self.started=true;self.complete=self.position==self.target}else if shifting{if self.insertion{self.position-=1}else{self.position+=1}self.started=true;self.complete=self.insertion&&self.position==self.target}else{self.expected_length-=1;self.complete=true}
        let Ok(units)=u64::try_from(child_work) else{return Some(RetainedWorkOutcome{result:Err(refuse("ordered edit child work receipt overflow")),work:effect.performed_since(boundary).unwrap_or(0)})};
        progress.work=units;
        let Some(placed)=progress.placed_bytes.checked_add(state_bytes) else{return Some(RetainedWorkOutcome{result:Err(refuse("ordered edit receipt overflow")),work:effect.performed_since(boundary).unwrap_or(0)})};
        progress.placed_bytes=placed;
        let Some(next)=progress.work.checked_add(1) else{return Some(RetainedWorkOutcome{result:Err(refuse("ordered edit work receipt overflow")),work:effect.performed_since(boundary).unwrap_or(0)})};
        progress.work=next;
        Some(RetainedWorkOutcome{result:Ok(PagedListEditOutcome::Ready(PagedListEditStep{complete:self.complete,moved_items:1,progress})),work:effect.performed_since(boundary).expect("ordered edit retains its original work receipt")})
    }
}
  