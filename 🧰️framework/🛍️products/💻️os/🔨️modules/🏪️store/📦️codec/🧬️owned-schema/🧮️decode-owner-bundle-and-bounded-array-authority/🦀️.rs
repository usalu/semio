//! 🧮️ Decode owner bundle and bounded array authority: the envelope decode owner bundle, the bounded array state and the credit bounded array authority with its drop. See [`super`].

use super::*;

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
