//! 🎯️ Focus selection work: the staged focus that sums and measures the selected objects page by page before it publishes the result.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dFocusSelectionStage {
    Selection,
    SumObjects,
    DistanceObjects,
    Publish,
    Complete,
    Closing,
}

/// ♻️ The owners one `Puzzle3dFocusSelectionWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dFocusSelectionWorkOwners {
    pub(super) selected: HashSet<String>,
}

pub struct Puzzle3dFocusSelectionWork {
    pub(super) stage: Puzzle3dFocusSelectionStage,
    selection_cursor: usize,
    pub(super) object_cursor: usize,
    pub(super) selected: HashSet<String>,
    center: [f64; 3],
    matched: usize,
    maximum_distance: f64,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dFocusSelectionWorkOwners>,
}

impl Default for Puzzle3dFocusSelectionWork {
    fn default() -> Self {
        Self { close_owners: Default::default(),
            stage: Puzzle3dFocusSelectionStage::Selection,
            selection_cursor: 0,
            object_cursor: 0,
            selected: HashSet::with_capacity(semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS),
            center: [0.0; 3],
            matched: 0,
            maximum_distance: 1.0,
        }
    }
}

impl Puzzle3dFocusSelectionWork {
    pub(super) fn selection(interaction: &protocol::InteractionState) -> &[String] {
        interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).filter(|selection| selection.granularity == PUZZLE3D_GRANULARITY_OBJECT).map(|selection| selection.ids.as_slice()).unwrap_or_default()
    }

    /// 🎥️ Which objects this focus frames. An EMPTY selection frames the whole document rather than
    /// completing with `Emit::default()` — a camera verb with nothing selected has an obvious subject
    /// (everything), and the silent no-op made `f` a dead key on every boot before the user had picked
    /// anything (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 14). Costs no extra capacity: the
    /// two object scans this work already declares in `extent` run either way.
    fn frames(&self, object_id: &str) -> bool {
        self.selected.is_empty() || self.selected.contains(object_id)
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dFocusSelectionWork {
    fn tool_id(&self) -> &'static str {
        "focusSelection"
    }

    fn extent(&self, _command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        let selected = Self::selection(interaction).len();
        let objects = snapshot.typed().objects.len();
        let items = selected.checked_add(objects.checked_mul(2)?)?.checked_add(1)?;
        (selected <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS && items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
    }

    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command: _command, snapshot, config: _config, interaction, hover: _hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        match self.stage {
            Puzzle3dFocusSelectionStage::Selection => {
                if let Some(id) = Self::selection(interaction).get(self.selection_cursor) {
                    if self.selected.len() >= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_DECODED_ITEMS {
                        return Err(Fault::from("puzzle3d-focus-selection-capacity"));
                    }
                    self.selected.insert(id.clone());
                    self.selection_cursor += 1;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-owner", "Reading selected object", "Ausgewähltes Objekt wird gelesen"));
                }
                self.stage = Puzzle3dFocusSelectionStage::SumObjects;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-center", "Measuring selection center", "Auswahlzentrum wird gemessen"))
            }
            Puzzle3dFocusSelectionStage::SumObjects => {
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else {
                    self.object_cursor = 0;
                    if self.matched > 0 {
                        let divisor = self.matched as f64;
                        self.center = [self.center[0] / divisor, self.center[1] / divisor, self.center[2] / divisor];
                    }
                    self.stage = Puzzle3dFocusSelectionStage::DistanceObjects;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-distance", "Measuring selection radius", "Auswahlradius wird gemessen"));
                };
                self.object_cursor += 1;
                if self.frames(&object.id) {
                    self.center[0] += object.origin[0];
                    self.center[1] += object.origin[1];
                    self.center[2] += object.origin[2];
                    self.matched += 1;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-center", "Scanning selected object", "Ausgewähltes Objekt wird geprüft"))
            }
            Puzzle3dFocusSelectionStage::DistanceObjects => {
                let Some(object) = snapshot.typed().objects.get(self.object_cursor) else {
                    self.stage = Puzzle3dFocusSelectionStage::Publish;
                    return Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-publish", "Preparing camera focus", "Kamerafokus wird vorbereitet"));
                };
                self.object_cursor += 1;
                if self.frames(&object.id) {
                    let delta = [object.origin[0] - self.center[0], object.origin[1] - self.center[1], object.origin[2] - self.center[2]];
                    self.maximum_distance = self.maximum_distance.max((delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt());
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-focus-selection-distance", "Scanning selection radius", "Auswahlradius wird geprüft"))
            }
            Puzzle3dFocusSelectionStage::Publish => {
                self.stage = Puzzle3dFocusSelectionStage::Complete;
                if self.matched == 0 {
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::default()));
                }
                let distance = self.maximum_distance * 3.0 + 2.0;
                let mut next = window_ownership::config_from_snapshot(window_config_snapshot);
                next.camera.position = [self.center[0] + distance * 0.6, self.center[1] - distance * 0.6, self.center[2] + distance * 0.5];
                next.camera.target = self.center;
                let view = view_state.ok_or_else(|| Fault::from("puzzle3d-focus-window-context-required"))?;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![window_ownership::addressed_config(view, next)?], ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("focusSelection")), ..Default::default() }))
            }
            Puzzle3dFocusSelectionStage::Complete => Err(Fault::from("puzzle3d-focus-selection-complete-repolled")),
            Puzzle3dFocusSelectionStage::Closing => Err(Fault::from("puzzle3d-focus-selection-closing")),
        }
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dFocusSelectionStage::Closing;
        self.close_owners.stage(Puzzle3dFocusSelectionWorkOwners { selected: std::mem::take(&mut self.selected) });
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.close_owners.close_step(grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_owners.demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.stage == Puzzle3dFocusSelectionStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
