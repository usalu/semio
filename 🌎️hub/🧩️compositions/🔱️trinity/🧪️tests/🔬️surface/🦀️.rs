use semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[test]
fn jack_activation_matches_both_registered_surface_ids() {
    use semio_framework_value::serde_json;
    use semio_plugin::kernel::ActivationEvent;

    let plugin = super::plugin().expect("registered Trinity plugin");
    let descriptor: serde_json::Value = serde_json::from_str(include_str!("../../🧩️component/🔣️.json")).expect("checked-in Trinity descriptor");
    let apps = descriptor["manifest"]["apps"].as_array().expect("descriptor apps");
    let activations = descriptor["activationEvents"].as_array().expect("descriptor activations");
    for app_id in ["s.trinity.jack@1/*#editor", "s.trinity.jack@1/*#viewer"] {
        let (expected, _) = app_id.split_once('@').expect("registered Jack surface grammar");
        let live = plugin.manifest.apps.iter().find(|app| app.id == app_id).expect("registered Jack surface");
        assert_eq!(live.dialect.artifact_kind, expected);
        let committed = apps.iter().find(|app| app["id"].as_str() == Some(app_id)).expect("checked-in Jack surface");
        assert_eq!(committed["dialect"]["artifactKind"].as_str(), Some(expected));
        assert!(plugin.descriptor_extras().activation_events.iter().any(|event| matches!(event, ActivationEvent::OnArtifactKind { kind } if kind == expected)));
        assert!(activations.iter().any(|event| event["onArtifactKind"]["kind"].as_str() == Some(expected)));
    }
}

#[semio_framework_async_macros::async_test]
async fn trinity_jack_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_trinity_jack::viewer::jack::TrinityJackViewer<semio_s_dev_trinity_jack::SemioJackComposition>>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn trinity_jack_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_trinity_jack::editor::jack::TrinityJackPlayApp<semio_s_dev_trinity_jack::SemioJackComposition>, semio_s_artifact_trinity_jack::viewer::jack::TrinityJackViewer<semio_s_dev_trinity_jack::SemioJackComposition>>().await;
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_trinity_rewriting::viewer::rewriting::TrinityRewritingViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn trinity_rewriting_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_trinity_rewriting::editor::rewriting::TrinityRewritingPlayApp, semio_s_artifact_trinity_rewriting::viewer::rewriting::TrinityRewritingViewer>().await;
}
