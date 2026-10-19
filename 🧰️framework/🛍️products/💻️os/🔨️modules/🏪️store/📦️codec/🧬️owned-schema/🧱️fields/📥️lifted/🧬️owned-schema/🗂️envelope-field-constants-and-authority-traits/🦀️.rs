//! 🗂️ Envelope field constants and authority traits: the envelope field names and owned schema, the field decode step and reservation, the field, snapshot, mutation and conflict authority traits and the history entry decoder. See [`super`].

use super::*;

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
    /// ⏳️ The exact token remains owned by the field authority and must be offered again.
    Pending,
    /// 🆔️ The exact token was consumed, but the enclosing field still owns later tokens.
    TokenComplete,
    /// 🌐️ The exact token and its enclosing field were both completed.
    FieldComplete,
    RecordComplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactEnvelopeFieldReservation {
    pub(super) field_id: u16,
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
pub struct ArtifactRepositoryHistoryEntryDecoder<T>(pub(super) PhantomData<fn() -> T>);

impl<T> ArtifactRepositoryHistoryEntryDecoder<T> {
    pub fn new() -> Self {
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
