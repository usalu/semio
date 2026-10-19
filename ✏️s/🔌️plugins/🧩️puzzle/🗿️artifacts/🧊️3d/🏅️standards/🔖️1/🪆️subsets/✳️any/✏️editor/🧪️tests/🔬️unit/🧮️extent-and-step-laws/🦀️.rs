/// 🧮️ Extent and step laws: the Nakagin-scale extents and step loops of world relocation, attraction creation, suggestion acceptance, inspector patches and the active example switch.

/// 🧮️ The Relocate utility's retained work reads the drop, pages its proximity scan — one step per
/// `PUZZLE3D_RELOCATE_SCAN_PAGE` objects — and commits ONE selection record through the transform tool, so even
/// Nakagin's 180 objects admit it far inside the fixed work cap.
#[test]
fn world_relocate_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "nonexistent", "position": [0.0, 0.0, 0.0] })), None).expect("worldRelocate command decodes");
    let work = Puzzle3dTransformWork::new("worldRelocate", "seed".into());
    let pages = NAKAGIN_EXAMPLE_SNAPSHOT.objects.len().div_ceil(semio_s_artifact_puzzle_3d_editor_transform::PUZZLE3D_RELOCATE_SCAN_PAGE);
    assert_eq!(work.extent(&command, &snapshot, &interaction, None), Some(2 + pages), "a relocate reads its gesture, scans one page per step, then commits it");
    assert!(2 + pages <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
}

/// ⏹️ A Nakagin drop's proximity scan is paged and cancellable: every page is its own `Progress` step, and a work
/// closed in the middle of the scan retires to its terminal-empty shell without ever having published a mutation.
#[test]
fn world_relocate_scan_pages_progress_and_cancels_with_zero_trace() {
    let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);

    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "25b0dba0-8f81-423a-94a1-b911a6031010", "position": [-8.84, -2.8499999999999996, 7.7] })), None).expect("worldRelocate command decodes");
    let mut work = Puzzle3dTransformWork::new("worldRelocate", "seed".into());
    let mut stages = Vec::new();
    for _ in 0..3 {
        match semio_s_plugin_puzzle_retained::testing::step(&mut original, &mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Pending { .. } => continue,
            PuzzleCommandWorkStep::Progress { stage, en, de } => {
                assert!(!en.is_empty() && !de.is_empty() && en != de, "every progress step is localized in both languages");
                stages.push(stage);
            }
            other => panic!("a 180-object scan cannot finish in three steps: {}", matches!(other, PuzzleCommandWorkStep::Complete(_))),
        }
    }
    assert_eq!(stages, ["puzzle3d-transform-read", "puzzle3d-transform-scan", "puzzle3d-transform-scan"], "the scan reports one progress per page");
    let turns = semio_s_plugin_puzzle_retained::testing::close(&mut original, &mut work);
    assert!(turns < 8, "closing a scanning relocate retires in a bounded number of turns");
    original.finish();
}

/// 🔁️ A real Nakagin drop: object `25b0dba0-8f81-423a-94a1-b911a6031010` ("Capsule With Balcony
/// Backslash") moves one centimetre along x, and its one vortex (`…:link`, kind "door capsule right") still
/// lands within `proximity_radius` (0.75, `default_proximity_radius()`) of the "door tambour right" vortices
/// on neighbour `5f0266bc-…`. The work finishes inside its own extent and commits ONE transaction: the
/// `drag-selection` plus at least one `connect-vortices`, all stamped with the ref it minted.
#[test]
fn world_relocate_step_loop_stays_within_its_own_extent_for_nakagin() {
    let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);

    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("worldRelocate", Some(json!({ "objectId": "25b0dba0-8f81-423a-94a1-b911a6031010", "position": [-8.84, -2.8499999999999996, 7.7] })), None).expect("worldRelocate command decodes");
    let mut work = Puzzle3dTransformWork::new("worldRelocate", "seed".into());
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("a relocate always fits the fixed bounded work envelope");
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= extent, "worldRelocate step() ran past its own declared extent {extent}");
        match semio_s_plugin_puzzle_retained::testing::step(&mut original, &mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Pending { .. } => continue,
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(matches!(emit.artifact_mutations.first(), Some(Puzzle3dMutation::DragSelection(_))), "the drop yields its drag first: {:?}", emit.artifact_mutations.first());
    assert!(emit.artifact_mutations.iter().skip(1).any(|mutation| matches!(mutation, Puzzle3dMutation::ConnectVortices(_))), "the relocated vortex sits within proximity_radius of a real neighbour, so the drop attracts it; observed {:?}", emit.artifact_mutations);
    assert!(emit.transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#worldRelocate")), "the drop is ONE tool transaction: {:?}", emit.transaction);
    semio_s_plugin_puzzle_retained::testing::close(&mut original, &mut work);
    original.finish();
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dCreateAttractionWork::extent` doubled the
/// same flat per-object 64-vortex charge across two endpoint scans, computing `objects * 128 + ...`
/// = 23,055 on Nakagin's 180 objects — nearly 6x the cap. The rewritten bound counts the document's
/// actual vortices instead of assuming every object carries the worst-case vortex count.
#[test]
fn create_attraction_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "nonexistent-a", "attracted": "nonexistent-b" })), None).expect("createAttraction command decodes");
    let work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, "createAttraction extent {extent} must not exceed the fixed cap {}", semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real
/// `Attracting`/`Attracted` full-document vortex scans instead of only checking the bound fits
/// under the cap. `attracting`/`attracted` name a genuine compatible pair already present in the
/// Nakagin scene_snapshot — `25b0dba0-…:link` (kind "door capsule right", object index 27 in DSL
/// declaration order) and `5f0266bc-…:sl0_d0` (kind "door tambour right", object index 64) — which
/// `kind-compatibility` marks bidirectionally compatible, so `step()` runs every real stage
/// (`Existing → Attracting → Attracted → Compatibility → Publish`) to a genuine success instead of
/// one of the Work's several early-`Complete` short-circuits (duplicate/incompatible/empty-id).
#[test]
fn create_attraction_step_loop_stays_within_its_own_extent_for_nakagin() {
    let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);

    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command =
        Puzzle3dCommand::from_action("createAttraction", Some(json!({ "attracting": "25b0dba0-8f81-423a-94a1-b911a6031010:link", "attracted": "5f0266bc-856b-4ef2-9eb0-16ef5e1fb952:sl0_d0" })), None).expect("createAttraction command decodes");
    let mut work = Puzzle3dCreateAttractionWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "createAttraction step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match semio_s_plugin_puzzle_retained::testing::step(&mut original, &mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Pending { .. } => continue,
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
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
    semio_s_plugin_puzzle_retained::testing::close(&mut original, &mut work);
    original.finish();
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dAcceptSuggestionWork::extent`'s
/// `target_scans` term charged every scene object the same flat 64-vortex worst case
/// (`objects * 64`), computing 7,888+ on Nakagin's 180 objects. The rewritten bound counts the
/// document's actual vortices for the target scan while keeping the catalog-kind-cap terms
/// (`PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT`), which bound a real runtime-enforced invariant on
/// catalog kinds, not scene object instances.
#[test]
fn accept_suggestion_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", None, None).expect("acceptSuggestion command decodes");
    let work = original.construct(Puzzle3dAcceptSuggestionWork::original_birth_demands().unwrap(), |scope| Puzzle3dAcceptSuggestionWork::from_original_construction(scope));
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, "acceptSuggestion extent {extent} must not exceed the fixed cap {}", semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
}

/// 🔁️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2 (coordinator follow-up): drives the real `Target`
/// scan plus the full `Candidate → Vortices → ExistingAttractions → PublishObject → PublishAttraction →
/// PublishResult` chain to a genuine success, using a real Nakagin vortex (`25b0dba0-…:link`, object index 27
/// in DSL declaration order) as `fullId` and one free candidate the brush suggestions run published for it,
/// instead of the "no target requested" early-`Complete` short-circuit.
#[test]
fn accept_suggestion_step_loop_stays_within_its_own_extent_for_nakagin() {
    let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);

    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), None).expect("acceptSuggestion command decodes");
    let work = original.construct(Puzzle3dAcceptSuggestionWork::original_birth_demands().unwrap(), |scope| Puzzle3dAcceptSuggestionWork::from_original_construction(scope));
    let kind = snapshot.typed().meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.objects.first()).expect("nakagin catalogues object kinds").id.clone();
    let owner = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(Box::new(Puzzle3dInstanceOperationOwner::default()));
    owner
        .with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| {
            let preview = semio_s_artifact_puzzle_3d_model::BrushPreviewState { target_vortex_full_id: "25b0dba0-8f81-423a-94a1-b911a6031010:link".into(), object_kind_id: kind, source_vortex_index: 0, mesh_url: "/nakagin/capsule.glb".into(), origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
            owner.brush_suggestions.found = Some(semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsFound { writer: (1, 0), target: preview.target_vortex_full_id.clone(), previews: vec![Some(preview)], verdicts: vec![semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionVerdict::Free], done: true });
            Ok(())
        })
        .expect("the instance owner");
    let mut work = work.bound(Puzzle3dInstanceBinding { app_instance_id: 1, parent_document_id: "test".to_string(), owner });
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's catalogs and real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "acceptSuggestion step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match semio_s_plugin_puzzle_retained::testing::step(&mut original, &mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Pending { .. } => continue,
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
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
    semio_s_plugin_puzzle_retained::testing::close(&mut original, &mut work);
    original.finish();
}

/// 🧮️ ticket 26/09/02/PUZZLE-3D-END-TO-END §Y2: `Puzzle3dPatchInspectorWork::extent`'s `"vortex"`
/// arm charged `objects * PUZZLE_COMMAND_DECODED_ITEMS` (objects * 512) regardless of real vortex
/// count, computing ~92,160 on Nakagin's 180 objects — a 512x-over-budget design error, not a
/// precision edge case. The rewritten bound counts the document's actual vortex instances plus one
/// `step()` per object, matching the `Vortices` stage's real per-object "owner advance" call.
#[test]
fn patch_inspector_vortex_extent_fits_within_cap_for_nakagin() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let interaction = protocol::InteractionState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex" })), None).expect("patchInspector command decodes");
    let work = Puzzle3dPatchInspectorWork::default();
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    assert!(extent <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, "patchInspector vortex extent {extent} must not exceed the fixed cap {}", semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS);
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
    let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);

    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXAMPLE_SNAPSHOT.clone()).expect("typed fixture admits"));
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("patchInspector", Some(json!({ "entity": "vortex", "field": "hidden", "value": true, "ids": ["25b0dba0-8f81-423a-94a1-b911a6031010:link"] })), None).expect("patchInspector command decodes");
    let mut work = original.construct(Puzzle3dPatchInspectorWork::original_birth_demands().unwrap(), |scope| Puzzle3dPatchInspectorWork::from_original_construction(scope));
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin's real vortex count must fit the fixed bounded work envelope");
    let guard = extent.saturating_mul(4).saturating_add(1000);
    let mut iterations = 0usize;
    let emit = loop {
        assert!(iterations <= guard, "patchInspector step() did not reach Complete within a generous multiple of its own extent {extent}; runaway loop suspected");
        match semio_s_plugin_puzzle_retained::testing::step(&mut original, &mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Pending { .. } => continue,
            PuzzleCommandWorkStep::Progress { .. } => iterations += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(iterations <= extent, "patchInspector step() ran {iterations} real steps, exceeding its own declared extent {extent} — the bound is unsound");
    assert!(
        iterations > 400,
        "patchInspector's Vortices stage unconditionally walks every object and every real vortex regardless of selection, so a real Nakagin run must take hundreds of steps, not a trivial early exit; observed only {iterations} iterations"
    );
    assert_eq!(emit.artifact_mutations.len(), 1, "exactly the one selected vortex must be patched out of the full unconditional scan; observed {} mutations", emit.artifact_mutations.len());
    semio_s_plugin_puzzle_retained::testing::close(&mut original, &mut work);
    original.finish();
}

/// 🧵️ ticket 26/09/02/PUZZLE-3D-END-TO-END: direct functional proof that the previously-dead
/// `Puzzle3dSetActiveExampleWork` state machine actually walks its own stages across MULTIPLE
/// bounded `step()` calls for a real scene_snapshot (not a single-shot reducer), accumulating exactly
/// one mutation per deleted/created item and only publishing them all in the final `Complete`
/// emit — the same turn-by-turn contract `crate::retained_command::RetainedPuzzleCommandJob`
/// relies on when it drives an app-owned `PuzzleCommandWork` (`🎮️commands/🧵️retained/🦀️.rs`,
/// `PuzzleCommandPhase::Work`). The hostile-static-law test above only proves this arm's source
/// text exists; this proves it runs.
#[test]
fn set_active_example_work_advances_through_multiple_bounded_steps_for_nakagin() {
    let mut original = semio_s_plugin_puzzle_retained::testing::OriginalFixture::new(
        semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_s_plugin_puzzle_retained::testing::fixture_stock(), || Some(0), u64::MAX, 1 << 24);

    use semio_s_plugin_puzzle_retained::testing::PuzzleCommandWorkStep;
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample command decodes");
    let mut work = original.construct(Puzzle3dSetActiveExampleWork::original_birth_demands().unwrap(), |scope| Puzzle3dSetActiveExampleWork::from_original_construction(scope));
    let extent = work.extent(&command, &snapshot, &interaction, None).expect("nakagin example fits the fixed bounded work envelope");
    assert!(extent > 4, "nakagin example must require more than the handful of fixed transition steps alone");
    let mut progress_steps = 0usize;
    let emit = loop {
        assert!(progress_steps <= extent + 8, "setActiveExample work did not reach Complete within its own declared extent");
        match semio_s_plugin_puzzle_retained::testing::step(&mut original, &mut work, &command, &snapshot, &config, &interaction, &hover, None).expect("bounded step") {
            PuzzleCommandWorkStep::Pending { .. } => continue,
            PuzzleCommandWorkStep::Progress { .. } => progress_steps += 1,
            PuzzleCommandWorkStep::Complete(emit) => break emit,
            PuzzleCommandWorkStep::Download(_) => panic!("this work must publish a store emission, never a segmented download"),
        }
    };
    assert!(progress_steps > 1, "setActiveExample must require multiple bounded step() calls for the nakagin example, not a single-shot reducer; observed {progress_steps}");
    assert!(emit.artifact_mutations.len() > 1, "the completed emit must carry every incrementally-collected mutation, one per deleted/created item; observed {}", emit.artifact_mutations.len());
    let Some(Puzzle3dConfigMutation::SetActiveExampleId(Puzzle3dConfigSetActiveExampleId { value: published })) = emit.config_mutations.first().cloned() else {
        panic!("example loading must stamp the active example id on the shared config: {:?}", emit.config_mutations);
    };
    assert_eq!(emit.config_mutations.len(), 1, "example loading publishes one config row, not many");
    assert_eq!(published, PUZZLE3D_EXAMPLE_NAKAGIN, "the stamped id must be the example that was loaded");
    semio_s_plugin_puzzle_retained::testing::close(&mut original, &mut work);
    original.finish();
}
