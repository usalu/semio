//! 📦️ Typed pack load: the load original, the typed window config pack load with its stepping, its erased pack load implementation and drop, and the begin and commit entry points. See [`super`].

use super::*;
use crate::owner_contract_and_mutation::WindowConfigAuthorityIdentity;

#[derive(semio_framework_value::RetireOwned)]
enum LoadOriginal<O: WindowConfigOwner> {
    State(O::State),
    History(store::HistoryLog),
    Files((Vec<u8>, Vec<u8>, String)),
    Text(String),
    SharedText(semio_framework_value::SharedUtf8),
}

/// 📜️ The History read policy of one window config pack: its SPR bytes are the whole file and frame ceiling, its record count the envelope ceiling.
fn window_config_history_limits(spr_bytes: u32) -> store::os_store::HistoryReadLimits {
    store::os_store::HistoryReadLimits {
        file_bytes: spr_bytes,
        frame_body_bytes: spr_bytes,
        records: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES as u32,
        maximum_fields: 128,
        maximum_text_bytes: 1_048_576,
        maximum_binary_bytes: 1_048_576,
        maximum_json_bytes: 1_048_576,
        maximum_json_allocation_bytes: 8_388_608,
        maximum_json_items: 65_536,
        maximum_json_depth: 64,
    }
}

/// 🧮️ The cumulative native allocation ceiling of one window config History decode.
const WINDOW_CONFIG_HISTORY_ALLOCATION_BYTES: usize = 8_388_608;

pub(super) struct TypedWindowConfigPackLoad<O: WindowConfigOwner> {
    pub(super) candidate_authority: Option<WindowConfigAuthorityIdentity>,
    completed_states:ManuallyDrop<[Option<RetainedWindowConfigStateDecode<O>>;3]>,
    pending_files:ManuallyDrop<Option<store::os_store::serialization::ArtifactPackFiles>>,
    pending_address:ManuallyDrop<Option<String>>,
    opened_actor: ManuallyDrop<Option<protocol::ActorId>>,
    window_id: ManuallyDrop<Option<String>>,
    pub(super) window_kind_id: ManuallyDrop<Option<String>>,
    expected_id: ManuallyDrop<Option<String>>,
    files: ManuallyDrop<Option<store::os_store::serialization::ArtifactPackFiles>>,
    state_decode: ManuallyDrop<Option<RetainedWindowConfigStateDecode<O>>>,
    initial: ManuallyDrop<Option<O::State>>,

    validation: ManuallyDrop<Option<O::State>>,
    current: ManuallyDrop<Option<O::State>>,
    initial_digest: Option<[u8; 32]>,
    history_decode: ManuallyDrop<Option<store::RetainedHistoryDecode>>,
    history: ManuallyDrop<Option<store::HistoryLog>>,
    hydration: ManuallyDrop<Option<store::ReferenceConfigStoreHydration<O::State, O::Mutation>>>,
    pub(super) owners: ManuallyDrop<Option<store::os_store::document_store_owners::DocumentStoreOwners<O::State, O::Mutation>>>,
    pub(super) candidate: ManuallyDrop<Option<WindowConfigPartition<O>>>,
    displaced: ManuallyDrop<Option<WindowConfigPartition<O>>>,
    pub(super) active: Option<ControlledRetirement<LoadOriginal<O>>>,
    pub(super) original: ManuallyDrop<Option<LoadOriginal<O>>>,
    pub(super) registry_lifetime: u64,
    partition_generation: Option<u64>,
    pub(super) phase: WindowConfigPackLoadPhase,
    pub(super) diagnostic: Option<WindowConfigPackLoadDiagnostic>,
    pub(super) completed_bytes: u64,
    pub(super) total_bytes: u64,
    decoded_states: u8,
    pub(crate) committed: bool,
    terminal: bool,
}

impl<O: WindowConfigOwner> TypedWindowConfigPackLoad<O> {
    pub(super) fn new(registry_lifetime: u64, partition_generation: Option<u64>, pack: WindowConfigPack, opened_actor: protocol::ActorId) -> Self {
        let WindowConfigPack { window_id, window_kind_id, files } = pack;
        let total_bytes = files.pack.len().saturating_mul(3).saturating_add(files.spr.len()).saturating_add(files.ops.len()) as u64;
        let expected_id = format!("window-config:{}:{window_id}", O::WINDOW_KIND_ID);
        let candidate_authority = crate::owner_contract_and_mutation::WindowConfigAuthorityIdentity::issue();
        let owners=None;let mut diagnostic=None;
        if candidate_authority.is_none() { diagnostic = Some(WindowConfigPackLoadDiagnostic::Capacity); }
        Self {
            candidate_authority,
            completed_states:ManuallyDrop::new(std::array::from_fn(|_|None)),
            pending_files:ManuallyDrop::new(None),
            pending_address:ManuallyDrop::new(None),
            opened_actor: ManuallyDrop::new(Some(opened_actor)),
            window_id: ManuallyDrop::new(Some(window_id)),
            window_kind_id: ManuallyDrop::new(Some(window_kind_id)),
            expected_id: ManuallyDrop::new(Some(expected_id)),
            files: ManuallyDrop::new(Some(files)),
            state_decode: ManuallyDrop::new(Some(RetainedWindowConfigStateDecode::new())),
            initial: ManuallyDrop::new(None),

            validation: ManuallyDrop::new(None),
            current: ManuallyDrop::new(None),
            initial_digest: None,
            history_decode: ManuallyDrop::new(None),
            history: ManuallyDrop::new(None),
            hydration: ManuallyDrop::new(None),
            owners: ManuallyDrop::new(owners),
            candidate: ManuallyDrop::new(None),
            displaced: ManuallyDrop::new(None),
            active: None,
            original: ManuallyDrop::new(None),
            registry_lifetime,
            partition_generation,
            phase: if diagnostic.is_some() { WindowConfigPackLoadPhase::RetiringRejectedCandidate } else { WindowConfigPackLoadPhase::EnvelopeIdentity },
            diagnostic,
            completed_bytes: 0,
            total_bytes,
            decoded_states: 0,
            committed: false,
            terminal: false,
        }
    }

    fn progress_now(&self) -> WindowConfigPackLoadProgress {
        WindowConfigPackLoadProgress { phase: self.phase, completed_bytes: self.completed_bytes.min(self.total_bytes), total_bytes: self.total_bytes }
    }

    #[doc(hidden)]
    pub fn pending(&self) -> WindowConfigPackLoadStep {
        WindowConfigPackLoadStep::Pending(self.progress_now())
    }

    fn reject(&mut self, diagnostic: WindowConfigPackLoadDiagnostic) -> WindowConfigPackLoadStep {
        self.diagnostic.get_or_insert(diagnostic);
        self.phase = WindowConfigPackLoadPhase::RetiringRejectedCandidate;
        WindowConfigPackLoadStep::Rejected(self.diagnostic.unwrap())
    }

    fn map_hydration_diagnostic(diagnostic: store::ConfigStoreHydrationDiagnostic) -> WindowConfigPackLoadDiagnostic {
        match diagnostic {
            store::ConfigStoreHydrationDiagnostic::Identity => WindowConfigPackLoadDiagnostic::InnerIdentity,
            store::ConfigStoreHydrationDiagnostic::Malformed => WindowConfigPackLoadDiagnostic::History,
            store::ConfigStoreHydrationDiagnostic::Replay => WindowConfigPackLoadDiagnostic::Replay,
            store::ConfigStoreHydrationDiagnostic::Capacity => WindowConfigPackLoadDiagnostic::Capacity,
            store::ConfigStoreHydrationDiagnostic::Initialization => WindowConfigPackLoadDiagnostic::Replay,
            store::ConfigStoreHydrationDiagnostic::Cancelled => WindowConfigPackLoadDiagnostic::Cancelled,
        }
    }

    fn advance_decode(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let pack = &self.files.as_ref().expect("retained window config input remains").pack;
        let decoder = self.state_decode.as_mut().expect("retained window config state decoder remains");
        let before = decoder.admitted;
        match decoder.advance(pack, RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}) {
            Ok(_) => {
                self.completed_bytes = self.completed_bytes.saturating_add(decoder.admitted.saturating_sub(before) as u64);
                self.phase = match decoder.phase {
                    RetainedStatePhase::Envelope => WindowConfigPackLoadPhase::EnvelopeIdentity,
                    RetainedStatePhase::Ingress => WindowConfigPackLoadPhase::PackIngress,
                    RetainedStatePhase::Replay => WindowConfigPackLoadPhase::PackReplay,
                    RetainedStatePhase::Ready => {
                        let (state, digest) = decoder.take_ready().expect("ready retained window config state remains");
                        if self.initial_digest.is_some_and(|expected| expected != digest) {
                            return self.reject(WindowConfigPackLoadDiagnostic::TypedState);
                        }
                        self.initial_digest.get_or_insert(digest);
                        match self.decoded_states {
                            0 => *self.initial = Some(state),
                            1 => *self.validation = Some(state),
                            2 => *self.current = Some(state),
                            _ => return self.reject(WindowConfigPackLoadDiagnostic::TypedState),
                        }
                        self.decoded_states += 1;
                        WindowConfigPackLoadPhase::PackRetirement
                    }
                    RetainedStatePhase::Closing | RetainedStatePhase::Closed => return self.reject(WindowConfigPackLoadDiagnostic::Pack),
                };
                self.pending()
            }
            Err(diagnostic) => self.reject(diagnostic),
        }
    }

    fn advance_pack_retirement(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let decoder = self.state_decode.as_mut().expect("retained window config state decoder remains during close");
        match decoder.close_step(grant) {
            Ok(RetainedCloneStep::Complete(_)) if decoder.terminal_is_empty() => {
                self.state_decode.take();
                if self.decoded_states < 3 {
                    *self.state_decode = Some(RetainedWindowConfigStateDecode::new());
                    self.phase = WindowConfigPackLoadPhase::EnvelopeIdentity;
                    return self.pending();
                }
                let spr_len = self.files.as_ref().expect("retained window config SPR input remains").spr.len();
                let admitted = u32::try_from(spr_len).ok().and_then(|bytes| store::RetainedHistoryDecode::new((0, spr_len), window_config_history_limits(bytes), true, WINDOW_CONFIG_HISTORY_ALLOCATION_BYTES).ok());
                match admitted {
                    Some(decoder) => {
                        *self.history_decode = Some(decoder);
                        self.phase = WindowConfigPackLoadPhase::HistoryReplay;
                        self.pending()
                    }
                    None => self.reject(WindowConfigPackLoadDiagnostic::History),
                }
            }
            Ok(RetainedCloneStep::Complete(_)) => self.reject(WindowConfigPackLoadDiagnostic::Retirement),
            Ok(_) => self.pending(),
            Err(_) => self.reject(WindowConfigPackLoadDiagnostic::Retirement),
        }
    }

    fn advance_history(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let files = self.files.as_ref().expect("retained window config SPR input remains");
        let pack_bytes = files.pack.len() as u64;
        let decoder = self.history_decode.as_mut().expect("retained window config history decoder remains");
        match decoder.step(&files.spr, &store::RetainedHistoryUninterrupted, grant, &mut self.history) {
            Ok(store::RetainedHistorySourceStep::Progress | store::RetainedHistorySourceStep::Blocked) => {
                self.completed_bytes = pack_bytes.saturating_add(decoder.consumed_end() as u64);
                self.pending()
            }
            Ok(store::RetainedHistorySourceStep::Ready) => {
                if self.history.is_none() || !decoder.terminal_is_empty() {
                    return self.reject(WindowConfigPackLoadDiagnostic::Retirement);
                }
                drop(self.history_decode.take());
                self.phase = WindowConfigPackLoadPhase::InputRetirement;
                self.pending()
            }
            Ok(store::RetainedHistorySourceStep::Fault | store::RetainedHistorySourceStep::Closed) | Err(_) => self.reject(WindowConfigPackLoadDiagnostic::History),
        }
    }

    fn adopt_original(&mut self, original: LoadOriginal<O>) -> Result<(), ValueError> {
        match ControlledRetirement::new(original) {
            Ok(owner) => { self.active = Some(owner); Ok(()) }
            Err((error, original)) => { *self.original = Some(original); Err(error) }
        }
    }

    fn drive_active(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, ValueError> {
        let Some(owner) = self.active.as_mut() else { return Ok(None) };
        let step = owner.step(grant)?;
        if owner.terminal_is_empty() { self.active = None; }
        Ok(Some(RetainedCloneStep::Progress(step.progress())))
    }

    fn original_demand() -> RetirementDemand { RetirementDemand { copy_bytes: size_of::<LoadOriginal<O>>(), depth: 1, ..Default::default() } }

    fn retire_files(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, ValueError> {
        if self.files.is_none() { return Ok(None); }
        let demand = Self::original_demand();
        if !super::super::registry::fits(grant, demand) { return Ok(Some(RetainedCloneStep::Progress(Default::default()))); }
        let files = self.files.take().expect("granted original files remain");
        self.adopt_original(LoadOriginal::Files((files.pack, files.spr, files.ops)))?;
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })))
    }

    fn advance_input_retirement(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        match self.drive_active(grant) {
            Ok(Some(_)) => return self.pending(),
            Ok(None) => {}
            Err(_) => return self.reject(WindowConfigPackLoadDiagnostic::Retirement),
        }
        if self.hydration.is_none() {
            if grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return self.pending();
            }
            let initial = self.initial.take().expect("decoded initial window config state remains");
            let pack = std::mem::take(&mut self.files.as_mut().expect("verified window config Pack remains").pack);
            let digest = self.initial_digest.take().expect("decoded window config digest remains");
            let genesis = semio_framework_os_kernel::os_vcs::io::binary::genesis::AdmittedArtifactGenesis::from_verified_pack(initial, pack, digest);
            let validation = self.validation.take().expect("decoded validation window config state remains");
            let current = self.current.take().expect("decoded current window config state remains");
            let history = self.history.take().expect("decoded window config history remains");
            let expected_id = self.expected_id.take().expect("exact window config partition id remains");
            let owners = self.owners.take().expect("window config store owners remain");
            *self.hydration = Some(store::ReferenceConfigStoreHydration::from_snapshots(
                genesis,
                validation,
                current,
                history,
                expected_id,
                O::SCHEMA.to_string(),
                owners,
                self.partition_generation.map_or(0, |generation| generation.saturating_add(1)),
                O::MAXIMUM_PUBLICATION_BYTES,
                self.opened_actor.take().expect("retained window load retains its opened actor before hydration"),
            ));
            return self.pending();
        }
        match self.retire_files(grant) { Ok(Some(_)) => return self.pending(), Ok(None) => {}, Err(_) => return self.reject(WindowConfigPackLoadDiagnostic::Retirement) }
        self.phase = WindowConfigPackLoadPhase::StoreHydration;
        self.pending()
    }

    fn advance_hydration(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        let hydration = self.hydration.as_mut().expect("retained window config hydration remains");
        match hydration.advance(grant) {
            store::ConfigStoreHydrationStep::Pending(_) => self.pending(),
            store::ConfigStoreHydrationStep::Rejected(diagnostic) => self.reject(Self::map_hydration_diagnostic(diagnostic)),
            store::ConfigStoreHydrationStep::Ready(store) => {

                *self.candidate = Some(WindowConfigPartition { catalog:None,address:None,semantic_preview:None,semantic_epoch:0,published_preview:None,finish_preview_base:None,finish_preview_read:None, authority: self.candidate_authority.expect("admitted original candidate authority"), store: *store, disposer: Some(O::build_store_disposer()), pending_preview:None,pending_preview_reads:None,pending_preview_projection:None,pending_preview_projection_read:None,preview_retirement:None,returned_read_retirement:None });
                self.phase = WindowConfigPackLoadPhase::StoreAddress;
                self.pending()
            }
        }
    }

    fn advance_address(&mut self,grant:RetainedCloneGrant)->WindowConfigPackLoadStep{
        let demand=semio_framework_value::SharedUtf8::admission_demand();if !crate::registry::fits(grant,demand){return self.pending();}let original=self.window_id.take().expect("original hydrated Window keeps its exact address");match semio_framework_value::SharedUtf8::admit(original,grant){Ok((address,p))=>{self.candidate.as_mut().expect("original hydrated Window candidate remains").address=Some(ControlledRetirement::new(address).expect("original Window address supports controlled closure"));self.phase=WindowConfigPackLoadPhase::Ready;WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p)},Err((_,original))=>{*self.window_id=Some(original);self.reject(WindowConfigPackLoadDiagnostic::Capacity)}}
    }

    fn advance_inner(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        if let Some(diagnostic) = self.diagnostic {
            return WindowConfigPackLoadStep::Rejected(diagnostic);
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return self.pending();
        }
        if self.owners.is_none()&&self.hydration.is_none()&&self.candidate.is_none()&&matches!(self.phase,WindowConfigPackLoadPhase::EnvelopeIdentity){return match O::build_store_owners(grant){Ok((owners,p))=>{*self.owners=Some(owners);WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p)},Err(error)=>{*self.owners=error.owners;if self.owners.is_none()&&error.progress==Default::default(){self.pending()}else{self.diagnostic=Some(WindowConfigPackLoadDiagnostic::Capacity);WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),error.progress)}}}}
        if let Some(owners)=self.owners.as_mut(){if !owners.constructor_is_complete(){return match owners.admit_constructor(grant){Ok(p)=>WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p),Err((_,p))=>{self.diagnostic=Some(WindowConfigPackLoadDiagnostic::Capacity);WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),p)}}}}
        match self.phase {
            WindowConfigPackLoadPhase::EnvelopeIdentity | WindowConfigPackLoadPhase::PackIngress | WindowConfigPackLoadPhase::PackReplay => self.advance_decode(grant),
            WindowConfigPackLoadPhase::PackRetirement => self.advance_pack_retirement(grant),
            WindowConfigPackLoadPhase::HistoryReplay => self.advance_history(grant),
            WindowConfigPackLoadPhase::InputRetirement => self.advance_input_retirement(grant),
            WindowConfigPackLoadPhase::StoreHydration => self.advance_hydration(grant),
            WindowConfigPackLoadPhase::StoreAddress => self.advance_address(grant),
            WindowConfigPackLoadPhase::Ready => WindowConfigPackLoadStep::Ready,
            WindowConfigPackLoadPhase::RetiringRejectedCandidate => WindowConfigPackLoadStep::Rejected(self.diagnostic.unwrap_or(WindowConfigPackLoadDiagnostic::Cancelled)),
            WindowConfigPackLoadPhase::RetiringDisplacedStore => self.pending(),
            WindowConfigPackLoadPhase::Complete => WindowConfigPackLoadStep::Complete,
        }
    }

    pub(super) fn reject_stale(&mut self) -> WindowConfigPackLoadStep {
        self.reject(WindowConfigPackLoadDiagnostic::Stale)
    }

    fn install(&mut self, partitions: &mut WindowRegistry<String, WindowConfigPartition<O>>, registry_lifetime: u64) -> WindowConfigPackLoadStep {
        if self.phase != WindowConfigPackLoadPhase::Ready || self.registry_lifetime != registry_lifetime {
            return self.reject_stale();
        }
        let window_id = self.candidate.as_ref().and_then(|candidate|candidate.address.as_ref()).and_then(ControlledRetirement::original).expect("ready window config load retains its original shared address").as_str();
        let live_generation = partitions.get(window_id).map(|partition| partition.store.generation());
        if live_generation != self.partition_generation {
            return self.reject_stale();
        }
        let address=window_id.to_string();
        let candidate = self.candidate.take().expect("ready window config candidate remains");
        *self.displaced = partitions.insert(address, candidate);
        self.committed = true;
        self.phase = WindowConfigPackLoadPhase::RetiringDisplacedStore;
        self.pending()
    }

    fn partition_demands(partition: &WindowConfigPartition<O>, body: usize) -> Result<RetirementDemand, ValueError> {
        if partition.preview_close_pending() { return partition.preview_close_demand(body); }
        if partition.returned_read_pending() { return partition.returned_read_demands(body); }
        if let Some(address)=partition.address.as_ref(){return if address.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<semio_framework_value::SharedUtf8>>>(),depth:1,..Default::default()})}else{address.next_demand(body)};}
        partition.disposer.as_ref().map_or(Ok(RetirementDemand { copy_bytes: size_of::<WindowConfigPartition<O>>(), depth: 1, ..Default::default() }), |owner| {
            if owner.terminal_is_empty(&partition.store) { Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }) }
            else { owner.retirement_demands(&partition.store, body) }
        })
    }

    fn close_partition(slot: &mut Option<WindowConfigPartition<O>>, grant: RetainedCloneGrant, demand: RetirementDemand) -> Result<PluginLifecycleStep, String> {
        let partition = slot.as_mut().expect("granted original partition remains");
        if partition.preview_close_pending() { return partition.preview_close_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
        if partition.returned_read_pending() { return partition.returned_read_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
        if let Some(address)=partition.address.as_mut(){if address.terminal_is_empty(){partition.address=None;return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}return address.step(grant).map(|step|PluginLifecycleStep::retained(step,false)).map_err(ValueError::into_message);}
        if let Some(owner) = partition.disposer.as_mut() {
            if !owner.terminal_is_empty(&partition.store) { return owner.close_step(&mut partition.store, grant).map(|step| match step { PluginLifecycleStep::Complete(progress) => PluginLifecycleStep::Progress(progress), other => other }).map_err(|fault| fault.message); }
            drop(partition.disposer.take());
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        drop(slot.take());
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    #[doc(hidden)]
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let frame = |copy_bytes| Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
        if self.terminal { return Ok(Default::default()); }
        if self.original.is_some() { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "retained original window input has no controlled authority")); }
        if let Some(owner) = self.active.as_ref() { return controlled_demands(owner, body); }
        if let Some(owner) = self.state_decode.as_ref() { return if owner.terminal_is_empty() { frame(size_of::<RetainedWindowConfigStateDecode<O>>()) } else { owner.retirement_demands(body) }; }
        if let Some(owner) = self.history_decode.as_ref() { return owner.demands(body); }
        if let Some(owner) = self.hydration.as_ref() {
            return if store::ErasedSnapshotRetirement::terminal_is_empty(owner) { frame(size_of::<store::ReferenceConfigStoreHydration<O::State, O::Mutation>>()) }
            else { Ok(RetirementDemand { copy_bytes: store::ErasedSnapshotRetirement::next_copy_byte_demand(owner)?, capacity_bytes: store::ErasedSnapshotRetirement::next_capacity_byte_demand(owner, body)?, release_bytes: store::ErasedSnapshotRetirement::next_release_byte_demand(owner)?, depth: store::ErasedSnapshotRetirement::next_depth_demand(owner)? }) };
        }
        if let Some(partition) = self.displaced.as_ref().or(self.candidate.as_ref()) { return Self::partition_demands(partition, body); }
        if self.current.is_some() || self.validation.is_some() || self.initial.is_some() || self.history.is_some() || self.files.is_some() { return Ok(Self::original_demand()); }
        if let Some(owner) = self.owners.as_ref() { return if owner.uninstalled_owners_terminal_is_empty() { frame(size_of::<store::os_store::document_store_owners::DocumentStoreOwners<O::State, O::Mutation>>()) } else { owner.uninstalled_owners_demands(body) }; }
        if self.opened_actor.is_some() || self.expected_id.is_some() || self.window_kind_id.is_some() || self.window_id.is_some() { return Ok(Self::original_demand()); }
        Ok(RetirementDemand { depth: 1, ..Default::default() })
    }

    fn close_inner(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, String> {
        if self.terminal { return Ok(PluginLifecycleStep::Complete(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_message)?;
        if !super::super::registry::fits(grant, demand) { return Ok(PluginLifecycleStep::Progress(Default::default())); }
        self.diagnostic.get_or_insert(WindowConfigPackLoadDiagnostic::Cancelled);
        if let Some(step) = self.drive_active(grant).map_err(ValueError::into_message)? { return Ok(PluginLifecycleStep::retained(step, false)); }
        if let Some(owner) = self.state_decode.as_mut() {
            if !owner.terminal_is_empty() { return owner.close_step(grant).map(|step| PluginLifecycleStep::retained(step, false)); }
            drop(self.state_decode.take());
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(owner) = self.history_decode.as_mut() {
            if owner.terminal_is_empty() {
                drop(self.history_decode.take());
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
            }
            let files = self.files.as_ref().ok_or_else(|| "retained window config SPR input is absent while its history closes".to_string())?;
            return owner.close_step(&files.spr, &store::RetainedHistoryUninterrupted, grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message);
        }
        if let Some(owner) = self.hydration.as_mut() {
            if !store::ErasedSnapshotRetirement::terminal_is_empty(owner) { return store::ErasedSnapshotRetirement::close_step(owner, grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
            drop(self.hydration.take());
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if self.displaced.is_some() { return Self::close_partition(&mut self.displaced, grant, demand); }
        if self.candidate.is_some() { return Self::close_partition(&mut self.candidate, grant, demand); }
        if let Some(state) = self.current.take().or_else(|| self.validation.take()).or_else(|| self.initial.take()) {
            self.adopt_original(LoadOriginal::State(state)).map_err(ValueError::into_message)?;
        } else if let Some(history) = self.history.take() {
            self.adopt_original(LoadOriginal::History(history)).map_err(ValueError::into_message)?;
        } else if let Some(step) = self.retire_files(grant).map_err(ValueError::into_message)? {
            return Ok(PluginLifecycleStep::retained(step, false));
        } else if let Some(owner) = self.owners.as_mut() {
            if !owner.uninstalled_owners_terminal_is_empty() { return owner.close_uninstalled_owners_step(grant).map(|step| PluginLifecycleStep::retained(step, false)).map_err(ValueError::into_message); }
            drop(self.owners.take());
        } else if let Some(actor) = self.opened_actor.take() {
            self.adopt_original(LoadOriginal::SharedText(actor.0)).map_err(ValueError::into_message)?;
        } else if let Some(text) = self.expected_id.take().or_else(|| self.window_kind_id.take()).or_else(|| self.window_id.take()) {
            self.adopt_original(LoadOriginal::Text(text)).map_err(ValueError::into_message)?;
        } else {
            self.initial_digest = None;
            self.terminal = true;
            self.phase = WindowConfigPackLoadPhase::Complete;
            return Ok(PluginLifecycleStep::Complete(Default::default()));
        }
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn ownership_is_empty(&self) -> bool {
        self.terminal
            && self.completed_states.iter().all(Option::is_none)
            && self.pending_files.is_none()
            && self.pending_address.is_none()
            && self.opened_actor.is_none()
            && self.window_id.is_none()
            && self.window_kind_id.is_none()
            && self.expected_id.is_none()
            && self.files.is_none()
            && self.state_decode.is_none()
            && self.initial.is_none()

            && self.validation.is_none()
            && self.current.is_none()
            && self.initial_digest.is_none()
            && self.history_decode.is_none()
            && self.history.is_none()
            && self.hydration.is_none()
            && self.owners.is_none()
            && self.candidate.is_none()
            && self.displaced.is_none()
            && self.active.is_none()
            && self.original.is_none()
    }
}

impl<O: WindowConfigOwner> ErasedWindowConfigPackLoad for TypedWindowConfigPackLoad<O> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn window_kind_id(&self) -> &str {
        self.window_kind_id.as_deref().unwrap_or("")
    }

    fn registry_lifetime(&self) -> u64 {
        self.registry_lifetime
    }

    fn phase(&self) -> WindowConfigPackLoadPhase {
        self.phase
    }

    fn progress(&self) -> WindowConfigPackLoadProgress {
        self.progress_now()
    }

    fn diagnostic(&self) -> Option<WindowConfigPackLoadDiagnostic> {
        self.diagnostic
    }

    fn advance(&mut self, grant: RetainedCloneGrant) -> WindowConfigPackLoadStep {
        self.advance_inner(grant)
    }

    fn request_cancel(&mut self) {
        self.reject(WindowConfigPackLoadDiagnostic::Cancelled);
    }

    fn reject_stale(&mut self) -> WindowConfigPackLoadStep {
        TypedWindowConfigPackLoad::reject_stale(self)
    }

    fn demand_bytes(&self) -> usize {
        let envelope_id = <O::State as pack::snapshot::ArtifactDsl>::envelope_id();
        12usize.saturating_add(envelope_id.len()).saturating_add(".pack v1".len()).saturating_add(size_of::<RetainedWindowConfigTypedState<O>>())
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { TypedWindowConfigPackLoad::retirement_demands(self, body) }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, String> { self.close_inner(grant) }

    fn terminal_is_empty(&self) -> bool {
        self.ownership_is_empty()
    }
}

impl<O: WindowConfigOwner> Drop for TypedWindowConfigPackLoad<O> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "typed window config Pack load reached Drop before terminal-empty handoff or retirement");
    }
}

pub(in super::super) fn begin_typed_window_config_pack_load<O: WindowConfigOwner>(registry_lifetime: u64, partition_generation: Option<u64>, pack: WindowConfigPack, opened_actor: protocol::ActorId) -> WindowConfigPackLoad {
    let over_bound = pack.files.pack.len() > O::MAXIMUM_PUBLICATION_BYTES;
    let mut load = TypedWindowConfigPackLoad::<O>::new(registry_lifetime, partition_generation, pack, opened_actor);
    if over_bound {
        load.reject(WindowConfigPackLoadDiagnostic::Capacity);
    }
    WindowConfigPackLoad { inner: ManuallyDrop::new(Some(Box::new(load))),closed_progress:WindowConfigPackLoadProgress{phase:WindowConfigPackLoadPhase::Complete,completed_bytes:0,total_bytes:0},closed_diagnostic:None }
}

pub(in super::super) fn commit_typed_window_config_pack_load<O: WindowConfigOwner>(
    registry_lifetime: u64,
    partitions: &mut WindowRegistry<String, WindowConfigPartition<O>>,
    load: &mut dyn ErasedWindowConfigPackLoad,
) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic> {
    let typed = load.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>().ok_or(WindowConfigPackLoadDiagnostic::Stale)?;
    Ok(typed.install(partitions, registry_lifetime))
}
