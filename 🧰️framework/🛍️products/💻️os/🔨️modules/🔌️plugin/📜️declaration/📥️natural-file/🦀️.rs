//! 📥️ The framework-owned bounded natural-file importer every editor declaring a natural file codec inherits.
use semio_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_plugin_decl_app::tool_contract::ArtifactToolCompletionRejection;
use semio_plugin_decl_app::tool_contract::ArtifactReservedToolInput;
use semio_plugin_decl_app::tool_contract::ArtifactReservedToolJobRequest;
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
use semio_plugin_decl_app::NaturalFileDecodeOwner;
use semio_plugin_decl_media::original_media::natural_source::NaturalFilePagedSource;
use semio_plugin_decl_app::NaturalFileDecodeProgress;
use semio_plugin_decl_app::NaturalFileDecodeStep;
use semio_plugin_decl_app::document_reference_load_emit;
use semio_plugin_decl_app::natural_file_input_bytes;
use semio_plugin_decl_app::app_emission::EphemeralEmit;
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_plugin_decl_app::tool_contract::ArtifactReservedJob;
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_decl_emission::ArtifactMutationOutcome;
use crate::ArtifactEditor;
use crate::EditorApp;
use semio_framework_value::{ErasedSnapshotRetirement,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value::retirement::{RetireOwned,admit_owned_retirement,record_retirement_work};
use semio_framework_value::close_factory_ticket;
use semio_os_kernel::os_store::serialization::{ArtifactStoreSnapshot, OriginalSnapshotPackExport, OriginalSnapshotPackOutput};
use semio_os_kernel::os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use std::mem::{size_of,size_of_val};
fn invariant(message:&'static str)->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,message)}
fn settled(result:Result<RetainedCloneStep,ValueError>,work:u128)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(Into::into),work})}
fn admit<T:RetireOwned>(slot:&mut Option<T>,ticket:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let turn=admit_owned_retirement(slot,effect);
 settled(turn.result.map(|admitted|match admitted{Some((owner,progress))=>{*ticket=Some(owner);RetainedCloneStep::Progress(progress)},None=>RetainedCloneStep::Progress(Default::default())}),turn.work)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NaturalFileImportStage {
    Account,
    Decode,
    Encode,
    Publish,
    AdoptGenesis,
}

/// 📂️ Reserved natural-file import owner. It accounts every source byte before the temporary
/// synchronous small-input codec bridge, refuses that bridge when the input cannot fit one scheduler
/// grant, checkpoints before publication, and retains every input/output owner until close. Source-byte
/// admission is not a parser work or expansion bound; scalable formats supply a controlled decoder.
pub(crate) struct NaturalFileImportJob<E: ArtifactEditor> {
    pub port: String,
    media: Option<Media>,
    decoder: Option<NaturalFileDecodeOwner<E::Snapshot>>,
    completion: Option<ArtifactToolCompletion<EditorApp<E>>>,
    emit: Option<ArtifactMutationOutcome<E::Mutation, E::ConfigMutation, E::DraftMutation>>,
    pending_completion_rejection: Option<ArtifactToolCompletionRejection<EditorApp<E>>>,
    retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    pending_snapshot: Option<E::Snapshot>,
    pack_export: Option<Box<OriginalSnapshotPackExport>>,
    verified_pack: Option<OriginalSnapshotPackOutput>,
    pending_genesis: Option<AdmittedArtifactGenesis<E::Snapshot>>,
    publication_work: u64,
    pending_bytes: Option<Vec<u8>>,
    pending_paged: Option<NaturalFilePagedSource>,
    pending_media_value: Option<(String, DslValue)>,
    pending_media_text: Option<(String, String)>,
    pending_port: Option<String>,
    pub contract: semio_framework::ToolExecutionContract,
    byte_cursor: usize,
    byte_length: usize,
    decode_progress: NaturalFileDecodeProgress,
    stage: NaturalFileImportStage,
    completed: bool,
    state: [u8; 17],
    fault: semio_plugin_decl_app::tool_contract::BoundedFaultText,
    pending: Option<semio_plugin_decl_app::tool_contract::ReservedOutcomeKind>,
    outcomes: semio_plugin_decl_app::tool_contract::ReservedJobOutcomes,
    pub closing: bool,
}

impl<E: ArtifactEditor> NaturalFileImportJob<E> {
    pub fn new(request: ArtifactReservedToolJobRequest<EditorApp<E>>) -> Result<Self, Fault> {
        let ArtifactReservedToolJobRequest { retained: _, contract, input, completion, .. } = request;
        let Some(ArtifactReservedToolInput::Media { port, mut media }) = input else { unreachable!("natural-file import builder admits only media") };
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
            pending_paged: None,
            pending_media_value: None,
            pending_snapshot: None,
            pack_export: None,
            verified_pack: None,
            pending_genesis: None,
            publication_work: 0,
            pending_media_text: None,
            pending_port: None,
            contract,
            byte_cursor: 0,
            byte_length,
            decode_progress: NaturalFileDecodeProgress::default(),
            stage,
            completed: false,
            state: [0; 17],
            fault: semio_plugin_decl_app::tool_contract::BoundedFaultText::new(),
            pending: None,
            outcomes: semio_plugin_decl_app::tool_contract::ReservedJobOutcomes::new(),
            closing: false,
        })
    }

    fn fail(&mut self, detail: &str) {
        self.fault.set(detail);
        self.pending = Some(semio_plugin_decl_app::tool_contract::ReservedOutcomeKind::Fault);
    }

    fn checkpoint(&mut self) {
        self.state[0] = self.stage as u8;
        self.state[1..9].copy_from_slice(&(self.byte_cursor as u64).to_le_bytes());
        self.state[9..17].copy_from_slice(&(self.byte_length as u64).to_le_bytes());
        let decode_work = if matches!(self.decoder, Some(NaturalFileDecodeOwner::Controlled(_))) {
            self.decode_progress.completed_work_units
        } else {
            self.byte_cursor as u64
        };
        let applied_progress = decode_work.saturating_add(self.publication_work);
        self.pending = Some(semio_plugin_decl_app::tool_contract::ReservedOutcomeKind::Checkpoint { applied_progress });
    }


    fn note_publication_work(
        &mut self, previous_stage: NaturalFileImportStage, fuel_before: u64, cx: &semio_framework_job::StepContext<'_>,
    ) {
        let previous_work = self.publication_work;
        let spent = fuel_before.saturating_sub(cx.fuel_remaining());
        self.publication_work = previous_work.saturating_add(spent);
        if publication_checkpoint_due(previous_work, self.publication_work,
            self.contract.max_work_units_per_step, previous_stage != self.stage) {
            self.checkpoint();
        }
    }

    fn advance_pack_export(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let snapshot = self.pending_snapshot.as_ref().ok_or_else(|| invariant("natural-file Pack export lost its snapshot"))?;
        let source = || snapshot.native_pack_source().ok_or_else(|| invariant("natural-file Pack source changed"));
        let epoch = cx.generation().0;
        if self.pack_export.is_none() {
            OriginalSnapshotPackExport::admit(source()?, epoch, self.contract.max_output_bytes, cx, &mut self.pack_export)?;
            return Ok(false);
        }
        let export = self.pack_export.as_mut().expect("native Pack exporter was admitted");
        if export.ready() {
            return export.take(cx, &mut self.verified_pack);
        }
        for _ in 0..64 {
            let before = cx.retained_progress();
            if export.advance(source()?, epoch, cx)? || cx.should_yield() || cx.retained_progress() == before { break; }
        }
        Ok(false)
    }

    fn adopt_genesis(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let demand = AdmittedArtifactGenesis::<E::Snapshot>::verified_pack_birth_demand()?;
        let copy = 2 * (size_of_val(&self.pending_snapshot) + size_of_val(&self.verified_pack) + size_of_val(&self.pending_genesis));
        let quote = RetirementDemand { copy_bytes: copy, capacity_bytes: demand.capacity_bytes, depth: demand.depth, ..Default::default() };
        if cx.should_yield() || !cx.retained_work_can_enter(quote) { return Ok(false); }
        let grant = cx.retained_grant();
        let Some((result, work)) = cx.with_original_effect(grant, |_, effect| -> Result<_, ValueError> {
            if !effect.performed(1)? { return Ok(None); }
            let snapshot = self.pending_snapshot.take().ok_or_else(|| invariant("natural-file genesis lost its original snapshot"))?;
            let Some(pack) = self.verified_pack.take() else {
                self.pending_snapshot = Some(snapshot);
                return Err(invariant("natural-file genesis lost its verified Pack"));
            };
            match pack.admit_snapshot(snapshot, grant) {
                Ok((genesis, mut progress)) => {
                    self.pending_genesis = Some(genesis);
                    progress.copied_bytes = copy;
                    Ok(Some(progress))
                }
                Err((error, snapshot, pack)) => {
                    self.pending_snapshot = Some(snapshot);
                    self.verified_pack = Some(pack);
                    Err(error)
                }
            }
        })? else { return Ok(false); };
        cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));
        let Some(progress) = result? else { return Ok(false); };
        cx.consume_retained(progress)?;
        Ok(true)
    }

    fn close_pack_export(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
        use semio_framework_job::InteractiveJobCloseStep as Step;

        let export = self.pack_export.as_mut().expect("native Pack exporter remains owned");
        if !export.terminal_is_empty() {
            let before = cx.retained_progress();
            export.close(cx)?;
            let progress = semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, cx.retained_progress())?;
            return Ok(Step::Pending { progress });
        }
        let demand = OriginalSnapshotPackExport::boxed_release_demands();
        let step = semio_framework_job::close_original_retained_step(cx, demand, |_| {
            self.pack_export = None;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() };
            RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(progress)), work: 1 }
        })?;
        Ok(match step { Step::Complete { progress } => Step::Pending { progress }, step => step })
    }

    fn decode_synchronous(&self) -> Result<E::Snapshot, MediaError> {
        let Some(NaturalFileDecodeOwner::Synchronous(bytes)) = self.decoder.as_ref() else {
            return Err(MediaError::Payload(self.port.clone(), "natural-file synchronous decoder lost its byte owner".into()));
        };
        E::decode_natural_file(bytes)
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
        if progress.consumed_input_bytes > self.byte_length {
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
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>, _: &mut semio_framework_job::OriginalJobReturnClaim<'_>) -> semio_framework_job::InteractiveJobStep<'a> {
        semio_framework_job::InteractiveJobStep::from_callback(self.step_original(cx))
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Step;
        self.closing=true;
        if self.pack_export.is_some() { return self.close_pack_export(cx); }
        let demand=self.close_owned_demands(cx.retained_grant().maximum_copy_bytes)?;
        let step=semio_framework_job::close_original_effect_step(cx,demand,|effect|{self.pending=None;self.close_owned_turn(effect)})?;
        Ok(match step{Step::Pending{progress}|Step::Complete{progress}=>if self.terminal_is_empty(){Step::Complete{progress}}else{Step::Pending{progress}},Step::Blocked=>Step::Blocked})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_owned_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_owned_demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.closing&&self.port.capacity()==0&&self.media.is_none()&&self.decoder.is_none()&&self.pending_snapshot.is_none()&&self.pack_export.is_none()&&self.verified_pack.is_none()&&self.pending_genesis.is_none()&&self.completion.is_none()&&self.emit.is_none()&&self.pending_completion_rejection.is_none()&&self.retirement.is_none()&&self.pending_bytes.is_none()&&self.pending_paged.is_none()&&self.pending_media_value.is_none()&&self.pending_media_text.is_none()&&self.pending_port.is_none()&&self.outcomes.terminal_is_empty()}
}

impl<E: ArtifactEditor> NaturalFileImportJob<E> {
    fn trace_original_file_worker(&self, cx: &semio_framework_job::StepContext<'_>) {
        if !semio_framework_job::runtime_diagnostics_enabled() { return; }
        thread_local! {
            static WITNESS: std::cell::Cell<(u64, u64, u8, u64)> =
                const { std::cell::Cell::new((0, 0, 255, 0)) };
        }
        let id = (cx.operation().0, cx.generation().0, self.stage as u8);
        WITNESS.with(|witness| {
            let (operation, generation, stage, calls) = witness.get();
            let calls = if (operation, generation, stage) == id { calls.saturating_add(1) } else { 1 };
            witness.set((id.0, id.1, id.2, calls));
            if calls.is_power_of_two() {
                let demand = self.pack_export.as_ref().map(|export| (export.ready(), export.advance_demands()));
                eprintln!("[DEBUG] natural-file worker operation={} phase={:?} calls={calls} bytes={}/{} pack_work={} pending_checkpoint={} checkpoint_delivered={} next_pack_demand={demand:?} copy_left={} fuel_left={} yield={}",
                    id.0, self.stage, self.byte_cursor, self.byte_length, self.publication_work, self.pending.is_some(),
                    self.outcomes.is_delivered(), cx.retained_grant().maximum_copy_bytes, cx.fuel_remaining(), cx.should_yield());
            }
        });
    }

    fn step_original<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        use semio_plugin_decl_app::tool_contract::ReservedOutcomeKind;
        self.trace_original_file_worker(cx);
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
                        Ok(NaturalFileDecodeStep::Yield) => return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
                        Ok(NaturalFileDecodeStep::Progress(progress)) => {
                            match self.admit_controlled_progress(progress, false, cx) {
                                Ok(()) => self.checkpoint(),
                                Err(error) => self.fail(&error.message),
                            }
                        }
                        Ok(NaturalFileDecodeStep::Complete { snapshot, progress }) => {
                            self.pending_snapshot = Some(snapshot);
                            match self.admit_controlled_progress(progress, true, cx) {
                                Ok(()) => {
                                    self.stage = NaturalFileImportStage::Encode;
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
                    Ok(snapshot) => {
                        self.pending_snapshot = Some(snapshot);
                        self.stage = NaturalFileImportStage::Encode;
                    }
                    Err(error) => {
                        self.emit = Some(Err(Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import"), error.to_string())));
                        self.stage = NaturalFileImportStage::Publish;
                    }
                }
                cx.consume_fuel(units as u64);
                self.checkpoint();
            }
            NaturalFileImportStage::Encode => {
                cx.set_stage("natural-file-import-encode");
                let snapshot = self.pending_snapshot.as_ref().ok_or_else(|| invariant("natural-file import lost its decoded snapshot"))?;
                if snapshot.native_pack_source().is_some() {
                    let before = cx.fuel_remaining();
                    if self.advance_pack_export(cx)? { self.stage = NaturalFileImportStage::AdoptGenesis; }
                    self.note_publication_work(NaturalFileImportStage::Encode, before, cx);
                    return Ok(None);
                }
                self.emit = Some(document_reference_load_emit(snapshot, E::DOCUMENT_SCHEMA)
                    .map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("media.natural-import"), error.to_string())));
                cx.consume_fuel(1);
                self.stage = NaturalFileImportStage::Publish;
                self.checkpoint();
            }
            NaturalFileImportStage::AdoptGenesis => {
                cx.set_stage("natural-file-import-adopt-genesis");
                let before = cx.fuel_remaining();
                if self.adopt_genesis(cx)? { self.stage = NaturalFileImportStage::Publish; }
                self.note_publication_work(NaturalFileImportStage::AdoptGenesis, before, cx);
            }
            NaturalFileImportStage::Publish => {
                cx.set_stage("natural-file-import-publish");
                if !self.completed && self.pending_genesis.is_some() {
                    let completion = self.completion.as_ref().ok_or_else(|| invariant("natural-file import lost its genesis receiver"))?;
                    if !completion.has_mounted_consumer() {
                        self.fail("natural-file import completion consumer is absent");
                        return Ok(None);
                    }
                    if !completion.complete_genesis_original(&mut self.pending_genesis, &mut self.completed, cx)? { return Ok(None); }
                }
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
}

/// 📶 Coalesces byte-level Pack work by the existing contract quantum and publishes stage boundaries immediately.
pub(crate) fn publication_checkpoint_due(previous: u64, current: u64, quantum: u64, changed_stage: bool) -> bool {
    let quantum = quantum.max(1);
    changed_stage || current / quantum != previous / quantum
}

impl<E: ArtifactEditor> ArtifactReservedJob for NaturalFileImportJob<E> {}

fn admits_paged_input(port: &str, binary: bool, length: usize, maximum: u32, raw_wire_bytes: usize) -> bool {
    port == semio_plugin_decl_app::NATURAL_FILE_PORT && binary && raw_wire_bytes == 0
        && length != 0 && length <= maximum as usize
}

pub(crate) struct NaturalFileImportWork<E:ArtifactEditor>{job:Option<NaturalFileImportJob<E>>,closing:bool}
impl<E:ArtifactEditor> Default for NaturalFileImportWork<E>{fn default()->Self{Self{job:None,closing:false}}}
impl<E:ArtifactEditor> semio_plugin_decl_app::tool_contract::ArtifactReservedWork<EditorApp<E>> for NaturalFileImportWork<E>{
 fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<E>>,cx:&mut semio_framework_job::StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_request(request,cx))}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,ValueError>{use semio_framework_job::InteractiveJob;self.job.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"natural-file work has no outcome owner"))?.borrow_outcome(descriptor)}
 fn begin_close(&mut self){self.closing=true;if let Some(job)=self.job.as_mut(){semio_framework_job::InteractiveJob::begin_close(job)}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(job)=self.job.as_ref(){job.close_owned_demands(body)}else{Ok(Default::default())}}
 fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{use semio_framework_job::InteractiveJob;if let Some(job)=self.job.as_mut(){let step=job.close_step(cx)?;if job.terminal_is_empty(){self.job=None;}return Ok(step)}Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
 fn terminal_is_empty(&self)->bool{self.job.is_none()}
}
impl<E:ArtifactEditor> NaturalFileImportWork<E>{
 fn step_request<'a>(&'a mut self,request:&mut ArtifactReservedToolJobRequest<EditorApp<E>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
  use semio_framework_value::ValueRefusalKind;
        if self.job.is_none() {
            let copied_bytes = size_of::<NaturalFileImportJob<E>>() + size_of::<ArtifactReservedToolInput>()
                + size_of::<ArtifactToolCompletion<EditorApp<E>>>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() };
            if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0
                || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
                return Ok(None);
            }
            let codec = E::natural_file_codec().ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner,
                "natural-file work lost its declared codec"))?;
            let invalid = || ValueError::literal(ValueRefusalKind::InvalidValue,
                "natural-file work source does not match its declared format or byte bound");
            let byte_length = match request.input.as_ref() {
                Some(ArtifactReservedToolInput::Media { port, media }) => {
                    let MediaPayload::Intrinsic { schema, value: DslValue::Bytes(input) } = &media.payload else { return Err(invalid().into()); };
                    if port != semio_plugin_decl_app::NATURAL_FILE_PORT || schema != codec.format_kind
                        || input.len() > request.contract.max_raw_wire_bytes { return Err(invalid().into()); }
                    input.len()
                }
                Some(ArtifactReservedToolInput::NaturalPaged { port, source }) => {
                    let length = source.original().ok_or_else(invalid)?.len();
                    if !semio_plugin_decl_app::tool_contract::admits_paged_natural_request(
                        &request.tool_id, &request.payload_schema_id, request.contract, request.raw_wire.len(),
                    ) { return Err(invalid().into()); }
                    if !admits_paged_input(port, codec.binary, length, E::PAGED_NATURAL_SOURCE_MAXIMUM_BYTES, request.raw_wire.len()) {
                        return Err(invalid().into());
                    }
                    length
                }
                _ => return Err(invalid().into()),
            };
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            let (port, media, pending_bytes, pending_paged) = match request.input.take().expect("verified natural source") {
                ArtifactReservedToolInput::Media { port, mut media } => {
                    let MediaPayload::Intrinsic { value: DslValue::Bytes(source), .. } = &mut media.payload else { unreachable!() };
                    let bytes = std::mem::take(source);
                    (port, Some(media), Some(bytes), None)
                }
                ArtifactReservedToolInput::NaturalPaged { port, source } => (port, None, None, Some(source)),
                _ => unreachable!("verified natural source purpose"),
            };
            self.job = Some(NaturalFileImportJob {
                port, media, decoder: None, completion: Some(request.completion.clone()), emit: None,
                pending_completion_rejection: None, retirement: None, pending_snapshot: None, pack_export: None,
                verified_pack: None, pending_genesis: None, publication_work: 0, pending_bytes, pending_paged,
                pending_media_value: None, pending_media_text: None, pending_port: None, contract: request.contract,
                byte_cursor: 0, byte_length, decode_progress: Default::default(), stage: NaturalFileImportStage::Decode,
                completed: false, state: [0;17], fault: semio_plugin_decl_app::tool_contract::BoundedFaultText::new(),
                pending: None, outcomes: semio_plugin_decl_app::tool_contract::ReservedJobOutcomes::new(), closing: false,
            });
            return Ok(None);
        }
        let job = self.job.as_mut().unwrap();
        if job.decoder.is_none() {
            if job.pending_paged.is_some() {
                let admitted = E::admit_paged_natural_file_decode_owner(&mut job.pending_paged, job.contract, &mut job.decoder, cx)?;
                if admitted != matches!(job.decoder, Some(NaturalFileDecodeOwner::Controlled(_)))
                    || admitted == job.pending_paged.is_some() || matches!(job.decoder, Some(NaturalFileDecodeOwner::Synchronous(_))) {
                    return Err(invariant("paged natural decoder changed original source ownership").into());
                }
            } else if E::admit_natural_file_decode_owner(&mut job.pending_bytes, job.contract, &mut job.decoder, cx)?
                && matches!(job.decoder, Some(NaturalFileDecodeOwner::Synchronous(_))) {
                job.stage = NaturalFileImportStage::Account;
            }
            return Ok(None);
        }
        job.step_original(cx)
 }
}

impl<E:ArtifactEditor> NaturalFileImportJob<E>{
    fn close_owned_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        use semio_framework_value::retirement::owned_retirement_birth_bytes;
        if let Some(export) = self.pack_export.as_ref() {
            return if export.terminal_is_empty() { Ok(OriginalSnapshotPackExport::boxed_release_demands()) } else { export.close_demands() };
        }
        if !self.outcomes.terminal_is_empty(){return self.outcomes.close_demands();}
        if let Some(ticket)=self.retirement.as_ref(){return semio_framework_value::factory_ticket_demands(ticket,body);}
        if self.pending_genesis.is_some() { return Ok(RetirementDemand { capacity_bytes: owned_retirement_birth_bytes::<AdmittedArtifactGenesis<E::Snapshot>>(), depth: 1, ..Default::default() }); }
        if self.verified_pack.is_some() { return Ok(RetirementDemand { capacity_bytes: owned_retirement_birth_bytes::<OriginalSnapshotPackOutput>(), depth: 1, ..Default::default() }); }
        if self.pending_snapshot.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<E::Snapshot>(),depth:1,..Default::default()});}
        if let Some(source) = self.pending_paged.as_ref() {
            return if source.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<NaturalFilePagedSource>>(), depth: 1, ..Default::default() })
            } else { source.next_demand(body) };
        }
        if self.pending_bytes.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<Vec<u8>>(),depth:1,..Default::default()});}
        if self.pending_media_value.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<(String,DslValue)>(),depth:1,..Default::default()});}
        if self.pending_media_text.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<(String,String)>(),depth:1,..Default::default()});}
        if self.pending_port.is_some(){return Ok(RetirementDemand{capacity_bytes:owned_retirement_birth_bytes::<String>(),depth:1,..Default::default()});}
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
    fn close_owned_turn(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        let grant=effect.grant();let body=grant.maximum_copy_bytes;
        if !self.outcomes.terminal_is_empty(){let step=self.outcomes.close_step(effect);return settled(step,effect.performed_work())}
        if self.retirement.is_some(){return close_factory_ticket(&mut self.retirement,body,effect)}
        if self.pending_genesis.is_some() { return admit(&mut self.pending_genesis, &mut self.retirement, effect); }
        if self.verified_pack.is_some() { return admit(&mut self.verified_pack, &mut self.retirement, effect); }
        if self.pending_snapshot.is_some(){return admit(&mut self.pending_snapshot,&mut self.retirement,effect)}
        if let Some(source) = self.pending_paged.as_mut() {
            if !source.terminal_is_empty() { return source.step(body, effect); }
            let bytes = size_of::<Option<NaturalFilePagedSource>>();
            let work = record_retirement_work(effect, 1);
            if work.is_ok() { self.pending_paged.take(); }
            return settled(work.map(|_| RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }))
                .map_err(|(error, _)| error), effect.performed_work());
        }
        if self.pending_bytes.is_some(){return admit(&mut self.pending_bytes,&mut self.retirement,effect)}
        if self.pending_media_value.is_some(){return admit(&mut self.pending_media_value,&mut self.retirement,effect)}
        if self.pending_media_text.is_some(){return admit(&mut self.pending_media_text,&mut self.retirement,effect)}
        if self.pending_port.is_some(){return admit(&mut self.pending_port,&mut self.retirement,effect)}
        if let Some(decoder)=self.decoder.as_mut(){match decoder{
            NaturalFileDecodeOwner::Controlled(owner)=>{
                if owner.terminal_is_empty(){let released_bytes=size_of_val(&**owner);self.decoder.take();let work=record_retirement_work(effect,1);return settled(work.map(|_|RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()})).map_err(|(error,_)|error),effect.performed_work())}
                owner.begin_close();let step=owner.close_step(effect).map_err(|_|invariant("natural-file decoder close failed while retaining its original owner"));
                let step=step.and_then(|step|{if step.progress().is_some_and(|progress|!progress.fits(grant))||matches!(step,PluginLifecycleStep::Complete(_))&&!owner.terminal_is_empty(){return Err(invariant("natural-file decoder violated its original retirement grant or terminal witness"))}Ok(match step{PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)=>RetainedCloneStep::Progress(progress),PluginLifecycleStep::Blocked{..}|PluginLifecycleStep::AwaitingInput{..}=>RetainedCloneStep::Progress(Default::default())})});
                let work=record_retirement_work(effect,1);return settled(work.map_err(|(error,_)|error).and_then(|_|step),effect.performed_work())
            },
            NaturalFileDecodeOwner::Synchronous(raw)=>{let mut original=Some(std::mem::take(raw));let poll=admit(&mut original,&mut self.retirement,effect);match original{None=>{self.decoder.take();}Some(original)=>*raw=original}return poll}
        }}
        if self.pending_completion_rejection.is_some(){return admit(&mut self.pending_completion_rejection,&mut self.retirement,effect)}
        if self.emit.is_some(){return admit(&mut self.emit,&mut self.retirement,effect)}
        if self.media.is_some(){return admit(&mut self.media,&mut self.retirement,effect)}
        if self.port.capacity()!=0{let mut original=Some(std::mem::take(&mut self.port));let poll=admit(&mut original,&mut self.retirement,effect);if let Some(original)=original{self.port=original}return poll}
        if self.completion.is_some(){return admit(&mut self.completion,&mut self.retirement,effect)}
        settled(Ok(RetainedCloneStep::Complete(Default::default())),0)
    }
}
