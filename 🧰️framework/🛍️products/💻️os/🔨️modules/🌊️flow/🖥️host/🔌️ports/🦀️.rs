//! 🔌️ Port value types: side, declared types, compatibility and synapse port resolution.

use super::*;

pub(super) /// 📥️ Load normalization of a freshly decoded document: the first widget of every identity stays and each later duplicate is retired cold.
fn deduplicated_loaded_widgets(mut loaded: FlowHostSnapshot) -> FlowHostSnapshot {
    let mut seen = BTreeSet::new();
    let (kept, duplicates): (Vec<_>, Vec<_>) = loaded.widgets.into_iter().partition(|widget| seen.insert(widget_id_for(widget).to_string()));
    loaded.widgets = kept;
    duplicates.into_iter().for_each(|widget| widget.retire_cold());
    loaded
}



/// 🔌️ Which side of a node a port sits on — the ONE thing that decides whether a port id is looked
/// up in a widget's inputs or its outputs, since `"{nodeId}@{portId}"` carries no direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSide {
    Input,
    Output,
}

/// 🔤️ The value schemas one endpoint declares, read straight off the port the operator catalogue
/// published — an untyped declared port answers with an empty list; connection admission separately checks exact endpoint identity.
pub fn widget_port_value_types(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Vec<String> {
    let Some(widget) = widgets.iter().find(|widget| widget_id_for(widget) == widget_id) else {
        return Vec::new();
    };
    let (inputs, outputs, _, _) = widget_io_ports(widget, synapses, kind_infos);
    let ports = if side == PortSide::Output { outputs } else { inputs };
    ports.iter().find(|port| port.id == port_id).and_then(|port| port.value_type.clone()).map(|declared| declared.split(',').filter(|entry| !entry.is_empty()).map(str::to_string).collect()).unwrap_or_default()
}

/// 🔌️ The ONE port-compatibility rule the flow graph enforces, over the value schemas both ends
/// declare: a pair is refused only when both sides declare and the sets are disjoint.
///
/// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the fixture that owns the pairs
/// @see `neural_engine::Registry::channel_compatible` — the same rule over two `ChannelSpec`s
pub fn port_value_types_compatible(source: &[String], target: &[String]) -> bool {
    if source.is_empty() || target.is_empty() {
        return true;
    }
    source.iter().any(|provided| target.iter().any(|accepted| accepted == provided))
}

pub(super) fn widget_has_declared_port(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> bool {
    widgets.iter().find(|widget| widget_id_for(widget) == widget_id).is_some_and(|widget| {
        let (inputs, outputs, _, _) = widget_io_ports(widget, synapses, kind_infos);
        (if side == PortSide::Output { outputs } else { inputs }).iter().any(|port| port.id == port_id)
    })
}

pub(super) fn widget_has_output(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| widget_id_for(w) == widget_id && !widget_io_ports(w, synapses, kind_infos).1.is_empty())
}

/// 🔌️ Maps a synapse endpoint onto the `IoPortSpec.id` the board keys its handles with.
/// An exact id is kept. Legacy `out` / `in` and an empty id select the first port on that side.
/// A missing widget or an unknown port stays unresolved so the canvas draws no wire.
pub(super) fn resolve_synapse_port(widget_id: &str, port_id: &str, side: PortSide, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> Option<String> {
    let widget = widgets.iter().find(|widget| widget_id_for(widget) == widget_id)?;
    let (inputs, outputs, _, _) = widget_io_ports(widget, synapses, kind_infos);
    let ports: Vec<String> = match side {
        PortSide::Output => outputs.iter().map(|port| port.id.clone()).collect(),
        PortSide::Input if inputs.is_empty() && matches!(widget, Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. }) => vec![String::new()],
        PortSide::Input => inputs.iter().map(|port| port.id.clone()).collect(),
    };
    if ports.iter().any(|id| id == port_id) {
        return Some(port_id.to_string());
    }
    let legacy = matches!((side, port_id), (PortSide::Output, "" | "out") | (PortSide::Input, "" | "in"));
    if legacy { ports.first().cloned() } else { None }
}

pub(super) fn first_output_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> String {
    widgets.iter().find(|w| widget_id_for(w) == widget_id).and_then(|w| widget_io_ports(w, synapses, kind_infos).1.first().map(|port| port.id.clone())).unwrap_or_default()
}

pub(super) fn first_input_port(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> String {
    widgets
        .iter()
        .find(|w| widget_id_for(w) == widget_id)
        .map(|w| match w {
            Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. } => String::new(),
            _ => widget_io_ports(w, synapses, kind_infos).0.first().map(|port| port.id.clone()).unwrap_or_default(),
        })
        .unwrap_or_default()
}

pub(super) fn widget_has_input(widget_id: &str, widgets: &[Widget], synapses: &[SynapseSpec], kind_infos: &HistoryFoldIndex<String, OperatorInfo>) -> bool {
    widgets.iter().any(|w| {
        if widget_id_for(w) != widget_id {
            return false;
        }
        matches!(w, Widget::OutputPreview { .. } | Widget::OutputAction { .. } | Widget::OutputExport { .. }) || !widget_io_ports(w, synapses, kind_infos).0.is_empty()
    })
}
