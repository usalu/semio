//! 🧵️ Fixed-capacity retained command session shared by Puzzle 2d, 3d, and 5d.

use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::{ArtifactToolCompletion, EphemeralEmit, InteractionHoverState};
use semio_framework_plugin::{ArtifactApp, Emit, Fault};
use std::sync::Arc;

//#region 🔖️Limits
pub const PUZZLE_COMMAND_RAW_BYTES: usize = 8_192;
pub const PUZZLE_COMMAND_DECODED_ITEMS: usize = 512;
pub const PUZZLE_COMMAND_WORK_ITEMS: usize = 4_096;
pub const PUZZLE_COMMAND_OUTPUT_BYTES: usize = 262_144;
pub const PUZZLE_COMMAND_STEP_MICROS: u32 = 7_500;

pub fn puzzle_command_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(PUZZLE_COMMAND_RAW_BYTES, PUZZLE_COMMAND_DECODED_ITEMS, 1, PUZZLE_COMMAND_OUTPUT_BYTES, PUZZLE_COMMAND_STEP_MICROS, 1, 1)
}
//#endregion 🔖️Limits

//#region 🧵️Session
pub type PuzzleCommandReducer<A> = fn(
    &<A as ArtifactApp>::Command,
    &<A as ArtifactApp>::Snapshot,
    &<A as ArtifactApp>::Config,
    &protocol::InteractionState,
    &InteractionHoverState,
) -> Result<Emit<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, Fault>;

pub type PuzzleCommandExtent<A> = fn(&<A as ArtifactApp>::Command, &<A as ArtifactApp>::Snapshot, &protocol::InteractionState) -> Option<usize>;

pub enum PuzzleCommandWorkStep<A: ArtifactApp> {
    Progress { stage: &'static str, en: &'static str, de: &'static str },
    Complete(Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>),
}

pub trait PuzzleCommandWork<A: ArtifactApp>: Send {
    fn tool_id(&self) -> &'static str;
    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState) -> Option<usize>;
    fn step(
        &mut self,
        command: &A::Command,
        snapshot: &A::Snapshot,
        config: &A::Config,
        interaction: &protocol::InteractionState,
        hover: &InteractionHoverState,
    ) -> Result<PuzzleCommandWorkStep<A>, Fault>;
    fn begin_close(&mut self) {}
    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self) -> bool {
        true
    }
}

pub struct BoundedFirstStepCommandWork<A: ArtifactApp> {
    tool_id: &'static str,
    reducer: PuzzleCommandReducer<A>,
    extent: PuzzleCommandExtent<A>,
    consumed: bool,
}

impl<A: ArtifactApp> BoundedFirstStepCommandWork<A> {
    pub fn new(tool_id: &'static str, reducer: PuzzleCommandReducer<A>, extent: PuzzleCommandExtent<A>) -> Self {
        Self { tool_id, reducer, extent, consumed: false }
    }
}

impl<A: ArtifactApp> PuzzleCommandWork<A> for BoundedFirstStepCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        (self.extent)(command, snapshot, interaction)
    }

    fn step(
        &mut self,
        command: &A::Command,
        snapshot: &A::Snapshot,
        config: &A::Config,
        interaction: &protocol::InteractionState,
        hover: &InteractionHoverState,
    ) -> Result<PuzzleCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle-command-bounded-work-repeated"));
        }
        let emit = (self.reducer)(command, snapshot, config, interaction, hover)?;
        self.consumed = true;
        Ok(PuzzleCommandWorkStep::Complete(emit))
    }
}

pub struct NoopPuzzleCommandWork<A: ArtifactApp> {
    tool_id: &'static str,
    consumed: bool,
    marker: std::marker::PhantomData<fn() -> A>,
}

impl<A: ArtifactApp> NoopPuzzleCommandWork<A> {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, consumed: false, marker: std::marker::PhantomData }
    }
}

impl<A: ArtifactApp> PuzzleCommandWork<A> for NoopPuzzleCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &A::Command, _snapshot: &A::Snapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
        Some(1)
    }

    fn step(
        &mut self,
        _command: &A::Command,
        _snapshot: &A::Snapshot,
        _config: &A::Config,
        _interaction: &protocol::InteractionState,
        _hover: &InteractionHoverState,
    ) -> Result<PuzzleCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle-command-noop-repeated"));
        }
        self.consumed = true;
        Ok(PuzzleCommandWorkStep::Complete(Emit::default()))
    }
}

pub struct RetainedPuzzleCommandPayload<A: ArtifactApp> {
    pub command: A::Command,
    pub snapshot: Arc<A::Snapshot>,
    pub config: Arc<A::Config>,
    pub interaction_state: Arc<protocol::InteractionState>,
    pub interaction_hover: Arc<InteractionHoverState>,
    pub completion: ArtifactToolCompletion<A>,
    pub command_id: fn(&A::Command) -> &'static str,
    pub work: Box<dyn PuzzleCommandWork<A>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PuzzleCommandPhase {
    WirePages,
    WireBytes,
    Decode,
    Preflight,
    Work,
    Publish,
    Complete,
    Fault,
}

pub struct RetainedPuzzleCommandJob<A: ArtifactApp> {
    command: Option<A::Command>,
    snapshot: Option<Arc<A::Snapshot>>,
    config: Option<Arc<A::Config>>,
    interaction_state: Option<Arc<protocol::InteractionState>>,
    interaction_hover: Option<Arc<InteractionHoverState>>,
    completion: Option<ArtifactToolCompletion<A>>,
    command_id: fn(&A::Command) -> &'static str,
    work: Option<Box<dyn PuzzleCommandWork<A>>>,
    raw_input: Option<RetainedToolWireInput>,
    raw: [u8; PUZZLE_COMMAND_RAW_BYTES],
    raw_len: usize,
    raw_page_cursor: usize,
    raw_scan_cursor: usize,
    work_extent: usize,
    work_cursor: usize,
    emit: Option<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>,
    phase: PuzzleCommandPhase,
    closing: bool,
}

impl<A: ArtifactApp> RetainedPuzzleCommandJob<A> {
    pub fn new(payload: RetainedPuzzleCommandPayload<A>) -> Self {
        Self::from_payload(payload, None)
    }

    pub fn from_wire(payload: RetainedPuzzleCommandPayload<A>, input: RetainedToolWireInput) -> Self {
        Self::from_payload(payload, Some(input))
    }

    fn from_payload(payload: RetainedPuzzleCommandPayload<A>, raw_input: Option<RetainedToolWireInput>) -> Self {
        let phase = if raw_input.is_some() { PuzzleCommandPhase::WirePages } else { PuzzleCommandPhase::Preflight };
        Self {
            command: Some(payload.command),
            snapshot: Some(payload.snapshot),
            config: Some(payload.config),
            interaction_state: Some(payload.interaction_state),
            interaction_hover: Some(payload.interaction_hover),
            completion: Some(payload.completion),
            command_id: payload.command_id,
            work: Some(payload.work),
            raw_input,
            raw: [0; PUZZLE_COMMAND_RAW_BYTES],
            raw_len: 0,
            raw_page_cursor: 0,
            raw_scan_cursor: 0,
            work_extent: 0,
            work_cursor: 0,
            emit: None,
            phase,
            closing: false,
        }
    }

    fn retained_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
        cx.payload_from_bytes(stream, bytes).unwrap_or_else(|rejected| {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        })
    }

    fn checkpoint(&self, cx: &mut StepContext<'_>) -> StepOutcome {
        let state = [self.phase as u8, (self.raw_page_cursor & 0xff) as u8, (self.raw_scan_cursor & 0xff) as u8, (self.work_cursor & 0xff) as u8];
        StepOutcome::CheckpointReady(Checkpoint { state: Self::retained_payload(cx, JobPayloadStream::CheckpointState, &state), applied_progress: self.raw_page_cursor.saturating_add(self.raw_scan_cursor).saturating_add(self.work_cursor) as u64 })
    }

    fn preview(&self, cx: &mut StepContext<'_>, en: &str, de: &str) -> StepOutcome {
        let bytes = serde_json::to_vec(&serde_json::json!({ "en": en, "de": de })).unwrap_or_default();
        StepOutcome::PreviewReady(Self::retained_payload(cx, JobPayloadStream::Preview, &bytes))
    }

    fn fault(&mut self, cx: &mut StepContext<'_>, message: &'static [u8]) -> StepOutcome {
        self.phase = PuzzleCommandPhase::Fault;
        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, message) })
    }
}

impl<A: ArtifactApp> InteractiveJob for RetainedPuzzleCommandJob<A> {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if cx.should_yield() || cx.fuel_remaining() == 0 {
            return StepOutcome::Yield;
        }
        cx.consume_fuel(1);
        match self.phase {
            PuzzleCommandPhase::WirePages => {
                cx.set_stage("puzzle-command-wire-page");
                let Some(input) = self.raw_input.as_ref() else { return self.fault(cx, b"puzzle command lost retained wire input") };
                if let Some(page) = input.page(self.raw_page_cursor) {
                    let Some(end) = self.raw_len.checked_add(page.len()).filter(|end| *end <= self.raw.len()) else { return self.fault(cx, b"puzzle command wire input exceeds fixed byte capacity") };
                    self.raw[self.raw_len..end].copy_from_slice(page);
                    self.raw_len = end;
                    self.raw_page_cursor = self.raw_page_cursor.saturating_add(1);
                    return self.preview(cx, "Reading command page", "Befehlsseite wird gelesen");
                }
                self.phase = PuzzleCommandPhase::WireBytes;
                self.checkpoint(cx)
            }
            PuzzleCommandPhase::WireBytes => {
                cx.set_stage("puzzle-command-wire-byte");
                if self.raw_scan_cursor < self.raw_len {
                    self.raw_scan_cursor = self.raw_scan_cursor.saturating_add(1);
                    return self.checkpoint(cx);
                }
                self.phase = PuzzleCommandPhase::Decode;
                self.checkpoint(cx)
            }
            PuzzleCommandPhase::Decode => {
                cx.set_stage("puzzle-command-decode");
                let decoded = match <A::Command as protocol::OpBinary>::decode_op(&self.raw[..self.raw_len]) {
                    Ok(command) => command,
                    Err(_) => return self.fault(cx, b"puzzle command wire payload is malformed"),
                };
                let Some(work) = self.work.as_ref() else { return self.fault(cx, b"puzzle command work owner is absent") };
                if (self.command_id)(&decoded) != work.tool_id() {
                    return self.fault(cx, b"puzzle command wire authority does not match the admitted tool");
                }
                self.command = Some(decoded);
                self.phase = PuzzleCommandPhase::Preflight;
                self.preview(cx, "Validating command capacity", "Befehlskapazität wird geprüft")
            }
            PuzzleCommandPhase::Preflight => {
                cx.set_stage("puzzle-command-preflight");
                if self.work_extent == 0 {
                    let (Some(command), Some(snapshot), Some(interaction), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.interaction_state.as_ref(), self.work.as_ref()) else {
                        return self.fault(cx, b"puzzle command preflight owner is absent");
                    };
                    let Some(extent) = work.extent(command, snapshot, interaction).filter(|extent| *extent <= PUZZLE_COMMAND_WORK_ITEMS) else { return self.fault(cx, b"puzzle command exceeds fixed semantic work capacity") };
                    self.work_extent = extent.max(1);
                }
                if self.work_cursor < self.work_extent {
                    self.work_cursor = self.work_cursor.saturating_add(1);
                    return self.checkpoint(cx);
                }
                self.phase = PuzzleCommandPhase::Work;
                self.preview(cx, "Applying command", "Befehl wird angewendet")
            }
            PuzzleCommandPhase::Work => {
                cx.set_stage("puzzle-command-work");
                let (Some(command), Some(snapshot), Some(config), Some(interaction), Some(hover), Some(work)) = (self.command.as_ref(), self.snapshot.as_ref(), self.config.as_ref(), self.interaction_state.as_ref(), self.interaction_hover.as_ref(), self.work.as_mut()) else {
                    return self.fault(cx, b"puzzle command work owner is absent");
                };
                match work.step(command, snapshot, config, interaction, hover) {
                    Ok(PuzzleCommandWorkStep::Progress { stage, en, de }) => {
                        cx.set_stage(stage);
                        self.preview(cx, en, de)
                    }
                    Ok(PuzzleCommandWorkStep::Complete(emit)) => {
                        self.emit = Some(emit);
                        self.phase = PuzzleCommandPhase::Publish;
                        self.preview(cx, "Publishing result", "Ergebnis wird veröffentlicht")
                    }
                    Err(_) => return self.fault(cx, b"puzzle command reducer rejected the admitted operation"),
                }
            }
            PuzzleCommandPhase::Publish => {
                cx.set_stage("puzzle-command-publish");
                let Some(completion) = self.completion.as_ref() else { return self.fault(cx, b"puzzle command completion authority is absent") };
                if !completion.has_mounted_consumer() {
                    return self.fault(cx, b"puzzle command completion consumer is absent");
                }
                let Some(emit) = self.emit.take() else { return self.fault(cx, b"puzzle command result owner is absent") };
                if completion.complete(Ok(emit), EphemeralEmit::default()).is_err() {
                    return self.fault(cx, b"puzzle command result publication was rejected");
                }
                self.phase = PuzzleCommandPhase::Complete;
                StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
            }
            PuzzleCommandPhase::Complete => StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) }),
            PuzzleCommandPhase::Fault => self.fault(cx, b"puzzle command remains faulted"),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
        if let Some(work) = self.work.as_mut() {
            work.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if self.raw_len != 0 {
            if maximum_bytes == 0 {
                return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            self.raw_len -= 1;
            self.raw[self.raw_len] = 0;
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 1 };
        }
        if let Some(input) = self.raw_input.as_mut() {
            let step = input.close_step(maximum_items.min(1), maximum_bytes);
            if input.terminal_is_empty() {
                self.raw_input = None;
            }
            return step;
        }
        macro_rules! retire_one {
            ($field:ident) => {
                if self.$field.is_some() {
                    if maximum_items == 0 {
                        return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    drop(self.$field.take());
                    return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            };
        }
        retire_one!(emit);
        if let Some(work) = self.work.as_mut() {
            let step = work.close_step(maximum_items.min(1), maximum_bytes);
            if work.terminal_is_empty() {
                if maximum_items == 0 {
                    return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                }
                self.work = None;
                return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            return step;
        }
        retire_one!(command);
        retire_one!(snapshot);
        retire_one!(config);
        retire_one!(interaction_state);
        retire_one!(interaction_hover);
        if let Some(completion) = self.completion.as_ref() {
            if maximum_items == 0 || !completion.has_mounted_consumer() {
                return if maximum_items == 0 { InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 } } else { InteractiveJobCloseStep::Blocked };
            }
            self.completion = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.raw_len == 0
            && self.raw_input.is_none()
            && self.emit.is_none()
            && self.work.is_none()
            && self.command.is_none()
            && self.snapshot.is_none()
            && self.config.is_none()
            && self.interaction_state.is_none()
            && self.interaction_hover.is_none()
            && self.completion.is_none()
    }
}
//#endregion 🧵️Session

//#region 🧪️FixtureOracle
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    const VECTOR_IDS: &[&str] = &[
        "zero",
        "max",
        "maxPlusOne",
        "malformed",
        "staleGeneration",
        "wrongOperation",
        "abaGeneration",
        "cancelWirePage",
        "cancelWireByte",
        "cancelPreflight",
        "cancelWork",
        "cancelPublish",
        "faultWork",
        "retry",
        "close",
        "replay",
    ];

    #[derive(Debug, PartialEq, Eq)]
    struct PuzzleRetainedOracleOutput {
        owner: String,
        document_schema: String,
        payload_schema: String,
        tool_ids: Vec<String>,
        capacities: [u64; 6],
        locales: Vec<String>,
        vector_ids: Vec<String>,
        fingerprints: Vec<String>,
    }

    trait PuzzleRetainedFixtureOracle {
        fn evaluate(&self, fixture: &str) -> Result<PuzzleRetainedOracleOutput, String>;
    }

    struct SerdeJsonFixtureOracle;

    impl PuzzleRetainedFixtureOracle for SerdeJsonFixtureOracle {
        fn evaluate(&self, fixture: &str) -> Result<PuzzleRetainedOracleOutput, String> {
            let root: Value = serde_json::from_str(fixture).map_err(|error| error.to_string())?;
            let text = |key: &str| root.get(key).and_then(Value::as_str).map(str::to_string).ok_or_else(|| format!("fixture lacks {key}"));
            let capacities = root.get("capacities").ok_or_else(|| "fixture lacks capacities".to_string())?;
            let cap = |key: &str| capacities.get(key).and_then(Value::as_u64).ok_or_else(|| format!("fixture lacks capacity {key}"));
            let strings = |key: &str| {
                root.get(key)
                    .and_then(Value::as_array)
                    .ok_or_else(|| format!("fixture lacks {key}"))?
                    .iter()
                    .map(|value| value.as_str().map(str::to_string).ok_or_else(|| format!("fixture {key} contains a non-string")))
                    .collect::<Result<Vec<_>, _>>()
            };
            let vectors = root.get("vectors").and_then(Value::as_array).ok_or_else(|| "fixture lacks vectors".to_string())?;
            let vector_ids = vectors.iter().map(|vector| vector.get("id").and_then(Value::as_str).map(str::to_string).ok_or_else(|| "fixture vector lacks id".to_string())).collect::<Result<Vec<_>, _>>()?;
            let fingerprints = vectors.iter().map(|vector| vector.get("fingerprint").and_then(Value::as_str).map(str::to_string).ok_or_else(|| "fixture vector lacks fingerprint".to_string())).collect::<Result<Vec<_>, _>>()?;
            let locales = root
                .get("locales")
                .and_then(Value::as_object)
                .ok_or_else(|| "fixture lacks locales".to_string())?
                .keys()
                .cloned()
                .collect();
            Ok(PuzzleRetainedOracleOutput {
                owner: text("owner")?,
                document_schema: text("documentSchema")?,
                payload_schema: text("payloadSchema")?,
                tool_ids: strings("toolIds")?,
                capacities: [cap("rawBytes")?, cap("decodedItems")?, cap("workItems")?, cap("outputBytes")?, cap("stepMicros")?, cap("semanticUnitsPerGrant")?],
                locales,
                vector_ids,
                fingerprints,
            })
        }
    }

    fn expected(owner: &str, document_schema: &str, tools: &[&str]) -> PuzzleRetainedOracleOutput {
        PuzzleRetainedOracleOutput {
            owner: owner.into(),
            document_schema: document_schema.into(),
            payload_schema: format!("{document_schema}.tool-command.v1"),
            tool_ids: tools.iter().map(|tool| (*tool).to_string()).collect(),
            capacities: [PUZZLE_COMMAND_RAW_BYTES as u64, PUZZLE_COMMAND_DECODED_ITEMS as u64, PUZZLE_COMMAND_WORK_ITEMS as u64, PUZZLE_COMMAND_OUTPUT_BYTES as u64, PUZZLE_COMMAND_STEP_MICROS as u64, 1],
            locales: vec!["de".into(), "en".into()],
            vector_ids: VECTOR_IDS.iter().map(|vector| (*vector).to_string()).collect(),
            fingerprints: VECTOR_IDS.iter().map(|vector| if *vector == "maxPlusOne" { "8193:0:0:0:0:0" } else if *vector == "malformed" { "1:0:0:0:0:0" } else if matches!(*vector, "staleGeneration" | "wrongOperation" | "abaGeneration") { "1:1:1:0:0:0" } else { "0:0:0:0:0:0" }.to_string()).collect(),
        }
    }

    fn assert_fixture(fixture: &str, expected: PuzzleRetainedOracleOutput) {
        let oracle = SerdeJsonFixtureOracle;
        let actual = oracle.evaluate(fixture).expect("third-party fixture oracle parses");
        assert_eq!(actual, expected);
        let root: Value = serde_json::from_str(fixture).expect("fixture parses");
        assert!(root.pointer("/locales/en/cancel").and_then(Value::as_str).is_some());
        assert!(root.pointer("/locales/de/cancel").and_then(Value::as_str).is_some());
        assert!(root.pointer("/locales/default").is_none());
        assert!(root.get("vectors").and_then(Value::as_array).is_some_and(|vectors| vectors.iter().all(|vector| vector.get("control").is_some() || vector.get("authority").is_some() || vector.get("closeGrant").is_some())));
    }

    #[test]
    fn language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle() {
        assert_fixture(
            include_str!("../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️component.json"),
            expected("puzzle2d", "puzzle.2d.fixture", crate::editor::puzzle2d::PUZZLE2D_RETAINED_TOOL_IDS),
        );
        assert_fixture(
            include_str!("../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️component.json"),
            expected("puzzle3d", "puzzle.3d.fixture", crate::editor::puzzle3d::PUZZLE3D_RETAINED_TOOL_IDS),
        );
        assert_fixture(
            include_str!("../../🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️component.json"),
            expected("puzzle5d", "puzzle.5d", crate::editor::puzzle5d::PUZZLE5D_RETAINED_TOOL_IDS),
        );
    }

    #[test]
    fn hostile_fixture_mutations_change_the_oracle_result_or_fail_closed() {
        let fixture = include_str!("../../🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️component.json");
        let oracle = SerdeJsonFixtureOracle;
        let baseline = oracle.evaluate(fixture).expect("baseline");
        for mutated in [
            fixture.replacen("8192", "8193", 1),
            fixture.replacen("addNode", "missingNode", 1),
            fixture.replacen("maxPlusOne", "maxPlusTwo", 1),
            fixture.replacen("\"de\":", "\"fr\":", 1),
            fixture.replacen("0:0:0:0:0:0", "9:9:9:9:9:9", 1),
        ] {
            assert_ne!(oracle.evaluate(&mutated).expect("mutation remains parseable"), baseline);
        }
        assert!(oracle.evaluate(&fixture.replacen("\"fingerprint\": \"0:0:0:0:0:0\"", "\"missingFingerprint\": true", 1)).is_err());
    }
}
//#endregion 🧪️FixtureOracle
