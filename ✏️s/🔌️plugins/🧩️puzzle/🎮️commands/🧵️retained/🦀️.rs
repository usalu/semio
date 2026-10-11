//! 🧵️ Fixed-capacity retained command session shared by Puzzle 2d, 3d, and 5d.

use semio_framework_job::{InteractiveJobCloseStep, StepContext};
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, InteractionHoverState};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactApp, Emit, Fault, ViewModel};
use semio_framework_value::{RetirementDemand, ValueError};

//#region 🔖️Limits
pub const PUZZLE_COMMAND_RAW_BYTES: usize = 8_192;
pub const PUZZLE_COMMAND_DECODED_ITEMS: usize = 512;
pub const PUZZLE_COMMAND_WORK_ITEMS: usize = 4_096;
pub const PUZZLE_COMMAND_OUTPUT_BYTES: usize = 262_144;
pub const PUZZLE_COMMAND_STEP_MICROS: u32 = 7_500;
pub const PUZZLE_COMMAND_CHECKPOINT_BYTES: usize = 120;

/// 📥️ Largest file ONE puzzle import may carry — the budget one export may stream ([`PUZZLE_COMMAND_OUTPUT_BYTES`]), so
/// a file a puzzle app wrote is always a file it can read back.
pub const PUZZLE_IMPORT_TOTAL_BYTES: usize = PUZZLE_COMMAND_OUTPUT_BYTES;

/// 📏️ Raw wire bytes of the largest command a puzzle route carries: one WHOLE import (the framework reassembles the
/// picked file before `importSnapshot` runs, `semio_framework::kernel::ImportStaging`) as a JSON string — at most two
/// wire bytes per text byte once escaped (`"`, `\` and the JSON whitespace escapes) — plus one command envelope.
pub const PUZZLE_IMPORT_RAW_BYTES: usize = 2 * PUZZLE_IMPORT_TOTAL_BYTES + PUZZLE_COMMAND_RAW_BYTES;

/// 📏️ Bytes one `WireBytes` ladder step admits from the assembled wire owner.
///
/// 🧨️ Derived from the wire's OWN page extent ([`semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES`]),
/// never a literal, because the ladder exists to make the scan RESUMABLE at the granularity the pages
/// arrived in — not to spend a host turn per byte. Before ticket 26/09/02/PUZZLE-3D-END-TO-END wave B59
/// the stride was one byte AND every one of those steps published a checkpoint whose `input_hash`
/// re-folded the whole buffer, so the ladder cost O(bytes²): a 160 314-byte `importSnapshot` (the browser's
/// 145 924-byte Nakagin payload) reached scan cursor ≈28 000 in 60 s and the import never reached
/// `Decode` at all — no document edit, no history row and no notice, which is exactly the silent
/// `paneObjects=180→180` the live `import-distinct` verdict read.
pub const PUZZLE_COMMAND_WIRE_SCAN_STRIDE_BYTES: usize = semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES;

pub fn puzzle_command_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(PUZZLE_COMMAND_RAW_BYTES, PUZZLE_COMMAND_DECODED_ITEMS, 1, PUZZLE_COMMAND_OUTPUT_BYTES, PUZZLE_COMMAND_STEP_MICROS, 1, 1)
}
//#endregion 🔖️Limits

//#region 🧵️Work
/// 🧩️ A bounded reducer one retained step runs exactly once; `view_state` is the admitted context's own.
pub type PuzzleCommandReducer<A> = fn(
    &<A as ArtifactApp>::Command,
    &<A as ArtifactApp>::Snapshot,
    &<A as ArtifactApp>::Config,
    &protocol::InteractionState,
    &InteractionHoverState,
    Option<&ViewModel>,
) -> Result<Emit<<A as ArtifactApp>::Mutation, <A as ArtifactApp>::ConfigMutation, <A as ArtifactApp>::DraftMutation>, Fault>;

pub type PuzzleCommandExtent<A> = fn(&<A as ArtifactApp>::Command, &<A as ArtifactApp>::Snapshot, &protocol::InteractionState) -> Option<usize>;

/// 📋️ One literal progress step whose `{"en":…,"de":…}` preview is assembled at compile time.
#[macro_export]
macro_rules! puzzle_progress_step {
    ($stage:expr, $en:literal, $de:literal) => {
        semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Progress { stage: $stage, preview: concat!("{\"en\":\"", $en, "\",\"de\":\"", $de, "\"}").as_bytes() }
    };
}

/// 🧮️ The transient bytes one puzzle work step may materialize: the work's own frame plus the one emit it can stage.
pub fn step_demands<A: ArtifactApp>(work_bytes: usize) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand { copy_bytes: work_bytes.saturating_add(std::mem::size_of::<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>>()), depth: 1, ..Default::default() })
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

impl<A: ArtifactApp> ArtifactCommandWork<A> for BoundedFirstStepCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<A>>) -> Option<usize> {
        (self.extent)(command, snapshot, interaction)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, A>, _maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        step_demands::<A>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, A>, _cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle-command-bounded-work-repeated"));
        }
        let emit = (self.reducer)(input.command, input.snapshot, input.config, input.interaction, input.hover, input.context.and_then(|context| context.view_state.as_ref()))?;
        self.consumed = true;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
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

impl<A: ArtifactApp> ArtifactCommandWork<A> for NoopPuzzleCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, _command: &A::Command, _snapshot: &A::Snapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<A>>) -> Option<usize> {
        Some(1)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, A>, _maximum_copy_bytes: usize) -> Result<RetirementDemand, ValueError> {
        step_demands::<A>(std::mem::size_of::<Self>())
    }

    fn step(&mut self, _input: &ArtifactCommandInputs<'_, A>, _cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle-command-noop-repeated"));
        }
        self.consumed = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}
//#endregion 🧵️Work

/// 🧪️ Drives a puzzle command work turn by turn the way the framework's retained command job does.
#[cfg(test)]
pub mod testing {
    use super::*;
    use semio_framework_job::{Generation, OperationId, StepBudget};
    use semio_framework_plugin::app::ArtifactToolCompletion;
    use semio_framework_plugin::{AppOperationContext, HistoryView};
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};

    pub use crate::puzzle_job::testing::unbounded_grant;

    /// 🧾️ One work step as a test reads it: the progress preview split into its two languages, an emission, or a segmented download.
    pub enum PuzzleCommandWorkStep<A: ArtifactApp> {
        Progress { stage: &'static str, en: String, de: String },
        Complete(Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>),
        Download(semio_framework_plugin::app::ArtifactDownloadOutput),
    }

    fn preview_languages(preview: &[u8]) -> (String, String) {
        let text = std::str::from_utf8(preview).expect("a progress preview is utf-8");
        let json = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("a progress preview is json");
        let language = |key: &str| json.get(key).and_then(semio_framework_pack_json::Value::as_str).unwrap_or_default().to_string();
        (language("en"), language("de"))
    }

    /// ▶️ Runs one step of `work` under an unbounded budget and the bare operation facts a test needs.
    pub fn step<A: ArtifactApp, W: ArtifactCommandWork<A> + ?Sized>(
        work: &mut W,
        command: &A::Command,
        snapshot: &A::Snapshot,
        config: &A::Config,
        interaction: &protocol::InteractionState,
        hover: &InteractionHoverState,
        context: Option<&ArtifactOwnedToolJobContext<A>>,
    ) -> Result<PuzzleCommandWorkStep<A>, Fault> {
        let history = HistoryView::empty();
        let operation = AppOperationContext { app_instance_id: 1, parent_document_id: "test".to_string(), operation_id: 1, generation: 1, canonical_base_revision: [0; 32], retained: unbounded_grant(), authoring_seed: String::new() };
        let input = ArtifactCommandInputs { command, snapshot, snapshot_owner: None, config, history: &history, interaction, hover, context, operation: &operation };
        let mut sequence = 0u64;
        let mut progress = RetainedCloneProgress::default();
        let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(u64::MAX, u64::MAX, unbounded_grant()), semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut progress);
        Ok(match work.step(&input, &mut cx)? {
            ArtifactCommandWorkStep::Progress { stage, preview } | ArtifactCommandWorkStep::Replay { stage, preview } => {
                let (en, de) = preview_languages(preview);
                PuzzleCommandWorkStep::Progress { stage, en, de }
            }
            ArtifactCommandWorkStep::Complete(emit) | ArtifactCommandWorkStep::CompleteWithEphemeral { emit, .. } => PuzzleCommandWorkStep::Complete(emit),
            ArtifactCommandWorkStep::CompleteDownload { download, .. } => PuzzleCommandWorkStep::Download(download),
        })
    }

    /// 🧹️ Closes `work` to its terminal-empty shell and answers how many granted turns that took.
    pub fn close<A: ArtifactApp, W: ArtifactCommandWork<A> + ?Sized>(work: &mut W) -> usize {
        work.begin_close();
        let mut turns = 0;
        while !matches!(work.close_step(unbounded_grant()), InteractiveJobCloseStep::Complete { .. }) {
            turns += 1;
            assert!(turns < 100_000, "a work closes in a bounded number of turns");
        }
        assert!(work.terminal_is_empty(), "a closed work is terminal-empty");
        turns
    }

    /// 🔕️ A completion no test needs: tests that exercise a work never publish.
    pub type NoCompletion<A> = Option<ArtifactToolCompletion<A>>;
}
