//! 📦️ Original fixed page storage is born, initialized, moved and physically returned under explicit caller grants.
use super::{TypedOperationResultPage,TypedOperationResultToken,TypedOperationResultLane,TYPED_OPERATION_RESULT_PAGE_BYTES};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::mem::{ManuallyDrop,size_of};

/// 🪙️ The producer retains every partially initialized original allocation until funded handoff or closure.
pub struct OriginalResultPageBacking{
    bytes:ManuallyDrop<Option<Vec<u8>>>,
    closing:Option<ControlledRetirement<Vec<u8>>>,
}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original page backing has no admitted next action")}
fn paused()->RetainedWorkOutcome<RetainedCloneStep>{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
impl OriginalResultPageBacking{
    /// 🌱️ Empty custody owns no allocation and requires no hidden constructor.
    pub const fn empty()->Self{Self{bytes:ManuallyDrop::new(None),closing:None}}
    pub fn terminal_is_empty(&self)->bool{self.bytes.is_none()&&self.closing.is_none()}
    pub fn initialized_len(&self)->usize{self.bytes.as_ref().map_or(0,Vec::len)}
    pub fn initialized_bytes(&mut self)->Option<&mut[u8]>{self.bytes.as_mut().map(Vec::as_mut_slice)}
    pub fn is_ready(&self)->bool{self.closing.is_none()&&self.initialized_len()==TYPED_OPERATION_RESULT_PAGE_BYTES}
    pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Option<Vec<u8>>>(),capacity_bytes:TYPED_OPERATION_RESULT_PAGE_BYTES,depth:1,..Default::default()}}
    /// 🏗️ Fixed capacity is retained before the first initialized byte exists.
    pub fn birth_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
        if !self.terminal_is_empty(){return RetainedWorkOutcome{result:Err(invalid()),work:0}}
        let d=Self::birth_demands();if !fits(d,grant){return paused()}
        *self.bytes=Some(Vec::with_capacity(TYPED_OPERATION_RESULT_PAGE_BYTES));
        assert_eq!(self.bytes.as_ref().unwrap().capacity(),TYPED_OPERATION_RESULT_PAGE_BYTES);
        RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})),work:1}
    }
    /// 📏️ A cursor-only body cannot initialize storage or consume an item.
    pub fn initialization_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let bytes=self.bytes.as_ref().filter(|_|self.closing.is_none()).ok_or_else(invalid)?;
        let remaining=TYPED_OPERATION_RESULT_PAGE_BYTES.checked_sub(bytes.len()).ok_or_else(invalid)?;
        if remaining==0{return Ok(Default::default())}
        let count=remaining.min(2048).min(body.saturating_sub(size_of::<usize>()).max(1));
        Ok(RetirementDemand{copy_bytes:count+size_of::<usize>(),depth:1,..Default::default()})
    }
    /// ✍️ Initializes only positive payload bytes paid by this exact body.
    pub fn initialize_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
        let d=match self.initialization_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
        if d==Default::default(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
        if !fits(d,grant){return paused()}
        let count=d.copy_bytes-size_of::<usize>();let bytes=self.bytes.as_mut().unwrap();bytes.resize(bytes.len()+count,0);
        let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
        RetainedWorkOutcome{result:Ok(if self.is_ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:1}
    }
    pub fn page_demands(&self)->Result<RetirementDemand,ValueError>{
        if !self.is_ready(){return Err(invalid())}
        Ok(RetirementDemand{copy_bytes:2*size_of::<TypedOperationResultPage>()+2*size_of::<Option<Vec<u8>>>(),depth:1,..Default::default()})
    }
    /// 📤️ Transfers the original fixed allocation without copying or replacing its payload.
    pub fn take_page(&mut self,token:TypedOperationResultToken,lane:TypedOperationResultLane,len:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(TypedOperationResultPage,RetainedCloneProgress)>>{
        let d=match self.page_demands(){Ok(d)if len<=TYPED_OPERATION_RESULT_PAGE_BYTES=>d,_=>return RetainedWorkOutcome{result:Err(invalid()),work:0}};
        if !fits(d,grant){return RetainedWorkOutcome{result:Ok(None),work:0}}
        let bytes=self.bytes.take().unwrap();let page=TypedOperationResultPage::from_original(token,lane,bytes,len);
        RetainedWorkOutcome{result:Ok(Some((page,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))),work:1}
    }
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let metadata=size_of::<Option<ControlledRetirement<Vec<u8>>>>();
        if let Some(owner)=self.closing.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:metadata,depth:1,..Default::default()})}
            let mut d=owner.next_demand(body.saturating_sub(metadata))?;d.copy_bytes+=metadata;return Ok(d)
        }
        if self.bytes.is_some(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Option<Vec<u8>>>()+metadata,depth:1,..Default::default()})}
        Ok(Default::default())
    }
    /// ♻️ The same typed retirement returns empty, partial and fully initialized backing.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep>{
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
        let d=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};
        if !fits(d,grant){return paused()}
        if self.bytes.is_some(){
            self.closing=Some(ControlledRetirement::retain(self.bytes.take().unwrap()));
            return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
        }
        if self.closing.as_ref().unwrap().terminal_is_empty(){
            self.closing=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}
        }
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-size_of::<Option<ControlledRetirement<Vec<u8>>>>(),..grant};
        let result=self.closing.as_mut().unwrap().step(child);
        RetainedWorkOutcome{result,work:self.closing.as_ref().unwrap().last_turn_work()}
    }
}
impl Drop for OriginalResultPageBacking{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original page backing retains its allocation until funded handoff or closure")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
