
use super::testkit::*;
use super::*;

use semio_framework_plugin::{ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, PluginApp, UiMenuRef};

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
    let reserved = [
        <Puzzle5dCopyJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
        <Puzzle5dCutJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
        <Puzzle5dPasteJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
        <Puzzle5dImportJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS[0],
    ];
    assert_eq!(reserved.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), vec!["copy", "cut", "paste", "import-media"]);
    assert_eq!(reserved[0].lanes, &[ArtifactToolPublicationLane::HostOnly]);
    assert!(reserved[1..].iter().all(|contract| contract.lanes == &[ArtifactToolPublicationLane::Artifact]));
}

#[test]
fn retained_import_media_has_no_live_synchronous_fallback() {
    let source = include_str!("../../🦀️.rs");
    let production = source.split_once("//#region 🧪️Testkit").map(|(production, _)| production).expect("production prefix");
    let fallback = production.split_once("fn import_media(_port:").and_then(|(_, suffix)| suffix.split_once("fn render(").map(|(fallback, _)| fallback)).expect("closed synchronous import callback");
    assert!(fallback.contains("Err(MediaError::NotImplemented)"));
    assert!(!fallback.contains("serde_json::from_str"));
    assert!(!fallback.contains("artifact_mutations"));
    let hostile = fallback.replace("Err(MediaError::NotImplemented)", "serde_json::from_str(\"{}\").map(|_| Emit::default()).map_err(|_| MediaError::NotImplemented)");
    assert!(hostile.contains("serde_json::from_str"));
}

fn precompute_routes_are_cursorized(source: &str) -> bool {
    [
        r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" => Box::new(Puzzle5dPrecomputeCommandWork::new(tool_id))"#,
        "Puzzle5dPrecomputeCommandStage::Parts",
        "Puzzle5dPrecomputeCommandStage::Grips",
        "Puzzle5dPrecomputeCommandStage::Fasteners",
        "Puzzle5dPrecomputeCommandStage::CatalogParts",
        "Puzzle5dPrecomputeCommandStage::CatalogGrips",
        "Puzzle5dPrecomputeCommandStage::Positions",
        "Puzzle5dPrecomputeCommandStage::Indices",
        "Puzzle5dPrecomputeCommandStage::FillCount",
        "Puzzle5dPrecomputeCommandStage::BoardUtility",
        "Puzzle5dPrecomputeCommandStage::WorldUtility",
        "Puzzle5dPrecomputeCommandStage::Publish",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains(r#""cycleBrushCandidate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""registerBrushMesh" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries() {
    let source = include_str!("../../🦀️.rs");
    assert!(precompute_routes_are_cursorized(source));
    let direct = source.replace(
        r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" => Box::new(Puzzle5dPrecomputeCommandWork::new(tool_id))"#,
        r#""cycleBrushCandidate" | "registerBrushMesh" | "setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!precompute_routes_are_cursorized(&direct));
    for marker in [
        "Puzzle5dPrecomputeCommandStage::Parts",
        "Puzzle5dPrecomputeCommandStage::Grips",
        "Puzzle5dPrecomputeCommandStage::Fasteners",
        "Puzzle5dPrecomputeCommandStage::CatalogParts",
        "Puzzle5dPrecomputeCommandStage::CatalogGrips",
        "Puzzle5dPrecomputeCommandStage::Positions",
        "Puzzle5dPrecomputeCommandStage::Indices",
        "Puzzle5dPrecomputeCommandStage::FillCount",
        "Puzzle5dPrecomputeCommandStage::BoardUtility",
        "Puzzle5dPrecomputeCommandStage::WorldUtility",
        "Puzzle5dPrecomputeCommandStage::Publish",
    ] {
        assert!(!precompute_routes_are_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing retained boundary was falsely accepted: {marker}");
    }
}

fn complex_retained_route_is_cursorized(source: &str) -> bool {
    source.contains("\"applyBoardEvents\" => Box::new(Puzzle5dBoardEventsWork::default())")
        && source.contains("struct Puzzle5dBoardEventsWork")
        && source.contains("self.scan_one(source)?")
        && source.contains("Puzzle5dBoardEventsStage::FindMovePart")
        && source.contains("Puzzle5dBoardEventsStage::ScanEdge")
        && source.contains("Puzzle5dBoardEventsStage::ScanDeleteEdges")
        && source.contains("Puzzle5dBoardEventsStage::Brush")
        && source.contains("Puzzle5dBoardEventsStage::CloseBrush")
        && !source.contains("\"applyBoardEvents\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
}

#[test]
fn apply_board_events_hostile_static_law_rejects_the_old_one_grant_reducer() {
    let source = include_str!("../../🦀️.rs");
    assert!(complex_retained_route_is_cursorized(source));
    let direct = source
        .replace("\"applyBoardEvents\" => Box::new(Puzzle5dBoardEventsWork::default())", "\"applyBoardEvents\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))");
    assert!(!complex_retained_route_is_cursorized(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["self.scan_one(source)?", "Puzzle5dBoardEventsStage::FindMovePart", "Puzzle5dBoardEventsStage::ScanEdge", "Puzzle5dBoardEventsStage::ScanDeleteEdges", "Puzzle5dBoardEventsStage::CloseBrush"] {
        assert!(!complex_retained_route_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing cursor marker was falsely accepted: {marker}");
    }
}

fn focus_selection_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default())"#)
        && source.contains("Puzzle5dFocusSelectionStage::Selection")
        && source.contains("Puzzle5dFocusSelectionStage::Parts")
        && source.contains("Puzzle5dFocusSelectionStage::Publish")
        && source.contains("self.part_cursor += 1")
        && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn focus_selection_hostile_static_law_rejects_whole_selection_reducers() {
    let source = include_str!("../../🦀️.rs");
    assert!(focus_selection_route_is_cursorized(source));
    let direct = source
        .replace(r#""focusSelection" => Box::new(Puzzle5dFocusSelectionWork::default())"#, r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
    assert!(!focus_selection_route_is_cursorized(&direct));
    for marker in ["Puzzle5dFocusSelectionStage::Selection", "Puzzle5dFocusSelectionStage::Parts", "Puzzle5dFocusSelectionStage::Publish", "self.part_cursor += 1"] {
        assert!(!focus_selection_route_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing focus cursor marker was falsely accepted: {marker}");
    }
}

fn scalar_config_routes_are_direct(source: &str) -> bool {
    source.contains("struct Puzzle5dScalarConfigWork")
        && source.contains(
            r#""setCamera"
            | "setCamera2d"
            | "setCamera3d""#,
        )
        && source.contains(r#"| "setSunIntensity" => Box::new(Puzzle5dScalarConfigWork::new(tool_id))"#)
        && source.contains(
            r#"| "engagementInput"
            | "toggleSun""#,
        )
        && source.contains("Puzzle5dConfigMutation::SetCamera2d")
        && source.contains("Puzzle5dConfigMutation::SetBrushCandidateIndex")
        && source.contains("Puzzle5dConfigMutation::SetEngagementInput")
        && source.contains("Puzzle5dConfigMutation::SetGridFactor")
        && source.contains("Puzzle5dConfigMutation::SetOverlapBudget")
        && source.contains("Puzzle5dConfigMutation::SetSun")
        && !source.contains(r#""setCamera" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setSunIntensity" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn scalar_config_hostile_static_law_rejects_old_reducer_and_missing_exact_mutations() {
    let source = include_str!("../../🦀️.rs");
    assert!(scalar_config_routes_are_direct(source));
    for marker in [
        "struct Puzzle5dScalarConfigWork",
        "Puzzle5dConfigMutation::SetCamera2d",
        "Puzzle5dConfigMutation::SetBrushCandidateIndex",
        "Puzzle5dConfigMutation::SetEngagementInput",
        "Puzzle5dConfigMutation::SetGridFactor",
        "Puzzle5dConfigMutation::SetOverlapBudget",
        "Puzzle5dConfigMutation::SetSun",
    ] {
        assert!(!scalar_config_routes_are_direct(&source.replacen(marker, "route-removed", 1)), "missing scalar route marker was falsely accepted: {marker}");
    }
    let direct = source.replace(
        r#"| "setSunIntensity" => Box::new(Puzzle5dScalarConfigWork::new(tool_id))"#,
        r#"| "setSunIntensity" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!scalar_config_routes_are_direct(&direct), "hostile scalar old-reducer replacement must fail closed");
}

fn engagement_abort_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""engagementAbort" => Box::new(Puzzle5dEngagementAbortWork::default())"#)
        && source.contains("Puzzle5dEngagementAbortStage::Input")
        && source.contains("Puzzle5dEngagementAbortStage::BoardUtility")
        && source.contains("Puzzle5dEngagementAbortStage::WorldUtility")
        && source.contains("Puzzle5dEngagementAbortStage::Publish")
        && source.contains("self.effects[0].take()")
        && source.contains("self.effects[1].take()")
        && !source.contains(r#""engagementAbort" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn engagement_abort_hostile_static_law_rejects_old_reducer_and_missing_transfer_boundaries() {
    let source = include_str!("../../🦀️.rs");
    assert!(engagement_abort_route_is_cursorized(source));
    for marker in ["Puzzle5dEngagementAbortStage::Input", "Puzzle5dEngagementAbortStage::BoardUtility", "Puzzle5dEngagementAbortStage::WorldUtility", "Puzzle5dEngagementAbortStage::Publish", "self.effects[0].take()", "self.effects[1].take()"] {
        assert!(!engagement_abort_route_is_cursorized(&source.replacen(marker, "route-removed", 1)), "missing engagement abort marker was falsely accepted: {marker}");
    }
    let direct = source
        .replace(r#""engagementAbort" => Box::new(Puzzle5dEngagementAbortWork::default())"#, r#""engagementAbort" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
    assert!(!engagement_abort_route_is_cursorized(&direct));
}

fn add_part_kind_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""addBrushPart" | "addBrushObject" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id))"#)
        && source.contains("Puzzle5dAddBrushPartStage::Catalog")
        && source.contains("Puzzle5dAddBrushPartStage::Grips")
        && source.contains("Puzzle5dAddBrushPartStage::Target")
        && source.contains("Puzzle5dAddBrushPartStage::Create")
        && source.contains("Puzzle5dAddBrushPartStage::Connect")
        && !source.contains(r#""addPartKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_part_kind_hostile_static_law_rejects_old_brush_reducer_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(add_part_kind_route_is_cursorized(source));
    let direct = source.replace(
        r#""addBrushPart" | "addBrushObject" | "addPartKind" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id))"#,
        r#""addBrushPart" | "addBrushObject" => Box::new(Puzzle5dAddBrushPartWork::new(tool_id)),
            "addPartKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!add_part_kind_route_is_cursorized(&direct));
}

fn kind_weight_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id))"#)
        && source.contains("Puzzle5dKindWeightStage::Catalog")
        && source.contains("Puzzle5dKindWeightStage::InferParts")
        && source.contains("Puzzle5dKindWeightStage::InferGrips")
        && source.contains("Puzzle5dKindWeightStage::Validate")
        && source.contains("Puzzle5dKindWeightStage::SumOthers")
        && source.contains("Puzzle5dKindWeightStage::Build")
        && source.contains("Puzzle5dConfigMutation::SetObjectKindWeights")
        && source.contains("Puzzle5dConfigMutation::SetVortexKindWeights")
        && !source.contains(r#""setObjectKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(kind_weight_route_is_cursorized(source));
    let direct = source.replace(
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle5dKindWeightWork::new(tool_id))"#,
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!kind_weight_route_is_cursorized(&direct));
    assert!(!source.contains("puzzle5d_normalize_kind_weight_group(self.weights"));
}

fn engagement_submit_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""engagementSubmit" => Box::new(Puzzle5dEngagementSubmitWork::default())"#)
        && source.contains("Puzzle5dEngagementSubmitStage::Parse")
        && source.contains("Puzzle5dEngagementSubmitStage::BoardConfig")
        && source.contains("Puzzle5dEngagementSubmitStage::WorldConfig")
        && source.contains("Puzzle5dEngagementSubmitStage::BoardEffect")
        && source.contains("Puzzle5dEngagementSubmitStage::WorldEffect")
        && source.contains("Puzzle5dEngagementSubmitStage::Input")
        && source.contains("Puzzle5dEngagementSubmitStage::Publish")
        && source.contains("Puzzle5dConfigMutation::SetActiveUtility")
        && source.contains("Puzzle5dConfigMutation::SetEngagementInput")
        && !source.contains(r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn engagement_submit_hostile_static_law_rejects_old_reducer_and_missing_transfers() {
    let source = include_str!("../../🦀️.rs");
    assert!(engagement_submit_route_is_cursorized(source));
    for marker in [
        "Puzzle5dEngagementSubmitStage::Parse",
        "Puzzle5dEngagementSubmitStage::BoardConfig",
        "Puzzle5dEngagementSubmitStage::WorldConfig",
        "Puzzle5dEngagementSubmitStage::BoardEffect",
        "Puzzle5dEngagementSubmitStage::WorldEffect",
        "Puzzle5dEngagementSubmitStage::Input",
        "Puzzle5dEngagementSubmitStage::Publish",
    ] {
        assert!(!engagement_submit_route_is_cursorized(&source.replacen(marker, "route-removed", 1)), "missing engagement submit marker was falsely accepted: {marker}");
    }
    let direct = source.replace(
        r#""engagementSubmit" => Box::new(Puzzle5dEngagementSubmitWork::default())"#,
        r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#,
    );
    assert!(!engagement_submit_route_is_cursorized(&direct));
}

fn world_relocate_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default())"#)
        && source.contains("struct Puzzle5dWorldRelocateWork")
        && source.contains("Puzzle5dWorldRelocateStage::SourcePart")
        && source.contains("Puzzle5dWorldRelocateStage::ExistingFasteners")
        && source.contains("Puzzle5dWorldRelocateStage::CandidatePart")
        && source.contains("Puzzle5dWorldRelocateStage::CandidateGrip")
        && source.contains("Puzzle5dWorldRelocateStage::PublishFastener")
        && source.contains("PUZZLE5D_RELOCATE_GRIPS_PER_PART")
        && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn world_relocate_hostile_static_law_rejects_whole_proximity_scans() {
    let source = include_str!("../../🦀️.rs");
    assert!(world_relocate_route_is_cursorized(source));
    let direct =
        source.replace(r#""worldRelocate" => Box::new(Puzzle5dWorldRelocateWork::default())"#, r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle5d_retained_reduce, puzzle5d_retained_extent))"#);
    assert!(!world_relocate_route_is_cursorized(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["Puzzle5dWorldRelocateStage::ExistingFasteners", "Puzzle5dWorldRelocateStage::CandidatePart", "Puzzle5dWorldRelocateStage::CandidateGrip", "Puzzle5dWorldRelocateStage::PublishFastener", "PUZZLE5D_RELOCATE_GRIPS_PER_PART"] {
        assert!(!world_relocate_route_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing world-relocate marker was falsely accepted: {marker}");
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
    assert!(!render_body(&mut app, document_panel::BODY_KEY).is_empty());
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
    let menu = semio_framework::io::resolve_ready(app.context_menu(&request));
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
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty");
    assert_eq!(part_count(&app), 0, "empty example clears the parts");
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
    semio_framework::io::resolve_ready(app.handle_action("redo", None, &meta("local"))).expect("redo");
    assert_eq!(part_count(&app), 0);
}

#[semio_framework_async_macros::async_test]
async fn patch_fastener_updates_transform_offsets_and_undoes() {
    let mut app = app();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin (has fasteners)");
    let projection = projection_of(&app);
    let fastener_id = projection["fasteners"][0]["id"].as_str().expect("seeded fastener").to_string();
    dispatch(&mut app, "patchFastener", Some(&dsl::json!({ "fastenerId": fastener_id, "field": "gap", "value": 2.5 })), None).expect("patch gap");
    let after = projection_of(&app);
    let fastener = after["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener["gap"], 2.5);
    assert_eq!(fastener["shift"], 0.0);
    dispatch(&mut app, "patchFastener", Some(&dsl::json!({ "fastenerId": fastener_id, "field": "rotation", "value": 30.0 })), None).expect("patch rotation");
    let after2 = projection_of(&app);
    let fastener2 = after2["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener2["gap"], 2.5, "earlier gap edit must survive a later rotation edit");
    assert_eq!(fastener2["rotation"], 30.0);
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    let undone = projection_of(&app);
    let fastener3 = undone["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
    assert_eq!(fastener3["rotation"], 0.0, "undo restores the pre-rotation-edit value");
    assert_eq!(fastener3["gap"], 2.5, "undo of rotation edit must not also revert the earlier gap edit");
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
/// `Puzzle5dMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s. Deliberately
/// dispatches through a standalone typed `Puzzle5dStore` — NOT through `Puzzle5dPlayApp`/
/// `Puzzle5dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app's real
/// `ArtifactApp` still uses) — since `Puzzle5dMutation`'s canonical `Mutation<Puzzle5dSnapshot>`
/// impl (not its `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::binary::Puzzle5dStore;
    use crate::{PUZZLE_5D_SCHEMA, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::create_document_envelope;

    let mut store = semio_framework::io::resolve_ready(Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", Puzzle5dSnapshot::default(), None))).expect("store");
    let part = Puzzle5dPart { id: "p1".into(), part_kind: None, anchor: Default::default(), part_2d: Puzzle5dPart2d::default(), part_3d: Puzzle5dPart3d::default(), grips: Vec::new() };
    semio_framework::io::resolve_ready(store.dispatch(store::ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_part(part, None)], description: None })).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle5dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle5dSnapshot, Puzzle5dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️Clipboard
#[semio_framework_async_macros::async_test]
async fn copy_emits_clipboard_fragment_for_the_closed_selection() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let result = semio_framework::io::resolve_ready(app.handle_action("copy", None, &meta("local"))).expect("copy");
    assert!(result.mutations.is_empty(), "copy must not record an undo entry");
    assert_eq!(result.requested_effects.len(), 1);
    let Effect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
    assert_eq!(fragment.source_app, PUZZLE5D_PLAY_APP_ID);
    let fragment_value: serde_json::Value = serde_json::from_str(&fragment.dsl_text).expect("fragment dsl_text is JSON");
    assert_eq!(fragment_value["parts"].as_array().expect("parts").len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn copy_with_no_selection_is_a_benign_no_operation() {
    let mut app = app();
    let result = semio_framework::io::resolve_ready(app.handle_action("copy", None, &meta("local"))).expect("copy");
    assert!(result.mutations.is_empty());
    assert!(result.requested_effects.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn cut_removes_selected_part_and_undo_restores_it() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let before_count = part_count(&app);
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let result = semio_framework::io::resolve_ready(app.handle_action("cut", None, &meta("local"))).expect("cut");
    assert_eq!(result.requested_effects.len(), 1, "cut must also copy to the clipboard");
    assert_eq!(part_count(&app), before_count - 1);
    let after = projection_of(&app);
    assert!(!after["parts"].as_array().unwrap().iter().any(|part| part["id"] == first_part_id));
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert_eq!(part_count(&app), before_count, "one undo restores the cut part as a single edit");
}

#[semio_framework_async_macros::async_test]
async fn paste_materializes_fragment_parts_at_original_anchor_with_fresh_ids() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let projection = projection_of(&app);
    let first_part_id = first_part_id(&app);
    select_id(&mut app, PUZZLE5D_GRANULARITY_PART, &first_part_id).expect("select");
    let copy_result = semio_framework::io::resolve_ready(app.handle_action("copy", None, &meta("local"))).expect("copy");
    let Effect::ClipboardWrite { fragment } = &copy_result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
    let before_count = part_count(&app);
    let before_ids: HashSet<String> = projection["parts"].as_array().unwrap().iter().map(|part| part["id"].as_str().unwrap_or_default().to_string()).collect();
    let paste_args: dsl::DslValue = serde_json::json!({ "fragment": fragment, "anchor": "original", "position": [10.0, 0.0, 0.0] }).into();
    semio_framework::io::resolve_ready(app.handle_action("paste", Some(&paste_args), &meta("local"))).expect("paste");
    assert_eq!(part_count(&app), before_count + 1);
    let after = projection_of(&app);
    let pasted_parts: Vec<&Value> = after["parts"].as_array().unwrap().iter().filter(|part| !before_ids.contains(part["id"].as_str().unwrap_or_default())).collect();
    assert_eq!(pasted_parts.len(), 1);
    // "original" anchor uses the raw position override verbatim as the 2D delta.
    let original_x = projection["parts"][0]["2d"]["x"].as_f64().unwrap_or(0.0);
    assert_eq!(pasted_parts[0]["2d"]["x"].as_f64().unwrap(), original_x + 10.0);
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert_eq!(part_count(&app), before_count, "one undo removes the whole pasted fragment");
}

#[semio_framework_async_macros::async_test]
async fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
    let mut app = app();
    let before_count = part_count(&app);
    let result = semio_framework::io::resolve_ready(app.handle_action("paste", None, &meta("local"))).expect("paste");
    assert!(result.mutations.is_empty());
    assert_eq!(part_count(&app), before_count);
}
//#endregion 🔖️Clipboard

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn app_definition_has_the_paired_windows() {
    let definition = create_puzzle5d_app();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    assert!(ids.contains(&board2d::WINDOW_KIND_ID) && ids.contains(&world3d::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn window_kind_actions_scope_transform_to_3d_only() {
    let definition = create_puzzle5d_app();
    let resolve = |window_id: &str| -> Vec<String> {
        let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
        semio_framework_plugin::resolve_window_actions(&definition, window).into_iter().map(|action| action.id.clone()).collect()
    };
    let board = resolve(board2d::WINDOW_KIND_ID);
    let world = resolve(world3d::WINDOW_KIND_ID);
    for transform_operation in ["translateSelection", "rotateSelection", "scaleSelection", "worldRelocate", "setCamera3d"] {
        assert!(world.contains(&transform_operation.to_string()), "3D must expose {transform_operation}");
        assert!(!board.contains(&transform_operation.to_string()), "2D must NOT expose {transform_operation}");
    }
    assert!(board.contains(&"applyBoardEvents".to_string()), "2D must expose applyBoardEvents");
    assert!(!world.contains(&"applyBoardEvents".to_string()), "3D must NOT expose applyBoardEvents");
    for shared in ["addBrushPart", "deleteSelection"] {
        assert!(board.contains(&shared.to_string()) && world.contains(&shared.to_string()), "{shared} stays on both windows");
    }
}

/// 📑️ The three declared panel tabs must survive the `panel_tab_def` stitch. Asserts PRESENCE
/// only — the framework injects tabs of its own, so a total count would be brittle.
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_its_three_panel_tabs() {
    let definition = create_puzzle5d_app();
    let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
    for body_key in [document_panel::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY] {
        assert!(body_keys.contains(&body_key), "panel tab {body_key} must be declared, got {body_keys:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_cover_both_windows() {
    let mut app = app();
    let engagements = semio_framework::io::resolve_ready(app.window_engagements());
    assert!(engagements.contains_key(board2d::WINDOW_KIND_ID));
    assert!(engagements.contains_key(world3d::WINDOW_KIND_ID));
}

/// 🎯️ Every action id `dispatch_puzzle5d_action` matches on must have a `Puzzle5dCommand` variant
/// under the SAME literal — the two lists are the whole app's dispatch contract and drift between
/// them is silent. (Deliberately NOT the framework's own
/// `assert_declared_actions_bridge_to_commands`, which probes `command_from_action`, the
/// string-dispatch path this app does not implement — its commands carry an opaque `args: Value`,
/// see the `🔖️Puzzle5dCommand` macro.)
#[semio_framework_async_macros::async_test]
async fn every_dispatched_action_bridges_to_a_command() {
    for action in [
        "setFixtureJson",
        "setActiveExample",
        "importComposeKit",
        "selectSameKindSelection",
        "selectSameKind",
        "addNode",
        "addPartKind",
        "deleteSelection",
        "duplicateSelection",
        "setSelectionFlag",
        "patchPart",
        "patchGrip",
        "patchFastener",
        "setCamera",
        "setCamera2d",
        "setCamera3d",
        "zoomToSelection",
        "focusSelection",
        "toggleSun",
        "setSunAzimuth",
        "setSunElevation",
        "setSunIntensity",
        "setLodMode",
        "setGridSnapEnabled",
        "setGridFactor",
        "addBrushPart",
        "addBrushObject",
        "cycleBrushCandidate",
        "registerBrushMesh",
        "setBrushPlacementOverlapBudget",
        "setObjectKindWeight",
        "setVortexKindWeight",
        "engagementControlSelect",
        "setSuggestionOffset",
        "setFillCount",
        "engagementInput",
        "engagementSubmit",
        "engagementAbort",
        "translateSelection",
        "rotateSelection",
        "scaleSelection",
        "worldRelocate",
        "applyBoardEvents",
        "worldPointerDown",
        "canvasPointerDown",
        SET_ACTIVE_UTILITY_ACTION_ID,
    ] {
        assert_eq!(Puzzle5dCommand::from_action(action, None, None).action_id(), action, "dispatched action {action} must have a Puzzle5dCommand variant");
    }
}
//#endregion 🔖️Manifest

//#region 🧰️ Window Actions & Utilities contract
#[semio_framework_async_macros::async_test]
async fn add_part_kind_materializes_the_declared_kind_default() {
    // 📝️ P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&dsl::json!({ "exampleId": "" })), None).expect("empty");
    let before = part_count(&app);
    let result = dispatch(&mut app, "addPartKind", None, None).expect("addPartKind");
    assert!(!result.mutations.is_empty(), "addPartKind is a Mutation that emits mutations");
    assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
    let projection = projection_of(&app);
    let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
    assert_eq!(kind, Some("Part"), "the declared partKind default was materialized host-side");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework View action: no document operations, no undo entry, no re-emitted effect.
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&dsl::json!({ "utilityId": "brush" })), None).expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn set_camera_actions_write_runtime_and_emit_no_operations() {
    // 📷️ Camera pose is session-only view state (`ActionKind::View`): `setCamera2d`/`setCamera3d`
    // must mutate the app's runtime (visible via the rendered scene) without ever touching the
    // VCS-tracked document or emitting an operation.
    let mut app = app();
    let before = projection_of(&app);
    let camera2d_result = dispatch(&mut app, "setCamera2d", Some(&dsl::json!({ "camera": { "x": 12.5, "y": -6.5, "zoom": 3.5 } })), None).expect("setCamera2d");
    assert!(camera2d_result.mutations.is_empty(), "setCamera2d is a View action and must never emit a document operation");
    assert_eq!(projection_of(&app), before, "setCamera2d must not mutate the document");
    let board = render_body(&mut app, board2d::BODY_KEY);
    assert!(board.contains("12.5") && board.contains("-6.5"), "the new 2D camera pose must be reflected in the rendered runtime state");
    let camera3d_result = dispatch(&mut app, "setCamera3d", Some(&dsl::json!({ "camera": { "position": [42.5, 7.5, 3.5], "target": [1.5, 2.5, 3.5], "zoom": 5.5 } })), None).expect("setCamera3d");
    assert!(camera3d_result.mutations.is_empty(), "setCamera3d is a View action and must never emit a document operation");
    assert_eq!(projection_of(&app), before, "setCamera3d must not mutate the document");
    let world = render_body(&mut app, world3d::BODY_KEY);
    assert!(world.contains("42.5") && world.contains("7.5") && world.contains("1.5"), "the new 3D camera pose must be reflected in the rendered runtime state");
}

#[semio_framework_async_macros::async_test]
async fn engagements_expose_no_utility_switch_options_for_either_window() {
    // 🧰️ select/brush/fill switching lives only on the framework utility bar; neither the 2D nor the 3D
    // engagement HUD may duplicate it as options.
    let mut app = app();
    let engagements = semio_framework::io::resolve_ready(app.window_engagements());
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose utility switching as options");
    }
}

/// 🎯️ D-3 follow-up: the fill-count slider and brush placement picker are tagged `WindowMeasure::Group`s
/// in each window's `window_measures` (surfaced by `partition_window_measures` only for their active
/// utility), never `WindowEngagementControl`s on the HUD — for both the 2D and 3D windows.
#[semio_framework_async_macros::async_test]
async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
    let labels = puzzle5d_labels(&semio_framework_plugin::ViewModel::default());
    let session = Puzzle5dPrecomputeSession::new();
    // 🪣️ Fill utility: the fill-count slider lives in a "fill"-tagged Utility Options group (per window),
    // NOT the engagement HUD.
    let fill_runtime = Puzzle5dRuntime { fill_count: 3, ..Default::default() };
    let fill_scene = Puzzle5dScene { document: default_document(), runtime: fill_runtime, active_utility: "fill".into() };
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&fill_scene, &session, labels) } else { world3d::window_measures(&fill_scene, &session, labels) };
        assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-fill"), Some(Some("fill".into())), "{window} fill Utility Options must be tagged for the fill utility");
        assert!(has_measure_slider(&measures, "puzzle5d-fill-count"), "{window} fill Utility Options must carry the fill-count slider");
        let fill_hud = edit::puzzle5d_engagement(&fill_scene, window, labels);
        assert!(fill_hud.control.is_none() && fill_hud.controls.is_none(), "{window} fill engagement HUD must no longer carry the relocated control");
    }
    // 🖌️ Brush utility: with no candidates to place, the "brush"-tagged group still surfaces (matching the
    // old gate), and the engagement HUD is likewise bare.
    let brush_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: "brush".into() };
    for window in [board2d::WINDOW_KIND_ID, world3d::WINDOW_KIND_ID] {
        let measures = if window == board2d::WINDOW_KIND_ID { board2d::window_measures(&brush_scene, &session, labels) } else { world3d::window_measures(&brush_scene, &session, labels) };
        assert_eq!(measure_group_tag(&measures, "puzzle5d-play-utility-options-brush"), Some(Some("brush".into())), "{window} brush Utility Options surfaces even without candidates");
        let brush_hud = edit::puzzle5d_engagement(&brush_scene, window, labels);
        assert!(brush_hud.control.is_none() && brush_hud.controls.is_none(), "{window} brush engagement HUD must no longer carry the relocated control");
    }
}

#[semio_framework_async_macros::async_test]
async fn engagement_submit_switches_utility_via_host_effect_for_both_windows() {
    // 🧰️ Reconciled dual entry point: the engagement token drives the same host-owned utility switch, once per window.
    let mut app = app();
    let result = dispatch(&mut app, "engagementSubmit", Some(&dsl::json!({ "window": world3d::WINDOW_KIND_ID, "value": "brush" })), None).expect("submit");
    let windows: Vec<&str> = result
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::SetActiveUtility { window_id, utility_id } if utility_id == "brush" => Some(window_id.as_str()),
            _ => None,
        })
        .collect();
    assert!(windows.contains(&board2d::WINDOW_KIND_ID) && windows.contains(&world3d::WINDOW_KIND_ID), "brush switch is pushed to both windows, got {windows:?}");
}

#[semio_framework_async_macros::async_test]
async fn gumball_translate_drag_coalesces_into_one_edit() {
    // 🌀️ Coalescing regression: three translate ticks with the same key are ONE undoable edit.
    let mut app = app();
    let part_id = first_part_id(&app);
    let origin_x = |app: &Puzzle5dApp| -> f64 {
        projection_of(app)
            .get("parts")
            .and_then(Value::as_array)
            .and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(part_id.as_str())).cloned())
            .and_then(|part| part.pointer("/3d/origin/0").and_then(Value::as_f64))
            .unwrap_or(0.0)
    };
    let start = origin_x(&app);
    for dx in [1.0, 2.0, 3.0] {
        dispatch(&mut app, "translateSelection", Some(&dsl::json!({ "ids": [part_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), None).expect("drag tick");
    }
    assert!((origin_x(&app) - start - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
    semio_framework::io::resolve_ready(app.handle_action("undo", None, &meta("local"))).expect("undo");
    assert!((origin_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced gumball drag");
}
//#endregion 🧰️ Window Actions & Utilities contract

//#region 🔖️KitInPort
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_dispatches_the_exact_factory_and_applies_canonical_output() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let fragment = serde_json::json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "retained-capsule",
            "name": "retained-capsule",
            "label": "Retained Capsule",
            "meshUrl": "/mesh/retained-capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "retained-door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "retained-door", "name": "retained-door", "label": "Retained Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "retained-door", "target": "retained-door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };
    let result = app.import_media("kit:in", media, &meta("local")).await.expect("retained kit:in dispatch");
    assert!(!result.mutations.is_empty(), "exact retained import must publish document mutations");
    let after = projection_of(&app);
    assert_ne!(after, before, "exact retained import must apply its completion output");
    let snapshot: Puzzle5dSnapshot = dsl::json::from_json_str(&after.to_string()).expect("retained projection deserializes");
    let catalogs = crate::kind_catalogs_of(&snapshot.kind_catalogs, &snapshot.kind_catalogs_extra).expect("retained catalog replacement applied");
    let part = catalogs.parts.iter().find(|part| part.id == "retained-capsule").expect("retained part catalog row");
    assert_eq!(part.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("retained-door"));
    assert!(catalogs.grips.iter().any(|grip| grip.id == "retained-door"));
}

#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode() {
    let prefix = r#"{"objectKinds":[],"vortexKinds":[{"id":"grip","name":"Grip","label":""#;
    let suffix = r##"","color":"#fff","defaultCableKind":""}],"kindCompatibility":[]}"##;
    let label = "x".repeat(PUZZLE5D_IMPORT_MEDIA_BYTES.checked_sub(prefix.len() + suffix.len()).expect("retained max fixture shell"));
    let maximum = format!("{prefix}{label}{suffix}");
    assert_eq!(maximum.len(), PUZZLE5D_IMPORT_MEDIA_BYTES);
    let mut app = app_with_registry();
    let exact = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: maximum.clone() } };
    app.import_media("kit:in", exact, &meta("local")).await.expect("exact media maximum retained dispatch");
    let after_exact = projection_of(&app);
    let plus_one = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: format!("{maximum} ") } };
    let error = app.import_media("kit:in", plus_one, &meta("local")).await.expect_err("media maximum plus one must fail before Serde");
    assert!(error.message.contains("predecode cap"));
    assert_eq!(projection_of(&app), after_exact, "rejected plus-one media must not mutate the document");
}

/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like
/// block3d's `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle5dPlayApp::import_media` must normalize `objectKinds` into the typed
/// `kindCatalogs.parts` (with each per-object `vortices[]` entry becoming a grip template) and
/// `vortexKinds` into `kindCatalogs.grips`, and land both after applying the returned operations.
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_upserts_part_and_grip_kinds_into_kind_catalogs() {
    let mut app = app_with_registry();
    let fragment = serde_json::json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "capsule",
            "name": "capsule",
            "label": "Capsule",
            "meshUrl": "/mesh/capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    let result = app.import_media("kit:in", media, &meta("local")).await.expect("retained kit:in import succeeds");
    assert!(!result.mutations.is_empty(), "importing a non-empty fragment must emit real operations");
    let next_projection = projection_of(&app);

    // 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: `next_projection`'s raw
    // `kindCatalogs` key is now the composed `{childId,target}` handle, not the embedded
    // `{parts:[...],...}` shape a JSON pointer could probe directly — reassemble the full
    // `Puzzle5dKindCatalogs` through the typed snapshot + `kind_catalogs_of` accessor instead
    // (same pattern `sourcing`'s `stock_of` established for its own composed catalog field).
    let next_snapshot: Puzzle5dSnapshot = dsl::json::from_json_str(&next_projection.to_string()).expect("next_projection deserializes as Puzzle5dSnapshot");
    let catalogs = crate::kind_catalogs_of(&next_snapshot.kind_catalogs, &next_snapshot.kind_catalogs_extra).expect("parts catalog present");
    let capsule = catalogs.parts.iter().find(|entry| entry.id == "capsule").expect("the imported part kind must appear in kindCatalogs.parts");
    assert_eq!(capsule.representations.first().map(|representation| representation.url.as_str()), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.grips.first().and_then(|grip| grip.grip_kind.as_deref()), Some("door"), "the per-part grip template keeps its gripKind after normalization");
    assert_eq!(capsule.grips.first().map(|grip| grip.point), Some([0.0, 0.0, 0.0]));
    assert_eq!(capsule.grips.first().map(|grip| grip.direction), Some([0.0, 1.0, 0.0]));
    assert_eq!(capsule.grips.first().and_then(|grip| grip.radius), Some(0.3));

    let door = catalogs.grips.iter().find(|entry| entry.id == "door").expect("the imported grip kind must appear in kindCatalogs.grips");
    assert_eq!(door.default_rope_kind.as_str(), "", "defaultCableKind maps onto defaultRopeKind (a naming judgment call — see import_media's doc comment)");

    let compatibility = next_projection.pointer("/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (simulating a second producer edge, or a redelivered
/// message on a `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_retained_import_media_is_idempotent_on_repeated_delivery() {
    let mut app = app_with_registry();
    let fragment = serde_json::json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        app.import_media("kit:in", media.clone(), &meta("local")).await.expect("retained kit:in import succeeds");
    }

    let current = projection_of(&app);
    let current_snapshot: Puzzle5dSnapshot = dsl::json::from_json_str(&current.to_string()).expect("current deserializes as Puzzle5dSnapshot");
    let catalogs = crate::kind_catalogs_of(&current_snapshot.kind_catalogs, &current_snapshot.kind_catalogs_extra).expect("parts catalog present");
    assert_eq!(catalogs.parts.iter().filter(|entry| entry.id == "capsule").count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle5dPlayApp::io().expect("puzzle5d declares an AppIo");
    let kit_in = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(kit_in.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(kit_in.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(kit_in.multiplicity, PortMultiplicity::Many));
    let design_out = io.ports.iter().find(|port| port.id == "design:out").expect("design:out port declared");
    assert_eq!(design_out.kind_id.as_deref(), Some("5d.puzzle"));
    assert_eq!(design_out.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Design });
    assert!(matches!(design_out.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort
