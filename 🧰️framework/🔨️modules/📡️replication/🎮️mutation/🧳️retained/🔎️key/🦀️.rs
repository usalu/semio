//! 🔎️ Keyed lookup retains genuine immutable list/key sources and compares only one admitted byte chunk.
use super::retained_mutation_demand_fits;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

/// 🎯️ Complete key uniqueness is decided before any projection write.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedKeyMatch { Missing, Unique(usize), Duplicate(usize,usize) }

/// 🧷️ Only fixed scan state and source aliases are retained; list rows and keys are never copied.
#[derive(Default)]
pub struct RetainedKeyLookup {
    rows:Option<RetainedCloneBinding>,
    key:Option<RetainedCloneBinding>,
    index:usize,
    offset:usize,
    first:Option<usize>,
    result:Option<RetainedKeyMatch>,
    closing:bool,
}

impl RetainedKeyLookup {
    /// 🌱️ Constructs no payload, list census or child allocation.
    pub fn new()->Self { Self::default() }

    /// 📏️ Quotes the next genuine alias, one bounded key comparison, or final fixed result.
    pub fn demand<R>(&self,rows:RetainedCloneRef<'_,Vec<R>>,key:RetainedCloneRef<'_,String>,row_key:fn(&R)->&str,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError> {
        if self.closing {return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"key lookup already closing"));}
        if self.rows.is_none(){return Ok(RetirementDemand{copy_bytes:rows.binding_copy_bytes(),depth:1,..Default::default()});}
        if self.key.is_none(){return Ok(RetirementDemand{copy_bytes:key.binding_copy_bytes(),depth:1,..Default::default()});}
        if self.result.is_some(){return Ok(Default::default());}
        let Some(row)=rows.get().get(self.index)else{return Ok(RetirementDemand{copy_bytes:size_of::<RetainedKeyMatch>(),depth:1,..Default::default()});};
        let current=row_key(row);
        let count=if current.len()!=key.get().len(){0}else{current.len().saturating_sub(self.offset).min((maximum_body_bytes.saturating_sub(size_of::<usize>()*2)/2).max(1)).min(65_536)};
        let copy_bytes=count.checked_mul(2).and_then(|bytes|bytes.checked_add(size_of::<usize>()*2)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"key lookup chunk extent overflow"))?;
        Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})
    }

    /// ▶️ Scans immutable original rows under one physical work quote and reports no transient key clone.
    pub fn advance<R>(&mut self,rows:RetainedCloneRef<'_,Vec<R>>,key:RetainedCloneRef<'_,String>,row_key:fn(&R)->&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let demand=self.demand(rows,key,row_key,grant.maximum_copy_bytes)?;
        if !retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(progress)=rows.bind(&mut self.rows,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        if let Some(progress)=key.bind(&mut self.key,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.result.is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        let Some(row)=rows.get().get(self.index)else{self.result=Some(self.first.map_or(RetainedKeyMatch::Missing,RetainedKeyMatch::Unique));return Ok(RetainedCloneStep::Complete(progress));};
        let current=row_key(row).as_bytes();let needle=key.get().as_bytes();
        if current.len()!=needle.len(){self.index+=1;self.offset=0;return Ok(RetainedCloneStep::Progress(progress));}
        let count=(demand.copy_bytes-size_of::<usize>()*2)/2;
        let end=self.offset+count;
        if current[self.offset..end]!=needle[self.offset..end]{self.index+=1;self.offset=0;return Ok(RetainedCloneStep::Progress(progress));}
        self.offset=end;
        if end==current.len(){
            if let Some(first)=self.first{self.result=Some(RetainedKeyMatch::Duplicate(first,self.index));return Ok(RetainedCloneStep::Complete(progress));}
            self.first=Some(self.index);self.index+=1;self.offset=0;
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    /// 👁️ Borrows the resolved fixed result only after complete unique-key preflight.
    pub fn result(&self)->Option<RetainedKeyMatch>{self.result}

    /// 🛑️ Stops all scans before settling the original immutable source aliases.
    pub fn begin_close(&mut self){self.closing=true;}

    /// ♻️ Quotes the exact next source alias close without taking list or key ownership.
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let binding=if self.key.is_some(){&self.key}else{&self.rows};
        Ok(RetirementDemand{copy_bytes:RetainedCloneBinding::copy_demand(binding)?,capacity_bytes:RetainedCloneBinding::capacity_demand(binding,body)?,release_bytes:RetainedCloneBinding::release_demand(binding)?,depth:RetainedCloneBinding::depth_demand(binding)?})
    }

    /// 🪜️ Closes one genuine source alias at a time through its installed typed authority.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"key lookup closure not started"));}
        if !retained_mutation_demand_fits(self.close_demand(grant.maximum_copy_bytes)?,grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let step=RetainedCloneBinding::close_one(if self.key.is_some(){&mut self.key}else{&mut self.rows},grant)?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }

    /// 🫙️ No original list/key alias remains.
    pub fn terminal_is_empty(&self)->bool{self.rows.is_none()&&self.key.is_none()}
}

impl Drop for RetainedKeyLookup{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"retained key lookup requires original source closure");}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
