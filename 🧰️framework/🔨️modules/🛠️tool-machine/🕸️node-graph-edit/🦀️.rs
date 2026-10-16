//! 🕸️ Node graph editing: the node drag record, the once and drag emitters and the node graph edit rows.

use super::*;

//#region 🔖️NodeDrag
/// ✋️ The `nodeGraphEdit` row operation of a released node drag (design §13.3).
pub const NODE_DRAG_OPERATION: &str = "move";
/// 🧾️ The closed field set of a [`NODE_DRAG_OPERATION`] row.
pub const NODE_DRAG_ROW_FIELDS: [&str; 5] = ["operation", "gestureId", "nodeIds", "dx", "dy"];

/// ✋️ The node-graph gesture record every node-graph host dispatches for a released node drag (design §13.3): the press
/// it closes, the nodes it moved, and the ONE offset every one of them moved by, relative to where it started. A guest
/// turns it into its own relative leaf (`drag-nodes`, `move-nodes`, …) and commits it through [`node_drag_commit`], so
/// editing the drag in history replays the offset on whatever base it lands on.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeDragRecord {
    pub gesture_id: String,
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

impl NodeDragRecord {
    /// 🧾️ Decodes one `{operation:"move", gestureId, nodeIds, dx, dy}` row; any other field, a missing one, an empty
    /// gesture or node id, no node or one node twice, or a non-finite offset is refused by name.
    pub fn from_row(row: &protocol::DslValue) -> Result<Self, String> {
        let fields = row.as_object().ok_or("a node drag row must be an object")?;
        if fields.len() != NODE_DRAG_ROW_FIELDS.len() || NODE_DRAG_ROW_FIELDS.iter().any(|field| fields.iter().filter(|(name, _)| name == field).count() != 1) {
            return Err(format!("a node drag row has exactly the fields {NODE_DRAG_ROW_FIELDS:?}"));
        }
        if row.get("operation").and_then(protocol::DslValue::as_str) != Some(NODE_DRAG_OPERATION) {
            return Err(format!("a node drag row is the `{NODE_DRAG_OPERATION}` operation"));
        }
        let gesture_id = row.get("gestureId").and_then(protocol::DslValue::as_str).filter(|id| !id.is_empty()).ok_or("a node drag row names its gestureId")?.to_string();
        let node_ids = row
            .get("nodeIds")
            .and_then(protocol::DslValue::as_array)
            .ok_or("a node drag row lists its nodeIds")?
            .iter()
            .map(|id| id.as_str().filter(|id| !id.is_empty()).map(str::to_string).ok_or("every nodeIds entry is a non-empty node id"))
            .collect::<Result<Vec<_>, _>>()?;
        if node_ids.is_empty() || node_ids.iter().enumerate().any(|(at, id)| node_ids[..at].contains(id)) {
            return Err("a node drag row names at least one node and each node once".into());
        }
        let offset = |field: &str| row.get(field).and_then(protocol::DslValue::as_f64).filter(|value| value.is_finite()).ok_or(format!("a node drag row's {field} is a finite number"));
        Ok(Self { gesture_id, node_ids, dx: offset("dx")?, dy: offset("dy")? })
    }

    /// 📤️ The row a host writes for this record.
    pub fn to_row(&self) -> protocol::DslValue {
        protocol::DslValue::object([
            ("operation".to_string(), protocol::DslValue::String(NODE_DRAG_OPERATION.to_string())),
            ("gestureId".to_string(), protocol::DslValue::String(self.gesture_id.clone())),
            ("nodeIds".to_string(), protocol::DslValue::Array(self.node_ids.iter().cloned().map(protocol::DslValue::String).collect())),
            ("dx".to_string(), protocol::DslValue::float(self.dx)),
            ("dy".to_string(), protocol::DslValue::float(self.dy)),
        ])
    }

    /// 🎚️ Whether the record moves anything: at least one node and a finite offset that is not zero.
    pub fn moves(&self) -> bool {
        !self.node_ids.is_empty() && self.dx.is_finite() && self.dy.is_finite() && (self.dx, self.dy) != (0.0, 0.0)
    }
}

/// 🛠️ The ONE node-drag machine: a released drag of the press `gesture` committed as ONE tool transaction of the guest's
/// net `leaves`, through the continuous-control [`Scrub`] (a release from rest is a one-shot press; a streaming host only
/// adds `Tick`s of the same press, each carrying the cumulative offset). The ref is minted from `actor` (the admission's
/// authoring seed), `clock` and `tool` (`<appId>#<verb>`). `None` when nothing is yielded: an empty drag leaves zero trace.
pub fn node_drag_commit<M: Clone + 'static>(tool: impl Into<String>, actor: ActorId, gesture: &str, leaves: Vec<M>, clock: HybridLogicalTimestamp) -> Option<(TransactionRef, Vec<M>)> {
    if leaves.is_empty() {
        return None;
    }
    match Scrub::start(tool, actor, "").send(ScrubInput::Commit { gesture: gesture.to_string(), leaves }, clock).ok()? {
        ToolStep::Committed(transaction, mutations) => Some((transaction, mutations)),
        ToolStep::Idle | ToolStep::Open | ToolStep::Aborted(..) | ToolStep::Empty(_) => None,
    }
}

/// ⏱️ The clock a guest mints its tool transaction refs and tool ticks from: the host's wall-clock millisecond at `logical`
/// (actor 0 — the admission's authoring seed, not the clock, names the writer).
pub fn authoring_clock(logical: u64) -> HybridLogicalTimestamp {
    HybridLogicalTimestamp { actor: 0, physical_ms: semio_framework_job::default_now_ms().unwrap_or(0), logical }
}

/// 🛠️ What a released node drag publishes ([`node_drag_emit`]): ONE tool transaction of its leaves, the leaves as a plain
/// edit (a view without command authority: no authoring seed), or nothing (no leaves: zero trace).
#[derive(Clone, Debug, PartialEq)]
pub enum NodeDragEmit<M> {
    Commit(TransactionRef, Vec<M>),
    Plain(Vec<M>),
    Nothing,
}

impl<M> NodeDragEmit<M> {
    /// 🔓️ The committed tool transaction and its leaves; `None` for a plain or an empty release.
    pub fn committed(self) -> Option<(TransactionRef, Vec<M>)> {
        match self {
            Self::Commit(transaction, leaves) => Some((transaction, leaves)),
            Self::Plain(_) | Self::Nothing => None,
        }
    }
}

/// 🛠️ The ONE node-drag emission of every guest: `leaves` of the press `gesture` through [`node_drag_commit`] as the tool
/// `<app_id>#<verb>`, minted from the admission's `authoring_seed` and [`authoring_clock`].
pub fn node_drag_emit<M: Clone + 'static>(app_id: &str, verb: &str, authoring_seed: &str, gesture: &str, leaves: Vec<M>) -> NodeDragEmit<M> {
    match node_drag_commit(format!("{app_id}#{verb}"), ActorId(authoring_seed.into()), gesture, leaves, authoring_clock(GESTURE_CLOCK_TICK.fetch_add(1, std::sync::atomic::Ordering::Relaxed))) {
        Some((transaction, leaves)) if !authoring_seed.is_empty() => NodeDragEmit::Commit(transaction, leaves),
        Some((_, leaves)) => NodeDragEmit::Plain(leaves),
        None => NodeDragEmit::Nothing,
    }
}
//#endregion 🔖️NodeDrag

//#region 🎯️Once
/// 🎯️ The ONE one-step tool (design §22.32): a single dispatch — a click that places, a keyboard nudge — yields `leaves` and
/// commits them as ONE tool transaction of `<app_id>#<verb>`, its ref minted from the admission's `authoring_seed` and
/// [`authoring_clock`]: the continuous-control machine driven as a press that opens and releases in one event
/// ([`node_drag_commit`]). Nothing yielded leaves zero trace; without an admission (a render or test view) the leaves publish
/// plainly.
pub fn tool_once_emit<M: Clone + 'static>(app_id: &str, verb: &str, authoring_seed: &str, leaves: Vec<M>) -> NodeDragEmit<M> {
    node_drag_emit(app_id, verb, authoring_seed, verb, leaves)
}
//#endregion 🎯️Once

//#region 🔖️NodeGraphEditRows
/// 📏️ The most rows one `nodeGraphEdit` dispatch carries.
pub const NODE_GRAPH_EDIT_MAX_ROWS: usize = 256;
/// 🧾️ The root fields a `nodeGraphEdit` dispatch may carry: its rows, and the press of a dragged inline slider that the
/// framework scrub machine reads (design §13.1).
pub const NODE_GRAPH_EDIT_ROOT_FIELDS: [&str; 4] = ["operations", SCRUB_GESTURE_ARG, SCRUB_COMMIT_ARG, SCRUB_ABORT_ARG];

/// 🔌️ The side of a node a variadic port is inserted on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodePortSide {
    Input,
    Output,
}

/// 🔗️ One node-graph gesture record every renderer dispatches as `nodeGraphEdit` arguments (design §13.3; schema
/// `🧬️schema/🔣️node-graph-edit-rows`): each row names its entities by id and carries an intent, never a
/// whole fixture. A guest maps every row to its own id-keyed leaf; adding a node stays each guest's own verb.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeGraphEditRow {
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    Disconnect { synapse_id: String },
    Move(NodeDragRecord),
    SetSlider { widget_id: String, value: f64 },
    InsertPort { node_id: String, side: NodePortSide, index: u32 },
    Delete { node_ids: Vec<String>, synapse_ids: Vec<String> },
}

impl NodeGraphEditRow {
    /// 🧾️ Decodes one closed row; an unknown operation, a field outside the row's schema, an empty id, a repeated id, a
    /// non-numeric or non-finite number, an unknown port side or a non-integer index is refused by name.
    pub fn from_row(row: &protocol::DslValue) -> Result<Self, String> {
        let operation = row.get("operation").and_then(protocol::DslValue::as_str).ok_or("a nodeGraphEdit row names its operation")?;
        let closed = |expected: &[&str]| {
            let fields = row.as_object().ok_or(format!("a nodeGraphEdit {operation} row must be an object"))?;
            match fields.len() == expected.len() && expected.iter().all(|field| fields.iter().filter(|(name, _)| name == field).count() == 1) {
                true => Ok(()),
                false => Err(format!("a nodeGraphEdit {operation} row has exactly the fields {expected:?}")),
            }
        };
        let text = |field: &str| row.get(field).and_then(protocol::DslValue::as_str).map(str::to_string).ok_or(format!("nodeGraphEdit {operation}.{field} must be a string"));
        let id = |field: &str| text(field).and_then(|value| if value.is_empty() { Err(format!("nodeGraphEdit {operation}.{field} must not be empty")) } else { Ok(value) });
        let ids = |field: &str| {
            let ids = row
                .get(field)
                .and_then(protocol::DslValue::as_array)
                .ok_or(format!("nodeGraphEdit {operation}.{field} must be an array"))?
                .iter()
                .map(|id| id.as_str().filter(|id| !id.is_empty()).map(str::to_string).ok_or(format!("nodeGraphEdit {operation}.{field} holds non-empty ids")))
                .collect::<Result<Vec<_>, _>>()?;
            match ids.iter().enumerate().any(|(at, id)| ids[..at].contains(id)) {
                true => Err(format!("nodeGraphEdit {operation}.{field} names each id once")),
                false => Ok(ids),
            }
        };
        match operation {
            "connect" => {
                closed(&["operation", "sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"])?;
                Ok(Self::Connect { source_node_id: id("sourceNodeId")?, source_port_id: text("sourcePortId")?, target_node_id: id("targetNodeId")?, target_port_id: text("targetPortId")? })
            }
            "disconnect" => {
                closed(&["operation", "synapseId"])?;
                Ok(Self::Disconnect { synapse_id: id("synapseId")? })
            }
            NODE_DRAG_OPERATION => NodeDragRecord::from_row(row).map(Self::Move),
            "setSlider" => {
                closed(&["operation", "widgetId", "value"])?;
                let value = row.get("value").and_then(protocol::DslValue::as_f64).filter(|value| value.is_finite()).ok_or("nodeGraphEdit setSlider.value must be a finite number")?;
                Ok(Self::SetSlider { widget_id: id("widgetId")?, value })
            }
            "insertPort" => {
                closed(&["operation", "nodeId", "side", "index"])?;
                let side = match text("side")?.as_str() {
                    "input" => NodePortSide::Input,
                    "output" => NodePortSide::Output,
                    other => return Err(format!("nodeGraphEdit insertPort.side is input or output, not {other:?}")),
                };
                let index = row.get("index").and_then(protocol::DslValue::as_f64).filter(|index| index.fract() == 0.0 && (0.0..=f64::from(u32::MAX)).contains(index)).ok_or("nodeGraphEdit insertPort.index must be a non-negative integer")?;
                Ok(Self::InsertPort { node_id: id("nodeId")?, side, index: index as u32 })
            }
            "delete" => {
                closed(&["operation", "nodeIds", "synapseIds"])?;
                let (node_ids, synapse_ids) = (ids("nodeIds")?, ids("synapseIds")?);
                match node_ids.is_empty() && synapse_ids.is_empty() {
                    true => Err("nodeGraphEdit delete names at least one node or synapse".into()),
                    false => Ok(Self::Delete { node_ids, synapse_ids }),
                }
            }
            other => Err(format!("nodeGraphEdit has no operation {other:?}")),
        }
    }
}

/// 🧾️ Decodes the arguments of one `nodeGraphEdit` dispatch: an object holding exactly its `operations` (at most
/// [`NODE_GRAPH_EDIT_MAX_ROWS`] closed rows) and optionally the scrub press fields. Any malformed row refuses the whole batch
/// before a guest maps anything; an empty batch (a host abort of a slider press) decodes to no rows.
pub fn node_graph_edit_rows(args: &protocol::DslValue) -> Result<Vec<NodeGraphEditRow>, String> {
    let root = args.as_object().ok_or("nodeGraphEdit arguments must be an object")?;
    if root.iter().any(|(field, _)| !NODE_GRAPH_EDIT_ROOT_FIELDS.contains(&field.as_str())) || root.iter().filter(|(field, _)| field == "operations").count() != 1 {
        return Err(format!("nodeGraphEdit arguments hold exactly an operations array and optionally {:?}", &NODE_GRAPH_EDIT_ROOT_FIELDS[1..]));
    }
    let rows = args.get("operations").and_then(protocol::DslValue::as_array).ok_or("nodeGraphEdit operations must be an array")?;
    if rows.len() > NODE_GRAPH_EDIT_MAX_ROWS {
        return Err(format!("nodeGraphEdit carries at most {NODE_GRAPH_EDIT_MAX_ROWS} rows"));
    }
    rows.iter().map(NodeGraphEditRow::from_row).collect()
}
//#endregion 🔖️NodeGraphEditRows
