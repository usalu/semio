//! 🔄️ Jack mutation decode: the retained mutation decode authority, the rejected conflict authority and the owned field catalog.

use super::*;

//#region 🔖️OwnedSprCatalog
#[expect(clippy::large_enum_variant, reason = "The active decoder retains its fixed-size owned field authority inline so cursor transitions do not allocate another retirement owner.")]
enum JackMutationDecodeState {
    AwaitToken,
    Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct JackMutationDecodeAuthority {
    pub operation: semio_framework_job::OperationId,
    pub generation: semio_framework_job::Generation,
    pub path: store::os_store::owned_schema_decode::OwnedSchemaPath,
    pub state: JackMutationDecodeState,
    pub value: std::mem::ManuallyDrop<Option<TrinityGraphMutation>>,
    pub retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackMutationDecodeAuthority {
    pub fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackMutationDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    pub fn diagnostic(&self, code: &'static str, offset: u64) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() }
    }

    pub fn native(&self, code: &'static str, error: ValueError) -> store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic {
        self.diagnostic(code, 0).with_native(error)
    }

    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if matches!(self.state, JackMutationDecodeState::Decode(_)) {
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

impl store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation> for JackMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path , refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        if matches!(self.state, JackMutationDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.mutation-pack-must-be-scalar", token.start));
            }
            self.state = JackMutationDecodeState::Decode(store::os_store::owned_schema_decode::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackMutationDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.mutation-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Pending => Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.mutation-pack-missing", token.start))?;
                let value = TrinityGraphMutation::decode_op(bytes).map_err(|_| diagnostic("jack-envelope.mutation-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack mutation pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackMutationDecodeState::Ready;
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.mutation-pack-cancelled", token.start)),
            store::os_store::owned_schema_decode::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldTarget<TrinityGraphMutation>,
        reservation: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackMutationDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.mutation-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = JackMutationDecodeState::Published;
        Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))?;
        if !admit_demand(grant, demand, "jack mutation decode close exceeds admitted depth").map_err(|error| self.native("jack-envelope.mutation-close-depth", error))? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if let JackMutationDecodeState::Decode(authority) = &mut self.state {
            authority.cancel();
            self.state = JackMutationDecodeState::Closing;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: JACK_OWNED_FIELD_BYTES, ..empty }));
        }
        if self.retirement.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| self.native("jack-envelope.mutation-retirement-fault", error));
        }
        if self.value.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| self.native("jack-envelope.mutation-retirement-birth", error));
        }
        self.state = JackMutationDecodeState::Complete;
        Ok(RetainedCloneStep::Complete(empty))
    }

    fn terminal_is_empty(&self) -> bool {
        matches!(self.state, JackMutationDecodeState::Published | JackMutationDecodeState::Complete) && self.value.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackMutationDecodeAuthority {
    fn drop(&mut self) {
        assert!(store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self), "Jack mutation decode reached Drop before publication or bounded retirement");
    }
}

struct JackRejectedConflictAuthority {
    pub terminal: bool,
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority for JackRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Err(store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::os_store::owned_schema_decode::OwnedSchemaPath::ROOT , refusal_kind: ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        let empty = RetainedCloneProgress::default();
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct JackEnvelopeOwnedFieldCatalog;

impl store::os_store::owned_schema_decode::ArtifactEnvelopeOwnedFieldCatalog<JackSnapshot, TrinityGraphMutation> for JackEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Result<Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<JackSnapshot, TrinityGraphMutation>>, Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot>>> {
        store::os_store::owned_schema_decode::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(OwnedValueRetirementFactory::<JackSnapshot>::default()), std::sync::Arc::new(OwnedValueRetirementFactory::<TrinityGraphMutation>::default()), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeVcsFieldAuthority<JackSnapshot, TrinityGraphMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot>> {
        Box::new(JackSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation>> {
        Box::new(JackMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::os_store::owned_schema_decode::OwnedSchemaPath) -> Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(JackRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::os_store::owned_schema_decode::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<TrinityGraphMutation>>> {
        store::os_store::owned_schema_decode::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(OwnedValueRetirementFactory::<TrinityGraphMutation>::default()))
    }
}

pub fn jack_envelope_decode_owner_bundle() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<JackSnapshot, TrinityGraphMutation> {
    store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(JackEnvelopeOwnedFieldCatalog), std::sync::Arc::new(OwnedValueRetirementFactory::<JackSnapshot>::default()), std::sync::Arc::new(OwnedValueRetirementFactory::<TrinityGraphMutation>::default()))
}
//#endregion 🔖️OwnedSprCatalog
