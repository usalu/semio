//! 🪟️ Window command work: the one-action-per-window route through the shared action prologue, spending one bounded turn on the scene, one per owed mesh fallback and one on the dispatch.

use super::*;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;

/// 🧾️ The shared action prologue's three halves as this work's own stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Puzzle3dWindowCommandStage {
    Scene,
    Sync,
    Dispatch,
    Complete,
    Closing,
}

/// 🪟️ The one-action-per-window route: every tool id whose whole semantic work IS
/// [`Puzzle3dActionPrologue`] — no document scan of its own, no extra cursor. It used to run that
/// prologue whole in ONE turn, which on the 180-object Nakagin document measured 17.6 ms, twice the
/// framework's interactive step ceiling (ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3); it now spends one
/// bounded turn on the scene, one per owed mesh fallback, and one on the dispatch.
/// 🪪️ The admission identity every instance-bound puzzle3d work takes at construction.
#[derive(Clone)]
pub struct Puzzle3dInstanceBinding {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub owner: ArtifactInstanceOperationOwnerHandle,
}

/// ♻️ The owners one `Puzzle3dWindowCommandWork` still holds when its job closes, retired as one controlled bundle.
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Puzzle3dWindowWorkOwners {
    pub(super) prologue: Puzzle3dActionPrologue,
    pub(super) session: Option<(u32, Option<String>)>,
    pub(super) instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
}

pub struct Puzzle3dWindowCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) stage: Puzzle3dWindowCommandStage,
    turns: usize,
    pub(super) prologue: Puzzle3dActionPrologue,
    pub(super) session: Option<(u32, Option<String>)>,
    pub(super) instance_owner: Option<ArtifactInstanceOperationOwnerHandle>,
    pub(super) close_owners: semio_s_plugin_puzzle_jobs::WorkClosing<Puzzle3dWindowWorkOwners>,
}

impl Puzzle3dWindowCommandWork {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, stage: Puzzle3dWindowCommandStage::Scene, turns: 0, prologue: Puzzle3dActionPrologue::default(), session: None, instance_owner: None, close_owners: Default::default() }
    }



    /// 🪪️ Binds the admission's instance identity and retained operation owner.
    pub fn bound(mut self, instance: Puzzle3dInstanceBinding) -> Self {
        self.session = Some((instance.app_instance_id, Some(instance.parent_document_id)));
        self.instance_owner = Some(instance.owner);
        self
    }

}

impl<A: Puzzle3dEditor> semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<A>> for Puzzle3dWindowCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }
    fn extent(&self, _command: &Puzzle3dCommand, _snapshot: &Puzzle3dPlaySnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<A>>>) -> Option<usize> {
        Some(1)
    }
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_s_plugin_puzzle_retained::step_demands::<EditorApp<A>>(size_of::<Self>())
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<A>>, _cx: &mut StepContext<'_>) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {
        let original = (|| -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<A>>, semio_framework_job::OriginalTurnFailure> {

        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, interaction, hover: hover, context, .. } = *input;
        let view_state = context.and_then(|context| context.view_state.as_ref());
        let window_config_snapshot = context.and_then(|context| context.window_config.as_ref());
        let window_transient_snapshot = context.and_then(|context| context.window_transient.as_ref());
        if self.turns >= Puzzle3dActionPrologue::WORK_ITEMS {
            return Err(Fault::from("puzzle3d-window-work-capacity"));
        }
        self.turns += 1;
        let resolved_window_id = {
            let view = view_state.ok_or_else(|| Fault::from("puzzle3d-window-context-required"))?;
            let roster: Vec<String> = view.window_instances.iter().map(|instance| instance.id.clone()).collect();
            puzzle3d_addressed_window_id(Some(view), None, command.window_id(), &roster).to_string()
        };
        let addressed_view = view_state.filter(|view| view.window_id.as_deref() != Some(resolved_window_id.as_str())).map(|view| {
            view.for_window_instance(&resolved_window_id).unwrap_or_else(|| {
                let mut next = view.clone();
                next.window_id = Some(resolved_window_id.clone());
                next
            })
        });
        let view = addressed_view.as_ref().or(view_state).ok_or_else(|| Fault::from("puzzle3d-window-context-required"))?;
        let window_id = resolved_window_id.as_str();
        let window_config = window_ownership::config_from_snapshot(window_config_snapshot);
        let window_transient = window_ownership::transient_from_snapshot(window_transient_snapshot);
        let runtime = window_ownership::runtime(config, &window_config, &window_transient, Some(view));
        match self.stage {
            Puzzle3dWindowCommandStage::Scene => {
                if let Some(shell) = puzzle3d_shell_only_emit(command.action_id()) {
                    self.stage = Puzzle3dWindowCommandStage::Complete;
                    return Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit: shell.0, ephemeral: shell.1 });
                }
                if command.action_id() == "exportSnapshot" {
                    self.stage = Puzzle3dWindowCommandStage::Complete;
                    return match export_snapshot::puzzle3d_export_publication(&puzzle3d_scene_snapshot_from_document(snapshot.typed()), &runtime.active_example_id)? {
                        export_snapshot::Puzzle3dExportPublication::Inline(effect) => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::effect(effect))),
                        export_snapshot::Puzzle3dExportPublication::Segmented(download) => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteDownload { download: download, ephemeral: Default::default() }),
                        export_snapshot::Puzzle3dExportPublication::Refused(message) => Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::Complete(Emit::effect(Effect::Notify { message }))),
                    };
                }
                self.prologue.scene_step(snapshot, &runtime, Some(view), Some(window_id));
                self.stage = Puzzle3dWindowCommandStage::Sync;
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-scene", "Reading the artifact", "Artefakt wird gelesen"))
            }
            Puzzle3dWindowCommandStage::Sync => {
                let owed = with_puzzle3d_session_for(self.session.clone(), &runtime, |app| self.prologue.sync_step(app, command.action_id(), &runtime, Some(view), Some(window_id)));
                if !owed {
                    self.stage = Puzzle3dWindowCommandStage::Dispatch;
                }
                Ok(semio_s_plugin_puzzle_retained::puzzle_progress_step!("puzzle3d-action-sync", "Preparing the placement session", "Platzierungssitzung wird vorbereitet"))
            }
            Puzzle3dWindowCommandStage::Dispatch => {
                let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
                let instance_owner = self.instance_owner.as_ref();
                let (emit, ephemeral) = with_puzzle3d_session_for(self.session.clone(), &runtime, |app| self.prologue.dispatch_step(app, instance_owner, if self.tool_id=="engagementAbort"{context.and_then(|context|context.tool_run())}else{None}, "", command, Some(window_id), &runtime, Some(view), &snapshot_interaction));
                self.stage = Puzzle3dWindowCommandStage::Complete;
                Ok(semio_framework_plugin::retained_command::ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral })
            }
            Puzzle3dWindowCommandStage::Complete => Err(Fault::from("puzzle3d-window-work-repeated")),
            Puzzle3dWindowCommandStage::Closing => Err(Fault::from("puzzle3d-window-work-closing")),
        }
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) {
        self.stage = Puzzle3dWindowCommandStage::Closing;
        self.close_owners.stage(Puzzle3dWindowWorkOwners { prologue: std::mem::take(&mut self.prologue), session: self.session.take(), instance_owner: self.instance_owner.take() });
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> { semio_framework_job::close_original_nested_step(cx, |cx, _grant| {
        self.close_owners.close_step(cx)
    }) }

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
        self.stage == Puzzle3dWindowCommandStage::Closing && self.close_owners.is_empty()
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        Some(size_of::<Self>())
    }
}
