//! 🌱️ Fresh snapshot target and VCS authority: the fresh VCS fields, the fresh snapshot target and its field target, the active state and the fresh VCS authority. See [`super`].

use super::*;

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
    pub(super) fn accept_active_token(&mut self, token: OwnedSchemaToken, _terminal: bool, source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
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
    pub(super) fn begin_vcs_field(&mut self, field_id: u16) -> Result<(), OwnedSchemaDecodeDiagnostic> {
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
    pub(super) fn finish_vcs_record(&mut self) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
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

    pub(super) fn owners_terminal_empty(&self) -> bool {
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
    pub(super) fn hand_off_ledger<T: Send + 'static, F: ArtifactOwnedValueRetirementFactory<T> + semio_framework_value::FactoryPayloadRetirement>(ledger: &mut ArtifactHistoryLedger<T>, factory: F, slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if ledger.terminal_is_empty() && ledger.backing_is_empty() {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        artifact_history_ledger_handoff(ledger, factory, slot, grant)
    }
}
