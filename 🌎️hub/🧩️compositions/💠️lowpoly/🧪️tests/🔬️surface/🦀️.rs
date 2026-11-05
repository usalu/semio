/// 🧪️ Contract §2.5 (closed by W0-F, `📓️w0-f-report.md` Gap 2): the real framework test context
/// functions now exist — this packet uses them directly rather than the pilot's local stand-ins.
use semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn lowpoly_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::lowpoly::LowpolyViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn lowpoly_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::lowpoly::LowpolyPlayApp, crate::viewer::lowpoly::LowpolyViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn lowpoly_activation_matches_registered_artifact() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("activation.json")).expect("activation contract");
    let expected = contract["artifactKind"].as_str().expect("artifact kind");
    let descriptor: serde_json::Value = serde_json::from_str(include_str!("../../🧩️component/🔣️.json")).expect("independent descriptor parser");
    assert_eq!(crate::artifacts::lowpoly::artifact_kind().id, expected);
    assert_eq!(semio_s_artifact_lowpoly_lowpoly_model::snapshot::LOWPOLY_DIALECT.artifact_kind, expected);
    assert_eq!(descriptor["activationEvents"][0]["onArtifactKind"]["kind"], expected);
    assert_eq!(descriptor["manifest"]["apps"][0]["artifactKinds"][0]["id"], expected);
    assert_eq!(descriptor["manifest"]["apps"][0]["io"]["artifact"]["id"], expected);
    let plugin = super::plugin().expect("plugin assembles");
    assert!(plugin.descriptor_extras().activation_events.iter().any(|event| matches!(event, semio_framework_plugin::kernel::ActivationEvent::OnArtifactKind { kind } if kind == expected)));
}

#[test]
fn lowpoly_built_manifest_projects_exact_cancellation_contracts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("cancellation.json")).unwrap();
    let plugin = super::plugin().expect("typed plugin assembly");
    let definition = plugin.manifest.apps.iter().find(|app| app.id == fixture["controller"].as_str().unwrap()).unwrap();
    let descriptor: serde_json::Value = serde_json::from_str(include_str!("../../🧩️component/🔣️.json")).unwrap();
    let committed = descriptor["manifest"]["apps"].as_array().unwrap().iter().find(|app| app["id"] == fixture["controller"]).unwrap();
    for tool in fixture["tools"].as_array().unwrap() {
        let id = tool.as_str().unwrap();
        let actions = definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).filter(|action| action.id == id).collect::<Vec<_>>();
        assert!(!actions.is_empty(), "{id} is declared");
        assert!(actions.iter().all(|action| action.semantics.execution.cancellable), "{id} projects per-operation cancellation");
        let committed_actions = committed["actions"].as_array().unwrap().iter().chain(committed["windowKinds"].as_array().unwrap().iter().flat_map(|window| window["actions"].as_array().unwrap().iter())).filter(|action| action["id"] == id).collect::<Vec<_>>();
        assert!(!committed_actions.is_empty());
        assert!(committed_actions.iter().all(|action| action["semantics"]["execution"]["cancellable"] == fixture["cancellable"]), "{id} committed descriptor matches typed manifest");
    }
}

#[test]
fn lowpoly_built_manifest_exposes_required_localized_snapshot_json_arguments() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("snapshot-arguments.json")).unwrap();
    let plugin = super::plugin().expect("typed snapshot import manifest");
    let manifest = serde_json::to_value(&plugin.manifest).unwrap();
    let editor = manifest["apps"].as_array().unwrap().iter().find(|app| app["id"] == fixture["controller"]).unwrap();
    for expected in fixture["actions"].as_array().unwrap() {
        let actions = editor["actions"].as_array().unwrap().iter().filter(|action| action["id"] == expected["id"]).collect::<Vec<_>>();
        assert_eq!(actions.len(), 1);
        let action = actions[0];
        assert_eq!(action["inPalette"], true);
        let args = action["args"].as_array().unwrap();
        assert_eq!(args.len(), 1);
        let expected = &expected["args"][0];
        assert_eq!(args[0]["id"], expected["id"]);
        assert_eq!(args[0]["schema"]["kind"], expected["kind"]);
        assert_eq!(args[0]["required"], expected["required"]);
        assert_eq!(args[0]["label"]["native"]["en"], expected["label"]["en"]);
        assert_eq!(args[0]["label"]["native"]["de"], expected["label"]["de"]);
    }
}
