pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{ActionMeta, App, EditorApp, Fault, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    pub type NoteApp = VcsArtifactApp<EditorApp<NotePlayApp>>;

    /// 🧪️ SDK gap (contract §2.4/§7.4 handoff): `context::new_app_with_registry`'s signature is still
    /// `fn(manifest: fn() -> App)`, not yet updated for the `AppDefinition`-returning `create_note_app`
    /// convention — this tiny local wrapper adapts it back into the `App { definition, examples }`
    /// shape that fn still expects (mirrors trinity/jack's `trinity_jack_manifest_for_tests`, the
    /// first real W2 packet to hit this exact gap).
    fn note_manifest_for_tests() -> App {
        App { definition: create_note_app(), examples: Vec::new() }
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn note_app() -> NoteApp {
        let mut app = new_app::<EditorApp<NotePlayApp>>().await;
        app.bind_instance_id(1).await;
        app
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn note_app_with_registry() -> NoteApp {
        note_app_with_registry_id(1).await
    }

    /// 🪪️ A registry-backed app with an exact runtime identity for parallel ownership laws.
    pub async fn note_app_with_registry_id(instance_id: u32) -> NoteApp {
        let mut app = new_app_with_registry::<EditorApp<NotePlayApp>>(note_manifest_for_tests).await;
        app.bind_instance_id(instance_id).await;
        app
    }

    pub fn composite_view(id: &str) -> ViewModel {
        ViewModel {
            window_instances: vec![
                ViewWindowInstance { id: id.into(), window_kind_id: NOTE_PLAY_WINDOW_COMPOSITE.into() },
                ViewWindowInstance { id: NOTE_PLAY_WINDOW_NAVIGATOR.into(), window_kind_id: NOTE_PLAY_WINDOW_NAVIGATOR.into() },
            ],
            ..Default::default()
        }
        .for_window_instance(id)
        .expect("Note composite test window roster")
    }

    fn action_meta(actor: &str, instance_id: u32, view_state: ViewModel) -> ActionMeta {
        ActionMeta { instance_id, view_state: Some(view_state), ..meta(actor) }
    }

    /// 📬️ Advances and consumes one host turn at a time, including the mandatory completion
    /// witness counted by `has_pending_typed_operations` between the event and UI scope channels.
    async fn settle(app: &mut NoteApp, receiver: u32, result: Result<InvocationResult, Fault>) -> Result<InvocationResult, Fault> {
        let mut result = result?;
        for _ in 0..1_048_576 {
            if !app.has_pending_typed_operations() {
                return Ok(result);
            }
            PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
            app.advance_typed_operation_publication().await?;
            if let Some(page) = app.take_typed_operation_result_page(receiver) {
                if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                    return Err(Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
                }
                app.acknowledge_typed_operation_result(page.token)?;
            }
            result.requested_effects.extend(app.take_typed_operation_effect());
            result.events.extend(app.take_typed_operation_event());
            let _ = app.take_typed_operation_completion().await?;
            if let Some(scope) = app.take_typed_operation_ui_scope() {
                result.ui_scope = scope;
            }
        }
        Err(Fault::from("Note test operation did not settle"))
    }

    pub async fn dispatch(app: &mut NoteApp, command: NoteCommand) -> InvocationResult {
        dispatch_with_view(app, command, composite_view(NOTE_PLAY_WINDOW_COMPOSITE)).await
    }

    pub async fn dispatch_with_view(app: &mut NoteApp, command: NoteCommand, view_state: ViewModel) -> InvocationResult {
        dispatch_with_view_for_instance(app, command, 1, view_state).await
    }

    pub async fn dispatch_with_view_for_instance(app: &mut NoteApp, command: NoteCommand, instance_id: u32, view_state: ViewModel) -> InvocationResult {
        let result = app.dispatch_typed(command, &action_meta("local", instance_id, view_state)).await;
        settle(app, instance_id, result).await.expect("dispatch")
    }

    pub async fn render(app: &mut NoteApp, body_key: &str) -> String {
        render_with_view(app, body_key, &composite_view(NOTE_PLAY_WINDOW_COMPOSITE)).await
    }

    pub async fn render_with_view(app: &mut NoteApp, body_key: &str, view_state: &ViewModel) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb, dispatched against the "blocks" domain declared on this app —
    /// requires `note_app_with_registry()` (a bare `note_app()` has no declared interaction domains to
    /// select against). `ids` are raw block ids, converted to the row-id-prefixed `InteractionTarget`
    /// id the document panel tree/`interaction_topology` both use (see `note_blocks_topology`'s doc
    /// comment).
    pub async fn select_blocks(app: &mut NoteApp, ids: &[&str]) {
        let target_list: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": "block", "id": format!("note-play-block:{id}") })).collect();
        let targets = serde_json::to_string(&target_list).expect("targets json");
        let args = semio_framework_plugin::optional_json_to_dsl(Some(serde_json::json!({ "domainId": NOTE_INTERACTION_BLOCKS, "targets": targets, "merge": "replace" })));
        let result = app.handle_action("interactionSelect", args.as_ref(), &action_meta("test", 1, composite_view(NOTE_PLAY_WINDOW_COMPOSITE))).await;
        settle(app, 1, result).await.expect("interactionSelect");
    }

    pub async fn close_app(app: &mut NoteApp) {
        for _ in 0..1_048_576 {
            if app.close_terminal_is_empty() {
                return;
            }
            if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Note registered app close") == semio_framework_plugin::PluginCloseStep::Complete {
                break;
            }
        }
        assert!(app.close_terminal_is_empty(), "Note registered app close did not reach terminal-empty ownership");
    }
}

use super::*;
use crate::editor::note::unit_tests::context::note_app;
use crate::editor::note::window;
use semio_framework_plugin::{artifact_app_laws, ActionKind as Kind, PluginApp};

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
/// hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_across_every_row() {
    let ids: Vec<&str> = every_command().iter().map(|command| NotePlayApp::command_id(command)).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 34, "every NoteCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<NoteCommand> {
    vec![
        NoteCommand::SetGridVisible(set_grid_visible::SetGridVisible { value: Some(true) }),
        NoteCommand::SetGridSpacing(set_grid_spacing::SetGridSpacing { value: 16.0 }),
        NoteCommand::SetGridSubdivisions(set_grid_subdivisions::SetGridSubdivisions { value: 8.0 }),
        NoteCommand::SetGridOpacity(set_grid_opacity::SetGridOpacity { value: 0.6 }),
        NoteCommand::SetSnapEnabled(set_snap_enabled::SetSnapEnabled { value: Some(false) }),
        NoteCommand::SetSnapGridSpacing(set_snap_grid_spacing::SetSnapGridSpacing { value: 4.0 }),
        NoteCommand::SetPencilWidth(set_pencil_width::SetPencilWidth { value: 5.0 }),
        NoteCommand::SetEraserRadius(set_eraser_radius::SetEraserRadius { value: 20.0 }),
        NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 10.0, y: 20.0 }),
        NoteCommand::MoveBlock(move_block::MoveBlock { block_id: "b1".into(), target_row_id: "note-play-block:b2".into(), drop_position: "after".into() }),
        NoteCommand::DeleteBlock(delete_block::DeleteBlock { block_id: "b1".into() }),
        NoteCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        NoteCommand::DuplicateBlock(duplicate_block::DuplicateBlock { block_id: "b1".into() }),
        NoteCommand::DuplicateSelection(duplicate_selection::DuplicateSelection {}),
        NoteCommand::PatchBlocks(patch_blocks::PatchBlocks { block_ids: vec!["b1".into()], field: "name".into(), value: "Renamed".into() }),
        NoteCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "semio".into() }),
        NoteCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{\"schema\":\"note.document\"}".into() }),
        NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None }),
        NoteCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("Renamed".into()) }),
        NoteCommand::NudgeSelection(nudge_selection::NudgeSelection { dx: 1.0, dy: -1.0 }),
        NoteCommand::NudgeSelectionUp(nudge_selection_up::NudgeSelectionUp {}),
        NoteCommand::NudgeSelectionDown(nudge_selection_down::NudgeSelectionDown {}),
        NoteCommand::NudgeSelectionLeft(nudge_selection_left::NudgeSelectionLeft {}),
        NoteCommand::NudgeSelectionRight(nudge_selection_right::NudgeSelectionRight {}),
        NoteCommand::NudgeSelectionUpFast(nudge_selection_up_fast::NudgeSelectionUpFast {}),
        NoteCommand::NudgeSelectionDownFast(nudge_selection_down_fast::NudgeSelectionDownFast {}),
        NoteCommand::NudgeSelectionLeftFast(nudge_selection_left_fast::NudgeSelectionLeftFast {}),
        NoteCommand::NudgeSelectionRightFast(nudge_selection_right_fast::NudgeSelectionRightFast {}),
        NoteCommand::SetCamera(set_camera::SetCamera { camera: crate::NoteCamera { x: 9.0, y: 9.0, zoom: 2.0 } }),
        NoteCommand::SetCameraZoom(set_camera_zoom::SetCameraZoom { value: 1.5 }),
        NoteCommand::EngagementInput(engagement_input::EngagementInput { value: "Renaming…".into() }),
        NoteCommand::NavigatorEngagementInput(navigator_engagement_input::NavigatorEngagementInput {}),
        NoteCommand::SaveDownload(save_download::SaveDownload {}),
        NoteCommand::LoadRequest(load_request::LoadRequest {}),
    ]
}

/// 🎞️ Pins the exact hex for rows whose `Option` fields make `None`/`Some` distinct wire cases —
/// copied from the pre-migration `🧪️wire-baseline-before.txt` dump.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::SetGridVisible(set_grid_visible::SetGridVisible { value: None }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::SetSnapEnabled(set_snap_enabled::SetSnapEnabled { value: None }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }));
    store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: "[]".into(), phase: "begin".into(), select_ids: None }));
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_note_app()).expect("app definition json");
    for id in [NOTE_PLAY_WINDOW_COMPOSITE, NOTE_PLAY_WINDOW_NAVIGATOR] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for body in [NOTE_PLAY_BODY_DOCUMENT, NOTE_PLAY_BODY_CATALOGUE, NOTE_PLAY_BODY_PROPERTIES] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("2d.note"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_canvas_utilities_scoped_to_composite_window() {
    let definition = create_note_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["selectDirect", "selectMarquee", "text", "image", "table", "math", "pencil", "eraserStroke", "eraserPoint", "pan"]);
    let selects: Vec<&str> = definition.utilities.iter().filter(|utility| utility.category == Some(UtilityCategory::Selection)).map(|utility| utility.id.as_str()).collect();
    assert_eq!(selects, ["selectDirect", "selectMarquee"]);
    let composite_window = definition.window_kinds.iter().find(|window| window.id == NOTE_PLAY_WINDOW_COMPOSITE).expect("composite window");
    assert_eq!(composite_window.utilities.len(), definition.utilities.len(), "every utility is scoped to the composite canvas");
    assert!(composite_window.actions.iter().any(|action| action.id == semio_framework::SET_ACTIVE_UTILITY_ACTION_ID && matches!(action.kind, Kind::View)));
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
/// 🕹️ The `blocks` domain is declared `HierarchyProvider::Topology`, transitive on both hover and
/// selection, and scoped to the composite (canvas) window kind.
#[semio_framework_async_macros::async_test]
async fn blocks_interaction_domain_is_declared_topology_and_transitive_on_the_composite_window() {
    let definition = create_note_app();
    let blocks = definition.interactions.iter().find(|interaction| interaction.id == NOTE_INTERACTION_BLOCKS).expect("blocks interaction domain declared");
    assert!(matches!(blocks.hierarchy, HierarchyProvider::Topology));
    assert!(blocks.hover.transitive, "blocks hover must be transitive so a hovered group covers its descendants");
    assert!(blocks.selection.transitive, "blocks selection must be transitive so a selected group covers its descendants");
    let composite_window = definition.window_kinds.iter().find(|window| window.id == NOTE_PLAY_WINDOW_COMPOSITE).expect("composite window kind declared");
    assert!(composite_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == NOTE_INTERACTION_BLOCKS), "composite window must reference the blocks interaction domain");
}

/// 🌳️ `interaction_topology` walks the document's own Group nesting into `TopologyNode.parent`
/// links — a top-level block has no parent, every group child's parent is the group's own row id.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_walks_group_nesting_into_parent_links() {
    let document = crate::schema::semio_example_snapshot();
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = NotePlayApp::interaction_topology(&doc, &cfg);
    let blocks = topology.domains.get(NOTE_INTERACTION_BLOCKS).expect("blocks domain present in topology");
    assert!(!blocks.ordered.is_empty(), "the semio example document must produce a non-empty blocks topology");
    assert_eq!(blocks.ordered.len(), crate::schema::flatten_blocks(&document.blocks).len(), "topology must cover every block, nested or not");
}

/// 🌱️ An empty document has no blocks to select — an empty topology (every stale `blocks`
/// selection id gets pruned).
#[semio_framework_async_macros::async_test]
async fn interaction_topology_is_empty_for_a_document_with_no_blocks() {
    let document = empty_note_snapshot();
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = NotePlayApp::interaction_topology(&doc, &cfg);
    assert!(topology.domains.get(NOTE_INTERACTION_BLOCKS).expect("blocks domain present in topology").ordered.is_empty());
}

/// 🕹️ Retained verb over the framework-owned selection: `delete-selection` reads
/// `InteractionView::selection("blocks")` (via `NoteDispatchCtx`) — picking through the real
/// injected `interactionSelect` verb, not a deleted app command.
#[semio_framework_async_macros::async_test]
async fn delete_selection_deletes_the_blocks_picked_via_interaction_select() {
    use crate::editor::note::unit_tests::context::{dispatch as note_dispatch, note_app_with_registry, select_blocks};
    let mut app = note_app_with_registry().await;
    note_dispatch(&mut app, NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 })).await;
    let new_id = crate::schema::block_id(&app.snapshot().expect("snapshot").blocks[0]).to_string();
    select_blocks(&mut app, &[&new_id]).await;
    note_dispatch(&mut app, NoteCommand::DeleteSelection(delete_selection::DeleteSelection {})).await;
    assert!(app.snapshot().expect("snapshot").blocks.is_empty(), "the picked block must be deleted");
}
//#endregion 🔖️Interaction

//#region 🔖️Locale
#[semio_framework_async_macros::async_test]
async fn note_labels_resolve_native_by_default() {
    let mut app = note_app().await;
    let document_json = crate::editor::note::unit_tests::context::render(&mut app, NOTE_PLAY_BODY_DOCUMENT).await;
    assert!(document_json.contains("Add Text"));
    let catalogue_json = crate::editor::note::unit_tests::context::render(&mut app, NOTE_PLAY_BODY_CATALOGUE).await;
    assert!(catalogue_json.contains("Block kinds"));
}
//#endregion 🔖️Locale

//#region 🪟️WindowOwnership
#[semio_framework_async_macros::async_test]
async fn note_empty_config_owner_registry_rejects_nonempty_pack_and_retires_terminal_empty() {
    Box::pin(async {
        use crate::editor::note::unit_tests::context::{close_app, note_app_with_registry_id};
        let mut app = Box::new(note_app_with_registry_id(71_100).await);
        let config = app.config_pack().await.expect("framework NoConfig pack");
        assert!(config.pack.is_empty(), "framework NoConfig must encode no app-owned bytes");
        let foreign = store::ArtifactPackFiles { pack: b"foreign-app-config".to_vec(), spr: Vec::new(), ops: String::new() };
        assert!(app.load_config_pack(&foreign).await.is_err(), "framework NoConfig must reject nonempty app bytes");
        assert!(app.config_pack().await.expect("NoConfig after rejection").pack.is_empty());
        close_app(&mut app).await;
        eprintln!("[DEBUG] Note registry used framework NoConfig, rejected foreign app bytes, and retired its exact config owner terminal-empty");
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn note_empty_config_owner_exact_composite_window_cameras_isolate_and_reload_without_document_or_app_config_changes() {
    Box::pin(async {
        use crate::editor::note::unit_tests::context::{close_app, composite_view, dispatch_with_view_for_instance, note_app_with_registry_id, render_with_view};
        let mut app = Box::new(note_app_with_registry_id(71_101).await);
        let mut reopened = Box::new(note_app_with_registry_id(71_102).await);
        let view_a = composite_view("note-composite-a");
        let view_b = composite_view("note-composite-b");
        let document_before = app.snapshot().expect("document before window configuration");
        let app_config_before = app.config_pack().await.expect("app config before window configuration");

        let result_a = dispatch_with_view_for_instance(&mut app, NoteCommand::SetCamera(set_camera::SetCamera { camera: crate::NoteCamera { x: 12.5, y: -6.5, zoom: 3.5 } }), 71_101, view_a.clone()).await;
        let result_b = dispatch_with_view_for_instance(&mut app, NoteCommand::SetCamera(set_camera::SetCamera { camera: crate::NoteCamera { x: -42.5, y: 7.5, zoom: 1.5 } }), 71_101, view_b.clone()).await;
        assert!(result_a.mutations.is_empty() && result_b.mutations.is_empty());
        assert_eq!(app.snapshot().expect("document after window configuration"), document_before);
        let app_config_after = app.config_pack().await.expect("app config after window configuration");
        assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));

        let composite_a = render_with_view(&mut app, NOTE_PLAY_BODY_COMPOSITE, &view_a).await;
        let composite_b = render_with_view(&mut app, NOTE_PLAY_BODY_COMPOSITE, &view_b).await;
        let scene_a = artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::InkCanvasScene>(&composite_a).expect("window a ink canvas scene");
        let scene_b = artifact_app_laws::decode_fixture_scene::<semio_framework_plugin::InkCanvasScene>(&composite_b).expect("window b ink canvas scene");
        let scene_document_a: serde_json::Value = serde_json::from_str(&scene_a.document_json).expect("window a scene document JSON");
        let scene_document_b: serde_json::Value = serde_json::from_str(&scene_b.document_json).expect("window b scene document JSON");
        assert_eq!(scene_document_a["camera"], serde_json::json!({ "x": 12.5, "y": -6.5, "zoom": 3.5 }));
        assert_eq!(scene_document_b["camera"], serde_json::json!({ "x": -42.5, "y": 7.5, "zoom": 1.5 }));
        assert_eq!(app.window_config_generation(&view_a).await.expect("window a generation"), Some(1));
        assert_eq!(app.window_config_generation(&view_b).await.expect("window b generation"), Some(1));

        let packs = app.window_config_packs().await.expect("two exact Note window packs");
        assert_eq!(packs.len(), 2);
        for pack in packs {
            reopened.load_window_config_pack(pack).await.expect("reload exact Note window pack");
        }
        assert_eq!(render_with_view(&mut reopened, NOTE_PLAY_BODY_COMPOSITE, &view_a).await, composite_a);
        assert_eq!(render_with_view(&mut reopened, NOTE_PLAY_BODY_COMPOSITE, &view_b).await, composite_b);
        close_app(&mut reopened).await;
        close_app(&mut app).await;
        eprintln!("[DEBUG] two Note composite windows published and rendered independent cameras, preserved document and app configuration, reloaded both exact persistent partitions, and closed their registered apps");
    })
    .await;
}

#[semio_framework_async_macros::async_test]
async fn note_empty_config_owner_exact_composite_window_transient_isolates_resets_and_cancels_with_registered_app() {
    Box::pin(async {
        use crate::editor::note::unit_tests::context::{close_app, composite_view, dispatch_with_view_for_instance, note_app_with_registry_id};
        let mut app = Box::new(note_app_with_registry_id(71_103).await);
        let view_a = composite_view("note-transient-a");
        let view_b = composite_view("note-transient-b");

        dispatch_with_view_for_instance(&mut app, NoteCommand::EngagementInput(engagement_input::EngagementInput { value: "Alpha".into() }), 71_103, view_a.clone()).await;
        let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
        let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
        assert_eq!(transient_a.get::<window::NoteCompositeWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some("Alpha"));
        assert_eq!(transient_b.get::<window::NoteCompositeWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
        drop((transient_a, transient_b));

        let document = app.document_pack().await.expect("document pack before reset");
        app.load_document_pack(&document).await.expect("reload document and reset window transient");
        let reset_a = app.window_transient_snapshot(&view_a).expect("reset window a transient").expect("reset window a owner");
        assert_eq!(reset_a.get::<window::NoteCompositeWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
        assert_eq!(app.window_transient_generation(&view_a).expect("reset window a generation"), Some(0));
        drop(reset_a);

        let pending = app
            .dispatch_typed(
                NoteCommand::EngagementInput(engagement_input::EngagementInput { value: "Cancelled".into() }),
                &semio_framework_plugin::ActionMeta { instance_id: 71_103, view_state: Some(view_a.clone()), ..artifact_app_laws::meta("cancel") },
            )
            .await
            .expect("start retained transient command");
        assert!(pending.mutations.is_empty());
        assert!(app.has_pending_typed_operations(), "retained transient command must remain owned before publication advances");
        app.load_document_pack(&document).await.expect("replacement cancels captured transient publication authority");
        let cancelled = app.window_transient_snapshot(&view_a).expect("cancelled window a transient").expect("cancelled window a owner");
        assert_eq!(cancelled.get::<window::NoteCompositeWindowTransientOwner>().map(|value| value.engagement_input.as_str()), Some(""));
        drop(cancelled);
        close_app(&mut app).await;
        eprintln!("[DEBUG] Note composite transient input stayed exact-window isolated, document reload reset ephemeral state, replacement cancelled an unpublished retained mutation, and registered app close reached terminal-empty ownership");
    })
    .await;
}
//#endregion 🪟️WindowOwnership

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = note_app().await;
    artifact_app_laws::assert_undo_redo_round_trip(&mut app, NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }), |app| app.snapshot().expect("snapshot").blocks.len(), 0, 1).await;
}

/// 🧪️ The definitional regression proof: two independent instances start from the same document,
/// apply DISJOINT edits, and exchanging operations over a `MemoryBackbone` converges both sides to
/// contain BOTH edits.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    artifact_app_laws::assert_two_instances_converge::<semio_framework_plugin::EditorApp<NotePlayApp>, (usize, Option<bool>)>(
        "mem://note-convergence",
        NoteCommand::AddBlock(add_block::AddBlock { kind: "text".into(), x: 0.0, y: 0.0 }),
        NoteCommand::SetGridVisible(set_grid_visible::SetGridVisible { value: Some(false) }),
        |app| {
            let projection = app.snapshot().expect("snapshot");
            (projection.blocks.len(), projection.grid_visible)
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_note() {
    artifact_app_laws::assert_ingest_idempotent::<semio_framework_plugin::EditorApp<NotePlayApp>, f64>(NoteCommand::SetGridSpacing(set_grid_spacing::SetGridSpacing { value: 48.0 }), |app| app.snapshot().expect("snapshot").grid_spacing.unwrap_or_default())
        .await;
}
//#endregion 🔖️CrossCutting
