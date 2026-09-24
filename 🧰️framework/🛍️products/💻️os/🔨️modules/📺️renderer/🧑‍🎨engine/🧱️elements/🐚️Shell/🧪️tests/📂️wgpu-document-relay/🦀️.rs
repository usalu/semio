//! 📂️ The `os.open-artifact` relay's document half on both shell builds: which bindings each
//! document host admits before any actor exists, the default binding set a single-plugin browser
//! playground (no identity, no space, no data dir) hands the relay, and the one execution-target
//! lease a hub document of a codec-less kind may bind.

use super::*;

fn hub() -> PersistenceBinding {
    PersistenceBinding::Hub { base_url: "http://127.0.0.1:7501".into(), space_id: "space-1".into(), surface: None }
}

fn folder() -> PersistenceBinding {
    PersistenceBinding::Folder { path: std::path::PathBuf::from("spaces/space-1") }
}

const NATIVE: ShellDocumentTransports = ShellDocumentTransports { folder: true, hub: true };
const BROWSER: ShellDocumentTransports = ShellDocumentTransports { folder: false, hub: true };
const NONE: ShellDocumentTransports = ShellDocumentTransports { folder: false, hub: false };

#[test]
fn a_host_serving_both_transports_admits_every_binding_kind() {
    for bindings in [Vec::new(), vec![hub()], vec![folder()], vec![hub(), folder()]] {
        assert_eq!(document_bindings_admitted(NATIVE, &bindings), Ok(()));
    }
}

#[test]
fn a_host_without_transports_serves_ephemeral_documents_and_refuses_what_it_cannot_persist() {
    assert_eq!(document_bindings_admitted(NONE, &[]), Ok(()));
    assert_eq!(document_bindings_admitted(NONE, &[hub(), folder()]), Err("document-binding.hub-transport-unavailable"));
    assert_eq!(document_bindings_admitted(NONE, &[folder()]), Err("document-binding.folder-unavailable"));
    assert_eq!(document_bindings_admitted(ShellDocumentTransports { folder: true, hub: false }, &[folder(), hub()]), Err("document-binding.hub-transport-unavailable"));
}

#[test]
fn the_browser_host_serves_hub_documents_and_refuses_a_folder() {
    assert_eq!(document_bindings_admitted(BROWSER, &[]), Ok(()));
    assert_eq!(document_bindings_admitted(BROWSER, &[hub()]), Ok(()));
    assert_eq!(document_bindings_admitted(BROWSER, &[folder()]), Err("document-binding.folder-unavailable"));
    assert_eq!(document_bindings_admitted(BROWSER, &[hub(), folder()]), Err("document-binding.folder-unavailable"));
}

#[test]
fn a_playground_relay_opens_an_ephemeral_local_document_on_this_build() {
    let bindings = default_persistence_bindings(None, None, None, None);
    assert!(bindings.is_empty());
    assert_eq!(store_sync::sync::bindings_data_class(&bindings), store_sync::sync::PersistenceDataClass::EphemeralLocalOnly);
    assert_eq!(document_bindings_admitted(SHELL_DOCUMENT_TRANSPORTS, &bindings), Ok(()));
}

#[test]
fn a_refused_document_attach_is_localized() {
    let english = shell_chrome_string("open-artifact.document-failed", false);
    let german = shell_chrome_string("open-artifact.document-failed", true);
    assert!(english.contains("document"), "{english}");
    assert!(german.contains("Dokument"), "{german}");
    assert_ne!(english, german);
}

#[test]
fn this_build_declares_the_transports_its_document_actor_serves() {
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(SHELL_DOCUMENT_TRANSPORTS, NATIVE);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(SHELL_DOCUMENT_TRANSPORTS, BROWSER);
}

fn note_lease() -> semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1 {
    use semio_framework_os_kernel::os_directory as directory;
    directory::DocumentExecutionTargetLeaseFieldsV1 {
        schema: "semio.os.document-execution-target-lease/v1".into(),
        version: 1,
        scope: directory::DocumentScope::new("space-1", "note-1"),
        descriptor_digest_v1: "11".repeat(32),
        catalog: directory::DocumentOpenCatalogV1 { generation_id: "22".repeat(32) },
        package: directory::DocumentOpenPackageV1 {
            plugin_id: "note".into(),
            package_id: "semio:note".into(),
            version: "0.1.0".into(),
            component_sha256: "33".repeat(32),
            component_blake3: "44".repeat(32),
            descriptor_byte_sha256: "55".repeat(32),
            execution_protocol: directory::DocumentExecutionProtocolV1 { app_channel_version: semio_framework_os_kernel::os_spr::CHANNEL_VERSION },
        },
        component: directory::DocumentExecutionTargetComponentV1 { sha256: "33".repeat(32), blake3: "44".repeat(32), byte_length: 1024 },
        descriptor: directory::DocumentExecutionTargetDescriptorV1 { sha256: "55".repeat(32), byte_length: 512 },
        browser_actor: directory::schema::DocumentExecutionTargetBrowserActorV1::None,
        artifact: directory::DocumentOpenArtifactV1 { kind: "s.note.note".into(), schema: "note.document".into(), pack_schema_hash: "66".repeat(32) },
        parent_dialect: directory::DocumentOpenParentDialectV1 { artifact_kind: "s.note.note".into(), standard: "1".into(), subset: "*".into() },
        surface: directory::DocumentOpenSurfaceV1 {
            surface_id: "s.note.note@1/*#editor".into(),
            app_id: "s.note.note@1/*#editor".into(),
            window_kind_id: "note-composite".into(),
            role: directory::DocumentOpenSurfaceRoleV1::Editor,
            renderer_target: directory::DocumentOpenRendererTargetV1::Wgpu,
        },
        grant: directory::DocumentOpenGrantV1 { read: true, write: true, observe: true },
        checkpoint: directory::DocumentOpenCheckpointV1 {
            checkpoint_id: "77".repeat(32),
            descriptor_digest_v1: "11".repeat(32),
            baseline_frontier: directory::ArtifactFrontier { document_id: "note-1".into(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: directory::ArtifactHash::new([0; 32]) },
            aggregate_sha256: "88".repeat(32),
        },
        revalidation: directory::DocumentOpenRevalidationV1 { directory_revision: 1, membership_generation: 1, session_generation: Some(1), share_generation: None },
    }
}

/// 🪪️ A hub-selected lease binds only the package this shell mounted: plugin, package, component
/// digest, the document's schema and the requested surface — every other combination names code that
/// is not running here, and a program that cannot name its own package binds none.
#[test]
fn an_execution_target_lease_binds_only_the_mounted_package() {
    let lease = note_lease();
    let component = "33".repeat(32);
    let other_component = "99".repeat(32);
    let admitted = |plugin: &str, package: Option<&str>, sha: Option<&str>, schema: &str, surface: &str| document_execution_target_admitted(&lease, plugin, package, sha, schema, surface);
    assert_eq!(admitted("note", Some("semio:note"), Some(&component), "note.document", "s.note.note@1/*#editor"), Ok(()));
    assert_eq!(admitted("draw", Some("semio:note"), Some(&component), "note.document", "s.note.note@1/*#editor"), Err("document-execution-target.plugin-mismatch"));
    assert_eq!(admitted("note", Some("semio:draw"), Some(&component), "note.document", "s.note.note@1/*#editor"), Err("document-execution-target.package-mismatch"));
    assert_eq!(admitted("note", None, Some(&component), "note.document", "s.note.note@1/*#editor"), Err("document-execution-target.package-mismatch"));
    assert_eq!(admitted("note", Some("semio:note"), Some(&other_component), "note.document", "s.note.note@1/*#editor"), Err("document-execution-target.component-mismatch"));
    assert_eq!(admitted("note", Some("semio:note"), None, "note.document", "s.note.note@1/*#editor"), Err("document-execution-target.component-mismatch"));
    assert_eq!(admitted("note", Some("semio:note"), Some(&component), "draw.document", "s.note.note@1/*#editor"), Err("document-execution-target.schema-mismatch"));
    assert_eq!(admitted("note", Some("semio:note"), Some(&component), "note.document", "s.note.note@1/*#viewer"), Err("document-execution-target.surface-mismatch"));
}

/// 🌐️ The sync card's remote uri names all three parts exactly as React's `parseRemoteBackboneUri`
/// reads them — the regression React's C1c fixed was a parser that took `space/document` as the space.
#[test]
fn a_remote_backbone_uri_names_host_space_and_document() {
    assert_eq!(
        parse_remote_backbone_uri("remote://127.0.0.1:7800/space-1/doc-a"),
        Some(RemoteBackboneUri { host_port: "127.0.0.1:7800".into(), space_id: "space-1".into(), document_id: "doc-a".into() })
    );
    assert_eq!(parse_remote_backbone_uri("remote://127.0.0.1:7800/space-1"), None, "no document part");
    assert_eq!(parse_remote_backbone_uri("remote:///space-1/doc-a"), None, "no host part");
    assert_eq!(parse_remote_backbone_uri("folder:///tmp/space"), None, "not a remote uri");
    assert!(ShellState::parse_persistence_binding("remote://127.0.0.1:7800/space-1").is_err(), "a partial remote uri is refused, never guessed");
}

/// 🧭️ The `/hub` workspace overlay is shell chrome: focusing one of its controls must never make it
/// the active plugin window (measured: it did, and the next plugin action faulted the frame with
/// "action window instance framework.hub has no declared kind").
#[test]
fn the_hub_workspace_overlay_is_never_the_active_plugin_window() {
    let shell = super::panel_anchor_model_tests::host_test_shell();
    assert!(shell.retained_surface_is_panel(crate::hub_connection::FRAMEWORK_HUB_PANEL_ID));
    let plugin_window = shell.session.as_ref().expect("host test shell has a session").app.window_kinds.first().id.clone();
    assert!(!shell.retained_surface_is_panel(&plugin_window), "a plugin window stays a window");
}

/// 🔄️ Every sync-card verb republishes the mounted Sync panel: choosing Remote must show its path input
/// and a typed path must enable Attach (measured: the card kept its first publication, so a browser user
/// could pick Remote and never see where to type the hub document).
#[test]
fn every_sync_card_verb_republishes_the_sync_panel() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let initial = shell.publish_shell_panel_document(FRAMEWORK_SYNC_PANEL_TAB_ID).expect("sync panel publication").expect("the sync panel owns a retained document");
    shell.panel_documents.insert(FRAMEWORK_SYNC_PANEL_TAB_ID.into(), initial);
    for (action, args) in [("selectRemote", None), ("setSyncDraft", Some(serde_json::json!({ "value": "127.0.0.1:7800/space-1/doc-a" })))] {
        let before = shell.panel_documents.get(FRAMEWORK_SYNC_PANEL_TAB_ID).unwrap().header().expect("sync header before");
        semio_framework_async::block_on(shell.dispatch_action(ActionDescriptor { controller_id: "framework.sync".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) })).expect("sync card verb");
        let after = shell.panel_documents.get(FRAMEWORK_SYNC_PANEL_TAB_ID).unwrap().header().expect("sync header after");
        assert!(after.revision != before.revision && after.generation > before.generation, "{action} republished the sync panel");
    }
    let tree = serde_json::to_string(&shell.build_sync_attach_ui()).expect("sync ui json");
    assert!(tree.contains("framework.sync.remote.path") && tree.contains("127.0.0.1:7800/space-1/doc-a"), "the republished card carries the path input and the typed uri");
}

/// 🪟️ A panel control's action runs in the active window, never in the panel it was pressed in
/// (measured: the note artifact panel's "Add Text" faulted with "action window instance
/// framework.panel.artifact has no declared kind"); a window's own action keeps its window.
#[test]
fn a_panel_action_is_dispatched_in_the_active_window() {
    let panels = ["framework.panel.artifact", "framework.panel.history"];
    assert_eq!(action_window_instance_id(Some("framework.panel.artifact"), &panels, None, Some("note-composite"), "note-composite"), "note-composite");
    assert_eq!(action_window_instance_id(Some("framework.panel.artifact"), &panels, Some("note-navigator"), Some("note-composite"), "note-composite"), "note-navigator");
    assert_eq!(action_window_instance_id(Some("note-navigator"), &panels, None, Some("note-composite"), "note-composite"), "note-navigator");
    assert_eq!(action_window_instance_id(None, &panels, None, None, "note-composite"), "note-composite");
}
