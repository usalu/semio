//! 📥️ The framework-owned bounded natural-file importer every editor declaring a natural file codec inherits.
use crate::tool_contract::ArtifactToolCompletion;
use crate::tool_contract::ArtifactToolCompletionRejection;
use crate::tool_contract::ArtifactReservedToolInput;
use crate::tool_contract::ArtifactReservedToolJobRequest;
use semio_framework_value::DslValue;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use semio_framework::Media;
use semio_framework::MediaError;
use semio_framework::MediaPayload;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use crate::app_contract::NaturalFileDecodeOwner;
use crate::app_contract::NaturalFileDecodeProgress;
use crate::app_contract::NaturalFileDecodeStep;
use crate::app_contract::document_load_emit;
use crate::app_contract::natural_file_input_bytes;
use crate::app_emission::EphemeralEmit;
use crate::bounded_ownership::PluginLifecycleStep;
use crate::tool_contract::ArtifactReservedJob;
use crate::sdk_fault::plugin_sdk_fault;
use crate::app_emission::ArtifactMutationOutcome;
use crate::surface_contract::ArtifactEditor;
use crate::surface_contract::EditorApp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NaturalFileImportStage {
    Account,
    Decode,
    Publish,
}

/// 📂️ Reserved natural-file import owner. It accounts every source byte before the temporary
/// synchronous small-input codec bridge, refuses that bridge when the input cannot fit one scheduler
/// grant, checkpoints before publication, and retains every input/output owner until close. Source-byte
/// admission is not a parser work or expansion bound; scalable formats supply a controlled decoder.
pub(crate) struct NaturalFileImportJob<E: ArtifactEditor> {
    pub(crate) port: String,
    media: Option<Media>,
    decoder: Option<NaturalFileDecodeOwner<E::Snapshot>>,
    completion: Option<ArtifactToolCompletion<EditorApp<E>>>,
    emit: Option<ArtifactMutationOutcome<E::Mutation, E::ConfigMutation, E::DraftMutation>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<E>>>,
    retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pending_bytes: Option<Vec<u8>>,
    pending_media_value: Option<(String, DslValue)>,
    pending_media_text: Option<(String, String)>,
    pending_port: Option<String>,
    pub(crate) contract: semio_framework::ToolExecutionContract,
    byte_cursor: usize,
    byte_length: usize,
    decode_progress: NaturalFileDecodeProgress,
    stage: NaturalFileImportStage,
    completed: bool,
    state: [u8; 17],
    fault: crate::tool_contract::BoundedFaultText,
    pending: Option<crate::tool_contract::ReservedOutcomeKind>,
    outcomes: crate::tool_contract::ReservedJobOutcomes,
    pub(crate) closing: bool,
}

impl<E: ArtifactEditor> NaturalFileImportJob<E> {
    pub(crate) fn new(request: ArtifactReservedToolJobRequest<EditorApp<E>>) -> Result<Self, Fault> {
        let ArtifactReservedToolJobRequest { retained: _, contract, input, completion, .. } = request;
        let ArtifactReservedToolInput::Media { port, mut media } = input else { unreachable!("natural-file import builder admits only media") };
        let codec = E::natural_file_codec().ok_or_else(|| plugin_sdk_fault("natural-file import lost its codec declaration"))?;
        let byte_length = natural_file_input_bytes(&port, codec, &media).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import"), error.to_string()))?.len();
        if byte_length > contract.max_raw_wire_bytes {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("media.natural-import-raw-limit"), format!("natural-file input contains {byte_length} bytes; maximum is {}", contract.max_raw_wire_bytes)));
        }
        let bytes = match &mut media.payload {
            MediaPayload::Intrinsic { value: DslValue::Bytes(bytes), .. } => std::mem::take(bytes),
            _ => unreachable!("natural-file input bytes were validated above"),
        };
        let decoder = E::build_natural_file_decode_owner(bytes, contract).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import"), error.to_string()))?;
        let stage = if matches!(decoder, NaturalFileDecodeOwner::Controlled(_)) { NaturalFileImportStage::Decode } else { NaturalFileImportStage::Account };
        Ok(Self {
            port,
            media: Some(media),
            decoder: Some(decoder),
            completion: Some(completion),
            emit: None,
            pending_completion_rejection: None,
            retirement: None,
            pending_bytes: None,
            pending_media_value: None,
            pending_media_text: None,
            pending_port: None,
            contract,
            byte_cursor: 0,
            byte_length,
            decode_progress: NaturalFileDecodeProgress::default(),
            stage,
            completed: false,
            state: [0; 17],
            fault: crate::tool_contract::BoundedFaultText::new(),
            pending: None,
            outcomes: crate::tool_contract::ReservedJobOutcomes::new(),
            closing: false,
        })
    }

    fn fail(&mut self, detail: &str) {
        self.fault.set(detail);
        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Fault);
    }

    fn checkpoint(&mut self) {
        self.state[0] = self.stage as u8;
        self.state[1..9].copy_from_slice(&(self.byte_cursor as u64).to_le_bytes());
        self.state[9..17].copy_from_slice(&(self.byte_length as u64).to_le_bytes());
        let applied_progress = if matches!(self.decoder, Some(NaturalFileDecodeOwner::Controlled(_))) { self.decode_progress.completed_work_units } else { self.byte_cursor as u64 };
        self.pending = Some(crate::tool_contract::ReservedOutcomeKind::Checkpoint { applied_progress });
    }

    fn decode_synchronous(&mut self) -> Result<ArtifactMutationOutcome<E::Mutation, E::ConfigMutation, E::DraftMutation>, Fault> {
        let outcome = (|| {
            let Some(NaturalFileDecodeOwner::Synchronous(bytes)) = self.decoder.as_ref() else {
                return Err(MediaError::Payload(self.port.clone(), "natural-file synchronous decoder lost its byte owner".into()));
            };
            let snapshot = E::decode_natural_file(bytes)?;
            Ok(document_load_emit(&snapshot, E::DOCUMENT_SCHEMA))
        })();
        Ok(outcome.map_err(|error: MediaError| Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import"), error.to_string())))
    }

    fn admit_controlled_progress(&mut self, progress: NaturalFileDecodeProgress, complete: bool, context: &semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let previous = self.decode_progress;
        if progress.consumed_input_bytes < previous.consumed_input_bytes
            || progress.decoded_items < previous.decoded_items
            || progress.owned_output_bytes < previous.owned_output_bytes
            || progress.completed_work_units < previous.completed_work_units
        {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import-progress"), "natural-file decoder progress moved backwards"));
        }
        if progress.consumed_input_bytes > self.byte_length || progress.consumed_input_bytes > self.contract.max_raw_wire_bytes {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import-input-limit"), "natural-file decoder exceeded its admitted input owner"));
        }
        if progress.decoded_items > self.contract.max_decoded_items {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import-decoded-limit"), format!("natural-file decoder produced {} items; maximum is {}", progress.decoded_items, self.contract.max_decoded_items)));
        }
        if progress.owned_output_bytes > self.contract.max_output_bytes {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import-owned-limit"), format!("natural-file decoder retained {} output bytes; maximum is {}", progress.owned_output_bytes, self.contract.max_output_bytes)));
        }
        if complete && progress.consumed_input_bytes != self.byte_length {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import-incomplete"), "natural-file decoder completed before consuming its admitted input"));
        }
        if !complete && progress == previous && !context.should_yield() && !context.is_cancelled() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import-stalled"), "natural-file decoder returned successful zero progress with an available grant"));
        }
        self.decode_progress = progress;
        self.byte_cursor = progress.consumed_input_bytes;
        Ok(())
    }
}

impl<E: ArtifactEditor> semio_framework_job::InteractiveJob for NaturalFileImportJob<E> {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use crate::tool_contract::ReservedOutcomeKind;
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.outcomes.is_delivered() {
            self.outcomes.retire(cx)?;
            return Ok(None);
        }
        if let Some(kind) = self.pending {
            let source: &[u8] = if kind == ReservedOutcomeKind::Fault { self.fault.as_bytes() } else { &self.state };
            let outcome = self.outcomes.advance(kind, source, cx)?;
            if outcome.is_some() {
                self.pending = None;
            }
            return Ok(outcome);
        }
        if cx.should_yield() {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if self.pending_completion_rejection.is_some() {
            self.fail("natural-file import completion remains rejected");
            return Ok(None);
        }
        match self.stage {
            NaturalFileImportStage::Account => {
                cx.set_stage("natural-file-import-account");
                let units = self.byte_length.saturating_sub(self.byte_cursor).min(self.contract.max_work_units_per_step as usize).min(cx.fuel_remaining() as usize);
                self.byte_cursor = self.byte_cursor.saturating_add(units);
                cx.consume_fuel(units as u64);
                if self.byte_cursor >= self.byte_length {
                    self.stage = NaturalFileImportStage::Decode;
                }
                self.checkpoint();
            }
            NaturalFileImportStage::Decode => {
                cx.set_stage("natural-file-import-decode");
                if matches!(self.decoder, Some(NaturalFileDecodeOwner::Controlled(_))) {
                    let step = {
                        let Some(NaturalFileDecodeOwner::Controlled(cursor)) = self.decoder.as_mut() else { unreachable!("controlled decoder checked above") };
                        cursor.advance(cx)
                    };
                    match step {
                        Ok(NaturalFileDecodeStep::Progress(progress)) => {
                            match self.admit_controlled_progress(progress, false, cx) {
                                Ok(()) => self.checkpoint(),
                                Err(error) => self.fail(&error.message),
                            }
                        }
                        Ok(NaturalFileDecodeStep::Complete { snapshot, progress }) => {
                            match self.admit_controlled_progress(progress, true, cx) {
                                Ok(()) => {
                                    self.emit = Some(Ok(document_load_emit(&snapshot, E::DOCUMENT_SCHEMA)));
                                    self.stage = NaturalFileImportStage::Publish;
                                    self.checkpoint();
                                }
                                Err(error) => self.fail(&error.message),
                            }
                        }
                        Err(error) => self.fail(&error.to_string()),
                    }
                    return Ok(None);
                }
                let units = self.byte_length.max(1);
                if units > self.contract.max_work_units_per_step as usize {
                    self.fail("natural-file codec requires a controlled decoder for this input size");
                    return Ok(None);
                }
                if units > cx.fuel_remaining() as usize {
                    return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
                }
                match self.decode_synchronous() {
                    Ok(emit) => self.emit = Some(emit),
                    Err(error) => {
                        self.fail(&error.message);
                        return Ok(None);
                    }
                }
                cx.consume_fuel(units as u64);
                self.stage = NaturalFileImportStage::Publish;
                self.checkpoint();
            }
            NaturalFileImportStage::Publish => {
                cx.set_stage("natural-file-import-publish");
                if !self.completed {
                    let Some(emit) = self.emit.take() else {
                        self.fail("natural-file import lost its decoded publication");
                        return Ok(None);
                    };
                    let Some(completion) = self.completion.as_ref() else {
                        self.fail("natural-file import lost its completion authority");
                        return Ok(None);
                    };
                    if !completion.has_mounted_consumer() {
                        self.emit = Some(emit);
                        self.fail("natural-file import completion consumer is absent");
                        return Ok(None);
                    }
                    if let Err(rejected) = completion.complete(emit, EphemeralEmit::default()) {
                        let message = rejected.fault.message.clone();
                        self.pending_completion_rejection = Some(rejected);
                        self.fail(&message);
                        return Ok(None);
                    }
                    self.completed = true;
                }
                return Ok(semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)?);
            }
        }
        Ok(None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        self.closing=true;let demand=self.close_owned_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{self.pending=None;if !self.outcomes.terminal_is_empty(){return self.outcomes.close_step(grant)}self.close_owned_turn(grant).map(|step|match step{PluginLifecycleStep::Progress(progress)=>RetainedCloneStep::Progress(progress),PluginLifecycleStep::Complete(progress)=>RetainedCloneStep::Complete(progress),PluginLifecycleStep::Blocked{..}|PluginLifecycleStep::AwaitingInput{..}=>RetainedCloneStep::Progress(Default::default())})})?;
        Ok(match step{Step::Pending{progress}|Step::Complete{progress}=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Step::Blocked=>Step::Blocked})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_owned_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.port.capacity()==0&&self.media.is_none()&&self.decoder.is_none()&&self.completion.is_none()&&self.emit.is_none()&&self.pending_completion_rejection.is_none()&&self.retirement.is_none()&&self.outcomes.terminal_is_empty()}
}

impl<E: ArtifactEditor> ArtifactReservedJob for NaturalFileImportJob<E> {}

impl<E:ArtifactEditor> NaturalFileImportJob<E>{
    fn close_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        use semio_framework_value::retirement::owned_retirement_birth_bytes;
        if !self.outcomes.terminal_is_empty(){return self.outcomes.close_demands();}
        if let Some(ticket)=self.retirement.as_ref(){return semio_framework_value::factory_ticket_demands(ticket,body);}
        if let Some(decoder)=self.decoder.as_ref(){return match decoder{
            NaturalFileDecodeOwner::Controlled(owner) if owner.terminal_is_empty()=>Ok(RetirementDemand{release_bytes:size_of_val(&**owner),depth:1,..Default::default()}),
            NaturalFileDecodeOwner::Controlled(owner)=>owner.retirement_demands(body),
            NaturalFileDecodeOwner::Synchronous(_)=>Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<Vec<u8>>(),depth:1,..Default::default()}),
        };}
        if self.pending_completion_rejection.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<ArtifactToolCompletionRejection<EditorApp<E>>>(),depth:1,..Default::default()});}
        if self.emit.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<ArtifactMutationOutcome<E::Mutation,E::ConfigMutation,E::DraftMutation>>(),depth:1,..Default::default()});}
        if self.media.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<Media>(),depth:1,..Default::default()});}
        if self.port.capacity()!=0{return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<String>(),depth:1,..Default::default()});}
        if self.completion.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<ArtifactToolCompletion<EditorApp<E>>>(),depth:1,..Default::default()});}
        Ok(Default::default())
    }
    fn close_owned_turn(&mut self,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,ValueError>{
        let demand=self.close_owned_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(PluginLifecycleStep::Progress(Default::default()));}
        if self.retirement.is_some(){let step=semio_framework_value::close_factory_ticket(&mut self.retirement,grant)?;return Ok(PluginLifecycleStep::Progress(step.progress()));}
        if let Some(decoder)=self.decoder.as_mut(){match decoder{
            NaturalFileDecodeOwner::Controlled(owner)=>{
                if owner.terminal_is_empty(){let released_bytes=size_of_val(&**owner);self.decoder.take();return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}));}
                owner.begin_close();let step=owner.close_step(grant).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"natural-file decoder close failed while retaining its original owner"))?;
                if step.progress().is_some_and(|progress|!progress.fits(grant))||matches!(step,PluginLifecycleStep::Complete(_))&&!owner.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"natural-file decoder violated its original retirement grant or terminal witness"));}
                return Ok(match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step});
            },
            NaturalFileDecodeOwner::Synchronous(raw)=>{let original=std::mem::take(raw);return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{self.decoder.take();self.retirement=Some(owner);Ok(PluginLifecycleStep::Progress(progress))},Err((error,original))=>{*raw=original;Err(error)}};}
        }}
        if let Some(original)=self.pending_completion_rejection.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{self.retirement=Some(owner);Ok(PluginLifecycleStep::Progress(progress))},Err((error,original))=>{self.pending_completion_rejection=Some(original);Err(error)}};}
        if let Some(original)=self.emit.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{self.retirement=Some(owner);Ok(PluginLifecycleStep::Progress(progress))},Err((error,original))=>{self.emit=Some(original);Err(error)}};}
        if let Some(original)=self.media.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{self.retirement=Some(owner);Ok(PluginLifecycleStep::Progress(progress))},Err((error,original))=>{self.media=Some(original);Err(error)}};}
        if self.port.capacity()!=0{let original=std::mem::take(&mut self.port);return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{self.retirement=Some(owner);Ok(PluginLifecycleStep::Progress(progress))},Err((error,original))=>{self.port=original;Err(error)}};}
        if let Some(original)=self.completion.take(){return match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{self.retirement=Some(owner);Ok(PluginLifecycleStep::Progress(progress))},Err((error,original))=>{self.completion=Some(original);Err(error)}};}
        Ok(PluginLifecycleStep::Complete(Default::default()))
    }
}
