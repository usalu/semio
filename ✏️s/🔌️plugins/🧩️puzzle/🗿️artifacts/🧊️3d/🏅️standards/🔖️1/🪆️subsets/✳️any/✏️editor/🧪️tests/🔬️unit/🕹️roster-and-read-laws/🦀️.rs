/// 🕹️ Roster and read laws: the dispatchable action roster, the paged local interaction reads under render pressure and the interactive step budget of every maintenance unit.

//#region 🕹️ActionRoster
/// 🕹️ Every action DISPATCHABLE in this app, once per id, in declaration order.
///
/// 🪪️ App-level actions are no longer CLONED onto every window kind (that made a package descriptor
/// grow as `apps × window kinds × actions`): `semio_framework::window_kind_actions` resolves a
/// window's own roster plus every app-level row no window claims, at read time. So
/// `WindowKindDefinition::actions` alone now holds only the window's OWN declarations — for this app,
/// the framework-injected interaction verbs — and a law that reads it sees none of the authored
/// vocabulary. This is the roster the shell offers and `dispatch_action` routes.

/// 🪪️ Every DECLARATION of one action id across the manifest — the app-level roster plus every window
/// kind's own roster. A verb declared once app-wide and resolved into N windows is still exactly one
/// declaration, which is what the bijection laws mean by "exactly one manifest declaration".
fn action_declarations<'a>(definition: &'a semio_framework_plugin::AppDefinition, id: &str) -> Vec<&'a ActionDefinition> {
    definition.actions.iter().chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter())).filter(|action| action.id == id).collect()
}
//#endregion 🕹️ActionRoster

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
        let (raw, cx) = app.parts();
        raw.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("advance one local interaction publication unit");
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
    eprintln!("local interaction read {request_id} turns={turns} bytes={}", capture.len());
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
    let capture: protocol::LocalInteractionCapture = semio_framework_pack_json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("a terminated read yields a whole capture");
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
    let capture: protocol::LocalInteractionCapture = semio_framework_pack_json::from_json_str(std::str::from_utf8(&capture).expect("a capture is canonical UTF-8 JSON"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("a terminated read yields a whole capture");
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
    let (raw, cx) = app.parts();
    semio_s_plugin_puzzle_jobs::testing::maintain(raw, 4096, cx).expect("maintenance step after a returned snapshot read lease must not fault");
}
//#endregion 🕹️LocalInteractionRead

/// 🔁️ Full round-robin sweeps driven after the example lands, so every fixed stage runs its own
/// bounded unit several times (first unit does the real work, later ones prove it stays terminal).
const MAINTENANCE_UNIT_SWEEPS: usize = 8;

/// 🚧️ Runaway guard for the measured host-turn loop; the flagship example loads need thousands of
/// publication turns, so this only has to be far above them, never tight.
const MAINTENANCE_BUDGET_TURNS: usize = 1_048_576;

/// ⏱️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave R2: the OS runtime's cooperative-maintenance clock
/// (`RuntimeLiveCleanupJob::step` → `maintenance_step(1, 4096)`) faults its instance with
/// `plugin.internal.interactive-ceiling` the moment one unit overruns
/// `semio_framework_trace::INTERACTIVE_STEP_CEILING_US`, which is exactly how the browser boot died. A unit stays inside the
/// ceiling because it is bounded by its grant — one item and [`RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP`] bytes. This law drives
/// the REAL app — the real typed `setActiveExample` command, the real store, the real store-replacement/envelope registries
/// and the real fixed round robin — over both flagship documents and holds every unit of every stage to its grant, naming
/// the offending stage when it does not. The bound is a count, so the verdict is the same on an idle and a saturated
/// machine. The measured turn deliberately does NOT assert the operation's own result lane: a domain command that faults is
/// a different law's subject, while the clock must stay inside its grant either way.
#[semio_framework_async_macros::async_test]
async fn every_maintenance_unit_stays_inside_the_interactive_step_budget() {
    for example in [PUZZLE3D_EXAMPLE_CONCRETE_FOREST, PUZZLE3D_EXAMPLE_NAKAGIN] {
        let mut app = app().await;
        let command = Puzzle3dCommand::from_action("setActiveExample", Some(json!({ "exampleId": example })), None).expect("setActiveExample is a declared puzzle3d command");
        let action_meta = app.meta("local");
        let (raw, cx) = app.parts();
        raw.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("setActiveExample mints its retained whole-document operation");
        for _ in 0..MAINTENANCE_BUDGET_TURNS {
            if !app.has_pending_typed_operations() {
                break;
            }
            measured_host_turn(&mut app).await;
        }
        for _ in 0..MAINTENANCE_UNIT_SWEEPS * semio_framework_plugin::MAINTENANCE_STAGES as usize {
            app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
        }
        assert!(app.maintenance.units() > 0, "{example}: the law drove no maintenance unit");
        assert_eq!(app.maintenance.first_overrun(), None, "{example}: a maintenance unit released more than its grant as (stage, items, bytes, granted items, granted bytes) (per-stage worstItems/worstBytes/units: {})", app.maintenance.report());
    }
}

/// 🔁️ One host actor turn with its cooperative-maintenance unit measured: the same
/// `maintenance_step` → `advance_typed_operation_publication` → present/ACK → drain shape the
/// runtime drives, minus any assertion on the operation's own outcome lane.
async fn measured_host_turn(app: &mut Puzzle3dApp) {
    app.measure_maintenance_step(1, RUNTIME_LIVE_CLEANUP_BYTES_PER_STEP).expect("a live-cleanup maintenance unit never faults");
    let (raw, cx) = app.parts();
    raw.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await.expect("advance one typed operation publication unit");
    let instance_id = app.instance_id;
    if let Some(page) = app.take_typed_operation_result_page(instance_id) {
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
