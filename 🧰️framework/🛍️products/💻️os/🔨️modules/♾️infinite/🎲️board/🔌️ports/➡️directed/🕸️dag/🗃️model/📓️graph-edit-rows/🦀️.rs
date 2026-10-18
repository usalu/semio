//! 📓️ Graph edit rows of the DAG model: the journal refusal, the row sink, the graph edit row writers and strings, the drag gesture id and the edit capacity. See [`super`].

use super::*;

/// 🚫️ A gesture refused whole because its rows (`rows`) outgrow one `nodeGraphEdit` dispatch (`limit`,
/// [`DAG_GRAPH_EDIT_CAPACITY`]): nothing was journalled and the embedding host restores the gesture's baseline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DagJournalRefusal {
    pub rows: usize,
    pub limit: usize,
}

/// ✍️ Where the ONE row encoder ([`write_dag_graph_edit_rows`]) writes a row: a JSON answer for React and the flow wasm
/// session ([`dag_graph_edit_rows_json`]), the bounded action builder for wgpu, a string census for credit pricing.
pub trait DagGraphEditRowSink<'a> {
    type Error;
    fn begin_row(&mut self) -> Result<(), Self::Error>;
    fn text(&mut self, field: &'static str, value: &'a str) -> Result<(), Self::Error>;
    fn number(&mut self, field: &'static str, value: f64) -> Result<(), Self::Error>;
    fn integer(&mut self, field: &'static str, value: u64) -> Result<(), Self::Error>;
    fn texts(&mut self, field: &'static str, values: &'a [String]) -> Result<(), Self::Error>;
    fn end_row(&mut self) -> Result<(), Self::Error>;
}

/// 🧾️ THE node-graph row encoder (design §13.3): writes every edit as its `nodeGraphEdit` row into `sink` — the one
/// place a row's operation, fields and their order are spelled: `connect {sourceNodeId, sourcePortId, targetNodeId,
/// targetPortId}`, `disconnect {synapseId}`, `move {gestureId, nodeIds, dx, dy}`, `setSlider {widgetId, value}`,
/// `insertPort {nodeId, side, index}`.
pub fn write_dag_graph_edit_rows<'a, S: DagGraphEditRowSink<'a>>(edits: &'a [DagGraphEdit], sink: &mut S) -> Result<(), S::Error> {
    for edit in edits {
        sink.begin_row()?;
        match edit {
            DagGraphEdit::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                sink.text("operation", "connect")?;
                sink.text("sourceNodeId", source_node_id)?;
                sink.text("sourcePortId", source_port_id)?;
                sink.text("targetNodeId", target_node_id)?;
                sink.text("targetPortId", target_port_id)?;
            }
            DagGraphEdit::Disconnect { synapse_id } => {
                sink.text("operation", "disconnect")?;
                sink.text("synapseId", synapse_id)?;
            }
            DagGraphEdit::Move { gesture_id, node_ids, dx, dy } => {
                sink.text("operation", "move")?;
                sink.text("gestureId", gesture_id)?;
                sink.texts("nodeIds", node_ids)?;
                sink.number("dx", *dx)?;
                sink.number("dy", *dy)?;
            }
            DagGraphEdit::SetSlider { node_id, value } => {
                sink.text("operation", "setSlider")?;
                sink.text("widgetId", node_id)?;
                sink.number("value", *value)?;
            }
            DagGraphEdit::InsertPort { node_id, side, index } => {
                sink.text("operation", "insertPort")?;
                sink.text("nodeId", node_id)?;
                sink.text("side", side.as_str())?;
                sink.integer("index", *index as u64)?;
            }
        }
        sink.end_row()?;
    }
    Ok(())
}

/// 🧾️ The JSON sink of [`write_dag_graph_edit_rows`].
#[derive(Default)]
struct DagGraphEditJsonRows {
    pub rows: Vec<Value>,
    pub row: Vec<(String, Value)>,
}

impl<'a> DagGraphEditRowSink<'a> for DagGraphEditJsonRows {
    type Error = std::convert::Infallible;
    fn begin_row(&mut self) -> Result<(), Self::Error> {
        self.row.clear();
        Ok(())
    }
    fn text(&mut self, field: &'static str, value: &'a str) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), Value::from(value)));
        Ok(())
    }
    fn number(&mut self, field: &'static str, value: f64) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), Value::from(value)));
        Ok(())
    }
    fn integer(&mut self, field: &'static str, value: u64) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), Value::from(value)));
        Ok(())
    }
    fn texts(&mut self, field: &'static str, values: &'a [String]) -> Result<(), Self::Error> {
        self.row.push((field.to_string(), semio_framework_pack_json::array(values.iter().map(|value| Value::from(value.as_str())))));
        Ok(())
    }
    fn end_row(&mut self) -> Result<(), Self::Error> {
        self.rows.push(semio_framework_pack_json::object(std::mem::take(&mut self.row)));
        Ok(())
    }
}

/// 🧮️ The string census sink of [`write_dag_graph_edit_rows`]: every field name and text value a row carries.
#[derive(Default)]
struct DagGraphEditRowStrings<'a>(Vec<&'a str>);

impl<'a> DagGraphEditRowSink<'a> for DagGraphEditRowStrings<'a> {
    type Error = std::convert::Infallible;
    fn begin_row(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn text(&mut self, field: &'static str, value: &'a str) -> Result<(), Self::Error> {
        self.0.extend([field, value]);
        Ok(())
    }
    fn number(&mut self, field: &'static str, _value: f64) -> Result<(), Self::Error> {
        self.0.push(field);
        Ok(())
    }
    fn integer(&mut self, field: &'static str, _value: u64) -> Result<(), Self::Error> {
        self.0.push(field);
        Ok(())
    }
    fn texts(&mut self, field: &'static str, values: &'a [String]) -> Result<(), Self::Error> {
        self.0.push(field);
        self.0.extend(values.iter().map(String::as_str));
        Ok(())
    }
    fn end_row(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// 🧮️ Every field name and text value the rows of `edits` carry, for a bounded writer to price its string credits before
/// it builds them.
pub fn dag_graph_edit_row_strings(edits: &[DagGraphEdit]) -> Vec<&str> {
    let mut strings = DagGraphEditRowStrings::default();
    if let Err(never) = write_dag_graph_edit_rows(edits, &mut strings) {
        match never {}
    }
    strings.0
}

/// 🧾️ A gesture's journal answer as the JSON a renderer reads: the `nodeGraphEdit` arguments `{"operations":[…]}` it
/// dispatches, plus `"refused": {"rows", "limit"}` when the gesture was refused whole ([`DagJournalRefusal`]).
pub fn dag_graph_edit_rows_json(edits: &[DagGraphEdit], refused: Option<DagJournalRefusal>) -> String {
    let mut rows = DagGraphEditJsonRows::default();
    if let Err(never) = write_dag_graph_edit_rows(edits, &mut rows) {
        match never {}
    }
    let refused = refused.map(|refusal| ("refused".to_string(), semio_framework_pack_json::object([("rows".to_string(), Value::from(refusal.rows)), ("limit".to_string(), Value::from(refusal.limit))])));
    semio_framework_pack_json::to_string(&semio_framework_pack_json::object(std::iter::once(("operations".to_string(), semio_framework_pack_json::array(rows.rows))).chain(refused)))
}

/// 🆔️ The gesture id of the drag a press began: `node-drag:<press>`, unique per host for the press's projection revision.
pub fn dag_drag_gesture_id(press: u64) -> String {
    format!("node-drag:{press}")
}

/// 📏️ How many rows one drain may carry: the rows one `nodeGraphEdit` dispatch admits (`NODE_GRAPH_EDIT_MAX_ROWS` of
/// `🛠️tool-machine`). A press journals at most one wire (and the edge it replaced), one slider value and one port; only the
/// moves of a release or an align grow with the graph (one row per distinct offset), so those are reserved whole or the
/// gesture is refused ([`DagJournalRefusal`]) — the journal never drops a row.
pub const DAG_GRAPH_EDIT_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagNodeEvalStatusKind {
    Ok,
    Queued,
    Computing,
    Error,
    Blocked,
}
