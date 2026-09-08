
use super::testkit::*;
use super::*;

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END: reproduces `runtime live cleanup faulted for
/// instance 1` — the vortex-picking local interaction query is the only path by which the
/// puzzle3d document store's `snapshot_read()` lease is ever taken and returned, so it is
/// driven to completion here (Started → terminal Page → acknowledge → Closed) exactly as the
/// host does every actor turn, then one plain `maintenance_step` must not fault.
#[test]
fn local_interaction_query_return_does_not_fault_the_next_maintenance_step() {
    use semio_framework_plugin::PluginApp;
    let mut app = app_with_registry();
    semio_framework::io::resolve_ready(app.bind_instance_id(1));
    assert!(app.begin_local_interaction_query(1, 1).is_none(), "a fresh instance must admit its first local interaction query");
    let mut started = false;
    let mut terminal_token: Option<protocol::LocalInteractionQueryToken> = None;
    for _ in 0..2000 {
        if let Some(reply) = app.take_local_interaction_query_reply() {
            match reply {
                protocol::LocalInteractionQueryReply::Started { .. } => started = true,
                protocol::LocalInteractionQueryReply::Page { page } => {
                    if page.terminal {
                        terminal_token = Some(protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal });
                    }
                }
                other => panic!("unexpected local interaction reply before its terminal page: {other:?}"),
            }
        }
        if terminal_token.is_some() {
            break;
        }
        semio_framework::io::resolve_ready(app.advance_typed_operation_publication()).expect("advance local interaction query toward its terminal page");
    }
    assert!(started, "local interaction query never reached Started");
    let token = terminal_token.expect("local interaction query never produced a terminal page");
    assert!(app.acknowledge_local_interaction_query(&token), "terminal page token must be accepted");
    let mut closed = false;
    for _ in 0..2000 {
        if matches!(app.take_local_interaction_query_reply(), Some(protocol::LocalInteractionQueryReply::Closed { .. })) {
            closed = true;
            break;
        }
        semio_framework::io::resolve_ready(app.advance_typed_operation_publication()).expect("advance local interaction query toward its returned snapshot leases");
    }
    assert!(closed, "local interaction query never closed and returned its captured snapshot leases");
    app.maintenance_step(1, 4096).expect("maintenance step after a returned snapshot read lease must not fault");
}

#[test]
fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
    let fixture: Value = parse(include_str!("../../../🗄️retained-jobs/🔣️.json")).expect("Puzzle3D retained route fixture");
    assert_eq!(fixture.get("toolIds"), Some(&Value::Array(PUZZLE3D_RETAINED_TOOL_IDS.iter().map(|id| Value::from(*id)).collect())));
    let manifest = create_puzzle3d_app();
    for tool_id in PUZZLE3D_RETAINED_TOOL_IDS {
        let actions = manifest.window_kinds.iter().flat_map(|window| &window.actions).filter(|action| action.id == *tool_id).collect::<Vec<_>>();
        assert_eq!(actions.len(), 1, "{tool_id} requires exactly one manifest declaration");
        assert_eq!(actions[0].semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{tool_id}");
    }
    let exact = |contracts: &[ArtifactToolPublicationContract]| {
        let ids = contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>();
        ids == PUZZLE3D_RETAINED_TOOL_IDS.iter().copied().collect()
            && ids.len() == contracts.len()
            && contracts.iter().all(|contract| !contract.lanes.is_empty() && (!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1))
    };
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert!(exact(contracts));
    assert!(!exact(&contracts[..contracts.len() - 1]));
    let mut duplicate = contracts.to_vec();
    let copied = duplicate[1];
    duplicate[0] = copied;
    assert!(!exact(&duplicate));
}

#[test]
fn retained_command_catalog_excludes_framework_owned_shared_actions() {
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_TOOL_ACTION_ID));
    assert!(!PUZZLE3D_RETAINED_TOOL_IDS.contains(&SET_ACTIVE_UTILITY_ACTION_ID));
}

fn suggestion_and_precompute_routes_are_cursorized(source: &str) -> bool {
    [
        r#""acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default())"#,
        r#"| "suggestionsTick" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))"#,
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Candidate",
        "Puzzle3dAcceptSuggestionStage::Representation",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "Puzzle3dPrecomputeCommandStage::CheckpointBytes",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ]
    .into_iter()
    .all(|marker| source.contains(marker))
        && !source.contains(r#""acceptSuggestion" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""cycleBrushCandidate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""fillBuildTick" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""registerBrushMesh" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setFillCount" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""suggestionsTick" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn suggestion_and_precompute_hostile_static_law_rejects_one_grant_reducers_and_missing_boundaries() {
    let source = include_str!("../../🦀️.rs");
    assert!(suggestion_and_precompute_routes_are_cursorized(source));
    let direct_accept = source.replace(
        r#""acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default())"#,
        r#""acceptSuggestion" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_accept));
    let direct_precompute = source.replace(
        r#"| "suggestionsTick" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))"#,
        r#"| "suggestionsTick" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!suggestion_and_precompute_routes_are_cursorized(&direct_precompute));
    for marker in [
        "Puzzle3dAcceptSuggestionStage::Target",
        "Puzzle3dAcceptSuggestionStage::Representation",
        "Puzzle3dAcceptSuggestionStage::Vortices",
        "Puzzle3dAcceptSuggestionStage::ExistingAttractions",
        "Puzzle3dAcceptSuggestionStage::PublishObject",
        "Puzzle3dAcceptSuggestionStage::PublishAttraction",
        "Puzzle3dPrecomputeCommandStage::Objects",
        "Puzzle3dPrecomputeCommandStage::Vortices",
        "Puzzle3dPrecomputeCommandStage::Attractions",
        "Puzzle3dPrecomputeCommandStage::CatalogObjects",
        "Puzzle3dPrecomputeCommandStage::CatalogVortices",
        "Puzzle3dPrecomputeCommandStage::Positions",
        "Puzzle3dPrecomputeCommandStage::Indices",
        "Puzzle3dPrecomputeCommandStage::CheckpointBytes",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ] {
        assert!(!suggestion_and_precompute_routes_are_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing retained boundary was falsely accepted: {marker}");
    }
}

fn selection_transforms_are_cursorized(source: &str) -> bool {
    source.contains("\"translateSelection\" | \"rotateSelection\" | \"scaleSelection\" => Box::new(Puzzle3dScaleWork::new(tool_id))")
        && source.contains("Puzzle3dScaleStage::ObjectSelection")
        && source.contains("Puzzle3dScaleStage::VolumeSelection")
        && source.contains("Puzzle3dScaleStage::Objects")
        && source.contains("Puzzle3dScaleStage::Volumes")
        && !source.contains("\"translateSelection\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
        && !source.contains("\"rotateSelection\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork")
}

#[test]
fn selection_transform_hostile_static_law_rejects_one_grant_reducers_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(selection_transforms_are_cursorized(source));
    let direct = source.replace(
        "\"translateSelection\" | \"rotateSelection\" | \"scaleSelection\" => Box::new(Puzzle3dScaleWork::new(tool_id))",
        "\"translateSelection\" | \"rotateSelection\" | \"scaleSelection\" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))",
    );
    assert!(!selection_transforms_are_cursorized(&direct), "hostile old-reducer replacement must fail closed");
    for marker in ["Puzzle3dScaleStage::ObjectSelection", "Puzzle3dScaleStage::VolumeSelection", "Puzzle3dScaleStage::Objects", "Puzzle3dScaleStage::Volumes"] {
        assert!(!selection_transforms_are_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing transform cursor was falsely accepted: {marker}");
    }
}

fn focus_selection_is_cursorized(source: &str) -> bool {
    source.contains(r#""focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default())"#)
        && source.contains("Puzzle3dFocusSelectionStage::Selection")
        && source.contains("Puzzle3dFocusSelectionStage::SumObjects")
        && source.contains("Puzzle3dFocusSelectionStage::DistanceObjects")
        && source.contains("Puzzle3dFocusSelectionStage::Publish")
        && !source.contains(r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn focus_selection_hostile_static_law_rejects_hidden_whole_collection_work() {
    let source = include_str!("../../🦀️.rs");
    assert!(focus_selection_is_cursorized(source));
    let direct = source
        .replace(r#""focusSelection" => Box::new(Puzzle3dFocusSelectionWork::default())"#, r#""focusSelection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!focus_selection_is_cursorized(&direct));
    for marker in ["Puzzle3dFocusSelectionStage::Selection", "Puzzle3dFocusSelectionStage::SumObjects", "Puzzle3dFocusSelectionStage::DistanceObjects", "Puzzle3dFocusSelectionStage::Publish"] {
        assert!(!focus_selection_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing focus cursor was falsely accepted: {marker}");
    }
}

fn patch_inspector_is_cursorized(source: &str) -> bool {
    source.contains(r#""patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default())"#)
        && source.contains("Puzzle3dPatchInspectorStage::Selection")
        && source.contains("Puzzle3dPatchInspectorStage::Objects")
        && source.contains("Puzzle3dPatchInspectorStage::Vortices")
        && source.contains("Puzzle3dPatchInspectorStage::Attractions")
        && source.contains("Puzzle3dPatchInspectorStage::AttractionReconnect")
        && source.contains("Puzzle3dPatchInspectorStage::References")
        && source.contains("Puzzle3dPatchInspectorStage::Volumes")
        && !source.contains(r#""patchInspector" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn patch_inspector_hostile_static_law_rejects_old_reducer_and_hidden_collection_loops() {
    let source = include_str!("../../🦀️.rs");
    assert!(patch_inspector_is_cursorized(source));
    let direct = source
        .replace(r#""patchInspector" => Box::new(Puzzle3dPatchInspectorWork::default())"#, r#""patchInspector" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!patch_inspector_is_cursorized(&direct));
    for marker in [
        "Puzzle3dPatchInspectorStage::Selection",
        "Puzzle3dPatchInspectorStage::Objects",
        "Puzzle3dPatchInspectorStage::Vortices",
        "Puzzle3dPatchInspectorStage::Attractions",
        "Puzzle3dPatchInspectorStage::AttractionReconnect",
        "Puzzle3dPatchInspectorStage::References",
        "Puzzle3dPatchInspectorStage::Volumes",
    ] {
        assert!(!patch_inspector_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing inspector cursor was falsely accepted: {marker}");
    }
}

fn world_relocate_is_cursorized(source: &str) -> bool {
    source.contains(r#""worldRelocate" => Box::new(Puzzle3dWorldRelocateWork::default())"#)
        && source.contains("Puzzle3dWorldRelocateStage::Object")
        && source.contains("Puzzle3dWorldRelocateStage::ExistingAttractions")
        && source.contains("Puzzle3dWorldRelocateStage::CandidateObject")
        && source.contains("Puzzle3dWorldRelocateStage::CandidateVortex")
        && source.contains("Puzzle3dWorldRelocateStage::PublishAttraction")
        && source.contains("PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT")
        && !source.contains(r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn world_relocate_hostile_static_law_rejects_whole_proximity_scans() {
    let source = include_str!("../../🦀️.rs");
    assert!(world_relocate_is_cursorized(source));
    let direct =
        source.replace(r#""worldRelocate" => Box::new(Puzzle3dWorldRelocateWork::default())"#, r#""worldRelocate" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!world_relocate_is_cursorized(&direct));
    for marker in [
        "Puzzle3dWorldRelocateStage::Object",
        "Puzzle3dWorldRelocateStage::ExistingAttractions",
        "Puzzle3dWorldRelocateStage::CandidateObject",
        "Puzzle3dWorldRelocateStage::CandidateVortex",
        "Puzzle3dWorldRelocateStage::PublishAttraction",
        "PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT",
    ] {
        assert!(!world_relocate_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing relocate cursor was falsely accepted: {marker}");
    }
}

fn create_attraction_is_cursorized(source: &str) -> bool {
    source.contains(r#""createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default())"#)
        && source.contains("Puzzle3dCreateAttractionStage::Existing")
        && source.contains("Puzzle3dCreateAttractionStage::Attracting")
        && source.contains("Puzzle3dCreateAttractionStage::Attracted")
        && source.contains("Puzzle3dCreateAttractionStage::Compatibility")
        && source.contains("Puzzle3dCreateAttractionStage::Publish")
        && !source.contains(r#""createAttraction" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn create_attraction_hostile_static_law_rejects_nested_whole_scans() {
    let source = include_str!("../../🦀️.rs");
    assert!(create_attraction_is_cursorized(source));
    let direct = source.replace(
        r#""createAttraction" => Box::new(Puzzle3dCreateAttractionWork::default())"#,
        r#""createAttraction" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!create_attraction_is_cursorized(&direct));
    for marker in ["Puzzle3dCreateAttractionStage::Existing", "Puzzle3dCreateAttractionStage::Attracting", "Puzzle3dCreateAttractionStage::Attracted", "Puzzle3dCreateAttractionStage::Compatibility", "Puzzle3dCreateAttractionStage::Publish"] {
        assert!(!create_attraction_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing attraction cursor was falsely accepted: {marker}");
    }
}

fn set_active_example_is_cursorized(source: &str) -> bool {
    source.contains(r#""setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default())"#)
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::DeleteCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateObjects")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateAttractions")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateVolumes")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateReferences")
        && source.contains("Puzzle3dSetActiveExampleStage::CreateCompatibility")
        && source.contains("Puzzle3dSetActiveExampleStage::Publish")
        && !source.contains(r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn set_active_example_hostile_static_law_rejects_whole_document_reset() {
    let source = include_str!("../../🦀️.rs");
    assert!(set_active_example_is_cursorized(source));
    let direct = source.replace(
        r#""setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default())"#,
        r#""setActiveExample" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!set_active_example_is_cursorized(&direct));
    for marker in [
        "Puzzle3dSetActiveExampleStage::DeleteAttractions",
        "Puzzle3dSetActiveExampleStage::DeleteObjects",
        "Puzzle3dSetActiveExampleStage::DeleteVolumes",
        "Puzzle3dSetActiveExampleStage::DeleteReferences",
        "Puzzle3dSetActiveExampleStage::DeleteCompatibility",
        "Puzzle3dSetActiveExampleStage::CreateObjects",
        "Puzzle3dSetActiveExampleStage::CreateAttractions",
        "Puzzle3dSetActiveExampleStage::CreateVolumes",
        "Puzzle3dSetActiveExampleStage::CreateReferences",
        "Puzzle3dSetActiveExampleStage::CreateCompatibility",
        "Puzzle3dSetActiveExampleStage::Publish",
    ] {
        assert!(!set_active_example_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing example cursor was falsely accepted: {marker}");
    }
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dWorldRelocateWork::extent` used to charge
/// every object a flat `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT` (64) regardless of its real vortex
/// count, so Nakagin's 180 objects (358 real vortex instances) computed `objects * 66 + attractions`
/// = 11,880 and faulted preflight with "puzzle command exceeds fixed semantic work capacity" before
/// `step()` ever ran. The rewritten bound counts the document's actual vortices instead.
#[test]
fn world_relocate_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "nonexistent", "position": [0.0, 0.0, 0.0] })), None).expect("worldRelocate command decodes");
    let work = Puzzle3dWorldRelocateWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "worldRelocate extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): the extent test above only
/// proves the bound is REALISTIC (fits under the cap) — it says nothing about whether the bound is
/// SOUND (real `step()` calls never outrun it). This drives the actual `CandidateVortex ⇄
/// PublishAttraction` cycle (the genuine ping-pong at editor `🦀️.rs` `Puzzle3dWorldRelocateStage::
/// CandidateVortex`/`PublishAttraction`) against a real Nakagin joint: object
/// `25b0dba0-8f81-423a-94a1-b911a6031010` ("Capsule With Balcony Backslash") is "relocated" to its
/// own current origin — a real, non-degenerate command, not a synthetic no-op — and its one vortex
/// (`…:link`, kind "door capsule right") sits, by the fixture's own real assembled geometry
/// (independently recomputed here with the same `quat_rotate_vector` Hamilton-product formula
/// `step()` uses), within `proximity_radius` (0.75, `default_proximity_radius()`) of four
/// "door tambour right" vortices already on neighbour object `5f0266bc-…`. That forces the
/// `PublishAttraction` loop-back more than once, closing the gap a trivial early-exit run leaves
/// open, and empirically exercises the `candidate_scan_stage`'s `object_vortices * 2` term this
/// ticket introduced.
#[test]
fn world_relocate_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "25b0dba0-8f81-423a-94a1-b911a6031010", "position": [-8.85, -2.8499999999999996, 7.7] })), None).expect("worldRelocate command decodes");
    let mut work = Puzzle3dWorldRelocateWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "worldRelocate step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert!(iterations <= extent, "worldRelocate step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(
        iterations > 100,
        "worldRelocate's CandidateObject/CandidateVortex stages unconditionally walk every real object and vortex, so a real Nakagin run must take hundreds of steps, not a trivial early exit; observed only {iterations} iterations"
    );
    assert!(
        emit.artifact_mutations.len() >= 2,
        "the relocated vortex sits within proximity_radius of a real neighbour already present in the Nakagin fixture, so PublishAttraction must fire at least once beyond the move itself; observed {} mutations",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dCreateAttractionWork::extent` doubled the
/// same flat per-object 64-vortex charge across two endpoint scans, computing `objects * 128 + ...`
/// = 23,055 on Nakagin's 180 objects — nearly 6x the cap. The rewritten bound counts the document's
/// actual vortices instead of assuming every object carries the worst-case vortex count.
#[test]
fn create_attraction_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "nonexistent-a", "attracted": "nonexistent-b" })), None).expect("createAttraction command decodes");
    let work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "createAttraction extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real
/// `Attracting`/`Attracted` full-document vortex scans instead of only checking the bound fits
/// under the cap. `attracting`/`attracted` name a genuine compatible pair already present in the
/// Nakagin fixture — `25b0dba0-…:link` (kind "door capsule right", object index 27 in DSL
/// declaration order) and `5f0266bc-…:sl0_d0` (kind "door tambour right", object index 64) — which
/// `kind-compatibility` marks bidirectionally compatible, so `step()` runs every real stage
/// (`Existing → Attracting → Attracted → Compatibility → Publish`) to a genuine success instead of
/// one of the Work's several early-`Complete` short-circuits (duplicate/incompatible/empty-id).
#[test]
fn create_attraction_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command =
        Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "25b0dba0-8f81-423a-94a1-b911a6031010:link", "attracted": "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952:sl0_d0" })), None).expect("createAttraction command decodes");
    let mut work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "createAttraction step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert!(iterations <= extent, "createAttraction step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(iterations > 50, "createAttraction must scan real objects and vortices to find both endpoints, not take a trivial early exit; observed only {iterations} iterations");
    assert_eq!(
        emit.artifact_mutations.len(),
        1,
        "a real, compatible, non-duplicate attracting/attracted pair must reach Publish and emit exactly one connect_vortices mutation, not one of the early-Complete short-circuits; observed {}",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dAcceptSuggestionWork::extent`'s
/// `target_scans` term charged every scene object the same flat 64-vortex worst case
/// (`objects * 64`), computing 7,888+ on Nakagin's 180 objects. The rewritten bound counts the
/// document's actual vortices for the target scan while keeping the catalog-kind-cap terms
/// (`PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT`), which bound a real runtime-enforced invariant on
/// catalog kinds, not scene object instances.
#[test]
fn accept_suggestion_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", None, None).expect("acceptSuggestion command decodes");
    let work = Puzzle3dAcceptSuggestionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "acceptSuggestion extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real `Target`
/// scan (the term this ticket rewrote) plus the full `Candidate → Representation → Vortices →
/// ExistingAttractions → PublishObject → PublishAttraction → PublishResult` chain to a genuine
/// success, using a real Nakagin vortex (`25b0dba0-…:link`, object index 27 in DSL declaration
/// order) as `fullId` instead of the "no target requested" early-`Complete` short-circuit.
#[test]
fn accept_suggestion_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), None).expect("acceptSuggestion command decodes");
    let mut work = Puzzle3dAcceptSuggestionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "acceptSuggestion step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert!(iterations <= extent, "acceptSuggestion step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(iterations > 20, "acceptSuggestion must scan real objects to find the requested target vortex, not take a trivial early exit; observed only {iterations} iterations");
    assert_eq!(
        emit.artifact_mutations.len(),
        2,
        "a real target vortex and a real catalog kind must reach PublishResult and emit exactly the created object plus its connecting attraction, not one of the early-Complete short-circuits; observed {}",
        emit.artifact_mutations.len()
    );
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dPatchInspectorWork::extent`'s `"vortex"`
/// arm charged `objects * PUZZLE_COMMAND_DECODED_ITEMS` (objects * 512) regardless of real vortex
/// count, computing ~92,160 on Nakagin's 180 objects — a 512x-over-budget design error, not a
/// precision edge case. The rewritten bound counts the document's actual vortex instances plus one
/// `step()` per object, matching the `Vortices` stage's real per-object "owner advance" call.
#[test]
fn patch_inspector_vortex_extent_fits_within_cap_for_nakagin() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex" })), None).expect("patchInspector command decodes");
    let work = Puzzle3dPatchInspectorWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "patchInspector vortex extent {extent} must not exceed the fixed cap {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): the `Vortices` stage walks
/// every object and every real vortex UNCONDITIONALLY (selection only gates whether a visited
/// vortex is mutated, never whether it is visited), so this is a genuine full-document run by
/// construction, not a trivial early exit — selecting one real vortex
/// (`25b0dba0-…:link`) and setting `field: "hidden"` proves the walk both completes within its own
/// extent AND actually mutates the one entity that was selected out of the full scan.
///
/// Writing this test surfaced a real off-by-one in `extent()`'s own `Selection` accounting: the
/// `Selection` stage's own exhaustion call (`source_id` returning `None`) is a real `step()` call
/// that returns `Progress` — a `+1` distinct from each entity arm's own terminal call (which
/// returns `Complete` instead, contributing zero to the `Progress` count `work_cursor` tracks). The
/// old `let items = source.checked_add(scan)?;` omitted that `Selection`-stage `+1` entirely; fixed
/// to `let items = source.checked_add(scan)?.checked_add(1)?;`, which this test's `iterations <=
/// extent` assertion is what actually catches — the extent-only test above could not have (it
/// never drives the real loop, so no test previously computed the true `Progress`-call count).
#[test]
fn patch_inspector_vortex_step_loop_stays_within_its_own_extent_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into());
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex", "field": "hidden", "value": true, "ids": ["25b0dba0-8f81-423a-94a1-b911a6031010:link"] })), None).expect("patchInspector command decodes");
    let mut work = Puzzle3dPatchInspectorWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "patchInspector step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert!(iterations <= extent, "patchInspector step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(
        iterations > 400,
        "patchInspector's Vortices stage unconditionally walks every object and every real vortex regardless of selection, so a real Nakagin run must take hundreds of steps, not a trivial early exit; observed only {iterations} iterations"
    );
    assert_eq!(emit.artifact_mutations.len(), 1, "exactly the one selected vortex must be patched out of the full unconditional scan; observed {} mutations", emit.artifact_mutations.len());
}

/// 🧵️ ticket 26/09/02/PUZZLE-3D-END-TO-END: direct functional proof that the previously-dead
/// `Puzzle3dSetActiveExampleWork` state machine actually walks its own stages across MULTIPLE
/// bounded `step()` calls for a real fixture (not a single-shot reducer), accumulating exactly
/// one mutation per deleted/created item and only publishing them all in the final `Complete`
/// emit — the same turn-by-turn contract `crate::retained_command::RetainedPuzzleCommandJob`
/// relies on when it drives an app-owned `PuzzleCommandWork` (`🎮️commands/🧵️retained/🦀️.rs`,
/// `PuzzleCommandPhase::Work`). The hostile-static-law test above only proves this arm's source
/// text exists; this proves it runs.
#[test]
fn set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample command decodes");
    let mut work = Puzzle3dSetActiveExampleWork::default();
    let extent = work.extent(&command, &snapshot, &interaction).expect("nakagin example fits the fixed bounded work envelope");
    assert!(extent > 4, "nakagin example must require more than the handful of fixed transition steps alone");
    let mut progress_steps = 0usize;
    let emit = loop {
        assert!(progress_steps <= extent + 8, "setActiveExample work did not reach Complete within its own declared extent");
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => progress_steps += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert!(progress_steps > 1, "setActiveExample must require multiple bounded step() calls for the nakagin example, not a single-shot reducer; observed {progress_steps}");
    assert!(emit.artifact_mutations.len() > 1, "the completed emit must carry every incrementally-collected mutation, one per deleted/created item; observed {}", emit.artifact_mutations.len());
    assert_eq!(emit.config_mutations.len(), 1, "setActiveExample resets the runtime config exactly once, in the same completed emit");
}

/// 🪣️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S, fixture group 7: drives
/// `Puzzle3dPrecomputeCommandWork`'s own `step()` loop directly (not the full `dispatch()` path,
/// which never touches `build_tool_job`'s retained-Work machinery at all) and proves the
/// `"fillBuildTick"` completion now carries the SAME real effect the legacy
/// `fill_build_tick::fill_build_tick` (`🎮️commands/🪣️fill-build-tick/🦀️.rs`) produces via
/// `dispatch_puzzle3d_action` — an isolated `SpawnJob` plus the advanced `fill_checkpoint`
/// snapshot — instead of the old stub's bare `ui_scope`. Mirrors
/// `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job`'s exact starting state (no
/// object needed — the fill tool admits its planner job unconditionally on the first tick).
#[test]
fn fill_build_tick_work_spawns_the_isolated_planner_and_persists_the_checkpoint() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig { active_tool_id: Some(fill_tool::TOOL_ID.to_string()), ..Default::default() };
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("fillBuildTick", None, None).expect("fillBuildTick command decodes");
    let mut work = Puzzle3dPrecomputeCommandWork::new("fillBuildTick");
    let emit = loop {
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => {}
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert!(
        matches!(emit.effects.as_slice(), [Effect::SpawnJob { kind, placement: semio_framework_plugin::kernel::JobPlacement::Isolated, .. }] if kind == crate::editor::puzzle3d::precompute::FILL_JOB_KIND),
        "the real fillBuildTick completion must spawn the isolated fill planner job, not just flip ui_scope; got {:?}",
        emit.effects
    );
    assert_eq!(emit.config_mutations.len(), 1, "the real fillBuildTick completion must persist the newly-admitted fill_checkpoint as a config Snapshot");
}

/// 🪣️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S: proves `"registerBrushMesh"` and
/// `"suggestionsTick"` now delegate to `puzzle3d_retained_reduce` (the same reducer
/// `BoundedFirstStepCommandWork` already uses for the 11 generic-fallback ids), which falls
/// through to the real `app.handle_action_impl(...)` for both — instead of the old stub's bare
/// `ui_scope: UiDirtyScope::None`/`puzzle3d_suggestions_tick_scope()` with zero mutations. Uses a
/// deterministically-undecodable `fill_checkpoint` (`decode_fill_envelope_request` rejects
/// anything not starting with the 8-byte `P3FILL04` magic — `⏳️precompute/🦀️.rs:138`, no
/// dependency on the process-global fill registry's live state) so `handle_action_impl`'s
/// trailing `&scene.runtime != config` diff (`✏️editor/🦀️.rs`, the `config_mutations` line right
/// before `dispatch_puzzle3d_action`'s call site) is guaranteed to see the checkpoint clear from
/// stale bytes to empty and therefore emit a real `Puzzle3dConfigMutation::Snapshot` — something
/// the old stub could never produce for either id.
#[test]
fn register_brush_mesh_and_suggestions_tick_work_clear_a_stale_checkpoint_via_real_dispatch() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig { fill_checkpoint: vec![9, 9, 9, 9], ..Default::default() };
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    for (tool_id, args) in [("registerBrushMesh", Some(json!({ "url": "test.glb", "positions": [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], "indices": [0, 1, 2] }))), ("suggestionsTick", None)] {
        let command = Puzzle3dCommand::from_action(tool_id, args, None).expect("command decodes");
        let mut work = Puzzle3dPrecomputeCommandWork::new(tool_id);
        let emit = loop {
            match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
                PuzzleCommandWorkStep::Progress { .. } => {}
                PuzzleCommandWorkStep::Complete(emit) => break emit,
            }
        };
        assert_eq!(emit.config_mutations.len(), 1, "{tool_id} must clear a stale fill_checkpoint via a real config Snapshot, matching handle_action_impl; the old stub emitted none");
    }
}

/// 🪣️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S: `Puzzle3dEngagementRepeatWork`'s old Publish
/// stage only ever emitted `self.effect` (the `setFillCount` bump) with a bare `ui_scope: Full`.
/// `engagementRepeatLast` is a `document_action` (`puzzle3d_action_document_intent`), so real
/// `handle_action_impl` ALWAYS resets `fill_checkpoint` to empty for it regardless of the active
/// utility (the `document_action && action != "setFillCount"` branch, `✏️editor/🦀️.rs`) — proves
/// that real side effect now surfaces as a config `Snapshot`, alongside the `setFillCount` request
/// effect the old code already got right.
#[test]
fn engagement_repeat_last_work_clears_checkpoint_and_requests_more_fill() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig { active_tool_id: Some(fill_tool::TOOL_ID.to_string()), fill_count: 3, fill_checkpoint: vec![9, 9, 9, 9], ..Default::default() };
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("engagementRepeatLast", None, None).expect("engagementRepeatLast command decodes");
    let mut work = Puzzle3dEngagementRepeatWork::default();
    let emit = loop {
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => {}
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert!(matches!(emit.effects.as_slice(), [Effect::DispatchAction { action, .. }] if action == "setFillCount"), "engagementRepeatLast must still request one more fill unit while the fill utility is active; got {:?}", emit.effects);
    assert_eq!(emit.config_mutations.len(), 1, "engagementRepeatLast is a document_action, so real handle_action_impl always clears a stale fill_checkpoint too; the old stub emitted none");
}

/// 🪣️ ticket 26/09/02/PUZZLE-3D-END-TO-END: `"setFillCountStep"` (`set_fill_count::STEP_ACTION_ID`)
/// routes to `Puzzle3dPrecomputeCommandWork` through `build_tool_job`. Constructed directly here to
/// prove the Work-side behaviour independently of that routing: without its own arm, the generic
/// `puzzle3d_retained_reduce` fallback would call `dispatch_puzzle3d_action`, whose match has no
/// `"setFillCountStep"` arm (falls to `_ => {}`), silently producing `Emit::default()` — exactly
/// the "compiles, dispatches, does nothing" failure this ticket exists to eliminate. A real,
/// synchronously-driven fill job (`drive_enqueued_fill_job_for_test`, `⏳️precompute/🦀️.rs:1885`)
/// is admitted first so `set_fill_count::step`'s own `precompute.restore_persisted_fill(...)`
/// guard can actually succeed — `set_fill_count::step` always emits a `SetFillAppliedCount`
/// config mutation plus a coalesce key once that guard passes, even for a zero-item chunk
/// (`🎮️commands/🧮️set-fill-count/🦀️.rs`), so this discriminates real dispatch from the silent
/// no-op without needing any placeable document content.
#[test]
fn set_fill_count_step_work_advances_a_real_admitted_fill_plan() {
    use crate::retained_command::{PuzzleCommandWork, PuzzleCommandWorkStep};
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let default_config = Puzzle3dConfig::default();
    let active_utility = puzzle3d_scene_active_utility(&default_config, None);
    let scene = scene_from_projection(&puzzle3d_projection_value(snapshot.value()), default_config, &active_utility);
    let mut precompute = Puzzle3dPrecomputeSession::new();
    sync_precompute_session(&mut precompute, &scene);
    assert!(precompute.enqueue_fill_job().is_some(), "a fresh session with no live job must admit a fill worker");
    precompute.drive_enqueued_fill_job_for_test(64);
    let checkpoint = precompute.fill_checkpoint_bytes();
    assert!(!checkpoint.is_empty(), "a driven fill job must leave a restorable checkpoint token");
    let config = Puzzle3dConfig { fill_checkpoint: checkpoint, fill_count: 0, fill_apply_generation: 0, fill_applied_count: 0, ..Default::default() };
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action(set_fill_count::STEP_ACTION_ID, Some(json!({ "generation": 0, "target": 0 })), None).expect("setFillCountStep command decodes");
    let mut work = Puzzle3dPrecomputeCommandWork::new(set_fill_count::STEP_ACTION_ID);
    let emit = loop {
        match work.step(&command, &snapshot, &config, &interaction, &hover).expect("bounded step") {
            PuzzleCommandWorkStep::Progress { .. } => {}
            PuzzleCommandWorkStep::Complete(emit) => break emit,
        }
    };
    assert_eq!(emit.config_mutations.len(), 1, "a real setFillCountStep must persist the advanced applied-count checkpoint; the generic fallback would silently no-op instead");
    assert!(emit.coalesce_key.is_some(), "setFillCountStep must coalesce its continuation, matching set_fill_count::step's real Emit");
}

/// 🪣️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S: `transformBegin`/`transformEnd` are left on
/// `NoopPuzzleCommandWork` (`🎮️commands/🧵️retained/🦀️.rs:98`) deliberately — see
/// `📓️wave-S-report.md` for the full source-backed argument. In short: `EditorApp<E>`
/// (`🧰️framework/…/🔌️plugin/🦀️.rs:27160`) stores no `E` instance at all, and
/// `with_puzzle3d_app_for` (`✏️editor/🦀️.rs:2168`) always builds a fresh
/// `Puzzle3dPlayApp::default()` restoring ONLY `fill_checkpoint` — `Puzzle3dConfig` has no field
/// for `transform_base`/`transform_scratch`/`transform_drag_active` (unlike `fill_checkpoint`,
/// which is bridged through the process-global `fill_envelope_registry`). So on every real
/// dispatch `transform_drag_active` starts `false`, `commit_transform`'s scratch is always `None`,
/// and `handle_action_impl` ITSELF returns bare `Emit::default()` for both actions — proven here
/// directly against `puzzle3d_retained_reduce` (the exact reducer the generic ids use), which
/// falls through to `app.handle_action_impl(...)` for both since neither is special-cased.
#[test]
fn transform_begin_and_end_real_dispatch_is_already_the_noop_the_work_emits() {
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    for action in ["transformBegin", "transformEnd"] {
        let command = Puzzle3dCommand::from_action(action, None, None).expect("command decodes");
        let emit = puzzle3d_retained_reduce(&command, &snapshot, &config, &interaction, &hover).expect("real dispatch");
        assert!(
            emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.effects.is_empty(),
            "{action}'s real handle_action_impl must be a true no-op given the current stateless-per-call architecture; got {:?}/{:?}/{:?}",
            emit.artifact_mutations.len(),
            emit.config_mutations.len(),
            emit.effects.len()
        );
    }
}

/// 🌉️ ticket 26/09/02/PUZZLE-3D-END-TO-END: exercises the wiring this ticket adds — the
/// classification flip to `Migrated`, `PUZZLE3D_RETAINED_TOOL_IDS` registration, the
/// `ArtifactToolPublicationContract` for the Artifact+Config lanes, and
/// `Puzzle3dArtifactStorePreparationFactory` — by dispatching `setActiveExample` through the
/// real `InteractiveJob`/tool-job path (`Puzzle3dRetainedCommandJobFactory` ->
/// `RetainedPuzzleCommandJob` -> `ArtifactToolCompletion` -> the shared publication loop's
/// `self.store.begin_apply_one(..., self.artifact_one_item_factory.as_deref())`), driving the
/// resulting typed operation to completion via repeated `maintenance_step` turns exactly as a
/// real host does every actor tick, then asserting the document was actually swapped. Uses
/// `app_with_registry()` + a bound instance id: this plugin declares
/// `bounded_first_step_tool_proofs!`, so the bare registry-less `testkit::app()` faults closed
/// with `interactive-job.catalog-authority` before any dispatch is even attempted.
#[test]
fn set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document() {
    use semio_framework_plugin::PluginApp;
    let mut app = app_with_registry();
    semio_framework::io::resolve_ready(app.bind_instance_id(1));
    let before_first_id = first_object_id(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("dispatch setActiveExample through the migrated tool-job path");
    let mut ticks = 0usize;
    while ticks < 5_000 && (object_count(&app) == 0 || first_object_id(&app) == before_first_id) {
        app.maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert!(object_count(&app) > 0, "nakagin document did not land after {ticks} maintenance turns");
    assert_ne!(first_object_id(&app), before_first_id, "setActiveExample did not actually swap the document's objects through the tool-job path");
}

/// 🌉️ ticket 26/09/02/PUZZLE-3D-END-TO-END: exercises the wiring this ticket adds for
/// `setFillCount` — the classification flip to `Migrated`, `PUZZLE3D_RETAINED_TOOL_IDS`
/// registration, and the `ArtifactToolPublicationContract` Config lane — by dispatching
/// `setFillCount` through the real `InteractiveJob`/tool-job path
/// (`Puzzle3dRetainedCommandJobFactory` -> `RetainedPuzzleCommandJob` ->
/// `Puzzle3dPrecomputeCommandWork`'s bounded `step()` cursor -> the config-store publication
/// loop), driving the resulting typed operation to completion via repeated `maintenance_step`
/// turns exactly as a real host does every actor tick, then asserting the fill tool's own count
/// slider actually observed the requested target land in the live config (`SetFillRequest`'s
/// `next.fill_count = *count`, `✏️s/…/🎚️config/🦀️.rs:539`). Uses `app_with_registry()` + a bound
/// instance id: this plugin declares `bounded_first_step_tool_proofs!`, so the bare
/// registry-less `testkit::app()` faults closed with `interactive-job.catalog-authority` before
/// any dispatch is even attempted. Deliberately reads only the config-side slider measure, not
/// the document — `fillBuildTick` (unmigrated; see its own blocker note in
/// `🔏️publication-authority/🔣️.json`) is what materializes actual fill objects, not this tool.
#[test]
fn set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count() {
    use semio_framework_plugin::PluginApp;
    let mut app = app_with_registry();
    semio_framework::io::resolve_ready(app.bind_instance_id(1));
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    let fill_count_slider = |app: &mut Puzzle3dApp| -> Option<f64> {
        let measures = semio_framework::io::resolve_ready(app.tool_measures());
        find_measure_slider(measures.get(fill_tool::TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count")
    };
    assert_eq!(fill_count_slider(&mut app), Some(0.0), "fill count starts at zero before any request");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).expect("dispatch setFillCount through the migrated tool-job path");
    let mut ticks = 0usize;
    while ticks < 5_000 && fill_count_slider(&mut app) != Some(3.0) {
        app.maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert_eq!(fill_count_slider(&mut app), Some(3.0), "setFillCount did not update the live config's requested fill count through the tool-job path after {ticks} maintenance turns");
}

fn add_brush_object_is_cursorized(source: &str) -> bool {
    source.contains(r#""addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default())"#)
        && source.contains("Puzzle3dAddBrushObjectStage::Decode")
        && source.contains("Puzzle3dAddBrushObjectStage::Kind")
        && source.contains("Puzzle3dAddBrushObjectStage::Representation")
        && source.contains("Puzzle3dAddBrushObjectStage::Vortices")
        && source.contains("Puzzle3dAddBrushObjectStage::ExistingAttractions")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishObject")
        && source.contains("Puzzle3dAddBrushObjectStage::PublishAttraction")
        && !source.contains(r#""addBrushObject" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_brush_object_hostile_static_law_rejects_engine_run_to_completion() {
    let source = include_str!("../../🦀️.rs");
    assert!(add_brush_object_is_cursorized(source));
    let direct = source
        .replace(r#""addBrushObject" => Box::new(Puzzle3dAddBrushObjectWork::default())"#, r#""addBrushObject" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_brush_object_is_cursorized(&direct));
    for marker in [
        "Puzzle3dAddBrushObjectStage::Decode",
        "Puzzle3dAddBrushObjectStage::Kind",
        "Puzzle3dAddBrushObjectStage::Representation",
        "Puzzle3dAddBrushObjectStage::Vortices",
        "Puzzle3dAddBrushObjectStage::ExistingAttractions",
        "Puzzle3dAddBrushObjectStage::PublishObject",
        "Puzzle3dAddBrushObjectStage::PublishAttraction",
    ] {
        assert!(!add_brush_object_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing brush cursor was falsely accepted: {marker}");
    }
}

fn add_object_kind_is_cursorized(source: &str) -> bool {
    source.contains(r#""addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default())"#)
        && source.contains("Puzzle3dAddObjectKindStage::Decode")
        && source.contains("Puzzle3dAddObjectKindStage::Kind")
        && source.contains("Puzzle3dAddObjectKindStage::Representation")
        && source.contains("Puzzle3dAddObjectKindStage::Vortex")
        && source.contains("Puzzle3dAddObjectKindStage::Publish")
        && source.contains("PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT")
        && !source.contains(r#""addObjectKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn add_object_kind_hostile_static_law_rejects_whole_catalog_conversion() {
    let source = include_str!("../../🦀️.rs");
    assert!(add_object_kind_is_cursorized(source));
    let direct =
        source.replace(r#""addObjectKind" => Box::new(Puzzle3dAddObjectKindWork::default())"#, r#""addObjectKind" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!add_object_kind_is_cursorized(&direct));
    for marker in ["Puzzle3dAddObjectKindStage::Kind", "Puzzle3dAddObjectKindStage::Representation", "Puzzle3dAddObjectKindStage::Vortex", "Puzzle3dAddObjectKindStage::Publish", "PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT"] {
        assert!(!add_object_kind_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing add-object-kind cursor was falsely accepted: {marker}");
    }
}

fn scalar_config_routes_are_direct(source: &str) -> bool {
    source.contains("struct Puzzle3dScalarConfigWork")
        && source.contains(
            r#""setCamera"
            | "setProjection"
            | "setProjectionParam""#,
        )
        && source.contains(r#"| "engagementInput" => Box::new(Puzzle3dScalarConfigWork::new(tool_id))"#)
        && source.contains("Puzzle3dScalarConfigStage::Prepare")
        && source.contains("Puzzle3dScalarConfigStage::Publish")
        && source.contains("Puzzle3dConfigMutation::SetWindowCamera")
        && source.contains("Puzzle3dConfigMutation::SetWindowSun")
        && source.contains("Puzzle3dConfigMutation::SetWindowGridSpacing")
        && source.contains("Puzzle3dConfigMutation::SetOverlapBudget")
        && source.contains("Puzzle3dConfigMutation::SetWindowVoxelDims")
        && source.contains("Puzzle3dConfigMutation::SetSuggestionMenu")
        && source.contains("Puzzle3dConfigMutation::SetBrushCandidateIndex")
        && source.contains("Puzzle3dConfigMutation::SetWindowEngagementInput")
        && !source.contains(r#""setCamera" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
        && !source.contains(r#""setVortexDirection" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn scalar_config_hostile_static_law_rejects_old_reducer_and_missing_exact_fields() {
    let source = include_str!("../../🦀️.rs");
    assert!(scalar_config_routes_are_direct(source));
    for marker in [
        "Puzzle3dScalarConfigStage::Prepare",
        "Puzzle3dScalarConfigStage::Publish",
        "Puzzle3dConfigMutation::SetWindowCamera",
        "Puzzle3dConfigMutation::SetWindowSun",
        "Puzzle3dConfigMutation::SetWindowGridSpacing",
        "Puzzle3dConfigMutation::SetOverlapBudget",
        "Puzzle3dConfigMutation::SetWindowVoxelDims",
        "Puzzle3dConfigMutation::SetSuggestionMenu",
        "Puzzle3dConfigMutation::SetBrushCandidateIndex",
        "Puzzle3dConfigMutation::SetWindowEngagementInput",
    ] {
        assert!(!scalar_config_routes_are_direct(&source.replacen(marker, "route-removed", 1)), "missing scalar route marker was falsely accepted: {marker}");
    }
    let direct = source.replace(
        r#"| "engagementInput" => Box::new(Puzzle3dScalarConfigWork::new(tool_id))"#,
        r#"| "engagementInput" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!scalar_config_routes_are_direct(&direct), "hostile scalar old-reducer replacement must fail closed");
}

#[test]
fn transform_lifecycle_is_an_explicit_bounded_retained_boundary() {
    let source = include_str!("../../🦀️.rs");
    let route = r#""worldPointerDown" | "transformBegin" | "transformEnd" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id))"#;
    assert!(source.contains(route));
    let direct = source.replace(
        route,
        r#""worldPointerDown" => Box::new(crate::retained_command::NoopPuzzleCommandWork::new(tool_id)),
            "transformBegin" | "transformEnd" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!direct.contains(route));
    assert!(direct.contains(r#""transformBegin" | "transformEnd" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#));
}

fn engagement_submit_is_cursorized(source: &str) -> bool {
    source.contains(r#""engagementSubmit" => Box::new(Puzzle3dEngagementSubmitWork::default())"#)
        && source.contains("Puzzle3dEngagementSubmitStage::Focus")
        && source.contains("Puzzle3dEngagementSubmitStage::UtilityConfig")
        && source.contains("Puzzle3dEngagementSubmitStage::UtilityEffect")
        && source.contains("Puzzle3dEngagementSubmitStage::FillEffect")
        && source.contains("Puzzle3dEngagementSubmitStage::Input")
        && source.contains("Puzzle3dEngagementSubmitStage::Publish")
        && !source.contains(r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn engagement_submit_hostile_static_law_rejects_old_reducer_and_missing_transfers() {
    let source = include_str!("../../🦀️.rs");
    assert!(engagement_submit_is_cursorized(source));
    let direct = source.replace(
        r#""engagementSubmit" => Box::new(Puzzle3dEngagementSubmitWork::default())"#,
        r#""engagementSubmit" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!engagement_submit_is_cursorized(&direct));
}

fn engagement_repeat_is_direct(source: &str) -> bool {
    source.contains(r#""engagementRepeatLast" => Box::new(Puzzle3dEngagementRepeatWork::default())"#)
        && source.contains("Puzzle3dEngagementRepeatStage::Prepare")
        && source.contains("set_fill_count::request(config.fill_count.saturating_add(1).min(PUZZLE3D_FILL_COUNT_MAX))")
        && !source.contains(r#""engagementRepeatLast" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn engagement_repeat_is_a_direct_retained_fill_request() {
    let production = include_str!("../../🦀️.rs").split_once("//#region 🧪️Testkit").map_or(include_str!("../../🦀️.rs"), |(production, _)| production);
    assert!(engagement_repeat_is_direct(production));
    let fallback = production.replace(
        r#""engagementRepeatLast" => Box::new(Puzzle3dEngagementRepeatWork::default())"#,
        r#""engagementRepeatLast" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!engagement_repeat_is_direct(&fallback));
}

fn kind_weight_route_is_cursorized(source: &str) -> bool {
    source.contains(r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id))"#)
        && source.contains("Puzzle3dKindWeightStage::Catalog")
        && source.contains("Puzzle3dKindWeightStage::Validate")
        && source.contains("Puzzle3dKindWeightStage::SumOthers")
        && source.contains("Puzzle3dKindWeightStage::Build")
        && source.contains("Puzzle3dConfigMutation::SetObjectKindWeights")
        && source.contains("Puzzle3dConfigMutation::SetVortexKindWeights")
        && !source.contains(r#""setObjectKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn kind_weight_hostile_static_law_rejects_whole_normalizer_and_missing_cursors() {
    let source = include_str!("../../🦀️.rs");
    assert!(kind_weight_route_is_cursorized(source));
    let direct = source.replace(
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(Puzzle3dKindWeightWork::new(tool_id))"#,
        r#""setObjectKindWeight" | "setVortexKindWeight" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#,
    );
    assert!(!kind_weight_route_is_cursorized(&direct));
    assert!(!source.contains("puzzle3d_normalize_kind_weight_group(self.weights"));
}

fn engagement_abort_is_cursorized(source: &str) -> bool {
    source.contains(r#""engagementAbort" => Box::new(Puzzle3dEngagementAbortWork::default())"#)
        && source.contains("Puzzle3dEngagementAbortStage::Input")
        && source.contains("Puzzle3dEngagementAbortStage::Candidate")
        && source.contains("Puzzle3dEngagementAbortStage::Utility")
        && source.contains("Puzzle3dEngagementAbortStage::Publish")
        && source.contains("Puzzle3dConfigMutation::SetActiveUtility")
        && !source.contains(r#""engagementAbort" => Box::new(crate::retained_command::BoundedFirstStepCommandWork"#)
}

#[test]
fn engagement_abort_hostile_static_law_rejects_atomic_multi_owner_reset() {
    let source = include_str!("../../🦀️.rs");
    assert!(engagement_abort_is_cursorized(source));
    let direct = source
        .replace(r#""engagementAbort" => Box::new(Puzzle3dEngagementAbortWork::default())"#, r#""engagementAbort" => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))"#);
    assert!(!engagement_abort_is_cursorized(&direct));
    for marker in ["Puzzle3dEngagementAbortStage::Input", "Puzzle3dEngagementAbortStage::Candidate", "Puzzle3dEngagementAbortStage::Utility", "Puzzle3dEngagementAbortStage::Publish", "Puzzle3dConfigMutation::SetActiveUtility"] {
        assert!(!engagement_abort_is_cursorized(&source.replacen(marker, "cursor-removed", 1)), "missing engagement-abort boundary was falsely accepted: {marker}");
    }
}
use crate::editor::puzzle3d::config::Puzzle3dCamera;
use protocol::MutationDiff;
use semio_framework_plugin::{EditorApp, PluginApp};

#[test]
fn two_documents_carry_independent_serialized_checkpoints() {
    let first = Puzzle3dConfig::default();
    let second = Puzzle3dConfig::default();
    assert!(first.fill_checkpoint.is_empty());
    assert!(second.fill_checkpoint.is_empty());
    assert_eq!(dsl::ToValue::to_value(&first), dsl::ToValue::to_value(&second));
}

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn renders_world_scene() {
    let mut app = app();
    assert!(render_composite(&mut app).to_string().contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_concrete_forest_fixture() {
    let app = app();
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
    assert!(object_count(&app) > 0, "the concrete-forest default fixture ships with objects");
}

/// 📦️ `Puzzle3dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
#[semio_framework_async_macros::async_test]
async fn puzzle3d_play_projection_pack_round_trips() {
    let app = app();
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}

#[semio_framework_async_macros::async_test]
async fn open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change() {
    let mut app = app();
    let before = object_count(&app);
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).expect("openAddObjectDialog");
    assert!(
        matches!(result.requested_effects.as_slice(), [Effect::OpenDialog { dialog_id, args, .. }] if dialog_id == "addObject" && args.is_none()),
        "expected a single OpenDialog effect for the addObject dialog, got {:?}",
        result.requested_effects,
    );
    assert_eq!(object_count(&app), before, "opening the dialog does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_the_document_and_undo_restores_it() {
    let mut app = app();
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
    dispatch(&mut app, "redo", None, None).expect("redo");
    assert_eq!(object_count(&app), 0);
}

#[semio_framework_async_macros::async_test]
async fn nakagin_example_loads_via_operations() {
    let mut app = app();
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("nakagin");
    let projection = projection_of(&app);
    assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
    assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn document_and_inspector_panels_render() {
    let mut app = app();
    for body in [document::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(!render_body(&mut app, body).to_string().is_empty());
    }
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
/// dispatches through a standalone typed `Puzzle3dStore` — NOT through `Puzzle3dPlayApp`/
/// `Puzzle3dPlaySnapshot` (the `🔖️ValueBridge` `serde_json::Value` wrapper this app still uses)
/// — since `Puzzle3dMutation`'s canonical `Mutation<Puzzle3dSnapshot>` impl (not its
/// `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::standards::v1::subsets::any::schema::mutations::binary::Puzzle3dStore;
    use crate::{PUZZLE_3D_SCHEMA, Puzzle3dObject as TypedObject};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{ArtifactCommand, create_document_envelope};

    let mut store = semio_framework::io::resolve_ready(Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None))).expect("store");
    let object = TypedObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    semio_framework::io::resolve_ready(store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_object(object, None)], description: None })).expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework::io::resolve_ready(semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())));
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️Inspector
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
/// selected object's Origin nests x/y/z steppers. `panels::inspection::render` has no live
/// selection to switch on any more (see that module's doc comment — `ArtifactApp::render` never
/// gained an `InteractionView` parameter) and always falls through to the document summary now,
/// selected or not — this proves that degraded floor instead of the since-unreachable steppers.
#[semio_framework_async_macros::async_test]
async fn selected_object_inspector_nests_origin_into_x_y_z_steppers() {
    let mut app = app_with_registry();
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("interactionSelect");
    let json = render_body(&mut app, inspection::BODY_KEY).to_string();
    assert!(json.contains("puzzle3d-play-inspector.empty"), "render has no InteractionView, so the inspector cannot key off the selection and always shows the document summary: {json}");
}

fn object_origin_x(app: &Puzzle3dApp, object_id: &str) -> f64 {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).and_then(|origin| origin.first()).and_then(Value::as_f64))
        .expect("origin.x")
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_sets_absolute_value_and_preserves_other_axes() {
    let mut app = app();
    let object_id = first_object_id(&app);
    let before_y = projection_of(&app)
        .get("objects")
        .and_then(|value| value.as_array())
        .and_then(|objects| objects.first())
        .and_then(|object| object.get("origin"))
        .and_then(|value| value.as_array())
        .and_then(|origin| origin.get(1))
        .and_then(|value| value.as_f64())
        .expect("origin.y");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin.x", "value": 42.5 })), None).expect("patchInspector");
    let projection = projection_of(&app);
    let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
    let object = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(object_id.as_str())).expect("patched object");
    let origin = object.get("origin").and_then(|value| value.as_array()).expect("origin");
    assert_eq!(origin[0].as_f64(), Some(42.5), "origin.x should be set to the absolute value");
    assert_eq!(origin[1].as_f64(), Some(before_y), "origin.y should be untouched by an origin.x edit");
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_delta_offsets_each_selected_object_from_its_own_current_value() {
    let mut app = app();
    let id_a = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [10.0, 0.0, 0.0] })), None).expect("addObjectKind");
    let id_b = projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("added object id").to_string();
    assert_ne!(id_a, id_b, "the added object must be distinct from the first fixture object");
    let x_a_before = object_origin_x(&app, &id_a);
    let x_b_before = object_origin_x(&app, &id_b);
    assert_ne!(x_a_before, x_b_before, "the two objects must start at different x values for this test to prove per-object offset preservation");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [id_a.clone(), id_b.clone()], "field": "origin.x", "delta": 3.0 })), None).expect("patchInspector");
    assert_eq!(object_origin_x(&app, &id_a), x_a_before + 3.0, "a delta edit adds to each object's own current x");
    assert_eq!(object_origin_x(&app, &id_b), x_b_before + 3.0, "a delta edit preserves each object's own starting offset");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the inspector chrome no longer
/// renders per-entity stepper controls to inspect (see the sibling test's doc comment above), so
/// this now proves the same "resolve selection without embedding ids" contract one level down, at
/// `patchInspector`'s own `interaction.selection(vortex)` fallback (`commands::patch_inspector`) —
/// a bare `field`/`value` patch (no `ids` arg) must resolve against whatever the `vortex` domain's
/// `object` granularity currently holds.
#[semio_framework_async_macros::async_test]
async fn inspector_field_actions_resolve_selection_without_embedding_ids() {
    let mut app = app_with_registry();
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("interactionSelect");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "field": "origin.x", "value": 42.5 })), None).expect("patchInspector without ids");
    assert_eq!(object_origin_x(&app, &object_id), 42.5, "patchInspector must resolve the patched object from the live selection, not an embedded id");
}
//#endregion 🔖️Inspector

//#region 🔖️Manifest
#[semio_framework_async_macros::async_test]
async fn app_definition_has_the_main_world_window() {
    let definition = create_puzzle3d_app();
    assert!(definition.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn app_definition_declares_the_add_object_dialog() {
    let definition = create_puzzle3d_app();
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    assert_eq!(dialog.submit_action.as_str(), "addObjectKind");
    assert_eq!(dialog.args.len(), 1);
}

/// 📌️ The four declared panel tabs are present (the framework injects its own tabs alongside, so
/// this asserts presence, never a total count).
#[semio_framework_async_macros::async_test]
async fn app_definition_declares_its_four_panel_tabs() {
    let definition = create_puzzle3d_app();
    let body_keys: Vec<&str> = definition.panel_tabs.iter().filter_map(|tab| tab.body_key.as_deref()).collect();
    for expected in [document::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(body_keys.contains(&expected), "panel tab body {expected} must be declared, got {body_keys:?}");
    }
}

/// 🌉️ Every declared action must bridge through `command_from_action` and round-trip
/// `command_id` via the shared framework harness.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_bridges_to_a_command() {
    semio_framework::io::resolve_ready(semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_testkit));
    assert!(Puzzle3dPlayApp::command_from_action("noSuchAction", None).is_err());
}

/// 🌉️ Every declared app action (framework-injected verbs never reach `Puzzle3dCommand::from_action`
/// by design, so they simply fall through the `None` branch below) round-trips through the
/// macro-generated `Puzzle3dCommand::from_action`/`action_id` pair as well as the `ArtifactApp`
/// bridge asserted above.
#[semio_framework_async_macros::async_test]
async fn every_declared_action_round_trips_through_the_command_enum() {
    let definition = create_puzzle3d_app();
    for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
        let Some(command) = Puzzle3dCommand::from_action(&action.id, None, None) else {
            continue;
        };
        assert_eq!(command.action_id(), action.id.as_str(), "declared action {} must round-trip through Puzzle3dCommand", action.id);
    }
}

/// 🗣️ B1: manifest text is baked into `AppDefinition`/`App` as `LocalizedLabel` and resolved
/// directly via `.resolve(Terminology, Locale)` — no shell round-trip needed to assert on it.
#[semio_framework_async_macros::async_test]
async fn app_definition_labels_resolve_german_reuse_branded_for_aggregator() {
    use semio_framework_plugin::{Locale, Terminology};
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Reuse, Locale::De);
    let actions = || def.window_kinds.iter().flat_map(|window| window.actions.iter());
    let action = |id: &str| actions().find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Bearbeiten");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Aggregator");
    let dialog = def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog");
    assert_eq!(dialog.title.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(dialog.submit_label.resolve(terminology, locale), "Hinzufügen");
    let arg = dialog.args.iter().find(|entry| entry.id == "objectKind").expect("objectKind arg");
    let option = match arg.control() {
        semio_framework_plugin::ActionArgControl::Select { options } => options.iter().find(|entry| entry.value == "Object").cloned().expect("Object option"),
        _ => panic!("objectKind arg is not a select"),
    };
    assert_eq!(option.label.resolve(terminology, locale), "Baukomponente");
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Baukomponente hinzufügen");
    assert_eq!(action("openVortexSuggestions").label.resolve(terminology, locale), "Verbindungspunkt-Vorschläge öffnen");
    assert_eq!(action("createAttraction").label.resolve(terminology, locale), "Verbindung erstellen");
    assert_eq!(def.utilities.iter().find(|entry| entry.id == utilities::transform::UTILITY_ID).expect("transform utility").label.resolve(terminology, locale), "Transformieren");
    // 🎭️✏️ `create_puzzle3d_app()` no longer registers examples (see its own doc comment — `Editor::builder`
    // has no `.example(...)` and `AppDefinition` carries no `examples` field), so the concrete-forest
    // example's German label can no longer be asserted here; dropped, not silently left stale.
    let framework_interaction_actions = [
        INTERACTION_SELECT_ACTION_ID,
        semio_framework_plugin::INTERACTION_HOVER_ACTION_ID,
        semio_framework_plugin::CLEAR_SELECTION_ACTION_ID,
        semio_framework_plugin::SELECT_ALL_ACTION_ID,
        semio_framework_plugin::SET_SELECTION_MODE_ACTION_ID,
        semio_framework_plugin::SET_INTERACTION_GRANULARITY_ACTION_ID,
    ];
    for entry in actions() {
        if framework_interaction_actions.contains(&entry.id.as_str()) {
            continue;
        }
        let text = entry.label.resolve(terminology, locale);
        assert!(!text.contains("Hover") && !text.contains("Pick") && !text.contains("hovern"), "leftover English/mistranslation in {}: {text}", entry.id);
    }
}

#[semio_framework_async_macros::async_test]
async fn app_definition_labels_stay_english_native_without_brand_locks() {
    use semio_framework_plugin::{Locale, Terminology};
    let definition = create_puzzle3d_app();
    let def = &definition;
    let (terminology, locale) = (Terminology::Native, Locale::En);
    let action = |id: &str| def.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == id).unwrap_or_else(|| panic!("{id} action declared"));
    assert_eq!(def.modes.iter().find(|entry| entry.id == "edit").expect("edit mode").label.resolve(terminology, locale), "Edit");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == main::WINDOW_KIND_ID).expect("window kind").label.resolve(terminology, locale), "Puzzle 3D");
    assert_eq!(def.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog").title.resolve(terminology, locale), "Add Object");
    assert_eq!(action("addObjectKind").label.resolve(terminology, locale), "Add Object");
}

#[semio_framework_async_macros::async_test]
async fn document_and_kinds_trees_use_german_reuse_section_labels() {
    let mut app = app();
    dispatch(&mut app, "setLocale", Some(&json!({ "value": "de" })), None).expect("setLocale");
    dispatch(&mut app, "setTerminology", Some(&json!({ "value": "reuse" })), None).expect("setTerminology");
    let document_json = render_body(&mut app, document::BODY_KEY).to_string();
    let kinds = render_body(&mut app, catalogue::BODY_KEY).to_string();
    let measures_json = to_json_string(&semio_framework::io::resolve_ready(app.window_measures()));
    assert!(document_json.contains("Baukomponenten"), "document tree objects section");
    assert!(document_json.contains("Verbindungen"), "document tree attractions section");
    assert!(document_json.contains("Referenzen"), "document tree references section");
    assert!(document_json.contains("Zielvolumina"), "document tree target volumes section");
    assert!(kinds.contains("Kabel"), "catalogue cables section");
    assert!(kinds.contains("Verbindungen"), "catalogue attractions section");
    assert!(!document_json.contains("\"Attractions\"") && !kinds.contains("\"Attractions\""), "English Attractions must not appear");
    assert!(!kinds.contains("\"Cables\""), "English Cables must not appear");
    assert!(measures_json.contains("Verbindungen"), "select measures attractions toggle");
    assert!(!measures_json.contains("\"Attractions\""), "select measures must not hardcode Attractions");
}

#[semio_framework_async_macros::async_test]
async fn main_window_utilities_lead_with_transform_without_select_tool_and_no_default_utility() {
    let definition = create_puzzle3d_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert!(!utility_ids.contains(&"select"), "puzzle 3d must not declare a select utility");
    assert!(!utility_ids.contains(&"scale"), "puzzle 3d must not declare a scale utility");
    assert!(!utility_ids.contains(&fill_tool::TOOL_ID), "fill is a mode-level tool, not a window utility");
    let window = definition.window_kinds.iter().find(|window| window.id == main::WINDOW_KIND_ID).expect("main window");
    let main_utilities: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(main_utilities.first().copied(), Some(utilities::transform::UTILITY_ID));
    assert!(!main_utilities.contains(&"select"));
    assert!(!main_utilities.contains(&fill_tool::TOOL_ID), "fill must not be bound to the main window as a utility");
    assert_eq!(PUZZLE3D_DEFAULT_UTILITY, "", "unset/cleared host utility must not impersonate transform");
}

/// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
#[semio_framework_async_macros::async_test]
async fn tool_registry_declares_fill_tool() {
    let definition = create_puzzle3d_app();
    let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
    assert_eq!(tool_ids, vec![fill_tool::TOOL_ID]);
    assert_eq!(definition.modes[0].tools, vec![semio_framework::io::resolve_ready(ToolRef::new(fill_tool::TOOL_ID))]);
    assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
}
//#endregion 🔖️Manifest

//#region 🔖️Suggestions
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
    let mut app = app();
    let vortex = first_vortex_full_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex);
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
    assert!(menu_json.contains("openVortexSuggestions"));
    assert!(menu_json.contains("sparkles"), "menu should include suggest icon: {menu_json}");
    assert!(menu_json.contains("Zoom to selection"), "menu should include zoom: {menu_json}");
    assert!(menu_json.contains("deleteSelection"), "menu should include delete: {menu_json}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
    let mut app = app();
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id);
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
    assert!(menu_json.contains("menu.group.targets"), "hide/lock rows should be grouped under targets: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "destructive delete must be the last top-level row: {menu_json}");
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id.as_str(), "flag": "hidden", "value": true })), None).expect("setTargetVolumeFlag");
    let hidden = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
    assert_eq!(hidden, Some(true));
}

/// 🗂️ Grouped-disclosure contract for the object-selection branch: the top-level menu stays
/// scannable (leaves + groups + separator combined) and the destructive `deleteSelection` row is
/// the last top-level entry (`organize_context_menu` inserts the separator ahead of it).
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_object_groups_flags_and_keeps_delete_last() {
    let mut app = app();
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).expect("addObjectKind");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id);
    assert!(menu.len() <= 9, "top-level menu should stay scannable, got {} rows: {menu:?}", menu.len());
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("menu.group.hand"), "hide/lock rows should be grouped under hand: {menu_json}");
    assert!(menu_json.contains("duplicateSelection"), "menu should be {menu_json}");
    assert_eq!(menu.last().map(|item| item.id.as_str()), Some("delete"), "delete must be the last top-level row: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "delete must be marked destructive: {menu_json}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_opens_the_suggestion_popup() {
    let mut app = app();
    let vortex = first_vortex_full_id(&app);
    let result = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 34.0 })), None).expect("openVortexSuggestions");
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "opening a one-shot suggestion must not switch the host-owned utility or tool: {:?}",
        result.requested_effects,
    );
    let interaction = interaction_of(&render_composite(&mut app));
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "context-menu suggestion stays in the current selection mode");
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("open").and_then(Value::as_bool), Some(true));
    assert_eq!(menu.get("x").and_then(Value::as_f64), Some(12.0));
    assert_eq!(menu.get("y").and_then(Value::as_f64), Some(34.0));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
    assert!(menu.get("windowId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "suggestion menu is scoped to the opening window: {menu}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_records_explicit_window_id() {
    let mut app = app();
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 8.0, "y": 16.0, "windowId": main::WINDOW_INSTANCE_TOP })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).expect("openVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app));
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("windowId").and_then(Value::as_str), Some(main::WINDOW_INSTANCE_TOP));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_with_full_id_places_even_if_selection_was_cleared() {
    let mut app = app();
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).expect("openVortexSuggestions");
    let before_count = object_count(&app);
    // 🧹️ Simulate the split-pane outside-dismiss race clearing vortex selection before accept.
    dispatch(&mut app, "clearSelection", None, None).expect("clearSelection");
    let result = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex.as_str() })), None).expect("acceptSuggestion");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accept must not switch utility/tool: {:?}", result.requested_effects);
    assert!(object_count(&app) > before_count, "accept with fullId must place even after selection clear");
    let interaction = interaction_of(&render_composite(&mut app));
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_the_menu() {
    let mut app = app();
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app));
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🖱️ Hovering a row in the suggestion popup must live-update the 3D brush preview (rendered by
/// `world_brush_preview_json`, which reads `runtime.brush_candidate_index`) to the hovered
/// candidate, so the UI can highlight it in 3D before the user clicks — without switching the
/// host-owned active utility into brush mode.
#[semio_framework_async_macros::async_test]
async fn hover_suggestion_updates_the_brush_candidate_index_and_live_preview() {
    let mut app = app();
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).expect("openVortexSuggestions");
    let composite = render_composite(&mut app);
    let interaction = interaction_of(&composite);
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "suggestion hover must not enter brush mode");
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(0), "opening suggestions starts hover at the first candidate");
    let candidates = interaction.pointer("/suggestionMenu/candidates").and_then(Value::as_array).cloned().unwrap_or_default();
    assert!(!candidates.is_empty(), "suggestion candidates should be present");
    assert!(candidates[0].get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "candidates carry object-kind color: {candidates:?}");
    assert!(candidates[0].get("icon").and_then(Value::as_str).is_some_and(|icon| !icon.is_empty()), "candidates carry icon: {candidates:?}");
    let preview = brush_preview_of(&composite);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the live preview must target the vortex the suggestion menu was opened on");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "the live preview must resolve to a real candidate object kind");
    assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "brush preview carries object-kind color: {preview}");

    dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 1 })), None).expect("hoverSuggestion");
    let composite = render_composite(&mut app);
    let interaction = interaction_of(&composite);
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
    let preview = brush_preview_of(&composite);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the preview must keep targeting the same vortex while only the hovered candidate changes");
    assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "hovered brush preview still carries color: {preview}");
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_appends_an_object_and_closes_the_menu() {
    let mut app = app();
    let object_count_before = object_count(&app);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).expect("openVortexSuggestions");
    let result = dispatch(&mut app, "acceptSuggestion", None, None).expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before + 1);
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "accepting a one-shot suggestion must leave the host-owned utility/tool unchanged: {:?}",
        result.requested_effects,
    );
    let composite = render_composite(&mut app);
    let interaction = interaction_of(&composite);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
    assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()), "accept must clear sticky vortex hover");
    let selected_vortices = vortices_of(&composite).iter().filter(|entry| entry.get("selected").and_then(Value::as_bool) == Some(true)).count();
    assert_eq!(selected_vortices, 0, "one-shot accept must leave no sticky vortex selection");
}

/// 🧹️ A failed place (unknown vortex) must still close the suggestion menu — otherwise
/// `suggestionMenu.open` stays true and every split pane's regular context menu is gated shut.
#[semio_framework_async_macros::async_test]
async fn accept_suggestion_closes_menu_even_when_placement_fails() {
    // 🕹️ `hover_id` dispatches through the real `interactionHover` verb, which resolves the
    // `vortex` domain against `self.registry` — a plain `app()` carries no registry (see
    // `testkit::new_app`'s doc), so this needs the registry-backed `app_with_registry()`.
    let mut app = app_with_registry();
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).expect("openVortexSuggestions");
    let before = interaction_of(&render_composite(&mut app));
    assert_eq!(before.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    let object_count_before = object_count(&app);
    dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": "missing-object::missing-vortex.as_str()" })), None).expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before, "unknown-vortex accept must not place");
    let interaction = interaction_of(&render_composite(&mut app));
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()), "failed accept must still dismiss the suggestion menu");
}

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_sticky_hover() {
    let mut app = app_with_registry();
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app));
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🧰️ Context-menu / Alt+right-click suggestions are a one-shot placement: opening and accepting
/// must leave whatever host-owned utility was already active (e.g. transform) untouched.
#[semio_framework_async_macros::async_test]
async fn open_and_accept_vortex_suggestions_preserve_active_utility() {
    let mut app = app();
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).expect("activate transform");
    let vortex = first_vortex_full_id(&app);
    let open = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), Some(main::WINDOW_KIND_ID)).expect("openVortexSuggestions");
    assert!(open.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "opening suggestions must not emit utility/tool switches: {:?}", open.requested_effects);
    let open_node = render_window(&mut app, main::WINDOW_KIND_ID);
    let open_interaction = interaction_of(&open_node);
    assert_eq!(open_interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "transform remains non-brush scene mode during suggestions");
    assert_eq!(open_interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    assert!(brush_preview_of(&open_node).get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "one-shot suggestions still emit a placement preview without entering brush mode");
    let accept = dispatch(&mut app, "acceptSuggestion", None, Some(main::WINDOW_KIND_ID)).expect("acceptSuggestion");
    assert!(accept.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accepting suggestions must not emit utility/tool switches: {:?}", accept.requested_effects);
    let accept_interaction = interaction_of(&render_window(&mut app, main::WINDOW_KIND_ID));
    assert!(accept_interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(accept_interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
}
//#endregion 🔖️Suggestions

//#region 🔖️WindowOptions
#[semio_framework_async_macros::async_test]
async fn grid_window_options_control_one_visible_grid_spacing() {
    let mut app = app();
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), None).expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 7.5 })), None).expect("setGridSpacing");
    let lod = lod_of(&render_composite(&mut app));
    assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
    let measures = semio_framework::io::resolve_ready(app.window_measures());
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid")), Some(None));
    assert_eq!(find_measure_slider(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(7.5));
}

/// 🪟️ Two window instances of the same kind (a split top/perspective pane pair) must never share
/// window options — toggling grid visibility in one instance must leave every other instance's
/// grid untouched, both in its measures chrome and in its own rendered scene.
#[semio_framework_async_macros::async_test]
async fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut app = app();
    let second_window = "puzzle3d-main-2";
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");

    // Register both instances by dispatching a no-op-ish view action from each.
    dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).expect("register base window");
    dispatch(&mut app, "worldPointerDown", None, Some(second_window)).expect("register second window");

    // Both instances start visible (the type default).
    let initial_measures = semio_framework::io::resolve_ready(app.window_measures());
    assert_eq!(find_measure_toggle(initial_measures.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true));
    assert_eq!(find_measure_toggle(initial_measures.get(second_window).expect("second measures"), &toggle_id), Some(true));

    // Hide the grid, but ONLY on the second window instance.
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(second_window)).expect("setGridVisible on second window");

    let measures_after = semio_framework::io::resolve_ready(app.window_measures());
    assert_eq!(find_measure_toggle(measures_after.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true), "the base window instance's grid must stay visible");
    assert_eq!(find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id), Some(false), "only the targeted window instance's grid toggles off");

    // The rendered scenes agree: the base window still draws its LOD grid, the second does not.
    assert_eq!(lod_of(&render_window(&mut app, main::WINDOW_KIND_ID)).get("showLodGrid").and_then(Value::as_bool), Some(true));
    assert_eq!(lod_of(&render_window(&mut app, second_window)).get("showLodGrid").and_then(Value::as_bool), Some(false));
}

/// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
/// they are View-kind and must never emit VCS operations, no matter what they mutate.
#[semio_framework_async_macros::async_test]
async fn camera_actions_are_view_actions_that_emit_no_artifact_mutations() {
    let app_definition = create_puzzle3d_app();
    for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
        let def = app_definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
        assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
    }
    let mut live = app_with_registry();
    let before = projection_of(&live);
    let result = dispatch(&mut live, "setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), None).expect("setCamera");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&live), before, "setCamera must not mutate the document");
}

/// 🪟️📷️ Orbiting one window instance's camera must never move any sibling instance's camera, and
/// must never touch the shared document.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
    let mut app = app();
    let window_a = "puzzle3d-main-a";
    let window_b = "puzzle3d-main-b";
    dispatch(&mut app, "worldPointerDown", None, Some(window_a)).expect("register a");
    dispatch(&mut app, "worldPointerDown", None, Some(window_b)).expect("register b");

    let before_document = projection_of(&app);
    let camera_b_before = camera_of(&render_window(&mut app, window_b));

    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window_a)).expect("setCamera on window A");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&app), before_document, "setCamera must never mutate the shared document");

    let camera_a_after = camera_of(&render_window(&mut app, window_a));
    assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");
    assert_eq!(camera_of(&render_window(&mut app, window_b)), camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
}

#[semio_framework_async_macros::async_test]
async fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
    let mut app = app();
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
    let measures = semio_framework::io::resolve_ready(app.window_measures());
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

    assert!(vortices_of(&render_composite(&mut app)).is_empty(), "Selected mode must hide vortices while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).expect("setVortexShow always");
    let measures_always = semio_framework::io::resolve_ready(app.window_measures());
    let window_measures_always = measures_always.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_always, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
    assert_eq!(vortices_of(&render_composite(&mut app)).len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), None).expect("setVortexShow selected");
    assert!(vortices_of(&render_composite(&mut app)).is_empty(), "switching back to Selected must hide idle vortices");
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
    let mut app = app();
    let measures = semio_framework::io::resolve_ready(app.window_measures());
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).expect("setVortexShow always");
    let outwards_vortices = vortices_of(&render_composite(&mut app));
    assert!(!outwards_vortices.is_empty(), "fixture must expose vortices");
    assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), None).expect("setVortexDirection inwards");
    let measures_inwards = semio_framework::io::resolve_ready(app.window_measures());
    let window_measures_inwards = measures_inwards.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_inwards, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
    assert!(vortices_of(&render_composite(&mut app)).iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_option_is_local_to_the_window_instance() {
    let mut app = app();
    let second_window = "puzzle3d-main-2";
    dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).expect("register base window");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).expect("setVortexShow always on base");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(second_window)).expect("setVortexShow always on second");
    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), Some(second_window)).expect("setVortexDirection inwards on second window");

    let base_vortices = vortices_of(&render_window(&mut app, main::WINDOW_KIND_ID));
    assert!(!base_vortices.is_empty(), "the base window must still emit vortices");
    assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    let second_vortices = vortices_of(&render_window(&mut app, second_window));
    assert!(second_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}
//#endregion 🔖️WindowOptions

//#region 🔖️Fill
#[semio_framework_async_macros::async_test]
async fn fill_build_tick_is_ignored_when_fill_tool_is_inactive() {
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `handle_action_impl` now takes
    // an `InteractionView`, which no external crate can construct directly (its fields are
    // `pub(crate)` to `semio_framework_plugin`, no public constructor exists — flagged to the
    // coordinator as a testability gap). Routed through the real `dispatch()`/`with_puzzle3d_app`
    // machinery instead, which builds one internally.
    //
    // 🕹️ Pre-existing (unrelated to this ticket) framework testability gap in
    // `VcsArtifactApp::dispatch_typed`/`finish_recorded`: `dispatch_typed` always tags its call to
    // `finish_recorded` with the literal verb `"typed-command"` (never the real action id), so
    // `finish_recorded`'s `self.registry.get(verb)` lookup can never resolve `fillBuildTick`'s
    // declared `ActionKind::View` — `skip_history_panel` is unreachable via this path regardless of
    // registry population, and every `dispatch_typed` call that logs anything (`log_generation`
    // advances) picks up a `Partial { panel_bodies: ["framework.body.history"] }` refresh. Confirmed
    // present before this ticket too (`finish_recorded`'s registry lookup, not its
    // `ActionKind::View | ActionKind::Interaction` match arm, is what fails) — flagged to the
    // coordinator, not fixed here (framework file, out of this crate's remit). Asserts the real
    // regression guard (no progression while inactive) plus the weaker-but-true scope bound (never
    // a `Full` refresh) instead of the unreachable exact `None`.
    let mut app = app();
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("activate fill");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": Value::Null })), None).expect("deactivate fill");
    let before = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    for _ in 0..64 {
        let result = dispatch(&mut app, "fillBuildTick", None, None).expect("fillBuildTick");
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "an inactive fill tick must never force a full app refresh");
    }
    let after = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    assert_eq!(after, before, "stale or queued fill ticks must not advance planning after the Fill tool is deactivated");
}

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job() {
    let mut app = app_with_registry();
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("activate fill");
    let before = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    let first = dispatch(&mut app, "fillBuildTick", None, None).expect("enqueue fill");
    let after = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    assert_eq!(after, before, "the view action must not execute a solver transition inline");
    assert!(matches!(
        first.requested_effects.as_slice(),
        [Effect::SpawnJob { kind, placement: semio_framework_plugin::kernel::JobPlacement::Isolated, .. }] if kind == crate::editor::puzzle3d::precompute::FILL_JOB_KIND
    ));
    let second = dispatch(&mut app, "fillBuildTick", None, None).expect("poll fill");
    assert!(!second.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { .. })), "a live fill request must not be enqueued twice");
}

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_only_plans_available_slider_range() {
    // 🐢️ `drive_precompute` is bounded to a small per-call budget (the fix for the UI-freeze bug:
    // a single action must never grind the whole precompute queue synchronously), so the build
    // converges over several ticks — exactly like the real 120ms `fillBuildTick` loop.
    let mut app = app_with_registry();
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    drive_fill_until_ready(&mut app, 4.0);
    let measures = semio_framework::io::resolve_ready(app.tool_measures());
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    match find_measure_slider(tool_measures, "puzzle3d-fill-count") {
        Some(value) => assert_eq!(value, 0.0, "background planning must not change the selected fill count"),
        None => panic!("expected a fill-count slider in the fill tool measures"),
    }
    assert_eq!(object_count(&app), object_count_before, "background planning must not append generated objects below the slider count");
    assert_eq!(find_measure_slider_max(tool_measures, "puzzle3d-fill-count"), Some(PUZZLE3D_FILL_COUNT_MAX as f64), "fill slider range stays fixed at the fill count max");
    let available_count = find_measure_slider_ready(tool_measures, "puzzle3d-fill-count").expect("expected a fill-count slider ready extent") as usize;
    assert!(available_count > 0, "the fill slider ready extent must expose collision-free compatible placements");
    let begin = dispatch(&mut app, "setFillCount", Some(&json!({ "value": available_count })), None).expect("setFillCount");
    assert_eq!(object_count(&app), object_count_before, "the slider gesture only publishes the reveal cutoff; document materialization is resumable");
    let immediate = render_composite(&mut app);
    assert_eq!(instance_count(&immediate), object_count_before + available_count, "the complete planned prefix is previewed immediately before document continuations finish");
    assert_eq!(interaction_of(&immediate).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(available_count as u64), "the reveal cutoff updates in the initiating interaction step");
    let (_, max_step) = finish_fill_count(&mut app, begin);
    assert!(max_step < std::time::Duration::from_millis(8), "every fill-count continuation must remain below the hard 8 ms interaction ceiling");
    assert_eq!(object_count(&app), object_count_before + available_count, "the fill slider must materialize exactly its available placement count");
    assert_eq!(instance_count(&render_composite(&mut app)), object_count_before + available_count, "the viewport must show every materialized fill object immediately");
    let initial_fill_ids: HashSet<String> = projection_of(&app).get("objects").and_then(Value::as_array).into_iter().flatten().skip(object_count_before).filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    // 🪪️ Incidental actions re-sync the applied document into the precompute session. That used to
    // rebuild `fill.base` around the materialized objects, after which the slider could neither
    // remove them nor replan — reproduce with a hover sync before clearing.
    let hovered_id = first_object_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&hovered_id)).expect("interactionHover after fill");
    let reduced = available_count / 2;
    set_fill_count_and_finish(&mut app, reduced as u32, None);
    assert_eq!(object_count(&app), object_count_before + reduced, "sliding down after an incidental sync must still remove fill objects from the document");
    let reduced_render = render_composite(&mut app);
    // 🪣️ The viewport keeps showing the FULL available plan (tagged revealIndex) even after
    // reducing — hiding is a client-side reveal-cutoff concern now, not a server-side instance
    // count concern; only the document (checked above) and the committed cutoff actually shrink.
    assert_eq!(instance_count(&reduced_render), object_count_before + available_count, "the viewport still exposes the full plan for instant re-reveal — nothing was discarded");
    assert_eq!(interaction_of(&reduced_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64), "the committed reveal cutoff tracks the reduced count");
    // 🔽️🔼️ Prefix-stable plan: moving back up to a count that was already planned before must be
    // INSTANT — no replanning, no `fillBuildTick` catch-up dispatch.
    set_fill_count_and_finish(&mut app, available_count as u32, None);
    assert_eq!(object_count(&app), object_count_before + available_count, "moving back up within the preserved plan is instant, not gated on another fillBuildTick");
    let target_measures = semio_framework::io::resolve_ready(app.tool_measures());
    let target_tool_measures = target_measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_slider(target_tool_measures, "puzzle3d-fill-count"), Some(available_count as f64));
    let restored_fill_ids: HashSet<String> = projection_of(&app).get("objects").and_then(Value::as_array).into_iter().flatten().skip(object_count_before).filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    assert_eq!(restored_fill_ids, initial_fill_ids, "up-down-up restores the exact same planned objects — the plan is prefix-stable, never discarded and re-rolled");
    set_fill_count_and_finish(&mut app, 0, None);
    assert_eq!(object_count(&app), object_count_before, "moving the fill slider to zero must remove every generated object");
}

#[semio_framework_async_macros::async_test]
async fn set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up() {
    // 🔒️ Requesting more than is currently planned must clamp (never leave `runtime.fill_count`
    // and the applied document disagreeing), and `fillBuildTick` must never self-dispatch another
    // `setFillCount` — the viewport already shows every planned piece (tagged `revealIndex`), so
    // there is nothing left for a catch-up round trip to accomplish.
    let mut app = app();
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    let available_count = drive_fill_until_ready(&mut app, PUZZLE3D_FILL_COUNT_MAX as f64) as u32;
    assert!(available_count > 0, "the maximum-delta timing proof requires a planned prefix");
    // Request far beyond what a single tick could have planned.
    let (steps, max_step) = set_fill_count_and_finish(&mut app, PUZZLE3D_FILL_COUNT_MAX, None);
    assert!(steps <= available_count.div_ceil(set_fill_count::MAX_PLACEMENTS_PER_STEP as u32) as usize, "a maximum slider request must use only fixed-size continuation chunks");
    assert!(max_step < std::time::Duration::from_millis(8), "maximum-delta fill materialization measured {max_step:?}; every continuation must remain below 8 ms");
    let measures = semio_framework::io::resolve_ready(app.tool_measures());
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    let clamped = find_measure_slider(tool_measures, "puzzle3d-fill-count").expect("fill-count slider value");
    assert!(clamped <= available_count as f64, "runtime.fill_count must clamp to what's actually planned, not the raw request");
    assert_eq!(clamped as usize, object_count(&app) - object_count_before, "the clamped measure value must match what the document actually materialized");
    let tick = dispatch(&mut app, "fillBuildTick", None, None).expect("fillBuildTick after an above-ready request");
    assert!(
        !tick.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "setFillCount")),
        "fillBuildTick must never self-dispatch setFillCount — the clamp at commit time means fill_count can never run ahead of what's planned"
    );
}

#[semio_framework_async_macros::async_test]
async fn newer_fill_count_request_cancels_a_stale_continuation() {
    let mut app = app_with_registry();
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    let ready = drive_fill_until_ready(&mut app, 4.0) as u32;
    assert!(ready > 0, "need a planned prefix for cancellation");
    let first = dispatch(&mut app, "setFillCount", Some(&json!({ "value": ready })), None).expect("begin first request");
    let stale_args = first.requested_effects.into_iter().find_map(|effect| match effect {
        Effect::DispatchAction { action, args, .. } if action == set_fill_count::STEP_ACTION_ID => args.map(|value| semio_framework::from_dsl_value::<Value>(value).expect("stale fill-count args decode")),
        _ => None,
    });
    let second = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 0 })), None).expect("supersede first request");
    finish_fill_count(&mut app, second);
    let before = projection_of(&app);
    let stale = dispatch(&mut app, set_fill_count::STEP_ACTION_ID, stale_args.as_ref(), None).expect("stale continuation is a no-op");
    assert!(stale.mutations.is_empty() && stale.requested_effects.is_empty(), "a stale generation must not mutate or requeue");
    assert_eq!(projection_of(&app), before, "a stale continuation cannot revive a superseded fill target");
}

#[semio_framework_async_macros::async_test]
async fn fill_render_reveals_the_full_available_plan_tagged_with_reveal_index() {
    // 🪣️ `render()` composes EVERY currently-planned piece (not just the committed `fill_count`),
    // each tagged `revealIndex` — the viewport applies its own live, main-thread cutoff to show or
    // hide them per drag value with zero WASM round trips. The committed cutoff is separately
    // exposed as `interactionJson.revealCutoffs["puzzle3d-fill"]`.
    let mut app = app();
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    let ready = drive_fill_until_ready(&mut app, 3.0) as usize;
    assert!(ready >= 3, "fill planning must expose at least three ready placements");
    assert_eq!(object_count(&app), object_count_before, "background planning must not mutate the document before setFillCount");

    let rendered = render_composite(&mut app);
    assert_eq!(instance_count(&rendered), object_count_before + ready, "render must already expose every planned piece, tagged for client-side reveal");
    let instances = instances_of(&rendered);
    let reveal_indices: Vec<u64> = instances.iter().skip(object_count_before).filter_map(|instance| instance.get("revealIndex").and_then(Value::as_u64)).collect();
    assert_eq!(reveal_indices.len(), ready, "every planned (not-yet-committed) instance must carry revealIndex");
    let mut sorted_indices = reveal_indices.clone();
    sorted_indices.sort_unstable();
    assert_eq!(sorted_indices, (0..ready as u64).collect::<Vec<_>>(), "revealIndex is a dense 0-based sequence matching plan order");
    // 🪣️ Untagged objects omit the `revealIndex` key entirely — a `null` would compare as `0`
    // against the host's boot cutoff and hide every ordinary object.
    let base_reveal_keys = instances.iter().take(object_count_before).filter(|instance| instance.get("revealIndex").is_some()).count();
    assert_eq!(base_reveal_keys, 0, "base (non-plan) objects never carry a revealIndex key, not even a null one");
    let interaction = interaction_of(&rendered);
    assert_eq!(interaction.pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(0), "nothing committed yet — the reveal cutoff mirrors runtime.fill_count (0)");
    assert_eq!(interaction.pointer("/fillBuild/appliedCount").and_then(Value::as_u64), Some(0));

    set_fill_count_and_finish(&mut app, ready as u32, None);
    let after_commit = render_composite(&mut app);
    assert_eq!(instance_count(&after_commit), object_count_before + ready, "instance count is unchanged by commit — only the cutoff (and document) advanced");
    let committed_interaction = interaction_of(&after_commit);
    assert_eq!(committed_interaction.pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(ready as u64));
    assert_eq!(committed_interaction.pointer("/fillBuild/appliedCount").and_then(Value::as_u64), Some(ready as u64));
}

/// 🪣️ Fill count drives the shared document + reveal cutoff — split top/perspective panes must
/// never disagree about which planned objects are visible after a slider commit on either pane.
#[semio_framework_async_macros::async_test]
async fn fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances() {
    let mut app = app();
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), Some(top)).expect("select fill tool");
    let ready = drive_fill_until_ready(&mut app, 3.0) as u32;
    assert!(ready >= 3, "need a planned fill prefix to assert cross-pane sync");

    // Commit from the top pane only — the perspective pane must still track the same cutoff.
    let committed = ready.min(3);
    set_fill_count_and_finish(&mut app, committed as u32, Some(top));

    let top_render = render_window(&mut app, top);
    let perspective_render = render_window(&mut app, perspective);
    assert_eq!(interaction_of(&top_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(committed as u64), "top pane reveal cutoff must track the committed fill count");
    assert_eq!(interaction_of(&perspective_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(committed as u64), "perspective pane must share the same reveal cutoff — fill is document-global, not per-window");
    assert_eq!(instance_count(&top_render), instance_count(&perspective_render), "both panes must emit the same instance list for the shared fill plan");

    let instance_ids = |node: &Value| -> Vec<String> { instances_of(node).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect() };
    assert_eq!(instance_ids(&top_render), instance_ids(&perspective_render), "top and perspective must show the exact same object ids after a fill slider commit");

    // Sliding from the other pane must keep both panes in lockstep.
    let reduced = committed.saturating_sub(1);
    set_fill_count_and_finish(&mut app, reduced as u32, Some(perspective));
    let top_after = render_window(&mut app, top);
    let perspective_after = render_window(&mut app, perspective);
    assert_eq!(interaction_of(&top_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
    assert_eq!(interaction_of(&perspective_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
    assert_eq!(instance_count(&top_after), instance_count(&perspective_after));
}

#[semio_framework_async_macros::async_test]
async fn seeded_objects_omit_reveal_index_so_the_boot_cutoff_cannot_hide_them() {
    let mut app = app();
    let rendered = render_composite(&mut app);
    let instances = instances_of(&rendered);
    assert!(!instances.is_empty(), "the default fixture seeds at least one object");
    for instance in &instances {
        assert!(instance.get("revealIndex").is_none(), "seeded object {} must omit revealIndex — a null coerces to 0 and the boot cutoff would hide its mesh", instance.get("id").and_then(Value::as_str).unwrap_or("?"));
    }
    assert_eq!(interaction_of(&rendered).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(0), "the boot cutoff really is 0 — this is the value that hid every mesh while revealIndex serialized as null");
}

#[semio_framework_async_macros::async_test]
async fn fill_count_measure_shows_planning_progress_while_precompute_incomplete() {
    let mut session = Puzzle3dPrecomputeSession::new();
    let scene = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
    sync_precompute_session(&mut session, &scene);
    session.precompute_step(1);
    match fill_tool::count_measure(&scene, &session, &Puzzle3dLabels::NATIVE_EN) {
        WindowMeasure::Slider { label: Some(label), max, ready, loading, .. } => {
            assert_eq!(label, Puzzle3dLabels::NATIVE_EN.count.as_str(), "fill count label stays fixed as Count while planning");
            assert_eq!(max, PUZZLE3D_FILL_COUNT_MAX as f64, "fill slider max stays fixed while planning");
            let ready = ready.expect("planning must expose a ready extent");
            assert!(ready >= 0.0 && ready <= max, "ready extent must lie on the fixed range");
            assert_eq!(loading, Some(true), "planning must mark the measure tree leaf as loading");
        }
        other => panic!("expected a slider measure, got {other:?}"),
    }
}
//#endregion 🔖️Fill

//#region 🔖️Distribution
#[semio_framework_async_macros::async_test]
async fn puzzle3d_normalize_kind_weight_group_redistributes_siblings_proportionally() {
    let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut weights = HashMap::from([("a".to_string(), 0.2), ("b".to_string(), 0.3), ("c".to_string(), 0.5)]);
    weights = puzzle3d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
    let sum: f64 = ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    assert!((sum - 1.0).abs() < 1e-9, "simplex must stay at 1, got {sum}");
    assert!((weights.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 1e-9);
    // b:c were 0.3:0.5 — remainder 0.5 splits 0.3/0.8 and 0.5/0.8
    assert!((weights.get("b").copied().unwrap_or(0.0) - 0.5 * 0.3 / 0.8).abs() < 1e-9);
    assert!((weights.get("c").copied().unwrap_or(0.0) - 0.5 * 0.5 / 0.8).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_vortex_measure_exposes_joint_weight_scaled_by_object() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_weights = HashMap::from([("c-b".to_string(), 0.75), ("b-s".to_string(), 0.25)]);
    let object_weight = *object_weights.get("Object").unwrap();
    let measures = puzzle3d_joint_vortex_measures("Object", object_weight, &vortex_ids, &vortex_weights);
    match &measures[0] {
        WindowMeasure::Slider { value, max, step, disabled, .. } => {
            let expected_joint = puzzle3d_joint_vortex_weight(object_weight, 0.75);
            assert!((*value - expected_joint).abs() < 1e-9, "slider must show P(object)×P(vortex), got {value}");
            assert!((*max - object_weight).abs() < 1e-9, "joint range max is P(object)");
            assert_eq!(*step, Some(object_weight * 0.01), "step tracks 1% of P(object)");
            assert_eq!(*disabled, None);
        }
        other => panic!("expected vortex slider, got {other:?}"),
    }
    let raised = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.8);
    let raised_weight = *raised.get("Object").unwrap();
    let raised_measures = puzzle3d_joint_vortex_measures("Object", raised_weight, &vortex_ids, &vortex_weights);
    match (&measures[0], &raised_measures[0]) {
        (WindowMeasure::Slider { value: before, .. }, WindowMeasure::Slider { value: after, .. }) => {
            assert!(*after > *before, "raising P(object) must raise joint vortex percentages");
            assert!((*after - raised_weight * 0.75).abs() < 1e-9);
        }
        _ => panic!("expected vortex sliders"),
    }
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_distribution_lists_global_vortices_and_joints_sum_to_one() {
    let fixture = nakagin_fixture();
    let object_ids = puzzle3d_kind_ids(&fixture, "objects");
    let vortex_ids = puzzle3d_kind_ids(&fixture, "vortices");
    assert!(object_ids.len() >= 2, "default fixture needs multiple object kinds");
    assert!(vortex_ids.len() >= 2, "default fixture needs multiple vortex kinds");
    let object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_kind_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
    let scene = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime { object_kind_weights, vortex_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
    let distribution_children = puzzle3d_distribution_children(&scene, Some(true));
    assert_eq!(distribution_children.len(), object_ids.len());
    let mut joint_sum = 0.0;
    for measure in &distribution_children {
        let WindowMeasure::Group { children, value: Some(object_weight), .. } = measure else {
            panic!("expected object-kind group");
        };
        assert_eq!(children.len(), vortex_ids.len(), "each object must list the full global vortex catalog");
        let local_sum: f64 = children
            .iter()
            .map(|child| match child {
                WindowMeasure::Slider { value, .. } => *value,
                _ => panic!("expected vortex slider"),
            })
            .sum();
        assert!((local_sum - object_weight).abs() < 1e-6, "under one object joints sum to P(object), not 1");
        joint_sum += local_sum;
    }
    assert!((joint_sum - 1.0).abs() < 1e-6, "all nested joint percentages across objects must sum to 1, got {joint_sum}");
}

#[semio_framework_async_macros::async_test]
async fn puzzle3d_object_weight_change_scales_joint_sampling_product() {
    let object_ids = vec!["Object".to_string(), "Placed".to_string()];
    let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
    let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
    let vortex_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
    object_weights = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.6);
    let object_weight = *object_weights.get("Object").unwrap();
    let vortex_weight = *vortex_weights.get("c-b").unwrap();
    let joint_before = puzzle3d_joint_vortex_weight(0.5, vortex_weight);
    let joint_after = puzzle3d_joint_vortex_weight(object_weight, vortex_weight);
    assert!(joint_after > joint_before);
}

/// 🚫️ Zero object-kind weight disables every vortex slider under that kind — anything × 0 is 0.
#[semio_framework_async_macros::async_test]
async fn zero_object_kind_weight_disables_joint_vortex_sliders() {
    let labels = puzzle3d_labels(&Puzzle3dConfig::default()).expect("default puzzle3d axes are explicit");
    let session = Puzzle3dPrecomputeSession::new();
    let fixture = nakagin_fixture();
    let object_ids = puzzle3d_kind_ids(&fixture, "objects");
    assert!(!object_ids.is_empty(), "default fixture must expose object kinds");
    let zeroed_id = object_ids[0].clone();
    let mut object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
    object_kind_weights = puzzle3d_normalize_kind_weight_group(&object_kind_weights, &object_ids, &zeroed_id, 0.0);
    assert!(object_kind_weights.get(&zeroed_id).copied().unwrap_or(1.0) <= f64::EPSILON);
    let scene = Puzzle3dScene { fixture, runtime: Puzzle3dRuntime { object_kind_weights, ..Puzzle3dRuntime::default() }, active_utility: fill_tool::TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&scene, &session, labels);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    let zeroed_group_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{zeroed_id}");
    let zeroed_group = distribution_children.iter().find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == &zeroed_group_id)).expect("zeroed object kind must appear in distribution");
    match zeroed_group {
        WindowMeasure::Group { value: Some(value), children, .. } => {
            assert!(*value <= f64::EPSILON, "object-kind header must read 0%");
            assert!(!children.is_empty(), "object kind must still list vortex sliders");
            assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: Some(true), value, .. } if *value <= f64::EPSILON)), "every joint vortex slider under a 0% object kind must be disabled at 0%");
        }
        other => panic!("expected object-kind group, got {other:?}"),
    }
    let live_group = distribution_children.iter().find(|measure| match measure {
        WindowMeasure::Group { id, value: Some(value), .. } if id != &zeroed_group_id => *value > f64::EPSILON,
        _ => false,
    });
    if let Some(WindowMeasure::Group { children, .. }) = live_group {
        assert!(children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: None | Some(false), .. })), "joint vortex sliders under a non-zero object kind must stay enabled");
    }
}

/// 🎯️ Fill tool measures expose count + nested distribution tree under the Fill toggle; the Volume
/// Brush voxel dims live in a utility-options group in the window's own measures.
#[semio_framework_async_macros::async_test]
async fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
    let labels = puzzle3d_labels(&Puzzle3dConfig::default()).expect("default puzzle3d axes are explicit");
    let session = Puzzle3dPrecomputeSession::new();
    let fill_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: fill_tool::TOOL_ID.into() };
    let fill_measures = fill_tool::measures(&fill_scene, &session, labels);
    let distribution_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution");
    assert!(!fill_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle3d-play-tool-options-fill")), "fill must not wrap its options in a nested Fill group — the tool toggle already owns that row");
    assert_eq!(measure_group_tag(&fill_measures, &distribution_id), Some(None));
    let distribution_children = fill_measures
        .iter()
        .find_map(|measure| match measure {
            WindowMeasure::Group { id, children, .. } if id == &distribution_id => Some(children.as_slice()),
            _ => None,
        })
        .expect("fill must expose a Distribution group");
    assert!(!distribution_children.is_empty(), "distribution must list object-kind groups");
    assert!(distribution_children.iter().all(|measure| matches!(measure, WindowMeasure::Group { value: Some(_), on_change: Some(_), .. })), "each object-kind group must carry a header weight slider");
    assert!(
        distribution_children.iter().all(|measure| match measure {
            WindowMeasure::Group { label, value: Some(_), on_change: Some(_), .. } => !label.contains('%'),
            _ => false,
        }),
        "object-kind group labels must not embed percentages — the header slider owns the value readout"
    );
    assert!(
        distribution_children.iter().any(|measure| match measure {
            WindowMeasure::Group { children, .. } => children.iter().any(|child| matches!(child, WindowMeasure::Slider { label: Some(label), .. } if !label.contains('%'))),
            _ => false,
        }),
        "vortex joint sliders must label kinds without embedding percentages"
    );
    assert!(find_measure_toggle(&fill_measures, "puzzle3d-edit-volumes").is_none(), "fill must not carry edit-volumes toggle");
    assert!(find_measure_slider(&fill_measures, "puzzle3d-voxel-w").is_none(), "fill must not carry voxel-dimension sliders");
    assert!(find_measure_slider(&fill_measures, "puzzle3d-fill-count").is_some(), "fill-count slider always lives in the fill tool measures");
    assert!(
        !main::window_measures(&fill_scene, &session, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
        "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
    );
    let volume_brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::volume_brush::UTILITY_ID.into() };
    let volume_brush_measures = main::window_measures(&volume_brush_scene, &session, labels);
    assert_eq!(measure_group_tag(&volume_brush_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush")), Some(Some(utilities::volume_brush::UTILITY_ID.into())));
    assert!(find_measure_slider(&volume_brush_measures, "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
    let fill_engagement = main::engagement(&fill_scene, &Puzzle3dLabels::NATIVE_EN);
    assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
    let brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(measure_group_tag(&main::window_measures(&brush_scene, &session, labels), &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())));
    let brush_engagement = main::engagement(&brush_scene, &Puzzle3dLabels::NATIVE_EN);
    assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
    // 🖌️ Positive case: opening a vortex's suggestions selects it and drives precompute so real
    // candidates exist — the brush Utility Options group must then surface, tagged for "brush".
    let mut app = app();
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).expect("openVortexSuggestions");
    let brush_app_measures = semio_framework::io::resolve_ready(app.window_measures());
    let window_measures = brush_app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())), "the brush Utility Options group surfaces once there are candidates to place");
}
//#endregion 🔖️Distribution

//#region 🔖️UiScope
#[semio_framework_async_macros::async_test]
async fn fill_build_tick_is_a_view_action_with_narrow_ui_scope() {
    let definition = create_puzzle3d_app();
    let def = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == "fillBuildTick").expect("fillBuildTick declared");
    assert_eq!(def.kind, ActionKind::View, "fillBuildTick must stay a View action — it only advances background planning");
    let mut live = app();
    dispatch(&mut live, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    let result = dispatch(&mut live, "fillBuildTick", None, None).expect("fillBuildTick");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools, "fill planning must refresh the fill-count slider range in the fill tool's measures");
            assert!(!measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for fillBuildTick, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_fill_count_declares_narrow_ui_scope() {
    let mut app = app();
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 1 })), None).expect("setFillCount");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools);
            assert!(!measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_object_kind_weight_declares_fill_options_ui_scope() {
    let mut app = app();
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).expect("select fill tool");
    let object_ids = puzzle3d_kind_ids(&nakagin_fixture(), "objects");
    let kind_id = object_ids.first().expect("object kind");
    let result = dispatch(&mut app, "setObjectKindWeight", Some(&json!({ "kindId": kind_id.as_str(), "value": 0.75 })), None).expect("setObjectKindWeight");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            assert!(panel_bodies.is_empty());
            assert!(tools);
            assert!(measures, "distribution sliders live in tool + window measures");
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setObjectKindWeight, got {other:?}"),
    }
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `set_hover_is_a_view_action_with_no_ops_after_document_mutation`
// and `world_pick_declares_selection_ui_scope` deleted — both dispatched the now-deleted
// `setHover`/`worldPick` actions and asserted on the deleted `Effect::PatchWorld3dChrome`
// push-setter effect (selection/hover are framework-owned actions now, dispatched exclusively
// through the six reserved `interactionSelect`-family verbs; see `select_id`/`hover_id`).
//#endregion 🔖️UiScope

//#region 🔖️Utilities
#[semio_framework_async_macros::async_test]
async fn add_object_kind_honors_drop_origin() {
    let mut app = app();
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), None).expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1);
    let projection = projection_of(&app);
    let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("added object");
    let origin = object.get("origin").and_then(Value::as_array).expect("origin array");
    assert_eq!(origin.first().and_then(Value::as_f64), Some(2.5));
    assert_eq!(origin.get(1).and_then(Value::as_f64), Some(3.5));
    assert_eq!(origin.get(2).and_then(Value::as_f64), Some(0.0));
}

#[semio_framework_async_macros::async_test]
async fn add_object_kind_materializes_the_declared_kind_default() {
    // 📝️ P1 arg form: firing addObjectKind with no args must materialize the declared `objectKind`
    // default and emit the object-add operation under registry enforcement.
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
    let before = object_count(&app);
    let result = dispatch(&mut app, "addObjectKind", None, None).expect("addObjectKind");
    assert!(!result.mutations.is_empty(), "addObjectKind is a Mutation that emits mutations");
    assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
    let projection = projection_of(&app);
    let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
    assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework-injected View action: no document operations, no undo
    // entry, no re-emitted utility-switch effect (the command IS the direct switch).
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), None).expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn engagement_exposes_no_utility_switch_options() {
    // 🧰️ select/brush/fill switching lives only on the framework utility bar; the engagement HUD
    // must not duplicate it as options.
    let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let engagement = main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN);
    assert!(engagement.options.is_none(), "the puzzle3d engagement must not re-expose utility switching as options");
}

#[semio_framework_async_macros::async_test]
async fn transform_engagement_does_not_block_background_deselect() {
    let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    assert_eq!(main::engagement(&scene, &Puzzle3dLabels::NATIVE_EN).session_active, Some(false));
}
//#endregion 🔖️Utilities

//#region 🔖️WorldSelection
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the app-owned `worldSelect`
/// command is deleted — selection now goes exclusively through the framework's `interactionSelect`
/// verb (`select_id`), which is view-only by construction (`dispatch_interaction_action` never
/// touches `self.store`). Proves the `vortex` domain wiring reaches that same guarantee.
#[semio_framework_async_macros::async_test]
async fn world_select_emits_no_artifact_mutations() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let object_id = first_object_id(&app);
    let result = select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("interactionSelect");
    assert!(result.mutations.is_empty(), "interactionSelect is framework-owned and view-only, must not diff the document");
    assert_eq!(projection_of(&app), before);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: `selectionJson`'s `ids`
/// used to mirror the live pick. `world_selection_json` has no `InteractionView` to draw from any
/// more (see that function's doc comment) and always emits an empty `ids: []` now — the real
/// selection is verified below via `VcsArtifactApp::interaction_state()` instead, the framework's
/// own sanctioned test-visible source of truth.
#[semio_framework_async_macros::async_test]
async fn world_pick_keeps_instances_geometry_json_stable() {
    let mut app = app_with_registry();
    let instances_before = instances_of(&render_composite(&mut app));
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("interactionSelect");
    let after = render_composite(&mut app);
    assert_eq!(instances_of(&after), instances_before, "picking must never perturb instance geometry");
    assert_eq!(semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()), Some(vec![object_id]));
}

#[semio_framework_async_macros::async_test]
async fn world_pick_null_clears_without_reselecting_first_object() {
    let mut app = app_with_registry();
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("select");
    assert!(semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| !selection.ids.is_empty()));
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).expect("clear");
    assert!(
        semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "clicking empty background must clear, never fall back to reselecting the first object"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove that
/// world-picking a LOCKED object clears the selection instead of selecting it — the app-owned
/// `worldPick` command, which could read `object.locked` before deciding, is deleted.
/// `interactionSelect` is a framework-generic id/merge verb blind to app data, and this app's
/// `interaction_topology` does not filter locked ids out of the `vortex` domain either (see that
/// function's doc) — "never select a locked object" is now entirely a host click→pick
/// translation concern, not reachable from this crate. Proves the Rust-side floor instead: the
/// `vortex` domain has no lock awareness, so selecting a locked object's id still succeeds.
#[semio_framework_async_macros::async_test]
async fn world_pick_locked_object_clears_like_background() {
    let mut app = app_with_registry();
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "flag": "locked", "value": true })), None).expect("lock");
    let instances = instances_of(&render_composite(&mut app));
    assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("select locked object");
    assert_eq!(
        semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()),
        Some(vec![object_id]),
        "the vortex domain has no lock awareness — this now succeeds, the host must gate locked picks itself"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove
/// `PUZZLE3D_VORTEX_SHOW_SELECTED` reveals markers on hover/selection (`worldHover`/`worldPick`,
/// both deleted — selection/hover are framework-owned now). `render` has no `InteractionView` to
/// check against (see `object_vortices_visible`'s doc comment), so `Selected` mode degrades to
/// "never reveal" until that framework gap closes — this now proves that degraded floor instead.
#[semio_framework_async_macros::async_test]
async fn world_vortices_stay_hidden_in_selected_mode_pending_the_render_interaction_gap() {
    let mut app = app();
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
    assert!(vortices_of(&render_composite(&mut app)).is_empty(), "Selected mode with no render-time interaction access must hide every vortex marker");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).expect("setVortexShow");
    assert!(!vortices_of(&render_composite(&mut app)).is_empty(), "Always mode must still reveal every vortex marker");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `worldVortexSelect`/`worldPick`
/// are deleted; a `vortex`-domain `DomainSelection` only ever carries one granularity at a time
/// (see `Puzzle3dActionCtx::selected_ids`'s doc), so a `merge: "replace"` pick at a different
/// granularity inherently replaces the whole prior selection — verified against
/// `interaction_state()` (the render-time `selectionJson`/`vorticesJson` fields carry no live ids
/// any more, per `world_selection_json`'s known-gap doc comment).
#[semio_framework_async_macros::async_test]
async fn world_pick_object_replaces_vortex_selection() {
    let mut app = app_with_registry();
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).expect("select vortex");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("select object");
    let selection = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids, vec![object_id]);
}

#[semio_framework_async_macros::async_test]
async fn world_vortex_select_clears_object_selection() {
    let mut app = app_with_registry();
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("select object");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).expect("select vortex");
    let selection = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_VORTEX);
    assert_eq!(selection.ids, vec![vortex]);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to prove a
/// PERSISTED "default merge mode" (`selection_mode_default`, a config field — on this crate's
/// DELETE list) flips `worldVortexSelect`'s implicit merge between replace/invertive. The
/// framework has no equivalent persisted concept: `interactionSelect`'s `merge` arg is supplied
/// explicitly on every dispatch (the host decides per click — e.g. a held modifier key — never
/// defaulted from stored state). Proves the underlying `merge: "invertive"` primitive still
/// toggles a second target back into the selection instead.
#[semio_framework_async_macros::async_test]
async fn world_vortex_click_replaces_until_invertive_mode_is_selected() {
    let mut app = app_with_registry();
    let vortices = vortex_full_ids(&app);
    assert!(vortices.len() >= 2, "fixture must expose two vortices");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[0]).expect("select first vortex");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[1]).expect("replace with second vortex");
    let replaced = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(replaced.ids, vec![vortices[1].clone()]);

    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), id: vortices[0].clone() }]);
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "invertive", "method": "pick" })), None).expect("invertive toggle");
    let invertive = semio_framework::io::resolve_ready(app.interaction_state()).selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(invertive.ids.len(), 2, "invertive merge toggles the first vortex back into the selection alongside the second");
}
//#endregion 🔖️WorldSelection

//#region 🔖️Gumball
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: `gumballActive` used to
/// require BOTH the transform utility active AND a live object selection — `render`'s
/// `gumball_active` has no `InteractionView` to check selection against any more (see that
/// function's doc comment) and always degrades to `false`. `transformMode`/`gumballConfig` never
/// depended on selection (only on the active utility, per-window), so those stay meaningfully
/// tested; the selection setup and the once-`true` gumball assertion are gone.
#[semio_framework_async_macros::async_test]
async fn gumball_active_only_for_transform_utilities_with_object_selection() {
    let mut app = app_with_registry();
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("interactionSelect");
    let idle_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
    assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
    assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");

    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).expect("transform");
    let transform_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
    assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "render has no InteractionView, so gumballActive can no longer track the live selection");
    assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
    assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));

    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).expect("brush");
    let brush_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
    assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
    assert!(brush_selection.get("transformMode").is_none());
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: `gumballActive` no
/// longer proves anything per-window (always `false` — see the sibling test's doc comment above);
/// `transformMode` never depended on selection, only on each window instance's own
/// `active_utility_by_window_id`, so it stays the meaningful per-window-isolation proof here.
#[semio_framework_async_macros::async_test]
async fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut app = app();
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(top)).expect("transform on top");
    let top_selection = selection_of(&render_window(&mut app, top));
    assert_eq!(top_selection.get("transformMode").and_then(Value::as_str), Some("transform"), "transform on top pane must switch that pane's own scene mode");
    let perspective_selection = selection_of(&render_window(&mut app, perspective));
    assert!(perspective_selection.get("transformMode").is_none(), "perspective pane must not inherit top pane's transform utility");
}

#[semio_framework_async_macros::async_test]
async fn transform_utility_options_expose_move_and_rotate_flags() {
    let labels = puzzle3d_labels(&Puzzle3dConfig::default()).expect("default puzzle3d axes are explicit");
    let session = Puzzle3dPrecomputeSession::new();
    let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    let measures = main::window_measures(&scene, &session, labels);
    assert_eq!(measure_group_tag(&measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform")), Some(Some(utilities::transform::UTILITY_ID.into())));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
    let mut app = app();
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).expect("disable rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID));
    assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
    let app_measures = semio_framework::io::resolve_ready(app.window_measures());
    let window_measures = app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_toggle(window_measures, "puzzle3d-transform-rotate"), Some(false));
}

fn object_origin(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("origin").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn gumball_translate_drag_coalesces_into_one_edit() {
    // 🌀️ Unbracketed translate ticks still coalesce via AmendLast (compat path without transformBegin).
    let mut app = app();
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    for dx in [1.0, 2.0, 3.0] {
        dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": dx, "dy": 0.0, "dz": 0.0 })), None).expect("drag tick");
    }
    let dragged = object_origin(&app, &object_id);
    assert!((dragged[0] - start[0] - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole coalesced gumball drag");
}

#[semio_framework_async_macros::async_test]
async fn gumball_transform_session_commits_once_on_end() {
    // 🧲️ Scratch-commit: mid-drag ticks emit ZERO operations; transformEnd commits ONE edit from
    // base→scratch. Incremental host deltas accumulate on scratch — 1 then 5 → final +6.
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).expect("add object");
    let object_id = first_object_id(&app);
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).expect("transform");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).expect("interactionSelect");
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "transformBegin", None, None).expect("begin");
    let tick_a = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).expect("tick a");
    let tick_b = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 5.0, "dy": 0.0, "dz": 0.0 })), None).expect("tick b");
    assert!(tick_a.mutations.is_empty() && tick_b.mutations.is_empty(), "mid-drag transform ticks emit no operations");
    assert_eq!(object_origin(&app, &object_id), start, "document stays at the drag-start pose mid-drag");
    let preview: Vec<f64> = instances_of(&render_window(&mut app, main::WINDOW_KIND_ID))
        .iter()
        .find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id.as_str()))
        .and_then(|instance| instance.get("position").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default();
    assert!((preview[0] - start[0] - 6.0).abs() < 1e-9, "scratch render accumulates incremental ticks");
    let end = dispatch(&mut app, "transformEnd", None, None).expect("end");
    assert_eq!(end.mutations.len(), 1, "the whole drag commits as exactly one operation");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 6.0).abs() < 1e-9, "transformEnd lands on the accumulated total");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole scratch-committed gumball drag");
    dispatch(&mut app, "transformBegin", None, None).expect("begin again");
    dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), None).expect("second drag tick");
    dispatch(&mut app, "transformEnd", None, None).expect("second end");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 2.0).abs() < 1e-9, "a second gumball drag session works from the restored base");
}
//#endregion 🔖️Gumball

//#region 🔖️GesturePreview
/// 🔬️ CW7 preview-law seam: `Puzzle3dPlayApp::gesture_preview` reads `transform_base`/
/// `transform_scratch` only, never a `Puzzle3dMutation` — exercised directly against
/// `Puzzle3dPlayApp` (bypassing the `VcsArtifactApp` wrapper, which has no accessor into the
/// inner app) since `transform_drag_tick` is the natural per-tick gesture handler.
#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_none_without_an_active_transform_drag() {
    let app = Puzzle3dPlayApp::default();
    assert!(app.gesture_preview().is_none(), "no live gumball drag, nothing to preview");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_reflects_the_live_gumball_drag_and_clears_on_commit() {
    let app = Puzzle3dPlayApp::default();
    let fixture = default_fixture();
    let object_id = fixture.objects[0].id.clone();
    let projection = json::from_dsl_value(&dsl::ToValue::to_value(&fixture));
    *app.transform_drag_active.borrow_mut() = true;
    let no_volumes: Vec<String> = Vec::new();

    let tick_a = app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id.clone()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), &projection, &[object_id.clone()], &no_volumes);
    assert!(tick_a.artifact_mutations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
    let (key, seq_after_a, payload_a) = app.gesture_preview().expect("a live gumball drag is previewable");
    assert_eq!(key, "gesture:transform");
    let value_a: Value = parse(std::str::from_utf8(&payload_a).expect("payload is valid utf8")).expect("payload is valid json");
    assert!(!value_a["operations"].as_array().expect("operations array").is_empty(), "the delta anchored to the drag-start snapshot must reflect the first tick");

    let tick_b = app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id.clone()], "dx": 5.0, "dy": 0.0, "dz": 0.0 })), &projection, &[object_id.clone()], &no_volumes);
    assert!(tick_b.artifact_mutations.is_empty());
    let (_, seq_after_b, payload_b) = app.gesture_preview().expect("still live mid-drag");
    assert!(seq_after_b > seq_after_a, "seq is monotone per tick, for staleness detection on the receiving end");
    assert_ne!(payload_a, payload_b, "the base-anchored delta accumulates both ticks, not just the latest one");

    let end = app.commit_transform(&projection, &[object_id]);
    assert_eq!(end.artifact_mutations.len(), 1, "the whole drag commits as exactly one real operation");
    assert!(app.gesture_preview().is_none(), "the drag ended: nothing left to preview, and the commit above already carried the real operation");
}

#[semio_framework_async_macros::async_test]
async fn gesture_preview_is_a_pure_read_never_mutating_the_transform_scratch() {
    let app = Puzzle3dPlayApp::default();
    let fixture = default_fixture();
    let object_id = fixture.objects[0].id.clone();
    let projection = json::from_dsl_value(&dsl::ToValue::to_value(&fixture));
    *app.transform_drag_active.borrow_mut() = true;
    app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id.clone()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), &projection, &[object_id], &[]);
    let scratch_before = app.transform_scratch.borrow().clone();
    let _ = app.gesture_preview();
    let _ = app.gesture_preview();
    assert_eq!(*app.transform_scratch.borrow(), scratch_before, "gesture_preview must never mutate the live transform scratch it reads");
}
//#endregion 🔖️GesturePreview

//#region 🔖️KitInPort
/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like block3d's
/// `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle3dPlayApp::import_media` must normalize `objectKinds` → `objects` / `vortexKinds` →
/// `vortices` and, after applying the returned operations, land that object kind inside
/// `meta.kind_catalogs.objects` (and the vortex kind inside `.vortices`).
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_upserts_object_and_vortex_kinds_into_meta_kind_catalogs() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);

    let fragment = json!({
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

    let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
    assert!(!emit.artifact_mutations.is_empty(), "importing a non-empty fragment must emit real operations");

    let mut next_projection = projection.value().clone();
    for operation in &emit.artifact_mutations {
        next_projection = protocol::Mutation::<serde_json::Value>::diff(operation, &next_projection).diff().apply(&next_projection).expect("valid mutation diff");
    }

    let next_projection = parse(&next_projection.to_string()).expect("mutated snapshot JSON");
    let objects = next_projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert!(objects.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")), "the imported object kind must appear in meta.kind_catalogs.objects");
    let capsule = objects.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).unwrap();
    assert_eq!(capsule.pointer("/representations/0/url").and_then(Value::as_str), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.pointer("/vortices/0/vortexKind").and_then(Value::as_str), Some("door"), "the per-object vortex template keeps its vortexKind after normalization");

    let vortices = next_projection.pointer("/meta/kindCatalogs/vortices").and_then(Value::as_array).expect("vortices catalog present");
    assert!(vortices.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("door")), "the imported vortex kind must appear in meta.kind_catalogs.vortices");

    let compatibility = next_projection.pointer("/meta/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (a second producer edge, or a redelivered message on a
/// `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
    let projection = Puzzle3dPlayApp::initial_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let mut current = projection.value().clone();

    let fragment = json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        let doc_projection = Puzzle3dPlaySnapshot::new(current.clone());
        let doc = ArtifactView::new(&doc_projection, &history);
        let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
        for operation in &emit.artifact_mutations {
            current = protocol::Mutation::<serde_json::Value>::diff(operation, &current).diff().apply(&current).expect("valid mutation diff");
        }
    }

    let current = parse(&current.to_string()).expect("mutated snapshot JSON");
    let objects = current.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert_eq!(objects.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle3dPlayApp::io().expect("puzzle3d declares an AppIo");
    let port = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(port.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(port.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(port.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort

//#region 🔖️Convergence
/// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT object edits and,
/// after exchanging operations, both converge to contain BOTH objects — impossible under
/// whole-document `setSnapshot` snapshots, which would clobber one side.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_object_edits_via_backbone() {
    use store::MemoryBackbone;
    let mut instance_a = app();
    let mut instance_b = app();
    let seeded = object_count(&instance_a);
    let (backbone_a, backbone_b) = semio_framework::io::resolve_ready(MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence"));
    semio_framework::io::resolve_ready(instance_a.attach_backbone(store::Backbones::Memory(backbone_a))).expect("attach a");
    semio_framework::io::resolve_ready(instance_b.attach_backbone(store::Backbones::Memory(backbone_b))).expect("attach b");

    dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).expect("a adds object");
    dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).expect("b adds object");

    // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
    dispatch(&mut instance_a, "commitCheckpoint", None, None).expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).expect("pump b");

    assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
    assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
    let _ = Puzzle3dCamera::default();
}

//#endregion 🔖️Convergence
