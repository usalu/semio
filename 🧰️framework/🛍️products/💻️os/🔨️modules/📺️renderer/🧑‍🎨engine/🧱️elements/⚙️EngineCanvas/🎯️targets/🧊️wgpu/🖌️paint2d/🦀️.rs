//! 🖌️ Paint2d layers, context menu targets, marquee and pointer.
//!
//! Paint2d layer pick, the context menu targets, the marquee and the pointer button, move and wheel handlers.

use super::board2d::with_board_host;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_camera_from_json;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{NodeGraphEngine, ENGINE_SURFACES};
use crate::element_contracts::WorkerCell;
use semio_framework_canvas as canvas;
use semio_framework_surface::paint::RasterHost;
use serde_json::{json, Value};
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, Rect, UiComponentSceneNode};

//#region Paint2d
/** 🕹️ Raster's framework-owned layer interaction domain and its one granularity — the same pair
 * `✏️editor/🦀️.rs`'s `.interaction(InteractionDefinition { id: "layers", granularities: [layer] })`
 * declares, and the same pair React dispatches `interactionHover`/`interactionSelect` under. Raster
 * declares NO bespoke `setHover`: the bespoke verbs were refused "undeclared-action" on every
 * pointer move (ticket 26/09/05/RASTER-PLUGIN-END-TO-END).
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `PAINT2D_INTERACTION_DOMAIN` */
const PAINT2D_INTERACTION_DOMAIN: &str = "layers";
const PAINT2D_LAYER_GRANULARITY: &str = "layer";

pub fn with_raster_host_mut<R>(surface_id: &str, f: impl FnOnce(&mut RasterHost) -> R) -> Option<R> {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let entry = map.get_mut(surface_id)?;
        let host = entry.raster_host.as_mut()?;
        let result = f(host);
        entry.scene_revision = entry.scene_revision.wrapping_add(1);
        Some(result)
    })
}

/// 🖱️ The topmost pixel layer under a surface-local point — the wgpu twin of
/// `resolveTargetsAtClient`'s `pickTargetsAtScreenJson(...)` read, narrowed to the `layer`
/// granularity that raster's interaction domain actually declares.
pub fn paint2d_pick_layer(surface_id: &str, sx: f64, sy: f64) -> Option<String> {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let host = map.get(surface_id)?.raster_host.as_ref()?;
        let hits: Vec<Value> = serde_json::from_str(&host.pick_targets_at_screen_json(sx, sy)).ok()?;
        hits.iter().find(|hit| hit.get("domain").and_then(Value::as_str) == Some("pixel")).and_then(|hit| hit.get("id").and_then(Value::as_str)).map(str::to_owned)
    })
}

//#region 🖱️ContextMenuTargets
/// 🖱️ Every pick target under a surface-local point, as the `hits` rows a context-menu request
/// carries — the wgpu twin of each React host's `JSON.parse(session.pickTargetsAtScreenJson(sx, sy))
/// .map(target => ({ domain, id, label }))`. Order is the host's own most-general-last order, which
/// is what React sends and what a plugin's `hits[0]` reads as "the thing under the pointer".
pub fn pick_target_hits(json: &str) -> Vec<ui_wgpu::wgpu::ContextMenuHit> {
    serde_json::from_str::<Vec<Value>>(json)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|target| {
            let domain = target.get("domain").and_then(Value::as_str)?.to_string();
            let id = target.get("id").and_then(Value::as_str)?.to_string();
            Some(ui_wgpu::wgpu::ContextMenuHit { domain, id, label: target.get("label").and_then(Value::as_str).map(str::to_owned) })
        })
        .collect()
}

/// 🖱️ `nodes`/`edges`/`handles` off a node-graph host's own selection read, tolerating both the
/// object form and the bare id array — port of `parseSelectionDomainsFromSession`
/// (`🧱️elements/🌐️World3dHost/🟦️.tsx:1801`).
pub fn selection_domains(json: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let Ok(parsed) = serde_json::from_str::<Value>(json) else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    if let Some(array) = parsed.as_array() {
        return (array.iter().filter_map(Value::as_str).map(str::to_owned).collect(), Vec::new(), Vec::new());
    }
    let list = |primary: &str, fallback: &str| -> Vec<String> { parsed.get(primary).or_else(|| parsed.get(fallback)).and_then(Value::as_array).map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_owned).collect()).unwrap_or_default() };
    (list("nodes", "nodes"), list("edges", "edgeIds"), list("handles", "handleIds"))
}

/// 🕸️ One node-graph right-click's surface target: the pick targets under the point plus the live
/// selection split into React's `node`/`edge`/`handle` groups — port of `NodeGraph`'s own
/// `onContextMenu` (`🧱️elements/🕸️NodeGraph/🟦️.tsx:3509`, `selectionGroupsFromDomains`).
pub fn node_graph_context_menu_target(surface_id: &str, sx: f64, sy: f64) -> (Vec<ui_wgpu::wgpu::ContextMenuHit>, Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>) {
    ENGINE_SURFACES
        .with(|cell| {
            let map = cell.borrow();
            let engine = map.get(surface_id)?.node_graph.as_ref()?;
            let (picks, domains) = match engine {
                NodeGraphEngine::Flow(host) => (host.pick_targets_at_screen_json(sx, sy), host.selection_domains()),
                NodeGraphEngine::Dag(host) => (host.pick_targets_at_screen_json(sx, sy), host.dag.selection_domains()),
            };
            let (nodes,edges,handles)=(domains.nodes,domains.edges,domains.handles);
            let mut selection = Vec::new();
            for (domain, ids) in [("node", nodes), ("edge", edges), ("handle", handles)] {
                if !ids.is_empty() {
                    selection.push(ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: domain.into(), ids });
                }
            }
            Some((pick_target_hits(&picks), selection))
        })
        .unwrap_or_default()
}

/// 🖥️ One board-2d right-click's surface target: every canvas pick target under the point, plus the
/// scene's own selection as the `node` group — port of `Board2dHost`'s `onContextMenu`
/// (`🧱️elements/🖥️Board2dHost/🟦️.tsx:1355`).
pub fn board2d_context_menu_target(surface_id: &str, selection_json: &str, sx: f64, sy: f64) -> (Vec<ui_wgpu::wgpu::ContextMenuHit>, Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>) {
    let hits = with_board_host(surface_id, |host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default();
    let ids: Vec<String> = serde_json::from_str(selection_json).unwrap_or_default();
    let selection = if ids.is_empty() { Vec::new() } else { vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "node".into(), ids }] };
    (hits, selection)
}

/// 🖌️ One paint-2d right-click's surface target: the raster host's pick targets, fresh at the click
/// point (raster keeps no cached hover), plus the scene's selection as the `layer` group — port of
/// `Paint2dHost`'s `onContextMenu` (`🧱️elements/🖌️Paint2dHost/🟦️.tsx:540`).
pub fn paint2d_context_menu_target(surface_id: &str, selection_json: &str, sx: f64, sy: f64) -> (Vec<ui_wgpu::wgpu::ContextMenuHit>, Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>) {
    let hits = ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        map.get(surface_id).and_then(|entry| entry.raster_host.as_ref()).map(|host| pick_target_hits(&host.pick_targets_at_screen_json(sx, sy))).unwrap_or_default()
    });
    let ids: Vec<String> = serde_json::from_str(selection_json).unwrap_or_default();
    let selection = if ids.is_empty() { Vec::new() } else { vec![ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "layer".into(), ids }] };
    (hits, selection)
}

/// ✏️ One text-editor right-click's surface target: the editor's own pick targets under the point and
/// the `text` context a menu resolver reads (caret, whether a selection exists, whether rename and
/// completions are offered) — port of `TextEditor`'s `onContextMenu`
/// (`🧱️elements/✏️TextEditor/🟦️.tsx:479`). `selection` stays empty: the editor reports its text state
/// through `text`, never as selection groups.
pub fn text_editor_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> (Vec<ui_wgpu::wgpu::ContextMenuHit>, Option<ui_wgpu::wgpu::ContextMenuTextContext>) {
    let sx = f64::from(x - inner.x);
    let sy = f64::from(y - inner.y);
    ENGINE_SURFACES
        .with(|cell| {
            let map = cell.borrow();
            let host = map.get(&scene.host_id)?.editor.as_ref()?;
            let editor = scene.text_editor.as_ref();
            let text = ui_wgpu::wgpu::ContextMenuTextContext {
                caret: host.caret(),
                has_selection: host.anchor() != host.caret(),
                word: None,
                can_rename: editor.and_then(|editor| editor.rename_json.as_deref()).is_some(),
                has_completions: editor.and_then(|editor| editor.completions_json.as_deref()).is_some_and(|json| json != "[]"),
            };
            Some((pick_target_hits(&host.pick_targets_at_screen_json(sx, sy)), Some(text)))
        })
        .unwrap_or_default()
}
//#endregion 🖱️ContextMenuTargets

/// 🎯️ `[{ granularity, id }]` — the `targets` string both interaction verbs carry, byte-identical to
/// React's `JSON.stringify(targets)` in `world3dHoverActionArgs`/`world3dSelectionActionArgs`.
fn interaction_targets_json(granularity: &str, ids: &[String]) -> String {
    serde_json::to_string(&ids.iter().map(|id| json!({ "granularity": granularity, "id": id })).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into())
}

pub(super) fn write_interaction_hover(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, domain_id: &str, targets: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "interactionHover", "domainId", domain_id, "channel", "pointer", "targets", targets])?;
    batch.action(controller_id, "interactionHover", bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("domainId"), domain_id)?;
        builder.string(Some("channel"), "pointer")?;
        builder.string(Some("targets"), targets)?;
        builder.end_container()
    })
}

fn write_interaction_select(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, controller_id: &str, domain_id: &str, targets: &str, merge: &str) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "interactionSelect", "domainId", domain_id, "targets", targets, "merge", merge, "method", "pick"])?;
    batch.action(controller_id, "interactionSelect", bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("domainId"), domain_id)?;
        builder.string(Some("targets"), targets)?;
        builder.string(Some("merge"), merge)?;
        builder.string(Some("method"), "pick")?;
        builder.end_container()
    })
}

fn write_surface_camera(batch: &mut ui_wgpu::wgpu::BoundedActionBatchReservation<'_>, scene: &UiComponentSceneNode, camera: (f64, f64, f64)) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let controller_id = scene.controller_id.as_str();
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
    batch.action(controller_id, "setCamera", bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.begin_object(Some("camera"))?;
        builder.number(Some("x"), camera.0)?;
        builder.number(Some("y"), camera.1)?;
        builder.number(Some("zoom"), camera.2)?;
        builder.end_container()?;
        builder.end_container()
    })
}

/// 🖌️ Whether this utility puts the surface in selection mode rather than paint mode — React's
/// `isPaint2dSelectionUtility`.
fn paint2d_selection_utility(active_utility: &str) -> bool {
    matches!(active_utility, "selectMarquee" | "selectLasso" | "selectWand")
}

/// 🕹️ The merge mode a modifier combination selects — the wgpu twin of `marqueeModeFromModifiers`.
fn paint2d_merge_mode(shift: bool, ctrl_or_meta: bool) -> &'static str {
    if shift {
        "add"
    } else if ctrl_or_meta {
        "subtract"
    } else {
        "replace"
    }
}

/// 🕹️ `selectionMergeIds` — the id list `interactionSelect` carries once a merge mode is applied to
/// the scene's current selection.
fn paint2d_merge_ids(mode: &str, current: &[String], next: &[String]) -> Vec<String> {
    match mode {
        "add" => {
            let mut merged = current.to_vec();
            for id in next {
                if !merged.contains(id) {
                    merged.push(id.clone());
                }
            }
            merged
        }
        "subtract" => current.iter().filter(|id| !next.contains(id)).cloned().collect(),
        _ => next.to_vec(),
    }
}

//#region Paint2dMarquee
/// 📏️ `PAINT_2D_MARQUEE_THRESHOLD_PX` (`🖌️Paint2dHost/🟦️.tsx:45`) — how far the pointer must travel
/// before a selection press becomes a marquee rather than a single-point pick.
const PAINT2D_MARQUEE_THRESHOLD_PX: f32 = 4.0;

/// 🖱️ One live paint-2d marquee gesture, the wgpu twin of `marqueeRef` in `🖌️Paint2dHost/🟦️.tsx:168`.
/// `points` holds the lasso path, or the two rectangle corners.
#[derive(Clone, Debug, Default)]
struct Paint2dMarquee {
    tracking: bool,
    pub(super) active: bool,
    pub(super) start: (f32, f32),
    points: Vec<(f32, f32)>,
    /// 🧭️ The navigator's own middle-button pan anchor (`panRef`, `🖌️Paint2dHost/🟦️.tsx:430`). A
    /// navigator surface runs no marquee, so the two gestures share one per-surface slot.
    pan_last: Option<(f32, f32)>,
}

/// 🧮️ Marquee gestures are per SURFACE and never outlive one press/release pair, so a plain map
/// keyed by surface id is the whole bound — the same shape `scenes`' own `map_marquee_points` uses.
static PAINT2D_MARQUEES: WorkerCell<HashMap<String, Paint2dMarquee>> = WorkerCell::new(Default::default);

fn with_paint2d_marquee<R>(surface_id: &str, f: impl FnOnce(&mut Paint2dMarquee) -> R) -> R {
    PAINT2D_MARQUEES.with(|cell| {
        let mut map = cell.borrow_mut();
        f(map.entry(surface_id.to_string()).or_default())
    })
}

fn paint2d_marquee_snapshot(surface_id: &str) -> Option<Paint2dMarquee> {
    PAINT2D_MARQUEES.with(|cell| cell.borrow().get(surface_id).filter(|marquee| marquee.active).cloned())
}

/// 🖱️ The selection METHOD a utility puts the surface in — `paint2dSelectionMethod`
/// (`🖌️Paint2dHost/🟦️.tsx:101`). `selectWand` is a selection utility with no marquee gesture.
fn paint2d_selection_method(active_utility: &str) -> Option<&'static str> {
    match active_utility {
        "selectMarquee" => Some("rectangle"),
        "selectLasso" => Some("lasso"),
        _ => None,
    }
}

/// 🖱️ The marquee path as `marquee_hits_json` wants it, plus the crossing flag React derives from
/// `marqueeCoverageFromGesture` (a right-to-left drag is a partial/crossing marquee).
fn paint2d_marquee_path(marquee: &Paint2dMarquee, lasso: bool, point: (f32, f32)) -> (Vec<[f32; 2]>, bool) {
    let points: Vec<[f32; 2]> = if lasso { marquee.points.iter().chain(std::iter::once(&point)).map(|(x, y)| [*x, *y]).collect() } else { vec![[marquee.start.0, marquee.start.1], [point.0, point.1]] };
    let crossing = ui_wgpu::wgpu::marquee_is_crossing_from_path(&points, lasso);
    (points, crossing)
}

/// 🖌️ The overlay a live marquee paints, in SURFACE-LOCAL coordinates, or `None` when no marquee is
/// running — read by the scene paint's overlay phase. React renders the same gesture as a
/// `SelectionMarquee` element layered over the canvas (`🖌️Paint2dHost/🟦️.tsx:178`).
pub fn paint2d_marquee_overlay(surface_id: &str, active_utility: &str) -> Option<(Vec<[f32; 2]>, bool, bool)> {
    let lasso = paint2d_selection_method(active_utility)? == "lasso";
    let marquee = paint2d_marquee_snapshot(surface_id)?;
    let last = marquee.points.last().copied().unwrap_or(marquee.start);
    let (points, crossing) = paint2d_marquee_path(&marquee, lasso, last);
    (points.len() >= 2).then_some((points, crossing, lasso))
}

/// 🧭️ The navigator's "you are here" rectangle in navigator-surface-local coordinates — the wgpu twin
/// of `session.navigatorViewportOverlayJson(scene.cameraJson, scene.compositeViewportJson)` and the
/// `overlayRect` React draws from it (`🖌️Paint2dHost/🟦️.tsx:275`). `None` whenever the scene is not a
/// navigator, carries no composite viewport, or the host refuses the query.
pub fn paint2d_navigator_overlay_rect(scene: &UiComponentSceneNode) -> Option<[f32; 4]> {
    let paint = scene.paint_2d.as_ref()?;
    if paint.view_mode != "navigator" {
        return None;
    }
    let viewport_json = paint.composite_viewport_json.as_deref()?;
    let json = with_raster_host_mut(&scene.host_id, |host| host.navigator_viewport_overlay_json(&paint.camera_json, viewport_json).ok()).flatten()?;
    let value: Value = serde_json::from_str(&json).ok()?;
    let read = |key: &str| value.get(key).and_then(Value::as_f64).unwrap_or(0.0) as f32;
    let rect = [read("x"), read("y"), read("width"), read("height")];
    (rect[2] > 0.0 && rect[3] > 0.0).then_some(rect)
}
//#endregion Paint2dMarquee

fn paint2d_selection(scene: &UiComponentSceneNode) -> Vec<String> {
    scene.paint_2d.as_ref().and_then(|paint| serde_json::from_str::<Vec<String>>(&paint.selection_json).ok()).unwrap_or_default()
}

/** 🖱️ One paint-2d pointer press or release. Selection utilities pick a layer and publish
 * `interactionSelect`; every other utility is a brush gesture driven straight into the `RasterHost`,
 * whose resulting camera is republished as `setCamera` exactly as React's `onPointerUp` does.
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `onPointerDown`/`onPointerUp` */
/// 🧭️ A navigator pans the CONTENT camera with the middle button and does nothing else —
/// React's `onPointerDown`/`onPointerUp` navigator arms (`🖌️Paint2dHost/🟦️.tsx:430`, `:453`).
///
/// 🖱️ React arms `marqueeRef` on the press and only promotes it to a real marquee once the
/// pointer travels (`🖌️Paint2dHost/🟦️.tsx:438`), so a press that never moves stays a pick.
///
/// 🖱️ A committed MARQUEE takes the release: it asks the host which pixel layers the path
/// covers and publishes them merged, never the single point under the pointer
/// (`commitMarqueeSelection`, `🖌️Paint2dHost/🟦️.tsx:406-420`).
///
/// 🖱️ React reaches `onSelectTarget(target, …)` only through a resolved pick, so an empty
/// release is a dismissed pick menu, never a selection clear.
pub fn paint2d_pointer_button_into(
    scene: &UiComponentSceneNode,
    inner: Rect,
    x: f32,
    y: f32,
    down: bool,
    button: i16,
    shift: bool,
    ctrl_or_meta: bool,
    input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>,
) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(paint) = scene.paint_2d.as_ref() else {
        return Ok(false);
    };
    if paint.view_mode == "navigator" {
        if button == 1 {
            with_paint2d_marquee(&scene.host_id, |marquee| marquee.pan_last = down.then_some((x, y)));
        }
        return Ok(false);
    }
    let sx = f64::from(x - inner.x);
    let sy = f64::from(y - inner.y);
    if paint2d_selection_utility(&paint.active_utility) {
        let method = paint2d_selection_method(&paint.active_utility);
        if down {
            if method.is_some() {
                with_paint2d_marquee(&scene.host_id, |marquee| {
                    *marquee = Paint2dMarquee { tracking: true, active: false, start: (x, y), points: vec![(x, y)], pan_last: None };
                });
            }
            return Ok(false);
        }
        let committed = method.and_then(|method| {
            let marquee = with_paint2d_marquee(&scene.host_id, |marquee| {
                let snapshot = marquee.clone();
                *marquee = Paint2dMarquee::default();
                snapshot
            });
            marquee.active.then(|| paint2d_marquee_path(&marquee, method == "lasso", (x, y)))
        });
        if let Some((points, crossing)) = committed {
            let merge = paint2d_merge_mode(shift, ctrl_or_meta);
            let local: Vec<[f64; 2]> = points.iter().map(|point| [f64::from(point[0] - inner.x), f64::from(point[1] - inner.y)]).collect();
            let query = json!({ "points": local.iter().map(|point| json!({ "x": point[0], "y": point[1] })).collect::<Vec<_>>(), "crossing": crossing }).to_string();
            let hits: Vec<String> = with_raster_host_mut(&scene.host_id, |host| host.marquee_hits_json(&query).ok()).flatten().and_then(|json| serde_json::from_str::<Vec<String>>(&json).ok()).unwrap_or_default();
            let ids = paint2d_merge_ids(merge, &paint2d_selection(scene), &hits);
            let targets = interaction_targets_json(PAINT2D_LAYER_GRANULARITY, &ids);
            let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "interactionSelect", "domainId", PAINT2D_INTERACTION_DOMAIN, "targets", &targets, "merge", merge, "method", "pick"])?;
            let mut batch = input.reserve_actions(1, bytes)?;
            write_interaction_select(&mut batch, &scene.controller_id, PAINT2D_INTERACTION_DOMAIN, &targets, merge)?;
            batch.publish()?;
            return Ok(true);
        }
        let merge = paint2d_merge_mode(shift, ctrl_or_meta);
        let Some(hit) = paint2d_pick_layer(&scene.host_id, sx, sy) else {
            return Ok(false);
        };
        let ids = paint2d_merge_ids(merge, &paint2d_selection(scene), std::slice::from_ref(&hit));
        let targets = interaction_targets_json(PAINT2D_LAYER_GRANULARITY, &ids);
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "interactionSelect", "domainId", PAINT2D_INTERACTION_DOMAIN, "targets", &targets, "merge", merge, "method", "pick"])?;
        let mut batch = input.reserve_actions(1, bytes)?;
        write_interaction_select(&mut batch, &scene.controller_id, PAINT2D_INTERACTION_DOMAIN, &targets, merge)?;
        batch.publish()?;
        return Ok(true);
    }
    let flushed = flush_paint2d_edit(scene, input)?;
    let Some(camera) = with_raster_host_mut(&scene.host_id, |host| {
        if down {
            host.pointer_down_screen(sx, sy, button.max(0) as u8);
        } else {
            host.pointer_up_screen(sx, sy);
        }
        engine_camera_from_json(&host.camera_json())
    })
    .flatten() else {
        return Ok(flushed);
    };
    if flush_paint2d_edit(scene, input)? || down {
        return Ok(true);
    }
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
    let mut batch = input.reserve_actions(1, bytes)?;
    write_surface_camera(&mut batch, scene, camera)?;
    batch.publish()?;
    Ok(true)
}

/** 🖌️ Publishes the raster host's pending paint-2d edit, if any, and takes it: `true` when one was published.
 *
 * @see [`write_paint2d_edit`] */
fn flush_paint2d_edit(scene: &UiComponentSceneNode, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(edit) = with_raster_host_mut(&scene.host_id, |host| host.paint_edit().cloned()).flatten() else {
        return Ok(false);
    };
    write_paint2d_edit(scene, &edit, input)?;
    with_raster_host_mut(&scene.host_id, |host| host.take_paint_edit());
    Ok(true)
}

/** 🖌️ Publishes one paint-2d edit intent exactly as React's `Paint2dHost` dispatches it: a stroke as `paintStroke`
 * `{ surfaceId, layerId, tool, xs, ys }` — with `phase` (`stream` | `commit` | `abort` and its `reason`) and `gesture` when
 * it streams — and a bucket click as `fillRegion` `{ surfaceId, layerId, x, y }`, in the layer image's pixels; the editor
 * reads brush, colour, tolerance, target and selection from its session.
 *
 * @see `🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.tsx` — `pick`/`up`/`move` */
fn write_paint2d_edit(scene: &UiComponentSceneNode, edit: &semio_framework_surface::paint::PaintEditCommand, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<(), ui_wgpu::wgpu::BoundedActionFault> {
    let fill = edit.action() == "fillRegion";
    let (click, stroke) = (["x", "y"], ["tool", edit.tool, "xs", "ys"]);
    let mut fields = vec![scene.controller_id.as_str(), edit.action(), "surfaceId", scene.surface_id.as_str(), "layerId", edit.layer_id.as_str()];
    fields.extend(if fill { click.as_slice() } else { stroke.as_slice() });
    fields.extend(edit.phase.into_iter().flat_map(|phase| ["phase", phase]));
    fields.extend(edit.reason.into_iter().flat_map(|reason| ["reason", reason]));
    fields.extend(edit.gesture.iter().flat_map(|gesture| ["gesture", gesture.as_str()]));
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&fields)?;
    let mut batch = input.reserve_actions(1, bytes)?;
    batch.action(&scene.controller_id, edit.action(), bytes, |builder| {
        builder.begin_object(None)?;
        builder.string(Some("surfaceId"), &scene.surface_id)?;
        builder.string(Some("layerId"), &edit.layer_id)?;
        match edit.points.first() {
            Some(point) if fill => {
                builder.number(Some("x"), point[0])?;
                builder.number(Some("y"), point[1])?;
            }
            _ => {
                builder.string(Some("tool"), edit.tool)?;
                for (key, axis) in [("xs", 0), ("ys", 1)] {
                    builder.begin_array(Some(key))?;
                    for point in &edit.points {
                        builder.number(None, point[axis])?;
                    }
                    builder.end_container()?;
                }
                for (key, value) in [("phase", edit.phase), ("reason", edit.reason), ("gesture", edit.gesture.as_deref())] {
                    if let Some(value) = value {
                        builder.string(Some(key), value)?;
                    }
                }
            }
        }
        builder.end_container()
    })?;
    batch.publish()?;
    Ok(())
}

/** 🖱️ One paint-2d pointer move: a hover pick under a selection utility, a live brush stroke plus
 * `setCamera` republish otherwise.
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `onPointerMove` */
/// 🧭️ Navigator pan drives the CONTENT camera, in content-world units: React divides the
/// screen delta by the CONTENT camera's zoom and dispatches `setCamera`
/// (`🖌️Paint2dHost/🟦️.tsx:453-462`). The navigator's own camera stays fit to the document.
///
/// 🖱️ The armed gesture becomes a marquee once the pointer clears the threshold, and the lasso
/// accumulates its path — `onPointerMove`'s marquee branch (`🖌️Paint2dHost/🟦️.tsx:466-475`).
pub fn paint2d_pointer_move_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(paint) = scene.paint_2d.as_ref() else {
        return Ok(false);
    };
    if paint.view_mode == "navigator" {
        let Some((last_x, last_y)) = with_paint2d_marquee(&scene.host_id, |marquee| {
            let last = marquee.pan_last;
            if last.is_some() {
                marquee.pan_last = Some((x, y));
            }
            last
        }) else {
            return Ok(false);
        };
        let Some((cx, cy, zoom)) = engine_camera_from_json(&paint.camera_json) else {
            return Ok(false);
        };
        let camera = (cx - f64::from(x - last_x) / zoom, cy - f64::from(y - last_y) / zoom, zoom);
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
        let mut batch = input.reserve_actions(1, bytes)?;
        write_surface_camera(&mut batch, scene, camera)?;
        batch.publish()?;
        return Ok(true);
    }
    let sx = f64::from(x - inner.x);
    let sy = f64::from(y - inner.y);
    if paint2d_selection_utility(&paint.active_utility) {
        if let Some(method) = paint2d_selection_method(&paint.active_utility) {
            with_paint2d_marquee(&scene.host_id, |marquee| {
                if !marquee.tracking {
                    return;
                }
                let distance = ((x - marquee.start.0).powi(2) + (y - marquee.start.1).powi(2)).sqrt();
                if !marquee.active && distance >= PAINT2D_MARQUEE_THRESHOLD_PX {
                    marquee.active = true;
                }
                if marquee.active && method == "lasso" && marquee.points.last().copied() != Some((x, y)) {
                    marquee.points.push((x, y));
                }
            });
        }
        let hit = paint2d_pick_layer(&scene.host_id, sx, sy);
        if hit.as_deref() == paint.hovered_id.as_deref() {
            return Ok(false);
        }
        let targets = interaction_targets_json(PAINT2D_LAYER_GRANULARITY, hit.as_slice());
        let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "interactionHover", "domainId", PAINT2D_INTERACTION_DOMAIN, "channel", "pointer", "targets", &targets])?;
        let mut batch = input.reserve_actions(1, bytes)?;
        write_interaction_hover(&mut batch, &scene.controller_id, PAINT2D_INTERACTION_DOMAIN, &targets)?;
        batch.publish()?;
        return Ok(true);
    }
    let Some(camera) = with_raster_host_mut(&scene.host_id, |host| {
        host.pointer_move_screen(sx, sy);
        engine_camera_from_json(&host.camera_json())
    })
    .flatten() else {
        return Ok(false);
    };
    let streamed = flush_paint2d_edit(scene, input)?;
    if engine_camera_from_json(&paint.camera_json) == Some(camera) {
        return Ok(streamed);
    }
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
    let mut batch = input.reserve_actions(1, bytes)?;
    write_surface_camera(&mut batch, scene, camera)?;
    batch.publish()?;
    Ok(true)
}

pub fn paint2d_pointer_cancel_into(surface_id: &str) -> bool {
    with_paint2d_marquee(surface_id, |marquee| *marquee = Paint2dMarquee::default());
    with_raster_host_mut(surface_id, |host| host.pointer_cancel_screen()).is_some()
}

/** 🎡️ One paint-2d wheel notch — the host owns the zoom-at-cursor math and the resulting camera is
 * republished, matching React's `onWheel`.
 *
 * @see `🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `onWheel` */
pub fn paint2d_wheel_into(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32, delta: f32, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Result<bool, ui_wgpu::wgpu::BoundedActionFault> {
    let Some(paint) = scene.paint_2d.as_ref() else {
        return Ok(false);
    };
    let sx = f64::from(x - inner.x);
    let sy = f64::from(y - inner.y);
    let camera = if paint.view_mode == "navigator" {
        let Some((x, y, zoom)) = engine_camera_from_json(&paint.camera_json) else { return Ok(false) };
        let size = paint.composite_viewport_json.as_deref().and_then(|value| serde_json::from_str::<Value>(value).ok());
        let dimension = |key: &str, fallback: u32| size.as_ref().and_then(|value| value.get(key)).and_then(Value::as_f64).filter(|value| value.is_finite() && *value > 0.0).map(|value| value as u32).unwrap_or(fallback);
        let viewport = canvas::camera::Viewport { width: dimension("width", 800), height: dimension("height", 600), dpr: 1.0 };
        let mut camera = canvas::camera::Camera { x, y, zoom };
        canvas::camera::wheel_screen(&mut camera, &viewport, sx, sy, f64::from(delta));
        Some((camera.x, camera.y, camera.zoom))
    } else {
        with_raster_host_mut(&scene.host_id, |host| {
            host.wheel_screen(sx, sy, f64::from(delta));
            engine_camera_from_json(&host.camera_json())
        }).flatten()
    };
    let Some(camera) = camera else {
        return Ok(false);
    };
    let bytes = ui_wgpu::wgpu::checked_action_string_bytes(&[&scene.controller_id, "setCamera", "surfaceId", &scene.surface_id, "camera", "x", "y", "zoom"])?;
    let mut batch = input.reserve_actions(1, bytes)?;
    write_surface_camera(&mut batch, scene, camera)?;
    batch.publish()?;
    Ok(true)
}
//#endregion Paint2d
