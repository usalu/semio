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

/// ⏳️ One owned-schema close turn: still pending, or settled with its step and the actual work its effect performed.
pub type ArtifactEnvelopeClosePoll = RetainedWorkPoll<RetainedCloneStep, OwnedSchemaDecodeDiagnostic>;

/// 🚦️ Control flow of a close ladder: a pending child or a fault, so `?` forwards both.
#[derive(Debug, PartialEq, Eq)]
pub enum ArtifactEnvelopeCloseExit<E = OwnedSchemaDecodeDiagnostic> {
    Pending,
    Fault(E),
}

impl<E> From<E> for ArtifactEnvelopeCloseExit<E> {
    fn from(error: E) -> Self {
        Self::Fault(error)
    }
}

/// 🪆️ Forwards a child poll into a ladder: `Pending` stays pending, a settled fault is converted.
pub fn artifact_envelope_close_resume<T, E, F: From<E>>(poll: RetainedWorkPoll<T, E>) -> Result<T, ArtifactEnvelopeCloseExit<F>> {
    artifact_envelope_close_resume_with(poll, F::from)
}

/// 🪆️ Forwards a child poll into a ladder through an explicit fault mapping.
pub fn artifact_envelope_close_resume_with<T, E, F>(poll: RetainedWorkPoll<T, E>, map: impl FnOnce(E) -> F) -> Result<T, ArtifactEnvelopeCloseExit<F>> {
    match poll {
        RetainedWorkPoll::Pending { .. } => Err(ArtifactEnvelopeCloseExit::Pending),
        RetainedWorkPoll::Complete(turn) => turn.result.map_err(|error| ArtifactEnvelopeCloseExit::Fault(map(error))),
    }
}

/// 🧾️ Settles a ladder against its effect once: a progressing turn that recorded no work pays one unit, and the poll carries exactly the effect's performed work.
pub fn artifact_envelope_close_finish<T, E>(effect: &mut RetainedAllocationEffect<'_>, result: Result<T, ArtifactEnvelopeCloseExit<E>>, progressed: impl FnOnce(&T) -> bool, refused: impl FnOnce() -> E) -> RetainedWorkPoll<T, E> {
    let result = match result {
        Err(ArtifactEnvelopeCloseExit::Pending) => return RetainedWorkPoll::Pending { work: effect.performed_work() },
        Err(ArtifactEnvelopeCloseExit::Fault(error)) => Err(error),
        Ok(value) if effect.performed_work() == 0 && progressed(&value) => match effect.performed(1) {
            Ok(true) => Ok(value),
            Ok(false) | Err(_) => Err(refused()),
        },
        Ok(value) => Ok(value),
    };
    RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: effect.performed_work() })
}

/// 🧾️ Settles a field close ladder against its effect.
pub fn artifact_envelope_close_settle(effect: &mut RetainedAllocationEffect<'_>, result: Result<RetainedCloneStep, ArtifactEnvelopeCloseExit>) -> ArtifactEnvelopeClosePoll {
    artifact_envelope_close_finish(effect, result, |step| step.progress() != RetainedCloneProgress::default(), || {
        let mut diagnostic = OwnedSchemaDecodeDiagnostic::before("artifact-envelope.close-work-limit", OwnedSchemaPath::ROOT);
        diagnostic.refusal_kind = ValueRefusalKind::WorkLimit;
        diagnostic
    })
}

/// 🧾️ Settles a value-error close ladder into an original retirement poll against its effect.
pub fn artifact_envelope_retirement_settle(effect: &mut RetainedAllocationEffect<'_>, result: Result<RetainedCloneStep, ArtifactEnvelopeCloseExit<ValueError>>) -> OriginalRetirementPoll {
    let result = result.map_err(|exit| match exit {
        ArtifactEnvelopeCloseExit::Pending => ArtifactEnvelopeCloseExit::Pending,
        ArtifactEnvelopeCloseExit::Fault(error) => ArtifactEnvelopeCloseExit::Fault(error.into()),
    });
    artifact_envelope_close_finish(effect, result, |step| step.progress() != RetainedCloneProgress::default(), || artifact_envelope_close_work_error().into())
}

/// 🧾️ Records the work a grant-shaped child reported into the caller's effect so the ladder's poll keeps one exact charge.
pub fn artifact_envelope_close_charge<T, E, F: From<E>>(effect: &mut RetainedAllocationEffect<'_>, turn: RetainedWorkOutcome<T, E>, refused: impl FnOnce() -> F) -> Result<T, ArtifactEnvelopeCloseExit<F>> {
    let admitted = u64::try_from(turn.work).map_or(false, |work| work == 0 || effect.performed(work).unwrap_or(false));
    match turn.result {
        Err(error) => Err(ArtifactEnvelopeCloseExit::Fault(F::from(error))),
        Ok(value) if admitted => Ok(value),
        Ok(_) => Err(ArtifactEnvelopeCloseExit::Fault(refused())),
    }
}

/// ⛽️ The refusal of a close ladder whose effect cannot fund its recorded work.
pub fn artifact_envelope_close_work_error() -> ValueError {
    ValueError::literal(ValueRefusalKind::WorkLimit, "artifact envelope close exceeds its work ceiling")
}

/// 🧯️ Converts an original retirement failure into its owning value refusal, keeping the physical effects it recorded.
pub fn artifact_envelope_failure_error(failure: OriginalRetirementFailure) -> ValueError {
    let progress = failure.retained_progress();
    let (primary, work, receipt, ..) = failure.into_parts();
    primary.or(work).or(receipt).unwrap_or_else(|| ValueError::invariant("original retirement failed without a recorded cause")).with_retained_progress(progress)
}

/// 🧯️ Carries the refusal kind and physical effects of an original retirement failure into a decode diagnostic.
pub fn artifact_envelope_failure_diagnostic(diagnostic: OwnedSchemaDecodeDiagnostic, failure: &OriginalRetirementFailure) -> OwnedSchemaDecodeDiagnostic {
    let kind = failure.primary().or(failure.work_refusal()).or(failure.receipt_refusal()).map_or(diagnostic.refusal_kind, |error| error.kind);
    OwnedSchemaDecodeDiagnostic { refusal_kind: kind, retained_progress: failure.retained_progress(), ..diagnostic }
}

pub use semio_framework_value::factory_retire_owned as artifact_envelope_factory_retire;

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
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll;
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
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll;
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
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll;
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
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll;
    fn terminal_is_empty(&self) -> bool;
}

/// 🏭️ Mandatory domain catalog for every nested envelope decoder; there is no default implementation.
pub trait ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn begin_vcs(
        &self,
        replacement: Option<semio_framework_job::ReplacementIdentity>,
        path: OwnedSchemaPath,
    ) -> Result<Box<dyn ArtifactEnvelopeVcsFieldAuthority<P, Mutation>>, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>>;
    fn maximum_vcs_close_byte_demand(&self) -> usize;
    fn maximum_retained_vcs_close_bytes(&self) -> usize;
    fn begin_snapshot(&self, replacement: Option<semio_framework_job::ReplacementIdentity>, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<P>>;
    fn begin_mutation(&self, replacement: Option<semio_framework_job::ReplacementIdentity>, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeMutationFieldAuthority<Mutation>>;
    fn begin_spr_conflict(&self, replacement: Option<semio_framework_job::ReplacementIdentity>, path: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSprConflictAuthority>;
    fn edit_history_decoder(&self) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<Mutation>>>;
}

/// 🧬️ Owner-supplied decoder for one pre-admitted history entry. The framework
/// never invokes `DeserializeOwned` for a domain edit behind the catalog's back.
pub trait ArtifactOwnedHistoryEntryDecoder<T>: semio_framework_value::FactoryRetirement + Send + Sync {
    fn begin_entry(
        &self,
        replacement: Option<semio_framework_job::ReplacementIdentity>,
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
    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll;
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
