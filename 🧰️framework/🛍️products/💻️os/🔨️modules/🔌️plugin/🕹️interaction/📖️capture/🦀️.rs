//! 📖️ Exact frozen local-interaction reading; transport mounting and authority capture remain at the app owner.

use protocol::{DomainSelection, InteractionState, LocalInteractionIdentity, SelectionMode};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonArray as JsonArray, ArtifactCanonicalJsonNode as JsonNode, ArtifactCanonicalJsonObject as JsonObject, ArtifactCanonicalJsonReader, ArtifactCanonicalJsonValue as JsonValue, ArtifactStoreOneItemGrant,
    ErasedSnapshotRetirement, SnapshotRead, SnapshotRetirementFactory,
};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

//#region 🔒️CapturedRoot
struct CapturedRoot {
    read: Option<SnapshotRead<InteractionState>>,
    identity: LocalInteractionIdentity,
    generation: [u8; 20],
    generation_start: usize,
    revisions: [[u8; 64]; 3],
}

impl CapturedRoot {
    fn new(read: SnapshotRead<InteractionState>, identity: LocalInteractionIdentity) -> Self {
        let mut generation = [b'0'; 20];
        let mut value = identity.generation;
        let mut generation_start = 19;
        loop {
            generation[generation_start] = b'0' + (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
            generation_start -= 1;
        }
        let mut revisions = [[0; 64]; 3];
        for (output, input) in revisions.iter_mut().zip([&identity.revision, &identity.document_revision, &identity.topology_revision]) {
            for (index, byte) in input.iter().copied().enumerate() {
                output[index * 2] = b"0123456789abcdef"[(byte >> 4) as usize];
                output[index * 2 + 1] = b"0123456789abcdef"[(byte & 15) as usize];
            }
        }
        Self { read: Some(read), identity, generation, generation_start, revisions }
    }

    fn identity_json(&self) -> JsonValue<'_> {
        JsonValue::Object(JsonObject::new(
            [
                ("appInstanceId", JsonValue::Scalar(JsonNode::U64(self.identity.app_instance_id as u64))),
                ("documentRevision", text(std::str::from_utf8(&self.revisions[1]).expect("fixed hex is ASCII"))),
                ("generation", text(std::str::from_utf8(&self.generation[self.generation_start..]).expect("fixed decimal is ASCII"))),
                ("revision", text(std::str::from_utf8(&self.revisions[0]).expect("fixed hex is ASCII"))),
                ("topologyRevision", text(std::str::from_utf8(&self.revisions[2]).expect("fixed hex is ASCII"))),
            ]
            .into_iter(),
        ))
    }
}

impl ArtifactCanonicalJson for CapturedRoot {
    fn canonical_json_borrowed_root(&self) -> Result<Option<JsonValue<'_>>, String> {
        let state = self.read.as_ref().ok_or_else(|| "local-interaction.capture-root-returned".to_string())?.get();
        Ok(Some(JsonValue::Object(JsonObject::new([("identity", self.identity_json()), ("state", state_json(state))].into_iter()))))
    }
}

fn text(value: &str) -> JsonValue<'_> {
    JsonValue::Scalar(JsonNode::String(value))
}

fn state_json(state: &InteractionState) -> JsonValue<'_> {
    JsonValue::Object(JsonObject::new(
        [
            ("activeGranularity", JsonValue::Object(JsonObject::new(state.active_granularity.iter().map(|(key, value)| (key.as_str(), text(value)))))),
            (
                "activeMode",
                JsonValue::Object(JsonObject::new(state.active_mode.iter().map(|(key, value)| {
                    (
                        key.as_str(),
                        text(match value {
                            SelectionMode::Single => "single",
                            SelectionMode::Multiple => "multiple",
                        }),
                    )
                }))),
            ),
            ("selection", JsonValue::Object(JsonObject::new(state.selection.iter().map(|(key, value)| (key.as_str(), selection_json(value)))))),
        ]
        .into_iter(),
    ))
}

fn selection_json(selection: &DomainSelection) -> JsonValue<'_> {
    JsonValue::Object(JsonObject::new(
        [selection.anchor_id.as_deref().map(|anchor| ("anchorId", text(anchor))), Some(("granularity", text(&selection.granularity))), Some(("ids", JsonValue::Array(JsonArray::new(selection.ids.iter().map(|id| text(id))))))].into_iter().flatten(),
    ))
}
//#endregion 🔒️CapturedRoot

//#region ♻️ExactReadReturn
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct CapturedRootRetirementFactory;
struct CapturedRootRetirement {
    owned: ManuallyDrop<CapturedRootRetirementState>,
}
struct CapturedRootRetirementState {
    root: Option<Arc<CapturedRoot>>,
}

impl SnapshotRetirementFactory<CapturedRoot> for CapturedRootRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<CapturedRoot>) -> usize { std::mem::size_of::<CapturedRootRetirement>() }

    fn retire(&self, root: Arc<CapturedRoot>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<CapturedRoot>)> {
        match (RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<CapturedRootRetirement>(), depth: 1 }).admit(grant) {
            Ok(progress) => Ok((Box::new(CapturedRootRetirement { owned: ManuallyDrop::new(CapturedRootRetirementState { root: Some(root) }) }), progress)),
            Err(error) => Err((error, root)),
        }
    }
}

impl CapturedRootRetirement {
    fn demands(&self) -> RetirementDemand {
        RetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() }
    }
}

impl ErasedSnapshotRetirement for CapturedRootRetirement {
    /// 🧹️ Hands the captured lease back to its exact registry. The registry's ACCEPTANCE is the
    /// witness; reclaiming the accepted slot into an owned-value retirement belongs to the Store's
    /// own one-slot-per-step cursor, which this owner must never wait on.
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(idle));
        }
        if grant.maximum_depth < self.demands().depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "local interaction capture return exceeds admitted depth"));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        if let Some(root) = self.owned.root.take() {
            if let Some(mut root) = Arc::into_inner(root) {
                if !root.read.take().is_some_and(SnapshotRead::return_to_registry) {
                    return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "local-interaction.capture-read-return"));
                }
            }
        }
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..idle }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.owned.root.is_none()
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demands().copy_bytes)
    }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.demands().capacity_bytes)
    }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demands().release_bytes)
    }

    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demands().depth)
    }
}

impl Drop for CapturedRootRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("local interaction capture dropped before its exact Store read return was reclaimed");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.owned);
        }
    }
}
//#endregion ♻️ExactReadReturn

//#region 📖️BoundedCapture
/// 📖️ Writes caller-owned pages without cloning the state or allocating a complete encoded document.
pub(crate) struct LocalInteractionCaptureCursor {
    reader: ArtifactCanonicalJsonReader<CapturedRoot>,
    identity: LocalInteractionIdentity,
}

impl LocalInteractionCaptureCursor {
    /// 🔒️ The app owner supplies the exact immutable read and current fixed-width authority identity.
    pub(crate) fn new(read: SnapshotRead<InteractionState>, identity: LocalInteractionIdentity) -> Self {
        Self { identity: identity.clone(), reader: ArtifactCanonicalJsonReader::new(Arc::new(CapturedRoot::new(read, identity)), Arc::new(CapturedRootRetirementFactory)) }
    }

    pub(crate) fn identity(&self) -> &LocalInteractionIdentity {
        &self.identity
    }
    pub(crate) fn write_chunk(&mut self, grant: ArtifactStoreOneItemGrant, output: &mut [u8]) -> Result<usize, store::ArtifactCanonicalJsonEncodeError> {
        self.reader.encode_chunk(grant, output)
    }
    pub(crate) fn completed_bytes(&self) -> u64 {
        self.reader.completed_bytes()
    }
    pub(crate) fn complete(&self) -> bool {
        self.reader.is_complete()
    }
    pub(crate) fn cancel(&mut self) {
        self.reader.cancel();
        self.reader.begin_close();
    }
    pub(crate) fn begin_close(&mut self) {
        self.reader.begin_close();
    }
    pub(crate) fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.reader.retirement_demands(body)
    }
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.reader.close_step(grant)
    }
    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.reader.terminal_is_empty()
    }
}
//#endregion 📖️BoundedCapture

#[cfg(test)]
#[path = "🧪️tests/📖️capture/🦀️.rs"]
mod tests;
