//! 🔘️ there, then click to place a free one. The candidate search is a read-only framework tool run
//! 🖌️ Main-window utility — Brush: point at a vortex and WATCH the engine test every candidate that could dock
//! (`📋️tool-run-contract.md` §2.4, §3.7): the manifest carries its [`ToolRunDefinition`], the framework owns the
//! run, its progress panel and its trace, and the viewport paints every tested candidate with its verdict.
//! Accepting a candidate stays the separate one-shot `acceptSuggestion` placement.

use semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_PLAY_CONTROLLER_ID;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsLink;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsRequest;
use semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsRunJob;
use semio_s_artifact_puzzle_3d_precompute::shared_brush_mesh;
use semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_brush_suggestions_run_counters;
use semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_brush_suggestions_run_reasons;
use semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_brush_suggestions_run_stages;
use semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_brush_suggestions_run_unit;
use semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels;
use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime;
use {semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_action, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_distribution_group, semio_s_artifact_puzzle_3d_editor_session::puzzle3d_fallback_mesh_buffers, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_session::scene_config, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dEditor, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene};
use semio_framework_plugin::Effect;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::RequestId;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobPurpose;
use semio_framework_plugin::ToolRunJobRequest;
use semio_framework_plugin::ToolRunView;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowMeasure;
use semio_framework_tool_run::JobKindId;
use semio_framework_tool_run::ToolRunDefinition;
use semio_framework_tool_run::ToolRunRebasePolicy;
use semio_framework_tool_run::ToolRunReconfigurePolicy;
use semio_framework_tool_run::ToolRunSettingsReads;
use semio_framework_tool_run::ToolRunState;
use semio_framework_tool_run::ToolRunTraceKind;
use semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID;
use semio_framework_tool_run::TOOL_RUN_ARG_GENERATION;
use semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID;
use semio_framework_tool_run::TOOL_RUN_ARG_TOOL_ID;
use semio_framework_tool_run::TOOL_RUN_START_ACTION_ID;
use std::sync::Arc;

//#region 🔖️Constants
pub const UTILITY_ID: &str = "brush";
/// 🧵️ `ToolRunDefinition.runJob` of the brush suggestions run.
pub const RUN_JOB_KIND: &str = "s.puzzle.puzzle3d.brush.suggestions";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition { run: Some(run_definition()), ..UtilityDefinition::new(UTILITY_ID, label, "paintbrush") }
}

/// ⏯️ A read-only `instance3d` run. It authors no provisional edit, so a document change or a change of the
/// settings its job reads (contact tolerance and kind weights) simply restarts the search against what is now there
/// (`restart` for both); there is nothing to revalidate.
pub fn run_definition() -> ToolRunDefinition {
    ToolRunDefinition { receiving: semio_framework_tool_run::ToolRunReceivingLimits::REPORTS,
        mutating: false,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Restart,
        unit: puzzle3d_brush_suggestions_run_unit(),
        stages: puzzle3d_brush_suggestions_run_stages(),
        counters: puzzle3d_brush_suggestions_run_counters(),
        reasons: puzzle3d_brush_suggestions_run_reasons(),
        trace: ToolRunTraceKind::Instance3d,
        run_job: JobKindId::new(RUN_JOB_KIND),
        revalidate_job: None,
        settings: ToolRunSettingsReads { config: vec!["/contactTolerance".into(), "/objectKindWeights".into(), "/vortexKindWeights".into()], window_config: Default::default() },
        windows: Vec::new(),
        member: None,
    }
}

/// 🖌️ Utility Options for the Brush utility: the contact tolerance and the shared object/vortex distribution tree.
/// Tagged with this utility's id as a routing envelope only; `partition_window_measures` unwraps the children.
/// The candidate search's progress, status and every candidate with its verdict are the framework ToolRun panel's.
pub fn options(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush"),
        label: labels.brush.into(),
        default_open: Some(true),
        active_utility_id: Some(UTILITY_ID.into()),
        children: vec![
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-contact-tolerance"),
                label: Some(labels.contact_tolerance.into()),
                value: envelope.runtime.contact_tolerance,
                min: 0.0,
                max: 0.05,
                step: Some(0.001),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: puzzle3d_action("setBrushPlacementContactTolerance", None),
            },
            puzzle3d_distribution_group(envelope, labels, Some(false)),
        ],
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
    }
}
//#endregion 🔖️Definition

//#region 🔖️Run
/// 🧵️ Builds the brush suggestions run job for the framework ledger; `None` for any other tool.
pub fn build_run_job<A: Puzzle3dEditor>(request: ToolRunJobRequest<'_, EditorApp<A>>) -> Result<Option<ToolRunJob>, Fault> {
    if request.tool_id != UTILITY_ID || request.purpose != ToolRunJobPurpose::Run {
        return Ok(None);
    }
    let config = request.config.as_ref();
    let runtime = Puzzle3dRuntime { contact_tolerance: config.contact_tolerance, object_kind_weights: config.object_kind_weights.clone(), vortex_kind_weights: config.vortex_kind_weights.clone(), ..Puzzle3dRuntime::default() };
    let envelope = Puzzle3dScene { scene_snapshot: puzzle3d_scene_snapshot_from_document(request.snapshot.typed()), runtime, active_utility: UTILITY_ID.into() };
    let scene = scene_config(&envelope).ok_or_else(|| Fault::from("puzzle3d-brush-suggestions-scene"))?;
    let job = BrushSuggestionsRunJob::<Puzzle3dInstanceOperationOwner>::new(request.instance_owner, request.port, request.identity, Arc::new(scene), semio_s_artifact_puzzle_3d_editor_session::mesh_lane(&envelope.scene_snapshot), shared_brush_mesh, puzzle3d_fallback_mesh_buffers()).ok_or_else(|| Fault::from("puzzle3d-brush-suggestions-fallback-mesh"))?;
    Ok(Some(Box::new(job)))
}

/// 🚦️ Reconciles the brush suggestions run with the vortex the link points at, once per refresh and after every
/// landed gesture: a target with no live brush run starts one, a live run with no target is aborted, and a live
/// run with a target is woken to re-read it. A request stays outstanding until the run view answers it (a new
/// run for a start, no live run for an abort), so a refresh never repeats it; a gesture releases it. A faulted
/// run is only restarted by a gesture, never by a refresh.
pub fn run_effects(link: &mut BrushSuggestionsLink, run: Option<semio_framework_plugin::ToolRunPollView<'_>>) -> Vec<Effect> {
    let run = run.filter(|run| run.tool_id == UTILITY_ID);
    let identity = run.map(|run| run.identity.id.run);
    let live = run.filter(|run| !run.state.is_terminal());
    if let Some((request, asked)) = link.requested {
        let answered = match request {
            BrushSuggestionsRequest::Start => identity != asked,
            BrushSuggestionsRequest::Abort => live.is_none() || identity != asked,
        };
        if !answered {
            return Vec::new();
        }
        link.requested = None;
    }
    match (link.target().is_some(), live) {
        (true, Some(_)) => {
            link.wake();
            Vec::new()
        }
        (true, None) if run.is_some_and(|run| run.state == ToolRunState::Faulted) && !link.retry => Vec::new(),
        (true, None) => {
            link.requested = Some((BrushSuggestionsRequest::Start, identity));
            link.retry = false;
            vec![run_action(TOOL_RUN_START_ACTION_ID, semio_framework_value::DslValue::object([(TOOL_RUN_ARG_TOOL_ID.to_string(), semio_framework_value::DslValue::String(UTILITY_ID.into()))]))]
        }
        (false, Some(run)) => {
            link.requested = Some((BrushSuggestionsRequest::Abort, identity));
            vec![run_action(TOOL_RUN_ABORT_ACTION_ID, semio_framework_value::DslValue::object([(TOOL_RUN_ARG_RUN_ID.to_string(), semio_framework_value::DslValue::String(run.identity.id.run.to_string())), (TOOL_RUN_ARG_GENERATION.to_string(), semio_framework_value::DslValue::uint(u64::from(run.identity.generation)))]))]
        }
        (false, None) => Vec::new(),
    }
}

fn run_action(action: &str, args: semio_framework_value::DslValue) -> Effect {
    Effect::DispatchAction { req: RequestId(semio_framework_job::allocate_operation_id().0), action: action.into(), args: Some(args), delay_ms: 0 }
}
//#endregion 🔖️Run

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
