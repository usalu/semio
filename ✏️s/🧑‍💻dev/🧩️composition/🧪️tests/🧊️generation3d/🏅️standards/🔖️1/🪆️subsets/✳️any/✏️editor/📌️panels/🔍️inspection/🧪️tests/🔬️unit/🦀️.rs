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
    let tree = render(&fixture, &["height".to_string()], labels, &semio_framework_plugin::TreeWindows::unhosted(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native, &[], "{}", "{}", true).expect("inspector builds");
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
    let commit = bindings.iter().find(|binding| binding["trigger"] == "commit").unwrap_or_else(|| panic!("the control publishes a committed edit: {control}"));
    assert_eq!(control["component"]["commit"], "blur");
    assert_eq!(control["accessibility"]["label"], "Value");
    assert!(commit["action"].to_string().contains("patchFlowWidgets"), "{commit}");
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
    let tree = render(&host.host_snapshot, &[shape.clone()], labels, &semio_framework_plugin::TreeWindows::unhosted(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native, &[], "{}", "{}", true).unwrap();
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
    let projection: serde_json::Value = serde_json::from_str(&projection).unwrap();
    let rows = projection["children"][0]["children"].as_array().unwrap();
    let row = rows.iter().find(|row| row["key"] == "procedural-play-inspector.input.width").unwrap();
    assert_eq!(row["children"][0]["component"]["type"], "input");
    assert!(row["children"][0]["bindings"].to_string().contains("setWidgetInput"));
    assert!(row["children"][0]["bindings"].to_string().contains("commit"));
    host.connect_ports(&source, "number", &shape, "width").unwrap();
    let tree = render(&host.host_snapshot, &[shape], labels, &semio_framework_plugin::TreeWindows::unhosted(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native, &[], "{}", "{}", true).unwrap();
    let connected = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(tree)).unwrap();
    assert!(connected.contains("Connected"));
    assert!(!connected.contains("procedural-play-inspector.input.width.input"));
    host.retire_cold();
}


#[test]
fn inspector_brep_controls_keep_geometry_connected_and_numeric_commits_localized() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{empty_generation3d_snapshot, with_host};
    use semio_framework_ui_locale::{Locale, Terminology};
    use semio_s_artifact_procedural_generation3d::editor::generation3d::terminology::Generation3dLabels;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    crate::flow_operators::installed();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧫️fixtures/🔣️.json")).unwrap();
    fn node<'a>(tree: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
        if tree["key"] == key { return Some(tree); }
        tree["children"].as_array()?.iter().find_map(|child| node(child, key))
    }
    for case in fixture["selectedBrepControls"]["cases"].as_array().unwrap() {
        let mut snapshot = crate::SnapshotRead::new(empty_generation3d_snapshot());
        let projected = with_host(&snapshot.host_snapshot, |host| {
            let selected = host.add_widget(&serde_json::json!({"kind":"neuron","id":"selected","neuronKind":case["kind"]}).to_string(), 200.0, 0.0).unwrap();
            if let Some(inputs) = case["readOnlyInputs"].as_array() {
                let params = inputs.iter().map(|row| (row["channel"].as_str().unwrap().to_string(), serde_json::json!({"$schema":"text","value":row["value"]}))).collect::<serde_json::Map<_, _>>();
                host.set_neuron_params(&selected, &serde_json::Value::Object(params).to_string()).unwrap();
            }
            for connection in case["connections"].as_array().unwrap() {
                let channel = connection["channel"].as_str().unwrap();
                let source = host.add_widget(&serde_json::json!({"kind":"variable","id":connection["source"]}).to_string(), 0.0, 0.0).unwrap();
                host.set_variable_name(&source, connection["port"].as_str().unwrap());
                host.set_variable_schema(&source, match channel { "edges" | "openFaces" => "list", "shell" => "shell", _ => "geometry" });
                host.connect_ports(&source, connection["port"].as_str().unwrap(), &selected, channel).unwrap();
            }
            host.host_snapshot.clone()
        });
        std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
        let before = semio_framework_pack_json::to_json_string(&*snapshot);
        let outputs = case["outputs"].as_array().unwrap().iter().map(|row| (row["port"].as_str().unwrap().to_string(), row["value"].clone())).collect::<serde_json::Map<_, _>>();
        let evaluation = serde_json::json!({"selected":{"out":outputs}}).to_string();
        for (locale, locale_key, labels) in [(Locale::En, "en", &Generation3dLabels::NATIVE_EN), (Locale::De, "de", &Generation3dLabels::NATIVE_DE)] {
            for status in fixture["selectedBrepControls"]["statuses"].as_array().unwrap() {
                let statuses = serde_json::json!({"selected":{"status":status}}).to_string();
                let built = render(&snapshot.host_snapshot, &["selected".into()], labels, &semio_framework_plugin::TreeWindows::unhosted(), locale, Terminology::Native, &[], &evaluation, &statuses, true).unwrap();
                let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(built)).unwrap();
                let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
                for expected in case["controls"].as_array().unwrap() {
                    let control = node(&tree, &format!("procedural-play-inspector.input.{}.input", expected["channel"].as_str().unwrap())).unwrap_or_else(|| panic!("{projection}"));
                    assert_eq!(control["component"]["kind"], expected["kind"]);
                    assert_eq!(control["component"]["value"], expected["value"].as_f64().unwrap().to_string());
                    assert_eq!(control["component"]["commit"], expected["commit"]);
                    assert_eq!(control["accessibility"]["label"], expected[locale_key]);
                    let binding = &control["bindings"][0];
                    assert_eq!(binding["trigger"], expected["trigger"]);
                    assert_eq!(binding["action"]["name"], expected["action"]);
                    assert_eq!(binding["args"]["widgetId"], "selected");
                    assert_eq!(binding["args"]["channel"], expected["channel"]);
                }
                for expected in case["connections"].as_array().unwrap() {
                    let row = node(&tree, &format!("procedural-play-inspector.input.{}", expected["channel"].as_str().unwrap())).unwrap_or_else(|| panic!("{projection}"));
                    let text = row.to_string();
                    assert!(text.contains(expected[locale_key].as_str().unwrap()));
                    assert!(text.contains(labels.input_connected.as_str()));
                    assert!(text.contains(expected["source"].as_str().unwrap()));
                    assert!(!text.contains("setWidgetInput"));
                }
                if let Some(inputs) = case["readOnlyInputs"].as_array() {
                    for expected in inputs {
                        let channel = expected["channel"].as_str().unwrap();
                        let row = node(&tree, &format!("procedural-play-inspector.input.{channel}")).unwrap_or_else(|| panic!("{projection}"));
                        let text = row.to_string();
                        assert!(text.contains(expected[locale_key].as_str().unwrap()), "{projection}");
                        for label in expected["labels"].as_array().unwrap() { assert!(text.contains(label.as_str().unwrap()), "{projection}"); }
                        assert!(!text.contains("setWidgetInput"), "derived topology selection labels stay read-only: {projection}");
                        assert!(node(row, &format!("procedural-play-inspector.input.{channel}.input")).is_none());
                    }
                }
                for expected in case["outputs"].as_array().unwrap() {
                    let row = node(&tree, &format!("procedural-play-inspector.output.{}", expected["port"].as_str().unwrap())).unwrap_or_else(|| panic!("{projection}"));
                    let text = row.to_string();
                    assert!(text.contains(expected[locale_key].as_str().unwrap()));
                    assert!(text.contains(match status.as_str().unwrap() { "stale" => labels.mesh_output_stale.as_str(), "error" => labels.mesh_output_error.as_str(), _ => expected["expectedText"].as_str().unwrap() }), "{}: {projection}", case["id"]);
                    assert!(!text.contains("setWidgetInput"));
                }
                assert!(semio_framework_plugin::artifact_app_laws::document_input_commit_findings(&tree, &|name| name == "setWidgetInput").is_empty());
                assert_eq!(semio_framework_pack_json::to_json_string(&*snapshot), before);
            }
        }
        if case["kind"] == "brep.brep" {
            let window = &fixture["selectedBrepControls"]["selectionWindow"];
            let channel = window["channel"].as_str().unwrap();
            let key = format!("procedural-play-inspector.input.{channel}");
            let start = window["start"].as_str().unwrap().parse::<u64>().unwrap();
            let values = (0..window["count"].as_u64().unwrap()).map(|index| (start + index).to_string()).collect::<Vec<_>>();
            let projected = with_host(&snapshot.host_snapshot, |host| {
                host.set_neuron_params("selected", &serde_json::json!({(channel):{"$schema":"text","value":serde_json::to_string(&values).unwrap()}}).to_string()).unwrap();
                host.host_snapshot.clone()
            });
            std::mem::replace(&mut snapshot.host_snapshot, projected).retire_cold();
            let before = semio_framework_pack_json::to_json_string(&*snapshot);
            for (locale, labels) in [(Locale::En, &Generation3dLabels::NATIVE_EN), (Locale::De, &Generation3dLabels::NATIVE_DE)] {
                let view = semio_framework_plugin::ViewModel { tree_windows: vec![semio_framework_plugin::TreeWindowRequest { body_key: GENERATION_3D_PLAY_BODY_INSPECTION.into(), node_key: key.clone(), open: Some(true), offset: window["offset"].as_u64().unwrap() as u32, rows: window["rows"].as_u64().unwrap() as u32 }], ..semio_framework_plugin::ViewModel::new(locale, Terminology::Native) };
                let built = render(&snapshot.host_snapshot, &["selected".into()], labels, &semio_framework_plugin::TreeWindows::for_body(&view, GENERATION_3D_PLAY_BODY_INSPECTION), locale, Terminology::Native, &[], &evaluation, "{}", true).unwrap();
                let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(built)).unwrap();
                let tree: serde_json::Value = serde_json::from_str(&projection).unwrap();
                for (index, expected) in window["expected"].as_array().unwrap().iter().enumerate() {
                    let row = node(&tree, &format!("{key}.{}", window["offset"].as_u64().unwrap() as usize + index)).unwrap_or_else(|| panic!("{projection}"));
                    assert!(row.to_string().contains(expected.as_str().unwrap()));
                    assert!(!row.to_string().contains("setWidgetInput"));
                }
                assert!(node(&tree, &format!("{key}.0")).is_none());
                assert_eq!(semio_framework_pack_json::to_json_string(&*snapshot), before);
            }
        }
        eprintln!("[DEBUG] selected BRep {} retains connected topology inputs, localized absolute controls and read-only declared outputs", case["kind"]);
    }
}
