//! 👯️ Block 5D play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the board/world
//! windows in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`,
//! view state in `🦀️config.rs`, document-side compute in `crate::artifacts::block5d::schema`/
//! `crate::artifacts::block5d::schema::inferences`, and this app's own typed media I/O surface (below —
//! constitutional: general, an artifact must never depend on an app, so it lives here rather than under
//! `🗿️artifacts`).

use crate::apps::block5d::commands::{edit, set_active_example};
use crate::apps::block5d::commands::{add_grip, remove_grip};
use crate::apps::block5d::commands::{add_grip_kind, remove_grip_kind};
use crate::apps::block5d::commands::patch_part_kind;
use crate::apps::block5d::config::{Block5dConfig, Block5dConfigMutation};
use crate::apps::block5d::modes::edit as edit_mode;
use crate::apps::block5d::modes::edit::windows::{board, world};
use crate::apps::block5d::panels::{document as document_panel, inspection as inspection_panel};
use crate::apps::block5d::terminology::block5d_labels;
use crate::artifacts::block5d::op::Block5dMutation;
use crate::artifacts::block5d::{artifact_kind, Block5dSnapshot, BLOCK_5D_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    ActionDescriptor, App, ArtifactKindSpec, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode,
};
use semio_framework_plugin::app::InteractionView;
use semio_framework::{DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, TopologyNode};
use store::EngineHandles;
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Constants
pub const BLOCK5D_PLAY_APP_ID: &str = "block5d-play";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the framework-owned hover/selection
/// domain over this app's rim-grip templates ("grip" granularity, the default) and grip-kind catalog
/// ("gripKind" granularity) — replaces the deleted `Block5dConfig.selected_ids`.
pub const BLOCK5D_INTERACTION_GRIP: &str = "grip";
pub const BLOCK5D_GRANULARITY_GRIP: &str = "grip";
pub const BLOCK5D_GRANULARITY_GRIP_KIND: &str = "gripKind";
/// 🗂️ The `s/plugin/puzzle` 5d catalog artifact kind block5d's `"catalog:out"` port produces — see
/// `block5d_io` and `Block5dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn block5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(BLOCK5D_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ `Block5dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"5d.block"` artifact kind) plus a `"catalog:out"` port giving
/// `puzzle5d_catalog_fragment` a real caller (see `export_media` below).
pub fn block5d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo::from_document(
        BLOCK_5D_SCHEMA,
        semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        semio_framework_plugin::ArtifactPresentation { id: "5d.block".into(), name: "Part Kind".into(), dimension: "5d".into(), component_kind: "block5d".into() },
    )
    .with_ports(vec![semio_framework_plugin::MediaPortSpec {
        id: "catalog:out".into(),
        label: "Kit Catalog".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Kit, form: semio_framework_plugin::MediaForm::Type },
        kind_id: Some("kit.catalog".into()),
        required: false,
        multiplicity: semio_framework_plugin::PortMultiplicity::Many,
    }])
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Block5dPlayApp::Command` — the SOLE dispatch surface for block5d's own behavior, covering
    /// every action `create_block5d_app` declares. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. Every id/key pair here is IDENTICAL (the pre-migration
    /// `#[dsl(key)]` already used the camelCase action id, not kebab-case) — preserved verbatim.
    pub enum Block5dCommand for Block5dSnapshot, Block5dMutation, Block5dConfig, Block5dConfigMutation {
        "patchPartKind" as "patchPartKind" => patch_part_kind::PatchPartKind,
        "addGripKind" as "addGripKind" => add_grip_kind::AddGripKind,
        "removeGripKind" as "removeGripKind" => remove_grip_kind::RemoveGripKind,
        "addGrip" as "addGrip" => add_grip::AddGrip,
        "removeGrip" as "removeGrip" => remove_grip::RemoveGrip,
        "setActiveExample" as "setActiveExample" => set_active_example::SetActiveExample,
        "edit" as "edit" => edit::Edit,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Block5dPlayApp
/// 🧪️ B1: unit struct — the former `selected_ids` `RefCell` field now lives in
/// `crate::apps::block5d::config::Block5dConfig`, written through `Block5dConfigMutation`s.
#[derive(Default)]
pub struct Block5dPlayApp;

impl ArtifactApp for Block5dPlayApp {
    type Snapshot = Block5dSnapshot;
    type Mutation = Block5dMutation;
    type Config = Block5dConfig;
    type ConfigMutation = Block5dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::apps::block5d::presence::Block5dPresence;
    type PresenceMutation = crate::apps::block5d::presence::Block5dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Block5dCommand;

    const APP_ID: &'static str = BLOCK5D_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_5D_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::apps::block5d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Block5dSnapshot {
        crate::artifacts::block5d::schema::empty_block5d_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(block5d_io())
    }

    fn command_id(command: &Block5dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block5dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "patchPartKind" => Ok(Block5dCommand::PatchPartKind(patch_part_kind::PatchPartKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            "addGripKind" => Ok(Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})),
            "removeGripKind" => Ok(Block5dCommand::RemoveGripKind(remove_grip_kind::RemoveGripKind { id: str_field("id").unwrap_or_default() })),
            "addGrip" => Ok(Block5dCommand::AddGrip(add_grip::AddGrip {})),
            "removeGrip" => Ok(Block5dCommand::RemoveGrip(remove_grip::RemoveGrip { id: str_field("id").unwrap_or_default() })),
            "setActiveExample" => Ok(Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() })),
            "edit" => Ok(Block5dCommand::Edit(edit::Edit { text: str_field("text").unwrap_or_default() })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(command: &Block5dCommand, doc: &ArtifactView<'_, Block5dSnapshot>, cfg: &ConfigView<'_, Block5dConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Block5dMutation, Block5dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `grip` domain's
    /// `HierarchyProvider::Topology` — every grip-kind is a root (`gripKind` granularity), every grip
    /// nests under its own `grip_kind` (`grip` granularity), so a stale selection is pruned the moment
    /// `removeGripKind`/`removeGrip` deletes its target, and hovering/selecting a kind can transitively
    /// reach its grips.
    fn interaction_topology(doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> InteractionTopology {
        let mut ordered: Vec<TopologyNode> = Vec::new();
        for kind in &doc.snapshot.grip_kinds {
            ordered.push(TopologyNode { id: format!("gripKind:{}", kind.id), granularity: BLOCK5D_GRANULARITY_GRIP_KIND.into(), parent: None });
        }
        for grip in &doc.snapshot.grips {
            ordered.push(TopologyNode { id: format!("grip:{}", grip.id), granularity: BLOCK5D_GRANULARITY_GRIP.into(), parent: Some(format!("gripKind:{}", grip.grip_kind)) });
        }
        let mut domains = BTreeMap::new();
        domains.insert(BLOCK5D_INTERACTION_GRIP.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Block5dSnapshot>, cfg: &ConfigView<'_, Block5dConfig>) -> UiNode {
        let labels = block5d_labels(&cfg.snapshot.locale);
        match body_key {
            board::BLOCK5D_BODY_BOARD => board::render(doc.snapshot, labels),
            world::BLOCK5D_BODY_WORLD => world::render(doc.snapshot, labels),
            document_panel::BLOCK5D_BODY_DOCUMENT => document_panel::render(doc.snapshot, labels),
            inspection_panel::BLOCK5D_BODY_INSPECTOR => inspection_panel::render(doc.snapshot, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    /// 🌉️ `puzzle5d_catalog_fragment`'s first real caller — wraps the block-5d document's
    /// puzzle5d-shaped catalog fragment (`parts`/`grips`/`fasteners`/`ropes`/`kindCompatibility`) as
    /// a `kit.catalog`-schema `Media` value for the `"catalog:out"` port declared in `block5d_io`.
    /// Falls through to the default whole-document pack export for every other port
    /// (`"document:out"`).
    fn export_media(port: &str, doc: &ArtifactView<'_, Block5dSnapshot>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `ArtifactApp::export_media`'s default `"document:out"` behavior
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
        let fragment = crate::artifacts::block5d::schema::inferences::puzzle5d_catalog_fragment(doc.snapshot);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block5dPlayApp

//#region 🔖️Manifest
pub fn create_block5d_app() -> App {
    App::from_builder(
        App::builder(BLOCK5D_PLAY_APP_ID, LocalizedLabel::native("Block 5D", "Block 5D"))
            .document(["semio", "block", "5d"])
            .artifact_kind(artifact_kind())
            // 🗂️ The puzzle5d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block5d_io`/`Block5dPlayApp::export_media`.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "5d".into(),
                media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("layers")
            .mode_def(edit_mode::definition())
            .default_mode_id(edit_mode::BLOCK5D_PLAY_MODE_EDIT)
            .window_kind_def(board::definition())
            .window_kind_def(world::definition())
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `grip` domain
            // replaces the deleted `setSelection` view action — the framework auto-injects
            // `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
            // `setInteractionGranularity` for it.
            .interaction(InteractionDefinition {
                id: BLOCK5D_INTERACTION_GRIP.into(),
                label: LocalizedLabel::native("Grips", "Griffe"),
                granularities: vec![
                    GranularityDefinition { id: BLOCK5D_GRANULARITY_GRIP.into(), label: LocalizedLabel::native("Grip", "Griff"), icon_id: "circle-dot".into() },
                    GranularityDefinition { id: BLOCK5D_GRANULARITY_GRIP_KIND.into(), label: LocalizedLabel::native("Grip Kind", "Griffart"), icon_id: "circle".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace, MergeMode::Additive], transitive: false, broadcast: true },
            })
            .window_kind_interactions(board::BLOCK5D_WINDOW_BOARD, vec![InteractionRef::new(BLOCK5D_INTERACTION_GRIP)])
            .window_kind_interactions(world::BLOCK5D_WINDOW_WORLD, vec![InteractionRef::new(BLOCK5D_INTERACTION_GRIP)])
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("patchPartKind", LocalizedLabel::native("Patch Part Kind", "Teilart bearbeiten"))
            .mutation("addGripKind", LocalizedLabel::native("Add Grip Kind", "Griffart hinzufügen"))
            .mutation("removeGripKind", LocalizedLabel::native("Remove Grip Kind", "Griffart entfernen"))
            .mutation("addGrip", LocalizedLabel::native("Add Grip", "Griff hinzufügen"))
            .mutation("removeGrip", LocalizedLabel::native("Remove Grip", "Griff entfernen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .default_layout(edit_mode::layout())
            .io(block5d_io()),
    )
    .example(
        crate::apps::block5d::commands::set_active_example::BLOCK5D_EXAMPLE_FOREST_LEFT,
        LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
        serde_json::to_string(&crate::artifacts::block5d::dsl::parse_dsl(crate::artifacts::block5d::dsl::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "list-tree",
    )
    .example(
        crate::apps::block5d::commands::set_active_example::BLOCK5D_EXAMPLE_CAPSULE,
        LocalizedLabel::native("Nakagin Capsule", "Nakagin Capsule"),
        serde_json::to_string(&crate::artifacts::block5d::dsl::parse_dsl(crate::artifacts::block5d::dsl::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).unwrap_or_default()).unwrap_or_default(),
        "building",
    )
    .workflow("block5d", "Block 5D", "model")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Block5dApp = VcsArtifactApp<Block5dPlayApp>;

    pub fn new_app() -> Block5dApp {
        sdk_new_app::<Block5dPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> Block5dApp {
        new_app_with_registry::<Block5dPlayApp>(create_block5d_app)
    }

    pub fn dispatch(app: &mut Block5dApp, command: Block5dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Block5dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::block5d::testkit::{new_app, Block5dApp};
    use semio_framework_plugin::PluginApp;


    //#region 🔖️CommandSurface
    fn every_command() -> Vec<Block5dCommand> {
        vec![
            Block5dCommand::PatchPartKind(patch_part_kind::PatchPartKind { field: "name".into(), value: "x".into() }),
            Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {}),
            Block5dCommand::RemoveGripKind(remove_grip_kind::RemoveGripKind { id: "g0".into() }),
            Block5dCommand::AddGrip(add_grip::AddGrip {}),
            Block5dCommand::RemoveGrip(remove_grip::RemoveGrip { id: "g0".into() }),
            Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id: "forest".into() }),
            Block5dCommand::Edit(edit::Edit { text: "{}".into() }),
        ]
    }

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Block5dCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 7, "every Block5dCommand row must be covered by every_command()");
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
    /// silently rewritten — copied verbatim from the ticket's `🧪️wire-baseline-5d-before.txt`.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Block5dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(hex(&Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})), "01010000");
        assert_eq!(hex(&Block5dCommand::AddGrip(add_grip::AddGrip {})), "01030000");
    }

    /// 🌉️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`.
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<Block5dPlayApp>(create_block5d_app);
        assert!(Block5dPlayApp::command_from_action("noSuchAction", None).is_err());
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_block5d_app().definition;
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 2);
        for body_key in [document_panel::BLOCK5D_BODY_DOCUMENT, inspection_panel::BLOCK5D_BODY_INSPECTOR] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `grip` domain is declared
    /// once, with both granularities, a `Topology` hierarchy, and scoped to both window kinds.
    #[test]
    fn declares_the_grip_interaction_domain_scoped_to_both_windows() {
        let definition = create_block5d_app().definition;
        let interaction = definition.interactions.iter().find(|def| def.id == BLOCK5D_INTERACTION_GRIP).expect("grip domain declared");
        assert_eq!(interaction.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec![BLOCK5D_GRANULARITY_GRIP, BLOCK5D_GRANULARITY_GRIP_KIND]);
        assert!(matches!(interaction.hierarchy, HierarchyProvider::Topology));
        for window_id in [board::BLOCK5D_WINDOW_BOARD, world::BLOCK5D_WINDOW_WORLD] {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap_or_else(|| panic!("window {window_id} declared"));
            assert!(window.interactions.contains(&InteractionRef::new(BLOCK5D_INTERACTION_GRIP)), "window {window_id} scoped to the grip domain");
        }
    }

    /// 🕹️ `interaction_topology` nests every grip under its own grip-kind, enabling both pruning
    /// (`removeGripKind`/`removeGrip`) and transitive hover from a kind to its grips.
    #[test]
    fn interaction_topology_nests_grips_under_their_grip_kind() {
        let mut app: Block5dApp = new_app();
        testkit::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {}));
        testkit::dispatch(&mut app, Block5dCommand::AddGrip(add_grip::AddGrip {}));
        let snapshot = app.snapshot().expect("snapshot");
        let kind_id = snapshot.grip_kinds[0].id.clone();
        let grip_id = snapshot.grips[0].id.clone();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Block5dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let topology = Block5dPlayApp::interaction_topology(&doc, &cfg);
        let domain = topology.domains.get(BLOCK5D_INTERACTION_GRIP).expect("grip domain topology present");
        let grip_node = domain.ordered.iter().find(|node| node.id == format!("grip:{grip_id}")).expect("grip node present");
        assert_eq!(grip_node.parent.as_deref(), Some(format!("gripKind:{kind_id}").as_str()));
    }

    #[test]
    fn block5d_io_declares_the_catalog_out_port() {
        let io = block5d_io();
        assert_eq!(io.document_schema, BLOCK_5D_SCHEMA);
        let ports = io.all_ports();
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = new_app();
        assert!(testkit::render(&mut app, "block5d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Behavior
    #[test]
    fn renders_document_tree_board_and_world() {
        let mut app: Block5dApp = new_app();
        assert!(testkit::render(&mut app, document_panel::BLOCK5D_BODY_DOCUMENT).contains("Grip Kinds"));
        assert!(testkit::render(&mut app, board::BLOCK5D_BODY_BOARD).contains("2d grips"));
        assert!(testkit::render(&mut app, world::BLOCK5D_BODY_WORLD).contains("mesh:"));
    }

    #[test]
    fn add_grip_kind_then_add_grip_then_remove_round_trips() {
        let mut app: Block5dApp = new_app();
        testkit::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {}));
        testkit::dispatch(&mut app, Block5dCommand::AddGrip(add_grip::AddGrip {}));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.grips.len(), 1);
        let grip_id = projection.grips[0].id.clone();
        testkit::dispatch(&mut app, Block5dCommand::RemoveGrip(remove_grip::RemoveGrip { id: grip_id }));
        assert_eq!(app.snapshot().expect("snapshot").grips.len(), 0);
    }

    #[test]
    fn set_active_example_loads_forest_left_fixture() {
        let mut app: Block5dApp = new_app();
        testkit::dispatch(&mut app, Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id: crate::apps::block5d::commands::set_active_example::BLOCK5D_EXAMPLE_FOREST_LEFT.into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.part_kind.id, "Hexagonal Cut Concrete Forest Left");
        assert_eq!(projection.grips.len(), 1);
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app: Block5dApp = new_app();
        testkit::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {}));
        assert_eq!(app.snapshot().expect("snapshot").grip_kinds.len(), 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.snapshot().expect("snapshot").grip_kinds.len(), 0);
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("snapshot").grip_kinds.len(), 1);
    }

    /// 🌉️ `puzzle5d_catalog_fragment`'s new caller round-trips through the `"catalog:out"` media port.
    #[test]
    fn export_media_catalog_out_wraps_the_puzzle5d_fragment() {
        let mut app: Block5dApp = new_app();
        testkit::dispatch(&mut app, Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id: crate::apps::block5d::commands::set_active_example::BLOCK5D_EXAMPLE_FOREST_LEFT.into() }));
        let media = app.export_media("catalog:out").expect("export catalog");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let value: Value = serde_json::from_str(&json).expect("valid json");
                assert_eq!(value["parts"][0]["id"], "Hexagonal Cut Concrete Forest Left");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[test]
    fn command_from_action_bridges_set_active_example() {
        let app = Block5dPlayApp;
        assert!(matches!(Block5dPlayApp::command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": "forest" }))), Ok(Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "forest"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the old `setSelection` view
    /// action this test exercised is gone — 5d now declares zero app-level view actions (selection
    /// moved to the framework-owned `grip` domain, an `ActionKind::Interaction` verb entirely outside
    /// `Block5dCommand`/kind-discipline dispatch; that domain declaration itself is asserted by
    /// `declares_the_grip_interaction_domain_scoped_to_both_windows` above). `app_with_registry`
    /// still earns its keep here: a genuine `Mutation`-kind command must still emit document
    /// operations under the real, kind-discipline-enforcing registry.
    #[test]
    fn mutation_commands_still_emit_artifact_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {}));
        assert!(!result.mutations.is_empty(), "addGripKind is a mutation and must reach document operations under kind discipline");
    }
    //#endregion 🔖️Behavior
}
//#endregion 🧪️Tests
