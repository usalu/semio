//! 🎨️ Scene surface context menu and the component scene render entry.

use super::*;

//#region 🖱️SurfaceContextMenu
/** 🖱️ One component scene's answer to a right-click: the `surface` half of the context-menu
 * request its React host sends — `openSurfaceContextMenu({ menu, surface: { kind, hits, selection },
 * windowInstanceId })` in `🗣️Interpreter/🟦️.tsx:759`. Every field here is per-surface-kind by
 * convention, and the convention is the React host's: a kind that tracks no pick state reports
 * `hits: []`, a kind that tracks no selection reports `selection: []`, and only the text editor
 * carries a `text` block.
 *
 * The wgpu shell owns the CHROME (`ShellState::open_context_menu`) and the plugin round trip; this
 * type is the one thing that has to be resolved per kind, from the same paint geometry each kind's
 * pointer resolver already re-derives. */
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SceneContextMenuTarget {
    pub hits: Vec<semio_framework_ui_declaration::ui::ContextMenuHit>,
    pub selection: Vec<semio_framework_ui_declaration::ui::ContextMenuSelectionGroup>,
    pub text: Option<semio_framework_ui_declaration::ui::ContextMenuTextContext>,
}

/** 🖱️ Resolves `(x, y)` inside `scene` into that kind's own context-menu surface target.
 * `bounds` is the surface's absolute rect, the same rect its paint and pointer resolvers use, so a
 * row hit here is the row the pointer is actually over.
 *
 * Per-kind, against each React host's own `onContextMenu`:
 * - `table` — the body row under the pointer as `{domain:"row"}`, selection `{domain:"row"}`
 *   (`📊️Table/🟦️.tsx:240`).
 * - `virtualFileSystem` — the visible row under the pointer, selection `{domain:"row"}`
 *   (`🗣️Interpreter/🟦️.tsx:820`).
 * - `eventFeed` — the entry under the pointer as `{domain:"entry"}`, no selection
 *   (`📡️EventFeedHost/🟦️.tsx:89`).
 * - `graphTimeline`/`blockList`/`diffView`/`canvas2d` — whole-surface: no hits, no selection
 *   (`🌳️GraphTimelineHost/🟦️.tsx:50`, `🧩️BlockListHost/🟦️.tsx:212`, `🔺️DiffViewHost/🟦️.tsx:183`,
 *   `📐️Canvas2dHost/🟦️.tsx:912`).
 * - `inkCanvas` — every block under the pointer as `{domain:"block"}`, selection `{domain:"block"}`
 *   (`🖋️InkCanvasHost/🟦️.tsx:1367`).
 * - `paint2d`/`board2d`/`nodeGraph`/`textEditor` — the host's own screen pick targets
 *   (see `engine_canvas`'s `🖱️ContextMenuTargets`).
 * - `tiledMap`/`world3d` — resolved by the shell against their own state maps
 *   (`tiled_map_context_menu_target`, `infinite_world::world::world3d_context_menu_surface`). */
pub fn scene_context_menu_target(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32) -> SceneContextMenuTarget {
    let theme = scene_input_theme();
    match scene.component_kind {
        SurfaceKind::Table => table_context_menu_target(scene, bounds, y, &theme),
        SurfaceKind::VirtualFileSystem => vfs_context_menu_target(scene, bounds, y, &theme),
        SurfaceKind::EventFeed => event_feed_context_menu_target(scene, bounds, x, y, &theme),
        SurfaceKind::InkCanvas => ink_canvas_context_menu_target(scene, bounds, x, y),
        SurfaceKind::Paint2d => {
            let selection_json = scene.paint_2d.as_ref().map(|paint| paint.selection_json.clone()).unwrap_or_else(|| "[]".into());
            let (hits, selection) = semio_framework_os_renderer_wgpu_engine_board::paint2d::paint2d_context_menu_target(&scene.host_id, &selection_json, f64::from(x - bounds.x), f64::from(y - bounds.y));
            SceneContextMenuTarget { hits, selection, text: None }
        }
        SurfaceKind::Board2d => {
            let selection_json = scene.board2d.as_ref().map(|board| board.selection_json.clone()).unwrap_or_else(|| "[]".into());
            let (hits, selection) = semio_framework_os_renderer_wgpu_engine_board::paint2d::board2d_context_menu_target(&scene.host_id, &selection_json, f64::from(x - bounds.x), f64::from(y - bounds.y));
            SceneContextMenuTarget { hits, selection, text: None }
        }
        SurfaceKind::NodeGraph => {
            let (hits, selection) = semio_framework_os_renderer_wgpu_engine_board::paint2d::node_graph_context_menu_target(&scene.host_id, f64::from(x - bounds.x), f64::from(y - bounds.y));
            SceneContextMenuTarget { hits, selection, text: None }
        }
        SurfaceKind::TextEditor => {
            let (hits, text) = semio_framework_os_renderer_wgpu_engine_board::paint2d::text_editor_context_menu_target(scene, bounds, x, y);
            SceneContextMenuTarget { hits, selection: Vec::new(), text }
        }
        SurfaceKind::TiledMap => {
            let (hits, selection) = tiled_map_context_menu_target(&scene.host_id, bounds, x, y);
            SceneContextMenuTarget { hits, selection, text: None }
        }
        _ => SceneContextMenuTarget::default(),
    }
}

/// 📊️ `{domain:"row"}` for the body row under `y`, plus the table's own `selectedIds` — the payload
/// `TableHost`'s `onRowContextMenu` sends (`📊️Table/🟦️.tsx:240`). A press in the HEADER band reports
/// no hit: React only binds the menu on rows.
fn table_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, y: f32, theme: &Theme) -> SceneContextMenuTarget {
    let Some(table) = scene.table.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let columns: Vec<TableColumn> = serde_json::from_str(&table.columns_json).unwrap_or_default();
    let metrics = table_metrics(inner, columns.len(), theme);
    let rows: Vec<Value> = serde_json::from_str(&table.rows_json).unwrap_or_default();
    let mut hits = Vec::new();
    if y >= metrics.body.y {
        let scroll = scroll_offset(&scene.host_id, "body");
        if let Ok(index) = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64) {
            if let Some(row) = rows.get(index) {
                hits.push(semio_framework_ui_declaration::ui::ContextMenuHit { domain: "row".into(), id: table_row_id(row, index), label: None });
            }
        }
    }
    let ids: Vec<String> = table.selection_json.as_deref().and_then(|json| serde_json::from_str::<Value>(json).ok()).and_then(|value| value.get("selectedIds").cloned()).and_then(|value| serde_json::from_value(value).ok()).unwrap_or_default();
    let selection = if ids.is_empty() { Vec::new() } else { vec![semio_framework_ui_declaration::ui::ContextMenuSelectionGroup { domain: "row".into(), ids }] };
    SceneContextMenuTarget { hits, selection, text: None }
}

/// 🗂️ `{domain:"row"}` for the VISIBLE row under `y` (expansion-aware, exactly like `vfs_hit`), plus
/// `selectedRowIdsJson` — the payload `VirtualFileSystemHost`'s `onRowContextMenu` sends
/// (`🗣️Interpreter/🟦️.tsx:820`).
fn vfs_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, y: f32, theme: &Theme) -> SceneContextMenuTarget {
    let Some(vfs) = scene.virtual_file_system.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let metrics = vfs_metrics(inner, theme);
    let mut hits = Vec::new();
    if y >= metrics.body.y {
        let rows: Vec<Value> = serde_json::from_str(&vfs.rows_json).unwrap_or_default();
        let expanded = scene_state(&scene.host_id).vfs_expanded_ids;
        let visible = build_vfs_visible_rows(&rows, &expanded);
        let scroll = scroll_offset(&scene.host_id, "vfs");
        if let Ok(index) = usize::try_from(((y - metrics.body.y + scroll) / metrics.row_h.max(1.0)).floor() as i64) {
            if let Some(entry) = visible.get(index) {
                hits.push(semio_framework_ui_declaration::ui::ContextMenuHit { domain: "row".into(), id: vfs_row_id(&entry.row), label: None });
            }
        }
    }
    let ids: Vec<String> = vfs.selected_row_ids_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    let selection = if ids.is_empty() { Vec::new() } else { vec![semio_framework_ui_declaration::ui::ContextMenuSelectionGroup { domain: "row".into(), ids }] };
    SceneContextMenuTarget { hits, selection, text: None }
}

/// 📜️ `{domain:"entry"}` for the log row under `y`, no selection — an `EventFeedScene` tracks none
/// (`📡️EventFeedHost/🟦️.tsx:89`).
fn event_feed_context_menu_target(scene: &UiComponentSceneNode, bounds: Rect, x: f32, y: f32, theme: &Theme) -> SceneContextMenuTarget {
    let Some(feed) = scene.event_feed.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let entries: Vec<EventFeedEntryJson> = serde_json::from_str(&feed.entries_json).unwrap_or_default();
    let layout = event_feed_layout(bounds, theme);
    if !layout.inner.contains(x, y) {
        return SceneContextMenuTarget::default();
    }
    let mut top = layout.inner.y - scroll_offset(&scene.host_id, "feed");
    for entry in &entries {
        let entry_h = event_feed_row_height(entry, &layout);
        if Rect::new(layout.inner.x, top, layout.inner.w, entry_h).contains(x, y) {
            return SceneContextMenuTarget { hits: vec![semio_framework_ui_declaration::ui::ContextMenuHit { domain: "entry".into(), id: entry.id.clone(), label: None }], selection: Vec::new(), text: None };
        }
        top += entry_h + layout.card_gap;
    }
    SceneContextMenuTarget::default()
}

/** 🖋️ Every ink block under the pointer as `{domain:"block"}`, topmost first, plus the
 * selection the menu applies to — React's `inkItemsAtPoint` + the "right-click outside the selection
 * selects the topmost hit instead" rule (`🖋️InkCanvasHost/🟦️.tsx:1355-1367`). Unlike React this does
 * NOT dispatch `setSelection` as a side effect: the wgpu ink lane commits every selection write
 * through the bounded `InkInteractionJob`, and a right-click resolving a menu is not a commit. */
fn ink_canvas_context_menu_target(scene: &UiComponentSceneNode, inner: Rect, x: f32, y: f32) -> SceneContextMenuTarget {
    let Some(ink) = scene.ink_canvas.as_ref() else {
        return SceneContextMenuTarget::default();
    };
    let Ok(document) = serde_json::from_str::<Value>(&ink.document_json) else {
        return SceneContextMenuTarget::default();
    };
    let scene_camera = document.get("camera").cloned().and_then(|camera| serde_json::from_value::<InkCameraJson>(camera).ok()).map(InkCameraF::from).unwrap_or_default();
    let camera = scene_state(&scene.host_id).ink_camera.map(|(x, y, zoom)| InkCameraF { x, y, zoom }).unwrap_or(scene_camera);
    let (world_x, world_y) = ink_screen_to_world(camera, inner, x, y);
    let selected: Vec<String> = serde_json::from_str(&ink.selection_json).unwrap_or_default();
    let blocks = document.get("blocks").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut hits = Vec::new();
    for block in blocks.iter().rev() {
        if hits.len() >= INK_INTERACTION_ITEM_CAPACITY {
            break;
        }
        let Some(id) = block.get("id").and_then(Value::as_str) else { continue };
        if ink_hits_point(block, world_x, world_y, INK_CONTEXT_MENU_HIT_THRESHOLD) {
            hits.push(semio_framework_ui_declaration::ui::ContextMenuHit { domain: "block".into(), id: id.to_string(), label: None });
        }
    }
    let ids = match hits.first() {
        Some(top) if !selected.iter().any(|id| id == &top.id) => vec![top.id.clone()],
        _ => selected,
    };
    let selection = if ids.is_empty() { Vec::new() } else { vec![semio_framework_ui_declaration::ui::ContextMenuSelectionGroup { domain: "block".into(), ids }] };
    SceneContextMenuTarget { hits, selection, text: None }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs"]
mod surface_context_menu_tests;

/// 🖋️ The world-space slack a right-click gets when picking an ink block — React's own
/// `inkItemsAtPoint` tolerance.
const INK_CONTEXT_MENU_HIT_THRESHOLD: f64 = 6.0;

/** 🏷️ The context-menu vocabulary id for a surface kind — the string React sends as BOTH
 * `menu.id` and `surface.kind` (`openSurfaceContextMenu({ menu: { id: "virtualFileSystem" },
 * surface: { kind: "virtualFileSystem" } })`). It is deliberately NOT `SurfaceKind`'s serde tag: the
 * scene wire is kebab-case (`virtual-file-system`), while the menu vocabulary a plugin matches on —
 * and `contextMenuSurfaceTitleKeys` keys itself by (`🗣️Interpreter/🟦️.tsx:712`) — is camelCase. */
pub fn context_menu_surface_kind_id(kind: SurfaceKind) -> &'static str {
    match kind {
        SurfaceKind::BlockList => "blockList",
        SurfaceKind::Board2d => "board2d",
        SurfaceKind::Canvas2d => "canvas2d",
        SurfaceKind::DiffView => "diffView",
        SurfaceKind::EventFeed => "eventFeed",
        SurfaceKind::GraphTimeline => "graphTimeline",
        SurfaceKind::IconRender => "iconRender",
        SurfaceKind::InkCanvas => "inkCanvas",
        SurfaceKind::NodeGraph => "nodeGraph",
        SurfaceKind::Paint2d => "paint2d",
        SurfaceKind::Table => "table",
        SurfaceKind::TextEditor => "textEditor",
        SurfaceKind::TiledMap => "tiledMap",
        SurfaceKind::VirtualFileSystem => "virtualFileSystem",
        SurfaceKind::World3d => "world3d",
    }
}
//#endregion 🖱️SurfaceContextMenu

/// 🍿️ The editor's own popup chrome, painted LAST so it lands over the composited
/// `EditorHost` texture phase 6 staged — React's absolutely-positioned overlays inside the
/// host element (`🧱️elements/✏️TextEditor/🟦️.tsx:503-608`).
///
/// ✂️ Captions are clipped to the surface they annotate, exactly like React's overlay
/// (`🕸️NodeGraph/🟦️.tsx` paints them inside the canvas's own `overflow: hidden` box):
/// the engine publishes a row for EVERY node, including the ones the camera has pushed
/// off screen, so an unclipped run of glyphs lands on the neighbouring window. The
/// scissor also opens a fresh draw layer, which keeps the captions above the engine
/// raster pushed in phase 6 whatever a single layer's own channel order is.
///
/// 🖌️🧭️ Paint2d's own two chrome overlays — the live marquee and the navigator's
/// "you are here" rectangle — ride the SAME overlay phase, clipped to the surface so a
/// lasso dragged past the pane edge cannot paint over its neighbour.
pub fn render_component_scene_step(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    cursor: &mut semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintCursor,
    hosts: &mut SceneEngineHosts<'_>,
    driver_drag: UiDriverDrag,
    document_generation: u64,
) -> semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep {
    if cursor.phase() == 0 && scene.component_kind == SurfaceKind::Canvas2d && cancel_stale_canvas_authority(hosts.window_id, &scene.host_id, document_generation, ctx.input).is_err() {
        return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
    }
    if cursor.phase() >= ENGINE_PHASE {
        match scene.component_kind {
            SurfaceKind::World3d => return render_world3d_surface_step(scene, bounds, ctx, cursor, hosts),
            SurfaceKind::Canvas2d => return render_canvas_2d_step(scene, bounds, ctx, cursor),
            SurfaceKind::InkCanvas => return render_ink_canvas_step(scene, bounds, ctx, cursor),
            SurfaceKind::IconRender => return render_icon_render_step(scene, bounds, ctx, cursor, hosts),
            kind if scene_kind_is_list(kind) => return render_list_scene_step(scene, bounds, ctx, cursor, hosts.chrome_labels, hosts.window_id, driver_drag, document_generation),
            _ => {}
        }
    }
    match cursor.phase() {
        0 => {
            if scene.surface_id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            if cursor.byte() < scene.surface_id.len() {
                if cursor.advance_byte().is_err() {
                    return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
                }
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending;
            }
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        1 => {
            if scene.controller_id.len() > SCENE_SURFACE_ID_BYTE_CAPACITY {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            if cursor.byte() < scene.controller_id.len() {
                if cursor.advance_byte().is_err() {
                    return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
                }
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending;
            }
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        2 => {
            ctx.draw.set_screen_height(bounds.y + bounds.h);
            remember_scene_theme(ctx.theme);
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        3 => {
            if ctx.draw.try_reserve_retained_items(1).is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, bounds.h], ctx.theme.panel, ctx.theme.border_radius);
            if cursor.advance_item().is_err() || cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        4 => {
            if !semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::sync_engine_scene(scene, hosts.window_id, bounds, ctx.theme) {
                return cursor.finish();
            }
            let Some(token) = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_surface_token(&scene.host_id) else {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            };
            if !bind_scene_engine_token(&scene.host_id, token) {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        5 => {
            let progress = semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::advance_raster_composite(scene);
            if cursor.observe_external_progress(progress.completed).is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            if !progress.done {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending;
            }
            if !semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::stage_engine_scene_paint(scene, bounds, engine_surface_clear(scene.component_kind, ctx.theme)) {
                return cursor.finish();
            }
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        6 => {
            let Some(key) = semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::engine_raster_key(&scene.host_id) else {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            };
            if ctx.draw.try_reserve_retained_items(1).is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            ctx.draw.push_raster_quad(&key, [bounds.x, bounds.y, bounds.w, bounds.h], [0.0, 0.0, 1.0, 1.0], 1.0);
            if cursor.advance_item().is_err() || cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        7 => {
            if scene.component_kind == SurfaceKind::NodeGraph {
                ctx.draw.push_scissor(bounds);
                semio_framework_os_renderer_wgpu_engine_graph::node_graph::paint_node_graph_labels(ctx, scene, bounds);
                ctx.draw.pop_scissor();
            } else if !matches!(scene.component_kind, SurfaceKind::TextEditor | SurfaceKind::Paint2d) {
                return cursor.finish();
            }
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        8 => {
            if scene.component_kind == SurfaceKind::NodeGraph {
                semio_framework_os_renderer_wgpu_engine_graph::node_graph::paint_node_graph_overlays(ctx, scene, bounds);
            }
            if scene.component_kind == SurfaceKind::Paint2d {
                ctx.draw.push_scissor(bounds);
                semio_framework_os_renderer_wgpu_engine_graph::node_graph::paint_paint2d_overlays(ctx, scene, bounds);
                ctx.draw.pop_scissor();
            }
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        9 => {
            if scene.component_kind == SurfaceKind::NodeGraph {
                let Some(graph) = scene.node_graph.as_ref() else { return cursor.finish() };
                let Some(item) = graph.find_items.get(cursor.item()) else { return cursor.finish() };
                let find = ShellFindItem { id: item.id.clone(), label: item.label.clone(), description: None, category: Some(item.category.clone()), surface_id: scene.surface_id.clone(), node_id: item.id.clone() };
                if try_push_find_item(find).is_err() || cursor.advance_item().is_err() {
                    return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
                }
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending;
            }
            if scene.component_kind != SurfaceKind::TextEditor {
                return cursor.finish();
            }
            if ctx.draw.try_reserve_retained_items(TEXT_EDITOR_OVERLAY_ITEMS).is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            render_text_editor_overlays(scene, bounds, ctx);
            cursor.finish()
        }
        _ => cursor.finish(),
    }
}

/// 🎞️ The first cursor phase past the identifier/metrics/panel preamble — where a scene kind's own
/// content paint begins, whether that is an engine-texture attach (`sync_engine_scene`) or a direct
/// retained draw (`World3d`, `Canvas2d`, `InkCanvas`, `IconRender`).
const ENGINE_PHASE: u16 = 4;

/** 📃️ Paints one list-like component scene — `Table`, `VirtualFileSystem`, `GraphTimeline`,
 * `BlockList`, `DiffView`, `EventFeed` — straight into the window's retained draw list. Like the
 * `World3d`/`Canvas2d` branches there is no engine texture to attach: the whole surface is chrome,
 * so attach, paint and composite are one step. The per-row backing is pre-admitted first
 * (`reserve_list_rows`), sized from the surface's own visible band rather than its payload's row
 * count, because every one of these renderers culls to that band before it draws. */
fn render_list_scene_step(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    cursor: &mut semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintCursor,
    chrome_labels: SceneChromeLabels,
    window_id: &str,
    driver_drag: UiDriverDrag,
    document_generation: u64,
) -> semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep {
    cancel_scene_list_transfer_if_stale(window_id, &scene.host_id, document_generation, driver_drag);
    let capacity = list_visible_row_capacity(bounds, ctx.theme.control_height);
    if !reserve_list_rows(ctx, capacity) {
        return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
    }
    match scene.component_kind {
        SurfaceKind::Table => render_table(scene, bounds, ctx, driver_drag),
        SurfaceKind::VirtualFileSystem => render_vfs(scene, bounds, ctx, chrome_labels.virtual_file_system),
        SurfaceKind::GraphTimeline => render_graph_timeline(scene, bounds, ctx),
        SurfaceKind::BlockList => render_block_list(scene, bounds, ctx, driver_drag, chrome_labels.block_list),
        SurfaceKind::DiffView => render_diff_view(scene, bounds, ctx),
        SurfaceKind::EventFeed => render_event_feed(scene, bounds, ctx),
        _ => return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault,
    }
    cursor.finish()
}

/** 🖼️ Paints one `SurfaceKind::Canvas2d` scene straight into the window's retained draw list — no
 * engine texture, because a canvas-2d document IS a draw list (`drawSceneNode` in React walks the
 * same records into `CanvasRenderingContext2D` calls). Two phases: the content ladder, then the
 * surface hit target the generic pointer dispatch is admitted through.
 *
 * @see `🧱️elements/📐️Canvas2dHost/🟦️.tsx` — `drawSceneNode` */
fn render_canvas_2d_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintCursor) -> semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep {
    match cursor.phase() {
        ENGINE_PHASE => {
            if ctx.draw.try_reserve_retained_items(canvas_2d_reserve_items(scene)).is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            render_canvas_2d(scene, bounds, ctx);
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        _ => cursor.finish(),
    }
}

/// 🧾️ The retained-item grant one canvas-2d paint needs: grid + checkerboard ladder, plus a fixed
/// per-layer worst case (fill, outline, selection glow/ring, label).
fn canvas_2d_reserve_items(scene: &UiComponentSceneNode) -> usize {
    let layers = scene.canvas_2d.as_ref().map_or(0, |canvas| canvas.layers_json.len() / 64);
    CANVAS_2D_CHROME_ITEMS.saturating_add(layers.saturating_mul(CANVAS_2D_ITEMS_PER_LAYER))
}

const CANVAS_2D_CHROME_ITEMS: usize = 4096;
const CANVAS_2D_ITEMS_PER_LAYER: usize = 8;

/** 🖋️ Paints one `SurfaceKind::InkCanvas` scene straight into the window's retained draw list —
 * strokes, grid, block cards and the live marquee rectangle, all under the surface's own scissor.
 *
 * @see `🧱️elements/🖋️InkCanvasHost/🟦️.tsx` */
fn render_ink_canvas_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintCursor) -> semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep {
    match cursor.phase() {
        ENGINE_PHASE => {
            if ctx.draw.try_reserve_retained_items(INK_CANVAS_RESERVE_ITEMS).is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            render_ink_canvas(scene, bounds, ctx);
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        _ => cursor.finish(),
    }
}

/// 🧾️ The retained-item grant one ink paint needs — the grid ladder dominates, and the block count
/// is already capped by `INK_INTERACTION_ITEM_CAPACITY`.
const INK_CANVAS_RESERVE_ITEMS: usize = 8192;

/** 🖼️ Paints one `SurfaceKind::IconRender` scene: the GLB shot goes through the SAME world-3d pass
 * every `World3d` surface uses (React's `iconRenderPort` renders offscreen with the same renderer),
 * and the aspect-fit frame, size/shape badge and footer caption paint on top of it.
 *
 * @see `🧱️elements/🖼️IconRenderHost/🟦️.tsx` — `IconShotFrame` */
fn render_icon_render_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintCursor, hosts: &mut SceneEngineHosts<'_>) -> semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep {
    match cursor.phase() {
        ENGINE_PHASE => {
            if ctx.draw.try_reserve_retained_items(ICON_RENDER_RESERVE_ITEMS).is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            render_icon_render(scene, bounds, ctx, hosts);
            if cursor.advance_phase().is_err() {
                return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
            }
            semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Pending
        }
        _ => cursor.finish(),
    }
}

/// 🧾️ Frame border strips, badge chip, badge text and footer caption.
const ICON_RENDER_RESERVE_ITEMS: usize = 256;

/// 🎨️ The colour a composited engine texture clears to before its host paints — the panel behind a
/// node graph, the canvas ground behind a map or a board, exactly as each React host's own
/// `clear_color` resolves it.
fn engine_surface_clear(kind: SurfaceKind, theme: &semio_framework_ui_theme::Theme) -> semio_framework_ui_theme::Rgba {
    match kind {
        SurfaceKind::TiledMap | SurfaceKind::Board2d | SurfaceKind::Paint2d => theme.canvas_clear,
        _ => theme.panel,
    }
}

/// 🌍️ Attaches — and paints — the `World3dState` behind one `SurfaceKind::World3d` scene. Unlike the
/// vello-composited kinds this host paints a real 3-D pass straight into the window's own retained
/// draw list (`render_world_3d` → `DrawList::push_scene_pass`), so there is no engine texture and no
/// raster key: attach, paint and composite are the same step. The state lives in the shell's
/// `world3d_states` because every downstream ladder — asset fetch, snapshot apply, bounded pick/hover
/// authority, the OS event loop's orbit/wheel dispatch — addresses it there.
///
/// 🎥️ The one point per frame that holds both the surface id and its LIVE orbit — `dumpMeshStats`
/// reaches only the interpreter's `UI_ENGINE`, never the shell's `world3d_states`.
fn render_world3d_surface_step(scene: &UiComponentSceneNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintCursor, hosts: &mut SceneEngineHosts<'_>) -> semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep {
    let created = !hosts.world3d_states.contains_key(&scene.host_id);
    let surface_id = scene.surface_id.clone();
    let controller_id = scene.controller_id.clone();
    let Some(state) = hosts.world3d_states.get_or_insert_with(scene.host_id.clone(), || infinite_world::world::World3dState::new(surface_id, controller_id)) else {
        return semio_framework_ui_wgpu_hosting::scene_slots::ScenePaintStep::Fault;
    };
    if state.trace.tool_run_trace_window_id.as_deref() != Some(hosts.window_id) {
        state.trace.tool_run_trace_window_id = Some(hosts.window_id.to_string());
    }
    state.set_modelling_locale(hosts.chrome_labels.german);
    infinite_world::world::render_world_3d(scene, bounds, ctx, state, hosts.world_resources, infinite_world::world::World3dShadowProfile::World);
    stage_world3d_accessibility_controls(
        &scene.host_id,
        infinite_world::world::world3d_modelling_accessibility(&state.session, &state.overlay, scene).into_iter().map(|entry| World3dAccessibilityControl { key: entry.key, role: entry.role, label: entry.label, depth: entry.depth, polite: entry.polite }).collect(),
    );
    semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::register_engine_surface(scene, hosts.window_id, bounds, semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::EngineSurfaceKindDetail::World3d { status_json: scene.world_3d.as_ref().and_then(|world| world.status_json.clone()) }, created, None);
    world3d_surface_debug_log(scene, bounds, ctx, state);
    if semio_framework_trace::runtime_diagnostics_enabled() {
        semio_framework_os_renderer_wgpu_scene_state::note_world3d_live_camera(&scene.host_id, infinite_world::world::world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene));
    }
    cursor.finish()
}

/// 🌍️ `[TRACE] ` trace of what the World3d pass this step just pushed actually carries — the pass's
/// mesh draws, their instances, its line draws and the surface it belongs to. Temporary: it is the
/// only way to tell "no surface" from "a surface with no meshes" on 6118, where `eprintln!` is a
/// no-op inside the frame Worker.
///
/// 📐️ The ORIGIN belongs in this trace as much as the size: every pointer, wheel and pick the
/// surface receives is admitted by `bounds.contains(x, y)` in PAGE space, so a rect reported only
/// as `WxH` cannot be told apart from the same rect at the wrong origin — which is the shape a
/// silently undispatched hover/select/orbit takes (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-input-hit-runtime-2026-09-13.md` §10.5).
fn world3d_surface_debug_log(scene: &UiComponentSceneNode, bounds: Rect, ctx: &FrameworkWidgetContext<'_>, state: &infinite_world::world::World3dState) {
    if !semio_framework_trace::runtime_diagnostics_enabled() {
        return;
    }
    let payload = match scene.world_3d.as_ref() {
        Some(world) => {
            format!(
                "meshes={}b instances={}b delta={}b lanes=[{}] snapshot={} camera={}b",
                world.meshes_json.len(),
                world.instances_json.len(),
                world.instances_delta_json.as_ref().map_or(0, String::len),
                world.lanes.iter().map(|lane| format!("{}:{}", lane.lane, lane.bytes)).collect::<Vec<_>>().join(","),
                world.snapshot.is_some(),
                world.camera_json.len()
            ) + &format!(
                " meshesHead={:?} instancesHead={:?} camera={:?} status={:?}",
                &world.meshes_json[..world.meshes_json.len().min(700)],
                &world.instances_json[..world.instances_json.len().min(700)],
                &world.camera_json[..world.camera_json.len().min(120)],
                world.status_json.as_deref().map(|status| &status[..status.len().min(800)])
            )
        }
        None => "world3d=none".to_string(),
    };
    let geometry = match ctx.draw.scene_passes.last() {
        Some(pass) => format!(
            "draws={} translucent={} materials={} instances={} lines={} textured={} passes={}",
            pass.draws.len(),
            pass.translucent_draws.len(),
            pass.material_draws.len(),
            pass.draws.iter().chain(pass.translucent_draws.iter()).map(|draw| draw.instances.len()).sum::<usize>() + pass.material_draws.iter().map(|draw| draw.instances.len()).sum::<usize>(),
            pass.line_draws.len(),
            pass.textured_draws.len(),
            ctx.draw.scene_passes.len()
        ),
        None => "pass=none".to_string(),
    };
    let visual = infinite_world::world::world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene);
    debug_log_diagnostic(&format!(
        "[TRACE] world3d surface={} pane={:?} bounds={}x{}+{},{} {geometry} {} {} visual={} {payload}",
        scene.surface_id,
        scene.pane_id,
        bounds.w.round(),
        bounds.h.round(),
        bounds.x.round(),
        bounds.y.round(),
        state.ingest_census(),
        state.mesh_geometry_census(),
        visual
    ));
}

pub fn debug_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 🩺️ A PER-FRAME dump — printed only while `SEMIO_RUNTIME_DIAGNOSTICS` is armed, the same gate
/// `crate::host_base::log_debug_diagnostic` applies in the renderer. One `world3d surface=…` census per surface
/// per frame is two lines a frame on this playground alone, which is what buried the asset-decoder
/// boot fault (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md`).
fn debug_log_diagnostic(line: &str) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        debug_log(line);
    }
}

/** 🧭️ Surface kinds that already receive pointer/wheel input through their own bespoke per-frame host state (`world3d_states`/`node_graph_states`/`tiled_map_states`/`board2d_states`, driven directly by the OS event loop) and must not be double-dispatched through the generic `handle_scene_*` handlers below. `pub(crate)` so `interpreter::apply_scene_ui_command` (the real per-event `UiCommand::Scene` handler, and now the ONLY caller of `handle_scene_wheel`/`handle_scene_pointer_button`/`handle_scene_pointer_move` — see that fn's own doc comment) applies this SAME exclusion list. */
pub fn scene_has_bespoke_pointer_dispatch(kind: SurfaceKind) -> bool {
    matches!(kind, SurfaceKind::World3d | SurfaceKind::NodeGraph | SurfaceKind::TiledMap | SurfaceKind::Board2d)
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-render-entry/🦀️.rs"]
mod render_entry_tests;
//#endregion RenderEntry
