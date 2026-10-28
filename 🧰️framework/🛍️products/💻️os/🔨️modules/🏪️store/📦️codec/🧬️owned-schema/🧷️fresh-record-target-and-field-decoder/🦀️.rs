//! 🧷️ Fresh record target and field decoder: the fresh record target, its decoded record target, the active record state and the fresh field decoder with its registries. See [`super`].

use super::*;

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
    pub(super) fn new() -> Self {
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
    pub(super) completion: Arc<ArtifactEnvelopeDecodeCompletion>,
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

    pub(super) fn owners_terminal_empty(&self) -> bool {
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
                transitions: semio_framework_os_vcs::ArtifactHistoryLedger::empty(),
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
