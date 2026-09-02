//! 🖌️ Main-window utility — Brush: hover a vortex, cycle the collision-free candidates the engine
//! precomputed for it, click to dock one. Its Utility Options are the overlap budget, the shared
//! object/vortex distribution tree, and (only while there are candidates for the hovered vortex) the
//! placement picker.

use crate::editor::puzzle3d::precompute::Puzzle3dPrecomputeSession;
use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{puzzle3d_action, puzzle3d_brush_target_vortex, puzzle3d_distribution_group, Puzzle3dScene, PUZZLE3D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{LocalizedLabel, MeasureSelectItem, UtilityDefinition, WindowMeasure};

pub const UTILITY_ID: &str = "brush";

/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition(label: LocalizedLabel) -> UtilityDefinition {
    UtilityDefinition::new(UTILITY_ID, label, "paintbrush")
}

/// 🖌️ Utility Options for the Brush utility. Tagged with this utility's id as a routing envelope
/// only; `partition_window_measures` unwraps the children so the utility bar shows the option tree
/// directly (no nested "Brush"/"Pinsel" header — the utility toggle already owns that row).
pub fn options(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> WindowMeasure {
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
            reveal: None,
            on_change: puzzle3d_action("setBrushPlacementOverlapBudget", None),
        },
        puzzle3d_distribution_group(envelope, labels, Some(false)),
    ];
    if envelope.active_utility == UTILITY_ID {
        if let Some(target) = puzzle3d_brush_target_vortex(envelope) {
            let candidates = precompute.brush_candidates(&target).free;
            if !candidates.is_empty() {
                let items: Vec<MeasureSelectItem> = candidates
                    .iter()
                    .enumerate()
                    .map(|(index, candidate)| {
                        let label = candidate.object_kind_id.as_str();
                        let id = format!("puzzle3d.brush.candidate.{index}");
                        MeasureSelectItem { id: id.clone(), value: id, label: label.into() }
                    })
                    .collect();
                let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
                children.push(WindowMeasure::Select {
                    id: "puzzle3d-brush-placement".into(),
                    label: Some(labels.placement.into()),
                    value: format!("puzzle3d.brush.candidate.{selected_index}"),
                    items,
                    on_change: puzzle3d_action("engagementControlSelect", None),
                });
            }
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
