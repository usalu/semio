use super::testkit::*;
use super::*;
/// 🧰️ The framework-injected utility verb: the editor module itself no longer names it, so `use super::*`
/// cannot carry it into the tests. Imported from its owner instead of relying on a re-export.
use semio_framework_plugin::SET_ACTIVE_UTILITY_ACTION_ID;

//#region 🕹️LocalInteractionRead
/// 🚧️ Runaway guard for the host's own continuation drain. Far above the host's real 4096-turn
/// budget so a law that fails here fails on the state machine, never on the guard.
const LOCAL_INTERACTION_READ_TURNS: usize = 65_536;

/// 📤️ The exact frame admission `advance_typed_operation_output` hands
/// `publish_local_interaction_query_reply` every continuation turn.
const LOCAL_INTERACTION_READ_FRAMES: usize = 4;

/// 🕹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: plays the HOST half of one local-interaction
/// read exactly as `plugin_continue_typed_operations` → `advance_typed_operation_output` does —
/// one `advance_typed_operation_publication` unit, then `publish_local_interaction_query_reply`
/// into the same fixed frame admission — and stops draining the moment
/// `has_runnable_typed_operations` reports quiescence. That quiescence point is the ONLY place a
/// page acknowledgement can arrive: the shell's `settlePluginTurn(drainOperations=true)` owns the
/// call until the actor stops reporting more-work, so a query that stays runnable while it waits
/// on the host never sees its own ACK. Returns the exact capture bytes the shell assembles.
async fn read_local_interaction(app: &mut Puzzle3dApp, request_id: u64) -> Vec<u8> {
    read_local_interaction_while_rendering(app, request_id, false).await
}

/// 🖼️ The same host loop with the shell's own render pressure optionally interleaved, because a boot
/// shell renders on every turn while it waits for its surfaces — so a read must terminate while the
/// app is also doing its ordinary per-turn work, not only on an otherwise idle instance.
async fn read_local_interaction_while_rendering(app: &mut Puzzle3dApp, request_id: u64, render: bool) -> Vec<u8> {
    use semio_framework_plugin::PluginApp;
    assert!(app.begin_local_interaction_query(request_id, request_id).is_none(), "a live instance must admit local interaction read {request_id}");
    let (mut capture, mut started, mut closed) = (Vec::new(), false, false);
    let mut awaiting_ack: Option<protocol::LocalInteractionQueryToken> = None;
    let mut turns = 0_usize;
    for _ in 0..LOCAL_INTERACTION_READ_TURNS {
        turns += 1;
        if !app.has_runnable_typed_operations() {
            let Some(token) = awaiting_ack.take() else { break };
            assert!(app.acknowledge_local_interaction_query(&token), "the page the app itself published must accept its own exact token");
            continue;
        }
        if render {
            drop(render_composite(app).await);
        }
        app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("the live-cleanup clock keeps ticking during a read");
        app.advance_typed_operation_publication().await.expect("advance one local interaction publication unit");
        let mut frames = Vec::new();
        app.publish_local_interaction_query_reply(&mut frames, LOCAL_INTERACTION_READ_FRAMES);
        for frame in &frames {
            let protocol::AppFrame::LocalInteractionQuery { reply } = protocol::decode_app_frame(frame).await.expect("a published local interaction frame decodes") else {
                panic!("local interaction publication emitted a foreign app frame");
            };
            match reply {
                protocol::LocalInteractionQueryReply::Started { .. } => started = true,
                protocol::LocalInteractionQueryReply::Page { page } => {
                    assert!(awaiting_ack.is_none(), "a second page was published while the first still awaited its acknowledgement");
                    capture.extend_from_slice(&page.bytes);
                    awaiting_ack = Some(protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal });
                }
                protocol::LocalInteractionQueryReply::Closed { cancelled, .. } => {
                    assert!(!cancelled, "an acknowledged read closes uncancelled");
                    closed = true;
                }
                other => panic!("unexpected local interaction reply: {other:?}"),
            }
        }
        if closed {
            break;
        }
    }
    eprintln!("[DEBUG] local interaction read {request_id} turns={turns} bytes={}", capture.len());
    assert!(started, "local interaction read {request_id} never published its Started reply");
    assert!(closed, "local interaction read {request_id} never terminated: the actor stayed runnable without ever publishing Closed, or quiesced with nothing left for the host to answer");
    capture
}

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave I: reproduces the browser boot stall — the shell's
/// very first `readLocalInteraction` over a document with NO selection, driven through the exact
/// host continuation loop. The actor must reach `Closed` and hand its three captured roots back,
/// and it must never stay runnable while the only thing it waits for is a host acknowledgement the
/// drain cannot deliver.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_of_an_unselected_document_terminates() {
    let mut app = app().await;
    let capture = read_local_interaction(&mut app, 1).await;
    let capture: protocol::LocalInteractionCapture = protocol::json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON")).expect("a terminated read yields a whole capture");
    assert!(capture.state.selection.values().all(|selection| selection.ids.is_empty()), "this law's document carries no selection: {:?}", capture.state.selection);
}

/// 🩺️ The boot shape: the shell renders while it waits for its UI surfaces, so the read runs against
/// a busy instance rather than an idle one. A terminal reply withheld while unrelated app-owned
/// maintenance is in flight — while the query keeps reporting runnable — is the livelock the boot
/// hit, and it cannot be seen on an instance that does nothing else.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_terminates_under_concurrent_render_pressure() {
    let mut app = app().await;
    read_local_interaction_while_rendering(&mut app, 4, true).await;
}

/// 🩺️ The shell re-reads the local interaction on every refresh, so the SECOND and third reads of
/// one live instance must terminate exactly like the first — a store lease registry that still
/// reports a reclaimed slot, or a live slot that outlives its own terminal reply, strands them.
#[semio_framework_async_macros::async_test]
async fn repeated_local_interaction_reads_of_one_instance_terminate() {
    let mut app = app().await;
    for request_id in 1..=3 {
        read_local_interaction(&mut app, request_id).await;
    }
}

/// 🩺️ The same read over a LIVE selection: a non-empty capture spans many more fixed pages, so it
/// exercises every intermediate page's acknowledgement round trip as well as the terminal one.
#[semio_framework_async_macros::async_test]
async fn local_interaction_read_of_a_selected_document_terminates() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let capture = read_local_interaction(&mut app, 2).await;
    let capture: protocol::LocalInteractionCapture = protocol::json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON")).expect("a terminated read yields a whole capture");
    assert!(
        capture.state.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| selection.ids.iter().any(|id| id == &object_id)),
        "the capture carries the live selection: {:?}",
        capture.state.selection
    );
}

/// 🩺️ ticket 26/09/02/PUZZLE-3D-END-TO-END: reproduces `runtime live cleanup faulted for
/// instance 1` — the vortex-picking local interaction query is the only path by which the
/// puzzle3d document store's `snapshot_read()` lease is ever taken and returned, so it is
/// driven to completion here (Started → pages → acknowledge → Closed) exactly as the
/// host does every actor turn, then one plain `maintenance_step` must not fault.
#[semio_framework_async_macros::async_test]
async fn local_interaction_query_return_does_not_fault_the_next_maintenance_step() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    read_local_interaction(&mut app, 1).await;
    app.maintenance_step(1, 4096).expect("maintenance step after a returned snapshot read lease must not fault");
}
//#endregion 🕹️LocalInteractionRead

/// 📏️ Budget the TYPICAL cooperative-maintenance unit of one stage must respect. A quarter of
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000us): this runs at opt-level 0 where
/// every unit is far slower than the release wasm the ceiling actually guards, so a native unit
/// that already eats a quarter of the ceiling is the defect, not the noise. Applied to the
/// per-stage median for the reason `MaintenanceStageBudget` states.
const MAINTENANCE_UNIT_BUDGET_US: u64 = 2_000;

/// 🔁️ Full round-robin sweeps driven after the example lands, so every fixed stage runs its own
/// bounded unit several times (first unit does the real work, later ones prove it stays terminal).
const MAINTENANCE_UNIT_SWEEPS: usize = 8;

/// 🎲️ Independent repetitions of the whole measured scenario, folded per stage by
/// `MaintenanceStageBudget::keep_best_round`: an intrinsically over-budget unit costs the same in
/// every round, while a scheduler hiccup on a loaded machine hits one stage in one round.
const MAINTENANCE_BUDGET_ROUNDS: usize = 3;

/// 🚧️ Runaway guard for the measured host-turn loop; the flagship example loads need thousands of
/// publication turns, so this only has to be far above them, never tight.
const MAINTENANCE_BUDGET_TURNS: usize = 1_048_576;

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave R2: the OS runtime's cooperative-maintenance clock
/// (`RuntimeLiveCleanupJob::step` → `maintenance_step(1, 4096)`) faults its instance with
/// `plugin.internal.interactive-ceiling` the moment one unit overruns
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US`, which is exactly how the browser boot
/// died. This law drives the REAL app — the real typed `setActiveExample` command, the real store,
/// the real store-replacement/envelope registries and the real fixed round robin — over both
/// flagship documents, and holds every stage's typical unit inside [`MAINTENANCE_UNIT_BUDGET_US`],
/// naming the offending stage when it does not. The measured turn deliberately does NOT assert the
/// operation's own result lane: a domain command that faults is a different law's subject, while
/// the clock must stay inside its budget either way.
#[semio_framework_async_macros::async_test]
async fn every_maintenance_unit_stays_inside_the_interactive_step_budget() {
    for example in [PUZZLE3D_EXAMPLE_CONCRETE_FOREST, PUZZLE3D_EXAMPLE_NAKAGIN] {
        let mut best = MaintenanceStageBudget::default();
        for _ in 0..MAINTENANCE_BUDGET_ROUNDS {
            let mut app = app().await;
            let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": example })), None).expect("setActiveExample is a declared puzzle3d command");
            app.dispatch_typed(command, &meta("local")).await.expect("setActiveExample mints its retained whole-document operation");
            for _ in 0..MAINTENANCE_BUDGET_TURNS {
                if !app.has_pending_typed_operations() {
                    break;
                }
                measured_host_turn(&mut app).await;
            }
            for _ in 0..MAINTENANCE_UNIT_SWEEPS * semio_framework_plugin::MAINTENANCE_STAGES as usize {
                app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
            }
            best.keep_best_round(&app.maintenance);
        }
        let (typical_stage, median_us) = best.worst();
        let (peak_stage, peak_us) = best.worst_unit();
        eprintln!("[DEBUG] maintenance budget example={example} typical_stage={typical_stage} median_us={median_us} peak_stage={peak_stage} peak_us={peak_us} breakdown={}", best.report());
        assert!(median_us <= MAINTENANCE_UNIT_BUDGET_US, "{example}: maintenance stage {typical_stage}'s typical unit cost {median_us}us in every round, over the {MAINTENANCE_UNIT_BUDGET_US}us typical-unit budget (per-stage median/worst/units: {})", best.report());
        assert!(u128::from(peak_us) < PUZZLE3D_INTERACTIVE_STEP_CEILING.as_micros(), "{example}: maintenance stage {peak_stage} ran one unit for {peak_us}us in every round, at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?} (per-stage median/worst/units: {})", best.report());
    }
}

/// 🔁️ One host actor turn with its cooperative-maintenance unit measured: the same
/// `maintenance_step` → `advance_typed_operation_publication` → present/ACK → drain shape the
/// runtime drives, minus any assertion on the operation's own outcome lane.
async fn measured_host_turn(app: &mut Puzzle3dApp) {
    app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
    app.advance_typed_operation_publication().await.expect("advance one typed operation publication unit");
    if let Some(page) = app.take_typed_operation_result_page(FIXTURE_INSTANCE_ID) {
        assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge one presented result page"), "the app's own presented result page must accept its exact token");
    }
    drop(app.take_typed_operation_effect());
    drop(app.take_typed_operation_event());
    drop(app.take_typed_operation_completion().await.expect("take one typed operation completion witness"));
    drop(app.take_typed_operation_ui_scope());
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "the app's own local-interaction page must accept its exact token");
        }
    }
}

#[test]
fn retained_publication_contracts_are_an_exact_nonempty_tool_bijection() {
    let fixture: Value = parse(include_str!("../../../🧫️fixtures/🗄️retained-jobs/🔣️.json")).expect("Puzzle3D retained route fixture");
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
        r#"| "suggestionsTick""#,
        "=> Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))",
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
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
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
    let direct_precompute = source.replace("=> Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id))", "=> Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent))");
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
        "PUZZLE3D_MESH_PAGE_SCAN_CHARS",
        "Puzzle3dPrecomputeCommandStage::Publish",
    ] {
        assert!(!suggestion_and_precompute_routes_are_cursorized(&source.replace(marker, "cursor-removed")), "missing retained boundary was falsely accepted: {marker}");
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
        assert!(!selection_transforms_are_cursorized(&source.replace(marker, "cursor-removed")), "missing transform cursor was falsely accepted: {marker}");
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
        assert!(!focus_selection_is_cursorized(&source.replace(marker, "cursor-removed")), "missing focus cursor was falsely accepted: {marker}");
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
        assert!(!patch_inspector_is_cursorized(&source.replace(marker, "cursor-removed")), "missing inspector cursor was falsely accepted: {marker}");
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
        assert!(!world_relocate_is_cursorized(&source.replace(marker, "cursor-removed")), "missing relocate cursor was falsely accepted: {marker}");
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
        assert!(!create_attraction_is_cursorized(&source.replace(marker, "cursor-removed")), "missing attraction cursor was falsely accepted: {marker}");
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
        assert!(!set_active_example_is_cursorized(&source.replace(marker, "cursor-removed")), "missing example cursor was falsely accepted: {marker}");
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
    assert!(emit.config_mutations.is_empty(), "example loading leaves shared app preferences untouched");
}

/// 🧲️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave T: the gumball bracket is an honest `Migrated`
/// `HostOnly` pair. `World3dHost` (`🧰️framework/…/🌐️World3dHost/🟦️.tsx:4718`) dispatches
/// `transformBegin` on drag start, ONE absolute start→end `translateSelection`/`rotateSelection`/
/// `scaleSelection` delta on drag end, then `transformEnd`; mid-drag ticks never leave the host. So
/// both brackets carry no document or config transition of their own and complete empty on
/// `NoopPuzzleCommandWork` — but they must still be `Migrated`, or `validate_ui_dispatch_classification`
/// (`🧰️framework/…/🔌️plugin/🦀️.rs`) rejects every real drag with `interactive-job.not-ui-safe`.
#[test]
fn transform_brackets_are_migrated_host_only_routes_that_complete_empty() {
    let manifest = create_puzzle3d_app();
    let contracts = <Puzzle3dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    let snapshot = Puzzle3dPlayApp::initial_snapshot();
    let config = Puzzle3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    for action in ["transformBegin", "transformEnd"] {
        assert!(PUZZLE3D_RETAINED_TOOL_IDS.contains(&action), "{action} must be a retained tool id or no tool job is ever built for it");
        let declarations = manifest.window_kinds.iter().flat_map(|window| &window.actions).filter(|declared| declared.id == action).collect::<Vec<_>>();
        assert_eq!(declarations.len(), 1, "{action} requires exactly one manifest declaration");
        assert_eq!(declarations[0].semantics.execution.interactive_job, semio_framework_plugin::InteractiveJobClassification::Migrated, "{action} must pass the UI dispatch gate");
        let contract = contracts.iter().find(|contract| contract.tool_id == action).unwrap_or_else(|| panic!("{action} needs a publication contract"));
        assert_eq!(contract.lanes, &[ArtifactToolPublicationLane::HostOnly], "{action} publishes nothing: the drag itself is one absolute delta on another route");
        let command = Puzzle3dCommand::from_action(action, None, None).expect("command decodes");
        let emit = puzzle3d_retained_reduce(&command, &snapshot, &config, &interaction, &hover, None).expect("real dispatch");
        assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty() && emit.effects.is_empty(), "{action} must stay a true no-op; got {}/{}/{}", emit.artifact_mutations.len(), emit.config_mutations.len(), emit.effects.len());
    }
}

/// 🌉️ ticket 26/09/02/PUZZLE-3D-END-TO-END: exercises the wiring this ticket adds — the
/// classification flip to `Migrated`, `PUZZLE3D_RETAINED_TOOL_IDS` registration, the
/// `ArtifactToolPublicationContract` for the Artifact+Config lanes, and
/// `Puzzle3dArtifactStorePreparationFactory` — by dispatching `setActiveExample` through the
/// real `InteractiveJob`/tool-job path (`Puzzle3dRetainedCommandJobFactory` ->
/// `RetainedPuzzleCommandJob` -> `ArtifactToolCompletion` -> the shared publication loop's
/// `self.store.begin_apply_batch(..., self.artifact_one_item_factory.as_ref())`), driving the
/// resulting typed operation to completion via repeated `maintenance_step` turns exactly as a
/// real host does every actor tick, then asserting the document was actually swapped. Uses
/// the registry-backed, instance-bound `app()`: this plugin declares
/// `bounded_first_step_tool_proofs!`, so the bare registry-less `testkit::new_app` faults closed
/// with `interactive-job.catalog-authority` before any dispatch is even attempted.
#[semio_framework_async_macros::async_test]
async fn set_active_example_dispatches_through_the_tool_job_path_and_swaps_the_document() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let before_first_id = first_object_id(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("dispatch setActiveExample through the migrated tool-job path");
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
/// `next.fill_count = *count`, `✏️s/…/🎚️config/🦀️.rs:539`). Uses the registry-backed,
/// instance-bound `app()`: this plugin declares `bounded_first_step_tool_proofs!`, so the bare
/// registry-less `testkit::new_app` faults closed with `interactive-job.catalog-authority` before
/// any dispatch is even attempted. Deliberately reads only the config-side slider measure, not
/// the document — `fillBuildTick` (unmigrated; see its own blocker note in
/// `🔏️publication-authority/🔣️.json`) is what materializes actual fill objects, not this tool.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count() {
    use semio_framework_plugin::PluginApp;
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let ready = drive_fill_until_ready(&mut app, 3.0).await;
    assert!(ready >= 3.0, "the tool-job path needs a planned prefix before setFillCount can publish 3");
    async fn fill_count_slider(app: &mut Puzzle3dApp) -> Option<f64> {
        let view = app.window_view(main::WINDOW_KIND_ID);
        let measures = app.tool_measures(&view).await;
        find_measure_slider(measures.get(fill_tool::TOOL_ID).expect("fill tool measures"), "puzzle3d-fill-count")
    }
    assert_eq!(fill_count_slider(&mut app).await, Some(0.0), "fill count starts at zero before any request");
    dispatch(&mut app, "setFillCount", Some(&json!({ "value": 3 })), None).await.expect("dispatch setFillCount through the migrated tool-job path");
    let mut ticks = 0usize;
    while ticks < 5_000 && fill_count_slider(&mut app).await != Some(3.0) {
        app.maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
        ticks += 1;
    }
    assert_eq!(fill_count_slider(&mut app).await, Some(3.0), "setFillCount did not update the live config's requested fill count through the tool-job path after {ticks} maintenance turns");
}

/// 📏️ Sizes this artifact's three reserved refresh sections against the retained section carrier that
/// now publishes them (`semio_framework_plugin::section_component_tree`): the engagements, window
/// measure and tool measure maps must all admit into the bounded chunk tree and round-trip
/// byte-exactly, with the fill tool's count slider present in the tool payload.
#[semio_framework_async_macros::async_test]
async fn reserved_refresh_section_payloads_admit_into_the_retained_section_carrier() {
    use semio_framework_plugin::PluginApp;
    let mut app = app().await;
    let view = semio_framework_plugin::ViewModel { window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }], ..Default::default() };
    let payloads = [
        (semio_framework_plugin::UiRefreshSection::Engagements, serde_json::to_string(&app.window_engagements(&view).await).expect("engagements serialize")),
        (semio_framework_plugin::UiRefreshSection::Measures, serde_json::to_string(&app.window_measures(&view).await).expect("measures serialize")),
        (semio_framework_plugin::UiRefreshSection::Tools, serde_json::to_string(&app.tool_measures(&view).await).expect("tool measures serialize")),
    ];
    assert!(payloads[2].1.contains("puzzle3d-fill-count"), "the fill tool must contribute its count slider to the tools section");
    assert!(payloads[1].1.contains(main::WINDOW_KIND_ID), "the main window instance must key its own entry in the measures section");
    for (section, payload) in payloads {
        let tree = semio_framework_plugin::section_component_tree(section, &payload).expect("reserved section payload admits into the bounded carrier");
        let projected: serde_json::Value = serde_json::from_str(&semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).expect("carrier projects")).expect("carrier projection is json");
        assert_eq!(projected["key"], section.body_key());
        assert_eq!(semio_framework_plugin::testkit::fixture_carrier_text(&projected), payload, "{} carrier must round-trip byte-exactly", section.key());
        eprintln!("[DEBUG] puzzle3d {} section payload is {} bytes", section.key(), payload.len());
    }
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
        assert!(!add_brush_object_is_cursorized(&source.replace(marker, "cursor-removed")), "missing brush cursor was falsely accepted: {marker}");
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
        assert!(!add_object_kind_is_cursorized(&source.replace(marker, "cursor-removed")), "missing add-object-kind cursor was falsely accepted: {marker}");
    }
}

fn exact_window_routes_capture_instance_owners(source: &str) -> bool {
    source.contains("struct Puzzle3dWindowCommandWork")
        && source.contains("fn bind_window_owners")
        && source.contains("fn take_ephemeral")
        && source.contains("config_from_snapshot(self.window_config.as_ref())")
        && source.contains("transient_from_snapshot(self.window_transient.as_ref())")
        && source.contains("=> Box::new(Puzzle3dWindowCommandWork::new(tool_id))")
}

#[test]
fn exact_window_routes_reject_missing_owner_capture() {
    let source = include_str!("../../🦀️.rs");
    assert!(exact_window_routes_capture_instance_owners(source));
    for marker in ["fn bind_window_owners", "fn take_ephemeral", "config_from_snapshot(self.window_config.as_ref())", "transient_from_snapshot(self.window_transient.as_ref())"] {
        assert!(!exact_window_routes_capture_instance_owners(&source.replace(marker, "owner-capture-removed")));
    }
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

use crate::editor::puzzle3d::config::Puzzle3dCamera;
use protocol::MutationDiff;
use semio_framework_plugin::{EditorApp, PluginApp};

#[test]
fn app_config_serialization_excludes_operation_and_window_state() {
    let config = Puzzle3dConfig::default();
    let spr = to_json_string(&config);
    let oracle: serde_json::Value = serde_json::from_str(&spr).expect("third-party JSON oracle accepts Puzzle 3D config");
    assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(4));
    for forbidden in ["fillCheckpoint", "fillApplyGeneration", "windowOptions", "camera", "suggestionMenu", "engagementInput"] {
        assert!(!spr.contains(forbidden));
    }
}

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn renders_world_scene() {
    let mut app = app().await;
    assert!(render_composite(&mut app).await.to_string().contains("world-3d"));
}

#[semio_framework_async_macros::async_test]
async fn initial_snapshot_is_the_concrete_forest_fixture() {
    let app = app().await;
    assert_eq!(projection_of(&app).get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
    assert!(object_count(&app) > 0, "the concrete-forest default fixture ships with objects");
}

/// 📦️ `Puzzle3dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture.
#[semio_framework_async_macros::async_test]
async fn puzzle3d_play_projection_pack_round_trips() {
    let app = app().await;
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
}

#[semio_framework_async_macros::async_test]
async fn open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change() {
    let mut app = app().await;
    let before = object_count(&app);
    let result = dispatch(&mut app, "openAddObjectDialog", None, None).await.expect("openAddObjectDialog");
    assert!(
        matches!(result.requested_effects.as_slice(), [Effect::OpenDialog { dialog_id, args, .. }] if dialog_id == "addObject" && args.is_none()),
        "expected a single OpenDialog effect for the addObject dialog, got {:?}",
        result.requested_effects,
    );
    assert_eq!(object_count(&app), before, "opening the dialog does not mutate the document");
}

#[semio_framework_async_macros::async_test]
async fn set_active_example_swaps_the_document_and_undo_restores_it() {
    let mut app = app().await;
    let loaded = object_count(&app);
    assert!(loaded > 0);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0, "empty example clears the objects");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_count(&app), 0);
}

#[semio_framework_async_macros::async_test]
async fn nakagin_example_loads_via_operations() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let projection = projection_of(&app);
    assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
    assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
}

#[semio_framework_async_macros::async_test]
async fn document_and_inspector_panels_render() {
    let mut app = app().await;
    for body in [document::BODY_KEY, catalogue::BODY_KEY, inspection::BODY_KEY, settings_panel::BODY_KEY] {
        assert!(!render_body(&mut app, body).await.to_string().is_empty());
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
    use crate::standards::v1::subsets::any::schema::mutations::binary::{close_puzzle3d_store, puzzle3d_store};
    use crate::{Puzzle3dObject as TypedObject, PUZZLE_3D_SCHEMA};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{create_document_envelope, ArtifactCommand};

    let mut store = puzzle3d_store(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", Puzzle3dSnapshot::default(), None)).await.expect("store");
    let object = TypedObject { id: "o1".into(), label: None, object_kind: None, anchor: Default::default(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, mesh_url: None, vortices: Vec::new(), hidden: false, locked: false };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_object(object, None)], description: None }).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle3dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle3dSnapshot, Puzzle3dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    close_puzzle3d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests

//#region 🔖️Inspector
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: with nothing selected the inspector
/// shows the document summary; a live `object`-granularity selection switches it to that object's own
/// field group, carrying its real id/origin plus the `hidden`/`locked` `patchInspector` toggles.
#[semio_framework_async_macros::async_test]
async fn selected_object_inspector_renders_that_object_field_group() {
    let mut app = app().await;
    let empty = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(empty.contains("puzzle3d-play-inspector.empty"), "an empty selection shows the document summary: {empty}");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!json.contains("puzzle3d-play-inspector.empty"), "a live selection must replace the document summary: {json}");
    for expected in ["puzzle3d-play-inspector.object.id", "puzzle3d-play-inspector.object.origin", "puzzle3d-play-inspector.object.hidden", "puzzle3d-play-inspector.object.locked"] {
        assert!(json.contains(expected), "inspector must render {expected}: {json}");
    }
    assert!(json.contains(object_id.as_str()), "the rendered field group carries the selected object's own id: {json}");
}

/// 🕹️ A `vortex`-granularity selection switches the inspector to the vortex field group instead — the
/// per-granularity switch the panel's `selected_section` performs.
#[semio_framework_async_macros::async_test]
async fn selected_vortex_inspector_renders_the_vortex_field_group() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let json = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(json.contains("puzzle3d-play-inspector.vortex.full-id"), "inspector must render the vortex field group: {json}");
    assert!(json.contains("puzzle3d-play-inspector.vortex.radius"), "inspector must render the vortex radius: {json}");
    assert!(!json.contains("puzzle3d-play-inspector.object.id"), "a vortex selection must not render the object group: {json}");
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
    let mut app = app().await;
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
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector");
    let projection = projection_of(&app);
    let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
    let object = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(object_id.as_str())).expect("patched object");
    let origin = object.get("origin").and_then(|value| value.as_array()).expect("origin");
    assert_eq!(origin[0].as_f64(), Some(42.5), "origin.x should be set to the absolute value");
    assert_eq!(origin[1].as_f64(), Some(before_y), "origin.y should be untouched by an origin.x edit");
}

#[semio_framework_async_macros::async_test]
async fn patch_inspector_origin_axis_delta_offsets_each_selected_object_from_its_own_current_value() {
    let mut app = app().await;
    let id_a = first_object_id(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [10.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let id_b = projection_of(&app).get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("added object id").to_string();
    assert_ne!(id_a, id_b, "the added object must be distinct from the first fixture object");
    let x_a_before = object_origin_x(&app, &id_a);
    let x_b_before = object_origin_x(&app, &id_b);
    assert_ne!(x_a_before, x_b_before, "the two objects must start at different x values for this test to prove per-object offset preservation");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "ids": [id_a.clone(), id_b.clone()], "field": "origin.x", "delta": 3.0 })), None).await.expect("patchInspector");
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
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    dispatch(&mut app, "patchInspector", Some(&json!({ "entity": "object", "field": "origin.x", "value": 42.5 })), None).await.expect("patchInspector without ids");
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
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest_for_testkit).await;
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
    // 🗂️ The select's own label is the terminology-aware half; its OPTIONS are the declared examples'
    // catalog rows (`puzzle3d_object_kind_options`), carried as `LocalizedLabel::data` — document data,
    // never authored UI text, so a row reads identically on every axis. The literal `"Object"` option
    // this law used to assert on was the hardcoded placeholder that catalog derivation replaced.
    let arg = dialog.args.iter().find(|entry| entry.id == "objectKind").expect("objectKind arg");
    assert_eq!(arg.label.resolve(terminology, locale), "Art");
    match arg.control() {
        semio_framework_plugin::ActionArgControl::Select { options } => {
            assert!(!options.is_empty() && options.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "the objectKind select offers the declared examples' bounded catalog rows; observed {}", options.len());
            for option in options {
                assert!(!option.value.is_empty(), "every offered object kind names a catalog row");
                assert_eq!(option.label.resolve(terminology, locale), option.label.resolve(semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::En), "catalog row {} is document data and must not be re-authored per axis", option.value);
            }
        }
        _ => panic!("objectKind arg is not a select"),
    }
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
    let mut app = app().await;
    // 🗣️ Panels resolve their label set from the host's own axes and fail closed on an unauthored one
    // (`puzzle3d_labels`), so the German reuse text this law is about only exists once the axes name it.
    app.set_label_axes(semio_framework_plugin::Locale::De, semio_framework_plugin::Terminology::Reuse);
    let document_json = render_body(&mut app, document::BODY_KEY).await.to_string();
    let kinds = render_body(&mut app, catalogue::BODY_KEY).await.to_string();
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_json = to_json_string(&app.window_measures(&view).await);
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
    assert_eq!(definition.modes[0].tools, vec![ToolRef::new(fill_tool::TOOL_ID).await]);
    assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
}

/// 🛠️ Wave W-AB: mid-fill actions must not bounce-disarm the host with an empty `setActiveTool`.
#[semio_framework_async_macros::async_test]
async fn fill_flow_does_not_emit_empty_set_active_tool() {
    let empty_tool = |result: &semio_framework_plugin::InvocationResult| {
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveTool { tool_id } if tool_id.is_empty()))
    };
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("arm fill");
    let tick = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
    assert!(!empty_tool(&tick), "fillBuildTick must not bounce-disarm: {:?}", tick.requested_effects);
    let abort = dispatch(&mut app, "engagementAbort", None, None).await.expect("engagementAbort");
    assert!(!empty_tool(&abort), "engagementAbort must not bounce-disarm fill: {:?}", abort.requested_effects);
}
//#endregion 🔖️Manifest

//#region 🔖️Suggestions
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
    assert!(menu_json.contains("openVortexSuggestions"));
    assert!(menu_json.contains("sparkles"), "menu should include suggest icon: {menu_json}");
    assert!(menu_json.contains("Zoom to selection"), "menu should include zoom: {menu_json}");
    assert!(menu_json.contains("deleteSelection"), "menu should include delete: {menu_json}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
    let mut app = app().await;
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
    assert!(menu_json.contains("menu.group.targets"), "hide/lock rows should be grouped under targets: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "destructive delete must be the last top-level row: {menu_json}");
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id.as_str(), "flag": "hidden", "value": true })), None).await.expect("setTargetVolumeFlag");
    let hidden = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
    assert_eq!(hidden, Some(true));
}

/// 🗂️ Grouped-disclosure contract for the object-selection branch: the top-level menu stays
/// scannable (leaves + groups + separator combined) and the destructive `deleteSelection` row is
/// the last top-level entry (`organize_context_menu` inserts the separator ahead of it).
#[semio_framework_async_macros::async_test]
async fn context_menu_at_selects_object_groups_flags_and_keeps_delete_last() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    assert!(menu.len() <= 9, "top-level menu should stay scannable, got {} rows: {menu:?}", menu.len());
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("menu.group.hand"), "hide/lock rows should be grouped under hand: {menu_json}");
    assert!(menu_json.contains("duplicateSelection"), "menu should be {menu_json}");
    assert_eq!(menu.last().map(|item| item.id.as_str()), Some("delete"), "delete must be the last top-level row: {menu_json}");
    assert_eq!(menu.last().and_then(|item| item.destructive), Some(true), "delete must be marked destructive: {menu_json}");
}

#[semio_framework_async_macros::async_test]
async fn open_vortex_suggestions_opens_the_suggestion_popup() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let result = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 34.0 })), None).await.expect("openVortexSuggestions");
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "opening a one-shot suggestion must not switch the host-owned utility or tool: {:?}",
        result.requested_effects,
    );
    let interaction = interaction_of(&render_composite(&mut app).await);
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
    // 🪟️ The popup is transient state of the pane the user acted in (the dispatch's own window
    // authority), and it RECORDS the explicit target pane the args named — so the assertion renders
    // the acting pane and reads the recorded `windowId` back out of it.
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 8.0, "y": 16.0, "windowId": main::WINDOW_INSTANCE_TOP })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_window(&mut app, main::WINDOW_INSTANCE_PERSPECTIVE).await);
    let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
    assert_eq!(menu.get("windowId").and_then(Value::as_str), Some(main::WINDOW_INSTANCE_TOP));
    assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_with_full_id_places_even_if_selection_was_cleared() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    let before_count = object_count(&app);
    // 🧹️ Simulate the split-pane outside-dismiss race clearing vortex selection before accept.
    dispatch(&mut app, "clearSelection", None, None).await.expect("clearSelection");
    let result = dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex.as_str() })), None).await.expect("acceptSuggestion");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accept must not switch utility/tool: {:?}", result.requested_effects);
    assert!(object_count(&app) > before_count, "accept with fullId must place even after selection clear");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_the_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🖱️ Hovering a row in the suggestion popup must live-update the 3D brush preview (rendered by
/// `world_brush_preview_json`, which reads `runtime.brush_candidate_index`) to the hovered
/// candidate, so the UI can highlight it in 3D before the user clicks — without switching the
/// host-owned active utility into brush mode.
#[semio_framework_async_macros::async_test]
async fn hover_suggestion_updates_the_brush_candidate_index_and_live_preview() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    let composite = render_composite(&mut app).await;
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

    dispatch(&mut app, "hoverSuggestion", Some(&json!({ "index": 1 })), None).await.expect("hoverSuggestion");
    let composite = render_composite(&mut app).await;
    let interaction = interaction_of(&composite);
    assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
    let preview = brush_preview_of(&composite);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the preview must keep targeting the same vortex while only the hovered candidate changes");
    assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "hovered brush preview still carries color: {preview}");
}

#[semio_framework_async_macros::async_test]
async fn accept_suggestion_appends_an_object_and_closes_the_menu() {
    let mut app = app().await;
    let object_count_before = object_count(&app);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    let result = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before + 1);
    assert!(
        result.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })),
        "accepting a one-shot suggestion must leave the host-owned utility/tool unchanged: {:?}",
        result.requested_effects,
    );
    let composite = render_composite(&mut app).await;
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
    // `vortex` domain against `self.registry` — the fixture `app()` is registry-backed for exactly
    // this reason (see its own doc comment).
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let before = interaction_of(&render_composite(&mut app).await);
    assert_eq!(before.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    let object_count_before = object_count(&app);
    dispatch(&mut app, "acceptSuggestion", Some(&json!({ "index": 0, "fullId": "missing-object::missing-vortex.as_str()" })), None).await.expect("acceptSuggestion");
    assert_eq!(object_count(&app), object_count_before, "unknown-vortex accept must not place");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()), "failed accept must still dismiss the suggestion menu");
}

/// 📏️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave K: a world scene names its built-in meshes by
/// REFERENCE and never carries their tessellation, so mesh geometry cannot spend the fixed
/// `UiFixedBytes` surface payload. Before this wave `world3d_mesh_kind_entry` inlined
/// `mesh_from_kind`'s buffers, and `vortex-marker` alone (an 80-triangle ico sphere printed as JSON
/// floats) took ~26 KiB of the 32 KiB payload — so a window that also carried a resolved suggestion
/// popup failed `scene-surface.encode` outright. Measured on the live popup-open scene AND on the
/// Nakagin catalog, whose 180 objects are the widest mesh set this editor can name.
#[semio_framework_async_macros::async_test]
async fn the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");
    let node = render_composite(&mut app).await;
    assert_eq!(interaction_of(&node).pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let live_meshes = scene_meshes_of(&node);
    let (payload, capacity) = world_surface_payload_bytes(&mut app, main::BODY_KEY).await;
    assert!(payload <= capacity, "the popup-open world scene packs to {payload} bytes, over the fixed surface capacity {capacity}");
    for meshes in [live_meshes, parse(&main::world_meshes_json(&nakagin_fixture())).expect("nakagin meshes").as_array().cloned().expect("mesh array")] {
        assert!(!meshes.is_empty(), "a world scene always declares its meshes");
        assert!(to_json_string(&meshes).len() <= PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET, "mesh references must stay a rounding error against the {capacity}-byte surface payload; observed {}", to_json_string(&meshes).len());
        for mesh in &meshes {
            assert!(mesh.get("id").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "every scene mesh is identified: {mesh}");
            assert!(mesh.get("data").is_none(), "a scene mesh must never carry inline geometry: {mesh}");
            assert!(mesh.get("kind").and_then(Value::as_str).is_some() || mesh.get("url").and_then(Value::as_str).is_some(), "every scene mesh resolves by kind or by url: {mesh}");
        }
        assert!(meshes.iter().any(|mesh| mesh.get("kind").and_then(Value::as_str) == Some("vortex-marker")), "the vortex marker is the kind that used to blow the payload and must still be declared");
    }
}

//#region 🚚️PagedSceneCarrier
/// 🚚️ The one language-neutral declaration of the world-3d lane carrier — the SAME file the scene
/// crate's Rust lane table and `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`'s `WORLD3D_SCENE_LANES` are
/// pinned against, read here so the real Nakagin publication is measured against the same bounds the
/// TypeScript assembler enforces.
const PUZZLE3D_WORLD3D_LANE_CONTRACT: &str = include_str!("../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json");

/// 🗼️ Loads the Nakagin Capsule Tower example through the REAL typed `setActiveExample` command and
/// drives it to quiescence, exactly as the navbar picker does.
async fn nakagin_app() -> Puzzle3dApp {
    let mut app = app().await;
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).expect("setActiveExample is a declared puzzle3d command");
    app.dispatch_typed(command, &meta("local")).await.expect("setActiveExample mints its retained whole-document operation");
    settle(&mut app).await;
    app
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (a). The Nakagin Capsule Tower with its
/// vortex suggestion popup open is the document that made `scene-surface.encode` refuse the whole
/// refresh (`surface payload exceeds fixed capacity with 57281 bytes`, browser rebuild #25). It now
/// publishes as a small spine inside the fixed-capacity doc plus one individually paged carrier per
/// payload lane, and every one of those carriers must obey the SAME bounds the TypeScript assembler
/// enforces on the other side of the wire — leaf bytes, children per node, and a byte/hash manifest
/// that lets a partially arrived lane be told from a settled one.
#[semio_framework_async_macros::async_test]
async fn the_nakagin_world_scene_publishes_every_lane_under_the_page_cap_with_the_popup_open() {
    let contract: Value = parse(PUZZLE3D_WORLD3D_LANE_CONTRACT).expect("world-3d lane contract is json");
    let leaf_bytes = contract.pointer("/carrier/leafBytes").and_then(Value::as_u64).expect("leafBytes") as usize;
    let children_max = contract.pointer("/carrier/childrenMax").and_then(Value::as_u64).expect("childrenMax") as usize;
    let doc_bytes_max = contract.pointer("/carrier/docBytesMax").and_then(Value::as_u64).expect("docBytesMax") as usize;

    let mut app = nakagin_app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": main::WINDOW_INSTANCE_TOP })), None).await.expect("openVortexSuggestions");

    let census = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    assert_eq!(census.capacity, doc_bytes_max, "the doc ceiling this law measures against is the contract's own");
    assert!(census.doc_bytes <= census.capacity, "the Nakagin spine packs to {} bytes, over the fixed surface capacity {}", census.doc_bytes, census.capacity);
    assert!(!census.lanes.is_empty(), "a published world scene always carries its payload lanes");

    let declared: Vec<&str> = contract["lanes"].as_array().expect("lanes").iter().map(|lane| lane["bodyKey"].as_str().expect("bodyKey")).collect();
    for lane in &census.lanes {
        assert!(declared.contains(&lane.key.as_str()), "lane carrier {} is not a declared world-3d lane", lane.key);
        assert!(lane.widest_leaf <= leaf_bytes, "lane {} has a {}-byte text leaf, over the {leaf_bytes}-byte cap", lane.key, lane.widest_leaf);
        assert!(lane.widest_children <= children_max, "lane {} has a node with {} children, over the {children_max} cap", lane.key, lane.widest_children);
        assert_eq!(lane.bytes, lane.declared_bytes as usize, "lane {} carrier text disagrees with the byte count its spine declared", lane.key);
        assert_eq!(lane.declared_hash.len(), 16, "lane {} must declare a 16-hex-digit content hash", lane.key);
        assert!(lane.leaves >= 1, "lane {} publishes at least one leaf", lane.key);
        assert_eq!(lane.leaves > children_max, lane.leaf_depth > 1, "lane {} with {} leaves must page into a nested carrier exactly when it outgrows one level of {children_max} children (observed leaf depth {})", lane.key, lane.leaves, lane.leaf_depth);
    }

    let interaction = parse(census.assembled.interaction_json.as_deref().expect("the interaction lane reassembles")).expect("interaction lane is json");
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "the law only measures the popup-open scene");
    let instances = census.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).expect("the instances lane always publishes");
    assert!(parse(&census.assembled.instances_json).expect("instances lane is json").as_array().is_some_and(|array| array.len() > 100), "Nakagin publishes its whole catalog of objects through the instances lane");
    eprintln!(
        "[DEBUG] nakagin popup-open world scene: spine={}B of {}B, {} lanes carrying {}B total ({}), widest lane={}B in {} leaves",
        census.doc_bytes,
        census.capacity,
        census.lanes.len(),
        census.payload_bytes(),
        census.report(),
        instances.bytes,
        instances.leaves
    );
    eprintln!("[DEBUG] nakagin lane paging: {}", census.lanes.iter().map(|lane| format!("{}={}leaves@depth{}", lane.key.trim_start_matches(semio_framework_plugin::WORLD3D_SCENE_LANE_KEY_PREFIX), lane.leaves, lane.leaf_depth)).collect::<Vec<_>>().join(" "));
    assert!(census.payload_bytes() > census.capacity, "this law is only meaningful while the Nakagin payload is past what one fixed doc could ever hold");
}

/// 🚚️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave P — LAW (b). A partial refresh that changes only the
/// camera must leave every lane carrier byte-identical, so the reconciler emits no op for any of
/// them; `instances`, `interaction` and `lod` have very different change rates and the whole point of
/// splitting them is that an unchanged lane costs nothing.
#[semio_framework_async_macros::async_test]
async fn a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh() {
    let mut app = nakagin_app().await;
    let before = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [80.0, 80.0, 80.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), Some(main::WINDOW_KIND_ID)).await.expect("setCamera");
    let moved = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let republished: Vec<&str> = moved.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(republished.is_empty(), "a camera move republished {republished:?}");
    assert_ne!(moved.assembled.camera_json, before.assembled.camera_json, "the camera itself must have moved for this law to mean anything");

    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("interactionSelect");
    let picked = world_surface_carrier_census(&mut app, main::BODY_KEY).await;
    let changed: Vec<&str> = picked.lanes.iter().filter(|lane| before.lane(&lane.key).is_none_or(|previous| previous.declared_hash != lane.declared_hash)).map(|lane| lane.key.as_str()).collect();
    assert!(!changed.contains(&semio_framework_plugin::World3dSceneLane::Instances.body_key()), "a selection change must not republish the instances lane");
    assert_eq!(picked.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()), before.lane(semio_framework_plugin::World3dSceneLane::Instances.body_key()).map(|lane| lane.declared_hash.as_str()));
    eprintln!("[DEBUG] nakagin partial refresh: camera move republished 0 of {} lanes; a selection republished {changed:?}", before.lanes.len());
}
//#endregion 🚚️PagedSceneCarrier

/// 📏️ A whole scene's mesh declarations, as references, against the 32 KiB fixed surface payload:
/// generous enough for the widest catalog the editor can name, tight enough that a single inlined
/// primitive (26 KiB) fails it.
const PUZZLE3D_SCENE_MESH_REFERENCE_BUDGET: usize = 4 * 1024;

#[semio_framework_async_macros::async_test]
async fn close_vortex_suggestions_clears_sticky_hover() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("interactionHover");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), None).await.expect("openVortexSuggestions");
    dispatch(&mut app, "closeVortexSuggestions", None, None).await.expect("closeVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
}

/// 🧰️ Context-menu / Alt+right-click suggestions are a one-shot placement: opening and accepting
/// must leave whatever host-owned utility was already active (e.g. transform) untouched.
#[semio_framework_async_macros::async_test]
async fn open_and_accept_vortex_suggestions_preserve_active_utility() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("activate transform");
    let vortex = first_vortex_full_id(&app);
    let open = dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 0.0, "y": 0.0 })), Some(main::WINDOW_KIND_ID)).await.expect("openVortexSuggestions");
    assert!(open.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "opening suggestions must not emit utility/tool switches: {:?}", open.requested_effects);
    let open_node = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let open_interaction = interaction_of(&open_node);
    assert_eq!(open_interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "transform remains non-brush scene mode during suggestions");
    assert_eq!(open_interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
    assert!(brush_preview_of(&open_node).get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "one-shot suggestions still emit a placement preview without entering brush mode");
    let accept = dispatch(&mut app, "acceptSuggestion", None, Some(main::WINDOW_KIND_ID)).await.expect("acceptSuggestion");
    assert!(accept.requested_effects.iter().all(|effect| !matches!(effect, Effect::SetActiveUtility { .. } | Effect::SetActiveTool { .. })), "accepting suggestions must not emit utility/tool switches: {:?}", accept.requested_effects);
    let accept_interaction = interaction_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(accept_interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    assert_eq!(accept_interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
}

/// 🖱️ Wave W-AB: a vortex pointermove storm admits 70 `interactionHover`s (then a click
/// `interactionSelect`) before Isolated reserved jobs finish — the #38 spawn-admit drop. Latest-wins
/// keeps one pending hover so the last target commits, brush preview publishes, and place lands.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_storm_admits_then_brush_preview_and_place() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let before = object_count(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm admitted at least one hover")).await.expect("latest hover must commit");
    let select_admit = select_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("vortex click select must admit after the hover storm");
    settle_reserved(&mut app, select_admit).await.expect("select commit");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let node = render_composite(&mut app).await;
    let preview = brush_preview_of(&node);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "hover storm must still publish a brush preview: {preview}");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "brush preview must name a kind: {preview}");
    dispatch(&mut app, "addBrushObject", Some(&preview), None).await.expect("addBrushObject from published preview");
    assert!(object_count(&app) > before, "vortex click place must land an object from the published preview");
}

/// 🖱️ Wave W-AB: after a committed vortex hover, Alt+right-click (`openVortexSuggestions` with the
/// hovered `fullId`) publishes `suggestionMenu.open` — the guest half of the host gesture.
#[semio_framework_async_macros::async_test]
async fn vortex_hover_then_open_vortex_suggestions_publishes_menu() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let mut last = None;
    for _ in 0..70 {
        last = Some(hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("suggestion hover storm must keep admitting"));
    }
    settle_reserved(&mut app, last.expect("storm")).await.expect("latest hover commits for suggestions");
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex.as_str(), "x": 12.0, "y": 24.0 })), None).await.expect("openVortexSuggestions");
    let interaction = interaction_of(&render_composite(&mut app).await);
    assert_eq!(interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true), "vortex hover + openVortexSuggestions must publish the menu: {interaction}");
    assert_eq!(interaction.pointer("/suggestionMenu/vortexFullId").or_else(|| interaction.pointer("/suggestionMenu/vortex_full_id")).and_then(Value::as_str), Some(vortex.as_str()));
}

/// Hover-committed leftover in brush mode publishes `brushPreviewJson` without a click-select.
/// Preview is a world scene lane (not an Effect); leftover InteractionView cannot carry it —
/// `suggestionsTick` must finish the hovered target so the dirty world body encodes the lane.
#[semio_framework_async_macros::async_test]
async fn hover_committed_in_brush_publishes_preview() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let preview = brush_preview_of(&render_composite(&mut app).await);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "hover-committed brush must publish preview for the hovered vortex: {preview}");
    assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "hover-committed preview must name a kind: {preview}");
}

/// Click place after hover-committed preview dispatches real `addBrushObject` with that vortex id.
#[semio_framework_async_macros::async_test]
async fn hover_committed_click_places_via_published_preview() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let vortex = first_vortex_full_id(&app);
    let before = object_count(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover admit");
    settle_reserved(&mut app, admitted).await.expect("hover leftover commits");
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let preview = brush_preview_of(&render_composite(&mut app).await);
    assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "place must use the hovered vortex preview: {preview}");
    dispatch(&mut app, "addBrushObject", Some(&preview), None).await.expect("addBrushObject from published hover preview");
    assert!(object_count(&app) > before, "hover-committed click must land an object via addBrushObject");
}


/// leftover InteractionView after vortex-domain interactionHover must carry the vortex full id.
#[semio_framework_async_macros::async_test]
async fn interaction_hover_leftover_carries_vortex_full_id() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    let admitted = hover_id_unsettled(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("vortex interactionHover admit");
    let settled = settle_reserved(&mut app, admitted).await.expect("vortex interactionHover leftover");
    let view = settled.output.get("interactionView").expect("leftover InteractionView");
    let hover = view.get("hoverTarget").expect("hoverTarget on leftover");
    assert_eq!(hover.get("id").and_then(dsl::DslValue::as_str), Some(vortex.as_str()), "leftover hoverTarget must be the vortex full id, got {hover:?}");
    assert_eq!(hover.get("domain").and_then(dsl::DslValue::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "next scene render must project leftover hover onto hoveredVortexFullId");
}


//#endregion 🔖️Suggestions

//#region 🔖️WindowOptions
#[semio_framework_async_macros::async_test]
async fn grid_window_options_control_one_visible_grid_spacing() {
    let mut app = app().await;
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), None).await.expect("setGridVisible");
    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 7.5 })), None).await.expect("setGridSpacing");
    let lod = lod_of(&render_composite(&mut app).await);
    assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid")), Some(None));
    assert_eq!(find_measure_slider(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing")), Some(7.5));
}

/// 🪟️ Two window instances of the same kind (a split top/perspective pane pair) must never share
/// window options — toggling grid visibility in one instance must leave every other instance's
/// grid untouched, both in its measures chrome and in its own rendered scene.
#[semio_framework_async_macros::async_test]
async fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut reopened = app().await;
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");
    let base_view = app.window_view(main::WINDOW_KIND_ID);
    let second_view = app.window_view(second_window);
    let document_before = projection_of(&app);
    let app_config_before = app.config_pack().await.expect("app config before exact window publications");

    dispatch(&mut app, "setGridSpacing", Some(&json!({ "value": 2.5 })), Some(main::WINDOW_KIND_ID)).await.expect("setGridSpacing on base window");
    dispatch(&mut app, "setGridVisible", Some(&json!({ "pressed": false })), Some(second_window)).await.expect("setGridVisible on second window");

    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_after = app.window_measures(&view).await;
    assert_eq!(find_measure_toggle(measures_after.get(main::WINDOW_KIND_ID).expect("base measures"), &toggle_id), Some(true), "the base window instance's grid must stay visible");
    assert_eq!(find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id), Some(false), "only the targeted window instance's grid toggles off");
    let base_render = render_window(&mut app, main::WINDOW_KIND_ID).await;
    let second_render = render_window(&mut app, second_window).await;
    assert_eq!(lod_of(&base_render).get("showLodGrid").and_then(Value::as_bool), Some(true));
    assert_eq!(lod_of(&base_render).get("gridFactor").and_then(Value::as_f64), Some(2.5));
    assert_eq!(lod_of(&second_render).get("showLodGrid").and_then(Value::as_bool), Some(false));
    assert_eq!(projection_of(&app), document_before);
    let app_config_after = app.config_pack().await.expect("app config after exact window publications");
    assert_eq!((app_config_after.pack, app_config_after.spr), (app_config_before.pack, app_config_before.spr));
    assert_eq!(app.window_config_generation(&base_view).await.expect("base generation"), Some(1));
    assert_eq!(app.window_config_generation(&second_view).await.expect("second generation"), Some(1));
    let packs = app.window_config_packs().await.expect("exact window packs");
    assert_eq!(packs.len(), 2);
    for pack in packs {
        reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
    }
    assert_eq!(render_window(&mut reopened, main::WINDOW_KIND_ID).await, base_render);
    assert_eq!(render_window(&mut reopened, second_window).await, second_render);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
    eprintln!("[DEBUG] two Puzzle 3D windows isolated and rendered persisted options, preserved document and app config, reloaded exact packs, and reached terminal-empty close");
}

#[semio_framework_async_macros::async_test]
async fn window_transient_is_exact_instance_local_and_resets_on_reload() {
    let mut reopened = app().await;
    let mut app = app().await;
    let window_a = "puzzle3d-transient-a";
    let window_b = "puzzle3d-transient-b";
    let view_a = app.window_view(window_a);
    let view_b = app.window_view(window_b);
    let vortex = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 10.0, "y": 20.0 })), Some(window_a)).await.expect("open suggestions in window a");
    let transient_a = app.window_transient_snapshot(&view_a).expect("window a transient").expect("window a owner");
    let transient_b = app.window_transient_snapshot(&view_b).expect("window b transient").expect("window b owner");
    assert_eq!(transient_a.get::<window_ownership::Puzzle3dWindowTransientOwner>().and_then(|value| value.suggestion_menu.as_ref()).map(|menu| menu.window_id.as_str()), Some(window_a));
    assert!(transient_b.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    dispatch(&mut app, "closeVortexSuggestions", None, Some(window_a)).await.expect("close suggestions in window a");
    let closed = app.window_transient_snapshot(&view_a).expect("closed transient").expect("closed owner");
    assert!(closed.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    let vortex_again = first_vortex_full_id(&app);
    dispatch(&mut app, "openVortexSuggestions", Some(&json!({ "fullId": vortex_again })), Some(window_a)).await.expect("open suggestions in window a again");
    let reset = reopened.window_transient_snapshot(&view_a).expect("reopened transient").expect("reopened owner");
    assert!(reset.get::<window_ownership::Puzzle3dWindowTransientOwner>().is_some_and(|value| value.suggestion_menu.is_none()));
    assert_eq!(app.window_transient_generation(&view_a).expect("window a transient generation"), Some(3));
    assert_eq!(app.window_transient_generation(&view_b).expect("window b transient generation"), Some(0));
    // 🧷️ Every `window_transient_snapshot` above is a live READ of the partition store, and close is
    // blocked — by design — while one is outstanding. The reads have to be surrendered before the
    // close witness, exactly as a host surrenders a rendered body before retiring its app.
    drop((transient_a, transient_b, closed));
    drop(reset);
    assert!(close_witness(reopened).expect("reopened app close"));
    assert!(close_witness(app).expect("source app close"));
    eprintln!("[DEBUG] Puzzle 3D transient suggestions stayed exact-window isolated, close cleared their owner, reload reset ephemeral state, and both registered apps reached terminal-empty close");
}

/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave S. A mounted worker job session is admitted out of a
/// FIXED process-wide array of `semio_framework_job::WORKER_JOB_SESSION_SLOTS` retirement slots, and an
/// app dropped while an operation is still mounted parks its node there instead of freeing it. ONLY a
/// live app's cooperative-maintenance round robin hands that slot back. Without that pump the array
/// filled up for the life of the process: from then on EVERY typed operation had its worker session
/// refused, and a refused session never runs the job, so the operation reached publication with no
/// completion value and the host's continuation loop spun on it forever — which is exactly how one
/// abandoned command turned a whole test binary into `pending typed operations outlived the settle
/// budget` for every later command, including plain view actions.
#[semio_framework_async_macros::async_test]
async fn an_abandoned_app_hands_its_worker_session_admission_back_to_the_next_app() {
    let mut abandoned = app().await;
    dispatch_unsettled(&mut abandoned, "setCamera", Some(&json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("setCamera mounts one typed operation");
    drop(abandoned);
    let mut live = app().await;
    for _ in 0..RETIREMENT_RECLAIM_TURNS {
        if !semio_framework_job::worker_job_retirements_are_parked() {
            break;
        }
        live.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("one cooperative maintenance unit");
    }
    assert!(
        !semio_framework_job::worker_job_retirements_are_parked(),
        "a live app's maintenance round robin must reclaim every parked worker-job retirement slot, or the process runs out of session admissions"
    );
    dispatch(&mut live, "setCamera", Some(&json!({ "position": [4.0, 5.0, 6.0], "target": [0.0, 0.0, 0.0] })), None).await.expect("the next app still quiesces on a plain view action");
}

/// 🔁️ Maintenance units the reclaim law grants: every stage of the round robin, many times over, so the
/// law fails on a missing pump rather than on a tight budget.
const RETIREMENT_RECLAIM_TURNS: usize = semio_framework_plugin::MAINTENANCE_STAGES as usize * 64;

/// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
/// they are View-kind and must never emit VCS operations, no matter what they mutate.
#[semio_framework_async_macros::async_test]
async fn camera_actions_are_view_actions_that_emit_no_artifact_mutations() {
    let app_definition = create_puzzle3d_app();
    for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
        let def = app_definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
        assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
    }
    let mut live = app().await;
    let before = projection_of(&live);
    let result = dispatch(&mut live, "setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), None).await.expect("setCamera");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&live), before, "setCamera must not mutate the document");
}

/// 🪟️ ONE per-window setting is ONE window-config publication that QUIESCES. The lane's own bounded
/// preparation used to demand the owner's whole declared publication ceiling
/// (`Puzzle3dWindowConfigOwner::MAXIMUM_PUBLICATION_BYTES`, 65 536) out of the publisher's per-turn
/// byte grant (`TYPED_OPERATION_RESULT_PAGE_BYTES`, 4 096) and answered `Blocked` on every single
/// turn, so `setCamera` — and every projection/grid/vortex/window option behind the same lane — sat
/// in stage `Publishing` forever and nothing queued behind it could ever run. The law is exactly:
/// one mutation, one generation, no pending operation left over.
#[semio_framework_async_macros::async_test]
async fn one_window_config_mutation_publishes_exactly_one_generation_and_quiesces() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let before = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [7.0, 8.0, 9.0], "target": [0.0, 0.0, 0.0], "zoom": 1.5 } })), None).await.expect("setCamera");
    assert!(!app.has_pending_typed_operations(), "one window-config gesture must leave no typed operation pending");
    let after = app.window_config_generation(&view).await.expect("window config generation").expect("puzzle3d registers one window config owner");
    assert_eq!(after, before + 1, "one window-config mutation publishes exactly one store generation, never zero and never a spin");
    assert_eq!(camera_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).pointer("/position/0").and_then(Value::as_f64), Some(7.0), "the published window config is what the window renders");
}

/// 🪟️📷️ Orbiting one window instance's camera must never move any sibling instance's camera, and
/// must never touch the shared document.
#[semio_framework_async_macros::async_test]
async fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
    let mut app = app().await;
    let window_a = "puzzle3d-main-a";
    let window_b = "puzzle3d-main-b";
    dispatch(&mut app, "worldPointerDown", None, Some(window_a)).await.expect("register a");
    dispatch(&mut app, "worldPointerDown", None, Some(window_b)).await.expect("register b");

    let before_document = projection_of(&app);
    let camera_b_before = camera_of(&render_window(&mut app, window_b).await);

    let result = dispatch(&mut app, "setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), Some(window_a)).await.expect("setCamera on window A");
    assert!(result.mutations.is_empty(), "setCamera must not emit document operations");
    assert_eq!(projection_of(&app), before_document, "setCamera must never mutate the shared document");

    let camera_a_after = camera_of(&render_window(&mut app, window_a).await);
    assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");
    assert_eq!(camera_of(&render_window(&mut app, window_b).await), camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
}

#[semio_framework_async_macros::async_test]
async fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode must hide vortices while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_always = app.window_measures(&view).await;
    let window_measures_always = measures_always.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_always, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show")).as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
    assert_eq!(vortices_of(&render_composite(&mut app).await).len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), None).await.expect("setVortexShow selected");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "switching back to Selected must hide idle vortices");
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
    let mut app = app().await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let window_measures = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow always");
    let outwards_vortices = vortices_of(&render_composite(&mut app).await);
    assert!(!outwards_vortices.is_empty(), "fixture must expose vortices");
    assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), None).await.expect("setVortexDirection inwards");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures_inwards = app.window_measures(&view).await;
    let window_measures_inwards = measures_inwards.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(window_measures_inwards, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction")).as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
    assert!(vortices_of(&render_composite(&mut app).await).iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
}

#[semio_framework_async_macros::async_test]
async fn vortex_direction_option_is_local_to_the_window_instance() {
    let mut app = app().await;
    let second_window = "puzzle3d-main-2";
    dispatch(&mut app, "worldPointerDown", None, Some(main::WINDOW_KIND_ID)).await.expect("register base window");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(main::WINDOW_KIND_ID)).await.expect("setVortexShow always on base");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), Some(second_window)).await.expect("setVortexShow always on second");
    dispatch(&mut app, "setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), Some(second_window)).await.expect("setVortexDirection inwards on second window");

    let base_vortices = vortices_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert!(!base_vortices.is_empty(), "the base window must still emit vortices");
    assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

    let second_vortices = vortices_of(&render_window(&mut app, second_window).await);
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
    // 🔒️ `fill_progress_summary` falls back to the PROCESS-WIDE envelope registry when this app has no
    // plan of its own, so without the shared guard this law reads whatever a concurrent fill law left
    // admitted and calls it this app's progression. W-F4 §7.6 named exactly this leak.
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": Value::Null })), None).await.expect("deactivate fill");
    let before = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    for _ in 0..64 {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "an inactive fill tick must never force a full app refresh");
        assert!(result.history_patch.is_none(), "a View-kind verb never dirties the history panel — `finish_recorded` skips the patch by declared kind");
    }
    let after = with_puzzle3d_app(|inner| inner.precompute.borrow().fill_progress_summary());
    assert_eq!(after, before, "stale or queued fill ticks must not advance planning after the Fill tool is deactivated");
}

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    // ⏳️ The document reaches the precompute session through the BOUNDED sync prologue and the
    // admission census spends a bounded unit budget per turn, so the tick that finally hands the
    // envelope to its job is not necessarily the first one — what the law states is that exactly ONE
    // tick out of the run spawns, and that it spawns an isolated `FILL_JOB_KIND` job.
    let mut spawns = 0_usize;
    for _ in 0..FILL_TICK_ADMISSION_TICKS {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        for effect in &result.requested_effects {
            assert!(
                matches!(effect, Effect::SpawnJob { kind, placement: semio_framework_plugin::kernel::JobPlacement::Isolated, .. } if kind == crate::editor::puzzle3d::precompute::FILL_JOB_KIND),
                "the only effect a fill tick may request is one isolated fill job"
            );
            spawns += 1;
        }
    }
    // 🔬️ Read the plan through the SAME path the slider does. A `with_puzzle3d_app` probe builds a
    // fresh, session-less app, so its `fill_progress_summary` answers off the process-wide envelope
    // registry rather than off this app's cursor — it states nothing about this dispatch at all.
    assert_eq!(fill_ready(&mut app).await, 0.0, "the view action must not execute a solver transition inline: nothing is planned until the isolated job is stepped");
    assert_eq!(spawns, 1, "a live fill request is enqueued exactly once, never once per tick");
}

/// ⏱️ Ticks the admission law grants the bounded sync prologue plus the admission census before it
/// expects the one and only `SpawnJob`.
const FILL_TICK_ADMISSION_TICKS: usize = 16;

/// 🔁️ How many host `fillBuildTick` cycles the growth law drives. The production loop runs one tick
/// per completed turn — 140–250 ms — and the guest died after 173 of them, so a law that means to
/// state the retention bound has to outlast that.
const FILL_TICK_GROWTH_CYCLES: usize = 320;

/// 🧾️ How many fill envelopes the whole run may admit. A fill plan is admitted ONCE and then driven
/// by its own bounded job; a completed plan may legitimately be superseded and re-admitted a handful
/// of times as the document changes underneath it. What this number rules out is the failure this
/// law exists for: one fresh admission per tick, each carrying a whole `FillBuilder` (its cloned
/// mesh roots included) and a mounted worker session into the process-wide registry.
const FILL_TICK_ADMISSION_CEILING: u64 = 8;

/// 🧮️ Process-wide retained-heap growth the 320-tick run may add after Fill activation. One live
/// `FillBuilder` plus its bounded job is a few megabytes; a fresh preparation every tick is
/// ~2.8 MiB * 320 = ~896 MiB. Sixty-four mebibytes is enough for one plan and its job slices,
/// and far below the per-tick guest leak this law exists to catch.
const FILL_TICK_HEAP_GROWTH_CEILING: isize = 64 * 1024 * 1024;

/// 🪣️ Activating Fill and letting the host tick must converge on ONE admitted plan that the bounded
/// job actually advances — not on an envelope per tick.
///
/// 🧮️ Retention is stated two ways: registry OCCUPANCY plus admission count (exact under a parallel
/// suite) AND the process-wide [`retained_heap_bytes`] delta from [`Puzzle3dHeapWitness`]. An
/// admitted envelope holds the megabyte-scale owners (`FillBuilder` + `MountedFillWorker`). The
/// guest heap is a fixed 512 MiB wasm linear memory (`.cargo/config.toml`'s `--max-memory=536870912`),
/// so one more envelope per tick is a hard `memory allocation of 16384 bytes failed` trap in
/// production. The witness makes that leak visible natively.
///
/// 🚚️ The jobs are driven through the REAL `reactor::jobs` runtime — `start_job` on the effect the
/// tick requested, then bounded `step_job` slices — never through
/// `drive_enqueued_fill_job_for_test`. That bypass reaches into the registry directly and therefore
/// reports a healthy plan even when no job is ever spawned at all, which is exactly the state the
/// shipped guest was in.
#[semio_framework_async_macros::async_test]
async fn fill_build_tick_converges_on_one_admitted_plan_the_bounded_job_advances() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    let baseline_heap = retained_heap_bytes();
    let (_, baseline_admissions) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    let mut live: Vec<u64> = Vec::new();
    let mut spawned = 0_usize;
    let mut completed = 0_usize;
    let mut faults: Vec<String> = Vec::new();
    let mut peak_ready = 0.0_f64;
    for _ in 0..FILL_TICK_GROWTH_CYCLES {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        for effect in &result.requested_effects {
            let Effect::SpawnJob { job, kind, input, .. } = effect else { continue };
            if kind != crate::editor::puzzle3d::precompute::FILL_JOB_KIND {
                continue;
            }
            spawned += 1;
            semio_framework_plugin::reactor::jobs::start_job(*job, kind, input).await;
            live.push(*job);
        }
        let mut still = Vec::new();
        for job in live.drain(..) {
            let mut terminal = None;
            for _ in 0..64 {
                match semio_framework_plugin::reactor::jobs::step_job(job, FILL_TICK_JOB_BUDGET).await {
                    semio_framework_plugin::reactor::jobs::JobStep::Running(_) => {}
                    semio_framework_plugin::reactor::jobs::JobStep::Done(_) => {
                        terminal = Some(true);
                        break;
                    }
                    semio_framework_plugin::reactor::jobs::JobStep::Failed(bytes) => {
                        faults.push(String::from_utf8_lossy(&bytes).to_string());
                        terminal = Some(false);
                        break;
                    }
                }
            }
            match terminal {
                Some(true) => completed += 1,
                Some(false) => {}
                None => still.push(job),
            }
        }
        live = still;
        let tick_ready = fill_ready(&mut app).await;
        if tick_ready > peak_ready {
            peak_ready = tick_ready;
        }
    }
    let (occupied, admissions) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    let registry_available = crate::editor::puzzle3d::precompute::fill_envelope_available_count();
    let admitted = admissions.saturating_sub(baseline_admissions);
    let census = format!(
        "{FILL_TICK_GROWTH_CYCLES} ticks admitted {admitted} envelopes ({occupied} still live), spawned {spawned} jobs, completed {completed}, faulted {} ({}), moved {} heap bytes",
        faults.len(),
        faults.first().map_or("none", String::as_str),
        retained_heap_bytes() - baseline_heap
    );
    assert!(spawned > 0, "the fill tick loop never requested a single background plan job: {census}");
    assert!(
        admitted <= FILL_TICK_ADMISSION_CEILING,
        "every tick admitted its own fill envelope instead of advancing the one already admitted — this is the retained megabyte per tick that traps the 512 MiB guest heap: {census}"
    );
    assert!(occupied <= crate::editor::puzzle3d::precompute::FILL_ENVELOPE_MAX_OPERATIONS, "the fill registry may never hold more envelopes than it has slots: {census}");
    let heap_delta = retained_heap_bytes() - baseline_heap;
    assert!(
        heap_delta < FILL_TICK_HEAP_GROWTH_CEILING,
        "retained heap grew {heap_delta} bytes across {FILL_TICK_GROWTH_CYCLES} ticks (ceiling {FILL_TICK_HEAP_GROWTH_CEILING}) — a fresh FillBuilder every tick: {census}"
    );
    let ready = fill_ready(&mut app).await;
    let census = format!("{census}; peak_ready={peak_ready} final_ready={ready} registry_available={registry_available}");
    assert!(
        peak_ready > 0.0 || ready > 0.0 || registry_available > 0,
        "the fill-count slider never left `ready: 0` — background planning produced nothing a user could commit: {census}"
    );
    assert!(ready > 0.0, "the fill-count slider ended at `ready: 0` after planning: {census}");
    // 🧹️ Four process-wide envelope slots: a 320-tick law that walks away from its live plan starves
    // every later fill law of an admission.
    drop(app);
    crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
}

/// ⛽️ One bounded slice per tick, exactly as the host's isolated worker grants it: the point of the
/// law is that the plan advances under the SAME per-turn budget production gives it, not that a test
/// can grind the solver to completion inside one tick.
const FILL_TICK_JOB_BUDGET: semio_framework_plugin::reactor::jobs::JobBudget = semio_framework_plugin::reactor::jobs::JobBudget { fuel: 1, deadline_ms: 1 };

//#region 🪣️FillJobLifetime
/// 🧵️ Slices the long-plan owner spends before it terminalizes on its OWN report. Deliberately above
/// the 65 536-step lifetime cap the React host used to impose on every bounded job
/// (`🔌️PluginRuntime/🟦️.tsx`'s deleted `PLUGIN_JOB_STEP_LIMIT`): a real fill plan places up to
/// [`PUZZLE3D_FILL_COUNT_MAX`] pieces over the census of a whole document, and the host that cancels
/// it at a constant is the host that trapped the guest mid-run.
const LONG_PLAN_SLICES: u32 = 70_000;

/// 🧪️ A bounded owner whose only property is length — the shortest statement of "larger than one host
/// slice budget" that does not depend on the fill solver's own cost model.
const LONG_PLAN_JOB_KIND: &str = "puzzle3d.test.long-plan";

struct LongPlanBoundedJob {
    remaining: u32,
    cancelled: bool,
}

impl semio_framework_plugin::reactor::jobs::BoundedJob for LongPlanBoundedJob {
    fn step(&mut self, _budget: semio_framework_plugin::reactor::jobs::JobBudget) -> semio_framework_plugin::reactor::jobs::JobStep {
        if self.cancelled {
            return semio_framework_plugin::reactor::jobs::JobStep::Failed(b"long-plan.cancelled".to_vec());
        }
        self.remaining = self.remaining.saturating_sub(1);
        if self.remaining == 0 {
            return semio_framework_plugin::reactor::jobs::JobStep::Done(b"long-plan.done".to_vec());
        }
        semio_framework_plugin::reactor::jobs::JobStep::Running(None)
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        None
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        true
    }
}

fn long_plan_job_factory(_job: u64, _input: &[u8]) -> Result<Box<dyn semio_framework_plugin::reactor::jobs::BoundedJob>, Vec<u8>> {
    Ok(Box::new(LongPlanBoundedJob { remaining: LONG_PLAN_SLICES, cancelled: false }) as Box<dyn semio_framework_plugin::reactor::jobs::BoundedJob>)
}

/// 🪣️ A plan bigger than one host slice budget must reach its OWN terminal — no host may end it at a
/// step count of its own choosing.
///
/// The native shard host already obeys this: `ShardLoop::pump` walks `running_jobs` granting one
/// `job_budget_from_grant` per turn and has no lifetime counter at all
/// (`🔌️plugin/🖥️host/🧵️shard/🦀️.rs`). The React host did not — it cancelled at 65 536 steps, which a
/// 1 000-placement fill plan reaches in about forty seconds of ticking, and the cancel then trapped
/// the guest. The law states the reactor-side half of the contract both hosts share: an unchanged
/// per-slice [`JobBudget`] over [`LONG_PLAN_SLICES`] slices is NOT a stall, and the owner alone says
/// when the run is over. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5.
#[semio_framework_async_macros::async_test]
async fn a_plan_larger_than_one_host_slice_completes_without_a_host_imposed_cancellation() {
    use semio_framework_plugin::reactor::jobs::{self, JobStep};
    jobs::register_bounded_job_kind(LONG_PLAN_JOB_KIND, long_plan_job_factory as semio_framework_plugin::reactor::jobs::BoundedJobFactory);
    let job = 0xF5_00_00_01_u64;
    jobs::start_job(job, LONG_PLAN_JOB_KIND, &[]).await;
    let mut slices = 0_u32;
    let mut done = None;
    while slices < LONG_PLAN_SLICES.saturating_add(1) {
        slices += 1;
        match jobs::step_job(job, FILL_TICK_JOB_BUDGET).await {
            JobStep::Running(_) => {}
            JobStep::Done(bytes) => {
                done = Some(bytes);
                break;
            }
            JobStep::Failed(bytes) => panic!("a bounded owner that is still running must never be failed by its runtime after {slices} slices: {}", String::from_utf8_lossy(&bytes)),
        }
    }
    assert_eq!(done.as_deref(), Some(b"long-plan.done".as_slice()), "the owner's own terminal is the only terminal: {slices} slices");
    assert_eq!(slices, LONG_PLAN_SLICES, "a bounded owner reaches its terminal on exactly the slices it declared, whatever the host's own step counter says");
}

/// 🛑 Cancelling a fill job that is mid-flight must never trap the guest.
///
/// Production sequence this reproduces, verbatim: the Fill panel publishes `Cancel fill` with the live
/// `(job, operation, generation)`; the host dispatches `cancelFillBuild`; the guest answers
/// `Effect::CancelJob`; the host calls `jobs::cancel-job`, which drops the owner. Before W-F5 that drop
/// released a still-armed [`FillEnvelopeWorkerFaultGuard`], so a user cancel reported the envelope as a
/// FAULT — the `fill_failed` notice on a run the user stopped on purpose — and the guest carried on
/// against a torn envelope. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5.
#[semio_framework_async_macros::async_test]
async fn cancelling_a_stepping_fill_job_never_panics_and_reports_a_cancelled_run() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    let mut live: Vec<u64> = Vec::new();
    let mut identity = None;
    for _ in 0..FILL_TICK_GROWTH_CYCLES {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        if fill_ready(&mut app).await > 0.0 {
            identity = fill_cancel_identity(&mut app).await;
            if identity.is_some() {
                break;
            }
        }
    }
    let (job, operation, generation) = identity.expect("the Cancel fill affordance publishes the live job identity once planning has produced readiness");
    let cancel = dispatch(&mut app, "cancelFillBuild", Some(&json!({ "job": job, "operation": operation, "generation": generation })), None).await.expect("cancelFillBuild");
    let cancelled: Vec<u64> = cancel
        .requested_effects
        .iter()
        .filter_map(|effect| match effect {
            Effect::CancelJob { job } => Some(*job),
            _ => None,
        })
        .collect();
    assert_eq!(cancelled, vec![job], "a cancel naming the live run asks the host to stop exactly that job: {:?}", cancel.requested_effects);
    for job in &cancelled {
        semio_framework_plugin::reactor::jobs::cancel_job(*job).await;
        live.retain(|live_job| live_job != job);
    }
    let mut notices: Vec<String> = Vec::new();
    for _ in 0..FILL_TICK_ADMISSION_TICKS {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick after cancel");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        notices.extend(result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }));
    }
    assert!(
        !notices.iter().any(|message| message == Puzzle3dLabels::NATIVE_EN.fill_failed.as_str()),
        "a run the user cancelled is not a failure — the fault guard must not outlive a deliberate cancel: {notices:?}"
    );
    // 🔁️ Not `is_none`: once the cancelled envelope has given its slot back the tick loop legitimately
    // admits a FRESH plan, and the panel offers to cancel THAT one. What may never happen is the
    // affordance still naming the run the user already stopped.
    assert_ne!(fill_cancel_identity(&mut app).await, Some((job, operation, generation)), "the Cancel fill affordance must stop naming the run it already cancelled");
    let (occupied, _) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    assert!(occupied < crate::editor::puzzle3d::precompute::FILL_ENVELOPE_MAX_OPERATIONS, "a cancelled envelope must give its slot back: {occupied} still live");
    drop(app);
    crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
}

/// 🛑 The other cancel order, and the one the browser actually took: the HOST cancels a job the guest
/// never asked it to, so `jobs::cancel-job` arrives with no `cancelFillBuild` before it and no live
/// cancel token already tripped. That is what the deleted `PLUGIN_JOB_STEP_LIMIT` did at 65 536 steps,
/// and it must be survivable on its own terms — an actor whose instance goes away mid-plan takes the
/// same route. Ticket 26/09/02/PUZZLE-3D-END-TO-END W-F5.
#[semio_framework_async_macros::async_test]
async fn a_host_initiated_cancel_of_a_stepping_fill_job_leaves_the_guest_serving_further_ticks() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("activate fill");
    let mut live: Vec<u64> = Vec::new();
    let mut stepping = None;
    for _ in 0..FILL_TICK_GROWTH_CYCLES {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        if fill_ready(&mut app).await > 0.0 {
            stepping = live.first().copied();
            if stepping.is_some() {
                break;
            }
        }
    }
    let job = stepping.expect("a fill job that has stepped far enough to publish readiness");
    semio_framework_plugin::reactor::jobs::cancel_job(job).await;
    live.retain(|live_job| *live_job != job);
    let mut notices: Vec<String> = Vec::new();
    for _ in 0..FILL_TICK_ADMISSION_TICKS {
        let result = dispatch(&mut app, "fillBuildTick", None, None).await.expect("the guest must keep answering ticks after a host-side cancel");
        step_spawned_fill_jobs(&result.requested_effects, &mut live).await;
        notices.extend(result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }));
    }
    assert!(
        !notices.iter().any(|message| message == Puzzle3dLabels::NATIVE_EN.fill_failed.as_str()),
        "a host that stops a job it started is not reporting a plan failure to the user: {notices:?}"
    );
    let (occupied, _) = crate::editor::puzzle3d::precompute::fill_envelope_occupancy();
    assert!(occupied < crate::editor::puzzle3d::precompute::FILL_ENVELOPE_MAX_OPERATIONS, "a host-cancelled envelope must give its slot back: {occupied} still live");
    drop(app);
    crate::editor::puzzle3d::precompute::drain_fill_envelope_registry_for_test();
}
//#endregion 🪣️FillJobLifetime

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_only_plans_available_slider_range() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    // 🐢️ `drive_precompute` is bounded to a small per-call budget (the fix for the UI-freeze bug:
    // a single action must never grind the whole precompute queue synchronously), so the build
    // converges over several ticks — exactly like the real 120ms `fillBuildTick` loop.
    let mut app = app().await;
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    drive_fill_until_ready(&mut app, 4.0).await;
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    match find_measure_slider(tool_measures, "puzzle3d-fill-count") {
        Some(value) => assert_eq!(value, 0.0, "background planning must not change the selected fill count"),
        None => panic!("expected a fill-count slider in the fill tool measures"),
    }
    assert_eq!(object_count(&app), object_count_before, "background planning must not append generated objects below the slider count");
    assert_eq!(find_measure_slider_max(tool_measures, "puzzle3d-fill-count"), Some(PUZZLE3D_FILL_COUNT_MAX as f64), "fill slider range stays fixed at the fill count max");
    let available_count = find_measure_slider_ready(tool_measures, "puzzle3d-fill-count").expect("expected a fill-count slider ready extent") as usize;
    assert!(available_count > 0, "the fill slider ready extent must expose collision-free compatible placements");
    let begin = dispatch(&mut app, "setFillCount", Some(&json!({ "value": available_count })), None).await.expect("setFillCount");
    assert_eq!(object_count(&app), object_count_before + available_count, "the retained setFillCount command materializes the clamped ready prefix during settle");
    let immediate = render_composite(&mut app).await;
    assert_eq!(instance_count(&immediate), object_count_before + available_count, "the complete planned prefix is previewed immediately before document continuations finish");
    assert_eq!(interaction_of(&immediate).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(available_count as u64), "the reveal cutoff updates in the initiating interaction step");
    let (_, max_step) = finish_fill_count(&mut app, begin).await;
    assert!(max_step < std::time::Duration::from_millis(8), "every fill-count continuation must remain below the hard 8 ms interaction ceiling");
    assert_eq!(object_count(&app), object_count_before + available_count, "the fill slider must materialize exactly its available placement count");
    assert_eq!(instance_count(&render_composite(&mut app).await), object_count_before + available_count, "the viewport must show every materialized fill object immediately");
    let initial_fill_ids: HashSet<String> = projection_of(&app).get("objects").and_then(Value::as_array).into_iter().flatten().skip(object_count_before).filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    // 🪪️ Incidental actions re-sync the applied document into the precompute session. That used to
    // rebuild `fill.base` around the materialized objects, after which the slider could neither
    // remove them nor replan — reproduce with a hover sync before clearing.
    let hovered_id = first_object_id(&app);
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&hovered_id)).await.expect("interactionHover after fill");
    let reduced = available_count / 2;
    set_fill_count_and_finish(&mut app, reduced as u32, None).await;
    assert_eq!(object_count(&app), object_count_before + reduced, "sliding down after an incidental sync must still remove fill objects from the document");
    let reduced_render = render_composite(&mut app).await;
    // 🪣️ The viewport keeps showing the FULL available plan (tagged revealIndex) even after
    // reducing — hiding is a client-side reveal-cutoff concern now, not a server-side instance
    // count concern; only the document (checked above) and the committed cutoff actually shrink.
    assert_eq!(instance_count(&reduced_render), object_count_before + available_count, "the viewport still exposes the full plan for instant re-reveal — nothing was discarded");
    assert_eq!(interaction_of(&reduced_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64), "the committed reveal cutoff tracks the reduced count");
    // 🔽️🔼️ Prefix-stable plan: moving back up to a count that was already planned before must be
    // INSTANT — no replanning, no `fillBuildTick` catch-up dispatch.
    set_fill_count_and_finish(&mut app, available_count as u32, None).await;
    assert_eq!(object_count(&app), object_count_before + available_count, "moving back up within the preserved plan is instant, not gated on another fillBuildTick");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let target_measures = app.tool_measures(&view).await;
    let target_tool_measures = target_measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    assert_eq!(find_measure_slider(target_tool_measures, "puzzle3d-fill-count"), Some(available_count as f64));
    let restored_fill_ids: HashSet<String> = projection_of(&app).get("objects").and_then(Value::as_array).into_iter().flatten().skip(object_count_before).filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    assert_eq!(restored_fill_ids, initial_fill_ids, "up-down-up restores the exact same planned objects — the plan is prefix-stable, never discarded and re-rolled");
    set_fill_count_and_finish(&mut app, 0, None).await;
    assert_eq!(object_count(&app), object_count_before, "moving the fill slider to zero must remove every generated object");
}

#[semio_framework_async_macros::async_test]
async fn set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    // 🔒️ Requesting more than is currently planned must clamp (never leave `runtime.fill_count`
    // and the applied document disagreeing), and `fillBuildTick` must never self-dispatch another
    // `setFillCount` — the viewport already shows every planned piece (tagged `revealIndex`), so
    // there is nothing left for a catch-up round trip to accomplish.
    let mut app = app().await;
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let available_count = drive_fill_until_ready(&mut app, PUZZLE3D_FILL_COUNT_MAX as f64).await as u32;
    assert!(available_count > 0, "the maximum-delta timing proof requires a planned prefix");
    // Request far beyond what a single tick could have planned.
    let (steps, max_step) = set_fill_count_and_finish(&mut app, PUZZLE3D_FILL_COUNT_MAX, None).await;
    assert!(steps <= available_count.div_ceil(set_fill_count::MAX_PLACEMENTS_PER_STEP as u32) as usize, "a maximum slider request must use only fixed-size continuation chunks");
    assert!(max_step < std::time::Duration::from_millis(8), "maximum-delta fill materialization measured {max_step:?}; every continuation must remain below 8 ms");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.tool_measures(&view).await;
    let tool_measures = measures.get(fill_tool::TOOL_ID).expect("fill tool measures");
    let clamped = find_measure_slider(tool_measures, "puzzle3d-fill-count").expect("fill-count slider value");
    assert!(clamped <= available_count as f64, "runtime.fill_count must clamp to what's actually planned, not the raw request");
    assert_eq!(clamped as usize, object_count(&app) - object_count_before, "the clamped measure value must match what the document actually materialized");
    let tick = dispatch(&mut app, "fillBuildTick", None, None).await.expect("fillBuildTick after an above-ready request");
    assert!(
        !tick.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "setFillCount")),
        "fillBuildTick must never self-dispatch setFillCount — the clamp at commit time means fill_count can never run ahead of what's planned"
    );
}

#[semio_framework_async_macros::async_test]
async fn fill_render_reveals_the_full_available_plan_tagged_with_reveal_index() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    // 🪣️ `render()` composes EVERY currently-planned piece (not just the committed `fill_count`),
    // each tagged `revealIndex` — the viewport applies its own live, main-thread cutoff to show or
    // hide them per drag value with zero WASM round trips. The committed cutoff is separately
    // exposed as `interactionJson.revealCutoffs["puzzle3d-fill"]`.
    let mut app = app().await;
    let object_count_before = object_count(&app);
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let ready = drive_fill_until_ready(&mut app, 3.0).await as usize;
    assert!(ready >= 3, "fill planning must expose at least three ready placements");
    assert_eq!(object_count(&app), object_count_before, "background planning must not mutate the document before setFillCount");

    let rendered = render_composite(&mut app).await;
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

    set_fill_count_and_finish(&mut app, ready as u32, None).await;
    let after_commit = render_composite(&mut app).await;
    assert_eq!(instance_count(&after_commit), object_count_before + ready, "instance count is unchanged by commit — only the cutoff (and document) advanced");
    let committed_interaction = interaction_of(&after_commit);
    assert_eq!(committed_interaction.pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(ready as u64));
    assert_eq!(committed_interaction.pointer("/fillBuild/appliedCount").and_then(Value::as_u64), Some(ready as u64));
}

/// 🪣️ Fill count drives the shared document + reveal cutoff — split top/perspective panes must
/// never disagree about which planned objects are visible after a slider commit on either pane.
#[semio_framework_async_macros::async_test]
async fn fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances() {
    let _guard = crate::editor::puzzle3d::precompute::fill_envelope_test_guard();
    crate::editor::puzzle3d::precompute::initialize();
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), Some(top)).await.expect("select fill tool");
    let ready = drive_fill_until_ready(&mut app, 3.0).await as u32;
    assert!(ready >= 3, "need a planned fill prefix to assert cross-pane sync");

    // Commit from the top pane only — the perspective pane must still track the same cutoff.
    let committed = ready.min(3);
    set_fill_count_and_finish(&mut app, committed as u32, Some(top)).await;

    let top_render = render_window(&mut app, top).await;
    let perspective_render = render_window(&mut app, perspective).await;
    assert_eq!(interaction_of(&top_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(committed as u64), "top pane reveal cutoff must track the committed fill count");
    assert_eq!(interaction_of(&perspective_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(committed as u64), "perspective pane must share the same reveal cutoff — fill is document-global, not per-window");
    assert_eq!(instance_count(&top_render), instance_count(&perspective_render), "both panes must emit the same instance list for the shared fill plan");

    let instance_ids = |node: &Value| -> Vec<String> { instances_of(node).iter().filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect() };
    assert_eq!(instance_ids(&top_render), instance_ids(&perspective_render), "top and perspective must show the exact same object ids after a fill slider commit");

    // Sliding from the other pane must keep both panes in lockstep.
    let reduced = committed.saturating_sub(1);
    set_fill_count_and_finish(&mut app, reduced as u32, Some(perspective)).await;
    let top_after = render_window(&mut app, top).await;
    let perspective_after = render_window(&mut app, perspective).await;
    assert_eq!(interaction_of(&top_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
    assert_eq!(interaction_of(&perspective_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
    assert_eq!(instance_count(&top_after), instance_count(&perspective_after));
}

#[semio_framework_async_macros::async_test]
async fn seeded_objects_omit_reveal_index_so_the_boot_cutoff_cannot_hide_them() {
    let mut app = app().await;
    let rendered = render_composite(&mut app).await;
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
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
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
    {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
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
        !main::window_measures(&fill_scene, &session, labels, &Puzzle3dInteractionSnapshot::default()).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
        "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
    );
    let volume_brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::volume_brush::UTILITY_ID.into() };
    let volume_brush_measures = main::window_measures(&volume_brush_scene, &session, labels, &Puzzle3dInteractionSnapshot::default());
    assert_eq!(measure_group_tag(&volume_brush_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush")), Some(Some(utilities::volume_brush::UTILITY_ID.into())));
    assert!(find_measure_slider(&volume_brush_measures, "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
    let fill_engagement = main::engagement(&fill_scene, &Puzzle3dLabels::NATIVE_EN);
    assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
    let brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::brush::UTILITY_ID.into() };
    assert_eq!(measure_group_tag(&main::window_measures(&brush_scene, &session, labels, &Puzzle3dInteractionSnapshot::default()), &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())));
    let brush_engagement = main::engagement(&brush_scene, &Puzzle3dLabels::NATIVE_EN);
    assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
    }
    assert_brush_options_after_open_vortex_suggestions().await;
}

#[inline(never)]
async fn assert_brush_options_after_open_vortex_suggestions() {
    let mut app = app().await;
    activate_window_utility(&mut app, utilities::brush::UTILITY_ID);
    let view = app.window_view(main::WINDOW_KIND_ID);
    let brush_app_measures = app.window_measures(&view).await;
    let window_measures = brush_app_measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(measure_group_tag(window_measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush")), Some(Some(utilities::brush::UTILITY_ID.into())), "the brush Utility Options group surfaces on the live window-measures path when brush is the active utility");
}
//#endregion 🔖️Distribution

//#region 🔖️UiScope
/// @emoji 🐢️ THE scope law: every declared MUTATING command's `UiDirtyScope` names the panels its
/// mutation changes. A document edit moves the artifact outliner (the object roster), the field
/// inspector (the selected entity's own fields) and the framework history panel (one command row per
/// dispatch) — so a mutation whose scope is `Partial` and omits any of those three leaves that panel
/// showing pre-edit content until some unrelated later action happens to repaint it.
///
/// 🧯️ Why this is a law and not a review note: before wave N every hand-written `Partial` in this app
/// carried `panel_bodies: Vec::new()` (`📓️2026-09-09-wave-L-…md` §5.3) and the shared dispatcher's
/// default was `Full`, so the only two states a command could be in were "repaint the whole shell" and
/// "silently stale panels". `puzzle3d_command_scope_class` is now the one place that decides, and this
/// law is what stops a narrower class from being handed to a verb that edits the document.
///
/// 🈳️ `Chrome` (i.e. `Full`) always satisfies this law — it is the widest, always-correct answer, and a
/// newly declared command that nobody has classified yet is slow rather than wrong.
#[semio_framework_async_macros::async_test]
async fn command_scope_classes_name_the_panels_they_change() {
    use crate::editor::puzzle3d::{puzzle3d_command_scope_class, puzzle3d_scope, Puzzle3dScopeClass};
    let definition = create_puzzle3d_app();
    let mut mutating = 0_usize;
    let mut narrowed = 0_usize;
    for action in definition.window_kinds.iter().flat_map(|window| window.actions.iter()) {
        let class = puzzle3d_command_scope_class(&action.id);
        let scope = puzzle3d_scope(class);
        // 🧾️ The declared `ActionKind` is a HISTORY/undo classification, not a paint one. Both
        // `relocateTargetVolume` and `worldRelocate` are Mutations (they emit artifact edits), so this
        // law requires them to name inspector, outliner and history. A `View` verb that happens to carry
        // the document class is over-painting at worst, never stale.
        if action.kind != ActionKind::Mutation {
            continue;
        }
        mutating += 1;
        match scope {
            UiDirtyScope::Full => {}
            UiDirtyScope::None => panic!("{} is a declared Mutation, so it cannot paint nothing", action.id),
            UiDirtyScope::Partial { ref panel_bodies, ref window_bodies, .. } => {
                narrowed += 1;
                assert_eq!(window_bodies, &vec![main::BODY_KEY.to_string()], "{} edits the document, so the world body is dirty", action.id);
                for body in [inspection::BODY_KEY, document::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
                    assert!(panel_bodies.iter().any(|named| named == body), "{} is a declared Mutation with a narrowed scope, so it must name the {body} panel body: {panel_bodies:?}", action.id);
                }
            }
        }
    }
    assert!(mutating > 0, "the app declares mutating commands");
    assert!(narrowed >= 10, "the scope table must actually narrow the mutating verbs, not fall back to Full for all of them: {narrowed}/{mutating}");
}

/// 🕹️ A pure SELECTION change repaints the field inspector — the panel that renders the selected
/// entity's own fields — and the outliner that marks the selected rows, but never the catalogue, which
/// lists object KINDS and cannot move when only the selection moves.
#[semio_framework_async_macros::async_test]
async fn selection_scope_names_the_inspector_and_not_the_catalogue() {
    use crate::editor::puzzle3d::{puzzle3d_scope, Puzzle3dScopeClass};
    let UiDirtyScope::Partial { panel_bodies, measures, utilities, engagements, labels, .. } = puzzle3d_scope(Puzzle3dScopeClass::Selection) else {
        panic!("the selection class is a narrowed scope");
    };
    assert!(panel_bodies.iter().any(|body| body == inspection::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == document::BODY_KEY), "{panel_bodies:?}");
    assert!(panel_bodies.iter().any(|body| body == FRAMEWORK_HISTORY_BODY_KEY), "{panel_bodies:?}");
    assert!(!panel_bodies.iter().any(|body| body == catalogue::BODY_KEY), "a selection cannot change the kind roster: {panel_bodies:?}");
    assert!(measures, "selection-dependent window measures are re-read");
    assert!(!utilities);
    assert!(!engagements);
    assert!(!labels);
}

/// 🈳️ The four narrow non-document classes deliberately name NO panel body — a camera move, a fill
/// planning tick, a distribution slider and a suggestion tick each touch the world body (and at most
/// the tool/window measures) and nothing else. Pins the claim their doc comments make, so widening one
/// of them has to widen this law too.
#[semio_framework_async_macros::async_test]
async fn viewport_and_tick_scope_classes_name_no_panel_body() {
    use crate::editor::puzzle3d::{puzzle3d_scope, Puzzle3dScopeClass};
    for class in [Puzzle3dScopeClass::Viewport, Puzzle3dScopeClass::FillBuild, Puzzle3dScopeClass::FillOptions, Puzzle3dScopeClass::SuggestionsTick] {
        let UiDirtyScope::Partial { panel_bodies, window_bodies, .. } = puzzle3d_scope(class) else {
            panic!("{class:?} is a narrowed scope");
        };
        assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()], "{class:?}");
        assert!(panel_bodies.is_empty(), "{class:?} claims to paint no panel: {panel_bodies:?}");
    }
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Quiet), UiDirtyScope::None));
    assert!(matches!(puzzle3d_scope(Puzzle3dScopeClass::Chrome), UiDirtyScope::Full));
}

#[semio_framework_async_macros::async_test]
async fn fill_build_tick_is_a_view_action_with_narrow_ui_scope() {
    let definition = create_puzzle3d_app();
    let def = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == "fillBuildTick").expect("fillBuildTick declared");
    assert_eq!(def.kind, ActionKind::View, "fillBuildTick must stay a View action — it only advances background planning");
    let mut live = app().await;
    dispatch(&mut live, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let result = dispatch(&mut live, "fillBuildTick", None, None).await.expect("fillBuildTick");
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

/// 🪣️ `setFillCount` APPLIES planned placements, so it is a document edit that also moves the
/// fill-count slider range: narrow (no utilities, no engagements, no labels) but panel-naming. It used
/// to declare the pure fill-planning scope, which named no panel at all — so committing a fill left the
/// outliner, the inspector, the catalogue and the history panel showing the pre-fill document.
#[semio_framework_async_macros::async_test]
async fn set_fill_count_declares_a_narrow_document_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let result = dispatch(&mut app, "setFillCount", Some(&json!({ "value": 1 })), None).await.expect("setFillCount");
    match result.ui_scope {
        UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
            assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]);
            for body in [inspection::BODY_KEY, document::BODY_KEY, catalogue::BODY_KEY, FRAMEWORK_HISTORY_BODY_KEY] {
                assert!(panel_bodies.iter().any(|named| named == body), "a committed fill changes the {body} panel: {panel_bodies:?}");
            }
            assert!(tools, "the fill-count slider range lives in the fill tool's measures");
            assert!(measures);
            assert!(!engagements);
            assert!(!utilities);
            assert!(!labels);
        }
        other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn set_object_kind_weight_declares_fill_options_ui_scope() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": fill_tool::TOOL_ID })), None).await.expect("select fill tool");
    let object_ids = puzzle3d_kind_ids(&nakagin_fixture(), "objects");
    let kind_id = object_ids.first().expect("object kind");
    let result = dispatch(&mut app, "setObjectKindWeight", Some(&json!({ "kindId": kind_id.as_str(), "value": 0.75 })), None).await.expect("setObjectKindWeight");
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
    let mut app = app().await;
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), None).await.expect("addObjectKind");
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
    // 📝️ P1 arg form: firing addObjectKind with no args on a document whose catalogs were cleared must
    // materialize the declared `objectKind` default — the catalog row AND the object referencing it —
    // in ONE gesture. A retained tool job never publishes inline (`settle`'s own docstring), so the
    // settled document, not the dispatch's `mutations` vector, is where the gesture is observed.
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    let before = object_count(&app);
    dispatch(&mut app, "addObjectKind", None, None).await.expect("addObjectKind");
    assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
    let projection = projection_of(&app);
    let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
    assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
    let catalog = projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("the declared default kind catalog was materialized");
    assert!(catalog.iter().any(|row| row.get("id").and_then(Value::as_str) == Some("Object")), "the created object references a catalogued kind, never a dangling one: {catalog:?}");
}

#[semio_framework_async_macros::async_test]
async fn set_active_utility_emits_no_ops_and_no_history_entry() {
    // 🧰️ Switching utilities is the framework-injected View action: no document operations, no undo
    // entry, no re-emitted utility-switch effect (the command IS the direct switch).
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), None).await.expect("switch utility");
    assert!(result.mutations.is_empty(), "utility switching never emits document operations");
    assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
    assert_eq!(projection_of(&app), before, "utility switching does not mutate the document");
}

#[test]
fn set_active_utility_dirties_the_world_body() {
    use crate::editor::puzzle3d::{puzzle3d_command_scope_class, puzzle3d_scope, Puzzle3dScopeClass};
    assert_eq!(puzzle3d_command_scope_class(SET_ACTIVE_UTILITY_ACTION_ID), Puzzle3dScopeClass::Viewport, "utility switch must republish the world body (interaction + vortices), not Full chrome and not None");
    match puzzle3d_scope(Puzzle3dScopeClass::Viewport) {
        UiDirtyScope::Partial { window_bodies, .. } => assert_eq!(window_bodies, vec![main::BODY_KEY.to_string()]),
        other => panic!("utility switch scope must be the world body, got {other:?}"),
    }
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

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the Brush utility's "Placement"
/// picker exists only for a live brush target — an explicitly selected vortex, else the hovered one.
/// It reaches `window_measures` through `window_measures_with_request_context`, so this drives the
/// real app chrome rather than calling `main::window_measures` with a fabricated snapshot.
///
/// ⏰️ Host ticks the brush lane needs before the picker has rows: each `suggestionsTick` spends ONE
/// bounded `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US` slice, so on an opt-level-0 build a target costs a
/// couple of them. Fixed, never "until it resolves" — an unbounded loop here would turn the assertion
/// into a machine-load reading.
const PUZZLE3D_BRUSH_PICKER_TICKS: usize = 8;

#[semio_framework_async_macros::async_test]
async fn brush_placement_picker_appears_only_for_a_live_brush_target() {
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let idle = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert_eq!(find_measure_select(idle, "puzzle3d-brush-placement"), None, "no brush target means no placement picker");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    // ⏰️ The brush lane is warmed by the host's own 120 ms `suggestionsTick`, never by the render or
    // the measures call: `setActiveTool`/`setActiveUtility` are answered by the framework with an empty
    // `Emit` and reach no app reducer at all, so nothing in this app runs at the moment a utility is
    // activated (ticket 26/09/02/PUZZLE-3D-END-TO-END wave D3). Playing that clock here is what a
    // running host does between the click and the next frame.
    for _ in 0..PUZZLE3D_BRUSH_PICKER_TICKS {
        dispatch(&mut app, "suggestionsTick", None, Some(main::WINDOW_KIND_ID)).await.expect("suggestionsTick");
    }
    let view = app.window_view(main::WINDOW_KIND_ID);
    let measures = app.window_measures(&view).await;
    let targeted = measures.get(main::WINDOW_KIND_ID).expect("main window measures");
    assert!(find_measure_select(targeted, "puzzle3d-brush-placement").is_some(), "an explicitly selected vortex is a brush target, so the placement picker must render");
}
//#endregion 🔖️Utilities

//#region 🔖️WorldSelection
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the app-owned `worldSelect`
/// command is deleted — selection now goes exclusively through the framework's `interactionSelect`
/// verb (`select_id`), which is view-only by construction (`dispatch_interaction_action` never
/// touches `self.store`). Proves the `vortex` domain wiring reaches that same guarantee.
#[semio_framework_async_macros::async_test]
async fn world_select_emits_no_artifact_mutations() {
    let mut app = app().await;
    let before = projection_of(&app);
    let object_id = first_object_id(&app);
    let result = select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    assert!(result.mutations.is_empty(), "interactionSelect is framework-owned and view-only, must not diff the document");
    assert_eq!(projection_of(&app), before);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection paint rides on
/// `selectionJson` only — instance GEOMETRY stays byte-stable across a pick, so the host never has to
/// re-upload meshes just because something was selected.
#[semio_framework_async_macros::async_test]
async fn world_pick_keeps_instances_geometry_json_stable() {
    let mut app = app().await;
    let instances_before = instances_of(&render_composite(&mut app).await);
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let after = render_composite(&mut app).await;
    assert_eq!(instances_of(&after), instances_before, "picking must never perturb instance geometry");
    assert_eq!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()), Some(vec![object_id.clone()]));
    assert_eq!(
        selection_of(&after).get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![object_id]),
        "`selectionJson.ids` is what World3dHost paints instance selection from"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selectionJson` carries the exact
/// field names `World3dHost`'s `parseSelection`/`WorldSelectionRecord` reads — `ids` for object
/// instances, `targetVolumeIds`, `referenceSelectedId`, `hoveredId`. Vortex marks deliberately do NOT
/// appear here (that record has no vortex field; see `world_selection_json`'s doc comment).
#[semio_framework_async_macros::async_test]
async fn world_selection_json_carries_the_host_field_names_per_granularity() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, Some(&object_id)).await.expect("hover object");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(selection.get("hoveredId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(selection.get("targetVolumeIds").and_then(Value::as_array).map(Vec::len), Some(0));
    assert!(selection.get("vortexIds").is_none(), "the host's WorldSelectionRecord has no vortexIds field");

    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [0.0, 0.0, 0.0] })), Some(main::WINDOW_KIND_ID)).await.expect("addTargetVolume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.last().cloned()).and_then(|volume| volume.get("id").and_then(Value::as_str).map(str::to_string)).expect("target volume id");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select target volume");
    let selection = selection_of(&render_composite(&mut app).await);
    assert_eq!(
        selection.get("targetVolumeIds").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()),
        Some(vec![volume_id]),
        "a target-volume granularity selection paints through `targetVolumeIds`"
    );
    assert_eq!(selection.get("ids").and_then(Value::as_array).map(Vec::len), Some(0), "object `ids` stay empty while a target volume is the live granularity");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the world scene binds `domainId` (and
/// its own `domainGranularityId`) so `World3dHost` dispatches the generic `interactionSelect`/
/// `interactionHover` verbs — this crate has no handler for the legacy `worldPick`/`worldSelect`/
/// `setHover` fallbacks the host would otherwise use.
#[semio_framework_async_macros::async_test]
async fn world_scene_binds_the_vortex_interaction_domain_and_its_granularity() {
    let mut app = app().await;
    let scene = render_composite(&mut app).await.get("world3d").cloned().unwrap_or(Value::Null);
    assert_eq!(scene.get("domainId").and_then(Value::as_str), Some(PUZZLE3D_INTERACTION_DOMAIN));
    assert_eq!(scene.get("domainGranularityId").and_then(Value::as_str), Some(PUZZLE3D_GRANULARITY_OBJECT));
}

#[semio_framework_async_macros::async_test]
async fn world_pick_null_clears_without_reselecting_first_object() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_some_and(|selection| !selection.ids.is_empty()));
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "clicking empty background must clear, never fall back to reselecting the first object");
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
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.clone()], "flag": "locked", "value": true })), None).await.expect("lock");
    let instances = instances_of(&render_composite(&mut app).await);
    assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select locked object");
    assert_eq!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()),
        Some(vec![object_id]),
        "the vortex domain has no lock awareness — this now succeeds, the host must gate locked picks itself"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `PUZZLE3D_VORTEX_SHOW_SELECTED`
/// reveals a marker only while its own object (or one of its markers) is selected/hovered — the
/// live read `render_with_request_context` threads in as a `Puzzle3dInteractionSnapshot`.
#[semio_framework_async_macros::async_test]
async fn world_vortices_reveal_in_selected_mode_only_for_the_selected_object() {
    let mut app = app().await;
    let all_vortex_ids = vortex_full_ids(&app);
    assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "Selected mode with nothing selected reveals no vortex marker");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let revealed = vortices_of(&render_composite(&mut app).await);
    assert!(!revealed.is_empty(), "Selected mode must reveal the selected object's own vortex markers");
    assert!(revealed.iter().all(|vortex| vortex.get("objectId").and_then(Value::as_str) == Some(object_id.as_str())), "Selected mode must reveal ONLY the selected object's markers");
    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert!(vortices_of(&render_composite(&mut app).await).is_empty(), "clearing the selection hides the markers again");
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    assert!(!vortices_of(&render_composite(&mut app).await).is_empty(), "Always mode must still reveal every vortex marker");
}

/// 🕹️ A selected vortex marker carries its own `selected` flag on `vorticesJson` — the host's
/// `WorldVortexMarkers` reads it off each record (`WorldSelectionRecord` has no vortex field), and a
/// hovered marker carries `hovered` the same way.
#[semio_framework_async_macros::async_test]
async fn world_vortices_carry_their_own_selected_and_hovered_flags() {
    let mut app = app().await;
    dispatch(&mut app, "setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), None).await.expect("setVortexShow");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selected = vortices_of(&render_composite(&mut app).await);
    let record = selected.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("selected vortex record");
    assert_eq!(record.get("selected").and_then(Value::as_bool), Some(true));
    assert!(selected.iter().filter(|entry| entry.get("fullId").and_then(Value::as_str) != Some(vortex.as_str())).all(|entry| entry.get("selected").and_then(Value::as_bool) == Some(false)), "only the picked marker is selected");
    hover_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, Some(&vortex)).await.expect("hover vortex");
    let hovered = vortices_of(&render_composite(&mut app).await);
    let record = hovered.iter().find(|entry| entry.get("fullId").and_then(Value::as_str) == Some(vortex.as_str())).expect("hovered vortex record");
    assert_eq!(record.get("hovered").and_then(Value::as_bool), Some(true));
    assert_eq!(interaction_of(&render_composite(&mut app).await).get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `worldVortexSelect`/`worldPick`
/// are deleted; a `vortex`-domain `DomainSelection` only ever carries one granularity at a time
/// (see `Puzzle3dActionCtx::selected_ids`'s doc), so a `merge: "replace"` pick at a different
/// granularity inherently replaces the whole prior selection — verified against
/// `interaction_state()` (the render-time `selectionJson`/`vorticesJson` fields carry no live ids
/// any more, per `world_selection_json`'s known-gap doc comment).
#[semio_framework_async_macros::async_test]
async fn world_pick_object_replaces_vortex_selection() {
    let mut app = app().await;
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids, vec![object_id]);
}

#[semio_framework_async_macros::async_test]
async fn world_vortex_select_clears_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select object");
    let vortex = first_vortex_full_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortex).await.expect("select vortex");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
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
    let mut app = app().await;
    let vortices = vortex_full_ids(&app);
    assert!(vortices.len() >= 2, "fixture must expose two vortices");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[0]).await.expect("select first vortex");
    select_id(&mut app, PUZZLE3D_GRANULARITY_VORTEX, &vortices[1]).await.expect("replace with second vortex");
    let replaced = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(replaced.ids, vec![vortices[1].clone()]);

    let targets = to_json_string(&vec![InteractionTarget { granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), id: vortices[0].clone() }]);
    dispatch(&mut app, "interactionSelect", Some(&json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "invertive", "method": "pick" })), None).await.expect("invertive toggle");
    let invertive = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(invertive.ids.len(), 2, "invertive merge toggles the first vortex back into the selection alongside the second");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: "Select Same Kind" emits a real
/// `Emit.interaction_writes` entry, which `VcsArtifactApp` applies through the same `next_selection`
/// machine the reserved `interactionSelect` verb uses — so the widened selection is observable on
/// `interaction_state()` and painted into `selectionJson` on the next render.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_widens_the_selection_to_every_object_of_that_kind() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    for _ in 0..3 {
        dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    }
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert_eq!(ids.len(), 3, "three same-kind objects must exist");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &ids[0]).await.expect("select one");
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    let mut widened = selection.ids.clone();
    widened.sort();
    let mut expected = ids.clone();
    expected.sort();
    assert_eq!(widened, expected, "every object of the clicked object's kind is selected");
    let painted = selection_of(&render_composite(&mut app).await).get("ids").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
    let mut painted_sorted = painted.clone();
    painted_sorted.sort();
    assert_eq!(painted_sorted, expected, "the widened selection reaches the host through selectionJson");
}

/// 🕹️ "Select Same Kind" with nothing selected still aborts (nothing to widen from) and leaves the
/// selection untouched.
#[semio_framework_async_macros::async_test]
async fn select_same_kind_with_no_selection_leaves_the_selection_untouched() {
    let mut app = app().await;
    dispatch(&mut app, "selectSameKindSelection", None, None).await.expect("selectSameKindSelection");
    assert!(app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()), "widening from nothing must not invent a selection");
}

/// 🎯️ A selection-scoped command dispatched with NOTHING selected must refuse VISIBLY: exactly one
/// `Effect::Notify`, no document edit, and no history row. This is the browser defect measured
/// 2026-09-09 21:05 — "Duplicate Selection" from the context menu completed with no clone, no command
/// row and no notice of any kind, which is indistinguishable from a dead menu entry. Every arm that
/// reads the framework-owned selection is covered here, so the refusal cannot be reintroduced one
/// command at a time.
///
/// 🧾️ A command-log ROW is deliberately not asserted absent: `dispatch_emit` records one row per
/// dispatch by design, mutation-kind actions that produced zero operations included (see its own doc
/// in `🔌️plugin/🦀️.rs`), so a refused `Mutation` still shows up in history with no edit id. What must
/// never happen is a silent nothing — that is what the notice below pins.
#[semio_framework_async_macros::async_test]
async fn selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let objects_before = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    for action in ["duplicateSelection", "deleteSelection", "selectSameKindSelection"] {
        let result = dispatch(&mut app, action, None, None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(!matches!(result.ui_scope, UiDirtyScope::Full), "{action} painted nothing, so it must not force a full refresh");
    }
    // 🧲️ The three gumball verbs carry a `coalesce_key`, so they enter the latest-wins channel whose
    // accepted invocation answers BEFORE the command runs — their whole outcome (notice, scope) lives on
    // the completion lane, which `testkit::settle` now drains into `InvocationResult` instead of
    // dropping. They also never reach `refuse_without_selection`: `build_tool_job` routes them to
    // `Puzzle3dScaleWork`, not through `dispatch_step`, which is why they used to complete with an empty
    // edit, a coalesce key and `UiDirtyScope::Full` — measured 2026-09-09 in-process as
    // `completion scope=Full, effects=[]` while `duplicateSelection` on the same fixture refused.
    for (action, args) in [
        ("translateSelection", json!({ "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
        ("rotateSelection", json!({ "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": 1.0 })),
        ("scaleSelection", json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
    ] {
        let result = dispatch(&mut app, action, Some(&args), None).await.unwrap_or_else(|error| panic!("{action} must complete, not fault: {error:?}"));
        let raised = notices(&result);
        assert_eq!(raised.len(), 1, "{action} with nothing selected must raise exactly one notice on its completion lane: {:?}", result.requested_effects);
        assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored axis, so {action}'s refusal must be real prose");
        assert!(result.mutations.is_empty(), "{action} refused, so it must emit no document mutation: {:?}", result.mutations);
        assert!(matches!(result.ui_scope, UiDirtyScope::None), "{action} painted nothing, so its completion must carry UiDirtyScope::None, got {:?}", result.ui_scope);
    }
    let objects_after = projection_of(&app).get("objects").and_then(Value::as_array).map(Vec::len).unwrap_or_default();
    assert_eq!(objects_after, objects_before, "a refused selection command must leave the document untouched");
    assert!(
        app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).is_none_or(|selection| selection.ids.is_empty()),
        "a refused selection command must not invent a selection either"
    );
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `duplicateSelection` re-selects the
/// clones it created — the interaction write is applied AFTER the document mutations land, so the new
/// ids are already in `interaction_topology` and survive `validate_state`'s pruning.
#[semio_framework_async_macros::async_test]
async fn duplicate_selection_reselects_the_created_clones() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "duplicateSelection", None, None).await.expect("duplicateSelection");
    let selection = app.interaction_state().await.selection.get(PUZZLE3D_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    assert_eq!(selection.granularity, PUZZLE3D_GRANULARITY_OBJECT);
    assert_eq!(selection.ids.len(), 1, "exactly the one clone is selected");
    assert_ne!(selection.ids.first().map(String::as_str), Some(object_id.as_str()), "the CLONE is selected, not the original");
    let live_ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).map(|objects| objects.iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect()).unwrap_or_default();
    assert!(live_ids.contains(selection.ids.first().expect("clone id")), "the re-selected id must exist in the document");
}
//#endregion 🔖️WorldSelection

//#region 🔖️Gumball
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `gumballActive` requires BOTH the
/// transform utility active AND a live object (or target-volume) selection — the live read
/// `render_with_request_context` threads in. `transformMode`/`gumballConfig` depend only on the
/// active utility, per window.
#[semio_framework_async_macros::async_test]
async fn gumball_active_only_for_transform_utilities_with_object_selection() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let idle_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
    assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");

    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    let transform_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(true), "transform utility plus a live object selection shows the gumball");
    assert_eq!(transform_selection.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect::<Vec<_>>()), Some(vec![object_id.clone()]));
    assert_eq!(transform_selection.get("activeObjectId").and_then(Value::as_str), Some(object_id.as_str()));
    assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
    assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));

    dispatch(&mut app, semio_framework_plugin::CLEAR_SELECTION_ACTION_ID, None, None).await.expect("clear");
    assert_eq!(selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await).get("gumballActive").and_then(Value::as_bool), Some(false), "an unattached gumball must never render");

    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("reselect");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::brush::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("brush");
    let brush_selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
    assert!(brush_selection.get("transformMode").is_none());
}

/// 🕹️ Both handle flags off leaves nothing to grab, so the gumball must not render even with a live
/// selection and the transform utility active (`setTransformGumballFlag` is what the user toggles).
#[semio_framework_async_macros::async_test]
async fn gumball_inactive_when_every_handle_flag_is_off() {
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "move", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no move");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("no rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.get("gumballActive").and_then(Value::as_bool), Some(false), "a gumball with no handles must not render");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `transformMode` depends only on
/// each window instance's host-owned `ViewModel` utility map, so it is the per-window-isolation proof.
#[semio_framework_async_macros::async_test]
async fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
    let mut app = app().await;
    let top = main::WINDOW_INSTANCE_TOP;
    let perspective = main::WINDOW_INSTANCE_PERSPECTIVE;
    dispatch(&mut app, "worldPointerDown", None, Some(perspective)).await.expect("register perspective");
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(top)).await.expect("transform on top");
    let top_selection = selection_of(&render_window(&mut app, top).await);
    assert_eq!(top_selection.get("transformMode").and_then(Value::as_str), Some("transform"), "transform on top pane must switch that pane's own scene mode");
    let perspective_selection = selection_of(&render_window(&mut app, perspective).await);
    assert!(perspective_selection.get("transformMode").is_none(), "perspective pane must not inherit top pane's transform utility");
}

#[semio_framework_async_macros::async_test]
async fn transform_utility_options_expose_move_and_rotate_flags() {
    let labels = puzzle3d_labels(&semio_framework_plugin::ViewModel::default()).expect("admitted host axis");
    let session = Puzzle3dPrecomputeSession::new();
    let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: utilities::transform::UTILITY_ID.into() };
    let measures = main::window_measures(&scene, &session, labels, &Puzzle3dInteractionSnapshot::default());
    assert_eq!(measure_group_tag(&measures, &format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform")), Some(Some(utilities::transform::UTILITY_ID.into())));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
    assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
    let mut app = app().await;
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    dispatch(&mut app, "setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), Some(main::WINDOW_KIND_ID)).await.expect("disable rotate");
    let selection = selection_of(&render_window(&mut app, main::WINDOW_KIND_ID).await);
    assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
    assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
    let view = app.window_view(main::WINDOW_KIND_ID);
    let app_measures = app.window_measures(&view).await;
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
    // 🌀️ Repeated translate dispatches coalesce into one undo entry via AmendLast.
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    for dx in [1.0, 2.0, 3.0] {
        dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": dx, "dy": 0.0, "dz": 0.0 })), None).await.expect("drag tick");
    }
    let dragged = object_origin(&app, &object_id);
    assert!((dragged[0] - start[0] - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole coalesced gumball drag");
}

fn object_orientation(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("orientation").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

fn object_scale(app: &Puzzle3dApp, object_id: &str) -> Vec<f64> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get("scale").and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn gumball_rotate_drag_coalesces_into_one_edit() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_orientation(&app, &object_id);
    for angle in [0.25, 0.50, 0.75] {
        dispatch(&mut app, "rotateSelection", Some(&json!({ "ids": [object_id.as_str()], "ax": 0.0, "ay": 0.0, "az": 1.0, "angle": angle })), None).await.expect("rotate tick");
    }
    assert_ne!(object_orientation(&app, &object_id), start, "three rotate ticks must move the pose");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_orientation(&app, &object_id), start, "one undo restores the whole coalesced rotate drag");
}

#[semio_framework_async_macros::async_test]
async fn gumball_scale_drag_coalesces_into_one_edit() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_scale(&app, &object_id);
    for sx in [1.1, 1.2, 1.3] {
        dispatch(&mut app, "scaleSelection", Some(&json!({ "ids": [object_id.as_str()], "sx": sx, "sy": 1.0, "sz": 1.0 })), None).await.expect("scale tick");
    }
    assert_ne!(object_scale(&app, &object_id), start, "three scale ticks must change the scale");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_scale(&app, &object_id), start, "one undo restores the whole coalesced scale drag");
}

/// 🧲️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave T: the real gumball gesture — `transformBegin`, ONE
/// absolute start→end delta, `transformEnd` — moves the object by exactly that delta and undoes as
/// one edit. The brackets themselves contribute nothing; a second gesture starts from the pose the
/// first one left, with no app-side drag session to carry between them.
#[semio_framework_async_macros::async_test]
async fn gumball_gesture_commits_one_absolute_delta_between_its_host_brackets() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    dispatch(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": utilities::transform::UTILITY_ID })), Some(main::WINDOW_KIND_ID)).await.expect("transform");
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("interactionSelect");
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "transformBegin", None, None).await.expect("begin");
    dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 6.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("drag-end delta");
    dispatch(&mut app, "transformEnd", None, None).await.expect("end");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 6.0).abs() < 1e-9, "the one absolute delta lands verbatim on the document");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "one undo restores the whole gumball gesture");
    dispatch(&mut app, "transformBegin", None, None).await.expect("begin again");
    dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("second gesture delta");
    dispatch(&mut app, "transformEnd", None, None).await.expect("second end");
    assert!((object_origin(&app, &object_id)[0] - start[0] - 2.0).abs() < 1e-9, "a second gesture works from the restored pose");
}
//#endregion 🔖️Gumball

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
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    let seeded = object_count(&instance_a);
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("a adds object");
    dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("b adds object");

    // A neutral history action always calls store.dispatch(), which pumps inbound operations first.
    dispatch(&mut instance_a, "commitCheckpoint", None, None).await.expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).await.expect("pump b");

    assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
    assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
    let _ = Puzzle3dCamera::default();
}

//#endregion 🔖️Convergence

//#region 🔖️Close
/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END: the app-close contract, stated exactly once and asserted
/// by a test rather than by a destructor. `VcsArtifactApp::close_step` walks seven owned lanes in a
/// fixed order (`document-store`, `config-store`, `draft-store`, `presence-store`, `transient-store`,
/// window-transient, `interaction-store`) and every lane is mandatory: `drive_artifact_owned_disposer`
/// faults `interactive-job.close-owned-disposer-missing` the moment `A::build_<lane>_disposer()`
/// returned `None`. Until that holds for every lane, no puzzle3d instance can ever reach its
/// terminal-empty witness — in a test process OR in a host, where the same `close_step` runs on
/// shutdown and the framework-installed `ArtifactStoreCursorDisposer` members then panic in `Drop`.
#[semio_framework_async_macros::async_test]
async fn fixture_app_reaches_its_terminal_empty_close_witness() {
    match close_witness(app().await) {
        Ok(true) => {}
        Ok(false) => panic!("close_step reported Complete without reaching terminal-empty ownership"),
        Err(fault) => panic!("every owned close lane must supply its bounded disposer, got {fault:?}"),
    }
}
//#endregion 🔖️Close

/// 🎟️ Wave W-P: `Puzzle3dPlayApp` used to be rebuilt from `default()` on every dispatch and every render,
/// so `geometry_cache` was structurally unable to observe two calls in a row and the whole fixture was
/// re-serialized every time (`📓️2026-09-08-performance-architecture-audit.md` §1, fix #1). With a session
/// slot keyed by `app_instance_id`, the second call for the same document must serialize NOTHING.
#[test]
fn a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing() {
    let config = Puzzle3dRuntime::default();
    let fixture = default_fixture();
    let fingerprint = main::fixture_geometry_fingerprint(&fixture);
    let session = Some((4_001_u32, Some("document-geometry".to_string())));
    let cold = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    let first = with_puzzle3d_app_for(session.clone(), &config, |app| app.geometry_jsons(&fixture));
    let after_first = PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get);
    assert_eq!(after_first - cold, 1, "the first call for a cold instance serializes exactly once");
    let second = with_puzzle3d_app_for(session, &config, |app| {
        let cached = app.geometry_cache.lock().expect("geometry cache");
        assert_eq!(cached.as_ref().map(|(cached, _, _)| *cached), Some(fingerprint), "the session slot handed the warm cache to a brand-new app object");
        drop(cached);
        app.geometry_jsons(&fixture)
    });
    assert_eq!(PUZZLE3D_GEOMETRY_SERIALIZATIONS.with(std::cell::Cell::get), after_first, "the second call on the same instance must not re-serialize anything");
    assert_eq!(first, second, "a cache hit returns byte-identical instance and mesh json");
}

/// 🥽️ Wave W-P: a mesh registered by one dispatch used to be gone by the next, because the collision
/// engine died with its app object (audit bottleneck (h)). A worker hop is exactly "a new app object with
/// the same instance id", so the resumed call must still hold the registered geometry.
#[test]
fn a_worker_hop_resume_still_holds_the_registered_brush_mesh() {
    let config = Puzzle3dRuntime::default();
    let session = Some((4_002_u32, Some("document-mesh".to_string())));
    let url = "/test/session-hop.glb";
    let positions: Vec<f32> = vec![-1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2];
    with_puzzle3d_app_for(session.clone(), &config, |app| {
        app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
        assert!(app.precompute.borrow().has_mesh(url), "the registering dispatch sees its own mesh");
    });
    with_puzzle3d_app_for(session, &config, |app| {
        assert!(app.precompute.borrow().has_mesh(url), "the resumed dispatch adopted the same instance's registered mesh");
    });
    with_puzzle3d_app_for(Some((4_003_u32, Some("document-other".to_string()))), &config, |app| {
        assert!(app.precompute.borrow_mut().adopt_shared_mesh(url, None), "a different document reaches the same geometry by id alone");
    });
    with_puzzle3d_app_for(None, &config, |app| {
        assert!(!app.precompute.borrow().has_mesh(url), "a call with no instance identity stays session-less, exactly as before");
    });
}

/// 🎫 Wave W-P: instance ids are reused by the framework, so a slot re-keyed to a different parent
/// document must retire — and every lease taken against the previous document must fail to check in
/// rather than leak one document's cached geometry into another's render.
#[test]
fn a_stale_session_lease_is_rejected_and_a_rekeyed_instance_starts_cold() {
    let instance = 4_004_u32;
    let stale = {
        let mut registry = puzzle3d_session_registry().lock().expect("session registry");
        let (stale, held) = registry.check_out(instance, Some("document-a")).expect("first lease");
        assert!(held.geometry.is_none(), "a cold slot hands out no cached geometry");
        let (fresh, rekeyed) = registry.check_out(instance, Some("document-b")).expect("re-keyed lease");
        assert!(rekeyed.geometry.is_none(), "a re-keyed instance starts cold instead of adopting the previous document");
        assert_ne!(stale.generation, fresh.generation, "re-keying bumps the slot generation");
        registry.check_in(stale, Puzzle3dSessionState { geometry: Some((7, "stale-instances".into(), "stale-meshes".into())), ..Default::default() });
        stale
    };
    let mut registry = puzzle3d_session_registry().lock().expect("session registry");
    let (_, adopted) = registry.check_out(instance, Some("document-b")).expect("post-stale lease");
    assert!(adopted.geometry.is_none(), "the stale lease's state was refused, so document-b is still cold");
    assert_ne!(stale.generation, registry.generations[usize::try_from(instance).expect("slot base") % PUZZLE3D_SESSION_SLOTS], "the retired generation is never handed out again");
}

/// ⚖️ Wave W-P: the session census is a real bound, not a slot count — a check-in whose bytes would cross
/// `PUZZLE3D_SESSION_PROCESS_BYTES` is dropped, which costs one cold rebuild and never corrupts anything.
#[test]
fn a_session_check_in_over_the_process_byte_ceiling_is_dropped() {
    let mut registry = Puzzle3dSessionRegistry::default();
    let (lease, _) = registry.check_out(11, Some("document-census")).expect("lease");
    registry.check_in(lease, Puzzle3dSessionState { geometry: Some((1, "x".repeat(PUZZLE3D_SESSION_PROCESS_BYTES), String::new())), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "a census exactly at the ceiling is still admissible");
    let (lease, held) = registry.check_out(12, Some("document-second")).expect("second lease");
    assert!(held.geometry.is_none(), "a different instance owns a different slot and starts cold");
    registry.check_in(lease, Puzzle3dSessionState { geometry: Some((2, "y".into(), String::new())), ..Default::default() });
    assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "one byte past the ceiling is refused rather than admitted");
    let (_, refused) = registry.check_out(12, Some("document-second")).expect("third lease");
    assert!(refused.geometry.is_none(), "the refused state is simply absent on the next call, so that instance rebuilds cold");
}

/// 📐️ Wave W-P: the session row is a fixed 64-slot array, so one slot's inline size is multiplied by 64
/// every time the registry is constructed. Anything multi-kilobyte by value (a `BuiltNode`, a fixture)
/// belongs behind a pointer, not inline — a fat slot is how a fixed row turns into a stack overflow.
#[test]
fn one_session_slot_stays_small_enough_for_a_fixed_row() {
    let slot = size_of::<Puzzle3dSessionSlot>();
    let state = size_of::<Puzzle3dSessionState>();
    let collision = size_of::<Puzzle3dCollisionSession>();
    let app = size_of::<Puzzle3dPlayApp>();
    println!("[wave-P sizes] slot={slot} state={state} collision={collision} app={app} row={}", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(slot <= 64, "one session slot grew to {slot} bytes; keep the cached state behind a pointer");
    assert!(slot * PUZZLE3D_SESSION_SLOTS <= 8 * 1024, "the whole session row grew to {} bytes and is built by value", slot * PUZZLE3D_SESSION_SLOTS);
    assert!(state <= 2048, "one session state grew to {state} bytes");
    assert!(collision <= 2048, "the carried collision session grew to {collision} bytes");
    assert!(app <= 32 * 1024, "Puzzle3dPlayApp grew to {app} bytes; it is built on the stack on every dispatch and every render, and async dispatch futures hold several copies inline");
}

//#region ⏱️InteractiveStepBudget
/// 🚨️ Mirror of `semio_framework_trace::INTERACTIVE_STEP_CEILING_US` (8 000 µs) — that crate is not a
/// direct dependency of this artifact. `semio_framework_job` faults any interactive step at or over it
/// with `interactive_step_contract_violated`, and `MountedTypedCommandFullOperation`'s worker pump
/// cancels the lease before the fault body is surfaced, so the user reads a real budget overrun as a
/// bare "typed-operation cancelled". Every retained step of every puzzle3d command must stay under it.
const PUZZLE3D_INTERACTIVE_STEP_CEILING: std::time::Duration = std::time::Duration::from_micros(8_000);

/// 🎯️ What this artifact holds itself to in the UNOPTIMIZED test profile — a quarter of the framework
/// ceiling, so an optimized guest keeps an order of magnitude of headroom over the same document.
const PUZZLE3D_MEASURED_STEP_BUDGET: std::time::Duration = std::time::Duration::from_micros(2_000);

/// 🔁️ Cold runs each measured law drives, keeping each TURN's best across them — the standard robust
/// estimator for "how much work does one turn do", which is the only thing this artifact controls. This
/// repository is worked on by several sessions at once and this box runs their cargo builds alongside
/// the suite: at the millisecond scale a single wall-clock sample measures the scheduler, not the
/// artifact. Measured on the SAME binary, same turn: `fillBuildTick` gave 1.55 / 3.68 / 2.69 / 3.72 /
/// 2.19 ms across five consecutive runs at load average 64, and 12.67 ms at load average 79, against a
/// best of 1.25 ms (ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3). Both bounds below are therefore read
/// off those per-turn minima — including the framework ceiling, whose per-turn contract the framework
/// itself enforces at runtime; what a test can prove is that the WORK inside a turn fits it.
const PUZZLE3D_MEASURED_STEP_RUNS: u32 = 5;

/// ⏱️ Drives one retained command work to `Complete` through its REAL `step()` and answers how long
/// every single turn took, in order. The bounds are asserted by the caller, over `measured_cold_runs`'
/// per-turn best of several cold runs.
fn measured_step_loop(work: &mut dyn crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>>, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig, guard: usize, label: &str) -> Vec<std::time::Duration> {
    use crate::retained_command::PuzzleCommandWorkStep;
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let mut turns = Vec::with_capacity(guard);
    loop {
        assert!(turns.len() <= guard, "{label} step() did not reach Complete within {guard} bounded turns");
        let started = std::time::Instant::now();
        let outcome = work.step(command, snapshot, config, &interaction, &hover).expect("bounded step");
        turns.push(started.elapsed());
        if matches!(outcome, PuzzleCommandWorkStep::Complete(_)) {
            break;
        }
    }
    turns
}

/// 🎛️ The exact work `build_tool_job` routes this tool id to, bound to a window context the way the
/// framework binds every admitted tool job. The routing itself is pinned by this file's own
/// source-text guards; this mirrors it so a measurement drives the REAL production work. Each run is
/// bound to its OWN instance id so it starts from a cold session slot — a warm slot skips the mesh
/// seeding that is exactly what the budget is being measured against.
fn measured_tool_work(tool_id: &'static str, run: u32) -> Box<dyn crate::retained_command::PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>>> {
    use crate::retained_command::PuzzleCommandWork;
    let mut work: Box<dyn PuzzleCommandWork<EditorApp<Puzzle3dPlayApp>>> = match tool_id {
        "acceptSuggestion" => Box::new(Puzzle3dAcceptSuggestionWork::default()),
        "setActiveExample" => Box::new(Puzzle3dSetActiveExampleWork::default()),
        "fillBuildTick" => Box::new(Puzzle3dPrecomputeCommandWork::new(tool_id)),
        "openVortexSuggestions" => Box::new(Puzzle3dWindowCommandWork::new(tool_id)),
        _ => Box::new(crate::retained_command::BoundedFirstStepCommandWork::new(tool_id, puzzle3d_retained_reduce, puzzle3d_retained_extent)),
    };
    work.bind_view_state(Some(measured_view_state()));
    work.bind_window_owners(None, None);
    work.bind_instance(9_000 + run, &format!("measured-{tool_id}-{run}"));
    work
}

/// ⏱️ Drives one tool id's REAL work to `Complete` `PUZZLE3D_MEASURED_STEP_RUNS` times, each from its
/// own cold session slot, and answers `(turns, the worst PER-TURN BEST)`: every run of the same cold
/// document takes the same bounded turns in the same order (asserted), so turn `t`'s cost is the
/// minimum of that turn across the runs, and the law is on the worst of those minima. A scheduler
/// preemption lands on one turn of one run and is cancelled by the others — see
/// `PUZZLE3D_MEASURED_STEP_RUNS` for why that matters on this box.
fn measured_cold_runs(tool_id: &'static str, command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, config: &Puzzle3dConfig) -> (usize, std::time::Duration) {
    let mut best: Vec<std::time::Duration> = Vec::new();
    for run in 0..PUZZLE3D_MEASURED_STEP_RUNS {
        let mut work = measured_tool_work(tool_id, run);
        let turns = measured_step_loop(work.as_mut(), command, snapshot, config, crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, tool_id);
        if best.is_empty() {
            best = turns;
            continue;
        }
        assert_eq!(turns.len(), best.len(), "{tool_id} run {run} took a different number of bounded turns than its cold siblings");
        for (slot, measured) in best.iter_mut().zip(turns) {
            *slot = (*slot).min(measured);
        }
    }
    let (index, worst) = best.iter().enumerate().max_by_key(|(_, turn)| **turn).map_or((0, std::time::Duration::ZERO), |(index, turn)| (index + 1, *turn));
    eprintln!("[DEBUG] puzzle3d {tool_id}: {} turns, worst turn {index} at {worst:?}", best.len());
    (best.len(), worst)
}

fn measured_view_state() -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        window_id: Some(main::WINDOW_KIND_ID.to_string()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: main::WINDOW_KIND_ID.to_string(), window_kind_id: main::WINDOW_KIND_ID.to_string() }],
        ..Default::default()
    }
}

fn measured_nakagin_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&NAKAGIN_EXAMPLE_FIXTURE.clone())).into())
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END: `openVortexSuggestions` syncs the whole precompute session
/// and refreshes one vortex's brush candidates. Measured on Nakagin at opt-level 0: 82.5 ms (W-P2's
/// baseline), 17.6 ms once W-P2 removed the doubled session sync and the doubled projection decode, and
/// under this artifact's own 2 000 µs budget once W-P3 split `handle_action_impl`'s prologue into the
/// scene / session-sync / dispatch turns [`Puzzle3dActionPrologue`] declares and made each of them typed.
/// This drives the REAL `Puzzle3dWindowCommandWork` `build_tool_job` routes this tool id to.
#[test]
fn open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("openVortexSuggestions", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("openVortexSuggestions command decodes");
    let (steps, worst) = measured_cold_runs("openVortexSuggestions", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "openVortexSuggestions worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "openVortexSuggestions worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END: `fillBuildTick` scans the document across its bounded census
/// turns and then runs the shared action prologue, which used to be ONE 17.6 ms publish turn and is now
/// the scene turn, one turn per owed collision-mesh fallback, the engine-scene build and push turns, and
/// the dispatch turn — every one of them inside this artifact's own 2 000 µs budget.
#[test]
fn fill_build_tick_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("fillBuildTick", None, Some(main::WINDOW_KIND_ID.to_string())).expect("fillBuildTick command decodes");
    let (steps, worst) = measured_cold_runs("fillBuildTick", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "fillBuildTick worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "fillBuildTick worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P2: `acceptSuggestion` walks the document for its target
/// vortex and publishes the placement. Measured at 11 796 µs in ONE step before this wave.
#[test]
fn accept_suggestion_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_nakagin_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("acceptSuggestion", Some(json!({ "fullId": "25b0dba0-8f81-423a-94a1-b911a6031010:link" })), Some(main::WINDOW_KIND_ID.to_string())).expect("acceptSuggestion command decodes");
    let (steps, worst) = measured_cold_runs("acceptSuggestion", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "acceptSuggestion worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "acceptSuggestion worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

fn measured_concrete_forest_snapshot() -> Puzzle3dPlaySnapshot {
    Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&CONCRETE_FOREST_EXAMPLE_FIXTURE.clone())).into())
}

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END W-P3: swapping the Concrete Forest document for the
/// 180-object Nakagin one is the single largest document gesture this artifact has — it deletes every
/// existing attraction and object and creates every Nakagin one. Its work is fully typed and cursorized
/// (`Puzzle3dSetActiveExampleWork`), so no turn of it may cross the interactive step ceiling either.
#[test]
fn set_active_example_every_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let snapshot = measured_concrete_forest_snapshot();
    let config = Puzzle3dConfig::default();
    let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), Some(main::WINDOW_KIND_ID.to_string())).expect("setActiveExample command decodes");
    let (steps, worst) = measured_cold_runs("setActiveExample", &command, &snapshot, &config);
    assert!(worst < PUZZLE3D_INTERACTIVE_STEP_CEILING, "setActiveExample worst turn {worst:?} over {steps} turns is at or over the framework's interactive step ceiling {PUZZLE3D_INTERACTIVE_STEP_CEILING:?}");
    assert!(worst < PUZZLE3D_MEASURED_STEP_BUDGET, "setActiveExample worst turn {worst:?} over {steps} turns exceeds this artifact's own unoptimized budget {PUZZLE3D_MEASURED_STEP_BUDGET:?}");
}

/// 🌉️ Differential law for W-P3's typed `scene_from_snapshot`: the derived `ToValue`/`FromValue`
/// machinery is an independent implementation of the same structural-twin translation, so the typed
/// fixture must equal what the persisted-projection bridge produced for every shipped document. A
/// disagreement here is a real behaviour change, not a performance one — the semantic document delta
/// every editing action publishes is taken against exactly this fixture.
///
/// 🪪️ Each snapshot is canonicalized first (rebuilt from its OWN typed authority) because that is the
/// only projection production ever hands the app: a store-driven `Puzzle3dPlaySnapshot` carries the
/// typed document and materializes `value()` from it, so `value()` is by construction
/// `ToValue(typed())`. Feeding a raw editor-side fixture straight into `new()` can hand the two halves
/// different content — `empty_fixture()`'s `meta` serializes both members as `Null`, which the typed
/// decode refuses, and `new()` silently falls back to `Puzzle3dSnapshot::default()`; see this wave's
/// report §6.
///
/// 🗝️ The two untyped `meta` members are compared as the TYPED catalogs they stand for
/// (`Puzzle3dKindCatalogs` / `Vec<Puzzle3dKindCompatibility>` — lossless, and what every reader of
/// those members ultimately decodes them into), because raw `DslValue` equality would compare key
/// ORDER: the bridge inherits the persisted projection's own `serde_json` map order and the typed
/// construction emits declaration order, for byte-identical content. Every typed member of the fixture
/// is compared directly, unnormalized.
#[test]
fn puzzle3d_typed_fixture_matches_the_projection_bridge_for_every_example() {
    for (label, fixture) in [("empty", empty_fixture()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()), ("nakagin", NAKAGIN_EXAMPLE_FIXTURE.clone())] {
        let seed = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(&fixture)).into());
        let snapshot = Puzzle3dPlaySnapshot::new((&dsl::ToValue::to_value(seed.typed())).into());
        let bridged = scene_from_projection(&puzzle3d_projection_value(snapshot.value()), Puzzle3dRuntime::default(), "utility");
        let typed = scene_from_snapshot(snapshot.typed(), Puzzle3dRuntime::default(), "utility");
        assert_eq!(typed.fixture.schema, bridged.fixture.schema, "{label}: schema disagrees");
        assert_eq!(typed.fixture.domain, bridged.fixture.domain, "{label}: domain disagrees");
        assert_eq!(typed.fixture.objects, bridged.fixture.objects, "{label}: objects disagree");
        assert_eq!(typed.fixture.attractions, bridged.fixture.attractions, "{label}: attractions disagree");
        assert_eq!(typed.fixture.target_volumes, bridged.fixture.target_volumes, "{label}: target volumes disagree");
        assert_eq!(typed.fixture.references, bridged.fixture.references, "{label}: references disagree");
        let catalogs = |meta: &Puzzle3dFixtureMeta| -> crate::Puzzle3dKindCatalogs { meta.kind_catalogs.clone().map_or_else(crate::Puzzle3dKindCatalogs::default, |rows| dsl::FromValue::from_value(rows).expect("kind catalogs decode")) };
        let compatibility = |meta: &Puzzle3dFixtureMeta| -> Vec<crate::Puzzle3dKindCompatibility> { meta.kind_compatibility.clone().map_or_else(Vec::new, |rows| dsl::FromValue::from_value(rows).expect("kind compatibility decodes")) };
        assert_eq!(catalogs(&typed.fixture.meta), catalogs(&bridged.fixture.meta), "{label}: kind catalogs disagree");
        assert_eq!(compatibility(&typed.fixture.meta), compatibility(&bridged.fixture.meta), "{label}: kind compatibility disagrees");
        assert_eq!(typed.active_utility, bridged.active_utility, "{label}: active utility disagrees");
    }
}

/// 🌉️ Differential law for W-P3's typed `scene_config`: the same engine scene the all-`DslValue` bridge
/// (`scene_config_value` + the derived `FromValue`) produced, field for field, on every shipped document
/// and with real kind weights on the runtime. `SceneConfig: PartialEq` is the engine's OWN resync
/// verdict, so equality here is exactly the property the precompute session reads.
#[test]
fn puzzle3d_typed_scene_config_matches_the_value_bridge_for_every_example() {
    for (label, fixture) in [("empty", empty_fixture()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()), ("nakagin", NAKAGIN_EXAMPLE_FIXTURE.clone())] {
        let mut runtime = Puzzle3dRuntime::default();
        runtime.overlap_budget = 0.375;
        runtime.object_kind_weights.insert("capsule".into(), 0.25);
        runtime.vortex_kind_weights.insert("rim".into(), 0.75);
        let envelope = Puzzle3dScene { fixture, runtime, active_utility: "utility".into() };
        let bridged: crate::standards::v1::subsets::any::schema::SceneConfig = dsl::FromValue::from_value(scene_config_value(&envelope)).expect("value bridge decodes");
        let typed = scene_config(&envelope).expect("typed scene config builds");
        assert_eq!(typed, bridged, "{label}: typed engine scene disagrees with the value bridge");
    }
}

/// 🥽️ W-P3: the mesh-url index answers exactly what the per-object catalog scan answered, for every
/// object of every shipped document, and `collect_mesh_urls` still returns the same SET of identities.
#[test]
fn puzzle3d_kind_mesh_index_matches_a_per_object_catalog_scan() {
    for (label, fixture) in [("empty", empty_fixture()), ("concrete-forest", CONCRETE_FOREST_EXAMPLE_FIXTURE.clone()), ("nakagin", NAKAGIN_EXAMPLE_FIXTURE.clone())] {
        let index = Puzzle3dKindMeshIndex::of(&fixture.meta);
        for object in &fixture.objects {
            let scanned = fixture
                .meta
                .kind_catalogs
                .as_ref()
                .and_then(|catalogs| catalogs.get("objects"))
                .and_then(dsl::DslValue::as_array)
                .and_then(|rows| rows.iter().find(|row| row.get("id").and_then(dsl::DslValue::as_str) == object.object_kind.as_deref()))
                .and_then(|row| row.get("meshUrl"))
                .and_then(dsl::DslValue::as_str);
            let expected = object.mesh_url.as_deref().filter(|url| !url.is_empty()).or(scanned);
            assert_eq!(index.resolve(object), expected, "{label}: {} resolved to a different mesh identity", object.id);
        }
        let indexed: std::collections::BTreeSet<String> = collect_mesh_urls(&fixture).into_iter().collect();
        assert!(indexed.iter().all(|url| !url.is_empty()), "{label}: an empty mesh identity was collected");
    }
}

//#endregion ⏱️InteractiveStepBudget

//#region 🩹️CheckedDefects
/// 🎯️ Every action id one built context menu dispatches, groups and submenus included.
fn context_menu_action_ids(items: &[semio_framework_plugin::ContextMenuItemSpec], into: &mut Vec<(String, String)>) {
    for item in items {
        if let Some(action) = item.action.as_deref() {
            into.push((item.id.clone(), action.to_string()));
        }
        if let Some(children) = item.children.as_ref() {
            context_menu_action_ids(children, into);
        }
    }
}

/// 🎯️ Every action id `create_puzzle3d_app()` declares — the set `dispatch_action` can actually route.
fn declared_action_ids() -> std::collections::BTreeSet<String> {
    create_puzzle3d_app().window_kinds.iter().flat_map(|window| window.actions.iter()).map(|action| action.id.clone()).collect()
}

/// 🖱️ Context-menu rows bypass the registry-validated `Menu::action` builder (they carry
/// `Puzzle3dLabels` text the English-only `ActionDefinition` cannot resolve), so nothing catches a
/// typo'd action id at build time. `📓️2026-09-09-user-feature-checklist.md` §15/§22 and summary #7:
/// "Zoom to Selection" dispatched `zoomToSelection`, which this crate never declares — the registered
/// verb is `focusSelection`. This law walks EVERY selection kind's menu and holds every emitted id
/// inside the declared set, so the whole class cannot come back.
#[semio_framework_async_macros::async_test]
async fn every_context_menu_row_dispatches_a_declared_action() {
    let declared = declared_action_ids();
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let object_id = first_object_id(&app);
    let vortex_id = first_vortex_full_id(&app);
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    let mut zoom_rows = 0usize;
    for (granularity, id) in [
        (PUZZLE3D_GRANULARITY_OBJECT, object_id.as_str()),
        (PUZZLE3D_GRANULARITY_VORTEX, vortex_id.as_str()),
        (PUZZLE3D_GRANULARITY_TARGET_VOLUME, volume_id.as_str()),
        (PUZZLE3D_GRANULARITY_REFERENCE, "reference-1"),
        (PUZZLE3D_GRANULARITY_ATTRACTION, "attraction-1"),
    ] {
        let menu = context_menu_for_selection(&mut app, granularity, id).await;
        let mut rows = Vec::new();
        context_menu_action_ids(&menu, &mut rows);
        assert!(!rows.is_empty(), "{granularity} selection built a menu with no actionable row");
        for (row, action) in &rows {
            assert!(declared.contains(action), "{granularity} row {row} dispatches undeclared action {action}; declared: {declared:?}");
            if row == "zoom" {
                zoom_rows += 1;
                assert_eq!(action, "focusSelection", "the zoom row must dispatch the registered camera verb");
            }
        }
        eprintln!("[DEBUG] context menu {granularity} rows={rows:?}");
    }
    assert_eq!(zoom_rows, 3, "object, vortex and reference selections each carry a Zoom to Selection row");
    // 🎯️ Reachability, not just declaration: the id the row carries must be one the typed command
    // channel actually admits (an unknown id panics inside `dispatch`).
    dispatch(&mut app, "focusSelection", None, None).await.expect("the context menu's zoom row must dispatch");
}

/// 🗂️ `📓️2026-09-09-user-feature-checklist.md` §23/summary #9: both the "Add Object" dialog and the
/// standalone `addObjectKind` arg form hardcoded ONE static option (`"Object"`), a kind neither
/// example's catalog declares — so the dialog could not add a single real object kind. Both forms must
/// now offer exactly the declared examples' own catalog rows, bounded by
/// [`PUZZLE3D_OBJECT_KIND_OPTIONS_MAX`], with a default that IS one of them.
#[semio_framework_async_macros::async_test]
async fn the_add_object_dialog_offers_every_object_kind_of_both_examples() {
    use semio_framework_plugin::ArgSchema;
    let definition = create_puzzle3d_app();
    let expected: Vec<String> = [&*CONCRETE_FOREST_EXAMPLE_FIXTURE, &*NAKAGIN_EXAMPLE_FIXTURE]
        .into_iter()
        .flat_map(|fixture| puzzle3d_kind_ids(fixture, "objects"))
        .fold(Vec::new(), |mut ids, id| {
            if !ids.contains(&id) {
                ids.push(id);
            }
            ids
        });
    assert!(expected.len() >= 2, "both shipped examples must declare object kinds, got {expected:?}");
    let dialog = definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
    let standalone = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "addObjectKind").expect("addObjectKind declared").args.clone();
    for (surface, args) in [("dialog", dialog.args.clone()), ("action", standalone)] {
        let arg = args.iter().find(|arg| arg.id == "objectKind").unwrap_or_else(|| panic!("{surface} declares an objectKind arg"));
        let ArgSchema::String { options, .. } = &arg.schema else { panic!("{surface}'s objectKind must stay a select") };
        let values: Vec<String> = options.iter().map(|option| option.value.clone()).collect();
        assert_eq!(values, expected, "{surface} must offer the live catalog's object kinds");
        assert!(values.len() <= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX, "{surface} exceeded its fixed option ceiling");
        let default = arg.default.as_ref().and_then(dsl::DslValue::as_str).unwrap_or_default().to_string();
        assert!(values.contains(&default), "{surface}'s default {default} is not one of its own options");
    }
    assert!(dialog.args.iter().all(|arg| arg.required), "the dialog's kind select stays required");
    eprintln!("[DEBUG] add-object kinds={expected:?}");
}

/// 🗣️ `📓️2026-09-09-user-feature-checklist.md` §14/summary #10: the engagement input advertised
/// `clear`/`rectangle`/`lasso` while the parser silently dropped all three. The placeholder is now
/// DERIVED from the verb list, and every verb in it must do something observable: the three marquee
/// verbs move the viewport's `selection.method`, and `clear` empties the framework-owned domain.
#[semio_framework_async_macros::async_test]
async fn every_advertised_engagement_verb_is_implemented() {
    use crate::editor::puzzle3d::commands::engagement_submit::PUZZLE3D_ENGAGEMENT_VERBS;
    let mut app = app().await;
    let envelope = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    let placeholder = main::engagement(&envelope, &Puzzle3dLabels::NATIVE_EN).input.and_then(|input| input.placeholder).unwrap_or_default();
    for verb in PUZZLE3D_ENGAGEMENT_VERBS {
        assert!(placeholder.contains(verb), "the engagement placeholder must advertise {verb}: {placeholder}");
    }
    let method_of = |node: &Value| selection_of(node).get("method").and_then(Value::as_str).unwrap_or_default().to_string();
    assert_eq!(method_of(&render_composite(&mut app).await), PUZZLE3D_SELECTION_METHOD_PICK, "a fresh window sweeps with the pick default");
    for method in [PUZZLE3D_SELECTION_METHOD_RECTANGLE, PUZZLE3D_SELECTION_METHOD_LASSO, PUZZLE3D_SELECTION_METHOD_PICK] {
        dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": method })), None).await.unwrap_or_else(|error| panic!("engagementSubmit {method}: {error:?}"));
        let rendered = render_composite(&mut app).await;
        assert_eq!(method_of(&rendered), method, "typing {method} must move the viewport marquee method");
    }
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let selected = |node: &Value| selection_of(node).get("ids").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
    assert_eq!(selected(&render_composite(&mut app).await), 1, "the object must be selected before clear");
    dispatch(&mut app, "engagementSubmit", Some(&json!({ "value": "clear" })), None).await.expect("engagementSubmit clear");
    assert_eq!(selected(&render_composite(&mut app).await), 0, "typing clear must empty the framework-owned selection");
    eprintln!("[DEBUG] engagement verbs={PUZZLE3D_ENGAGEMENT_VERBS:?}");
}

/// 🧯️ `📓️2026-09-09-user-feature-checklist.md` §9/§13/summary #13: a placement that produces nothing
/// used to produce NOTHING — no fault, no notice, no visible change. Every refusal path of the two
/// placement verbs must now carry exactly one `Effect::Notify`, and a SUCCESSFUL placement must carry
/// none (a notice on the happy path would be noise, and would hide the real ones).
#[semio_framework_async_macros::async_test]
async fn a_refused_placement_surfaces_exactly_one_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result
            .requested_effects
            .iter()
            .filter_map(|effect| match effect {
                Effect::Notify { message } => Some(message.clone()),
                _ => None,
            })
            .collect()
    };
    let mut app = app().await;
    // 🎯️ Nothing selected, no popup open, no `fullId` — the accept has no target at all.
    let orphan = dispatch(&mut app, "acceptSuggestion", None, None).await.expect("acceptSuggestion without a target still completes");
    assert_eq!(notices(&orphan).len(), 1, "an accept with no target must say so: {:?}", orphan.requested_effects);
    // 🎯️ A brush placement naming a vortex no object owns, on a kind the catalog does not declare.
    let unknown = dispatch(
        &mut app,
        "addBrushObject",
        Some(&json!({ "targetVortexFullId": "no-such-object:v0", "objectKindId": "no-such-kind", "sourceVortexIndex": 0, "origin": [0.0, 0.0, 0.0], "orientation": [0.0, 0.0, 0.0, 1.0] })),
        None,
    )
    .await
    .expect("addBrushObject on an unknown kind still completes");
    assert_eq!(notices(&unknown).len(), 1, "a brush placement onto an undeclared kind must say so: {:?}", unknown.requested_effects);
    for message in notices(&orphan).into_iter().chain(notices(&unknown)) {
        assert_ne!(message, PUZZLE3D_LOCALIZATION_UNSUPPORTED, "the test host declares an authored locale/terminology axis, so the notice must be real prose");
        assert!(!message.is_empty(), "a notice must carry text");
    }
    // 🎯️ The happy path stays quiet.
    let placed = dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    assert!(notices(&placed).is_empty(), "a successful action must not raise a notice: {:?}", placed.requested_effects);
    eprintln!("[DEBUG] refusal notices orphan={:?} unknown={:?}", notices(&orphan), notices(&unknown));
}

/// 🙈️ `📓️2026-09-09-user-feature-checklist.md` §16 (re-checked for W-D4's §17 law): the inspection
/// panel's `patchInspector` flag rows must carry the INVERSE of the row's current state, so an object
/// hidden from here can be un-hidden from here. Driven end to end — hide through the panel's own
/// dispatch shape, then re-read the panel and show again.
#[semio_framework_async_macros::async_test]
async fn inspection_flag_rows_toggle_back_off() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("addObjectKind");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select the object");
    let hidden_of = |app: &Puzzle3dApp, id: &str| {
        projection_of(app)
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(id)).cloned())
            .and_then(|object| object.get("hidden").and_then(Value::as_bool))
            .unwrap_or(false)
    };
    for expected in [true, false] {
        let panel = to_json_string(&render_body(&mut app, inspection::BODY_KEY).await);
        assert!(panel.contains("puzzle3d-play-inspector.object.hidden"), "the inspector must render the object's hidden row: {panel}");
        dispatch(&mut app, "patchInspector", Some(&json!({ "entity": PUZZLE3D_GRANULARITY_OBJECT, "field": "hidden", "ids": [object_id.as_str()], "value": expected })), None).await.expect("patchInspector hidden");
        assert_eq!(hidden_of(&app, &object_id), expected, "the inspector's hidden row must reach {expected}");
    }
}


fn first_target_volume_id(app: &Puzzle3dApp) -> String {
    let projection = projection_of(app);
    projection
        .get("targetVolumes")
        .or_else(|| projection.get("target_volumes"))
        .and_then(Value::as_array)
        .and_then(|volumes| volumes.first())
        .and_then(|volume| volume.get("id").and_then(Value::as_str))
        .expect("a target volume is present")
        .to_string()
}

fn volume_origin(app: &Puzzle3dApp, volume_id: &str) -> Vec<f64> {
    let projection = projection_of(app);
    let volumes = projection.get("targetVolumes").or_else(|| projection.get("target_volumes")).and_then(Value::as_array).cloned().unwrap_or_default();
    volumes
        .iter()
        .find(|volume| volume.get("id").and_then(Value::as_str) == Some(volume_id))
        .and_then(|volume| volume.get("origin").or_else(|| volume.get("position")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()))
        .unwrap_or_default()
}

#[semio_framework_async_macros::async_test]
async fn relocate_target_volume_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("addTargetVolume");
    let volume_id = first_target_volume_id(&app);
    let start = volume_origin(&app, &volume_id);
    dispatch(&mut app, "relocateTargetVolume", Some(&json!({ "volumeId": volume_id, "after": { "position": [4.0, 5.0, 6.0] } })), None).await.expect("relocate");
    let moved = volume_origin(&app, &volume_id);
    assert!((moved[0] - 4.0).abs() < 1e-9 && (moved[1] - 5.0).abs() < 1e-9 && (moved[2] - 6.0).abs() < 1e-9, "relocateTargetVolume must write the after pose, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(volume_origin(&app, &volume_id), start, "undo restores the volume pose");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(volume_origin(&app, &volume_id), moved, "redo reapplies the volume pose");
}

#[semio_framework_async_macros::async_test]
async fn world_relocate_undoes_and_redoes_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [9.0, 8.0, 7.0] })), None).await.expect("worldRelocate");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 9.0).abs() < 1e-9 && (moved[1] - 8.0).abs() < 1e-9 && (moved[2] - 7.0).abs() < 1e-9, "worldRelocate must write the world position, got {moved:?} from {start:?}");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_origin(&app, &object_id), start, "undo restores the object origin");
    dispatch(&mut app, "redo", None, None).await.expect("redo");
    assert_eq!(object_origin(&app, &object_id), moved, "redo reapplies the object origin");
}


/// 🚚️ Wave W-Y: Nakagin-scale `worldRelocate` of one object must admit and complete inside the
/// command work budget — the extent is the true per-item vortex walk, not `objects + attractions`.
#[semio_framework_async_macros::async_test]
async fn world_relocate_on_nakagin_admits_and_completes() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin");
    let object_id = first_object_id(&app);
    let start = object_origin(&app, &object_id);
    dispatch(&mut app, "worldRelocate", Some(&json!({ "objectId": object_id, "position": [4.0, 5.0, 6.0] })), None).await.expect("worldRelocate nakagin");
    let moved = object_origin(&app, &object_id);
    assert!((moved[0] - 4.0).abs() < 1e-6 && (moved[1] - 5.0).abs() < 1e-6 && (moved[2] - 6.0).abs() < 1e-6, "nakagin relocate must land, got {moved:?} from {start:?}");
}

/// 📋️ Wave W-Y: copy then paste clones the selection with new ids as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn copy_then_paste_clones_selection_as_one_mutation() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let copied = dispatch(&mut app, "copy", None, None).await.expect("copy");
    let fragment = copied.requested_effects.iter().find_map(|effect| match effect {
        Effect::ClipboardWrite { fragment } => Some(fragment.clone()),
        _ => None,
    }).expect("copy must emit ClipboardWrite");
    let before = object_count(&app);
    let paste_args = json!({ "fragment": json::from_dsl_value(&dsl::ToValue::to_value(&fragment)) });
    let pasted = dispatch(&mut app, "paste", Some(&paste_args), None).await.expect("paste");
    assert!(!pasted.mutations.is_empty(), "paste must emit one mutation edit: {:?}", pasted.mutations);
    assert_eq!(object_count(&app), before + 1, "paste clones the selection");
    let ids: Vec<String> = projection_of(&app).get("objects").and_then(Value::as_array).unwrap().iter().filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string)).collect();
    assert!(ids.iter().any(|id| id != &object_id), "the clone must carry a new id: {ids:?}");
}

/// ✂️ Wave W-Y: cut is copy + delete as one undo step.
#[semio_framework_async_macros::async_test]
async fn cut_undoes_as_one_step() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add");
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    let before = object_count(&app);
    dispatch(&mut app, "cut", None, None).await.expect("cut");
    assert_eq!(object_count(&app), before - 1, "cut removes the selection");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), before, "one undo restores the cut");
}

/// ⬇️ Wave W-Y: export carries the full round-trippable fixture JSON.
#[semio_framework_async_macros::async_test]
async fn export_fixture_downloads_round_trippable_json() {
    let mut app = app().await;
    let before = projection_of(&app);
    let result = dispatch(&mut app, "exportFixture", None, None).await.expect("export");
    let data = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::DownloadMediaExport { filename, mime_type, data, .. } => {
            assert_eq!(filename, "puzzle-3d.json");
            assert_eq!(mime_type, "application/json");
            Some(data.clone())
        }
        _ => None,
    }).expect("export must emit DownloadMediaExport");
    let exported: Value = parse(&data).expect("export JSON parses");
    assert_eq!(exported.get("schema"), before.get("schema"), "export schema must match the live fixture");
    assert_eq!(exported.get("objects").and_then(Value::as_array).map(Vec::len), before.get("objects").and_then(Value::as_array).map(Vec::len));
}

/// 🧬️ Identity of imported objects without fixture/snapshot projection twins (anchor / null scale).
fn object_cores(value: &Value) -> Vec<(String, String, String, Value, Vec<String>)> {
    value
        .get("objects")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|object| {
            let id = object.get("id").and_then(Value::as_str)?.to_string();
            let kind = object.get("objectKind").and_then(Value::as_str).unwrap_or_default().to_string();
            let mesh = object.get("meshUrl").and_then(Value::as_str).unwrap_or_default().to_string();
            let origin = object.get("origin").cloned().unwrap_or(Value::Null);
            let vortices = object
                .get("vortices")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|vortex| vortex.get("id").and_then(Value::as_str).map(str::to_string))
                .collect();
            Some((id, kind, mesh, origin, vortices))
        })
        .collect()
}

/// 📥️ Wave W-Y: importing that JSON reproduces the document as one Mutation edit.
#[semio_framework_async_macros::async_test]
async fn import_fixture_reproduces_the_exported_document() {
    let mut app = app().await;
    let source = projection_of(&app);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let imported = dispatch(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&source) })), None).await.expect("import");
    assert!(imported.history_patch.is_some(), "import must be one mutation edit");
    assert_eq!(object_cores(&projection_of(&app)), object_cores(&source), "import reproduces the exported objects");
    dispatch(&mut app, "undo", None, None).await.expect("undo");
    assert_eq!(object_count(&app), 0, "one undo restores the empty document");
}

/// 📥️ Wave W-AB: workspace Import is `openImportFixture` (file picker). `importFixture` stays
/// dispatchable for the host re-dispatch after the pick, but is not a file-menu row.
#[test]
fn file_menu_import_row_opens_the_file_picker() {
    let definition = create_puzzle3d_app();
    let file: Vec<(&str, bool)> = definition
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter())
        .filter(|action| action.category.as_deref() == Some("file"))
        .map(|action| (action.id.as_str(), action.in_palette))
        .collect();
    assert!(file.iter().any(|(id, _)| *id == "exportFixture"), "file menu keeps Export: {file:?}");
    assert!(file.iter().any(|(id, in_palette)| *id == "openImportFixture" && *in_palette), "file menu Import is openImportFixture: {file:?}");
    assert!(!file.iter().any(|(id, _)| *id == "importFixture"), "importFixture is the picker completion, not a menu row: {file:?}");
    let import = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "importFixture").expect("importFixture stays dispatchable");
    assert!(!import.in_palette);
    assert_eq!(import.category.as_deref(), None);
}

/// 📥️ Wave W-AB: activating Import requests a file open; completing it with fixture JSON imports.
#[semio_framework_async_macros::async_test]
async fn open_import_fixture_requests_file_open_then_import_applies_payload() {
    let mut app = app().await;
    let source = projection_of(&app);
    let opened = dispatch(&mut app, "openImportFixture", None, None).await.expect("openImportFixture");
    let req = opened.requested_effects.iter().find_map(|effect| match effect {
        Effect::RequestFileOpen { accept, read_as, import_action, multiple, .. } => {
            assert!(accept.contains("json"), "picker accepts JSON: {accept}");
            assert_eq!(read_as.as_deref(), Some("text"));
            assert_eq!(import_action, "importFixture");
            assert!(!*multiple);
            Some(())
        }
        _ => None,
    });
    assert!(req.is_some(), "Import must emit RequestFileOpen: {:?}", opened.requested_effects);
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    assert_eq!(object_count(&app), 0);
    let imported = dispatch(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&source) })), None).await.expect("import");
    assert!(imported.history_patch.is_some(), "import must be one mutation edit");
    assert_eq!(object_cores(&projection_of(&app)), object_cores(&source), "picked payload reproduces the exported objects");
}

/// 📥️ Wave W-AB: re-importing the live fixture is a store identity, not a guest payload dedupe.
/// `import_fixture` always assigns; a no-op history row means the document fold saw equal content.
#[semio_framework_async_macros::async_test]
async fn import_fixture_of_the_live_document_records_whether_identical_content_is_an_edit() {
    let mut app = app().await;
    let source = projection_of(&app);
    let imported = dispatch(&mut app, "importFixture", Some(&json!({ "payload": to_json_string(&source) })), None).await.expect("reimport");
    let objects_after = object_cores(&projection_of(&app));
    assert_eq!(objects_after, object_cores(&source), "identical payload must not rewrite object cores");
    assert!(imported.history_patch.is_none(), "identical live fixture is a store no-op, not a guest dedupe: ingress still delivers payload+name");
}

/// 🖱️ Wave W-Y: a selected-object context menu is puzzle-owned — never the shell fallback vocabulary.
#[semio_framework_async_macros::async_test]
async fn object_context_menu_owns_puzzle_rows_not_shell_fallback() {
    let mut app = app().await;
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("add");
    let object_id = first_object_id(&app);
    let menu = context_menu_for_selection(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await;
    let menu_json = to_json_string(&menu);
    assert!(menu_json.contains("duplicateSelection"), "object menu must own Duplicate: {menu_json}");
    assert!(menu_json.contains("selectSameKindSelection"), "object menu must own Select same kind: {menu_json}");
    assert!(!menu_json.contains("setActiveExample"), "object menu must not dress itself as the shell fallback: {menu_json}");
}

/// 📏️ Wave W-Y: gumball scale on a locked volume refuses with a notice and emits no edit.
#[semio_framework_async_macros::async_test]
async fn gumball_scale_on_locked_volume_refuses_without_edit() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), None).await.expect("volume");
    let volume_id = projection_of(&app).get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
    dispatch(&mut app, "setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "locked", "value": true })), None).await.expect("lock");
    select_id(&mut app, PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume_id).await.expect("select volume");
    let before = projection_of(&app);
    let result = dispatch(&mut app, "scaleSelection", Some(&json!({ "sx": 2.0, "sy": 2.0, "sz": 2.0 })), None).await.expect("scale locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked volume must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked volume must emit no edit: {:?}", result.mutations);
    assert_eq!(projection_of(&app).get("targetVolumes"), before.get("targetVolumes"));
}

/// 📋️ leftover vortex-granularity selection still captures the object, and paste clones it with a new id.
#[test]
fn leftover_copy_paste_clones_selected_object() {
    let mut fixture = empty_fixture();
    fixture.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: Vec::new(),
        hidden: false,
        locked: false,
        reveal_index: None,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["seed-left-001".into()],
        hovered: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &fixture);
    assert_eq!(objects.len(), 1, "leftover selected object id must copy even when granularity is vortex: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    assert_eq!(objects[0].origin, [1.0, 2.0, 3.0]);
    let fragment = puzzle3d_copy_fragment_from(&fixture, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations_on(&fixture, &fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "paste must commit one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001", "clone must mint a fresh id");
    assert!((created[0].object.origin[0] - 1.5).abs() < 1e-9, "clone origin must keep source fields plus paste offset: {:?}", created[0].object.origin);
    assert_eq!(created[0].object.object_kind.as_deref(), Some("Object"));
    assert_eq!(created[0].object.label.as_deref(), Some("seed"));
}

/// 📋️ leftover selected vortex uuid (not object.id) still captures the parent object for copy.
#[test]
fn leftover_copy_paste_clones_object_from_selected_vortex_uuid() {
    let mut fixture = empty_fixture();
    fixture.objects.push(Puzzle3dObject {
        id: "seed-left-001".into(),
        label: Some("seed".into()),
        object_kind: Some("Object".into()),
        origin: [1.0, 2.0, 3.0],
        orientation: None,
        scale: None,
        mesh_url: None,
        vortices: vec![Puzzle3dVortex {
            id: "5de35caa-0f02-43d7-ae74-aa730efd3386".into(),
            vortex_kind: None,
            position: [0.0, 0.0, 0.0],
            direction: None,
            radius: None,
            hidden: false,
            locked: false,
        }],
        hidden: false,
        locked: false,
        reveal_index: None,
    });
    let marks = Puzzle3dInteractionSnapshot {
        granularity: PUZZLE3D_GRANULARITY_VORTEX.into(),
        selected: vec!["5de35caa-0f02-43d7-ae74-aa730efd3386".into()],
        hovered: Vec::new(),
    };
    let objects = puzzle3d_selected_objects_from(&marks, &fixture);
    assert_eq!(objects.len(), 1, "leftover selected vortex uuid must resolve to the parent object: {objects:?}");
    assert_eq!(objects[0].id, "seed-left-001");
    let fragment = puzzle3d_copy_fragment_from(&fixture, objects).expect("copy fragment");
    let mutations = puzzle3d_paste_operations_on(&fixture, &fragment, &semio_framework_plugin::kernel::PastePlacement::default()).expect("paste");
    let created: Vec<_> = mutations
        .iter()
        .filter_map(|op| match op {
            Puzzle3dMutation::CreateObject(create) => Some(create),
            _ => None,
        })
        .collect();
    assert_eq!(created.len(), 1, "vortex-uuid leftover copy must paste one create-object, got {mutations:?}");
    assert_ne!(created[0].object.id, "seed-left-001");
}

/// 🕹️ leftover overlay → translateSelection on a locked object must refuse with a visible notice.
#[semio_framework_async_macros::async_test]
async fn leftover_translate_selection_on_locked_object_refuses_with_notice() {
    let notices = |result: &semio_framework_plugin::InvocationResult| -> Vec<String> {
        result.requested_effects.iter().filter_map(|effect| match effect {
            Effect::Notify { message } => Some(message.clone()),
            _ => None,
        }).collect()
    };
    let mut app = app().await;
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("select");
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "entity": "object", "ids": [object_id.as_str()], "flag": "locked", "value": true })), None).await.expect("lock");
    let before = object_origin_x(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("translate locked");
    let raised = notices(&result);
    assert_eq!(raised.len(), 1, "locked object must raise exactly one notice: {:?}", result.requested_effects);
    assert_ne!(raised[0], PUZZLE3D_LOCALIZATION_UNSUPPORTED);
    assert!(result.mutations.is_empty(), "locked object must emit no edit: {:?}", result.mutations);
    assert!((object_origin_x(&app, &object_id) - before).abs() < 1e-9, "locked object origin must not move");
}

/// 🕹️ leftover overlay → translateSelection carries explicit ids; guest snapshot granularity may be empty.
#[semio_framework_async_macros::async_test]
async fn leftover_overlay_translate_selection_moves_unlocked_object() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None).await.expect("empty");
    dispatch(&mut app, "addObjectKind", Some(&json!({ "objectKind": "Object" })), None).await.expect("add object");
    let object_id = first_object_id(&app);
    let before = object_origin(&app, &object_id);
    let result = dispatch(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 4.0, "dy": 0.0, "dz": 0.0 })), None).await.expect("leftover overlay translate");
    assert!(result.requested_effects.iter().all(|effect| !matches!(effect, Effect::Notify { .. })), "unlocked leftover overlay must not refuse: {:?}", result.requested_effects);
    assert!((object_origin(&app, &object_id)[0] - before[0] - 4.0).abs() < 1e-9, "leftover overlay ids must move the unlocked object by dx");
}


//#endregion 🩹️CheckedDefects

/// 📏️ Wave W-M2: one whole `registerBrushMesh` page validates inside one command's own work budget.
/// The page scan is cursorized at [`PUZZLE3D_MESH_PAGE_SCAN_CHARS`] characters per step over both
/// payload streams, and the largest page the plugin admits is
/// `PUZZLE3D_MESH_PAGE_BASE64_CHARS` characters per stream — so the extent a page claims can never
/// approach `PUZZLE_COMMAND_WORK_ITEMS`, which is what keeps every per-page unit under the 8 ms law.
#[test]
fn one_brush_mesh_page_validates_inside_one_command_work_budget() {
    let page = crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
    let steps = page.div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).saturating_mul(2).saturating_add(1);
    assert!(steps <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "a whole page validates in {steps} bounded steps, inside {}", crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);
    assert!(page + 8 < crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "a page's two payload streams — one page's values, split at most once and padded per stream — leave the JSON envelope room inside the raw wire");
}

/// 🚚️ Wave W-H: a client whose "already uploaded" bookkeeping outlived the guest that justified it
/// announces identities this instantiation holds nothing for. The arm may not answer that with a notice
/// — nothing on the host reads a notification's text, so the brush utility would silently keep no
/// collision body until a full page reload. It publishes the identity on the world body instead, and
/// widens its own otherwise-`Quiet` scope to the viewport so that body is actually republished. The
/// client answers with the page run, and the request retires.
#[semio_framework_async_macros::async_test]
async fn an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes() {
    let mut app = app().await;
    let (positions, indices) = crate::standards::v1::subsets::any::schema::testkit::unit_cube_mesh_buffers();
    let digest = crate::editor::puzzle3d::precompute::brush_mesh_digest(&positions, &indices);
    let url = "/test/restarted-guest-reannounce.glb";
    let announcement = json!({ "surfaceId": "world-3d", "url": url, "digest": digest });
    let refused = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("id-only announcement");
    assert!(!refused.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "a refusal the client cannot act on is not an answer");
    assert!(
        matches!(&refused.ui_scope, UiDirtyScope::Partial { window_bodies, .. } if window_bodies.iter().any(|body| body == main::BODY_KEY)),
        "the request is worthless until the world body carrying it repaints; got {:?}",
        refused.ui_scope
    );
    let requested = interaction_of(&render_composite(&mut app).await);
    assert_eq!(requested.pointer("/meshReuploadUrls").and_then(Value::as_array).cloned().unwrap_or_default(), vec![json!(url)], "the refused identity is published as a request for its bytes");
    let residency = requested.pointer("/meshResidency").and_then(Value::as_u64).expect("the world body publishes the guest's mesh residency");

    let position_bytes: Vec<u8> = positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    let index_bytes: Vec<u8> = indices.iter().flat_map(|value| value.to_le_bytes()).collect();
    let page = json!({
        "surfaceId": "world-3d",
        "url": url,
        "digest": digest,
        "page": 0,
        "pageCount": 1,
        "positionsB64": semio_framework_io_base64::base64_standard_encode(&position_bytes),
        "indicesB64": semio_framework_io_base64::base64_standard_encode(&index_bytes),
    });
    let accepted = dispatch(&mut app, "registerBrushMesh", Some(&page), None).await.expect("the client answers with the page run");
    assert!(!accepted.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "an accepted page run is silent");
    let settled = interaction_of(&render_composite(&mut app).await);
    assert_eq!(settled.pointer("/meshReuploadUrls").and_then(Value::as_array).map(Vec::len), Some(0), "an answered request retires, so a steady state carries no standing request");
    assert!(settled.pointer("/meshResidency").and_then(Value::as_u64).expect("residency") > residency, "the client's proof that this guest holds the geometry is the counter climbing");

    let readopted = dispatch(&mut app, "registerBrushMesh", Some(&announcement), None).await.expect("the identity is now resident");
    assert!(readopted.requested_effects.is_empty(), "an identity this guest holds is adopted by id alone, at no cost on the wire");
    assert!(matches!(readopted.ui_scope, UiDirtyScope::None), "the adopting fast path stays the quiet command it is declared to be");
    assert!(
        !puzzle3d_action_uses_precompute("registerBrushMesh"),
        "registerBrushMesh only WRITES geometry into the precompute session, so it may not owe the sync prologue: that prologue seeds a fallback body for every id the session has no geometry for — precisely the ids this command supplies — clearing the brush cache and rebuilding the queue each time, which the arm's own install then discards, and which is where the 0.65 s → 2.85 s per announcement measured on 2026-09-09 came from"
    );
}
