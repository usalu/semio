//! 🚪️ Original normal authority admits session births before any source custody moves.
use super::*;
/// 🪙️ Carries the original identity, clock and cumulative normal receipt into physical session admission.
pub trait WorkerJobAdmissionControl {
    fn admission_identity(&self)->(OperationId,Generation);
    fn admission_grant(&self)->Result<RetainedCloneGrant,ValueError>;
    fn admission_is_open(&self)->bool;
    /// 🪪️ The original cancellation token of the admitting context, when it carries one.
    fn admission_cancel(&self)->Option<&CancelToken>{None}
    fn receive_admission(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>;
}
/// 🧾️ Borrows a caller-authored normal budget and recipient without allocating another context ledger.
pub struct WorkerJobAdmissionContext<'recipient>{operation:OperationId,generation:Generation,budget:StepBudget,now_us:fn()->Option<u64>,recipient:&'recipient mut RetainedCloneProgress}
impl<'recipient> WorkerJobAdmissionContext<'recipient>{
    pub fn new(operation:OperationId,generation:Generation,budget:StepBudget,now_us:fn()->Option<u64>,recipient:&'recipient mut RetainedCloneProgress)->Result<Self,ValueError>{if !recipient.fits(budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission recipient exceeds its original normal authority"))}Ok(Self{operation,generation,budget,now_us,recipient})}
}
impl WorkerJobAdmissionControl for WorkerJobAdmissionContext<'_>{
    fn admission_identity(&self)->(OperationId,Generation){(self.operation,self.generation)}
    fn admission_grant(&self)->Result<RetainedCloneGrant,ValueError>{if !self.recipient.fits(self.budget.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session admission recipient exceeds its original normal authority"))}Ok(RetainedCloneGrant{maximum_items:self.budget.retained.maximum_items-self.recipient.copied_items,maximum_copy_bytes:self.budget.retained.maximum_copy_bytes-self.recipient.copied_bytes,maximum_capacity_bytes:self.budget.retained.maximum_capacity_bytes-self.recipient.retained_capacity_bytes,maximum_release_bytes:self.budget.retained.maximum_release_bytes-self.recipient.released_bytes,maximum_depth:self.budget.retained.maximum_depth})}
    fn admission_is_open(&self)->bool{self.budget.fuel>0&&(self.now_us)().is_some_and(|now|now<self.budget.deadline_us)}
    fn receive_admission(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{if !progress.fits(self.admission_grant()?){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session birth receipt exceeds remaining original normal authority"))}self.recipient.copied_items+=progress.copied_items;self.recipient.copied_bytes+=progress.copied_bytes;self.recipient.retained_capacity_bytes+=progress.retained_capacity_bytes;self.recipient.released_bytes+=progress.released_bytes;Ok(())}
}
impl WorkerJobAdmissionControl for StepContext<'_>{
    fn admission_identity(&self)->(OperationId,Generation){(self.operation,self.generation)}
    fn admission_grant(&self)->Result<RetainedCloneGrant,ValueError>{if !self.retained_progress().fits(self.retained){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"session context recipient exceeds original normal authority"))}Ok(self.retained_grant())}
    fn admission_is_open(&self)->bool{!self.is_cancelled()&&!self.should_yield()}
    fn admission_cancel(&self)->Option<&CancelToken>{Some(self.original_cancel_token())}
    fn receive_admission(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{self.consume_retained(progress)}
}

/// 🏗️ Initializes one original job in its source storage under separately admitted input preparation.
/// 🧷️ Implementors retain every partial child owner until completion or their own paid retirement.
pub unsafe trait WorkerJobInitializer<J> {
    fn initialization_demand(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>;
    unsafe fn initialize_step(&mut self,target:*mut MaybeUninit<J>,grant:RetainedCloneGrant)->Result<WorkerJobInitializationStep,ValueError>;
    fn initialization_input_is_empty(&self)->bool;
    fn partial_retirement_demands(&self,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>;
    unsafe fn close_partial_step(&mut self,target:*mut MaybeUninit<J>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
    fn partial_terminal_is_empty(&self)->bool;
}
/// 🧾️ Receipts initialized source writes separately from later destination admission.
pub struct WorkerJobInitializationStep {pub progress:RetainedCloneProgress,pub complete:bool}
impl<J> WorkerJobSource<J> {
    /// 🌱️ Starts original input storage in place without moving an uninitialized job carrier.
    pub fn start_preparation<'source>(target:&'source mut MaybeUninit<Self>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(&'source mut Self,RetainedCloneProgress)>,ValueError>{
        let grant=control.admission_grant()?;let copied_bytes=0;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0||!control.admission_is_open(){return Ok(None)}
        let identity=control.admission_identity();
        unsafe{std::ptr::addr_of_mut!((*target.as_mut_ptr()).present).write(false);std::ptr::addr_of_mut!((*target.as_mut_ptr()).operation).write(identity.0);std::ptr::addr_of_mut!((*target.as_mut_ptr()).generation).write(identity.1);}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()};control.receive_admission(progress)?;
        Ok(Some((unsafe{target.assume_init_mut()},progress)))
    }
    /// 🖋️ Admits declared initialized source fields and their final presence before any writer enters.
    pub fn prepare_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,control:&mut impl WorkerJobAdmissionControl)->Result<WorkerJobInitializationStep,ValueError>{
        if self.present{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job source is already initialized"))}
        if control.admission_identity()!=(self.operation,self.generation){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"source preparation received another operation identity"))}
        let grant=control.admission_grant()?;let presence=0;
        if !control.admission_is_open()||grant.maximum_items==0||grant.maximum_copy_bytes<presence||grant.maximum_depth==0{return Ok(WorkerJobInitializationStep{progress:Default::default(),complete:false})}
        let child=grant;let demand=initializer.initialization_demand(child.maximum_copy_bytes)?;
        if child.maximum_copy_bytes<demand.copy_bytes||child.maximum_capacity_bytes<demand.capacity_bytes||child.maximum_release_bytes<demand.release_bytes||child.maximum_depth<demand.depth{return Ok(WorkerJobInitializationStep{progress:Default::default(),complete:false})}
        let mut step=match unsafe{initializer.initialize_step(&mut self.original,child)}{Ok(step)=>step,Err(error)=>{semio_framework_value::retained_clone::admit_retained_clone_progress(child,error.retained_progress(),"original failed source preparation")?;control.receive_admission(error.retained_progress())?;return Err(error)}};
        semio_framework_value::retained_clone::admit_retained_clone_progress(child,step.progress,"original job source preparation")?;
        if step.complete{self.present=true;step.progress.copied_bytes+=presence;}
        control.receive_admission(step.progress)?;Ok(step)
    }
    /// 🔎️ Borrows the same initialized source throughout staged destination admission.
    pub fn source(&self)->Option<&J>{self.original()}
}
/// 🚚️ Borrows the original source until every copied range and final presence handoff is paid.
pub struct WorkerJobAdmission<'source,J:InteractiveJob+'static>{source:&'source mut WorkerJobSource<J>,params:&'source mut Option<BatchJobParams>,session:Option<WorkerJobSession<J>>,copied:usize,identity:(OperationId,Generation)}
impl<'source,J:InteractiveJob+'static> WorkerJobAdmission<'source,J>{
    /// 📏️ Quotes real destination births independently of original input preparation.
    pub fn birth_demand()->RetirementDemand{WorkerJobSession::<J>::admission_demand()}
    /// 🚪️ Leaves source and parameters unchanged unless the original parent admits destination metadata.
    pub fn try_begin(source:&'source mut WorkerJobSource<J>,params:&'source mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let original=params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission requires original parameters"))?;let identity=(original.operation,original.generation);
        if source.source().is_none()||identity!=control.admission_identity()||(source.operation,source.generation)!=identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission requires its original initialized source identity"))}
        let grant=control.admission_grant()?;let demand=Self::birth_demand();
        if !control.admission_is_open()||original.cancel.is_cancelled_now()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Ok(None)}
        let Some(session)=WorkerJobSession::<J>::birth_storage(identity.0,identity.1,control)?else{return Ok(None)};
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:0};control.receive_admission(progress)?;
        Ok(Some((Self{source,params,session:Some(session),copied:0,identity},progress)))
    }
    /// 📍️ Observes the exact original source without moving its initialized carrier.
    pub fn source(&self)->&J{self.source.source().expect("staged admission retains original source")}
    /// 🧮️ Observes the paid initialized byte frontier before the next caller turn.
    pub fn copied_bytes(&self)->usize{self.copied}
    fn destination_owner(&self)->&WorkerJobAuthorityOwner<J>{let inner=self.session.as_ref().expect("original staged session remains owned").inner.0.as_ref().expect("original staged Arc remains owned");unsafe{(&*inner.authority.get()).as_ref().expect("original staged authority remains owned")}}
    fn destination_owner_mut(&mut self)->&mut WorkerJobAuthorityOwner<J>{let inner=self.session.as_mut().expect("original staged session remains owned").inner.0.as_ref().expect("original staged Arc remains owned");unsafe{(&mut *inner.authority.get()).as_mut().expect("original staged authority remains owned")}}
    /// 📍️ Borrows the same prepaid native page before and after its paid payload publication.
    pub fn destination_page_identity(&self)->Option<*const std::mem::MaybeUninit<u8>>{let owner=self.destination_owner();owner.admission_fault_source.as_ref().map(JobPayloadPageSource::backing_identity).or_else(||owner.preadmitted_fault.original().and_then(|payload|payload.pages[0].as_ref().map(|page|page.source.backing_identity())))}
    /// 🧩️ Observes publication of all initialized payload fields without taking their child ownership.
    pub fn destination_is_initialized(&self)->bool{self.destination_owner().preadmitted_fault.original().is_some()}
    /// 📏️ Quotes the next initialized page slot and cursor, or the final original page publication.
    pub fn destination_initialization_demands(&self)->Result<RetirementDemand,ValueError>{Ok(self.destination_owner().original_admission_initialization_demands())}
    /// 🎟️ Initializes the original fixed metadata array or publishes its same prepaid native page.
    pub fn advance_destination_initialization(&mut self,control:&mut impl WorkerJobAdmissionControl)->Result<RetainedCloneProgress,ValueError>{
        if control.admission_identity()!=self.identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"destination initialization received another operation identity"))}let grant=control.admission_grant()?;if !control.admission_is_open()||self.params.as_ref().is_some_and(|params|params.cancel.is_cancelled_now()){return Ok(Default::default())}let progress=self.destination_owner_mut().advance_original_admission_initialization(grant);control.receive_admission(progress)?;Ok(progress)
    }
    /// 🎟️ Copies a bounded original byte range or performs the separately paid final ownership handoff.
    pub fn advance(&mut self,control:&mut impl WorkerJobAdmissionControl)->Result<(Option<WorkerJobSession<J>>,RetainedCloneProgress),ValueError>{
        if control.admission_identity()!=self.identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission received another operation identity"))}
        if !self.destination_is_initialized(){return self.advance_destination_initialization(control).map(|progress|(None,progress))}
        let original=self.params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission lost original parameters"))?;let grant=control.admission_grant()?;
        if !control.admission_is_open()||original.cancel.is_cancelled_now()||grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,Default::default()))}
        let session=self.session.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission is already consumed"))?;
        let inner=session.inner.0.as_ref().expect("staged destination retains its original Arc");let authority=unsafe{(&mut *inner.authority.get()).as_mut().expect("staged destination retains authority")};
        let remaining=size_of::<J>()-self.copied;
        if remaining>0{
            if grant.maximum_copy_bytes==0{return Ok((None,Default::default()))}let bytes=remaining.min(grant.maximum_copy_bytes);
            let source=self.source.source().expect("staged source presence is original")as*const J;let target=authority.job.original.as_mut_ptr();
            unsafe{std::ptr::copy_nonoverlapping(source.cast::<MaybeUninit<u8>>().add(self.copied),target.cast::<MaybeUninit<u8>>().add(self.copied),bytes);}
            self.copied+=bytes;let progress=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};control.receive_admission(progress)?;return Ok((None,progress))
        }
        let copied_bytes=0;
        if grant.maximum_copy_bytes<copied_bytes{return Ok((None,Default::default()))}
        authority.params=self.params.take();self.source.present=false;authority.job.present=true;authority.close_stage=0;
        let output=self.session.take();let progress=RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()};control.receive_admission(progress)?;Ok((output,progress))
    }
}

#[cfg(test)]
#[path="🚚️staged/🎟️metadata/🧪️tests/🦀️.rs"]
mod original_staged_metadata_tests;

#[path="🏗️preparation/🦀️.rs"]
mod native_preparation;
pub use native_preparation::WorkerJobSessionPreparation;
