//! 🏠️ S Home launcher editor — `ArtifactEditor` impl, command dispatch, manifest (constitutional: ui).
//!
//! WIRING + DISPATCH ONLY: every command's real body lives in its own `🎮️commands/<group>/🦀️component.rs`
//! payload module (see `app_commands!` below). The catalog/draft/backbone document-helper functions this
//! file used to hold (`catalog_port`, `resolve_studio_document`, `list_all_space_catalog_entries`, …)
//! moved to the PLUGIN ROOT `🦀️component.rs` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
//! W2 packet P7) — they are genuinely shared by 3 surfaces now (this editor, the new `👁️viewer`, and the
//! sibling `🪐️space` studio app's own commands), and a viewer file can never import through `::editor::`
//! (`policyViewerPurityBreaches`), so the shared code cannot live here anymore. Reach it as `crate::X`
//! from any module in this crate.

use crate::editor::home::commands::set_active_panel_tab;
use crate::editor::home::commands::{bind_space_file, create_studio, import_space, open_space};
use crate::editor::home::commands::{delete_virtual_file_system_node, go_home, navigate_virtual_file_system_node};
use crate::editor::home::commands::{copy_invite_link, create_space, delete_space, fold_directory_events, presence_heartbeat, rename_space, set_client, share_space};
use crate::editor::home::config::HomeConfig;
use crate::editor::home::presence::{HomePresence, HomePresenceMutation};
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, app_commands, create_tab_stack_layout, ConfigView, ArtifactEditor, ArtifactView, Editor, Emit, Fault, FaultOrigin, Label, LocalizedLabel, UiNode};
use semio_framework_plugin::{ActionArgDef, ActionArgOption, ActionRef, DialogDefinition};
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;
use serde_json::Value;

//#region 🔖️Constants
pub const S_HOME_CONTROLLER_ID: &str = "s-home";
//#endregion 🔖️Constants

//#region 🔖️HomeCommand
app_commands! {
    /// 🎯️ `HomeApp::Command` — the SOLE dispatch surface for the Home launcher's own behavior, one
    /// variant per action declared in `create_home_app`'s manifest.
    pub enum HomeCommand for SHomeSnapshot, crate::artifacts::home::op::SHomeMutation, HomeConfig, crate::editor::home::config::HomeConfigMutation {
        "createStudio" as "create-studio" => create_studio::CreateStudio,
        "bindSpaceFile" as "bind-space-file" => bind_space_file::BindSpaceFile,
        "importSpace" as "import-space" => import_space::ImportSpace,
        "openSpace" as "open-space" => open_space::OpenSpace,
        "navigateVirtualFileSystemNode" as "navigate-vfs-node" => navigate_virtual_file_system_node::NavigateVirtualFileSystemNode,
        "deleteVirtualFileSystemNode" as "delete-vfs-node" => delete_virtual_file_system_node::DeleteVirtualFileSystemNode,
        "goHome" as "go-home" => go_home::GoHome,
        "setActivePanelTab" as "active-panel-tab" => set_active_panel_tab::SetActivePanelTab,
        // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: Home = a real table of
        // every space, fed by the event-sourced hub directory read model (contract §C1/§C6).
        "createSpace" as "create-space" => create_space::CreateSpace,
        "deleteSpace" as "delete-space" => delete_space::DeleteSpace,
        "renameSpace" as "rename-space" => rename_space::RenameSpace,
        "shareSpace" as "share-space" => share_space::ShareSpace,
        "copyInviteLink" as "copy-invite-link" => copy_invite_link::CopyInviteLink,
        "foldDirectoryEvents" as "fold-directory-events" => fold_directory_events::FoldDirectoryEvents,
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
        "setClient" as "set-client" => set_client::SetClient,
    }
}
//#endregion 🔖️HomeCommand

//#region 🔖️HomeApp
/// 🧪️ Unit struct — the Home launcher holds catalog bootstrap ports plus per-session studio port
/// bindings for folder/file-backed studios.
#[derive(Default, Clone, Copy)]
pub struct HomeApp;

impl ArtifactEditor for HomeApp {
    type Snapshot = SHomeSnapshot;
    type Mutation = crate::artifacts::home::op::SHomeMutation;
    type Config = HomeConfig;
    type ConfigMutation = crate::editor::home::config::HomeConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = HomePresence;
    type PresenceMutation = HomePresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = HomeCommand;

    const DIALECT: Dialect = crate::artifacts::home::HOME_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::home::S_HOME_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SHomeSnapshot {
        SHomeSnapshot::default()
    }

    fn command_id(command: &HomeCommand) -> &'static str {
        command.command_id()
    }

    /// 🪪️ `s.space.home`'s config+presence schema descriptor (ticket
    /// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1c) — `register_document_app` registers it the
    /// moment this type is bound to the plugin, completing the app-schema declaration for `🪐️space`.
    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::home::config::schema::app_schema_descriptor())
    }

    /// 🎯️ Bridges shell `{action,args}` JSON onto typed `HomeCommand` until every call site speaks OpBinary.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<HomeCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "createStudio" => Ok(HomeCommand::CreateStudio(create_studio::CreateStudio {
                name: str_field("name").unwrap_or_else(|| "Untitled".into()),
                kind: str_field("kind").unwrap_or_else(|| "catalog".into()),
                folder_path: str_field("folderPath").or_else(|| str_field("folder_path")),
            })),
            "bindSpaceFile" => Ok(HomeCommand::BindSpaceFile(bind_space_file::BindSpaceFile {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                file_path: str_field("filePath").or_else(|| str_field("file_path")).unwrap_or_default(),
            })),
            "importSpace" => Ok(HomeCommand::ImportSpace(import_space::ImportSpace { dsl: str_field("dsl").or_else(|| str_field("payload")) })),
            "openSpace" => Ok(HomeCommand::OpenSpace(open_space::OpenSpace { space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default() })),
            "navigateVirtualFileSystemNode" => Ok(HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode {
                node_id: str_field("nodeId")
                    .or_else(|| str_field("node_id"))
                    .or_else(|| str_field("spaceId"))
                    .or_else(|| str_field("space_id"))
                    .unwrap_or_default(),
            })),
            "deleteVirtualFileSystemNode" => Ok(HomeCommand::DeleteVirtualFileSystemNode(delete_virtual_file_system_node::DeleteVirtualFileSystemNode {
                node_id: str_field("nodeId")
                    .or_else(|| str_field("node_id"))
                    .or_else(|| str_field("spaceId").map(|id| format!("studio:{id}")))
                    .or_else(|| str_field("space_id").map(|id| format!("studio:{id}")))
                    .unwrap_or_default(),
            })),
            "goHome" => Ok(HomeCommand::GoHome(go_home::GoHome {})),
            "setActivePanelTab" => Ok(HomeCommand::SetActivePanelTab(set_active_panel_tab::SetActivePanelTab { tab_id: str_field("tabId").or_else(|| str_field("tab_id")).unwrap_or_default() })),
            "createSpace" => Ok(HomeCommand::CreateSpace(create_space::CreateSpace {
                name: str_field("name").unwrap_or_default(),
                kind: str_field("kind").or_else(|| str_field("spaceKind")).unwrap_or_default(),
                visibility: str_field("visibility").unwrap_or_default(),
            })),
            "deleteSpace" => Ok(HomeCommand::DeleteSpace(delete_space::DeleteSpace {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                confirmed: args.and_then(|value| value.get("confirmed")).and_then(Value::as_bool).unwrap_or(false),
            })),
            "renameSpace" => Ok(HomeCommand::RenameSpace(rename_space::RenameSpace {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                name: str_field("name").unwrap_or_default(),
            })),
            "shareSpace" => Ok(HomeCommand::ShareSpace(share_space::ShareSpace {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                email: str_field("email").unwrap_or_default(),
                role: str_field("role").unwrap_or_default(),
            })),
            "copyInviteLink" => Ok(HomeCommand::CopyInviteLink(copy_invite_link::CopyInviteLink {
                space_id: str_field("spaceId").or_else(|| str_field("space_id")).unwrap_or_default(),
                role: str_field("role").unwrap_or_default(),
                ttl_secs: args.and_then(|value| value.get("ttlSecs")).and_then(Value::as_u64).unwrap_or(0),
            })),
            "foldDirectoryEvents" => Ok(HomeCommand::FoldDirectoryEvents(fold_directory_events::FoldDirectoryEvents {
                events_json: args.and_then(|value| value.get("eventsJson")).and_then(Value::as_str).map(str::to_string).unwrap_or_else(|| "[]".into()),
            })),
            "presenceHeartbeat" => Ok(HomeCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat {})),
            "setClient" => Ok(HomeCommand::SetClient(set_client::SetClient {
                client_id: str_field("clientId").or_else(|| str_field("client_id")).unwrap_or_default(),
                client_name: str_field("clientName").or_else(|| str_field("client_name")).unwrap_or_default(),
            })),
            other => Err(Fault::new(FaultOrigin::App, "s.home.unhandled-action", format!("home: unhandled action id {other}"))),
        }
    }

    /// 🕹️ Home declares NO interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// its VFS rows (`🏠️main` window) render through `build_virtual_file_system_scene`, a
    /// `UiNode::ComponentScene` the framework's `stamp_and_cache_interaction_ui` post-pass never walks
    /// (that pass only stamps `UiNode::Tree`), and every row-scoped command (`navigateVirtualFileSystemNode`,
    /// `deleteVirtualFileSystemNode`) already takes an explicit `node_id` argument from the click event
    /// rather than reading a stored selection — there was no bespoke selection/hover config, mutation, or
    /// command here to delete. `_interaction` is accepted (trait-required) and unused.
    fn handle(command: &HomeCommand, doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<crate::artifacts::home::op::SHomeMutation, crate::editor::home::config::HomeConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, _doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> UiNode {
        // 🪟 `VcsArtifactApp::render` appends `:{windowInstanceId}` when `view_state.window_id` is set —
        // strip it so Home's single body key still matches.
        let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
        match base_body_key {
            crate::editor::home::modes::explore::windows::main::S_HOME_BODY => crate::editor::home::modes::explore::windows::main::render(cfg.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️HomeApp

//#region 🔖️HomeManifest
/// 🧱️ The manifest stitch: one call per taxonomy node. `.example(...)`/`.workflow(...)` do not exist on
/// `EditorBuilder` (contract §2.4, W0-F gap 4) — `create_home_app` never called either, so nothing is
/// dropped here (unlike other W2 packets that had to note a loss).
pub fn create_home_app() -> semio_framework_plugin::AppDefinition {
    let mut definition = Editor::builder(crate::artifacts::home::HOME_DIALECT)
        .document(["semio", "s", "home"])
        .icon_id("home")
        .mode_def(crate::editor::home::modes::explore::definition())
        .default_mode_id("explore")
        .window_kind_def(crate::editor::home::modes::explore::windows::main::definition())
        .default_layout(create_tab_stack_layout(&[crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW.into()], Some(&["Studios".into()])))
        .mutation("createStudio", LocalizedLabel::native("Create Studio", "Studio erstellen"))
        .shell_action("bindSpaceFile", LocalizedLabel::native("Bind Studio File", "Studio-Datei verknüpfen"))
        .mutation("importSpace", LocalizedLabel::native("Import Studio", "Studio importieren"))
        .shell_action("openSpace", LocalizedLabel::native("Open Studio", "Studio öffnen"))
        .shell_action("navigateVirtualFileSystemNode", LocalizedLabel::native("Navigate File System Node", "Dateisystemknoten navigieren"))
        .mutation("deleteVirtualFileSystemNode", LocalizedLabel::native("Delete File System Node", "Dateisystemknoten löschen"))
        .shell_action("goHome", LocalizedLabel::native("Go Home", "Zur Startseite"))
        .view_action("setActivePanelTab", LocalizedLabel::native("Set Active Panel Tab", "Aktiven Panel-Tab festlegen"))
        // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the overview table's
        // row-scoped actions. Every one of these is a pure `HostEffect` relay (contract §C6) — never a
        // document mutation — so each is `.shell_action`, matching `openSpace`/`goHome` above, not
        // `.mutation`. `createSpace`/`deleteSpace`/`renameSpace`/`shareSpace` are each their own dialog's
        // submit action too (`DialogDefinition::new(id, …, ActionRef::new(id))`, the same self-
        // referencing shape `PluginBuilder`'s own `declaring_dialog_appends_to_definition` test uses).
        .shell_action("createSpace", LocalizedLabel::native("Create Space", "Space erstellen"))
        .dialog(
            DialogDefinition::new("createSpace", LocalizedLabel::native("Create Space", "Space erstellen"), ActionRef::new("createSpace"))
                .args(vec![
                    ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                    ActionArgDef::select(
                        "kind",
                        LocalizedLabel::native("Kind", "Art"),
                        vec![ActionArgOption::new("atelier", LocalizedLabel::native("Atelier", "Atelier")), ActionArgOption::new("studio", LocalizedLabel::native("Studio", "Studio"))],
                    )
                    .default_value("atelier"),
                    ActionArgDef::select(
                        "visibility",
                        LocalizedLabel::native("Visibility", "Sichtbarkeit"),
                        vec![ActionArgOption::new("private", LocalizedLabel::native("Private", "Privat")), ActionArgOption::new("public", LocalizedLabel::native("Public", "Öffentlich"))],
                    )
                    .default_value("private"),
                ])
                .submit_label(LocalizedLabel::native("Create", "Erstellen")),
        )
        .shell_action("deleteSpace", LocalizedLabel::native("Delete Space", "Space löschen"))
        .dialog(
            DialogDefinition::new("deleteSpace", LocalizedLabel::native("Delete Space?", "Space löschen?"), ActionRef::new("deleteSpace"))
                .body(LocalizedLabel::native("This cannot be undone.", "Dies kann nicht rückgängig gemacht werden."))
                .submit_label(LocalizedLabel::native("Delete", "Löschen")),
        )
        .shell_action("renameSpace", LocalizedLabel::native("Rename Space", "Space umbenennen"))
        .dialog(
            DialogDefinition::new("renameSpace", LocalizedLabel::native("Rename Space", "Space umbenennen"), ActionRef::new("renameSpace"))
                .args(vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required()])
                .submit_label(LocalizedLabel::native("Rename", "Umbenennen")),
        )
        .shell_action("shareSpace", LocalizedLabel::native("Share Space", "Space teilen"))
        .dialog(
            DialogDefinition::new("shareSpace", LocalizedLabel::native("Share Space", "Space teilen"), ActionRef::new("shareSpace"))
                .args(vec![
                    ActionArgDef::text("email", LocalizedLabel::native("Email", "E-Mail")).required(),
                    ActionArgDef::select(
                        "role",
                        LocalizedLabel::native("Role", "Rolle"),
                        vec![ActionArgOption::new("author", LocalizedLabel::native("Author", "Autor")), ActionArgOption::new("spectator", LocalizedLabel::native("Spectator", "Betrachter"))],
                    )
                    .default_value("spectator"),
                ])
                .submit_label(LocalizedLabel::native("Share", "Teilen")),
        )
        .shell_action("copyInviteLink", LocalizedLabel::native("Copy Invite Link", "Einladungslink kopieren"))
        .view_action("foldDirectoryEvents", LocalizedLabel::native("Fold Directory Events", "Verzeichnisereignisse einspielen"))
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Präsenz-Heartbeat"))
        .view_action("setClient", LocalizedLabel::native("Set Client", "Client setzen"))
        .window_kind_action_refs(crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW, vec![
            "createStudio".into(),
            "bindSpaceFile".into(),
            "importSpace".into(),
            "openSpace".into(),
            "navigateVirtualFileSystemNode".into(),
            "deleteVirtualFileSystemNode".into(),
            "goHome".into(),
            "setActivePanelTab".into(),
            "createSpace".into(),
            "deleteSpace".into(),
            "renameSpace".into(),
            "shareSpace".into(),
            "copyInviteLink".into(),
        ])
        .keybinding("mod+n", "createStudio")
        .keybinding("mod+o", "importSpace")
        .build_definition();
    definition.controller_id = S_HOME_CONTROLLER_ID.into();
    definition
}
//#endregion 🔖️HomeManifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::EditorApp;

    pub type HomeEditorApp = semio_framework_plugin::VcsArtifactApp<EditorApp<HomeApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> HomeEditorApp {
        semio_framework_plugin::testkit::new_app::<EditorApp<HomeApp>>()
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use semio_framework_os::{
        create_backbone_document, empty_space_snapshot, load_os_space_document, seed_os_space_catalog_if_empty,
        LocalStorageBackbonePort, OsBackbonePort, OsSpaceDocument, SpaceKind, SpaceVisibility, S_SPACE_SCHEMA,
    };

    fn empty_history() -> semio_framework_plugin::HistoryView {
        semio_framework_plugin::HistoryView::empty()
    }

    #[test]
    fn home_manifest_derives_the_canonical_surface_id() {
        let definition = create_home_app();
        assert_eq!(definition.id, semio_framework::surface_app_id(&HomeApp::DIALECT.into(), semio_framework::AppRole::Editor));
        assert_eq!(definition.controller_id, "s-home");
    }

    #[test]
    fn home_declares_create_space_action() {
        let definition = create_home_app();
        let main = definition.window_kinds.iter().find(|window| window.id == crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW).expect("home main window");
        assert!(main.actions.iter().any(|action| action.id == "createStudio"));
    }

    #[test]
    fn space_document_persists_through_backbone_port() {
        // 🕳️ `parse_demo_space_document()` yields a `workflow::WorkflowSnapshot` (the demo fixture's own
        // artifact content), not a `space::SpaceSnapshot`-backed catalog entry
        // `seed_os_space_catalog_if_empty` expects. This test exercises the space-manifest persistence
        // path specifically, so it mints its own manifest instead.
        let port: Arc<dyn OsBackbonePort> = Arc::new(LocalStorageBackbonePort::new());
        let projection = empty_space_snapshot("Persist Test", SpaceKind::Atelier, SpaceVisibility::Private);
        let demo: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, "persist-test", "Persist Test", projection);
        let _ = seed_os_space_catalog_if_empty(demo, port.clone()).expect("seed");
        let loaded = load_os_space_document("persist-test", port.clone()).expect("load");
        assert_eq!(loaded.id, "persist-test");
        assert_eq!(loaded.name, "Persist Test");
    }

    /// 🧪️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the pre-ticket version of
    /// these two tests asserted on the VFS scene's ALWAYS-present `emptyMessage` field, which happened
    /// to make them incidentally immune to `crate::list_all_space_catalog_entries()`'s process-global
    /// catalog singleton being polluted by other tests in this same test binary. The new table render
    /// has no such structural field (`TableView` carries no message), so these are rewritten to fold a
    /// KNOWN directory event (deterministic, independent of the global catalog) and assert on the
    /// locale-correct COLUMN HEADERS instead — the real thing "labels resolve to the right locale" means
    /// for a table.
    fn config_with_one_folded_space(locale: &str) -> HomeConfig {
        let event_json = serde_json::json!({
            "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
            "body": {"kind": "space.created", "spaceId": "sp-1", "name": "Fixture", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
            "recordedAtMs": 1000
        })
        .to_string();
        let base = HomeConfig { locale: locale.into(), ..HomeConfig::default() };
        protocol::Mutation::diff(&crate::editor::home::config::HomeConfigMutation::FoldDirectoryEvent { event_json }, &base).diff().clone()
    }

    #[test]
    fn home_labels_resolve_native_english_by_default() {
        let history = empty_history();
        let home_doc = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = ArtifactView::new(&home_doc, &history);
        let config = config_with_one_folded_space("en-US");
        let cfg = ConfigView { snapshot: &config };
        let home_node = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg);
        let json = serde_json::to_string(&home_node).unwrap();
        assert!(json.contains("Updated"), "English column header must resolve: {json}");
        assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
    }

    #[test]
    fn home_labels_resolve_native_german_locale() {
        let history = empty_history();
        let home_doc = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
        let home_view = ArtifactView::new(&home_doc, &history);
        let config = config_with_one_folded_space("de");
        let cfg = ConfigView { snapshot: &config };
        let home_node = HomeApp::render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &home_view, &cfg);
        let json = serde_json::to_string(&home_node).unwrap();
        assert!(json.contains("Aktualisiert"), "German column header must resolve: {json}");
        assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
    }
}
//#endregion 🧪️Tests
