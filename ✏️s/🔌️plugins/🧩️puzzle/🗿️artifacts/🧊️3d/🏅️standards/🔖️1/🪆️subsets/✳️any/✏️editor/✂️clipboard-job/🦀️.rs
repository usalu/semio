//! ✂️ Clipboard job: the one-step reserved copy, cut and paste job and the cut operations.

use super::*;
use semio_s_artifact_procedural_3d_delete_selection as delete_selection;

/// ♻️ The owners one `Puzzle3dClipboardJob` still holds when it closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub(super) struct Puzzle3dClipboardOwners {
    pub args: Option<semio_framework_value::DslValue>,
    pub(super) interaction: Option<semio_framework::InteractionState>,
    pub(super) hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    pub tool_id: String,
    pub snapshot: Option<std::sync::Arc<Puzzle3dPlaySnapshot>>,
    raw_wire: Vec<u8>,
    pub(super) completion: Option<semio_framework_plugin::app::ArtifactToolCompletion<EditorApp<Puzzle3dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle3dPlayApp>>>,
}

/// 📋️ One-step reserved copy/cut/paste job — the framework route is an empty stub unless the app owns this producer.
pub(super) struct Puzzle3dClipboardJob {
    pub tool_id: String,
    pub snapshot: Option<std::sync::Arc<Puzzle3dPlaySnapshot>>,
    raw_wire: Vec<u8>,
    pub args: Option<semio_framework_value::DslValue>,
    pub(super) interaction: Option<semio_framework::InteractionState>,
    pub(super) hover: Option<semio_framework_plugin::app::InteractionHoverState>,
    pub(super) completion: Option<semio_framework_plugin::app::ArtifactToolCompletion<EditorApp<Puzzle3dPlayApp>>>,
    rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<Puzzle3dPlayApp>>>,
    outbox: semio_s_plugin_puzzle_jobs::JobOutbox,
    closing: bool,
    pub(super) owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dClipboardOwners>,
}

impl Puzzle3dClipboardJob {
    pub fn new(request: ArtifactReservedToolJobRequest<EditorApp<Puzzle3dPlayApp>>) -> Self {
        let ArtifactReservedToolInput::Action { args, interaction, hover } = request.input else { unreachable!("clipboard routes carry action input") };
        Self { tool_id: request.tool_id, snapshot: Some(request.snapshot), raw_wire: request.raw_wire, args, interaction: Some(interaction), hover: Some(hover), completion: Some(request.completion), rejection: None, outbox: Default::default(), closing: false, owners: Default::default() }
    }

    pub fn emit(&mut self) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation, NoDraftMutation> {
        let (Some(interaction), Some(hover)) = (self.interaction.as_ref(), self.hover.as_ref()) else {
            return Emit::default();
        };
        let args = self.args.as_ref();
        let Some(snapshot) = self.snapshot.clone() else { return Emit::default() };
        let scene_snapshot = puzzle3d_scene_snapshot_from_play_snapshot(snapshot.as_ref());
        let marks = Puzzle3dInteractionSnapshot::from_state(&interaction, &hover);
        match self.tool_id.as_str() {
            "copy" => {
                let selected = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
                match puzzle3d_copy_fragment_from(&scene_snapshot, selected) {
                    Ok(fragment) => Emit { effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            "cut" => {
                let objects = puzzle3d_selected_objects_from(&marks, &scene_snapshot);
                let Ok(fragment) = puzzle3d_copy_fragment_from(&scene_snapshot, objects) else { return Emit::default() };
                let mutations = puzzle3d_cut_operations_from(snapshot.typed(), &marks);
                Emit { artifact_mutations: mutations, effects: vec![Effect::ClipboardWrite { fragment }], ..Default::default() }
            },
            "paste" => {
                let Some(args) = args else { return Emit::default() };
                let Some(fragment) = args.get("fragment").and_then(|value| FromValue::from_value(value.clone()).ok()) else { return Emit::default() };
                let placement = PastePlacement::default();
                match puzzle3d_paste_operations(&fragment, &placement) {
                    Ok(mutations) => Emit { artifact_mutations: mutations, ..Default::default() },
                    Err(_) => Emit::default(),
                }
            }
            _ => Emit::default(),
        }
    }
}

impl Puzzle3dClipboardJob {
    /// 🪜️ The step body; its value refusals are lifted into the original failure at the trait boundary.
    fn step_value<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        match self.outbox.phase(cx)? {
            semio_s_plugin_puzzle_jobs::OutboxPhase::Building => return self.outbox.advance(cx).map_err(semio_framework_job::OriginalTurnFailure::from),
            semio_s_plugin_puzzle_jobs::OutboxPhase::Delivered | semio_s_plugin_puzzle_jobs::OutboxPhase::Retiring => {
                self.outbox.retire_step(cx)?;
                return Ok(None);
            }
            semio_s_plugin_puzzle_jobs::OutboxPhase::Idle => {}
        }
        let turn = self.turn(cx);
        self.outbox.settle(turn, cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

impl InteractiveJob for Puzzle3dClipboardJob {
    fn step<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{self.step_value(cx).map_err(semio_framework_job::OriginalTurnFailure::from)}

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        self.outbox.borrow_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        if std::mem::replace(&mut self.closing, true) {
            return;
        }
        self.owners.stage(Puzzle3dClipboardOwners { args: self.args.take(), interaction: self.interaction.take(), hover: self.hover.take(), tool_id: std::mem::take(&mut self.tool_id), snapshot: self.snapshot.take(), raw_wire: std::mem::take(&mut self.raw_wire), completion: self.completion.take(), rejection: self.rejection.take() });
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { semio_framework_job::close_original_nested_step(cx, |cx, _grant| {
        self.begin_close();
        semio_s_plugin_puzzle_jobs::job_close_step(&mut self.outbox, &mut self.owners, cx)
    }) }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, 0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, 0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(semio_s_plugin_puzzle_jobs::job_close_demands(&self.outbox, &self.owners, 0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.outbox.terminal_is_empty() && self.owners.is_empty()
    }
}

impl Puzzle3dClipboardJob {
    pub fn turn(&mut self, cx: &StepContext<'_>) -> semio_s_plugin_puzzle_jobs::JobTurn {
        if self.closing || cx.is_cancelled() {
            return semio_s_plugin_puzzle_jobs::JobTurn::Cancelled;
        }
        if !self.raw_wire.is_empty() {
            return semio_s_plugin_puzzle_jobs::JobTurn::Prepare(std::mem::take(&mut self.raw_wire));
        }
        let emit = self.emit();
        let Some(completion) = self.completion.as_ref() else { return semio_s_plugin_puzzle_jobs::JobTurn::Fault(b"puzzle3d-clipboard-completion-owner".to_vec()) };
        if let Err(rejected) = completion.complete(Ok(emit), EphemeralEmit::default()) {
            self.rejection = Some(rejected);
            return semio_s_plugin_puzzle_jobs::JobTurn::Fault(b"puzzle3d-clipboard-completion-rejected".to_vec());
        }
        semio_s_plugin_puzzle_jobs::JobTurn::Complete
    }
}

impl ArtifactReservedJob for Puzzle3dClipboardJob {}

/// ✂️ Copies then deletes the selected objects as one mutation list.
pub(super) fn puzzle3d_cut_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, interaction: &InteractionView<'_>) -> Vec<Puzzle3dMutation> {
    puzzle3d_cut_operations_from(doc.snapshot.typed(), &Puzzle3dInteractionSnapshot::from_interaction(interaction))
}

pub fn puzzle3d_cut_operations_from(before: &Puzzle3dSnapshot, marks: &Puzzle3dInteractionSnapshot) -> Vec<Puzzle3dMutation> {
    let host = puzzle3d_scene_snapshot_from_document(before);
    let ids: Vec<String> = puzzle3d_selected_objects_from(marks, &host).into_iter().map(|object| object.id).collect();
    semio_s_artifact_puzzle_3d_commands_document::delete_selection::delete_selection_mutations(before, &ids, &HashSet::new(), &[], &[], &[])
}
