//! 📔️ Exact shared registry ownership and incremental retirement.
#![feature(allocator_api)]

use std::collections::HashSet;
use protocol::causal::transition::HistoryFoldIndex;
use std::hash::Hash;
use semio_framework_value::ValueType;
use semio_framework_os_kernel_neural_engine_model::{original_retirement_cursor, OperatorRetirement, Atom, Cardinality, ChannelSpec, ColdDictionaryBuilder, ColdOwner, ColdRetire, Dictionary, EvalError, FieldSpec, NeuralEngineError, Operator, OperatorInfo, OperatorPlanAdmission, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, SCHEMA_KEY, Schema, SchemaRef, VALUE_TYPE_LIST, VALUE_TYPE_NUMBER, Value, ValueRetirement, derive_channel_names, operator_signature, validate_neuron_inputs, validate_operator_outputs};

#[path = "✅️output/🦀️.rs"]
mod output;
pub use output::OperatorFinishCursor;

use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement,RetirementStep};
use semio_framework_value::retained_clone::admit_retained_clone_close;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};
use std::alloc::{AllocError,Allocator,Global,Layout};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize,Ordering};
//#region 🔗️SharedOwnership
struct Shell {address:usize,layout:Layout}
struct FinalRootSlot {registry:OnceLock<ManuallyDrop<Registry>>,shell:OnceLock<Shell>,registry_bytes:AtomicUsize}
#[derive(Default)]
struct ShellAllocator(AtomicUsize);
impl Clone for ShellAllocator {fn clone(&self)->Self {Self(AtomicUsize::new(self.0.load(Ordering::Acquire)))}}
unsafe impl Allocator for ShellAllocator {
    fn allocate(&self,layout:Layout)->Result<NonNull<[u8]>,AllocError> {let allocation=Global.allocate(layout)?;self.0.store(layout.size(),Ordering::Release);Ok(allocation)}
    unsafe fn deallocate(&self,pointer:NonNull<u8>,layout:Layout) {unsafe {Global.deallocate(pointer,layout)};}
}
type FinalRoot=Arc<FinalRootSlot,ShellAllocator>;
/// 🧷️ The calling SharedRegistry reader keeps this original final-root slot alive through its allocation handoff.
#[derive(Clone,Copy)]
struct RegistryAllocator {final_root:usize}
unsafe impl Allocator for RegistryAllocator {
    fn allocate(&self,layout:Layout)->Result<NonNull<[u8]>,AllocError> {let allocation=Global.allocate(layout)?;let root=unsafe {&*(self.final_root as*const FinalRootSlot)};root.registry_bytes.store(layout.size(),Ordering::Release);Ok(allocation)}
    unsafe fn deallocate(&self,pointer:NonNull<u8>,layout:Layout) {let root=unsafe {&*(self.final_root as *const FinalRootSlot)};assert!(root.shell.set(Shell {address:pointer.as_ptr()as usize,layout}).is_ok(),"one original registry allocation shell");}
}

/// 🪪️ Identifies one original registry root while its exact source readers remain alive.
#[derive(Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,Hash)]
pub struct RegistryIdentity(usize);
semio_framework_value::artifact_retire_leaf!(RegistryIdentity);

/// 🔗️ Every registry reader participates in the exact last-reader handoff; raw Arc roots never escape.
#[derive(Clone)]
pub struct SharedRegistry { root: Option<Arc<Registry,RegistryAllocator>>, final_root: FinalRoot }
impl SharedRegistry {
    pub fn into_retirement(self)->RegistryLeaseRetirement {RegistryLeaseRetirement {source:Some(self)}}
    /// 🪪️ Borrows execution identity from this original source lease without allocating or hashing metadata.
    pub fn owner_identity(&self)->RegistryIdentity {RegistryIdentity(Arc::as_ptr(self.root.as_ref().expect("open registry reader")) as usize)}
    /// 🎟️ Creates readers and their unique retirement authority before any shared alias can escape.
    pub fn new(registry: Registry) -> (Self, RegistryRetirement) {
        let final_root=Arc::new_in(FinalRootSlot {registry:OnceLock::new(),shell:OnceLock::new(),registry_bytes:AtomicUsize::new(0)},ShellAllocator::default());
        let retirement=RegistryRetirement {final_root:ManuallyDrop::new(Some(Arc::clone(&final_root))),registry:None};
        let root=Arc::new_in(registry,RegistryAllocator {final_root:Arc::as_ptr(&final_root)as usize});
        (Self { root: Some(root), final_root }, retirement)
    }
}
impl Deref for SharedRegistry { type Target = Registry; fn deref(&self) -> &Registry { self.root.as_deref().expect("open registry reader") } }
impl AsRef<Registry> for SharedRegistry { fn as_ref(&self) -> &Registry { self } }
impl Drop for SharedRegistry {
    fn drop(&mut self) {
        if let Some(root) = self.root.take().and_then(Arc::into_inner) {
            assert!(self.final_root.registry.set(ManuallyDrop::new(root)).is_ok(), "registry final root is handed off exactly once");
        }
    }
}
//#endregion 🔗️SharedOwnership
//#region 🧹️RegistryRetirement
/// 🧹️ Preserves the original registry payload and both allocator-recorded shells until funded closure.
#[must_use="registry retirement must reach terminal-empty"]
pub struct RegistryRetirement {final_root:ManuallyDrop<Option<FinalRoot>>,registry:Option<ControlledRetirement<Registry>>}
impl RegistryRetirement {
    pub fn terminal_is_empty(&self)->bool {self.final_root.is_none()&&self.registry.is_none()}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(0)}else{self.registry.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand)}}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(0)}else{self.registry.as_ref().map_or(Ok(0),|value|value.next_capacity_byte_demand(copy))}}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        if let Some(root)=self.final_root.as_ref() {
            if Arc::strong_count(root)!=1 {return Ok(0);}
            if let Some(shell)=root.shell.get(){return Ok(shell.layout.size());}
            if root.registry.get().is_some(){return Ok(0);}
            return Ok(Arc::allocator(root).0.load(Ordering::Acquire));
        }
        self.registry.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)
    }
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {if self.final_root.is_some(){Ok(1)}else{self.registry.as_ref().map_or(Ok(0),ControlledRetirement::next_depth_demand)}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"registry retained ownership exceeds admitted depth"));}
        if let Some(root)=self.final_root.as_mut() {
            let Some(slot)=Arc::get_mut(root) else {return Ok(RetainedCloneStep::Progress(empty));};
            if let Some(shell)=slot.shell.get() {
                if shell.layout.size()>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
                let shell=slot.shell.take().unwrap();
                unsafe {Global.deallocate(NonNull::new_unchecked(shell.address as *mut u8),shell.layout)};
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:shell.layout.size(),..empty}));
            }
            if let Some(value)=slot.registry.take() {
                self.registry=Some(ControlledRetirement::new(ManuallyDrop::into_inner(value)).unwrap_or_else(|(error,_)|panic!("original registry typed ownership refused: {error}")));
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
            }
            if self.registry.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"registry final-root payload is missing"));}
            let bytes=Arc::allocator(root).0.load(Ordering::Acquire);
            if bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            drop(self.final_root.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
        }
        let value=self.registry.as_mut().unwrap();
        let step=value.step(grant)?;
        let progress=admit_retained_clone_close(grant,step,value.terminal_is_empty(),"original registry fields")?.progress();
        if value.terminal_is_empty(){self.registry=None;}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
impl Drop for RegistryRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"registry must finish explicit retirement before drop");if self.terminal_is_empty(){unsafe {ManuallyDrop::drop(&mut self.final_root);}}}}

/// 🔗️ A reader transfers only its actual lease; the original external authority keeps every final shell.
#[must_use="registry lease must reach terminal-empty"]
pub struct RegistryLeaseRetirement {source:Option<SharedRegistry>}
impl RegistryLeaseRetirement {
    fn demands(&self,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:usize::from(self.source.is_some()),..Default::default()})}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    pub fn next_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError> {Ok(0)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(usize::from(self.source.is_some()))}
    pub fn terminal_is_empty(&self)->bool {self.source.is_none()}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let empty=RetainedCloneProgress::default();
        let Some(source)=self.source.as_ref() else {return Ok(RetainedCloneStep::Complete(empty));};
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"registry reader handoff requires admitted depth"));}
        if Arc::strong_count(&source.final_root)<2{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"registry reader has no retained external shell authority"));}
        drop(self.source.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..empty}))
    }
}
impl Drop for RegistryLeaseRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"registry source lease abandoned before explicit handoff");}}
impl RetireOwned for SharedRegistry {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(self.into_retirement())}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<RegistryLeaseRetirement>())}
    fn controlled_retirement_supported()->bool {true}
}

original_retirement_cursor!(RegistryLeaseRetirement);
//#endregion 🧹️RegistryRetirement

#[cfg(test)]
#[path = "🧪️tests/📔️registry/🦀️.rs"]
pub(crate) mod tests;

#[path="🏗️admission/🦀️.rs"]
mod admission;
pub use admission::{SharedRegistryIssuer,SharedRegistryAdmissionStep};

// #region 🔖️SchemaComponent
fn schema_component_operator_id(schema: &Schema) -> String {
    format!("{}.{}", schema.module, schema.id)
}

fn should_auto_register_schema_component(schema: &Schema) -> bool {
    schema.module != "core" && !schema.fields.is_empty() && schema.id != "list" && schema.id != "dictionary"
}

fn schema_field_input_cardinality(value: &ValueType) -> Cardinality {
    match value {
        ValueType::List(_) => Cardinality::ZeroOrMore,
        _ => Cardinality::ZeroOrOne,
    }
}

fn schema_field_output_cardinality(value: &ValueType) -> Cardinality {
    match value {
        ValueType::List(_) => Cardinality::ZeroOrMore,
        _ => Cardinality::ExactlyOne,
    }
}

/// 🔤️ The value schemas a declared schema FIELD carries, in `ChannelSpec::value_types` vocabulary.
/// `Integer` and `Decimal` collapse to one `number` port type — whether a number is whole is an
/// input-validation concern, never a wiring one — and `Any` declares nothing, which stays connectable.
fn field_channel_value_types(value: &ValueType) -> Vec<String> {
    match value {
        ValueType::Any => Vec::new(),
        ValueType::List(_) => vec![VALUE_TYPE_LIST.into()],
        ValueType::Integer | ValueType::Decimal => vec![VALUE_TYPE_NUMBER.into()],
        _ => vec![value.id()],
    }
}

fn field_channel_operators(value: &ValueType) -> Vec<String> {
    match value {
        ValueType::List(inner) => vec![inner.id()],
        _ => vec![value.id()],
    }
}

/// 📃️ Declares element schemas for schema-derived homogeneous list channels.
fn field_channel_item_types(value: &ValueType) -> Vec<String> {
    match value { ValueType::List(inner) => field_channel_value_types(inner), _ => Vec::new() }
}

/// 🏷️ The id an OUTPUT channel takes when it carries the same noun as one of its operator's own
/// INPUTS. An operator's input ids and output ids are disjoint, because `"{nodeId}@{portId}"` is the
/// one public name a wire endpoint has: a graph, a journalled edit and a pointer press all address a
/// port by that string, so a port id that names a channel on both sides names two handles and the
/// press resolves to whichever the lookup happened to reach first. The display names (`code`,
/// `abbreviation`, `fullName`) are untouched — this is an identity, not a label.
///
/// @see `✏️s/🧑‍💻dev/🌊️flow/🧫️fixtures/🔌️port-sides/🔣️.json` — the catalogue-wide law
pub fn produced_channel_id(input_id: &str) -> String {
    format!("{input_id}Out")
}

/// 🧩️ Builds construct/deconstruct/modify operator metadata for a schema.
pub fn schema_component_info(schema: &Schema) -> OperatorInfo {
    let operator_id = schema_component_operator_id(schema);
    let mut inputs = vec![ChannelSpec::requires(&schema.id, &[schema.id.as_str()]).with_value_types(&[schema.id.as_str()]).with_cardinality(Cardinality::ZeroOrOne)];
    for field in &schema.fields {
        let operators = field_channel_operators(&field.value);
        inputs.push(ChannelSpec::requires(&field.key, &operators).with_value_types(&field_channel_value_types(&field.value)).with_item_types(&field_channel_item_types(&field.value)).with_cardinality(schema_field_input_cardinality(&field.value)));
    }
    let (instance_code, instance_abbreviation, instance_full_name) = derive_channel_names(&schema.id);
    let mut outputs = vec![ChannelSpec::named(instance_code, instance_abbreviation, produced_channel_id(&schema.id), instance_full_name).with_operators(vec![schema.id.clone()]).with_value_types(&[schema.id.as_str()])];
    for field in &schema.fields {
        let (code, abbreviation, full_name) = derive_channel_names(&field.key);
        outputs.push(ChannelSpec::named(code, abbreviation, produced_channel_id(&field.key), full_name).with_operators(field_channel_operators(&field.value)).with_value_types(&field_channel_value_types(&field.value)).with_item_types(&field_channel_item_types(&field.value)).with_cardinality(schema_field_output_cardinality(&field.value)));
    }
    outputs.push(ChannelSpec::list_output("errors", vec![]));
    OperatorInfo {
        id: operator_id,
        extension: schema.module.clone(),
        name: schema.name.clone(),
        abbreviation: schema.name.clone(),
        icon: schema.icon.clone(),
        summary: format!("Constructs, deconstructs, or modifies {}", schema.name),
        inputs,
        outputs,
        group: vec!["Schemas".into()],
        ..Default::default()
    }
}

fn schema_errors_list(messages: &[String]) -> Dictionary {
    let mut list = Dictionary::with_schema("list");
    for (index, message) in messages.iter().enumerate() {
        list = list.insert(index.to_string(), Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(message.clone())))));
    }
    list
}

fn schema_input_present(input: &Dictionary, key: &str) -> bool {
    input.get(key).is_some_and(|value| !value.is_null())
}

fn field_to_channel(value: &Value, value_type: &ValueType) -> Result<Value, NeuralEngineError> {
    match value_type {
        ValueType::Decimal => {
            let number = value.as_atom().and_then(|atom| atom.as_f64()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "decimal", reason: "numeric" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(number)))))
        }
        ValueType::Integer => {
            let number = value
                .as_atom()
                .and_then(|atom| match atom {
                    Atom::Integer(value) => Some(*value),
                    Atom::Decimal(value) => Some(value.round() as i64),
                    _ => None,
                })
                .ok_or(NeuralEngineError::FieldTypeMismatch { kind: "integer", reason: "integral" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Integer(number)))))
        }
        ValueType::Boolean => {
            let boolean = value.as_atom().and_then(|atom| atom.as_bool()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "boolean", reason: "boolean" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("boolean").insert("value", Value::Atom(Atom::Boolean(boolean)))))
        }
        ValueType::Text => {
            let text = value.as_atom().and_then(|atom| atom.as_str()).ok_or(NeuralEngineError::FieldTypeMismatch { kind: "text", reason: "text" })?;
            Ok(Value::Dictionary(Dictionary::with_schema("text").insert("value", Value::Atom(Atom::String(text.to_string())))))
        }
        ValueType::List(_) | ValueType::Schema(_) | ValueType::Any => value.as_dictionary().cloned().map(Value::Dictionary).ok_or(NeuralEngineError::FieldNotDictionary),
    }
}

fn channel_to_field(value: &Value, value_type: &ValueType) -> Result<Value, NeuralEngineError> {
    if value.is_null() {
        return Err(NeuralEngineError::ChannelNull);
    }
    match value_type {
        ValueType::Decimal => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "decimal" })?;
            let number = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_f64()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "decimal" })?;
            Ok(Value::Atom(Atom::Decimal(number)))
        }
        ValueType::Integer => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "integer" })?;
            let number = dictionary
                .get("value")
                .and_then(|entry| entry.as_atom())
                .and_then(|atom| match atom {
                    Atom::Integer(value) => Some(*value),
                    Atom::Decimal(value) => Some(value.round() as i64),
                    _ => None,
                })
                .ok_or(NeuralEngineError::ChannelMissingValue { kind: "integer" })?;
            Ok(Value::Atom(Atom::Integer(number)))
        }
        ValueType::Boolean => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "boolean" })?;
            let boolean = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_bool()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "boolean" })?;
            Ok(Value::Atom(Atom::Boolean(boolean)))
        }
        ValueType::Text => {
            let dictionary = value.as_dictionary().ok_or(NeuralEngineError::ChannelTypeMismatch { kind: "text" })?;
            let text = dictionary.get("value").and_then(|entry| entry.as_atom()).and_then(|atom| atom.as_str()).ok_or(NeuralEngineError::ChannelMissingValue { kind: "text" })?;
            Ok(Value::Atom(Atom::String(text.to_string())))
        }
        ValueType::List(_) | ValueType::Schema(_) | ValueType::Any => value.as_dictionary().cloned().map(Value::Dictionary).ok_or(NeuralEngineError::ChannelNotDictionary),
    }
}

fn read_schema_instance<'a>(input: &'a Dictionary, schema: &Schema) -> Result<&'a Dictionary, NeuralEngineError> {
    let instance = input.get(&schema.id).and_then(|value| value.as_dictionary()).ok_or_else(|| NeuralEngineError::Missing(schema.id.clone()))?;
    if instance.schema() != Some(schema.id.as_str()) {
        return Err(NeuralEngineError::Invalid(schema.id.clone()));
    }
    Ok(instance)
}

fn read_schema_field_input(input: &Dictionary, field: &FieldSpec) -> Result<Value, NeuralEngineError> {
    let value = input.get(&field.key).ok_or_else(|| NeuralEngineError::Missing(field.key.clone()))?;
    channel_to_field(value, &field.value)
}

/// 🧩️ Construct, deconstruct, or modify dictionaries for one schema.
pub struct SchemaComponent {
    pub schema: Schema,
}

impl SchemaComponent {
    fn construct(&self, input: &Dictionary, provided: &[&FieldSpec]) -> Result<Dictionary, NeuralEngineError> {
        let mut dictionary = ColdDictionaryBuilder::from_dictionary(self.schema.default_dictionary());
        for field in provided {
            dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        for field in &self.schema.fields {
            if dictionary.dictionary().get(&field.key).is_none() {
                return Err(NeuralEngineError::MissingField(field.key.clone()));
            }
        }
        self.schema.validate(dictionary.dictionary())?;
        Ok(dictionary.finish())
    }

    fn deconstruct(&self, input: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let instance = read_schema_instance(input, &self.schema)?;
        self.schema.validate(instance)?;
        Ok(instance.clone())
    }

    fn modify(&self, input: &Dictionary, provided: &[&FieldSpec]) -> Result<Dictionary, NeuralEngineError> {
        let mut dictionary = ColdDictionaryBuilder::from_dictionary(read_schema_instance(input, &self.schema)?.clone());
        for field in provided {
            dictionary.insert(field.key.clone(), read_schema_field_input(input, field)?);
        }
        self.schema.validate(dictionary.dictionary())?;
        Ok(dictionary.finish())
    }

    fn success_output(&self, instance: &Dictionary) -> Result<Dictionary, NeuralEngineError> {
        let mut output = ColdDictionaryBuilder::from_dictionary(Dictionary::new().insert(produced_channel_id(&self.schema.id), Value::Dictionary(instance.clone())));
        for field in &self.schema.fields {
            // 🛡️ `instance` only reaches here after `Schema::validate` confirmed every
            // declared field.key is present, so this lookup can never miss.
            let value = instance.get(&field.key).expect("validated field");
            let channel = field_to_channel(value, &field.value)?;
            output.insert(produced_channel_id(&field.key), channel);
        }
        output.insert("errors".into(), Value::Dictionary(schema_errors_list(&[])));
        Ok(output.finish())
    }

    fn error_output(&self, messages: &[String]) -> Dictionary {
        let mut output = Dictionary::new().insert(produced_channel_id(&self.schema.id), Value::null()).insert("errors", Value::Dictionary(schema_errors_list(messages)));
        for field in &self.schema.fields {
            output = output.insert(produced_channel_id(&field.key), Value::null());
        }
        output
    }
}

impl Operator for SchemaComponent {
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)>{OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_copy:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(1)}

        fn retire_cold(self: Box<Self>) { self.schema.retire_cold(); }

    fn retirement_is_empty(&self) -> bool {
        self.schema.id.capacity()==0 && self.schema.module.capacity()==0 && self.schema.name.capacity()==0 && self.schema.icon.capacity()==0 && self.schema.summary.capacity()==0 && self.schema.fields.capacity()==0
    }

    fn next_retire_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    fn next_retire_capacity_byte_demand(&self,_copy:usize)->Result<usize,ValueError> {Ok(0)}
    fn next_retire_release_byte_demand(&self)->Result<usize,ValueError> {Ok(0)}
    fn next_retire_depth_demand(&self)->Result<usize,ValueError> {Ok(usize::from(!self.retirement_is_empty()))}
    fn retire_step(&mut self,grant:RetainedCloneGrant,values:&mut ValueRetirement)->Result<RetainedCloneStep,ValueError> {
        if self.retirement_is_empty() {return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));}
        if grant.maximum_items==0 {return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));}
        if grant.maximum_depth==0 {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"Schema component handoff requires admitted depth"));}
        let schema=std::mem::take(&mut self.schema);
        match values.push_schema(schema,grant) {Ok(progress)=>Ok(RetainedCloneStep::Complete(progress)),Err((error,schema))=>{self.schema=schema;Err(error)}}
    }

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let has_instance = schema_input_present(input, &self.schema.id);
        let provided: Vec<&FieldSpec> = self.schema.fields.iter().filter(|field| schema_input_present(input, &field.key)).collect();
        let has_fields = !provided.is_empty();
        let result = match (has_instance, has_fields) {
            (false, false) => Err(NeuralEngineError::NoInputProvided),
            (false, true) => self.construct(input, &provided),
            (true, false) => self.deconstruct(input),
            (true, true) => self.modify(input, &provided),
        };
        Ok(match result.and_then(|instance| self.success_output(&ColdOwner::new(instance))) {
            Ok(output) => output,
            Err(error) => self.error_output(&[error.to_string()]),
        })
    }
}
// #endregion 🔖️SchemaComponent
// #region 🔖️OperatorRecord

pub struct OperatorImpl {
    pub schemas: Vec<String>,
    pub operator: Box<dyn Operator>,
}

pub struct OperatorRecord {
    pub info: OperatorInfo,
    pub implementations: Vec<OperatorImpl>,
}

/// 🧭️ Retains only scalar positions into the original registry and input owners.
#[derive(Clone,Copy,Default)]
pub struct OperatorPlanCursor {slot:usize,record:Option<usize>,implementation:usize,fallback:Option<usize>,channel:usize,signature:usize,rank:usize,offset:usize,value_rank:Option<usize>,schema_rank:Option<usize>,use_default:bool,phase:u8,progress:RetainedCloneProgress}
semio_framework_value::artifact_retire_leaf!(OperatorPlanCursor);
pub enum OperatorPlanStep {Working,Admitted(OperatorPlanAdmission)}
impl OperatorPlanCursor {
    pub fn new()->Self{Self::default()}
    pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
    fn value<'a>(&self,record:&'a OperatorRecord,input:&'a Dictionary)->Option<&'a Value>{if self.use_default{record.info.inputs.get(self.channel)?.default.as_ref()}else{input.entry_at_rank(self.value_rank?).map(|(_,value)|value)}}
    fn schema<'a>(&self,record:&'a OperatorRecord,input:&'a Dictionary)->&'a str{self.value(record,input).and_then(Value::as_dictionary).and_then(|dict|dict.entry_at_rank(self.schema_rank?)).and_then(|(_,value)|value.as_atom()).and_then(Atom::as_str).unwrap_or("")}
    fn compare(&mut self,left:&str,right:&str,grant:RetainedCloneGrant)->Option<bool>{let length=left.len().min(right.len());let count=(length-self.offset).min(grant.maximum_copy_bytes);if count==0&&self.offset<length{return None}let same=left.as_bytes()[self.offset..self.offset+count]==right.as_bytes()[self.offset..self.offset+count];self.offset+=count;self.progress=RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()};if !same{self.offset=0;Some(false)}else if self.offset==length{self.offset=0;Some(left.len()==right.len())}else{None}}
}

/// 📋️ Registry of schemas and operators by id.
#[derive(Default)]
pub struct Registry {
    schemas: HistoryFoldIndex<String, Schema>,
    operators: HistoryFoldIndex<String, OperatorRecord>,
    operator_produces: HistoryFoldIndex<String, Vec<String>>,
    schema_providers: HistoryFoldIndex<String, HistoryFoldIndex<String,()>>,
    finalized: bool,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_schema(&mut self, schema: Schema) {
        self.schemas.insert(schema.id.clone(), schema).retire_cold();
        self.finalized = false;
    }

    pub fn register_operator(&mut self, info: OperatorInfo, implementations: Vec<OperatorImpl>, produces: &[&str]) {
        let id = info.id.clone();
        for schema in produces {
            self.schema_providers.get_or_insert(schema.to_string(),HistoryFoldIndex::default());
        }
        for implementation in &implementations {
            for schema in &implementation.schemas {
                self.schema_providers.get_or_insert(schema.clone(),HistoryFoldIndex::default()).insert(id.clone(),());
            }
        }
        self.operator_produces.insert(id.clone(), produces.iter().map(|entry| (*entry).to_string()).collect());
        self.operators.insert(id, OperatorRecord { info, implementations }).retire_cold();
        self.finalized = false;
    }

    pub fn finalize(&mut self) {
        if self.finalized {
            return;
        }
        let schema_ids: Vec<String> = self.schemas.keys().cloned().collect();
        for schema_id in schema_ids {
            let Some(schema) = self.schemas.get(&schema_id).cloned().map(ColdOwner::new) else { continue };
            if !should_auto_register_schema_component(&schema) {
                continue;
            }
            let operator_id = schema_component_operator_id(&schema);
            if self.operators.contains_key(&operator_id) {
                continue;
            }
            let info = schema_component_info(&schema);
            let produces = vec![schema.id.clone()];
            for produced in &produces {
                self.schema_providers.get_or_insert(produced.clone(),HistoryFoldIndex::default());
            }
            self.schema_providers.get_or_insert(schema.id.clone(),HistoryFoldIndex::default()).insert(operator_id.clone(),());
            self.operator_produces.insert(operator_id.clone(), produces);
            self.operators.insert(operator_id, OperatorRecord { info, implementations: vec![OperatorImpl { schemas: vec![], operator: Box::new(SchemaComponent { schema: schema.into_inner() }) }] }).retire_cold();
        }
        let operator_produces = self.operator_produces.clone();
        let schema_providers = self.schema_providers.clone();
        for (operator_id, operator) in self.operators.slot_entries_mut() {
            let produces = operator_produces.get(operator_id).cloned().unwrap_or_default();
            for channel in &mut operator.info.outputs {
                if !channel.operators.is_empty() {
                    continue;
                }
                let mut provided = HashSet::new();
                for schema in &produces {
                    if let Some(providers) = schema_providers.get(schema) {
                        provided.extend(providers.keys().cloned());
                    }
                }
                let mut operators: Vec<String> = provided.into_iter().collect();
                operators.sort();
                channel.operators = operators;
            }
        }
        self.finalized = true;
    }

    pub fn operators_for_schema(&self, schema_id: &str) -> Vec<String> {
        let mut operators: Vec<String> = self.schema_providers.get(schema_id).map(|entries| entries.keys().cloned().collect()).unwrap_or_default();
        operators.sort();
        operators
    }

    /// 🔌️ Whether a wire may carry `output` into `input` — the ONE port-compatibility oracle every
    /// node-graph surface answers to. Declared VALUE SCHEMAS decide it: a pair is refused only when
    /// both sides declare and the declared sets are disjoint, so an undeclared channel stays
    /// connectable and no shipped graph is retro-refused.
    ///
    /// @see `🧫️fixtures/🔌️port-types/🔣️.json` — the fixture that owns the compatible/incompatible pairs
    pub fn channel_compatible(output: &ChannelSpec, input: &ChannelSpec) -> bool {
        if output.value_types.is_empty() || input.value_types.is_empty() {
            return true;
        }
        output.value_types.iter().any(|provided| input.value_types.iter().any(|accepted| accepted == provided))
    }

    pub fn schema(&self, schema_id: &str) -> Option<&Schema> {
        self.schemas.get(schema_id)
    }

    pub fn operator(&self, operator_id: &str) -> Option<&OperatorRecord> {
        self.operators.get(operator_id)
    }

    pub fn operator_info(&self, operator_id: &str) -> Option<&OperatorInfo> {
        self.operators.get(operator_id).map(|entry| &entry.info)
    }

    /// 📚️ Borrows current metadata in registry order without acquiring nested default-value owners.
    pub fn operator_infos(&self) -> impl DoubleEndedIterator<Item = &OperatorInfo> + ExactSizeIterator {
        self.operators.values().map(|entry| &entry.info)
    }

    pub fn schema_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.schemas.keys().cloned().collect();
        ids.sort();
        ids
    }

    /// 🏷️ Lightweight schema metadata for pickers and catalogues.
    pub fn schema_refs(&self) -> Vec<SchemaRef> {
        self.schema_ids().into_iter().filter_map(|id| self.schemas.get(&id).map(|schema| SchemaRef { id: schema.id.clone(), name: schema.name.clone(), icon: schema.icon.clone() })).collect()
    }

    pub fn schema_catalogue(&self) -> Vec<Schema> {
        let mut items: Vec<Schema> = self.schemas.values().cloned().collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    pub fn operator_catalogue(&self) -> Vec<OperatorInfo> {
        let mut items: Vec<OperatorInfo> = self.operators.values().map(|entry| Self::finalize_operator_info(&entry.info, self.operator_produces.get(&entry.info.id).map(Vec::as_slice), &self.schema_providers)).collect();
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items
    }

    fn finalize_operator_info(info: &OperatorInfo, produces: Option<&[String]>, schema_providers: &HistoryFoldIndex<String, HistoryFoldIndex<String,()>>) -> OperatorInfo {
        let mut finalized = info.clone();
        let produces = produces.unwrap_or(&[]);
        for channel in &mut finalized.outputs {
            if !channel.operators.is_empty() {
                continue;
            }
            let mut provided = HashSet::new();
            for schema in produces {
                if let Some(providers) = schema_providers.get(schema) {
                    provided.extend(providers.keys().cloned());
                }
            }
            let mut operators: Vec<String> = provided.into_iter().collect();
            operators.sort();
            channel.operators = operators;
        }
        finalized
    }

    pub fn dispatch(&self, operator_id: &str, input: &Dictionary) -> Result<Dictionary, EvalError> {
        let operator = self.operator(operator_id).ok_or_else(|| EvalError::UnknownKind(operator_id.into()))?;
        validate_neuron_inputs(input, Some(&operator.info))?;
        let signature = operator_signature(&operator.info, input);
        let implementation = operator
            .implementations
            .iter()
            .find(|implementation| implementation.schemas == signature)
            .or_else(|| operator.implementations.iter().find(|implementation| implementation.schemas.is_empty()))
            .ok_or_else(|| EvalError::InvalidInput(format!("no implementation for {operator_id}({})", signature.join(", "))))?;
        let output = ColdOwner::new(implementation.operator.evaluate(input)?);
        validate_operator_outputs(&operator.info, &output)?;
        Ok(output.into_inner())
    }

    /// 🧊️ The cold-boundary form of [`Registry::dispatch`]: takes EXACT ownership of `input` and
    /// answers the operator's dictionary already inside its cold boundary, so a batch caller never
    /// becomes the final owner of either dictionary and never has to remember
    /// [`Dictionary`]'s drop law (`final Dictionary ownership must be explicitly retired or owned
    /// by a cold boundary`) by hand. The refusal path retires `input` too, which a caller writing
    /// `registry.dispatch(id, &input)?` cannot do without a second owner.
    ///
    /// This is the same shape the guest path already takes
    /// (`flow_extension_sdk::evaluate_json` wraps both its input and its answer in a
    /// [`ColdOwner`]); it is named here so every batch caller — test harness, export bridge,
    /// catalogue probe — states one cold scope instead of open-coding two.
    // 🚫️async: E1 pure registry dispatch mirroring `dispatch` (no I/O) — see R9
    pub fn dispatch_cold(&self, operator_id: &str, input: Dictionary) -> Result<ColdOwner<Dictionary>, EvalError> {
        let input = ColdOwner::new(input);
        self.dispatch(operator_id, &input).map(ColdOwner::new)
    }

    /// 🪙️ Quotes the next scalar selection or actual concrete plan admission from the same owners.
    pub fn next_dispatch_job_demands(&self,operator_id:&str,input:&Option<Dictionary>,cursor:&OperatorPlanCursor,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        let input=input.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original plan input is absent"))?;
        if cursor.phase==6{let record=self.operators.slot_entry(cursor.record.unwrap()).unwrap().1;let operator=&record.implementations[cursor.implementation].operator;return Ok(semio_framework_value::RetirementDemand{copy_bytes:operator.next_plan_copy_byte_demand(input)?,capacity_bytes:operator.next_plan_capacity_byte_demand(input,copy)?,release_bytes:operator.next_plan_release_byte_demand(input)?,depth:operator.next_plan_depth_demand(input)?})}
        let bytes=match cursor.phase{0=>self.operators.slot_entry(cursor.slot).map_or(0,|(key,_)|usize::from(cursor.offset<key.len().min(operator_id.len()))),2=>cursor.record.and_then(|slot|self.operators.slot_entry(slot)).and_then(|(_,record)|record.info.inputs.get(cursor.channel)).and_then(|channel|input.entry_at_rank(cursor.rank).map(|(key,_)|usize::from(cursor.offset<key.len().min(channel.name.len())))).unwrap_or(0),3=>cursor.record.and_then(|slot|self.operators.slot_entry(slot)).and_then(|(_,record)|cursor.value(record,input)).and_then(Value::as_dictionary).and_then(|dictionary|dictionary.entry_at_rank(cursor.rank)).map_or(0,|(key,_)|usize::from(cursor.offset<key.len().min(SCHEMA_KEY.len()))),4=>cursor.record.and_then(|slot|self.operators.slot_entry(slot)).map_or(0,|(_,record)|usize::from(cursor.offset<cursor.schema(record,input).len().min(record.implementations[cursor.implementation].schemas[cursor.signature].len()))),_=>0};
        Ok(semio_framework_value::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()})
    }
    /// ⏱️ Selects the original implementation without a cold lookup or allocated signature mirror.
    pub fn dispatch_job(&self,operator_id:&str,input:&mut Option<Dictionary>,cursor:&mut OperatorPlanCursor,grant:RetainedCloneGrant)->Result<OperatorPlanStep,EvalError>{
        cursor.progress=Default::default();let result=self.dispatch_job_source(operator_id,input,cursor,grant);result.map_err(|error|match error{EvalError::Retained(error)=>EvalError::Retained(error.with_retained_progress(cursor.progress)),other=>other})
    }
    fn dispatch_job_source(&self,operator_id:&str,input:&mut Option<Dictionary>,cursor:&mut OperatorPlanCursor,grant:RetainedCloneGrant)->Result<OperatorPlanStep,EvalError>{
        use semio_framework_value::ValueRefusalKind;
        if grant.maximum_items==0{return Ok(OperatorPlanStep::Working)}let demand=self.next_dispatch_job_demands(operator_id,input,cursor,grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original plan selection requires admitted depth").into())}if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(OperatorPlanStep::Working)}
        cursor.progress=RetainedCloneProgress{copied_items:1,..Default::default()};
        if cursor.phase==0{if cursor.slot==self.operators.slot_count(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original operator kind is absent").into())}if let Some((key,_))=self.operators.slot_entry(cursor.slot){match cursor.compare(key,operator_id,grant){Some(true)=>{cursor.record=Some(cursor.slot);cursor.phase=1;},Some(false)=>cursor.slot+=1,None=>{}}}else{cursor.slot+=1}return Ok(OperatorPlanStep::Working)}
        let record=self.operators.slot_entry(cursor.record.unwrap()).unwrap().1;let original=input.as_ref().unwrap();
        match cursor.phase {
            1=>{if cursor.implementation==record.implementations.len(){if let Some(index)=cursor.fallback{cursor.implementation=index;cursor.phase=6;}else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original operator implementation is absent").into())}}else if record.implementations[cursor.implementation].schemas.is_empty(){cursor.fallback.get_or_insert(cursor.implementation);cursor.implementation+=1;}else if record.info.variadic_input.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original variadic signature requires its retained contract cursor").into())}else{cursor.channel=0;cursor.signature=0;cursor.phase=5;}},
            2=>{if let Some((key,_))=original.entry_at_rank(cursor.rank){match cursor.compare(key,&record.info.inputs[cursor.channel].name,grant){Some(true)=>{cursor.value_rank=Some(cursor.rank);cursor.use_default=false;cursor.rank=0;cursor.phase=3;},Some(false)=>cursor.rank+=1,None=>{}}}else{cursor.value_rank=None;cursor.use_default=true;cursor.rank=0;cursor.phase=3;}},
            3=>{if let Some(dictionary)=cursor.value(record,original).and_then(Value::as_dictionary){if let Some((key,_))=dictionary.entry_at_rank(cursor.rank){match cursor.compare(key,SCHEMA_KEY,grant){Some(true)=>{cursor.schema_rank=Some(cursor.rank);cursor.phase=4;},Some(false)=>cursor.rank+=1,None=>{}}}else{cursor.schema_rank=None;cursor.phase=4;}}else{cursor.schema_rank=None;cursor.phase=4;}},
            4=>{match cursor.compare(cursor.schema(record,original),&record.implementations[cursor.implementation].schemas[cursor.signature],grant){Some(true)=>{cursor.signature+=1;cursor.channel+=1;cursor.phase=5;},Some(false)=>{cursor.implementation+=1;cursor.phase=1;},None=>{}}},
            5=>{if cursor.channel==record.info.inputs.len(){if cursor.signature==record.implementations[cursor.implementation].schemas.len(){cursor.phase=6;}else{cursor.implementation+=1;cursor.phase=1;}}else if record.info.inputs[cursor.channel].name.len()==1{if grant.maximum_copy_bytes==0{cursor.progress=Default::default();return Ok(OperatorPlanStep::Working)}cursor.progress.copied_bytes=1;if record.info.inputs[cursor.channel].name.as_bytes()[0]==b'*'{cursor.channel+=1;}else if cursor.signature==record.implementations[cursor.implementation].schemas.len(){cursor.implementation+=1;cursor.phase=1;}else{cursor.rank=0;cursor.phase=2;}}else if cursor.signature==record.implementations[cursor.implementation].schemas.len(){cursor.implementation+=1;cursor.phase=1;}else{cursor.rank=0;cursor.phase=2;}},
            6=>{match record.implementations[cursor.implementation].operator.step_plan(input.take().unwrap(),grant){Ok((admission,progress))=>{cursor.progress=progress;cursor.phase=7;return Ok(OperatorPlanStep::Admitted(admission))},Err((error,original))=>{*input=Some(original);cursor.progress=match &error{EvalError::Retained(error)=>error.retained_progress(),_=>Default::default()};return Err(error)}}},
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original plan was already admitted").into()),
        }
        Ok(OperatorPlanStep::Working)
    }

}
// #endregion 🔖️OperatorRecord

//#region 🧵️DomainRetirement
semio_framework_value::artifact_retire_struct!(OperatorImpl {schemas,operator});
semio_framework_value::artifact_retire_struct!(OperatorRecord {info,implementations});
semio_framework_value::artifact_retire_struct!(Registry {schemas,operators,operator_produces,schema_providers,finalized});
//#endregion 🧵️DomainRetirement

//#region 🧬️DomainOwners
impl ColdRetire for OperatorRecord {
    fn retire_cold(self) { self.info.retire_cold(); for implementation in self.implementations { implementation.operator.retire_cold(); } }
}
impl ColdRetire for Registry { fn retire_cold(self) { self.schemas.retire_cold(); self.operators.retire_cold(); } }
//#endregion 🧬️DomainOwners
