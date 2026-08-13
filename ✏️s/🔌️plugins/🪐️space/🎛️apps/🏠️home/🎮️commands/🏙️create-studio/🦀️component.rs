//! 🏙️ 🏙️ S Home launcher app command — `create-studio`.

use crate::apps::home::config::{HomeConfig, HomeConfigMutation};
use crate::artifacts::home::mutations::change_catalog_generation;
use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HostEffect};

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::VcsError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "create-studio")]
pub struct CreateStudio {
    pub name: String,
    pub kind: String,
    pub folder_path: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn create_folder_studio(name: &str, folder_path: &str) -> Result<semio_framework_os::OsSpaceCatalogEntry, VcsError> {
    use semio_framework_os::{create_os_space, SpaceKind, SpaceRole, SpaceUser, SpaceVisibility};
    let port = semio_framework_os::open_folder_space_backbone(folder_path)?;
    let owner = SpaceUser { id: "local".into(), name: name.into(), avatar: None, role: SpaceRole::Author };
    let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, port.clone())?;
    crate::apps::home::register_studio_port(&entry.id, port);
    Ok(entry)
}

/// @emoji 🧭️ Builds the typed emit for a freshly-created studio: bump the catalog counter (operation)
/// and navigate the shell to the new studio route (host effect).
fn created_studio_emit(catalog_generation: u64, space_id: &str) -> Emit<SHomeMutation, HomeConfigMutation> {
    Emit { artifact_mutations: vec![change_catalog_generation(catalog_generation + 1)], effects: vec![HostEffect::Navigate { uri: format!("/spaces/{space_id}") }], ..Default::default() }
}

pub fn handle(payload: &CreateStudio, doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let generation = doc.snapshot.catalog_generation;
    match payload.kind.as_str() {
        "folder" => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Some(folder_path) = &payload.folder_path {
                    if let Ok(entry) = create_folder_studio(&payload.name, folder_path) {
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
            let space_id = crate::apps::home::create_and_register_ephemeral_studio(&payload.name);
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
    use semio_framework_plugin::{testkit, HistoryView, VcsArtifactApp};

    #[test]
    fn home_command_op_text_round_trips_every_variant() {
        use crate::apps::home::HomeCommand;
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(CreateStudio { name: "Untitled".into(), kind: "catalog".into(), folder_path: None }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(CreateStudio { name: "Untitled".into(), kind: "folder".into(), folder_path: Some("/tmp/x".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::BindSpaceFile(crate::apps::home::commands::bind_space_file::BindSpaceFile { space_id: "s1".into(), file_path: "/tmp/x.os".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(crate::apps::home::commands::import_space::ImportSpace { dsl: Some("programs=[]".into()) }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(crate::apps::home::commands::import_space::ImportSpace { dsl: None }));
        store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::OpenSpace(crate::apps::home::commands::open_space::OpenSpace { space_id: "s1".into() }));
    }

    #[test]
    fn creates_studio_via_home_action() {
        let port = crate::apps::home::catalog_port();
        let before = list_os_space_catalog_entries(port.clone()).expect("list").len();
        let mut home = VcsArtifactApp::new(crate::apps::home::HomeApp::default());
        home.dispatch_typed(crate::apps::home::HomeCommand::CreateStudio(CreateStudio { name: "Test Studio".into(), kind: "catalog".into(), folder_path: None }), &testkit::meta("local")).expect("create");
        let after = list_os_space_catalog_entries(port).expect("list").len();
        assert!(after >= before);
    }

    #[test]
    fn temporary_studio_uses_ephemeral_registry_not_catalog() {
        let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let config = HomeConfig::default();
        let cfg = ConfigView { snapshot: &config };
        let emit = handle(&CreateStudio { name: "Temp Studio".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg).expect("handle");
        assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
        assert!(!emit.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })), "ephemeral create must not download");
        let persistent = list_os_space_catalog_entries(crate::apps::home::catalog_port()).expect("list");
        assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
        let ephemeral_catalog = list_os_space_catalog_entries(crate::apps::home::temp_catalog_port()).unwrap_or_default();
        assert!(!ephemeral_catalog.iter().any(|entry| entry.name == "Temp Studio"));
        let uri = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.as_str()),
                _ => None,
            })
            .expect("navigate");
        let space_id = uri.trim_start_matches("/spaces/");
        let document = crate::apps::home::resolve_studio_document(space_id).expect("ephemeral studio");
        assert_eq!(document.name, "Temp Studio");
        assert!(document.backbone.is_none());
        assert!(document.vcs.initial_snapshot.collections.is_empty());
    }
}
//#endregion 🧪️Tests
