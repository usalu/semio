pub(crate) mod context {
    
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    /// ✏️ `Block5dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Block5dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Block5dPlayApp>` builds it.
    pub type Block5dApp = VcsArtifactApp<EditorApp<Block5dPlayApp>>;
    
    pub async fn new_app() -> Block5dApp {
        app_with_registry().await
    }
    
    /// ✏️ Adapts `create_block5d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`
    /// still expect — framework test context gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub fn block5d_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_block5d_app(), examples: Vec::new() }
    }
    
    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub async fn app_with_registry() -> Block5dApp {
        new_app_with_registry::<EditorApp<Block5dPlayApp>>(block5d_app_manifest_for_tests).await
    }
    
    pub async fn dispatch(app: &mut Block5dApp, command: Block5dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(app: &mut Block5dApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
    }
}


use super::*;
use crate::editor::block5d::unit_tests::context::{Block5dApp, new_app};
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

#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(Block5dCommand::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 7, "every Block5dCommand row must be covered by every_command()");
}

#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = protocol::OpText::print_op(&command);
        assert!(printed.starts_with(command.command_id()), "row {} printed {printed:?}", command.command_id());
    }
}

/// 🧷️ Pins the exact pre-migration bytes for the rows the `app_commands!` decomposition could have
/// silently rewritten — copied verbatim from the ticket's `🧪️wire-baseline-5d-before.txt`.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &Block5dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
    assert_eq!(hex(&Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})), "01010000");
    assert_eq!(hex(&Block5dCommand::AddGrip(add_grip::AddGrip {})), "01030000");
}

/// 🌉️ Every app-declared action must bridge through `command_from_action` and round-trip
/// `command_id`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    semio_framework_plugin::artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Block5dPlayApp>>(context::block5d_app_manifest_for_tests).await;
    assert!(<Block5dPlayApp as ArtifactEditor>::command_from_action("noSuchAction", None).is_err());
}
//#endregion 🔖️CommandSurface

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_block5d_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 2);
    for body_key in [document_panel::BLOCK5D_BODY_DOCUMENT, inspection_panel::BLOCK5D_BODY_INSPECTOR] {
        assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
    }
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `grip` domain is declared
/// once, with both granularities, a `Topology` hierarchy, and scoped to both window kinds.
#[semio_framework_async_macros::async_test]
async fn declares_the_grip_interaction_domain_scoped_to_both_windows() {
    let definition = create_block5d_app();
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
#[semio_framework_async_macros::async_test]
async fn interaction_topology_nests_grips_under_their_grip_kind() {
    let mut app: Block5dApp = new_app().await;
    context::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})).await;
    context::dispatch(&mut app, Block5dCommand::AddGrip(add_grip::AddGrip {})).await;
    let snapshot = app.snapshot().expect("snapshot");
    let kind_id = snapshot.grip_kinds[0].id.clone();
    let grip_id = snapshot.grips[0].id.clone();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = Block5dConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let topology = Block5dPlayApp::interaction_topology(&doc, &cfg);
    let domain = topology.domains.get(BLOCK5D_INTERACTION_GRIP).expect("grip domain topology present");
    let grip_node = domain.ordered.iter().find(|node| node.id == format!("grip:{grip_id}")).expect("grip node present");
    assert_eq!(grip_node.parent.as_deref(), Some(format!("gripKind:{kind_id}").as_str()));
}

#[semio_framework_async_macros::async_test]
async fn block5d_io_declares_the_catalog_out_port() {
    let io = block5d_io();
    assert_eq!(io.document_schema, BLOCK_5D_SCHEMA);
    let ports = io.all_ports().await;
    let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
    assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = new_app().await;
    assert!(context::render(&mut app, "block5d.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️Manifest

//#region 🔖️Behavior
#[semio_framework_async_macros::async_test]
async fn renders_document_tree_board_and_world() {
    let mut app: Block5dApp = new_app().await;
    assert!(context::render(&mut app, document_panel::BLOCK5D_BODY_DOCUMENT).await.contains("Grip Kinds"));
    assert!(context::render(&mut app, board::BLOCK5D_BODY_BOARD).await.contains("2d grips"));
    assert!(context::render(&mut app, world::BLOCK5D_BODY_WORLD).await.contains("mesh:"));
}

#[semio_framework_async_macros::async_test]
async fn add_grip_kind_then_add_grip_then_remove_round_trips() {
    let mut app: Block5dApp = new_app().await;
    let booted = app.snapshot().expect("snapshot");
    let (kinds, grips) = (booted.grip_kinds.len(), booted.grips.len());
    let booted_ids: Vec<String> = booted.grips.iter().map(|grip| grip.id.clone()).collect();
    context::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})).await;
    assert_eq!(app.snapshot().expect("snapshot").grip_kinds.len(), kinds + 1);
    context::dispatch(&mut app, Block5dCommand::AddGrip(add_grip::AddGrip {})).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.grips.len(), grips + 1);
    let grip_id = projection.grips.iter().map(|grip| grip.id.clone()).find(|id| !booted_ids.contains(id)).expect("the added grip");
    context::dispatch(&mut app, Block5dCommand::RemoveGrip(remove_grip::RemoveGrip { id: grip_id })).await;
    assert_eq!(app.snapshot().expect("snapshot").grips.len(), grips);
}

/// 📄️ The app boots on a real document, so every window renders content before the first action.
#[semio_framework_async_macros::async_test]
async fn boots_on_the_forest_left_example_document() {
    let mut app: Block5dApp = new_app().await;
    let booted = app.snapshot().expect("snapshot");
    assert_ne!(booted, crate::standards::v1::subsets::any::schema::empty_block5d_snapshot());
    assert_eq!(booted.part_kind.label, "Hexagonal Cut Concrete Forest Left");
    assert!(booted.representations.first().and_then(|representation| representation.mesh_url.as_deref()).is_some());
    assert!(!booted.grip_kinds.is_empty());
    assert!(context::render(&mut app, board::BLOCK5D_BODY_BOARD).await.contains("Hexagonal Cut Concrete Forest Left"));
    assert!(context::render(&mut app, world::BLOCK5D_BODY_WORLD).await.contains("hexagonal-cut-concrete-forest-left.glb"));
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_forest_left_fixture() {
    let mut app: Block5dApp = new_app().await;
    context::dispatch(&mut app, Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK5D_EXAMPLE_FOREST_LEFT.into() })).await;
    let projection = app.snapshot().expect("snapshot");
    assert_eq!(projection.part_kind.id, "Hexagonal Cut Concrete Forest Left");
    assert_eq!(projection.grips.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app: Block5dApp = new_app().await;
    let kinds = app.snapshot().expect("snapshot").grip_kinds.len();
    context::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})).await;
    assert_eq!(app.snapshot().expect("snapshot").grip_kinds.len(), kinds + 1);
    app.handle_action("undo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("undo");
    assert_eq!(app.snapshot().expect("snapshot").grip_kinds.len(), kinds);
    app.handle_action("redo", None, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("redo");
    assert_eq!(app.snapshot().expect("snapshot").grip_kinds.len(), kinds + 1);
}

/// 🌉️ `puzzle5d_catalog_fragment`'s new caller round-trips through the `"catalog:out"` media port.
#[semio_framework_async_macros::async_test]
async fn export_media_catalog_out_wraps_the_puzzle5d_fragment() {
    let mut app: Block5dApp = new_app().await;
    context::dispatch(&mut app, Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK5D_EXAMPLE_FOREST_LEFT.into() })).await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("catalog:out")).expect("export catalog");
    assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "kit.catalog");
            let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
            assert_eq!(value["parts"][0]["id"], "Hexagonal Cut Concrete Forest Left");
        }
        other => panic!("expected Structured payload, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn command_from_action_bridges_set_active_example() {
    let _app = Block5dPlayApp;
    assert!(
        matches!(<Block5dPlayApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&dsl::json::to_dsl_value(&dsl::json!({ "exampleId": "forest" })))), Ok(Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "forest")
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the old `setSelection` view
/// action this test exercised is gone — 5d now declares zero app-level view actions (selection
/// moved to the framework-owned `grip` domain, an `ActionKind::Interaction` verb entirely outside
/// `Block5dCommand`/kind-discipline dispatch; that domain declaration itself is asserted by
/// `declares_the_grip_interaction_domain_scoped_to_both_windows` above). `app_with_registry`
/// still earns its keep here: a genuine `Mutation`-kind command must still emit document
/// operations under the real, kind-discipline-enforcing registry.
#[semio_framework_async_macros::async_test]
async fn mutation_commands_still_emit_artifact_mutations_under_the_real_registry() {
    let mut app = context::app_with_registry().await;
    let result = context::dispatch(&mut app, Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})).await;
    assert!(!result.mutations.is_empty(), "addGripKind is a mutation and must reach document operations under kind discipline");
}
//#endregion 🔖️Behavior
