use super::*;
use crate::editor_domain::editor_laws::context::{self, app_with_registry};
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_SHELL;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity;
use semio_framework_plugin::{PluginApp, ViewModel, ViewWindowInstance};

/// 🧮 The declared numbers this lane pins, printed once so a runtime report can quote them without
/// re-deriving anything: the route capacity, the rows one durable item folds, and the extent a
/// one-item command answers preflight with. All three come from ONE declaration.


/// 🧾️ Every retained route's `extent` — the exact value `ArtifactRetainedCommandPhase::Preflight`
/// measures — answered against the boot document, and asserted admissible by the SAME capacity the
/// job's `maximum_work_items` carries. `Generation3dPreviewCommandWork` and the session work are the
/// two reducers `build_tool_job` picks; `flowEvalTick`'s window-addressed work is driven through the
/// real job below, because its extent reads a context only the host can build.
/// 🎬️ `setActiveExample` through the REAL job ladder (wire pages → decode → preflight → work →
/// publish) — the phase that faulted in the browser with `retained command exceeds semantic work
/// capacity` is the one this asserts passes.
#[semio_framework_async_macros::async_test]
async fn set_active_example_passes_the_retained_preflight() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("setActiveExample dispatches");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample faulted in the retained job ladder");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🕹️ The framework-reserved local observation on the same instance. Its footprint is the framework's
/// `admit_bounded_config_mutation` declaration, and its reserved job's work-item budget now comes
/// from the SAME capacity — the two disagreed before this lane.
#[semio_framework_async_macros::async_test]
async fn interaction_select_passes_the_reserved_preflight() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let node_id = context::snapshot(&app).host_snapshot.widgets.first().map(semio_s_artifact_procedural_generation3d::widget_id).expect("default fixture node").to_string();
    // 🧯️ The reserved spawn-job the admission requests has to be DRIVEN before the selection exists —
    // see `context::select_graph`. Settling a typed operation instead left this law reading a `None`
    // selection and asserting nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let settled = context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    eprintln!("interactionSelect reserved work items={} effects={}", ArtifactRetainedWorkCapacity::for_invertible_items(1).work_items(), settled.requested_effects.len());
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🪟️ The runtime reproduction. `pending_effects` arms `flowEvalTick` with a bare
/// `Effect::DispatchAction` that carries NO window address, so the shell redispatches it under
/// whichever window is current — in the served app the flow window `procedural-main`, never the
/// preview. `Generation3dFlowEvalWindowWork::extent` then answers `None`, and preflight reported that
/// refusal as `retained command exceeds semantic work capacity`: a capacity message for an addressing
/// miss. This law pins the two apart — an unaddressed tick is REFUSED, with the refusal's own fault,
/// and the addressed tick is admitted (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn an_unaddressed_flow_eval_tick_is_refused_not_over_capacity() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let main = ViewModel {
        window_instances: vec![ViewWindowInstance { id: "procedural-main".into(), window_kind_id: flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.into() }],
        ..Default::default()
    }
    .for_window_instance("procedural-main")
    .expect("flow window instance");
    let refused = context::dispatch_with_view(&mut app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: "procedural-preview".into(), window_kind_id: semio_s_artifact_procedural_generation3d::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() }), main).await;
    let detail = match refused {
        Ok(receipt) => format!("{:?}", receipt.lanes),
        Err(fault) => format!("{fault:?}"),
    };
    assert!(!detail.contains("exceeds semantic work capacity"), "an addressing refusal must not be reported as a work-capacity fault: {detail}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
