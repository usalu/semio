// 🔓️ Decode authority and field decoders: the composed parent snapshot decode state and authority, the rejected field authority, the mutation and conflict field decoders.

//#region 🧬️ComposedParentFixture
#[expect(clippy::large_enum_variant, reason = "The fixture keeps its bounded retained hex authority inline through one exact field transition.")]
enum ComposedParentSnapshotDecodeState {
    AwaitToken,
    Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority<{ store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct ComposedParentSnapshotDecodeAuthority {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub state: ComposedParentSnapshotDecodeState,
    pub value: std::mem::ManuallyDrop<Option<ComposedParentSnapshot>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl ComposedParentSnapshotDecodeAuthority {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self {
            operation,
            generation,
            path,
            state: ComposedParentSnapshotDecodeState::AwaitToken,
            value: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
        }
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self.state, ComposedParentSnapshotDecodeState::Published | ComposedParentSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }

    pub fn close_demands(&self, maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if matches!(self.state, ComposedParentSnapshotDecodeState::Decode(_)) {
            return Ok(semio_framework_value::RetirementDemand { release_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, depth: 1, ..Default::default() });
        }
        let demand = match self.retirement.as_ref() {
            Some(owner) => store::os_store::snapshot_read::artifact_retirement_box_demands(owner, maximum_copy_bytes),
            None => store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value),
        };
        demand.map_err(|_| self.diagnostic("composed-parent-envelope.snapshot-retirement-demand", 0))
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<ComposedParentSnapshot> for ComposedParentSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if matches!(self.state, ComposedParentSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(self.diagnostic("composed-parent-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = ComposedParentSnapshotDecodeState::Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let path = self.path;
        let ComposedParentSnapshotDecodeState::Decode(authority) = &mut self.state else {
            return Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.snapshot-pack-token-replayed", offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() });
        };
        match authority.step(source, cx) {
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Pending => Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
                    code: "composed-parent-envelope.snapshot-pack-missing",
                    offset: token.start,
                    line: 0,
                    column: 0,
                    path,
                 refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })?;
                let value = <ComposedParentSnapshot as ArtifactPack>::decode_pack(bytes).map_err(|_| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
                    code: "composed-parent-envelope.snapshot-pack-malformed",
                    offset: token.start,
                    line: 0,
                    column: 0,
                    path,
                 refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })?;
                assert!(authority.release(), "completed composed-parent snapshot pack releases its retained bytes exactly once");
                *self.value = Some(value);
                self.state = ComposedParentSnapshotDecodeState::Ready;
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Cancelled => Err(self.diagnostic("composed-parent-envelope.snapshot-pack-cancelled", token.start)),
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldTarget<ComposedParentSnapshot>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, ComposedParentSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("composed-parent-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("composed-parent-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = ComposedParentSnapshotDecodeState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let empty = store::RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(store::RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth {
            return Err(self.diagnostic("composed-parent-envelope.snapshot-retirement-depth", 0));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(store::RetainedCloneStep::Progress(empty));
        }
        if let ComposedParentSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = ComposedParentSnapshotDecodeState::Closing;
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, ..empty }));
        }
        if self.retirement.is_some() {
            let fault = self.diagnostic("composed-parent-envelope.snapshot-retirement-fault", 0);
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map_err(|_| fault);
        }
        if self.value.is_some() {
            let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|_| self.diagnostic("composed-parent-envelope.snapshot-retirement-admission", 0))?;
            self.state = ComposedParentSnapshotDecodeState::Closing;
            return Ok(step);
        }
        self.state = ComposedParentSnapshotDecodeState::Complete;
        Ok(store::RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        ComposedParentSnapshotDecodeAuthority::terminal_is_empty(self)
    }
}

impl Drop for ComposedParentSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "composed-parent snapshot decoder dropped before publication or bounded retirement");
    }
}

struct ComposedParentRejectedFieldAuthority {
    pub path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<RecursiveFixtureMutation> for ComposedParentRejectedFieldAuthority {
    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.fresh-mutation-not-admitted", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn publish_reserved(
        &mut self,
        _target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldTarget<RecursiveFixtureMutation>,
        _reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.fresh-mutation-not-admitted", offset: 0, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(store::RetainedCloneStep::Complete(store::RetainedCloneProgress::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for ComposedParentRejectedFieldAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "composed-parent-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }
    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> { Ok(0) }

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if grant.maximum_items == 0 {
            return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress::default()));
        }
        self.terminal = true;
        Ok(store::RetainedCloneStep::Complete(store::RetainedCloneProgress::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}
//#endregion 🧬️ComposedParentFixture
