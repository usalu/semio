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

/// 🎟️ The live payload a backing still owes before its allocation is freed. An element vector that
/// has already been drained owes nothing — its remaining `capacity` is an allocation, not payload,
/// and `capacity * size_of::<T>()` is machine-width dependent, so it can never be charged against a
/// caller's byte grant (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn owner_backing_payload(owner: &FlowOwner) -> usize {
    match owner {
        FlowOwner::Bytes(values) => values.len(),
        _ => 0,
    }
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
    /// 🎟️ Payload bytes already charged against the CURRENT root's backing. A `Vec` cannot be freed
    /// in pieces, so the caller's grant is drawn down `min(grant, left)` per turn and the allocation
    /// is released once the charge is settled — never `Blocked`, which is what made every fixed-page
    /// driver spin (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    root_backing_credit: usize,
    fault: Option<&'static str>,
}

impl Default for FlowRetirement {
    fn default() -> Self {
        Self { root: ManuallyDrop::new(None), frontier: ManuallyDrop::new(PagedList::empty()), root_backing_credit: 0, fault: None }
    }
}

impl FlowRetirement {
    pub fn from_owner(owner: FlowOwner) -> Self {
        Self { root: ManuallyDrop::new(Some(owner)), frontier: ManuallyDrop::new(PagedList::empty()), root_backing_credit: 0, fault: None }
    }

    pub fn push(&mut self, owner: FlowOwner) {
        if self.root.is_none() {
            *self.root = Some(owner);
            self.root_backing_credit = 0;
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
        self.root_backing_credit = 0;
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

    /// 🎟️ One byte of credit per turn is all a close needs. Payload is drawn down `min(grant, left)`
    /// and every ALLOCATION this frontier owns — a reserved page, an emptied element vector — is
    /// paid out of the frontier's own reservation currency (`allocated_bytes`), never out of the
    /// caller's payload grant, because `capacity * size_of::<T>()` is machine-width dependent
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn next_close_byte_demand(&self) -> Result<usize, &'static str> {
        if let Some(owner) = self.root.as_ref() {
            return owner_release_demand(owner);
        }
        Ok(usize::from(!self.terminal_is_empty()))
    }

    /// 📄️ Pays this frontier's OWN demands under the caller's page grant, then closes one owner —
    /// the single entry point every retained driver should use.
    ///
    /// ⚠️ `FlowRetirement` is a RESERVE-then-CLOSE frontier: a bare
    /// [`ErasedSnapshotRetirement::close_step`] answers `Blocked` — never an error — for as long as
    /// `next_allocation_bytes` still names a page the current owner's decomposition needs, so a
    /// driver that only ever closes spins silently forever
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn close_page(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(demand) = self.next_allocation_bytes().map_err(str::to_owned)? {
            self.reserve_allocation(demand).map_err(|error| error.reason.to_owned())?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        <Self as ErasedSnapshotRetirement>::close_step(self, maximum_items, maximum_bytes)
    }

    /// 🧊️ Explicit cold-only teardown; retained callers use [`FlowRetirement::close_page`].
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
            self.root_backing_credit = 0;
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

    /// 🧩️ One owner family's retirement step, `None` when the family blocked on its grant. Every
    /// family owns a SEPARATE function on purpose: `FlowOwner` is `size_of` 3160 bytes (its
    /// `SetCursor`/`LayoutCursor`/`NodeCursor` payloads inline `Retirement<V>`'s
    /// `MAX_AVL_HEIGHT + 3` slot array), and an unoptimised build gives every `install([…])`
    /// temporary of every arm its own slot in the enclosing frame — one flat `match` compiled to a
    /// 518 KiB `close_step` frame, two of which overflow a 2 MiB test thread mid-teardown and abort
    /// the binary under whichever test happened to be running
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    fn retire_owner(&mut self, owner: FlowOwner, maximum_bytes: usize) -> Option<usize> {
        match owner {
            FlowOwner::Bytes(_) => unreachable!("byte backing is released before logical dispatch"),
            FlowOwner::Strings(values) => self.strings(values),
            FlowOwner::Set(values) => self.set(values),
            FlowOwner::SetCursor(values) => self.set_cursor(values, maximum_bytes),
            FlowOwner::Dictionary(value) => self.dictionary(value),
            FlowOwner::Value(value) => self.value(value),
            FlowOwner::Neural(value) => self.neural(value, maximum_bytes),
            FlowOwner::Fixture(value) => self.fixture(value),
            FlowOwner::Widget(value) => {
                self.widget(value);
                Some(0)
            }
            FlowOwner::Widgets(values) => self.widgets(values),
            FlowOwner::Specs(values) => self.specs(values),
            FlowOwner::Layouts(values) => self.layouts(values),
            FlowOwner::LayoutCursor(values) => self.layout_cursor(values, maximum_bytes),
            FlowOwner::Tree(value) => self.tree(value),
            FlowOwner::Neurons(values) => self.neurons(values),
            FlowOwner::Synapses(values) => self.synapses(values),
            FlowOwner::Gui(value) => self.gui(value),
            FlowOwner::Nodes(values) => self.nodes(values),
            FlowOwner::NodeCursor(values) => self.node_cursor(values, maximum_bytes),
            FlowOwner::Previews(values) => self.previews(values),
            FlowOwner::Layout(values) => self.layout(values),
            FlowOwner::Chrome(value) => {
                self.chrome(value);
                Some(0)
            }
        }
    }

    fn strings(&mut self, mut values: Vec<String>) -> Option<usize> {
        let value = values.pop().expect("nonempty string owner");
        self.install([Some(FlowOwner::Strings(values)), Some(FlowOwner::Bytes(value.into_bytes()))]);
        Some(0)
    }

    fn set(&mut self, values: OrderedSet) -> Option<usize> {
        self.install([Some(FlowOwner::SetCursor(values.retire()))]);
        Some(0)
    }

    fn set_cursor(&mut self, mut values: Retirement<()>, maximum_bytes: usize) -> Option<usize> {
        let step = values.advance(Grant { maximum_items: 1, maximum_bytes });
        let pending = !values.is_empty();
        let blocked = matches!(step, RetirementStep::Blocked);
        let mut released_bytes = 0;
        match step {
            RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
            RetirementStep::OwnedValue(()) | RetirementStep::Complete | RetirementStep::Blocked => {}
        }
        if pending {
            self.install([Some(FlowOwner::SetCursor(values))]);
        }
        (!blocked).then_some(released_bytes)
    }

    fn dictionary(&mut self, value: neural::Dictionary) -> Option<usize> {
        self.install([Some(FlowOwner::Neural(neural::ValueRetirement::from_dictionary(value)))]);
        Some(0)
    }

    fn value(&mut self, value: neural::Value) -> Option<usize> {
        self.install([Some(FlowOwner::Neural(neural::ValueRetirement::from_value(value)))]);
        Some(0)
    }

    fn neural(&mut self, mut value: neural::ValueRetirement, maximum_bytes: usize) -> Option<usize> {
        let step = value.close_step(1, maximum_bytes);
        let blocked = matches!(step, neural::ValueRetirementStep::Blocked);
        let mut released_bytes = 0;
        match step {
            neural::ValueRetirementStep::Pending { released_bytes: bytes, .. } => released_bytes = bytes,
            neural::ValueRetirementStep::Blocked | neural::ValueRetirementStep::Complete => {}
        }
        if !value.terminal_is_empty() {
            self.install([Some(FlowOwner::Neural(value))]);
        }
        (!blocked).then_some(released_bytes)
    }

    fn fixture(&mut self, value: FlowFixture) -> Option<usize> {
        self.install([
            Some(FlowOwner::Bytes(value.schema.into_bytes())),
            Some(FlowOwner::Widgets(value.widgets)),
            Some(FlowOwner::Specs(value.synapses)),
            Some(FlowOwner::Layouts(value.layout)),
        ]);
        Some(0)
    }

    fn widgets(&mut self, mut values: Vec<Widget>) -> Option<usize> {
        let value = values.pop().expect("nonempty widget owner");
        self.install([Some(FlowOwner::Widgets(values)), Some(FlowOwner::Widget(value))]);
        Some(0)
    }

    fn specs(&mut self, mut values: Vec<SynapseSpec>) -> Option<usize> {
        let value = values.pop().expect("nonempty specification owner");
        self.install([
            Some(FlowOwner::Specs(values)),
            Some(FlowOwner::Bytes(value.id.into_bytes())),
            Some(FlowOwner::Bytes(value.from.into_bytes())),
            Some(FlowOwner::Bytes(value.to.into_bytes())),
            Some(FlowOwner::Bytes(value.from_port.into_bytes())),
            Some(FlowOwner::Bytes(value.to_port.into_bytes())),
        ]);
        Some(0)
    }

    fn layouts(&mut self, values: OrderedMap<WidgetLayout>) -> Option<usize> {
        self.install([Some(FlowOwner::LayoutCursor(values.retire()))]);
        Some(0)
    }

    fn layout_cursor(&mut self, mut values: Retirement<WidgetLayout>, maximum_bytes: usize) -> Option<usize> {
        let step = values.advance(Grant { maximum_items: 1, maximum_bytes });
        let pending = !values.is_empty();
        let blocked = matches!(step, RetirementStep::Blocked);
        let mut released_bytes = 0;
        match step {
            RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
            RetirementStep::OwnedValue(_) | RetirementStep::Complete | RetirementStep::Blocked => {}
        }
        if pending {
            self.install([Some(FlowOwner::LayoutCursor(values))]);
        }
        (!blocked).then_some(released_bytes)
    }

    fn tree(&mut self, value: neural::Tree) -> Option<usize> {
        self.install([Some(FlowOwner::Neurons(value.neurons)), Some(FlowOwner::Synapses(value.synapses))]);
        Some(0)
    }

    fn neurons(&mut self, mut values: Vec<neural::Neuron>) -> Option<usize> {
        let value = values.pop().expect("nonempty neuron owner");
        self.install([
            Some(FlowOwner::Neurons(values)),
            Some(FlowOwner::Bytes(value.id.into_bytes())),
            Some(FlowOwner::Bytes(value.kind.into_bytes())),
            Some(FlowOwner::Dictionary(value.params)),
            value.tree.map(|tree| FlowOwner::Tree(*tree)),
        ]);
        Some(0)
    }

    fn synapses(&mut self, mut values: Vec<neural::Synapse>) -> Option<usize> {
        let value = values.pop().expect("nonempty synapse owner");
        self.install([
            Some(FlowOwner::Synapses(values)),
            Some(FlowOwner::Bytes(value.id.into_bytes())),
            Some(FlowOwner::Bytes(value.from.into_bytes())),
            Some(FlowOwner::Bytes(value.to.into_bytes())),
            Some(FlowOwner::Bytes(value.from_port.into_bytes())),
            Some(FlowOwner::Bytes(value.to_port.into_bytes())),
        ]);
        Some(0)
    }

    fn gui(&mut self, value: FlowGui) -> Option<usize> {
        self.install([Some(FlowOwner::Nodes(value.nodes)), Some(FlowOwner::Previews(value.previews))]);
        Some(0)
    }

    fn nodes(&mut self, values: OrderedMap<FlowNodeGui>) -> Option<usize> {
        self.install([Some(FlowOwner::NodeCursor(values.retire()))]);
        Some(0)
    }

    fn node_cursor(&mut self, mut values: Retirement<FlowNodeGui>, maximum_bytes: usize) -> Option<usize> {
        let step = values.advance(Grant { maximum_items: 1, maximum_bytes });
        let pending = !values.is_empty();
        let blocked = matches!(step, RetirementStep::Blocked);
        let mut released_bytes = 0;
        match step {
            RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
            RetirementStep::OwnedValue(value) => self.install([Some(FlowOwner::Chrome(value.chrome))]),
            RetirementStep::Complete | RetirementStep::Blocked => {}
        }
        if pending {
            self.install([Some(FlowOwner::NodeCursor(values))]);
        }
        (!blocked).then_some(released_bytes)
    }

    fn previews(&mut self, mut values: Vec<FlowPreviewGui>) -> Option<usize> {
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
        Some(0)
    }

    fn layout(&mut self, mut values: Vec<FlowLayoutEntry>) -> Option<usize> {
        let value = values.pop().expect("nonempty layout owner");
        self.install([Some(FlowOwner::Layout(values)), Some(FlowOwner::Bytes(value.id.into_bytes()))]);
        Some(0)
    }

    /// 🎟️ Draws the current root's backing payload down by at most `maximum_bytes` and frees the
    /// whole allocation once the charge is settled. `Some((released_items, charged))` is one turn of
    /// progress; `released_items` is 1 only on the turn that actually frees.
    fn release_root_backing(&mut self, maximum_bytes: usize) -> Option<(usize, usize)> {
        let owner = self.root.as_ref()?;
        if !owner_waits_for_backing_release(owner) {
            return None;
        }
        let payload = owner_backing_payload(owner);
        let charged = maximum_bytes.min(payload.saturating_sub(self.root_backing_credit));
        self.root_backing_credit = self.root_backing_credit.saturating_add(charged);
        if self.root_backing_credit < payload {
            return Some((0, charged));
        }
        self.root_backing_credit = 0;
        *self.root = None;
        Some((1, charged))
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
        if let Some((released_items, released_bytes)) = self.release_root_backing(maximum_bytes) {
            return Ok(Step::Pending { released_items, released_bytes });
        }
        if self.root.is_none() {
            if let Some(owner) = self.frontier.pop() {
                *self.root = Some(owner);
                self.root_backing_credit = 0;
                return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
            }
            let demand = self.frontier.next_release_allocation_bytes().map_err(str::to_owned)?;
            let release = self.frontier.release_empty_page(maximum_bytes.max(demand)).map_err(str::to_owned)?;
            if !release.progressed {
                return Ok(Step::Blocked);
            }
            return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
        }
        let owner = self.root.take().expect("nonempty Flow retirement");
        let Some(released_bytes) = self.retire_owner(owner, maximum_bytes) else {
            return Ok(Step::Blocked);
        };
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
