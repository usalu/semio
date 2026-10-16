//! 📥️ Items of `owned_schema_decode` that belong to `os-store-owned-schema-fields` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/📦️codec/🧬️owned-schema/🦀️.rs`.
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreOperationRowsRetirement;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::FromValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{Edit, Mutation};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use semio_framework_os_store_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_os_store_read::ArtifactStoreOperationRowOwners;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_os_vcs::ArtifactVcs;
use semio_framework_os_store_ledger::ArtifactEditMessageLedger;
use semio_framework_pack_json_retained_schema::*;
pub const ARTIFACT_ENVELOPE_SCHEMA_FIELD: u16 = 1;

pub const ARTIFACT_ENVELOPE_ID_FIELD: u16 = 2;

pub const ARTIFACT_ENVELOPE_VCS_FIELD: u16 = 3;

pub const ARTIFACT_ENVELOPE_BACKBONE_FIELD: u16 = 4;

pub const ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD: u16 = 5;

pub const ARTIFACT_ENVELOPE_CURSOR_FIELD: u16 = 6;

pub const ARTIFACT_ENVELOPE_DIALECT_FIELD: u16 = 7;

pub const ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD: u16 = 8;

pub const ARTIFACT_ENVELOPE_OWNER_FIELD: u16 = 9;

pub const ARTIFACT_ENVELOPE_LANES_FIELD: u16 = 10;

pub const ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD: u16 = 11;

pub const ARTIFACT_ENVELOPE_CONFLICTS_FIELD: u16 = 12;

pub const ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_SCHEMA_FIELD, key: "schema", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_ID_FIELD, key: "id", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_VCS_FIELD, key: "vcs", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_BACKBONE_FIELD, key: "backbone", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, key: "activeAlternativeId", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_CURSOR_FIELD, key: "cursor", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_DIALECT_FIELD, key: "dialect", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_MIGRATED_FROM_FIELD, key: "migratedFrom", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_OWNER_FIELD, key: "owner", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_LANES_FIELD, key: "lanes", required: false },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_EDIT_MESSAGES_FIELD, key: "editMessages", required: true },
    OwnedSchemaFieldSpec { id: ARTIFACT_ENVELOPE_CONFLICTS_FIELD, key: "conflicts", required: true },
];

/// 🧬️ Returns the sole field-ID/key contract for owned envelope decode.
pub fn artifact_envelope_owned_schema() -> OwnedSchemaRecordSpec {
    OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_OWNED_SCHEMA_FIELDS }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeFieldDecodeStep {
    /// The exact token remains owned by the field authority and must be offered again.
    Pending,
    /// The exact token was consumed, but the enclosing field still owns later tokens.
    TokenComplete,
    /// The exact token and its enclosing field were both completed.
    FieldComplete,
    RecordComplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeFieldReservation {
    field_id: u16,
    pub generation: u64,
}

impl ArtifactEnvelopeFieldReservation {
    pub fn new(field_id: u16, generation: u64) -> Option<Self> {
        (field_id != 0 && generation != 0).then_some(Self { field_id, generation })
    }

    pub fn field_id(self) -> u16 {
        self.field_id
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

/// 🎯️ Exact pre-admission target for one decoded envelope field.
pub trait ArtifactEnvelopeDecodedRecordTarget<P, Mutation>: Send {
    fn reserve_field(&mut self, field_id: u16) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic>;
    fn publish_vcs_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>);
    fn publish_edit_messages_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: ArtifactEditMessageLedger);
    fn publish_conflicts_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: Box<dyn ArtifactEnvelopeSprConflictAuthority>);
    fn cancel_field_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic>;
}

/// 🧬️ Terminalizable owned decoder for the nested VCS record.
pub trait ArtifactEnvelopeVcsFieldAuthority<P, Mutation>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeDecodedRecordTarget<P, Mutation>,
        reservation: ArtifactEnvelopeFieldReservation,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn maximum_close_byte_demand(&self) -> usize;
    fn maximum_retained_close_bytes(&self) -> usize;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪨️ Terminalizable owner-supplied decoder for a domain snapshot value.
pub trait ArtifactEnvelopeSnapshotFieldAuthority<P>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeSnapshotFieldTarget<P>,
        reservation: ArtifactEnvelopeFieldReservation,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn maximum_close_byte_demand(&self) -> usize;
    fn maximum_retained_close_bytes(&self) -> usize;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🪨️ Exact pre-admission target for a nested domain snapshot.
pub trait ArtifactEnvelopeSnapshotFieldTarget<P>: Send {
    fn reserve_snapshot(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic>;
    fn publish_snapshot_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: P);
    fn cancel_snapshot_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic>;
}

/// 🧬️ Terminalizable owner-supplied decoder for one domain mutation.
pub trait ArtifactEnvelopeMutationFieldAuthority<Mutation>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn publish_reserved(
        &mut self,
        target: &mut dyn ArtifactEnvelopeMutationFieldTarget<Mutation>,
        reservation: ArtifactEnvelopeFieldReservation,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🧬️ Exact pre-admission target for one nested domain mutation.
pub trait ArtifactEnvelopeMutationFieldTarget<Mutation>: Send {
    fn reserve_mutation(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic>;
    fn publish_mutation_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: Mutation);
    fn cancel_mutation_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic>;
}

/// ⚔️ Terminalizable repository-owned SPR conflict record authority.
pub trait ArtifactEnvelopeSprConflictAuthority: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏭️ Mandatory domain catalog for every nested envelope decoder; there is no default implementation.
pub trait ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn begin_vcs(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
    ) -> Result<Box<dyn ArtifactEnvelopeVcsFieldAuthority<P, Mutation>>, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>>;
    fn maximum_vcs_close_byte_demand(&self) -> usize;
    fn maximum_retained_vcs_close_bytes(&self) -> usize;
    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>;
    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeMutationFieldAuthority<Mutation>>;
    fn begin_spr_conflict(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSprConflictAuthority>;
    fn edit_history_decoder(&self) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>>;
}

/// 🧬️ Owner-supplied decoder for one pre-admitted history entry. The framework
/// never invokes `DeserializeOwned` for a domain edit behind the catalog's back.
pub trait ArtifactOwnedHistoryEntryDecoder<T>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<T>>;
}

/// 🧬 Terminalizable schema cursor for one pre-reserved history entry.
pub trait ArtifactOwnedHistoryEntryAuthority<T>: Send {
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn take_value(&mut self) -> Option<T>;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactRepositoryHistoryEntryDecoder<T>(PhantomData<fn() -> T>);

impl<T> ArtifactRepositoryHistoryEntryDecoder<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

/// 🧱 Retained source-bounded repository decoder used only by cohorts whose domain cursor
/// remains explicitly fail-closed pending a schema-owned implementation.
pub fn artifact_bounded_history_entry_decoder<T>() -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<T>>
where
    T: FromValue + Send + Sync + 'static,
{
    Arc::new(ArtifactRepositoryHistoryEntryDecoder::new())
}

pub const ARTIFACT_OWNED_SPR_EDIT_FIELDS: &[OwnedSchemaFieldSpec] = &[
    OwnedSchemaFieldSpec { id: 1, key: "id", required: true },
    OwnedSchemaFieldSpec { id: 2, key: "actor", required: false },
    OwnedSchemaFieldSpec { id: 3, key: "forwards", required: true },
    OwnedSchemaFieldSpec { id: 4, key: "inverse", required: true },
    OwnedSchemaFieldSpec { id: 5, key: "mutationMeta", required: false },
    OwnedSchemaFieldSpec { id: 8, key: "sequenceNumber", required: true },
    OwnedSchemaFieldSpec { id: 9, key: "startedAt", required: true },
    OwnedSchemaFieldSpec { id: 10, key: "finishedAt", required: false },
    OwnedSchemaFieldSpec { id: 11, key: "verb", required: false },
    OwnedSchemaFieldSpec { id: 12, key: "line", required: true },
];

pub struct ArtifactOwnedSprMutationTarget<Mutation>
where
    Mutation: Send,
{
    pub next_generation: u64,
    pub reservation: Option<ArtifactEnvelopeFieldReservation>,
    pub value: std::mem::ManuallyDrop<Option<Mutation>>,
}

impl<Mutation: Send> ArtifactOwnedSprMutationTarget<Mutation> {
    fn new() -> Self {
        Self { next_generation: 0, reservation: None, value: std::mem::ManuallyDrop::new(None) }
    }
}

impl<Mutation: Send> ArtifactEnvelopeMutationFieldTarget<Mutation> for ArtifactOwnedSprMutationTarget<Mutation> {
    fn reserve_mutation(&mut self) -> Result<ArtifactEnvelopeFieldReservation, OwnedSchemaDecodeDiagnostic> {
        if self.reservation.is_some() || self.value.is_some() {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-occupied", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-generation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let reservation = ArtifactEnvelopeFieldReservation::new(1, self.next_generation).ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-reservation", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        self.reservation = Some(reservation);
        Ok(reservation)
    }

    fn publish_mutation_reserved(&mut self, reservation: ArtifactEnvelopeFieldReservation, value: Mutation) {
        assert_eq!(self.reservation, Some(reservation), "SPR mutation publication requires its exact reservation");
        assert!(self.value.is_none(), "SPR mutation publication cannot replace an owner");
        self.reservation = None;
        *self.value = Some(value);
    }

    fn cancel_mutation_reservation(&mut self, reservation: ArtifactEnvelopeFieldReservation) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.reservation != Some(reservation) {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-target-stale", offset: 0, line: 0, column: 0, path: OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.reservation = None;
        Ok(())
    }
}

impl<Mutation: Send> Drop for ArtifactOwnedSprMutationTarget<Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.reservation.is_none() && self.value.is_none()), "SPR mutation target reached Drop with a live reservation or value owner");
    }
}

pub enum ArtifactOwnedSprMutationArrayState {
    AwaitStart,
    Entries,
    Publishing,
    Complete,
    Closing,
}

pub struct ArtifactOwnedSprMutationArrayAuthority<P, Mutation: Send> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub path: OwnedSchemaPath,
    pub catalog: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>>>,
    pub mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub factories_close: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
    rows_retirement: std::mem::ManuallyDrop<Option<ArtifactStoreOperationRowsRetirement<Mutation>>>,
    pub target: ArtifactOwnedSprMutationTarget<Mutation>,
    pub reservation: Option<ArtifactEnvelopeFieldReservation>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeMutationFieldAuthority<Mutation>>>>,
    pub values: std::mem::ManuallyDrop<Option<ArtifactStoreOperationRowOwners<Mutation>>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub depth: usize,
    scalar_entry: bool,
    expect_entry: bool,
    pub state: ArtifactOwnedSprMutationArrayState,
    pub taken: bool,
}

impl<P, Mutation: Send + 'static> ArtifactOwnedSprMutationArrayAuthority<P, Mutation> {
    pub fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let diagnostic = self.diagnostic("artifact-spr.mutation-array-close-demand", 0);
        if let Some(owner) = self.active.as_ref() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(copy)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()?.checked_add(1).ok_or(diagnostic)? });
        }
        if self.reservation.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(value) = self.target.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.mutation_factory.as_ref().expect("mutation target retains its factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy).map_err(|_| diagnostic); }
        if self.values.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<ArtifactStoreOperationRowOwners<Mutation>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.rows_retirement.as_ref() {
            let mut demand = owner.demands(copy).map_err(|_| diagnostic)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if self.catalog.is_some() || self.mutation_factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factories_close.iter().flatten().next() {
            let mut demand = owner.demands(copy).map_err(|_| diagnostic)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
    fn try_new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        inverse: bool,
    ) -> Result<Self, OwnedSchemaDecodeDiagnostic> {
        let values = if inverse { ArtifactStoreOperationRowOwners::Inverse(Default::default()) } else {
            let mut values = Vec::new();
            values.try_reserve_exact(ARTIFACT_ENVELOPE_HISTORY_ITEMS).map_err(|_| OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-admission", offset: 0, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
            ArtifactStoreOperationRowOwners::Forwards(values)
        };
        Ok(Self {
            operation,
            generation,
            path,
            catalog: std::mem::ManuallyDrop::new(Some(catalog)),
            mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)),
            factories_close: std::mem::ManuallyDrop::new(std::array::from_fn(|_| None)),
            rows_retirement: std::mem::ManuallyDrop::new(None),
            target: ArtifactOwnedSprMutationTarget::new(),
            reservation: None,
            active: std::mem::ManuallyDrop::new(None),
            values: std::mem::ManuallyDrop::new(Some(values)),
            retirement: std::mem::ManuallyDrop::new(None),
            depth: 0,
            scalar_entry: false,
            expect_entry: true,
            state: ArtifactOwnedSprMutationArrayState::AwaitStart,
            taken: false,
        })
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    pub fn accept(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-spr.mutation-array-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-spr.mutation-array-cancelled", token.start));
        }
        if matches!(self.state, ArtifactOwnedSprMutationArrayState::Publishing) {
            if let Some(ArtifactStoreOperationRowOwners::Inverse(values)) = self.values.as_mut() {
                if !values.has_reserved_slot() {
                    let diagnostic = OwnedSchemaDecodeDiagnostic { code: "artifact-spr.inverse-page-admission", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
                    let demand = values.next_allocation_bytes().map_err(|_| diagnostic)?;
                    if demand > ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES { return Err(diagnostic); }
                    let progress = values.reserve_one(demand).map_err(|_| diagnostic)?;
                    cx.consume_fuel(progress.allocated_bytes.max(1) as u64);
                    return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
                }
            }
            let reservation = self.reservation.ok_or_else(|| self.diagnostic("artifact-spr.mutation-array-reservation-missing", token.start))?;
            let path = self.path;
            let active = self.active.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-owner-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
            return match active.publish_reserved(&mut self.target, reservation, cx)? {
                ArtifactEnvelopeFieldDecodeStep::Pending => Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                ArtifactEnvelopeFieldDecodeStep::FieldComplete | ArtifactEnvelopeFieldDecodeStep::TokenComplete => {
                    self.reservation = None;
                    let value = self.target.value.take().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-value-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                    let values = self.values.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.mutation-array-values-missing", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                    match values {
                        ArtifactStoreOperationRowOwners::Forwards(values) => {
                            if values.len() == values.capacity() {
                                *self.target.value = Some(value);
                                return Err(self.diagnostic("artifact-spr.mutation-array-capacity", token.start));
                            }
                            values.push(value);
                        }
                        ArtifactStoreOperationRowOwners::Inverse(values) => {
                            if let Err(value) = values.push_reserved(value) {
                                *self.target.value = Some(value);
                                return Err(self.diagnostic("artifact-spr.inverse-page-reservation", token.start));
                            }
                        }
                    }
                    if !active.terminal_is_empty() {
                        return Err(self.diagnostic("artifact-spr.mutation-array-live-after-publish", token.start));
                    }
                    drop(self.active.take());
                    self.scalar_entry = false;
                    self.expect_entry = false;
                    self.state = ArtifactOwnedSprMutationArrayState::Entries;
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                ArtifactEnvelopeFieldDecodeStep::RecordComplete => Err(self.diagnostic("artifact-spr.mutation-array-record-terminal", token.start)),
            };
        }
        if let Some(active) = self.active.as_mut() {
            let entry_terminal = self.scalar_entry || (self.depth == 1 && matches!(token.kind, OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd));
            return match active.accept_token(token, entry_terminal, source, cx)? {
                ArtifactEnvelopeFieldDecodeStep::Pending => Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                ArtifactEnvelopeFieldDecodeStep::FieldComplete if entry_terminal => {
                    self.depth = 0;
                    self.state = ArtifactOwnedSprMutationArrayState::Publishing;
                    Ok(ArtifactEnvelopeFieldDecodeStep::Pending)
                }
                ArtifactEnvelopeFieldDecodeStep::TokenComplete if !entry_terminal => {
                    match token.kind {
                        OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => self.depth += 1,
                        OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => self.depth -= 1,
                        _ => {}
                    }
                    Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
                }
                _ => Err(self.diagnostic("artifact-spr.mutation-array-terminal-discipline", token.start)),
            };
        }
        match self.state {
            ArtifactOwnedSprMutationArrayState::AwaitStart if token.kind == OwnedSchemaTokenKind::ArrayStart && !terminal => {
                self.expect_entry = true;
                self.state = ArtifactOwnedSprMutationArrayState::Entries;
                Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if token.kind == OwnedSchemaTokenKind::ArrayEnd && terminal && self.values.as_ref().is_some_and(|values| match values { ArtifactStoreOperationRowOwners::Forwards(values) => values.is_empty(), ArtifactStoreOperationRowOwners::Inverse(values) => values.is_empty() }) => {
                self.state = ArtifactOwnedSprMutationArrayState::Complete;
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if token.kind == OwnedSchemaTokenKind::ArrayEnd && terminal && !self.expect_entry => {
                self.state = ArtifactOwnedSprMutationArrayState::Complete;
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if token.kind == OwnedSchemaTokenKind::Comma && !self.expect_entry => {
                self.expect_entry = true;
                Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
            }
            ArtifactOwnedSprMutationArrayState::Entries if self.expect_entry && matches!(token.kind, OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart | OwnedSchemaTokenKind::String) => {
                let reservation = self.target.reserve_mutation()?;
                self.reservation = Some(reservation);
                self.scalar_entry = token.kind == OwnedSchemaTokenKind::String;
                *self.active = Some(self.catalog.as_ref().expect("live original mutation catalog").begin_mutation(self.operation, self.generation, self.path));
                self.accept(token, self.scalar_entry, source, cx)
            }
            ArtifactOwnedSprMutationArrayState::AwaitStart => Err(self.diagnostic("artifact-spr.mutation-array-start", token.start)),
            ArtifactOwnedSprMutationArrayState::Entries => Err(self.diagnostic("artifact-spr.mutation-array-entry", token.start)),
            ArtifactOwnedSprMutationArrayState::Complete => Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete),
            ArtifactOwnedSprMutationArrayState::Publishing => unreachable!("publishing handled before token admission"),
            ArtifactOwnedSprMutationArrayState::Closing => Err(self.diagnostic("artifact-spr.mutation-array-closing", token.start)),
        }
    }

    pub fn take_values(&mut self) -> Option<ArtifactStoreOperationRowOwners<Mutation>> {
        if !matches!(self.state, ArtifactOwnedSprMutationArrayState::Complete) || self.active.is_some() || self.reservation.is_some() || self.taken {
            return None;
        }
        self.taken = true;
        self.values.take()
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close};
        let diagnostic = self.diagnostic("artifact-spr.mutation-array-close-authority", 0);
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(diagnostic); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.state = ArtifactOwnedSprMutationArrayState::Closing;
        if let Some(owner) = self.active.as_mut() {
            if owner.terminal_is_empty() { drop(self.active.take()); self.depth = 0; self.scalar_entry = false; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child)?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original SPR mutation decoder").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(reservation) = self.reservation {
            self.target.cancel_mutation_reservation(reservation)?;
            self.reservation = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(value) = self.target.value.take() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            return match self.mutation_factory.as_ref().expect("mutation target retains original factory").retire_owned(value, child) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(diagnostic.with_progress(progress)); } Ok(RetainedCloneStep::Progress(progress)) }
                Err((error, value)) => { *self.target.value = Some(value); Err(diagnostic.with_native(error)) }
            };
        }
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (diagnostic).with_native(error)); }
        if let Some(values) = self.values.take() {
            *self.rows_retirement = Some(ArtifactStoreOperationRowsRetirement::new(values, self.mutation_factory.as_ref().expect("original operation rows retain their factory").clone()));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(owner) = self.rows_retirement.as_mut() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.close_step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original SPR row owner").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(self.rows_retirement.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(factory) = self.mutation_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[0] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(factory) = self.catalog.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factories_close[1] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original mutation factory");
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.step(child).map_err(|error| (diagnostic).with_native(error))?;
            admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original SPR mutation factory").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if owner.terminal_is_empty() { drop(slot.take()); }
            self.taken = self.factories_close.iter().all(Option::is_none);
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        self.taken = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.taken && self.active.is_none() && self.reservation.is_none() && self.target.reservation.is_none() && self.target.value.is_none() && self.values.is_none() && self.retirement.is_none() && self.rows_retirement.is_none() && self.catalog.is_none() && self.mutation_factory.is_none() && self.factories_close.iter().all(Option::is_none)
    }
}

impl<P, Mutation: Send> Drop for ArtifactOwnedSprMutationArrayAuthority<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.taken && self.active.is_none() && self.reservation.is_none() && self.target.reservation.is_none() && self.target.value.is_none() && self.values.is_none() && self.retirement.is_none() && self.rows_retirement.is_none() && self.catalog.is_none() && self.mutation_factory.is_none() && self.factories_close.iter().all(Option::is_none)), "SPR mutation array reached Drop before every exact mutation owner was published or cursor-retired");
    }
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its admitted fixed string authority inline across bounded steps without a new allocation.")]
pub enum ArtifactOwnedSprEditActive<P, Mutation: Send> {
    String { field_id: u16, authority: OwnedSchemaStringAuthority<ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES> },
    Mutations { field_id: u16, authority: ArtifactOwnedSprMutationArrayAuthority<P, Mutation> },
    EmptyMetadata(OwnedSchemaEmptyArrayAuthority),
}

pub struct ArtifactOwnedSprEditAuthority<P, Mutation>
where
    P: Send + Sync + 'static,
    Mutation: Send + 'static,
{
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub path: OwnedSchemaPath,
    pub catalog: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>>>,
    pub mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>>,
    pub cursor: OwnedSchemaNestedRecordCursor,
    pub pending: Option<(OwnedSchemaToken, bool)>,
    pub active: std::mem::ManuallyDrop<Option<ArtifactOwnedSprEditActive<P, Mutation>>>,
    completed_arrays: [std::mem::ManuallyDrop<Option<ArtifactOwnedSprMutationArrayAuthority<P, Mutation>>>; 2],
    pub factories_close: [Option<semio_framework_value::FactoryAuthority>; 3],
    pub strings: [std::mem::ManuallyDrop<Option<String>>; 6],
    pub actor: std::mem::ManuallyDrop<Option<semio_framework_value::SharedUtf8>>,
    pub actor_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
    pub forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,
    pub inverse: std::mem::ManuallyDrop<Option<semio_framework_value::list::PagedList<Mutation, {usize::MAX}>>>,
    pub sequence_number: Option<i32>,
    pub value: std::mem::ManuallyDrop<Option<Edit<Mutation>>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    close_string: usize,
    pub terminal: bool,
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactOwnedSprEditAuthority<P, Mutation> {
    pub fn new(
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
        mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
    ) -> Self {
        Self {
            operation,
            generation,
            path,
            catalog: std::mem::ManuallyDrop::new(Some(catalog)),
            mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)),
            retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)),
            cursor: OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_OWNED_SPR_EDIT_FIELDS }).expect("SPR edit schema is a validated static catalog"),
            pending: None,
            active: std::mem::ManuallyDrop::new(None),
            completed_arrays: std::array::from_fn(|_| std::mem::ManuallyDrop::new(None)),
            factories_close: std::array::from_fn(|_| None),
            strings: std::array::from_fn(|_| std::mem::ManuallyDrop::new(None)),
            actor: std::mem::ManuallyDrop::new(None),
            actor_close: None,
            forwards: std::mem::ManuallyDrop::new(None),
            inverse: std::mem::ManuallyDrop::new(None),
            sequence_number: None,
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            close_string: 0,
            terminal: false,
        }
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    /// 🌱️ Installs the exact owner for one edit field, or finishes a scalar field inline. Returns
    /// `Some(step)` when the field needed no owner at all.
    fn begin_edit_field(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor) -> Result<Option<ArtifactEnvelopeFieldDecodeStep>, OwnedSchemaDecodeDiagnostic> {
        if Self::string_index(field_id).is_some() {
            if token.kind == OwnedSchemaTokenKind::Null && matches!(field_id, 2 | 10 | 11 | 12) {
                return Ok(Some(ArtifactEnvelopeFieldDecodeStep::TokenComplete));
            }
            if !terminal {
                return Err(self.diagnostic("artifact-spr.edit-string-scalar", token.start));
            }
            let authority = OwnedSchemaStringAuthority::try_new(self.operation, self.generation, token, self.path).map_err(|token| self.diagnostic("artifact-spr.edit-string", token.start))?;
            *self.active = Some(ArtifactOwnedSprEditActive::String { field_id, authority });
            return Ok(None);
        }
        if matches!(field_id, 3 | 4) {
            let authority = ArtifactOwnedSprMutationArrayAuthority::try_new(self.operation, self.generation, self.path, self.catalog.as_ref().expect("live original edit catalog").clone(), self.mutation_factory.as_ref().expect("live original edit mutation factory").clone(), field_id == 4)?;
            *self.active = Some(ArtifactOwnedSprEditActive::Mutations { field_id, authority });
            return Ok(None);
        }
        if field_id == 5 {
            *self.active = Some(ArtifactOwnedSprEditActive::EmptyMetadata(OwnedSchemaEmptyArrayAuthority::new(self.path)));
            return Ok(None);
        }
        if field_id == 8 && terminal && token.kind == OwnedSchemaTokenKind::Number {
            let mut bytes = [0u8; 64];
            let len = usize::try_from(token.end.saturating_sub(token.start)).unwrap_or(usize::MAX);
            if len == 0 || len > bytes.len() || source.copy_token_bytes(token, 0, &mut bytes[..len]) != len {
                return Err(self.diagnostic("artifact-spr.edit-sequence-token", token.start));
            }
            self.sequence_number = std::str::from_utf8(&bytes[..len]).ok().and_then(|value| value.parse().ok());
            if self.sequence_number.is_none() {
                return Err(self.diagnostic("artifact-spr.edit-sequence-value", token.start));
            }
            return Ok(Some(ArtifactEnvelopeFieldDecodeStep::TokenComplete));
        }
        Err(self.diagnostic("artifact-spr.edit-field", token.start))
    }

    /// 🎯️ Routes one cursor-attributed token into the live edit-field owner.
    fn accept_active_edit_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let Some(active) = self.active.as_mut() else {
            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-field", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        };
        match active {
            ArtifactOwnedSprEditActive::String { field_id, authority } => match authority.step(source, cx) {
                OwnedSchemaStringStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::Pending),
                OwnedSchemaStringStep::Complete => {
                    let Some(index) = Self::string_index(*field_id) else {
                        return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-string-field", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                    };
                    if index==1 {
                        let grant=cx.retained_grant();
                        if self.strings[1].is_none(){
                            let original=authority.as_str().ok_or(OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-text",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                            let copied_bytes=original.len().checked_add(size_of::<Option<String>>()).ok_or(OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-overflow",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
                            if grant.maximum_items==0||grant.maximum_depth==0||copied_bytes>grant.maximum_copy_bytes||original.len()>grant.maximum_capacity_bytes{return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                            let mut value=String::new();value.try_reserve_exact(original.len()).map_err(|_|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-allocation",offset:token.start,line:0,column:0,path, refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;value.push_str(original);
                            let receipt=RetainedCloneProgress{copied_items:1,copied_bytes,retained_capacity_bytes:value.capacity(),released_bytes:0};
                            *self.strings[1]=Some(value);cx.consume_retained(receipt).map_err(|error|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-receipt",offset:token.start,line:0,column:0,path,refusal_kind:error.kind,retained_progress:receipt})?;
                            return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);
                        }
                        let placement=size_of::<Option<semio_framework_value::SharedUtf8>>();
                        if grant.maximum_copy_bytes<placement{return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-placement,..grant};
                        match semio_framework_value::SharedUtf8::admit(self.strings[1].take().expect("original actor text retained"),child){
                            Ok((original,mut receipt))=>{receipt.copied_bytes+=placement;*self.actor=Some(original);cx.consume_retained(receipt).map_err(|error|OwnedSchemaDecodeDiagnostic{code:"artifact-spr.edit-actor-receipt",offset:token.start,line:0,column:0,path,refusal_kind:error.kind,retained_progress:receipt})?;}
                            Err((_,original))=>{*self.strings[1]=Some(original);return Ok(ArtifactEnvelopeFieldDecodeStep::Pending);}
                        }
                    }else{*self.strings[index] = authority.take_string();}
                }
                OwnedSchemaStringStep::Cancelled => return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-string-cancelled", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }),
                OwnedSchemaStringStep::Fault(diagnostic) => return Err(diagnostic),
            },
            ArtifactOwnedSprEditActive::Mutations { field_id, authority } => match authority.accept(token, terminal, source, cx) {
                Err(diagnostic) => return Err(diagnostic),
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {
                    let index = match *field_id { 3 => 0, 4 => 1, _ => return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-native-array-owner", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }) };
                    if self.completed_arrays[index].is_some() { return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-array-custody", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }); }
                    let Some(values) = authority.take_values() else {
                        return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-mutation-values", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                    };
                    match (*field_id, values) {
                        (3, ArtifactStoreOperationRowOwners::Forwards(values)) => *self.forwards = Some(values),
                        (4, ArtifactStoreOperationRowOwners::Inverse(values)) => *self.inverse = Some(values),
                        (_, values) => {
                            *authority.values = Some(values);
                            authority.taken = false;
                            return Err(OwnedSchemaDecodeDiagnostic { code: "artifact-spr.edit-native-array-owner", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
                        }
                    }
                }
                Ok(step) => return Ok(step),
            },
            ArtifactOwnedSprEditActive::EmptyMetadata(authority) => match authority.accept(token, terminal) {
                Err(diagnostic) => return Err(diagnostic),
                Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete) => {}
                Ok(step) => return Ok(step),
            },
        }
        if let Some(ArtifactOwnedSprEditActive::Mutations { field_id, authority }) = self.active.take() {
            *self.completed_arrays[usize::from(field_id == 4)] = Some(authority);
        }
        Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete)
    }

    pub fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::RetirementDemand;
        let diagnostic = self.diagnostic("artifact-spr.edit-close-demand", 0);
        if let Some(active) = self.active.as_ref() {
            let mut demand = match active {
                ArtifactOwnedSprEditActive::Mutations { authority, .. } => authority.close_demands(copy)?,
                _ => RetirementDemand::default(),
            };
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if let Some(owner) = self.completed_arrays.iter().find_map(|slot| slot.as_ref()) {
            let mut demand = owner.close_demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy).map_err(|_| diagnostic); }
        if let Some(value) = self.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("edit retains value factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.inverse.is_some() || self.forwards.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<ArtifactStoreOperationRowsRetirement<Mutation>>(), depth: 2, ..Default::default() }); }
        if let Some(owner)=self.actor_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of_val(&self.actor_close),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand().map_err(|error|diagnostic.with_native(error))?,capacity_bytes:owner.next_capacity_byte_demand(copy).map_err(|error|diagnostic.with_native(error))?,release_bytes:owner.next_release_byte_demand().map_err(|error|diagnostic.with_native(error))?,depth:owner.next_depth_demand().map_err(|error|diagnostic.with_native(error))?.checked_add(1).ok_or(diagnostic)?});}
        if self.actor.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value::SharedUtf8>()+size_of_val(&self.actor_close),depth:1,..Default::default()});}
        if let Some(value) = self.strings[self.close_string..].iter().find_map(|slot| slot.as_ref()) { return Ok(RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() }); }
        if self.retirement_factory.is_some() || self.mutation_factory.is_some() || self.catalog.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factories_close.iter().flatten().next() {
            let mut demand = owner.demands(copy).map_err(|_| diagnostic)?;
            demand.depth = demand.depth.checked_add(1).ok_or(diagnostic)?;
            return Ok(demand);
        }
        Ok(Default::default())
    }

    fn string_index(field_id: u16) -> Option<usize> {
        match field_id {
            1 => Some(0),
            2 => Some(1),
            9 => Some(2),
            10 => Some(3),
            11 => Some(4),
            12 => Some(5),
            _ => None,
        }
    }

    pub fn finish_record(&mut self) -> Result<(), OwnedSchemaDecodeDiagnostic> {
        if self.strings[1].is_some(){return Err(self.diagnostic("artifact-spr.edit-actor-unpublished-text",0));}
        let id = self.strings[0].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-id-missing", 0))?;
        let forwards = self.forwards.take().ok_or_else(|| self.diagnostic("artifact-spr.edit-forwards-missing", 0))?;
        let inverse = self.inverse.take().ok_or_else(|| self.diagnostic("artifact-spr.edit-inverse-missing", 0))?;
        let started_at = self.strings[2].take().ok_or_else(|| self.diagnostic("artifact-spr.edit-started-at-missing", 0))?;
        let sequence_number = self.sequence_number.ok_or_else(|| self.diagnostic("artifact-spr.edit-sequence-missing", 0))?;
        *self.value = Some(Edit {
            id,
            actor: self.actor.take(),
            line: self.strings[5].take(),
            forwards,
            inverse: inverse.into(),
            mutation_meta: Vec::new(),
            verb: self.strings[4].take(),
            sequence_number,
            started_at,
            finished_at: self.strings[3].take(),
        });
        Ok(())
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactOwnedHistoryEntryAuthority<Edit<Mutation>> for ArtifactOwnedSprEditAuthority<P, Mutation> {
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(copy).map(|demand| demand.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth) }
    fn accept_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-spr.edit-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-spr.edit-cancelled", token.start));
        }
        let (token, terminal) = match self.pending {
            Some(pending) => pending,
            None => match self.cursor.accept(token, source) {
                OwnedSchemaNestedRecordStep::Pending => return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete),
                OwnedSchemaNestedRecordStep::FieldToken { field_id, token, terminal } => {
                    if self.active.is_none() {
                        if let Some(step) = self.begin_edit_field(field_id, token, terminal, source)? {
                            return Ok(step);
                        }
                    }
                    (token, terminal)
                }
                OwnedSchemaNestedRecordStep::Complete => {
                    self.finish_record()?;
                    return Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete);
                }
                OwnedSchemaNestedRecordStep::Fault(diagnostic) => return Err(diagnostic),
            },
        };
        self.pending = Some((token, terminal));
        let outcome = self.accept_active_edit_token(token, terminal, source, cx);
        if !matches!(outcome, Ok(ArtifactEnvelopeFieldDecodeStep::Pending)) {
            self.pending = None;
        }
        outcome
    }

    fn take_value(&mut self) -> Option<Edit<Mutation>> {
        let value = self.value.take()?;
        self.terminal = true;
        Some(value)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        let diagnostic = self.diagnostic("artifact-spr.edit-close-authority", 0);
        if grant.maximum_depth < demand.depth { return Err(diagnostic); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.pending = None;
        self.terminal = true;
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(active) = self.active.as_mut() {
            let (step, terminal) = match active {
                ArtifactOwnedSprEditActive::String { authority, .. } => { authority.cancel(); (RetainedCloneStep::Complete(empty), authority.terminal_is_empty()) }
                ArtifactOwnedSprEditActive::Mutations { authority, .. } => { let step = authority.close_step(child)?; (step, authority.terminal_is_empty()) }
                ArtifactOwnedSprEditActive::EmptyMetadata(_) => (RetainedCloneStep::Complete(empty), true),
            };
            admit_retained_clone_close(child, step, terminal, "original edit active field").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(self.active.take()); }
            return Ok(RetainedCloneStep::Progress(if terminal && step.progress().copied_items == 0 { RetainedCloneProgress { copied_items: 1, ..step.progress() } } else { step.progress() }));
        }
        if let Some(slot) = self.completed_arrays.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original completed mutation array");
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original edit completed field").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(slot.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (diagnostic).with_native(error)); }
        if let Some(value) = self.value.take() {
            match self.retirement_factory.as_ref().expect("edit retains value factory").retire_owned(value, child) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(diagnostic.with_progress(progress)); } return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.value = Some(value); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(values) = self.inverse.take() {
            let factory = self.mutation_factory.as_ref().expect("edit inverse retains factory").clone();
            match admit_artifact_retirement(values, grant, |values| ArtifactStoreOperationRowsRetirement::new(values, factory)) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, values)) => { *self.inverse = Some(values); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(values) = self.forwards.take() {
            let factory = self.mutation_factory.as_ref().expect("edit forwards retains factory").clone();
            match admit_artifact_retirement(values, grant, |values| ArtifactStoreOperationRowsRetirement::new(values, factory)) {
                Ok((owner, progress)) => { *self.retirement = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, values)) => { *self.forwards = Some(values); return Err(diagnostic.with_native(error)); }
            }
        }
        if let Some(owner)=self.actor_close.as_mut(){if owner.terminal_is_empty(){self.actor_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}let step=owner.step(child).map_err(|error| (diagnostic).with_native(error))?;admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original decoded actor lease").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(original)=self.actor.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){Ok(owner)=>self.actor_close=Some(owner),Err((error,original))=>{*self.actor=Some(original);return Err(diagnostic.with_native(error));}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        while self.close_string < self.strings.len() {
            let index = self.close_string;
            self.close_string += 1;
            if self.strings[index].is_some() { drop(self.strings[index].take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        }
        let factory = if let Some(factory) = self.retirement_factory.take() { Some((0, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) }
            else if let Some(factory) = self.mutation_factory.take() { Some((1, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) }
            else { self.catalog.take().map(|factory| (2, factory as Arc<dyn semio_framework_value::FactoryRetirement>)) };
        if let Some((index, factory)) = factory {
            self.factories_close[index] = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        if let Some(slot) = self.factories_close.iter_mut().find(|slot| slot.is_some()) {
            let owner = slot.as_mut().expect("observed original edit factory");
            let step = owner.step(child).map_err(|error| (diagnostic).with_native(error))?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original edit factory").map_err(|error| (diagnostic).with_native(error).with_progress(step.progress()))?;
            if terminal { drop(slot.take()); }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.actor.is_none() && self.actor_close.is_none() && self.completed_arrays.iter().all(|slot| slot.is_none()) && self.factories_close.iter().all(Option::is_none) && self.catalog.is_none() && self.mutation_factory.is_none() && self.retirement_factory.is_none() && self.active.is_none() && self.forwards.is_none() && self.inverse.is_none() && self.value.is_none() && self.retirement.is_none() && self.strings.iter().all(|value| value.is_none())
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> Drop for ArtifactOwnedSprEditAuthority<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (ArtifactOwnedHistoryEntryAuthority::terminal_is_empty(self)), "SPR edit decode reached Drop before exact publication or bounded retirement");
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct ArtifactOwnedSprEditDecoder<P, Mutation> {
    #[factory_child]
    pub catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    #[factory_child]
    pub mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
}

impl<P: Send + Sync + 'static, Mutation: Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>> for ArtifactOwnedSprEditDecoder<P, Mutation> {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<Edit<Mutation>>> {
        Box::new(ArtifactOwnedSprEditAuthority::new(operation, generation, path, self.catalog.clone(), self.mutation_factory.clone(), retirement_factory))
    }
}

/// 📡️ Builds the repository-owned retained `.spr`/`.ops` edit decoder around an
/// exact domain catalog. Domain snapshots and mutations never cross this seam as serde owners.
pub fn artifact_owned_spr_edit_history_decoder<P: Send + Sync + 'static, Mutation: Send + Sync + 'static>(
    catalog: Arc<dyn ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>> {
    Arc::new(ArtifactOwnedSprEditDecoder { catalog, mutation_factory })
}

impl<T: FromValue + Send + Sync + 'static> ArtifactOwnedHistoryEntryDecoder<T> for ArtifactRepositoryHistoryEntryDecoder<T> {
    fn begin_entry(
        &self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        path: OwnedSchemaPath,
        retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,
    ) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<T>> {
        Box::new(ArtifactRepositoryHistoryEntryAuthority::new(operation, generation, path, retirement_factory))
    }
}

pub struct ArtifactRepositoryHistoryEntryAuthority<T> {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub path: OwnedSchemaPath,
    pub raw: std::mem::ManuallyDrop<Option<Box<[u8; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]>>>,
    raw_len: usize,
    pub value: std::mem::ManuallyDrop<Option<T>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub retirement_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    pub factory_close: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
    pub terminal: bool,
}

impl<T> ArtifactRepositoryHistoryEntryAuthority<T> {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: OwnedSchemaPath, retirement_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>) -> Self {
        Self { operation, generation, path, raw: std::mem::ManuallyDrop::new(Some(Box::new([0; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]))), raw_len: 0, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None), retirement_factory: std::mem::ManuallyDrop::new(Some(retirement_factory)), factory_close: std::mem::ManuallyDrop::new(None), terminal: false }
    }

    pub fn close_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if let Some(owner) = self.retirement.as_ref() { return artifact_retirement_box_demands(owner, copy); }
        if let Some(value) = self.value.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.retirement_factory.as_ref().expect("history value retains its factory").retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.raw_len != 0 { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.raw.is_some() { return Ok(RetirementDemand { release_bytes: size_of::<[u8; ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES]>(), depth: 1, ..Default::default() }); }
        if self.retirement_factory.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>(), depth: 1, ..Default::default() }); }
        if let Some(owner) = self.factory_close.as_ref() {
            let mut demand = owner.demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history decoder factory depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> OwnedSchemaDecodeDiagnostic {
        OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }
}

impl<T: FromValue + Send + 'static> ArtifactOwnedHistoryEntryAuthority<T> for ArtifactRepositoryHistoryEntryAuthority<T> {
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.copy_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-copy-demand", 0).with_native(error)) }
    fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(copy).map(|demand| demand.capacity_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-capacity-demand", 0).with_native(error)) }
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.release_bytes).map_err(|error| self.diagnostic("artifact-envelope.history-entry-release-demand", 0).with_native(error)) }
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { self.close_demands(0).map(|demand| demand.depth).map_err(|error| self.diagnostic("artifact-envelope.history-entry-depth-demand", 0).with_native(error)) }
    fn accept_token(&mut self, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            return Err(self.diagnostic("artifact-envelope.history-entry-stale", token.start));
        }
        if cx.is_cancelled() {
            return Err(self.diagnostic("artifact-envelope.history-entry-cancelled", token.start));
        }
        let path = self.path;
        let diagnostic = |code| OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        let span = usize::try_from(token.end.saturating_sub(token.start)).unwrap_or(usize::MAX);
        let raw = self.raw.as_mut().ok_or(OwnedSchemaDecodeDiagnostic { code: "artifact-envelope.history-entry-closed-source", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })?;
        let Some(end) = self.raw_len.checked_add(span).filter(|end| *end <= raw.len()) else {
            return Err(diagnostic("artifact-envelope.history-entry-byte-capacity"));
        };
        if source.copy_token_bytes(token, 0, &mut raw[self.raw_len..end]) != span {
            return Err(diagnostic("artifact-envelope.history-entry-source"));
        }
        self.raw_len = end;
        cx.consume_fuel(span.max(1) as u64);
        if !terminal {
            return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        let text = std::str::from_utf8(&raw[..self.raw_len]).map_err(|_| diagnostic("artifact-envelope.history-entry-decode"))?;
        let value = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| diagnostic("artifact-envelope.history-entry-decode"))?;
        *self.value = Some(value);
        self.raw_len = 0;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn take_value(&mut self) -> Option<T> {
        let value = self.value.take()?;
        self.terminal = true;
        Some(value)
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.close_demands(grant.maximum_copy_bytes).map_err(|error| (self.diagnostic("artifact-envelope.history-entry-demand", 0)).with_native(error))?;
        if grant.maximum_depth < demand.depth { return Err(self.diagnostic("artifact-envelope.history-entry-depth-refused", 0)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.terminal = true;
        if self.retirement.is_some() { return artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| (self.diagnostic("artifact-envelope.history-entry-retirement-fault", 0)).with_native(error)); }
        if let Some(value) = self.value.take() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            match self.retirement_factory.as_ref().expect("history value retains its factory").retire_owned(value, child) {
                Ok((owner, progress)) => {
                    *self.retirement = Some(owner);
                    if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(self.diagnostic("artifact-envelope.history-entry-constructor-receipt", 0).with_progress(progress)); }
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                Err((error, value)) => { *self.value = Some(value); return Err(self.diagnostic("artifact-envelope.history-entry-retirement-admission", 0).with_native(error)); }
            }
        }
        if self.raw_len != 0 { self.raw_len = 0; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
        if self.raw.is_some() { drop(self.raw.take()); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty })); }
        if let Some(factory) = self.retirement_factory.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            *self.factory_close = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
        }
        let failure = self.diagnostic("artifact-envelope.history-entry-factory-close", 0);
        let owner = self.factory_close.as_mut().expect("remaining original history factory");
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.step(child).map_err(|error| (failure).with_native(error))?;
        let terminal = owner.terminal_is_empty();
        admit_retained_clone_close(child, step, terminal, "original history decoder factory").map_err(|error| (self.diagnostic("artifact-envelope.history-entry-factory-receipt", 0)).with_native(error).with_progress(step.progress()))?;
        if terminal { drop(self.factory_close.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.retirement.is_none() && self.raw_len == 0 && self.raw.is_none() && self.retirement_factory.is_none() && self.factory_close.is_none()
    }
}

impl<T> Drop for ArtifactRepositoryHistoryEntryAuthority<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal && self.value.is_none() && self.retirement.is_none() && self.raw_len == 0 && self.raw.is_none() && self.retirement_factory.is_none() && self.factory_close.is_none()), "repository history entry authority reached Drop before exact terminal ownership");
    }
}

/// 🧬️ Domain-supplied typed field owner used by envelope decode; no serde type crosses this seam.
pub trait ArtifactEnvelopeFieldDecoder<P, Mutation>: Send {
    fn accept_field_token(&mut self, field_id: u16, token: OwnedSchemaToken, terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic>;
    fn maximum_close_byte_demand(&self) -> usize;
    fn maximum_retained_close_bytes(&self) -> usize;
    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic>;
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;
    fn terminal_is_empty(&self) -> bool;
}

/// 📏️ Observes every independent demand of one field decoder's next close turn.
pub fn artifact_envelope_field_demands<P, Mutation>(owner: &dyn ArtifactEnvelopeFieldDecoder<P, Mutation>, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, OwnedSchemaDecodeDiagnostic> {
    if owner.terminal_is_empty() { return Ok(Default::default()); }
    Ok(semio_framework_value::RetirementDemand { copy_bytes: owner.next_close_copy_byte_demand()?, capacity_bytes: owner.next_close_capacity_byte_demand(maximum_copy_bytes)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
}

/// 🎟️ Whether one admitted item, copy, capacity and release grant covers every independent demand axis.
pub fn artifact_retirement_grant_covers(grant: RetainedCloneGrant, demand: semio_framework_value::RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🎟️ Self-funds one turn from its own quoted demand for a caller no scheduler grants.
pub fn artifact_retirement_self_grant(demand: semio_framework_value::RetirementDemand, minimum_copy_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(minimum_copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 🧯️ Converts a decode diagnostic into the value-layer invariant refusal.
pub fn artifact_envelope_diagnostic_error(diagnostic: OwnedSchemaDecodeDiagnostic) -> ValueError {
    ValueError::literal(diagnostic.refusal_kind, diagnostic.code).with_retained_progress(diagnostic.retained_progress)
}

pub const ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeFieldDecoderTicket {
    pub index: u16,
    pub generation: u64,
}

impl ArtifactEnvelopeFieldDecoderTicket {
    pub fn index(self) -> usize {
        usize::from(self.index)
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeFieldDecoderRegistryFault {
    Contended,
    Capacity,
    CloseByteCapacity,
    GenerationExhausted,
    Stale,
    Returned,
    FalseTerminal,
}

pub struct ArtifactEnvelopeFieldDecoderSlot<P, Mutation> {
    pub generation: u64,
    pub owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>>,
}

pub struct ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation> {
    pub slots: Box<[ArtifactEnvelopeFieldDecoderSlot<P, Mutation>]>,
    pub free: [u16; ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY],
    pub free_len: usize,
}

impl<P, Mutation> Drop for ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.slots.iter().all(|slot| slot.owner.is_none())), "artifact envelope field registry reached Drop before every exact decoder owner was terminal-empty");
    }
}

/// 🗄️ Fixed app-owned registry that keeps nested field decoders out of stack/job destructors.
pub struct ArtifactEnvelopeFieldDecoderRegistry<P, Mutation> {
    pub state: Mutex<ArtifactEnvelopeFieldDecoderRegistryState<P, Mutation>>,
    pub returned: Box<[std::sync::atomic::AtomicU64]>,
    returned_mask: std::sync::atomic::AtomicU64,
    pub live: std::sync::atomic::AtomicUsize,
    reclaimed: Box<[std::sync::atomic::AtomicU64]>,
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderRegistry<P, Mutation> {
    /// 🏗️ Preallocates the entire fixed decoder authority before any input owner exists.
    pub fn new() -> Arc<Self> {
        let slots = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| ArtifactEnvelopeFieldDecoderSlot { generation: 0, owner: std::mem::ManuallyDrop::new(None) }).collect::<Vec<_>>().into_boxed_slice();
        let returned = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| std::sync::atomic::AtomicU64::new(0)).collect::<Vec<_>>().into_boxed_slice();
        let reclaimed = (0..ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY).map(|_| std::sync::atomic::AtomicU64::new(0)).collect::<Vec<_>>().into_boxed_slice();
        let mut free = [0; ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY];
        for (index, value) in free.iter_mut().enumerate() {
            *value = index as u16;
        }
        Arc::new(Self {
            state: Mutex::new(ArtifactEnvelopeFieldDecoderRegistryState { slots, free, free_len: ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY }),
            returned,
            returned_mask: std::sync::atomic::AtomicU64::new(0),
            live: std::sync::atomic::AtomicUsize::new(0),
            reclaimed,
        })
    }

    /// 🎫️ Admits one exact decoder or returns it untouched on contention/capacity/exhaustion.
    pub fn try_admit(
        self: &Arc<Self>,
        owner: Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>,
    ) -> Result<ArtifactEnvelopeFieldDecoderLease<P, Mutation>, (ArtifactEnvelopeFieldDecoderRegistryFault, Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>)> {
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(_) => return Err((ArtifactEnvelopeFieldDecoderRegistryFault::Contended, owner)),
        };
        if state.free_len == 0 {
            return Err((ArtifactEnvelopeFieldDecoderRegistryFault::Capacity, owner));
        }
        state.free_len -= 1;
        let index = usize::from(state.free[state.free_len]);
        let Some(generation) = state.slots[index].generation.checked_add(1) else {
            let free_index = state.free_len;
            state.free[free_index] = index as u16;
            state.free_len += 1;
            return Err((ArtifactEnvelopeFieldDecoderRegistryFault::GenerationExhausted, owner));
        };
        state.slots[index].generation = generation;
        *state.slots[index].owner = Some(owner);
        self.returned[index].store(0, std::sync::atomic::Ordering::Release);
        self.returned_mask.fetch_and(!(1u64 << index), std::sync::atomic::Ordering::AcqRel);
        self.reclaimed[index].store(0, std::sync::atomic::Ordering::Release);
        self.live.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(ArtifactEnvelopeFieldDecoderLease { registry: Arc::clone(self), ticket: ArtifactEnvelopeFieldDecoderTicket { index: index as u16, generation }, returned: false })
    }

    pub fn with_owner<R>(&self, ticket: ArtifactEnvelopeFieldDecoderTicket, f: impl FnOnce(&mut dyn ArtifactEnvelopeFieldDecoder<P, Mutation>) -> R) -> Result<R, ArtifactEnvelopeFieldDecoderRegistryFault> {
        let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeFieldDecoderRegistryFault::Contended)?;
        let index = usize::from(ticket.index);
        let Some(slot) = state.slots.get_mut(index) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale) };
        if slot.generation != ticket.generation {
            return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale);
        }
        if self.returned[index].load(std::sync::atomic::Ordering::Acquire) == ticket.generation {
            return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Returned);
        }
        let owner = slot.owner.as_deref_mut().ok_or(ArtifactEnvelopeFieldDecoderRegistryFault::Stale)?;
        Ok(f(owner))
    }

    /// 🧹️ Detaches one exact returned decoder without calling it or dropping it under the registry lock.
    pub fn take_returned_ticket(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> Result<ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>, ArtifactEnvelopeFieldDecoderRegistryFault> {
        let owner = {
            let mut state = self.state.try_lock().map_err(|_| ArtifactEnvelopeFieldDecoderRegistryFault::Contended)?;
            let index = usize::from(ticket.index);
            let Some(slot) = state.slots.get_mut(index) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale) };
            if slot.generation != ticket.generation {
                return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Stale);
            }
            if self.returned[index].load(std::sync::atomic::Ordering::Acquire) != ticket.generation {
                return Err(ArtifactEnvelopeFieldDecoderRegistryFault::Returned);
            }
            let owner = slot.owner.take().expect("validated returned decoder owner remains present");
            self.returned[index].store(0, std::sync::atomic::Ordering::Release);
            self.returned_mask.fetch_and(!(1u64 << index), std::sync::atomic::Ordering::AcqRel);
            self.reclaimed[index].store(ticket.generation, std::sync::atomic::Ordering::Release);
            self.live.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            let free_index = state.free_len;
            state.free[free_index] = ticket.index;
            state.free_len += 1;
            owner
        };
        Ok(ArtifactEnvelopeReturnedFieldDecoder { owner: std::mem::ManuallyDrop::new(Some(owner)) })
    }

    /// 🔎 Returns one exact atomically handed-back ticket without scanning the fixed registry.
    pub fn next_returned_ticket(&self) -> Option<ArtifactEnvelopeFieldDecoderTicket> {
        let mask = self.returned_mask.load(std::sync::atomic::Ordering::Acquire);
        if mask == 0 {
            return None;
        }
        let index = mask.trailing_zeros() as usize;
        let generation = self.returned[index].load(std::sync::atomic::Ordering::Acquire);
        (generation != 0).then_some(ArtifactEnvelopeFieldDecoderTicket { index: index as u16, generation })
    }

    /// ✅ Exact generation witness that the app maintenance owner detached this ticket.
    /// 🪪️ Whether THIS ticket's decoder has been handed back. Monotone by slot generation, so
    /// a later lease reusing the same slot can never make an already-reclaimed ticket read as
    /// outstanding again: an `==` test flipped a finished authority's own `Drop` witness back to
    /// false the moment a sibling authority took the next generation of the same slot, which is
    /// exactly the ABA the per-slot generation counter exists to rule out.
    pub fn ticket_reclaimed(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> bool {
        self.reclaimed[ticket.index()].load(std::sync::atomic::Ordering::Acquire) >= ticket.generation
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.live.load(std::sync::atomic::Ordering::Acquire) == 0
    }
}

/// 🪪️ Shallow non-cloneable lease; Drop atomically hands its exact decoder back to the registry.
pub struct ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    pub registry: Arc<ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>>,
    pub ticket: ArtifactEnvelopeFieldDecoderTicket,
    pub returned: bool,
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    pub fn with_owner<R>(&self, f: impl FnOnce(&mut dyn ArtifactEnvelopeFieldDecoder<P, Mutation>) -> R) -> Result<R, ArtifactEnvelopeFieldDecoderRegistryFault> {
        self.registry.with_owner(self.ticket, f)
    }

    pub fn ticket(&self) -> ArtifactEnvelopeFieldDecoderTicket {
        self.ticket
    }

    /// ↩️ Atomically returns the lease exactly once without acquiring or blocking on the owner mutex.
    pub fn return_now(&mut self) -> bool {
        if self.returned {
            return false;
        }
        let index = usize::from(self.ticket.index);
        self.returned = self.registry.returned[index].compare_exchange(0, self.ticket.generation, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok();
        if self.returned {
            self.registry.returned_mask.fetch_or(1u64 << index, std::sync::atomic::Ordering::Release);
        }
        self.returned
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeFieldDecoderLease<P, Mutation> {
    fn drop(&mut self) {
        let _ = self.return_now();
    }
}

/// 🧹️ Detached decoder owner driven outside the registry lock by the bounded maintenance lane.
pub struct ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    pub owner: std::mem::ManuallyDrop<Option<Box<dyn ArtifactEnvelopeFieldDecoder<P, Mutation>>>>,
}

impl<P, Mutation> ErasedSnapshotRetirement for ArtifactEnvelopeReturnedFieldDecoder<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.owner.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned artifact envelope field decoder exceeds admitted depth")); }
        if !artifact_retirement_grant_covers(grant, demand) { return Ok(RetainedCloneStep::Progress(empty)); }
        let owner = self.owner.as_mut().expect("returned field decoder remains present");
        if owner.terminal_is_empty() {
            drop(self.owner.take());
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..empty }));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = owner.close_step(child).map_err(artifact_envelope_diagnostic_error)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "returned artifact envelope field decoder")?;
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owner.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError> { Ok(self.demands(maximum_body_bytes)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    /// 📏️ Observes the detached decoder's next turn, or the separate release of its terminal shell.
    pub fn demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let Some(owner) = self.owner.as_ref() else { return Ok(Default::default()) };
        if owner.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
        let demand = artifact_envelope_field_demands(owner.as_ref(), maximum_copy_bytes).map_err(artifact_envelope_diagnostic_error)?;
        Ok(semio_framework_value::RetirementDemand { depth: demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned artifact envelope field decoder depth overflow"))?, ..demand })
    }
}

impl<P, Mutation> Drop for ArtifactEnvelopeReturnedFieldDecoder<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owner.is_none()), "returned artifact envelope field decoder reached Drop before its terminal-empty witness");
    }
}

impl<P, Mutation> ArtifactEnvelopeFieldDecoderRegistry<P, Mutation>
where
    P: Send,
    Mutation: Send,
{
    /// ↩️ Detaches and bounded-closes one exact returned decoder on the caller thread so a
    /// worker [`release_step`] can pass its `ticket_reclaimed` gate without waiting for the app's
    /// next maintenance visit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END `env-20`).
    pub fn reclaim_returned_ticket_now(&self, ticket: ArtifactEnvelopeFieldDecoderTicket) -> Result<(), ArtifactEnvelopeFieldDecoderRegistryFault> {
        if self.ticket_reclaimed(ticket) {
            return Ok(());
        }
        let mut retired = self.take_returned_ticket(ticket)?;
        for _ in 0..4096 {
            if retired.terminal_is_empty() {
                drop(retired);
                return Ok(());
            }
            let Ok(demand) = retired.demands(ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) else { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal) };
            if ErasedSnapshotRetirement::close_step(&mut retired, artifact_retirement_self_grant(demand, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)).is_err() { return Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal); }
        }
        Err(ArtifactEnvelopeFieldDecoderRegistryFault::FalseTerminal)
    }
}

/// 🧺️ Token-local authority for an exact empty JSON array. The opening token is
/// consumed separately from the terminal token so a surrounding record never replays either one.
pub struct OwnedSchemaEmptyArrayAuthority {
    pub path: OwnedSchemaPath,
    pub opened: bool,
    pub complete: bool,
}

pub const ARTIFACT_ENVELOPE_HISTORY_ITEMS: usize = 64;

pub const ARTIFACT_ENVELOPE_HISTORY_ENTRY_BYTES: usize = ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

impl OwnedSchemaEmptyArrayAuthority {
    pub fn new(path: OwnedSchemaPath) -> Self {
        Self { path, opened: false, complete: false }
    }

    pub fn accept(&mut self, token: OwnedSchemaToken, terminal: bool) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        if self.complete {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.trailing-empty-array-token", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        if !self.opened {
            if token.kind != OwnedSchemaTokenKind::ArrayStart || terminal {
                return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-empty-array", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
            }
            self.opened = true;
            return Ok(ArtifactEnvelopeFieldDecodeStep::TokenComplete);
        }
        if token.kind != OwnedSchemaTokenKind::ArrayEnd || !terminal {
            return Err(OwnedSchemaDecodeDiagnostic { code: "schema-json.expected-empty-array-end", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() });
        }
        self.complete = true;
        Ok(ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.complete
    }
}

pub const ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES: usize = 4_096;
