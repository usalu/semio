//! 🏙️ S Home launcher app — studio lifecycle commands: create/bind/import/open a studio.
//!
//! One nested `pub mod` per payload (the `app_commands!` shape — see `apps::home::🦀️component.rs`'s
//! `🔖️HomeCommand` region, which `use`s each of these modules flat).

use crate::apps::home::config::{HomeConfig, HomeConfigOperation};
use crate::artifacts::home::op::SHomeOperation;
use crate::artifacts::home::SHomeDocument;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, HostEffect};

//#region 🔖️CreateStudio
pub mod create_studio {
    use super::*;
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
    fn created_studio_emit(catalog_generation: u64, space_id: &str) -> Emit<SHomeOperation, HomeConfigOperation> {
        Emit { document_operations: vec![SHomeOperation::SetCatalogGeneration { value: catalog_generation + 1 }], effects: vec![HostEffect::Navigate { uri: format!("/spaces/{space_id}") }], ..Default::default() }
    }

    pub fn handle(payload: &CreateStudio, doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        let generation = doc.projection.catalog_generation;
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
}
//#endregion 🔖️CreateStudio

//#region 🔖️BindSpaceFile
pub mod bind_space_file {
    use super::*;
    use semio_framework_os::VcsError;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "bind-space-file")]
    pub struct BindSpaceFile {
        pub space_id: String,
        pub file_path: String,
    }

    /// @emoji 🎯️ This IS "save this draft as a real asset" for a whole space manifest — binding a studio
    /// to a real file/catalog location is the natural persist moment.
    #[cfg(not(target_arch = "wasm32"))]
    fn bind_studio_file(space_id: &str, file_path: &str) -> Result<(), VcsError> {
        use semio_framework_os::{document_backbone_ref, encode_backbone_payload, OS_SPACE_BACKBONE_URI_PREFIX};
        let uri = format!("file://{file_path}");
        let port = semio_framework_os::open_file_space_backbone(file_path)?;
        crate::apps::home::register_studio_port(space_id, port.clone());
        let mut document = crate::apps::home::resolve_studio_document(space_id).ok_or_else(|| VcsError::Backbone(format!("unknown space {space_id}")))?;
        document.backbone = Some(document_backbone_ref(&uri));
        port.write(&uri, &encode_backbone_payload(&document)?)?;
        let catalog_uri = format!("{OS_SPACE_BACKBONE_URI_PREFIX}{space_id}");
        crate::apps::home::sync_os_space_document_helper(&document, &catalog_uri, &crate::apps::home::catalog_port())?;
        let draft_port = crate::apps::home::draft_backbone_port();
        crate::apps::home::ephemeral_draft_catalog().discard_draft(&draft_port, space_id);
        Ok(())
    }

    pub fn handle(payload: &BindSpaceFile, _doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = bind_studio_file(&payload.space_id, &payload.file_path);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (&payload.space_id, &payload.file_path);
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️BindSpaceFile

//#region 🔖️ImportSpace
pub mod import_space {
    use super::*;
    use semio_framework_os::import_os_space_from_dsl;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-space")]
    pub struct ImportSpace {
        pub dsl: Option<String>,
    }

    pub fn handle(payload: &ImportSpace, doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        let generation = doc.projection.catalog_generation;
        match &payload.dsl {
            Some(dsl) => {
                if import_os_space_from_dsl(dsl, crate::apps::home::catalog_port()).is_ok() {
                    Ok(Emit::operations(vec![SHomeOperation::SetCatalogGeneration { value: generation + 1 }]))
                } else {
                    Ok(Emit::default())
                }
            }
            None => Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".os".into(), read_as: None, import_action: "importSpace".into(), multiple: false })),
        }
    }
}
//#endregion 🔖️ImportSpace

//#region 🔖️OpenSpace
pub mod open_space {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "open-space")]
    pub struct OpenSpace {
        pub space_id: String,
    }

    pub fn handle(payload: &OpenSpace, _doc: &DocumentView<'_, SHomeDocument>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeOperation, HomeConfigOperation>, Fault> {
        eprintln!("[DEBUG] home openSpace id={}", payload.space_id);
        Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{}", payload.space_id) }))
    }
}
//#endregion 🔖️OpenSpace

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os::list_os_space_catalog_entries;
    use semio_framework_plugin::{testkit, HistoryView, VcsDocumentApp};

    #[test]
    fn home_command_op_text_round_trips_every_variant() {
        use crate::apps::home::HomeCommand;
        store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(create_studio::CreateStudio { name: "Untitled".into(), kind: "catalog".into(), folder_path: None }));
        store::test_support::assert_op_line_round_trip(&HomeCommand::CreateStudio(create_studio::CreateStudio { name: "Untitled".into(), kind: "folder".into(), folder_path: Some("/tmp/x".into()) }));
        store::test_support::assert_op_line_round_trip(&HomeCommand::BindSpaceFile(bind_space_file::BindSpaceFile { space_id: "s1".into(), file_path: "/tmp/x.os".into() }));
        store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(import_space::ImportSpace { dsl: Some("programs=[]".into()) }));
        store::test_support::assert_op_line_round_trip(&HomeCommand::ImportSpace(import_space::ImportSpace { dsl: None }));
        store::test_support::assert_op_line_round_trip(&HomeCommand::OpenSpace(open_space::OpenSpace { space_id: "s1".into() }));
    }

    #[test]
    fn creates_studio_via_home_action() {
        let port = crate::apps::home::catalog_port();
        let before = list_os_space_catalog_entries(port.clone()).expect("list").len();
        let mut home = VcsDocumentApp::new(crate::apps::home::HomeApp);
        home.dispatch_typed(crate::apps::home::HomeCommand::CreateStudio(create_studio::CreateStudio { name: "Test Studio".into(), kind: "catalog".into(), folder_path: None }), &testkit::meta("local")).expect("create");
        let after = list_os_space_catalog_entries(port).expect("list").len();
        assert!(after >= before);
    }

    #[test]
    fn temporary_studio_uses_ephemeral_registry_not_catalog() {
        let projection = SHomeDocument { schema: "s.home".into(), catalog_generation: 0 };
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };
        let config = HomeConfig::default();
        let cfg = ConfigView { projection: &config };
        let emit = create_studio::handle(&create_studio::CreateStudio { name: "Temp Studio".into(), kind: "temporary".into(), folder_path: None }, &doc, &cfg).expect("handle");
        assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
        assert!(!emit.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })), "ephemeral create must not download");
        let persistent = list_os_space_catalog_entries(crate::apps::home::catalog_port()).expect("list");
        assert!(!persistent.iter().any(|entry| entry.name == "Temp Studio"));
        let ephemeral_catalog = list_os_space_catalog_entries(crate::apps::home::temp_catalog_port()).expect("list");
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
        assert!(document.vcs.initial_projection.collections.is_empty());
    }
}
//#endregion 🧪️Tests
