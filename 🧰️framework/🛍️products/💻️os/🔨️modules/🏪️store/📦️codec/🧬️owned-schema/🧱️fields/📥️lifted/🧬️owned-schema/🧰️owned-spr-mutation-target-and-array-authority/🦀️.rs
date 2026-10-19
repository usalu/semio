//! 🧰️ Owned SPR mutation target and array authority: the owned SPR edit fields, the mutation target, the mutation array state and authority and the edit authority struct. See [`super`].

use super::*;

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
    pub(super) fn new() -> Self {
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
    pub(super) fn try_new(
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
    pub(super) completed_arrays: [std::mem::ManuallyDrop<Option<ArtifactOwnedSprMutationArrayAuthority<P, Mutation>>>; 2],
    pub factories_close: [Option<semio_framework_value::FactoryAuthority>; 3],
    pub strings: [std::mem::ManuallyDrop<Option<String>>; 6],
    pub actor: std::mem::ManuallyDrop<Option<semio_framework_value::SharedUtf8>>,
    pub actor_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_value::SharedUtf8>>,
    pub forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,
    pub inverse: std::mem::ManuallyDrop<Option<semio_framework_value_persistent::list::PagedList<Mutation, {usize::MAX}>>>,
    pub sequence_number: Option<i32>,
    pub value: std::mem::ManuallyDrop<Option<Edit<Mutation>>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(super) close_string: usize,
    pub terminal: bool,
}
