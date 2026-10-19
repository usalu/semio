//! 🏷️ Node labels, ids and placement geometry: the display labels, the id allocation, the target regions, the handle geometry and kind compatibility and the clipboard and lock helpers. See [`super`].

use super::*;
use std::collections::HashSet;

/// 📐️ Upper bound a placement-tuning measure (contact tolerance, overlap budget) is clamped to —
/// half the default node footprint, past which every candidate would collide or none would.
pub const PUZZLE2D_PLACEMENT_MEASURE_MAX: f64 = 48.0;

/// 🔢️ A stepper's absolute `value`, else `current + delta`; `None` when the args name neither a
/// finite absolute nor a finite delta — the 2d twin of `puzzle3d_absolute_or_delta`.
pub fn puzzle2d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    if let Some(value) = args.and_then(|args| args.get("value")).and_then(Value::as_f64).filter(|value| value.is_finite()) {
        return Some(value);
    }
    args.and_then(|args| args.get("delta")).and_then(Value::as_f64).filter(|delta| delta.is_finite()).map(|delta| current + delta)
}

/// 🏷️ One node-kind catalog row's display name — the same `name` / `id` precedence the catalogue
/// panel renders, resolved against the document's own `meta.kindCatalogs` else its inferred rows.
pub fn puzzle2d_kind_catalog_label(snapshot: &Value, kind_id: &str) -> String {
    let inferred = inferred_kind_entries(snapshot, "nodes");
    let entries = kind_catalog_entries(snapshot, "nodes").unwrap_or(inferred.as_slice());
    entries
        .iter()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(kind_id))
        .and_then(|entry| entry.get("label").or_else(|| entry.get("name")).and_then(Value::as_str))
        .filter(|label| !label.is_empty())
        .map_or_else(|| kind_id.to_string(), str::to_string)
}

/// 🏷️ What one outliner row and one board glyph should read — authored label (`label`, else the
/// board's own `text`), else the kind's catalog name, else the raw id. Ported from puzzle3d's
/// `puzzle3d_object_display_label` (ticket 26/09/15/PUZZLE3D-OBJECT-TREE-LABELS).
pub fn puzzle2d_node_display_label(node: &Value, snapshot: &Value) -> String {
    for key in ["label", "text"] {
        if let Some(label) = node.get(key).and_then(Value::as_str).filter(|label| !label.is_empty()) {
            return label.to_string();
        }
    }
    node.get("nodeKind")
        .and_then(Value::as_str)
        .map(|kind| puzzle2d_kind_catalog_label(snapshot, kind))
        .filter(|label| !label.is_empty())
        .unwrap_or_else(|| node.get("id").and_then(Value::as_str).unwrap_or("node").to_string())
}

/// 🏷️ What a reference to the board entity `id` (one of `kinds`: `node`, `targetRegion`, `edge`, `handle`; every kind when
/// none is declared) reads in `snapshot` — a node its display label ([`puzzle2d_node_display_label`]), a target region its
/// authored label, an edge its two endpoint nodes, a handle its node and handle kind. Authored and catalogued names are
/// data, the same in every locale; `None` when only the raw id would name the entity.
pub fn puzzle2d_entity_label(snapshot: &Value, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    let wants = |kind: &str| kinds.is_empty() || kinds.iter().any(|declared| declared == kind);
    let by_id = |entities: &[Value]| entities.iter().find(|entity| entity.get("id").and_then(Value::as_str) == Some(id)).cloned();
    let node_of = |node_id: &str| board_snapshot_nodes(snapshot).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(node_id)).map(|node| puzzle2d_node_display_label(node, snapshot)).unwrap_or_else(|| node_id.to_string());
    let label = if let Some(node) = wants(PUZZLE2D_GRANULARITY_NODE).then(|| by_id(board_snapshot_nodes(snapshot))).flatten() {
        Some(puzzle2d_node_display_label(&node, snapshot))
    } else if let Some(region) = wants(PUZZLE2D_GRANULARITY_TARGET_REGION).then(|| by_id(snapshot_target_regions(snapshot))).flatten() {
        region.get("label").and_then(Value::as_str).filter(|label| !label.is_empty()).map(str::to_string)
    } else if let Some(edge) = wants(PUZZLE2D_GRANULARITY_EDGE).then(|| by_id(board_snapshot_edges(snapshot))).flatten() {
        let endpoint = |key: &str| edge.get(key).and_then(Value::as_str).map(|handle| node_of(handle.split_once(':').map_or(handle, |(node, _)| node)));
        endpoint("source").zip(endpoint("target")).map(|(source, target)| format!("{source} → {target}"))
    } else if wants(PUZZLE2D_GRANULARITY_HANDLE) {
        board_snapshot_nodes(snapshot).iter().find_map(|node| {
            let handle = node.get("handles").and_then(Value::as_array)?.iter().find(|handle| handle.get("id").and_then(Value::as_str) == Some(id))?;
            Some(format!("{} \u{b7} {}", puzzle2d_node_display_label(node, snapshot), handle.get("handleKind").and_then(Value::as_str).unwrap_or(id)))
        })
    } else {
        None
    };
    label.filter(|label| label != id).map(|label| LocalizedLabel::data(&label))
}

fn puzzle2d_label_root(label: &str) -> String {
    let Some((base, suffix)) = label.rsplit_once(' ') else {
        return label.to_string();
    };
    if suffix.parse::<u32>().is_ok() {
        base.to_string()
    } else {
        label.to_string()
    }
}

fn puzzle2d_label_number(label: &str, root: &str) -> Option<u32> {
    if label == root {
        return Some(1);
    }
    label.strip_prefix(root)?.strip_prefix(' ')?.parse().ok()
}

/// 🔢️ The next distinct node label for one kind — the first instance takes the catalog name, further
/// ones append ` 2`, ` 3`, … to the root taken from its peers. Every creation path (`addNode`, the
/// brush commit, the fill placement, duplicate, paste) stamps this so a document never shows two rows
/// reading the same word. Ported from `puzzle3d_next_object_label`.
pub fn puzzle2d_next_node_label(nodes: &[Value], snapshot: &Value, kind_id: &str) -> String {
    let catalog_base = puzzle2d_kind_catalog_label(snapshot, kind_id);
    let peers: Vec<&Value> = nodes.iter().filter(|node| node.get("nodeKind").and_then(Value::as_str) == Some(kind_id)).collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let authored = |node: &&Value| node.get("label").or_else(|| node.get("text")).and_then(Value::as_str).filter(|label| !label.is_empty()).map(str::to_string);
    let root = peers.iter().find_map(authored).map(|label| puzzle2d_label_root(&label)).unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for node in peers {
        match authored(&node) {
            Some(label) => {
                if let Some(number) = puzzle2d_label_number(&label, &root) {
                    max = max.max(number);
                }
            }
            None => unlabeled += 1,
        }
    }
    max = max.max(unlabeled);
    if max == 0 {
        root
    } else {
        format!("{root} {}", max + 1)
    }
}

pub fn puzzle2d_kind_ids(snapshot: &Value, field: &str) -> Vec<String> {
    let inferred = inferred_kind_entries(snapshot, field);
    let entries = kind_catalog_entries(snapshot, field).unwrap_or(inferred.as_slice());
    entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

pub fn new_node_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    format!("{prefix}-{}", NEXT.fetch_add(1, Ordering::Relaxed))
}

pub fn puzzle_extension_id() -> &'static str {
    let _extension = Puzzle2dExtension;
    "puzzle.2d"
}

/// 🙈️ Whether a node, handle or edge is hidden. The document stores the inverse, `visible` (absent = shown): a `hidden`
/// key written onto one of them never reaches the typed model, so it would neither persist nor reach a peer.
pub fn puzzle2d_entity_hidden(entity: &Value) -> bool {
    entity.get("visible").and_then(Value::as_bool) == Some(false)
}

/// 🎯️ Every node/handle id sharing a `nodeKind`/`handleKind` with anything currently selected.
pub fn select_same_kind_ids(snapshot: &Value, selected: &[String]) -> Vec<String> {
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut node_kinds: HashSet<&str> = HashSet::new();
    let mut handle_kinds: HashSet<&str> = HashSet::new();
    for node in board_snapshot_nodes(snapshot) {
        if node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
            if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                node_kinds.insert(kind);
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
                if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                    handle_kinds.insert(kind);
                }
            }
        }
    }
    let mut ids: Vec<String> = Vec::new();
    for node in board_snapshot_nodes(snapshot) {
        if node.get("nodeKind").and_then(|value| value.as_str()).is_some_and(|kind| node_kinds.contains(kind)) {
            if let Some(id) = node.get("id").and_then(|value| value.as_str()) {
                ids.push(id.to_string());
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("handleKind").and_then(|value| value.as_str()).is_some_and(|kind| handle_kinds.contains(kind)) {
                if let Some(id) = handle.get("id").and_then(|value| value.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }
    }
    ids
}

/// 🎥️ Writes an `{ x, y, zoom }` camera payload into the config — session-only view state, never the snapshot.
pub fn set_runtime_camera(runtime: &mut Puzzle2dPlayRuntime, camera: &Value) {
    if let Some(x) = camera.get("x").and_then(Value::as_f64) {
        runtime.camera_x = x;
    }
    if let Some(y) = camera.get("y").and_then(Value::as_f64) {
        runtime.camera_y = y;
    }
    if let Some(zoom) = camera.get("zoom").and_then(Value::as_f64) {
        runtime.camera_zoom = zoom;
    }
}

/// 📐️ The value one inspector row asks an entity's `field` to take: an absolute `value` wins,
/// otherwise a numeric `delta` rides on that entity's own current reading — offset-preserving across
/// a multi-select whose members start apart.
pub fn patched_field_value(entity: &Value, field: &str, value: Option<&Value>, delta: Option<&Value>) -> Option<Value> {
    if let Some(absolute) = value {
        return Some(absolute.clone());
    }
    delta.and_then(Value::as_f64).map(|delta| json!(entity.get(field).and_then(Value::as_f64).unwrap_or(0.0) + delta))
}

/// 🎯️ The document's fill-constraining rectangles, or an empty slice when the board declares none.
pub fn snapshot_target_regions(snapshot: &Value) -> &[Value] {
    snapshot.get("targetRegions").and_then(Value::as_array).map_or(&[], Vec::as_slice)
}

/// 🎯️ The region ids inside `selected` — the framework selection is one flat id list per domain, so a
/// region is simply a selected id the document holds a region for.
pub fn puzzle2d_selected_target_region_ids(snapshot: &Value, selected: &[String]) -> Vec<String> {
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    snapshot_target_regions(snapshot).iter().filter_map(|region| region.get("id").and_then(Value::as_str)).filter(|id| selected.contains(id)).map(str::to_string).collect()
}

/// 📐️ Normalized `[min_x, min_y, max_x, max_y]` of one region entry — the `Value` twin of
/// [`crate::Puzzle2dTargetRegion::bounds`], so the brush's corner order never reaches a reader.
pub fn puzzle2d_region_bounds(region: &Value) -> [f64; 4] {
    let read = |key: &str| region.get(key).and_then(Value::as_f64).unwrap_or(0.0);
    let (x, y, width, height) = (read("x"), read("y"), read("width"), read("height"));
    let (min_x, max_x) = if width < 0.0 { (x + width, x) } else { (x, x + width) };
    let (min_y, max_y) = if height < 0.0 { (y + height, y) } else { (y, y + height) };
    [min_x, min_y, max_x, max_y]
}

/// 🎯️ Whether ANY visible region of this document contains the axis-aligned box — the fill rule, on
/// the `Value` snapshot. An empty visible set is unconstrained, exactly as in puzzle3d.
pub fn puzzle2d_snapshot_regions_admit(snapshot: &Value, aabb: [f64; 4]) -> bool {
    let visible: Vec<&Value> = snapshot_target_regions(snapshot).iter().filter(|region| region.get("hidden").and_then(Value::as_bool) != Some(true)).collect();
    if visible.is_empty() {
        return true;
    }
    visible.into_iter().any(|region| {
        let bounds = puzzle2d_region_bounds(region);
        aabb[0] >= bounds[0] && aabb[1] >= bounds[1] && aabb[2] <= bounds[2] && aabb[3] <= bounds[3]
    })
}

/// 🖍️ The Area Brush's W/H steppers as WORLD extent: the steppers count grid cells, the engine and
/// the board both work in board units, and [`Puzzle2dRecorder::paint_region_cells`] uses the same product —
/// so a click on the canvas and a dispatch of `addTargetRegion` paint the identical rectangle.
pub fn puzzle2d_area_brush_extent_world(runtime: &Puzzle2dPlayRuntime) -> (f64, f64) {
    let grid = runtime.grid_factor.abs().max(0.1);
    (runtime.area_brush_width.max(1.0) * grid, runtime.area_brush_height.max(1.0) * grid)
}

/// 🎲️ Re-mints a node id when it collides with an existing one — client-side brush serials restart every session.
pub fn unique_node_id(snapshot: &Value, candidate: String) -> String {
    if board_snapshot_nodes(snapshot).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("node")
    } else {
        candidate
    }
}

pub fn unique_edge_id(snapshot: &Value, candidate: String) -> String {
    if board_snapshot_edges(snapshot).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("edge")
    } else {
        candidate
    }
}

/// 🔖️ The handles of a brush-placed node, each carrying the `"{node}:v{index}"` id the placed edge
/// already addresses ([`Puzzle2dRecorder::place_brush`]'s `target`). The engine's `brushPlace` payload
/// describes handles by kind/angle/radius only; an id-less handle is not a `Puzzle2dHandle` at all, so
/// the placed node would make the whole board fail to decode.
pub fn puzzle2d_placed_handles(node_id: &str, handles: Option<&Value>) -> Value {
    let rows = handles.and_then(Value::as_array).map(|rows| rows.iter().enumerate().map(|(index, handle)| {
        let mut handle = handle.clone();
        if let Some(object) = handle.as_object_mut() {
            if object.get("id").and_then(Value::as_str).is_none_or(str::is_empty) {
                object.insert("id", json!(format!("{node_id}:v{index}")));
            }
        }
        handle
    }).collect::<Vec<_>>()).unwrap_or_default();
    Value::Array(rows)
}

/// ➡️ A fresh edge id no edge of `snapshot` already carries.
pub fn new_edge_id(snapshot: &Value) -> String {
    unique_edge_id(snapshot, new_node_id("edge"))
}

/// ➡️ Appends one edge to a SCRATCH host view, creating the `edges` array when a hand-written document omitted it — only
/// the select tool's proximity bookkeeping calls it, never a command that edits the document.
pub fn puzzle2d_push_edge(snapshot: &mut Value, edge: Value) {
    puzzle2d_push_entity(snapshot, "edges", edge);
}

fn puzzle2d_push_entity(snapshot: &mut Value, key: &str, entity: Value) {
    let Some(object) = snapshot.as_object_mut() else {
        return;
    };
    if !object.contains_key(key) {
        object.insert(key, json!([]));
    }
    if let Some(entities) = object.get_mut(key).and_then(Value::as_array_mut) {
        entities.push(entity);
    }
}

/// 📐️ Board units from a node's centre to its furthest possible handle — the circle's radius, or the
/// rectangle's half-diagonal. Every handle of the node lies inside this circle whatever its angle, so
/// a proximity search can reject a whole node on its centre alone.
pub fn puzzle2d_node_reach(node: &Value) -> f64 {
    if node.get("shape").and_then(Value::as_str) == Some("rectangle") {
        let half_width = node.get("width").and_then(Value::as_f64).unwrap_or(48.0) / 2.0;
        let half_height = node.get("height").and_then(Value::as_f64).unwrap_or(48.0) / 2.0;
        return half_width.hypot(half_height);
    }
    node.get("radius").and_then(Value::as_f64).unwrap_or(24.0).abs()
}

/// 📍️ World position of one handle on its node — the SAME rim geometry the board engine draws with
/// (`handle_position_on_circle`'s east-zero angle, `handle_position_on_rectangle`'s north-zero one),
/// so a proximity hit is exactly a visual touch.
pub fn puzzle2d_handle_world_position(node: &Value, handle: &Value) -> (f64, f64) {
    let centre = Point::new(node.get("x").and_then(Value::as_f64).unwrap_or(0.0), node.get("y").and_then(Value::as_f64).unwrap_or(0.0));
    let angle = handle.get("angle").and_then(Value::as_f64).unwrap_or(0.0);
    let point = if node.get("shape").and_then(Value::as_str) == Some("rectangle") {
        handle_position_on_rectangle(centre, node.get("width").and_then(Value::as_f64).unwrap_or(48.0), node.get("height").and_then(Value::as_f64).unwrap_or(48.0), angle)
    } else {
        handle_position_on_circle(centre, node.get("radius").and_then(Value::as_f64).unwrap_or(24.0), angle)
    };
    (point.x, point.y)
}

/// 🔌️ The `handleKind` of one handle id (empty string when the handle declares none), `None` when
/// the document carries no such handle at all.
pub fn puzzle2d_handle_kind(snapshot: &Value, handle_id: &str) -> Option<String> {
    board_snapshot_nodes(snapshot)
        .iter()
        .flat_map(|node| node.get("handles").and_then(Value::as_array).into_iter().flatten())
        .find(|handle| handle.get("id").and_then(Value::as_str) == Some(handle_id))
        .map(|handle| handle.get("handleKind").and_then(Value::as_str).unwrap_or_default().to_string())
}

/// 🤝️ Whether `meta.kindCompatibility` admits a `source → target` handle-kind link. A document that
/// declares no rules is permissive (the board engine's own linking gate behaves the same), and a
/// handle carrying no kind is admitted by every rule set.
pub fn puzzle2d_kinds_compatible(snapshot: &Value, source_kind: &str, target_kind: &str) -> bool {
    let rows = snapshot.get("meta").and_then(|meta| meta.get("kindCompatibility")).or_else(|| snapshot.get("kindCompatibility")).and_then(Value::as_array);
    let Some(rows) = rows.filter(|rows| !rows.is_empty()) else {
        return true;
    };
    if source_kind.is_empty() || target_kind.is_empty() {
        return true;
    }
    rows.iter().any(|row| {
        let (Some(source), Some(target)) = (row.get("source").and_then(Value::as_str), row.get("target").and_then(Value::as_str)) else {
            return false;
        };
        let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
        (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind)
    })
}

/// 🔗️ Every handle id an edge already ends on — an occupied handle refuses a second connection.
pub fn puzzle2d_occupied_handles(snapshot: &Value) -> HashSet<String> {
    board_snapshot_edges(snapshot)
        .iter()
        .flat_map(|edge| [edge.get("source"), edge.get("target")])
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

/// 📋️ The fragment schema a puzzle 2d copy writes and a puzzle 2d paste accepts.
pub const PUZZLE2D_CLIPBOARD_SCHEMA: &str = "puzzle.2d.clipboard.v1";

/// 📋️ Board units a paste with no explicit placement offsets the clone by — the same nudge
/// `duplicateSelection` uses, so a pasted copy never lands exactly on its source.
pub const PUZZLE2D_PASTE_OFFSET: f64 = 24.0;

pub fn puzzle2d_clipboard_media_type() -> MediaType {
    MediaType { class: MediaClass::TwoD, form: MediaForm::Design }
}

/// 🕹️ The node ids one selection copies: selected node ids, plus the parent node of every selected
/// handle id — a handle-granularity selection still copies the whole node it belongs to, exactly as
/// puzzle3d's `puzzle3d_selected_objects_from` resolves a vortex selection to its object.
pub fn puzzle2d_selected_node_ids(snapshot: &Value, selected: &[String]) -> Vec<String> {
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    board_snapshot_nodes(snapshot)
        .iter()
        .filter(|node| {
            let own = node.get("id").and_then(Value::as_str).is_some_and(|id| selected.contains(id));
            own || node.get("handles").and_then(Value::as_array).into_iter().flatten().any(|handle| handle.get("id").and_then(Value::as_str).is_some_and(|id| selected.contains(id)))
        })
        .filter_map(|node| node.get("id").and_then(Value::as_str).map(str::to_string))
        .collect()
}

/// 🔐️ Whether ANY addressed entity — node, handle, edge or target region — carries `locked: true`.
/// The one predicate every destructive/moving verb asks before it touches the document, so a lock is
/// the same promise for `deleteSelection`, a board drag, the rotate ring, the three transform verbs
/// and an inspector patch. Wider than [`puzzle2d_selection_is_locked`], which only answers for the
/// node/handle pair the clipboard's cut fragment is built from.
pub fn puzzle2d_addresses_locked_entity(snapshot: &Value, ids: &[String]) -> bool {
    if ids.is_empty() {
        return false;
    }
    let addressed: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let locked = |entity: &Value| entity.get("locked").and_then(Value::as_bool) == Some(true);
    let hits = |entity: &Value| entity.get("id").and_then(Value::as_str).is_some_and(|id| addressed.contains(id));
    let node_locked = board_snapshot_nodes(snapshot).iter().any(|node| {
        (hits(node) && locked(node)) || node.get("handles").and_then(Value::as_array).into_iter().flatten().any(|handle| hits(handle) && locked(handle))
    });
    node_locked || board_snapshot_edges(snapshot).iter().any(|edge| hits(edge) && locked(edge)) || snapshot_target_regions(snapshot).iter().any(|region| hits(region) && locked(region))
}

/// 🔒️ Whether any of these nodes (or one of their handles) refuses to be cut.
pub fn puzzle2d_selection_is_locked(snapshot: &Value, node_ids: &[String]) -> bool {
    let ids: HashSet<&str> = node_ids.iter().map(String::as_str).collect();
    board_snapshot_nodes(snapshot).iter().filter(|node| node.get("id").and_then(Value::as_str).is_some_and(|id| ids.contains(id))).any(|node| {
        node.get("locked").and_then(Value::as_bool) == Some(true) || node.get("handles").and_then(Value::as_array).into_iter().flatten().any(|handle| handle.get("locked").and_then(Value::as_bool) == Some(true))
    })
}
