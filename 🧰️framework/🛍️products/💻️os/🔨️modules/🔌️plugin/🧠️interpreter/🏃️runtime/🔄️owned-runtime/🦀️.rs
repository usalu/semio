//! 🔄️ Owned runtime: the operations of the owned interpreter runtime over its guest instances. See [`super`].

use super::*;

//#region 🧠️OwnedRuntime
impl OwnedRuntime {
    /// 🌱️ A runtime whose first instance is id 1.
    pub fn new(original_clock:fn()->Option<u64>) -> Self {
        Self { next_instance_id: std::sync::atomic::AtomicU32::new(1), original_clock }
    }

    /// 🧩️ Parses `bytes` as an owned-ABI component, ready to instantiate; no codec origin is assembled yet.
    pub fn compile_component(&self, bytes: &[u8]) -> Result<OwnedCompiledGuest, PluginHostError> {
        let artifact = OwnedSemioArtifact::parse(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        Ok(OwnedCompiledGuest { artifact: Arc::new(artifact), codec_origin: Arc::new(OwnedCodecOriginCell::new()) })
    }

    /// 🏃️ A fresh instance of `compiled` whose app channel is admitted ([`admit_owned_channel`]).
    pub fn instantiate_actor(&self, compiled: &OwnedCompiledGuest) -> Result<OwnedGuestInstance, PluginHostError> {
        let artifact = &compiled.artifact;
        let owned = artifact.instantiate().map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        if owned.startup_active() {
            return Err(PluginHostError::Plugin("owned actor has an undriven start function".to_string()));
        }
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut state = OwnedGuestInstance { original_job_clock:None, artifact: Arc::clone(artifact), actor:Some(owned), original_core:std::mem::ManuallyDrop::new(None),original_source:std::mem::ManuallyDrop::new(None),original_startup_input:None,original_call:None, pending: None, original_receipt: None, poisoned: false, diagnostics: Vec::new(), context: 0, next_resource: 1, instance_id };
        admit_owned_channel(&mut state).map_err(turn_fault_host)?;
        Ok(state)
    }

    /// 🧊️ `owned`'s codec origin for one call ([`OwnedCodecOriginUse`]). While another call assembles it, this one waits,
    /// relays that assembly's fuel through `progress` and observes `cancellation`; if that assembly ends without an origin,
    /// this call assembles it.
    pub(super) fn codec_origin(&self, owned: &OwnedCompiledGuest, budget: Budget, progress: &mut impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<OwnedCodecOriginUse, TurnFault> {
        let cell: &OwnedCodecOriginCell = &owned.codec_origin;
        let started = std::time::Instant::now();
        let mut relayed = 0;
        let mut state = cell.lock();
        loop {
            if original_deadline_at((self.original_clock)(),budget.deadline_us)?{return Err(TurnFault::DeadlineExceeded)}
            match &*state {
                OwnedCodecOriginState::Ready(origin) => return Ok(OwnedCodecOriginUse { origin: Arc::clone(origin), spent_fuel: 0, reported_fuel: relayed }),
                OwnedCodecOriginState::Absent => break,
                OwnedCodecOriginState::Assembling { fuel } => {
                    if *fuel > relayed {
                        relayed = *fuel;
                        progress(relayed, started.elapsed());
                    }
                    if cancellation.is_some_and(GuestCallCancellation::is_cancelled) {
                        return Err(TurnFault::Cancelled);
                    }
                    state = cell.settled.wait_timeout(state, OWNED_CODEC_ORIGIN_WAIT_POLL).unwrap_or_else(std::sync::PoisonError::into_inner).0;
                }
            }
        }
        *state = OwnedCodecOriginState::Assembling { fuel: 0 };
        drop(state);
        let mut assembly = OwnedCodecOriginAssembly { cell, origin: None };
        let origin = Arc::new(self.assemble_codec_origin(
            owned,
            budget,
            |fuel, elapsed| {
                cell.assembled_so_far(fuel);
                progress(fuel, elapsed);
            },
            cancellation,
        )?);
        assembly.origin = Some(Arc::clone(&origin));
        let fuel = origin.assembly_fuel;
        Ok(OwnedCodecOriginUse { origin, spent_fuel: fuel, reported_fuel: fuel })
    }

    /// 🏗️ Admits a fresh instance's channel ([`admit_owned_channel`]: the codec decodes channel-versioned `.spr`, envelope
    /// and native bytes), then runs `codec.pack-schema-hash` for [`OWNED_CODEC_ORIGIN_SCHEMA`] on it — every owned codec
    /// export first assembles the plugin bundle — and keeps the instance it completed on. Its answer, a refusal, is read only
    /// to know the export completed as the ABI says.
    fn assemble_codec_origin(&self, owned: &OwnedCompiledGuest, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<OwnedCodecOrigin, TurnFault> {
        let actor = owned.artifact.instantiate()?;
        if actor.startup_active() {
            return Err(TurnFault::Host(PluginHostError::Plugin("owned actor has an undriven start function".to_string())));
        }
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut state = OwnedGuestInstance { original_job_clock:None, artifact: Arc::clone(&owned.artifact), actor:Some(actor), original_core:std::mem::ManuallyDrop::new(None),original_source:std::mem::ManuallyDrop::new(None),original_startup_input:None,original_call:None, pending: None, original_receipt: None, poisoned: false, diagnostics: Vec::new(), context: 0, next_resource: 1, instance_id };
        admit_owned_channel(&mut state)?;
        let encoded = serde_json::to_vec(&OwnedCodecInput { artifact_schema: OWNED_CODEC_ORIGIN_SCHEMA, document_id: "", pack: &[], spr: &[], ops: &[] }).map_err(|error| PluginHostError::Json(error.to_string()))?;
        begin_owned_operation(&mut state, OwnedOperation::PackSchemaHash, Some(encoded))?;
        let invocation = resume_owned_operation_observed(&mut state, OwnedOperation::PackSchemaHash, budget.fuel, budget.deadline_us, OwnedDeadline::Original, progress, cancellation)?;
        match decode_owned_result::<Vec<u8>>(&invocation.output) {
            Ok(_) | Err(TurnFault::Guest(_)) => {}
            Err(fault) => return Err(fault),
        }
        Ok(OwnedCodecOrigin { actor: state.actor.take().ok_or_else(||identity_turn_fault(operation_authority::refusal(4)))?, diagnostics: std::mem::take(&mut state.diagnostics), context: state.context, next_resource: state.next_resource, assembly_fuel: invocation.fuel_used })
    }

    /// 🧬️ A throwaway instance of `owned` in its codec origin's state.
    fn origin_instance(&self, owned: &OwnedCompiledGuest, origin: &OwnedCodecOrigin) -> OwnedGuestInstance {
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        OwnedGuestInstance { original_job_clock:None, artifact: Arc::clone(&owned.artifact), actor:Some(origin.actor.clone()),original_core:std::mem::ManuallyDrop::new(None),original_source:std::mem::ManuallyDrop::new(None),original_startup_input:None,original_call:None, pending: None, original_receipt: None, poisoned: false, diagnostics: origin.diagnostics.clone(), context: origin.context, next_resource: origin.next_resource, instance_id }
    }

    /// 🧬️ A throwaway instance of `compiled` in its codec origin's state, for a caller that drives its own codec operation on
    /// it (a retained SQLite snapshot call), with the fuel this call spent assembling the origin ([`Self::codec_origin`]).
    pub fn codec_instance(&self, compiled: &OwnedCompiledGuest, budget: Budget, progress: &mut impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<(OwnedGuestInstance, u64), TurnFault> {
        let used = self.codec_origin(compiled, budget, progress, cancellation)?;
        Ok((self.origin_instance(compiled, &used.origin), used.spent_fuel))
    }

    fn actor_context_call(&self,state:&mut OwnedGuestInstance,operation:OwnedOperation,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorFixedCall,TurnFault>{
        if state.pending.is_none(){let encoded=identity.encode(|control|operation::input::encode_json_input(&(input,budget.retained_policy),control)).map_err(identity_turn_fault)?;identity.encode(|control|control.charge(encoded.len())).map_err(identity_turn_fault)?;begin_owned_operation(state,operation,Some(encoded))?;}
        let invocation=resume_owned_operation_observed(state,operation,budget.fuel,budget.deadline_us,OwnedDeadline::Original,|_,_|{},None)?;
        original_actor_context::OriginalActorFixedReceipt::read(&invocation.output).map_err(identity_turn_fault)
    }

    /// 🎬️ Runs one actor turn over `events`, or resumes the turn `state` has in flight when `events` is empty.
    pub fn execute_actor_turn(&self, state: &mut OwnedGuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        if state.pending.is_some() && !events.is_empty() {
            return Err(TurnFault::Trapped(format!("owned turn is mid-flight and cannot admit {} more event(s) — resume it with no events until it settles", events.len())));
        }
        let fresh=state.pending.is_none();
        if fresh{
        if state.original_receipt.is_some(){return Err(identity_turn_fault(operation_authority::refusal(4)));}
        let mut ordinary_events = identity.encode(|control|control.allocate_vec::<&Event>(events.len())).map_err(identity_turn_fault)?;
        let mut command_page = None;
        let mut cold_pair_page = None;
        for event in events {
            match event {
                Event::CommandIngressPage { cursor, bytes } if command_page.is_none() && bytes.len() <= semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES && (!bytes.is_empty() || (cursor.kind == 28 && cursor.item_count == 0)) => {
                    command_page = Some((cursor, bytes));
                }
                Event::CommandIngressPage { .. } => return Err(TurnFault::Trapped("turn carries more than one command page or an invalid page size".to_string())),
                Event::ColdDocumentPairPage(page) => {
                    if cold_pair_page.is_some() {
                        return Err(TurnFault::Trapped("turn carries duplicate cold-pair page".into()));
                    }
                    validate_cold_page(page)?;
                    cold_pair_page = Some(page);
                }
                event => ordinary_events.push(event),
            }
        }
        let input = identity.encode(|control|operation::input::encode_json_input(&OwnedPollInput { events: &ordinary_events, command_page, cold_pair_page, budget },control)).map_err(identity_turn_fault)?;
        identity.encode(|control|control.charge(input.len())).map_err(identity_turn_fault)?;
        begin_owned_operation(state, OwnedOperation::Poll, Some(input))?;
        }
        let mut receiver=if fresh{semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiver::new(identity)}else{let receipt=state.original_receipt.take().ok_or_else(||identity_turn_fault(operation_authority::refusal(4)))?;match semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiver::resume(receipt,identity){Ok(receiver)=>receiver,Err((error,receipt))=>{state.original_receipt=Some(receipt);return Err(identity_turn_fault(error));}}};
        let invocation = resume_owned_operation_authored(state, OwnedOperation::Poll, budget.fuel, budget.deadline_us, OwnedDeadline::Original, |_,_|{}, None, &mut receiver);
        let receipt=receiver.pause();
        state.original_receipt=if state.pending.is_some()||invocation.is_err(){Some(receipt)}else{None};
        let invocation=invocation?;
        let mut result: TurnResult = decode_owned_result(&invocation.output)?;
        result.fuel_used = invocation.fuel_used;
        Ok(result)
    }

    /// 🛂️ Executes owned `describe` on a fresh instance of `compiled`.
    pub async fn describe(&self, compiled: &OwnedCompiledGuest, budget: Budget) -> Result<Vec<u8>, TurnFault> {
        self.describe_observed(compiled, budget, |_, _| {}).await
    }

    /// 🌱️ Runs one of the component's pure `codec` functions on a throwaway instance. This is
    /// the host half of ticket 26/09/18 slice TC3b: a headless server resolves a document kind's
    /// schema fingerprint, its canonical empty document, its pair-validation mirror and its edit
    /// apply from the component itself, so a package whose Rust codec the server does not link is
    /// still fully creatable and editable. The instance is a copy of the compiled guest's codec
    /// origin ([`OwnedCodecOrigin`]), created and dropped per call — nothing here observes or mutates
    /// live actor state, and no call sees what another did. `budget.fuel` bounds the call including
    /// the origin assembly it may run first; `progress` reports both as one count.
    pub(super) fn codec_call<T: serde::de::DeserializeOwned, I: serde::Serialize>(&self, compiled: &OwnedCompiledGuest, operation: OwnedOperation, input: &I, budget: Budget, mut progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<T, TurnFault> {
        let used = self.codec_origin(compiled, budget, &mut progress, cancellation)?;
        let state = &mut self.origin_instance(compiled, &used.origin);
        let encoded = serde_json::to_vec(input).map_err(|error| PluginHostError::Json(error.to_string()))?;
        begin_owned_operation(state, operation, Some(encoded))?;
        let invocation = resume_owned_operation_observed(state, operation, budget.fuel.saturating_sub(used.spent_fuel), budget.deadline_us, OwnedDeadline::Original, |fuel, elapsed| progress(used.reported_fuel.saturating_add(fuel), elapsed), cancellation)?;
        decode_owned_result(&invocation.output)
    }

    fn codec_call_authored<T:serde::de::DeserializeOwned,I:serde::Serialize,O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self,compiled:&OwnedCompiledGuest,operation:OwnedOperation,input:&I,budget:Budget,mut progress:impl FnMut(u64,std::time::Duration),cancellation:Option<&GuestCallCancellation>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>)->Result<T,TurnFault>{
        let used=self.codec_origin(compiled,budget,&mut progress,cancellation)?;
        let state=&mut self.origin_instance(compiled,&used.origin);
        let input=identity.encode(|control|operation::input::encode_json_input(input,control)).map_err(identity_turn_fault)?;
        identity.encode(|control|control.charge(input.len())).map_err(identity_turn_fault)?;
        begin_owned_operation(state,operation,Some(input))?;
        let mut receiver=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiver::new(identity);
        let invocation=resume_owned_operation_authored(state,operation,budget.fuel.saturating_sub(used.spent_fuel),budget.deadline_us,OwnedDeadline::Original,|fuel,elapsed|progress(used.reported_fuel.saturating_add(fuel),elapsed),cancellation,&mut receiver)?;
        decode_owned_result(&invocation.output)
    }

    /// 🏛️ Resolves only the exact declared dialect's handwritten SQLite schema.
    pub async fn codec_sqlite_schema(&self, compiled: &OwnedCompiledGuest, dialect: &str, budget: Budget) -> Result<String, TurnFault> {
        self.codec_call(compiled, OwnedOperation::SqliteSchema, &OwnedSnapshotInput { dialect, encoding: "", payload: &[], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits::default().into(), native: semio_framework_value::RetainedCloneGrant::default().into() }, budget, |_, _| {}, None)
    }

    /// 📤️ Projects native values using the component's own semantic provider under observed fuel.
    pub async fn codec_sqlite_export<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &OwnedCompiledGuest, dialect: &str, encoding: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotFileResult, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::SqliteExport, &OwnedSnapshotInput { dialect, encoding, payload, limits: limits.into(), native: native_grant.into() }, budget, progress, Some(cancellation), identity)
    }

    /// 📥️ Reconstructs native values after the component validates exact dialect and domain DDL.
    pub async fn codec_sqlite_import<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &OwnedCompiledGuest, dialect: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotPayloadResult, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::SqliteImport, &OwnedSnapshotInput { dialect, encoding: "", payload, limits: limits.into(), native: native_grant.into() }, budget, progress, Some(cancellation), identity)
    }

    /// 🧬️ `codec.pack-schema-hash` — the kind's 32-byte structural snapshot fingerprint.
    pub async fn codec_pack_schema_hash(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, budget: Budget) -> Result<[u8; 32], TurnFault> {
        self.codec_pack_schema_hash_observed(compiled, artifact_schema, budget, |_, _| {}, &GuestCallCancellation::default()).await
    }

    /// 🧬️ `codec.pack-schema-hash` with the same original deadline and fuel observations as [`Self::codec_genesis_observed`].
    /// Catalog startup verifies every unlinked package through this call under
    /// `OperationContext::stall_bounded`; without guest fuel reaching that context a 30 s quiet span
    /// on a loaded machine looks like a wedged load (`trusted-catalog-load-stalled-before-it-finished`).
    pub async fn codec_pack_schema_hash_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<[u8; 32], TurnFault> {
        let bytes: Vec<u8> = self.codec_call(compiled, OwnedOperation::PackSchemaHash, &OwnedCodecInput { artifact_schema, document_id: "", pack: &[], spr: &[], ops: &[] }, budget, progress, Some(cancellation))?;
        <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| TurnFault::Trapped("guest pack schema hash is not 32 bytes".to_string()))
    }

    /// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
    pub async fn codec_genesis(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, document_id: &str, budget: Budget) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_genesis_observed(compiled, artifact_schema, document_id, budget, |_, _| {}, &GuestCallCancellation::default()).await
    }

    /// 📈️ `codec.genesis` with bounded fuel-progress observations, for a caller whose own bound is a
    /// STALL bound. A hub creates a document on a detached task under
    /// `OperationContext::stall_bounded`, and the guest call is the single longest thing that task
    /// does — without the guest's fuel progress reaching it, the hub cannot tell an interpreter that
    /// is stepping from one that is wedged, and has to pick between refusing honest work and never
    /// refusing anything. The observations are the same ones `describe_observed` reports: one per
    /// 25 M fuel or per 5 s, whichever comes first.
    pub async fn codec_genesis_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, document_id: &str, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call(compiled, OwnedOperation::Genesis, &OwnedCodecInput { artifact_schema, document_id, pack: &[], spr: &[], ops: &[] }, budget, progress, Some(cancellation))
    }

    /// 📥️ `codec.print-mirror` — the host's pair-validation fence for an unlinked package.
    pub async fn codec_print_mirror(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: Budget) -> Result<GuestDocumentMirror, TurnFault> {
        self.codec_call(compiled, OwnedOperation::PrintMirror, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops: &[] }, budget, |_, _| {}, None)
    }

    /// 📥️ `codec.print-mirror` with the same original deadline and fuel observations as [`Self::codec_genesis_observed`]:
    /// validating a large document's pair interprets for as long as a genesis does.
    pub async fn codec_print_mirror_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentMirror, TurnFault> {
        self.codec_call(compiled, OwnedOperation::PrintMirror, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops: &[] }, budget, progress, Some(cancellation))
    }

    /// 🧩️ `codec.apply-ops` — the host-authoritative edit apply for an unlinked package.
    pub async fn codec_apply_ops(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::ApplyOps, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops }, budget, |_, _| {}, None, identity)
    }

    /// 🧩️ `codec.apply-ops` with the same original deadline and fuel observations as [`Self::codec_genesis_observed`].
    pub async fn codec_apply_ops_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::ApplyOps, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops }, budget, progress, Some(cancellation), identity)
    }

    /// 📜️ `codec.replay-envelopes` — the hub's Check In fold, with the same stall-bound fuel
    /// observations as [`Self::codec_genesis_observed`]: a long ledger tail is the longest guest call
    /// a Check In makes, so its fuel progress is what its stall bound watches.
    pub async fn codec_replay_envelopes_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8], budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call(compiled, OwnedOperation::ReplayEnvelopes, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops: envelopes }, budget, progress, Some(cancellation))
    }

    /// 📈️ Executes owned `describe` with bounded fuel-progress observations for build tooling.
    pub async fn describe_observed(&self, compiled: &OwnedCompiledGuest, budget: Budget, progress: impl FnMut(u64, std::time::Duration)) -> Result<Vec<u8>, TurnFault> {
        let state = &mut self.instantiate_actor(compiled).map_err(TurnFault::Host)?;
        begin_owned_operation(state, OwnedOperation::Describe, None)?;
        resume_owned_operation_observed(state, OwnedOperation::Describe, budget.fuel, budget.deadline_us, OwnedDeadline::Original, progress, None).map(|invocation| invocation.output)
    }

    /// 🎟️ Admits the original actor context of `state` for one retained turn.
    pub fn admit_context(&self,state:&mut OwnedGuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{let call=self.actor_context_call(state,OwnedOperation::AdmitActorContext,input,budget,identity)?;match call.kind{original_actor_context::OriginalActorFixedKind::Admission{ready,refusal}=>Ok(semio_framework_actor::retained_context::OriginalActorAdmissionReply{input:call.input,spent:call.spent,work_units_spent:call.work_units_spent,ready,refusal}),_=>Err(TurnFault::ActorCall(call))}}
    /// ♻️ Runs one closing step of the original actor context of `state`.
    pub fn close_context(&self,state:&mut OwnedGuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{let call=self.actor_context_call(state,OwnedOperation::CloseActorContext,input,budget,identity)?;match call.kind{original_actor_context::OriginalActorFixedKind::Close{complete,blocked,refusal}=>Ok(semio_framework_actor::retained_context::OriginalActorCloseReply{input:call.input,spent:call.spent,work_units_spent:call.work_units_spent,complete,blocked,refusal}),_=>Err(TurnFault::ActorCall(call))}}

    /// 👷️ Starts guest job `job` of `kind` on `state`.
    pub fn start_actor_job(&self, state: &mut OwnedGuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        let input = semio_framework_pack_json::to_json_string(&OwnedStartJobInput { job, kind, input }).into_bytes();
        begin_owned_operation(state, OwnedOperation::StartJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::StartJob, u64::MAX, 1_000_000)?;
        decode_owned_result(&invocation.output)
    }

    /// 🪜️ Steps guest job `job` of `state` once under `budget`.
    pub fn step_actor_job(&self, state: &mut OwnedGuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        let input = semio_framework_pack_json::to_json_string(&OwnedStepJobInput { job, budget }).into_bytes();
        begin_owned_operation(state, OwnedOperation::StepJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::StepJob, budget.fuel, u64::from(budget.deadline_ms)*1000)?;
        decode_owned_result(&invocation.output)
    }

    /// 🛑️ Cancels guest job `job` of `state`, abandoning whatever owned operation is in flight first.
    pub fn cancel_actor_job(&self, state: &mut OwnedGuestInstance, job: u64) -> Result<(), TurnFault> {
        if state.pending.as_ref().is_some_and(|pending| pending.operation != OwnedOperation::CancelJob) {
            cancel_owned_operation(state)?;
        }
        let input = semio_framework_pack_json::to_json_string(&OwnedCancelJobInput { job }).into_bytes();
        begin_owned_operation(state, OwnedOperation::CancelJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::CancelJob, u64::MAX, u64::from(OWNED_SLICE_DEADLINE_MS)*1000)?;
        decode_owned_result(&invocation.output)
    }

    /// 📸️ Frames `state` — its in-flight operation, original receipt, guest checkpoint and interpreter memory — as one owned
    /// host checkpoint.
    pub fn checkpoint_actor(&self, state: &mut OwnedGuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let guest_checkpoint = if state.pending.is_none() {
            begin_owned_operation(state, OwnedOperation::Checkpoint, None).map_err(turn_fault_host)?;
            let invocation = match resume_owned_operation(state, OwnedOperation::Checkpoint, 100_000_000, 1_000_000) {
                Ok(invocation) => invocation,
                Err(fault) => {
                    cancel_owned_operation(state).map_err(turn_fault_host)?;
                    return Err(turn_fault_host(fault));
                }
            };
            Some(decode_owned_result::<Vec<u8>>(&invocation.output).map_err(turn_fault_host)?)
        } else {
            None
        };
        let metadata = OwnedCheckpointView { pending: &state.pending, original_receipt:&state.original_receipt, guest_checkpoint:&guest_checkpoint, context: state.context, next_resource: state.next_resource, instance_id: state.instance_id };
        identity.encode(|control|{
            let layout=state.actor.as_mut().ok_or_else(||operation_authority::refusal(4))?.checkpoint_layout(control)?;
            operation::checkpoint::encode_checkpoint_frame(&metadata,&mut|sink|layout.visit(sink),control)
        }).map_err(identity_turn_fault).map_err(turn_fault_host)
    }

    /// ⏮️ Restores `state` from an owned host checkpoint [`Self::checkpoint_actor`] framed.
    pub fn restore_actor(&self, state: &mut OwnedGuestInstance, checkpoint: &[u8]) -> Result<(), PluginHostError> {
        if state.original_core.is_some(){return Err(PluginHostError::NativeIo(operation_authority::refusal(4)))}
        if checkpoint.get(..8) != Some(OWNED_CHECKPOINT_MAGIC) {
            return Err(PluginHostError::Plugin("invalid owned host checkpoint header".to_string()));
        }
        let metadata_length =
            checkpoint.get(8..12).and_then(|bytes| bytes.try_into().ok()).map(u32::from_le_bytes).map(|length| length as usize).ok_or_else(|| PluginHostError::Plugin("truncated owned host checkpoint metadata length".to_string()))?;
        let actor_length =
            checkpoint.get(12..20).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).and_then(|length| usize::try_from(length).ok()).ok_or_else(|| PluginHostError::Plugin("invalid owned host checkpoint actor length".to_string()))?;
        let metadata_end = 20usize.checked_add(metadata_length).ok_or_else(|| PluginHostError::Plugin("owned host checkpoint metadata length overflow".to_string()))?;
        let actor_end = metadata_end.checked_add(actor_length).ok_or_else(|| PluginHostError::Plugin("owned host checkpoint actor length overflow".to_string()))?;
        if actor_end != checkpoint.len() {
            return Err(PluginHostError::Plugin("owned host checkpoint length mismatch".to_string()));
        }
        let metadata_text = std::str::from_utf8(&checkpoint[20..metadata_end]).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let metadata: OwnedCheckpointMetadata = semio_framework_pack_json::from_json_str(metadata_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        state.actor = Some(state.artifact.restore(&checkpoint[metadata_end..]).map_err(|error| PluginHostError::Plugin(error.to_string()))?);
        state.pending = metadata.pending;
        state.original_receipt=metadata.original_receipt;
        state.context = metadata.context;
        state.next_resource = metadata.next_resource;
        state.instance_id = metadata.instance_id;
        if let Some(guest_checkpoint) = metadata.guest_checkpoint {
            let input = semio_framework_pack_json::to_json_string(&OwnedRestoreInput { state: guest_checkpoint }).into_bytes();
            begin_owned_operation(state, OwnedOperation::Restore, Some(input)).map_err(turn_fault_host)?;
            let invocation = resume_owned_operation(state, OwnedOperation::Restore, 100_000_000, 1_000_000).map_err(turn_fault_host)?;
            decode_owned_result::<()>(&invocation.output).map_err(turn_fault_host)?;
        }
        Ok(())
    }
}
//#endregion 🧠️OwnedRuntime
