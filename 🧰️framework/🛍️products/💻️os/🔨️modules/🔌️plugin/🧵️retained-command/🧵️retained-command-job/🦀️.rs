//! 🧵️ Retained command job: the retained command inputs and payload, the phase and publication enums, the job record, its admission, close, checkpoint, restore and reducer advance and the interactive job implementation. See [`super`].

use super::*;

//#region 🧳️Payload
/// 🧳️ Owned command roots and completion authority admitted into a retained payload.
pub struct ArtifactRetainedCommandInputs<A: ArtifactApp> {
    pub command: A::Command,
    pub snapshot: Arc<A::Snapshot>,
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub history: Arc<HistoryView>,
    pub interaction_state: Arc<protocol::InteractionState>,
    pub interaction_hover: Arc<InteractionHoverState>,
    pub context: Option<ArtifactOwnedContextHandle<A>>,
    pub operation: AppOperationContext,
    pub completion: ArtifactToolCompletion<A>,
}

pub struct ArtifactRetainedCommandPayload<A: ArtifactApp> {
    pub command: A::Command,
    pub snapshot: Arc<A::Snapshot>,
    pub snapshot_read: store::os_store::snapshot_read::SnapshotRead<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub history: Arc<HistoryView>,
    pub interaction_state: Arc<protocol::InteractionState>,
    pub interaction_hover: Arc<InteractionHoverState>,
    pub context: Option<ArtifactOwnedContextHandle<A>>,
    pub operation: AppOperationContext,
    pub completion: ArtifactToolCompletion<A>,
    pub command_id: fn(&A::Command) -> &'static str,
    pub maximum_raw_bytes: usize,
    pub maximum_work_items: usize,
    pub admission_refusal:Option<ArtifactRetainedAdmissionRefusal>,
    pub work: Box<dyn ArtifactCommandWork<A>>,
    pub(crate) producer_window_config:Option<semio_framework_plugin_window_config::WindowConfigSnapshot>,
    pub(crate) producer_instance_operation_owner:Option<semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle>,
    pub(crate) producer_output_chunks:Option<semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks>,
    pub(crate) producer_controller_id:Option<String>,
    pub(crate) producer_tool_id:Option<String>,
    pub(crate) producer_payload_schema_id:Option<String>,
}

impl<A: ArtifactApp> ArtifactRetainedCommandPayload<A> {
    pub fn new(inputs: ArtifactRetainedCommandInputs<A>, command_id: fn(&A::Command) -> &'static str, maximum_raw_bytes: usize, maximum_work_items: usize, work: Box<dyn ArtifactCommandWork<A>>) -> Self {
        let admission_refusal=admission::admit_limits(maximum_raw_bytes,maximum_work_items);
        let ArtifactRetainedCommandInputs { command, snapshot, snapshot_read, config, history, interaction_state, interaction_hover, context, operation, completion } = inputs;
        Self { command, snapshot, snapshot_read, config, history, interaction_state, interaction_hover, context, operation, completion, command_id, maximum_raw_bytes, maximum_work_items, admission_refusal,work,producer_window_config:None,producer_instance_operation_owner:None,producer_output_chunks:None,producer_controller_id:None,producer_tool_id:None,producer_payload_schema_id:None }

    }
}
//#endregion 🧳️Payload

//#region 🧵️Job
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ArtifactRetainedCommandPhase {
    CheckpointPages,
    CheckpointRetire,
    WirePages,
    Decode,
    Preflight,
    Work,
    Publish,
    Complete,
    Fault,
}

#[derive(Clone,Copy)]
pub(super) enum ArtifactCommandPublication {
    Static { kind: JobPublicationKind, bytes: &'static [u8] },
    Checkpoint,
}

pub struct ArtifactRetainedCommandJob<A: ArtifactApp> {
    pub(super) producer_window_config:Option<semio_framework_plugin_window_config::WindowConfigSnapshot>,
    pub(super) producer_instance_operation_owner:Option<semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwnerHandle>,
    pub(super) producer_output_chunks:Option<semio_framework_plugin_decl_app::tool_contract::ArtifactOutputChunks>,
    pub(super) producer_controller_id:Option<String>,
    pub(super) producer_tool_id:Option<String>,
    pub(super) producer_payload_schema_id:Option<String>,
    pub(crate) source_binding:Option<semio_framework::action_bus::OriginalToolSourceBinding>,
    pub command: Option<A::Command>,
    pub(super) command_received:bool,
    pub snapshot: Option<Arc<A::Snapshot>>,
    pub snapshot_read: std::cell::RefCell<Option<store::os_store::snapshot_read::SnapshotRead<A::Snapshot>>>,
    pub config: Option<Arc<A::Config>>,
    pub history: Option<Arc<HistoryView>>,
    pub interaction_state: Option<Arc<protocol::InteractionState>>,
    pub interaction_hover: Option<Arc<InteractionHoverState>>,
    pub context: Option<ArtifactOwnedContextHandle<A>>,
    pub operation: Option<AppOperationContext>,
    pub completion: Option<ArtifactToolCompletion<A>>,
    pub command_id: fn(&A::Command) -> &'static str,
    pub(super) maximum_raw_bytes: usize,
    pub(super) maximum_work_items: usize,
    pub(super) work: Option<Box<dyn ArtifactCommandWork<A>>>,
    pub(super) checkpoint_input: Option<RetainedToolWireInput>,
    pub(super) checkpoint_bytes: [u8; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
    pub(super) checkpoint_target: [MaybeUninit<u8>; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
    pub(super) checkpoint_capture: Option<ArtifactCommandCheckpointCursor>,
    pub(super) publication: RetainedJobPublication,
    pub(super) fault_publication: RetainedFaultPublication,
    pub(super) pending_publication: Option<ArtifactCommandPublication>,
    pub(super) publication_delivered: bool,
    pub(super) fault: Option<Fault>,
    pub(super) fault_retirement: Option<ControlledRetirement<Fault>>,
    pub(super) decode_error: Option<protocol::ProtocolError>,
    pub(super) checkpoint_byte_len: usize,
    pub(super) checkpoint_page_cursor: usize,
    pub(super) checkpoint_page_offset: usize,
    pub(super) raw_input: Option<RetainedToolWireInput>,
    pub(super) raw: Vec<u8>,
    pub(super) admission_refusal:Option<ArtifactRetainedAdmissionRefusal>,
    pub(super) raw_page_cursor: usize,
    pub(super) raw_page_offset: usize,
    pub(super) emit: Option<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>,
    pub(super) download:Option<ArtifactDownloadOutput>,
    pub(super) download_retirement:Option<ControlledRetirement<ArtifactDownloadOutput>>,
    pub(super) completion_retirement:Option<ControlledRetirement<ArtifactToolCompletion<A>>>,
    pub(super) input_retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pub(super) ephemeral: Option<EphemeralEmit<A>>,
    pub(super) phase: ArtifactRetainedCommandPhase,
    pub(super) checkpoint_pending: bool,
    pub(super) work_progress: u64,
    pub(crate) closing: bool,
}

impl<A: ArtifactApp> ArtifactRetainedCommandJob<A> {
    fn controlled_close_step(step:Result<RetainedCloneStep,ValueError>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Pending{progress:step?.progress()})}
    fn close_one_original(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if !self.publication.terminal_is_empty(){return Self::controlled_close_step(self.publication.close_step(grant))}
        if !self.fault_publication.terminal_is_empty(){return Self::controlled_close_step(self.fault_publication.close_step(grant))}
        if let Some(cursor)=self.checkpoint_capture.as_mut().filter(|cursor|!cursor.terminal_is_empty()){return Self::controlled_close_step(cursor.close_step(grant))}
        if self.checkpoint_capture.is_some()||self.pending_publication.is_some()||self.publication_delivered{self.checkpoint_capture.take();self.pending_publication=None;self.publication_delivered=false;self.checkpoint_pending=false;return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if let Some(fault)=self.fault.take(){match ControlledRetirement::new(fault){Ok(owner)=>self.fault_retirement=Some(owner),Err((error,original))=>{self.fault=Some(original);return Err(error.into())}}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if let Some(owner)=self.fault_retirement.as_mut(){if owner.terminal_is_empty(){self.fault_retirement.take();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}return Self::controlled_close_step(owner.step(grant))}
        if self.decode_error.is_some(){return Self::controlled_close_step(protocol::close_protocol_error_one(&mut self.decode_error,grant))}
        if self.admission_refusal.take().is_some(){return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if self.raw.capacity()!=0{let bytes=self.raw.capacity();drop(std::mem::take(&mut self.raw));return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})}
        for input in[&mut self.checkpoint_input,&mut self.raw_input]{if let Some(owner)=input.as_mut(){if owner.terminal_is_empty(){input.take();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}return Ok(match owner.close_step(grant)?{InteractiveJobCloseStep::Complete{progress}=>InteractiveJobCloseStep::Pending{progress},step=>step})}}
        if let Some(download)=self.download.take(){match ControlledRetirement::new(download){Ok(owner)=>self.download_retirement=Some(owner),Err((error,original))=>{self.download=Some(original);return Err(error.into())}}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if let Some(owner)=self.download_retirement.as_mut(){if owner.terminal_is_empty(){self.download_retirement.take();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}return Self::controlled_close_step(owner.step(grant))}
        if let Some(emit)=self.emit.as_mut(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};return match emit.close_child_one(child)?{Some(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress)|semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress))=>Ok(InteractiveJobCloseStep::Pending{progress}),Some(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{..}|semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput{..})=>Ok(InteractiveJobCloseStep::Blocked),None=>Err(Self::unadmitted_input().into())}}
        if self.ephemeral.is_some(){return Err(Self::unadmitted_input().into())}
        if let Some(work)=self.work.as_mut(){if !work.terminal_is_empty(){return work.close_step(cx)}let bytes=work.terminal_frame_release_bytes().ok_or_else(Self::unadmitted_input)?;self.work.take();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})}
        if self.input_retirement.is_some(){return Self::controlled_close_step(store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.input_retirement,grant))}
        let admitted=if self.command.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.command,&mut self.input_retirement,grant))}
            else if self.snapshot_read.get_mut().is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(self.snapshot_read.get_mut(),&mut self.input_retirement,grant))}
            else if self.snapshot.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.snapshot,&mut self.input_retirement,grant))}
            else if self.config.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.config,&mut self.input_retirement,grant))}
            else if self.history.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.history,&mut self.input_retirement,grant))}
            else if self.interaction_state.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.interaction_state,&mut self.input_retirement,grant))}
            else if self.interaction_hover.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_shared_lease(&mut self.interaction_hover,&mut self.input_retirement,grant))}
            else if self.operation.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.operation,&mut self.input_retirement,grant))}
            else if self.context.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.context,&mut self.input_retirement,grant))}else if self.producer_controller_id.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_controller_id,&mut self.input_retirement,grant))}else if self.producer_tool_id.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_tool_id,&mut self.input_retirement,grant))}else if self.producer_payload_schema_id.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_payload_schema_id,&mut self.input_retirement,grant))}else if self.producer_window_config.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_window_config,&mut self.input_retirement,grant))}else if self.producer_instance_operation_owner.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_instance_operation_owner,&mut self.input_retirement,grant))}else if self.producer_output_chunks.is_some(){Some(store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.producer_output_chunks,&mut self.input_retirement,grant))}else{None};
        if let Some(step)=admitted{return Self::controlled_close_step(step)}
        if let Some(completion)=self.completion.take(){match ControlledRetirement::new(completion){Ok(owner)=>self.completion_retirement=Some(owner),Err((error,original))=>{self.completion=Some(original);return Err(error.into())}}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        if let Some(owner)=self.completion_retirement.as_mut(){if owner.terminal_is_empty(){self.completion_retirement.take();return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}return Self::controlled_close_step(owner.step(grant))}
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }
    pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.publication.terminal_is_empty(){return self.publication.retirement_demands();}
        if !self.fault_publication.terminal_is_empty(){return self.fault_publication.retirement_demands();}
        if let Some(cursor)=self.checkpoint_capture.as_ref().filter(|cursor|!cursor.terminal_is_empty()){return Ok(cursor.retirement_demands());}
        if self.checkpoint_capture.is_some()||self.pending_publication.is_some()||self.publication_delivered{return Ok(RetirementDemand{depth:1,..Default::default()});}
        if self.fault.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.fault_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{Ok(owner.selected_demand(copy)?)};}
        if self.decode_error.is_some(){return protocol::protocol_error_retirement_demand(&self.decode_error);}
        if self.admission_refusal.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if self.raw.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.raw.capacity(),depth:1,..Default::default()});}
        for input in [&self.checkpoint_input,&self.raw_input]{if let Some(owner)=input.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:owner.next_close_copy_byte_demand()?,capacity_bytes:owner.next_close_capacity_byte_demand(copy)?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?})};}}
        if self.download.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.download_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{Ok(owner.selected_demand(copy)?)};}
        if let Some(emit)=self.emit.as_ref(){
            if let Some(mut demand)=emit.child_close_demands(copy)?{
                demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"retained emitted child depth overflow"))?;
                return Ok(demand);
            }
            return Err(Self::unadmitted_input());
        }
        if self.ephemeral.is_some(){return Err(Self::unadmitted_input());}
        if let Some(work)=self.work.as_ref(){return if work.terminal_is_empty(){Ok(RetirementDemand{release_bytes:work.terminal_frame_release_bytes().ok_or_else(Self::unadmitted_input)?,depth:1,..Default::default()})}else{Ok(RetirementDemand{copy_bytes:work.next_close_copy_byte_demand()?,capacity_bytes:work.next_close_capacity_byte_demand(copy)?,release_bytes:work.next_close_release_byte_demand()?,depth:work.next_close_depth_demand()?})};}
        if let Some(owner)=self.input_retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,copy);}
        if self.command.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.command);}
        if let Ok(original)=self.snapshot_read.try_borrow(){if original.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*original);}}else{return Ok(RetirementDemand{depth:1,..Default::default()});}
        if self.snapshot.is_some(){return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.snapshot);}
        if self.config.is_some(){return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.config);}
        if self.history.is_some(){return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.history);}
        if self.interaction_state.is_some(){return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.interaction_state);}
        if self.interaction_hover.is_some(){return store::os_store::snapshot_read::artifact_retirement_shared_lease_birth_demands(&self.interaction_hover);}
        if self.operation.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.operation);}
        if self.context.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.context);}
        if self.producer_window_config.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_window_config);}
        if self.producer_instance_operation_owner.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_instance_operation_owner);}
        if self.producer_output_chunks.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_output_chunks);}
        if self.producer_controller_id.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_controller_id);}
        if self.producer_tool_id.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_tool_id);}
        if self.producer_payload_schema_id.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.producer_payload_schema_id);}
        if self.completion.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
        if let Some(owner)=self.completion_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{Ok(owner.selected_demand(copy)?)};}
        Ok(Default::default())
    }
    fn unadmitted_input()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained command input requires its original app-owned retirement authority")}

    pub fn new(payload: ArtifactRetainedCommandPayload<A>) -> Self {
        Self::from_payload(payload, None, None)
    }

    pub fn from_wire(payload: ArtifactRetainedCommandPayload<A>, input: RetainedToolWireInput) -> Self {
        Self::from_payload(payload, Some(input), None)
    }

    pub fn from_wire_with_checkpoint(payload: ArtifactRetainedCommandPayload<A>, input: RetainedToolWireInput, checkpoint: RetainedToolWireInput) -> Self {
        Self::from_payload(payload, Some(input), Some(checkpoint))
    }

    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn test_admit_original_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.source_binding.is_some(){return Ok(self.source_binding.unwrap().matches(cx))}let mut output=None;let admitted=semio_framework::action_bus::admit_original_tool_construction(cx,&mut output,semio_framework::action_bus::ToolJobAdmissionDemand::reused_job(),|identity|Ok((identity.source_binding(),RetainedCloneProgress{copied_items:1,..Default::default()})))?;if admitted{self.source_binding=output;}Ok(admitted)
    }
    pub fn from_payload(payload: ArtifactRetainedCommandPayload<A>, raw_input: Option<RetainedToolWireInput>, checkpoint_input: Option<RetainedToolWireInput>) -> Self {

        let (raw,admission_refusal)=match (payload.admission_refusal,raw_input.as_ref()) {
            (None,Some(input))=>admission::admit_raw(input.declared_bytes(),payload.maximum_raw_bytes),
            (refusal,_)=>(Vec::new(),refusal),
        };
        let phase = if admission_refusal.is_some(){ArtifactRetainedCommandPhase::Fault}else if checkpoint_input.is_some() {
            ArtifactRetainedCommandPhase::CheckpointPages
        } else if raw_input.is_some() {
            ArtifactRetainedCommandPhase::WirePages
        } else {
            ArtifactRetainedCommandPhase::Preflight
        };
        Self {
            source_binding:None,
            command: Some(payload.command),
            command_received:false,
            snapshot: Some(payload.snapshot),
            snapshot_read: std::cell::RefCell::new(Some(payload.snapshot_read)),
            config: Some(payload.config),
            history: Some(payload.history),
            interaction_state: Some(payload.interaction_state),
            interaction_hover: Some(payload.interaction_hover),
            context: payload.context,
            operation: Some(payload.operation),
            completion: Some(payload.completion),
            command_id: payload.command_id,
            maximum_raw_bytes: payload.maximum_raw_bytes,
            maximum_work_items: payload.maximum_work_items,
            work: Some(payload.work),
            producer_controller_id:payload.producer_controller_id,
            producer_tool_id:payload.producer_tool_id,
            producer_payload_schema_id:payload.producer_payload_schema_id,
            producer_window_config:payload.producer_window_config,
            producer_instance_operation_owner:payload.producer_instance_operation_owner,
            producer_output_chunks:payload.producer_output_chunks,
            checkpoint_input,
            checkpoint_bytes: [0; ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_target: [MaybeUninit::uninit(); ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES],
            checkpoint_capture: None,
            publication: RetainedJobPublication::new(),
            fault_publication: RetainedFaultPublication::new(),
            pending_publication: None,
            publication_delivered: false,
            fault: None,
            fault_retirement: None,
            decode_error: None,
            checkpoint_byte_len: 0,
            checkpoint_page_cursor: 0,
            checkpoint_page_offset: 0,
            raw_input,
            raw,
            admission_refusal,
            raw_page_cursor: 0,
            raw_page_offset: 0,
            emit: None,
            download:None,
            download_retirement:None,
            completion_retirement:None,
            input_retirement:None,
            ephemeral: None,
            phase,
            checkpoint_pending: false,
            work_progress: 0,
            closing: false,
        }
    }

    pub fn metadata(cx: &mut StepContext<'_>) -> Result<bool, ValueError> {
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(false); }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(true)
    }

    pub fn receive_step(cx: &mut StepContext<'_>, step: Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
        match step {
            Ok(step) => { cx.consume_retained(step.progress())?; Ok(step) }
            Err(error) => { cx.consume_retained(error.retained_progress())?; Err(error) }
        }
    }

    fn retire_publication(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        if !self.publication.terminal_is_empty() {
            let grant=cx.retained_grant();
            let step=self.publication.close_step(grant);
            Self::receive_step(cx,step)?;
            return Ok(());
        }
        if !self.fault_publication.terminal_is_empty() {
            let grant=cx.retained_grant();
            let step=self.fault_publication.close_step(grant);
            Self::receive_step(cx,step)?;
            return Ok(());
        }
        if let Some(cursor) = self.checkpoint_capture.as_mut().filter(|cursor| !cursor.terminal_is_empty()) {
            let grant=cx.retained_grant();
            let step=cursor.close_step(grant);
            Self::receive_step(cx,step)?;
            return Ok(());
        }
        if Self::metadata(cx)? {
            if matches!(self.pending_publication, Some(ArtifactCommandPublication::Checkpoint)) { self.checkpoint_pending = false; }
            self.checkpoint_capture.take();
            self.pending_publication = None;
            self.publication_delivered = false;
        }
        Ok(())
    }

    pub fn checkpoint<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if self.checkpoint_capture.is_none() {
            if !Self::metadata(cx)? { return Ok(None); }
            let work = self.work.as_ref().ok_or_else(Self::unadmitted_input)?;
            self.checkpoint_capture = Some(ArtifactCommandCheckpointCursor::new(self.phase == ArtifactRetainedCommandPhase::Work, [
                self.raw_page_cursor as u64, self.raw.len() as u64, self.work_progress,
                self.context.as_ref().map_or(0, |context| context.identity_digest()), work.workspace_identity(),
            ]));
            self.pending_publication = Some(ArtifactCommandPublication::Checkpoint);
            return Ok(None);
        }
        let work = self.work.as_deref().ok_or_else(Self::unadmitted_input)?;
        let cursor = self.checkpoint_capture.as_mut().ok_or_else(Self::unadmitted_input)?;
        if !cursor.is_complete() {
            let step = cursor.advance_one(work, &mut self.checkpoint_target, |work, index| work.checkpoint_byte(index), cx.retained_grant());
            Self::receive_step(cx, step)?;
            return Ok(None);
        }
        let source = cursor.bytes(&self.checkpoint_target)?;
        let result = self.publication.advance_from_source(JobPublicationKind::Checkpoint { applied_progress: self.work_progress.max(self.raw_page_cursor as u64) }, source, cx)?;
        if result.is_some() { self.publication_delivered = true; }
        Ok(result)
    }

    fn advance_publication<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        if let Some(fault) = self.fault.as_ref() {
            let result = self.fault_publication.advance_from_fault(fault, cx)?;
            if result.is_some() { self.publication_delivered = true; }
            return Ok(result);
        }
        match self.pending_publication {
            Some(ArtifactCommandPublication::Static { kind, bytes }) => {
                let result = self.publication.advance_from_source(kind, bytes, cx)?;
                if result.is_some() { self.publication_delivered = true; }
                Ok(result)
            }
            Some(ArtifactCommandPublication::Checkpoint) => self.checkpoint(cx),
            None => Ok(None),
        }
    }

    fn restore_checkpoint(&mut self) -> Result<(), Fault> {
        let bytes = &self.checkpoint_bytes[..self.checkpoint_byte_len];
        let input = self.raw_input.as_ref().ok_or_else(|| Fault::from("retained-command-checkpoint-wire-owner-absent"))?;
        let current_context_digest = self.context.as_ref().map_or(0, |context| context.identity_digest());
        let current_workspace_identity = self.work.as_ref().map_or(0, |work| work.workspace_identity());
        let checkpoint = decode_artifact_command_checkpoint(bytes, self.maximum_raw_bytes, input.page_count(), input.declared_bytes(), current_context_digest, current_workspace_identity)?;
        self.work_progress = checkpoint.work_progress;
        if checkpoint.work_phase {
            self.work.as_mut().ok_or_else(|| Fault::from("retained-command-checkpoint-work-absent"))?.restore(checkpoint.work)?;
        }
        self.raw.clear();
        self.raw_page_cursor = 0;
        self.raw_page_offset = 0;
        Ok(())
    }

    pub(super) fn reports_progress(&self) -> bool {
        self.work.as_ref().is_none_or(|work| work.reports_progress())
    }

    pub fn checkpoint_due(&mut self) {
        self.checkpoint_pending = self.reports_progress();
    }

    pub fn preview(&mut self, bytes: &'static [u8]) {
        if self.reports_progress() {
            self.pending_publication = Some(ArtifactCommandPublication::Static { kind: JobPublicationKind::Preview, bytes });
        }
    }

    pub fn fault(&mut self, bytes: &'static [u8]) {
        self.phase = ArtifactRetainedCommandPhase::Fault;
        self.pending_publication = Some(ArtifactCommandPublication::Static { kind: JobPublicationKind::Fault, bytes });
    }

    fn reducer_fault(&mut self, fault: Fault) {
        self.phase = ArtifactRetainedCommandPhase::Fault;
        self.fault = Some(fault);
    }

    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn test_pending_emit_shape(&self) -> Option<(usize, usize, Vec<String>)> {
        self.emit.as_ref().map(|emit| (emit.child_emits.len(), emit.artifact_mutations.len(), emit.child_emits.iter().map(|child| child.child_id.clone()).collect()))
    }
}

/// 🧯️ The bounded canonical Fault wire one refused reducer step reports. A normal fault crosses exactly;
/// an oversized one keeps its typed code, source span, and named params while prose is narrowed.
pub fn reducer_fault_detail(fault: &Fault) -> Vec<u8> {
    semio_framework_diagnostic::encode_fault_bytes_bounded(fault, ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES).unwrap_or_else(|| {
        semio_framework_diagnostic::encode_fault_bytes(&Fault::new(semio_framework::FaultOrigin::Framework, "retained-command.fault-capacity", "the reducer fault exceeded its bounded diagnostic carrier"))
    })
}

impl<A:ArtifactApp> ArtifactRetainedCommandJob<A>{
    pub fn advance_original<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() { return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into); }
        if cx.should_yield() || cx.retained_grant().maximum_items == 0 || cx.retained_grant().maximum_depth == 0 { return Ok(None); }
        if self.publication_delivered { self.retire_publication(cx)?; return Ok(None); }
        if self.pending_publication.is_some() || self.fault.is_some() { return self.advance_publication(cx).map_err(Into::into); }
        if self.checkpoint_pending { return self.checkpoint(cx).map_err(Into::into); }
        match self.phase {
            ArtifactRetainedCommandPhase::CheckpointPages => {
                cx.set_stage("retained-command-checkpoint-page");
                let input = self.checkpoint_input.as_ref().ok_or_else(Self::unadmitted_input)?;
                if let Some(page) = input.page(self.checkpoint_page_cursor) {
                    let source = page.get(self.checkpoint_page_offset..).ok_or_else(Self::unadmitted_input)?;
                    if self.checkpoint_byte_len.checked_add(source.len()).is_none_or(|length| length > self.checkpoint_bytes.len()) {
                        if Self::metadata(cx)? { self.fault(b"retained command checkpoint exceeds capacity"); }
                        return Ok(None);
                    }
                    let bytes = source.len().min(cx.retained_grant().maximum_copy_bytes);
                    if bytes == 0 && !source.is_empty() { return Ok(None); }
                    let end = self.checkpoint_byte_len + bytes;
                    self.checkpoint_bytes[self.checkpoint_byte_len..end].copy_from_slice(&source[..bytes]);
                    self.checkpoint_byte_len = end;
                    self.checkpoint_page_offset += bytes;
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                    cx.consume_fuel(1);
                    if self.checkpoint_page_offset == page.len() {
                        self.checkpoint_page_cursor += 1;
                        self.checkpoint_page_offset = 0;
                        self.preview(b"{\"en\":\"Reading checkpoint\",\"de\":\"Pr\xC3\xBCfpunkt wird gelesen\"}");
                    }
                    return Ok(None);
                }
                if !Self::metadata(cx)? { return Ok(None); }
                match self.restore_checkpoint() {
                    Ok(()) => {
                        self.checkpoint_input.as_mut().ok_or_else(Self::unadmitted_input)?.begin_close();
                        self.phase = ArtifactRetainedCommandPhase::CheckpointRetire;
                        self.preview(br#"{"en":"Restoring command","de":"Befehl wird wiederhergestellt"}"#);
                    }
                    Err(fault) => self.reducer_fault(fault),
                }
            }
            ArtifactRetainedCommandPhase::CheckpointRetire => {
                let owner = self.checkpoint_input.as_mut().ok_or_else(Self::unadmitted_input)?;
                if !owner.terminal_is_empty() {
                    let demand=RetirementDemand{copy_bytes:owner.next_close_copy_byte_demand()?,capacity_bytes:owner.next_close_capacity_byte_demand(cx.retained_grant().maximum_copy_bytes)?,release_bytes:owner.next_close_release_byte_demand()?,depth:owner.next_close_depth_demand()?};
                    semio_framework_job::close_original_retained_step(cx,demand,|grant|{let step=owner.close_step(grant)?.admit(grant,owner.terminal_is_empty())?;Ok(RetainedCloneStep::Progress(step.progress()))})?;
                    return Ok(None);
                }
                if Self::metadata(cx)? {
                    self.checkpoint_input.take();
                    self.phase = ArtifactRetainedCommandPhase::WirePages;
                    self.checkpoint_due();
                }
            }
            ArtifactRetainedCommandPhase::WirePages => {
                cx.set_stage("retained-command-wire-page");
                let input = self.raw_input.as_ref().ok_or_else(Self::unadmitted_input)?;
                if let Some(page) = input.page(self.raw_page_cursor) {
                    let source = page.get(self.raw_page_offset..).ok_or_else(Self::unadmitted_input)?;
                    if self.raw.len().checked_add(source.len()).is_none_or(|length| length > self.maximum_raw_bytes || length > self.raw.capacity()) {
                        if Self::metadata(cx)? { self.fault(b"retained command exceeds original raw capacity"); }
                        return Ok(None);
                    }
                    let bytes = source.len().min(cx.retained_grant().maximum_copy_bytes);
                    if bytes == 0 && !source.is_empty() { return Ok(None); }
                    self.raw.extend_from_slice(&source[..bytes]);
                    self.raw_page_offset += bytes;
                    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
                    cx.consume_fuel(1);
                    if self.raw_page_offset == page.len() {
                        self.raw_page_cursor += 1;
                        self.raw_page_offset = 0;
                        self.checkpoint_due();
                        self.preview(br#"{"en":"Reading command page","de":"Befehlsseite wird gelesen"}"#);
                    }
                    return Ok(None);
                }
                if Self::metadata(cx)? {
                    self.phase = ArtifactRetainedCommandPhase::Decode;
                    self.checkpoint_due();
                }
            }
            ArtifactRetainedCommandPhase::Decode => {
                cx.set_stage("retained-command-decode");
                if !Self::metadata(cx)? { return Ok(None); }
                let decoded = match <A::Command as protocol::OpBinary>::decode_op(&self.raw) {
                    Ok(command) => command,
                    Err(error) => { self.decode_error=Some(error); self.fault(b"retained command wire payload is malformed"); return Ok(None); }
                };
                let work = self.work.as_ref().ok_or_else(Self::unadmitted_input)?;
                if (self.command_id)(&decoded) != work.tool_id() {
                    return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "received command does not match original app work authority").into());
                }
                self.command = Some(decoded);
                self.phase = ArtifactRetainedCommandPhase::Preflight;
                self.preview(b"{\"en\":\"Validating command\",\"de\":\"Befehl wird gepr\xC3\xBCft\"}");
            }
            ArtifactRetainedCommandPhase::Preflight => {
                cx.set_stage("retained-command-preflight");
                if !Self::metadata(cx)? { return Ok(None); }
                let (Some(command), Some(snapshot), Some(interaction), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.interaction_state.as_ref(), self.work.as_ref()) else { return Err(Self::unadmitted_input().into()); };
                let Some(extent) = work.extent(command, snapshot, interaction, self.context.as_deref()) else {
                    self.fault(b"retained command work refused the command before any capacity was measured");
                    return Ok(None);
                };
                if extent == 0 || extent > self.maximum_work_items {
                    self.fault(b"retained command exceeds semantic work capacity");
                    return Ok(None);
                }
                self.phase = ArtifactRetainedCommandPhase::Work;
                self.preview(br#"{"en":"Applying command","de":"Befehl wird angewendet"}"#);
            }
            ArtifactRetainedCommandPhase::Work => {
                cx.set_stage("retained-command-work");
                if !self.command_received{
                    let (Some(command),Some(operation),Some(work))=(self.command.as_mut(),self.operation.as_ref(),self.work.as_mut())else{return Err(Self::unadmitted_input().into())};
                    if !self.source_binding.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command receiving requires its original physical source binding").into())}
                    let before=cx.retained_progress();let fuel=cx.fuel_remaining();
                    if !work.receive_original_command(command,cx)?{return Ok(None)}
                    if cx.retained_progress()==before&&!Self::metadata(cx)?{return Ok(None)}
                    if cx.fuel_remaining()==fuel{cx.consume_fuel(1)}
                    self.command_received=true;return Ok(None)
                }
                let (Some(command), Some(snapshot), Some(config), Some(history), Some(interaction), Some(hover), Some(operation), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.config.as_ref(), self.history.as_ref(), self.interaction_state.as_ref(), self.interaction_hover.as_ref(), self.operation.as_ref(), self.work.as_mut()) else { return Err(Self::unadmitted_input().into()); };
                let before = cx.retained_progress();
                let fuel = cx.fuel_remaining();
                let input = ArtifactCommandInputs { command, snapshot: snapshot.as_ref(), snapshot_owner: Some(snapshot), snapshot_read:ArtifactCommandSnapshotRead{original:Some(&self.snapshot_read),operation,source:self.source_binding}, config: config.as_ref(), history, interaction, hover, context: self.context.as_deref(), operation };
                let demand = work.work_demands(&input, cx.retained_grant().maximum_copy_bytes)?;
                if !cx.retained_work_can_enter(demand) {
                    if !semio_framework_job::retained_work::RetainedWorkBudget::new(cx.original_total_retained_grant()).can_enter(demand) { self.fault(b"retained command normal-work demand exceeds its original funding"); return Ok(None); }
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                let step = work.step(&input, cx)?;
                if cx.retained_progress() == before { Self::metadata(cx)?; }
                if cx.fuel_remaining() == fuel { cx.consume_fuel(1); }
                match step {
                    ArtifactCommandWorkStep::Pending { stage, preview } => { cx.set_stage(stage); self.checkpoint_due(); self.preview(preview); }
                    ArtifactCommandWorkStep::Replay { stage, preview } => { cx.set_stage(stage); self.checkpoint_due(); self.preview(preview); }
                    ArtifactCommandWorkStep::Progress { stage, preview } => { cx.set_stage(stage); self.work_progress = self.work_progress.saturating_add(1); self.checkpoint_due(); self.preview(preview); }
                    ArtifactCommandWorkStep::CompleteDownload { download, ephemeral } => {
                        self.download = Some(download); self.ephemeral = Some(ephemeral); self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(r#"{"en":"Publishing download","de":"Download wird veröffentlicht"}"#.as_bytes());
                    }
                    ArtifactCommandWorkStep::Complete(emit) => {
                        self.emit = Some(emit); self.ephemeral = Some(EphemeralEmit::default()); self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(b"{\"en\":\"Publishing result\",\"de\":\"Ergebnis wird ver\xC3\xB6ffentlicht\"}");
                    }
                    ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral } => {
                        self.emit = Some(emit); self.ephemeral = Some(ephemeral); self.phase = ArtifactRetainedCommandPhase::Publish;
                        self.preview(b"{\"en\":\"Publishing result\",\"de\":\"Ergebnis wird ver\xC3\xB6ffentlicht\"}");
                    }
                }
            }
            ArtifactRetainedCommandPhase::Publish => {
                cx.set_stage("retained-command-publish");
                if let Some(emit) = self.emit.as_mut() {
                    let step = emit.prepare_child_one(cx.retained_grant());
                    let progress = match &step { Ok(step) => step.progress(), Err(fault) => fault.retained_progress() };
                    cx.consume_retained(progress)?;
                    match step {
                        Ok(semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep::Ready(progress)) if progress == RetainedCloneProgress::default() => {}
                        Ok(semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep::Ready(_) | semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep::Pending(_)) => { cx.consume_fuel(1); return Ok(None); }
                        Ok(semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep::Refused(fault, _)) | Err(fault) => { self.reducer_fault(fault); return Ok(None); }
                    }
                }
                if !Self::metadata(cx)? { return Ok(None); }
                let completion = self.completion.as_ref().ok_or_else(Self::unadmitted_input)?;
                if !completion.has_mounted_consumer() { self.fault(b"retained command completion consumer is absent"); return Ok(None); }
                if let Some(download) = self.download.take() {
                    let Some(ephemeral) = self.ephemeral.take() else { self.download = Some(download); return Err(Self::unadmitted_input().into()); };
                    if let Err(rejected) = completion.complete_download(Ok(download), ephemeral) {
                        self.download = rejected.download.ok(); self.ephemeral = Some(rejected.ephemeral); self.reducer_fault(rejected.fault); return Ok(None);
                    }
                } else {
                    let emit = self.emit.take().ok_or_else(Self::unadmitted_input)?;
                    let Some(ephemeral) = self.ephemeral.take() else { self.emit = Some(emit); return Err(Self::unadmitted_input().into()); };
                    if let Err(rejected) = completion.complete(Ok(emit), ephemeral) {
                        self.emit = rejected.emit.ok(); self.ephemeral = Some(rejected.ephemeral); self.reducer_fault(rejected.fault); return Ok(None);
                    }
                }
                self.phase = ArtifactRetainedCommandPhase::Complete;
            }
            ArtifactRetainedCommandPhase::Complete => return JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into),
            ArtifactRetainedCommandPhase::Fault => {
                if Self::metadata(cx)? { self.fault(self.admission_refusal.map(ArtifactRetainedAdmissionRefusal::detail).unwrap_or(b"retained command remains faulted")); }
            }
        }
        Ok(None)
    }

}

impl<A: ArtifactApp> InteractiveJob for ArtifactRetainedCommandJob<A> {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{if !self.source_binding.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained command source binding changed before work").into())}self.advance_original(cx)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.phase == ArtifactRetainedCommandPhase::Complete => descriptor.complete(None, None),
            JobOutcomeKind::Fault if self.fault.is_some() => self.fault_publication.borrow_outcome(descriptor),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(Self::unadmitted_input()),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(checkpoint) = self.checkpoint_input.as_mut() {
            checkpoint.begin_close();
        }
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
        if let Some(work) = self.work.as_mut() {
            work.begin_close();
        }
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.close_demands(copy)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if !self.source_binding.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original retained command source binding changed before retirement").into())}
        if !self.closing{return Ok(InteractiveJobCloseStep::Blocked)}
        let grant=cx.retained_grant();let demand=self.close_demands(grant.maximum_copy_bytes)?;
        let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,loan|{
            let step=self.close_one_original(same,loan)?;
            let step=match step{InteractiveJobCloseStep::Pending{progress}=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress),InteractiveJobCloseStep::Blocked=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{reason:"original retained command awaits child custody"},InteractiveJobCloseStep::Complete{progress}=>semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress)};
            Ok((step,self.terminal_is_empty()))
        })?;
        Ok(match step{semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress)=>InteractiveJobCloseStep::Pending{progress},semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(progress)=>InteractiveJobCloseStep::Complete{progress},semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked{..}|semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput{..}=>InteractiveJobCloseStep::Blocked})
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.publication.terminal_is_empty()
            && self.fault_publication.terminal_is_empty()
            && self.checkpoint_capture.is_none()
            && self.pending_publication.is_none()
            && !self.publication_delivered
            && self.fault.is_none()
            && self.fault_retirement.is_none()
            && self.decode_error.is_none()
            && self.admission_refusal.is_none()
            && self.raw.is_empty()
            && self.raw.capacity() == 0
            && self.checkpoint_input.is_none()
            && self.raw_input.is_none()
            && self.emit.is_none()
            && self.download.is_none()
            && self.download_retirement.is_none()
            && self.ephemeral.is_none()
            && self.work.is_none()
            && self.command.is_none()
            && self.snapshot_read.try_borrow().is_ok_and(|original|original.is_none())
            && self.snapshot.is_none()
            && self.config.is_none()
            && self.history.is_none()
            && self.interaction_state.is_none()
            && self.interaction_hover.is_none()
            && self.context.is_none()
            && self.input_retirement.is_none()
            && self.operation.is_none()
            && self.completion.is_none()
            && self.completion_retirement.is_none()
            && self.producer_controller_id.is_none()
            && self.producer_tool_id.is_none()
            && self.producer_payload_schema_id.is_none()
            && self.producer_window_config.is_none()
            && self.producer_instance_operation_owner.is_none()
            && self.producer_output_chunks.is_none()
    }
}
//#endregion 🧵️Job
