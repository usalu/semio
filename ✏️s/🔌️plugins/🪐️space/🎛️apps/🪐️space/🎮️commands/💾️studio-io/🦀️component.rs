//! 💾️ S Studio app — whole-studio export/import + open commands.

use crate::apps::space::config::{SpaceConfig, SpaceConfigOperation};
use semio_framework_os::host::{export_os_space_dsl, export_os_space_pack, import_os_space_from_pack};
use semio_framework_os::{create_backbone_document, WorkflowDocument, WorkflowOperation, OS_SPACE_SCHEMA, S_SPACE_SCHEMA};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault, FaultCode, FaultOrigin, HostEffect};

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        if payload.example_id.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::effect(HostEffect::Navigate { uri: format!("/spaces/{}", payload.example_id) }))
        }
    }
}
//#endregion 🔖️SetActiveExample

//#region 🔖️ExportStudioPack
pub mod export_studio_pack {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-studio-pack")]
    pub struct ExportStudioPack {}

    pub fn handle(_payload: &ExportStudioPack, _doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        let space_id = crate::apps::space::config_space_id(cfg.projection);
        match crate::apps::home::resolve_studio_document(&space_id) {
            Some(document) => match export_os_space_pack(&document) {
                Ok(pack_files) => {
                    use base64::Engine;
                    Ok(Emit {
                        effects: vec![
                            HostEffect::DownloadMediaExport { filename: format!("{space_id}.pack"), mime_type: "application/octet-stream".into(), data: base64::engine::general_purpose::STANDARD.encode(&pack_files.pack), encoding: Some("base64".into()) },
                            HostEffect::DownloadMediaExport { filename: format!("{space_id}.ops"), mime_type: "text/plain".into(), data: pack_files.ops, encoding: None },
                        ],
                        ..Default::default()
                    })
                }
                Err(_) => Ok(Emit::default()),
            },
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportStudioPack

//#region 🔖️ExportStudioDsl
pub mod export_studio_dsl {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "export-studio-dsl")]
    pub struct ExportStudioDsl {}

    pub fn handle(_payload: &ExportStudioDsl, _doc: &DocumentView<'_, WorkflowDocument>, cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        let space_id = crate::apps::space::config_space_id(cfg.projection);
        match crate::apps::home::resolve_studio_document(&space_id) {
            Some(document) => match export_os_space_dsl(&document) {
                Ok(text_files) => Ok(Emit::effect(HostEffect::DownloadMediaExport { filename: format!("{space_id}.os"), mime_type: "text/plain".into(), data: text_files.dsl, encoding: None })),
                Err(_) => Ok(Emit::default()),
            },
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️ExportStudioDsl

//#region 🔖️ImportSpacePack
pub mod import_space_pack {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-space-pack")]
    pub struct ImportSpacePack {}

    pub fn handle(_payload: &ImportSpacePack, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        Ok(Emit::effect(HostEffect::RequestFileOpen { accept: ".pack".into(), read_as: Some("dataUrl".into()), import_action: "importSpacePackPayload".into(), multiple: false }))
    }
}
//#endregion 🔖️ImportSpacePack

//#region 🔖️ImportSpacePackPayload
pub mod import_space_pack_payload {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "import-space-pack-payload")]
    pub struct ImportSpacePackPayload {
        pub payload: String,
    }

    pub fn handle(payload: &ImportSpacePackPayload, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        use base64::Engine;
        let base64_part = payload.payload.split_once(',').map_or(payload.payload.as_str(), |(_, data)| data);
        if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_part) {
            // 🌱️ A single `.pack` file carries no separate `.spr` sidecar (unlike `exportStudioPack`'s
            // two-file output) — `store::empty_document_spr` builds a bare, edit-free op log so the
            // pack+spr-first codec path still decodes to a document with no replayed edit history, i.e.
            // its bare initial projection.
            let empty_spr = store::empty_document_spr("", OS_SPACE_SCHEMA);
            let _ = import_os_space_from_pack(&bytes, &empty_spr, crate::apps::home::catalog_port());
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️ImportSpacePackPayload

//#region 🔖️OpenSpace
pub mod open_space {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "open-space")]
    pub struct OpenSpace {
        pub space_id: String,
    }

    pub fn handle(payload: &OpenSpace, _doc: &DocumentView<'_, WorkflowDocument>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowOperation, SpaceConfigOperation>, Fault> {
        let space_id = &payload.space_id;
        // 🚧️ `parse_demo_space_document()` yields a `workflow::WorkflowDocument`, not a
        // `space::SpaceProjection`-backed catalog entry — the "demo" id fallback below synthesizes a
        // minimal ephemeral space manifest with the same id/name instead, so `"demo"` still resolves to
        // *something* openable.
        let document = crate::apps::home::resolve_studio_document(space_id).or_else(|| {
            if space_id == "demo" {
                let name = { let demo = crate::parse_demo_space_document(); if demo.name.trim().is_empty() { "Demo Studio".into() } else { demo.name } };
                let projection = semio_framework_os::empty_space_projection(&name, semio_framework_os::SpaceKind::Atelier, semio_framework_os::SpaceVisibility::Private);
                Some(create_backbone_document(S_SPACE_SCHEMA, "demo", &name, projection))
            } else {
                None
            }
        });
        let Some(document) = document else {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.not-found"), format!("studio `{space_id}` not found")));
        };
        let mut config_operations = vec![
            SpaceConfigOperation::SetSpaceId { space_id: Some(space_id.clone()) },
            SpaceConfigOperation::SetFocusedNode { node_id: None },
            SpaceConfigOperation::SetSelection { node_ids: Vec::new() },
            SpaceConfigOperation::SetClipboard { node_ids: Vec::new() },
        ];
        // 🕸️ `document` is a `space::SpaceProjection`-backed manifest — it carries no workflow graph of
        // its own anymore; the graph lives on a separate `s.workflow` artifact document within one of
        // the space's collections. Resolve, in order: (1) a real workflow artifact already registered
        // in one of `document`'s collections, (2) the bundled demo fixture's real content for the demo
        // space, (3) a freshly-minted, valid, empty `WorkflowDocument` for any other space that has none
        // yet — never the space manifest's own bytes.
        let is_demo_space = space_id == "demo" || document.name == crate::DEMO_STUDIO_NAME;
        let workflow_document = crate::apps::home::resolve_workflow_artifact_document(space_id, &document)
            .or_else(|| is_demo_space.then(crate::parse_demo_space_document))
            .unwrap_or_else(|| crate::apps::home::empty_workflow_artifact_document(space_id, &document.name));
        let active_node_id = workflow_document.vcs.initial_projection.graph.nodes.first().map(|node| node.id.clone());
        config_operations.push(SpaceConfigOperation::SetActiveNode { node_id: active_node_id });
        match crate::apps::home::workflow_artifact_envelope_pack(&workflow_document) {
            Some(files) => {
                eprintln!(
                    "[DEBUG] openSpace id={} workflow_id={} nodes={} collections={}",
                    space_id,
                    workflow_document.id,
                    workflow_document.vcs.initial_projection.graph.nodes.len(),
                    document.vcs.initial_projection.collections.len()
                );
                Ok(Emit { config_operations, effects: vec![HostEffect::LoadDocument { pack: files.pack, spr: files.spr }], ..Default::default() })
            }
            None => {
                eprintln!("[DEBUG] openSpace workflow pack export failed id={space_id}");
                Ok(Emit::config(config_operations))
            }
        }
    }
}
//#endregion 🔖️OpenSpace

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::space::testkit::studio_emit;
    use crate::apps::space::SpaceCommand;
    use semio_framework_os::empty_workflow_document;

    #[test]
    fn space_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SpaceCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportStudioPack(export_studio_pack::ExportStudioPack {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportStudioDsl(export_studio_dsl::ExportStudioDsl {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportSpacePack(import_space_pack::ImportSpacePack {}));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportSpacePackPayload(import_space_pack_payload::ImportSpacePackPayload { payload: "data:...".into() }));
        store::test_support::assert_op_line_round_trip(&SpaceCommand::OpenSpace(open_space::OpenSpace { space_id: "demo".into() }));
    }

    #[test]
    fn open_studio_loads_created_empty_catalog_studio() {
        use semio_framework_os::{create_os_space, MemoryBackbonePort, SpaceKind, SpaceRole, SpaceUser, SpaceVisibility};
        use std::sync::Arc;
        let port: Arc<dyn semio_framework_os::OsBackbonePort> = Arc::new(MemoryBackbonePort::new());
        let owner = SpaceUser { id: "tester".into(), name: "Tester".into(), avatar: None, role: SpaceRole::Author };
        let entry = create_os_space("Opened Empty", SpaceKind::Atelier, SpaceVisibility::Private, owner, port.clone()).expect("create");
        crate::apps::home::register_studio_port_for_test(&entry.id, port);
        let empty = empty_workflow_document();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(open_space::OpenSpace { space_id: entry.id.clone() })).expect("handle");
        assert!(emit.config_operations.contains(&SpaceConfigOperation::SetSpaceId { space_id: Some(entry.id) }));
        assert!(emit.config_operations.contains(&SpaceConfigOperation::SetActiveNode { node_id: None }));
        assert!(emit.effects.iter().any(|effect| matches!(effect, HostEffect::LoadDocument { .. })));
        assert!(!emit.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
    }

    #[test]
    fn open_studio_unknown_id_returns_not_found() {
        let empty = empty_workflow_document();
        let config = SpaceConfig::default();
        let err = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(open_space::OpenSpace { space_id: "unknown-studio-id".into() })).err().expect("not found");
        assert_eq!(err.code.0, "s.space.not-found");
    }

    fn load_document_projection(emit: &Emit<WorkflowOperation, SpaceConfigOperation>) -> (WorkflowDocument, String) {
        let (pack, spr) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::LoadDocument { pack, spr } => Some((pack.as_slice(), spr.as_slice())),
                _ => None,
            })
            .expect("load document");
        let parsed: store::ParsedDocumentText<WorkflowDocument, WorkflowOperation> = store::parse_document_pack(pack, spr).expect("parse document pack");
        let id = parsed.envelope.id.clone();
        (parsed.projection, id)
    }

    #[test]
    fn open_studio_demo_explicit_loads_demo_fixture() {
        let empty = empty_workflow_document();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(open_space::OpenSpace { space_id: "demo".into() })).expect("handle");
        let (projection, id) = load_document_projection(&emit);
        assert!(id.contains("demo-studio"));
        assert!(!projection.graph.nodes.is_empty());
    }

    #[test]
    fn open_studio_loads_ephemeral_created_studio() {
        use crate::apps::home::commands::studio::create_studio;
        use semio_framework_plugin::{ConfigView, DocumentApp, DocumentView, HistoryView};
        let home = crate::apps::home::HomeApp;
        let home_projection = home.initial_projection();
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &home_projection, history: &history };
        let home_config = crate::apps::home::config::HomeConfig::default();
        let home_cfg = ConfigView { projection: &home_config };
        let create = create_studio::handle(&create_studio::CreateStudio { name: "Ephemeral Open".into(), kind: "catalog".into(), folder_path: None }, &doc, &home_cfg).expect("handle");
        let space_id = create
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.trim_start_matches("/spaces/").to_string()),
                _ => None,
            })
            .expect("navigate");
        let empty = empty_workflow_document();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(open_space::OpenSpace { space_id: space_id.clone() })).expect("handle");
        let (projection, id) = load_document_projection(&emit);
        assert_eq!(id, space_id);
        assert!(projection.graph.nodes.is_empty());
    }

    /// 🌉️ Exercises BOTH apps together (Home's `createStudio` followed by Space's `openSpace`).
    #[test]
    fn create_space_navigates_without_download_and_opens_empty() {
        use crate::apps::home::commands::studio::create_studio;
        use semio_framework_plugin::{ConfigView, DocumentApp, DocumentView, HistoryView};
        let home = crate::apps::home::HomeApp;
        let home_projection = home.initial_projection();
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &home_projection, history: &history };
        let home_config = crate::apps::home::config::HomeConfig::default();
        let home_cfg = ConfigView { projection: &home_config };
        let emit = create_studio::handle(&create_studio::CreateStudio { name: "Fresh Studio".into(), kind: "catalog".into(), folder_path: None }, &doc, &home_cfg).expect("handle");
        assert!(!emit.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })), "create must not download a file");
        let uri = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                HostEffect::Navigate { uri } => Some(uri.as_str()),
                _ => None,
            })
            .expect("navigate");
        assert!(uri.starts_with("/spaces/"), "uri={uri}");
        assert!(!uri.ends_with("/demo") && !uri.ends_with("/default"), "uri={uri}");
        let space_id = uri.trim_start_matches("/spaces/");
        let document = crate::apps::home::resolve_studio_document(space_id).expect("created studio");
        assert_eq!(document.name, "Fresh Studio");
        assert!(document.backbone.is_none(), "ephemeral studio must not attach backbone");
        assert!(document.vcs.initial_projection.collections.is_empty());

        let empty = empty_workflow_document();
        let studio_doc = DocumentView { projection: &empty, history: &history };
        let studio_config = SpaceConfig::default();
        let studio_cfg = ConfigView { projection: &studio_config };
        let studio = crate::apps::space::SpaceApp;
        let open = studio.handle(&SpaceCommand::OpenSpace(open_space::OpenSpace { space_id: space_id.to_string() }), &studio_doc, &studio_cfg).expect("handle");
        assert!(open.effects.iter().any(|effect| matches!(effect, HostEffect::LoadDocument { .. })), "openSpace must load the created studio");
        assert!(!open.effects.iter().any(|effect| matches!(effect, HostEffect::Navigate { .. })));
        assert!(!open.effects.iter().any(|effect| matches!(effect, HostEffect::DownloadMediaExport { .. })));
    }
}
//#endregion 🧪️Tests
