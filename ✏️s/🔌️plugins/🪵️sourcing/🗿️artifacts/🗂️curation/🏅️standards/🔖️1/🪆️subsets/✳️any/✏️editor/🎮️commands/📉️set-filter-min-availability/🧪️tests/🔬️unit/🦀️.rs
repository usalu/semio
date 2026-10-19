
use crate::editor::sourcing::commands::set_filter_min_availability::SetFilterMinAvailability;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::modes::edit::windows::pool;
use crate::editor_unit_tests::context::{dispatch, new_app};

#[semio_framework_async_macros::async_test]
async fn set_filter_min_availability_clamps_to_zero() {
    let mut app = new_app().await;
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(-1000.0), value: None })).await;
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(0.0), value: None })).await;
    let scene = semio_framework_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, pool::SOURCING_CURATION_BODY_POOL, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx(), |root| {
        let surface = root.children.iter().find(|child| matches!(child.component, semio_framework_plugin::Component::Surface(_))).expect("pool table surface");
        crate::editor_unit_tests::context::table_scene_of(surface).expect("complete retained table scene")
    }).await.expect("render");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.rows_json).unwrap().as_array().unwrap().len(), crate::stock_of(&app.snapshot().expect("snapshot")).len());
}
