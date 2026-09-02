//! 🗿️ Retained typed scene copying under an immutable rooted borrow witness.

use super::{FlowOwner as Owner, FlowRetirement as Retirement};
use crate::os_store::{ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
use std::mem::ManuallyDrop;
use crate::{FlowFixture, neural, FlowChannelRef, FlowGui, FlowNodeGui, FlowPreviewGui, NodeChrome, SynapseSpec, Widget, WidgetLayout};
use std::any::Any;
use std::collections::LinkedList;
use std::sync::Arc;

//#region 🔐️RootedProjection
/// 🔒️ Private projections remain valid because the immutable scene allocation outlives every task.
struct Rooted<T> {
    root: Arc<dyn Any + Send + Sync>,
    pointer: *const T,
}

impl<T> Clone for Rooted<T> {
    fn clone(&self) -> Self { Self { root: Arc::clone(&self.root), pointer: self.pointer } }
}

/// 🧷️ Only immutable shared field references enter this witness; the scene guard is Send and Sync.
unsafe impl<T: Sync> Send for Rooted<T> {}

impl<T> Rooted<T> {
    fn get(&self) -> &T { unsafe { &*self.pointer } }

    fn project<U>(&self, field: impl for<'a> FnOnce(&'a T) -> &'a U) -> Rooted<U> {
        Rooted { root: Arc::clone(&self.root), pointer: field(self.get()) as *const U }
    }
}
//#endregion 🔐️RootedProjection

//#region 🧵️CopyTasks
trait Retire: Send + 'static { fn retire(self, retirement: &mut Retirement); }
trait Copy: Retire + Sync + Sized { fn task(source: Rooted<Self>) -> Box<dyn Task>; }

trait Copied: Send {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send>;
    fn retire(self: Box<Self>, retirement: &mut Retirement);
}

struct Value<T: Retire>(T);
impl<T: Retire> Copied for Value<T> {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> { Box::new(self.0) }
    fn retire(self: Box<Self>, retirement: &mut Retirement) { self.0.retire(retirement); }
}

fn take<T: Retire>(value: Box<dyn Copied>) -> T {
    match value.into_any().downcast::<T>() { Ok(value) => *value, Err(_) => panic!("Flow copy task result type mismatch") }
}

enum Advance { Child(Box<dyn Task>), Bytes(usize), Complete(Box<dyn Copied>), Fault(String) }
trait Task: Send {
    fn advance(&mut self, maximum_bytes: usize, allocation: &mut FlowCopyAllocationBudget) -> Advance;
    fn accept(&mut self, value: Box<dyn Copied>);
    fn retire(self: Box<Self>, retirement: &mut Retirement);
}

struct TextTask { source: Rooted<String>, bytes: Vec<u8>, reserved: bool }
impl Retire for String { fn retire(self, retirement: &mut Retirement) { retirement.push(Owner::Bytes(self.into_bytes())); } }
impl Copy for String {
    fn task(source: Rooted<Self>) -> Box<dyn Task> {
        Box::new(TextTask { source, bytes: Vec::new(), reserved: false })
    }
}

/// 🧬️ A completed buffer is the exact immutable valid UTF-8 source, copied in disjoint prefix slices.
impl Task for TextTask {
    fn advance(&mut self, maximum_bytes: usize, allocation: &mut FlowCopyAllocationBudget) -> Advance {
        let source = self.source.get().as_bytes();
        if !self.reserved {
            if let Err(error) = allocation.reserve(&mut self.bytes, source.len()) { return Advance::Fault(error); }
            self.reserved = true;
            return Advance::Bytes(0);
        }
        if self.bytes.len() == source.len() {
            return Advance::Complete(Box::new(Value(unsafe { String::from_utf8_unchecked(std::mem::take(&mut self.bytes)) })));
        }
        let start = self.bytes.len();
        let count = maximum_bytes.min(source.len() - start);
        self.bytes.extend_from_slice(&source[start..start + count]);
        Advance::Bytes(count)
    }
    fn accept(&mut self, _: Box<dyn Copied>) { unreachable!() }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { retirement.push(Owner::Bytes(std::mem::take(&mut self.bytes))); }
}

struct RecordTask<T: Copy> {
    source: Rooted<T>, target: Option<T>, index: usize,
    next: fn(&Rooted<T>, usize) -> Option<Box<dyn Task>>,
    set: fn(&mut T, usize, Box<dyn Copied>),
}

impl<T: Copy> Task for RecordTask<T> {
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocationBudget) -> Advance {
        match (self.next)(&self.source, self.index) {
            Some(child) => Advance::Child(child),
            None => Advance::Complete(Box::new(Value(self.target.take().expect("record copy target")))),
        }
    }
    fn accept(&mut self, value: Box<dyn Copied>) { (self.set)(self.target.as_mut().unwrap(), self.index, value); self.index += 1; }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { if let Some(target) = self.target.take() { target.retire(retirement); } }
}

macro_rules! record {
    ($type:ty, $empty:expr, { $($index:literal => $field:ident : $field_type:ty),* $(,)? }) => {
        impl Copy for $type {
            fn task(source: Rooted<Self>) -> Box<dyn Task> {
                let target = ($empty)(source.get());
                Box::new(RecordTask { source, target: Some(target), index: 0,
                    next: |source, index| match index { $($index => Some(<$field_type as Copy>::task(source.project(|value| &value.$field))),)* _ => None },
                    set: |target, index, value| match index { $($index => target.$field = take::<$field_type>(value),)* _ => unreachable!() },
                })
            }
        }
    };
}

struct VectorTask<T: Copy> where Vec<T>: Retire { source: Rooted<Vec<T>>, target: Vec<T>, index: usize, reserved: bool }
impl<T: Copy> Copy for Vec<T> where Vec<T>: Retire {
    fn task(source: Rooted<Self>) -> Box<dyn Task> {
        Box::new(VectorTask { source, target: Vec::new(), index: 0, reserved: false })
    }
}
impl<T: Copy> Task for VectorTask<T> where Vec<T>: Retire {
    fn advance(&mut self, _: usize, allocation: &mut FlowCopyAllocationBudget) -> Advance {
        if !self.reserved {
            if let Err(error) = allocation.reserve(&mut self.target, self.source.get().len()) { return Advance::Fault(error); }
            self.reserved = true;
            return Advance::Bytes(0);
        }
        if self.index == self.source.get().len() { return Advance::Complete(Box::new(Value(std::mem::take(&mut self.target)))); }
        let index = self.index;
        Advance::Child(T::task(self.source.project(move |values| &values[index])))
    }
    fn accept(&mut self, value: Box<dyn Copied>) { self.target.push(take::<T>(value)); self.index += 1; }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { std::mem::take(&mut self.target).retire(retirement); }
}

/// 🪞️ Immutable collection roots share one pointer; payload bytes are neither copied nor compared.
struct SharedTask<T: Copy + Clone> { source: Rooted<T> }
impl<T: Copy + Clone> Task for SharedTask<T> {
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocationBudget) -> Advance { Advance::Complete(Box::new(Value(self.source.get().clone()))) }
    fn accept(&mut self, _: Box<dyn Copied>) { unreachable!("shared root has no child copy"); }
    fn retire(self: Box<Self>, _: &mut Retirement) {}
}
impl<T: Copy> Copy for crate::OrderedMap<T> where crate::OrderedMap<T>: Retire {
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(SharedTask { source }) }
}
impl Copy for neural::Dictionary {
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(SharedTask { source }) }
}
impl Copy for crate::OrderedSet {
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(SharedTask { source }) }
}

impl<T: Retire> Retire for Option<T> { fn retire(self, retirement: &mut Retirement) { if let Some(value) = self { value.retire(retirement); } } }
impl<T: Retire> Retire for Box<T> { fn retire(self, retirement: &mut Retirement) { (*self).retire(retirement); } }
struct OptionTask<T: Copy> { source: Rooted<Option<T>>, target: Option<T>, complete: bool }
impl<T: Copy> Copy for Option<T> {
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(OptionTask { source, target: None, complete: false }) }
}
impl<T: Copy> Task for OptionTask<T> {
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocationBudget) -> Advance {
        if self.complete || self.source.get().is_none() { return Advance::Complete(Box::new(Value(self.target.take()))); }
        Advance::Child(T::task(self.source.project(|source| source.as_ref().unwrap())))
    }
    fn accept(&mut self, value: Box<dyn Copied>) { self.target = Some(take::<T>(value)); self.complete = true; }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { self.target.take().retire(retirement); }
}
struct BoxTask<T: Copy> { source: Rooted<Box<T>>, target: Option<Box<T>> }
impl<T: Copy> Copy for Box<T> {
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(BoxTask { source, target: None }) }
}
impl<T: Copy> Task for BoxTask<T> {
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocationBudget) -> Advance {
        if let Some(target) = self.target.take() { return Advance::Complete(Box::new(Value(target))); }
        Advance::Child(T::task(self.source.project(|value| value.as_ref())))
    }
    fn accept(&mut self, value: Box<dyn Copied>) { self.target = Some(Box::new(take::<T>(value))); }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { self.target.take().retire(retirement); }
}
//#endregion 🧵️CopyTasks

//#region 🧬️DomainRecords
macro_rules! retire_owner {
    ($type:ty, $variant:ident) => { impl Retire for $type { fn retire(self, retirement: &mut Retirement) { retirement.push(Owner::$variant(self)); } } };
}
retire_owner!(FlowFixture, Fixture);
retire_owner!(Widget, Widget);
retire_owner!(Vec<Widget>, Widgets);
retire_owner!(Vec<String>, Strings);
retire_owner!(Vec<SynapseSpec>, Specs);
retire_owner!(crate::OrderedMap<WidgetLayout>, Layouts);
retire_owner!(neural::Dictionary, Dictionary);
retire_owner!(neural::Value, Value);
retire_owner!(neural::Tree, Tree);
retire_owner!(Vec<neural::Neuron>, Neurons);
retire_owner!(Vec<neural::Synapse>, Synapses);
retire_owner!(FlowGui, Gui);
retire_owner!(crate::OrderedMap<FlowNodeGui>, Nodes);
retire_owner!(Vec<FlowPreviewGui>, Previews);
retire_owner!(crate::OrderedSet, Set);
retire_owner!(NodeChrome, Chrome);

impl Retire for SynapseSpec { fn retire(self, retirement: &mut Retirement) { vec![self].retire(retirement); } }
impl Retire for neural::Neuron { fn retire(self, retirement: &mut Retirement) { vec![self].retire(retirement); } }
impl Retire for neural::Synapse { fn retire(self, retirement: &mut Retirement) { vec![self].retire(retirement); } }
impl Retire for FlowPreviewGui { fn retire(self, retirement: &mut Retirement) { vec![self].retire(retirement); } }
impl Retire for FlowNodeGui { fn retire(self, retirement: &mut Retirement) { self.chrome.retire(retirement); } }
impl Retire for FlowChannelRef { fn retire(self, retirement: &mut Retirement) { vec![self.neuron, self.channel].retire(retirement); } }
impl Retire for WidgetLayout { fn retire(self, _: &mut Retirement) {} }
impl Copy for WidgetLayout {
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(RecordTask { target: Some(source.get().clone()), source, index: 0, next: |_, _| None, set: |_, _, _| unreachable!() }) }
}

record!(FlowFixture, |source: &FlowFixture| FlowFixture { schema: String::new(), camera: source.camera.clone(), widgets: Vec::new(), synapses: Vec::new(), layout: crate::OrderedMap::new() }, { 0 => schema: String, 1 => widgets: Vec<Widget>, 2 => synapses: Vec<SynapseSpec>, 3 => layout: crate::OrderedMap<WidgetLayout> });
record!(SynapseSpec, |_: &SynapseSpec| SynapseSpec { id: String::new(), from: String::new(), to: String::new(), from_port: String::new(), to_port: String::new() }, { 0 => id: String, 1 => from: String, 2 => to: String, 3 => from_port: String, 4 => to_port: String });
record!(neural::Tree, |_: &neural::Tree| neural::Tree::default(), { 0 => neurons: Vec<neural::Neuron>, 1 => synapses: Vec<neural::Synapse> });
record!(neural::Neuron, |_: &neural::Neuron| neural::Neuron { id: String::new(), kind: String::new(), params: neural::Dictionary::new(), tree: None }, { 0 => id: String, 1 => kind: String, 2 => params: neural::Dictionary, 3 => tree: Option<Box<neural::Tree>> });
record!(neural::Synapse, |_: &neural::Synapse| neural::Synapse { id: String::new(), from: String::new(), to: String::new(), from_port: String::new(), to_port: String::new() }, { 0 => id: String, 1 => from: String, 2 => to: String, 3 => from_port: String, 4 => to_port: String });
record!(FlowGui, |source: &FlowGui| FlowGui { camera: source.camera.clone(), nodes: crate::OrderedMap::new(), previews: Vec::new() }, { 0 => nodes: crate::OrderedMap<FlowNodeGui>, 1 => previews: Vec<FlowPreviewGui> });
record!(FlowNodeGui, |source: &FlowNodeGui| FlowNodeGui { layout: source.layout.clone(), chrome: NodeChrome::Plain { preview: false } }, { 0 => chrome: NodeChrome });
record!(FlowPreviewGui, |source: &FlowPreviewGui| FlowPreviewGui { id: String::new(), source: None, mode: String::new(), preview: neural::Dictionary::new(), expanded: crate::OrderedSet::new(), layout: source.layout.clone() }, { 0 => id: String, 1 => source: Option<FlowChannelRef>, 2 => mode: String, 3 => preview: neural::Dictionary, 4 => expanded: crate::OrderedSet });
record!(FlowChannelRef, |_: &FlowChannelRef| FlowChannelRef { neuron: String::new(), channel: String::new() }, { 0 => neuron: String, 1 => channel: String });

macro_rules! variants {
    ($type:ty, $empty:expr, { $($variant:path => { $($index:literal => $field:ident : $field_type:ty),* $(,)? }),* $(,)? }) => {
        impl Copy for $type {
            fn task(source: Rooted<Self>) -> Box<dyn Task> {
                let target = ($empty)(source.get());
                Box::new(RecordTask { source, target: Some(target), index: 0,
                    next: |source, index| match (source.get(), index) {
                        $($(($variant { .. }, $index) => Some(<$field_type as Copy>::task(source.project(|value| match value { $variant { $field, .. } => $field, _ => unreachable!() }))),)*)*
                        _ => None,
                    },
                    set: |target, index, value| match (target, index) {
                        $($(($variant { $field, .. }, $index) => *$field = take::<$field_type>(value),)*)*
                        _ => unreachable!(),
                    },
                })
            }
        }
    };
}

fn empty_widget(source: &Widget) -> Widget {
    match source {
        Widget::Neuron { preview, .. } => Widget::Neuron { id: String::new(), neuron_kind: String::new(), params: neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: *preview },
        Widget::InputSlider { value, min, max, step, .. } => Widget::InputSlider { id: String::new(), label: String::new(), value: *value, min: *min, max: *max, step: *step },
        Widget::InputNote { .. } => Widget::InputNote { id: String::new(), text: String::new() },
        Widget::InputImage { .. } => Widget::InputImage { id: String::new(), src: String::new() },
        Widget::Variable { .. } => Widget::Variable { id: String::new(), name: String::new(), schema: String::new() },
        Widget::OutputPreview { .. } => Widget::OutputPreview { id: String::new(), preview: neural::Dictionary::new(), expanded: crate::OrderedSet::new() },
        Widget::OutputAction { .. } => Widget::OutputAction { id: String::new(), action: String::new() },
        Widget::OutputExport { .. } => Widget::OutputExport { id: String::new(), format: String::new() },
        Widget::Cluster { .. } => Widget::Cluster { id: String::new(), name: String::new(), tree: neural::Tree::default(), flow: FlowGui::default() },
    }
}
variants!(Widget, empty_widget, {
    Widget::Neuron => { 0 => id: String, 1 => neuron_kind: String, 2 => params: neural::Dictionary, 3 => input_ports: Vec<String>, 4 => output_ports: Vec<String> },
    Widget::InputSlider => { 0 => id: String, 1 => label: String },
    Widget::InputNote => { 0 => id: String, 1 => text: String },
    Widget::InputImage => { 0 => id: String, 1 => src: String },
    Widget::Variable => { 0 => id: String, 1 => name: String, 2 => schema: String },
    Widget::OutputPreview => { 0 => id: String, 1 => preview: neural::Dictionary, 2 => expanded: crate::OrderedSet },
    Widget::OutputAction => { 0 => id: String, 1 => action: String },
    Widget::OutputExport => { 0 => id: String, 1 => format: String },
    Widget::Cluster => { 0 => id: String, 1 => name: String, 2 => tree: neural::Tree, 3 => flow: FlowGui },
});

fn empty_chrome(source: &NodeChrome) -> NodeChrome {
    match source {
        NodeChrome::Plain { preview } => NodeChrome::Plain { preview: *preview },
        NodeChrome::Slider { min, max, step, value, .. } => NodeChrome::Slider { label: String::new(), min: *min, max: *max, step: *step, value: *value },
        NodeChrome::Note { .. } => NodeChrome::Note { text: String::new() },
        NodeChrome::Image { .. } => NodeChrome::Image { src: String::new() },
        NodeChrome::Variable { .. } => NodeChrome::Variable { name: String::new(), schema: String::new() },
    }
}
variants!(NodeChrome, empty_chrome, {
    NodeChrome::Slider => { 0 => label: String },
    NodeChrome::Note => { 0 => text: String }, NodeChrome::Image => { 0 => src: String }, NodeChrome::Variable => { 0 => name: String, 1 => schema: String },
});

impl Copy for neural::Value {
    fn task(source: Rooted<Self>) -> Box<dyn Task> {
        let target = match source.get() {
            neural::Value::Atom(neural::Atom::String(_)) | neural::Value::Dictionary(_) => neural::Value::null(),
            value => value.clone(),
        };
        Box::new(RecordTask { source, target: Some(target), index: 0,
            next: |source, index| if index != 0 { None } else { match source.get() {
                neural::Value::Atom(neural::Atom::String(_)) => Some(String::task(source.project(|value| match value { neural::Value::Atom(neural::Atom::String(value)) => value, _ => unreachable!() }))),
                neural::Value::Dictionary(_) => Some(neural::Dictionary::task(source.project(|value| match value { neural::Value::Dictionary(value) => value, _ => unreachable!() }))),
                _ => None,
            } },
            set: |target, _, value| {
                let value = value.into_any();
                *target = match value.downcast::<String>() {
                    Ok(value) => neural::Value::Atom(neural::Atom::String(*value)),
                    Err(value) => neural::Value::Dictionary(*value.downcast::<neural::Dictionary>().expect("dictionary clone value")),
                };
            },
        })
    }
}
//#endregion 🧬️DomainRecords


//#region 🗿️SelectedCopyCursor
/// 🎟️ The owning domain admits contiguous uninitialized reservations separately from copied byte work.
#[derive(Debug)]
pub struct FlowCopyAllocationBudget {
    maximum_single_bytes: usize,
    maximum_total_bytes: usize,
    reserved_bytes: usize,
    reservation_count: usize,
}

impl FlowCopyAllocationBudget {
    pub fn new(maximum_single_bytes: usize, maximum_total_bytes: usize) -> Self {
        Self { maximum_single_bytes, maximum_total_bytes, reserved_bytes: 0, reservation_count: 0 }
    }
    pub fn reserved_bytes(&self) -> usize { self.reserved_bytes }
    pub fn reservation_count(&self) -> usize { self.reservation_count }
    fn reserve<T>(&mut self, target: &mut Vec<T>, count: usize) -> Result<(), String> {
        if !target.is_empty() || target.capacity() != 0 { return Err("Flow allocation reservation requires an empty unallocated target".into()); }
        let bytes = count.checked_mul(std::mem::size_of::<T>()).ok_or("Flow allocation size overflow")?;
        let total = self.reserved_bytes.checked_add(bytes).ok_or("Flow allocation cumulative overflow")?;
        if bytes > self.maximum_single_bytes || total > self.maximum_total_bytes { return Err("Flow allocation exceeds owner admission".into()); }
        target.try_reserve_exact(count).map_err(|_| "Flow allocation reservation failed")?;
        self.reserved_bytes = total;
        self.reservation_count += 1;
        Ok(())
    }
}

struct CopyState<R: Send + Sync + 'static, T: Copy> {
    tasks: LinkedList<Box<dyn Task>>,
    result: Option<T>,
    retirement: Retirement,
    active_root_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    source: Option<Arc<R>>,
    root_retirement: Option<Arc<dyn SnapshotRetirementFactory<R>>>,
    allocation: FlowCopyAllocationBudget,
    project: for<'a> fn(&'a R, usize) -> Option<&'a T>,
    index: usize,
    started: bool,
    finished: bool,
    failed: bool,
    closing: bool,
}

struct CopyCursor<R: Send + Sync + 'static, T: Copy> { owned: ManuallyDrop<CopyState<R, T>> }

impl<R: Send + Sync + 'static, T: Copy> CopyCursor<R, T> {
    fn new(source: Arc<R>, index: usize, project: for<'a> fn(&'a R, usize) -> Option<&'a T>, root_retirement: Arc<dyn SnapshotRetirementFactory<R>>, allocation: FlowCopyAllocationBudget) -> Self {
        Self { owned: ManuallyDrop::new(CopyState { tasks: LinkedList::new(), result: None, retirement: Retirement::default(), active_root_retirement: None, source: Some(source), root_retirement: Some(root_retirement), allocation, project, index, started: false, finished: false, failed: false, closing: false }) }
    }
    fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<Option<usize>, String> {
        let state = &mut *self.owned;
        if state.closing || state.failed || maximum_items == 0 || maximum_bytes == 0 { return Ok(None); }
        if state.finished { return Ok(Some(0)); }
        if !state.started {
            state.started = true;
            let source = state.source.as_ref().expect("selected copy root");
            let Some(selected) = (state.project)(source, state.index) else {
                state.failed = true;
                return Err("selected Flow copy projection is absent".into());
            };
            let pointer = selected as *const T;
            let root: Arc<dyn Any + Send + Sync> = source.clone();
            state.tasks.push_front(T::task(Rooted { root, pointer }));
            return Ok(Some(0));
        }
        let Some(mut task) = state.tasks.pop_front() else { return Err("selected Flow copy lost its task".into()); };
        match task.advance(maximum_bytes, &mut state.allocation) {
            Advance::Child(child) => { state.tasks.push_front(task); state.tasks.push_front(child); Ok(Some(0)) }
            Advance::Bytes(bytes) => { state.tasks.push_front(task); Ok(Some(bytes)) }
            Advance::Fault(error) => { state.tasks.push_front(task); state.failed = true; Err(error) }
            Advance::Complete(value) => {
                if let Some(parent) = state.tasks.front_mut() { parent.accept(value); }
                else { state.result = Some(take::<T>(value)); state.finished = true; }
                Ok(Some(0))
            }
        }
    }
    fn complete(&self) -> bool { self.owned.finished && !self.owned.failed && !self.owned.closing }
    fn take(&mut self) -> Option<T> { if self.complete() { self.owned.result.take() } else { None } }
    fn begin_close(&mut self) { self.owned.closing = true; }
    fn terminal_is_empty(&self) -> bool {
        let state = &*self.owned;
        state.closing && state.tasks.is_empty() && state.result.is_none() && state.retirement.is_empty() && state.active_root_retirement.is_none() && state.source.is_none() && state.root_retirement.is_none()
    }
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        use SnapshotRetirementStep as Step;
        if self.terminal_is_empty() { return Ok(Step::Complete); }
        let state = &mut *self.owned;
        if !state.closing || maximum_items == 0 || maximum_bytes == 0 { return Ok(Step::Blocked); }
        if !state.retirement.is_empty() { return state.retirement.close_step(1, maximum_bytes); }
        if let Some(task) = state.tasks.pop_front() { task.retire(&mut state.retirement); }
        else if let Some(result) = state.result.take() { result.retire(&mut state.retirement); }
        else if let Some(active) = state.active_root_retirement.as_mut() {
            let step = active.close_step(1, maximum_bytes)?;
            if matches!(step, Step::Pending { released_items, released_bytes } if released_items > 1 || released_bytes > maximum_bytes) {
                return Err("selected Flow root retirement exceeded its grant".into());
            }
            if matches!(step, Step::Complete) {
                if !active.terminal_is_empty() { return Err("selected Flow root retirement is not terminal".into()); }
                state.active_root_retirement = None;
                return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        else if let Some(root) = state.source.take() { state.active_root_retirement = Some(state.root_retirement.as_ref().expect("selected copy retirement factory").retire(root)); }
        else if state.root_retirement.take().is_some() {}
        else { return Ok(Step::Complete); }
        Ok(Step::Pending { released_items: 1, released_bytes: 0 })
    }
}

impl<R: Send + Sync + 'static, T: Copy> Drop for CopyCursor<R, T> {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() { panic!("selected Flow copy dropped with live ownership"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}

macro_rules! selected_cursor {
    ($name:ident, $value:ty) => {
        #[must_use = "selected Flow copy must transfer its result and close to terminal-empty"]
        pub struct $name<R: Send + Sync + 'static> { cursor: CopyCursor<R, $value> }
        impl<R: Send + Sync + 'static> $name<R> {
            pub fn new(root: Arc<R>, index: usize, project: for<'a> fn(&'a R, usize) -> Option<&'a $value>, retirement: Arc<dyn SnapshotRetirementFactory<R>>, allocation: FlowCopyAllocationBudget) -> Self {
                Self { cursor: CopyCursor::new(root, index, project, retirement, allocation) }
            }
            pub fn allocation(&self) -> &FlowCopyAllocationBudget { &self.cursor.owned.allocation }
            pub fn advance(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<Option<usize>, String> { self.cursor.advance(maximum_items, maximum_bytes) }
            pub fn complete(&self) -> bool { self.cursor.complete() }
            pub fn take(&mut self) -> Option<$value> { self.cursor.take() }
            pub fn begin_close(&mut self) { self.cursor.begin_close(); }
            pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> { self.cursor.close_step(maximum_items, maximum_bytes) }
            pub fn terminal_is_empty(&self) -> bool { self.cursor.terminal_is_empty() }
        }
    };
}
selected_cursor!(FlowWidgetCopy, Widget);
selected_cursor!(FlowSynapseCopy, SynapseSpec);
selected_cursor!(FlowFixtureCopy, FlowFixture);
//#endregion 🗿️SelectedCopyCursor

//#region 🧪️CopyLaws
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️CopyLaws
