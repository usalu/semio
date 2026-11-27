
use crate::editor::sourcing::commands::set_filter_min_availability::SetFilterMinAvailability;
use crate::editor::sourcing::SourcingCurationCommand;
use crate::editor::sourcing::modes::edit::windows::pool;
use crate::editor_unit_tests::context::{dispatch, new_app};

#[semio_framework_async_macros::async_test]
async fn set_filter_min_availability_clamps_to_zero() {
    let mut app = new_app().await;
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(-1000.0), value: None })).await;
    dispatch(&mut app, SourcingCurationCommand::SetFilterMinAvailability(SetFilterMinAvailability { delta: Some(0.0), value: None })).await;
    let mut scene = None;
    crate::editor_unit_tests::context::observe(&mut app, pool::SOURCING_CURATION_BODY_POOL, &mut |root| { scene = Some({
        let surface = root.children.iter().find(|child| matches!(child.component, semio_plugin::Component::Surface(_))).expect("pool table surface");
        crate::editor_unit_tests::context::table_scene_of(surface).expect("complete retained table scene")
    }); }).await;
    let scene = scene.expect("render");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&scene.rows_json).unwrap().as_array().unwrap().len(), crate::stock_of(&app.snapshot().expect("snapshot")).len());
}
