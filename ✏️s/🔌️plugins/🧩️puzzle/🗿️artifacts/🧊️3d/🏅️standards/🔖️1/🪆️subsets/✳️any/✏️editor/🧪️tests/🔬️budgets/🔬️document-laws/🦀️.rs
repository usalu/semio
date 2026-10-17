/// 🔬️ Document laws: the rendered versus the mutated document, the shipped kind catalog, the initial snapshot cost and the host twin.

//#region 🔬️B23RenderedVsMutatedDocument
/// 🔬️ Wave B23 — the browser's own sequence, on the two-pane session the browser boots: pick the
/// object, read Inspection through BOTH host render routes (the panel route, which carries no
/// `window_id` at all, and the window-instance route), lock the picked selection through the
/// context-menu path (no explicit ids — the arm reads the selection the render just painted), then
/// translate it and read the world lane the pane publishes.
///
/// 🕳️ Every failing verdict of the `#48` selection-scoped family is one row of this law
/// (`inspection-object-fields id=null`, `locked-flag-row lockChrome=false`,
/// `gumball-scene-delta sceneDelta=false`, `relocate-pose-delta`), and the law is GREEN — the guest
/// keeps the pick across both routes and both mutating dispatches, including the
/// `revalidate_interaction_on_document_change` pass every artifact-intent dispatch runs.
/// What the browser has and this law cannot reach is the surface-context route
/// (`plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`): every test context render
/// helper calls `PluginApp::render` directly with a hand-built `ViewModel`, so a body that is never
/// re-rendered — or re-rendered against another surface's retained view — is invisible here.
#[semio_framework_async_macros::async_test]
async fn a_browser_shaped_pick_survives_every_render_route_and_both_mutating_dispatches() {
    let mut app = app().await;
    for pane in [main::WINDOW_INSTANCE_TOP, main::WINDOW_INSTANCE_PERSPECTIVE] {
        drop(render_window(&mut app, pane).await);
    }
    let object_id = first_object_id(&app);
    select_id(&mut app, PUZZLE3D_GRANULARITY_OBJECT, &object_id).await.expect("browser-shaped pick");
    let panel_route = render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string();
    assert!(!panel_route.contains("puzzle3d-play-inspector.empty"), "the panel route — the only one a real Inspection refresh uses — must see the pick: {panel_route}");
    assert!(panel_route.contains(object_id.as_str()), "the panel route must name the picked object: {panel_route}");
    assert!(panel_route.contains("puzzle3d-play-inspector.object.locked"), "the panel route must assemble the lock flag_row the context menu drives: {panel_route}");
    let window_route = render_body(&mut app, inspection::BODY_KEY).await.to_string();
    assert!(!window_route.contains("puzzle3d-play-inspector.empty"), "the window-instance route must see the same pick: {window_route}");

    let before = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    dispatch(&mut app, "setSelectionFlag", Some(&json!({ "flag": "locked", "value": true })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("lock the picked object");
    assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "the context-menu lock path must land on the object the pick named");
    dispatch(&mut app, "translateSelection", Some(&json!({ "dx": 3.0, "dy": 0.0, "dz": 0.0 })), Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.expect("translate the picked object");
    let after = render_window_refresh(&mut app, main::BODY_KEY, main::WINDOW_INSTANCE_PERSPECTIVE).await.to_string();
    assert_ne!(before, after, "a translate taken on the SECOND mutating dispatch after the pick must still change the world lane the pane renders");
    assert!(
        !render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_INSTANCE_PERSPECTIVE)).await.to_string().contains("puzzle3d-play-inspector.empty"),
        "two artifact-intent dispatches run two `revalidate_interaction_on_document_change` passes; neither may eat the pick"
    );
}

fn object_flag(app: &Puzzle3dApp, object_id: &str, flag: &str) -> Option<bool> {
    projection_of(app)
        .get("objects")
        .and_then(Value::as_array)
        .and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id)).cloned())
        .and_then(|object| object.get(flag).and_then(Value::as_bool))
}
//#endregion 🔬️B23RenderedVsMutatedDocument

//#region 🔖️Pz2ShippedKindCatalog
/// 🗂️ The authored [`PUZZLE3D_SHIPPED_OBJECT_KINDS`] IS the two shipped examples' own
/// `meta.kindCatalogs` rows, in their own order — the law that lets the `objectKind` select be a
/// `const` instead of a parse of 136 395 B of DSL on the `describe()` path (slice PZ2, 2026-09-22).
/// It pays that parse HERE, once, and fails the moment an example's catalog changes without the
/// authored list following.
#[semio_framework_async_macros::async_test]
async fn shipped_object_kinds_are_the_two_examples_own_catalog_rows() {
    let mut derived: Vec<(String, String)> = Vec::new();
    for scene_snapshot in [&*CONCRETE_FOREST_EXAMPLE_SNAPSHOT, &*NAKAGIN_EXAMPLE_SNAPSHOT] {
        for entry in puzzle3d_catalog_entries(scene_snapshot, "objects") {
            if derived.len() >= PUZZLE3D_OBJECT_KIND_OPTIONS_MAX {
                break;
            }
            let Some(id) = entry.get("id").and_then(semio_framework_value::DslValue::as_str) else {
                continue;
            };
            if derived.iter().any(|(existing, _)| existing == id) {
                continue;
            }
            derived.push((id.to_string(), catalogue::catalog_entry_label(entry)));
        }
    }
    let authored: Vec<(String, String)> = PUZZLE3D_SHIPPED_OBJECT_KINDS.iter().map(|(id, label)| ((*id).to_string(), (*label).to_string())).collect();
    assert_eq!(authored, derived, "PUZZLE3D_SHIPPED_OBJECT_KINDS drifted from the shipped documents — re-author it from `concrete-forest` then `nakagin-capsule-tower`");
}
//#endregion 🔖️Pz2ShippedKindCatalog

//#region 🔖️InitialSnapshotCost
/// 🚀️ LAW (ticket 26/09/23, H12): the initial snapshot costs its document and nothing else. Every construction of
/// this app pays for it — each mount, and every `codec` call a hub makes to create or validate a document, where
/// the interpreter multiplies it — so it must never build a scene, sync a precompute session or parse an example
/// the document does not read. Weighed by this thread's heap peak (`HeapWitness`), deterministic and independent
/// of other laws: the snapshot may peak no higher than converting the default scene_snapshot itself, within 4 KiB.
#[test]
fn the_initial_snapshot_costs_only_its_document() {
    let _ = default_scene_snapshot();
    let peak_of = |work: &dyn Fn()| {
        let before = semio_framework_trace::retained_heap_bytes_on_this_thread();
        semio_framework_trace::reset_heap_peak_on_this_thread();
        work();
        semio_framework_trace::peak_heap_bytes_on_this_thread() - before
    };
    let document = peak_of(&|| drop(Puzzle3dPlaySnapshot::new(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("typed fixture admits"))));
    let snapshot = peak_of(&|| drop(<Puzzle3dPlayApp as ArtifactEditor>::initial_snapshot()));
    assert!(document > 0, "the witness weighs the document conversion: {document}");
    assert!(snapshot <= document + 4096, "the initial snapshot peaked at {snapshot} B, its document at {document} B: it does work the document never reads");
}
//#endregion 🔖️InitialSnapshotCost

/// 🛡️ The host twin carries every object of the typed document and refuses a scale the typed record cannot admit.
#[test]
fn puzzle3d_host_twin_round_trips_its_vocabulary_and_refuses_bad_scale() {
    let document=semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("authored fixture admits");
    let mut host=puzzle3d_scene_snapshot_from_document(&document);
    let admitted=semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&host).expect("the twin readmits");
    assert_eq!(admitted.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>(),document.objects.iter().map(|object|object.id.as_str()).collect::<Vec<_>>());
    host.objects[0].scale=Some(semio_framework_value::DslValue::String("invalid".into()));
    assert!(semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_snapshot_from_host_snapshot(&host).is_err());
}
