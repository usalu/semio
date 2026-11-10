//! 🔏️ Pack, authority and publication of the window config: the pack, the authority, the erased publication trait and the typed publication with its drop. See [`super`].

use super::*;
#[path="📦️source/🦀️.rs"]
mod pack_source;
pub use pack_source::WindowConfigPackSource;

/// 💾 Persisted envelope for one exact window config partition.
#[derive(semio_framework_value::RetireOwned)]
pub struct WindowConfigPack {
    pub window_id: String,
    pub window_kind_id: String,
    pub source:WindowConfigPackSource,
}

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
#[doc(hidden)]
pub struct WindowConfigAuthority {
    pub window_id: String,
    pub window_kind_id: String,
    pub generation: u64,
    pub revision: [u8; 32],
    pub snapshot: WindowConfigSnapshot,
    pub pending_snapshot:Option<WindowConfigSnapshot>,
    pub retired_snapshots:semio_framework_value_retirement_queue::RetirementQueue,
}

#[doc(hidden)]
pub trait ErasedWindowConfigPublication: Send {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn window_kind_id(&self) -> &str;
    fn phase(&self) -> store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase;
    fn accepted_receipt(&self)->Option<store::os_store::one_item::LaneItemReceipt>;
    fn fault(&self) -> Option<&str>;
    fn preparation_refusal(&self) -> Option<&ValueError>;
    fn acknowledge(&mut self) -> bool;
    fn begin_close(&mut self);
    fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll;
    fn ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn close_ingress(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll;
    fn terminal_is_empty(&self) -> bool;
}

#[doc(hidden)]
pub struct TypedWindowConfigPublication<O: WindowConfigOwner> {
    #[doc(hidden)]
    pub window_id: ControlledRetirement<String>,
    pub operation: semio_framework_job::OperationId,
    #[doc(hidden)]
    pub generation: u64,
    #[doc(hidden)]
    pub revision: [u8;32],
    #[doc(hidden)]
    pub actor: Option<ControlledRetirement<semio_framework_value::SharedUtf8>>,
    #[doc(hidden)]
    pub pending: std::mem::ManuallyDrop<Vec<O::Mutation>>,
    pub transaction: Option<protocol::TransactionRef>,
    #[doc(hidden)]
    pub publication: std::mem::ManuallyDrop<Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<O::State, O::Mutation>>>,
    pub ingress: ControlledRetirement<FactoryBoxedPublication<O::Mutation>>,
    pub cancelled: Option<ControlledRetirement<Box<FactoryBoxedValue<O::Mutation>>>>,
    pub closing: bool,
}

impl<O: WindowConfigOwner> TypedWindowConfigPublication<O> {
    fn pending_bytes(&self)->usize{self.pending.capacity().checked_mul(size_of::<O::Mutation>()).expect("admitted original window backing remains bounded")}
    fn controlled_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{
        Ok(owner.next_demand(body)?)
    }
    fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.pending.is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<O::Mutation>()+size_of::<FactoryBoxedValue<O::Mutation>>()+size_of::<Option<ControlledRetirement<Box<FactoryBoxedValue<O::Mutation>>>>>(),depth:1,..Default::default()});}
        if self.pending_bytes()!=0{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<O::Mutation>>(),release_bytes:self.pending_bytes(),depth:1,..Default::default()});}
        if let Some(actor)=self.actor.as_ref(){return if actor.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<semio_framework_value::SharedUtf8>>>(),depth:1,..Default::default()})}else{Self::controlled_demand(actor,body)};}
        if !self.ingress.terminal_is_empty(){return Self::controlled_demand(&self.ingress,body);}
        if let Some(cancelled)=self.cancelled.as_ref(){return if cancelled.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<Box<FactoryBoxedValue<O::Mutation>>>>>(),depth:1,..Default::default()})}else{Self::controlled_demand(cancelled,body)};}
        if let Some(publication)=self.publication.as_ref(){return if publication.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<O::State,O::Mutation>>>(),depth:1,..Default::default()})}else{publication.retirement_demands(body)};}
        Self::controlled_demand(&self.window_id,body)
    }
}

impl<O: WindowConfigOwner> Drop for TypedWindowConfigPublication<O>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||self.terminal_is_empty(),"original window publication abandoned before paid closure");
        if self.terminal_is_empty(){unsafe{std::mem::ManuallyDrop::drop(&mut self.pending);std::mem::ManuallyDrop::drop(&mut self.publication);}}
    }
}

impl<O: WindowConfigOwner> ErasedWindowConfigPublication for TypedWindowConfigPublication<O> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn window_kind_id(&self) -> &str {
        O::WINDOW_KIND_ID
    }

    fn phase(&self) -> store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase {
        self.publication.as_ref().map_or(if self.closing{store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Closing}else{store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Preparing},|publication|publication.phase())
    }

    fn accepted_receipt(&self)->Option<store::os_store::one_item::LaneItemReceipt>{self.publication.as_ref().and_then(|publication|publication.accepted_receipt())}

    fn fault(&self) -> Option<&str> {
        self.publication.as_ref().and_then(|publication|publication.fault())
    }
    fn preparation_refusal(&self) -> Option<&ValueError> { self.publication.as_ref().and_then(|publication|publication.preparation_refusal()) }

    fn acknowledge(&mut self) -> bool {
        let accepted=self.publication.as_mut().is_some_and(|publication|publication.acknowledge());
        if accepted{self.closing=true;}
        accepted
    }

    fn begin_close(&mut self) {
        self.closing=true;
        if let Some(publication)=self.publication.as_mut(){publication.begin_close();}
    }

    fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{
        use semio_framework_value::RetirementPollExit as Exit;
        let result=(||->Result<RetainedCloneStep,Exit>{
            if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
            if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original window publication requires its close decision").into())}
            let demand=self.close_demand(body)?;
            if effect.maximum_work_units()==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
            if !self.pending.is_empty(){
                if self.publication.is_some()||self.pending.len()!=1||!self.ingress.original_is_untouched(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original window cancellation lost its unconsumed typed carrier").into())}
                let original=self.pending.pop().expect("quoted original window mutation remains");
                let carrier=self.ingress.take_original().expect("quoted original carrier remains untouched").restore(original);
                self.cancelled=Some(ControlledRetirement::new(carrier).map_err(|(error,_)|error)?);
            }else if self.pending_bytes()!=0{drop(std::mem::take(&mut *self.pending));}
            else if let Some(actor)=self.actor.as_mut(){if actor.terminal_is_empty(){drop(self.actor.take());}else{return Ok(RetainedCloneStep::Progress(Exit::resume(actor.step(body,effect))?.progress()))}}
            else if !self.ingress.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(Exit::resume(self.ingress.step(body,effect))?.progress()))}
            else if let Some(cancelled)=self.cancelled.as_mut(){if cancelled.terminal_is_empty(){drop(self.cancelled.take());}else{return Ok(RetainedCloneStep::Progress(Exit::resume(cancelled.step(body,effect))?.progress()))}}
            else if let Some(publication)=self.publication.as_mut(){if publication.terminal_is_empty(){drop(self.publication.take());}else{return Ok(RetainedCloneStep::Progress(publication.close_step(effect)?.progress()))}}
            else{return Ok(RetainedCloneStep::Progress(Exit::resume(self.window_id.step(body,effect))?.progress()))}
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))
        })();
        Exit::settle(effect,result)
    }

    fn ingress_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{Ok(self.ingress.next_demand(body)?)}
    fn close_ingress(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{self.ingress.step(body,effect)}
    fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close_demand(body)}
    fn terminal_is_empty(&self) -> bool {
        self.closing&&self.pending.is_empty()&&self.pending_bytes()==0&&self.transaction.is_none()&&self.actor.is_none()&&self.publication.is_none()&&self.ingress.terminal_is_empty()&&self.cancelled.is_none()&&self.window_id.terminal_is_empty()
    }
}
