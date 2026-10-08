//! 🧹️ Exact nested-value retirement and explicitly synchronous construction owners.

use super::{Atom, ChannelSpec, Dictionary, EvalChannels, FieldSpec, NeuronSnapshot, OperatorInfo, Schema, TreeSnapshot, Value, ValueType};
use protocol::value::ordered::{OrderedMap, Retirement, RetirementStep};
use semio_framework_value::retained_clone::RetainedCloneGrant;
use std::collections::{BTreeMap, LinkedList};
use std::mem::{size_of, ManuallyDrop};
use std::sync::Arc;

//#region 🧵️DomainRetirement
enum Owner {
    Map(Retirement<Value>), Value(Value), Shared(Arc<Value>),
    Owned(Box<dyn semio_framework_value::ErasedSnapshotRetirement>),
    /// 🎟️ A byte buffer plus the payload bytes still to be drawn down before it is freed. A
    /// `Vec<u8>` cannot be freed in pieces, so the grant is charged against `remaining_bytes` one
    /// turn at a time and the whole buffer is released once the charge reaches zero — the
    /// `min(grant, left)` drawdown the language-agnostic contract states
    /// (`🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts`). An all-or-nothing release would answer
    /// `Blocked` to every fixed-page driver in the tree and spin forever
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    Bytes { values: Vec<u8>, remaining_bytes: usize },
    Strings(Vec<String>),
    Dictionaries(BTreeMap<String, Dictionary>), Snapshot(TreeSnapshot), Neurons(BTreeMap<String, NeuronSnapshot>), Seeds(BTreeMap<String, u64>),
    Operator(OperatorInfo), Channels(Vec<ChannelSpec>), Schema(Schema), Fields(Vec<FieldSpec>), Type(ValueType),
}

/// 🎟️ One byte-buffer owner whose drawdown charge starts at its live payload length.
fn byte_owner(values: Vec<u8>) -> Owner {
    let remaining_bytes = values.len();
    Owner::Bytes { values, remaining_bytes }
}
/// 🎟️ Exact released payload bytes and one retained structural ownership operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueRetirementStep { Blocked, Pending { released_items: usize, released_bytes: usize }, Complete }

/// 🔒️ Owns all nested dictionary and string frontiers until explicit terminal-empty close.
#[must_use = "nested value retirement must reach terminal-empty before drop"]
pub struct ValueRetirement { owners: ManuallyDrop<LinkedList<Owner>> }
impl Default for ValueRetirement { fn default() -> Self { Self { owners: ManuallyDrop::new(LinkedList::new()) } } }
impl ValueRetirement {
    pub fn from_value(value: Value) -> Self { let mut owner = Self::default(); owner.push_value(value); owner }
    pub fn from_dictionary(value: Dictionary) -> Self { let mut owner = Self::default(); owner.push_dictionary(value); owner }
    pub fn push_value(&mut self, value: Value) { self.owners.push_back(Owner::Value(value)); }
    /// 🎒️ Takes one domain-neutral candidate into this same nested-value retirement authority.
    pub fn push_owned<T:semio_framework_value::retirement::RetireOwned>(&mut self,value:T) {self.owners.push_back(Owner::Owned(semio_framework_value::retirement::owned_retirement(value)));}
    pub fn push_shared(&mut self, value: Arc<Value>) { self.owners.push_back(Owner::Shared(value)); }
    pub fn push_dictionary(&mut self, mut dictionary: Dictionary) { self.push_map(std::mem::take(&mut dictionary.pairs)); }
    pub fn text(&mut self, text: String) { self.owners.push_back(byte_owner(text.into_bytes())); }
    pub fn push_dictionaries(&mut self, values: BTreeMap<String, Dictionary>) { self.owners.push_back(Owner::Dictionaries(values)); }
    pub fn push_channels(&mut self, channels: EvalChannels) { self.push_dictionaries(channels.outputs); self.push_dictionaries(channels.inputs); }
    pub fn push_snapshot(&mut self, snapshot: TreeSnapshot) { self.owners.push_back(Owner::Snapshot(snapshot)); }
    pub fn push_operator(&mut self, operator: OperatorInfo) { self.owners.push_back(Owner::Operator(operator)); }
    pub fn push_schema(&mut self, schema: Schema) { self.owners.push_back(Owner::Schema(schema)); }
    pub fn push_strings(&mut self, strings: Vec<String>) { self.owners.push_back(Owner::Strings(strings)); }
    pub fn terminal_is_empty(&self) -> bool { self.owners.is_empty() }
    pub fn allocated_bytes(&self) -> usize {
        self.owners.iter().fold(0usize, |total, owner| total.saturating_add(match owner {
            Owner::Map(values) => values.allocated_bytes(),
            Owner::Bytes { values, .. } => values.capacity(),
            Owner::Strings(values) => values.capacity().saturating_mul(size_of::<String>()),
            Owner::Channels(values) => values.capacity().saturating_mul(size_of::<ChannelSpec>()),
            Owner::Fields(values) => values.capacity().saturating_mul(size_of::<FieldSpec>()),
            _ => 0,
        }))
    }
    /// 📏️ Borrows the next typed owner's exact allocation demand before ownership moves.
    pub fn next_close_byte_demand(&self) -> Result<usize, &'static str> {
        Ok(match self.owners.front() {
            Some(Owner::Owned(value)) => value.next_close_byte_demand(),
            Some(Owner::Map(value)) => value.next_close_byte_demand().map_err(|_| "Neural ordered retirement demand refused")?,
            Some(_) => 1,
            None => 0,
        })
    }
    /// 📏️ Preserves the logical page while admitting the next indivisible typed cleanup allocation.
    pub fn next_step_byte_demand(&self, logical_bytes: usize) -> Result<usize, &'static str> { Ok(logical_bytes.max(self.next_close_byte_demand()?)) }
    pub(crate) fn push_map(&mut self, map: OrderedMap<Value>) { let retirement = map.retire(); if !retirement.is_empty() { self.owners.push_back(Owner::Map(retirement)); } }


    /// ♻️ Advances one original owner; an undersized typed allocation grant retains it unchanged.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> ValueRetirementStep {
        if self.owners.is_empty() { return ValueRetirementStep::Complete; }
        if maximum_items == 0 || maximum_bytes == 0 { return ValueRetirementStep::Blocked; }
        if maximum_bytes < self.next_close_byte_demand().expect("finite Neural owner demand") { return ValueRetirementStep::Blocked; }
        let owner = self.owners.pop_front().expect("checked nonempty neural retirement");
        let mut released_bytes = 0;
        match owner {
            Owner::Owned(mut value)=>{let step=value.close_step(maximum_items,maximum_bytes).expect("typed input payload retirement");if !value.terminal_is_empty() {self.owners.push_front(Owner::Owned(value));}match step {semio_framework_value::SnapshotRetirementStep::Pending {released_bytes:bytes,..}=>released_bytes=bytes,semio_framework_value::SnapshotRetirementStep::Blocked=>return ValueRetirementStep::Blocked,semio_framework_value::SnapshotRetirementStep::Complete=>{}}},
            Owner::Map(mut map) => {
                let step = map.advance(RetainedCloneGrant { maximum_items: maximum_items.min(1), maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: 0, maximum_release_bytes: maximum_bytes, maximum_depth: map.next_depth_demand() });
                if !map.is_empty() { self.owners.push_front(Owner::Map(map)); }
                match step {
                    RetirementStep::OwnedValue(value) => self.owners.push_front(Owner::Value(value)),
                    RetirementStep::Progress { released_bytes: bytes, .. } => released_bytes = bytes,
                    RetirementStep::ProcessedBytes(_) => return ValueRetirementStep::Pending { released_items: 0, released_bytes: 0 },
                    RetirementStep::Failure(error) => panic!("Neural ordered retirement refused: {error}"),
                    RetirementStep::Blocked => return ValueRetirementStep::Blocked,
                    RetirementStep::Complete => {}
                }
            }
            Owner::Shared(value) => if let Some(value) = Arc::into_inner(value) { self.owners.push_front(Owner::Value(value)); },
            Owner::Value(Value::Dictionary(dictionary)) => self.push_dictionary(dictionary),
            Owner::Value(Value::Atom(Atom::String(text))) => self.owners.push_front(byte_owner(text.into_bytes())),
            Owner::Value(Value::Atom(_)) => {}
            Owner::Strings(mut values) if !values.is_empty() => {
                if let Some(value) = values.pop() { self.text(value); }
                self.owners.push_front(Owner::Strings(values));
            }
            Owner::Strings(values) => drop(values),
            Owner::Dictionaries(mut values) => {
                if let Some((key, value)) = values.pop_first() { self.text(key); self.push_dictionary(value); }
                if !values.is_empty() { self.owners.push_front(Owner::Dictionaries(values)); }
            }
            Owner::Snapshot(value) => { self.owners.push_front(Owner::Neurons(value.neurons)); self.owners.push_front(Owner::Seeds(value.seed_keys)); }
            Owner::Neurons(mut values) => {
                if let Some((key, value)) = values.pop_first() { self.text(key); self.owners.push_back(Owner::Strings(value.dependents)); }
                if !values.is_empty() { self.owners.push_front(Owner::Neurons(values)); }
            }
            Owner::Seeds(mut values) => {
                if let Some((key, _)) = values.pop_first() { self.text(key); }
                if !values.is_empty() { self.owners.push_front(Owner::Seeds(values)); }
            }
            Owner::Operator(value) => {
                self.text(value.id); self.text(value.extension); self.text(value.name); self.text(value.abbreviation); self.text(value.icon); self.text(value.summary);
                self.owners.push_back(Owner::Channels(value.inputs)); self.owners.push_back(Owner::Channels(value.outputs)); self.owners.push_back(Owner::Strings(value.group));
                if let Some(value) = value.variadic_input { self.text(value.slot_key); }
                if let Some(value) = value.variadic_output { self.text(value.slot_key); }
            }
            Owner::Channels(mut values) if !values.is_empty() => {
                if let Some(value) = values.pop() {
                    self.text(value.code); self.text(value.abbreviation); self.text(value.name); self.text(value.full_name);
                    if let Some(label) = value.label { self.text(label); }
                    if let Some(default) = value.default { self.push_value(default); }
                    self.owners.push_back(Owner::Strings(value.operators));
                }
                self.owners.push_front(Owner::Channels(values));
            }
            Owner::Channels(values) => drop(values),
            Owner::Schema(value) => {
                self.text(value.id); self.text(value.module); self.text(value.name); self.text(value.icon); self.text(value.summary);
                self.owners.push_back(Owner::Fields(value.fields));
            }
            Owner::Fields(mut values) if !values.is_empty() => {
                if let Some(value) = values.pop() {
                    self.text(value.key);
                    if let Some(label) = value.label { self.text(label); }
                    if let Some(default) = value.default { self.push_value(default); }
                    self.owners.push_back(Owner::Type(value.value));
                }
                self.owners.push_front(Owner::Fields(values));
            }
            Owner::Fields(values) => drop(values),
            Owner::Type(ValueType::Schema(id)) => self.text(id),
            Owner::Type(ValueType::List(inner)) => self.owners.push_front(Owner::Type(*inner)),
            Owner::Type(_) => {}
            Owner::Bytes { values, remaining_bytes } => {
                released_bytes = maximum_bytes.min(remaining_bytes);
                let remaining_bytes = remaining_bytes - released_bytes;
                if remaining_bytes != 0 {
                    self.owners.push_front(Owner::Bytes { values, remaining_bytes });
                    return ValueRetirementStep::Pending { released_items: 1, released_bytes };
                }
                drop(values);
            }
        }
        ValueRetirementStep::Pending { released_items: 1, released_bytes }
    }
}
impl Drop for ValueRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "neural values must finish explicit domain retirement before drop"); } }
}
impl semio_framework_value::retirement::RetireOwned for Dictionary {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::RetireOwned::retirement(self.pairs)}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.pairs)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Value {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {match self {Value::Atom(value)=>semio_framework_value::retirement::RetireOwned::retirement(value),Value::Dictionary(value)=>semio_framework_value::retirement::RetireOwned::retirement(value)}}
    fn retirement_birth_bytes(&self)->Option<usize> {match self {Value::Atom(value)=>semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(value),Value::Dictionary(value)=>semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(value)}}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for Atom {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {match self {Atom::String(value)=>semio_framework_value::retirement::RetireOwned::retirement(value),Atom::Null|Atom::Boolean(_)|Atom::Integer(_)|Atom::Decimal(_)=>semio_framework_value::retirement::RetireOwned::retirement(())}}
    fn retirement_birth_bytes(&self)->Option<usize> {match self {Atom::String(value)=>semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(value),Atom::Null|Atom::Boolean(_)|Atom::Integer(_)|Atom::Decimal(_)=>semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&())}}
    fn controlled_retirement_supported()->bool {true}
}
semio_framework_value::artifact_retire_struct!(FieldSpec {key,value,default,label});
semio_framework_value::artifact_retire_struct!(Schema {id,module,name,icon,summary,fields});
semio_framework_value::artifact_retire_struct!(ChannelSpec {code,abbreviation,name,full_name,operators,value_types,item_types,default,label,cardinality});
semio_framework_value::artifact_retire_struct!(OperatorInfo {id,extension,name,abbreviation,icon,summary,inputs,outputs,variadic_input,variadic_output,group});
impl semio_framework_value::retirement::RetireOwned for super::VariadicSpec {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::RetireOwned::retirement(self.slot_key)}
    fn retirement_birth_bytes(&self)->Option<usize> {semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.slot_key)}
    fn controlled_retirement_supported()->bool {true}
}
impl semio_framework_value::retirement::RetireOwned for super::Cardinality {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {semio_framework_value::retirement::RetireOwned::retirement(())}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(semio_framework_value::retirement::leaf_birth_bytes::<()>())}
    fn controlled_retirement_supported()->bool {true}
}
//#endregion 🧵️DomainRetirement

//#region 🧊️ColdOwners
/// 🧊️ Explicit synchronous boundary; never used by retained advance or close operations.
pub fn retire_value_cold(mut owner: ValueRetirement) {
    while !matches!(owner.close_step(1, 4096), ValueRetirementStep::Complete) {}
}

/// 🧊️ Cold construction owns replacement and error cleanup; its name makes unbounded work explicit.
pub struct ColdDictionaryBuilder { dictionary: Option<Dictionary> }
impl Default for ColdDictionaryBuilder { fn default() -> Self { Self { dictionary: Some(Dictionary::new()) } } }
impl ColdDictionaryBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn from_dictionary(dictionary: Dictionary) -> Self { Self { dictionary: Some(dictionary) } }
    pub fn dictionary(&self) -> &Dictionary { self.dictionary.as_ref().unwrap() }
    pub fn insert(&mut self, key: String, value: Value) {
        let dictionary = self.dictionary.as_mut().unwrap();
        let mut update = dictionary.pairs.begin_set(key, value);
        while !update.is_complete() { update.advance(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: update.next_capacity_byte_demand().expect("cold dictionary allocation demand"), maximum_release_bytes: 0, maximum_depth: update.next_depth_demand() }).expect("cold dictionary update"); }
        let displaced = std::mem::replace(&mut dictionary.pairs, update.take_result().unwrap());
        let mut retirement = ValueRetirement::default(); retirement.push_map(displaced); retire_value_cold(retirement);
        update.begin_close();
        loop {
            match update.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: update.next_close_byte_demand().expect("cold dictionary release demand"), maximum_depth: update.next_close_depth_demand() }) {
                RetirementStep::OwnedValue(value) => retire_value_cold(ValueRetirement::from_value(value)),
                RetirementStep::Complete => break,
                RetirementStep::Blocked => unreachable!("positive cold builder grant"),
                RetirementStep::Progress { .. } => {}
                RetirementStep::ProcessedBytes(_) => {}
                RetirementStep::Failure(error) => panic!("cold dictionary close refused: {error}"),
            }
        }
        assert!(update.terminal_is_empty());
    }
    /// 🛬️ Constructs one canonical dictionary entry with admitted metadata and resumable key comparisons.
    pub fn insert_controlled(&mut self, key: String, value: Value, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), semio_framework_value::ValueError> {
        let value = ColdValueOwner::new(value);
        control.charge(size_of::<String>() + size_of::<Value>() + 4 * size_of::<usize>())?;
        let dictionary = self.dictionary.as_mut().unwrap();
        let mut update = dictionary.pairs.begin_set(key, value.into_value());
        let result = (|| {
            while !update.is_complete() { update.advance_insert_controlled(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: update.next_capacity_byte_demand()?, maximum_release_bytes: 0, maximum_depth: update.next_depth_demand() }, control)?; }
            let displaced = std::mem::replace(&mut dictionary.pairs, update.take_result().unwrap());
            let mut retirement = ValueRetirement::default();
            retirement.push_map(displaced);
            retire_value_cold(retirement);
            Ok(())
        })();
        update.begin_close();
        loop {
            match update.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: update.next_close_byte_demand().expect("native dictionary release demand"), maximum_depth: update.next_close_depth_demand() }) {
                RetirementStep::OwnedValue(value) => retire_value_cold(ValueRetirement::from_value(value)),
                RetirementStep::Complete => break,
                RetirementStep::Blocked => unreachable!("positive native dictionary retirement grant"),
                RetirementStep::Progress { .. } => {}
                RetirementStep::ProcessedBytes(_) => {}
                RetirementStep::Failure(error) => panic!("native dictionary close refused: {error}"),
            }
        }
        assert!(update.terminal_is_empty());
        result
    }
    pub fn finish(mut self) -> Dictionary { self.dictionary.take().unwrap() }
}
impl Drop for ColdDictionaryBuilder {
    fn drop(&mut self) { if let Some(dictionary) = self.dictionary.take() { retire_value_cold(ValueRetirement::from_dictionary(dictionary)); } }
}

/// 🧊️ Explicit cold value scope for batch evaluation, decoding, and tests; retained owners use ValueRetirement.
pub struct ColdValueOwner { value: Option<Value> }
impl ColdValueOwner {
    pub fn new(value: Value) -> Self { Self { value: Some(value) } }
    pub fn value(&self) -> &Value { self.value.as_ref().unwrap() }
    pub fn into_value(mut self) -> Value { self.value.take().unwrap() }
}
impl Drop for ColdValueOwner {
    fn drop(&mut self) { if let Some(value) = self.value.take() { retire_value_cold(ValueRetirement::from_value(value)); } }
}
//#endregion 🧊️ColdOwners

struct DictionaryInputFrame {entries:std::vec::IntoIter<(String,protocol::value::DslValue)>,dictionary:Dictionary,key:Option<String>}

/// 🎒️ Retains typed input binding through the existing Dictionary update and retirement owners.
pub struct RetainedDictionaryInput {
    pending:Option<protocol::value::DslValue>,value:Option<Value>,frames:Vec<DictionaryInputFrame>,
    update:Option<protocol::value::ordered::UpdateCursor<Value>>,closing_update:bool,retirement:ValueRetirement,
    cancelled:bool,done:bool,units:usize,phase:&'static str,
}
impl RetainedDictionaryInput {
    /// 🌱️ Takes the admitted canonical candidate without cloning its text or nested objects.
    pub fn new(value:protocol::value::DslValue)->Self {Self {pending:Some(value),value:None,frames:Vec::new(),update:None,closing_update:false,retirement:Default::default(),cancelled:false,done:false,units:0,phase:"input-bind-value"}}
    /// 📍️ Reports retained binding transitions and the current input phase.
    pub fn progress(&self)->(usize,usize,&'static str) {(self.units,self.units.saturating_add(usize::from(!self.done)),self.phase)}
    /// 📏️ Borrows the next indivisible allocation without moving its retained input custody.
    pub fn next_step_byte_demand(&self,logical_bytes:usize)->Result<usize,protocol::value::ValueError> {
        let demand=if !self.retirement.terminal_is_empty() {self.retirement.next_close_byte_demand().map_err(|message|protocol::value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,message))?}
        else if let Some(update)=self.update.as_ref() {if self.closing_update {update.next_close_byte_demand()?}else {update.next_capacity_byte_demand()?}}
        else {0};
        Ok(logical_bytes.max(demand))
    }
    /// ⏱️ Binds at most the granted structural transitions and key-comparison bytes.
    pub fn step(&mut self,maximum_units:usize,maximum_bytes:usize)->Result<Option<Dictionary>,protocol::value::ValueError> {
        use protocol::value::{DslValue,FromValue,ValueError};use semio_framework_value::ValueRefusalKind;
        if maximum_units==0 || maximum_bytes==0 {return Ok(None);}
        if self.cancelled {return Err(ValueError::new(ValueRefusalKind::Canceled,"typed input binding canceled"));}
        for _ in 0..maximum_units {
            if !self.retirement.terminal_is_empty() {self.phase="input-bind-retire";self.retirement.close_step(1,maximum_bytes);}
            else if self.update.is_some() {
                if self.closing_update {self.phase="input-bind-retire";self.close_update(maximum_bytes)?;}
                else {self.phase="input-bind-update";let update=self.update.as_mut().unwrap();update.advance(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:maximum_bytes,maximum_capacity_bytes:maximum_bytes,maximum_release_bytes:0,maximum_depth:update.next_depth_demand()})?;if update.is_complete() {let dictionary=&mut self.frames.last_mut().unwrap().dictionary;self.retirement.push_map(std::mem::replace(&mut dictionary.pairs,update.take_result().unwrap()));update.begin_close();self.closing_update=true;}}
            } else if let Some(value)=self.value.take() {
                self.phase="input-bind-value";
                if let Some(frame)=self.frames.last_mut() {self.update=Some(frame.dictionary.pairs.begin_set(frame.key.take().unwrap(),value));self.closing_update=false;}
                else {let Value::Dictionary(dictionary)=value else {unreachable!()};self.units=self.units.saturating_add(1);self.done=true;return Ok(Some(dictionary));}
            } else if let Some(value)=&self.pending {
                self.phase="input-bind-value";
                if self.frames.is_empty() && !matches!(value,DslValue::Object(_)) {return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected an object for Dictionary"));}
                match value {
                    DslValue::Object(_)=>{if self.frames.len()>128 {return Err(ValueError::new(ValueRefusalKind::DepthLimit,"typed input nesting limit exceeded"));}let Some(DslValue::Object(entries))=self.pending.take() else {unreachable!()};self.frames.push(DictionaryInputFrame {entries:entries.into_iter(),dictionary:Dictionary::new(),key:None});},
                    DslValue::Array(_)|DslValue::Bytes(_)=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"expected an atom, found an array or bytes")),
                    DslValue::Number(protocol::value::Number::UInt(value)) if *value>i64::MAX as u64=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsigned atom exceeds the signed integer range")),
                    _=>self.value=Some(Value::Atom(Atom::from_value(self.pending.take().unwrap())?)),
                }
            } else if let Some(frame)=self.frames.last_mut() {
                self.phase="input-bind-value";
                if let Some((key,value))=frame.entries.next() {frame.key=Some(key);self.pending=Some(value);}
                else {let frame=self.frames.pop().unwrap();self.value=Some(Value::Dictionary(frame.dictionary));}
            } else {return Ok(None);}
            self.units=self.units.saturating_add(1);
        }
        Ok(None)
    }
    fn close_update(&mut self,maximum_bytes:usize)->Result<(),protocol::value::ValueError> {
        let update=self.update.as_mut().unwrap();match update.close_step(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:maximum_bytes,maximum_capacity_bytes:0,maximum_release_bytes:maximum_bytes,maximum_depth:update.next_close_depth_demand()}) {RetirementStep::OwnedValue(value)=>self.retirement.push_value(value),RetirementStep::Complete=>{assert!(update.terminal_is_empty());self.update=None;self.closing_update=false;},RetirementStep::Blocked|RetirementStep::Progress {..}|RetirementStep::ProcessedBytes(_)=>{},RetirementStep::Failure(error)=>return Err(error)}
        Ok(())
    }
    /// 🛑️ Records cancellation without releasing any candidate or partial update.
    pub fn cancel(&mut self) {self.cancelled=true;}
    /// ♻️ Closes the same raw, typed, update and displaced-value ownership frontiers.
    pub fn close_step(&mut self,maximum_units:usize,maximum_bytes:usize)->ValueRetirementStep {
        if maximum_units==0 || maximum_bytes==0 {return ValueRetirementStep::Blocked;}
        self.cancelled=true;
        if !self.retirement.terminal_is_empty() {return self.retirement.close_step(1,maximum_bytes);}
        if let Some(update)=&mut self.update {if !self.closing_update {update.begin_close();self.closing_update=true;}self.close_update(maximum_bytes).expect("Neural retained input close");}
        else if let Some(value)=self.pending.take() {self.retirement.push_owned(value);}
        else if let Some(value)=self.value.take() {self.retirement.push_value(value);}
        else if let Some(frame)=self.frames.pop() {self.retirement.push_dictionary(frame.dictionary);self.retirement.push_owned(frame.entries);if let Some(key)=frame.key {self.retirement.text(key);}}
        else {self.done=true;return ValueRetirementStep::Complete;}
        ValueRetirementStep::Pending {released_items:1,released_bytes:0}
    }
    /// 🔒️ Confirms that all private input ownership has drained or moved to its caller.
    pub fn terminal_is_empty(&self)->bool {self.pending.is_none() && self.value.is_none() && self.frames.is_empty() && self.update.is_none() && self.retirement.terminal_is_empty()}
}
impl Drop for RetainedDictionaryInput {fn drop(&mut self) {assert!(std::thread::panicking() || self.terminal_is_empty(),"typed input owner dropped before terminal-empty");}}

#[cfg(test)]
#[path = "🧪️tests/🧵️retirement/🦀️.rs"]
mod tests;
