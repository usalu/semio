use super::*;
use crate::editor_domain::editor_laws::context::{app, dispatch, snapshot};
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;

fn patch(widget_id: &str, value: f64) -> Generation3dCommand {
    Generation3dCommand::PatchFlowWidgets(PatchFlowWidgets { widget_ids: vec![widget_id.into()], field: "value".into(), value: Some(value) })
}

/// ⚖️ LAW: the Inspection panel's number field — the OTHER door a continuous control reaches this document through —
/// maps a value to the ABSOLUTE `change-slider-value` leaf on the committed document, so the last value wins; the framework scrub machine turns a held press of it into ONE transaction.
///
/// 🐛️ Before the scrub machine, one second of a held spinner cost 29 undo steps and 24 preview evaluations, measured
/// on 6018 (`📓️slider-preview-update-2026-09-15.md` §4).
#[semio_framework_async_macros::async_test]
async fn a_patch_sets_the_absolute_value_the_field_names() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    for value in [6.5_f64, 7.5, 9.0] {
        dispatch(&mut app, patch("height", value)).await;
    }
    let landed = {
        let read = snapshot(&app);
        read.host_snapshot.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == "height" => Some(*value),
            _ => None,
        })
    };
    assert_eq!(landed, Some(9.0), "the document holds the last value the field set");
}
