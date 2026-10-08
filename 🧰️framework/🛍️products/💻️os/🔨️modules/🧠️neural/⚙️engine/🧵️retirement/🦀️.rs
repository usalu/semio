//! 🧹️ Exact nested-value retirement and explicitly synchronous construction owners.

use super::{Atom, ChannelSpec, Dictionary, EvalChannels, FieldSpec, NeuronSnapshot, OperatorInfo, Schema, TreeSnapshot, Value, ValueType};
use protocol::value::ordered::{OrderedMap, RetirementStep,SharedOwner,UpdateCursor};
use semio_framework_value::list::PagedList;
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{controlled::ControlledRetirement,queue::RetirementQueue,RetireOwned,RetirementCursor};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close};
use std::collections::VecDeque;
use std::mem::{size_of, ManuallyDrop};
use std::sync::Arc;

//#region 🧵️DomainRetirement
#[derive(semio_framework_value::RetireOwned)]
enum Owner {
    Map(OrderedMap<Value>), Value(Value), Shared(Arc<Value>), Bytes(Vec<u8>), Strings(Vec<String>),
    Dictionaries(HistoryFoldIndex<String,Dictionary>), Snapshot(TreeSnapshot), Operator(OperatorInfo),
    Channels(EvalChannels), Schema(Schema), Fields(Vec<FieldSpec>), Type(ValueType),
    SchemaEntry(String,Schema), OperatorEntry(String,OperatorInfo), StringsEntry(String,Vec<String>),
    Input(protocol::value::DslValue), InputFrame(DictionaryInputFrame),InputFrames(PagedList<DictionaryInputFrame,128>),
    Update(UpdateCursor<Value>),InputSharedKey(SharedOwner<String>),InputSharedValue(SharedOwner<Value>),
}

/// 🔒️ Keeps one original domain root inline and every additional frame in admitted custody.
#[must_use = "nested value retirement must reach terminal-empty before drop"]
pub struct ValueRetirement { root:Option<ControlledRetirement<Owner>>, owners:RetirementQueue }
impl Default for ValueRetirement {fn default()->Self {Self {root:None,owners:RetirementQueue::default()}}}
macro_rules! domain_admission {
    ($name:ident,$variant:ident,$type:ty) => {
        pub fn $name(&mut self,value:$type,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,$type)> {
            self.admit_domain(Owner::$variant(value),grant).map_err(|(error,value)|match value {Owner::$variant(value)=>(error,value),_=>unreachable!("original domain admission kind")})
        }
    };
}
impl ValueRetirement {
    fn from_owner(value:Owner)->Self {Self {root:Some(ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("Neural domain retirement refused: {error}"))),owners:RetirementQueue::default()}}
    pub fn from_value(value:Value)->Self {Self::from_owner(Owner::Value(value))}
    pub fn from_dictionary(value:Dictionary)->Self {Self::from_value(Value::Dictionary(value))}
    pub fn from_shared(value:Arc<Value>)->Self {Self::from_owner(Owner::Shared(value))}
    pub fn from_channels(value:EvalChannels)->Self {Self::from_owner(Owner::Channels(value))}
    pub fn from_snapshot(value:TreeSnapshot)->Self {Self::from_owner(Owner::Snapshot(value))}
    pub fn from_schema_entry(key:String,value:Schema)->Self {Self::from_owner(Owner::SchemaEntry(key,value))}
    pub fn from_operator_entry(key:String,value:OperatorInfo)->Self {Self::from_owner(Owner::OperatorEntry(key,value))}
    pub fn from_strings_entry(key:String,value:Vec<String>)->Self {Self::from_owner(Owner::StringsEntry(key,value))}
    fn from_map(value:OrderedMap<Value>)->Self {Self::from_owner(Owner::Map(value))}
    fn from_input(value:protocol::value::DslValue)->Self {Self::from_owner(Owner::Input(value))}
    fn from_input_frame(value:DictionaryInputFrame)->Self {Self::from_owner(Owner::InputFrame(value))}
    fn from_input_frames(value:PagedList<DictionaryInputFrame,128>)->Self {Self::from_owner(Owner::InputFrames(value))}
    fn from_update(value:UpdateCursor<Value>)->Self {Self::from_owner(Owner::Update(value))}
    pub fn original_value(&self)->Option<&Value> {match self.root.as_ref()?.original()? {Owner::Value(value)=>Some(value),_=>None}}
    pub fn terminal_is_empty(&self)->bool {self.root.is_none()&&self.owners.terminal_is_empty()}
    pub fn next_reserve_capacity_byte_demand(&self)->Result<usize,ValueError> {self.owners.next_reserve_capacity_byte_demand()}
    pub fn reserve_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {self.owners.reserve_step(grant)}
    pub const fn domain_frame_birth_bytes()->usize {semio_framework_value::retirement::owned_retirement_birth_bytes::<Owner>()}
    fn admit_domain(&mut self,value:Owner,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Owner)> {
        if self.root.is_some() {return self.owners.admit_owned(value,grant);}
        if grant.maximum_items==0 {return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"Neural original handoff requires one admitted item"),value));}
        if grant.maximum_depth==0 {return Err((ValueError::literal(ValueRefusalKind::DepthLimit,"Neural original handoff requires admitted depth"),value));}
        match ControlledRetirement::new(value) {Ok(owner)=>{self.root=Some(owner);Ok(RetainedCloneProgress {copied_items:1,..Default::default()})},Err(error)=>Err(error)}
    }
    domain_admission!(push_value,Value,Value);
    domain_admission!(push_shared,Shared,Arc<Value>);
    domain_admission!(push_dictionaries,Dictionaries,HistoryFoldIndex<String,Dictionary>);
    domain_admission!(push_channels,Channels,EvalChannels);
    domain_admission!(push_snapshot,Snapshot,TreeSnapshot);
    domain_admission!(push_operator,Operator,OperatorInfo);
    domain_admission!(push_schema,Schema,Schema);
    domain_admission!(push_strings,Strings,Vec<String>);
    pub fn push_dictionary(&mut self,value:Dictionary,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,Dictionary)> {
        self.push_value(Value::Dictionary(value),grant).map_err(|(error,value)|match value {Value::Dictionary(value)=>(error,value),_=>unreachable!("original dictionary admission kind")})
    }
    pub fn text(&mut self,value:String,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,String)> {
        self.push_value(Value::Atom(Atom::String(value)),grant).map_err(|(error,value)|match value {Value::Atom(Atom::String(value))=>(error,value),_=>unreachable!("original string admission kind")})
    }
    pub fn push_owned<T:RetireOwned>(&mut self,value:T,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,(ValueError,T)> {self.owners.admit_owned(value,grant)}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.owners.next_copy_byte_demand(),ControlledRetirement::next_copy_byte_demand)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.owners.next_capacity_byte_demand(copy),|owner|owner.next_capacity_byte_demand(copy))}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.owners.next_release_byte_demand(),ControlledRetirement::next_release_byte_demand)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {self.root.as_ref().map_or_else(||self.owners.next_depth_demand(),ControlledRetirement::next_depth_demand)}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let Some(root)=self.root.as_mut() else {return self.owners.step(grant);};
        let step=root.step(grant)?;
        let progress=admit_retained_clone_close(grant,step,root.terminal_is_empty(),"Neural original inline root")?.progress();
        if root.terminal_is_empty() {self.root=None;}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
impl Drop for ValueRetirement {fn drop(&mut self) {assert!(std::thread::panicking()||self.terminal_is_empty(),"neural values must finish explicit domain retirement before drop");}}
impl RetireOwned for ValueRetirement {
    fn retirement(self)->Box<dyn RetirementCursor> {Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool {true}
}
impl RetirementCursor for ValueRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep as Step;
        match Self::close_step(self,grant) {Err(error)=>Step::Failure(error),Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>Step::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Step::Progress(progress)}
    }
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
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
semio_framework_value::artifact_retire_struct!(super::Tree {neurons,synapses});
semio_framework_value::artifact_retire_struct!(super::Neuron {id,kind,params,tree});
semio_framework_value::artifact_retire_struct!(super::Synapse {id,from,to,from_port,to_port});
semio_framework_value::artifact_retire_struct!(NeuronSnapshot {key,incoming,dependents});
semio_framework_value::artifact_retire_struct!(TreeSnapshot {neurons,seed_keys});
semio_framework_value::artifact_retire_struct!(EvalChannels {outputs,inputs});
semio_framework_value::artifact_retire_struct!(super::OperatorImpl {schemas,operator});
semio_framework_value::artifact_retire_struct!(super::OperatorRecord {info,implementations});
semio_framework_value::artifact_retire_struct!(super::Registry {schemas,operators,operator_produces,schema_providers,finalized});
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
    while !owner.terminal_is_empty() {
        let copy=4096.max(owner.next_copy_byte_demand().expect("cold Neural copy demand"));
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).expect("cold Neural capacity demand"),maximum_release_bytes:owner.next_release_byte_demand().expect("cold Neural release demand"),maximum_depth:owner.next_depth_demand().expect("cold Neural depth demand")};
        owner.close_step(grant).expect("cold Neural retirement");
    }
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
        retire_value_cold(ValueRetirement::from_map(displaced));
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
            retire_value_cold(ValueRetirement::from_map(displaced));
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

#[derive(semio_framework_value::RetireOwned)]
struct DictionaryInputFrame {entries:VecDeque<(String,protocol::value::DslValue)>,dictionary:Dictionary,key:Option<String>}

/// 📤️ Preserves the full physical receipt of one original input phase.
pub struct DictionaryInputStep {pub progress:RetainedCloneProgress,pub dictionary:Option<Dictionary>}

/// 🎒️ Retains typed input binding through the existing Dictionary update and retirement owners.
pub struct RetainedDictionaryInput {
    pending:Option<protocol::value::DslValue>,value:Option<Value>,frames:PagedList<DictionaryInputFrame,128>,
    update:Option<UpdateCursor<Value>>,closing_update:bool,retirement:ValueRetirement,
    shared_key:Option<SharedOwner<String>>,shared_value:Option<SharedOwner<Value>>,
    cancelled:bool,done:bool,units:usize,phase:&'static str,
}
impl RetainedDictionaryInput {
    /// 🌱️ Takes the original candidate without allocating a new ownership frontier.
    pub fn new(value:protocol::value::DslValue)->Self {Self {pending:Some(value),value:None,frames:Default::default(),update:None,closing_update:false,retirement:Default::default(),shared_key:None,shared_value:None,cancelled:false,done:false,units:0,phase:"input-bind-value"}}
    /// 📍️ Reports retained binding transitions and the current input phase.
    pub fn progress(&self)->(usize,usize,&'static str) {(self.units,self.units.saturating_add(usize::from(!self.done)),self.phase)}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {
        if !self.retirement.terminal_is_empty() {return self.retirement.next_copy_byte_demand();}
        Ok(self.update.as_ref().filter(|_|!self.closing_update).map_or(0,UpdateCursor::next_copy_byte_demand))
    }
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {
        if !self.retirement.terminal_is_empty() {return self.retirement.next_capacity_byte_demand(copy);}
        if let Some(update)=self.update.as_ref() {return if self.closing_update {Ok(0)}else {update.next_capacity_byte_demand()};}
        if self.value.is_some()&&!self.frames.is_empty() {
            return Ok(if self.shared_key.is_none() {SharedOwner::<String>::allocation_bytes()}else if self.shared_value.is_none() {SharedOwner::<Value>::allocation_bytes()}else {0});
        }
        if matches!(self.pending,Some(protocol::value::DslValue::Object(_))) {
            return self.frames.next_capacity_allocation_bytes(self.frames.len()+1).map(|bytes|bytes.unwrap_or(0)).map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason));
        }
        Ok(0)
    }
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {if self.retirement.terminal_is_empty(){Ok(0)}else{self.retirement.next_release_byte_demand()}}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {
        if !self.retirement.terminal_is_empty() {return self.retirement.next_depth_demand();}
        Ok(self.update.as_ref().filter(|_|!self.closing_update).map_or(usize::from(!self.done),UpdateCursor::next_depth_demand))
    }
    /// ⏱️ Performs one admitted binding phase and preserves every independent receipt currency.
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<DictionaryInputStep,ValueError> {
        use protocol::value::{DslValue,FromValue};
        if self.cancelled {return Err(ValueError::literal(ValueRefusalKind::Canceled,"typed input binding canceled"));}
        if grant.maximum_items==0 {return Ok(DictionaryInputStep {progress:Default::default(),dictionary:None});}
        if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed input phase requires admitted depth"));}
        if !self.retirement.terminal_is_empty() {
            self.phase="input-bind-retire";
            let step=self.retirement.close_step(grant)?;
            let progress=admit_retained_clone_close(grant,step,self.retirement.terminal_is_empty(),"Neural input child")?.progress();
            return Ok(DictionaryInputStep {progress,dictionary:None});
        }
        let mut progress=RetainedCloneProgress {copied_items:1,..Default::default()};
        if self.update.is_some() {
            if self.closing_update {
                self.phase="input-bind-retire";self.retirement=ValueRetirement::from_update(self.update.take().unwrap());self.closing_update=false;
            } else {
                self.phase="input-bind-update";let update=self.update.as_mut().unwrap();
                let step=update.advance(grant)?;
                progress=admit_retained_clone_close(grant,step,update.is_complete(),"Neural input update")?.progress();
                if update.is_complete() {
                    let dictionary=&mut self.frames.last_mut().unwrap().dictionary;
                    self.retirement=ValueRetirement::from_map(std::mem::replace(&mut dictionary.pairs,update.take_result().unwrap()));
                    self.closing_update=true;
                }
            }
        } else if self.value.is_some() {
            self.phase="input-bind-value";
            if let Some(frame)=self.frames.last_mut() {
                if self.shared_key.is_none() {
                    match SharedOwner::admit(frame.key.take().unwrap(),grant) {Ok((owner,receipt))=>{self.shared_key=Some(owner);progress=receipt;},Err((error,key))=>{frame.key=Some(key);return Err(error);}}
                } else if self.shared_value.is_none() {
                    match SharedOwner::admit(self.value.take().unwrap(),grant) {Ok((owner,receipt))=>{self.shared_value=Some(owner);progress=receipt;},Err((error,value))=>{self.value=Some(value);return Err(error);}}
                } else {
                    self.update=Some(frame.dictionary.pairs.begin_set_shared(self.shared_key.take().unwrap(),self.shared_value.take().unwrap()));
                }
            } else if !self.frames.terminal_is_empty() {
                self.retirement=ValueRetirement::from_input_frames(std::mem::take(&mut self.frames));
            } else {
                let Value::Dictionary(dictionary)=self.value.take().unwrap() else {unreachable!()};
                self.units=self.units.saturating_add(1);self.done=true;return Ok(DictionaryInputStep {progress,dictionary:Some(dictionary)});
            }
        } else if self.shared_value.is_some() {
            self.phase="input-bind-update";
            self.update=Some(self.frames.last().unwrap().dictionary.pairs.begin_set_shared(self.shared_key.take().unwrap(),self.shared_value.take().unwrap()));
        } else if let Some(value)=self.pending.as_ref() {
            self.phase="input-bind-value";
            if self.frames.is_empty()&&!matches!(value,DslValue::Object(_)) {return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"expected an object for Dictionary"));}
            match value {
                DslValue::Object(_)=>{
                    if self.frames.len()>=128 {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed input nesting limit exceeded"));}
                    if !self.frames.has_reserved_slot() {
                        let receipt=self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))?;
                        progress=RetainedCloneProgress {copied_items:usize::from(receipt.progressed),retained_capacity_bytes:receipt.allocated_bytes,..Default::default()};
                    } else {
                        let Some(DslValue::Object(entries))=self.pending.take() else {unreachable!()};
                        self.frames.push_reserved(DictionaryInputFrame {entries:VecDeque::from(entries),dictionary:Dictionary::new(),key:None}).unwrap_or_else(|_|unreachable!("admitted original input frame"));
                    }
                },
                DslValue::Array(_)|DslValue::Bytes(_)=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"expected an atom, found an array or bytes")),
                DslValue::Number(protocol::value::Number::UInt(value)) if *value>i64::MAX as u64=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unsigned atom exceeds the signed integer range")),
                _=>self.value=Some(Value::Atom(Atom::from_value(self.pending.take().unwrap())?)),
            }
        } else if let Some(frame)=self.frames.last_mut() {
            self.phase="input-bind-value";
            if let Some((key,value))=frame.entries.pop_front() {frame.key=Some(key);self.pending=Some(value);}
            else {
                let mut frame=self.frames.pop().unwrap();
                self.value=Some(Value::Dictionary(std::mem::replace(&mut frame.dictionary,Dictionary::new())));
                self.retirement=ValueRetirement::from_input_frame(frame);
            }
        } else {progress=Default::default();}
        self.units=self.units.saturating_add(progress.copied_items);
        Ok(DictionaryInputStep {progress,dictionary:None})
    }
    /// 🛑️ Records cancellation without releasing any candidate or partial update.
    pub fn cancel(&mut self) {self.cancelled=true;}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {if self.retirement.terminal_is_empty(){Ok(0)}else{self.retirement.next_copy_byte_demand()}}
    pub fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if self.retirement.terminal_is_empty(){Ok(0)}else{self.retirement.next_capacity_byte_demand(copy)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {if self.retirement.terminal_is_empty(){Ok(0)}else{self.retirement.next_release_byte_demand()}}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError> {if self.retirement.terminal_is_empty(){Ok(usize::from(!self.terminal_is_empty()))}else{self.retirement.next_depth_demand()}}
    /// ♻️ Moves each original inline frontier before closing its genuine payload and backing.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        self.cancelled=true;
        if !self.retirement.terminal_is_empty() {return self.retirement.close_step(grant).map(|step|if self.terminal_is_empty(){step}else{RetainedCloneStep::Progress(step.progress())});}
        if self.terminal_is_empty() {self.done=true;return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0 {return Ok(RetainedCloneStep::Progress(Default::default()));}
        if grant.maximum_depth==0 {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed input handoff requires admitted depth"));}
        self.retirement=if let Some(update)=self.update.take() {self.closing_update=false;ValueRetirement::from_update(update)}
        else if let Some(value)=self.pending.take() {ValueRetirement::from_input(value)}
        else if let Some(value)=self.value.take() {ValueRetirement::from_value(value)}
        else if let Some(value)=self.shared_key.take() {ValueRetirement::from_owner(Owner::InputSharedKey(value))}
        else if let Some(value)=self.shared_value.take() {ValueRetirement::from_owner(Owner::InputSharedValue(value))}
        else {ValueRetirement::from_input_frames(std::mem::take(&mut self.frames))};
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}))
    }
    /// 🔒️ Confirms that all private input ownership has drained or moved to its caller.
    pub fn terminal_is_empty(&self)->bool {self.pending.is_none()&&self.value.is_none()&&self.frames.terminal_is_empty()&&self.update.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.retirement.terminal_is_empty()}
}
impl Drop for RetainedDictionaryInput {fn drop(&mut self) {assert!(std::thread::panicking()||self.terminal_is_empty(),"typed input owner dropped before terminal-empty");}}

#[cfg(test)]
#[path = "🧪️tests/🧵️retirement/🦀️.rs"]
mod tests;
