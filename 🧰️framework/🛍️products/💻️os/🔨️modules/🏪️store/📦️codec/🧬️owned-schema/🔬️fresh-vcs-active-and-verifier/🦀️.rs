//! 🔬️ Fresh VCS active and verifier: the active fresh VCS operations, the verifier and the authority drop. See [`super`].

use super::*;

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
