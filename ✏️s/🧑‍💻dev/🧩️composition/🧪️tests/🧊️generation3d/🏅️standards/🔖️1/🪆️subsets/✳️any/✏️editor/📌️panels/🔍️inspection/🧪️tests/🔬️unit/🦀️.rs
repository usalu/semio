use super::*;
use crate::editor_domain::editor_laws::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn inspector_shows_no_selection_by_default() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_INSPECTION).await.contains("Schema:"));
}

/// ⏎️ LAW: the slider's editable control is a CHILD OF A TREE ROW inside the section, never a bare
/// `field` beside the rows. A panel section keeps only `treeItem` children (`collectTreeItems`,
/// `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`),
/// and a row's non-`treeItem` children are what the renderer mounts as its inline controls
/// (`collectTreeItemControls`, same file).
///
/// 🐛️ Authored as a sibling `field`, the input never reached the DOM: the panel painted `Id: height`
/// and `Range: 0..10` and nothing to type into, so a selected slider could not be edited from the
/// inspector at all (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `🐍️react-gap-probe.mjs` step
/// `inspection-edit`, run `🗑️generated/react-verify/gaps/`).
#[test]
fn inspector_slider_control_rides_on_a_tree_row() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture = semio_framework_os_flow::FlowHost::parse_host_snapshot_json(
        r#"{"schema":"flow.host_snapshot","camera":{"x":0.0,"y":0.0,"zoom":1.0},"widgets":[{"kind":"inputSlider","id":"height","label":"Height","value":3.0,"min":0.0,"max":10.0,"step":0.5}],"synapses":[],"layout":{"height":{"x":0.0,"y":0.0}}}"#,
    )
    .expect("law fixture parses");
    let labels = semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let tree = render(&fixture, &["height".to_string()], labels).expect("inspector builds");
    fixture.retire_cold();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).expect("inspector projects");
    let projection: serde_json::Value = serde_json::from_str(&projection).expect("inspector projection json");
    let rows = projection["children"][0]["children"].as_array().cloned().unwrap_or_default();
    let value_row = rows.iter().find(|row| row["key"] == "procedural-play-inspector.value").unwrap_or_else(|| panic!("the inspector renders a value row: {projection}"));
    assert_eq!(value_row["component"]["type"].as_str(), Some("treeItem"), "the value row must be a tree row the section keeps: {value_row}");
    let control = value_row["children"].as_array().and_then(|children| children.first()).unwrap_or_else(|| panic!("the value row carries its control as a child: {value_row}"));
    assert_eq!(control["key"].as_str(), Some("procedural-play-inspector.value.input"), "{control}");
    assert_eq!(control["component"]["type"].as_str(), Some("input"), "{control}");
    let bindings = control["bindings"].as_array().cloned().unwrap_or_default();
    let change = bindings.iter().find(|binding| binding["trigger"] == "change").unwrap_or_else(|| panic!("the control commits on change: {control}"));
    assert!(change["action"].to_string().contains("patchFlowWidgets"), "{change}");
}

#[test]
fn inspector_exposes_typed_operator_fields_and_connected_sources() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    crate::flow_operators::installed();
    let mut host = semio_framework_os_flow::FlowHost::default();
    host.set_neuron_kind_info_map(semio_framework_os_flow::flow_neuron_kind_info_map());
    let source = host.add_widget(r#"{"kind":"inputSlider","id":"width"}"#, 0.0, 0.0).unwrap();
    let shape = host.add_widget(r#"{"kind":"neuron","id":"shape","neuronKind":"brep.mesh.box"}"#, 200.0, 0.0).unwrap();
    let labels = semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::generation3d_labels(&semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    let tree = render(&host.host_snapshot, &[shape.clone()], labels).unwrap();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
    let projection: serde_json::Value = serde_json::from_str(&projection).unwrap();
    let rows = projection["children"][0]["children"].as_array().unwrap();
    let row = rows.iter().find(|row| row["key"] == "procedural-play-inspector.input.width").unwrap();
    assert_eq!(row["children"][0]["component"]["type"], "input");
    assert!(row["children"][0]["bindings"].to_string().contains("setWidgetInput"));
    assert!(row["children"][0]["bindings"].to_string().contains("commit"));
    host.connect_ports(&source, "number", &shape, "width").unwrap();
    let tree = render(&host.host_snapshot, &[shape], labels).unwrap();
    let connected = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
    assert!(connected.contains("Connected"));
    assert!(!connected.contains("procedural-play-inspector.input.width.input"));
    host.retire_cold();
}

