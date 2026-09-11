//#region 🪟️SurfaceViewStateRouting
/// 🪟️ A registry whose ONE interaction domain is `HierarchyProvider::Flat` — deliberately not
/// `interaction_registry`'s `Topology` twin: a `Flat` domain is omitted from
/// `build_full_interaction_topology`'s pruning map, so the pick this module dispatches survives
/// `validate_state` without seeding a document label first, and this law measures the surface route
/// instead of a typed-command factory proof.
async fn surface_routing_registry() -> AppActionRegistry {
    let app = App::from_builder(
        App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .interaction(InteractionDefinition {
                id: "items".into(),
                label: LocalizedLabel::data("Items"),
                granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            })
            .await
            .window_kind_interactions("main", vec![InteractionRef::new("items")])
            .await,
    )
    .await;
    AppActionRegistry::from_definition(&app.definition)
}

/// 🪟️ Materializes ONE surface's mounted host view exactly as the shell builds it — the shared base
/// context of the refresh generation that mounted it, overlaid with that generation's own fields.
/// Mirrors `windowViewContext`/`panelViewContext` (`🛂️manifest/🟦️.ts`).
fn surface_routing_view(base: &Value, overrides: &Value) -> ViewModel {
    let mut merged = base.clone();
    for (key, value) in overrides.as_object().expect("surface view overrides") {
        merged[key] = value.clone();
    }
    serde_json::from_value(merged).expect("surface view context")
}

/// 🎯️ The projection a mounted surface MUST be rendered against: its own window instance, or the
/// panel projection when it names no window.
fn surface_routing_projection(view: &ViewModel) -> ViewModel {
    match view.window_id.as_deref() {
        Some(window) => view.for_window_instance(window).expect("mounted window instance"),
        None => view.for_panel(),
    }
}

async fn surface_routing_render(runtime: &super::PluginRuntime<VcsArtifactApp<TestApp>>, surface: &str, expected: &ViewModel, node_key: &str) {
    let (_, presence) = super::plugin_render_surface(runtime, 1, surface).await.expect("mounted surface renders");
    let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take()).expect("the app body was rendered");
    assert_eq!(
        serde_json::to_value(&actual).unwrap(),
        serde_json::to_value(expected).unwrap(),
        "surface {surface} (body {body}) was rendered against another surface's view state"
    );
    assert!(presence.iter().all(|update| update.surface.as_ref() == surface), "surface {surface} published presence addressed at another surface");
    assert!(presence.iter().any(|update| update.node_key == node_key && update.own.selected), "the pick must reach surface {surface}'s own render");
}

/// 🪟️ THE route the browser renders every body through and no law ever covered:
/// `Event::SurfaceVisible` → `plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`.
/// Mounts the four surfaces a split puzzle3d session mounts — two panes of the SAME window kind, the
/// Inspection panel, and the leftover `1:window` alias bound to the last pane
/// (`DEFAULT_LEFTOVER_WINDOW_SURFACE`, `🔌️PluginRuntime/🟦️.tsx`) — each with the host view of the
/// refresh generation that mounted it, dispatches one browser-shaped `interactionSelect`, and then
/// renders every body.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B25: `SurfaceContexts` kept ONE `view_state` that
/// every mount overwrote, so a body's `ViewModel` was rebuilt from whichever sibling surface mounted
/// LAST — the Inspection panel rendered against the last pane's projection and the first pane
/// rendered against the alias's. B23 §3 named this as the one hop no testkit render helper
/// (`render_body`/`render_panel_body`/`render_window_refresh`/`render_window`/`render_composite`)
/// can reach, because every one of them calls `PluginApp::render` with a hand-built `ViewModel`.
#[semio_framework_async_macros::async_test]
async fn every_mounted_surface_renders_against_its_own_view_state_while_one_pick_reaches_every_body() {
    let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧫️fixtures/🪟️surface-view-state-routing/🔣️.json")).expect("surface view-state routing fixture");
    let node_key = fixture["pickedNodeKey"].as_str().expect("picked node");
    let domain = fixture["domainId"].as_str().expect("domain");
    let mut app = VcsArtifactApp::with_registry(TestApp::<false>::default(), surface_routing_registry().await).await;
    reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": domain, "merge": "replace", "method": "pick" }), node_key))).await;
    assert_eq!(app.interaction_state().await.selection.get(domain).map(|selection| selection.ids.clone()), Some(vec![node_key.to_string()]), "the browser-shaped pick must be persisted before any surface renders");
    // 🕹️ The pick's own stamping pass already queued its presence; draining it here makes every
    // assertion below about the presence THAT surface's render produced.
    drop(PluginApp::take_pending_presence(&mut app).await);
    // 🧹 The reserved pick parks its worker-session retirement PROCESS-globally (maintenance stage
    // 23); left behind it pollutes the heap `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`
    // measures — the same rule `close_fixture_app_to_terminal_emptiness` states for every fixture app.
    for _ in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
        if !semio_framework_job::worker_job_retirements_are_parked() {
            break;
        }
        drop(PluginApp::maintenance_step(&mut app, 1, TYPED_OPERATION_RESULT_PAGE_BYTES).expect("bounded maintenance"));
        crate::app::plugin_job_yield_once().await;
    }

    let runtime = super::PluginRuntime::new();
    runtime.instances.borrow_mut().insert_admitted(1, std::sync::Arc::new(super::RuntimeAppCell::new(AppInstance { id: 1, app, surface_contexts: Default::default() })));
    let surfaces = fixture["surfaces"].as_array().expect("surfaces");
    let mut expected: BTreeMap<String, ViewModel> = BTreeMap::new();
    for surface in surfaces {
        let id = surface["id"].as_str().expect("surface id");
        let mounted = surface_routing_view(&fixture["base"], &surface["view"]);
        super::plugin_mount_surface(&runtime, 1, id.into(), surface["bodyKey"].as_str().expect("body key").into(), &super::encode_wire_serialized(&mounted)).await.expect("surface mounts");
        expected.insert(id.into(), surface_routing_projection(&mounted));
    }

    for surface in surfaces {
        surface_routing_render(&runtime, surface["id"].as_str().expect("surface id"), &expected[surface["id"].as_str().expect("surface id")], node_key).await;
    }

    let alias = fixture["aliasSurface"].as_str().expect("alias surface");
    let aliased_window = fixture["aliasWindowSurface"].as_str().expect("aliased window surface");
    assert_eq!(serde_json::to_value(&expected[alias]).unwrap(), serde_json::to_value(&expected[aliased_window]).unwrap(), "the leftover window alias resolves to the window it was bound to");
    surface_routing_render(&runtime, fixture["pickedSurface"].as_str().expect("picked surface"), &expected[fixture["pickedSurface"].as_str().expect("picked surface")], node_key).await;
    surface_routing_render(&runtime, fixture["inspectionSurface"].as_str().expect("inspection surface"), &expected[fixture["inspectionSurface"].as_str().expect("inspection surface")], node_key).await;

    let refreshed = surface_routing_view(&fixture["base"], &fixture["refreshedView"]);
    super::plugin_render(&runtime, 1, "graph", &serde_json::to_string(&refreshed).expect("host view serializes")).await.expect("host refresh renders");
    RENDER_CONTEXT_PROBE.with(|probe| probe.take());
    for surface in surfaces {
        let id = surface["id"].as_str().expect("surface id");
        let rebound = match expected[id].window_id.as_deref() {
            Some(window) => refreshed.for_window_instance(window).expect("live window instance"),
            None => refreshed.for_panel(),
        };
        surface_routing_render(&runtime, id, &rebound, node_key).await;
    }
    eprintln!("[DEBUG] four mounted surfaces each rendered against their own view state, one pick reached every body, and a host refresh rebound every surface without losing its identity");
}
//#endregion 🪟️SurfaceViewStateRouting
