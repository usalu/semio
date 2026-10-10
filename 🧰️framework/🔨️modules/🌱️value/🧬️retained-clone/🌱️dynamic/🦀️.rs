//! 🌱️ Original dynamic values clone one paid payload atom or retained child transition per turn.

use super::*;
use crate::{DslValue,RetirementDemand,ValueError,ValueRefusalKind};

struct State {
    source:Option<RetainedCloneBinding>,
    target:Option<DslValue>,
    bytes:Option<Vec<u8>>,
    key:Option<String>,
    child:Option<Box<DslValueRetainedCloneCursor>>,
    index:usize,
    phase:u8,
    draining:bool,
    closing:bool,
    spent:bool,
    close:RetainedCloneClose,
}

/// 🌳️ Inline cold custody retains partial payload and exact source leases until granted closure.
pub struct DslValueRetainedCloneCursor {state:ManuallyDrop<State>}

impl Default for DslValueRetainedCloneCursor {
    fn default()->Self {Self {state:ManuallyDrop::new(State {source:None,target:None,bytes:None,key:None,child:None,index:0,phase:0,draining:false,closing:false,spent:false,close:Default::default()})}}
}

impl DslValueRetainedCloneCursor {
    fn refusal(reason:&'static str)->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
    fn extent<T>(length:usize)->Result<usize,ValueError> {length.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"dynamic clone destination layout overflow"))}
    fn child_source<'a>(&self,source:RetainedCloneRef<'a,DslValue>)->Result<RetainedCloneRef<'a,DslValue>,ValueError> {
        let index=self.state.index;
        match source.get(){
            DslValue::Array(values)if index<values.len()=>Ok(source.project(index+1,move |source|match source {DslValue::Array(values)=>&values[index],_=>unreachable!()})),
            DslValue::Object(values)if index<values.len()=>Ok(source.project(index+1,move |source|match source {DslValue::Object(values)=>&values[index].1,_=>unreachable!()})),
            _=>Err(Self::refusal("dynamic clone child path no longer names the original owner")),
        }
    }
    fn verify_source(&self,source:RetainedCloneRef<'_,DslValue>)->Result<(),ValueError> {
        if let Some(expected)=&self.state.source {
            if expected.lease.as_ref().map(|lease|lease.id)!=Some(source.lease.id)||expected.projection!=source.projection {return Err(Self::refusal("dynamic clone original source identity changed"));}
        }
        Ok(())
    }
    fn empty(&self)->bool {let s=&self.state;s.source.is_none()&&s.target.is_none()&&s.bytes.is_none()&&s.key.is_none()&&s.child.is_none()&&s.close.is_empty()}
    fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError> {demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"dynamic clone child depth overflow"))?;Ok(demand)}
    fn child_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        let child=self.state.child.as_ref().unwrap();
        if !child.state.closing {return Ok(RetirementDemand {depth:1,..Default::default()});}
        if child.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:size_of::<Self>(),depth:1,..Default::default()});}
        Self::nested(child.close_demands(copy)?)
    }
    fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        let s=&self.state;
        if s.child.is_some(){return self.child_close_demands(copy);}
        if !s.close.is_empty(){return Ok(RetirementDemand {copy_bytes:s.close.next_copy_byte_demand()?,capacity_bytes:s.close.next_capacity_byte_demand(copy)?,release_bytes:s.close.next_release_byte_demand()?,depth:s.close.next_depth_demand()?});}
        let capacity=if s.bytes.is_some(){crate::owned_retirement_birth_bytes::<Vec<u8>>()}else if s.key.is_some(){crate::owned_retirement_birth_bytes::<String>()}else if s.target.is_some(){crate::owned_retirement_birth_bytes::<DslValue>()}else{return Ok(RetirementDemand {copy_bytes:RetainedCloneBinding::copy_demand(&s.source)?,capacity_bytes:RetainedCloneBinding::capacity_demand(&s.source,copy)?,release_bytes:RetainedCloneBinding::release_demand(&s.source)?,depth:RetainedCloneBinding::depth_demand(&s.source)?});};
        Ok(RetirementDemand {capacity_bytes:capacity,depth:1,..Default::default()})
    }
    /// 🧮️ Borrows the exact next original clone effect, including normal spent-child closure.
    pub fn demands(&self,source:RetainedCloneRef<'_,DslValue>,copy:usize)->Result<RetirementDemand,ValueError> {
        self.verify_source(source)?;
        if self.state.closing{return Err(Self::refusal("dynamic clone cursor is closing"));}
        if self.state.spent{return Err(Self::refusal("dynamic clone cursor has transferred its output"));}
        if self.state.phase==2{return Ok(Default::default());}
        let s=&self.state;
        if s.phase==0 {
            return Ok(match source.get(){
                DslValue::Null=>RetirementDemand {depth:1,..Default::default()},
                DslValue::Bool(_)=>RetirementDemand {copy_bytes:size_of::<bool>(),depth:1,..Default::default()},
                DslValue::Number(_)=>RetirementDemand {copy_bytes:size_of::<u64>(),depth:1,..Default::default()},
                DslValue::String(value)=>RetirementDemand {capacity_bytes:value.len(),depth:1,..Default::default()},
                DslValue::Bytes(value)=>RetirementDemand {capacity_bytes:value.len(),depth:1,..Default::default()},
                DslValue::Array(value)=>RetirementDemand {capacity_bytes:Self::extent::<DslValue>(value.len())?,depth:1,..Default::default()},
                DslValue::Object(value)=>RetirementDemand {capacity_bytes:Self::extent::<(String,DslValue)>(value.len())?,depth:1,..Default::default()},
            });
        }
        if let Some(child)=s.child.as_ref(){
            if s.draining{return self.child_close_demands(copy);}
            if child.state.phase==2{return Ok(RetirementDemand {depth:1,..Default::default()});}
            return Self::nested(child.demands(self.child_source(source)?,copy)?);
        }
        Ok(match source.get(){
            DslValue::String(value)=>RetirementDemand {copy_bytes:usize::from(s.bytes.as_ref().unwrap().len()<value.len()),depth:1,..Default::default()},
            DslValue::Bytes(value)=>RetirementDemand {copy_bytes:usize::from(s.index<value.len()),depth:1,..Default::default()},
            DslValue::Array(value)=>RetirementDemand {capacity_bytes:if s.index<value.len(){size_of::<Self>()}else{0},depth:1,..Default::default()},
            DslValue::Object(value)if s.index<value.len()=>{
                if s.key.is_some(){RetirementDemand {capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
                else if let Some(bytes)=s.bytes.as_ref(){RetirementDemand {copy_bytes:usize::from(bytes.len()<value[s.index].0.len()),depth:1,..Default::default()}}
                else{RetirementDemand {capacity_bytes:value[s.index].0.len(),depth:1,..Default::default()}}
            },
            DslValue::Object(_)=>RetirementDemand {depth:1,..Default::default()},
            _=>return Err(Self::refusal("dynamic clone original phase does not match its source variant")),
        })
    }
    fn admitted(demand:RetirementDemand,grant:RetainedCloneGrant)->Result<bool,ValueError> {
        if grant.maximum_items==0{return Ok(false);}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"dynamic clone requires admitted original depth"));}
        Ok(demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes)
    }
    fn close_child(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let child=self.state.child.as_mut().unwrap();
        if child.begin_close(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        if !child.terminal_is_empty(){let step=child.close_step(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant})?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        self.state.child.take();self.state.draining=false;self.state.index+=usize::from(!self.state.closing);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:size_of::<Self>(),..Default::default()}))
    }
}

impl RetainedClone for DslValue {
    type Cursor=DslValueRetainedCloneCursor;
    fn retained_clone_cursor()->Self::Cursor {Default::default()}
}

impl RetainedCloneDemandCursor<DslValue> for DslValueRetainedCloneCursor {
    fn normal_demands(&self,source:RetainedCloneRef<'_,DslValue>,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError> {self.demands(source,maximum_copy_bytes)}
}

impl RetainedCloneCursor<DslValue> for DslValueRetainedCloneCursor {
    fn advance(&mut self,source:RetainedCloneRef<'_,DslValue>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.demands(source,grant.maximum_copy_bytes)?;
        if !Self::admitted(demand,grant)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.state.phase==2{return Ok(RetainedCloneStep::Complete(Default::default()));}
        source.bind(&mut self.state.source)?;
        if self.state.draining{return self.close_child(grant);}
        if let Some(child)=self.state.child.as_mut(){
            if child.state.phase!=2 {let projected=self.child_source(source)?;let step=self.state.child.as_mut().unwrap().advance(projected,RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant})?;return Ok(RetainedCloneStep::Progress(step.progress()));}
            let value=child.take().ok_or_else(||Self::refusal("dynamic child completed without its original output"))?;
            let state:&mut State=&mut self.state;match state.target.as_mut().unwrap(){DslValue::Array(values)=>values.push(value),DslValue::Object(values)=>values.push((state.key.take().unwrap(),value)),_=>unreachable!()}
            self.state.draining=true;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));
        }
        if self.state.phase==0 {
            self.state.target=Some(match source.get(){
                DslValue::Null=>DslValue::Null,
                DslValue::Bool(value)=>DslValue::Bool(*value),
                DslValue::Number(value)=>DslValue::Number(*value),
                DslValue::String(value)=>{self.state.bytes=Some(Vec::with_capacity(value.len()));self.state.phase=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,retained_capacity_bytes:self.state.bytes.as_ref().unwrap().capacity(),..Default::default()}));},
                DslValue::Bytes(value)=>DslValue::Bytes(Vec::with_capacity(value.len())),
                DslValue::Array(value)=>DslValue::Array(Vec::with_capacity(value.len())),
                DslValue::Object(value)=>DslValue::Object(Vec::with_capacity(value.len())),
            });
            self.state.phase=if matches!(source.get(),DslValue::Null|DslValue::Bool(_)|DslValue::Number(_)){2}else{1};
        }else{
            match source.get(){
                DslValue::String(value)=>{let bytes=self.state.bytes.as_mut().unwrap();if bytes.len()<value.len(){bytes.push(value.as_bytes()[bytes.len()]);}else{self.state.target=Some(DslValue::String(unsafe {String::from_utf8_unchecked(self.state.bytes.take().unwrap())}));self.state.phase=2;}},
                DslValue::Bytes(value)=>{if self.state.index<value.len(){let state:&mut State=&mut self.state;let DslValue::Bytes(bytes)=state.target.as_mut().unwrap()else{unreachable!()};bytes.push(value[state.index]);state.index+=1;}else{self.state.phase=2;}},
                DslValue::Array(value)=>{if self.state.index<value.len(){self.state.child=Some(Box::new(Self::default()));}else{self.state.phase=2;}},
                DslValue::Object(value)=>{
                    if self.state.index==value.len(){self.state.phase=2;}
                    else if self.state.key.is_some(){self.state.child=Some(Box::new(Self::default()));}
                    else if self.state.bytes.is_some(){let index=self.state.index;let bytes=self.state.bytes.as_mut().unwrap();let key=&value[index].0;if bytes.len()<key.len(){bytes.push(key.as_bytes()[bytes.len()]);}else{self.state.key=Some(unsafe {String::from_utf8_unchecked(self.state.bytes.take().unwrap())});}}
                    else{self.state.bytes=Some(Vec::with_capacity(value[self.state.index].0.len()));}
                },
                _=>unreachable!(),
            }
        }
        let progress=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes};
        Ok(if self.state.phase==2{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    fn take(&mut self)->Option<DslValue> {if self.state.phase!=2{return None;}let output=self.state.target.take();self.state.spent|=output.is_some();output}
    fn begin_close(&mut self)->bool {if self.state.closing{return false;}self.state.closing=true;true}
    fn terminal_is_empty(&self)->bool {self.state.closing&&self.empty()}
    fn next_close_depth_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.close_demands(copy)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.close_demands(0)?.release_bytes)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if !self.state.closing{return Err(Self::refusal("dynamic clone must retain custody through begin_close"));}
        let demand=self.close_demands(grant.maximum_copy_bytes)?;
        if !Self::admitted(demand,grant)?{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.state.child.is_some(){return self.close_child(grant);}
        let s:&mut State=&mut self.state;
        if !s.close.is_empty(){return s.close.step_granted(grant);}
        if let Some(step)=s.close.begin_granted(&mut s.bytes,grant)?{return Ok(step);}
        if let Some(step)=s.close.begin_granted(&mut s.key,grant)?{return Ok(step);}
        if let Some(step)=s.close.begin_granted(&mut s.target,grant)?{return Ok(step);}
        close_retained_binding(&mut s.source,grant)
    }
}

impl Drop for DslValueRetainedCloneCursor {
    fn drop(&mut self){assert!(self.empty()||std::thread::panicking(),"dynamic clone dropped before original paid closure");if self.empty(){unsafe {ManuallyDrop::drop(&mut self.state);}}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
