//! 🩻️ Block 2D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the board window
//! in `🎭️modes/✏️edit/🪟️windows/📋️board`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`,
//! view state in `🦀️config.rs`, document-side compute in `crate::artifacts::block2d::schema`, and this
//! app's own typed media I/O surface + plugin registration (below — constitutional: general, an
//! artifact must never depend on an app, so both live here rather than under `🗿️artifacts`).

use crate::editor::block2d::commands::{add_compatibility_rule, remove_compatibility_rule};
use crate::editor::block2d::commands::{edit, set_active_example};
use crate::editor::block2d::commands::{add_handle, remove_handle};
use crate::editor::block2d::commands::{add_handle_kind, remove_handle_kind};
use crate::editor::block2d::commands::patch_node_kind;
use crate::editor::block2d::config::{Block2dConfig, Block2dConfigMutation};
use crate::editor::block2d::modes::edit as edit_mode;
use crate::editor::block2d::modes::edit::windows::board;
use crate::editor::block2d::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::block2d::terminology::block2d_labels;
use crate::artifacts::block2d::op::Block2dMutation;
use crate::artifacts::block2d::{artifact_kind, Block2dSnapshot, BLOCK_2D_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    ActionDescriptor, AppIo, ArtifactEditor, ArtifactKindSpec, ArtifactPresentation, ConfigView, ArtifactView, Dialect, Editor, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaPortDirection, MediaPortSpec, MediaType, PortMultiplicity, UiNode,
};
use semio_framework_plugin::app::InteractionView;
use semio_framework::{DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode};
use store::EngineHandles;
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Constants
pub const BLOCK2D_PLAY_APP_ID: &str = "block2d-play";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the framework-owned hover/selection
/// domain over this app's rim-handle templates ("handle" granularity, the default) and handle-kind
/// catalog ("handleKind" granularity) — replaces the deleted `Block2dConfig.selected_ids`.
pub const BLOCK2D_INTERACTION_HANDLE: &str = "handle";
pub const BLOCK2D_GRANULARITY_HANDLE: &str = "handle";
pub const BLOCK2D_GRANULARITY_HANDLE_KIND: &str = "handleKind";
/// 🗂️ The `s/plugin/puzzle` 2d catalog artifact kind block2d's `"catalog:out"` port produces — see
/// `block2d_io` and `Block2dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn block2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(BLOCK2D_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ `Block2dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"2d.block"` artifact kind) plus a `"catalog:out"` port giving
/// `puzzle2d_manifest_fragment` a real caller (see `export_media` above).
pub fn block2d_io() -> AppIo {
    AppIo::from_document(
        BLOCK_2D_SCHEMA,
        MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        ArtifactPresentation { id: "2d.block".into(), name: "Node Kind".into(), dimension: "2d".into(), component_kind: "block2d".into() },
    )
    .with_ports(vec![MediaPortSpec {
        id: "catalog:out".into(),
        label: "Kit Catalog".into(),
        direction: MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        kind_id: Some("kit.catalog".into()),
        required: false,
        multiplicity: PortMultiplicity::Many,
    }])
}
//#endregion 🔖️Io

//#region 🔌️Registration
// 🗂️ `Block2dSnapshot`'s pack↔dsl codec, `block2d`'s artifact schema/inference descriptors, its
// composer table and its pilot-language grammars now register declaratively via
// `crate::artifacts::block2d::declaration()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
// M1/W1d), consumed by `.artifact(crate::artifacts::block2d::declaration())` in the plugin root
// (`🧱️block/🦀️component.rs`) — replacing this app's former side-effecting `register()`. Nothing
// app-scope-only remains here: `Block2dPlayApp::app_schema()` now returns
// `crate::editor::block2d::config::schema::app_schema_descriptor()` directly (ticket W1c), so the
// plugin root's `.setup()` escape hatch is gone entirely.
//#endregion 🔌️Registration

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Block2dPlayApp::Command` — the SOLE dispatch surface for block2d's own behavior, covering
    /// every action `create_block2d_app` declares. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. Every id/key pair here is IDENTICAL (the pre-migration
    /// `#[dsl(key)]` already used the camelCase action id, not kebab-case) — preserved verbatim, not
    /// "fixed" to kebab, so the wire format stays byte-identical.
    pub enum Block2dCommand for Block2dSnapshot, Block2dMutation, Block2dConfig, Block2dConfigMutation {
        "patchNodeKind" as "patchNodeKind" => patch_node_kind::PatchNodeKind,
        "addHandleKind" as "addHandleKind" => add_handle_kind::AddHandleKind,
        "removeHandleKind" as "removeHandleKind" => remove_handle_kind::RemoveHandleKind,
        "addHandle" as "addHandle" => add_handle::AddHandle,
        "removeHandle" as "removeHandle" => remove_handle::RemoveHandle,
        "addCompatibilityRule" as "addCompatibilityRule" => add_compatibility_rule::AddCompatibilityRule,
        "removeCompatibilityRule" as "removeCompatibilityRule" => remove_compatibility_rule::RemoveCompatibilityRule,
        "setActiveExample" as "setActiveExample" => set_active_example::SetActiveExample,
        "edit" as "edit" => edit::Edit,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Block2dPlayApp
/// 🧪️ B1: unit struct — the former `selected_ids` `RefCell` field now lives in
/// `crate::editor::block2d::config::Block2dConfig`, written through `Block2dConfigMutation`s.
#[derive(Default)]
pub struct Block2dPlayApp;

impl ArtifactEditor for Block2dPlayApp {
    type Snapshot = Block2dSnapshot;
    type Mutation = Block2dMutation;
    type Config = Block2dConfig;
    type ConfigMutation = Block2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::block2d::presence::Block2dPresence;
    type PresenceMutation = crate::editor::block2d::presence::Block2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Block2dCommand;

    const DIALECT: Dialect = crate::artifacts::block2d::BLOCK2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_2D_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::block2d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Block2dSnapshot {
        crate::artifacts::block2d::schema::empty_block2d_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(block2d_io())
    }

    fn command_id(command: &Block2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block2dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
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
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(command: &Block2dCommand, doc: &ArtifactView<'_, Block2dSnapshot>, cfg: &ConfigView<'_, Block2dConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Block2dMutation, Block2dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `handle` domain's
    /// `HierarchyProvider::Topology` — every handle-kind is a root (`handleKind` granularity), every
    /// handle nests under its own `handle_kind` (`handle` granularity), so a stale selection is
    /// pruned the moment `removeHandleKind`/`removeHandle` deletes its target, and hovering/selecting
    /// a kind can transitively reach its handles.
    fn interaction_topology(doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> InteractionTopology {
        let mut ordered: Vec<TopologyNode> = Vec::new();
        for kind in &doc.snapshot.handle_kinds {
            ordered.push(TopologyNode { id: format!("handleKind:{}", kind.id), granularity: BLOCK2D_GRANULARITY_HANDLE_KIND.into(), parent: None });
        }
        for handle in &doc.snapshot.handles {
            ordered.push(TopologyNode { id: format!("handle:{}", handle.id), granularity: BLOCK2D_GRANULARITY_HANDLE.into(), parent: Some(format!("handleKind:{}", handle.handle_kind)) });
        }
        let mut domains = BTreeMap::new();
        domains.insert(BLOCK2D_INTERACTION_HANDLE.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Block2dSnapshot>, cfg: &ConfigView<'_, Block2dConfig>) -> UiNode {
        let labels = block2d_labels(&cfg.snapshot.locale);
        match body_key {
            board::BLOCK2D_BODY_BOARD => board::render(doc.snapshot, labels),
            document_panel::BLOCK2D_BODY_DOCUMENT => document_panel::render(doc.snapshot, labels),
            inspection_panel::BLOCK2D_BODY_INSPECTOR => inspection_panel::render(doc.snapshot, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🌉️ `puzzle2d_manifest_fragment`'s first real caller — wraps the block-2d document's
    /// puzzle2d-shaped catalog fragment (`portKinds`/`wireKinds`/`edgeKinds`/`nodeKinds`/
    /// `kindCompatibility`) as a `kit.catalog`-schema `Media` value for the `"catalog:out"` port
    /// declared in `block2d_io`. Falls through to the default whole-document pack export for every
    /// other port (`"document:out"`).
    fn export_media(port: &str, doc: &ArtifactView<'_, Block2dSnapshot>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `ArtifactEditor::export_media`'s default `"document:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this app.
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type }, |io| io.document_media_type);
            let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = crate::artifacts::block2d::schema::inferences::puzzle2d_manifest_fragment(doc.snapshot);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block2dPlayApp

//#region 🔖️Manifest
pub fn create_block2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::block2d::BLOCK2D_DIALECT)
            .document(["semio", "block", "2d"])
            .artifact_kind(artifact_kind())
            // 🗂️ The puzzle2d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block2d_io`/`Block2dPlayApp::export_media`.
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
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("layout-grid")
            .mode_def(edit_mode::definition())
            .default_mode_id(edit_mode::BLOCK2D_PLAY_MODE_EDIT)
            .window_kind_def(board::definition())
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `handle` domain
            // replaces the deleted `setSelection` view action — the framework auto-injects
            // `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
            // `setInteractionGranularity` for it.
            .interaction(InteractionDefinition {
                id: BLOCK2D_INTERACTION_HANDLE.into(),
                label: LocalizedLabel::native("Handles", "Griffe"),
                granularities: vec![
                    GranularityDefinition { id: BLOCK2D_GRANULARITY_HANDLE.into(), label: LocalizedLabel::native("Handle", "Griff"), icon_id: "circle-dot".into() },
                    GranularityDefinition { id: BLOCK2D_GRANULARITY_HANDLE_KIND.into(), label: LocalizedLabel::native("Handle Kind", "Griffart"), icon_id: "circle".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace, MergeMode::Additive], transitive: false, broadcast: true },
            })
            .window_kind_interactions(board::BLOCK2D_WINDOW_BOARD, vec![InteractionRef::new(BLOCK2D_INTERACTION_HANDLE)])
            .default_layout(edit_mode::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("patchNodeKind", LocalizedLabel::native("Patch Node Kind", "Knotenart bearbeiten"))
            .mutation("addHandleKind", LocalizedLabel::native("Add Handle Kind", "Griffart hinzufügen"))
            .mutation("removeHandleKind", LocalizedLabel::native("Remove Handle Kind", "Griffart entfernen"))
            .mutation("addHandle", LocalizedLabel::native("Add Handle", "Griff hinzufügen"))
            .mutation("removeHandle", LocalizedLabel::native("Remove Handle", "Griff entfernen"))
            .mutation("addCompatibilityRule", LocalizedLabel::native("Add Compatibility Rule", "Kompatibilitätsregel hinzufügen"))
            .mutation("removeCompatibilityRule", LocalizedLabel::native("Remove Compatibility Rule", "Kompatibilitätsregel entfernen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .io(block2d_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old
            // `BLOCK2D_EXAMPLE_LEFT`/`BLOCK2D_EXAMPLE_RIGHT` app-level example registrations and the
            // no-op `.workflow("block2d", …)` call are dropped here (not silently: reported in the
            // packet's migration report). The subset's own pre-existing
            // `🗿️artifacts/◻2d/…/📚️examples/🎬️hexagonal-cut-concrete-forest-{left,right}` facet is
            // the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Block2dApp = VcsArtifactApp<EditorApp<Block2dPlayApp>>;

    pub fn new_app() -> Block2dApp {
        sdk_new_app::<EditorApp<Block2dPlayApp>>()
    }

    /// ✏️ Adapts `create_block2d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` still
    /// expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub fn block2d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_block2d_app(), examples: Vec::new() }
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> Block2dApp {
        new_app_with_registry::<EditorApp<Block2dPlayApp>>(block2d_app_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut Block2dApp, command: Block2dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Block2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::block2d::testkit::{new_app, Block2dApp};
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
        assert_eq!(ids.len(), 9, "every Block2dCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
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
    }

    /// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`.
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<semio_framework_plugin::EditorApp<Block2dPlayApp>>(testkit::block2d_app_manifest_for_testkit);
        assert!(Block2dPlayApp::command_from_action("noSuchAction", None).is_err());
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_block2d_app();
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        for body_key in [document_panel::BLOCK2D_BODY_DOCUMENT, inspection_panel::BLOCK2D_BODY_INSPECTOR] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `handle` domain is declared
    /// once, with both granularities, a `Topology` hierarchy, and scoped to the board window kind.
    #[test]
    fn declares_the_handle_interaction_domain_scoped_to_the_board_window() {
        let definition = create_block2d_app();
        let interaction = definition.interactions.iter().find(|def| def.id == BLOCK2D_INTERACTION_HANDLE).expect("handle domain declared");
        assert_eq!(interaction.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec![BLOCK2D_GRANULARITY_HANDLE, BLOCK2D_GRANULARITY_HANDLE_KIND]);
        assert!(matches!(interaction.hierarchy, HierarchyProvider::Topology));
        let board_window = definition.window_kinds.iter().find(|window| window.id == board::BLOCK2D_WINDOW_BOARD).expect("board window declared");
        assert!(board_window.interactions.contains(&InteractionRef::new(BLOCK2D_INTERACTION_HANDLE)));
    }

    /// 🕹️ `interaction_topology` nests every handle under its own handle-kind, enabling both pruning
    /// (`removeHandleKind`/`removeHandle`) and transitive hover from a kind to its handles.
    #[test]
    fn interaction_topology_nests_handles_under_their_handle_kind() {
        let mut app: Block2dApp = new_app();
        testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {}));
        testkit::dispatch(&mut app, Block2dCommand::AddHandle(add_handle::AddHandle {}));
        let snapshot = app.snapshot().expect("snapshot");
        let kind_id = snapshot.handle_kinds[0].id.clone();
        let handle_id = snapshot.handles[0].id.clone();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Block2dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let topology = Block2dPlayApp::interaction_topology(&doc, &cfg);
        let domain = topology.domains.get(BLOCK2D_INTERACTION_HANDLE).expect("handle domain topology present");
        let handle_node = domain.ordered.iter().find(|node| node.id == format!("handle:{handle_id}")).expect("handle node present");
        assert_eq!(handle_node.parent.as_deref(), Some(format!("handleKind:{kind_id}").as_str()));
    }

    #[test]
    fn block2d_io_is_wired_into_the_manifest() {
        let definition = create_block2d_app();
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    #[test]
    fn block2d_io_declares_the_catalog_out_port() {
        let io = block2d_io();
        assert_eq!(io.document_schema, BLOCK_2D_SCHEMA);
        let ports = io.all_ports();
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, MediaPortDirection::Out);
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
        assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), 1);
        testkit::dispatch(&mut app, Block2dCommand::AddHandle(add_handle::AddHandle {}));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.handles.len(), 1);
        let handle_id = projection.handles[0].id.clone();
        testkit::dispatch(&mut app, Block2dCommand::RemoveHandle(remove_handle::RemoveHandle { id: handle_id }));
        assert_eq!(app.snapshot().expect("snapshot").handles.len(), 0);
    }

    #[test]
    fn patch_node_kind_updates_name() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::PatchNodeKind(patch_node_kind::PatchNodeKind { field: "name".into(), value: "Renamed".into() }));
        assert_eq!(app.snapshot().expect("snapshot").node_kind.name, "Renamed");
    }

    #[test]
    fn set_active_example_loads_left_fixture() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK2D_EXAMPLE_LEFT.into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.node_kind.id, "Hexagonal Cut Concrete Forest Left");
        assert_eq!(projection.handles.len(), 11);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {}));
        assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), 0);
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("snapshot").handle_kinds.len(), 1);
    }

    /// 🌉️ `puzzle2d_manifest_fragment`'s new caller round-trips through the `"catalog:out"` media port.
    #[test]
    fn export_media_catalog_out_wraps_the_puzzle2d_fragment() {
        let mut app = new_app();
        testkit::dispatch(&mut app, Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK2D_EXAMPLE_LEFT.into() }));
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
        let _app = Block2dPlayApp;
        assert!(matches!(Block2dPlayApp::command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": "left" }))), Ok(Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "left"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the old `setSelection` view
    /// action this test exercised is gone — 2d now declares zero app-level view actions (selection
    /// moved to the framework-owned `handle` domain, an `ActionKind::Interaction` verb entirely
    /// outside `Block2dCommand`/kind-discipline dispatch; that domain declaration itself is asserted
    /// by `declares_the_handle_interaction_domain_scoped_to_the_board_window` above).
    /// `app_with_registry` still earns its keep here: a genuine `Mutation`-kind command must still
    /// emit document operations under the real, kind-discipline-enforcing registry.
    #[test]
    fn mutation_commands_still_emit_artifact_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {}));
        assert!(!result.mutations.is_empty(), "addHandleKind is a mutation and must reach document operations under kind discipline");
    }
    //#endregion 🔖️Behavior
}
//#endregion 🧪️Tests
