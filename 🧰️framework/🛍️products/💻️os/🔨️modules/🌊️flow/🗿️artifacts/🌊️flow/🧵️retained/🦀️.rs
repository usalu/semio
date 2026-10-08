//! 🧵️ Shared typed Flow ownership frontiers for resumable copying and retirement.

use crate::{neural, FlowArtifact, FlowHostSnapshot, FlowGui, FlowLayoutEntry, FlowNodeGui, FlowPreviewGui, NodeChrome, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout};
use semio_framework_value::{ValueError,retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement,queue::RetirementQueue},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

//#region 📑️SelectedCopy
#[path = "📑️copy/🦀️.rs"]
pub mod copy;
pub use copy::{FlowCopyAllocationBudget, FlowHostSnapshotCopy, FlowSynapseCopy, FlowWidgetCopy};

#[path = "🧬️fields/🦀️.rs"]
mod fields;
//#endregion 📑️SelectedCopy

//#region 🧹️TypedRetirement
pub enum FlowOwner {
    Bytes(Vec<u8>),Strings(Vec<String>),Set(OrderedSet),Dictionary(neural::Dictionary),Value(neural::Value),
    HostSnapshot(FlowHostSnapshot),Widget(Widget),Widgets(Vec<Widget>),Specs(Vec<SynapseSpec>),Layouts(OrderedMap<WidgetLayout>),
    Tree(neural::Tree),Neurons(Vec<neural::Neuron>),Synapses(Vec<neural::Synapse>),Gui(FlowGui),Nodes(OrderedMap<FlowNodeGui>),
    Previews(Vec<FlowPreviewGui>),Layout(Vec<FlowLayoutEntry>),Chrome(NodeChrome),Mutation(crate::FlowMutation),
}
macro_rules! owned_variant {
    ($($variant:ident),+)=>{impl RetireOwned for FlowOwner {
        fn retirement(self)->Box<dyn RetirementCursor> {match self {$(Self::$variant(value)=>value.retirement()),+}}
        fn retirement_birth_bytes(&self)->Option<usize> {match self {$(Self::$variant(value)=>value.retirement_birth_bytes()),+}}
        fn controlled_retirement_supported()->bool {true}
    }};
}
owned_variant!(Bytes,Strings,Set,Dictionary,Value,HostSnapshot,Widget,Widgets,Specs,Layouts,Tree,Neurons,Synapses,Gui,Nodes,Previews,Layout,Chrome,Mutation);

#[must_use="Flow ownership must close every original owner and admitted scaffold"]
pub struct FlowRetirement {root:Option<ControlledRetirement<FlowOwner>>,queue:RetirementQueue}
impl Default for FlowRetirement {fn default()->Self {Self {root:None,queue:RetirementQueue::default()}}}
impl FlowRetirement {
    pub fn from_owner(owner:FlowOwner)->Self {Self {root:Some(ControlledRetirement::new(owner).unwrap_or_else(|(error,_)|panic!("Flow owner retirement refused: {error}"))),queue:RetirementQueue::default()}}
    pub fn push(&mut self,owner:FlowOwner)->Result<(),FlowOwner> {if self.root.is_some(){return Err(owner);}self.root=Some(ControlledRetirement::new(owner).unwrap_or_else(|(error,_)|panic!("Flow owner retirement refused: {error}")));Ok(())}
    pub fn text(&mut self,text:String)->Result<(),FlowOwner> {self.push(FlowOwner::Bytes(text.into_bytes()))}
    pub fn next_push_capacity_byte_demand(&self)->Result<usize,ValueError> {if self.queue.has_reserved_slot(){Ok(RetirementQueue::frame_birth_bytes::<FlowOwner>())}else{self.queue.next_reserve_capacity_byte_demand()}}
    pub fn reserve_push(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {self.queue.reserve_step(grant)}
    pub fn admit_owner(&mut self,owner:FlowOwner,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,FlowOwner)> {self.queue.admit_owned(owner,grant)}
    pub fn push_cold(&mut self,owner:FlowOwner) {
        if self.root.is_none(){self.push(owner).unwrap_or_else(|_|panic!("empty Flow root admission refused"));return;}
        while !self.queue.has_reserved_slot(){self.reserve_push(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:self.queue.next_reserve_capacity_byte_demand().expect("cold Flow queue capacity"),maximum_release_bytes:0,maximum_depth:self.queue.len()+1}).expect("cold Flow queue reservation");}
        self.admit_owner(owner,RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:RetirementQueue::frame_birth_bytes::<FlowOwner>(),maximum_release_bytes:0,maximum_depth:self.queue.len()+1}).unwrap_or_else(|(error,_)|panic!("cold Flow frame admission refused: {error}"));
    }
    pub fn terminal_is_empty(&self)->bool {self.root.is_none()&&self.queue.terminal_is_empty()}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.queue.next_copy_byte_demand(),ControlledRetirement::next_copy_byte_demand)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.queue.next_capacity_byte_demand(copy),|owner|owner.next_capacity_byte_demand(copy))}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.queue.next_release_byte_demand(),ControlledRetirement::next_release_byte_demand)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.queue.next_depth_demand(),ControlledRetirement::next_depth_demand)}
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));}
        if let Some(root)=self.root.as_mut(){if root.terminal_is_empty(){self.root=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}let step=root.step(grant)?;return Ok(if matches!(step,RetainedCloneStep::Complete(_)){RetainedCloneStep::Progress(step.progress())}else{step});}
        self.queue.step(grant)
    }
    pub fn retire_cold(mut self) {
        while !self.terminal_is_empty(){let copy=self.next_copy_byte_demand().expect("cold Flow copy demand");let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:self.next_capacity_byte_demand(copy).expect("cold Flow allocation demand"),maximum_release_bytes:self.next_release_byte_demand().expect("cold Flow release demand"),maximum_depth:self.next_depth_demand().expect("cold Flow depth demand")};let step=self.step(grant).expect("cold Flow retirement");assert!(step.progress().copied_items!=0,"cold Flow retirement stalled at exact demand");}
    }
}
impl Drop for FlowRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Flow retirement abandoned original ownership");}}

impl FlowHostSnapshot {
    /// 🧊️ Explicit cold-only disposal of a detached fixture.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::HostSnapshot(self)).retire_cold();
    }

    /// 🧊️ Installs a whole widget roster AND retires the one it displaces. A bare
    /// `fixture.widgets = vec![…]` drops every displaced `Widget` — each of which owns a
    /// `Dictionary`/`OrderedSet`/`Tree` that refuses a bare drop — so the assignment panics with
    /// `final Dictionary ownership must be explicitly retired or owned by a cold boundary` at the
    /// END of the caller, naming nothing that points back at the assignment. This is the paid form,
    /// the twin of the `std::mem::replace(&mut self.fixture, fixture).retire_cold()` the flow host's
    /// own `apply_fixture` already pays (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn replace_widgets(&mut self, widgets: Vec<Widget>) {
        FlowRetirement::from_owner(FlowOwner::Widgets(std::mem::replace(&mut self.widgets, widgets))).retire_cold();
    }
}

impl Widget {
    /// 🧊️ Explicit cold-only disposal of a detached widget.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Widget(self)).retire_cold();
    }
}

impl FlowGui {
    /// 🧊️ Explicit cold-only disposal of a detached gui projection — its `nodes` are an
    /// `OrderedMap<FlowNodeGui>` that refuses a bare drop.
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Gui(self)).retire_cold();
    }
}

impl FlowArtifact {
    /// 🧊️ Explicit cold-only disposal of a detached artifact projection. `FlowHost::document()`
    /// hands back an OWNED projection built by `FlowHostSnapshot::to_artifact`, and both of its halves
    /// refuse a bare drop — the `Tree`'s neurons own `Dictionary` params, the `FlowUi`'s `nodes` are
    /// an `OrderedMap` — so a caller that only reads it still has to close it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn retire_cold(self) {
        FlowRetirement::from_owner(FlowOwner::Tree(self.tree)).retire_cold();
        self.ui.retire_cold();
    }
}

//#region 🧪️RetirementLaws
#[cfg(test)]
#[path = "🧪️tests/🧵️retained/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧬️fields/🦀️.rs"]
mod field_tests;
//#endregion 🧪️RetirementLaws
