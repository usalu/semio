//! 🗿️ Retained typed scene copying under an immutable rooted borrow witness.

use semio_framework_artifact_flow_flow_document::*;

/// 📑️ Selects the next child-copy task for a rooted record.
type RecordCopyTaskFactory<T> = fn(&Rooted<T>, usize) -> Option<Box<dyn Task>>;
type RecordCopyTaskQuote<T> = fn(&Rooted<T>, usize) -> Option<TaskAdmission>;

use crate::retained::{FlowOwner as Owner, FlowRetirement as Retirement};
use crate::os_store::{ErasedSnapshotRetirement, SnapshotRetirementFactory};
use std::mem::{ManuallyDrop,size_of,size_of_val};
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use crate::{FlowHostComponents, FlowHostSnapshot, neural, FlowChannelRef, FlowGui, FlowNodeGui, FlowPreviewGui, NodeChrome, SynapseSpec, Widget, WidgetLayout};
use std::any::Any;
use semio_framework_value::retained_clone::RetainedCloneSource;
use crate::os_store::snapshot_read::SnapshotRead;
use semio_framework_value::retirement::RetireOwned;
use std::collections::LinkedList;
use std::sync::Arc;

//#region 🔐️RootedProjection
/// 🔒️ Private projections remain valid because the cursor retains its original source until every task closes.
struct Rooted<T> {
    pointer: *const T,
}

impl<T> Clone for Rooted<T> {
    fn clone(&self) -> Self { Self { pointer: self.pointer } }
}

/// 🧷️ Only immutable shared field references enter this witness; the scene guard is Send and Sync.
unsafe impl<T: Sync> Send for Rooted<T> {}

impl<T> Rooted<T> {
    fn get(&self) -> &T { unsafe { &*self.pointer } }

    fn project<U>(&self, field: impl for<'a> FnOnce(&'a T) -> &'a U) -> Rooted<U> {
        Rooted { pointer: field(self.get()) as *const U }
    }
}
//#endregion 🔐️RootedProjection

//#region 🧵️CopyTasks
trait Retire: Send + 'static { fn retire(self, retirement: &mut Retirement); }
trait Copy: Retire + Sync + Sized { fn task(source: Rooted<Self>) -> Box<dyn Task>; fn task_bytes()->usize {size_of::<RecordTask<Self>>() } fn task_copy_bytes(_: &Self)->usize {0} }

trait Copied: Send {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send>;
}

struct Value<T: Retire>(T);
impl<T: Retire> Copied for Value<T> {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> { self }
}

fn take<T: Retire>(value: Box<dyn Copied>) -> T {
    match value.into_any().downcast::<Value<T>>() { Ok(value) => value.0, Err(_) => panic!("Flow copy task result type mismatch") }
}

enum Advance { Child(Box<dyn Task>), Bytes(usize), Complete(Box<dyn Copied>), Fault(ValueError) }
#[derive(Default)]
struct TaskAdmission { copy:usize, capacity:usize, result:usize }
fn child<T:Copy>(source:&T)->TaskAdmission {TaskAdmission {capacity:T::task_bytes()+task_link_bytes(),copy:T::task_copy_bytes(source),..Default::default()}}
fn result<T:Retire>()->TaskAdmission {TaskAdmission {capacity:size_of::<Value<T>>(),result:size_of::<Value<T>>(),..Default::default()}}
fn task_link_bytes()->usize {size_of::<(usize,usize,Box<dyn Task>)>()}
trait Task: Send {
    fn next_admission(&self,maximum_bytes:usize)->TaskAdmission;
    fn accept_capacity(&self)->usize {0}
    fn retire_release(&self)->usize {0}
    fn advance(&mut self, maximum_bytes: usize, allocation: &mut FlowCopyAllocations) -> Advance;
    fn accept(&mut self, value: Box<dyn Copied>);
    fn retire(self: Box<Self>, retirement: &mut Retirement);
}

struct TextTask { source: Rooted<String>, bytes: Vec<u8>, reserved: bool }
impl Retire for String { fn retire(self, retirement: &mut Retirement) { retirement.push(Owner::Bytes(self.into_bytes())).unwrap_or_else(|_|panic!("Flow copy requires empty retirement handoff")); } }
impl Copy for String {
    fn task_bytes()->usize {size_of::<TextTask>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> {
        Box::new(TextTask { source, bytes: Vec::new(), reserved: false })
    }
}

/// 🧬️ A completed buffer is the exact immutable valid UTF-8 source, copied in disjoint prefix slices.
impl Task for TextTask {
    fn next_admission(&self,maximum_bytes:usize)->TaskAdmission {
        if !self.reserved{return TaskAdmission{capacity:self.source.get().len(),..Default::default()}}
        if self.bytes.len()==self.source.get().len(){return result::<String>()}
        TaskAdmission{copy:maximum_bytes.max(1).min(self.source.get().len()-self.bytes.len()),..Default::default()}
    }
    fn advance(&mut self, maximum_bytes: usize, allocation: &mut FlowCopyAllocations) -> Advance {
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
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { retirement.push(Owner::Bytes(std::mem::take(&mut self.bytes))).unwrap_or_else(|_|panic!("Flow copy requires empty retirement handoff")); }
}

struct RecordTask<T: Copy> {
    source: Rooted<T>, target: Option<T>, index: usize,
    next: RecordCopyTaskFactory<T>,
    quote: RecordCopyTaskQuote<T>,
    set: fn(&mut T, usize, Box<dyn Copied>),
}

impl<T: Copy> Task for RecordTask<T> {
    fn next_admission(&self,_:usize)->TaskAdmission {(self.quote)(&self.source,self.index).unwrap_or_else(result::<T>)}
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocations) -> Advance {
        match (self.next)(&self.source, self.index) {
            Some(child) => Advance::Child(child),
            None => Advance::Complete(Box::new(Value(self.target.take().expect("record copy target")))),
        }
    }
    fn accept(&mut self, value: Box<dyn Copied>) { (self.set)(self.target.as_mut().unwrap(), self.index, value); self.index += 1; }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { if let Some(target) = self.target.take() { target.retire(retirement); } }
}

macro_rules! record {
    ($type:ty, $copied:expr, $empty:expr, { $($index:literal => $field:ident : $field_type:ty),* $(,)? }) => {
        impl Copy for $type {
            fn task_copy_bytes(source:&Self)->usize {($copied)(source)}
            fn task(source: Rooted<Self>) -> Box<dyn Task> {
                let target = ($empty)(source.get());
                Box::new(RecordTask { source, target: Some(target), index: 0,
                    next: |source, index| match index { $($index => Some(<$field_type as Copy>::task(source.project(|value| &value.$field))),)* _ => None },
                    quote: |source, index| match index { $($index => Some(child::<$field_type>(&source.get().$field)),)* _ => None },
                    set: |target, index, value| match index { $($index => target.$field = take::<$field_type>(value),)* _ => unreachable!() },
                })
            }
        }
    };
}

struct VectorTask<T: Copy> where Vec<T>: Retire { source: Rooted<Vec<T>>, target: Vec<T>, index: usize, reserved: bool }
impl<T: Copy> Copy for Vec<T> where Vec<T>: Retire {
    fn task_bytes()->usize {size_of::<VectorTask<T>>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> {
        Box::new(VectorTask { source, target: Vec::new(), index: 0, reserved: false })
    }
}
impl<T: Copy> Task for VectorTask<T> where Vec<T>: Retire {
    fn next_admission(&self,_:usize)->TaskAdmission {
        if !self.reserved{return TaskAdmission{capacity:self.source.get().len().checked_mul(size_of::<T>()).unwrap_or(usize::MAX),..Default::default()}}
        if self.index==self.source.get().len(){result::<Vec<T>>()}else{child::<T>(&self.source.get()[self.index])}
    }
    fn advance(&mut self, _: usize, allocation: &mut FlowCopyAllocations) -> Advance {
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
    fn next_admission(&self,_:usize)->TaskAdmission {result::<T>()}
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocations) -> Advance { Advance::Complete(Box::new(Value(self.source.get().clone()))) }
    fn accept(&mut self, _: Box<dyn Copied>) { unreachable!("shared root has no child copy"); }
    fn retire(self: Box<Self>, _: &mut Retirement) {}
}
impl<T: Copy> Copy for crate::OrderedMap<T> where crate::OrderedMap<T>: Retire {
    fn task_bytes()->usize {size_of::<SharedTask<Self>>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(SharedTask { source }) }
}
impl Copy for neural::Dictionary {
    fn task_bytes()->usize {size_of::<SharedTask<Self>>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(SharedTask { source }) }
}
impl Copy for crate::OrderedSet {
    fn task_bytes()->usize {size_of::<SharedTask<Self>>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(SharedTask { source }) }
}

impl<T: Retire> Retire for Option<T> { fn retire(self, retirement: &mut Retirement) { if let Some(value) = self { value.retire(retirement); } } }
impl<T: Retire> Retire for Box<T> { fn retire(self, retirement: &mut Retirement) { (*self).retire(retirement); } }
struct OptionTask<T: Copy> { source: Rooted<Option<T>>, target: Option<T>, complete: bool }
impl<T: Copy> Copy for Option<T> {
    fn task_bytes()->usize {size_of::<OptionTask<T>>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(OptionTask { source, target: None, complete: false }) }
}
impl<T: Copy> Task for OptionTask<T> {
    fn next_admission(&self,_:usize)->TaskAdmission {if self.complete||self.source.get().is_none(){result::<Option<T>>()}else{child::<T>(self.source.get().as_ref().unwrap())}}
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocations) -> Advance {
        if self.complete || self.source.get().is_none() { return Advance::Complete(Box::new(Value(self.target.take()))); }
        Advance::Child(T::task(self.source.project(|source| source.as_ref().unwrap())))
    }
    fn accept(&mut self, value: Box<dyn Copied>) { self.target = Some(take::<T>(value)); self.complete = true; }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { self.target.take().retire(retirement); }
}
struct BoxTask<T: Copy> { source: Rooted<Box<T>>, target: Option<Box<T>> }
impl<T: Copy> Copy for Box<T> {
    fn task_bytes()->usize {size_of::<BoxTask<T>>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(BoxTask { source, target: None }) }
}
impl<T: Copy> Task for BoxTask<T> {
    fn retire_release(&self)->usize {if self.target.is_some(){size_of::<T>()}else{0}}
    fn accept_capacity(&self)->usize {size_of::<T>()}
    fn next_admission(&self,_:usize)->TaskAdmission {if self.target.is_some(){result::<Box<T>>()}else{child::<T>(self.source.get().as_ref())}}
    fn advance(&mut self, _: usize, _: &mut FlowCopyAllocations) -> Advance {
        if let Some(target) = self.target.take() { return Advance::Complete(Box::new(Value(target))); }
        Advance::Child(T::task(self.source.project(|value| value.as_ref())))
    }
    fn accept(&mut self, value: Box<dyn Copied>) { self.target = Some(Box::new(take::<T>(value))); }
    fn retire(mut self: Box<Self>, retirement: &mut Retirement) { self.target.take().retire(retirement); }
}
//#endregion 🧵️CopyTasks

//#region 🧬️DomainRecords
macro_rules! retire_owner {
    ($type:ty, $variant:ident) => { impl Retire for $type { fn retire(self, retirement: &mut Retirement) { retirement.push(Owner::$variant(self)).unwrap_or_else(|_|panic!("Flow copy requires empty retirement handoff")); } } };
}
impl Retire for FlowHostSnapshot { fn retire(self, retirement: &mut Retirement) { retirement.push(Owner::HostSnapshot(FlowHostComponents::from(self))).unwrap_or_else(|_|panic!("Flow copy requires empty retirement handoff")); } }
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

retire_owner!(SynapseSpec, Spec);
retire_owner!(neural::Neuron, Neuron);
retire_owner!(neural::Synapse, Synapse);
retire_owner!(FlowPreviewGui, Preview);
impl Retire for FlowNodeGui { fn retire(self, retirement: &mut Retirement) { self.chrome.retire(retirement); } }
retire_owner!(FlowChannelRef, Channel);
impl Retire for WidgetLayout { fn retire(self, _: &mut Retirement) {} }
impl Copy for WidgetLayout {
    fn task_copy_bytes(_: &Self)->usize {size_of::<Self>()}
    fn task(source: Rooted<Self>) -> Box<dyn Task> { Box::new(RecordTask { target: Some(source.get().clone()), source, index: 0, next: |_, _| None, quote: |_, _| None, set: |_, _, _| unreachable!() }) }
}

record!(FlowHostSnapshot,|source:&FlowHostSnapshot|size_of_val(&source.camera), |source: &FlowHostSnapshot| FlowHostSnapshot { schema: String::new(), camera: source.camera.clone(), widgets: Vec::new(), synapses: Vec::new(), layout: crate::OrderedMap::new() }, { 0 => schema: String, 1 => widgets: Vec<Widget>, 2 => synapses: Vec<SynapseSpec>, 3 => layout: crate::OrderedMap<WidgetLayout> });
record!(SynapseSpec,|_:&SynapseSpec|0, |_: &SynapseSpec| SynapseSpec { id: String::new(), from: String::new(), to: String::new(), from_port: String::new(), to_port: String::new() }, { 0 => id: String, 1 => from: String, 2 => to: String, 3 => from_port: String, 4 => to_port: String });
record!(neural::Tree,|_:&neural::Tree|0, |_: &neural::Tree| neural::Tree::default(), { 0 => neurons: Vec<neural::Neuron>, 1 => synapses: Vec<neural::Synapse> });
record!(neural::Neuron,|_:&neural::Neuron|0, |_: &neural::Neuron| neural::Neuron { id: String::new(), kind: String::new(), params: neural::Dictionary::new(), tree: None }, { 0 => id: String, 1 => kind: String, 2 => params: neural::Dictionary, 3 => tree: Option<Box<neural::Tree>> });
record!(neural::Synapse,|_:&neural::Synapse|0, |_: &neural::Synapse| neural::Synapse { id: String::new(), from: String::new(), to: String::new(), from_port: String::new(), to_port: String::new() }, { 0 => id: String, 1 => from: String, 2 => to: String, 3 => from_port: String, 4 => to_port: String });
record!(FlowGui,|source:&FlowGui|size_of_val(&source.camera), |source: &FlowGui| FlowGui { camera: source.camera.clone(), nodes: crate::OrderedMap::new(), previews: Vec::new() }, { 0 => nodes: crate::OrderedMap<FlowNodeGui>, 1 => previews: Vec<FlowPreviewGui> });
record!(FlowNodeGui,|source:&FlowNodeGui|size_of_val(&source.layout), |source: &FlowNodeGui| FlowNodeGui { layout: source.layout.clone(), chrome: NodeChrome::Plain { preview: false } }, { 0 => chrome: NodeChrome });
record!(FlowPreviewGui,|source:&FlowPreviewGui|size_of_val(&source.layout), |source: &FlowPreviewGui| FlowPreviewGui { id: String::new(), source: None, mode: String::new(), preview: neural::Dictionary::new(), expanded: crate::OrderedSet::new(), layout: source.layout.clone() }, { 0 => id: String, 1 => source: Option<FlowChannelRef>, 2 => mode: String, 3 => preview: neural::Dictionary, 4 => expanded: crate::OrderedSet });
record!(FlowChannelRef,|_:&FlowChannelRef|0, |_: &FlowChannelRef| FlowChannelRef { neuron: String::new(), channel: String::new() }, { 0 => neuron: String, 1 => channel: String });

macro_rules! variants {
    ($type:ty, $copied:expr, $empty:expr, { $($variant:path => { $($index:literal => $field:ident : $field_type:ty),* $(,)? }),* $(,)? }) => {
        impl Copy for $type {
            fn task_copy_bytes(source:&Self)->usize {($copied)(source)}
            fn task(source: Rooted<Self>) -> Box<dyn Task> {
                let target = ($empty)(source.get());
                Box::new(RecordTask { source, target: Some(target), index: 0,
                    next: |source, index| match (source.get(), index) {
                        $($(($variant { .. }, $index) => Some(<$field_type as Copy>::task(source.project(|value| match value { $variant { $field, .. } => $field, _ => unreachable!() }))),)*)*
                        _ => None,
                    },
                    quote: |source,index| match (source.get(),index) {
                        $($(($variant { $field,.. }, $index) => Some(child::<$field_type>($field)),)*)*
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
variants!(Widget,|source:&Widget|match source{Widget::Neuron{..}=>size_of::<bool>(),Widget::InputSlider{..}=>size_of::<[f64;4]>(),_=>0}, empty_widget, {
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
variants!(NodeChrome,|source:&NodeChrome|match source{NodeChrome::Plain{..}=>size_of::<bool>(),NodeChrome::Slider{..}=>size_of::<[f64;4]>(),_=>0}, empty_chrome, {
    NodeChrome::Slider => { 0 => label: String },
    NodeChrome::Note => { 0 => text: String }, NodeChrome::Image => { 0 => src: String }, NodeChrome::Variable => { 0 => name: String, 1 => schema: String },
});

impl Copy for neural::Value {
    fn task_copy_bytes(source:&Self)->usize {match source{neural::Value::Atom(neural::Atom::Integer(_)|neural::Atom::Decimal(_))=>8,neural::Value::Atom(neural::Atom::Boolean(_))=>1,_=>0}}
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
            quote: |source,index| if index!=0{None}else{match source.get(){
                neural::Value::Atom(neural::Atom::String(value))=>Some(child::<String>(value)),
                neural::Value::Dictionary(value)=>Some(child::<neural::Dictionary>(value)),_=>None,
            }},
            set: |target, _, value| {
                let value = value.into_any();
                *target = match value.downcast::<Value<String>>() {
                    Ok(value) => neural::Value::Atom(neural::Atom::String(value.0)),
                    Err(value) => neural::Value::Dictionary(value.downcast::<Value<neural::Dictionary>>().expect("dictionary clone value").0),
                };
            },
        })
    }
}
//#endregion 🧬️DomainRecords


//#region 🗿️SelectedCopyCursor
/// ⛔️ How many consecutive close steps a selected copy tolerates without a byte freed and without
/// an owner moved before it refuses. Every nested retirement is granted the demand it publishes
/// through its independent copy, capacity, release, and depth demands, so a run of non-progress steps is a
/// broken owner, not back-pressure, and a driver that kept asking would spin — which is exactly what
/// a predecessor's doubling-offer workaround did for 21 minutes
/// (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END).
const FLOW_COPY_CLOSE_STALL_BOUND: usize = 64;

/// 🛑️ Counts consecutive close steps that freed nothing and moved nothing, and refuses past
/// [`FLOW_COPY_CLOSE_STALL_BOUND`] with a named fault instead of handing its caller another
/// `Blocked` to spin on.
fn account(stalled:&mut usize,grant:RetainedCloneGrant,step:RetainedCloneStep,terminal:bool,owner:&str)->Result<RetainedCloneStep,ValueError> {
    let step=admit_retained_clone_close(grant,step,terminal,owner)?;
    if matches!(step,RetainedCloneStep::Progress(progress) if progress==RetainedCloneProgress::default()) {*stalled+=1;if *stalled>FLOW_COPY_CLOSE_STALL_BOUND{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,format!("{owner} made no progress at its published close demand")));}}else{*stalled=0;}
    Ok(step)
}

/// 🎟️ The owning domain admits contiguous uninitialized reservations separately from copied byte work.
#[derive(Debug)]
pub struct FlowCopyAllocations {
    reserved_bytes: usize,
    reservation_count: usize,
}

impl FlowCopyAllocations {
    fn new() -> Self {
        Self { reserved_bytes: 0, reservation_count: 0 }
    }
    pub fn reserved_bytes(&self) -> usize { self.reserved_bytes }
    pub fn reservation_count(&self) -> usize { self.reservation_count }
    fn reserve<T>(&mut self, target: &mut Vec<T>, count: usize) -> Result<(), ValueError> {
        if !target.is_empty() || target.capacity() != 0 { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Flow allocation reservation requires an empty unallocated target")); }
        let bytes = count.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Flow allocation size overflow"))?;
        let total = self.reserved_bytes.checked_add(bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Flow allocation cumulative overflow"))?;
        target.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"Flow allocation reservation failed"))?;
        self.reserved_bytes = total;
        self.reservation_count += 1;
        Ok(())
    }
}

struct CopyState<R: RetireOwned + Send + Sync + 'static, T: Copy> {
    tasks: LinkedList<Box<dyn Task>>,
    result: Option<T>,
    retirement: Retirement,
    active_root_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    factory_close:Option<Box<dyn semio_framework_value::FactoryRetirementTicket>>,
    source: Option<Arc<R>>,
    source_lease: Option<RetainedCloneSource<T,SnapshotRead<R>>>,
    source_generation: Option<u64>,
    root_retirement: Option<Arc<dyn SnapshotRetirementFactory<R>>>,
    factory_source:Option<Arc<dyn semio_framework_value::FactoryRetirement>>,
    allocation: FlowCopyAllocations,
    progress: RetainedCloneProgress,
    origin: Option<(u64,u64)>,
    revision: [u8;32],
    project: for<'a> fn(&'a R, usize) -> Option<&'a T>,
    index: usize,
    started: bool,
    finished: bool,
    failed: bool,
    closing: bool,
    stalled_steps: usize,
}

struct CopyCursor<R: RetireOwned + Send + Sync + 'static, T: Copy> { owned: ManuallyDrop<CopyState<R, T>> }

impl<R: RetireOwned + Send + Sync + 'static, T: Copy> CopyCursor<R, T> {
    fn new(source: Arc<R>, index: usize, project: for<'a> fn(&'a R, usize) -> Option<&'a T>, root_retirement: Arc<dyn SnapshotRetirementFactory<R>>,revision:[u8;32]) -> Self {
        Self { owned: ManuallyDrop::new(CopyState { tasks: LinkedList::new(), result: None, retirement: Retirement::default(), active_root_retirement: None,factory_close:None, source: Some(source),source_lease:None,source_generation:None, root_retirement: Some(root_retirement),factory_source:None, allocation:FlowCopyAllocations::new(), progress:Default::default(),origin:None,revision, project, index, started: false, finished: false, failed: false, closing: false, stalled_steps: 0 }) }
    }
    fn next_normal_demands(&self,maximum_bytes:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        let state=&*self.owned;
        if state.closing||state.failed||state.finished{return Ok(Default::default())}
        let admission=if !state.started{if let Some(source)=state.source_lease.as_ref(){child::<T>(source.try_borrow()?.get())}else{let source=state.source.as_ref().expect("selected copy root");(state.project)(source,state.index).map_or(TaskAdmission{capacity:T::task_bytes()+task_link_bytes(),..Default::default()},child::<T>)}}else{state.tasks.front().expect("selected copy task").next_admission(maximum_bytes)};
        let completed=admission.result>0;
        let parent_capacity=if completed{{let mut parents=state.tasks.iter();parents.next();parents.next().map_or(0,|parent|parent.accept_capacity())}}else{0};
        Ok(semio_framework_value::RetirementDemand{copy_bytes:admission.copy,capacity_bytes:admission.capacity.checked_add(parent_capacity).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"Flow copy capacity overflows"))?,release_bytes:if completed{size_of_val(state.tasks.front().unwrap().as_ref())+task_link_bytes()+admission.result}else{0},depth:1})
    }
    fn advance(&mut self,grant:RetainedCloneGrant)->Result<Option<usize>,ValueError> {
        self.owned.progress=Default::default();
        let demand=self.next_normal_demands(grant.maximum_copy_bytes)?;
        let state=&mut*self.owned;
        if state.closing||state.failed||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(None)}
        if state.finished{return Ok(Some(0))}
        if !state.started {
            let pointer=if let Some(source)=state.source_lease.as_ref(){source.try_borrow()?.get()as*const T}else{
                let source=state.source.as_ref().expect("selected copy root");
                let Some(selected)=(state.project)(source,state.index)else{state.failed=true;return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"selected Flow copy projection is absent"))};selected as*const T
            };
            state.tasks.push_front(T::task(Rooted{pointer}));
            state.started=true;
        }else{
            match state.tasks.front_mut().expect("selected copy task").advance(grant.maximum_copy_bytes,&mut state.allocation){
                Advance::Child(child)=>state.tasks.push_front(child),
                Advance::Bytes(_)=>{},
                Advance::Fault(error)=>{state.failed=true;return Err(error)},
                Advance::Complete(value)=>{
                    state.tasks.pop_front();
                    if let Some(parent)=state.tasks.front_mut(){parent.accept(value)}else{state.result=Some(take::<T>(value));state.finished=true;}
                }
            }
        }
        state.progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes};
        Ok(Some(demand.copy_bytes))
    }
    fn complete(&self) -> bool { self.owned.finished && !self.owned.failed && !self.owned.closing }
    fn take(&mut self) -> Option<T> { if self.complete() { self.owned.result.take() } else { None } }
    fn begin_close(&mut self) { self.owned.closing = true; }
    fn terminal_is_empty(&self) -> bool {
        let state = &*self.owned;
        state.closing && state.tasks.is_empty() && state.result.is_none() && state.retirement.terminal_is_empty() && state.active_root_retirement.is_none() && state.factory_close.is_none() && state.source.is_none() && state.source_lease.is_none() && state.root_retirement.is_none() && state.factory_source.is_none()
    }
    fn close_owner(&self)->Option<&dyn ErasedSnapshotRetirement>{self.owned.active_root_retirement.as_deref().or_else(||self.owned.factory_close.as_deref().map(|owner|owner as &dyn ErasedSnapshotRetirement))}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {let state=&*self.owned;if !state.retirement.terminal_is_empty(){return state.retirement.next_copy_byte_demand()}if let Some(owner)=self.close_owner(){return owner.next_copy_byte_demand()}if state.tasks.is_empty()&&state.result.is_none(){if let Some(source)=state.source_lease.as_ref(){return Ok(source.next_close_demand(0)?.copy_bytes)}}Ok(state.factory_source.as_ref().map_or(0,|factory|factory.factory_retirement_copy_byte_demand()))}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {let state=&*self.owned;if !state.retirement.terminal_is_empty(){return state.retirement.next_capacity_byte_demand(copy);}if let Some(owner)=self.close_owner(){return owner.next_capacity_byte_demand(copy);}if state.tasks.is_empty()&&state.result.is_none(){if let Some(source)=state.source_lease.as_ref(){return Ok(source.next_close_demand(copy)?.capacity_bytes)}}Ok(if state.tasks.is_empty()&&state.result.is_none(){if let Some(factory)=state.factory_source.as_ref(){factory.factory_retirement_birth_bytes()}else{state.root_retirement.as_ref().map_or(0,|factory|state.source.as_ref().map_or(0,|source|factory.retirement_birth_bytes(source)))}}else{0})}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {let state=&*self.owned;if !state.retirement.terminal_is_empty(){return state.retirement.next_release_byte_demand()}if let Some(task)=state.tasks.front(){return Ok(size_of_val(task.as_ref())+task_link_bytes()+task.retire_release())}if state.result.is_none(){if let Some(source)=state.source_lease.as_ref(){return Ok(source.next_close_demand(0)?.release_bytes)}}self.close_owner().map_or(Ok(0),|owner|if owner.terminal_is_empty(){Ok(size_of_val(owner))}else{owner.next_release_byte_demand()})}
    fn next_depth_demand(&self)->Result<usize,ValueError> {let state=&*self.owned;if !state.retirement.terminal_is_empty(){return state.retirement.next_depth_demand()}if state.tasks.is_empty()&&state.result.is_none(){if let Some(source)=state.source_lease.as_ref(){return Ok(source.next_close_demand(0)?.depth)}}self.close_owner().map_or(Ok(state.factory_source.as_ref().map_or(usize::from(!self.terminal_is_empty()),|factory|factory.factory_retirement_depth_demand())),|owner|owner.next_depth_demand())}
    fn next_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        let state=&*self.owned;
        if !state.retirement.terminal_is_empty(){return state.retirement.selected_demand(body)}
        if let Some(task)=state.tasks.front(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of_val(task.as_ref())+task_link_bytes()+task.retire_release(),depth:1,..Default::default()})}
        if state.result.is_some(){return Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
        if let Some(source)=state.source_lease.as_ref(){return source.next_close_demand(body)}
        if let Some(owner)=self.close_owner(){if owner.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of_val(owner),depth:1,..Default::default()})}return owner.next_demand(body)}
        Ok(semio_framework_value::RetirementDemand{copy_bytes:self.next_copy_byte_demand()?,capacity_bytes:self.next_capacity_byte_demand(body)?,release_bytes:self.next_release_byte_demand()?,depth:self.next_depth_demand()?})
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if !self.owned.closing||grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let d=self.next_close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<d.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"Flow copy close requires admitted depth"));}
        if grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
        let state=&mut*self.owned;
        if !state.retirement.terminal_is_empty(){let step=state.retirement.step(grant)?;return account(&mut state.stalled_steps,grant,step,state.retirement.terminal_is_empty(),"selected Flow copy frontier");}
        if let Some(task)=state.tasks.pop_front(){let bytes=size_of_val(task.as_ref())+task_link_bytes()+task.retire_release();task.retire(&mut state.retirement);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..empty}));}
        else if let Some(result)=state.result.take(){result.retire(&mut state.retirement);}
        else if let Some(source)=state.source_lease.as_mut(){let step=source.close_step(grant)?;if source.terminal_is_empty(){state.source_lease=None}return Ok(RetainedCloneStep::Progress(step.progress()));}
        else if let Some(active)=state.active_root_retirement.as_mut(){
            if active.terminal_is_empty(){let bytes=size_of_val(active.as_ref());state.active_root_retirement=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));}
            let step=active.close_step(grant)?;let terminal=active.terminal_is_empty();
            let step=account(&mut state.stalled_steps,grant,step,terminal,"selected Flow root retirement")?;
            if matches!(step,RetainedCloneStep::Complete(_)){return Ok(RetainedCloneStep::Progress(step.progress()));}
            return Ok(step);
        }
        else if let Some(root)=state.source.take(){match state.root_retirement.as_ref().expect("selected copy retirement factory").retire(root,grant){Ok((owner,progress))=>{state.active_root_retirement=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"Flow root retirement birth")?;return Ok(RetainedCloneStep::Progress(progress));},Err((error,root))=>{state.source=Some(root);return Err(error);}}}
        else if let Some(active)=state.factory_close.as_mut(){
            if active.terminal_is_empty(){let bytes=size_of_val(active.as_ref());state.factory_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));}
            let step=active.close_step(grant)?;let terminal=active.terminal_is_empty();let step=account(&mut state.stalled_steps,grant,step,terminal,"selected Flow factory retirement")?;return Ok(if matches!(step,RetainedCloneStep::Complete(_)){RetainedCloneStep::Progress(step.progress())}else{step});
        }
        else if let Some(factory)=state.root_retirement.take(){state.factory_source=Some(factory);}
        else if let Some(factory)=state.factory_source.take(){match factory.preborn_factory_retirement(grant){Ok((owner,progress))=>{state.factory_close=Some(owner);semio_framework_value::retained_clone::admit_retained_clone_progress(grant,progress,"Flow factory retirement birth")?;return Ok(RetainedCloneStep::Progress(progress));},Err(rejected)=>{state.factory_source=rejected.original;state.factory_close=rejected.ticket;return Err(rejected.error.with_retained_progress(rejected.progress));}}}
        else{return Ok(RetainedCloneStep::Complete(empty));}
        state.stalled_steps=0;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}))
    }

}

impl<R: RetireOwned + Send + Sync + 'static, T: Copy> Drop for CopyCursor<R, T> {
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
        pub struct $name<R: RetireOwned + Send + Sync + 'static> { cursor: CopyCursor<R, $value> }
        impl<R: RetireOwned + Send + Sync + 'static> $name<R> {
            pub fn new(root: Arc<R>, index: usize, project: for<'a> fn(&'a R, usize) -> Option<&'a $value>, retirement: Arc<dyn SnapshotRetirementFactory<R>>,revision:[u8;32]) -> Self {
                Self { cursor: CopyCursor::new(root, index, project, retirement,revision) }
            }
            pub fn allocations(&self) -> &FlowCopyAllocations { &self.cursor.owned.allocation }
            pub fn next_normal_demands(&self,maximum_bytes:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.cursor.next_normal_demands(maximum_bytes)}
            pub fn advance(&mut self,revision:[u8;32],cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<usize>,semio_framework_job::OriginalTurnFailure>{
                if cx.is_cancelled()||cx.should_yield(){return Ok(None)}
                let grant=cx.retained_grant();let d=self.cursor.next_normal_demands(grant.maximum_copy_bytes)?;
                if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(None)}
                let origin=(cx.operation().0,cx.generation().0);
                if revision!=self.cursor.owned.revision||self.cursor.owned.origin.is_some_and(|original|original!=origin)||self.cursor.owned.source_generation.is_some_and(|generation|self.cursor.owned.source_lease.as_ref().and_then(|source|source.original_authority()).is_none_or(|read|!read.commit_authority_matches(generation,revision))){cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Flow copy original revision or operation changed").into())}
                self.cursor.owned.origin=Some(origin);
                let result=self.cursor.advance(RetainedCloneGrant{maximum_items:1,..grant});cx.consume_fuel(1);
                match result{Ok(result)=>{cx.consume_retained(self.cursor.owned.progress)?;Ok(result)},Err(error)=>Err(semio_framework_job::OriginalTurnFailure::debit_original(error,cx))}
            }
            pub fn normal_step_progress(&self)->RetainedCloneProgress{self.cursor.owned.progress}
            pub fn complete(&self) -> bool { self.cursor.complete() }
            pub fn take(&mut self) -> Option<$value> { self.cursor.take() }
            pub fn begin_close(&mut self) { self.cursor.begin_close(); }
            pub fn next_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.cursor.next_close_demands(body)}
            pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {self.cursor.close_step(grant)}
            pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.cursor.next_copy_byte_demand()}
            pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.cursor.next_capacity_byte_demand(copy)}
            pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.cursor.next_release_byte_demand()}
            pub fn next_depth_demand(&self)->Result<usize,ValueError> {self.cursor.next_depth_demand()}
            pub fn terminal_is_empty(&self) -> bool { self.cursor.terminal_is_empty() }
        }
    };
}
selected_cursor!(FlowWidgetCopy, Widget);
selected_cursor!(FlowSynapseCopy, SynapseSpec);
selected_cursor!(FlowHostSnapshotCopy, FlowHostSnapshot);
/// 🧳️ Failed intake preserves either the unadmitted original read or its admitted copy owner.
pub enum FlowSnapshotCopyIntakeSource<R:RetireOwned+Sync>{Read(SnapshotRead<R>),Copy(FlowHostSnapshotCopy<R>)}
impl<R:RetireOwned+Sync> FlowHostSnapshotCopy<R>{
    /// 📥️ Admits the exact original typed Store read and stable projection before any snapshot task is created.
    pub fn from_read(read:SnapshotRead<R>,project:for<'source>fn(&'source R)->&'source FlowHostSnapshot,revision:[u8;32],source_generation:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<Self,(semio_framework_job::OriginalTurnFailure,FlowSnapshotCopyIntakeSource<R>)>{
        let grant=cx.retained_grant();let demand=RetainedCloneSource::<FlowHostSnapshot,SnapshotRead<R>>::borrowed_constructor_demand::<SnapshotRead<R>>();
        if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_depth<demand.depth{return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"Flow snapshot read intake awaits original admission").into(),FlowSnapshotCopyIntakeSource::Read(read)))}
        if !read.commit_authority_matches(source_generation,revision){cx.consume_fuel(1);return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"Flow snapshot read original revision changed").into(),FlowSnapshotCopyIntakeSource::Read(read)))}
        cx.consume_fuel(1);
        let(source,progress)=RetainedCloneSource::<FlowHostSnapshot,SnapshotRead<R>>::admit_borrowed(read,|read|project(read.get()),RetainedCloneGrant{maximum_items:1,..grant}).map_err(|(error,read)|(semio_framework_job::OriginalTurnFailure::debit_original(error,cx),FlowSnapshotCopyIntakeSource::Read(read)))?;
        let cursor=CopyCursor{owned:ManuallyDrop::new(CopyState{tasks:LinkedList::new(),result:None,retirement:Retirement::default(),active_root_retirement:None,factory_close:None,source:None,source_lease:Some(source),source_generation:Some(source_generation),root_retirement:None,factory_source:None,allocation:FlowCopyAllocations::new(),progress:Default::default(),origin:Some((cx.operation().0,cx.generation().0)),revision,project:|_,_|None,index:0,started:false,finished:false,failed:false,closing:false,stalled_steps:0})};let copy=Self{cursor};match cx.consume_retained(progress){Ok(())=>Ok(copy),Err(error)=>Err((error.into(),FlowSnapshotCopyIntakeSource::Copy(copy)))}
    }
}
/// 🏭️ The concrete Flow copy producer retains the original Store projection function.
pub struct FlowSnapshotIntakeProducer<R:RetireOwned+Sync>{pub project:for<'source>fn(&'source R)->&'source FlowHostSnapshot}
impl<R:RetireOwned+Sync+'static> semio_framework_job::OriginalSnapshotIntakeProducer<FlowHostSnapshot,SnapshotRead<R>> for FlowSnapshotIntakeProducer<R>{
 type Intake=FlowHostSnapshotCopy<R>;
 type Rejected=FlowSnapshotCopyIntakeSource<R>;
 fn source_demands(&self)->Result<semio_framework_value::RetirementDemand,ValueError>{let demand=RetainedCloneSource::<FlowHostSnapshot,SnapshotRead<R>>::borrowed_constructor_demand::<SnapshotRead<R>>();Ok(semio_framework_value::RetirementDemand{capacity_bytes:demand.capacity_bytes,depth:demand.depth,..Default::default()})}
 fn preserve_source(&self,source:SnapshotRead<R>)->Self::Rejected{FlowSnapshotCopyIntakeSource::Read(source)}
 fn admit(&self,source:SnapshotRead<R>,revision:[u8;32],source_generation:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<Self::Intake,(semio_framework_job::OriginalTurnFailure,Self::Rejected)>{FlowHostSnapshotCopy::from_read(source,self.project,revision,source_generation,cx)}
}
impl<R:RetireOwned+Sync> semio_framework_job::OriginalSnapshotIntake<FlowHostSnapshot> for FlowHostSnapshotCopy<R>{
    fn next_normal_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.next_normal_demands(body)}
    fn advance(&mut self,revision:[u8;32],cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<usize>,semio_framework_job::OriginalTurnFailure>{self.advance(revision,cx)}
    fn complete(&self)->bool{self.complete()}
    fn take(&mut self)->Option<FlowHostSnapshot>{self.take()}
    fn begin_close(&mut self){self.begin_close()}
    fn next_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if self.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{release_bytes:size_of::<Self>(),depth:1,..Default::default()})}else{self.next_close_demands(body)}}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.close_step(grant)}
    fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}
}
//#endregion 🗿️SelectedCopyCursor

//#region 🧪️CopyLaws
#[cfg(test)]
#[path = "🧪️tests/📑️copy/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../🧪️tests/🦀️.rs"]
mod retained_laws;
//#endregion 🧪️CopyLaws
