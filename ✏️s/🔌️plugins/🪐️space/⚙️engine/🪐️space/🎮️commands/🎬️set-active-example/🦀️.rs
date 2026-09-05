//! 💾️ 💾️ S Studio app command — `set-active-example`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};

use semio_framework_os::{WorkflowMutation, WorkflowSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    if payload.example_id.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::effect(Effect::Navigate { uri: format!("/spaces/{}", payload.example_id) }))
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::space::testkit::studio_emit;
    use crate::engine::space::SpaceCommand;
    use semio_framework_os::empty_workflow_snapshot;

    #[semio_framework_async_macros::async_test]
    async fn space_command_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::SetActiveExample(SetActiveExample { example_id: "demo".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportStudioPack(crate::engine::space::commands::export_studio_pack::ExportStudioPack {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ExportStudioDsl(crate::engine::space::commands::export_studio_dsl::ExportStudioDsl {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportSpacePack(crate::engine::space::commands::import_space_pack::ImportSpacePack {}));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::ImportSpacePackPayload(crate::engine::space::commands::import_space_pack_payload::ImportSpacePackPayload { payload: "data:...".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&SpaceCommand::OpenSpace(crate::engine::space::commands::open_space::OpenSpace { space_id: "demo".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn open_studio_loads_created_empty_catalog_studio() {
        use semio_framework_os::{create_os_space, MemoryBackbonePort, SpaceKind, SpaceRole, SpaceUser, SpaceVisibility};
        use std::sync::Arc;
        // 🧬️ O1 — concrete `store::BackbonePorts`, not `dyn OsBackbonePort`: `create_os_space` and
        // `register_studio_port_for_test` both take `Arc<dyn OsBackbonePort>` BY VALUE, so
        // `Arc<BackbonePorts>` unsizes at each call site.
        let port: Arc<store::BackbonePorts> = Arc::new(store::BackbonePorts::Memory(MemoryBackbonePort::default()));
        let owner = SpaceUser { id: "tester".into(), name: "Tester".into(), avatar: None, role: SpaceRole::Author };
        let entry = create_os_space("Opened Empty", SpaceKind::Atelier, SpaceVisibility::Private, owner, port.clone()).expect("create");
        crate::register_studio_port_for_test(&entry.id, port);
        let empty = empty_workflow_snapshot();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(crate::engine::space::commands::open_space::OpenSpace { space_id: entry.id.clone() })).expect("handle");
        assert!(emit.config_mutations.contains(&SpaceConfigMutation::SetSpaceId { space_id: Some(entry.id) }));
        assert!(emit.config_mutations.contains(&SpaceConfigMutation::SetActiveNode { node_id: None }));
        assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })));
        assert!(!emit.effects.iter().any(|effect| matches!(effect, Effect::Navigate { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn open_studio_unknown_id_returns_not_found() {
        let empty = empty_workflow_snapshot();
        let config = SpaceConfig::default();
        let err = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(crate::engine::space::commands::open_space::OpenSpace { space_id: "unknown-studio-id".into() })).err().expect("not found");
        assert_eq!(err.code.0, "s.space.not-found");
    }

    async fn load_document_snapshot(emit: &Emit<WorkflowMutation, SpaceConfigMutation>) -> (WorkflowSnapshot, String) {
        let (pack, spr) = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::LoadDocument { pack, spr } => Some((pack.as_slice(), spr.as_slice())),
                _ => None,
            })
            .expect("load document");
        let parsed: store::ParsedDocumentText<WorkflowSnapshot, WorkflowMutation> = store::parse_document_pack(pack, spr).expect("parse document pack");
        let id = parsed.envelope.id.clone();
        (parsed.snapshot, id)
    }

    #[semio_framework_async_macros::async_test]
    async fn open_studio_demo_explicit_loads_demo_fixture() {
        let empty = empty_workflow_snapshot();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(crate::engine::space::commands::open_space::OpenSpace { space_id: "demo".into() })).expect("handle");
        let (projection, id) = load_document_snapshot(&emit);
        assert!(id.contains("demo-studio"));
        assert!(!projection.graph.nodes.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn open_studio_loads_ephemeral_created_studio() {
        use crate::editor::home::commands::create_studio;
        use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, HistoryView};
        let _home = crate::editor::home::HomeApp;
        let home_projection = crate::editor::home::HomeApp::initial_snapshot();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&home_projection, &history);
        let home_config = crate::editor::home::config::HomeConfig::default();
        let home_cfg = ConfigView { snapshot: &home_config };
        let create = create_studio::handle(&create_studio::CreateStudio { name: "Ephemeral Open".into(), kind: "catalog".into(), folder_path: None }, &doc, &home_cfg).expect("handle");
        let space_id = create
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::Navigate { uri } => Some(uri.trim_start_matches("/spaces/").to_string()),
                _ => None,
            })
            .expect("navigate");
        let empty = empty_workflow_snapshot();
        let config = SpaceConfig::default();
        let emit = studio_emit(&empty, &config, &SpaceCommand::OpenSpace(crate::engine::space::commands::open_space::OpenSpace { space_id: space_id.clone() })).expect("handle");
        let (projection, id) = load_document_snapshot(&emit);
        assert_eq!(id, space_id);
        assert!(projection.graph.nodes.is_empty());
    }

    /// 🌉️ Exercises BOTH apps together (Home's `createStudio` followed by Space's `openSpace`).
    #[semio_framework_async_macros::async_test]
    async fn create_space_navigates_without_download_and_opens_empty() {
        use crate::editor::home::commands::create_studio;
        use semio_framework_plugin::{ArtifactEditor, ArtifactView, ConfigView, HistoryView};
        let _home = crate::editor::home::HomeApp;
        let home_projection = crate::editor::home::HomeApp::initial_snapshot();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&home_projection, &history);
        let home_config = crate::editor::home::config::HomeConfig::default();
        let home_cfg = ConfigView { snapshot: &home_config };
        let emit = create_studio::handle(&create_studio::CreateStudio { name: "Fresh Studio".into(), kind: "catalog".into(), folder_path: None }, &doc, &home_cfg).expect("handle");
        assert!(!emit.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })), "create must not download a file");
        let uri = emit
            .effects
            .iter()
            .find_map(|effect| match effect {
                Effect::Navigate { uri } => Some(uri.as_str()),
                _ => None,
            })
            .expect("navigate");
        assert!(uri.starts_with("/spaces/"), "uri={uri}");
        assert!(!uri.ends_with("/demo") && !uri.ends_with("/default"), "uri={uri}");
        let space_id = uri.trim_start_matches("/spaces/");
        let document = crate::resolve_studio_document(space_id).expect("created studio");
        assert_eq!(document.name, "Fresh Studio");
        assert!(document.backbone.is_none(), "ephemeral studio must not attach backbone");
        assert!(document.vcs.initial_snapshot.collections.is_empty());

        let empty = empty_workflow_snapshot();
        let studio_doc = ArtifactView::new(&empty, &history);
        let studio_config = SpaceConfig::default();
        let studio_cfg = ConfigView { snapshot: &studio_config };
        // 🕹️ `OpenSpace` isn't one of `SpaceApp::handle`'s interaction-aware bypass rows, so it's safe
        // to exercise via `SpaceCommand::dispatch` directly (the `app_commands!`-generated 3-arg path
        // — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) rather than through
        // `SpaceApp::handle`, which now needs a real `InteractionView` only a full `VcsArtifactApp`
        // dispatch can provide.
        let open = SpaceCommand::OpenSpace(crate::engine::space::commands::open_space::OpenSpace { space_id: space_id.to_string() }).dispatch(&studio_doc, &studio_cfg).expect("handle");
        assert!(open.effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. })), "openSpace must load the created studio");
        assert!(!open.effects.iter().any(|effect| matches!(effect, Effect::Navigate { .. })));
        assert!(!open.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. })));
    }
}
//#endregion 🧪️Tests
