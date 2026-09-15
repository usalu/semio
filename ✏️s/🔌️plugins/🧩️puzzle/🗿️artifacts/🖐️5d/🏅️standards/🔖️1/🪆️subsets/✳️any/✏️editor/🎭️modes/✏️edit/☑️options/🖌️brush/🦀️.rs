//! 🖌️ Edit-mode window option — the Brush utility's Utility Options group: suggestion offset, contact
//! tolerance and the part/grip distribution trees; the brush suggestions run's candidates show as its trace in the
//! windows. Tagged `Some("brush")`.
//!
//! 🎚️ SHARED at MODE level, not per window (TEMPLATE.md §12.2): BOTH the 2D board window and the 3D
//! world window bind the `brush` utility and expose the identical group, so this measure is declared
//! once here and each window's `window_measures()` collects from it.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    puzzle5d_action, puzzle5d_kind_ids, puzzle5d_kind_weight_sum, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID, PUZZLE5D_SUGGESTION_OFFSET_MAX, PUZZLE5D_SUGGESTION_OFFSET_MIN,
    PUZZLE5D_SUGGESTION_OFFSET_STEP,
};
use semio_framework_plugin::WindowMeasure;
use dsl::json;
use std::collections::HashMap;

//#region 🔖️Distribution
fn kind_weight_measures(prefix: &str, action: &str, ids: &[String], weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    ids.iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or_else(|| if ids.is_empty() { 0.0 } else { 1.0 / ids.len() as f64 });
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                value: weight,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                on_change: puzzle5d_action(action, Some(json!({ "kindId": kind_id.as_str() }))),
            }
        })
        .collect()
}

fn distribution_children(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    let part_ids = puzzle5d_kind_ids(&envelope.document, "parts");
    let grip_ids = puzzle5d_kind_ids(&envelope.document, "grips");
    vec![
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-parts"),
            label: format!("{} ({:.0}%)", labels.part_weights.as_str(), puzzle5d_kind_weight_sum(&envelope.runtime.object_kind_weights, &part_ids) * 100.0),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: kind_weight_measures("part-kind", "setObjectKindWeight", &part_ids, &envelope.runtime.object_kind_weights),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-grips"),
            label: format!("{} ({:.0}%)", labels.grip_weights.as_str(), puzzle5d_kind_weight_sum(&envelope.runtime.vortex_kind_weights, &grip_ids) * 100.0),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: kind_weight_measures("grip-kind", "setVortexKindWeight", &grip_ids, &envelope.runtime.vortex_kind_weights),
        },
    ]
}
//#endregion 🔖️Distribution

//#region 🔖️Measure
/// 🖌️ The Brush utility's Utility Options group, collected by both windows' `window_measures()`.
pub fn measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
    let children = vec![
        WindowMeasure::Slider {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-offset"),
            label: Some(labels.offset.into()),
            value: envelope.runtime.suggestion_offset,
            min: PUZZLE5D_SUGGESTION_OFFSET_MIN,
            max: PUZZLE5D_SUGGESTION_OFFSET_MAX,
            step: Some(PUZZLE5D_SUGGESTION_OFFSET_STEP),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            on_change: puzzle5d_action("setSuggestionOffset", None),
        },
        WindowMeasure::Slider {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-contact-tolerance"),
            label: Some(labels.contact_tolerance.into()),
            value: envelope.runtime.contact_tolerance,
            min: 0.0,
            max: 0.05,
            step: Some(0.001),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            on_change: puzzle5d_action("setBrushPlacementContactTolerance", None),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-distribution"),
            label: labels.suggestion.into(),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: distribution_children(envelope, labels),
        },
    ];
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-brush"),
        label: labels.brush.into(),
        default_open: Some(true),
        active_utility_id: Some("brush".into()),
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
//#endregion 🔖️Measure
