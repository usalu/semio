//! 🖌️ Main-window utility — Brush: hover a vortex and WATCH the engine test the candidates that could
//! dock there, cycle the collision-free ones, click to place one. Its Utility Options are the overlap
//! budget, the shared object/vortex distribution tree, and the placement picker — which appears with
//! the first free candidate, not with the last.

use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{puzzle3d_action, puzzle3d_brush_target_vortex, puzzle3d_distribution_group, Puzzle3dInteractionSnapshot, Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
use crate::standards::v1::subsets::any::schema::BrushSearchProgress;
use semio_framework_plugin::{LocalizedLabel, MeasureSelectItem, UtilityDefinition, WindowMeasure};

pub const UTILITY_ID: &str = "brush";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, label, "paintbrush")
}

/// 🖌️ Utility Options for the Brush utility. Tagged with this utility's id as a routing envelope
/// only; `partition_window_measures` unwraps the children so the utility bar shows the option tree
/// directly (no nested "Brush"/"Pinsel" header — the utility toggle already owns that row).
pub fn options(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels, interaction: &Puzzle3dInteractionSnapshot) -> WindowMeasure {
    let mut children = vec![
        WindowMeasure::Slider {
            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-overlap-budget"),
            label: Some(labels.overlap_budget.into()),
            value: envelope.runtime.overlap_budget,
            min: 0.0,
            max: 1.0,
            step: Some(0.01),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            on_change: puzzle3d_action("setBrushPlacementOverlapBudget", None),
        },
        puzzle3d_distribution_group(envelope, labels, Some(false)),
    ];
    if envelope.active_utility == UTILITY_ID {
        if let Some(target) = puzzle3d_brush_target_vortex(envelope, interaction) {
            children.extend(brush_search_measures(&target, envelope, precompute, labels));
        }
    }
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush"),
        label: labels.brush.into(),
        default_open: Some(true),
        active_utility_id: Some(UTILITY_ID.into()),
        children,
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

/// 🔎️ The candidate search, said out loud: the moment the FIRST collision-free candidate is known, not
/// when the whole list finishes, the placement picker appears, and its label repeats the counted phrase
/// so the user can tell a short list from an unfinished one. Leaving the vortex (or closing the popup)
/// is the cancel. The search's own progress row returns through the framework ToolRun panel
/// (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS §3.6, wave 2).
pub fn brush_search_measures(target: &str, envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    let progress = precompute.brush_search_progress(target);
    let candidates = precompute.brush_candidates(target).free;
    if candidates.is_empty() {
        return Vec::new();
    }
    let items: Vec<MeasureSelectItem> = candidates
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            let id = format!("puzzle3d.brush.candidate.{index}");
            MeasureSelectItem { id: id.clone(), value: id, label: candidate.object_kind_id.as_str().into() }
        })
        .collect();
    let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
    vec![WindowMeasure::Select {
        id: "puzzle3d-brush-placement".into(),
        label: Some(format!("{} — {}", labels.placement.as_str(), brush_search_summary(&progress, labels))),
        value: format!("puzzle3d.brush.candidate.{selected_index}"),
        items,
        on_change: puzzle3d_action("engagementControlSelect", None),
    }]
}

/// 🔤️ `3 free · 7 / 12 tested` — the one counted phrase the picker label and the popup both read, so
/// a partial list never reads as a complete one.
pub fn brush_search_summary(progress: &BrushSearchProgress, labels: &Puzzle3dLabels) -> String {
    format!("{} {} · {} / {} {}", progress.free, labels.brush_search_free.as_str(), progress.tested, progress.total_candidates, labels.brush_search_tested.as_str())
}

/// 🧭️ Stage caption of the candidate search: the counted phrase while candidates are still owed, with
/// the blocked tally once anything has been refused, and the completion caption once nothing is.
pub fn brush_search_stage(progress: &BrushSearchProgress, labels: &Puzzle3dLabels) -> String {
    let counted = brush_search_summary(progress, labels);
    let counted = if progress.blocked > 0 { format!("{counted} · {} {}", progress.blocked, labels.brush_search_blocked.as_str()) } else { counted };
    if progress.done {
        format!("{} · {counted}", labels.brush_search_done.as_str())
    } else {
        counted
    }
}
