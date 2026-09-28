//! 🛡️ The stdio component ships a bounded, declared app fleet — never the 88-subset library fleet.

use std::collections::BTreeSet;

/// 📏️ Apps one stdio component may assemble. Every registered app monomorphises the whole app runtime
/// (`VcsArtifactApp<EditorApp<E>/ViewerApp<V>, Members>`, its retained tool-job factories and snapshot-edit
/// factory) and is live code inside the component: the 18-app document fleet compiles in ~1.8 GB, while the
/// 176-app library fleet drove the wasm-dev rustc to 85 GB in one codegen unit and `wasm-component-ld` refused it
/// ("functions count exceeds limit of 1000000", chain b3 run 2, 2026-09-27). A family beyond this bound ships as
/// its own component.
const SHIPPED_APP_CEILING: usize = 24;

/// 📋️ The declared shipped editors: the `app` of every `[[package.metadata.semio.playground]]` row, which
/// declares ONE row per shipped editor (see the comment above those rows in `Cargo.toml`).
fn declared_shipped_editors() -> BTreeSet<String> {
    let manifest = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
    manifest
        .split("[[package.metadata.semio.playground]]")
        .skip(1)
        .filter_map(|row| row.lines().find_map(|line| line.strip_prefix("app = \"").and_then(|rest| rest.strip_suffix('"')).map(str::to_string)))
        .collect()
}

/// 🛡️ LAW: the default (`plugin-root`) assembly registers exactly the declared shipped editors plus one viewer per
/// shipped dialect, and stays under [`SHIPPED_APP_CEILING`] — so widening the component is a declared, reviewed
/// change of the playground rows, never a feature edit that silently ships the library fleet.
#[cfg(not(feature = "full-app-catalog"))]
#[test]
fn the_shipped_component_assembles_exactly_the_declared_bounded_fleet() {
    let plugin = semio_s_plugin_stdio::plugin().expect("shipped stdio assembly");
    let editors = plugin.manifest.apps.iter().filter(|app| app.id.ends_with("#editor")).map(|app| app.id.clone()).collect::<BTreeSet<_>>();
    let viewers = plugin.manifest.apps.iter().filter(|app| app.id.ends_with("#viewer")).map(|app| app.id.trim_end_matches("#viewer").to_string()).collect::<BTreeSet<_>>();
    let declared = declared_shipped_editors();
    assert_eq!(editors, declared, "the shipped editors are exactly the playground-declared ones");
    assert_eq!(viewers, declared.iter().map(|id| id.trim_end_matches("#editor").to_string()).collect::<BTreeSet<_>>(), "every shipped dialect ships its viewer, and only those");
    assert!(plugin.manifest.apps.len() <= SHIPPED_APP_CEILING, "{} apps exceed the per-component ceiling of {SHIPPED_APP_CEILING}", plugin.manifest.apps.len());
}

/// 🧬️ LAW: the shipped assembly publishes every declared document schema, so a shipped editor's snapshot-edit route
/// resolves its contract — `plugin()` alone, no test-side registration. Measured before (S18 served matrix, native
/// probe 2026-09-28): `.artifact(…)` declarations kept their schemas plugin-local, `s.stdio.json` stayed unregistered
/// and every json `set-node` was refused `snapshot-edit.schema-unregistered`. The JSON oracle is serde_json.
#[semio_framework_async_macros::async_test]
async fn the_shipped_assembly_publishes_every_editor_document_schema_and_a_json_node_edit_lands() {
    use semio_framework_os_kernel::DslValue;
    use semio_framework_plugin::PluginApp;
    let plugin = semio_s_plugin_stdio::plugin().expect("shipped stdio assembly");
    for app in plugin.manifest.apps.iter().filter(|app| app.id.ends_with("#editor")) {
        let kind = app.id.split('@').next().expect("an app id names its artifact kind");
        assert!(semio_framework_os_kernel::kernel_artifact_schema_descriptor_registered(kind), "{} edits {kind}, whose document schema the assembly must publish", app.id);
    }
    let mut app = semio_framework_plugin::artifact_app_laws::new_registered_app::<semio_framework_plugin::EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>, _>(async {
        semio_framework_plugin::App { definition: semio_s_artifact_stdio_json::editor::json_any::create_json_editor(), examples: Vec::new() }
    })
    .await;
    let meta = semio_framework_plugin::artifact_app_laws::meta("local");
    let revision = semio_s_artifact_stdio_contract::window_kit_canonical_revision(app.test_document_revision());
    let source = r#"{"edited":["node",1,true]}"#;
    let args = DslValue::Object(vec![("nodeId".into(), DslValue::String("$".into())), ("revision".into(), DslValue::String(revision)), ("value".into(), DslValue::String(source.into()))]);
    app.handle_action("set-node", Some(&args), &meta).await.expect("the shipped json editor admits a revision-bound root node edit");
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id).await.expect("the node edit publishes");
    let edited = semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::write_json_text(&app.snapshot().expect("json snapshot").value);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&edited).expect("the edited document is JSON"), serde_json::from_str::<serde_json::Value>(source).expect("serde_json oracle"), "the published document is exactly the applied source");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
