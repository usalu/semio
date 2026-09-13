//! 🧵️ Shared typed Flow ownership frontiers for resumable copying and retirement.

use crate::os_store::{ErasedSnapshotRetirement, SnapshotRetirementStep};
use crate::{neural, FlowFixture, FlowGui, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, NodeChrome, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout};
use protocol::value::list::{PagedList, PagedListAllocationError, PagedListProgress};
use protocol::value::ordered::{Grant, Retirement, RetirementStep};
use std::mem::{size_of, ManuallyDrop};

//#region 📑️SelectedCopy
#[path = "📑️copy/🦀️.rs"]
pub mod copy;
pub use copy::{FlowCopyAllocationBudget, FlowFixtureCopy, FlowSynapseCopy, FlowWidgetCopy};
//#endregion 📑️SelectedCopy

//#region 🧹️TypedRetirement
const FLOW_RETIREMENT_FRONTIER_OWNERS: usize = usize::MAX;

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

fn backing_bytes<T>(values: &Vec<T>) -> Result<usize, &'static str> {
    values.capacity().checked_mul(size_of::<T>()).ok_or("Flow backing capacity byte count overflow")
}

fn release_backing<T>(values: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<usize>, &'static str> {
    let bytes = backing_bytes(values)?;
    if bytes > maximum_bytes {
        return Ok(None);
    }
    drop(std::mem::take(values));
    Ok(Some(bytes))
}

fn owner_backing_bytes(owner: &FlowOwner) -> Result<usize, &'static str> {
    match owner {
        FlowOwner::Bytes(values) => backing_bytes(values),
        FlowOwner::Strings(values) => backing_bytes(values),
        FlowOwner::Widgets(values) => backing_bytes(values),
        FlowOwner::Specs(values) => backing_bytes(values),
        FlowOwner::Neurons(values) => backing_bytes(values),
        FlowOwner::Synapses(values) => backing_bytes(values),
        FlowOwner::Previews(values) => backing_bytes(values),
        FlowOwner::Layout(values) => backing_bytes(values),
        FlowOwner::SetCursor(values) => Ok(values.allocated_bytes()),
        FlowOwner::LayoutCursor(values) => Ok(values.allocated_bytes()),
        FlowOwner::NodeCursor(values) => Ok(values.allocated_bytes()),
        FlowOwner::Neural(values) => Ok(values.allocated_bytes()),
        _ => Ok(0),
    }
}

fn owner_release_demand(owner: &FlowOwner) -> Result<usize, &'static str> {
    match owner {
        FlowOwner::Bytes(values) => backing_bytes(values),
        FlowOwner::Strings(values) if values.is_empty() => backing_bytes(values),
        FlowOwner::Widgets(values) if values.is_empty() => backing_bytes(values),
        FlowOwner::Specs(values) if values.is_empty() => backing_bytes(values),
        FlowOwner::Neurons(values) if values.is_empty() => backing_bytes(values),
        FlowOwner::Synapses(values) if values.is_empty() => backing_bytes(values),
        FlowOwner::Previews(values) if values.is_empty() => backing_bytes(values),
        FlowOwner::Layout(values) if values.is_empty() => backing_bytes(values),
        FlowOwner::SetCursor(values) => values.next_close_byte_demand(),
        FlowOwner::LayoutCursor(values) => values.next_close_byte_demand(),
        FlowOwner::NodeCursor(values) => values.next_close_byte_demand(),
        FlowOwner::Neural(values) => values.next_close_byte_demand(),
        _ => Ok(1),
    }
    .map(|bytes| bytes.max(1))
}

fn owner_waits_for_backing_release(owner: &FlowOwner) -> bool {
    match owner {
        FlowOwner::Bytes(_) => true,
        FlowOwner::Strings(values) => values.is_empty(),
        FlowOwner::Widgets(values) => values.is_empty(),
        FlowOwner::Specs(values) => values.is_empty(),
        FlowOwner::Neurons(values) => values.is_empty(),
        FlowOwner::Synapses(values) => values.is_empty(),
        FlowOwner::Previews(values) => values.is_empty(),
        FlowOwner::Layout(values) => values.is_empty(),
        _ => false,
    }
}

fn owner_continuation_slots(owner: &FlowOwner) -> usize {
    match owner {
        FlowOwner::Strings(values) if !values.is_empty() => 1,
        FlowOwner::Widgets(values) if !values.is_empty() => 1,
        FlowOwner::Specs(values) if !values.is_empty() => 5,
        FlowOwner::Synapses(values) if !values.is_empty() => 5,
        FlowOwner::Neurons(values) if !values.is_empty() => 4,
        FlowOwner::Previews(values) if !values.is_empty() => 7,
        FlowOwner::Layout(values) if !values.is_empty() => 1,
        FlowOwner::Fixture(_) => 3,
        FlowOwner::Widget(Widget::Neuron { .. }) => 4,
        FlowOwner::Widget(Widget::InputSlider { .. })
        | FlowOwner::Widget(Widget::InputNote { .. })
        | FlowOwner::Widget(Widget::InputImage { .. })
        | FlowOwner::Widget(Widget::OutputAction { .. })
        | FlowOwner::Widget(Widget::OutputExport { .. }) => 1,
        FlowOwner::Widget(Widget::Variable { .. }) => 2,
        FlowOwner::Widget(Widget::OutputPreview { .. }) => 2,
        FlowOwner::Widget(Widget::Cluster { .. }) => 3,
        FlowOwner::Tree(_) | FlowOwner::Gui(_) => 1,
        FlowOwner::NodeCursor(_) => 1,
        FlowOwner::Chrome(NodeChrome::Variable { .. }) => 1,
        _ => 0,
    }
}

#[must_use = "Flow ownership must be transferred or retired to an empty frontier"]
pub struct FlowRetirement {
    root: ManuallyDrop<Option<FlowOwner>>,
    frontier: ManuallyDrop<PagedList<FlowOwner, FLOW_RETIREMENT_FRONTIER_OWNERS>>,
    fault: Option<&'static str>,
}

impl Default for FlowRetirement {
    fn default() -> Self {
        Self { root: ManuallyDrop::new(None), frontier: ManuallyDrop::new(PagedList::empty()), fault: None }
    }
}

impl FlowRetirement {
    pub fn from_owner(owner: FlowOwner) -> Self {
        Self { root: ManuallyDrop::new(Some(owner)), frontier: ManuallyDrop::new(PagedList::empty()), fault: None }
    }

    pub fn push(&mut self, owner: FlowOwner) {
        if self.root.is_none() {
            *self.root = Some(owner);
            return;
        }
        while !self.frontier.has_reserved_slot() {
            let demand = self.frontier.next_allocation_bytes().expect("unbounded Flow frontier has a finite next page");
            if let Err(error) = self.frontier.reserve_one(demand) {
                self.fault.get_or_insert(error.reason);
                std::mem::forget(owner);
                return;
            }
        }
        let previous = self.root.replace(owner).expect("checked Flow root");
        if let Err(previous) = self.frontier.push_reserved(previous) {
            std::mem::forget(previous);
            panic!("reserved Flow frontier rejected an owner");
        }
    }

    pub fn text(&mut self, value: String) {
        self.push(FlowOwner::Bytes(value.into_bytes()));
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none() && self.frontier.terminal_is_empty()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.is_empty() && self.allocated_bytes() == 0
    }

    pub fn fault(&self) -> Option<&'static str> {
        self.fault
    }

    pub fn allocated_bytes(&self) -> usize {
        self.root.iter().chain(self.frontier.iter()).fold(self.frontier.allocated_bytes(), |total, owner| {
            total.saturating_add(owner_backing_bytes(owner).unwrap_or(usize::MAX))
        })
    }

    fn target_frontier_capacity(&self) -> Result<usize, &'static str> {
        self.frontier.len().checked_add(self.root.as_ref().map_or(0, owner_continuation_slots)).ok_or("Flow frontier capacity overflow")
    }

    pub fn next_allocation_bytes(&self) -> Result<Option<usize>, &'static str> {
        self.frontier.next_capacity_allocation_bytes(self.target_frontier_capacity()?)
    }

    pub fn reserve_allocation(&mut self, maximum_bytes: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        let target = self.target_frontier_capacity().map_err(|reason| PagedListAllocationError { allocated_bytes: 0, reason })?;
        match self.frontier.reserve_capacity_one(target, maximum_bytes) {
            Ok(step) => Ok(step),
            Err(error) => {
                self.fault.get_or_insert(error.reason);
                Err(error)
            }
        }
    }

    pub fn next_close_byte_demand(&self) -> Result<usize, &'static str> {
        if let Some(owner) = self.root.as_ref() {
            return owner_release_demand(owner);
        }
        if !self.frontier.is_empty() {
            return Ok(1);
        }
        if !self.frontier.terminal_is_empty() {
            return self.frontier.next_release_allocation_bytes().map(|bytes| bytes.max(1));
        }
        Ok(0)
    }

    /// 🧊️ Explicit cold-only teardown; retained callers use demand, reserve, and close_step.
    pub fn retire_cold(mut self) {
        loop {
            while let Some(bytes) = self.next_allocation_bytes().expect("finite cold Flow allocation demand") {
                self.reserve_allocation(bytes).expect("cold Flow frontier allocation");
            }
            let bytes = self.next_close_byte_demand().expect("finite cold Flow release demand");
            if matches!(self.close_step(1, bytes.max(1)).expect("cold Flow retirement"), SnapshotRetirementStep::Complete) {
                break;
            }
        }
    }

    fn install<const N: usize>(&mut self, owners: [Option<FlowOwner>; N]) {
        for owner in owners.into_iter().flatten() {
            if let Some(previous) = self.root.replace(owner) {
                if let Err(previous) = self.frontier.push_reserved(previous) {
                    std::mem::forget(previous);
                    panic!("preflighted Flow continuation slot disappeared");
                }
            }
        }
    }

    fn widget(&mut self, widget: Widget) {
        match widget {
            Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, .. } => self.install([
                Some(FlowOwner::Bytes(id.into_bytes())),
                Some(FlowOwner::Bytes(neuron_kind.into_bytes())),
                Some(FlowOwner::Dictionary(params)),
                Some(FlowOwner::Strings(input_ports)),
                Some(FlowOwner::Strings(output_ports)),
            ]),
            Widget::InputSlider { id, label, .. } => self.install([Some(FlowOwner::Bytes(id.into_bytes())), Some(FlowOwner::Bytes(label.into_bytes()))]),
            Widget::InputNote { id, text } => self.install([Some(FlowOwner::Bytes(id.into_bytes())), Some(FlowOwner::Bytes(text.into_bytes()))]),
            Widget::InputImage { id, src } => self.install([Some(FlowOwner::Bytes(id.into_bytes())), Some(FlowOwner::Bytes(src.into_bytes()))]),
            Widget::Variable { id, name, schema } => self.install([Some(FlowOwner::Bytes(id.into_bytes())), Some(FlowOwner::Bytes(name.into_bytes())), Some(FlowOwner::Bytes(schema.into_bytes()))]),
            Widget::OutputPreview { id, preview, expanded } => self.install([Some(FlowOwner::Bytes(id.into_bytes())), Some(FlowOwner::Dictionary(preview)), Some(FlowOwner::Set(expanded))]),
            Widget::OutputAction { id, action } => self.install([Some(FlowOwner::Bytes(id.into_bytes())), Some(FlowOwner::Bytes(action.into_bytes()))]),
            Widget::OutputExport { id, format } => self.install([Some(FlowOwner::Bytes(id.into_bytes())), Some(FlowOwner::Bytes(format.into_bytes()))]),
            Widget::Cluster { id, name, tree, flow } => self.install([
                Some(FlowOwner::Bytes(id.into_bytes())),
                Some(FlowOwner::Bytes(name.into_bytes())),
                Some(FlowOwner::Tree(tree)),
                Some(FlowOwner::Gui(flow)),
            ]),
        }
    }

    fn chrome(&mut self, chrome: NodeChrome) {
        match chrome {
            NodeChrome::Plain { .. } => {}
            NodeChrome::Slider { label, .. } => self.install([Some(FlowOwner::Bytes(label.into_bytes()))]),
            NodeChrome::Note { text } => self.install([Some(FlowOwner::Bytes(text.into_bytes()))]),
            NodeChrome::Image { src } => self.install([Some(FlowOwner::Bytes(src.into_bytes()))]),
            NodeChrome::Variable { name, schema } => self.install([Some(FlowOwner::Bytes(name.into_bytes())), Some(FlowOwner::Bytes(schema.into_bytes()))]),
        }
    }

    fn release_root_backing(&mut self, maximum_bytes: usize) -> Result<Option<usize>, &'static str> {
        let Some(owner) = self.root.as_mut() else {
            return Ok(None);
        };
        match owner {
            FlowOwner::Bytes(values) => release_backing(values, maximum_bytes),
            FlowOwner::Strings(values) if values.is_empty() => release_backing(values, maximum_bytes),
            FlowOwner::Widgets(values) if values.is_empty() => release_backing(values, maximum_bytes),
            FlowOwner::Specs(values) if values.is_empty() => release_backing(values, maximum_bytes),
            FlowOwner::Neurons(values) if values.is_empty() => release_backing(values, maximum_bytes),
            FlowOwner::Synapses(values) if values.is_empty() => release_backing(values, maximum_bytes),
            FlowOwner::Previews(values) if values.is_empty() => release_backing(values, maximum_bytes),
            FlowOwner::Layout(values) if values.is_empty() => release_backing(values, maximum_bytes),
            _ => Ok(None),
        }
    }
}

impl FlowFixture {
    /// 🧊️ Explicit cold-only disposal of a detached fixture.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Fixture(self)).retire_cold();
    }
}

impl Widget {
    /// 🧊️ Explicit cold-only disposal of a detached widget.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Widget(self)).retire_cold();
    }
}

impl ErasedSnapshotRetirement for FlowRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        use SnapshotRetirementStep as Step;
        if self.terminal_is_empty() {
            return Ok(Step::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(Step::Blocked);
        }
        if self.next_allocation_bytes().map_err(str::to_owned)?.is_some() {
            return Ok(Step::Blocked);
        }
        if let Some(released_bytes) = self.release_root_backing(maximum_bytes).map_err(str::to_owned)? {
            *self.root = None;
            return Ok(Step::Pending { released_items: 1, released_bytes });
        }
        if self.root.as_ref().is_some_and(owner_waits_for_backing_release) {
            return Ok(Step::Blocked);
        }
        if self.root.is_none() {
            if let Some(owner) = self.frontier.pop() {
                *self.root = Some(owner);
                return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
            }
            let release = self.frontier.release_empty_page(maximum_bytes).map_err(str::to_owned)?;
            if !release.progressed {
                return Ok(Step::Blocked);
            }
            return Ok(Step::Pending { released_items: 1, released_bytes: release.released_allocation_bytes });
        }
        let owner = self.root.take().expect("nonempty Flow retirement");
        let mut released_bytes = 0;
        match owner {
            FlowOwner::Bytes(_) => unreachable!("byte backing is released before logical dispatch"),
            FlowOwner::Strings(mut values) => {
                let value = values.pop().expect("nonempty string owner");
                self.install([Some(FlowOwner::Strings(values)), Some(FlowOwner::Bytes(value.into_bytes()))]);
            }
            FlowOwner::Set(values) => self.install([Some(FlowOwner::SetCursor(values.retire()))]),
            FlowOwner::SetCursor(mut values) => {
                let step = values.advance(Grant { maximum_items: 1, maximum_bytes });
                let pending = !values.is_empty();
                let blocked = matches!(step, RetirementStep::Blocked);
                match step {
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::OwnedValue(()) | RetirementStep::Complete | RetirementStep::Blocked => {}
                }
                if pending {
                    self.install([Some(FlowOwner::SetCursor(values))]);
                }
                if blocked { return Ok(Step::Blocked); }
            }
            FlowOwner::Dictionary(value) => self.install([Some(FlowOwner::Neural(neural::ValueRetirement::from_dictionary(value)))]),
            FlowOwner::Value(value) => self.install([Some(FlowOwner::Neural(neural::ValueRetirement::from_value(value)))]),
            FlowOwner::Neural(mut value) => {
                let step = value.close_step(1, maximum_bytes);
                let blocked = matches!(step, neural::ValueRetirementStep::Blocked);
                match step {
                    neural::ValueRetirementStep::Pending { released_bytes: bytes, .. } => released_bytes = bytes,
                    neural::ValueRetirementStep::Blocked | neural::ValueRetirementStep::Complete => {}
                }
                if !value.terminal_is_empty() {
                    self.install([Some(FlowOwner::Neural(value))]);
                }
                if blocked { return Ok(Step::Blocked); }
            }
            FlowOwner::Fixture(value) => self.install([
                Some(FlowOwner::Bytes(value.schema.into_bytes())),
                Some(FlowOwner::Widgets(value.widgets)),
                Some(FlowOwner::Specs(value.synapses)),
                Some(FlowOwner::Layouts(value.layout)),
            ]),
            FlowOwner::Widget(value) => self.widget(value),
            FlowOwner::Widgets(mut values) => {
                let value = values.pop().expect("nonempty widget owner");
                self.install([Some(FlowOwner::Widgets(values)), Some(FlowOwner::Widget(value))]);
            }
            FlowOwner::Specs(mut values) => {
                let value = values.pop().expect("nonempty specification owner");
                self.install([
                    Some(FlowOwner::Specs(values)),
                    Some(FlowOwner::Bytes(value.id.into_bytes())),
                    Some(FlowOwner::Bytes(value.from.into_bytes())),
                    Some(FlowOwner::Bytes(value.to.into_bytes())),
                    Some(FlowOwner::Bytes(value.from_port.into_bytes())),
                    Some(FlowOwner::Bytes(value.to_port.into_bytes())),
                ]);
            }
            FlowOwner::Layouts(values) => self.install([Some(FlowOwner::LayoutCursor(values.retire()))]),
            FlowOwner::LayoutCursor(mut values) => {
                let step = values.advance(Grant { maximum_items: 1, maximum_bytes });
                let pending = !values.is_empty();
                let blocked = matches!(step, RetirementStep::Blocked);
                match step {
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::OwnedValue(_) | RetirementStep::Complete | RetirementStep::Blocked => {}
                }
                if pending {
                    self.install([Some(FlowOwner::LayoutCursor(values))]);
                }
                if blocked { return Ok(Step::Blocked); }
            }
            FlowOwner::Tree(value) => self.install([Some(FlowOwner::Neurons(value.neurons)), Some(FlowOwner::Synapses(value.synapses))]),
            FlowOwner::Neurons(mut values) => {
                let value = values.pop().expect("nonempty neuron owner");
                self.install([
                    Some(FlowOwner::Neurons(values)),
                    Some(FlowOwner::Bytes(value.id.into_bytes())),
                    Some(FlowOwner::Bytes(value.kind.into_bytes())),
                    Some(FlowOwner::Dictionary(value.params)),
                    value.tree.map(|tree| FlowOwner::Tree(*tree)),
                ]);
            }
            FlowOwner::Synapses(mut values) => {
                let value = values.pop().expect("nonempty synapse owner");
                self.install([
                    Some(FlowOwner::Synapses(values)),
                    Some(FlowOwner::Bytes(value.id.into_bytes())),
                    Some(FlowOwner::Bytes(value.from.into_bytes())),
                    Some(FlowOwner::Bytes(value.to.into_bytes())),
                    Some(FlowOwner::Bytes(value.from_port.into_bytes())),
                    Some(FlowOwner::Bytes(value.to_port.into_bytes())),
                ]);
            }
            FlowOwner::Gui(value) => self.install([Some(FlowOwner::Nodes(value.nodes)), Some(FlowOwner::Previews(value.previews))]),
            FlowOwner::Nodes(values) => self.install([Some(FlowOwner::NodeCursor(values.retire()))]),
            FlowOwner::NodeCursor(mut values) => {
                let step = values.advance(Grant { maximum_items: 1, maximum_bytes });
                let pending = !values.is_empty();
                let blocked = matches!(step, RetirementStep::Blocked);
                match step {
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::OwnedValue(value) => self.install([Some(FlowOwner::Chrome(value.chrome))]),
                    RetirementStep::Complete | RetirementStep::Blocked => {}
                }
                if pending {
                    self.install([Some(FlowOwner::NodeCursor(values))]);
                }
                if blocked { return Ok(Step::Blocked); }
            }
            FlowOwner::Previews(mut values) => {
                let value = values.pop().expect("nonempty preview owner");
                let (neuron, channel) = value.source.map_or((None, None), |source| {
                    (Some(FlowOwner::Bytes(source.neuron.into_bytes())), Some(FlowOwner::Bytes(source.channel.into_bytes())))
                });
                self.install([
                    Some(FlowOwner::Previews(values)),
                    Some(FlowOwner::Bytes(value.id.into_bytes())),
                    Some(FlowOwner::Bytes(value.mode.into_bytes())),
                    Some(FlowOwner::Dictionary(value.preview)),
                    Some(FlowOwner::Set(value.expanded)),
                    neuron,
                    channel,
                    None,
                ]);
            }
            FlowOwner::Layout(mut values) => {
                let value = values.pop().expect("nonempty layout owner");
                self.install([Some(FlowOwner::Layout(values)), Some(FlowOwner::Bytes(value.id.into_bytes()))]);
            }
            FlowOwner::Chrome(value) => self.chrome(value),
        }
        Ok(Step::Pending { released_items: 1, released_bytes })
    }

    fn terminal_is_empty(&self) -> bool {
        FlowRetirement::terminal_is_empty(self)
    }
}

impl Drop for FlowRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("Flow retirement dropped with live owned payloads");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.root);
            ManuallyDrop::drop(&mut self.frontier);
        }
    }
}
//#endregion 🧹️TypedRetirement

//#region 🧪️RetirementLaws
#[cfg(test)]
#[path = "🧪️tests/🧵️retained/🦀️.rs"]
mod tests;
//#endregion 🧪️RetirementLaws
