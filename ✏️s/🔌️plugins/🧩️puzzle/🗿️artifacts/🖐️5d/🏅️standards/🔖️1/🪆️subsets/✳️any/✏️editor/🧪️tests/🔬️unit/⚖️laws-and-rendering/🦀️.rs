//! ⚖️ Retained static laws, rendering, context menu, pack and operation tests of the puzzle 5d editor. See [`super`].
use crate::editor::puzzle5d::catalog::puzzle5d_normalize_kind_weight_group;
use semio_framework_test_fixtures::fixture_text;

use super::*;

#[test]
fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
    let exact = |contracts: &[ArtifactToolPublicationContract]| {
        let ids = contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
        ids == PUZZLE5D_RETAINED_TOOL_IDS.iter().copied().collect()
            && ids.len() == contracts.len()
            && contracts.iter().all(|contract| !contract.lanes.is_empty() && (!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1))
    };
    let contracts = <Puzzle5dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert!(exact(contracts));
    assert!(!exact(&contracts[..contracts.len() - 1]));
    let mut duplicate = contracts.to_vec();
    let copied = duplicate[1];
    duplicate[0] = copied;
    assert!(!exact(&duplicate));
}

/// 📚️ The editor module's production source as the unsplit editor file held it: every mounted child first, then the editor file itself, so the `UnitTests` region marker still closes the production prefix.
pub(crate) fn editor_source() -> String {
    [
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧩️entities/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🗂️catalog/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🕹️interaction/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🖌️session/📋️clipboard/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🖌️session/📌️paste/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🖌️session/📥️snapshot-import/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/👆️gestures/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/👆️gestures/🎛️selection-and-weights/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/👆️gestures/🩹️patches/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/👆️gestures/🔗️fastener-edits/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/👆️gestures/🧲️proximity-connect/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧰️kit/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧰️kit/➕️add-node-and-brush-part/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧰️kit/🧱️board-events-and-fasteners/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧰️kit/📚️example-and-brush-mesh/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🧰️kit/💾️store-preparation/🦀️.rs"),
        fixture_text!("../../🪆️subsets/✳️any/✏️editor/🦀️.rs"),
    ]
    .concat()
}

#[test]
fn retained_import_media_has_no_live_synchronous_fallback() {
    let source = editor_source();
    let source = source.as_str();
    let production = source.split_once("//#region 🧪️UnitTests").map(|(production, _)| production).expect("production prefix");
    let fallback = production.split_once("fn import_media(_port:").and_then(|(_, suffix)| suffix.split_once("fn render(").map(|(fallback, _)| fallback)).expect("closed synchronous import callback");
    assert!(fallback.contains("Err(MediaError::NotImplemented)"));
    assert!(!fallback.contains("serde_json::from_str"));
    assert!(!fallback.contains("artifact_mutations"));
    let hostile = fallback.replace("Err(MediaError::NotImplemented)", "serde_json::from_str(\"{}\").map(|_| Emit::default()).map_err(|_| MediaError::NotImplemented)");
    assert!(hostile.contains("serde_json::from_str"));
}

fn complex_retained_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""applyBoardEvents" => owned!(Puzzle5dBoardEventsWork, |request, scope| Puzzle5dBoardEventsWork::from_original_construction(scope).map(|(work, progress)| (work.bound(nonce), progress)))"#)
        && source.contains("struct Puzzle5dBoardEventsWork")
        && source.contains("self.scan_one(source)?")
        && source.contains("puzzle5d_transform_tool_commit(\"applyBoardEvents\"")
        && source.contains("Puzzle5dBoardEventsStage::ScanEdge")
        && source.contains("Puzzle5dBoardEventsStage::ScanDeleteEdges")
        && source.contains("Puzzle5dBoardEventsStage::Brush")
        && source.contains("Puzzle5dBoardEventsStage::CloseBrush")
        && !source.contains("\"applyBoardEvents\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
}

#[test]
fn apply_board_events_hostile_static_law_rejects_the_old_one_grant_reducer() {
    let source = editor_source();
    let source = source.as_str();
    assert!(complex_retained_route_is_cursorized(source));
    let direct = source
        .replace(r#""applyBoardEvents" => owned!(Puzzle5dBoardEventsWork, |request, scope| Puzzle5dBoardEventsWork::from_original_construction(scope).map(|(work, progress)| (work.bound(nonce), progress)))"#, "\"applyBoardEvents\" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))");
    assert!(!complex_retained_route_is_cursorized(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["self.scan_one(source)?", "puzzle5d_transform_tool_commit(\"applyBoardEvents\"", "Puzzle5dBoardEventsStage::ScanEdge", "Puzzle5dBoardEventsStage::ScanDeleteEdges", "Puzzle5dBoardEventsStage::CloseBrush"] {
        assert!(!complex_retained_route_is_cursorized(&source.replace(marker, "cursor-removed")), "missing cursor marker was falsely accepted: {marker}");
    }
}

fn focus_selection_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""focusSelection" => work!(|request| Puzzle5dFocusSelectionWork::default())"#)
        && source.contains("Puzzle5dFocusSelectionStage::Selection")
        && source.contains("Puzzle5dFocusSelectionStage::Parts")
        && source.contains("Puzzle5dFocusSelectionStage::Publish")
        && source.contains("self.part_cursor += 1")
        && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn focus_selection_hostile_static_law_rejects_whole_selection_reducers() {
    let source = editor_source();
    let source = source.as_str();
    assert!(focus_selection_route_is_cursorized(source));
    let direct = source
        .replace(r#""focusSelection" => work!(|request| Puzzle5dFocusSelectionWork::default())"#, r#""focusSelection" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
    assert!(!focus_selection_route_is_cursorized(&direct));
    for marker in ["Puzzle5dFocusSelectionStage::Selection", "Puzzle5dFocusSelectionStage::Parts", "Puzzle5dFocusSelectionStage::Publish", "self.part_cursor += 1"] {
        assert!(!focus_selection_route_is_cursorized(&source.replace(marker, "cursor-removed")), "missing focus cursor marker was falsely accepted: {marker}");
    }
}

fn window_owner_routes_are_exact(source: &str) -> bool {
    [
        "struct Puzzle5dWindowCommandWork",
        "window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => seed!(|request, seed| Puzzle5dWindowCommandWork::new(window, seed).bound(request.instance_operation_owner.clone()))",
        "context.and_then(|context| context.window_config.as_ref())",
        "config_from_snapshot(window_config_snapshot)",
        "transient_from_snapshot(window_transient_snapshot)",
        "window_config_mutations",
        "window_transient",
        "addressed_config(view, window_after)",
        "addressed_transient(view, transient_after)",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains("Puzzle5dConfigMutation::SetCamera2d")
        && !source.contains("Puzzle5dConfigMutation::SetBrushCandidateIndex")
        && !source.contains("Puzzle5dConfigMutation::SetEngagementInput")
        && !source.contains("Puzzle5dConfigMutation::SetGridFactor")
        && !source.contains("Puzzle5dConfigMutation::SetSun")
        && !source.contains("Puzzle5dEngagementAbortWork")
        && !source.contains("Puzzle5dEngagementSubmitWork")
        && !source.contains("Puzzle5dPrecomputeCommandWork")
        && !source.contains(r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" =>"#)
}

#[test]
fn window_owner_hostile_static_law_rejects_missing_owner_boundaries_and_app_config_leaks() {
    let source = editor_source();
    let source = source.as_str();
    assert!(window_owner_routes_are_exact(source));
    for marker in [
        "struct Puzzle5dWindowCommandWork",
        "context.and_then(|context| context.window_config.as_ref())",
        "config_from_snapshot(window_config_snapshot)",
        "transient_from_snapshot(window_transient_snapshot)",
        "addressed_config(view, window_after)",
        "addressed_transient(view, transient_after)",
    ] {
        assert!(!window_owner_routes_are_exact(&source.replace(marker, "route-removed")), "missing exact window-owner boundary was falsely accepted: {marker}");
    }
    let leaked = source.replace("Puzzle5dConfigMutation::Snapshot { config: shared_after }", "Puzzle5dConfigMutation::SetCamera2d");
    assert!(!window_owner_routes_are_exact(&leaked), "hostile app-config window leak must fail closed");
}

fn add_part_kind_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""addBrushPart" | "addPartKind" => owned!(Puzzle5dAddBrushPartWork, |request, scope| Puzzle5dAddBrushPartWork::from_original_construction(tool_id, scope).map(|(work, progress)| (work.bound(nonce), progress)))"#)
        && source.contains("Puzzle5dAddBrushPartStage::Catalog")
        && source.contains("Puzzle5dAddBrushPartStage::Grips")
        && source.contains("Puzzle5dAddBrushPartStage::Target")
        && source.contains("Puzzle5dAddBrushPartStage::Create")
        && source.contains("Puzzle5dAddBrushPartStage::Connect")
        && !source.contains(r#""addPartKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_part_kind_hostile_static_law_rejects_old_brush_reducer_and_missing_cursors() {
    let source = editor_source();
    let source = source.as_str();
    assert!(add_part_kind_route_is_cursorized(source));
    let direct = source.replace(
        r#""addBrushPart" | "addPartKind" => owned!(Puzzle5dAddBrushPartWork, |request, scope| Puzzle5dAddBrushPartWork::from_original_construction(tool_id, scope).map(|(work, progress)| (work.bound(nonce), progress)))"#,
        r#""addBrushPart" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id)),
            "addPartKind" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!add_part_kind_route_is_cursorized(&direct));
}

fn kind_weight_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""setPartKindWeight" | "setGripKindWeight" => work!(|request| Puzzle5dKindWeightWork::new(tool_id))"#)
        && source.contains("Puzzle5dKindWeightStage::Catalog")
        && source.contains("Puzzle5dKindWeightStage::InferParts")
        && source.contains("Puzzle5dKindWeightStage::InferGrips")
        && source.contains("Puzzle5dKindWeightStage::Validate")
        && source.contains("Puzzle5dKindWeightStage::SumOthers")
        && source.contains("Puzzle5dKindWeightStage::Build")
        && source.contains("Puzzle5dConfigMutation::SetObjectKindWeights")
        && source.contains("Puzzle5dConfigMutation::SetVortexKindWeights")
        && !source.contains(r#""setPartKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
    let source = editor_source();
    let source = source.as_str();
    assert!(kind_weight_route_is_cursorized(source));
    let direct = source.replace(
        r#""setPartKindWeight" | "setGripKindWeight" => work!(|request| Puzzle5dKindWeightWork::new(tool_id))"#,
        r#""setPartKindWeight" | "setGripKindWeight" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!kind_weight_route_is_cursorized(&direct));
    assert!(!source.contains("puzzle5d_normalize_kind_weight_group(self.weights"));
}

fn engagement_submit_route_is_cursorized(source: &str) -> bool {
    source.contains(r#"const PUZZLE5D_WINDOW_TOOL_IDS: &[&str] = &["#) && PUZZLE5D_WINDOW_TOOL_IDS.iter().all(|tool| source.contains(&format!("    \"{tool}\",\n")) || source.contains(&format!("    \"{tool}\",\r\n")))
        && source.contains("window if PUZZLE5D_WINDOW_TOOL_IDS.contains(&window) => seed!(|request, seed| Puzzle5dWindowCommandWork::new(window, seed).bound(request.instance_operation_owner.clone()))")
        && source.contains("transient_from_snapshot(window_transient_snapshot)")
        && source.contains("addressed_transient(view, transient_after)")
        && source.contains("EphemeralEmit { window_transient, ..Default::default() }")
        && !source.contains("Puzzle5dConfigMutation::SetEngagementInput")
        && !source.contains(r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn engagement_submit_hostile_static_law_rejects_old_reducer_and_missing_transfers() {
    let source = editor_source();
    let source = source.as_str();
    assert!(engagement_submit_route_is_cursorized(source));
    for marker in [
        "transient_from_snapshot(window_transient_snapshot)",
        "addressed_transient(view, transient_after)",
        "EphemeralEmit { window_transient, ..Default::default() }",
    ] {
        assert!(!engagement_submit_route_is_cursorized(&source.replace(marker, "route-removed")), "missing engagement submit marker was falsely accepted: {marker}");
    }
    let leaked = source.replace("Puzzle5dConfigMutation::Snapshot { config: shared_after }", "Puzzle5dConfigMutation::SetEngagementInput");
    assert!(!engagement_submit_route_is_cursorized(&leaked));
}

fn selection_transforms_run_the_transform_tool(source: &str) -> bool {
    source.contains(r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => seed!(|request, seed| Puzzle5dTransformWork::new(tool_id, seed))"#)
        && source.contains(r#""patchPart" if puzzle5d_inspector_nudge(original.command.args()).is_some() => seed!(|request, seed| Puzzle5dTransformWork::new(tool_id, seed))"#)
        && source.contains("Puzzle5dTransformStage::Read")
        && source.contains("Puzzle5dTransformStage::Scan")
        && source.contains("Puzzle5dRelocateScan::begin(document")
        && source.contains("Puzzle5dTransformStage::Commit")
        && source.contains("puzzle5d_transform_tool_commit(self.tool_id")
        && !source.contains(r#""translateSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

/// 🛠️ Every selection transform — the three gumball verbs, the target-volume relocate, the world drop and the
/// inspector pose nudges — is ONE staged work that states the gesture and commits it through the transform
/// tool: a one-grant reducer, a missing stage or a bypassed tool fails closed.
#[test]
fn selection_transform_hostile_static_law_rejects_one_grant_reducers_and_bypassed_tools() {
    let source = editor_source();
    let source = source.as_str();
    assert!(selection_transforms_run_the_transform_tool(source));
    let direct = source.replace(
        r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => seed!(|request, seed| Puzzle5dTransformWork::new(tool_id, seed))"#,
        r#""translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => work!(|request| crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!selection_transforms_run_the_transform_tool(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["Puzzle5dTransformStage::Read", "Puzzle5dTransformStage::Scan", "Puzzle5dRelocateScan::begin(document", "Puzzle5dTransformStage::Commit", "puzzle5d_transform_tool_commit(self.tool_id", "if puzzle5d_inspector_nudge(request.command.args()).is_some()"] {
        assert!(!selection_transforms_run_the_transform_tool(&source.replace(marker, "stage-removed")), "missing transform stage was falsely accepted: {marker}");
    }
}

//#region 🔖️Rendering
#[semio_framework_async_macros::async_test]
async fn renders_paired_board_and_world_scenes() {
    let mut app = app();
    assert!(render_body(&mut app, board2d::BODY_KEY).contains("board-2d"));
    assert!(render_body(&mut app, world3d::BODY_KEY).contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_concrete_forest_document() {
    let app = app();
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE5D_SCHEMA));
    assert!(part_count(&app) > 0, "the concrete-forest default document ships with parts");
}

#[semio_framework_async_macros::async_test]
async fn document_panel_renders() {
    let mut app = app();
    assert!(!render_body(&mut app, artifact_panel::BODY_KEY).is_empty());
}
//#endregion 🔖️Rendering

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
/// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
/// and the known destructive `deleteSelection` action stays the trailing group's last item.
#[semio_framework_async_macros::async_test]
async fn context_menu_is_grouped_and_keeps_delete_selection_last() {
    let mut app = app_with_registry();
    let part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &part_id).expect("select part");
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: world3d::WINDOW_KIND_ID.into(), kind: "world3d".into(), hits: vec![], selection: vec![ContextMenuSelectionGroup { domain: "part".into(), ids: vec![part_id] }], text: None }),
        window_instance_id: None,
        point: None,
    };
    let menu = ::semio_framework_async::poll::resolve_ready(app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)));
    assert!(menu.len() <= 9, "top-level context menu should stay progressively disclosed: {menu:?}");
    let last = menu.last().expect("selection context menu should not be empty");
    let last_is_destructive_leaf = last.action.as_deref() == Some("deleteSelection") && last.destructive == Some(true);
    let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.action.as_deref() == Some("deleteSelection") && child.destructive == Some(true));
    assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must stay last: {menu:?}");
}
//#endregion 🔖️ContextMenu

//#region 🔖️Pack
/// 📦️ `Puzzle5dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
#[semio_framework_async_macros::async_test]
async fn puzzle5d_play_projection_pack_round_trips() {
    let app = app();
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}
//#endregion 🔖️Pack

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_the_document_and_undo_restores_it() {
    let mut app = app();
    let loaded = part_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": "" })), None).expect("empty");
    assert_eq!(part_count(&app), 0, "empty example clears the parts");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
    dispatch(&mut app, "redo", None, None).expect("redo");
    assert_eq!(part_count(&app), 0);
}

#[semio_framework_async_macros::async_test]
async fn patch_fastener_updates_transform_offsets_and_undoes() {
    let mut app = app();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin (has fasteners)");
    let projection = projection_of(&app);
    let fastener_id = projection["fasteners"][0]["id"].as_str().expect("seeded fastener").to_string();
    dispatch(&mut app, "patchFastener", Some(&semio_framework_pack_json::json!({ "fastenerId": fastener_id, "field": "gap", "value": 2.5 })), None).expect("patch gap");
    let after = projection_of(&app);
    let fastener = after["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener["gap"], 2.5);
    assert_eq!(fastener["shift"], 0.0);
    dispatch(&mut app, "patchFastener", Some(&semio_framework_pack_json::json!({ "fastenerId": fastener_id, "field": "rotation", "value": 30.0 })), None).expect("patch rotation");
    let after2 = projection_of(&app);
    let fastener2 = after2["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener2["gap"], 2.5, "earlier gap edit must survive a later rotation edit");
    assert_eq!(fastener2["rotation"], 30.0);
    dispatch(&mut app, "undo", None, None).expect("undo");
    let undone = projection_of(&app);
    let fastener3 = undone["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener3["rotation"], 0.0, "undo restores the pre-rotation-edit value");
    assert_eq!(fastener3["gap"], 2.5, "undo of rotation edit must not also revert the earlier gap edit");
}
//#endregion 🔖️Operations
