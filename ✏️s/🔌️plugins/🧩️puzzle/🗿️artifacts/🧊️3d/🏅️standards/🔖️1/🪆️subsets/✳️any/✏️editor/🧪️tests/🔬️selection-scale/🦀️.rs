use super::unit_tests::context::*;
use super::*;

/// 🔎️ Whether `needle` appears anywhere in a rendered body — the panel-agnostic way to ask "does this
/// surface name the selected object at all", without pinning the row shape a peer may be relocating.
fn mentions(node: &Value, needle: &str) -> bool {
    match node {
        Value::String(text) => text.contains(needle),
        Value::Array(items) => items.iter().any(|item| mentions(item, needle)),
        Value::Object(fields) => fields.iter().any(|(key, value)| key.contains(needle) || mentions(value, needle)),
        _ => false,
    }
}

/// 🎯️ Wave B46 LAW: a pick BY ID on the flagship 180-object document persists and reaches Inspection.
///
/// 🧾️ Wave B44 §6.2 could establish no world selection at all on this document — six canvas picks and
/// the outliner both landed nothing — so every selection-scoped verb on it was unreachable. The
/// framework prunes a picked id that its `interaction_topology` does not contain
/// (`protocol::validate_state`), and that topology is rebuilt per pick off a fresh decode of the whole
/// fixture, so a pick on a large document is exactly where a narrowed universe would show up first.
#[semio_framework_async_macros::async_test]
async fn a_pick_by_id_on_the_flagship_document_persists_and_reaches_inspection() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let objects = object_count(&app);
    assert!(objects >= 100, "the law needs the large document; got {objects} objects");
    let victim = first_object_id(&app);
    select_id(&mut app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &victim).await.expect("a pick by id dispatches");

    let world = render_body(&mut app, main::BODY_KEY).await;
    let selected = selection_of(&world);
    let ids: Vec<String> = selected.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
    eprintln!("[DEBUG] b46.pick objects={objects} picked={victim} worldSelected={ids:?}");
    assert!(ids.contains(&victim), "the world lane of a {objects}-object document must carry the picked id; got {ids:?}");

    let inspection = render_panel_body(&mut app, inspection::BODY_KEY, Some(main::WINDOW_KIND_ID)).await;
    assert!(mentions(&inspection, &victim), "Inspection must name the object a pick just selected on a {objects}-object document");
}

/// 🕹️ Wave B48 LAW: a pick reaches the world body of EVERY window instance the host refreshes, through
/// the exact route the refresh uses — the window KIND's body key with the instance carried by the view
/// alone (`render_window_refresh`), never the `<body>:<instance>` spelling.
///
/// 🧾️ Measured live on `:6013` at wasm #58 (wave B48 §3): a pick declares the right scope, the host asks
/// for all three window instances (`asked:["puzzle3d-main","puzzle3d-main-top",
/// "puzzle3d-main-perspective"]`), drops nothing — and the guest answers `changed:[]` with every
/// retained surface still at **revision 1**, so `data-guest-selection-json` keeps `selectedIds:[]`. That
/// leaves exactly two readings: the guest re-rendered an identical tree, or it never re-rendered. This
/// law closes the first one, on BOTH documents, at every instance id the host addresses. It is green
/// today; its value is that it stays green while the render route it pins is refactored, so a later wave
/// reading a stale world lane in the browser can rule this hop out without a wasm build.
#[semio_framework_async_macros::async_test]
async fn a_pick_reaches_the_world_body_of_every_window_instance_the_host_refreshes() {
    const WINDOW_INSTANCES: [&str; 3] = [main::WINDOW_KIND_ID, "puzzle3d-main-top", "puzzle3d-main-perspective"];
    let mut app = app().await;
    for example in ["concrete-forest", PUZZLE3D_EXAMPLE_NAKAGIN] {
        if example == PUZZLE3D_EXAMPLE_NAKAGIN {
            dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": example })), None).await.expect("nakagin switch");
        }
        let objects = object_count(&app);
        let victim = first_object_id(&app);
        select_id(&mut app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &victim).await.expect("a pick by id dispatches");
        for window in WINDOW_INSTANCES {
            let world = render_window_refresh(&mut app, main::BODY_KEY, window).await;
            let selected = selection_of(&world);
            let ids: Vec<String> = selected.get("ids").and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default();
            eprintln!("[DEBUG] b48.worldLane example={example} objects={objects} window={window} picked={victim} ids={ids:?}");
            assert!(ids.contains(&victim), "the world body the host refreshes for {window} on the {objects}-object {example} document must carry the picked id; got {ids:?}");
            assert_eq!(selected.get("activeObjectId").and_then(Value::as_str), Some(victim.as_str()), "and must name it as the active object, which is what the pane's gumball anchors on");
        }
    }
}

/// ⏱️ Wave B46 LAW: one pick's interaction work is bounded independent of the document size.
///
/// 🧾️ The measurable unit is how many times the app's own `interaction_topology` is built — one build
/// is one full fixture decode plus one `TopologyNode` per object, vortex, attraction, target volume,
/// reference and object kind (~900 nodes on the flagship). A pick used to pay it TWICE, once in the
/// framework's `interactionSelect` arm and once in the revalidation behind it, with nothing between
/// them that can move the document. The count must be the same on a 1-object and a 180-object
/// document — that is what makes the per-pick cost a function of the delta rather than of `n`.
#[semio_framework_async_macros::async_test]
async fn one_pick_builds_the_interaction_topology_once_whatever_the_document_size() {
    let builds = || PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS.with(std::cell::Cell::get);
    let mut app = app().await;

    let small_objects = object_count(&app);
    let small_victim = first_object_id(&app);
    let before_small = builds();
    select_id(&mut app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &small_victim).await.expect("a pick on the small document dispatches");
    let small_builds = builds() - before_small;

    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let large_objects = object_count(&app);
    let large_victim = first_object_id(&app);
    let before_large = builds();
    select_id(&mut app, crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT, &large_victim).await.expect("a pick on the large document dispatches");
    let large_builds = builds() - before_large;

    eprintln!("[DEBUG] b46.topology small={small_objects}objects/{small_builds}builds large={large_objects}objects/{large_builds}builds");
    assert!(large_objects > small_objects, "the two measurements must differ in document size");
    assert_eq!(small_builds, large_builds, "a pick's topology work must not grow with the document");
    assert_eq!(large_builds, 1, "one pick builds the app interaction topology exactly once");
}

/// 🌳️ Wave B46 LAW: the outliner PANEL BODY of the flagship document carries selectable rows, paged.
///
/// 🧾️ The panel module's own law measures `document::render` directly; this one measures the route the
/// host actually asks for (`render_panel_body`, no `window_id`, over the live session), because wave
/// B44 §6.2 read zero entity rows in the browser while the builder was green in isolation.
#[semio_framework_async_macros::async_test]
async fn the_flagship_outliner_panel_body_carries_paged_object_rows() {
    let mut app = app().await;
    dispatch(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), None).await.expect("nakagin switch");
    let objects = object_count(&app);
    assert!(objects >= 100, "the law needs the large document; got {objects} objects");
    let first = first_object_id(&app);
    let body = render_panel_body(&mut app, document::BODY_KEY, Some(main::WINDOW_KIND_ID)).await;
    let rows = mentions(&body, &first);
    let paged = mentions(&body, ".objects.more");
    eprintln!("[DEBUG] b46.panelBody objects={objects} firstRowPresent={rows} continuation={paged} bytes={}", to_json_string(&body).len());
    assert!(rows, "the outliner panel body of a {objects}-object document must present its first object row");
    assert!(paged, "and must close the truncated objects section with a continuation row");
}
