//! 🧬️ Owned schema decoding: paged, credit-bounded decoding of artifact envelopes and records into retained owners,
//! with field decoder registries, reservations and completed-record tickets.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[path = "../../🚪️io/💾️binary/🌱️genesis/🦀️.rs"]
pub mod genesis_pack_admission;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreOperationRowsRetirement;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirementFactory;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreChangeRetirementFactory;
use crate::genesis_pack_admission::ArtifactGenesisPackCapture;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{Edit, Mutation};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use semio_framework_os_store_read::{artifact_retirement_admit_owned, artifact_retirement_box_close_step, artifact_retirement_box_demands, artifact_retirement_owned_birth_demands, retirement_owner_close, retirement_owner_demands};
use semio_framework_os_store_read::{ArtifactStoreAlternativeRetirementFactory, ArtifactStoreCheckpointRetirementFactory, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryLedgerRetirement, ArtifactStoreOperationRowOwners, ArtifactStoreVcsRetirement, artifact_history_ledger_birth, artifact_history_ledger_handoff};
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_vcs::{Alternative, ArtifactHistoryLedger, ArtifactHistoryReservation, ArtifactVcs, Change, Checkpoint, Identified};
use semio_framework_os_store_space_schema::{ArtifactEnvelope, ArtifactEnvelopeOwners};

use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_ledger::ArtifactEditMessageLedger;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_os_store_backbone::local_storage_backbone_key;

pub use semio_framework_pack_json_retained_schema::*;
pub use semio_framework_os_store_owned_schema_fields::lifted_owned_schema_decode::{ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, ARTIFACT_ENVELOPE_BACKBONE_FIELD, ARTIFACT_ENVELOPE_CONFLICTS_FIELD, ARTIFACT_ENVELOPE_CURSOR_FIELD, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ARTIFACT_ENVELOPE_DIALECT_FIELD, ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD, ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY, ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES, ARTIFACT_ENVELOPE_HISTORY_ITEMS, ARTIFACT_ENVELOPE_ID_FIELD, ARTIFACT_ENVELOPE_LANES_FIELD, ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD, ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS, ARTIFACT_ENVELOPE_OWNER_FIELD, ARTIFACT_ENVELOPE_SCHEMA_FIELD, ARTIFACT_ENVELOPE_VCS_FIELD, ARTIFACT_OWNED_SPR_EDIT_FIELDS, ArtifactEnvelopeDecodedRecordTarget, ArtifactEnvelopeFieldDecodeStep, ArtifactEnvelopeFieldDecoder, ArtifactEnvelopeFieldDecoderLease, ArtifactEnvelopeFieldDecoderRegistry, ArtifactEnvelopeFieldDecoderRegistryFault, ArtifactEnvelopeFieldDecoderRegistryState, ArtifactEnvelopeFieldDecoderSlot, ArtifactEnvelopeFieldDecoderTicket, ArtifactEnvelopeFieldReservation, ArtifactEnvelopeMutationFieldAuthority, ArtifactEnvelopeMutationFieldTarget, ArtifactEnvelopeOwnedFieldCatalog, ArtifactEnvelopeReturnedFieldDecoder, ArtifactEnvelopeSnapshotFieldAuthority, ArtifactEnvelopeSnapshotFieldTarget, ArtifactEnvelopeSprConflictAuthority, ArtifactEnvelopeVcsFieldAuthority, ArtifactOwnedHistoryEntryAuthority, ArtifactOwnedHistoryEntryDecoder, ArtifactOwnedSprEditActive, ArtifactOwnedSprEditAuthority, ArtifactOwnedSprEditDecoder, ArtifactOwnedSprMutationArrayAuthority, ArtifactOwnedSprMutationArrayState, ArtifactOwnedSprMutationTarget, ArtifactRepositoryHistoryEntryAuthority, ArtifactRepositoryHistoryEntryDecoder, OwnedSchemaEmptyArrayAuthority, artifact_bounded_history_entry_decoder, artifact_envelope_diagnostic_error, artifact_envelope_field_demands, artifact_envelope_owned_schema, artifact_owned_spr_edit_history_decoder, artifact_retirement_grant_covers, artifact_retirement_self_grant};









































































































/// 📦️ Exact owner-supplied catalog and retirement factories required to admit an
/// envelope decode. Construction is all-or-nothing so a caller cannot pair a field catalog with
/// unrelated nested-owner disposal authority.
pub struct ArtifactEnvelopeDecodeOwnerBundle<P, Mutation> {
    pub catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    pub initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    pub mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

impl<P, Mutation> ArtifactEnvelopeDecodeOwnerBundle<P, Mutation> {
    pub fn new(catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { catalog, initial_snapshot_factory, mutation_factory }
    }

    pub fn begin_fresh_decoder(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        completed: Arc<ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>>,
        completion: Arc<ArtifactEnvelopeDecodeCompletion>,
    ) -> Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>
    where
        P: Send + Sync + 'static,
        Mutation: Send + 'static,
    {
        Box::new(ArtifactEnvelopeFreshFieldDecoder::new(operation, generation, Arc::clone(&self.catalog), Arc::clone(&self.initial_snapshot_factory), Arc::clone(&self.mutation_factory), completed, completion))
    }

    pub fn retire_envelope(&self, envelope: ArtifactEnvelope<P, Mutation>) -> Box<dyn ErasedSnapshotRetirement>
    where
        P: Send + Sync + 'static,
        Mutation: Send + 'static,
    {
        Box::new(ArtifactStoreEnvelopeRetirement::new(envelope, Arc::clone(&self.initial_snapshot_factory), Arc::clone(&self.mutation_factory)))
    }
}



















































pub enum OwnedSchemaBoundedArrayState {
    AwaitStart,
    Entries,
    Complete,
    Closing,
}

/// 📚️ Retained one-entry-at-a-time decoder for repository-owned history arrays.
/// Each semantic entry is capped at one decode page and enters the fixed generation ledger once.
pub struct OwnedSchemaBoundedArrayAuthority<T> {
    pub path: OwnedSchemaPath,
    pub values: std::mem::ManuallyDrop<Option<ArtifactHistoryLedger<T>>>,
    ledger_retirement: std::mem::ManuallyDrop<Option<ArtifactStoreHistoryLedgerRetirement<T>>>,
    pub reservation: Option<ArtifactHistoryReservation>,
    pub rejected: std::mem::ManuallyDrop<Option<T>>,
    pub retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    pub decoder: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>>>,
    pub factories_close: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    pub active_decoder: std::mem::ManuallyDrop<Option<Box<dyn ArtifactOwnedHistoryEntryAuthority<T>>>>,
    pub active_retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    entry_depth: usize,
    pub state: OwnedSchemaBoundedArrayState,
    pub taken: bool,
}

impl<T: Send + 'static> OwnedSchemaBoundedArrayAuthority<T> {
    pub fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-close-demand", offset: 0, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        if let Some(owner) = self.active_decoder.as_ref() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(copy)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()?.checked_add(1).ok_or(diagnostic)? });
        }
        if let Some(owner) = self.active_retirement.as_ref() { return artifact_retirement_box_demands(owner, copy).map_err(|error| diagnostic.with_native(error)); }
        if self.reservation.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(value) = self.rejected.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("history entry retains its factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.values.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<ArtifactHistoryLedger<T>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.ledger_retirement.as_ref() {
            return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand().map_err(|error| diagnostic.with_native(error))?, capacity_bytes: owner.next_capacity_byte_demand(copy).map_err(|error| diagnostic.with_native(error))?, release_bytes: owner.next_release_byte_demand().map_err(|error| diagnostic.with_native(error))?, depth: owner.next_depth_demand().map_err(|error| diagnostic.with_native(error))?.checked_add(1).ok_or(diagnostic)? });
        }
        if self.retirement_factory.is_some() || self.decoder.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factories_close.iter().flatten().next() {
            let mut demand = owner.demands(copy).map_err(|error| diagnostic.with_native(error))?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
    pub fn new(path: OwnedSchemaPath, retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>, decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>) -> Self {
        Self {
            path,
            values: std::mem::ManuallyDrop::new(Some(ArtifactHistoryLedger::new())),
            ledger_retirement: std::mem::ManuallyDrop::new(None),
            reservation: None,
            rejected: std::mem::ManuallyDrop::new(None),
            retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)),
            decoder: std::mem::ManuallyDrop::new(Some(decoder)),
            factories_close: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            active_decoder: std::mem::ManuallyDrop::new(None),
            active_retirement: std::mem::ManuallyDrop::new(None),
            entry_depth: 0,
            state: OwnedSchemaBoundedArrayState::AwaitStart,
            taken: false,
        }
    }

    pub fn diagnostic(&self, code: &'static str, token: OwnedSchemaToken) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    pub fn accept(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if let Some(decoder) = self.active_decoder.as_mut() {
            let path = self.path;
            let entry_terminal = self.entry_depth == 1 && token.kind == OwnedSchemaTokenKind::ObjectEnd;
            return match decoder.accept_token(token, entry_terminal, source, cx)? {
                ArtifactEnvelopeFieldDecodeStep::Pending => Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                ArtifactEnvelopeFieldDecodeStep::TokenComplete if entry_terminal => Err(self.diagnostic("artifact-envelope.history-entry-false-terminal", token)),
                ArtifactEnvelopeFieldDecodeStep::FieldComplete if !entry_terminal => Err(self.diagnostic("artifact-envelope.history-entry-early-terminal", token)),
                ArtifactEnvelopeFieldDecodeStep::RecordComplete => Err(self.diagnostic("artifact-envelope.history-entry-record-terminal", token)),
                step => {
                    match token.kind {
                        OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                            self.entry_depth = self.entry_depth.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-depth", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                        }
                        OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => {
                            self.entry_depth = self.entry_depth.checked_sub(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-depth", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                        }
                        _ => {}
                    }
                    if self.entry_depth == 0 {
                        let value = decoder.take_value().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-entry-owner-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                        if !decoder.terminal_is_empty() {
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-entry-live-after-take", token));
                        }
                        drop(self.active_decoder.take());
                        let Some(reservation) = self.reservation.take() else {
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-reservation-missing", token));
                        };
                        let Some(values) = self.values.as_mut() else {
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-ledger-missing", token));
                        };
                        if let Err((reservation, value)) = values.insert_reserved(reservation, value) {
                            self.reservation = Some(reservation);
                            *self.rejected = Some(value);
                            return Err(self.diagnostic("artifact-envelope.history-reservation-stale", token));
                        }
                        return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
                    }
                    Ok(step)
                }
            };
        }
        match self.state {
            OwnedSchemaBoundedArrayState::AwaitStart if token.kind == OwnedSchemaTokenKind::ArrayStart && !terminal => {
                self.state = OwnedSchemaBoundedArrayState::Entries;
                Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            OwnedSchemaBoundedArrayState::AwaitStart => Err(self.diagnostic("artifact-envelope.history-expected-array", token)),
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 && token.kind == OwnedSchemaTokenKind::ArrayEnd && terminal => {
                self.state = OwnedSchemaBoundedArrayState::Complete;
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 && token.kind == OwnedSchemaTokenKind::Comma => Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 && token.kind == OwnedSchemaTokenKind::ObjectStart => {
                let Some(values) = self.values.as_mut() else {
                    return Err(self.diagnostic("artifact-envelope.history-ledger-missing", token));
                };
                let reservation = match values.reserve_one() {
                    Ok(reservation) => reservation,
                    Err(_) => return Err(self.diagnostic("artifact-envelope.history-item-capacity", token)),
                };
                self.reservation = Some(reservation);
                *self.active_decoder = Some(self.decoder.as_ref().expect("live history decoder").begin_entry(cx.operation(), cx.generation(), self.path, self.retirement_factory.as_ref().expect("live history retirement factory").clone()));
                self.accept(token, terminal, source, cx)
            }
            OwnedSchemaBoundedArrayState::Entries if self.entry_depth == 0 => Err(self.diagnostic("artifact-envelope.history-expected-record", token)),
            OwnedSchemaBoundedArrayState::Entries => Err(self.diagnostic("artifact-envelope.history-entry-authority-missing", token)),
            OwnedSchemaBoundedArrayState::Complete => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
            OwnedSchemaBoundedArrayState::Closing => Err(self.diagnostic("artifact-envelope.history-closing", token)),
        }
    }

    pub fn take_values(&mut self) -> Option<ArtifactHistoryLedger<T>> {
        if !matches!(self.state, OwnedSchemaBoundedArrayState::Complete) || self.taken || self.reservation.is_some() || self.active_decoder.is_some() || self.active_retirement.is_some() || self.rejected.is_some() {
            return None;
        }
        self.taken = true;
        self.values.take()
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close};
        let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-close-authority", offset: 0, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(diagnostic); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.state = OwnedSchemaBoundedArrayState::Closing;
        if let Some(owner) = self.active_decoder.as_mut() {
            if owner.terminal_is_empty() {
                drop(self.active_decoder.take());
                self.entry_depth = 0;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
            }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child)?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original schema history entry decoder").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.active_retirement.is_some() { return artifact_retirement_box_close_step(&mut self.active_retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (diagnostic).with_native(error)); }
        if self.reservation.is_some() {
            let values = self.values.as_mut().ok_or(diagnostic)?;
            let reservation = self.reservation.take().expect("observed live history reservation");
            if let Err(reservation) = values.cancel_reservation(reservation) {
                self.reservation = Some(reservation);
                return Err(diagnostic);
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(value) = self.rejected.take() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.retirement_factory.as_ref().expect("history entry retains its factory").retire_owned(value, child) {
                Ok((owner, progress)) => { *self.active_retirement = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(diagnostic.with_progress(progress)); } Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, value)) => { *self.rejected = Some(value); Err(diagnostic.with_native(error)) }
            };
        }
        if let Some(values) = self.values.take() {
            *self.ledger_retirement = Some(ArtifactStoreHistoryLedgerRetirement::new(values, Arc::clone(self.retirement_factory.as_ref().expect("original ledger retains its entry factory"))));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(owner) = self.ledger_retirement.as_mut() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original schema history ledger").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(self.ledger_retirement.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(factory) = self.retirement_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[0] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.decoder.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[1] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original schema factory");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original schema history factory").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(slot.take()); }
            self.taken = self.factories_close.iter().all(Option::is_none);
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        self.taken = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.values.is_none() && self.reservation.is_none() && self.rejected.is_none() && self.active_decoder.is_none() && self.active_retirement.is_none() && self.entry_depth == 0 && self.taken && self.ledger_retirement.is_none() && self.retirement_factory.is_none() && self.decoder.is_none() && self.factories_close.iter().all(Option::is_none)
    }
}

impl<T> Drop for OwnedSchemaBoundedArrayAuthority<T> {
    fn drop(&mut self) {
        assert!(
            self.values.is_none() && self.reservation.is_none() && self.rejected.is_none() && self.active_decoder.is_none() && self.active_retirement.is_none() && self.entry_depth == 0 && self.taken && self.ledger_retirement.is_none() && self.retirement_factory.is_none() && self.decoder.is_none() && self.factories_close.iter().all(Option::is_none),
            "bounded history array reached Drop before every exact entry owner was taken or cursor-retired"
        );
    }
}





pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES: usize = 262_144;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES / ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES * 4;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

pub const ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeDecodePage {
    inner: OwnedSchemaDecodePage,
}

/// 📚️ Seals exact caller-pre-admitted pages into the sole 12-field envelope cursor.
pub fn artifact_envelope_decode_record(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, pages: OwnedSchemaDecodePages) -> Result<OwnedSchemaRecordCursor, OwnedSchemaDecodePages> {
    let tokens = OwnedSchemaTokenCursor::try_new(operation, generation, pages)?;
    OwnedSchemaRecordCursor::try_new(artifact_envelope_owned_schema(), tokens).map_err(|(tokens, _)| tokens.into_pages())
}

impl ArtifactEnvelopeDecodePage {
    /// 📄️ Creates one definitionally shallow page from an already bounded producer buffer.
    pub fn try_from_array(bytes: [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES], len: usize) -> Result<Self, [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES]> {
        if len > ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
            return Err(bytes);
        }
        Ok(Self { inner: OwnedSchemaDecodePage { bytes, len: len as u16 } })
    }

    /// 📄️ Constructs the fixed page inside a producer callback whose extent was already accepted.
    pub fn from_preflighted_array(bytes: [u8; ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES], len: usize) -> Self {
        debug_assert!(len <= ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
        Self { inner: OwnedSchemaDecodePage { bytes, len: len as u16 } }
    }

    /// 📥️ Moves this exact page into pre-reserved schema storage or returns it untouched.
    pub fn admit_into(self, pages: &mut OwnedSchemaDecodePages) -> Result<(), (OwnedSchemaDecodeAdmissionFault, Self)> {
        pages.admit_page(self.inner).map_err(|(fault, inner)| (fault, Self { inner }))
    }

    /// 📨️ Commits this page after the same mutable ingress authority ran its producer preflight.
    pub fn admit_preflighted_into(self, pages: &mut OwnedSchemaDecodePages) {
        pages.admit_preflighted_page(self.inner);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeDecodeState {
    Fields,
    ReleaseSuccess,
    ReleaseCancelled,
    ReleaseFault(OwnedSchemaDecodeDiagnostic),
    Transferred,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

/// 🧬️ Persistent fixed-page envelope decode job; terminal outcomes are withheld until every nested owner is empty.
pub struct ArtifactEnvelopeDecodeAuthority<P, Mutation> {
    pub record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>,
    pub fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>,
    pub field_registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    pub field_ticket: ArtifactEnvelopeFieldDecoderTicket,
    field_returned: bool,
    maximum_field_close_byte_demand: usize,
    maximum_retained_field_close_bytes: usize,
    pub released_field_bytes: usize,
    pending_field: Option<(u16, OwnedSchemaToken, bool)>,
    pub state: ArtifactEnvelopeDecodeState,
    pub fault: semio_framework_job::RetainedPayloadBuilder,
    fault_cursor: usize,
}

impl<P, Mutation> ArtifactEnvelopeDecodeAuthority<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// 🏗️ Admits the field decoder before binding the shallow job lease and sealed record cursor.
    pub fn try_new(
        record: OwnedSchemaRecordCursor,
        field_registry: &Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
        fields: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>,
    ) -> Result<Self, (OwnedSchemaRecordCursor, ArtifactEnvelopeFieldDecoderRegistryFault, Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>)> {
        let maximum_field_close_byte_demand = fields.maximum_close_byte_demand();
        let maximum_retained_field_close_bytes = fields.maximum_retained_close_bytes();
        if maximum_field_close_byte_demand > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
            || maximum_retained_field_close_bytes > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES
            || maximum_field_close_byte_demand > maximum_retained_field_close_bytes
        {
            return Err((record, ArtifactEnvelopeFieldDecoderRegistryFault::CloseByteCapacity, fields));
        }
        let fields = match field_registry.try_admit(fields) {
            Ok(fields) => fields,
            Err((fault, fields)) => return Err((record, fault, fields)),
        };
        let field_ticket = fields.ticket();
        Ok(Self {
            record: std::mem::ManuallyDrop::new(Some(record)),
            fields: std::mem::ManuallyDrop::new(Some(fields)),
            field_registry: Arc::clone(field_registry),
            field_ticket,
            field_returned: false,
            maximum_field_close_byte_demand,
            maximum_retained_field_close_bytes,
            released_field_bytes: 0,
            pending_field: None,
            state: ArtifactEnvelopeDecodeState::Fields,
            fault: semio_framework_job::RetainedPayloadBuilder::new(semio_framework_job::JobPayloadStream::Fault),
            fault_cursor: 0,
        })
    }

    pub fn begin_release(&mut self, state: ArtifactEnvelopeDecodeState) {
        self.state = state;
    }

    fn record_release_fault(&mut self, diagnostic: OwnedSchemaDecodeDiagnostic) {
        if !matches!(self.state, ArtifactEnvelopeDecodeState::ReleaseFault(_)) {
            self.state = ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic);
        }
    }

    pub fn diagnostic(code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn checked_field_close_demand(&self, demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_field_close_byte_demand {
            return Err(Self::diagnostic("artifact-envelope.decode-close-demand-over-admitted-maximum"));
        }
        Ok(demand)
    }

    fn record_released_field_bytes(&mut self, released_bytes: usize) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        let released_field_bytes = self.released_field_bytes.checked_add(released_bytes).filter(|total| *total <= self.maximum_retained_field_close_bytes).ok_or_else(|| Self::diagnostic("artifact-envelope.decode-close-byte-ledger-overflow"))?;
        self.released_field_bytes = released_field_bytes;
        Ok(())
    }

    fn release_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        cx.set_stage("artifact-envelope-decode-close");
        if cx.fuel_exhausted()||cx.deadline_exceeded(){return Ok(())}
        semio_framework_job::InteractiveJob::close_step(self,cx)?;
        Ok(())
    }

    /// 🧹️ Transfers an unstarted exact decode owner into a retained close authority.
    pub fn reject(self, diagnostic: OwnedSchemaDecodeDiagnostic) -> Result<ArtifactEnvelopeDecodeRejected<P, Mutation>, Self> {
        if !matches!(self.state, ArtifactEnvelopeDecodeState::Fields) || self.record.is_none() || self.fields.is_none() || self.field_returned || self.pending_field.is_some() || !self.fault.terminal_is_empty() {
            return Err(self);
        }
        let mut source = std::mem::ManuallyDrop::new(self);
        let Some(record) = source.record.take() else { unreachable!("validated rejected record owner") };
        let Some(fields) = source.fields.take() else { unreachable!("validated rejected field owner") };
        let field_registry = Arc::clone(&source.field_registry);
        let field_ticket = source.field_ticket;
        let maximum_field_close_byte_demand = source.maximum_field_close_byte_demand;
        let maximum_retained_field_close_bytes = source.maximum_retained_field_close_bytes;
        let released_field_bytes = source.released_field_bytes;
        source.state = ArtifactEnvelopeDecodeState::Transferred;
        unsafe { std::mem::ManuallyDrop::drop(&mut source) };
        Ok(ArtifactEnvelopeDecodeRejected {
            record: std::mem::ManuallyDrop::new(Some(record)),
            fields: std::mem::ManuallyDrop::new(Some(fields)),
            field_registry,
            field_ticket,
            field_returned: false,
            maximum_field_close_byte_demand,
            maximum_retained_field_close_bytes,
            released_field_bytes,
            diagnostic,
        })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.fault.terminal_is_empty()
            && self.record.is_none()
            && self.fields.is_none()
            && self.field_returned
            && self.field_registry.ticket_reclaimed(self.field_ticket)
            && matches!(self.state, ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_))
    }

    /// 📏️ Observes the next close turn: the leased field decoder as a child, then one fixed page, then the terminal handoff.
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        if !self.fault.terminal_is_empty() {
            let mut demand = self.fault.retirement_demands().map_err(|error| Self::diagnostic("artifact-envelope.fault-close-demand").with_native(error))?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.fault-close-depth"))?;
            return Ok(demand);
        }
        if let Some(fields) = self.fields.as_ref() {
            let demand = match fields.with_owner(|owner| artifact_envelope_field_demands(owner, maximum_copy_bytes)) {
                Ok(demand) => demand?,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }),
                Err(_) => return Err(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")),
            };
            let demand = self.checked_field_close_demand(demand)?;
            return Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.decode-close-depth-overflow"))?, ..demand });
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })
    }

    pub fn released_field_bytes(&self) -> usize {
        self.released_field_bytes
    }
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<semio_framework_job::InteractiveJobCloseStep,ValueError> {
        use semio_framework_job::InteractiveJobCloseStep as Close;
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Close::Pending { progress: empty }); }
        if matches!(self.state,ArtifactEnvelopeDecodeState::Fields){self.state=ArtifactEnvelopeDecodeState::ReleaseCancelled;}
        self.pending_field=None;
        if !self.fault.terminal_is_empty() {
            let demand = match self.close_demands(grant.maximum_copy_bytes) {
                Ok(demand) => demand,
                Err(diagnostic) => return Err(artifact_envelope_diagnostic_error(diagnostic)),
            };
            if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes || demand.depth > grant.maximum_depth { return Ok(Close::Pending { progress: empty }); }
            let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.fault.close_step_granted(child_grant) {
                Ok(step) => Ok(Close::Pending { progress: step.progress() }),
                Err(error) => Err(error),
            };
        }
        if self.fields.is_some() {
            if grant.maximum_items == 0 {
                return Ok(Close::Pending { progress: empty });
            }
            let terminal = match self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| owner.terminal_is_empty()) {
                Ok(terminal) => terminal,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty));
                }
            };
            if terminal {
                if !self.fields.as_mut().expect("closing job holds its field decoder lease").return_now() {
                    return Ok(Close::Blocked);
                }
                drop(self.fields.take());
                self.field_returned = true;
                return Ok(Close::Pending { progress: RetainedCloneProgress { copied_items: 1, ..empty } });
            }
            let demand = match self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| artifact_envelope_field_demands(owner, grant.maximum_copy_bytes)) {
                Ok(Ok(demand)) => demand,
                Ok(Err(diagnostic)) => {
                    self.record_release_fault(diagnostic);
                    return Err(artifact_envelope_diagnostic_error(diagnostic));
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty));
                }
            };
            let demand = match self.checked_field_close_demand(demand) {
                Ok(demand) => demand,
                Err(diagnostic) => {
                    self.record_release_fault(diagnostic);
                    return Err(artifact_envelope_diagnostic_error(diagnostic));
                }
            };
            if grant.maximum_depth <= demand.depth {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original decoder close refused").with_retained_progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            if !artifact_retirement_grant_covers(child, demand) {
                return Ok(Close::Pending { progress: empty });
            }
            let close = self.fields.as_ref().expect("closing job holds its field decoder lease").with_owner(|owner| {
                let step = owner.close_step(child);
                (step, owner.terminal_is_empty())
            });
            return match close {
                Ok((Ok(step), terminal)) => {
                    let progress = step.progress();
                    if !progress.fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
                        self.record_release_fault(Self::diagnostic("artifact-envelope.decode-close-over-budget"));
                        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(progress));
                    }
                    if let Err(diagnostic) = self.record_released_field_bytes(progress.released_bytes) {
                        self.record_release_fault(diagnostic);
                        return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(progress));
                    }
                    Ok(Close::Pending { progress })
                }
                Ok((Err(diagnostic), _)) => {
                    let released=self.record_released_field_bytes(diagnostic.retained_progress.released_bytes);
                    self.record_release_fault(diagnostic);
                    if let Err(ledger)=released{self.record_release_fault(ledger.with_progress(diagnostic.retained_progress));}
                    Err(artifact_envelope_diagnostic_error(diagnostic))
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => Ok(Close::Blocked),
                Err(_) => {
                    self.record_release_fault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid"));
                    Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(empty))
                }
            };
        }
        if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
            if self.field_returned {
                let _ = self.field_registry.reclaim_returned_ticket_now(self.field_ticket);
            }
            if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
                return Ok(Close::Blocked);
            }
        }
        if let Some(record) = self.record.as_mut() {
            let nonterminal = !record.terminal_is_empty();
            if grant.maximum_items == 0 || nonterminal && grant.maximum_copy_bytes < record.next_close_copy_byte_demand() {
                return Ok(Close::Pending { progress: empty });
            }
            if !nonterminal {
                drop(self.record.take());
                return Ok(Close::Pending { progress: RetainedCloneProgress { copied_items: 1, ..empty } });
            }
            if grant.maximum_depth < 2 {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original decoder close refused").with_retained_progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            match record.close_step(child) {
                Ok(step) if step.progress().fits(child) => return Ok(Close::Pending { progress: step.progress() }),
                Ok(step) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original decoder close refused").with_retained_progress(step.progress())),
                Err(error) => return Err(error),
            }
        }
        let before = self.state;
        self.state = match self.state {
            ArtifactEnvelopeDecodeState::ReleaseSuccess | ArtifactEnvelopeDecodeState::Complete => ArtifactEnvelopeDecodeState::Complete,
            ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic) | ArtifactEnvelopeDecodeState::Fault(diagnostic) => ArtifactEnvelopeDecodeState::Fault(diagnostic),
            _ => ArtifactEnvelopeDecodeState::Cancelled,
        };
        if ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self) { Ok(Close::Complete { progress: RetainedCloneProgress { copied_items: usize::from(before != self.state), ..empty } }) } else { Ok(Close::Blocked) }
    }

    fn advance_decode(&mut self, cx: &mut semio_framework_job::StepContext<'_>) {
        match self.state {
            ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_) => return,
            ArtifactEnvelopeDecodeState::Fields => {}
            _ => unreachable!("release states returned before decode"),
        }
        if cx.is_cancelled() {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
            return ();
        }
        cx.set_stage("artifact-envelope-decode-fields");
        let Some(record) = self.record.as_mut() else {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.missing-record-owner")));
            return ();
        };
        let Some(fields) = self.fields.as_ref() else {
            self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.missing-field-owner")));
            return ();
        };
        if let Some((field_id, token, terminal)) = self.pending_field {
            return match fields.with_owner(|owner| owner.accept_field_token(field_id, token, terminal, record, cx)) {
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) => (),
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)) => {
                    self.pending_field = None;
                    ()
                }
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)) => {
                    self.pending_field = None;
                    ()
                }
                Ok(Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.field-published-record-before-outer-terminal")));
                    ()
                }
                Ok(Err(diagnostic)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                    ()
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => (),
                Err(_) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")));
                    ()
                }
            };
        }
        match record.step(cx) {
            OwnedSchemaRecordStep::Pending => (),
            OwnedSchemaRecordStep::FieldToken { field_id, token, terminal } => {
                self.pending_field = Some((field_id, token, terminal));
                ()
            }
            OwnedSchemaRecordStep::Complete => match fields.with_owner(|owner| {
                let step = owner.finish_record(cx);
                let terminal = owner.terminal_is_empty();
                (step, terminal)
            }) {
                Ok((Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete), true)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseSuccess);
                    ()
                }
                Ok((Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete), false)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.record-complete-with-live-owner")));
                    ()
                }
                Ok((Ok(_), _)) => (),
                Ok((Err(diagnostic), _)) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                    ()
                }
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => (),
                Err(_) => {
                    self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(Self::diagnostic("artifact-envelope.decode-field-lease-invalid")));
                    ()
                }
            },
            OwnedSchemaRecordStep::Cancelled => {
                self.begin_release(ArtifactEnvelopeDecodeState::ReleaseCancelled);
                ()
            }
            OwnedSchemaRecordStep::Fault(diagnostic) => {
                self.begin_release(ArtifactEnvelopeDecodeState::ReleaseFault(diagnostic));
                ()
            }
        }
    }

}

impl<P, Mutation> semio_framework_job::InteractiveJob for ArtifactEnvelopeDecodeAuthority<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if cx.should_yield() || grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        let before = cx.retained_progress();
        let state = self.state;
        if matches!(self.state,ArtifactEnvelopeDecodeState::ReleaseSuccess|ArtifactEnvelopeDecodeState::ReleaseCancelled|ArtifactEnvelopeDecodeState::ReleaseFault(_)){self.release_step(cx)?;}else{self.advance_decode(cx);}
        let outcome=match self.state {
            ArtifactEnvelopeDecodeState::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            ArtifactEnvelopeDecodeState::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            ArtifactEnvelopeDecodeState::Fault(diagnostic) => {
                if !self.fault.advance_initialization(cx)? { return Ok(None); }
                if !self.fault.append_original(cx, diagnostic.code.as_bytes(), &mut self.fault_cursor)? { return Ok(None); }
                if !self.fault.seal(cx)? { return Ok(None); }
                semio_framework_job::JobOutcomeBorrow::admit_fault(cx, self.fault.published().expect("sealed original fault remains inside decoder"))
            }
            _ if before == cx.retained_progress() && state == self.state => Ok(None),
            _ => semio_framework_job::JobOutcomeBorrow::admit_yield(cx),
        };
        outcome.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Complete if self.state == ArtifactEnvelopeDecodeState::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Cancelled if self.state == ArtifactEnvelopeDecodeState::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Fault if matches!(self.state, ArtifactEnvelopeDecodeState::Fault(_)) => descriptor.fault(self.fault.published().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original decoder fault payload is not published"))?),
            _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "decoder outcome descriptor lacks its original semantic result")),
        }
    }

    fn begin_close(&mut self) {
        if matches!(self.state, ArtifactEnvelopeDecodeState::Fields) {
            self.state = ArtifactEnvelopeDecodeState::ReleaseCancelled;
        }
        self.pending_field = None;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let original=cx.retained_grant();
        let grant=RetainedCloneGrant{maximum_items:original.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)),..original};
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        let step=self.close_granted(grant).and_then(|step|step.admit(grant,ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self))).map_err(|error|{cx.consume_fuel(error.retained_progress().copied_items as u64);semio_framework_job::OriginalTurnFailure::debit_original(error,cx)})?;
        cx.consume_fuel(step.progress().copied_items as u64);
        cx.consume_retained(step.progress()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        Ok(step)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.close_demands(maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.close_demands(0).map_err(artifact_envelope_diagnostic_error)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactEnvelopeDecodeAuthority::terminal_is_empty(self)
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeDecodeAuthority<P, Mutation> {
    fn drop(&mut self) {
        if matches!(self.state, ArtifactEnvelopeDecodeState::Transferred) {
            assert!(std::thread::panicking() || (self.record.is_none() && self.fields.is_none() && self.pending_field.is_none() && !self.field_returned), "transferred artifact envelope decode source retained an owner after exact rejection handoff");
            return;
        }
        let terminal = self.fault.terminal_is_empty()
            && self.record.is_none()
            && self.fields.is_none()
            && self.field_returned
            && self.field_registry.ticket_reclaimed(self.field_ticket)
            && matches!(self.state, ArtifactEnvelopeDecodeState::Complete | ArtifactEnvelopeDecodeState::Cancelled | ArtifactEnvelopeDecodeState::Fault(_));
        assert!(std::thread::panicking() || (terminal), "artifact envelope decode authority reached Drop before terminal publication or retained close");
    }
}

pub struct ArtifactEnvelopeDecodeRejected<P, Mutation> {
    pub record: std::mem::ManuallyDrop<Option<OwnedSchemaRecordCursor>>,
    pub fields: std::mem::ManuallyDrop<Option<ArtifactEnvelopeFieldDecoderLease<P, Mutation>>>,
    pub field_registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    pub field_ticket: ArtifactEnvelopeFieldDecoderTicket,
    field_returned: bool,
    maximum_field_close_byte_demand: usize,
    maximum_retained_field_close_bytes: usize,
    pub released_field_bytes: usize,
    pub diagnostic: OwnedSchemaDecodeDiagnostic,
}

impl<P, Mutation> ArtifactEnvelopeDecodeRejected<P, Mutation> {
    /// 📏️ Observes the leased field decoder as a child, then one fixed page; `None` is a contended lease.
    pub fn demands(&self, maximum_copy_bytes: usize) -> Result<Option<semio_framework_value::RetirementDemand>, ValueError> {
        if let Some(fields) = self.fields.as_ref() {
            let demand = match fields.with_owner(|owner| artifact_envelope_field_demands(owner, maximum_copy_bytes)) {
                Ok(demand) => demand.map_err(artifact_envelope_diagnostic_error)?,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(None),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_field_close_byte_demand {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field close demand exceeded its admitted maximum"));
            }
            return Ok(Some(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected field depth overflow"))?, ..demand }));
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(Some(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() }));
        }
        Ok(Some(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }))
    }

    fn demand_axis(&self, maximum_copy_bytes: usize, axis: impl FnOnce(semio_framework_value::RetirementDemand) -> usize) -> Result<usize, ValueError> {
        Ok(axis(self.demands(maximum_copy_bytes)?.unwrap_or(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })))
    }
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeDecodeRejected<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// 🧱️ A retained decode page is a GRANULE: it is released whole or not at all, so a turn
    /// that cannot pay a full page buys nothing. The empty record husk still completes under
    /// any grant — its last page has already been paid for.
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.fields.is_some() {
            let terminal = match self.fields.as_ref().expect("rejected decode retains its field lease").with_owner(|owner| owner.terminal_is_empty()) {
                Ok(terminal) => terminal,
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(RetainedCloneStep::Progress(empty)),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            if terminal {
                if !self.fields.as_mut().expect("rejected decode retains its field lease").return_now() {
                    return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner was returned twice"));
                }
                drop(self.fields.take());
                self.field_returned = true;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let Some(demand) = self.demands(grant.maximum_copy_bytes)? else { return Ok(RetainedCloneStep::Progress(empty)) };
            if grant.maximum_depth < demand.depth {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected field owner exceeds admitted depth"));
            }
            if !artifact_retirement_grant_covers(grant, demand) {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let (step, terminal) = match self.fields.as_ref().expect("rejected decode retains its field lease").with_owner(|owner| {
                let step = owner.close_step(child);
                (step, owner.terminal_is_empty())
            }) {
                Ok((Ok(step), terminal)) => (step, terminal),
                Ok((Err(diagnostic), _)) => return Err(artifact_envelope_diagnostic_error(diagnostic)),
                Err(ArtifactEnvelopeFieldDecoderRegistryFault::Contended) => return Ok(RetainedCloneStep::Progress(empty)),
                Err(_) => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field lease is invalid")),
            };
            let progress = step.progress();
            if !progress.fits(child) {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner exceeded its close grant"));
            }
            if matches!(step, RetainedCloneStep::Complete(_)) && !terminal {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field owner reported a false terminal"));
            }
            self.released_field_bytes = self
                .released_field_bytes
                .checked_add(progress.released_bytes)
                .filter(|total| *total <= self.maximum_retained_field_close_bytes)
                .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "artifact envelope rejected field close ledger exceeded its admitted aggregate"))?;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.field_returned || !self.field_registry.ticket_reclaimed(self.field_ticket) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let Some(record) = self.record.as_mut() {
            if record.terminal_is_empty() {
                drop(self.record.take());
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            if grant.maximum_copy_bytes < record.next_close_copy_byte_demand() {
                return Ok(RetainedCloneStep::Progress(empty));
            }
            if grant.maximum_depth < 2 {
                return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "artifact envelope rejected record exceeds admitted depth"));
            }
            let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = record.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_progress(child, step.progress(), "artifact envelope rejected record")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.record.is_none() && self.fields.is_none() && self.field_returned && self.field_registry.ticket_reclaimed(self.field_ticket)
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { self.demand_axis(maximum_body_bytes, |demand| demand.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.demand_axis(0, |demand| demand.depth) }
}

impl<P, Mutation> Drop for ArtifactEnvelopeDecodeRejected<P, Mutation> {
    fn drop(&mut self) {
        let _ = self.diagnostic;
        assert!(
            std::thread::panicking() || (self.record.is_none() && self.fields.is_none() && self.field_returned && self.field_registry.ticket_reclaimed(self.field_ticket)),
            "artifact envelope decode rejection reached Drop before every exact page owner was cursor-retired"
        );
    }
}

/// 🧹️ Exact close authority used when the shared field registry rejects admission
/// after the record pages and owner-supplied decoder have already been constructed.
pub struct ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    pub record: Option<OwnedSchemaRecordCursor>,
    pub fields: Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>,
    pub terminal: bool,
}

impl<P, Mutation> ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    pub fn new(record: OwnedSchemaRecordCursor, fields: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>) -> Self {
        Self { record: Some(record), fields: Some(fields), terminal: false }
    }

    /// 📏️ Observes the unadmitted decoder as a child, the separate release of its terminal shell, then one fixed page.
    pub fn demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(fields) = self.fields.as_ref() {
            if fields.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(fields.as_ref()), depth: 1, ..Default::default() }); }
            let demand = artifact_envelope_field_demands(fields.as_ref(), maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?;
            return Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "unadmitted envelope field decoder depth overflow"))?, ..demand });
        }
        if let Some(record) = self.record.as_ref().filter(|record| !record.terminal_is_empty()) {
            return Ok(semio_framework_value::RetirementDemand { copy_bytes: record.next_close_copy_byte_demand(), depth: 2, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() })
    }
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "unadmitted envelope rejection exceeds admitted depth"));
        }
        if !artifact_retirement_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(fields) = self.fields.as_mut() {
            if fields.terminal_is_empty() {
                drop(self.fields.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
            }
            let step = fields.close_step(child).map_err(artifact_envelope_diagnostic_error)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, fields.terminal_is_empty(), "unadmitted envelope field decoder")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(record) = self.record.as_mut() {
            if record.terminal_is_empty() {
                drop(self.record.take());
                self.terminal = true;
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let step = record.close_step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_progress(child, step.progress(), "unadmitted envelope record")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.record.is_none() && self.fields.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactEnvelopeUnadmittedDecodeRejected<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.record.is_none() && self.fields.is_none()), "unadmitted envelope decode rejection reached Drop before every exact owner was cursor-retired");
    }
}

pub const ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeCompletedRecordTicket {
    pub index: u16,
    pub generation: u64,
}

impl ArtifactEnvelopeCompletedRecordTicket {
    pub fn index(self) -> usize {
        usize::from(self.index)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeCompletedRecordFault {
    Contended,
    Capacity,
    GenerationExhausted,
    Stale,
    NotReady,
    FalseTerminal,
}

/// 🎯 Atomic consumer for one completed envelope. Rejection returns the exact record to
/// its app-owned slot; no caller receives a stack-local owner on a fallible branch.
pub trait ArtifactEnvelopeCompletedRecordTarget<P, Mutation> {
    fn try_adopt_completed(&mut self, envelope: ArtifactEnvelope<P, Mutation>) -> Result<(), ArtifactEnvelope<P, Mutation>>;
}

/// 🎁️ Exact completed envelope owner retained by the app until one consumer takes it or close retires it.
pub trait ArtifactEnvelopeCompletedRecord<P, Mutation>: Send {
    fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError>;
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_close_depth_demand(&self) -> Result<usize, ValueError>;
    fn terminal_is_empty(&self) -> bool;
}

pub struct ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    pub envelope: std::mem::ManuallyDrop<Option<ArtifactEnvelope<P, Mutation>>>,
    pub retirement: std::mem::ManuallyDrop<Option<ArtifactStoreEnvelopeRetirement<P, Mutation>>>,
    pub initial_snapshot_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub factory_retirement: [Option<semio_framework_value::FactoryAuthority>; 2],
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    pub fn new(envelope: ArtifactEnvelope<P, Mutation>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { envelope: std::mem::ManuallyDrop::new(Some(envelope)), retirement: std::mem::ManuallyDrop::new(None), initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(initial_snapshot_factory)), mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)), factory_retirement: std::array::from_fn(|_| None) }
    }

    pub fn ownership_is_empty(&self) -> bool {
        self.envelope.is_none() && self.retirement.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.factory_retirement.iter().all(Option::is_none)
    }
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecord<P, Mutation> for ArtifactEnvelopeCompletedRecordOwner<P, Mutation>
where
    P: Send + Sync + 'static,
    Mutation: Send + 'static,
{
    fn try_publish_to(&mut self, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> bool {
        if self.retirement.is_some() || self.initial_snapshot_factory.is_none() || self.mutation_factory.is_none() { return false; }
        let Some(envelope) = self.envelope.take() else { return false };
        match target.try_adopt_completed(envelope) {
            Ok(()) => true,
            Err(envelope) => { *self.envelope = Some(envelope); false }
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.ownership_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "completed record exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if self.retirement.is_some() {
            let step = retirement_owner_close(&mut self.retirement, grant)?;
            return Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        if self.envelope.is_some() {
            let snapshot = self.initial_snapshot_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed envelope lost its snapshot factory"))?;
            let mutation = self.mutation_factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed envelope lost its mutation factory"))?;
            let envelope = self.envelope.take().expect("completed envelope remains retained before original retirement");
            *self.retirement = Some(ArtifactStoreEnvelopeRetirement::new(envelope, Arc::clone(snapshot), Arc::clone(mutation)));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(factory) = self.initial_snapshot_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_retirement[0] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.mutation_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_retirement[1] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        for index in 0..self.factory_retirement.len() {
            if let Some(factory) = self.factory_retirement[index].as_mut() {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                let step = factory.step(child)?;
                semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "completed original factory")?;
                if factory.terminal_is_empty() { self.factory_retirement[index] = None; }
                return Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
            }
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, ValueError> { Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty() }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    /// 📏️ Quotes the original inline envelope child and separately retained factory capabilities.
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.retirement.is_some() { return retirement_owner_demands(&self.retirement, maximum_copy_bytes); }
        if self.envelope.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if self.initial_snapshot_factory.is_some() || self.mutation_factory.is_some() { return Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        for factory in self.factory_retirement.iter().flatten() {
            let mut demand = factory.demands(maximum_copy_bytes)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "completed factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeCompletedRecordOwner<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "completed envelope owner reached Drop before its original record and factories were empty");
    }
}

pub struct ArtifactEnvelopeCompletedRecordSlot<P, Mutation> {
    pub generation: u64,
    pub owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>>,
}

pub struct ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation> {
    pub slots: Box<[ArtifactEnvelopeCompletedRecordSlot<P, Mutation>]>,
    pub free: [u16; ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY],
    pub free_len: usize,
    pub live: usize,
    occupied: u64,
    pub closing: u64,
}

impl<P, Mutation> Drop for ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation> {
    fn drop(&mut self) {
        assert!(
            std::thread::panicking() || (self.live == 0 && self.occupied == 0 && self.closing == 0 && self.slots.iter().all(|slot| slot.owner.is_none())),
            "completed envelope registry reached Drop before every exact record owner was consumed or cursor-retired"
        );
    }
}

/// 🗄️ Fixed app-owned completed-record registry; result publication never puts a deep envelope on a job stack.
pub struct ArtifactEnvelopeCompletedRecordRegistry<P, Mutation> {
    pub state: Mutex<ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>>,
}

impl<P, Mutation> ArtifactEnvelopeCompletedRecordRegistry<P, Mutation> {
    pub fn new() -> Arc<Self> {
        let slots = (0..ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY).map(|_| ArtifactEnvelopeCompletedRecordSlot { generation: 0, owner: std::mem::ManuallyDrop::new(None) }).collect::<Vec<_>>().into_boxed_slice();
        let mut free = [0; ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY];
        for (index, slot) in free.iter_mut().enumerate() {
            *slot = index as u16;
        }
        Arc::new(Self { state: Mutex::new(ArtifactEnvelopeCompletedRecordRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY, live: 0, occupied: 0, closing: 0 }) })
    }

    pub fn try_admit(&self, owner: Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>) -> Result<ArtifactEnvelopeCompletedRecordTicket, (ArtifactEnvelopeCompletedRecordFault, Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>)> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err((ArtifactEnvelopeCompletedRecordFault::Contended, owner)),
        };
        if state.free_len == 0 {
            return Err((ArtifactEnvelopeCompletedRecordFault::Capacity, owner));
        }
        state.free_len -= 1;
        let index = usize::from(state.free[state.free_len]);
        let Some(generation) = state.slots[index].generation.checked_add(1) else {
            let free_index = state.free_len;
            state.free[free_index] = index as u16;
            state.free_len += 1;
            return Err((ArtifactEnvelopeCompletedRecordFault::GenerationExhausted, owner));
        };
        state.slots[index].generation = generation;
        *state.slots[index].owner = Some(owner);
        state.live += 1;
        state.occupied |= 1u64 << index;
        Ok(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation })
    }

    pub fn validate(state: &ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<usize, ArtifactEnvelopeCompletedRecordFault> {
        let index = ticket.index();
        let Some(slot) = state.slots.get(index) else { return Err(ArtifactEnvelopeCompletedRecordFault::Stale) };
        if slot.generation != ticket.generation || slot.owner.is_none() {
            return Err(ArtifactEnvelopeCompletedRecordFault::Stale);
        }
        Ok(index)
    }

    fn release_slot(state: &mut ArtifactEnvelopeCompletedRecordRegistryState<P, Mutation>, index: usize) {
        let free_index = state.free_len;
        state.free[free_index] = index as u16;
        state.free_len += 1;
        state.live -= 1;
        state.occupied &= !(1u64 << index);
        state.closing &= !(1u64 << index);
    }

    pub fn try_publish_to(&self, ticket: ArtifactEnvelopeCompletedRecordTicket, target: &mut dyn ArtifactEnvelopeCompletedRecordTarget<P, Mutation>) -> Result<bool, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        if state.closing & (1u64 << index) != 0 { return Err(ArtifactEnvelopeCompletedRecordFault::Stale); }
        let published = state.slots[index].owner.as_mut().is_some_and(|owner| owner.try_publish_to(target));
        if !published { return Ok(false); }
        state.closing |= 1u64 << index;
        Ok(true)
    }

    pub fn try_detach(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        let owner = state.slots[index].owner.take().expect("validated completed record remains established");
        Self::release_slot(&mut state, index);
        Ok(owner)
    }

    /// 🚫 Marks a rejected/cancelled consumer result for the app maintenance close pump.
    pub fn try_request_close(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<(), ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        let index = Self::validate(&state, ticket)?;
        state.closing |= 1u64 << index;
        Ok(())
    }

    pub fn try_next_ticket(&self, cursor: &mut usize) -> Result<Option<ArtifactEnvelopeCompletedRecordTicket>, ArtifactEnvelopeCompletedRecordFault> {
        let state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.live == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.occupied.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        Ok(Some(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation: state.slots[index].generation }))
    }

    /// 🧹 Returns only records explicitly handed back by a cancelled/rejected consumer.
    pub fn try_next_close_ticket(&self, cursor: &mut usize) -> Result<Option<ArtifactEnvelopeCompletedRecordTicket>, ArtifactEnvelopeCompletedRecordFault> {
        let state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.closing == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.closing.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        Ok(Some(ArtifactEnvelopeCompletedRecordTicket { index: index as u16, generation: state.slots[index].generation }))
    }

    pub fn try_detach_next(&self, cursor: &mut usize) -> Result<Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>, ArtifactEnvelopeCompletedRecordFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeCompletedRecordFault::Contended)?;
        if state.live == 0 {
            *cursor = 0;
            return Ok(None);
        }
        let start = *cursor % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let offset = state.occupied.rotate_right(start as u32).trailing_zeros() as usize;
        let index = (start + offset) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        *cursor = (index + 1) % ARTIFACT_ENVELOPE_COMPLETED_RECORD_CAPACITY;
        let owner = state.slots[index].owner.take().expect("occupied completed-record bit retains exact owner");
        Self::release_slot(&mut state, index);
        Ok(Some(owner))
    }

    pub fn ticket_reclaimed(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> bool {
        self.state.try_lock().is_ok_and(|state| Self::validate(&state, ticket).is_err())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.try_lock().is_ok_and(|state| state.live == 0)
    }
}

/// 🎫️ Shallow publication cell for the exact completed-record ticket produced by one decode job.
pub struct ArtifactEnvelopeDecodeCompletion {
    pub index: std::sync::atomic::AtomicU16,
    pub generation: std::sync::atomic::AtomicU64,
}

impl ArtifactEnvelopeDecodeCompletion {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { index: std::sync::atomic::AtomicU16::new(u16::MAX), generation: std::sync::atomic::AtomicU64::new(0) })
    }

    pub fn publish(&self, ticket: ArtifactEnvelopeCompletedRecordTicket) -> Result<(), ArtifactEnvelopeCompletedRecordTicket> {
        if self.generation.compare_exchange(0, ticket.generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err(ticket);
        }
        self.index.store(ticket.index, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn ticket(&self) -> Option<ArtifactEnvelopeCompletedRecordTicket> {
        let index = self.index.load(std::sync::atomic::Ordering::Acquire);
        let generation = self.generation.load(std::sync::atomic::Ordering::Acquire);
        (index != u16::MAX && generation != 0).then_some(ArtifactEnvelopeCompletedRecordTicket { index, generation })
    }
}

pub const ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: 1, key: "initialPack", required: true },
    OwnedSchemaFieldSpec { id: 2, key: "edits", required: true },
    OwnedSchemaFieldSpec { id: 3, key: "changes", required: true },
    OwnedSchemaFieldSpec { id: 4, key: "checkpoints", required: true },
    OwnedSchemaFieldSpec { id: 5, key: "alternatives", required: true },
];

pub struct ArtifactEnvelopeFreshSnapshotTarget<P> {
    pub next_generation: u64,
    pub reserved: Option<ArtifactEnvelopeFieldReservation>,
    pub value: Option<P>,
}

impl<P> ArtifactEnvelopeFreshSnapshotTarget<P> {
    pub fn new() -> Self {
        Self { next_generation: 0, reserved: None, value: None }
    }
}

impl<P: Send> ArtifactEnvelopeSnapshotFieldTarget<P> for ArtifactEnvelopeFreshSnapshotTarget<P> {
    fn reserve_snapshot(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic> {
        if self.reserved.is_some() || self.value.is_some() {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-duplicate", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-generation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let reservation = ArtifactEnvelopeFieldReservation::new(1, self.next_generation).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-invalid", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        self.reserved = Some(reservation);
        Ok(reservation)
    }

    fn publish_snapshot_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: P) {
        assert_eq!(self.reserved, Some(reservation), "snapshot publication requires its exact live reservation");
        assert!(self.value.is_none(), "snapshot publication cannot replace an established owner");
        self.reserved = None;
        self.value = Some(value);
    }

    fn cancel_snapshot_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.reserved != Some(reservation) {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.snapshot-reservation-stale", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.reserved = None;
        Ok(())
    }
}

pub enum ArtifactEnvelopeFreshVcsActive<Mutation> {
    Snapshot { reservation: ArtifactEnvelopeFieldReservation, publishing: bool },
    Edits(OwnedSchemaBoundedArrayAuthority<Edit<Mutation>>),
    Changes(OwnedSchemaBoundedArrayAuthority<Change>),
    Checkpoints(OwnedSchemaBoundedArrayAuthority<Checkpoint>),
    Alternatives(OwnedSchemaBoundedArrayAuthority<Alternative>),
}

/// 🌱️ Concrete repository VCS authority for a paged envelope. It accepts one
/// owner-supplied snapshot and four fixed-capacity, one-entry-at-a-time history arrays.
pub struct ArtifactEnvelopeFreshVcsAuthority<P: Send + Sync + 'static, Mutation: Send + 'static> {
    pub cursor: OwnedSchemaNestedRecordCursor,
    pub snapshot: Option<Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>>,
    pub snapshot_target: ArtifactEnvelopeFreshSnapshotTarget<P>,
    pub genesis_pack: Option<ArtifactGenesisPackCapture>,
    pub genesis_bytes: Option<Vec<u8>>,
    pub active: Option<ArtifactEnvelopeFreshVcsActive<Mutation>>,
    pub pending: Option<(OwnedSchemaToken, bool)>,
    pub edits: Option<ArtifactHistoryLedger<Edit<Mutation>>>,
    pub changes: Option<ArtifactHistoryLedger<Change>>,
    pub checkpoints: Option<ArtifactHistoryLedger<Checkpoint>>,
    pub alternatives: Option<ArtifactHistoryLedger<Alternative>>,
    pub value: Option<ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>>,
    pub retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    pub initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    pub mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    pub edit_decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>>,
    pub maximum_snapshot_close_byte_demand: usize,
    pub maximum_retained_snapshot_close_bytes: usize,
    pub terminal: bool,
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeFreshVcsAuthority<P, Mutation> {
    pub fn try_new(
        snapshot: Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>,
        initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        edit_decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>>,
    ) -> Result<Self, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>> {
        let maximum_snapshot_close_byte_demand = snapshot.maximum_close_byte_demand();
        let maximum_retained_snapshot_close_bytes = snapshot.maximum_retained_close_bytes();
        if maximum_snapshot_close_byte_demand > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
            || maximum_retained_snapshot_close_bytes > ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
            || maximum_snapshot_close_byte_demand > maximum_retained_snapshot_close_bytes
        {
            return Err(snapshot);
        }
        let cursor = match OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS }) {
            Ok(cursor) => cursor,
            Err(_) => return Err(snapshot),
        };
        Ok(Self {
            cursor,
            snapshot: Some(snapshot),
            snapshot_target: ArtifactEnvelopeFreshSnapshotTarget::new(),
            genesis_pack: None,
            genesis_bytes: None,
            active: None,
            pending: None,
            edits: None,
            changes: None,
            checkpoints: None,
            alternatives: None,
            value: None,
            retirement: None,
            initial_snapshot_factory,
            mutation_factory,
            edit_decoder,
            maximum_snapshot_close_byte_demand,
            maximum_retained_snapshot_close_bytes,
            terminal: false,
        })
    }

    pub fn diagnostic(code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: OwnedSchemaPath::field("vcs").unwrap_or(OwnedSchemaPath::ROOT) , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    /// 🎯️ Routes one cursor-attributed token into the exact live nested owner. The owner is taken
    /// so the borrow checker admits the sibling-field writes, and EVERY exit restores it — an
    /// abandoned `?` here dropped a live history array and aborted the process
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn accept_active_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let Some(mut active) = self.active.take() else {
            return Err(Self::diagnostic("artifact-envelope.fresh-vcs-field"));
        };
        return match &mut active {
            ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, publishing } => {
                if cx.is_cancelled() { self.active = Some(active); return Err(Self::diagnostic("artifact-envelope.genesis-pack-cancelled")); }
                if cx.should_yield() { self.active = Some(active); return Ok(ArtifactEnvelopeFieldDecodeStep::Pending); }
                if !*publishing {
                    if self.genesis_pack.is_none() {
                        match ArtifactGenesisPackCapture::new(token, cx) { Ok(capture) => self.genesis_pack = Some(capture), Err(error) => { self.active = Some(active); return Err(error); } }
                    }
                    match self.genesis_pack.as_mut().expect("genesis capture remains retained").step(source, cx) {
                        Ok(true) => {},
                        Ok(false) => { self.active = Some(active); return Ok(ArtifactEnvelopeFieldDecodeStep::Pending); },
                        Err(error) => { self.active = Some(active); return Err(error); },
                    }
                }
                let Some(snapshot) = self.snapshot.as_mut() else {
                    self.active = Some(active);
                    return Err(Self::diagnostic("artifact-envelope.fresh-vcs-snapshot-owner"));
                };
                let step = if *publishing { snapshot.publish_reserved(&mut self.snapshot_target, *reservation, cx) } else { snapshot.accept_token(token, _terminal, source, cx) };
                let publishing_now = *publishing;
                match step {
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) if publishing_now => Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) if publishing_now => Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                        *publishing = true;
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete) => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.snapshot-published-record"))
                    }
                    Err(diagnostic) => {
                        self.active = Some(active);
                        Err(diagnostic)
                    }
                }
            }
            ArtifactEnvelopeFreshVcsActive::Edits(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.edits = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshVcsActive::Changes(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.changes = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshVcsActive::Checkpoints(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.checkpoints = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshVcsActive::Alternatives(authority) => match authority.accept(token, _terminal, source, cx) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => match authority.take_values() {
                    Some(values) => {
                        self.alternatives = Some(values);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("artifact-envelope.vcs-history-not-taken"))
                    }
                },
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
        };
    }

    /// 🌱️ Installs the exact owner for one vcs field the moment its own nested cursor attributes a
    /// token to it. Every later token of that field is routed to this owner with the CURSOR's
    /// per-field terminal flag, never the parent record's — the parent only ever marks the vcs
    /// object's own closing brace, so an inner array could never see its terminal `]`.
    fn begin_vcs_field(&mut self, field_id: u16) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if field_id == 1 {
            let reservation = self.snapshot_target.reserve_snapshot()?;
            self.active = Some(ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, publishing: false });
            return Ok(());
        }
        if !(2..=5).contains(&field_id) {
            return Err(Self::diagnostic("artifact-envelope.fresh-vcs-field"));
        }
        let path = OwnedSchemaPath::field(ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS[field_id as usize - 1].key).unwrap_or(OwnedSchemaPath::ROOT);
        self.active = Some(match field_id {
            2 => ArtifactEnvelopeFreshVcsActive::Edits(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreDecodedEditRetirementFactory { mutation_factory: self.mutation_factory.clone() }), self.edit_decoder.clone())),
            3 => ArtifactEnvelopeFreshVcsActive::Changes(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreChangeRetirementFactory), Arc::new(ArtifactRepositoryHistoryEntryDecoder::new()))),
            4 => ArtifactEnvelopeFreshVcsActive::Checkpoints(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreCheckpointRetirementFactory), Arc::new(ArtifactRepositoryHistoryEntryDecoder::new()))),
            _ => ArtifactEnvelopeFreshVcsActive::Alternatives(OwnedSchemaBoundedArrayAuthority::new(path, Arc::new(ArtifactStoreAlternativeRetirementFactory), Arc::new(ArtifactRepositoryHistoryEntryDecoder::new()))),
        });
        Ok(())
    }

    /// 🧾️ Assembles the decoded repository once the nested cursor reports the vcs record complete.
    fn finish_vcs_record(&mut self) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let initial_snapshot = self.snapshot_target.value.take().ok_or_else(|| Self::diagnostic("artifact-envelope.fresh-vcs-missing-snapshot"))?;
        let edits = self.edits.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-edits"))?;
        let changes = self.changes.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-changes"))?;
        let checkpoints = self.checkpoints.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-checkpoints"))?;
        let alternatives = self.alternatives.take().ok_or_else(|| Self::diagnostic("artifact-envelope.vcs-missing-alternatives"))?;
        let capture = self.genesis_pack.take().ok_or_else(|| Self::diagnostic("artifact-envelope.genesis-pack-missing"))?;
        let genesis = capture.admit(initial_snapshot);
        self.value = Some(ArtifactVcs { genesis, edits, changes, checkpoints, alternatives });
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn owners_terminal_empty(&self) -> bool {
        self.terminal
            && self.snapshot.is_none()
            && self.genesis_pack.is_none()
            && self.genesis_bytes.is_none()
            && self.snapshot_target.reserved.is_none()
            && self.snapshot_target.value.is_none()
            && self.active.is_none()
            && self.edits.is_none()
            && self.changes.is_none()
            && self.checkpoints.is_none()
            && self.alternatives.is_none()
            && self.value.is_none()
            && self.retirement.is_none()
    }

    /// 📏️ Observes the next close turn in the exact order `close_step` consumes owners: the unfinished snapshot,
    /// the retained child, the active history array, the four history ledgers, then the three original value owners.
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let refusal = |_: ValueError| Self::diagnostic("artifact-envelope.fresh-vcs-close-demand");
        let deeper = |depth: usize| depth.checked_add(1).ok_or_else(|| Self::diagnostic("artifact-envelope.fresh-vcs-close-depth-overflow"));
        let capacity = |capacity_bytes: usize, depth: usize| RetirementDemand { capacity_bytes, depth, ..Default::default() };
        if let Some(snapshot) = self.snapshot.as_ref() {
            if snapshot.terminal_is_empty() {
                return Ok(RetirementDemand { release_bytes: size_of_val(snapshot.as_ref()), depth: 1, ..Default::default() });
            }
            let demand = RetirementDemand {
                copy_bytes: snapshot.next_close_copy_byte_demand()?,
                capacity_bytes: snapshot.next_close_capacity_byte_demand(maximum_copy_bytes)?,
                release_bytes: snapshot.next_close_release_byte_demand()?,
                depth: deeper(snapshot.next_close_depth_demand()?)?,
            };
            if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_snapshot_close_byte_demand {
                return Err(Self::diagnostic("artifact-envelope.snapshot-close-demand-over-admitted-maximum"));
            }
            return Ok(demand);
        }
        let demand = if let Some(retirement) = self.retirement.as_ref() {
            artifact_retirement_box_demands(retirement, maximum_copy_bytes).map_err(refusal)?
        } else if let Some(active) = self.active.as_ref() {
            if active.terminal_is_empty() {
                RetirementDemand { depth: 1, ..Default::default() }
            } else {
                let demand = active.close_demands(maximum_copy_bytes)?;
                RetirementDemand { depth: deeper(demand.depth)?, ..demand }
            }
        } else if let Some(ledger) = self.alternatives.as_ref() {
            Self::ledger_demand::<Alternative, ArtifactStoreAlternativeRetirementFactory>(ledger).map_err(refusal)?
        } else if let Some(ledger) = self.checkpoints.as_ref() {
            Self::ledger_demand::<Checkpoint, ArtifactStoreCheckpointRetirementFactory>(ledger).map_err(refusal)?
        } else if let Some(ledger) = self.changes.as_ref() {
            Self::ledger_demand::<Change, ArtifactStoreChangeRetirementFactory>(ledger).map_err(refusal)?
        } else if let Some(ledger) = self.edits.as_ref() {
            Self::ledger_demand::<Edit<Mutation>, ArtifactStoreDecodedEditRetirementFactory<Mutation>>(ledger).map_err(refusal)?
        } else if self.value.is_some() {
            capacity(size_of::<ArtifactStoreVcsRetirement<P, Mutation>>(), 2)
        } else if self.genesis_pack.is_some() {
            RetirementDemand { depth: 1, ..Default::default() }
        } else if self.genesis_bytes.is_some() {
            artifact_retirement_owned_birth_demands(&self.genesis_bytes).map_err(refusal)?
        } else if let Some(snapshot) = self.snapshot_target.value.as_ref() {
            capacity(self.initial_snapshot_factory.retirement_birth_bytes(snapshot), 2)
        } else {
            RetirementDemand::default()
        };
        if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_close_byte_demand() {
            return Err(Self::diagnostic("artifact-envelope.fresh-vcs-close-demand-over-admitted-maximum"));
        }
        Ok(demand)
    }

    /// 📚️ Prices handing one original history ledger to its retained ledger cursor; an empty ledger is merely dropped.
    fn ledger_demand<T: Send + 'static, F: semio_framework_value::FactoryPayloadRetirement>(ledger: &ArtifactHistoryLedger<T>) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if ledger.terminal_is_empty() && ledger.backing_is_empty() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { capacity_bytes: artifact_history_ledger_birth::<T, F>()?, depth: 2, ..Default::default() })
    }

    /// 🤝️ Hands one original ledger to its retained cursor, or reports an empty ledger as one droppable item.
    fn hand_off_ledger<T: Send + 'static, F: ArtifactOwnedValueRetirementFactory<T> + semio_framework_value::FactoryPayloadRetirement>(ledger: &mut ArtifactHistoryLedger<T>, factory: F, slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if ledger.terminal_is_empty() && ledger.backing_is_empty() {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        artifact_history_ledger_handoff(ledger, factory, slot, grant)
    }
}

/// 📚️ Routes the four history-array closes through one exhaustive owner.
impl<Mutation: Send + 'static> ArtifactEnvelopeFreshVcsActive<Mutation> {
    pub fn terminal_is_empty(&self) -> bool {
        match self {
            Self::Snapshot { .. } => true,
            Self::Edits(authority) => authority.terminal_is_empty(),
            Self::Changes(authority) => authority.terminal_is_empty(),
            Self::Checkpoints(authority) => authority.terminal_is_empty(),
            Self::Alternatives(authority) => authority.terminal_is_empty(),
        }
    }

    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        match self {
            Self::Snapshot { .. } => Ok(Default::default()),
            Self::Edits(authority) => authority.close_demands(maximum_copy_bytes),
            Self::Changes(authority) => authority.close_demands(maximum_copy_bytes),
            Self::Checkpoints(authority) => authority.close_demands(maximum_copy_bytes),
            Self::Alternatives(authority) => authority.close_demands(maximum_copy_bytes),
        }
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        match self {
            Self::Snapshot { .. } => Ok(RetainedCloneStep::Complete(Default::default())),
            Self::Edits(authority) => authority.close_step(grant),
            Self::Changes(authority) => authority.close_step(grant),
            Self::Checkpoints(authority) => authority.close_step(grant),
            Self::Alternatives(authority) => authority.close_step(grant),
        }
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeVcsFieldAuthority<P, Mutation> for ArtifactEnvelopeFreshVcsAuthority<P, Mutation> {
    fn accept_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let (token, _terminal) = match self.pending {
            Some(pending) => pending,
            None => match self.cursor.accept(token, source) {
                OwnedSchemaNestedRecordStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal } => {
                    if self.active.is_none() {
                        self.begin_vcs_field(field_id)?;
                    }
                    (token, terminal)
                }
                OwnedSchemaNestedRecordStep::Complete => return self.finish_vcs_record(),
                OwnedSchemaNestedRecordStep::Fault(diagnostic) => return Err(diagnostic),
            },
        };
        self.pending = Some((token, _terminal));
        let outcome = self.accept_active_token(token, _terminal, source, cx);
        if !matches!(outcome, Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) {
            self.pending = None;
        }
        outcome
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeDecodedRecordTarget<P, Mutation>,
        reservation: ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if !self.snapshot.as_ref().is_some_and(|snapshot| snapshot.terminal_is_empty()) {
            return Err(Self::diagnostic("artifact-envelope.snapshot-publication-with-live-owner"));
        }
        drop(self.snapshot.take());
        let value = self.value.take().ok_or_else(|| Self::diagnostic("artifact-envelope.fresh-vcs-not-ready"))?;
        target.publish_vcs_reserved(reservation, value);
        self.terminal = true;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        self.maximum_snapshot_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES)
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        self.maximum_retained_snapshot_close_bytes + ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.depth)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        let empty = RetainedCloneProgress::default();
        let fault = |code: &'static str| Self::diagnostic(code);
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        self.pending = None;
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(fault("artifact-envelope.fresh-vcs-close-depth"));
        }
        if !artifact_retirement_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(snapshot) = self.snapshot.as_mut() {
            if !snapshot.terminal_is_empty() {
                let step = snapshot.close_step(child)?;
                if !step.progress().fits(child) {
                    return Err(fault("artifact-envelope.snapshot-close-over-budget").with_progress(step.progress()));
                }
                if matches!(step, RetainedCloneStep::Complete(_)) && !snapshot.terminal_is_empty() {
                    return Err(fault("artifact-envelope.snapshot-close-false-terminal").with_progress(step.progress()));
                }
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
            drop(self.snapshot.take());
            if let Some(ArtifactEnvelopeFreshVcsActive::Snapshot { reservation, .. }) = self.active.as_ref() {
                let reservation = *reservation;
                self.active = None;
                self.snapshot_target.cancel_snapshot_reservation(reservation).map_err(|diagnostic|diagnostic.with_progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..empty}))?;
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        if self.retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (fault("artifact-envelope.fresh-vcs-retirement-fault")).with_native(error));
        }
        if let Some(active) = self.active.as_mut() {
            if active.terminal_is_empty() {
                drop(self.active.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let step = active.close_step(child)?;
            if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !active.terminal_is_empty() {
                return Err(fault("artifact-envelope.history-close-false-terminal").with_progress(step.progress()));
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(ledger) = self.alternatives.as_mut() {
            let step = Self::hand_off_ledger(ledger, ArtifactStoreAlternativeRetirementFactory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.alternatives.take()); }
            return Ok(step);
        }
        if let Some(ledger) = self.checkpoints.as_mut() {
            let step = Self::hand_off_ledger(ledger, ArtifactStoreCheckpointRetirementFactory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.checkpoints.take()); }
            return Ok(step);
        }
        if let Some(ledger) = self.changes.as_mut() {
            let step = Self::hand_off_ledger(ledger, ArtifactStoreChangeRetirementFactory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.changes.take()); }
            return Ok(step);
        }
        if let Some(ledger) = self.edits.as_mut() {
            let factory = ArtifactStoreDecodedEditRetirementFactory { mutation_factory: self.mutation_factory.clone() };
            let step = Self::hand_off_ledger(ledger, factory, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.history-ledger-handoff")).with_native(error))?;
            if step.progress().copied_items != 0 { drop(self.edits.take()); }
            return Ok(step);
        }
        if let Some(vcs) = self.value.take() {
            let (snapshots, mutations) = (self.initial_snapshot_factory.clone(), self.mutation_factory.clone());
            return match admit_artifact_retirement(vcs, child, |vcs| ArtifactStoreVcsRetirement::new(vcs, snapshots, mutations)) {
                Ok((owner, progress)) => {
                    self.retirement = Some(owner);
                    if !progress.fits(child) { return Err(fault("artifact-envelope.fresh-vcs-retirement-receipt").with_progress(progress)); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, vcs)) => {
                    self.value = Some(vcs);
                    Err(fault("artifact-envelope.fresh-vcs-retirement-admission").with_native(error))
                }
            };
        }
        if let Some(capture) = self.genesis_pack.take() {
            self.genesis_bytes = Some(capture.bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.genesis_bytes.is_some() {
            return artifact_retirement_admit_owned(&mut self.genesis_bytes, &mut self.retirement, grant).map_err(|error| (fault("artifact-envelope.genesis-pack-retirement")).with_native(error));
        }
        if let Some(snapshot) = self.snapshot_target.value.take() {
            return match self.initial_snapshot_factory.retire_owned(snapshot, child) {
                Ok((owner, progress)) => {
                    self.retirement = Some(owner);
                    if !progress.fits(child) { return Err(fault("artifact-envelope.fresh-vcs-retirement-receipt")); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((_, snapshot)) => {
                    self.snapshot_target.value = Some(snapshot);
                    Err(fault("artifact-envelope.fresh-vcs-snapshot-retirement-admission"))
                }
            };
        }
        self.active = None;
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> Drop for ArtifactEnvelopeFreshVcsAuthority<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "fresh VCS authority reached Drop before every nested owner was published or cursor-retired");
    }
}

pub struct ArtifactEnvelopeFreshRecordTarget<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    pub next_generation: u64,
    pub reserved: Option<ArtifactEnvelopeFieldReservation>,
    pub vcs: Option<ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>>,
}

impl<P: Send, Mutation: Send> ArtifactEnvelopeFreshRecordTarget<P, Mutation> {
    fn new() -> Self {
        Self { next_generation: 0, reserved: None, vcs: None }
    }
}

impl<P: Send + Sync, Mutation: Send> ArtifactEnvelopeDecodedRecordTarget<P, Mutation> for ArtifactEnvelopeFreshRecordTarget<P, Mutation> {
    fn reserve_field(&mut self, field_id: u16) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic> {
        if field_id != ARTIFACT_ENVELOPE_VCS_FIELD || self.reserved.is_some() || self.vcs.is_some() {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-reservation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-generation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let reservation = ArtifactEnvelopeFieldReservation::new(field_id, self.next_generation).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-reservation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        self.reserved = Some(reservation);
        Ok(reservation)
    }

    fn publish_vcs_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>) {
        assert_eq!(self.reserved, Some(reservation), "VCS publication requires its exact live reservation");
        assert!(self.vcs.is_none(), "VCS publication cannot replace a live owner");
        self.reserved = None;
        self.vcs = Some(value);
    }

    fn publish_edit_messages_reserved(&mut self, _reservation: ArtifactEnvelopeFieldReservation, _value: ArtifactEditMessageLedger) {
        panic!("fresh envelope edit-message ledger is validated in place and never crosses the VCS target");
    }

    fn publish_conflicts_reserved(&mut self, _reservation: ArtifactEnvelopeFieldReservation, _value: Box<dyn ArtifactEnvelopeSprConflictAuthority>) {
        panic!("fresh envelope conflict ledger is validated in place and never crosses the VCS target");
    }

    fn cancel_field_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.reserved != Some(reservation) {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.fresh-field-reservation-stale", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.reserved = None;
        Ok(())
    }
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its admitted fixed string authority inline across bounded steps without a new allocation.")]
pub enum ArtifactEnvelopeFreshRecordActive<P, Mutation> {
    String { field_id: u16, authority: OwnedSchemaStringAuthority<256> },
    Vcs { reservation: ArtifactEnvelopeFieldReservation, authority: Box<dyn ArtifactEnvelopeVcsFieldAuthority<P, Mutation>>, publishing: bool },
    RejectedSnapshot { reservation: ArtifactEnvelopeFieldReservation, authority: Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>> },
    Empty { authority: OwnedSchemaEmptyArrayAuthority },
}

/// 📋️ Concrete 12-field catalog consumer for one fresh document envelope. All optional
/// fields must be absent or null; history/edit/conflict collections must be exact empty arrays.
pub struct ArtifactEnvelopeFreshFieldDecoder<P: Send + Sync + 'static, Mutation: Send + 'static> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    pub initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    pub mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    pub completed: Arc<ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>>,
    completion: Arc<ArtifactEnvelopeDecodeCompletion>,
    pub target: ArtifactEnvelopeFreshRecordTarget<P, Mutation>,
    pub schema: Option<String>,
    pub id: Option<String>,
    pub active: Option<ArtifactEnvelopeFreshRecordActive<P, Mutation>>,
    pending_completed: Option<Box<dyn ArtifactEnvelopeCompletedRecord<P, Mutation>>>,
    pub active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    maximum_vcs_close_byte_demand: usize,
    maximum_retained_vcs_close_bytes: usize,
    pub terminal: bool,
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeFreshFieldDecoder<P, Mutation> {
    pub fn new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
        initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        completed: Arc<ArtifactEnvelopeCompletedRecordRegistry<P, Mutation>>,
        completion: Arc<ArtifactEnvelopeDecodeCompletion>,
    ) -> Self {
        let maximum_vcs_close_byte_demand = catalog.maximum_vcs_close_byte_demand();
        let maximum_retained_vcs_close_bytes = catalog.maximum_retained_vcs_close_bytes();
        Self {
            operation,
            generation,
            catalog,
            initial_snapshot_factory,
            mutation_factory,
            completed,
            completion,
            target: ArtifactEnvelopeFreshRecordTarget::new(),
            schema: None,
            id: None,
            active: None,
            pending_completed: None,
            active_retirement: None,
            maximum_vcs_close_byte_demand,
            maximum_retained_vcs_close_bytes,
            terminal: false,
        }
    }

    pub fn diagnostic(field: &'static str, code: &'static str) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset: 0, line: 0, column: 0, path: OwnedSchemaPath::field(field).unwrap_or(OwnedSchemaPath::ROOT) , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    fn owners_terminal_empty(&self) -> bool {
        self.terminal && self.active.is_none() && self.pending_completed.is_none() && self.active_retirement.is_none() && self.target.reserved.is_none() && self.target.vcs.is_none() && self.schema.is_none() && self.id.is_none()
    }

    fn begin_field(&mut self, field_id: u16, token: OwnedSchemaToken) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        match field_id {
            ARTIFACT_ENVELOPE_SCHEMA_FIELD | ARTIFACT_ENVELOPE_ID_FIELD => {
                let field = if field_id == ARTIFACT_ENVELOPE_SCHEMA_FIELD { "schema" } else { "id" };
                let authority = OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, OwnedSchemaPath::field(field).unwrap_or(OwnedSchemaPath::ROOT)).map_err(|token| OwnedSchemaDecodeDiagnostic {
                    code: "artifact-envelope.expected-string",
                    offset: token.start,
                    line: 0,
                    column: 0,
                    path: OwnedSchemaPath::field(field).unwrap_or(OwnedSchemaPath::ROOT),
                 refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                self.active = Some(ArtifactEnvelopeFreshRecordActive::String { field_id, authority });
            }
            ARTIFACT_ENVELOPE_VCS_FIELD => {
                let reservation = self.target.reserve_field(field_id)?;
                match self.catalog.begin_vcs(self.operation, self.generation, OwnedSchemaPath::field("vcs").unwrap_or(OwnedSchemaPath::ROOT)) {
                    Ok(authority) => {
                        if authority.maximum_close_byte_demand() > self.maximum_vcs_close_byte_demand || authority.maximum_retained_close_bytes() > self.maximum_retained_vcs_close_bytes {
                            self.active = Some(ArtifactEnvelopeFreshRecordActive::Vcs { reservation, authority, publishing: false });
                            return Err(Self::diagnostic("vcs", "artifact-envelope.vcs-close-capacity-contract"));
                        }
                        self.active = Some(ArtifactEnvelopeFreshRecordActive::Vcs { reservation, authority, publishing: false });
                    }
                    Err(authority) => {
                        self.active = Some(ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { reservation, authority });
                        return Err(Self::diagnostic("vcs", "artifact-envelope.vcs-construction-capacity"));
                    }
                }
            }
            ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD => {
                self.active = Some(ArtifactEnvelopeFreshRecordActive::Empty { authority: OwnedSchemaEmptyArrayAuthority::new(OwnedSchemaPath::field("editMessages").unwrap_or(OwnedSchemaPath::ROOT)) });
            }
            ARTIFACT_ENVELOPE_CONFLICTS_FIELD => {
                self.active = Some(ArtifactEnvelopeFreshRecordActive::Empty { authority: OwnedSchemaEmptyArrayAuthority::new(OwnedSchemaPath::field("conflicts").unwrap_or(OwnedSchemaPath::ROOT)) });
            }
            _ => {
                if token.kind != OwnedSchemaTokenKind::Null {
                    return Err(Self::diagnostic("optional", "artifact-envelope.fresh-optional-field-must-be-null"));
                }
            }
        }
        Ok(())
    }

    /// 📏️ Observes the next close turn in the exact order `close_step` consumes owners: the unfinished field,
    /// the published completed record, the retained VCS child, the original VCS, then the identity strings.
    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let refusal = |_: ValueError| Self::diagnostic("$", "artifact-envelope.fresh-close-demand");
        let deeper = |depth: usize| depth.checked_add(1).ok_or_else(|| Self::diagnostic("$", "artifact-envelope.close-depth-overflow"));
        if let Some(active) = self.active.as_ref() {
            return Ok(match active {
                ArtifactEnvelopeFreshRecordActive::Vcs { authority, .. } if authority.terminal_is_empty() => RetirementDemand { release_bytes: size_of_val(authority.as_ref()), depth: 1, ..Default::default() },
                ArtifactEnvelopeFreshRecordActive::Vcs { authority, .. } => RetirementDemand {
                    copy_bytes: authority.next_close_copy_byte_demand()?,
                    capacity_bytes: authority.next_close_capacity_byte_demand(maximum_copy_bytes)?,
                    release_bytes: authority.next_close_release_byte_demand()?,
                    depth: deeper(authority.next_close_depth_demand()?)?,
                },
                ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { authority, .. } if authority.terminal_is_empty() => RetirementDemand { release_bytes: size_of_val(authority.as_ref()), depth: 1, ..Default::default() },
                ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { authority, .. } => RetirementDemand {
                    copy_bytes: authority.next_close_copy_byte_demand()?,
                    capacity_bytes: authority.next_close_capacity_byte_demand(maximum_copy_bytes)?,
                    release_bytes: authority.next_close_release_byte_demand()?,
                    depth: deeper(authority.next_close_depth_demand()?)?,
                },
                ArtifactEnvelopeFreshRecordActive::String { .. } | ArtifactEnvelopeFreshRecordActive::Empty { .. } => RetirementDemand { depth: 1, ..Default::default() },
            });
        }
        if let Some(owner) = self.pending_completed.as_ref() {
            if owner.terminal_is_empty() {
                return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() });
            }
            return Ok(RetirementDemand {
                copy_bytes: owner.next_close_copy_byte_demand().map_err(refusal)?,
                capacity_bytes: owner.next_close_capacity_byte_demand(maximum_copy_bytes).map_err(refusal)?,
                release_bytes: owner.next_close_release_byte_demand().map_err(refusal)?,
                depth: deeper(owner.next_close_depth_demand().map_err(refusal)?)?,
            });
        }
        if let Some(retirement) = self.active_retirement.as_ref() {
            let demand = artifact_retirement_box_demands(retirement, maximum_copy_bytes).map_err(refusal)?;
            if demand.copy_bytes.max(demand.capacity_bytes).max(demand.release_bytes) > self.maximum_close_byte_demand() {
                return Err(Self::diagnostic("vcs", "artifact-envelope.target-close-demand-over-admitted-maximum"));
            }
            return Ok(demand);
        }
        if self.target.vcs.is_some() {
            return Ok(RetirementDemand { capacity_bytes: size_of::<ArtifactStoreVcsRetirement<P, Mutation>>(), depth: 2, ..Default::default() });
        }
        let release_bytes = self.schema.as_ref().map_or(0, String::capacity).checked_add(self.id.as_ref().map_or(0, String::capacity)).ok_or_else(|| Self::diagnostic("$", "artifact-envelope.close-byte-demand-overflow"))?;
        Ok(RetirementDemand { release_bytes, depth: 1, ..Default::default() })
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactEnvelopeFieldDecoder<P, Mutation> for ArtifactEnvelopeFreshFieldDecoder<P, Mutation> {
    fn accept_field_token(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.active.is_none() {
            self.begin_field(field_id, token)?;
            if self.active.is_none() {
                return if terminal { Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) } else { Err(Self::diagnostic("optional", "artifact-envelope.fresh-null-not-terminal")) };
            }
        }
        let mut active = self.active.take().expect("fresh field authority was established");
        match &mut active {
            ArtifactEnvelopeFreshRecordActive::String { field_id, authority } => match authority.step(source, cx) {
                OwnedSchemaStringStep::Pending => {
                    self.active = Some(active);
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                OwnedSchemaStringStep::Complete => match authority.take_string() {
                    Some(value) => {
                        if *field_id == ARTIFACT_ENVELOPE_SCHEMA_FIELD {
                            self.schema = Some(value);
                        } else {
                            self.id = Some(value);
                        }
                        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
                    }
                    None => {
                        self.active = Some(active);
                        Err(Self::diagnostic("string", "artifact-envelope.string-publication"))
                    }
                },
                OwnedSchemaStringStep::Cancelled => {
                    self.active = Some(active);
                    Err(Self::diagnostic("string", "artifact-envelope.string-cancelled"))
                }
                OwnedSchemaStringStep::Fault(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
            ArtifactEnvelopeFreshRecordActive::Vcs { reservation, authority, publishing } => {
                let publishing_now = *publishing;
                let step = if publishing_now { authority.publish_reserved(&mut self.target, *reservation, cx) } else { authority.accept_token(token, terminal, source, cx) };
                match step {
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete | ArtifactEnvelopeFieldDecodeStep::TokenComplete) if publishing_now => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete) => {
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                        *publishing = true;
                        self.active = Some(active);
                        Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete) => {
                        self.active = Some(active);
                        Err(Self::diagnostic("vcs", "artifact-envelope.vcs-published-record"))
                    }
                    Err(diagnostic) => {
                        self.active = Some(active);
                        Err(diagnostic)
                    }
                }
            }
            ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { .. } => {
                self.active = Some(active);
                Err(Self::diagnostic("vcs", "artifact-envelope.vcs-construction-capacity"))
            }
            ArtifactEnvelopeFreshRecordActive::Empty { authority } => match authority.accept(token, terminal) {
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
                Ok(step) => {
                    self.active = Some(active);
                    Ok(step)
                }
                Err(diagnostic) => {
                    self.active = Some(active);
                    Err(diagnostic)
                }
            },
        }
    }

    fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.terminal {
            return Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete);
        }
        if self.pending_completed.is_none() {
            let schema = self.schema.take().ok_or_else(|| Self::diagnostic("schema", "artifact-envelope.missing-schema"))?;
            let id = self.id.take().ok_or_else(|| Self::diagnostic("id", "artifact-envelope.missing-id"))?;
            let vcs = self.target.vcs.take().ok_or_else(|| Self::diagnostic("vcs", "artifact-envelope.missing-vcs"))?;
            let envelope = ArtifactEnvelope::from_owners(ArtifactEnvelopeOwners {
                schema,
                id,
                vcs,
                backbone: None,
                active_alternative_id: None,
                cursor: None,
                dialect: None,
                migrated_from: None,
                owner: None,
                lanes: protocol::HistoryFoldIndex::new(),
                edit_messages: ArtifactEditMessageLedger::new(),
                conflicts: Vec::new(),
                transitions: Vec::new(),
                history_shape: semio_framework_os_spr::HistoryShape::Document,
                open_transaction: None,
                viewer_checkpoint_id: None,
            });
            self.pending_completed = Some(Box::new(ArtifactEnvelopeCompletedRecordOwner::new(envelope, Arc::clone(&self.initial_snapshot_factory), Arc::clone(&self.mutation_factory))));
        }
        let owner = self.pending_completed.take().expect("fresh completed owner remains retained");
        match self.completed.try_admit(owner) {
            Ok(ticket) => {
                if self.completion.publish(ticket).is_err() {
                    self.completed.try_request_close(ticket).map_err(|_| Self::diagnostic("$", "artifact-envelope.completion-publication"))?;
                    return Err(Self::diagnostic("$", "artifact-envelope.completion-duplicate"));
                }
                self.terminal = true;
                Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)
            }
            Err((_fault, owner)) => {
                self.pending_completed = Some(owner);
                Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
            }
        }
    }

    fn maximum_close_byte_demand(&self) -> usize {
        self.maximum_vcs_close_byte_demand.max(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        self.maximum_retained_vcs_close_bytes.saturating_add(ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(self.close_demands(0)?.depth)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(Self::diagnostic("vcs", "artifact-envelope.fresh-close-depth"));
        }
        if !artifact_retirement_grant_covers(grant, demand) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if let Some(active) = self.active.as_mut() {
            let mut released_bytes = 0;
            match active {
                ArtifactEnvelopeFreshRecordActive::String { authority, .. } => authority.cancel(),
                ArtifactEnvelopeFreshRecordActive::Vcs { authority, .. } => {
                    if !authority.terminal_is_empty() {
                        let step = authority.close_step(child)?;
                        if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !authority.terminal_is_empty() {
                            return Err(Self::diagnostic("vcs", "artifact-envelope.vcs-close-false-terminal").with_progress(step.progress()));
                        }
                        return Ok(RetainedCloneStep::Progress(step.progress()));
                    }
                    released_bytes = size_of_val(authority.as_ref());
                }
                ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { authority, .. } => {
                    if !authority.terminal_is_empty() {
                        let step = authority.close_step(child)?;
                        if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !authority.terminal_is_empty() {
                            return Err(Self::diagnostic("vcs", "artifact-envelope.snapshot-rejection-close-false-terminal").with_progress(step.progress()));
                        }
                        return Ok(RetainedCloneStep::Progress(step.progress()));
                    }
                    released_bytes = size_of_val(authority.as_ref());
                }
                ArtifactEnvelopeFreshRecordActive::Empty { .. } => {}
            }
            let reservation = match active {
                ArtifactEnvelopeFreshRecordActive::Vcs { reservation, .. } | ArtifactEnvelopeFreshRecordActive::RejectedSnapshot { reservation, .. } => Some(*reservation),
                _ => None,
            };
            self.active = None;
            if let Some(reservation) = reservation {
                self.target.cancel_field_reservation(reservation).map_err(|error|error.with_progress(RetainedCloneProgress{copied_items:1,released_bytes,..empty}))?;
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
        }
        if let Some(owner) = self.pending_completed.as_mut() {
            if owner.terminal_is_empty() {
                let released_bytes = size_of_val(owner.as_ref());
                drop(self.pending_completed.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
            }
            let step = owner.close_step(child).map_err(|error| (Self::diagnostic("$", "artifact-envelope.completed-close")).with_native(error))?;
            if !step.progress().fits(child) || matches!(step, RetainedCloneStep::Complete(_)) && !owner.terminal_is_empty() {
                return Err(Self::diagnostic("$", "artifact-envelope.completed-close-false-terminal").with_progress(step.progress()));
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.active_retirement.is_some() {
            return artifact_retirement_box_close_step(&mut self.active_retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (Self::diagnostic("vcs", "artifact-envelope.target-retirement")).with_native(error));
        }
        if let Some(vcs) = self.target.vcs.take() {
            let (snapshots, mutations) = (self.initial_snapshot_factory.clone(), self.mutation_factory.clone());
            return match admit_artifact_retirement(vcs, child, |vcs| ArtifactStoreVcsRetirement::new(vcs, snapshots, mutations)) {
                Ok((owner, progress)) => {
                    self.active_retirement = Some(owner);
                    if !progress.fits(child) { return Err(Self::diagnostic("vcs", "artifact-envelope.target-retirement-receipt").with_progress(progress)); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, vcs)) => {
                    self.target.vcs = Some(vcs);
                    Err(Self::diagnostic("vcs", "artifact-envelope.target-retirement-admission").with_native(error))
                }
            };
        }
        let released_bytes = demand.release_bytes;
        self.schema = None;
        self.id = None;
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners_terminal_empty()
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> Drop for ArtifactEnvelopeFreshFieldDecoder<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owners_terminal_empty()), "fresh envelope field decoder reached Drop before its completed record or rejected owners reached terminal empty");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeWholeBufferIngressError;

impl std::fmt::Display for ArtifactEnvelopeWholeBufferIngressError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("artifact envelope JSON must be submitted as fixed pages to the app-owned persistent decoder")
    }
}

impl std::error::Error for ArtifactEnvelopeWholeBufferIngressError {}

/// 🚫️ Greenfield fail-closure for deleted whole-buffer envelope ingress; callers must use the app-owned paged submission handle.
pub fn reject_whole_buffer_artifact_envelope_ingress<P, Mutation>(_input: &str) -> Result<ArtifactEnvelope<P, Mutation>, ArtifactEnvelopeWholeBufferIngressError> {
    Err(ArtifactEnvelopeWholeBufferIngressError)
}
