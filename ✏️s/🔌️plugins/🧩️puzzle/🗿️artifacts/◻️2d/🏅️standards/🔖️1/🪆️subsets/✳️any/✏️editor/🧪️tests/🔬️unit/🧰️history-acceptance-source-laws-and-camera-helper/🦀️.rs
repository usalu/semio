// 🧰️ History acceptance, source laws and camera helper: the history edit acceptance law, the editor source reader, the cursorized cohort route laws and the rendered camera helper.

semio_framework_plugin::history_edit_acceptance_law!("puzzle", Puzzle2dPlayApp, puzzle2d_manifest_for_tests, "../..");
use crate::editor::puzzle2d::testkit::*;
use crate::standards::v1::subsets::any::schema::mutations::apply_puzzle2d_mutation;
use crate::editor::puzzle2d::*;
use crate::Puzzle2dSnapshot;
use semio_framework::SET_ACTIVE_UTILITY_ACTION_ID;
use semio_framework_plugin::PluginApp;
use store::os_store::backbone::Backbone;
use store::os_store::backbone::BackboneMessage;
use store::os_store::backbone::MemoryBackbone;

/// 📚️ The editor module's production source as the unsplit editor file held it: every mounted child and split part first, then the editor file itself, so the `UnitTests` region marker still closes the production prefix.
pub(crate) fn editor_source() -> String {
    [
        fixture_text!("../../../../🕹️interaction/🦀️.rs"),
        fixture_text!("../../../../🧰️kit/🦀️.rs"),
        fixture_text!("../../../../🧵️retained/🦀️.rs"),
        fixture_text!("../../../../🧵️retained/🧱️board-event-limits-window-export-and-dispatch-work/🦀️.rs"),
        fixture_text!("../../../../🧵️retained/🕸️force-layout-work/🦀️.rs"),
        fixture_text!("../../../../🧵️retained/🎨️force-layout-command-and-redraw-handles-work/🦀️.rs"),
        fixture_text!("../../../../🖌️session/🦀️.rs"),
        fixture_text!("../../../../🔧️play-app-retained-commands-and-tool-proofs/🦀️.rs"),
        fixture_text!("../../../../🧠️artifact-editor/🦀️.rs"),
        fixture_text!("../../../../🚀️app-assembly/🦀️.rs"),
        fixture_text!("../../../../🦀️.rs"),
    ]
    .concat()
}

fn cohort_routes_are_cursorized(source: &str) -> bool {
    [
        r#""forceLayout" => work!(|request| Puzzle2dForceLayoutWork::default())"#,
        r#""setActiveExample" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new("setActiveExample", puzzle2d_active_example_reduce, puzzle2d_active_example_extent))"#,
        "Effect::LoadDocument",
        "Puzzle2dForceStage::Nodes",
        "Puzzle2dForceStage::Handles",
        "Puzzle2dForceStage::Edges",
        "Puzzle2dForceStage::Repel",
        "Puzzle2dForceStage::Springs",
        "Puzzle2dForceStage::Integrate",
        "Puzzle2dForceStage::Emit",
        r#"matches!(command.action_id(), "addNode").then_some(1)"#,
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains(r#""forceLayout" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains("Puzzle2dActiveExampleWork")
}

#[test]
fn cohort_hostile_static_law_rejects_one_grant_complex_routes_and_missing_cursors() {
    let source = editor_source();
    let source = source.as_str();
    assert!(cohort_routes_are_cursorized(source));
    for (retained, direct) in [
        (r#""forceLayout" => work!(|request| Puzzle2dForceLayoutWork::default())"#, r#""forceLayout" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle2d_retained_reduce, puzzle2d_retained_extent))"#),
        (r#"Effect::LoadDocument { pack, spr }"#, r#"Effect::LoadDocument { pack, spr }; Puzzle2dActiveExampleWork::default()"#),
    ] {
        assert!(!cohort_routes_are_cursorized(&source.replace(retained, direct)));
    }
    for marker in [
        "Puzzle2dForceStage::Nodes",
        "Puzzle2dForceStage::Handles",
        "Puzzle2dForceStage::Edges",
        "Puzzle2dForceStage::Repel",
        "Puzzle2dForceStage::Springs",
        "Puzzle2dForceStage::Integrate",
        "Puzzle2dForceStage::Emit",
    ] {
        // 🧨️ EVERY occurrence, not the first: a stage marker like `Puzzle2dForceStage::Nodes` appears
        // more than once in the source (its declaration and each arm that advances to it), so
        // `replacen(.., 1)` left one behind and `cohort_routes_are_cursorized` still found it — the
        // negative snapshot could not bite and the law proved nothing about that cursor.
        assert!(!cohort_routes_are_cursorized(&source.replace(marker, "cursor-removed")), "missing retained cursor was falsely accepted: {marker}");
    }
}

/// 🎥️ Recovers the rendered pane camera `(x, y, zoom)` from a rendered `UiNode`'s embedded
/// `Board2dScene.cameraJson` — the only externally observable surface for the runtime camera
/// (the camera is never a document field, so it cannot be read back off `app.snapshot()`).
fn rendered_camera(rendered: &str) -> (f64, f64, f64) {
    fn find_camera_json(value: &Value) -> Option<String> {
        if let Some(json) = value.get("cameraJson").and_then(Value::as_str) {
            return Some(json.to_string());
        }
        match value {
            Value::Object(map) => map.iter().find_map(|(_, value)| find_camera_json(value)),
            Value::Array(items) => items.iter().find_map(find_camera_json),
            _ => None,
        }
    }
    let value: Value = semio_framework_pack_json::from_json_str(rendered, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rendered node parses");
    let camera_json = find_camera_json(&value).expect("rendered scene must carry cameraJson");
    let camera: Value = semio_framework_pack_json::from_json_str(&camera_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("cameraJson parses");
    (camera.get("x").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("y").and_then(Value::as_f64).unwrap_or(f64::NAN), camera.get("zoom").and_then(Value::as_f64).unwrap_or(f64::NAN))
}
