//! 🏠️ Artifact document-store and publication authorities.

use crate::op::RasterMutation;
use crate::{RasterAssetChild, RasterLayerNode, RasterOwnedMap, RasterOwnedMapInsert, RasterSnapshot};
use protocol::{Mutation, OpBinary};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
fn decode_raster_snapshot_pack(bytes: &[u8]) -> Result<RasterSnapshot, ()> {
    <RasterSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| ())
}

fn decode_raster_mutation_pack(bytes: &[u8]) -> Result<RasterMutation, ()> {
    RasterMutation::decode_op(bytes).map_err(|_| ())
}

macro_rules! raster_owned_field_close_capacity {
    (ArtifactEnvelopeSnapshotFieldAuthority) => {
        fn maximum_close_byte_demand(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }

        fn maximum_retained_close_bytes(&self) -> usize {
            store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
        }
    };
    (ArtifactEnvelopeMutationFieldAuthority) => {};
}

macro_rules! raster_owned_field_authority {
    ($state:ident, $authority:ident, $value:ty, $authority_trait:ident, $target_trait:ident, $publish:ident, $decode:path, $kind:literal) => {
        #[expect(clippy::large_enum_variant, reason = "The active decoder keeps its fixed path and admitted hex authority inline without a second allocation at the state transition.")]
        enum $state {
            AwaitToken,
            Decode(store::OwnedSchemaHexAuthority<RASTER_OWNED_FIELD_BYTES>),
            Ready,
            Published,
            Closing,
            Complete,
        }

        struct $authority {
            operation: semio_framework_job::OperationId,
            generation: semio_framework_job::Generation,
            path: store::OwnedSchemaPath,
            state: $state,
            value: std::mem::ManuallyDrop<Option<$value>>,
            retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
        }

        impl $authority {
            fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
                Self { operation, generation, path, state: $state::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
            }

            fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
                store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path, refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
            }

            fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::OwnedSchemaDecodeDiagnostic> {
                if matches!(self.state, $state::Decode(_)) {
                    return Ok(semio_framework_value::RetirementDemand { release_bytes: RASTER_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
                }
                let demand = match self.retirement.as_ref() {
                    Some(owner) => store::artifact_retirement_box_demands(owner, maximum_copy_bytes),
                    None => store::artifact_retirement_owned_birth_demands(&self.value),
                };
                demand.map_err(|_| self.diagnostic(concat!("raster-envelope.", $kind, "-retirement-demand"), 0))
            }
        }

        impl store::$authority_trait<$value> for $authority {
            fn accept_token(
                &mut self,
                token: store::OwnedSchemaToken,
                terminal: bool,
                source: &store::OwnedSchemaRecordCursor,
                cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                if cx.operation() != self.operation || cx.generation() != self.generation {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-stale-authority"), token.start));
                }
                if cx.is_cancelled() {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-cancelled"), token.start));
                }
                if cx.should_yield() {
                    return Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending);
                }
                let path = self.path;
                let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path, refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
                if matches!(self.state, $state::AwaitToken) {
                    if !terminal {
                        return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-must-be-scalar"), token.start));
                    }
                    self.state = $state::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
                }
                let $state::Decode(authority) = &mut self.state else {
                    return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-token-replayed"), token.start));
                };
                match authority.step(source, cx) {
                    store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
                    store::OwnedSchemaHexStep::Complete => {
                        let bytes = authority.as_bytes().ok_or_else(|| diagnostic(concat!("raster-envelope.", $kind, "-pack-missing"), token.start))?;
                        let value = $decode(bytes).map_err(|_| diagnostic(concat!("raster-envelope.", $kind, "-pack-malformed"), token.start))?;
                        if !authority.release() {
                            return Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-release-duplicate"), token.start));
                        }
                        *self.value = Some(value);
                        self.state = $state::Ready;
                        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    store::OwnedSchemaHexStep::Cancelled => Err(diagnostic(concat!("raster-envelope.", $kind, "-pack-cancelled"), token.start)),
                    store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
                }
            }

            fn publish_reserved(
                &mut self,
                target: &mut dyn store::$target_trait<$value>,
                reservation: store::ArtifactEnvelopeFieldReservation,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
                if !matches!(self.state, $state::Ready) {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-pack-not-ready"), 0));
                }
                let value = self.value.take().ok_or_else(|| self.diagnostic(concat!("raster-envelope.", $kind, "-owner-missing"), 0))?;
                target.$publish(reservation, value);
                self.state = $state::Published;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }

            raster_owned_field_close_capacity!($authority_trait);

            fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(0).map(|demand| demand.copy_bytes)
            }

            fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes)
            }

            fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(0).map(|demand| demand.release_bytes)
            }

            fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
                self.close_demands(0).map(|demand| demand.depth)
            }

            fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
                let empty = RetainedCloneProgress::default();
                if self.terminal_is_empty() {
                    return Ok(RetainedCloneStep::Complete(empty));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let demand = self.close_demands(grant.maximum_copy_bytes)?;
                if grant.maximum_depth < demand.depth {
                    return Err(self.diagnostic(concat!("raster-envelope.", $kind, "-retirement-depth"), 0));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                if let $state::Decode(authority) = &mut self.state {
                    authority.cancel();
                    self.state = $state::Closing;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: RASTER_OWNED_FIELD_BYTES, ..empty }));
                }
                let attempt = if self.retirement.is_some() {
                    store::artifact_retirement_box_close_step(&mut self.retirement, grant)
                } else if self.value.is_some() {
                    store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant)
                } else {
                    self.state = $state::Complete;
                    return Ok(RetainedCloneStep::Complete(empty));
                };
                let step = attempt.map_err(|error| store::OwnedSchemaDecodeDiagnostic { code: concat!("raster-envelope.", $kind, "-retirement-fault"), offset: 0, line: 0, column: 0, path: self.path, refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: error.retained_progress() })?;
                self.state = $state::Closing;
                if self.value.is_none() && self.retirement.is_none() {
                    self.state = $state::Complete;
                    return Ok(RetainedCloneStep::Complete(step.progress()));
                }
                Ok(RetainedCloneStep::Progress(step.progress()))
            }

            fn terminal_is_empty(&self) -> bool {
                matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()
            }
        }

        impl Drop for $authority {
            fn drop(&mut self) {
                assert!(
                    (matches!(self.state, $state::Published | $state::Complete) && self.value.is_none() && self.retirement.is_none()) || std::thread::panicking(),
                    concat!("Raster ", $kind, " decode reached Drop before publication or bounded retirement"),
                );
            }
        }
    };
}

raster_owned_field_authority!(
    RasterSnapshotDecodeState,
    RasterSnapshotDecodeAuthority,
    RasterSnapshot,
    ArtifactEnvelopeSnapshotFieldAuthority,
    ArtifactEnvelopeSnapshotFieldTarget,
    publish_snapshot_reserved,
    decode_raster_snapshot_pack,
    "snapshot"
);

raster_owned_field_authority!(
    RasterMutationDecodeState,
    RasterMutationDecodeAuthority,
    RasterMutation,
    ArtifactEnvelopeMutationFieldAuthority,
    ArtifactEnvelopeMutationFieldTarget,
    publish_mutation_reserved,
    decode_raster_mutation_pack,
    "mutation"
);

pub type RasterDocumentStoreOwners = store::DocumentStoreOwners<RasterSnapshot, RasterMutation>;

pub type RasterOwnersAdmission = Result<(RasterDocumentStoreOwners, RetainedCloneProgress), store::DocumentStoreOwnersAdmissionError<RasterSnapshot, RasterMutation>>;

pub fn raster_document_store_owners_admission(grant: RetainedCloneGrant) -> RasterOwnersAdmission {
    RasterDocumentStoreOwners::admit_source_constructor(grant, || (RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer::<RasterSnapshot, RasterMutation>::new()))
}

/// 📏️ Quotes the source Arcs and disposer the Raster owner catalog admits before any factory ticket is born.
pub fn raster_document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, ValueError> {
    let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>()?;
    Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
}

pub fn raster_document_store_owners() -> RasterDocumentStoreOwners {
    let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>().expect("Raster owner catalog source layout");
    store::fund_document_store_owners(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: 1 }, raster_document_store_owners_admission).expect("funded Raster owner catalog")
}

struct RasterStoreInitializationAuthority {
    actor: std::mem::ManuallyDrop<Option<protocol::ActorId>>,
    actor_close: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::ActorId>>>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<RasterSnapshot, RasterMutation>>>,
    owners: std::mem::ManuallyDrop<Option<RasterDocumentStoreOwners>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<RasterSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<RasterSnapshot, RasterMutation>>>,
    active: RasterRetirementSlot,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<RasterSnapshotCloneAuthority>>,
    mutation_candidate: std::mem::ManuallyDrop<Option<RasterMutationCandidateAuthority>>,
    publication: semio_framework_job::RetainedJobPublication,
    delivered: bool,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    phase: RasterStoreInitializationPhase,
    resume_phase: Option<RasterStoreInitializationPhase>,
    cancel_requested: bool,
    fault: Option<Vec<u8>>,
    terminal_handoff: bool,
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<RasterSnapshot, RasterMutation> for RasterStoreInitializationAuthority {
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        self.close_demands(maximum_copy_bytes)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, ValueError> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"raster-store.initializer-stale-authority");
        }
        if (self.cancel_requested || cx.is_cancelled()) && !matches!(self.phase, RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::Cancelled) {
            self.phase = RasterStoreInitializationPhase::RetireCancelled;
        }
        if cx.should_yield() {
            return Ok(None);
        }
        match self.pump_active(cx) {
            Ok(true) => return Ok(None),
            Ok(false) => {}
            Err(error) => {
                self.fault = Some(error.into_message().into_bytes());
                self.phase = RasterStoreInitializationPhase::RetireFault;
            }
        }
        if !matches!(self.phase, RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::RetireFault | RasterStoreInitializationPhase::Cancelled | RasterStoreInitializationPhase::Fault | RasterStoreInitializationPhase::Complete) {
            if let Some(runtime) = self.runtime.as_mut() {
                match runtime.settle_current_retirement_step(cx.retained_grant()) {
                    Ok(RetainedCloneStep::Complete(_)) => {}
                    Ok(RetainedCloneStep::Progress(progress)) => {
                        if let Err(error) = cx.consume_retained(progress) {
                            self.fault = Some(error.into_message().into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        }
                        cx.consume_fuel(1);
                        return Ok(None);
                    }
                    Err(error) => { self.fault = Some(error.into_message().into_bytes()); self.phase = RasterStoreInitializationPhase::RetireFault; }
                }
            }
        }
        match self.phase {
            RasterStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                let actor = self.actor.as_ref().expect("retained initializer actor").clone();
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), actor));
                self.phase = RasterStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"raster-store.initializer-envelope-missing");
                    return Ok(None);
                };
                if envelope.schema != crate::RASTER_DOCUMENT_SCHEMA || envelope.id.is_empty() || envelope.id.len() > RASTER_OWNED_FIELD_BYTES {
                    self.fail(b"raster-store.initializer-envelope-invalid");
                } else {
                    self.phase = RasterStoreInitializationPhase::ValidateEdit { index: 0 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Raster envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, RASTER_OWNED_FIELD_BYTES) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = RasterStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = RasterStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized => self.fail(b"raster-store.initializer-hostile-edit-id"),
                    store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"raster-store.initializer-duplicate-edit"),
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Raster envelope remains retained during initial clone").vcs.genesis.facts().snapshot();
                let clone = self.clone.as_mut().expect("Raster initial clone authority remains retained");
                let complete = match clone.step(source, cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return Ok(None);
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Raster initial snapshot was built one semantic item at a time");
                    drop(self.clone.take());
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, std::sync::Arc::new(RasterSnapshotRetirementFactory)) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            self.active.put(RasterDisplaced::Snapshot(initial));
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                }
                Ok(None)
            }
            RasterStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = RasterStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return Ok(None);
                };
                let runtime = self.runtime.as_mut().expect("Raster runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fault = Some(error.into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fault = Some(error.into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = RasterStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = RasterStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Raster runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = RasterStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = RasterStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fault = Some(error.into_bytes());
                        self.phase = RasterStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Raster runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = RasterStoreInitializationPhase::FindRedo { position: 0 };
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"raster-store.initializer-applied-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"raster-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    *self.clone = Some(RasterSnapshotCloneAuthority::new());
                    self.phase = RasterStoreInitializationPhase::CloneInitial;
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                if self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).is_none_or(|entry| mutation >= entry.forwards.len()) {
                    self.phase = RasterStoreInitializationPhase::CommitApplied { position, edit };
                    return Ok(None);
                }
                let withdrawn = {
                    let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while its forwards fold");
                    envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).is_none_or(|effective| effective.operation().is_none())
                };
                if withdrawn {
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                if self.mutation_candidate.is_none() {
                    if !raster_reserve_unit(cx) {
                        return Ok(None);
                    }
                    *self.mutation_candidate = Some(RasterMutationCandidateAuthority::new());
                    return Ok(None);
                }
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained while its forwards fold");
                let effective = envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).expect("Raster applied forward remains retained");
                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Raster runtime current snapshot remains retained");
                let stepped = self.mutation_candidate.as_mut().expect("Raster mutation candidate remains retained").step(current, effective.operation().expect("Raster effective forward was checked"), cx);
                drop(effective);
                let candidate_complete = match stepped {
                    Ok(value) => value,
                    Err(code) => {
                        self.fail(code.as_bytes());
                        return Ok(None);
                    }
                };
                if candidate_complete {
                    let next = self.mutation_candidate.as_mut().expect("Raster completed mutation candidate remains retained").take().expect("Raster mutation candidate handoff");
                    drop(self.mutation_candidate.take());
                    let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("Raster runtime current snapshot remains retained");
                    let previous = std::mem::replace(current, next);
                    self.active.put(RasterDisplaced::Snapshot(previous));
                    self.phase = RasterStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                }
                Ok(None)
            }
            RasterStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster applied edit remains retained");

                let runtime = self.runtime.as_mut().expect("Raster runtime remains retained");
                if let Err(error) = runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = RasterStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = RasterStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = RasterStoreInitializationPhase::RetireActor;
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Raster envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"raster-store.initializer-redo-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = RasterStoreInitializationPhase::CommitRedo { position, edit: scan };
                } else {
                    self.fail(b"raster-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Raster redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Raster runtime remains retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fault = Some(error.into_bytes());
                    self.phase = RasterStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = RasterStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::RetireActor => {
                if self.actor.is_none() && self.actor_close.is_none() {
                    self.phase = RasterStoreInitializationPhase::BuildCandidate;
                    return Ok(None);
                }
                match self.step_actor(cx.retained_grant()) {
                    Ok(step) => {
                        if let Err(error) = cx.consume_retained(step.progress()) {
                            self.fault = Some(error.into_message().into_bytes());
                            self.phase = RasterStoreInitializationPhase::RetireFault;
                        }
                    }
                    Err(error) => {
                        self.fault = Some(error.into_message().into_bytes());
                        self.phase = RasterStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            RasterStoreInitializationPhase::BuildCandidate => {
                if !raster_reserve_unit(cx) {
                    return Ok(None);
                }
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"raster-store.initializer-generation-exhausted");
                    return Ok(None);
                };
                let envelope = self.envelope.take().expect("Raster envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Raster runtime remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, raster_document_store_owners());
                *self.candidate = Some(candidate);
                self.phase = RasterStoreInitializationPhase::Complete;
                semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)
            }
            RasterStoreInitializationPhase::RetireCancelled | RasterStoreInitializationPhase::RetireFault => match self.pump_terminal_retirement(cx) {
                Ok(false) => Ok(None),
                Ok(true) => {
                    self.terminal_handoff = true;
                    if self.phase == RasterStoreInitializationPhase::RetireCancelled {
                        self.phase = RasterStoreInitializationPhase::Cancelled;
                        semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx)
                    } else {
                        self.phase = RasterStoreInitializationPhase::Fault;
                        Ok(None)
                    }
                }
                Err(error) => {
                    self.fault = Some(error.into_message().into_bytes());
                    Ok(None)
                }
            },
            RasterStoreInitializationPhase::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            RasterStoreInitializationPhase::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            RasterStoreInitializationPhase::Fault => {
                if self.delivered {
                    let step = self.publication.close_step(cx.retained_grant())?;
                    cx.consume_retained(step.progress())?;
                    if matches!(step, RetainedCloneStep::Complete(_)) {
                        self.delivered = false;
                    }
                    return Ok(None);
                }
                let result = self.publication.advance_from_source(semio_framework_job::JobPublicationKind::Fault, self.fault.as_deref().unwrap_or(b"raster-store.initializer-fault"), cx)?;
                if result.is_some() {
                    self.delivered = true;
                }
                Ok(result)
            }
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete if self.phase == RasterStoreInitializationPhase::Complete => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, RasterStoreInitializationPhase::Cancelled | RasterStoreInitializationPhase::Fault) {
            self.phase = RasterStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.begin_close();
        let step = self.close_original(grant)?;
        if self.close_is_exhausted() {
            self.terminal_handoff = true;
            return Ok(RetainedCloneStep::Complete(step.progress()));
        }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<RasterSnapshot, RasterMutation>> {
        if self.phase != RasterStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

const RASTER_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterSnapshotRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<RasterSnapshot> for RasterSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &RasterSnapshot) -> usize {
        semio_framework_value::retirement::owned_retirement_birth_bytes::<RasterSnapshot>()
    }

    fn retire_owned(&self, value: RasterSnapshot, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, RasterSnapshot)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

impl store::SnapshotRetirementFactory<RasterSnapshot> for RasterSnapshotRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<RasterSnapshot>) -> usize {
        semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<RasterSnapshot>()
    }

    fn retire(&self, snapshot: std::sync::Arc<RasterSnapshot>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, std::sync::Arc<RasterSnapshot>)> {
        semio_framework_value::retirement::shared::admit_shared_retirement(snapshot, grant, true)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterMutationRetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<RasterMutation> for RasterMutationRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &RasterMutation) -> usize {
        semio_framework_value::retirement::owned_retirement_birth_bytes::<RasterMutation>()
    }

    fn retire_owned(&self, value: RasterMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, RasterMutation)> {
        semio_framework_value::retirement::admit_owned_retirement(value, grant)
    }
}

/// ♻️ Every value a stepwise Raster authority displaces or abandons; the framework's owned retirement closes it under an explicit grant.
#[derive(semio_framework_value::RetireOwned)]
enum RasterDisplaced {
    String(String),
    Layer(RasterLayerNode),
    Snapshot(RasterSnapshot),
    AssetEntry((String, RasterAssetChild)),
    ValueEntry((String, semio_framework_value::DslValue)),
    Value(semio_framework_value::DslValue),
}

/// 🗑️ One displaced owner and its admitted retirement frame; the owner stays in `pending` until a grant funds its frame.
struct RasterRetirementSlot {
    pending: std::mem::ManuallyDrop<Option<RasterDisplaced>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl RasterRetirementSlot {
    fn new() -> Self {
        Self { pending: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(None) }
    }

    fn put(&mut self, owner: RasterDisplaced) {
        assert!(self.is_empty(), "Raster retirement slot admits one displaced owner at a time");
        *self.pending = Some(owner);
    }

    fn is_empty(&self) -> bool {
        self.pending.is_none() && self.active.is_none()
    }

    fn birth_demand() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<RasterDisplaced>(), depth: 2, ..Default::default() }
    }

    fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return store::artifact_retirement_box_demands(active, body);
        }
        if self.pending.is_some() {
            return Ok(Self::birth_demand());
        }
        Ok(Default::default())
    }

    fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Raster displaced owner exceeds admitted depth"));
        }
        if raster_short(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let step = if self.active.is_some() { store::artifact_retirement_box_close_step(&mut self.active, grant)? } else { store::artifact_retirement_admit_owned(&mut self.pending, &mut self.active, grant)? };
        Ok(RetainedCloneStep::Progress(step.progress()))
    }
}

impl Drop for RasterRetirementSlot {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.is_empty(), "Raster retirement slot reached Drop before its displaced owner retired");
        if self.is_empty() {
            unsafe {
                std::mem::ManuallyDrop::drop(&mut self.pending);
                std::mem::ManuallyDrop::drop(&mut self.active);
            }
        }
    }
}

fn raster_short(grant: RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes
}

fn raster_finish(step: RetainedCloneStep, terminal: bool) -> RetainedCloneStep {
    match step {
        RetainedCloneStep::Complete(progress) | RetainedCloneStep::Progress(progress) if terminal => RetainedCloneStep::Complete(progress),
        RetainedCloneStep::Complete(progress) | RetainedCloneStep::Progress(progress) => RetainedCloneStep::Progress(progress),
    }
}

fn raster_nested(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "Raster nested retirement depth overflow"))?;
    Ok(demand)
}

fn raster_child_grant(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}

fn raster_reserve_unit(cx: &mut semio_framework_job::StepContext<'_>) -> bool {
    if cx.should_yield() || cx.fuel_remaining() == 0 {
        return false;
    }
    cx.consume_fuel(1);
    true
}

struct RasterSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    retirement: RasterRetirementSlot,
    layer: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    pending_asset: std::mem::ManuallyDrop<Option<(String, RasterAssetChild)>>,
    bounds: RasterSnapshotBoundsAuthority,
    asset_key: RasterMapKeyCursor,
    phase: u8,
    index: usize,
    asset_field: u8,
    terminal: bool,
}

impl RasterSnapshotCloneAuthority {
    fn new() -> Self {
        let value = RasterSnapshot { schema: String::new(), id: String::new(), title: None, layers: Vec::new(), assets: RasterOwnedMap::new() };
        Self {
            value: std::mem::ManuallyDrop::new(Some(value)),
            retirement: RasterRetirementSlot::new(),
            layer: std::mem::ManuallyDrop::new(None),
            pending_asset: std::mem::ManuallyDrop::new(None),
            bounds: RasterSnapshotBoundsAuthority::new(),
            asset_key: RasterMapKeyCursor::new(),
            phase: 0,
            index: 0,
            asset_field: 0,
            terminal: false,
        }
    }

    fn step(&mut self, source: &RasterSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.phase == 0 {
            if self.bounds.step(source, cx)? {
                self.phase = 1;
            }
            return Ok(false);
        }
        let target = self.value.as_mut().ok_or("raster-store.initializer-clone-target")?;
        match self.phase {
            1 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.schema = raster_clone_owned_string(&source.schema)?;
            }
            2 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.id = raster_clone_owned_string(&source.id)?;
            }
            3 => {
                if let Some(title) = source.title.as_ref() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.title = Some(raster_clone_owned_string(title)?);
                } else {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                }
            }
            4 => {
                if self.index == 0 && target.layers.capacity() == 0 {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.layers.try_reserve_exact(source.layers.capacity().saturating_add(1)).map_err(|_| "raster-store.initializer-layer-admission")?;
                    self.bounds.totals.observe_candidate_capacity(source.layers.capacity().saturating_add(1), target.layers.capacity(), size_of::<RasterLayerNode>())?;
                    return Ok(false);
                }
                if let Some(layer) = self.layer.as_mut() {
                    if layer.step(source.layers.get(self.index).ok_or("raster-store.initializer-layer-source")?, cx)? {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.layers.push(layer.take().ok_or("raster-store.initializer-layer-handoff")?);
                        drop(self.layer.take());
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if let Some(layer) = source.layers.get(self.index) {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.layer = Some(Box::new(RasterLayerCloneAuthority::new(layer)));
                    return Ok(false);
                }
                self.index = 0;
            }
            5 => {
                if self.pending_asset.is_none() {
                    let Some((key, _)) = self.asset_key.next(&source.assets) else {
                        self.phase = 6;
                        return Ok(false);
                    };
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    let key = raster_clone_owned_string(key)?;
                    let child =
                        store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                    *self.pending_asset = Some((key, child));
                    self.asset_field = 0;
                    return Ok(false);
                }
                if self.asset_field >= 5 {
                    let (key, _) = self.pending_asset.as_ref().expect("Raster pending asset remains retained");
                    if target.assets.page_required_for_insert(key) {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.assets.admit_one_page()?;
                        return Ok(false);
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    let (key, mut child) = self.pending_asset.take().expect("Raster pending asset handoff remains exact");
                    // 🪆 A clone rebuilds every handle string into exact-capacity storage, so the
                    // source child's materialization has to be carried across explicitly — a cloned
                    // snapshot whose assets lost their pixels renders an empty composite.
                    if let Some(source_child) = source.assets.get(&key) {
                        crate::adopt_raster_asset_owner(source_child, &mut child);
                    }
                    match target.assets.insert_pre_admitted(key, child) {
                        Ok(RasterOwnedMapInsert::Inserted) => {}
                        Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                            self.retirement.put(RasterDisplaced::AssetEntry(previous.take()));
                            return Err("raster-store.initializer-duplicate-asset");
                        }
                        Err(rejected) => {
                            self.retirement.put(RasterDisplaced::AssetEntry((rejected.key, rejected.value)));
                            return Err(rejected.reason);
                        }
                    }
                    self.asset_key.advance("")?;
                    self.asset_field = 0;
                    return Ok(false);
                }
                let (key, pending) = self.pending_asset.as_mut().expect("Raster pending asset remains exact");
                let source_child = source.assets.get(key).ok_or("raster-store.initializer-asset-source")?;
                let source_value = match self.asset_field {
                    0 => &source_child.child_id,
                    1 => &source_child.target.artifact_id,
                    2 => &source_child.target.dialect.artifact_kind,
                    3 => &source_child.target.dialect.standard,
                    4 => &source_child.target.dialect.subset,
                    _ => unreachable!("Raster asset field cursor is exact"),
                };
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                let target_value = match self.asset_field {
                    0 => &mut pending.child_id,
                    1 => &mut pending.target.artifact_id,
                    2 => &mut pending.target.dialect.artifact_kind,
                    3 => &mut pending.target.dialect.standard,
                    4 => &mut pending.target.dialect.subset,
                    _ => unreachable!("Raster asset field cursor is exact"),
                };
                *target_value = raster_clone_owned_string(source_value)?;
                self.asset_field += 1;
                return Ok(false);
            }
            _ => {
                self.terminal = true;
                return Ok(true);
            }
        }
        self.phase += 1;
        Ok(false)
    }

    fn take_value(&mut self) -> Option<RasterSnapshot> {
        if !self.terminal {
            return None;
        }
        self.value.take()
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(layer) = self.layer.as_ref() {
            return raster_nested(layer.close_demands(body)?);
        }
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if self.pending_asset.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
        }
        Ok(Default::default())
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(layer) = self.layer.as_mut() {
            let demand = raster_nested(layer.close_demands(grant.maximum_copy_bytes)?)?;
            if grant.maximum_depth < demand.depth {
                return Err(invariant("Raster snapshot clone layer exceeds admitted depth"));
            }
            if raster_short(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let step = layer.close_step(raster_child_grant(grant))?;
            if layer.terminal_is_empty() {
                drop(self.layer.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_empty() {
            if let Some(entry) = self.pending_asset.take() {
                self.retirement.put(RasterDisplaced::AssetEntry(entry));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Snapshot(value));
            }
        }
        let step = self.retirement.close(grant)?;
        Ok(raster_finish(step, self.terminal_is_empty()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_empty() && self.layer.is_none() && self.pending_asset.is_none()
    }
}

impl Drop for RasterSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

struct RasterMutationCandidateAuthority {
    value: std::mem::ManuallyDrop<Option<RasterSnapshot>>,
    clone: std::mem::ManuallyDrop<Option<Box<RasterSnapshotCloneAuthority>>>,
    layer_clone: std::mem::ManuallyDrop<Option<Box<RasterLayerCloneAuthority>>>,
    pending_layer: std::mem::ManuallyDrop<Option<RasterLayerNode>>,
    pending_asset: std::mem::ManuallyDrop<Option<(String, RasterAssetChild)>>,
    retirement: RasterRetirementSlot,
    /// 🌉️ The canonical child this apply minted from the payload's real bytes, held between the
    /// mint step and the handoff that moves its materialization onto the pending handle.
    asset_mint: Option<RasterAssetChild>,
    asset_field: u8,
    locator: Option<RasterLayerLocator>,
    primary: Option<RasterLayerAddress>,
    secondary: Option<RasterLayerAddress>,
    container: Option<RasterLayerAddress>,
    shift_index: usize,
    shift_target: usize,
    phase: RasterMutationCandidatePhase,
    terminal: bool,
}

impl RasterMutationCandidateAuthority {
    fn new() -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            layer_clone: std::mem::ManuallyDrop::new(None),
            pending_layer: std::mem::ManuallyDrop::new(None),
            pending_asset: std::mem::ManuallyDrop::new(None),
            retirement: RasterRetirementSlot::new(),
            asset_mint: None,
            asset_field: 0,
            locator: None,
            primary: None,
            secondary: None,
            container: None,
            shift_index: 0,
            shift_target: 0,
            phase: RasterMutationCandidatePhase::Clone,
            terminal: false,
        }
    }

    fn target(operation: &RasterMutation) -> Option<&str> {
        match operation {
            RasterMutation::CreateLayer(value) => Some(RasterLayerLocator::node_id(&value.layer)),
            RasterMutation::DeleteLayer(value) => Some(&value.layer_id),
            RasterMutation::ReorderLayers(value) => Some(&value.layer_id),
            RasterMutation::RenameLayer(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerLocked(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerVisible(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerOpacity(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerBlendMode(value) => Some(&value.layer_id),
            RasterMutation::MoveLayer(value) => Some(&value.layer_id),
            RasterMutation::ResizeLayer(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerPixels(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerMask(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerTransform(value)=>Some(&value.layer_id),
            RasterMutation::ChangeLayerAdjustmentParameter(value) => Some(&value.layer_id),
            RasterMutation::ChangeLayerAdjustmentKind(value) => Some(&value.layer_id),
            RasterMutation::PaintStroke(value) => Some(&value.layer_id),
            RasterMutation::FillRegion(value) => Some(&value.layer_id),
            RasterMutation::ApplyFilter(value) => Some(&value.layer_id),
            RasterMutation::TransformImage(value) => Some(&value.layer_id),
            RasterMutation::FillSelection(value) => Some(&value.layer_id),
            RasterMutation::WritePixelRegion(value) => Some(&value.layer_id),
            RasterMutation::AddLayerAsset(_) | RasterMutation::RemoveLayerAsset(_) => None,
        }
    }

    fn parent(operation: &RasterMutation) -> Option<&str> {
        match operation {
            RasterMutation::CreateLayer(value) => value.parent_id.as_deref(),
            RasterMutation::ReorderLayers(value) => value.parent_id.as_deref(),
            _ => None,
        }
    }

    fn prepare_insert(&mut self, parent: Option<RasterLayerAddress>, index: usize) {
        self.container = parent;
        self.shift_target = index;
        self.phase = RasterMutationCandidatePhase::BeginInsert;
    }

    fn begin_insert(&mut self) -> Result<(), &'static str> {
        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-insert-snapshot")?;
        let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-insert-parent")?;
        if self.shift_target > values.len() || values.len() >= values.capacity() {
            return Err("raster-store.mutation-insert-capacity");
        }
        values.push(self.pending_layer.take().ok_or("raster-store.mutation-insert-owner")?);
        self.shift_index = values.len() - 1;
        self.phase = RasterMutationCandidatePhase::ShiftInsert;
        Ok(())
    }

    fn begin_remove(&mut self, address: RasterLayerAddress) {
        self.container = address.parent();
        self.shift_index = address.index();
        self.phase = RasterMutationCandidatePhase::ShiftRemove;
    }

    fn replace_string(target: &mut String, source: &String) -> Result<RasterDisplaced, &'static str> {
        let replacement = raster_clone_owned_string(source)?;
        let previous = std::mem::replace(target, replacement);
        Ok(RasterDisplaced::String(previous))
    }

    fn exact_string(value: &str) -> Result<String, &'static str> {
        raster_exact_string_from_parts(&[value.as_bytes()])
    }

    fn pump_retirement(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.retirement.is_empty() {
            return Ok(false);
        }
        if cx.should_yield() {
            return Ok(true);
        }
        let step = self.retirement.close(cx.retained_grant()).map_err(|_| "raster-store.mutation-retirement-fault")?;
        cx.consume_retained(step.progress()).map_err(|_| "raster-store.mutation-retirement-receipt")?;
        cx.consume_fuel(1);
        Ok(true)
    }

    fn step(&mut self, current: &RasterSnapshot, operation: &RasterMutation, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.pump_retirement(cx)? {
            return Ok(false);
        }
        match self.phase {
            RasterMutationCandidatePhase::Clone => {
                if self.clone.is_none() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.clone = Some(Box::new(RasterSnapshotCloneAuthority::new()));
                    return Ok(false);
                }
                let clone = self.clone.as_mut().ok_or("raster-store.mutation-clone")?;
                if !clone.step(current, cx)? {
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.value = clone.take_value();
                drop(self.clone.take());
                if Self::target(operation).is_some() {
                    self.locator = Some(RasterLayerLocator::new());
                    self.phase = RasterMutationCandidatePhase::LocatePrimary;
                } else {
                    self.phase = RasterMutationCandidatePhase::Apply;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::LocatePrimary => {
                let snapshot = self.value.as_ref().ok_or("raster-store.mutation-locate-snapshot")?;
                let locator = self.locator.as_mut().ok_or("raster-store.mutation-locator")?;
                if !locator.step(snapshot, Self::target(operation).ok_or("raster-store.mutation-target")?, cx)? {
                    return Ok(false);
                }
                self.primary = locator.found;
                self.locator = None;
                if matches!(operation, RasterMutation::CreateLayer(_)) {
                    if self.primary.is_some() {
                        return Err("raster-store.mutation-duplicate-layer");
                    }
                    if Self::parent(operation).is_some() {
                        self.locator = Some(RasterLayerLocator::new());
                        self.phase = RasterMutationCandidatePhase::LocateSecondary;
                    } else {
                        self.phase = RasterMutationCandidatePhase::PrepareLayer;
                    }
                } else if self.primary.is_none() {
                    return Err("raster-store.mutation-target-missing");
                } else {
                    self.phase = RasterMutationCandidatePhase::Apply;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::LocateSecondary | RasterMutationCandidatePhase::LocateDestination => {
                let snapshot = self.value.as_ref().ok_or("raster-store.mutation-parent-snapshot")?;
                let locator = self.locator.as_mut().ok_or("raster-store.mutation-parent-locator")?;
                if !locator.step(snapshot, Self::parent(operation).ok_or("raster-store.mutation-parent")?, cx)? {
                    return Ok(false);
                }
                self.secondary = locator.found;
                self.locator = None;
                let parent = self.secondary.ok_or("raster-store.mutation-parent-missing")?;
                if !matches!(RasterLayerLocator::node_at(snapshot, parent), Some(RasterLayerNode::Group { .. })) {
                    return Err("raster-store.mutation-parent-not-group");
                }
                if self.phase == RasterMutationCandidatePhase::LocateSecondary {
                    self.phase = RasterMutationCandidatePhase::PrepareLayer;
                } else {
                    let index = match operation {
                        RasterMutation::ReorderLayers(value) => value.index,
                        _ => return Err("raster-store.mutation-destination-variant"),
                    };
                    self.prepare_insert(Some(parent), index);
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::PrepareLayer => {
                let RasterMutation::CreateLayer(value) = operation else { return Err("raster-store.mutation-prepare-variant") };
                if self.layer_clone.is_none() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.layer_clone = Some(Box::new(RasterLayerCloneAuthority::new(&value.layer)));
                    return Ok(false);
                }
                let clone = self.layer_clone.as_mut().expect("Raster create layer clone remains retained");
                if !clone.step(&value.layer, cx)? {
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.pending_layer = clone.take();
                drop(self.layer_clone.take());
                self.phase = RasterMutationCandidatePhase::Apply;
                Ok(false)
            }
            RasterMutationCandidatePhase::Apply => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-apply-snapshot")?;
                match operation {
                    RasterMutation::CreateLayer(value) => {
                        self.prepare_insert(self.secondary, value.index);
                        return Ok(false);
                    }
                    RasterMutation::DeleteLayer(_) | RasterMutation::ReorderLayers(_) => {
                        self.begin_remove(self.primary.ok_or("raster-store.mutation-remove-address")?);
                        return Ok(false);
                    }
                    RasterMutation::RenameLayer(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { name, .. } | RasterLayerNode::Group { name, .. } | RasterLayerNode::Adjustment { name, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        self.retirement.put(Self::replace_string(name, &value.new_name)?);
                    }
                    RasterMutation::ChangeLayerLocked(value)=>{
                        if !raster_reserve_unit(cx){return Ok(false);}
                        crate::mutations::change_layer_locked::validate(value,snapshot).map_err(|code| code.as_str())?;
                        let (RasterLayerNode::Pixel {locked,..}|RasterLayerNode::Group {locked,..}|RasterLayerNode::Adjustment {locked,..})=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *locked=value.locked;
                    }
                    RasterMutation::ChangeLayerVisible(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { visible, .. } | RasterLayerNode::Group { visible, .. } | RasterLayerNode::Adjustment { visible, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *visible = value.new_visible;
                    }
                    RasterMutation::ChangeLayerOpacity(value) => {
                        if !value.new_opacity.is_finite() || !raster_reserve_unit(cx) {
                            if !value.new_opacity.is_finite() {
                                return Err("raster-store.mutation-opacity-invalid");
                            }
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { opacity, .. } | RasterLayerNode::Group { opacity, .. } | RasterLayerNode::Adjustment { opacity, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        *opacity = value.new_opacity;
                    }
                    RasterMutation::ChangeLayerBlendMode(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (RasterLayerNode::Pixel { blend_mode, .. } | RasterLayerNode::Group { blend_mode, .. } | RasterLayerNode::Adjustment { blend_mode, .. }) =
                            RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")?;
                        self.retirement.put(Self::replace_string(blend_mode, &value.new_blend_mode)?);
                    }
                    RasterMutation::MoveLayer(value) => {
                        if !value.new_x.is_finite() || !value.new_y.is_finite() || !raster_reserve_unit(cx) {
                            if !value.new_x.is_finite() || !value.new_y.is_finite() {
                                return Err("raster-store.mutation-transform-invalid");
                            }
                            return Ok(false);
                        }
                        match RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? {
                            RasterLayerNode::Pixel { transform, .. } | RasterLayerNode::Group { transform, .. } => {
                                transform.x = value.new_x;
                                transform.y = value.new_y;
                            }
                            RasterLayerNode::Adjustment { .. } => return Err("raster-store.mutation-transform-target"),
                        }
                    }
                    RasterMutation::ResizeLayer(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let RasterLayerNode::Pixel { width, height, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                            return Err("raster-store.mutation-resize-target");
                        };
                        *width = Some(value.new_width);
                        *height = Some(value.new_height);
                    }
                    RasterMutation::ChangeLayerAdjustmentParameter(value) => {
                        crate::mutations::change_layer_adjustment_parameter::validate(value,snapshot).map_err(|code| code.as_str())?;
                        let RasterLayerNode::Adjustment {params,..}=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {return Err("raster-store.mutation-adjustment-target");};
                        if value.value.is_some() && params.page_required_for_insert(&value.parameter) {
                            if !raster_reserve_unit(cx) {return Ok(false);}
                            params.admit_one_page()?;return Ok(false);
                        }
                        if !raster_reserve_unit(cx) {return Ok(false);}
                        let replacement=value.value.map(|v|Ok::<_,&'static str>((raster_clone_owned_string(&value.parameter)?,v.literal()))).transpose()?;
                        if let Some(mut entry)=params.remove_entry(&value.parameter) {
                            let (key,old)=entry.take();
                            self.retirement.put(RasterDisplaced::ValueEntry((key,old)));
                        }
                        if let Some((key,number))=replacement {params.insert_pre_admitted(key,number).map_err(|error|error.reason)?;}
                    }
                    RasterMutation::ChangeLayerTransform(value)=>{
                        if !raster_reserve_unit(cx){return Ok(false);}
                        crate::mutations::change_layer_transform::validate(value,snapshot).map_err(|code| code.as_str())?;
                        let (RasterLayerNode::Pixel {transform,..}|RasterLayerNode::Group {transform,..})=RasterLayerLocator::node_at_mut(snapshot,self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {return Err("raster-store.mutation-transform-target");};
                        *transform=value.transform.clone();
                    }
                    RasterMutation::ChangeLayerMask(value) => {
                        if !raster_reserve_unit(cx) { return Ok(false); }
                        crate::mutations::change_layer_mask::validate(value, snapshot).map_err(|code| code.as_str())?;
                        let replacement = value.mask.as_ref().map(|mask| {
                            Ok::<_, &'static str>(crate::RasterLayerMask {
                                enabled: mask.enabled, linked: mask.linked, invert: mask.invert,
                                width: mask.width, height: mask.height, transform: mask.transform.clone(),
                                image_key: mask.image_key.as_ref().map(raster_clone_owned_string).transpose()?,
                            })
                        }).transpose()?;
                        let (RasterLayerNode::Pixel { mask, .. } | RasterLayerNode::Group { mask, .. }) = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else { return Err("raster-store.mutation-mask-target"); };
                        if let Some(previous) = std::mem::replace(mask, replacement).and_then(|mask| mask.image_key) {
                            self.retirement.put(RasterDisplaced::String(previous));
                        }
                    }
                    RasterMutation::ChangeLayerPixels(value) => {
                        if !raster_reserve_unit(cx) { return Ok(false); }
                        crate::mutations::change_layer_pixels::validate(value, snapshot).map_err(|code| code.as_str())?;
                        let replacement = value.content.image_key.as_ref().map(raster_clone_owned_string).transpose()?;
                        let RasterLayerNode::Pixel { image_key, width, height, transform, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else { return Err("raster-store.mutation-pixels-target"); };
                        if let Some(previous) = std::mem::replace(image_key, replacement) {
                            self.retirement.put(RasterDisplaced::String(previous));
                        }
                        *width = value.content.width;
                        *height = value.content.height;
                        if let Some(next) = &value.transform { *transform = next.clone(); }
                    }
                    RasterMutation::ChangeLayerAdjustmentKind(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let RasterLayerNode::Adjustment { adjustment_kind, .. } = RasterLayerLocator::node_at_mut(snapshot, self.primary.ok_or("raster-store.mutation-address")?).ok_or("raster-store.mutation-target-lost")? else {
                            return Err("raster-store.mutation-adjustment-target");
                        };
                        self.retirement.put(Self::replace_string(adjustment_kind, &value.new_adjustment_kind)?);
                    }
                    RasterMutation::AddLayerAsset(value) => {
                        if value.asset.schema.capacity() > RASTER_OWNED_FIELD_BYTES || value.asset.frames.iter().map(|frame| frame.rgba8.len()).sum::<usize>() > RASTER_MAXIMUM_NESTED_BYTES || value.asset.icc.as_ref().is_some_and(|icc| icc.len() > RASTER_MAXIMUM_NESTED_BYTES) || value.asset.metadata.iter().any(|entry| entry.key.capacity().max(entry.value.capacity()) > RASTER_OWNED_FIELD_BYTES) {
                            return Err("raster-store.mutation-asset-capacity");
                        }
                        self.asset_field = 0;
                        self.phase = RasterMutationCandidatePhase::PrepareAsset;
                        return Ok(false);
                    }
                    RasterMutation::RemoveLayerAsset(value) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let mut removed = snapshot.assets.remove_entry(&value.asset_id).ok_or("raster-store.mutation-asset-missing")?;
                        let (key, child) = removed.take();
                        self.retirement.put(RasterDisplaced::AssetEntry((key, child)));
                    }
                    RasterMutation::PaintStroke(_) | RasterMutation::FillRegion(_) | RasterMutation::ApplyFilter(_) | RasterMutation::TransformImage(_) | RasterMutation::FillSelection(_) | RasterMutation::WritePixelRegion(_) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let outcome = protocol::Mutation::diff(operation, &*snapshot);
                        let refused = outcome.messages().iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
                        let (diff, _) = outcome.into_parts();
                        let painted = if refused { Err("raster-store.mutation-paint-refused") } else { protocol::apply_diff(&diff, snapshot).map_err(|_| "raster-store.mutation-paint-apply") };
                        protocol::MutationDiff::retire_cold(diff);
                        let previous = std::mem::replace(snapshot, painted?);
                        self.retirement.put(RasterDisplaced::Snapshot(previous));
                    }
                }
                self.phase = RasterMutationCandidatePhase::Drain;
                Ok(false)
            }
            RasterMutationCandidatePhase::PrepareAsset => {
                let RasterMutation::AddLayerAsset(value) = operation else { return Err("raster-store.mutation-asset-variant") };
                match self.asset_field {
                    0 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        // 🌉️ The SAME funnel the native apply uses (`🔺️diff/📝️text`, `io`'s import
                        // bridges): the payload's real bytes are decoded into the composed child's own
                        // content, which both mints the CANONICAL content-addressed child id (a raw
                        // `(mime, data)` digest minted here would disagree with every other route and
                        // break `add-layer-asset`'s inverse) and produces the materialization this
                        // handle must retain — without it the asset pool holds pixel-less handles and
                        // the composite surface renders nothing.
                        self.asset_mint = Some(crate::mint_raster_image_child(&value.asset_id, &value.asset));
                    }
                    1 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let key = raster_clone_owned_string(&value.asset_id)?;
                        let child =
                            store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
                        *self.pending_asset = Some((key, child));
                    }
                    2 => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let child_id = raster_clone_owned_string(&self.asset_mint.as_ref().ok_or("raster-store.mutation-asset-mint")?.child_id)?;
                        self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1.child_id = child_id;
                    }
                    3 => {
                        let length = value.asset_id.len().checked_add(6).ok_or("raster-store.mutation-asset-id-overflow")?;
                        if length > RASTER_OWNED_FIELD_BYTES || !raster_reserve_unit(cx) {
                            if length > RASTER_OWNED_FIELD_BYTES {
                                return Err("raster-store.mutation-asset-id-capacity");
                            }
                            return Ok(false);
                        }
                        let artifact_id = raster_exact_string_from_parts(&[value.asset_id.as_bytes(), b"-image"])?;
                        self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1.target.artifact_id = artifact_id;
                    }
                    4..=6 => {
                        let literal = match self.asset_field {
                            4 => "s.stdio.semio",
                            5 => "v1",
                            _ => "image",
                        };
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let child = &mut self.pending_asset.as_mut().ok_or("raster-store.mutation-asset-owner")?.1;
                        let target = match self.asset_field {
                            4 => &mut child.target.dialect.artifact_kind,
                            5 => &mut child.target.dialect.standard,
                            _ => &mut child.target.dialect.subset,
                        };
                        *target = Self::exact_string(literal)?;
                    }
                    _ => {
                        let snapshot = self.value.as_mut().ok_or("raster-store.mutation-asset-snapshot")?;
                        let pending_key = &self.pending_asset.as_ref().ok_or("raster-store.mutation-asset-owner")?.0;
                        if snapshot.assets.page_required_for_insert(pending_key) {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            snapshot.assets.admit_one_page()?;
                            return Ok(false);
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        let (key, mut child) = self.pending_asset.take().ok_or("raster-store.mutation-asset-owner")?;
                        crate::adopt_raster_asset_owner(&self.asset_mint.take().ok_or("raster-store.mutation-asset-mint")?, &mut child);
                        if let Some(slot) = snapshot.assets.get_mut(&value.asset_id) {
                            let previous = std::mem::replace(slot, child);
                            self.retirement.put(RasterDisplaced::AssetEntry((key, previous)));
                        } else {
                            match snapshot.assets.insert_pre_admitted(key, child) {
                                Ok(RasterOwnedMapInsert::Inserted) => {}
                                Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                                    self.retirement.put(RasterDisplaced::AssetEntry(previous.take()));
                                    return Err("raster-store.mutation-duplicate-asset");
                                }
                                Err(rejected) => {
                                    self.retirement.put(RasterDisplaced::AssetEntry((rejected.key, rejected.value)));
                                    return Err(rejected.reason);
                                }
                            }
                        }
                        self.phase = RasterMutationCandidatePhase::Drain;
                        return Ok(false);
                    }
                }
                self.asset_field += 1;
                Ok(false)
            }
            RasterMutationCandidatePhase::ShiftRemove => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
                let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
                if self.shift_index + 1 < values.len() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    values.swap(self.shift_index, self.shift_index + 1);
                    self.shift_index += 1;
                    return Ok(false);
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                *self.pending_layer = values.pop();
                if matches!(operation, RasterMutation::DeleteLayer(_)) {
                    let layer = self.pending_layer.take().ok_or("raster-store.mutation-delete-owner")?;
                    self.retirement.put(RasterDisplaced::Layer(layer));
                    self.phase = RasterMutationCandidatePhase::Drain;
                } else if let RasterMutation::ReorderLayers(value) = operation {
                    self.secondary = None;
                    if value.parent_id.is_some() {
                        self.locator = Some(RasterLayerLocator::new());
                        self.phase = RasterMutationCandidatePhase::LocateDestination;
                    } else {
                        self.prepare_insert(None, value.index);
                    }
                } else {
                    return Err("raster-store.mutation-remove-variant");
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::BeginInsert => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.begin_insert()?;
                Ok(false)
            }
            RasterMutationCandidatePhase::ShiftInsert => {
                let snapshot = self.value.as_mut().ok_or("raster-store.mutation-shift-snapshot")?;
                let values = RasterLayerLocator::container_mut(snapshot, self.container).ok_or("raster-store.mutation-shift-parent")?;
                if self.shift_index > self.shift_target {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    values.swap(self.shift_index, self.shift_index - 1);
                    self.shift_index -= 1;
                    return Ok(false);
                }
                self.phase = RasterMutationCandidatePhase::Drain;
                Ok(false)
            }
            RasterMutationCandidatePhase::Drain => {
                if self.retirement.is_empty() {
                    self.phase = RasterMutationCandidatePhase::Complete;
                }
                Ok(false)
            }
            RasterMutationCandidatePhase::Complete => {
                self.terminal = true;
                Ok(true)
            }
            RasterMutationCandidatePhase::Closing => Ok(false),
        }
    }

    fn take(&mut self) -> Option<RasterSnapshot> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if let Some(clone) = self.layer_clone.as_ref() {
            return raster_nested(clone.close_demands(body)?);
        }
        if let Some(clone) = self.clone.as_ref() {
            return raster_nested(clone.close_demands(body)?);
        }
        if self.pending_layer.is_some() || self.pending_asset.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
        }
        if self.asset_mint.is_some() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<RasterAssetChild>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        self.phase = RasterMutationCandidatePhase::Closing;
        if self.retirement.is_empty() {
            if let Some(clone) = self.layer_clone.as_mut() {
                let demand = raster_nested(clone.close_demands(grant.maximum_copy_bytes)?)?;
                if grant.maximum_depth < demand.depth {
                    return Err(invariant("Raster candidate layer clone exceeds admitted depth"));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let step = clone.close_step(raster_child_grant(grant))?;
                if clone.terminal_is_empty() {
                    drop(self.layer_clone.take());
                }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(clone) = self.clone.as_mut() {
                let demand = raster_nested(clone.close_demands(grant.maximum_copy_bytes)?)?;
                if grant.maximum_depth < demand.depth {
                    return Err(invariant("Raster candidate snapshot clone exceeds admitted depth"));
                }
                if raster_short(grant, demand) {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                let step = clone.close_step(raster_child_grant(grant))?;
                if clone.terminal_is_empty() {
                    drop(self.clone.take());
                }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            if self.asset_mint.is_some() {
                if grant.maximum_copy_bytes < size_of::<Option<RasterAssetChild>>() {
                    return Ok(RetainedCloneStep::Progress(empty));
                }
                drop(self.asset_mint.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<RasterAssetChild>>(), ..empty }));
            }
            if let Some(layer) = self.pending_layer.take() {
                self.retirement.put(RasterDisplaced::Layer(layer));
            } else if let Some(entry) = self.pending_asset.take() {
                self.retirement.put(RasterDisplaced::AssetEntry(entry));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Snapshot(value));
            }
        }
        let step = self.retirement.close(grant)?;
        self.terminal = self.terminal_is_empty();
        Ok(raster_finish(step, self.terminal))
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.clone.is_none() && self.layer_clone.is_none() && self.pending_layer.is_none() && self.pending_asset.is_none() && self.retirement.is_empty() && self.asset_mint.is_none()
    }
}

impl Drop for RasterMutationCandidateAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster mutation candidate reached Drop before exact handoff or retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    RetireActor,
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

impl RasterStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<RasterSnapshot, RasterMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        Self {
            actor: std::mem::ManuallyDrop::new(Some(actor)),
            actor_close: std::mem::ManuallyDrop::new(None),
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            owners: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            active: RasterRetirementSlot::new(),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            mutation_candidate: std::mem::ManuallyDrop::new(None),
            publication: semio_framework_job::RetainedJobPublication::new(),
            delivered: false,
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: RasterStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: None,
            terminal_handoff: false,
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = RasterStoreInitializationPhase::RetireFault;
    }

    fn pump_active(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        if self.active.is_empty() {
            return Ok(false);
        }
        let step = self.active.close(cx.retained_grant())?;
        cx.consume_retained(step.progress())?;
        cx.consume_fuel(1);
        Ok(true)
    }

    fn actor_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(actor) = self.actor_close.as_ref() {
            if actor.terminal_is_empty() {
                return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() });
            }
            return raster_nested(semio_framework_value::RetirementDemand { copy_bytes: actor.next_copy_byte_demand()?, capacity_bytes: actor.next_capacity_byte_demand(body)?, release_bytes: actor.next_release_byte_demand()?, depth: actor.next_depth_demand()? });
        }
        if self.actor.is_some() {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<protocol::ActorId>>() + size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    fn step_actor(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.actor.is_none() && self.actor_close.is_none() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.actor_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Raster initializer actor exceeds admitted depth"));
        }
        if raster_short(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = raster_child_grant(grant);
        if let Some(actor) = self.actor_close.as_mut() {
            if actor.terminal_is_empty() {
                drop(self.actor_close.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = actor.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, actor.terminal_is_empty(), "Raster initializer original actor")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let actor = self.actor.take().expect("Raster initializer actor remains retained until its retirement frame");
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(actor) {
            Ok(owner) => {
                *self.actor_close = Some(owner);
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
            }
            Err((error, actor)) => {
                *self.actor = Some(actor);
                Err(error)
            }
        }
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if self.delivered {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if !self.active.is_empty() {
            return self.active.demands(body);
        }
        if let Some(candidate) = self.candidate.as_ref() {
            return semio_framework_plugin::ArtifactDocumentStoreDisposer::<RasterSnapshot, RasterMutation>::new().retirement_demands(candidate, body);
        }
        if let Some(candidate) = self.mutation_candidate.as_ref() {
            return candidate.close_demands(body);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return runtime.initialization_retirement_demands(body);
        }
        if let Some(clone) = self.clone.as_ref() {
            return clone.close_demands(body);
        }
        if let Some(retirement) = self.envelope_retirement.as_ref() {
            return store::artifact_retirement_box_demands(retirement, body);
        }
        if let Some(envelope) = self.envelope.as_ref() {
            let Some(owners) = self.owners.as_ref() else {
                let capacity_bytes = RasterDocumentStoreOwners::source_birth_bytes::<RasterSnapshotRetirementFactory, RasterSnapshotRetirementFactory, RasterMutationRetirementFactory, store::ArtifactStoreCursorDisposer<RasterSnapshot, RasterMutation>>()?;
                return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<RasterDocumentStoreOwners>>(), capacity_bytes, depth: 1, ..Default::default() });
            };
            if !owners.constructor_is_complete() {
                return raster_nested(owners.constructor_demands(body)?);
            }
            let mut demand = raster_nested(owners.uninstalled_envelope_retirement_demands(envelope))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "Raster initializer envelope frame copy overflow"))?;
            return Ok(demand);
        }
        if let Some(owners) = self.owners.as_ref() {
            if owners.uninstalled_owners_terminal_is_empty() {
                return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Option<RasterDocumentStoreOwners>>(), depth: 1, ..Default::default() });
            }
            return raster_nested(owners.uninstalled_owners_demands(body)?);
        }
        self.actor_demands(body)
    }

    fn close_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_plugin::ArtifactOwnedDisposer;
        let empty = RetainedCloneProgress::default();
        if self.close_is_exhausted() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "Raster initializer close exceeds admitted depth"));
        }
        if raster_short(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = raster_child_grant(grant);
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.delivered {
            self.delivered = false;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if !self.active.is_empty() {
            return self.active.close(grant);
        }
        if let Some(candidate) = self.candidate.as_mut() {
            let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<RasterSnapshot, RasterMutation>::new();
            let step = disposer.close_step(candidate, grant).map_err(|fault| ValueError::new(ValueRefusalKind::InvariantViolated, format!("{}: {}", fault.code.0, fault.message)))?;
            if disposer.terminal_is_empty(candidate) {
                drop(self.candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()));
        }
        if let Some(candidate) = self.mutation_candidate.as_mut() {
            let step = candidate.close_step(grant)?;
            if candidate.terminal_is_empty() {
                drop(self.mutation_candidate.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let factory: std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<RasterSnapshot>> = std::sync::Arc::new(RasterSnapshotRetirementFactory);
            let step = runtime.close_step(&factory, grant)?;
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(clone) = self.clone.as_mut() {
            let step = clone.close_step(grant)?;
            if clone.terminal_is_empty() {
                drop(self.clone.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            let step = store::artifact_retirement_box_close_step(&mut self.envelope_retirement, grant)?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let funded = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match raster_document_store_owners_admission(funded) {
                    Ok((owners, mut receipt)) => {
                        *self.owners = Some(owners);
                        receipt.copied_bytes += placement;
                        Ok(RetainedCloneStep::Progress(receipt))
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        Err(refused.error.with_retained_progress(refused.progress))
                    }
                };
            }
            let owners = self.owners.as_mut().expect("Raster initializer catalog remains retained");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            let placement = size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();
            let funded = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("Raster initializer catalog remains retained");
            let envelope = self.envelope.take().expect("Raster initializer envelope remains retained");
            return match owners.retire_envelope_uninstalled(envelope, funded) {
                Ok((owner, mut receipt)) => {
                    *self.envelope_retirement = Some(owner);
                    receipt.copied_bytes += placement;
                    Ok(RetainedCloneStep::Progress(receipt))
                }
                Err((error, owners, envelope)) => {
                    *self.owners = Some(owners);
                    *self.envelope = Some(envelope);
                    Err(error)
                }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            return semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "Raster initializer original catalog").map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        self.step_actor(grant)
    }

    fn close_is_exhausted(&self) -> bool {
        self.publication.terminal_is_empty() && !self.delivered && self.active.is_empty() && self.candidate.is_none() && self.mutation_candidate.is_none() && self.runtime.is_none() && self.clone.is_none() && self.envelope_retirement.is_none() && self.envelope.is_none() && self.owners.is_none() && self.actor.is_none() && self.actor_close.is_none()
    }

    fn pump_terminal_retirement(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        if self.close_is_exhausted() {
            return Ok(true);
        }
        let step = self.close_original(cx.retained_grant())?;
        cx.consume_retained(step.progress())?;
        cx.consume_fuel(1);
        Ok(self.close_is_exhausted())
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.close_is_exhausted()
    }
}

impl Drop for RasterStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty_inner()) || std::thread::panicking(), "Raster store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn raster_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<RasterSnapshot, RasterMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<RasterSnapshot, RasterMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(RasterStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}

/// 🧹️ Closes one cold-owned Raster value to terminal-empty through grants sized exactly by the retirement's own quotes.
pub fn retire_raster_value_cold<T: semio_framework_value::retirement::RetireOwned>(value: T) {
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), maximum_depth: 2, ..Default::default() };
    match semio_framework_value::retirement::admit_owned_retirement(value, birth) {
        Ok((owner, _)) => close_raster_owner_cold(owner),
        Err((error, _value)) => panic!("Raster cold retirement refused its frame: {error}"),
    }
}

/// 🧹️ Drives one admitted retirement frame to terminal-empty through grants sized exactly by its own quotes.
pub fn close_raster_owner_cold(mut owner: Box<dyn store::ErasedSnapshotRetirement>) {
    let mut turns = 0usize;
    while !owner.terminal_is_empty() {
        let demand = owner.next_demand(RASTER_OWNED_FIELD_BYTES).expect("Raster cold retirement quotes its next turn");
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RASTER_OWNED_FIELD_BYTES.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        owner.close_step(grant).expect("Raster cold retirement closes within its quoted grant");
        turns += 1;
        assert!(turns < 1 << 24, "Raster cold retirement made no progress");
    }
}

/// 🧯️ A retirement or clone cursor found its own bookkeeping inconsistent.
fn invariant(message: &str) -> ValueError {
    ValueError::new(ValueRefusalKind::InvariantViolated, message)
}

const RASTER_MAXIMUM_NESTED_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

impl store::ArtifactEnvelopeOwnedFieldCatalog<RasterSnapshot, RasterMutation> for RasterEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<RasterSnapshot, RasterMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<RasterSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(RasterSnapshotRetirementFactory), std::sync::Arc::new(RasterMutationRetirementFactory), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<RasterSnapshot, RasterMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<RasterSnapshot>> {
        Box::new(RasterSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<RasterMutation>> {
        Box::new(RasterMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(RasterRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<RasterMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(RasterMutationRetirementFactory))
    }
}

pub fn raster_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<RasterSnapshot, RasterMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(RasterEnvelopeOwnedFieldCatalog), std::sync::Arc::new(RasterSnapshotRetirementFactory), std::sync::Arc::new(RasterMutationRetirementFactory))
}

impl RasterOwnerTotals {
    fn new() -> Self {
        Self { source_items: 0, source_bytes: 0, candidate_items: 0, candidate_bytes: 0 }
    }

    fn add(&mut self, items: usize, bytes: usize, candidate_items: usize, candidate_bytes: usize) -> Result<(), &'static str> {
        self.source_items = self.source_items.checked_add(items).ok_or("raster-store.preflight-source-item-overflow")?;
        self.source_bytes = self.source_bytes.checked_add(bytes).ok_or("raster-store.preflight-source-byte-overflow")?;
        self.candidate_items = self.candidate_items.checked_add(candidate_items).ok_or("raster-store.preflight-candidate-item-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(candidate_bytes).ok_or("raster-store.preflight-candidate-byte-overflow")?;
        if self.source_items > RASTER_MAXIMUM_NESTED_ITEMS || self.candidate_items > RASTER_MAXIMUM_NESTED_ITEMS {
            return Err("raster-store.preflight-item-capacity");
        }
        if self.source_bytes > RASTER_MAXIMUM_NESTED_BYTES || self.candidate_bytes > RASTER_MAXIMUM_NESTED_BYTES {
            return Err("raster-store.preflight-byte-capacity");
        }
        Ok(())
    }

    fn string(&mut self, value: &String) -> Result<(), &'static str> {
        if value.capacity() > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-string-allocation-capacity");
        }
        self.add(1, value.capacity(), 1, value.len())
    }

    fn vector<T>(&mut self, value: &Vec<T>) -> Result<(), &'static str> {
        let bytes = value.capacity().checked_mul(size_of::<T>()).ok_or("raster-store.preflight-vector-byte-overflow")?;
        if bytes > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-vector-allocation-capacity");
        }
        self.add(1, bytes, 1, bytes)
    }

    fn layer_vector(&mut self, value: &Vec<RasterLayerNode>) -> Result<(), &'static str> {
        let source_bytes = value.capacity().checked_mul(size_of::<RasterLayerNode>()).ok_or("raster-store.preflight-layer-vector-byte-overflow")?;
        let candidate_capacity = value.capacity().checked_add(1).ok_or("raster-store.preflight-layer-vector-capacity-overflow")?;
        let candidate_bytes = candidate_capacity.checked_mul(size_of::<RasterLayerNode>()).ok_or("raster-store.preflight-layer-candidate-byte-overflow")?;
        if source_bytes > RASTER_OWNED_FIELD_BYTES || candidate_bytes > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.preflight-layer-vector-allocation-capacity");
        }
        self.add(1, source_bytes, 1, candidate_bytes)
    }

    fn map<V>(&mut self, value: &RasterOwnedMap<V>, candidate_extra_entries: usize) -> Result<(), &'static str> {
        let source_pages = value.allocated_page_count();
        let candidate_entries = value.len().checked_add(candidate_extra_entries).ok_or("raster-store.preflight-map-entry-overflow")?;
        if candidate_entries > crate::RASTER_OWNED_MAP_CAPACITY {
            return Err("raster-store.preflight-map-item-capacity");
        }
        let candidate_pages = candidate_entries.div_ceil(crate::RASTER_OWNED_MAP_PAGE_CAPACITY);
        let page_bytes = RasterOwnedMap::<V>::conservative_page_credit_bytes();
        self.add(source_pages, source_pages.checked_mul(page_bytes).ok_or("raster-store.preflight-map-source-byte-overflow")?, candidate_pages, candidate_pages.checked_mul(page_bytes).ok_or("raster-store.preflight-map-candidate-byte-overflow")?)
    }

    fn observe_candidate_capacity(&mut self, requested: usize, observed: usize, element_bytes: usize) -> Result<(), &'static str> {
        if observed < requested {
            return Err("raster-store.candidate-capacity-underflow");
        }
        let extra = observed.checked_sub(requested).and_then(|value| value.checked_mul(element_bytes)).ok_or("raster-store.candidate-capacity-overflow")?;
        self.candidate_bytes = self.candidate_bytes.checked_add(extra).ok_or("raster-store.candidate-byte-overflow")?;
        if self.candidate_bytes > RASTER_MAXIMUM_NESTED_BYTES || extra > RASTER_OWNED_FIELD_BYTES {
            return Err("raster-store.candidate-observed-capacity");
        }
        Ok(())
    }
}

struct RasterMapKeyCursor {
    index: usize,
}

impl RasterMapKeyCursor {
    fn new() -> Self {
        Self { index: 0 }
    }

    fn next<'a, T>(&self, values: &'a RasterOwnedMap<T>) -> Option<(&'a String, &'a T)> {
        values.entry_at(self.index)
    }

    fn advance(&mut self, _key: &str) -> Result<(), &'static str> {
        self.index = self.index.checked_add(1).ok_or("raster-store.map-index-overflow")?;
        Ok(())
    }
}

impl RasterDslValueBoundsAuthority {
    fn new(layer_depth: usize) -> Self {
        Self { layer_depth, depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], terminal: false }
    }

    fn value_at<'a>(root: &'a semio_framework_value::DslValue, path: &[usize]) -> Option<&'a semio_framework_value::DslValue> {
        let mut value = root;
        for index in path {
            value = match value {
                semio_framework_value::DslValue::Array(values) => values.get(*index)?,
                semio_framework_value::DslValue::Object(values) => &values.get(*index)?.1,
                _ => return None,
            };
        }
        Some(value)
    }

    fn step(&mut self, root: &semio_framework_value::DslValue, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let _required_frames = raster_combined_depth_requirement(self.layer_depth, self.depth + 1)?;
        let value = Self::value_at(root, &self.path[..self.depth]).ok_or("raster-store.preflight-value-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            totals.add(1, size_of::<semio_framework_value::DslValue>(), 1, size_of::<semio_framework_value::DslValue>())?;
            match value {
                semio_framework_value::DslValue::String(value) => totals.string(value)?,
                semio_framework_value::DslValue::Bytes(value) => totals.vector(value)?,
                semio_framework_value::DslValue::Array(values) => totals.vector(values)?,
                semio_framework_value::DslValue::Object(values) => totals.vector(values)?,
                _ => {}
            }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        let child = frame.child;
        match value {
            semio_framework_value::DslValue::Object(values) if frame.phase == 1 && child < values.len() => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.string(&values[child].0)?;
                self.frames[self.depth].phase = 2;
                return Ok(false);
            }
            _ if match value {
                semio_framework_value::DslValue::Array(values) => child < values.len(),
                semio_framework_value::DslValue::Object(values) => frame.phase == 2 && child < values.len(),
                _ => false,
            } =>
            {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH || !raster_reserve_unit(cx) {
                    if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                        return Err("raster-store.preflight-value-depth");
                    }
                    return Ok(false);
                }
                self.path[self.depth] = child;
                self.frames[self.depth].child += 1;
                self.frames[self.depth].phase = 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            _ => {}
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.terminal = true;
        } else {
            self.depth -= 1;
        }
        Ok(self.terminal)
    }
}

impl RasterLayerBoundsAuthority {
    fn new() -> Self {
        Self { depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], parameter_key: RasterMapKeyCursor::new(), parameter_value: None, terminal: false }
    }

    fn layer_at<'a>(root: &'a RasterLayerNode, path: &[usize]) -> Option<&'a RasterLayerNode> {
        let mut value = root;
        for index in path {
            let RasterLayerNode::Group { children, .. } = value else { return None };
            value = children.get(*index)?;
        }
        Some(value)
    }

    fn strings(layer: &RasterLayerNode) -> [&String; 3] {
        match layer {
            RasterLayerNode::Pixel { id, name, blend_mode, .. } | RasterLayerNode::Group { id, name, blend_mode, .. } | RasterLayerNode::Adjustment { id, name, blend_mode, .. } => [id, name, blend_mode],
        }
    }

    fn step(&mut self, root: &RasterLayerNode, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let layer = Self::layer_at(root, &self.path[..self.depth]).ok_or("raster-store.preflight-layer-path")?;
        let frame = self.frames[self.depth];
        match frame.phase {
            0 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.add(1, size_of::<RasterLayerNode>(), 1, size_of::<RasterLayerNode>())?;
                self.frames[self.depth].phase = 1;
            }
            1..=3 => {
                let value = Self::strings(layer)[(frame.phase - 1) as usize];
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                totals.string(value)?;
                self.frames[self.depth].phase += 1;
            }
            4 => {
                let value = match layer {
                    RasterLayerNode::Pixel { image_key, .. } => image_key.as_ref(),
                    RasterLayerNode::Adjustment { adjustment_kind, .. } => Some(adjustment_kind),
                    RasterLayerNode::Group { .. } => None,
                };
                if let Some(value) = value {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    totals.string(value)?;
                } else if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.frames[self.depth].phase = 5;
            }
            5 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                match layer {
                    RasterLayerNode::Group { children, .. } => totals.layer_vector(children)?,
                    RasterLayerNode::Adjustment { params, .. } => totals.map(params, 0)?,
                    RasterLayerNode::Pixel { .. } => {}
                }
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if let RasterLayerNode::Adjustment { params, .. } = layer {
                    if let Some(value) = self.parameter_value.as_mut() {
                        let (_, source) = self.parameter_key.next(params).ok_or("raster-store.preflight-parameter-source")?;
                        if value.step(source, totals, cx)? {
                            let (key, _) = self.parameter_key.next(params).ok_or("raster-store.preflight-parameter-key")?;
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            self.parameter_value = None;
                            self.parameter_key.advance(key)?;
                        }
                        return Ok(false);
                    }
                    if let Some((key, _)) = self.parameter_key.next(params) {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        totals.string(key)?;
                        self.parameter_value = Some(RasterDslValueBoundsAuthority::new(self.depth + 1));
                        return Ok(false);
                    }
                    self.parameter_key = RasterMapKeyCursor::new();
                }
                self.frames[self.depth].phase = 7;
            }
            7 => {
                if let RasterLayerNode::Group { children, .. } = layer {
                    if frame.child < children.len() {
                        if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH || !raster_reserve_unit(cx) {
                            if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                                return Err("raster-store.preflight-layer-depth");
                            }
                            return Ok(false);
                        }
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 8;
            }
            8 => {
                if !raster_reserve_unit(cx) {return Ok(false);}
                if let RasterLayerNode::Pixel {mask:Some(mask),..}|RasterLayerNode::Group {mask:Some(mask),..}=layer {
                    if let Some(key)=&mask.image_key {totals.string(key)?;}
                }
                self.frames[self.depth].phase=9;
            }
            _ => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }
}

struct RasterSnapshotBoundsAuthority {
    totals: RasterOwnerTotals,
    layer: Option<RasterLayerBoundsAuthority>,
    asset_key: RasterMapKeyCursor,
    phase: u8,
    index: usize,
    asset_field: u8,
    terminal: bool,
}

fn raster_exact_string_from_parts(parts: &[&[u8]]) -> Result<String, &'static str> {
    let length = parts.iter().try_fold(0usize, |length, part| length.checked_add(part.len())).ok_or("raster-store.clone-string-length-overflow")?;
    if length > RASTER_OWNED_FIELD_BYTES {
        return Err("raster-store.clone-string-capacity");
    }
    let mut bytes = Box::<[u8]>::new_uninit_slice(length);
    let mut offset = 0;
    for part in parts {
        for byte in *part {
            bytes[offset].write(*byte);
            offset += 1;
        }
    }
    let bytes = unsafe { bytes.assume_init() };
    String::from_utf8(Vec::from(bytes)).map_err(|_| "raster-store.clone-string-utf8")
}

fn raster_clone_owned_string(source: &String) -> Result<String, &'static str> {
    if source.capacity() > RASTER_OWNED_FIELD_BYTES {
        return Err("raster-store.clone-string-capacity");
    }
    raster_exact_string_from_parts(&[source.as_bytes()])
}

impl RasterDslValueCloneAuthority {
    fn skeleton(source: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
        match source {
            semio_framework_value::DslValue::Null => semio_framework_value::DslValue::Null,
            semio_framework_value::DslValue::Bool(value) => semio_framework_value::DslValue::Bool(*value),
            semio_framework_value::DslValue::Number(value) => semio_framework_value::DslValue::Number(*value),
            semio_framework_value::DslValue::String(_) => semio_framework_value::DslValue::String(String::new()),
            semio_framework_value::DslValue::Bytes(_) => semio_framework_value::DslValue::Bytes(Vec::new()),
            semio_framework_value::DslValue::Array(values) => semio_framework_value::DslValue::Array(Vec::with_capacity(values.capacity())),
            semio_framework_value::DslValue::Object(values) => semio_framework_value::DslValue::Object(Vec::with_capacity(values.capacity())),
        }
    }

    fn new(source: &semio_framework_value::DslValue) -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(Some(Self::skeleton(source))),
            retirement: RasterRetirementSlot::new(),
            depth: 0,
            path: [0; RASTER_MAXIMUM_NESTED_DEPTH],
            frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH],
            pending_key: std::mem::ManuallyDrop::new(None),
            root_capacity_observed: false,
            terminal: false,
        }
    }

    fn target_at_mut<'a>(root: &'a mut semio_framework_value::DslValue, path: &[usize]) -> Option<&'a mut semio_framework_value::DslValue> {
        let Some((head, tail)) = path.split_first() else { return Some(root) };
        match root {
            semio_framework_value::DslValue::Array(values) => Self::target_at_mut(values.get_mut(*head)?, tail),
            semio_framework_value::DslValue::Object(values) => Self::target_at_mut(&mut values.get_mut(*head)?.1, tail),
            _ => None,
        }
    }

    fn observe_container_capacity(source: &semio_framework_value::DslValue, target: &semio_framework_value::DslValue, totals: &mut RasterOwnerTotals) -> Result<(), &'static str> {
        match (source, target) {
            (semio_framework_value::DslValue::Array(source), semio_framework_value::DslValue::Array(target)) => totals.observe_candidate_capacity(source.capacity(), target.capacity(), size_of::<semio_framework_value::DslValue>()),
            (semio_framework_value::DslValue::Object(source), semio_framework_value::DslValue::Object(target)) => totals.observe_candidate_capacity(source.capacity(), target.capacity(), size_of::<(String, semio_framework_value::DslValue)>()),
            _ => Ok(()),
        }
    }

    fn step(&mut self, source_root: &semio_framework_value::DslValue, totals: &mut RasterOwnerTotals, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.root_capacity_observed {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            Self::observe_container_capacity(source_root, self.value.as_ref().ok_or("raster-store.clone-value-observed-target")?, totals)?;
            self.root_capacity_observed = true;
            return Ok(false);
        }
        let source = RasterDslValueBoundsAuthority::value_at(source_root, &self.path[..self.depth]).ok_or("raster-store.clone-value-source")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("raster-store.clone-value-target")?, &self.path[..self.depth]).ok_or("raster-store.clone-value-target-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            match (source, target) {
                (semio_framework_value::DslValue::String(source), semio_framework_value::DslValue::String(target)) => {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *target = raster_clone_owned_string(source)?;
                }
                (semio_framework_value::DslValue::Bytes(source), semio_framework_value::DslValue::Bytes(target)) => {
                    if source.len() > RASTER_MAXIMUM_NESTED_BYTES {
                        return Err("raster-store.clone-value-bytes-capacity");
                    }
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    target.extend_from_slice(source);
                }
                _ if !raster_reserve_unit(cx) => return Ok(false),
                _ => {}
            }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        match (source, target) {
            (semio_framework_value::DslValue::Array(source), semio_framework_value::DslValue::Array(target)) if frame.child < source.len() => {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.clone-value-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.push(Self::skeleton(&source[frame.child]));
                Self::observe_container_capacity(&source[frame.child], target.last().ok_or("raster-store.clone-value-child-target")?, totals)?;
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            (semio_framework_value::DslValue::Object(source), semio_framework_value::DslValue::Object(target)) if frame.child < source.len() => {
                if frame.phase == 1 {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    *self.pending_key = Some(raster_clone_owned_string(&source[frame.child].0)?);
                    self.frames[self.depth].phase = 2;
                    return Ok(false);
                }
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.clone-value-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                target.push((self.pending_key.take().ok_or("raster-store.clone-value-key")?, Self::skeleton(&source[frame.child].1)));
                Self::observe_container_capacity(&source[frame.child].1, &target.last().ok_or("raster-store.clone-value-object-target")?.1, totals)?;
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.frames[self.depth].phase = 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
            _ => {}
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.terminal = true;
        } else {
            self.depth -= 1;
        }
        Ok(self.terminal)
    }

    fn take(&mut self) -> Option<semio_framework_value::DslValue> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if self.pending_key.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
        }
        Ok(Default::default())
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        if self.retirement.is_empty() {
            if let Some(key) = self.pending_key.take() {
                self.retirement.put(RasterDisplaced::String(key));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Value(value));
            }
        }
        let step = self.retirement.close(grant)?;
        Ok(raster_finish(step, self.terminal_is_empty()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_empty() && self.pending_key.is_none()
    }
}

impl Drop for RasterDslValueCloneAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster value clone reached Drop before exact handoff or retirement");
    }
}

struct RasterLayerCloneAuthority {
    value: std::mem::ManuallyDrop<Option<RasterLayerNode>>,
    retirement: RasterRetirementSlot,
    bounds: RasterLayerBoundsAuthority,
    totals: RasterOwnerTotals,
    admitted: bool,
    root_capacity_observed: bool,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    parameter_key: RasterMapKeyCursor,
    pending_parameter_key: std::mem::ManuallyDrop<Option<String>>,
    parameter_value: std::mem::ManuallyDrop<Option<Box<RasterDslValueCloneAuthority>>>,
    terminal: bool,
}

impl RasterLayerCloneAuthority {
    fn mask(source: &Option<crate::RasterLayerMask>) -> Option<crate::RasterLayerMask> {
        source.as_ref().map(|value| crate::RasterLayerMask { enabled: value.enabled, linked: value.linked, invert: value.invert, width: value.width, height: value.height, image_key:None, transform:value.transform.clone() })
    }

    fn skeleton(source: &RasterLayerNode) -> RasterLayerNode {
        match source {
            RasterLayerNode::Pixel { visible, locked, opacity, transform, mask, width, height, .. } => RasterLayerNode::Pixel {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                locked: *locked,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: transform.clone(),
                mask: Self::mask(mask),
                width: *width,
                height: *height,
                image_key: None,
            },
            RasterLayerNode::Group { visible, locked, opacity, transform, mask, children, .. } => RasterLayerNode::Group {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                locked: *locked,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: transform.clone(),
                mask: Self::mask(mask),
                children: Vec::with_capacity(children.capacity().saturating_add(1)),
            },
            RasterLayerNode::Adjustment { visible, locked, opacity, transform, .. } => RasterLayerNode::Adjustment {
                id: String::new(),
                name: String::new(),
                visible: *visible,
                locked: *locked,
                opacity: *opacity,
                blend_mode: String::new(),
                transform: transform.clone(),
                adjustment_kind: String::new(),
                params: RasterOwnedMap::new(),
            },
        }
    }

    fn new(_source: &RasterLayerNode) -> Self {
        Self {
            value: std::mem::ManuallyDrop::new(None),
            retirement: RasterRetirementSlot::new(),
            bounds: RasterLayerBoundsAuthority::new(),
            totals: RasterOwnerTotals::new(),
            admitted: false,
            root_capacity_observed: false,
            depth: 0,
            path: [0; RASTER_MAXIMUM_NESTED_DEPTH],
            frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH],
            parameter_key: RasterMapKeyCursor::new(),
            pending_parameter_key: std::mem::ManuallyDrop::new(None),
            parameter_value: std::mem::ManuallyDrop::new(None),
            terminal: false,
        }
    }

    fn target_at_mut<'a>(root: &'a mut RasterLayerNode, path: &[usize]) -> Option<&'a mut RasterLayerNode> {
        let Some((head, tail)) = path.split_first() else { return Some(root) };
        let RasterLayerNode::Group { children, .. } = root else { return None };
        Self::target_at_mut(children.get_mut(*head)?, tail)
    }

    fn strings<'a>(source: &'a RasterLayerNode, target: &'a mut RasterLayerNode) -> [(&'a String, &'a mut String); 3] {
        match (source, target) {
            (RasterLayerNode::Pixel { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Pixel { id: target_id, name: target_name, blend_mode: target_blend, .. })
            | (RasterLayerNode::Group { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Group { id: target_id, name: target_name, blend_mode: target_blend, .. })
            | (RasterLayerNode::Adjustment { id: source_id, name: source_name, blend_mode: source_blend, .. }, RasterLayerNode::Adjustment { id: target_id, name: target_name, blend_mode: target_blend, .. }) => {
                [(source_id, target_id), (source_name, target_name), (source_blend, target_blend)]
            }
            _ => unreachable!("Raster layer clone source and target variants remain exact"),
        }
    }

    fn step(&mut self, source_root: &RasterLayerNode, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if !self.admitted {
            if self.bounds.step(source_root, &mut self.totals, cx)? {
                self.admitted = true;
            }
            return Ok(false);
        }
        if self.value.is_none() {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            *self.value = Some(Self::skeleton(source_root));
            return Ok(false);
        }
        if !self.root_capacity_observed {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            if let (RasterLayerNode::Group { children: source, .. }, RasterLayerNode::Group { children: target, .. }) = (source_root, self.value.as_ref().ok_or("raster-store.clone-layer-observed-target")?) {
                self.totals.observe_candidate_capacity(source.capacity().saturating_add(1), target.capacity(), size_of::<RasterLayerNode>())?;
            }
            self.root_capacity_observed = true;
            return Ok(false);
        }
        let source = RasterLayerBoundsAuthority::layer_at(source_root, &self.path[..self.depth]).ok_or("raster-store.clone-layer-source")?;
        let target = Self::target_at_mut(self.value.as_mut().ok_or("raster-store.clone-layer-target")?, &self.path[..self.depth]).ok_or("raster-store.clone-layer-target-path")?;
        let frame = self.frames[self.depth];
        match frame.phase {
            0..=2 => {
                let (source, target) = &mut Self::strings(source, target)[frame.phase as usize];
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                **target = raster_clone_owned_string(source)?;
                self.frames[self.depth].phase += 1;
            }
            3 => {
                match (source, target) {
                    (RasterLayerNode::Pixel { image_key: source, .. }, RasterLayerNode::Pixel { image_key: target, .. }) => {
                        if let Some(source) = source {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            *target = Some(raster_clone_owned_string(source)?);
                        } else if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                    }
                    (RasterLayerNode::Adjustment { adjustment_kind: source, .. }, RasterLayerNode::Adjustment { adjustment_kind: target, .. }) => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        *target = raster_clone_owned_string(source)?;
                    }
                    _ if !raster_reserve_unit(cx) => return Ok(false),
                    _ => {}
                }
                self.frames[self.depth].phase = 4;
            }
            4 => {
                if let (RasterLayerNode::Adjustment { params: source, .. }, RasterLayerNode::Adjustment { params: target, .. }) = (source, target) {
                    if let Some(authority) = self.parameter_value.as_mut() {
                        let (_, source_value) = self.parameter_key.next(source).ok_or("raster-store.clone-parameter-source")?;
                        if authority.step(source_value, &mut self.totals, cx)? {
                            let (source_key, _) = self.parameter_key.next(source).ok_or("raster-store.clone-parameter-advance")?;
                            let pending_key = self.pending_parameter_key.as_ref().ok_or("raster-store.clone-parameter-key")?;
                            if target.page_required_for_insert(pending_key) {
                                if !raster_reserve_unit(cx) {
                                    return Ok(false);
                                }
                                target.admit_one_page()?;
                                return Ok(false);
                            }
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            let key = self.pending_parameter_key.take().ok_or("raster-store.clone-parameter-key")?;
                            let value = authority.take().ok_or("raster-store.clone-parameter-value")?;
                            drop(self.parameter_value.take());
                            match target.insert_pre_admitted(key, value) {
                                Ok(RasterOwnedMapInsert::Inserted) => {}
                                Ok(RasterOwnedMapInsert::Replaced(mut previous)) => {
                                    self.retirement.put(RasterDisplaced::ValueEntry(previous.take()));
                                    return Err("raster-store.clone-duplicate-parameter");
                                }
                                Err(rejected) => {
                                    self.retirement.put(RasterDisplaced::ValueEntry((rejected.key, rejected.value)));
                                    return Err(rejected.reason);
                                }
                            }
                            self.parameter_key.advance(source_key)?;
                        }
                        return Ok(false);
                    }
                    if let Some((key, value)) = self.parameter_key.next(source) {
                        if self.pending_parameter_key.is_none() {
                            if !raster_reserve_unit(cx) {
                                return Ok(false);
                            }
                            *self.pending_parameter_key = Some(raster_clone_owned_string(key)?);
                            return Ok(false);
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        *self.parameter_value = Some(Box::new(RasterDslValueCloneAuthority::new(value)));
                        return Ok(false);
                    }
                    self.parameter_key = RasterMapKeyCursor::new();
                }
                self.frames[self.depth].phase = 5;
            }
            5 => {
                if let (RasterLayerNode::Group { children: source, .. }, RasterLayerNode::Group { children: target, .. }) = (source, target) {
                    if frame.child < source.len() {
                        if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                            return Err("raster-store.clone-layer-depth");
                        }
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        target.push(Self::skeleton(&source[frame.child]));
                        if let (RasterLayerNode::Group { children: source_child, .. }, RasterLayerNode::Group { children: target_child, .. }) = (&source[frame.child], target.last().ok_or("raster-store.clone-layer-child-target")?) {
                            self.totals.observe_candidate_capacity(source_child.capacity().saturating_add(1), target_child.capacity(), size_of::<RasterLayerNode>())?;
                        }
                        self.path[self.depth] = frame.child;
                        self.frames[self.depth].child += 1;
                        self.depth += 1;
                        self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                        return Ok(false);
                    }
                }
                self.frames[self.depth].phase = 6;
            }
            6 => {
                if !raster_reserve_unit(cx) {return Ok(false);}
                let pair=match (source,target) {
                    (RasterLayerNode::Pixel {mask:source,..},RasterLayerNode::Pixel {mask:target,..})|(RasterLayerNode::Group {mask:source,..},RasterLayerNode::Group {mask:target,..})=>source.as_ref().zip(target.as_mut()),
                    _=>None,
                };
                if let Some((source,target))=pair {
                    if let Some(key)=&source.image_key {target.image_key=Some(raster_clone_owned_string(key)?);}
                }
                self.frames[self.depth].phase=7;
            }
            _ => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                if self.depth == 0 {
                    self.terminal = true;
                } else {
                    self.depth -= 1;
                }
            }
        }
        Ok(self.terminal)
    }

    fn take(&mut self) -> Option<RasterLayerNode> {
        self.terminal.then(|| self.value.take()).flatten()
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(authority) = self.parameter_value.as_ref() {
            return raster_nested(authority.close_demands(body)?);
        }
        if !self.retirement.is_empty() {
            return self.retirement.demands(body);
        }
        if self.pending_parameter_key.is_some() || self.value.is_some() {
            return Ok(RasterRetirementSlot::birth_demand());
        }
        Ok(Default::default())
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(authority) = self.parameter_value.as_mut() {
            let demand = raster_nested(authority.close_demands(grant.maximum_copy_bytes)?)?;
            if grant.maximum_depth < demand.depth {
                return Err(invariant("Raster layer clone parameter exceeds admitted depth"));
            }
            if raster_short(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let step = authority.close_step(raster_child_grant(grant))?;
            if authority.terminal_is_empty() {
                drop(self.parameter_value.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_empty() {
            if let Some(key) = self.pending_parameter_key.take() {
                self.retirement.put(RasterDisplaced::String(key));
            } else if let Some(value) = self.value.take() {
                self.retirement.put(RasterDisplaced::Layer(value));
            }
        }
        let step = self.retirement.close(grant)?;
        Ok(raster_finish(step, self.terminal_is_empty()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.value.is_none() && self.retirement.is_empty() && self.pending_parameter_key.is_none() && self.parameter_value.is_none()
    }
}

impl Drop for RasterLayerCloneAuthority {
    fn drop(&mut self) {
        assert!((self.terminal_is_empty()) || std::thread::panicking(), "Raster layer clone reached Drop before exact handoff or retirement");
    }
}

impl RasterSnapshotBoundsAuthority {
    fn new() -> Self {
        Self { totals: RasterOwnerTotals::new(), layer: None, asset_key: RasterMapKeyCursor::new(), phase: 0, index: 0, asset_field: 0, terminal: false }
    }

    fn step(&mut self, source: &RasterSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        match self.phase {
            0 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.add(1, size_of::<RasterSnapshot>(), 1, size_of::<RasterSnapshot>())?;
                self.phase = 1;
            }
            1 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(&source.schema)?;
                self.phase = 2;
            }
            2 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(&source.id)?;
                self.phase = 3;
            }
            3 => {
                if let Some(title) = &source.title {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    self.totals.string(title)?;
                } else if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.phase = 4;
            }
            4 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.layer_vector(&source.layers)?;
                self.phase = 5;
            }
            5 => {
                if let Some(layer) = self.layer.as_mut() {
                    if layer.step(source.layers.get(self.index).ok_or("raster-store.preflight-root-layer")?, &mut self.totals, cx)? {
                        self.layer = None;
                        self.index += 1;
                    }
                    return Ok(false);
                }
                if source.layers.get(self.index).is_some() {
                    if !raster_reserve_unit(cx) {
                        return Ok(false);
                    }
                    self.layer = Some(RasterLayerBoundsAuthority::new());
                    return Ok(false);
                }
                self.phase = 6;
                self.index = 0;
            }
            6 => {
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.map(&source.assets, usize::from(source.assets.len()<crate::RASTER_OWNED_MAP_CAPACITY))?;
                self.phase = 7;
            }
            7 => {
                let Some((key, child)) = self.asset_key.next(&source.assets) else {
                    self.terminal = true;
                    return Ok(true);
                };
                let value = match self.asset_field {
                    0 => key,
                    1 => &child.child_id,
                    2 => &child.target.artifact_id,
                    3 => &child.target.dialect.artifact_kind,
                    4 => &child.target.dialect.standard,
                    5 => &child.target.dialect.subset,
                    _ => {
                        if !raster_reserve_unit(cx) {
                            return Ok(false);
                        }
                        self.asset_key.advance(key)?;
                        self.asset_field = 0;
                        return Ok(false);
                    }
                };
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.totals.string(value)?;
                self.asset_field += 1;
            }
            _ => self.terminal = true,
        }
        Ok(self.terminal)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RasterLayerAddress {
    length: usize,
    indices: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
}

impl RasterLayerAddress {
    fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

struct RasterLayerLocator {
    root: usize,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    found: Option<RasterLayerAddress>,
    terminal: bool,
}

impl RasterLayerLocator {
    fn new() -> Self {
        Self { root: 0, depth: 0, path: [0; RASTER_MAXIMUM_NESTED_DEPTH], frames: [RasterTraversalFrame::EMPTY; RASTER_MAXIMUM_NESTED_DEPTH], found: None, terminal: false }
    }

    fn node_id(node: &RasterLayerNode) -> &String {
        match node {
            RasterLayerNode::Pixel { id, .. } | RasterLayerNode::Group { id, .. } | RasterLayerNode::Adjustment { id, .. } => id,
        }
    }

    fn node_at(snapshot: &RasterSnapshot, address: RasterLayerAddress) -> Option<&RasterLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let RasterLayerNode::Group { children, .. } = value else { return None };
            value = children.get(*index)?;
        }
        Some(value)
    }

    fn node_at_mut(snapshot: &mut RasterSnapshot, address: RasterLayerAddress) -> Option<&mut RasterLayerNode> {
        fn descend<'a>(value: &'a mut RasterLayerNode, path: &[usize]) -> Option<&'a mut RasterLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let RasterLayerNode::Group { children, .. } = value else { return None };
            descend(children.get_mut(*head)?, tail)
        }
        descend(snapshot.layers.get_mut(address.indices[0])?, &address.indices[1..address.length])
    }

    fn container_mut(snapshot: &mut RasterSnapshot, parent: Option<RasterLayerAddress>) -> Option<&mut Vec<RasterLayerNode>> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                RasterLayerNode::Group { children, .. } => Some(children),
                _ => None,
            },
        }
    }

    fn step(&mut self, snapshot: &RasterSnapshot, target: &str, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = RasterLayerBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("raster-store.locator-path")?;
        let frame = self.frames[self.depth];
        if frame.phase == 0 {
            if !raster_reserve_unit(cx) {
                return Ok(false);
            }
            self.frames[self.depth].phase = 1;
            if Self::node_id(node) == target {
                let mut indices = [0; RASTER_MAXIMUM_NESTED_DEPTH];
                indices[0] = self.root;
                if self.depth > 0 {
                    indices[1..self.depth + 1].copy_from_slice(&self.path[..self.depth]);
                }
                self.found = Some(RasterLayerAddress { length: self.depth + 1, indices });
                self.terminal = true;
            }
            return Ok(self.terminal);
        }
        if let RasterLayerNode::Group { children, .. } = node {
            if frame.child < children.len() {
                if self.depth + 1 >= RASTER_MAXIMUM_NESTED_DEPTH {
                    return Err("raster-store.locator-depth");
                }
                if !raster_reserve_unit(cx) {
                    return Ok(false);
                }
                self.path[self.depth] = frame.child;
                self.frames[self.depth].child += 1;
                self.depth += 1;
                self.frames[self.depth] = RasterTraversalFrame::EMPTY;
                return Ok(false);
            }
        }
        if !raster_reserve_unit(cx) {
            return Ok(false);
        }
        if self.depth == 0 {
            self.root += 1;
            self.frames[0] = RasterTraversalFrame::EMPTY;
        } else {
            self.depth -= 1;
        }
        Ok(false)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterMutationCandidatePhase {
    Clone,
    LocatePrimary,
    LocateSecondary,
    PrepareLayer,
    Apply,
    ShiftRemove,
    LocateDestination,
    BeginInsert,
    ShiftInsert,
    PrepareAsset,
    Drain,
    Complete,
    Closing,
}

/// 🧮 The interactive document lane's clone-free apply of ONE mutation over the live base — the
/// same `RasterMutationCandidateAuthority` the archive-load initializer replays edits with, driven
/// under a locally minted `StepContext` whose fuel is the store grant. `RasterStorePreparation`
/// (`✏️editor/🦀️.rs`) used to go through `Mutation::diff` + `RasterDiff::apply`, which `Clone`s the
/// base and every carried layer and refuses a populated asset map outright — so the moment the demo's
/// brighten adjustment (populated `params`) reached the store, the batch faulted and the framework's
/// close path dropped the un-begun `create-layer` into `RasterOwnedMap`'s Drop guard (react boots of
/// ticket 26/09/05/RASTER-PLUGIN-END-TO-END, 2026-09-16).
pub struct RasterOneItemApply {
    candidate: std::mem::ManuallyDrop<Option<RasterMutationCandidateAuthority>>,
    preview_sequence: u64,
}

impl RasterOneItemApply {
    pub fn new() -> Self {
        Self { candidate: std::mem::ManuallyDrop::new(Some(RasterMutationCandidateAuthority::new())), preview_sequence: 0 }
    }

    /// ▶️ Spends up to `fuel` units under the supplied retained grant; the first value is the post
    /// snapshot once the candidate has produced it, and the second is the exact physical receipt.
    pub fn advance(&mut self, base: &RasterSnapshot, operation: &RasterMutation, operation_id: semio_framework_job::OperationId, generation: semio_framework_job::Generation, fuel: u64, retained: RetainedCloneGrant) -> Result<(Option<RasterSnapshot>, RetainedCloneProgress), ValueError> {
        let candidate = self.candidate.as_mut().ok_or_else(|| invariant("raster-store.one-item-apply-candidate-absent"))?;
        let budget = semio_framework_job::StepBudget::new(fuel.max(1), u64::MAX, retained);
        let mut progress = RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(operation_id, generation, budget, semio_framework_job::CancelToken::root_now(), raster_frozen_now_us, &mut self.preview_sequence, &mut progress);
        let mut spins = fuel.saturating_mul(4).saturating_add(16);
        loop {
            match candidate.step(base, operation, &mut cx) {
                Ok(true) => {
                    let post = candidate.take();
                    drop(self.candidate.take());
                    let receipt = cx.retained_progress();
                    return Ok((post, receipt));
                }
                Ok(false) if cx.should_yield() || spins == 0 => return Ok((None, cx.retained_progress())),
                Ok(false) => spins -= 1,
                Err(code) => return Err(invariant(code).with_retained_progress(cx.retained_progress())),
            }
        }
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        self.candidate.as_ref().map_or(Ok(Default::default()), |candidate| candidate.close_demands(body))
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(candidate) = self.candidate.as_mut() else { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())) };
        let step = candidate.close_step(grant)?;
        if candidate.terminal_is_empty() {
            drop(self.candidate.take());
        }
        Ok(step)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.candidate.is_none()
    }
}

impl Drop for RasterOneItemApply {
    fn drop(&mut self) {
        assert!((self.candidate.is_none()) || std::thread::panicking(), "Raster one-item apply reached Drop before its candidate was closed");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.candidate) };
    }
}

/// 🪜 32 nested groups is far beyond any document; every bounds and clone authority carries two inline `[_; DEPTH]` traversal stacks, so the
/// deep-nesting tests derive their depths from this constant.
const RASTER_MAXIMUM_NESTED_DEPTH: usize = 32;

const RASTER_MAXIMUM_NESTED_ITEMS: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

struct RasterRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for RasterRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "raster-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
        }
        self.terminal = true;
        Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct RasterEnvelopeOwnedFieldCatalog;

#[derive(Clone, Copy)]
struct RasterTraversalFrame {
    phase: u8,
    child: usize,
}

impl RasterTraversalFrame {
    const EMPTY: Self = Self { phase: 0, child: 0 };
}

struct RasterOwnerTotals {
    source_items: usize,
    source_bytes: usize,
    candidate_items: usize,
    candidate_bytes: usize,
}

struct RasterDslValueBoundsAuthority {
    layer_depth: usize,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    terminal: bool,
}

struct RasterLayerBoundsAuthority {
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    parameter_key: RasterMapKeyCursor,
    parameter_value: Option<RasterDslValueBoundsAuthority>,
    terminal: bool,
}

struct RasterDslValueCloneAuthority {
    value: std::mem::ManuallyDrop<Option<semio_framework_value::DslValue>>,
    retirement: RasterRetirementSlot,
    depth: usize,
    path: [usize; RASTER_MAXIMUM_NESTED_DEPTH],
    frames: [RasterTraversalFrame; RASTER_MAXIMUM_NESTED_DEPTH],
    pending_key: std::mem::ManuallyDrop<Option<String>>,
    root_capacity_observed: bool,
    terminal: bool,
}

fn raster_frozen_now_us() -> Option<u64> {
    Some(0)
}

const RASTER_COMBINED_DEPTH_CAPACITY: usize = RASTER_MAXIMUM_NESTED_DEPTH + RASTER_MAXIMUM_NESTED_DEPTH * 2 + 16;

fn raster_combined_depth_requirement(layer_depth: usize, value_depth: usize) -> Result<usize, &'static str> {
    let required = layer_depth.checked_add(value_depth.checked_mul(2).ok_or("raster-store.preflight-combined-depth-overflow")?).and_then(|value| value.checked_add(16)).ok_or("raster-store.preflight-combined-depth-overflow")?;
    if required > RASTER_COMBINED_DEPTH_CAPACITY {
        return Err("raster-store.preflight-combined-depth");
    }
    Ok(required)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
