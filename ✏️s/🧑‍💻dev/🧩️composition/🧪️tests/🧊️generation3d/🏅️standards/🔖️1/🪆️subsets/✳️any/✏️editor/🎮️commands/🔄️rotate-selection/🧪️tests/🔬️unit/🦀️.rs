use super::*;
use crate::editor_domain::editor_laws::context::{self, app, app_with_registry, dispatch};
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;
use semio_s_artifact_procedural_generation3d::widget_id;
use semio_framework_artifact_flow_flow::Widget;

const ROTATE_ID: &str = "extrude__gumball_rotate";

#[semio_framework_async_macros::async_test]
async fn rotate_selection_composes_world_axes_and_preserves_the_result_selection() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 1.0, ay: 0.0, az: 0.0, angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None, window_id: None })).await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 1.0, az: 0.0, angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None, window_id: None })).await;
    let snapshot = context::snapshot(&app);
    let params = with_host(&snapshot.host_snapshot, |host| gumball_widget_json(host, ROTATE_ID).unwrap());
    let axis = params.get("params").unwrap().get("axis").unwrap();
    for (key, expected) in [("x", 1.0), ("y", 1.0), ("z", -1.0)] {
        assert!((axis.get(key).unwrap().as_f64().unwrap() - expected / 3.0_f64.sqrt()).abs() < 1e-12);
    }
    let angle = crate::gumball_param_number(&snapshot.host_snapshot, ROTATE_ID, "angle", 0.0);
    assert!((angle - std::f64::consts::TAU / 3.0).abs() < 1e-12);
    drop(snapshot);
    dispatch(&mut app, Generation3dCommand::TranslateSelection(semio_s_artifact_procedural_generation3d::editor::generation3d::commands::translate_selection::TranslateSelection { node_ids: Vec::new(), dx: 1.0, dy: 0.0, dz: 0.0, phase: None, reason: None, window_id: None })).await;
    let snapshot = context::snapshot(&app);
    assert!(snapshot.host_snapshot.synapses.iter().any(|wire| wire.from == ROTATE_ID && wire.to == "extrude__gumball_rotate__gumball_translate"));
    drop(snapshot);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}



/// 🔄️ A rotate names its own transform neuron once and ACCUMULATES into it — a second grab must not
/// splice a second node, or a gumball drag would grow the graph one neuron per frame.
#[semio_framework_async_macros::async_test]
async fn rotate_selection_accumulates_one_transform_neuron_in_the_flow_graph() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None, window_id: None })).await;
    let once = context::snapshot(&app);
    let neuron = once.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == ROTATE_ID).expect("rotate neuron spliced");
    assert!(matches!(neuron, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.rotate"));
    assert!((crate::gumball_param_number(&once.host_snapshot, ROTATE_ID, "angle", 0.0) - std::f64::consts::FRAC_PI_2).abs() < 1e-12);

    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: vec![ROTATE_ID.into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None, window_id: None })).await;
    let twice = context::snapshot(&app);
    assert_eq!(twice.host_snapshot.widgets.iter().filter(|widget| widget_id(widget) == ROTATE_ID).count(), 1);
    assert!((crate::gumball_param_number(&twice.host_snapshot, ROTATE_ID, "angle", 0.0) - std::f64::consts::PI).abs() < 1e-12);
}

/// 🎯️ An ids-less rotate is the real context-menu shape: the payload carries nothing and the command
/// reads the FRAMEWORK-owned `graph` selection instead (`apply_selected`, fed from
/// `protocol::InteractionState`). Before this test that fallback was exercised only as a menu-row
/// presence assertion, never as dispatch-and-assert-the-mutation.
#[semio_framework_async_macros::async_test]
async fn an_ids_less_rotate_transforms_the_framework_owned_graph_selection() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_4, phase: None, reason: None, window_id: None })).await;
    let projection = context::snapshot(&app);
    assert!(projection.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == ROTATE_ID && neuron_kind == "brep.xform.rotate")));
    assert!((crate::gumball_param_number(&projection.host_snapshot, ROTATE_ID, "angle", 0.0) - std::f64::consts::FRAC_PI_4).abs() < 1e-12);
    drop(projection);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🧯️ A rotate that names nothing at all changes nothing — no neuron, no mutation, no coalesce key.
#[semio_framework_async_macros::async_test]
async fn a_rotate_with_no_target_leaves_the_graph_untouched() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).host_snapshot.widgets.len();
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0, phase: None, reason: None, window_id: None })).await;
    let after = context::snapshot(&app);
    assert_eq!(after.host_snapshot.widgets.len(), before);
    assert!(!after.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == ROTATE_ID));
}
