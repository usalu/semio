use super::*;
use semio_s_artifact_procedural_generation3d::editor::generation3d::config::SetSnapshot;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::mutations::generation3d_host_snapshot_operations;
use crate::SnapshotRead;
use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{
    default_snapshot, example_snapshot, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use crate::editor_domain::editor_laws::context::{self, app_with_registry};
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::PluginApp;

/// 📚️ Every bundled example the picker offers — the same eight `each_example_loads_distinct_fixture_and_preview_geometry` walks.
const BUNDLED_EXAMPLES: [&str; 8] = [
    PROCEDURAL_EXAMPLE_HEX_COLUMN,
    PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    PROCEDURAL_EXAMPLE_SPHERE_TORUS,
    PROCEDURAL_EXAMPLE_BOX_FILLET,
    PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
    PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
    PROCEDURAL_EXAMPLE_BOX_SHELL,
];

/// 🪪️ The widget-id signature of a bundled example's authored host_snapshot. A synchronous frame on
/// purpose: a `Generation3dSnapshotRead` carries the whole `FlowHostSnapshot`, and parking one in an
/// `async fn` that also holds a live app makes the test future large enough to overflow the test
/// thread's stack the moment the eight-example walk awaits inside it.
fn bundled_widget_ids(example_id: &str) -> std::collections::BTreeSet<String> {
    let read = SnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
    read.host_snapshot.widgets.iter().map(semio_s_artifact_procedural_generation3d::widget_id).map(str::to_string).collect()
}

/// 🪪️ The same signature for the app's LIVE document — synchronous for the same reason.
fn live_widget_ids(app: &context::Generation3dApp) -> std::collections::BTreeSet<String> {
    context::snapshot(app).host_snapshot.widgets.iter().map(semio_s_artifact_procedural_generation3d::widget_id).map(str::to_string).collect()
}

/// 🧺️ Replays the store's OWN fold arithmetic over one lane's authored gesture: every item costs its
/// single forward row plus every row its inverse yields, and both the per-item gate in
/// `ArtifactStore::fold_batch_item` and the gesture-wide gate in `PreflightingCommit` compare that
/// against the MERGED footprint the preflights declared. Returns `(rows, declared)`.
fn folded_rows_against_declaration(items: &[(usize, usize)]) -> (usize, usize) {
    let declared = items.iter().map(|(_, work_items)| work_items).sum();
    let rows = items.iter().map(|(inverse_rows, _)| inverse_rows + 1).sum();
    (rows, declared)
}

/// 🧾️ The Artifact lane of `setActiveExample` declares a fold envelope that its own candidates fit —
/// per item AND for the whole gesture — for every hop of the picker cycle, so all eight bundled
/// fixtures appear as both a base and a target. The cycle, not eight diffs against the default: the
/// default document IS the hex-column host_snapshot, so `default → hex-column` authors nothing at all, and
/// the second hop is exactly the boot-time gesture. This is the law the runtime broke:
/// a declared `work_items: 1` leaves ZERO inverse capacity, so `fold_batch_item` answered
/// `batched item candidate failed its exact fixed fold contract` for the very first action of the app
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).


/// 🧾️ The Config lane of the same command is the single-item case — the one that fail-closed FIRST at
/// runtime, because a one-mutation gesture leaves the per-item gate no slack at all.


/// 🚫️ The hostile control, and the exact shape of the defect: a declaration of ONE work item cannot
/// carry a point-invertible item, so every single-mutation durable gesture fails closed. Kept as a law
/// so the literal can never come back through either lane.


/// 🔒️ Both durable lanes derive their footprint from the mutation's schema-declared inverse rows
/// (`ArtifactStoreOneItemFootprint::for_leaf`, design §20.5) — never a bare
/// `ArtifactStoreOneItemFootprint { work_items: … }` struct literal.
#[test]
fn both_durable_lanes_derive_their_footprint_from_the_mutation() {
    let source = include_str!("../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs");
    assert_eq!(source.matches("Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))").count(), 2, "both the artifact and the config preflight derive their footprint from the mutation");
    assert!(!source.contains("ArtifactStoreOneItemFootprint { work_items"), "a durable lane regained a hand-written work-items literal");
}

/// 🧾️ Every `{example}/{operator}@{input}` of the bundled examples whose declared default the operator does not record,
/// plus every operator whose kind the installed registry does not know — empty for a self-describing example set.
fn unrecorded_example_defaults() -> Vec<String> {
    use semio_framework_artifact_flow_flow::neural::ColdRetire;
    use semio_framework_artifact_flow_flow::{default_neuron_params_from_info, Widget};
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH;
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    BUNDLED_EXAMPLES.iter().copied().chain([PROCEDURAL_EXAMPLE_MESH_WORKBENCH]).flat_map(|example_id| {
        let read = SnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
        let unrecorded: Vec<String> = read.host_snapshot.widgets.iter().filter_map(|widget| match widget { Widget::Neuron { id, neuron_kind, params, .. } => Some((id, neuron_kind, params)), _ => None }).flat_map(|(id, kind, params)| {
            let declared = default_neuron_params_from_info(infos.get(kind));
            let missing = if infos.contains_key(kind) { declared.keys().filter(|key| params.get(key).is_none()).map(|key| format!("{example_id}/{id}@{key}")).collect() } else { vec![format!("{example_id}/{id}: unknown kind {kind}")] };
            declared.retire_cold();
            missing
        }).collect();
        unrecorded
    }).collect()
}

/// ⚖️ LAW (design §20.9): every bundled example is a self-describing document — each operator records every declared
/// input's default literal in its `params`, exactly as an inserted operator does — so an input edit on a loaded example
/// folds from the snapshot alone and never meets `mutation.target-missing`.
#[test]
fn every_bundled_example_records_every_declared_default_of_its_operators() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let unrecorded = unrecorded_example_defaults();
    assert!(unrecorded.is_empty(), "every operator of a bundled example records its declared defaults: {unrecorded:?}");
}

/// 🐛️ [DEBUG] Temporary: prints every bundled example with its operators' declared defaults recorded, for the one-time
/// normalization of the example assets (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). Deleted once the assets hold it.
#[test]
fn debug_print_normalized_examples() {
    use semio_framework_artifact_flow_flow::neural::ColdRetire;
    use semio_framework_artifact_flow_flow::{default_neuron_params_from_info, Widget};
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
    for example_id in BUNDLED_EXAMPLES.iter().copied().chain([PROCEDURAL_EXAMPLE_MESH_WORKBENCH]) {
        let mut read = SnapshotRead::new(example_snapshot(example_id).expect("bundled example snapshot"));
        for widget in read.host_snapshot.widgets.iter_mut() {
            if let Widget::Neuron { neuron_kind, params, .. } = widget {
                let declared = default_neuron_params_from_info(infos.get(neuron_kind));
                let recorded = declared.iter().filter(|(key, _)| params.get(key).is_none()).fold(params.clone(), |recorded, (key, value)| recorded.insert(key.clone(), value.clone()));
                std::mem::replace(params, recorded).retire_cold();
                declared.retire_cold();
            }
        }
        eprintln!("[DEBUG] BEGIN {example_id}\n{}\n[DEBUG] END {example_id}", store::ArtifactDsl::print_dsl(&*read));
    }
}

/// 🎬️ Boots ONE app, publishes `setActiveExample` through the real retained path (the host's bounded
/// publication/ACK protocol), and asserts the document actually became that example's host_snapshot.
/// Before the fold-envelope fix every one of these retired without publishing
/// (`typed-operation publication turn=… operations=["27:Retiring:true:true"] latest_wins_empty=true`).
/// 🎯️ One law PER bundled example, and the body EXPANDED into each — never one walk over all eight
/// and never an awaited helper: a live `Generation3dApp` plus the host's publication machine is a
/// multi-megabyte future, so nesting even two of those states in one test future overflows the test
/// thread's 8 MiB stack. Each example therefore gets its own test thread holding exactly one, and the
/// picker's whole roster stays covered.
macro_rules! set_active_example_publishes {
    ($($name:ident => $example:expr,)+) => {
        $(
            #[semio_framework_async_macros::async_test]
            async fn $name() {
                let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
                let mut app = app_with_registry().await;
                let before = live_widget_ids(&app);
                app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": $example }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("setActiveExample dispatches");
                let receipt = context::settle(&mut app).await;
                assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "{} published a fault lane", $example);
                let after = live_widget_ids(&app);
                assert_eq!(after, bundled_widget_ids($example), "{} did not reach the store: the published document is not the example's fixture", $example);
                assert!(after != before || $example == PROCEDURAL_EXAMPLE_HEX_COLUMN, "{} left the document untouched", $example);
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
            }
        )+
    };
}

set_active_example_publishes! {
    set_active_example_publishes_hex_column => PROCEDURAL_EXAMPLE_HEX_COLUMN,
    set_active_example_publishes_rect_extrude => PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
    set_active_example_publishes_sphere_torus => PROCEDURAL_EXAMPLE_SPHERE_TORUS,
    set_active_example_publishes_box_fillet => PROCEDURAL_EXAMPLE_BOX_FILLET,
    set_active_example_publishes_sphere_box_fuse => PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
    set_active_example_publishes_face_sweep_extrude => PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
    set_active_example_publishes_rectangle_wire => PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
    set_active_example_publishes_box_shell => PROCEDURAL_EXAMPLE_BOX_SHELL,
}

/// 🕹️ The framework-owned local-interaction route publishes on the same machine: `interactionSelect`
/// settles through the host's bounded publication protocol and the selection is readable afterwards.
/// It failed at runtime with the SAME fold-contract message as `setActiveExample`, because the whole
/// typed-operation lane of the instance was fail-closed behind the app's rejected candidate.
#[semio_framework_async_macros::async_test]
async fn interaction_select_publishes_through_the_retained_typed_path() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let node_id = context::snapshot(&app).host_snapshot.widgets.first().map(semio_s_artifact_procedural_generation3d::widget_id).expect("default fixture node").to_string();
    // 🧯️ `interactionSelect` is a framework-reserved TOOL JOB (`FrameworkInteractionSelectJob`), so
    // `handle_action` alone only admits its `Effect::SpawnJob` — the selection lands when the host
    // drives that job, which is what `select_graph` does. A `settle_registered_typed_operation`
    // cannot: it settles a typed operation, and the reserved spawn is not one.
    let settled = context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    eprintln!("interactionSelect published: effects={}", settled.requested_effects.len());
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
