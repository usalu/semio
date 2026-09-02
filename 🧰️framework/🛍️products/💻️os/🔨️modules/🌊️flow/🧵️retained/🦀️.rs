//! 🧵️ Shared typed Flow ownership frontiers for resumable copying and retirement.

use crate::{neural, FlowFixture, FlowGui, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, NodeChrome, SynapseSpec, Widget, WidgetLayout};
use crate::os_store::{ErasedSnapshotRetirement, SnapshotRetirementStep};
use std::collections::LinkedList;
use crate::{OrderedMap, OrderedSet};
use protocol::value::ordered::{Grant, Retirement, RetirementStep};
use std::mem::ManuallyDrop;

//#region 📑️SelectedCopy
#[path = "📑️copy/🦀️.rs"]
pub mod copy;
pub use copy::{FlowCopyAllocationBudget, FlowFixtureCopy, FlowSynapseCopy, FlowWidgetCopy};
//#endregion 📑️SelectedCopy

//#region 🧹️TypedRetirement
pub enum FlowOwner {
    Bytes(Vec<u8>),
    Strings(Vec<String>),
    Set(OrderedSet),
    SetCursor(Retirement<()>),
    Dictionary(neural::Dictionary),
    Value(neural::Value),
    Neural(neural::ValueRetirement),
    Fixture(FlowFixture),
    Widget(Widget),
    Widgets(Vec<Widget>),
    Specs(Vec<SynapseSpec>),
    Layouts(OrderedMap<WidgetLayout>),
    LayoutCursor(Retirement<WidgetLayout>),
    Tree(neural::Tree),
    Neurons(Vec<neural::Neuron>),
    Synapses(Vec<neural::Synapse>),
    Gui(FlowGui),
    Nodes(OrderedMap<FlowNodeGui>),
    NodeCursor(Retirement<FlowNodeGui>),
    Previews(Vec<FlowPreviewGui>),
    Layout(Vec<FlowLayoutEntry>),
    Chrome(NodeChrome),
}

#[must_use = "Flow ownership must be transferred or retired to an empty frontier"]
pub struct FlowRetirement { owners: ManuallyDrop<LinkedList<FlowOwner>> }

impl Default for FlowRetirement {
    fn default() -> Self { Self { owners: ManuallyDrop::new(LinkedList::new()) } }
}

impl FlowRetirement {
    pub fn push(&mut self, owner: FlowOwner) { self.owners.push_front(owner); }
    pub fn text(&mut self, value: String) { self.push(FlowOwner::Bytes(value.into_bytes())); }
    pub fn is_empty(&self) -> bool { self.owners.is_empty() }
    /// 🧊️ Explicit cold-only teardown; retained callers use close_step.
    pub fn retire_cold(mut self) {
        while !matches!(self.close_step(1, 4096).expect("cold Flow retirement"), SnapshotRetirementStep::Complete) {}
    }

    fn widget(&mut self, widget: Widget) {
        match widget {
            Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => {
                self.text(id); self.text(neuron_kind); self.push(FlowOwner::Dictionary(params));
                self.push(FlowOwner::Strings(input_ports)); self.push(FlowOwner::Strings(output_ports));
            }
            Widget::InputSlider { id, label, .. } => { self.text(id); self.text(label); }
            Widget::InputNote { id, text } => { self.text(id); self.text(text); }
            Widget::InputImage { id, src } => { self.text(id); self.text(src); }
            Widget::Variable { id, name, schema } => { self.text(id); self.text(name); self.text(schema); }
            Widget::OutputPreview { id, preview, expanded } => {
                self.text(id); self.push(FlowOwner::Dictionary(preview)); self.push(FlowOwner::Set(expanded));
            }
            Widget::OutputAction { id, action } => { self.text(id); self.text(action); }
            Widget::OutputExport { id, format } => { self.text(id); self.text(format); }
            Widget::Cluster { id, name, tree, flow } => {
                self.text(id); self.text(name); self.push(FlowOwner::Tree(tree)); self.push(FlowOwner::Gui(flow));
            }
        }
    }

    fn chrome(&mut self, chrome: NodeChrome) {
        match chrome {
            NodeChrome::Plain { .. } => {}
            NodeChrome::Slider { label, .. } => self.text(label),
            NodeChrome::Note { text } => self.text(text),
            NodeChrome::Image { src } => self.text(src),
            NodeChrome::Variable { name, schema } => { self.text(name); self.text(schema); }
        }
    }
}

impl FlowFixture {
    /// 🧊️ Explicit cold-only disposal of a detached fixture.
    pub fn retire_cold(self) { let mut retirement = FlowRetirement::default(); retirement.push(FlowOwner::Fixture(self)); retirement.retire_cold(); }
}

impl Widget {
    /// 🧊️ Explicit cold-only disposal of a detached widget.
    pub fn retire_cold(self) { let mut retirement = FlowRetirement::default(); retirement.push(FlowOwner::Widget(self)); retirement.retire_cold(); }
}

impl ErasedSnapshotRetirement for FlowRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        use SnapshotRetirementStep as Step;
        if self.is_empty() { return Ok(Step::Complete); }
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        let mut released_bytes = 0;
        match self.owners.pop_front().expect("nonempty Flow retirement") {
            FlowOwner::Bytes(mut bytes) => {
                released_bytes = maximum_bytes.min(bytes.len());
                bytes.truncate(bytes.len() - released_bytes);
                if !bytes.is_empty() { self.push(FlowOwner::Bytes(bytes)); }
            }
            FlowOwner::Strings(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(FlowOwner::Strings(values)); }
                if let Some(value) = next { self.text(value); }
            }
            FlowOwner::Set(values) => self.push(FlowOwner::SetCursor(values.retire())),
            FlowOwner::SetCursor(mut values) => {
                match values.advance(Grant { maximum_items: 1, maximum_bytes }) {
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::OwnedValue(()) | RetirementStep::Complete | RetirementStep::Blocked => {}
                }
                if !values.is_empty() { self.push(FlowOwner::SetCursor(values)); }
            }
            FlowOwner::Dictionary(value) => self.push(FlowOwner::Neural(neural::ValueRetirement::from_dictionary(value))),
            FlowOwner::Value(value) => self.push(FlowOwner::Neural(neural::ValueRetirement::from_value(value))),
            FlowOwner::Neural(mut value) => {
                match value.close_step(1, maximum_bytes) {
                    neural::ValueRetirementStep::Pending { released_bytes: bytes, .. } => released_bytes = bytes,
                    neural::ValueRetirementStep::Blocked | neural::ValueRetirementStep::Complete => {}
                }
                if !value.terminal_is_empty() { self.push(FlowOwner::Neural(value)); }
            }
            FlowOwner::Fixture(value) => {
                self.text(value.schema); self.push(FlowOwner::Widgets(value.widgets));
                self.push(FlowOwner::Specs(value.synapses)); self.push(FlowOwner::Layouts(value.layout));
            }
            FlowOwner::Widget(value) => self.widget(value),
            FlowOwner::Widgets(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(FlowOwner::Widgets(values)); }
                if let Some(value) = next { self.push(FlowOwner::Widget(value)); }
            }
            FlowOwner::Specs(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(FlowOwner::Specs(values)); }
                if let Some(value) = next { self.text(value.id); self.text(value.from); self.text(value.to); self.text(value.from_port); self.text(value.to_port); }
            }
            FlowOwner::Layouts(values) => self.push(FlowOwner::LayoutCursor(values.retire())),
            FlowOwner::LayoutCursor(mut values) => {
                match values.advance(Grant { maximum_items: 1, maximum_bytes }) {
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::OwnedValue(_) | RetirementStep::Complete | RetirementStep::Blocked => {}
                }
                if !values.is_empty() { self.push(FlowOwner::LayoutCursor(values)); }
            }
            FlowOwner::Tree(value) => { self.push(FlowOwner::Neurons(value.neurons)); self.push(FlowOwner::Synapses(value.synapses)); }
            FlowOwner::Neurons(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(FlowOwner::Neurons(values)); }
                if let Some(value) = next {
                    self.text(value.id); self.text(value.kind); self.push(FlowOwner::Dictionary(value.params));
                    if let Some(tree) = value.tree { self.push(FlowOwner::Tree(*tree)); }
                }
            }
            FlowOwner::Synapses(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(FlowOwner::Synapses(values)); }
                if let Some(value) = next { self.text(value.id); self.text(value.from); self.text(value.to); self.text(value.from_port); self.text(value.to_port); }
            }
            FlowOwner::Gui(value) => { self.push(FlowOwner::Nodes(value.nodes)); self.push(FlowOwner::Previews(value.previews)); }
            FlowOwner::Nodes(values) => self.push(FlowOwner::NodeCursor(values.retire())),
            FlowOwner::NodeCursor(mut values) => {
                match values.advance(Grant { maximum_items: 1, maximum_bytes }) {
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::OwnedValue(value) => self.push(FlowOwner::Chrome(value.chrome)),
                    RetirementStep::Complete | RetirementStep::Blocked => {}
                }
                if !values.is_empty() { self.push(FlowOwner::NodeCursor(values)); }
            }
            FlowOwner::Previews(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(FlowOwner::Previews(values)); }
                if let Some(value) = next {
                    self.text(value.id); self.text(value.mode); self.push(FlowOwner::Dictionary(value.preview)); self.push(FlowOwner::Set(value.expanded));
                    if let Some(source) = value.source { self.text(source.neuron); self.text(source.channel); }
                }
            }
            FlowOwner::Layout(mut values) => {
                let next = values.pop();
                if !values.is_empty() { self.push(FlowOwner::Layout(values)); }
                if let Some(value) = next { self.text(value.id); }
            }
            FlowOwner::Chrome(value) => self.chrome(value),
        }
        Ok(Step::Pending { released_items: 1, released_bytes })
    }
    fn terminal_is_empty(&self) -> bool { self.is_empty() }
}

impl Drop for FlowRetirement {
    fn drop(&mut self) {
        if !self.is_empty() {
            if !std::thread::panicking() { panic!("Flow retirement dropped with live owned payloads"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owners); }
    }
}
//#endregion 🧹️TypedRetirement

//#region 🧪️RetirementLaws
#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;
//#endregion 🧪️RetirementLaws
