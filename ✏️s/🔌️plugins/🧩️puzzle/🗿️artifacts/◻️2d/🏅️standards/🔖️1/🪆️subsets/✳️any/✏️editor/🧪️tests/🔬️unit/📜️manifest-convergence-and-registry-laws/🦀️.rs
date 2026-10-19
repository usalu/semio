// 📜️ Manifest, convergence and registry laws: the declared window kinds, actions and tools, the backbone convergence, the idempotent ingest and the registry view and context menu laws.

//#region 🔖️Manifest
#[test]
fn app_definition_has_three_lod_pane_window_kinds() {
    let definition = create_puzzle2d_app();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert_eq!(ids, vec![crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID]);
    for window in &definition.window_kinds {
        assert!(window.options.engagement.as_option().is_some(), "pane {} must have engagement", window.id);
        assert!(!window.options.measures.is_empty(), "pane {} must have LOD/suggestion measures", window.id);
    }
}

/// 🕹️ Every action DISPATCHABLE in one window: its OWN roster plus every app-level row no window
/// kind claims for itself.
///
/// 🪪️ App-level actions are no longer CLONED onto every window kind (that made a package descriptor
/// grow as `apps × window kinds × actions`) — `semio_framework::window_kind_actions` resolves the
/// union at read time, so `WindowKindDefinition::actions` alone now holds only the window's own
/// declarations (here: the framework-injected interaction verbs) and a law reading it sees none of
/// the authored vocabulary.
fn dispatchable_actions<'a>(definition: &'a semio_framework_plugin::AppDefinition, window: &'a semio_framework_plugin::WindowKindDefinition) -> Vec<&'a semio_framework_plugin::ActionDefinition> {
    semio_framework::window_kind_actions(definition, window)
}

/// 🧰️ The app declares exactly the select/brush canvas utilities and binds them to the interactive
/// overview pane; fill is declared as a mode-level tool instead.
#[test]
fn utility_registry_declares_utilities() {
    let definition = create_puzzle2d_app();
    let ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(ids, vec![crate::editor::puzzle2d::config::PUZZLE2D_SELECT_UTILITY_ID, brush_utility::UTILITY_ID, area_brush_utility::UTILITY_ID]);
    let overview_window = definition.window_kinds.iter().find(|window| window.id == crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID).expect("overview pane");
    let overview_utilities: Vec<&str> = overview_window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(overview_utilities, vec![crate::editor::puzzle2d::config::PUZZLE2D_SELECT_UTILITY_ID, brush_utility::UTILITY_ID, area_brush_utility::UTILITY_ID]);
    assert!(dispatchable_actions(&definition, overview_window).iter().any(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID), "declaring utilities must inject the setActiveUtility action");
    // 🧰️ D-1: select/brush are this window's whole exclusive utility set, NOT a sub-collection, so
    // each carries `group: None` and renders as a flat utility bar icon (never one collapsed dropdown).
    for utility in &definition.utilities {
        assert_eq!(utility.group, None, "utility {} must render flat (no shared group)", utility.id);
    }
}

/// 🎬️ Every app-declared action resolves to a `Puzzle2dCommand` — an action declared in the manifest
/// but missing from `puzzle2d_command_variants!` reaches the host as a row/keybinding and dies at
/// dispatch with `unknown Puzzle 2D action` (export/import/transforms, 2026-09-17).
#[test]
fn every_declared_action_resolves_to_a_command() {
    let definition = create_puzzle2d_app();
    let mut unresolved = Vec::new();
    let mut declared = Vec::new();
    for window in &definition.window_kinds {
        for action in dispatchable_actions(&definition, window) {
            let id = action.id.as_str();
            declared.push(id.to_string());
            // 🕰️ Framework-owned verbs never reach `command_from_action`: history, clipboard, the
            // interaction six, the injected utility/tool switches, the tool-run and history-edit controls, the
            // operation cancel and the whole-document export/import.
            let reserved = matches!(
                id,
                "undo" | "redo" | "commitCheckpoint" | "createAlternative" | "switchAlternative" | "checkoutCheckpoint" | "copy" | "cut" | "paste" | "revertToCommand" | "historyFilter" | "setHistoryCommandFilter" | "noteShellCommand" | "recordTutorial" | "interactionSelect" | "interactionHover" | "clearSelection" | "selectAll" | "setSelectionMode" | "setInteractionGranularity"
            ) || id == SET_ACTIVE_UTILITY_ACTION_ID
                || id == semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID
                || semio_framework_plugin::is_tool_run_action_id(id)
                || semio_framework_plugin::is_time_travel_action_id(id)
                || id == semio_framework_plugin::app::CANCEL_TYPED_OPERATION_ACTION_ID
                || matches!(id, semio_framework::EXPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::IMPORT_ARTIFACT_DOCUMENT_ACTION_ID | semio_framework::HOST_EVENT_ACTION_ID);
            if !reserved && Puzzle2dCommand::try_from_action(id, None, None).is_none() {
                unresolved.push(id.to_string());
            }
        }
    }
    for expected in ["exportSnapshot", "openImportSnapshot", "importSnapshot", "translateSelection", "rotateSelection", "scaleSelection"] {
        assert!(declared.iter().any(|id| id == expected), "the window action roster must carry the app-level action '{expected}' (declared: {declared:?})");
    }
    assert!(unresolved.is_empty(), "declared actions without a Puzzle2dCommand variant: {unresolved:?}");
}

/// ⚙️ The app settings panel is an app-level body (`ViewModel::for_panel` carries no window), so its
/// steppers address the FOCUSED pane; with no live pane they carry no `windowId` at all — an empty one
/// is refused by the host as an unknown window instance (`setFillCount {windowId: ""}`, 2026-09-17).
#[semio_framework_async_macros::async_test]
async fn settings_steppers_address_the_focused_pane_or_no_pane() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let focused = semio_framework_plugin::ViewModel { focused_window_id: Some(crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID.into()), ..window_view(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID) }.for_panel();
    let body = render_body_with_view(&mut app, settings::PUZZLE2D_PLAY_BODY_SETTINGS, &focused);
    assert!(body.contains("setFillCount"), "the settings body must carry the fill-count stepper: {}", &body[..body.len().min(300)]);
    assert!(body.contains(&format!("\"windowId\":\"{}\"", crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID)), "a stepper must address the focused pane: {}", &body[..body.len().min(600)]);
    let unhosted = render_body_with_view(&mut app, settings::PUZZLE2D_PLAY_BODY_SETTINGS, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native));
    close_app(&mut app);
    assert!(unhosted.contains("setFillCount") && !unhosted.contains("windowId"), "with no live pane the steppers must carry no windowId: {}", &unhosted[..unhosted.len().min(600)]);
}

/// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
#[test]
fn tool_registry_declares_fill_tool() {
    use semio_framework_plugin::{ToolRef, SET_ACTIVE_TOOL_ACTION_ID};
    let definition = create_puzzle2d_app();
    let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
    assert_eq!(tool_ids, vec![fill::TOOL_ID]);
    assert_eq!(definition.modes[0].tools, vec![block_on(ToolRef::new(fill::TOOL_ID))]);
    assert!(definition.window_kinds.iter().flat_map(|window| dispatchable_actions(&definition, window)).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
}

/// 🎥️ The camera is session-only runtime state, never a document field — a DWG import (which has
/// no live app instance to receive a runtime write) must produce a bare empty board with no
/// `"camera"` key at all, regardless of the drawing's extents.
//#endregion 🔖️Manifest

//#region 🔖️Convergence
/// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT node edits
/// (each adds its own node) and, after exchanging operations, both converge to contain BOTH nodes —
/// impossible under whole-document `setSnapshot` snapshots, which would clobber one side.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_node_edits_via_backbone() {
    let mut instance_a = app();
    let mut instance_b = app();
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle2d-convergence", "mem://puzzle2d-convergence").await;
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addNode", Some(&json!({ "kind": "seed" })), None).expect("a adds node");
    dispatch(&mut instance_b, "addNode", Some(&json!({ "kind": "other" })), None).expect("b adds node");

    // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
    dispatch(&mut instance_a, "commitCheckpoint", None, None).expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).expect("pump b");

    assert_eq!(board_snapshot_nodes(&fixture_of(&instance_a)).len(), 2, "instance A must contain both nodes");
    assert_eq!(board_snapshot_nodes(&fixture_of(&instance_b)).len(), 2, "instance B must contain both nodes");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
    close_app(&mut instance_a);
    close_app(&mut instance_b);
}

#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    let mut sender = app();
    let (near, mut far) = MemoryBackbone::pair("mem://puzzle2d-doc", "mem://puzzle2d-doc").await;
    sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach");
    dispatch(&mut sender, "addNode", Some(&json!({ "kind": "seed" })), None).expect("add");

    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(operations);
        }
    }
    assert!(!envelopes.is_empty(), "the applied operation must flow onto the channel");
    let operations = envelopes;

    let mut receiver = app();
    receiver.0.ingest_operations(&operations, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), receiver.1.cx()).await.expect("ingest once");
    receiver.0.ingest_operations(&operations, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), receiver.1.cx()).await.expect("ingest twice");
    assert_eq!(board_snapshot_nodes(&fixture_of(&receiver)).len(), 1, "feeding the same operation twice must not double-apply");
    sender.detach_backbone().await.expect("sender releases its backbone");
    close_app(&mut receiver);
    close_app(&mut sender);
}
//#endregion 🔖️Convergence

//#region 🔖️Registry
/// 🧭️ Kind discipline: every View-declared runtime/host action must run through the registry
/// without tripping the "must not emit operations" guard (proving each is correctly classified).
#[semio_framework_async_macros::async_test]
async fn view_actions_emit_no_ops_through_the_registry() {
    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let node_id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select");
    let view_dispatches: Vec<(&str, Value)> = vec![
        ("setCamera", json!({ "camera": { "x": 7.0, "y": 8.0, "zoom": 1.5 } })),
        ("selectSameKind", Value::Null),
        ("setGridSnapEnabled", json!({ "enabled": true })),
        ("setGridFactor", json!({ "value": 2.0 })),
        ("setLodModeForPane", json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, "value": "detail" })),
        ("setBrushKindWeights", json!({ "kindId": "node", "value": 0.5 })),
        ("setBrushNodeSize", json!({ "size": 12.0 })),
        ("setSuggestionOffset", json!({ "value": 40.0 })),
        ("engagementInput", json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, "value": "brush" })),
        ("engagementSubmit", json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, "value": "brush" })),
        ("engagementAbort", json!({ "pane": crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID })),
        ("cycleBrushCandidate", json!({ "forward": true })),
        ("cycleBrushCandidateBack", Value::Null),
        ("hoverSuggestion", json!({ "index": 0 })),
        ("targetBrushSuggestions", Value::Null),
        ("closeHandleSuggestions", Value::Null),
        ("openHandleSuggestions", json!({ "handleId": "" })),
        ("lodScaleJson", Value::Null),
    ];
    for (action, args) in view_dispatches {
        let args_ref = (!args.is_null()).then_some(&args);
        let result = dispatch(&mut app, action, args_ref, None).unwrap_or_else(|error| panic!("view action '{action}' must not error: {error:?}"));
        assert_eq!(committed_edits(&result), 0, "view action '{action}' must not emit document operations");
    }
    close_app(&mut app);
}

/// 🗂️ Grouped-context-menu disclosure: the top-level row budget stays small (leaves+groups
/// combined) and the known `deleteSelection` destructive row stays last.
#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
    use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, UiMenuRef};

    let mut app = app_with_registry();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let node_id = first_node_id(&app);
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `context_menu` reads the
    // CLIENT-supplied `request.surface.selection` now (selection is framework-owned, no live
    // config field to derive it from) — see `context_menu`'s own doc comment.
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "puzzle2d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: "puzzle2d".into(), kind: "board".into(), hits: Vec::new(), selection: vec![ContextMenuSelectionGroup { domain: PUZZLE2D_GRANULARITY_NODE.into(), ids: vec![node_id] }], text: None }),
        window_instance_id: None,
        point: None,
    };
    let menu = block_on(app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)));
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    let last = menu.last().expect("grouped disclosure menu should not be empty");
    assert_eq!(last.id, "deleteSelection", "the destructive row must stay last as a top-level leaf");
    assert_eq!(last.destructive, Some(true), "the destructive row must carry destructive: true");
    close_app(&mut app);
}
//#endregion 🔖️Registry
