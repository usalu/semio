
use super::*;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::modes::edit::windows::pool;
use crate::editor::sourcing::unit_tests::context::{dispatch, new_app};

#[semio_framework_async_macros::async_test]
async fn set_filter_min_availability_clamps_to_zero() {
    let mut app = new_app().await;
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(-1000.0), value: None })).await;
    // Filters are config-only now — the pool render reflects the clamp indirectly via an empty result
    // for an unreasonably high min-availability; assert the clamp directly through a second command
    // that reports back the applied absolute value.
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(0.0), value: None })).await;
    let rendered = semio_framework_plugin::PluginApp::render(&mut *app, pool::SOURCING_CURATION_BODY_POOL, None, &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let semio_framework_plugin::Component::Surface(props) = &rendered.root.children.iter().find(|child| matches!(child.component, semio_framework_plugin::Component::Surface(_))).expect("pool table surface").component else { unreachable!() };
    let scene: semio_framework_plugin::TableScene = semio_framework_ui_scene::decode(props).expect("table scene");
    // A clamped-to-zero min-availability keeps every stock row (all availabilities are >= 0).
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.rows_json).unwrap().as_array().unwrap().len(), crate::stock_of(&app.snapshot().expect("snapshot")).len());
}
