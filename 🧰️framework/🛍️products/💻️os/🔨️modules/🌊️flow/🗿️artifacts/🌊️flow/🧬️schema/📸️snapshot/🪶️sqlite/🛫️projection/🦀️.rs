//! 🌊️ Borrowed framework Flow projection preserving its actual persisted domain.
use crate::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use neural::{Atom, Dictionary, Tree, Value};
use store::sqlite_snapshot::{
    artifact::{insert_ieee754, Cell, FloatColumn, Projection},
    SqliteDatabase, SqliteSnapshotControl, SqliteSnapshotPhase,
};
const SQL: &str = include_str!("../🗄️.sql");
const CAMERA: &[FloatColumn] = &[FloatColumn::Binary64(2), FloatColumn::Binary64(3), FloatColumn::Binary64(4)];
const GUI_CAMERA: &[FloatColumn] = &[FloatColumn::Binary64(1), FloatColumn::Binary64(2), FloatColumn::Binary64(3)];
const SLIDER: &[FloatColumn] = &[FloatColumn::Binary64(3), FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6)];
const LAYOUT: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5)];
const PREVIEW_LAYOUT: &[FloatColumn] = &[FloatColumn::Binary64(2), FloatColumn::Binary64(3)];
const NUMBER: &[FloatColumn] = &[FloatColumn::Binary64(2)];
fn order(value: usize) -> Result<i64, ValueError> {
    i64::try_from(value).map_err(|_| ValueError::new(ValueRefusalKind::WorkLimit,"Flow ordinal exceeds signed64"))
}
fn boolean(value: bool) -> Cell<'static> {
    Cell::Integer(i64::from(value))
}

fn push_frontier<T>(frontier:&mut Vec<T>,value:T,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if frontier.len()==frontier.capacity(){let capacity=frontier.capacity().max(1).checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Flow borrowed frontier capacity overflow"))?;let mut next=store::sqlite_snapshot::transfer::reserve(capacity,control)?;let total=frontier.len();for(index,item)in frontier.drain(..).enumerate(){next.push(item);if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index+1,total)?;}}*frontier=next;}frontier.push(value);Ok(())
}
struct Forecast<'a, 'b> {
    control: &'a mut SqliteSnapshotControl<'b>,
    rows: usize,
    work: usize,
}
impl Forecast<'_, '_> {
    fn add(&mut self, n: usize) -> Result<(), ValueError> {
        self.rows = self.rows.checked_add(n).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow row overflow"))?;
        self.control.check_rows(self.rows)?;
        self.work = self.work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow work overflow"))?;
        if self.work % 256 == 0 {
            self.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, self.work, 0)?;
        }
        Ok(())
    }
    fn dictionary(&mut self, root: &Dictionary) -> Result<(), ValueError> {
        let mut stack = Vec::new(); push_frontier(&mut stack,root,self.control)?;
        while let Some(value) = stack.pop() {
            self.add(1)?;
            for (_, value) in value.iter() {
                self.add(2)?;
                match value {
                    Value::Atom(Atom::Null) => {}
                    Value::Atom(_) => self.add(1)?,
                    Value::Dictionary(value) => {
                        self.add(1)?;
                        push_frontier(&mut stack,value,self.control)?;
                    }
                }
            }
        }
        Ok(())
    }
    fn tree(&mut self, root: &Tree) -> Result<(), ValueError> {
        let mut stack = Vec::new(); push_frontier(&mut stack,root,self.control)?;
        while let Some(tree) = stack.pop() {
            self.add(1usize.checked_add(tree.synapses.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow tree row overflow"))?)?;
            for neuron in &tree.neurons {
                self.add(1)?;
                self.dictionary(&neuron.params)?;
                if let Some(tree) = &neuron.tree {
                    push_frontier(&mut stack,tree.as_ref(),self.control)?;
                }
            }
        }
        Ok(())
    }
    fn gui(&mut self, gui: &FlowUi) -> Result<(), ValueError> {
        self.add(1)?;
        for (_, _) in gui.nodes.iter() {
            self.add(2)?;
        }
        for preview in &gui.previews {
            self.add(1usize.checked_add(preview.expanded.len()).and_then(|n| n.checked_add(usize::from(preview.source.is_some()))).and_then(|n| n.checked_add(usize::from(preview.layout.is_some()))).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow preview row overflow"))?)?;
            self.dictionary(&preview.preview)?;
        }
        Ok(())
    }
}
pub(super) fn forecast(snapshot: &FlowHostSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<usize, ValueError> {
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0)?;
    if control.limits().max_tables < 37 || control.limits().max_columns < 15 { return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Flow declared schema work exceeds caller limit")); }
    if SQL.len() > control.limits().max_schema_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Flow authored schema exceeds caller byte limit")); }
    let mut count = Forecast { control, rows: 0, work: 0 };
    count.add(1usize.checked_add(snapshot.synapses.len()).and_then(|n| n.checked_add(snapshot.layout.len())).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow root row overflow"))?)?;
    for widget in &snapshot.widgets {
        count.add(2)?;
        match widget {
            Widget::Neuron { params, input_ports, output_ports, .. } => {
                count.add(input_ports.len().checked_add(output_ports.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Flow ports overflow"))?)?;
                count.dictionary(params)?;
            }
            Widget::OutputPreview { preview, expanded, .. } => {
                count.add(expanded.len())?;
                count.dictionary(preview)?;
            }
            Widget::Cluster { tree, flow, .. } => {
                count.tree(tree)?;
                count.gui(flow)?;
            }
            _ => {}
        }
    }
    count.control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, count.work, count.work)?;
    Ok(count.rows)
}
fn dictionary(root: &Dictionary, p: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    let id = p.insert("flow_dictionary", &[])?;
    let mut stack = p.allocate_frontier(1)?; stack.push((id,root));
    while let Some((dictionary, value)) = stack.pop() {
        for (ordinal, (key, value)) in value.iter().enumerate() {
            let variant = match value {
                Value::Atom(Atom::Null) => "null",
                Value::Atom(Atom::Boolean(_)) => "boolean",
                Value::Atom(Atom::Integer(_)) => "integer",
                Value::Atom(Atom::Decimal(_)) => "decimal",
                Value::Atom(Atom::String(_)) => "text",
                Value::Dictionary(_) => "dictionary",
            };
            let id = p.insert("flow_neural_value", &[Cell::Text(variant)])?;
            p.insert("flow_dictionary_entry", &[Cell::Integer(dictionary), Cell::Integer(order(ordinal)?), Cell::Text(key), Cell::Integer(id)])?;
            match value {
                Value::Atom(Atom::Null) => {}
                Value::Atom(Atom::Boolean(value)) => {
                    p.insert("flow_neural_boolean", &[Cell::Integer(id), boolean(*value)])?;
                }
                Value::Atom(Atom::Integer(value)) => {
                    p.insert("flow_neural_integer", &[Cell::Integer(id), Cell::Integer(*value)])?;
                }
                Value::Atom(Atom::Decimal(value)) => {
                    insert_ieee754(p, "flow_neural_decimal", &[Cell::Integer(id), Cell::Real(*value)], NUMBER)?;
                }
                Value::Atom(Atom::String(value)) => {
                    p.insert("flow_neural_text", &[Cell::Integer(id), Cell::Text(value)])?;
                }
                Value::Dictionary(value) => {
                    let child = p.insert("flow_dictionary", &[])?;
                    p.insert("flow_neural_dictionary", &[Cell::Integer(id), Cell::Integer(child)])?;
                    p.push_frontier(&mut stack,(child,value))?;
                }
            }
            p.checkpoint()?;
        }
    }
    Ok(id)
}
fn tree(root: &Tree, p: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    let id = p.insert("flow_tree", &[])?;
    let mut stack = p.allocate_frontier(1)?; stack.push((id,root));
    while let Some((tree, value)) = stack.pop() {
        for (ordinal, neuron) in value.neurons.iter().enumerate() {
            let params = dictionary(&neuron.params, p)?;
            let child = if let Some(value) = &neuron.tree {
                let child = p.insert("flow_tree", &[])?;
                p.push_frontier(&mut stack,(child,value.as_ref()))?;
                Cell::Integer(child)
            } else {
                Cell::Null
            };
            p.insert("flow_tree_neuron", &[Cell::Integer(tree), Cell::Integer(order(ordinal)?), Cell::Text(&neuron.id), Cell::Text(&neuron.kind), Cell::Integer(params), child])?;
            p.checkpoint()?;
        }
        for (ordinal, synapse) in value.synapses.iter().enumerate() {
            p.insert("flow_tree_synapse", &[Cell::Integer(tree), Cell::Integer(order(ordinal)?), Cell::Text(&synapse.id), Cell::Text(&synapse.from), Cell::Text(&synapse.to), Cell::Text(&synapse.from_port), Cell::Text(&synapse.to_port)])?;
            p.checkpoint()?;
        }
    }
    Ok(id)
}
fn gui(value: &FlowUi, p: &mut Projection<'_, '_>) -> Result<i64, ValueError> {
    let gui = insert_ieee754(p, "flow_gui", &[Cell::Real(value.camera.x), Cell::Real(value.camera.y), Cell::Real(value.camera.zoom)], GUI_CAMERA)?;
    for (ordinal, (key, node)) in value.nodes.iter().enumerate() {
        let kind = match &node.chrome {
            NodeChrome::Plain { .. } => "plain",
            NodeChrome::Slider { .. } => "slider",
            NodeChrome::Note { .. } => "note",
            NodeChrome::Image { .. } => "image",
            NodeChrome::Variable { .. } => "variable",
        };
        let id = insert_ieee754(p, "flow_gui_node", &[Cell::Integer(gui), Cell::Integer(order(ordinal)?), Cell::Text(key), Cell::Real(node.layout.x), Cell::Real(node.layout.y), Cell::Text(kind)], LAYOUT)?;
        match &node.chrome {
            NodeChrome::Plain { preview } => {
                p.insert("flow_chrome_plain", &[Cell::Integer(id), boolean(*preview)])?;
            }
            NodeChrome::Slider { label, min, max, step, value } => {
                insert_ieee754(p, "flow_chrome_slider", &[Cell::Integer(id), Cell::Text(label), Cell::Real(*min), Cell::Real(*max), Cell::Real(*step), Cell::Real(*value)], SLIDER)?;
            }
            NodeChrome::Note { text } => {
                p.insert("flow_chrome_note", &[Cell::Integer(id), Cell::Text(text)])?;
            }
            NodeChrome::Image { src } => {
                p.insert("flow_chrome_image", &[Cell::Integer(id), Cell::Text(src)])?;
            }
            NodeChrome::Variable { name, schema } => {
                p.insert("flow_chrome_variable", &[Cell::Integer(id), Cell::Text(name), Cell::Text(schema)])?;
            }
        }
        p.checkpoint()?;
    }
    for (ordinal, preview) in value.previews.iter().enumerate() {
        let value = dictionary(&preview.preview, p)?;
        let id = p.insert("flow_gui_preview", &[Cell::Integer(gui), Cell::Integer(order(ordinal)?), Cell::Text(&preview.id), Cell::Text(&preview.mode), Cell::Integer(value)])?;
        if let Some(source) = &preview.source {
            p.insert("flow_preview_source", &[Cell::Integer(id), Cell::Text(&source.neuron), Cell::Text(&source.channel)])?;
        }
        if let Some(layout) = &preview.layout {
            insert_ieee754(p, "flow_preview_layout", &[Cell::Integer(id), Cell::Real(layout.x), Cell::Real(layout.y)], PREVIEW_LAYOUT)?;
        }
        for (ordinal, path) in preview.expanded.iter().enumerate() {
            p.insert("flow_gui_expanded", &[Cell::Integer(id), Cell::Integer(order(ordinal)?), Cell::Text(path)])?;
        }
        p.checkpoint()?;
    }
    Ok(gui)
}
pub(super) fn project(snapshot: &FlowHostSnapshot, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> {
    let total = forecast(snapshot, control)?;
    let mut p = Projection::new(SQL, control)?;
    let document = insert_ieee754(&mut p, "flow_document", &[Cell::Text(&snapshot.schema), Cell::Real(snapshot.camera.x), Cell::Real(snapshot.camera.y), Cell::Real(snapshot.camera.zoom)], CAMERA)?;
    for (ordinal, widget) in snapshot.widgets.iter().enumerate() {
        let kind = match widget {
            Widget::Neuron { .. } => "neuron",
            Widget::InputSlider { .. } => "inputSlider",
            Widget::InputNote { .. } => "inputNote",
            Widget::InputImage { .. } => "inputImage",
            Widget::Variable { .. } => "variable",
            Widget::OutputPreview { .. } => "outputPreview",
            Widget::OutputAction { .. } => "outputAction",
            Widget::OutputExport { .. } => "outputExport",
            Widget::Cluster { .. } => "cluster",
        };
        let id = p.insert("flow_widget", &[Cell::Integer(document), Cell::Integer(order(ordinal)?), Cell::Text(widget_id_for(widget)), Cell::Text(kind)])?;
        match widget {
            Widget::Neuron { neuron_kind, params, input_ports, output_ports, preview, .. } => {
                let params = dictionary(params, &mut p)?;
                let neuron = p.insert("flow_neuron_widget", &[Cell::Integer(id), Cell::Text(neuron_kind), Cell::Integer(params), boolean(*preview)])?;
                for (side, ports) in [("input", input_ports), ("output", output_ports)] {
                    for (ordinal, name) in ports.iter().enumerate() {
                        p.insert("flow_widget_port", &[Cell::Integer(neuron), Cell::Text(side), Cell::Integer(order(ordinal)?), Cell::Text(name)])?;
                    }
                }
            }
            Widget::InputSlider { label, value, min, max, step, .. } => {
                insert_ieee754(&mut p, "flow_slider_widget", &[Cell::Integer(id), Cell::Text(label), Cell::Real(*value), Cell::Real(*min), Cell::Real(*max), Cell::Real(*step)], SLIDER)?;
            }
            Widget::InputNote { text, .. } => {
                p.insert("flow_note_widget", &[Cell::Integer(id), Cell::Text(text)])?;
            }
            Widget::InputImage { src, .. } => {
                p.insert("flow_image_widget", &[Cell::Integer(id), Cell::Text(src)])?;
            }
            Widget::Variable { name, schema, .. } => {
                p.insert("flow_variable_widget", &[Cell::Integer(id), Cell::Text(name), Cell::Text(schema)])?;
            }
            Widget::OutputPreview { preview, expanded, .. } => {
                let preview = dictionary(preview, &mut p)?;
                let preview = p.insert("flow_preview_widget", &[Cell::Integer(id), Cell::Integer(preview)])?;
                for (ordinal, path) in expanded.iter().enumerate() {
                    p.insert("flow_widget_expanded", &[Cell::Integer(preview), Cell::Integer(order(ordinal)?), Cell::Text(path)])?;
                }
            }
            Widget::OutputAction { action, .. } => {
                p.insert("flow_action_widget", &[Cell::Integer(id), Cell::Text(action)])?;
            }
            Widget::OutputExport { format, .. } => {
                p.insert("flow_export_widget", &[Cell::Integer(id), Cell::Text(format)])?;
            }
            Widget::Cluster { name, tree: cluster, flow, .. } => {
                let tree = tree(cluster, &mut p)?;
                let gui = gui(flow, &mut p)?;
                p.insert("flow_cluster_widget", &[Cell::Integer(id), Cell::Text(name), Cell::Integer(tree), Cell::Integer(gui)])?;
            }
        }
        p.checkpoint_total(total)?;
    }
    for (ordinal, synapse) in snapshot.synapses.iter().enumerate() {
        p.insert("flow_synapse", &[Cell::Integer(document), Cell::Integer(order(ordinal)?), Cell::Text(&synapse.id), Cell::Text(&synapse.from), Cell::Text(&synapse.to), Cell::Text(&synapse.from_port), Cell::Text(&synapse.to_port)])?;
        p.checkpoint_total(total)?;
    }
    for (ordinal, (key, layout)) in snapshot.layout.iter().enumerate() {
        insert_ieee754(&mut p, "flow_layout", &[Cell::Integer(document), Cell::Integer(order(ordinal)?), Cell::Text(key), Cell::Real(layout.x), Cell::Real(layout.y)], LAYOUT)?;
        p.checkpoint_total(total)?;
    }
    p.checkpoint_total(total)?;
    p.finish()
}
