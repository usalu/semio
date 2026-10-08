//! 📼️ Puzzle 2d kind-emitting recorder: every editor command that edits the board states its user action as the
//! concrete `Puzzle2dMutation` kinds it consists of, one entity at a time, and never builds a whole board to
//! difference it against the document. The recorder keeps the document as it stands after the kinds recorded so far
//! (applied through the central applier), so a later row of the same gesture — a second clone's label, a second
//! wire's id — reads what the earlier rows made.

use super::{board_snapshot_edges, board_snapshot_nodes, new_edge_id, new_node_id, patched_field_value, puzzle2d_next_node_label, puzzle2d_placed_handles, unique_edge_id, unique_node_id};
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations as kinds;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::{Puzzle2dEdge, Puzzle2dHandle, Puzzle2dNode, Puzzle2dSnapshot, Puzzle2dTargetRegion};
use protocol::{DiffAlgebra, Mutation, MutationDiff};
use semio_framework_pack_json::{json, Value};
use semio_framework_value::paged::PagedUtf8;
use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

type Text = PagedUtf8<{ usize::MAX }>;

/// 🧬️ Decodes one entity record from its host `Value`; `None` when the typed model refuses it.
fn decode<T: semio_framework_value::FromValue>(value: &Value) -> Option<T> {
    T::from_value(semio_framework_pack_json::to_dsl_value(value)).ok()
}

/// 🙈️ The document field and value one flag write stores on a node, handle or edge: `hidden` is written as its
/// inverse, `visible`, the field the typed model carries; `locked` is stored as itself.
fn flag_entry(flag: &str, value: bool) -> (&'static str, bool) {
    if flag == "locked" {
        ("locked", value)
    } else {
        ("visible", !value)
    }
}

/// 📼️ The rows one editor action recorded on a document, and the document those rows leave.
pub struct Puzzle2dRecorder {
    working: Arc<Puzzle2dSnapshot>,
    view: OnceCell<Arc<Value>>,
    rows: Vec<Puzzle2dMutation>,
    created: Vec<String>,
}

impl Puzzle2dRecorder {
    /// 🎬️ A recorder over the document `base`, sharing its cached host view.
    pub fn new(base: &Puzzle2dPlaySnapshot) -> Self {
        let view = OnceCell::new();
        let _ = view.set(base.shared_value());
        Self { working: base.shared_typed(), view, rows: Vec::new(), created: Vec::new() }
    }

    /// 🎬️ A recorder over an owned typed document.
    pub fn from_typed(base: Arc<Puzzle2dSnapshot>) -> Self {
        Self { working: base, view: OnceCell::new(), rows: Vec::new(), created: Vec::new() }
    }

    /// 🧬️ The document after every row recorded so far.
    pub fn typed(&self) -> &Puzzle2dSnapshot {
        self.working.as_ref()
    }

    /// 🤝️ The document after every row recorded so far, shared.
    pub fn shared(&self) -> Arc<Puzzle2dSnapshot> {
        Arc::clone(&self.working)
    }

    /// 👁️ The host view of [`Self::typed`], materialized once per recorded state.
    pub fn value(&self) -> &Value {
        &**self.view.get_or_init(|| Arc::new(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self.working.as_ref()))))
    }

    /// 🧾️ The kinds recorded so far, in recording order.
    pub fn rows(&self) -> &[Puzzle2dMutation] {
        &self.rows
    }

    /// 🌱️ The ids of the nodes this recorder created.
    pub fn created_nodes(&self) -> &[String] {
        &self.created
    }

    /// 📦️ Ends the recording with its rows.
    pub fn into_rows(self) -> Vec<Puzzle2dMutation> {
        self.rows
    }

    /// 📦️ Ends the recording with the document it leaves and its rows.
    pub fn into_parts(self) -> (Arc<Puzzle2dSnapshot>, Vec<Puzzle2dMutation>) {
        (self.working, self.rows)
    }

    /// ✍️ Records one concrete kind: its diff on the current document is applied centrally and the kind joins the rows.
    /// A kind the document refuses (an Error or Fatal outcome) or that changes nothing is dropped; answers whether it was kept.
    pub fn record(&mut self, mutation: Puzzle2dMutation) -> bool {
        let (diff, messages) = Mutation::<Puzzle2dSnapshot>::diff(&mutation, self.working.as_ref()).into_parts();
        let refused = messages.iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error);
        if refused || <Puzzle2dDiff as DiffAlgebra<Puzzle2dSnapshot>>::is_empty(&diff) {
            <Puzzle2dDiff as MutationDiff<Puzzle2dSnapshot>>::retire_cold(diff);
            return false;
        }
        let applied = protocol::apply_diff(&diff, self.working.as_ref());
        <Puzzle2dDiff as MutationDiff<Puzzle2dSnapshot>>::retire_cold(diff);
        match applied {
            Ok(next) => {
                self.working = Arc::new(next);
                self.view = OnceCell::new();
                self.rows.push(mutation);
                true
            }
            Err(_) => false,
        }
    }

    /// 🌱️ Records the creation of one node from its host record; answers the created id.
    fn create(&mut self, node: &Value) -> Option<String> {
        let node = decode::<Puzzle2dNode>(node)?;
        let id = node.id.to_string_owner();
        if self.record(kinds::create_node(node, None)) {
            self.created.push(id.clone());
            Some(id)
        } else {
            None
        }
    }

    /// ➕️ Records a node of `kind` at the position `args` names, labelled with the next free name of its kind.
    pub fn add_node(&mut self, kind: Option<&str>, args: Option<&Value>) -> Option<String> {
        let node_kind = kind.unwrap_or("node");
        let label = puzzle2d_next_node_label(board_snapshot_nodes(self.value()), self.value(), node_kind);
        let id = unique_node_id(self.value(), new_node_id("node"));
        let number = |key: &str, fallback: f64| args.and_then(|value| value.get(key)).and_then(Value::as_f64).unwrap_or(fallback);
        let shape = args.and_then(|value| value.get("shape")).and_then(Value::as_str).unwrap_or("circle");
        let mut node = json!({ "id": id, "nodeKind": node_kind, "shape": shape, "x": number("x", 0.0), "y": number("y", 0.0), "text": label, "anchor": "fixed", "handles": [] });
        if shape == "rectangle" {
            node["width"] = json!(number("width", 48.0));
            node["height"] = json!(number("height", 48.0));
        } else {
            node["radius"] = json!(number("radius", 24.0));
        }
        if let Some(icon_kind) = args.and_then(|value| value.get("iconKind")) {
            node["iconKind"] = icon_kind.clone();
        }
        self.create(&node)
    }

    /// 🖌️ Records one brush placement: the node, then the `link` edge back to the handle it was placed from.
    pub fn place_brush(&mut self, payload: &Value) -> Option<String> {
        let node_id = unique_node_id(self.value(), payload.get("nodeId").and_then(Value::as_str).map_or_else(|| new_node_id("node"), str::to_string));
        let edge_id = unique_edge_id(self.value(), payload.get("edgeId").and_then(Value::as_str).map_or_else(|| new_node_id("edge"), str::to_string));
        let node_kind = payload.get("nodeKind").and_then(Value::as_str).unwrap_or("node");
        let number = |key: &str, fallback: f64| payload.get(key).and_then(Value::as_f64).unwrap_or(fallback);
        let shape = payload.get("shape").and_then(Value::as_str).unwrap_or("circle");
        let label = puzzle2d_next_node_label(board_snapshot_nodes(self.value()), self.value(), node_kind);
        let mut node = json!({ "id": node_id, "nodeKind": node_kind, "shape": shape, "x": number("x", 0.0), "y": number("y", 0.0), "text": label, "handles": puzzle2d_placed_handles(&node_id, payload.get("handles")) });
        if shape == "rectangle" {
            node["width"] = json!(number("width", 48.0));
            node["height"] = json!(number("height", 48.0));
        } else {
            node["radius"] = json!(number("radius", 24.0));
        }
        if let Some(icon) = payload.get("iconKind") {
            node["iconKind"] = icon.clone();
        }
        let placed = self.create(&node)?;
        let source = payload.get("sourceHandleId").and_then(Value::as_str).unwrap_or("");
        if !source.is_empty() {
            let target = format!("{node_id}:v{}", payload.get("targetHandleIndex").and_then(Value::as_u64).unwrap_or(0));
            self.record(kinds::connect_handles(edge_id.as_str().into(), source.into(), target.as_str().into(), Some("link".into()), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None, None));
        }
        Some(placed)
    }

    /// 🔗️ Records the connection of one existing edge record, with the visibility and lock flags it carries.
    pub fn connect_edge(&mut self, edge: Puzzle2dEdge) -> bool {
        let id = edge.id.clone();
        let connected = self.record(kinds::connect_handles(edge.id, edge.source, edge.target, edge.edge_kind, edge.gap, edge.shift, edge.rise, edge.rotation, edge.turn, edge.tilt, edge.x, edge.y, edge.source_tip, edge.target_tip, None));
        if connected && edge.visible.is_some() {
            self.record(kinds::change_edge_visible(id.clone(), edge.visible));
        }
        if connected && edge.locked.is_some() {
            self.record(kinds::change_edge_locked(id, edge.locked));
        }
        connected
    }

    /// 🔗️ Records a fresh default edge from `source` to `target`; answers its id.
    pub fn connect(&mut self, source: &str, target: &str, edge_kind: Option<&str>) -> Option<String> {
        let id = new_edge_id(self.value());
        self.record(kinds::connect_handles(id.as_str().into(), source.into(), target.into(), edge_kind.map(Text::from), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, None, None, None)).then_some(id)
    }

    /// 💔️ Records the disconnection of one edge.
    pub fn disconnect(&mut self, id: &str) -> bool {
        self.record(kinds::disconnect_handles(id.into()))
    }

    /// 🗑️ Records the removal of whatever `ids` name: nodes (with the edges on their handles), handles (with their
    /// edges), edges, and target regions — then every edge still hanging off a named node or handle id.
    pub fn delete_entities(&mut self, ids: &[String]) {
        let selected: HashSet<&str> = ids.iter().map(String::as_str).collect();
        let base = self.shared();
        for node in base.nodes.iter().filter(|node| selected.contains(node.id.as_str())) {
            self.record(kinds::delete_node(node.id.clone()));
        }
        for node in base.nodes.iter().filter(|node| !selected.contains(node.id.as_str())) {
            for handle in node.handles.iter().filter(|handle| selected.contains(handle.id.as_str())) {
                self.record(kinds::remove_node_handle(node.id.clone(), handle.id.clone()));
            }
        }
        let hanging: Vec<Text> = self.typed().edges.iter().filter(|edge| selected.contains(edge.id.as_str()) || selected.contains(edge.source.as_str()) || selected.contains(edge.target.as_str())).map(|edge| edge.id.clone()).collect();
        for id in hanging {
            self.record(kinds::disconnect_handles(id));
        }
        self.delete_regions(ids);
    }

    /// 🙈️ Records `flag` (`hidden` or `locked`) set to `value` on every node, handle and edge `ids` name.
    pub fn set_flag(&mut self, ids: &[String], flag: &str, value: bool) {
        let selected: HashSet<&str> = ids.iter().map(String::as_str).collect();
        let (key, value) = flag_entry(flag, value);
        let base = self.shared();
        for node in base.nodes.iter() {
            if selected.contains(node.id.as_str()) {
                self.record(if key == "locked" { kinds::change_node_locked(node.id.clone(), Some(value)) } else { kinds::change_node_visible(node.id.clone(), Some(value)) });
            }
            for handle in node.handles.iter().filter(|handle| selected.contains(handle.id.as_str())) {
                let mut next = handle.clone();
                if key == "locked" {
                    next.locked = Some(value);
                } else {
                    next.visible = Some(value);
                }
                self.record(kinds::replace_node_handle(node.id.clone(), handle.id.clone(), next));
            }
        }
        for edge in base.edges.iter().filter(|edge| selected.contains(edge.id.as_str())) {
            self.record(if key == "locked" { kinds::change_edge_locked(edge.id.clone(), Some(value)) } else { kinds::change_edge_visible(edge.id.clone(), Some(value)) });
        }
    }

    /// 🩹️ Records one inspector field written over the addressed nodes and handles: an absolute `value`, else the
    /// entity's own reading plus `delta`. An empty `ids` addresses every node; a handle is reached by its own id.
    pub fn patch_fields(&mut self, ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) {
        let negated = (field == "hidden").then(|| value.and_then(Value::as_bool).map(|hidden| Value::Bool(!hidden))).flatten();
        let (field, value) = if field == "hidden" { ("visible", negated.as_ref()) } else { (field, value) };
        let base = self.shared();
        for node in base.nodes.iter() {
            for handle in node.handles.iter().filter(|handle| ids.iter().any(|id| handle.id.eq_str(id))) {
                if let Some(next) = Self::modified::<Puzzle2dHandle>(handle, field, value, delta) {
                    self.record(kinds::replace_node_handle(node.id.clone(), handle.id.clone(), next));
                }
            }
            if !ids.is_empty() && !ids.iter().any(|id| node.id.eq_str(id)) {
                continue;
            }
            if let Some(next) = Self::modified::<Puzzle2dNode>(node, field, value, delta) {
                self.record_node_field(&next, field);
            }
        }
    }

    /// 🩹️ `entity` with `field` written, `None` when the field has no reading or the typed model refuses the result.
    fn patched<T: semio_framework_value::ToValue + semio_framework_value::FromValue>(entity: &T, field: &str, value: Option<&Value>, delta: Option<&Value>) -> Option<T> {
        let mut record = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(entity));
        let resolved = patched_field_value(&record, field, value, delta)?;
        record.as_object_mut()?.insert(field.to_string(), resolved);
        decode(&record)
    }

    /// 🏷️ The one node kind that owns `field`, carrying the value `next` reads.
    fn record_node_field(&mut self, next: &Puzzle2dNode, field: &str) {
        let id = next.id.clone();
        let mutation = match field {
            "x" | "y" => kinds::move_node(id, next.x, next.y),
            "shape" | "radius" | "width" | "height" => kinds::replace_node_geometry(id, next.shape.clone(), next.radius, next.width, next.height),
            "nodeKind" => kinds::change_node_kind(id, next.node_kind.clone()),
            "text" => kinds::edit_node_text(id, next.text.clone()),
            "iconKind" => kinds::change_node_icon(id, next.icon_kind.clone()),
            "scale" => kinds::scale_node(id, next.scale),
            "visible" => kinds::change_node_visible(id, next.visible),
            "locked" => kinds::change_node_locked(id, next.locked),
            "root" => kinds::change_node_root(id, next.root),
            "anchor" => kinds::change_node_anchor(id, next.anchor),
            _ => return,
        };
        self.record(mutation);
    }

    /// 👯️ Records clones of `nodes` (host records) offset by `offset`, each under a fresh id with fresh handle ids and the
    /// next free label of its kind, and of every `edges` record whose two endpoints were cloned. Answers the new node ids.
    pub fn clone_fragment(&mut self, nodes: &[Value], edges: &[Value], offset: (f64, f64)) -> Vec<String> {
        let mut remap: HashMap<String, String> = HashMap::new();
        let mut cloned = Vec::new();
        for source in nodes {
            let mut clone = source.clone();
            let old_id = source.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
            let new_id = unique_node_id(self.value(), new_node_id("node"));
            remap.insert(old_id, new_id.clone());
            let label = source.get("nodeKind").and_then(Value::as_str).map(|kind| puzzle2d_next_node_label(board_snapshot_nodes(self.value()), self.value(), kind));
            if let Some(object) = clone.as_object_mut() {
                object.insert("id".into(), json!(new_id));
                object.insert("x".into(), json!(source.get("x").and_then(Value::as_f64).unwrap_or(0.0) + offset.0));
                object.insert("y".into(), json!(source.get("y").and_then(Value::as_f64).unwrap_or(0.0) + offset.1));
                if let Some(handles) = object.get_mut("handles").and_then(Value::as_array_mut) {
                    for handle in handles.iter_mut() {
                        let old_handle_id = handle.get("id").and_then(Value::as_str).unwrap_or_default().to_string();
                        let suffix = old_handle_id.rsplit(':').next().unwrap_or(old_handle_id.as_str()).to_string();
                        let new_handle_id = format!("{new_id}:{suffix}");
                        remap.insert(old_handle_id, new_handle_id.clone());
                        if let Some(handle) = handle.as_object_mut() {
                            handle.insert("id".into(), json!(new_handle_id));
                        }
                    }
                }
                if let Some(label) = label {
                    object.remove("label");
                    object.insert("text".into(), json!(label));
                }
            }
            if let Some(id) = self.create(&clone) {
                cloned.push(id);
            }
        }
        for edge in edges {
            let (Some(source), Some(target)) = (remap.get(edge.get("source").and_then(Value::as_str).unwrap_or_default()), remap.get(edge.get("target").and_then(Value::as_str).unwrap_or_default())) else {
                continue;
            };
            let mut clone = edge.clone();
            let id = new_edge_id(self.value());
            if let Some(object) = clone.as_object_mut() {
                object.insert("id".into(), json!(id));
                object.insert("source".into(), json!(source));
                object.insert("target".into(), json!(target));
            }
            if let Some(edge) = decode::<Puzzle2dEdge>(&clone) {
                self.connect_edge(edge);
            }
        }
        cloned
    }

    /// 👯️ Records clones of the nodes `ids` name, offset by `+24/+24`, with the edges among them.
    pub fn duplicate(&mut self, ids: &[String]) -> Vec<String> {
        let selected: HashSet<&str> = ids.iter().map(String::as_str).collect();
        let nodes: Vec<Value> = board_snapshot_nodes(self.value()).iter().filter(|node| node.get("id").and_then(Value::as_str).is_some_and(|id| selected.contains(id))).cloned().collect();
        let edges: Vec<Value> = board_snapshot_edges(self.value()).to_vec();
        self.clone_fragment(&nodes, &edges, (24.0, 24.0))
    }

    /// 🗑️ Records the removal of the cut nodes `ids` with everything attached to them.
    pub fn cut(&mut self, ids: &[String]) {
        self.delete_entities(ids);
    }

    /// 🎯️ Records one target region of the resolved world rectangle; answers its id.
    pub fn paint_region(&mut self, x: f64, y: f64, width: f64, height: f64) -> Option<String> {
        let id = new_node_id("target-region");
        let region = Puzzle2dTargetRegion { id: id.as_str().into(), x, y, width, height, label: None, hidden: false, locked: false };
        self.record(kinds::create_target_region(region, None)).then_some(id)
    }

    /// 🖍️ Records one grid-snapped target region at `origin` sized in grid cells by `size`.
    pub fn paint_region_cells(&mut self, origin: (f64, f64), size: (f64, f64), grid_factor: f64) -> Option<String> {
        let grid = grid_factor.abs().max(0.1);
        let snapped = ((origin.0 / grid).round() * grid, (origin.1 / grid).round() * grid);
        self.paint_region(snapped.0, snapped.1, size.0.max(1.0) * grid, size.1.max(1.0) * grid)
    }

    /// 🚚️ Records the absolute pose `after` (`position` and `size`) for one unlocked target region.
    pub fn relocate_region(&mut self, id: &str, after: &Value) {
        let Some(region) = self.typed().target_regions.iter().find(|region| region.id.eq_str(id)) else {
            return;
        };
        if region.locked {
            return;
        }
        let pair = |key: &str| after.get(key).and_then(Value::as_array).filter(|values| values.len() >= 2).map(|values| (values[0].as_f64().unwrap_or(0.0), values[1].as_f64().unwrap_or(0.0)));
        if let Some((x, y)) = pair("position") {
            self.record(kinds::move_target_region(id.into(), x, y));
        }
        if let Some((width, height)) = pair("size") {
            self.record(kinds::resize_target_region(id.into(), width, height));
        }
    }

    /// 🚩️ Records `flag` (`hidden` or `locked`) set to `value` on every target region `ids` name.
    pub fn set_region_flag(&mut self, ids: &[String], flag: &str, value: bool) {
        if !matches!(flag, "hidden" | "locked") {
            return;
        }
        let present: Vec<Text> = self.typed().target_regions.iter().filter(|region| ids.iter().any(|id| region.id.eq_str(id))).map(|region| region.id.clone()).collect();
        for id in present {
            self.record(if flag == "locked" { kinds::change_target_region_locked(id, value) } else { kinds::change_target_region_hidden(id, value) });
        }
    }

    /// 🪦️ Records the removal of every target region `ids` name.
    pub fn delete_regions(&mut self, ids: &[String]) {
        let present: Vec<Text> = self.typed().target_regions.iter().filter(|region| ids.iter().any(|id| region.id.eq_str(id))).map(|region| region.id.clone()).collect();
        for id in present {
            self.record(kinds::delete_target_region(id));
        }
    }

    /// 🌀️ Records the poses `nodes` (host records with `id`, `x`, `y`) put their nodes at.
    pub fn place_nodes(&mut self,nodes:&[semio_framework_os_infinite::board::ports::directed::schema::snapshot::BoardNodeSnapshot]){for node in nodes{if let(Some(x),Some(y))=(node.x,node.y){if x.is_finite()&&y.is_finite(){self.record(kinds::move_node(node.id.clone(),x,y));}}}}

    /// 🧱️ Records ONE board-tool row (`regionCreate`, `regionResize`, `brushPlace`, `edgeCreate`, `edgeDelete`,
    /// `nodeDelete`) as the kinds it consists of. Answers whether `name` is a board-tool kind.
    pub fn fold_board_row(&mut self, name: &str, payload: &Value) -> bool {
        let read = |key: &str| payload.get(key).and_then(Value::as_f64).filter(|value| value.is_finite());
        let id = payload.get("id").and_then(Value::as_str).filter(|id| !id.is_empty());
        match name {
            "regionCreate" => {
                if let (Some(x), Some(y), Some(width), Some(height)) = (read("x"), read("y"), read("width"), read("height")) {
                    if width > 0.0 && height > 0.0 {
                        self.paint_region(x, y, width, height);
                    }
                }
            }
            "regionResize" => {
                if let (Some(id), Some(x), Some(y), Some(width), Some(height)) = (id, read("x"), read("y"), read("width"), read("height")) {
                    self.relocate_region(id, &json!({ "position": [x, y], "size": [width, height] }));
                }
            }
            "brushPlace" => {
                self.place_brush(payload);
            }
            "edgeCreate" => {
                if let Some(edge) = decode::<Puzzle2dEdge>(payload) {
                    self.connect_edge(edge);
                }
            }
            "nodeDelete" => {
                if let Some(id) = id {
                    self.delete_entities(&[id.to_string()]);
                }
            }
            "edgeDelete" => {
                if let Some(id) = id {
                    self.disconnect(id);
                }
            }
            _ => return false,
        }
        true
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
