//! ✏️ S Space index editor — the `ArtifactEditor` impl (dispatch-only) for the space's artifact index.
//! Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4. Lane 2-B: the real table
//! (name · kind · subset · updated · updated-by · presence), create/open/delete/rename commands, the
//! members panel, and the folded-directory/presence `Config` state that feeds them both.

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::artifacts::space::SPACE_INDEX_DIALECT;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation};
use crate::editor::space_index::commands::{copy_invite_link, create_artifact, delete_artifact, fold_directory_events, invite_member, open_artifact, open_artifact_with, presence_heartbeat, remove_member, rename_artifact, request_delete_artifact, request_invite_member, set_visibility, touch_artifact};
use crate::editor::space_index::modes::edit;
use crate::editor::space_index::modes::edit::windows::main;
use crate::editor::space_index::panels::members as members_panel;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionFactory, ActionRef, ArtifactEditor, ArtifactView, ConfigView, DialogDefinition, DraftView, Editor, Emit, Fault, FaultCode, FaultOrigin, LocalizedLabel, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode,
};
use semio_framework_plugin::app::Dialect;
use serde_json::Value;
use store::EngineHandles;

//#region 🔖️KnownArtifactKinds
/// 🗂️ Task 2's "artifact kinds that actually have an editor registered" — a static fallback table,
/// NOT a live read of the `ArtifactKindSpec` registry: the space plugin runs as its own isolated
/// guest crate and has no in-process access to the host's registry of every OTHER plugin's editors.
/// Per the worker-brief's own escape hatch ("if the guest cannot see that list, take it from a
/// config-lane value the host sets and say so in your report") this should ideally be a
/// `SpaceIndexConfig` field the host folds in from its own `ArtifactKindSpec` catalog — deferred
/// (documented in `📓️w2-b-report.md`) since that catalog isn't exposed to the shell's directory/
/// opening lane yet either. Curated from four plugins confirmed to have real registered editors
/// (`🖍️draw`, `🗒️note`, `🕸️dag`, `✒️writer` — each grepped for its own `DIALECT`/`DOCUMENT_SCHEMA`
/// constants).
pub struct KnownArtifactKind {
    pub id: &'static str,
    pub schema: &'static str,
    pub dialect_artifact_kind: &'static str,
    pub standard: &'static str,
    pub subset: &'static str,
    pub label_en: &'static str,
    pub label_de: &'static str,
}

pub const KNOWN_ARTIFACT_KINDS: [KnownArtifactKind; 4] = [
    KnownArtifactKind { id: "draw", schema: "draw.document", dialect_artifact_kind: "s.draw.draw", standard: "1", subset: "*", label_en: "Draw", label_de: "Zeichnung" },
    KnownArtifactKind { id: "note", schema: "note.document", dialect_artifact_kind: "s.note.note", standard: "1", subset: "*", label_en: "Note", label_de: "Notiz" },
    KnownArtifactKind { id: "dag", schema: "dag.dag", dialect_artifact_kind: "s.dag.dag", standard: "1", subset: "*", label_en: "Graph", label_de: "Graph" },
    KnownArtifactKind { id: "writer", schema: "writer.document", dialect_artifact_kind: "s.writer.writer", standard: "1", subset: "*", label_en: "Writer", label_de: "Text" },
];

pub fn known_artifact_kind(id: &str) -> Option<&'static KnownArtifactKind> {
    KNOWN_ARTIFACT_KINDS.iter().find(|kind| kind.id == id)
}

fn create_artifact_kind_options() -> Vec<ActionArgOption> {
    KNOWN_ARTIFACT_KINDS.iter().map(|kind| ActionArgOption::new(kind.id, LocalizedLabel::native(kind.label_en, kind.label_de))).collect()
}
//#endregion 🔖️KnownArtifactKinds

//#region 🔖️Actions
/// 🎯️ Every panel/dialog-adjacent action this app declares addresses itself through this factory —
/// mirrors `draw_play_action`'s precedent (`🖍️draw`'s editor root).
pub const SPACE_INDEX_CONTROLLER_ID: &str = "s-space-index";

pub fn space_index_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    ActionFactory::new(SPACE_INDEX_CONTROLLER_ID).action(action, args)
}
//#endregion 🔖️Actions

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SpaceIndexEditor::Command` — the SOLE dispatch surface for the index editor's own behavior:
    /// the four frozen document mutations, plus the opening/deletion-confirm/directory-relay/presence
    /// commands lane 2-B adds on top (worker-brief tasks 2–3). Row order is the binary variant
    /// ordinal — appending is safe, reordering is a wire-format break.
    pub enum SpaceIndexCommand for SSpaceSnapshot, SSpaceMutation, SpaceIndexConfig, SpaceIndexConfigMutation {
        "createArtifact" as "create-artifact" => create_artifact::CreateArtifact,
        "deleteArtifact" as "delete-artifact" => delete_artifact::DeleteArtifact,
        "renameArtifact" as "rename-artifact" => rename_artifact::RenameArtifact,
        "touchArtifact" as "touch-artifact" => touch_artifact::TouchArtifact,
        "requestDeleteArtifact" as "request-delete-artifact" => request_delete_artifact::RequestDeleteArtifact,
        "openArtifact" as "open-artifact" => open_artifact::OpenArtifact,
        "openArtifactWith" as "open-artifact-with" => open_artifact_with::OpenArtifactWith,
        "foldDirectoryEvents" as "fold-directory-events" => fold_directory_events::FoldDirectoryEvents,
        "presenceHeartbeat" as "presence-heartbeat" => presence_heartbeat::PresenceHeartbeat,
        "inviteMember" as "invite-member" => invite_member::InviteMember,
        "removeMember" as "remove-member" => remove_member::RemoveMember,
        "setVisibility" as "set-visibility" => set_visibility::SetVisibility,
        "copyInviteLink" as "copy-invite-link" => copy_invite_link::CopyInviteLink,
        "requestInviteMember" as "request-invite-member" => request_invite_member::RequestInviteMember,
    }
}
//#endregion 🔖️Commands

//#region 🔖️SpaceIndexEditor
#[derive(Default)]
pub struct SpaceIndexEditor;

impl ArtifactEditor for SpaceIndexEditor {
    type Snapshot = SSpaceSnapshot;
    type Mutation = SSpaceMutation;
    type Config = SpaceIndexConfig;
    type ConfigMutation = SpaceIndexConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;

    type Command = SpaceIndexCommand;

    const DIALECT: Dialect = SPACE_INDEX_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::space::S_SPACE_INDEX_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SSpaceSnapshot {
        SSpaceSnapshot::default()
    }

    fn command_id(command: &SpaceIndexCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &SpaceIndexCommand, doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-F: this bridge was
    /// missing entirely — `ArtifactEditor::command_from_action`'s default impl unconditionally errors
    /// (`app.command.unsupported`), and `dispatch_action`'s final `else` arm (the ONLY path a plain
    /// `onAction`/`handleAction` click — every row button, every toolbar button — reaches an app's own
    /// command through) calls exactly this. Every one of this app's own actions was therefore a dead
    /// click until now: `openArtifact`/`requestDeleteArtifact` (table row buttons), `createArtifact`
    /// (the new `#s-space-create-artifact` toolbar button), the members panel's invite/remove/visibility/
    /// copy-link buttons — all of it. Mirrors `HomeCommand::command_from_action`'s `str_field` idiom
    /// (`🏠️home/…/✏️editor/🦀️component.rs`) field-for-field against each command payload struct.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<SpaceIndexCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let u64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_u64);
        match action {
            "createArtifact" => Ok(SpaceIndexCommand::CreateArtifact(create_artifact::CreateArtifact {
                name: str_field("name").unwrap_or_default(),
                kind_id: str_field("kindId").or_else(|| str_field("kind_id")).unwrap_or_default(),
                now_ms: u64_field("nowMs").or_else(|| u64_field("now_ms")).unwrap_or_default(),
                actor: str_field("actor").unwrap_or_default(),
            })),
            "deleteArtifact" => Ok(SpaceIndexCommand::DeleteArtifact(delete_artifact::DeleteArtifact { id: str_field("id").unwrap_or_default() })),
            "renameArtifact" => Ok(SpaceIndexCommand::RenameArtifact(rename_artifact::RenameArtifact {
                id: str_field("id").unwrap_or_default(),
                new_name: str_field("newName").or_else(|| str_field("new_name")).unwrap_or_default(),
            })),
            "touchArtifact" => Ok(SpaceIndexCommand::TouchArtifact(touch_artifact::TouchArtifact {
                id: str_field("id").unwrap_or_default(),
                now_ms: u64_field("nowMs").or_else(|| u64_field("now_ms")).unwrap_or_default(),
                actor: str_field("actor").unwrap_or_default(),
            })),
            "requestDeleteArtifact" => Ok(SpaceIndexCommand::RequestDeleteArtifact(request_delete_artifact::RequestDeleteArtifact { id: str_field("id").unwrap_or_default() })),
            "openArtifact" => Ok(SpaceIndexCommand::OpenArtifact(open_artifact::OpenArtifact { id: str_field("id").unwrap_or_default() })),
            "openArtifactWith" => Ok(SpaceIndexCommand::OpenArtifactWith(open_artifact_with::OpenArtifactWith {
                id: str_field("id").unwrap_or_default(),
                role: str_field("role").unwrap_or_default(),
                plugin_id: str_field("pluginId").or_else(|| str_field("plugin_id")).unwrap_or_default(),
                app_id: str_field("appId").or_else(|| str_field("app_id")).unwrap_or_default(),
            })),
            "foldDirectoryEvents" => Ok(SpaceIndexCommand::FoldDirectoryEvents(fold_directory_events::FoldDirectoryEvents { events_json: str_field("eventsJson").or_else(|| str_field("events_json")).unwrap_or_else(|| "[]".into()) })),
            "presenceHeartbeat" => Ok(SpaceIndexCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat {
                artifact_id: str_field("artifactId").or_else(|| str_field("artifact_id")).unwrap_or_default(),
                actors_csv: str_field("actorsCsv").or_else(|| str_field("actors_csv")).unwrap_or_default(),
            })),
            "inviteMember" => Ok(SpaceIndexCommand::InviteMember(invite_member::InviteMember { email: str_field("email").unwrap_or_default(), role: str_field("role").unwrap_or_default() })),
            "removeMember" => Ok(SpaceIndexCommand::RemoveMember(remove_member::RemoveMember { user_id: str_field("userId").or_else(|| str_field("user_id")).unwrap_or_default() })),
            "setVisibility" => Ok(SpaceIndexCommand::SetVisibility(set_visibility::SetVisibility { visibility: str_field("visibility").unwrap_or_default() })),
            "copyInviteLink" => Ok(SpaceIndexCommand::CopyInviteLink(copy_invite_link::CopyInviteLink { role: str_field("role").unwrap_or_default(), ttl_secs: u64_field("ttlSecs").or_else(|| u64_field("ttl_secs")).unwrap_or(0) })),
            "requestInviteMember" => Ok(SpaceIndexCommand::RequestInviteMember(request_invite_member::RequestInviteMember {})),
            other => Err(Fault::new(FaultOrigin::App, FaultCode::new("s.space.unhandled-action"), format!("space index: unhandled action id {other}"))),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>) -> UiNode {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot, cfg.snapshot),
            members_panel::SPACE_INDEX_BODY_MEMBERS => members_panel::render(cfg.snapshot),
            _ => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️SpaceIndexEditor

//#region 🔖️Manifest
pub fn create_space_index_editor() -> semio_framework_plugin::AppDefinition {
    Editor::builder(SPACE_INDEX_DIALECT)
        .document(["semio", "s", "space", "index"])
        .artifact_kind(crate::artifacts::space::artifact_kind())
        .icon_id("layout-grid")
        .mode_def(edit::definition())
        .default_mode_id(edit::SPACE_INDEX_MODE_EDIT)
        .window_kind_def(main::definition())
        .panel_tab_def(members_panel::definition())
        .default_layout(edit::layout())
        // 🌱 Document mutations — palette-visible.
        .mutation("createArtifact", LocalizedLabel::native("Create Artifact", "Artefakt erstellen"))
        .mutation("deleteArtifact", LocalizedLabel::native("Delete Artifact", "Artefakt löschen"))
        .mutation("renameArtifact", LocalizedLabel::native("Rename Artifact", "Artefakt umbenennen"))
        .mutation("touchArtifact", LocalizedLabel::native("Touch Artifact", "Artefakt aktualisieren"))
        // 🐚 Shell-effect relays — no document mutation of their own (contract §C6).
        .shell_action("requestDeleteArtifact", LocalizedLabel::native("Delete Artifact…", "Artefakt löschen…"))
        .shell_action("openArtifact", LocalizedLabel::native("Open Artifact", "Artefakt öffnen"))
        .shell_action("openArtifactWith", LocalizedLabel::native("Open Artifact With…", "Artefakt öffnen mit…"))
        .shell_action("inviteMember", LocalizedLabel::native("Invite Member (Submit)", "Mitglied einladen (Absenden)"))
        .shell_action("requestInviteMember", LocalizedLabel::native("Invite Member…", "Mitglied einladen…"))
        .shell_action("removeMember", LocalizedLabel::native("Remove Member", "Mitglied entfernen"))
        .shell_action("setVisibility", LocalizedLabel::native("Set Visibility", "Sichtbarkeit festlegen"))
        .shell_action("copyInviteLink", LocalizedLabel::native("Copy Invite Link", "Einladungslink kopieren"))
        // 👁️ View actions — fold host-pushed state into `Config`, never in the palette.
        .view_action("foldDirectoryEvents", LocalizedLabel::native("Fold Directory Events", "Verzeichnisereignisse übernehmen"))
        .view_action("presenceHeartbeat", LocalizedLabel::native("Presence Heartbeat", "Präsenz-Heartbeat"))
        // 🗨️ Dialogs (worker-brief tasks 2–3). `createArtifact`'s submit re-dispatches the real
        // mutation directly (its own payload has no field the staged form can't supply); the delete
        // confirm and the invite form each go through a `request*` opener (see those commands' own
        // doc comments for why).
        .dialog(
            DialogDefinition::new("createArtifact", LocalizedLabel::native("Create Artifact", "Artefakt erstellen"), ActionRef::new("createArtifact"))
                .args(vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(), ActionArgDef::select("kindId", LocalizedLabel::native("Kind", "Art"), create_artifact_kind_options()).required()])
                .submit_label(LocalizedLabel::native("Create", "Erstellen")),
        )
        .dialog(
            DialogDefinition::new("deleteArtifact", LocalizedLabel::native("Delete Artifact?", "Artefakt löschen?"), ActionRef::new("deleteArtifact"))
                .body(LocalizedLabel::native("This removes the artifact from the space. This cannot be undone.", "Dies entfernt das Artefakt aus dem Space. Dies kann nicht rückgängig gemacht werden."))
                .submit_label(LocalizedLabel::native("Delete", "Löschen")),
        )
        .dialog(
            DialogDefinition::new("inviteMember", LocalizedLabel::native("Invite Member", "Mitglied einladen"), ActionRef::new("inviteMember"))
                .args(vec![
                    ActionArgDef::text("email", LocalizedLabel::native("Email", "E-Mail")).required(),
                    ActionArgDef::select("role", LocalizedLabel::native("Role", "Rolle"), vec![ActionArgOption::new("author", LocalizedLabel::native("Author", "Autor")), ActionArgOption::new("spectator", LocalizedLabel::native("Spectator", "Betrachter"))]).default_value("spectator").required(),
                ])
                .submit_label(LocalizedLabel::native("Invite", "Einladen")),
        )
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as framework_new_app};
    use semio_framework_plugin::EditorApp;

    pub type SpaceIndexApp = semio_framework_plugin::VcsArtifactApp<EditorApp<SpaceIndexEditor>>;

    pub fn new_app() -> SpaceIndexApp {
        framework_new_app::<EditorApp<SpaceIndexEditor>>()
    }

    #[allow(dead_code)]
    pub fn dispatch(app: &mut SpaceIndexApp, command: SpaceIndexCommand) -> semio_framework_plugin::InvocationResult {
        use semio_framework_plugin::PluginApp;
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_space_index_editor_builds_a_definition_for_this_dialect() {
        let definition = create_space_index_editor();
        assert_eq!(definition.dialect, SPACE_INDEX_DIALECT.into());
    }

    #[test]
    fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<SpaceIndexEditor as ArtifactEditor>::DIALECT, SPACE_INDEX_DIALECT);
    }

    #[test]
    fn every_declared_mutation_action_is_registered() {
        let definition = create_space_index_editor();
        for command in ["createArtifact", "deleteArtifact", "renameArtifact", "touchArtifact", "requestDeleteArtifact", "openArtifact", "openArtifactWith", "inviteMember", "requestInviteMember", "removeMember", "setVisibility", "copyInviteLink", "foldDirectoryEvents", "presenceHeartbeat"] {
            assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == command), "registry declares {command}");
        }
    }

    #[test]
    fn the_three_dialogs_are_registered_with_the_right_submit_actions() {
        let definition = create_space_index_editor();
        assert_eq!(definition.dialogs.len(), 3);
        let by_id = |id: &str| definition.dialogs.iter().find(|dialog| dialog.id == id).unwrap_or_else(|| panic!("dialog {id} must be registered"));
        assert_eq!(by_id("createArtifact").submit_action, ActionRef::new("createArtifact"));
        assert_eq!(by_id("createArtifact").args.len(), 2);
        assert_eq!(by_id("deleteArtifact").submit_action, ActionRef::new("deleteArtifact"));
        assert_eq!(by_id("inviteMember").submit_action, ActionRef::new("inviteMember"));
        assert_eq!(by_id("inviteMember").args.len(), 2);
    }

    #[test]
    fn known_artifact_kinds_resolve_by_id_and_reject_unknown_ids() {
        assert_eq!(known_artifact_kind("draw").unwrap().dialect_artifact_kind, "s.draw.draw");
        assert_eq!(known_artifact_kind("note").unwrap().schema, "note.document");
        assert!(known_artifact_kind("nope").is_none());
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView};
        let snapshot = SSpaceSnapshot::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = SpaceIndexConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let json = serde_json::to_string(&<SpaceIndexEditor as ArtifactEditor>::render("nope", &doc, &cfg)).expect("json");
        assert!(json.contains("Unknown body"));
    }

    #[test]
    fn the_members_panel_body_renders_through_the_editor_dispatch() {
        use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView};
        let snapshot = SSpaceSnapshot::default();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = SpaceIndexConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let json = serde_json::to_string(&<SpaceIndexEditor as ArtifactEditor>::render(members_panel::SPACE_INDEX_BODY_MEMBERS, &doc, &cfg)).expect("json");
        assert!(json.contains("s-space-invite"));
    }

    /// 🆔️ Lane 4-F: `command_from_action` was entirely missing before this — every one of this app's
    /// own actions dispatched via a plain button click (`onAction`/`handleAction`) hit the default
    /// trait impl's unconditional error. Covers every declared action id, mirroring the sibling
    /// `command_from_action_covers_every_declared_action_and_rejects_unknown_ones` convention other
    /// artifact editors already use (e.g. `🌍️gis/🗿️artifacts/🗺️gismap`).
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        let cases: Vec<(&str, serde_json::Value)> = vec![
            ("createArtifact", serde_json::json!({ "name": "First", "kindId": "draw", "nowMs": 1, "actor": "user:1" })),
            ("deleteArtifact", serde_json::json!({ "id": "artifact-1" })),
            ("renameArtifact", serde_json::json!({ "id": "artifact-1", "newName": "Renamed" })),
            ("touchArtifact", serde_json::json!({ "id": "artifact-1", "nowMs": 2, "actor": "user:1" })),
            ("requestDeleteArtifact", serde_json::json!({ "id": "artifact-1" })),
            ("openArtifact", serde_json::json!({ "id": "artifact-1" })),
            ("openArtifactWith", serde_json::json!({ "id": "artifact-1", "role": "editor", "pluginId": "writer", "appId": "writer.editor" })),
            ("foldDirectoryEvents", serde_json::json!({ "eventsJson": "[]" })),
            ("presenceHeartbeat", serde_json::json!({ "artifactId": "artifact-1", "actorsCsv": "user:1" })),
            ("inviteMember", serde_json::json!({ "email": "a@example.com", "role": "author" })),
            ("removeMember", serde_json::json!({ "userId": "user:1" })),
            ("setVisibility", serde_json::json!({ "visibility": "public" })),
            ("copyInviteLink", serde_json::json!({ "role": "spectator", "ttlSecs": 604800u64 })),
            ("requestInviteMember", serde_json::json!({})),
        ];
        for (action, args) in cases {
            let command = SpaceIndexEditor::command_from_action(action, Some(&args)).unwrap_or_else(|error| panic!("{action} must bridge: {error:?}"));
            assert_eq!(command.command_id(), action, "the bridged command's own id must round-trip");
        }
        assert!(SpaceIndexEditor::command_from_action("bogus", None).is_err());
    }

    /// 🆔️ Lane 4-F: `#s-space-create-artifact`'s no-args click must bridge to an EMPTY `CreateArtifact`
    /// payload (not error on missing fields) — its own handler treats empty `name`/`kindId` as "open
    /// the dialog", mirroring Home's `createSpace`.
    #[test]
    fn command_from_action_bridges_an_empty_create_artifact_click() {
        let SpaceIndexCommand::CreateArtifact(payload) = SpaceIndexEditor::command_from_action("createArtifact", None).expect("no-args click must bridge") else { panic!("expected CreateArtifact") };
        assert_eq!(payload.name, "");
        assert_eq!(payload.kind_id, "");
    }
}
//#endregion 🧪️Tests
