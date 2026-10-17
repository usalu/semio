//! 📸️ Jack snapshot decode: the owned field budget helpers and the retained snapshot decode authority.

use super::*;

//#region 🔖️OwnedSprCatalog
pub const JACK_OWNED_FIELD_BYTES: usize = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

/// 🎟️ Whether one admitted item, copy, capacity, release and depth grant covers every independent demand axis.
pub(crate) fn funds(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🪆️ Counts one more logical owner frame above a child's quote.
pub(crate) fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "jack retained child depth overflow"))?;
    Ok(demand)
}

/// 🧯️ Refuses a grant whose depth cannot carry the quoted frontier and answers whether the whole quote is funded.
pub fn admit_demand(grant: RetainedCloneGrant, demand: RetirementDemand, scope: &'static str) -> Result<bool, ValueError> {
    if grant.maximum_depth < demand.depth {
        return Err(ValueError::literal(ValueRefusalKind::DepthLimit, scope));
    }
    Ok(funds(grant, demand))
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its fixed-size owned field authority inline so cursor transitions do not allocate another retirement owner.")]
enum JackSnapshotDecodeState {
    AwaitToken,
    Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

pub struct JackSnapshotDecodeAuthority {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub state: JackSnapshotDecodeState,
    pub value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackSnapshotDecodeAuthority {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackSnapshotDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    pub fn native(&self, code: &'static str, error: ValueError) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        self.diagnostic(code, 0).with_native(error)
    }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if matches!(self.state, JackSnapshotDecodeState::Decode(_)) {
            return Ok(RetirementDemand { release_bytes: JACK_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body);
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.value);
        }
        Ok(Default::default())
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot> for JackSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path , refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        if matches!(self.state, JackSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = JackSnapshotDecodeState::Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackSnapshotDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.snapshot-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Pending => Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.snapshot-pack-missing", token.start))?;
                let value = <JackSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|_| diagnostic("jack-envelope.snapshot-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack snapshot pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackSnapshotDecodeState::Ready;
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.snapshot-pack-cancelled", token.start)),
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldTarget<JackSnapshot>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = JackSnapshotDecodeState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))?;
        if !admit_demand(grant, demand, "jack snapshot decode close exceeds admitted depth").map_err(|error| self.native("jack-envelope.snapshot-close-depth", error))? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let JackSnapshotDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = JackSnapshotDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JACK_OWNED_FIELD_BYTES, ..empty }));
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| self.native("jack-envelope.snapshot-retirement-fault", error));
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| self.native("jack-envelope.snapshot-retirement-birth", error));
        }
        self.state = JackSnapshotDecodeState::Complete;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, JackSnapshotDecodeState::Published | JackSnapshotDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self), "Jack snapshot decode reached Drop before publication or bounded retirement");
    }
}
//#endregion 🔖️OwnedSprCatalog
