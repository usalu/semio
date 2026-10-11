//! 🏠️ Jack retained publication, replay and document host ownership.
#![allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::mutations::TrinityGraphMutation;
use crate::standards::v1::subsets::any::schema::mutations::set_query;
use crate::executor::GraphEffect;
use crate::{Edge, EntityRef, JackSnapshot, Node, Port, PropertyBag, PropertyDef, PropertyValue};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_diagnostic::TextError;
use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{controlled::ControlledRetirement, OwnedValueRetirementFactory, SharedValueRetirementFactory};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
#[cfg(test)]
use crate::standards::v1::subsets::any::io::binary::mutations::{encode_op, decode_op};
//#region 🔖️OwnedSprCatalog
const JACK_OWNED_FIELD_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

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
pub(crate) fn admit_demand(grant: RetainedCloneGrant, demand: RetirementDemand, scope: &'static str) -> Result<bool, ValueError> {
    if grant.maximum_depth < demand.depth {
        return Err(ValueError::literal(ValueRefusalKind::DepthLimit, scope));
    }
    Ok(funds(grant, demand))
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its fixed-size owned field authority inline so cursor transitions do not allocate another retirement owner.")]
enum JackSnapshotDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct JackSnapshotDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: JackSnapshotDecodeState,
    value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackSnapshotDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackSnapshotDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    fn native(&self, code: &'static str, error: ValueError) -> store::OwnedSchemaDecodeDiagnostic {
        self.diagnostic(code, 0).with_native(error)
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if matches!(self.state, JackSnapshotDecodeState::Decode(_)) {
            return Ok(RetirementDemand { release_bytes: JACK_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::artifact_retirement_box_demands(owner, body);
        }
        if self.value.is_some() {
            return store::artifact_retirement_owned_birth_demands(&self.value);
        }
        Ok(Default::default())
    }
}

impl store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot> for JackSnapshotDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
        if matches!(self.state, JackSnapshotDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.snapshot-pack-must-be-scalar", token.start));
            }
            self.state = JackSnapshotDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackSnapshotDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.snapshot-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.snapshot-pack-missing", token.start))?;
                let value = <JackSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|_| diagnostic("jack-envelope.snapshot-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack snapshot pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackSnapshotDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.snapshot-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeSnapshotFieldTarget<JackSnapshot>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackSnapshotDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.snapshot-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.snapshot-owner-missing", 0))?;
        target.publish_snapshot_reserved(reservation, value);
        self.state = JackSnapshotDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth).map_err(|error| self.native("jack-envelope.snapshot-close-demand", error))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
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
            return store::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| self.native("jack-envelope.snapshot-retirement-fault", error));
        }
        if self.value.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| self.native("jack-envelope.snapshot-retirement-birth", error));
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
        assert!(store::ArtifactEnvelopeSnapshotFieldAuthority::terminal_is_empty(self), "Jack snapshot decode reached Drop before publication or bounded retirement");
    }
}

#[expect(clippy::large_enum_variant, reason = "The active decoder retains its fixed-size owned field authority inline so cursor transitions do not allocate another retirement owner.")]
enum JackMutationDecodeState {
    AwaitToken,
    Decode(store::OwnedSchemaHexAuthority<JACK_OWNED_FIELD_BYTES>),
    Ready,
    Published,
    Closing,
    Complete,
}

struct JackMutationDecodeAuthority {
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    path: store::OwnedSchemaPath,
    state: JackMutationDecodeState,
    value: std::mem::ManuallyDrop<Option<TrinityGraphMutation>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl JackMutationDecodeAuthority {
    fn new(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Self {
        Self { operation, generation, path, state: JackMutationDecodeState::AwaitToken, value: std::mem::ManuallyDrop::new(None), retirement: std::mem::ManuallyDrop::new(None) }
    }

    fn diagnostic(&self, code: &'static str, offset: u64) -> store::OwnedSchemaDecodeDiagnostic {
        store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path: self.path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() }
    }

    fn native(&self, code: &'static str, error: ValueError) -> store::OwnedSchemaDecodeDiagnostic {
        self.diagnostic(code, 0).with_native(error)
    }

    fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if matches!(self.state, JackMutationDecodeState::Decode(_)) {
            return Ok(RetirementDemand { release_bytes: JACK_OWNED_FIELD_BYTES, depth: 1, ..Default::default() });
        }
        if let Some(owner) = self.retirement.as_ref() {
            return store::artifact_retirement_box_demands(owner, body);
        }
        if self.value.is_some() {
            return store::artifact_retirement_owned_birth_demands(&self.value);
        }
        Ok(Default::default())
    }
}

impl store::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation> for JackMutationDecodeAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        terminal: bool,
        source: &store::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        let path = self.path;
        let diagnostic = |code: &'static str, offset| store::OwnedSchemaDecodeDiagnostic { code, offset, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() };
        if matches!(self.state, JackMutationDecodeState::AwaitToken) {
            if !terminal {
                return Err(diagnostic("jack-envelope.mutation-pack-must-be-scalar", token.start));
            }
            self.state = JackMutationDecodeState::Decode(store::OwnedSchemaHexAuthority::try_new(self.operation, self.generation, token, self.path)?);
        }
        let JackMutationDecodeState::Decode(authority) = &mut self.state else { return Err(diagnostic("jack-envelope.mutation-pack-token-replayed", token.start)) };
        match authority.step(source, cx) {
            store::OwnedSchemaHexStep::Pending => Ok(store::ArtifactEnvelopeFieldDecodeStep::Pending),
            store::OwnedSchemaHexStep::Complete => {
                let bytes = authority.as_bytes().ok_or_else(|| diagnostic("jack-envelope.mutation-pack-missing", token.start))?;
                let value = TrinityGraphMutation::decode_op(bytes).map_err(|_| diagnostic("jack-envelope.mutation-pack-malformed", token.start))?;
                assert!(authority.release(), "completed Jack mutation pack releases its inline bytes exactly once");
                *self.value = Some(value);
                self.state = JackMutationDecodeState::Ready;
                Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
            }
            store::OwnedSchemaHexStep::Cancelled => Err(diagnostic("jack-envelope.mutation-pack-cancelled", token.start)),
            store::OwnedSchemaHexStep::Fault(diagnostic) => Err(diagnostic),
        }
    }

    fn publish_reserved(
        &mut self,
        target: &mut dyn store::ArtifactEnvelopeMutationFieldTarget<TrinityGraphMutation>,
        reservation: store::ArtifactEnvelopeFieldReservation,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        if !matches!(self.state, JackMutationDecodeState::Ready) {
            return Err(self.diagnostic("jack-envelope.mutation-pack-not-ready", 0));
        }
        let value = self.value.take().ok_or_else(|| self.diagnostic("jack-envelope.mutation-owner-missing", 0))?;
        target.publish_mutation_reserved(reservation, value);
        self.state = JackMutationDecodeState::Published;
        Ok(store::ArtifactEnvelopeFieldDecodeStep::FieldComplete)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.copy_bytes).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(maximum_copy_bytes).map(|demand| demand.capacity_bytes).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.release_bytes).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        self.close_demands(0).map(|demand| demand.depth).map_err(|error| self.native("jack-envelope.mutation-close-demand", error))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
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
            return store::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress())).map_err(|error| self.native("jack-envelope.mutation-retirement-fault", error));
        }
        if self.value.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant).map_err(|error| self.native("jack-envelope.mutation-retirement-birth", error));
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
        assert!(store::ArtifactEnvelopeMutationFieldAuthority::terminal_is_empty(self), "Jack mutation decode reached Drop before publication or bounded retirement");
    }
}

struct JackRejectedConflictAuthority {
    terminal: bool,
}

impl store::ArtifactEnvelopeSprConflictAuthority for JackRejectedConflictAuthority {
    fn accept_token(
        &mut self,
        token: store::OwnedSchemaToken,
        _terminal: bool,
        _source: &store::OwnedSchemaRecordCursor,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::ArtifactEnvelopeFieldDecodeStep, store::OwnedSchemaDecodeDiagnostic> {
        Err(store::OwnedSchemaDecodeDiagnostic { code: "jack-envelope.fresh-conflict-not-admitted", offset: token.start, line: 0, column: 0, path: store::OwnedSchemaPath::ROOT , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: semio_framework_value::RetainedCloneProgress::default() })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::OwnedSchemaDecodeDiagnostic> {
        Ok(usize::from(!self.terminal))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, store::OwnedSchemaDecodeDiagnostic> {
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
impl store::ArtifactEnvelopeOwnedFieldCatalog<JackSnapshot, TrinityGraphMutation> for JackEnvelopeOwnedFieldCatalog {
    fn begin_vcs(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Result<Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<JackSnapshot, TrinityGraphMutation>>, Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot>>> {
        store::ArtifactEnvelopeFreshVcsAuthority::try_new(self.begin_snapshot(operation, generation, path), std::sync::Arc::new(OwnedValueRetirementFactory::<JackSnapshot>::default()), std::sync::Arc::new(OwnedValueRetirementFactory::<TrinityGraphMutation>::default()), self.edit_history_decoder())
            .map(|authority| Box::new(authority) as Box<dyn store::ArtifactEnvelopeVcsFieldAuthority<JackSnapshot, TrinityGraphMutation>>)
    }

    fn maximum_vcs_close_byte_demand(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES
    }

    fn maximum_retained_vcs_close_bytes(&self) -> usize {
        store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES
    }

    fn begin_snapshot(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSnapshotFieldAuthority<JackSnapshot>> {
        Box::new(JackSnapshotDecodeAuthority::new(operation, generation, path))
    }

    fn begin_mutation(&self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeMutationFieldAuthority<TrinityGraphMutation>> {
        Box::new(JackMutationDecodeAuthority::new(operation, generation, path))
    }

    fn begin_spr_conflict(&self, _operation: semio_framework_job::OperationId, _generation: semio_framework_job::Generation, _path: store::OwnedSchemaPath) -> Box<dyn store::ArtifactEnvelopeSprConflictAuthority> {
        Box::new(JackRejectedConflictAuthority { terminal: false })
    }

    fn edit_history_decoder(&self) -> std::sync::Arc<dyn store::ArtifactOwnedHistoryEntryDecoder<protocol::Edit<TrinityGraphMutation>>> {
        store::artifact_owned_spr_edit_history_decoder(std::sync::Arc::new(Self), std::sync::Arc::new(OwnedValueRetirementFactory::<TrinityGraphMutation>::default()))
    }
}

pub fn jack_envelope_decode_owner_bundle() -> store::ArtifactEnvelopeDecodeOwnerBundle<JackSnapshot, TrinityGraphMutation> {
    store::ArtifactEnvelopeDecodeOwnerBundle::new(std::sync::Arc::new(JackEnvelopeOwnedFieldCatalog), std::sync::Arc::new(OwnedValueRetirementFactory::<JackSnapshot>::default()), std::sync::Arc::new(OwnedValueRetirementFactory::<TrinityGraphMutation>::default()))
}
//#endregion 🔖️OwnedSprCatalog

//#region 🔖️RetainedStoreInitialization
#[derive(semio_framework_value::RetireOwned)]
enum JackSnapshotCloneKind {
    Node { source: usize, property: usize, port: usize, value: crate::NodeKindDef },
    Edge { source: usize, property: usize, value: crate::EdgeKindDef },
    Port { source: usize, property: usize, value: crate::PortKindDef },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JackSnapshotCloneStep {
    Pending { copied_bytes: usize },
    Complete,
}

pub struct JackSnapshotCloneAuthority {
    value: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    active: std::mem::ManuallyDrop<Option<JackSnapshotCloneKind>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    phase: u8,
    index: usize,
    retain_local_owner: bool,
    terminal: bool,
}

impl Default for JackSnapshotCloneAuthority {
    fn default() -> Self {
        Self::new()
    }
}

impl JackSnapshotCloneAuthority {
    pub fn new() -> Self {
        Self::with_local_owner(true)
    }

    pub fn metadata_only() -> Self {
        Self::with_local_owner(false)
    }

    fn with_local_owner(retain_local_owner: bool) -> Self {
        let content = store::ArtifactChild::new(String::new(), semio_framework_artifact_reference::ArtifactRef { artifact_id: String::new(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: String::new(), standard: String::new(), subset: String::new() } });
        Self {
            value: std::mem::ManuallyDrop::new(Some(JackSnapshot { schema: String::new(), name: String::new(), manifest_id: None, manifest: Default::default(), camera: Default::default(), content, root_node_id: None, query: String::new() })),
            active: std::mem::ManuallyDrop::new(None),
            retirement: std::mem::ManuallyDrop::new(None),
            phase: 0,
            index: 0,
            retain_local_owner,
            terminal: false,
        }
    }

    fn clone_string(source: &str, maximum_bytes: usize) -> Result<String, ValueError> {
        if source.len() > maximum_bytes {
            return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-field-too-large"));
        }
        let mut value = String::new();
        value.try_reserve_exact(source.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-string-admission"))?;
        value.push_str(source);
        Ok(value)
    }

    fn value_type_owned_bytes(source: &semio_framework_value::ValueType, maximum_bytes: usize) -> Result<usize, ValueError> {
        let mut bytes = 0usize;
        let mut value = source;
        loop {
            match value {
                semio_framework_value::ValueType::List(inner) => {
                    bytes = bytes.checked_add(size_of::<Box<semio_framework_value::ValueType>>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-size"))?;
                    if bytes > maximum_bytes {
                        return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-too-large"));
                    }
                    value = inner;
                }
                semio_framework_value::ValueType::Schema(schema) => {
                    bytes = bytes.checked_add(schema.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-size"))?;
                    return (bytes <= maximum_bytes).then_some(bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-too-large"));
                }
                _ => return Ok(bytes),
            }
        }
    }

    fn property_owned_bytes(source: &PropertyDef, maximum_bytes: usize) -> Result<usize, ValueError> {
        let value_type = Self::value_type_owned_bytes(&source.value_type, maximum_bytes)?;
        source.name.len().checked_add(source.expr.as_ref().map_or(0, String::len)).and_then(|bytes| bytes.checked_add(value_type)).filter(|bytes| *bytes <= maximum_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"jack-store.initializer-property-too-large"))
    }

    fn clone_property(source: &PropertyDef, maximum_bytes: usize) -> Result<PropertyDef, ValueError> {
        Self::property_owned_bytes(source, maximum_bytes)?;
        Ok(source.clone())
    }

    fn begin_kind(&mut self, source: &JackSnapshot, maximum_bytes: usize) -> Result<bool, ValueError> {
        let target = self.value.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-clone-target"))?;
        match self.phase {
            4 => {
                if self.index == 0 && target.manifest.node_kinds.capacity() == 0 {
                    target.manifest.node_kinds.try_reserve_exact(source.manifest.node_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-node-kind-admission"))?;
                }
                let Some(kind) = source.manifest.node_kinds.get(self.index) else {
                    self.phase = 5;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-node-property-admission"))?;
                let mut port_kinds = Vec::new();
                port_kinds.try_reserve_exact(kind.port_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-node-port-admission"))?;
                *self.active =
                    Some(JackSnapshotCloneKind::Node { source: self.index, property: 0, port: 0, value: crate::NodeKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, properties, port_kinds } });
                Ok(true)
            }
            5 => {
                if self.index == 0 && target.manifest.edge_kinds.capacity() == 0 {
                    target.manifest.edge_kinds.try_reserve_exact(source.manifest.edge_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-edge-kind-admission"))?;
                }
                let Some(kind) = source.manifest.edge_kinds.get(self.index) else {
                    self.phase = 6;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-edge-property-admission"))?;
                *self.active = Some(JackSnapshotCloneKind::Edge { source: self.index, property: 0, value: crate::EdgeKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, properties } });
                Ok(true)
            }
            6 => {
                if self.index == 0 && target.manifest.port_kinds.capacity() == 0 {
                    target.manifest.port_kinds.try_reserve_exact(source.manifest.port_kinds.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-port-kind-admission"))?;
                }
                let Some(kind) = source.manifest.port_kinds.get(self.index) else {
                    self.phase = 7;
                    self.index = 0;
                    return Ok(true);
                };
                let mut properties = Vec::new();
                properties.try_reserve_exact(kind.properties.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"jack-store.initializer-port-property-admission"))?;
                *self.active =
                    Some(JackSnapshotCloneKind::Port { source: self.index, property: 0, value: crate::PortKindDef { name: Self::clone_string(&kind.name, maximum_bytes)?, direction: kind.direction, properties } });
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub fn advance(&mut self, source: &JackSnapshot, maximum_bytes: usize) -> Result<JackSnapshotCloneStep, ValueError> {
        if maximum_bytes == 0 {
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes: 0 });
        }
        if let Some(active) = self.active.as_mut() {
            let (completed, copied_bytes) = match active {
                JackSnapshotCloneKind::Node { source: source_index, property, port, value } => {
                    let source = source.manifest.node_kinds.get(*source_index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-node-kind-stale"))?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else if let Some(kind) = source.port_kinds.get(*port) {
                        value.port_kinds.push(Self::clone_string(kind, maximum_bytes)?);
                        *port += 1;
                        (false, kind.len())
                    } else {
                        (true, 0)
                    }
                }
                JackSnapshotCloneKind::Edge { source: source_index, property, value } => {
                    let source = source.manifest.edge_kinds.get(*source_index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-edge-kind-stale"))?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else {
                        (true, 0)
                    }
                }
                JackSnapshotCloneKind::Port { source: source_index, property, value } => {
                    let source = source.manifest.port_kinds.get(*source_index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-port-kind-stale"))?;
                    if let Some(definition) = source.properties.get(*property) {
                        value.properties.push(Self::clone_property(definition, maximum_bytes)?);
                        *property += 1;
                        (false, Self::property_owned_bytes(definition, maximum_bytes)?)
                    } else {
                        (true, 0)
                    }
                }
            };
            if completed {
                let active = self.active.take().expect("completed Jack kind clone remains exact");
                let target = self.value.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-clone-target"))?;
                match active {
                    JackSnapshotCloneKind::Node { value, .. } => target.manifest.node_kinds.push(value),
                    JackSnapshotCloneKind::Edge { value, .. } => target.manifest.edge_kinds.push(value),
                    JackSnapshotCloneKind::Port { value, .. } => target.manifest.port_kinds.push(value),
                }
                self.index += 1;
            }
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes });
        }
        if self.begin_kind(source, maximum_bytes)? {
            return Ok(JackSnapshotCloneStep::Pending { copied_bytes: 0 });
        }
        let target = self.value.as_mut().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"jack-store.initializer-clone-target"))?;
        let copied_bytes = match self.phase {
            0 => {
                target.schema = Self::clone_string(&source.schema, maximum_bytes)?;
                source.schema.len()
            }
            1 => {
                target.name = Self::clone_string(&source.name, maximum_bytes)?;
                source.name.len()
            }
            2 => {
                target.manifest_id = source.manifest_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.manifest_id.as_deref().map_or(0, str::len)
            }
            3 => {
                target.camera = source.camera.clone();
                0
            }
            7 => {
                target.content.child_id = Self::clone_string(&source.content.child_id, maximum_bytes)?;
                if self.retain_local_owner {
                    if let Some(owner) = source.content.local_owner::<crate::JackContentOwner>() {
                        target.content.set_local_owner(owner);
                    }
                }
                source.content.child_id.len()
            }
            8 => {
                target.content.target.artifact_id = Self::clone_string(&source.content.target.artifact_id, maximum_bytes)?;
                source.content.target.artifact_id.len()
            }
            9 => {
                target.content.target.dialect.artifact_kind = Self::clone_string(&source.content.target.dialect.artifact_kind, maximum_bytes)?;
                source.content.target.dialect.artifact_kind.len()
            }
            10 => {
                target.content.target.dialect.standard = Self::clone_string(&source.content.target.dialect.standard, maximum_bytes)?;
                source.content.target.dialect.standard.len()
            }
            11 => {
                target.content.target.dialect.subset = Self::clone_string(&source.content.target.dialect.subset, maximum_bytes)?;
                source.content.target.dialect.subset.len()
            }
            12 => {
                target.root_node_id = source.root_node_id.as_deref().map(|value| Self::clone_string(value, maximum_bytes)).transpose()?;
                source.root_node_id.as_deref().map_or(0, str::len)
            }
            13 => {
                target.query = Self::clone_string(&source.query, maximum_bytes)?;
                source.query.len()
            }
            _ => {
                self.terminal = true;
                return Ok(JackSnapshotCloneStep::Complete);
            }
        };
        self.phase += 1;
        Ok(JackSnapshotCloneStep::Pending { copied_bytes })
    }

    fn step(&mut self, source: &JackSnapshot, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        let step = self.advance(source, JACK_OWNED_FIELD_BYTES)?;
        match step {
            JackSnapshotCloneStep::Pending { copied_bytes } => {
                cx.consume_fuel(copied_bytes.max(1) as u64);
                Ok(false)
            }
            JackSnapshotCloneStep::Complete => Ok(true),
        }
    }

    pub fn take_value(&mut self) -> Option<JackSnapshot> {
        if !self.terminal || self.active.is_some() {
            return None;
        }
        self.value.take()
    }

    /// 📏️ Quotes the next close turn of the partial clone: the retained child cursor, else the partial kind, else the partial snapshot.
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retirement.as_ref() {
            return store::artifact_retirement_box_demands(owner, body);
        }
        if self.active.is_some() {
            return store::artifact_retirement_owned_birth_demands(&self.active);
        }
        if self.value.is_some() {
            return store::artifact_retirement_owned_birth_demands(&self.value);
        }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if !admit_demand(grant, demand, "jack snapshot clone close exceeds admitted depth")? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if self.retirement.is_some() {
            return store::artifact_retirement_box_close_step(&mut self.retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.active.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.active, &mut self.retirement, grant);
        }
        if self.value.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.value, &mut self.retirement, grant);
        }
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.value.is_none() && self.active.is_none() && self.retirement.is_none()
    }
}

impl Drop for JackSnapshotCloneAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "Jack snapshot clone reached Drop before exact handoff or cursor retirement");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JackStoreInitializationPhase {
    BindGenesis,
    ValidateEnvelope,
    ValidateEdit { index: usize },
    CloneInitial,
    SeedHistory { edit: usize, lane: u8, index: usize },
    FoldSupersessions { transition: usize },
    FindApplied { position: usize },
    ApplyForward { position: usize, edit: usize, mutation: usize },
    CommitApplied { position: usize, edit: usize },
    FindRedo { position: usize },
    CommitRedo { position: usize, edit: usize },
    BuildOwners,
    BuildCandidate,
    RetireCancelled,
    RetireFault,
    Complete,
    Cancelled,
    Fault,
}

struct JackStoreInitializationAuthority {
    actor: std::mem::ManuallyDrop<Option<protocol::ActorId>>,
    actor_close: std::mem::ManuallyDrop<Option<ControlledRetirement<protocol::ActorId>>>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    envelope: std::mem::ManuallyDrop<Option<store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>>>,
    owners: std::mem::ManuallyDrop<Option<store::DocumentStoreOwners<JackSnapshot, TrinityGraphMutation>>>,
    runtime: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationRuntime<JackSnapshot>>>,
    candidate: std::mem::ManuallyDrop<Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>>>,
    displaced: std::mem::ManuallyDrop<Option<JackSnapshot>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    envelope_retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    clone: std::mem::ManuallyDrop<Option<JackSnapshotCloneAuthority>>,
    edit_index: store::ArtifactStoreInitializationEditIndex,
    phase: JackStoreInitializationPhase,
    resume_phase: Option<JackStoreInitializationPhase>,
    cancel_requested: bool,
    fault: std::mem::ManuallyDrop<Option<semio_framework_diagnostic::Fault>>,
    fault_close: std::mem::ManuallyDrop<Option<ControlledRetirement<semio_framework_diagnostic::Fault>>>,
    fault_publication: semio_framework_job::RetainedFaultPublication,
    terminal_handoff: bool,
}

impl JackStoreInitializationAuthority {
    fn new(envelope: store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, actor: protocol::ActorId) -> Self {
        Self {
            actor: std::mem::ManuallyDrop::new(Some(actor)),
            actor_close: std::mem::ManuallyDrop::new(None),
            operation,
            generation,
            envelope: std::mem::ManuallyDrop::new(Some(envelope)),
            owners: std::mem::ManuallyDrop::new(None),
            runtime: std::mem::ManuallyDrop::new(None),
            candidate: std::mem::ManuallyDrop::new(None),
            displaced: std::mem::ManuallyDrop::new(None),
            active: std::mem::ManuallyDrop::new(None),
            envelope_retirement: std::mem::ManuallyDrop::new(None),
            clone: std::mem::ManuallyDrop::new(None),
            edit_index: store::ArtifactStoreInitializationEditIndex::default(),
            resume_phase: None,
            phase: JackStoreInitializationPhase::ValidateEnvelope,
            cancel_requested: false,
            fault: std::mem::ManuallyDrop::new(None),
            fault_close: std::mem::ManuallyDrop::new(None),
            fault_publication: semio_framework_job::RetainedFaultPublication::new(),
            terminal_handoff: false,
        }
    }

    fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    fn fail(&mut self, code: &[u8]) {
        if self.fault.is_none() {
            *self.fault = Some(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::App, "trinity.jack.initializer", String::from_utf8_lossy(code).into_owned()));
        }
        self.phase = JackStoreInitializationPhase::RetireFault;
    }

    fn fail_with(&mut self, error: ValueError) {
        if self.fault.is_none() {
            let progress = error.retained_progress();
            *self.fault = Some(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, error.kind.as_str(), error.into_message()).with_retained_progress(progress));
        }
        self.phase = JackStoreInitializationPhase::RetireFault;
    }

    fn snapshot_factory() -> std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<JackSnapshot>> {
        std::sync::Arc::new(OwnedValueRetirementFactory::<JackSnapshot>::default())
    }

    /// 🧹️ Drains one displaced projection or settles one replaced workspace owner under the step's own retained wallet; answers whether another turn is owed first.
    fn settle_displaced(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        let grant = cx.retained_grant();
        if self.active.is_some() {
            let step = store::artifact_retirement_box_close_step(&mut self.active, grant)?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if self.displaced.is_some() {
            let step = store::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, grant)?;
            cx.consume_retained(step.progress())?;
            return Ok(true);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let step = runtime.settle_current_retirement_step(grant)?;
            cx.consume_retained(step.progress())?;
            return Ok(!matches!(step, RetainedCloneStep::Complete(_)));
        }
        Ok(false)
    }

    fn fault_is_empty(&self) -> bool {
        self.fault.is_none() && self.fault_close.is_none() && self.fault_publication.terminal_is_empty()
    }

    fn nothing_retained(&self) -> bool {
        self.envelope.is_none()
            && self.owners.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.displaced.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.clone.is_none()
            && self.actor.is_none()
            && self.actor_close.is_none()
    }

    /// 📏️ Quotes the next original-ownership close turn in the fixed order: displaced workspace, runtime, clone, envelope cursor, envelope with its catalog, bare catalog, actor.
    fn close_demands(&self, body: usize, include_fault: bool) -> Result<RetirementDemand, ValueError> {
        if include_fault {
            if !self.fault_publication.terminal_is_empty() {
                return self.fault_publication.retirement_demands();
            }
            if self.fault.is_some() {
                return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<semio_framework_diagnostic::Fault>>() + std::mem::size_of::<Option<ControlledRetirement<semio_framework_diagnostic::Fault>>>(), depth: 1, ..Default::default() });
            }
            if let Some(owner) = self.fault_close.as_ref() {
                return if owner.terminal_is_empty() {
                    Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<ControlledRetirement<semio_framework_diagnostic::Fault>>>(), depth: 1, ..Default::default() })
                } else {
                    nested(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
                };
            }
        }
        if let Some(owner) = self.active.as_ref() {
            return store::artifact_retirement_box_demands(owner, body);
        }
        if self.displaced.is_some() {
            return store::artifact_retirement_owned_birth_demands(&self.displaced);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            if runtime.terminal_is_empty() {
                return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<store::ArtifactStoreInitializationRuntime<JackSnapshot>>>(), depth: 1, ..Default::default() });
            }
            return runtime.initialization_retirement_demands(body);
        }
        if let Some(clone) = self.clone.as_ref() {
            if clone.terminal_is_empty() {
                return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<JackSnapshotCloneAuthority>>(), depth: 1, ..Default::default() });
            }
            return clone.close_demands(body);
        }
        if let Some(owner) = self.envelope_retirement.as_ref() {
            return store::artifact_retirement_box_demands(owner, body);
        }
        if self.envelope.is_some() {
            let Some(owners) = self.owners.as_ref() else {
                return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<store::DocumentStoreOwners<JackSnapshot, TrinityGraphMutation>>>(), capacity_bytes: store::bounded_artifact_store_owners_birth_bytes::<JackSnapshot, TrinityGraphMutation>(), depth: 1, ..Default::default() });
            };
            if !owners.constructor_is_complete() {
                return nested(owners.constructor_demands(body)?);
            }
            let mut demand = nested(owners.uninstalled_envelope_retirement_demands(self.envelope.as_ref().expect("retained envelope")))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(std::mem::size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "jack initializer envelope frame copy overflow"))?;
            return Ok(demand);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.uninstalled_owners_terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<store::DocumentStoreOwners<JackSnapshot, TrinityGraphMutation>>>(), depth: 1, ..Default::default() })
            } else {
                nested(owners.uninstalled_owners_demands(body)?)
            };
        }
        if let Some(actor) = self.actor_close.as_ref() {
            return if actor.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() })
            } else {
                nested(RetirementDemand { copy_bytes: actor.next_copy_byte_demand()?, capacity_bytes: actor.next_capacity_byte_demand(body)?, release_bytes: actor.next_release_byte_demand()?, depth: actor.next_depth_demand()? })
            };
        }
        if self.actor.is_some() {
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Option<protocol::ActorId>>() + std::mem::size_of::<Option<ControlledRetirement<protocol::ActorId>>>(), depth: 1, ..Default::default() });
        }
        Ok(Default::default())
    }

    /// ♻️ Closes the initializer's original ownership one funded turn at a time. The fault (with its publication) stays retained until the job's own close asks for it, so a fault outcome can still be lent; terminal emptiness is published only after the last owner is gone.
    fn close_original(&mut self, grant: RetainedCloneGrant, include_fault: bool) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.nothing_retained() && (!include_fault || self.fault_is_empty()) {
            self.terminal_handoff |= include_fault;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes, include_fault)?;
        if !admit_demand(grant, demand, "jack initializer close exceeds admitted depth")? {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if include_fault {
            if !self.fault_publication.terminal_is_empty() {
                return self.fault_publication.close_step(grant).map(|step| RetainedCloneStep::Progress(step.progress()));
            }
            if let Some(fault) = self.fault.take() {
                return match ControlledRetirement::new(fault) {
                    Ok(owner) => {
                        *self.fault_close = Some(owner);
                        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
                    }
                    Err((error, fault)) => {
                        *self.fault = Some(fault);
                        Err(error)
                    }
                };
            }
            if let Some(owner) = self.fault_close.as_mut() {
                if owner.terminal_is_empty() {
                    drop(self.fault_close.take());
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
                }
                let step = owner.step(child)?;
                let step = admit_retained_clone_close(child, step, owner.terminal_is_empty(), "jack initializer fault")?;
                return Ok(RetainedCloneStep::Progress(step.progress()));
            }
        }
        if self.active.is_some() {
            return store::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.displaced.is_some() {
            return store::artifact_retirement_admit_owned(&mut self.displaced, &mut self.active, grant);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            if runtime.terminal_is_empty() {
                drop(self.runtime.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = runtime.close_step(&Self::snapshot_factory(), grant)?;
            let step = admit_retained_clone_close(grant, step, runtime.terminal_is_empty(), "jack initialization runtime")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(clone) = self.clone.as_mut() {
            if clone.terminal_is_empty() {
                drop(self.clone.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = clone.close_step(grant)?;
            let step = admit_retained_clone_close(grant, step, clone.terminal_is_empty(), "jack initialization clone")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope_retirement.is_some() {
            return store::artifact_retirement_box_close_step(&mut self.envelope_retirement, grant).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let owners_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match store::bounded_artifact_store_owners::<JackSnapshot, TrinityGraphMutation>(owners_grant) {
                    Ok((owners, mut receipt)) => {
                        *self.owners = Some(owners);
                        receipt.copied_bytes += placement;
                        Ok(RetainedCloneStep::Progress(receipt))
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        Err(refused.error.with_retained_progress(refused.progress))
                    }
                };
            }
            let owners = self.owners.as_mut().expect("retained catalog");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, receipt)| error.with_retained_progress(receipt));
            }
            let placement = std::mem::size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();
            let envelope_grant = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("retained catalog");
            let envelope = self.envelope.take().expect("retained envelope");
            return match owners.retire_envelope_uninstalled(envelope, envelope_grant) {
                Ok((owner, mut receipt)) => {
                    *self.envelope_retirement = Some(owner);
                    receipt.copied_bytes += placement;
                    Ok(RetainedCloneStep::Progress(receipt))
                }
                Err((error, owners, envelope)) => {
                    *self.owners = Some(owners);
                    *self.envelope = Some(envelope);
                    Err(error)
                }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            let step = admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "jack initializer catalog")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(actor) = self.actor_close.as_mut() {
            if actor.terminal_is_empty() {
                drop(self.actor_close.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = actor.step(child)?;
            let step = admit_retained_clone_close(child, step, actor.terminal_is_empty(), "jack initializer actor")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(actor) = self.actor.take() {
            return match ControlledRetirement::new(actor) {
                Ok(owner) => {
                    *self.actor_close = Some(owner);
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }))
                }
                Err((error, actor)) => {
                    *self.actor = Some(actor);
                    Err(error)
                }
            };
        }
        Ok(RetainedCloneStep::Progress(empty))
    }

    /// 🧹️ One terminal close turn under the step's retained wallet; answers whether the initializer is empty.
    fn close_for_step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, ValueError> {
        let step = self.close_original(cx.retained_grant(), false)?;
        cx.consume_retained(step.progress())?;
        Ok(matches!(step, RetainedCloneStep::Complete(_)))
    }

    fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff && self.nothing_retained() && self.fault_is_empty()
    }
}

impl semio_framework_plugin::ArtifactStoreInitializationAuthority<JackSnapshot, TrinityGraphMutation> for JackStoreInitializationAuthority {
    fn retirement_demands(&self, maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        self.close_demands(maximum_copy_bytes, true)
    }

    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, ValueError> {
        if cx.operation() != self.operation || cx.generation() != self.generation {
            self.fail(b"jack-store.initializer-stale-authority");
        }
        if self.cancel_requested && !matches!(self.phase, JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::Cancelled) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
        if !matches!(self.phase, JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::RetireFault | JackStoreInitializationPhase::Cancelled | JackStoreInitializationPhase::Fault | JackStoreInitializationPhase::Complete) {
            match self.settle_displaced(cx) {
                Ok(true) => {
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                Ok(false) => {}
                Err(error) => self.fail_with(error),
            }
        }
        match self.phase {
            JackStoreInitializationPhase::BindGenesis => {
                let envelope = self.envelope.as_ref().expect("retained initializer genesis");
                let Some(actor) = self.actor.take() else {
                    self.fail(b"jack-store.initializer-actor-missing");
                    return Ok(None);
                };
                *self.runtime = Some(store::ArtifactStoreInitializationRuntime::new(&envelope.id, &envelope.schema, envelope.vcs.genesis.facts().share_snapshot(), envelope.vcs.genesis.facts().digest(), actor));
                self.phase = JackStoreInitializationPhase::SeedHistory { edit: 0, lane: 0, index: 0 };
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::ValidateEnvelope => {
                let Some(envelope) = self.envelope.as_ref() else {
                    self.fail(b"jack-store.initializer-envelope-missing");
                    return Ok(None);
                };
                if envelope.schema != crate::TRINITY_GRAPH_SCHEMA || envelope.id.is_empty() || envelope.id.len() > JACK_OWNED_FIELD_BYTES {
                    self.fail(b"jack-store.initializer-envelope-invalid");
                } else {
                    self.phase = JackStoreInitializationPhase::ValidateEdit { index: 0 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::ValidateEdit { index } => {
                let envelope = self.envelope.as_ref().expect("validated Jack envelope remains retained");
                match self.edit_index.admit(&envelope.vcs.edits, index, JACK_OWNED_FIELD_BYTES) {
                    store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = JackStoreInitializationPhase::BindGenesis,
                    store::ArtifactStoreInitializationEditAdmission::Admitted => self.phase = JackStoreInitializationPhase::ValidateEdit { index: index + 1 },
                    store::ArtifactStoreInitializationEditAdmission::Oversized | store::ArtifactStoreInitializationEditAdmission::Duplicate => self.fail(b"jack-store.initializer-duplicate-or-hostile-edit"),
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::CloneInitial => {
                let source = &self.envelope.as_ref().expect("Jack envelope remains retained during initial clone").vcs.genesis.facts().snapshot();
                let clone = self.clone.as_mut().expect("Jack initial clone authority remains retained");
                let complete = match clone.step(source, cx) {
                    Ok(complete) => complete,
                    Err(code) => {
                        self.fail(code.message.as_bytes());
                        return Ok(None);
                    }
                };
                if complete {
                    let initial = clone.take_value().expect("Jack initial snapshot was built one semantic field at a time");
                    drop(self.clone.take());
                    match self.runtime.as_mut().expect("retained initializer runtime").adopt_current_owned(initial, Self::snapshot_factory()) {
                        Ok(()) => self.phase = self.resume_phase.take().expect("retained mutation resume phase"),
                        Err(initial) => {
                            *self.displaced = Some(initial);
                            self.fail(b"initializer-owned-workspace-adoption");
                        }
                    }
                }
                Ok(None)
            }
            JackStoreInitializationPhase::SeedHistory { edit, lane, index } => {
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while causal history is seeded");
                let Some(entry) = envelope.vcs.edits.get(edit) else {
                    self.phase = JackStoreInitializationPhase::FoldSupersessions { transition: 0 };
                    return Ok(None);
                };
                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained while history is seeded");
                match lane {
                    0 => {
                        if let Err(error) = runtime.seed_mutation(protocol::MutationId(entry.id.clone())) {
                            self.fail(error.as_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            runtime.observe_sequence(entry.sequence_number);
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 1, index: 0 };
                        }
                    }
                    1 if index < entry.forwards.len() => {
                        let id = entry.mutation_meta.get(index).and_then(|meta| meta.mutation_id.clone()).or_else(|| entry.forwards[index].mutation_id()).unwrap_or_else(|| protocol::MutationId(format!("{}#{index}", entry.id)));
                        if let Err(error) = runtime.seed_edit_operation(&entry.id, id) {
                            self.fail(error.as_bytes());
                            self.phase = JackStoreInitializationPhase::RetireFault;
                        } else {
                            self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                        }
                    }
                    1 => self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane: 2, index: 0 },
                    2 if index < entry.mutation_meta.len() => {
                        runtime.observe_timestamp(entry.mutation_meta[index].timestamp);
                        self.phase = JackStoreInitializationPhase::SeedHistory { edit, lane, index: index + 1 };
                    }
                    _ => self.phase = JackStoreInitializationPhase::SeedHistory { edit: edit + 1, lane: 0, index: 0 },
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::FoldSupersessions { transition } => {
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while its supersessions fold");
                match self.runtime.as_mut().expect("Jack runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {
                    Ok(true) => self.phase = JackStoreInitializationPhase::FoldSupersessions { transition: transition + 1 },
                    Ok(false) => self.phase = JackStoreInitializationPhase::FindApplied { position: 0 },
                    Err(error) => {
                        self.fail(error.as_bytes());
                        self.phase = JackStoreInitializationPhase::RetireFault;
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::FindApplied { position } => {
                let Some(id) = self.applied_id(position) else {
                    let checkpoint = self.envelope.as_ref().and_then(|envelope| envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone()).or_else(|| envelope.vcs.checkpoints.last().map(|checkpoint| checkpoint.id.clone())));
                    self.runtime.as_mut().expect("Writer runtime remains retained").set_current_checkpoint_id(checkpoint);
                    self.phase = JackStoreInitializationPhase::FindRedo { position: 0 };
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-applied-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = JackStoreInitializationPhase::ApplyForward { position, edit: scan, mutation: 0 };
                } else {
                    self.fail(b"jack-store.initializer-applied-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::ApplyForward { position, edit, mutation } => {
                let needs_workspace = {
                    let envelope = self.envelope.as_ref().expect("retained initializer envelope");
                    let runtime = self.runtime.as_ref().expect("retained initializer runtime");
                    envelope.vcs.edits.get(edit).and_then(|entry| runtime.effective_forward(entry, mutation, &envelope.schema)).is_some_and(|effective| effective.operation().is_some())
                };
                if needs_workspace && self.runtime.as_mut().expect("retained initializer runtime").current_mut().is_none() {
                    self.resume_phase = Some(self.phase);
                    *self.clone = Some(JackSnapshotCloneAuthority::new());
                    self.phase = JackStoreInitializationPhase::CloneInitial;
                    cx.consume_fuel(1);
                    return Ok(None);
                }
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained while its forwards fold");
                let entry = envelope.vcs.edits.get(edit).expect("Jack applied edit remains retained");
                match self.runtime.as_mut().expect("Jack runtime remains retained while its forwards fold").fold_forward(entry, mutation, &envelope.schema, JACK_OWNED_FIELD_BYTES) {
                    Ok(store::ArtifactStoreInitializationForward::Folded { displaced, fuel }) => {
                        if let Some(previous) = displaced {
                            *self.displaced = Some(previous);
                        }
                        self.phase = JackStoreInitializationPhase::ApplyForward { position, edit, mutation: mutation + 1 };
                        cx.consume_fuel(fuel as u64);
                    }
                    Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = JackStoreInitializationPhase::CommitApplied { position, edit },
                    Err(_) => self.fail(b"jack-store.initializer-forward-encoding"),
                }
                Ok(None)
            }
            JackStoreInitializationPhase::CommitApplied { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack applied edit remains retained");

                let runtime = self.runtime.as_mut().expect("Writer runtime remains retained");
                if let Err(error) = runtime.push_applied_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fail(error.as_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {

                    self.phase = JackStoreInitializationPhase::FindApplied { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::FindRedo { position } => {
                let Some(id) = self.redo_id(position) else {
                    self.edit_index.clear();
                    self.phase = JackStoreInitializationPhase::BuildOwners;
                    return Ok(None);
                };
                let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);
                let envelope = self.envelope.as_ref().expect("Jack envelope remains retained");
                let Some(edit) = envelope.vcs.edits.get(scan) else {
                    self.fail(b"jack-store.initializer-redo-edit-missing");
                    return Ok(None);
                };
                if edit.id == id {
                    self.phase = JackStoreInitializationPhase::CommitRedo { position, edit: scan };
                } else {
                    self.fail(b"jack-store.initializer-redo-edit-missing");
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::CommitRedo { position, edit } => {
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Jack redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Writer runtime remains retained").push_redo_edit(entry, self.envelope.as_ref().expect("retained history ledger").vcs.edits.key_at(edit).expect("authoritative retained edit key")) {
                    self.fail(error.as_bytes());
                    self.phase = JackStoreInitializationPhase::RetireFault;
                } else {
                    self.phase = JackStoreInitializationPhase::FindRedo { position: position + 1 };
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::BuildOwners => {
                match store::bounded_artifact_store_owners::<JackSnapshot, TrinityGraphMutation>(cx.retained_grant()) {
                    Ok((owners, progress)) => {
                        *self.owners = Some(owners);
                        match cx.consume_retained(progress) {
                            Ok(()) => self.phase = JackStoreInitializationPhase::BuildCandidate,
                            Err(error) => self.fail_with(error),
                        }
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        let _ = cx.consume_retained(refused.progress);
                        self.fail_with(refused.error);
                    }
                }
                cx.consume_fuel(1);
                Ok(None)
            }
            JackStoreInitializationPhase::BuildCandidate => {
                let Some(candidate_generation) = self.generation.0.checked_add(1) else {
                    self.fail(b"jack-store.initializer-generation-exhausted");
                    return Ok(None);
                };
                let envelope = self.envelope.take().expect("Jack envelope remains retained until atomic store construction");
                let runtime = self.runtime.take().expect("Writer runtime remains retained until atomic store construction");
                let owners = self.owners.take().expect("Jack store catalog remains retained until atomic store construction");
                let candidate = store::ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, candidate_generation, owners);
                *self.candidate = Some(candidate);
                self.phase = JackStoreInitializationPhase::Complete;
                Ok(None)
            }
            JackStoreInitializationPhase::RetireCancelled | JackStoreInitializationPhase::RetireFault => match self.close_for_step(cx) {
                Ok(false) => Ok(None),
                Ok(true) => {
                    self.phase = if self.phase == JackStoreInitializationPhase::RetireCancelled { JackStoreInitializationPhase::Cancelled } else { JackStoreInitializationPhase::Fault };
                    Ok(None)
                }
                Err(error) => {
                    if self.fault.is_none() {
                        self.fail_with(error);
                    }
                    Ok(None)
                }
            },
            JackStoreInitializationPhase::Complete => semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None),
            JackStoreInitializationPhase::Cancelled => semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx),
            JackStoreInitializationPhase::Fault => match self.fault.as_ref() {
                Some(fault) => self.fault_publication.advance_from_fault(fault, cx),
                None => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "jack initializer fault phase lost its fault")),
            },
        }
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete if self.phase == JackStoreInitializationPhase::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::Fault if self.fault.is_some() => self.fault_publication.borrow_outcome(descriptor),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "jack initializer cannot resolve this outcome kind")),
        }
    }

    fn request_cancel(&mut self) {
        self.cancel_requested = true;
    }

    fn begin_close(&mut self) {
        self.cancel_requested = true;
        if !matches!(self.phase, JackStoreInitializationPhase::Cancelled | JackStoreInitializationPhase::Fault) {
            self.phase = JackStoreInitializationPhase::RetireCancelled;
        }
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.begin_close();
        self.close_original(grant, true)
    }

    fn take_candidate(&mut self) -> Option<store::ArtifactStore<JackSnapshot, TrinityGraphMutation>> {
        if self.phase != JackStoreInitializationPhase::Complete || self.terminal_handoff {
            return None;
        }
        let candidate = self.candidate.take()?;
        self.terminal_handoff = true;
        Some(candidate)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal_is_empty_inner()
    }
}

impl Drop for JackStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_inner(), "Jack store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn jack_document_store_initialization_job(
    envelope: store::ArtifactEnvelope<JackSnapshot, TrinityGraphMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<JackSnapshot, TrinityGraphMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(JackStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}

//#endregion 🔖️RetainedStoreInitialization

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
