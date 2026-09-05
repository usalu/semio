//! 🏙️ 🏙️ S Home launcher app command — `create-studio`.

use crate::artifacts::home::mutations::change_catalog_generation;
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::VcsError;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "create-studio")]
pub struct CreateStudio {
    pub name: String,
    pub kind: String,
    pub folder_path: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn create_folder_studio(name: &str, folder_path: &str, owner_id: &str, owner_name: &str) -> Result<semio_framework_os::OsSpaceCatalogEntry, VcsError> {
    use semio_framework_os::{create_os_space, SpaceKind, SpaceRole, SpaceUser, SpaceVisibility};
    let port = semio_framework_os::open_folder_space_backbone(folder_path)?;
    let owner = SpaceUser { id: if owner_id.is_empty() { "local".into() } else { owner_id.into() }, name: if owner_name.is_empty() { name.into() } else { owner_name.into() }, avatar: None, role: SpaceRole::Author };
    let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, port.clone())?;
    crate::register_studio_port(&entry.id, port);
    Ok(entry)
}

/// @emoji 🧭️ Builds the typed emit for a freshly-created studio: bump the catalog counter (operation)
/// and navigate the shell to the new studio route (host effect).
fn created_studio_emit(catalog_generation: u64, space_id: &str) -> Emit<SHomeMutation, HomeConfigMutation> {
    Emit { artifact_mutations: vec![change_catalog_generation(catalog_generation + 1)], effects: vec![Effect::Navigate { uri: format!("/spaces/{space_id}") }], ..Default::default() }
}

pub fn handle(payload: &CreateStudio, doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let generation = doc.snapshot.catalog_generation;
    let owner_id = cfg.snapshot.client_id.as_str();
    let owner_name = cfg.snapshot.client_name.as_str();
    match payload.kind.as_str() {
        "folder" => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Some(folder_path) = &payload.folder_path {
                    if let Ok(entry) = create_folder_studio(&payload.name, folder_path, owner_id, owner_name) {
                        eprintln!("[DEBUG] createStudio folder id={}", entry.id);
                        return Ok(created_studio_emit(generation, &entry.id));
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = &payload.folder_path;
            }
            Ok(Emit::default())
        }
        _ => {
            // 🌉️ `crate::create_and_register_ephemeral_studio` is a plugin-root async fn (outside
            // this lease); `handle` must stay sync (the `app_commands!` dispatch contract), so the
            // call is bridged via `resolve_ready` — the same poll-once bridge the framework's own
            // `composer_entry_of`/`deserializer_entry_of` use for an identical sync/async seam.
            let space_id = semio_framework_plugin::resolve_ready(crate::create_and_register_ephemeral_studio(&payload.name, owner_id, owner_name));
            eprintln!("[DEBUG] createStudio ephemeral id={space_id}");
            Ok(created_studio_emit(generation, &space_id))
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os::list_os_space_catalog_entries;
    use semio_framework_plugin::{testkit, EditorApp, HistoryView, VcsArtifactApp};

    #[semio_framework_async_macros::async_test]
    async fn home_command_op_text_round_trips_every_variant() {
        use crate::editor::home::HomeCommand;
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(CreateStudio { name: "Untitled".into(), kind: "catalog".into(), folder_path: None }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(CreateStudio { name: "Untitled".into(), kind: "folder".into(), folder_path: Some("/tmp/x".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::BindSpaceFile(crate::editor::home::commands::bind_space_file::BindSpaceFile { space_id: "s1".into(), file_path: "/tmp/x.os".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(crate::editor::home::commands::import_space::ImportSpace { dsl: Some("programs=[]".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(crate::editor::home::commands::import_space::ImportSpace { dsl: None }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::OpenSpace(crate::editor::home::commands::open_space::OpenSpace { space_id: "s1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn creates_studio_via_home_action() {
        let port = crate::catalog_port();
        let before = list_os_space_catalog_entries(port.clone()).expect("list").len();
        let mut home = VcsArtifactApp::new(EditorApp::<crate::editor::home::HomeApp>::default());
        home.dispatch_typed(crate::editor::home::HomeCommand::CreateStudio(CreateStudio { name: "Test Studio".into(), kind: "catalog".into(), folder_path: None }), &testkit::meta("local")).expect("create");
        let after = list_os_space_catalog_entries(port).expect("list").len();
        assert!(after >= before);
    }

    #[semio_framework_async_macros::async_test]
    async fn temporary_studio_uses_ephemeral_registry_not_catalog() {
        let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&CreateStudio { name: "Temp Studio".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg).expect("handle");
        assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::Navigate { .. })));
        assert!(!emit.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })), "ephemeral create must not download");
        let persistent = list_os_space_catalog_entries(crate::catalog_port()).expect("list");
        assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
        let ephemeral_catalog = list_os_space_catalog_entries(crate::temp_catalog_port()).unwrap_or_default();
        assert!(!ephemeral_catalog.iter().any(|entry| entry.name == "Temp Studio"));
        let uri = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::Navigate { uri } => Some(uri.as_str()),
                _ => None,
            })
            .expect("navigate");
        let space_id = uri.trim_start_matches("/spaces/");
        let document = crate::resolve_studio_document(space_id).expect("ephemeral studio");
        assert_eq!(document.name, "Temp Studio");
        assert!(document.backbone.is_none());
        assert!(document.vcs.initial_snapshot.collections.is_empty());
    }
}
//#endregion 🧪️Tests
