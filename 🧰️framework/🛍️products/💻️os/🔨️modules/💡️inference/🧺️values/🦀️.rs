//! 🧺️ Original source ordinals retain typed values and every partial parent/output owner.
use super::{InferenceWorkFailure,OriginalInferenceScope};
use semio_framework_job::{OriginalCallerSourceLoan,StepContext,OriginalTurnFailure};
use semio_framework_value::{list::PagedList,retirement::{RetireOwned,controlled::ControlledRetirement},RetainedCloneProgress,ValueError,ValueRefusalKind};

type Rows<V> = PagedList<Option<V>,{usize::MAX}>;
type Payload<V> = (Rows<V>,Vec<V>,Option<V>,Option<V>,Option<ControlledRetirement<Vec<V>>>);

pub struct InferenceValues<V:RetireOwned> {
    pub(super) rows: Rows<V>,
    pub(super) parents: Vec<V>,
    pub(super) parent: Option<V>,
    pub(super) output: Option<V>,
    parents_retirement: Option<ControlledRetirement<Vec<V>>>,
    extent: Option<usize>,
    source: Option<OriginalCallerSourceLoan>,
    scope: Option<OriginalInferenceScope>,
    retirement: Option<ControlledRetirement<Payload<V>>>,
    closing: bool,
}

impl<V:RetireOwned> Default for InferenceValues<V> {
    fn default()->Self { Self { rows:Default::default(),parents:Vec::new(),parent:None,output:None,parents_retirement:None,extent:None,source:None,scope:None,retirement:None,closing:false } }
}

impl<V:RetireOwned> InferenceValues<V> {
    pub fn new()->Self { Self::default() }
    pub fn get(&self,source:usize)->Option<&V> { self.rows.get(source)?.as_ref() }
    pub fn iter(&self)->impl Iterator<Item=(usize,&V)> { (0..self.rows.len()).filter_map(|source|self.get(source).map(|value|(source,value))) }
    pub fn is_empty(&self)->bool { self.iter().next().is_none() }
    pub fn source_extent(&self)->Option<usize> { self.extent }
    pub fn rows_ready(&self)->bool { self.extent.is_some_and(|extent|self.rows.len()==extent) }

    pub(super) fn reserve_parents_step(&mut self,count:usize,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        self.validate(cx)?;
        if self.closing||self.parents_retirement.is_some()||!self.parents.is_empty(){return Err(changed().into())}
        if self.parents.capacity()>=count{return Ok(true)}
        let bytes=count.checked_mul(size_of::<V>()).ok_or_else(changed)?;let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(false)}
        if self.parents.capacity()!=0{return Err(changed().into())}
        if self.parents.try_reserve_exact(count).is_err(){cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"inferred parent vector allocation failed").into())}
        let actual=self.parents.capacity().checked_mul(size_of::<V>()).ok_or_else(changed)?;
        let receipt=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:actual,..Default::default()};cx.consume_fuel(1);
        if actual>grant.maximum_capacity_bytes{return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvariantViolated,"inferred parent vector exceeded its admitted physical backing").with_retained_progress(receipt),cx).into())}
        cx.consume_retained(receipt)?;Ok(false)
    }

    pub(super) fn push_parent_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        self.validate(cx)?;
        if self.closing||self.parent.is_none()||self.parents.len()==self.parents.capacity(){return Err(changed().into())}
        if cx.should_yield()||cx.is_cancelled()||!metadata(cx,size_of::<V>()+size_of::<Option<V>>(),1)?{return Ok(false)}
        self.parents.push(self.parent.take().unwrap());Ok(true)
    }

    pub(super) fn close_parents_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        self.validate(cx)?;if cx.retirement_should_yield(){return Ok(false)}
        if self.parents_retirement.is_none(){
            if self.parents.is_empty()&&self.parents.capacity()==0{return Ok(true)}
            if !metadata(cx,size_of::<Vec<V>>(),1)?{return Ok(false)}
            match ControlledRetirement::new(std::mem::take(&mut self.parents)){Ok(owner)=>self.parents_retirement=Some(owner),Err((error,parents))=>{self.parents=parents;return Err(error.into())}}
            return Ok(false)
        }
        let owner=self.parents_retirement.as_mut().unwrap();
        if !owner.terminal_is_empty(){let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;return Ok(false)}
        if !metadata(cx,size_of::<Option<ControlledRetirement<Vec<V>>>>(),1)?{return Ok(false)}self.parents_retirement=None;Ok(true)
    }

    fn validate(&self,cx:&StepContext<'_>)->Result<(),InferenceWorkFailure> {
        if let Some(scope)=self.scope.as_ref() { if !scope.matches(cx)? { return Err(changed().into()); } }
        Ok(())
    }

    /// 🎟️ Admits a genuine strong source loan before any retained value row is born.
    pub fn admit_source_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure> {
        self.validate(cx)?;
        if self.closing { return Err(changed().into()); }
        if self.source.is_some() { return Ok(true); }
        let scope=OriginalInferenceScope::capture(cx)?;
        let parent=cx.original_source_parent()?;
        let Some(source)=parent.admit_loan(cx)? else { return Ok(false); };
        self.scope=Some(scope);self.source=Some(source);Ok(false)
    }

    /// 🧱️ Initializes one source-ordinal row or one actual fixed-page backing allocation.
    pub fn initialize_step(&mut self,extent:usize,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure> {
        self.validate(cx)?;
        if self.closing||self.source.is_none() { return Err(changed().into()); }
        if cx.should_yield()||cx.is_cancelled() { return Ok(false); }
        if let Some(original)=self.extent { if original!=extent { return Err(changed().into()); } }
        else { if !metadata(cx,0,1)? { return Ok(false); } self.extent=Some(extent);return Ok(false); }
        if self.rows.len()==extent { return Ok(true); }
        if !self.rows.has_reserved_slot() {
            match self.rows.reserve_capacity_one_funded(extent,cx.retained_grant()) {
                Ok(progress)=>{if progress.copied_items==0 { return Ok(false); } cx.consume_retained(progress)?;cx.consume_fuel(1);return Ok(false);},
                Err(error)=>{cx.consume_fuel(1);let original=ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress {retained_capacity_bytes:error.allocated_bytes,..Default::default()});return Err(OriginalTurnFailure::debit_original(original,cx).into());}
            }
        }
        let depth=self.rows.next_push_depth_demand().map_err(ValueError::from)?;
        if !metadata(cx,size_of::<Option<V>>(),depth)? { return Ok(false); }
        self.rows.push_reserved(None).map_err(|_|changed())?;Ok(false)
    }

    /// 📥️ A refused publication retains the exact typed output in its donor.
    pub fn publish_step(&mut self,source:usize,donor:&mut Option<V>,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure> {
        self.validate(cx)?;
        if self.closing||!self.rows_ready()||donor.is_none()||self.rows.get(source).is_none_or(Option::is_some) { return Err(changed().into()); }
        if cx.should_yield()||cx.is_cancelled()||!metadata(cx,size_of::<Option<V>>(),2)? { return Ok(false); }
        *self.rows.get_mut(source).ok_or_else(changed)?=donor.take();Ok(true)
    }

    pub(super) fn publish_output_step(&mut self,source:usize,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure>{
        self.validate(cx)?;
        if self.closing||!self.rows_ready()||self.output.is_none()||self.rows.get(source).is_none_or(Option::is_some){return Err(changed().into())}
        if cx.should_yield()||cx.is_cancelled()||!metadata(cx,size_of::<Option<V>>(),2)?{return Ok(false)}
        *self.rows.get_mut(source).ok_or_else(changed)?=self.output.take();Ok(true)
    }

    /// 📏️ Quotes the actual payload cursor or the original source-loan return selected by close_step.
    pub fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if !self.closing{return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Payload<V>>(),depth:1,..Default::default()})}if let Some(owner)=self.retirement.as_ref(){if !owner.terminal_is_empty(){return owner.selected_demand(body)}}if let Some(source)=self.source.as_ref(){return Ok(source.retirement_demands())}Ok(Default::default())}
    /// ♻️ Closes typed payload before returning its original strong source loan.
    pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,InferenceWorkFailure> {
        self.validate(cx)?;
        if cx.retirement_should_yield() { return Ok(false); }
        if !self.closing {
            if !V::controlled_retirement_supported() { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"inferred value has no original retirement producer").into()); }
            if !metadata(cx,size_of::<Payload<V>>(),1)? { return Ok(false); }
            let payload=(std::mem::take(&mut self.rows),std::mem::take(&mut self.parents),self.parent.take(),self.output.take(),self.parents_retirement.take());
            match ControlledRetirement::new(payload) {
                Ok(owner)=>self.retirement=Some(owner),
                Err((error,(rows,parents,parent,output,parents_retirement)))=>{self.rows=rows;self.parents=parents;self.parent=parent;self.output=output;self.parents_retirement=parents_retirement;return Err(error.into());}
            }
            self.closing=true;return Ok(false);
        }
        if let Some(owner)=self.retirement.as_mut() {
            if !owner.terminal_is_empty() {
                let demand=owner.selected_demand(cx.retained_grant().maximum_copy_bytes)?;
                semio_framework_job::close_original_retained_step(cx,demand,|grant|owner.step(grant))?;
                return Ok(false);
            }
            self.retirement=None;
        }
        if let Some(source)=self.source.as_mut() {
            let parent=cx.original_source_parent()?;
            source.close_step(parent,cx)?;
            if !source.terminal_is_empty() { return Ok(false); }
            self.source=None;
        }
        self.scope=None;self.extent=None;Ok(true)
    }

    pub fn terminal_is_empty(&self)->bool { self.rows.terminal_is_empty()&&self.parents.is_empty()&&self.parents.capacity()==0&&self.parent.is_none()&&self.output.is_none()&&self.parents_retirement.is_none()&&self.retirement.is_none()&&self.source.is_none() }
}

impl<V:RetireOwned> Drop for InferenceValues<V> {
    fn drop(&mut self) { assert!(std::thread::panicking()||self.terminal_is_empty(),"inferred original values require paid payload closure and source return"); }
}

fn changed()->ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated,"original inferred value source or publication changed") }
fn metadata(cx:&mut StepContext<'_>,bytes:usize,depth:usize)->Result<bool,InferenceWorkFailure> {
    let grant=cx.retained_grant();
    if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth<depth { return Ok(false); }
    cx.consume_retained(RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
