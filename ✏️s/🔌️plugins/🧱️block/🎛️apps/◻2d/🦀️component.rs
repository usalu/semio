//! 🩻️ Block 2D play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the board window
//! in `🎭️modes/✏️edit/🪟️windows/📋️board`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`,
//! view state in `🦀️config.rs`, and document-side compute in `crate::artifacts::block2d::engine`.

use crate::apps::block2d::commands::compatibility::{add_compatibility_rule, remove_compatibility_rule};
use crate::apps::block2d::commands::example;
use crate::apps::block2d::commands::example::{edit, set_active_example};
use crate::apps::block2d::commands::handle::{add_handle, remove_handle};
use crate::apps::block2d::commands::handle_kind::{add_handle_kind, remove_handle_kind};
use crate::apps::block2d::commands::kind::patch_node_kind;
use crate::apps::block2d::commands::selection::set_selection;
use crate::apps::block2d::config::{Block2dConfig, Block2dConfigOperation};
use crate::apps::block2d::modes::edit as edit_mode;
use crate::apps::block2d::modes::edit::windows::board;
use crate::apps::block2d::panels::{document as document_panel, inspection as inspection_panel};
use crate::apps::block2d::terminology::block2d_labels;
use crate::artifacts::block2d::op::Block2dOperation;
use crate::artifacts::block2d::{artifact_kind, Block2dDefinition, BLOCK_2D_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, 
    ActionDescriptor, App, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode,
};
use store::EngineHandles;
use serde_json::Value;

//#region 🔖️Constants
pub const BLOCK2D_PLAY_APP_ID: &str = "block2d-play";
/// 🗂️ The `s/plugin/puzzle` 2d catalog artifact kind block2d's `"catalog:out"` port produces — see
/// `crate::artifacts::block2d::engine::block2d_io` and `Block2dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn block2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(BLOCK2D_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Block2dPlayApp::Command` — the SOLE dispatch surface for block2d's own behavior, covering
    /// every action `create_block2d_app` declares. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. Every id/key pair here is IDENTICAL (the pre-migration
    /// `#[dsl(key)]` already used the camelCase action id, not kebab-case) — preserved verbatim, not
    /// "fixed" to kebab, so the wire format stays byte-identical.
    pub enum Block2dCommand for Block2dDefinition, Block2dOperation, Block2dConfig, Block2dConfigOperation {
        "patchNodeKind" as "patchNodeKind" => patch_node_kind::PatchNodeKind,
        "addHandleKind" as "addHandleKind" => add_handle_kind::AddHandleKind,
        "removeHandleKind" as "removeHandleKind" => remove_handle_kind::RemoveHandleKind,
        "addHandle" as "addHandle" => add_handle::AddHandle,
        "removeHandle" as "removeHandle" => remove_handle::RemoveHandle,
        "addCompatibilityRule" as "addCompatibilityRule" => add_compatibility_rule::AddCompatibilityRule,
        "removeCompatibilityRule" as "removeCompatibilityRule" => remove_compatibility_rule::RemoveCompatibilityRule,
        "setActiveExample" as "setActiveExample" => set_active_example::SetActiveExample,
        "edit" as "edit" => edit::Edit,
        "setSelection" as "setSelection" => set_selection::SetSelection,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Block2dPlayApp
/// 🧪️ B1: unit struct — the former `selected_ids` `RefCell` field now lives in
/// `crate::apps::block2d::config::Block2dConfig`, written through `Block2dConfigOperation`s.
#[derive(Default)]
pub struct Block2dPlayApp;

impl DocumentApp for Block2dPlayApp {
    type Projection = Block2dDefinition;
    type Operation = Block2dOperation;
    type Config = Block2dConfig;
    type ConfigOperation = Block2dConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = Block2dCommand;

    const APP_ID: &'static str = BLOCK2D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_2D_SCHEMA;

    fn initial_projection() -> Block2dDefinition {
        crate::artifacts::block2d::engine::empty_block2d_definition()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(crate::artifacts::block2d::engine::block2d_io())
    }

    fn command_id(command: &Block2dCommand) -> &str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block2dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let str_vec_field = |key: &str| -> Vec<String> {
            args.and_then(|value| value.get(key))
                .and_then(|value| value.as_array())
                .map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect())
                .unwrap_or_default()
        };
        match action {
            "patchNodeKind" => Ok(Block2dCommand::PatchNodeKind(patch_node_kind::PatchNodeKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            "addHandleKind" => Ok(Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})),
            "removeHandleKind" => Ok(Block2dCommand::RemoveHandleKind(remove_handle_kind::RemoveHandleKind { id: str_field("id").unwrap_or_default() })),
            "addHandle" => Ok(Block2dCommand::AddHandle(add_handle::AddHandle {})),
            "removeHandle" => Ok(Block2dCommand::RemoveHandle(remove_handle::RemoveHandle { id: str_field("id").unwrap_or_default() })),
            "addCompatibilityRule" => Ok(Block2dCommand::AddCompatibilityRule(add_compatibility_rule::AddCompatibilityRule { source: str_field("source").unwrap_or_default(), target: str_field("target").unwrap_or_default() })),
            "removeCompatibilityRule" => Ok(Block2dCommand::RemoveCompatibilityRule(remove_compatibility_rule::RemoveCompatibilityRule { id: str_field("id").unwrap_or_default() })),
            "setActiveExample" => Ok(Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() })),
            "edit" => Ok(Block2dCommand::Edit(edit::Edit { text: str_field("text").unwrap_or_default() })),
            "setSelection" => Ok(Block2dCommand::SetSelection(set_selection::SetSelection { ids: str_vec_field("ids") })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(command: &Block2dCommand, doc: &DocumentView<'_, Block2dDefinition>, cfg: &ConfigView<'_, Block2dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Block2dOperation, Block2dConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Block2dDefinition>, cfg: &ConfigView<'_, Block2dConfig>) -> UiNode {
        let labels = block2d_labels(&cfg.projection.locale);
        match body_key {
            board::BLOCK2D_BODY_BOARD => board::render(doc.projection, labels),
            document_panel::BLOCK2D_BODY_DOCUMENT => document_panel::render(doc.projection, &cfg.projection.selected_ids, labels),
            inspection_panel::BLOCK2D_BODY_INSPECTOR => inspection_panel::render(doc.projection, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🌉️ `puzzle2d_manifest_fragment`'s first real caller — wraps the block-2d document's
    /// puzzle2d-shaped catalog fragment (`portKinds`/`wireKinds`/`edgeKinds`/`nodeKinds`/
    /// `kindCompatibility`) as a `kit.catalog`-schema `Media` value for the `"catalog:out"` port
    /// declared in `crate::artifacts::block2d::engine::block2d_io`. Falls through to the default
    /// whole-document pack export for every other port (`"document:out"`).
    fn export_media(port: &str, doc: &DocumentView<'_, Block2dDefinition>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `DocumentApp::export_media`'s default `"document:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this app.
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type }, |io| io.document_media_type);
            let bytes = store::DocumentPack::encode_pack(doc.projection);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = crate::artifacts::block2d::engine::puzzle2d_manifest_fragment(doc.projection);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block2dPlayApp

//#region 🔖️Manifest
pub fn create_block2d_app() -> App {
    App::from_builder(
        App::builder(BLOCK2D_PLAY_APP_ID, LocalizedLabel::native("Block 2D", "Block 2D"))
            .document(["semio", "block", "2d"])
            .artifact_kind(artifact_kind())
            // 🗂️ The puzzle2d catalog artifact this app's new `"catalog:out"` port produces — see
            // `crate::artifacts::block2d::engine::block2d_io`/`Block2dPlayApp::export_media`.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "2d".into(),
                media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("layout-grid")
            .mode_def(edit_mode::definition())
            .default_mode_id(edit_mode::BLOCK2D_PLAY_MODE_EDIT)
            .window_kind_def(board::definition())
            .default_layout(edit_mode::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .operation("patchNodeKind", LocalizedLabel::native("Patch Node Kind", "Knotenart bearbeiten"))
            .operation("addHandleKind", LocalizedLabel::native("Add Handle Kind", "Griffart hinzufügen"))
            .operation("removeHandleKind", LocalizedLabel::native("Remove Handle Kind", "Griffart entfernen"))
            .operation("addHandle", LocalizedLabel::native("Add Handle", "Griff hinzufügen"))
            .operation("removeHandle", LocalizedLabel::native("Remove Handle", "Griff entfernen"))
            .operation("addCompatibilityRule", LocalizedLabel::native("Add Compatibility Rule", "Kompatibilitätsregel hinzufügen"))
            .operation("removeCompatibilityRule", LocalizedLabel::native("Remove Compatibility Rule", "Kompatibilitätsregel entfernen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .io(crate::artifacts::block2d::engine::block2d_io()),
    )
    .example(
        example::BLOCK2D_EXAMPLE_LEFT,
        LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
        serde_json::to_string(&crate::artifacts::block2d::dsl::parse_dsl(crate::artifacts::block2d::dsl::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "list-tree",
    )
    .example(
        example::BLOCK2D_EXAMPLE_RIGHT,
        LocalizedLabel::native("Hexagonal Cut Concrete Forest Right", "Hexagonal Cut Concrete Forest Right"),
        serde_json::to_string(&crate::artifacts::block2d::dsl::parse_dsl(crate::artifacts::block2d::dsl::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "list-tree",
    )
    .workflow("block2d", "Block 2D", "model")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type Block2dApp = VcsDocumentApp<Block2dPlayApp>;

    pub fn new_app() -> Block2dApp {
        sdk_new_app::<Block2dPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> Block2dApp {
        new_app_with_registry::<Block2dPlayApp>(create_block2d_app)
    }

    pub fn dispatch(app: &mut Block2dApp, command: Block2dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Block2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::block2d::testkit::{new_app, Block2dApp};
    use semio_framework_plugin::PluginApp;

    //#region 🔖️CommandSurface
    fn every_command() -> Vec<Block2dCommand> {
        vec![
            Block2dCommand::PatchNodeKind(patch_node_kind::PatchNodeKind { field: "name".into(), value: "x".into() }),
            Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {}),
            Block2dCommand::RemoveHandleKind(remove_handle_kind::RemoveHandleKind { id: "h0".into() }),
            Block2dCommand::AddHandle(add_handle::AddHandle {}),
            Block2dCommand::RemoveHandle(remove_handle::RemoveHandle { id: "h0".into() }),
            Block2dCommand::AddCompatibilityRule(add_compatibility_rule::AddCompatibilityRule { source: "a".into(), target: "b".into() }),
            Block2dCommand::RemoveCompatibilityRule(remove_compatibility_rule::RemoveCompatibilityRule { id: "c0".into() }),
            Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: "left".into() }),
            Block2dCommand::Edit(edit::Edit { text: "{}".into() }),
            Block2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["h0".into()] }),
        ]
    }

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Block2dCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 10, "every Block2dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
            let printed = protocol::OpText::print_op(&command);
            assert!(printed.starts_with(command.command_id()), "row {} printed {printed:?}", command.command_id());
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for the rows the `app_commands!` decomposition could have
    /// silently rewritten — copied verbatim from the ticket's `🧪️wire-baseline-2d-before.txt`.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Block2dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(hex(&Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})), "01010000");
        assert_eq!(hex(&Block2dCommand::AddHandle(add_handle::AddHandle {})), "01030000");
        assert_eq!(hex(&Block2dCommand::SetSelection(set_selection::SetSelection { ids: Vec::new() })), "01090001000c00");
        assert_eq!(hex(&Block2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["h0".into()] })), "01090102683001000c010600");
    }

    /// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`.
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Block2dPlayApp>(create_block2d_app);
        assert!(Block2dPlayApp.command_from_action("noSuchAction", None).is_err());
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_block2d_app().definition;
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        for body_key in [document_panel::BLOCK2D_BODY_DOCUMENT, inspection_panel::BLOCK2D_BODY_INSPECTOR] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    #[test]
    fn block2d_io_is_wired_into_the_manifest() {
        let definition = create_block2d_app().definition;
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = new_app();
        assert!(testkit::render(&mut app, "block2d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Behavior
    #[test]
    fn add_handle_kind_then_add_handle_then_remove_round_trips() {
        let mut app: Block2dApp = new_app();
        testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {}));
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
        testkit::dispatch(&mut app, Block2dCommand::AddHandle(add_handle::AddHandle {}));
        let projection = app.projection().expect("projection");
        assert_eq!(projection.handles.len(), 1);
        let handle_id = projection.handles[0].id.clone();
        testkit::dispatch(&mut app, Block2dCommand::RemoveHandle(remove_handle::RemoveHandle { id: handle_id }));
        assert_eq!(app.projection().expect("projection").handles.len(), 0);
    }

    #[test]
    fn patch_node_kind_updates_name() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::PatchNodeKind(patch_node_kind::PatchNodeKind { field: "name".into(), value: "Renamed".into() }));
        assert_eq!(app.projection().expect("projection").node_kind.name, "Renamed");
    }

    #[test]
    fn set_active_example_loads_left_fixture() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: example::BLOCK2D_EXAMPLE_LEFT.into() }));
        let projection = app.projection().expect("projection");
        assert_eq!(projection.node_kind.id, "Hexagonal Cut Concrete Forest Left");
        assert_eq!(projection.handles.len(), 11);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {}));
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 0);
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").handle_kinds.len(), 1);
    }

    #[test]
    fn set_selection_writes_config_not_document() {
        let mut app = new_app();
        let result = app.dispatch_typed(Block2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["handle-kind:b-l".into()] }), &semio_framework_plugin::testkit::meta("local")).expect("select");
        assert!(result.operations.is_empty(), "setSelection is config-only and must emit no document operations");
    }

    /// 🌉️ `puzzle2d_manifest_fragment`'s new caller round-trips through the `"catalog:out"` media port.
    #[test]
    fn export_media_catalog_out_wraps_the_puzzle2d_fragment() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: example::BLOCK2D_EXAMPLE_LEFT.into() }));
        let media = app.export_media("catalog:out").expect("export catalog");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let value: Value = serde_json::from_str(&json).expect("valid json");
                assert_eq!(value["nodeKinds"][0]["id"], "Hexagonal Cut Concrete Forest Left");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn command_from_action_bridges_set_active_example() {
        let app = Block2dPlayApp;
        assert!(matches!(app.command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": "left" }))), Ok(Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "left"));
    }

    /// 🧬️ Kind-discipline wrapper: the real registry enforces View actions never emit document
    /// operations. Exercising it here (rather than only the plain `new_app()`) is the reason
    /// `testkit::app_with_registry` exists.
    #[test]
    fn view_actions_never_emit_document_operations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Block2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["h0".into()] }));
        assert!(result.operations.is_empty(), "setSelection is a view action and must never reach document operations under kind discipline");
    }
    //#endregion 🔖️Behavior
}
//#endregion 🧪️Tests
