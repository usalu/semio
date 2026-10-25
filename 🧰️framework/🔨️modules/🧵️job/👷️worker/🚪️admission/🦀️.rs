//! 🚪️ Original normal authority admits session births before any source custody moves.
use super::*;
/// 🚚️ Borrows the original source until every copied range and final presence handoff is paid.
pub struct WorkerJobAdmission<'source,J:InteractiveJob+'static>{source:&'source mut WorkerJobSource<J>,params:&'source mut Option<BatchJobParams>,session:Option<WorkerJobSession<J>>,copied:usize,identity:(OperationId,Generation)}
impl<'source,J:InteractiveJob+'static> WorkerJobAdmission<'source,J>{
    /// 📏️ Quotes real destination births independently of original input preparation.
    pub fn birth_demand()->RetirementDemand{WorkerJobSession::<J>::admission_demand()}
    /// ▶️ Leaves source and parameters unchanged unless the original parent admits destination metadata.
    pub fn try_begin(source:&'source mut WorkerJobSource<J>,params:&'source mut Option<BatchJobParams>,control:&mut impl WorkerJobAdmissionControl)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let original=params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission requires original parameters"))?;let identity=(original.operation,original.generation);
        if source.source().is_none()||identity!=control.admission_identity()||source.identity()!=identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission requires its original initialized source identity"))}
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
    fn destination_owner(&self)->&OriginalWorkerJobAuthority<J>{let inner=self.session.as_ref().expect("original staged session remains owned").inner.0.as_ref().expect("original staged Arc remains owned");unsafe{(&*inner.authority.get()).as_ref().expect("original staged authority remains owned")}}
    fn destination_owner_mut(&mut self)->&mut OriginalWorkerJobAuthority<J>{let inner=self.session.as_mut().expect("original staged session remains owned").inner.0.as_ref().expect("original staged Arc remains owned");unsafe{(&mut *inner.authority.get()).as_mut().expect("original staged authority remains owned")}}
    /// 🪪️ Borrows the same prepaid native page before and after its paid payload publication.
    pub fn destination_page_identity(&self)->Option<*const MaybeUninit<u8>>{self.destination_owner().admission_page_identity()}
    /// 🧩️ Observes publication of all initialized payload fields without taking their child ownership.
    pub fn destination_is_initialized(&self)->bool{self.destination_owner().is_initialized()}
    /// 📃️ Quotes the next initialized page slot and cursor, or the final original page publication.
    pub fn destination_initialization_demands(&self)->Result<RetirementDemand,ValueError>{Ok(self.destination_owner().initialization_demands())}
    /// 🎟️ Initializes the original fixed metadata array or publishes its same prepaid native page.
    pub fn advance_destination_initialization(&mut self,control:&mut impl WorkerJobAdmissionControl)->Result<RetainedCloneProgress,ValueError>{
        if control.admission_identity()!=self.identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"destination initialization received another operation identity"))}let grant=control.admission_grant()?;if !control.admission_is_open()||self.params.as_ref().is_some_and(|params|params.cancel.is_cancelled_now()){return Ok(Default::default())}let progress=self.destination_owner_mut().advance_initialization(grant);control.receive_admission(progress)?;Ok(progress)
    }
    /// ⏩️ Copies a bounded original byte range or performs the separately paid final ownership handoff.
    pub fn advance(&mut self,control:&mut impl WorkerJobAdmissionControl)->Result<(Option<WorkerJobSession<J>>,RetainedCloneProgress),ValueError>{
        if control.admission_identity()!=self.identity{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission received another operation identity"))}
        if !self.destination_is_initialized(){return self.advance_destination_initialization(control).map(|progress|(None,progress))}
        let original=self.params.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission lost original parameters"))?;let grant=control.admission_grant()?;
        if !control.admission_is_open()||original.cancel.is_cancelled_now()||grant.maximum_items==0||grant.maximum_depth==0{return Ok((None,Default::default()))}
        let session=self.session.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"staged admission is already consumed"))?;
        let inner=session.inner.0.as_ref().expect("staged destination retains its original Arc");let authority=unsafe{(&mut *inner.authority.get()).as_mut().expect("staged destination retains authority")};
        let (complete,progress)=authority.advance_staged_source(self.source,self.params,&mut self.copied,control)?;
        Ok((if complete{self.session.take()}else{None},progress))
    }
}


#[path="🏗️preparation/🦀️.rs"]
mod native_preparation;
pub use native_preparation::WorkerJobSessionPreparation;
