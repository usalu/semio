//! 🧹️ Exact nested-value retirement and explicitly synchronous construction owners.

use super::{Atom, ChannelSpec, Dictionary, EvalChannels, FieldSpec, Operator, OperatorInfo, Schema, TreeSnapshot, Value, ValueType};
use semio_framework_value_ordered::ordered::{OrderedMap, RetirementStep, SharedOwner, UpdateCursor};
use semio_framework_value_persistent::list::{PagedList,PagedListReservationFailure,PagedListReservationOutcome};
use protocol::causal::transition::HistoryFoldIndex;
use semio_framework_value::{AuthoringWallet,invocation_extent,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value_retirement_queue::RetirementQueue;
use semio_framework_value::retirement::{controlled::ControlledRetirement,RetireOwned,RetirementCursor,RetirementStep as CursorStep};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close};
use std::collections::VecDeque;
use std::mem::{size_of,size_of_val};
//#region 🧵️DomainRetirement
#[derive(semio_framework_value::RetireOwned)]
enum Owner {
    Map(OrderedMap<Value>), Value(Value), Shared(SharedOwner<Value>), Bytes(Vec<u8>), Strings(Vec<String>),
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
/// 🪆️ Admits a wrapped original and hands the exact original back whenever the wrapper stays unadmitted.
fn adopt<T,W,R>(value:&mut Option<T>,wrap:impl FnOnce(T)->W,unwrap:impl FnOnce(W)->Option<T>,admit:impl FnOnce(&mut Option<W>)->RetainedWorkOutcome<R,ValueError>)->RetainedWorkOutcome<R,ValueError> {
    let mut wrapped=value.take().map(wrap);let turn=admit(&mut wrapped);
    if let Some(returned)=wrapped {match unwrap(returned) {Some(original)=>*value=Some(original),None=>return RetainedWorkOutcome {result:Err(ValueError::invariant("original domain admission kind")),work:turn.work}}}
    turn
}
macro_rules! domain_admission {
    ($name:ident,$variant:ident,$type:ty) => {
        pub fn $name(&mut self,value:&mut Option<$type>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError> {
            adopt(value,Owner::$variant,|owner|match owner {Owner::$variant(value)=>Some(value),_=>None},|owner|self.admit_domain(owner,effect))
        }
    };
}
impl ValueRetirement {
    fn from_owner(value:Owner)->Self {Self {root:Some(ControlledRetirement::retain(value)),owners:RetirementQueue::default()}}
    pub fn from_value(value:Value)->Self {Self::from_owner(Owner::Value(value))}
    pub fn from_dictionary(value:Dictionary)->Self {Self::from_value(Value::Dictionary(value))}
    pub fn from_shared(value:SharedOwner<Value>)->Self {Self::from_owner(Owner::Shared(value))}
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
    pub fn next_reserve_depth_demand(&self)->Result<usize,ValueError> {Ok(self.owners.reserve_demand()?.depth)}
    pub fn reserve_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError> {self.owners.reserve_step(effect)}
    pub const fn domain_frame_birth_bytes()->usize {semio_framework_value::retirement::owned_retirement_birth_bytes::<Owner>()}
    fn admit_domain(&mut self,value:&mut Option<Owner>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError> {
        let refuse=|error:ValueError|RetainedWorkOutcome {result:Err(error),work:0};
        if self.root.is_some() {return self.owners.admit_owned(value,effect);}
        if value.is_none() {return RetainedWorkOutcome {result:Ok(None),work:0};}
        let grant=effect.grant();
        if grant.maximum_items==0 {return refuse(ValueError::literal(ValueRefusalKind::WorkLimit,"Neural original handoff requires one admitted item"));}
        if grant.maximum_depth==0 {return refuse(ValueError::literal(ValueRefusalKind::DepthLimit,"Neural original handoff requires admitted depth"));}
        if !<Owner as RetireOwned>::controlled_retirement_supported() {return refuse(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Neural original handoff has no controlled retirement authority"));}
        match effect.performed(1) {Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome {result:Ok(None),work:1},Err(error)=>return RetainedWorkOutcome {result:Err(error),work:1}}
        let Some(owner)=value.take() else {return refuse(ValueError::invariant("Neural original handoff lost its owner"));};
        self.root=Some(ControlledRetirement::retain(owner));
        RetainedWorkOutcome {result:Ok(Some(RetainedCloneProgress {copied_items:1,..Default::default()})),work:1}
    }
    domain_admission!(push_value,Value,Value);
    domain_admission!(push_shared,Shared,SharedOwner<Value>);
    domain_admission!(push_dictionaries,Dictionaries,HistoryFoldIndex<String,Dictionary>);
    domain_admission!(push_channels,Channels,EvalChannels);
    domain_admission!(push_snapshot,Snapshot,TreeSnapshot);
    domain_admission!(push_operator,Operator,OperatorInfo);
    domain_admission!(push_schema,Schema,Schema);
    domain_admission!(push_strings,Strings,Vec<String>);
    pub fn push_dictionary(&mut self,value:&mut Option<Dictionary>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError> {
        adopt(value,Value::Dictionary,|wrapped|match wrapped {Value::Dictionary(value)=>Some(value),_=>None},|wrapped|self.push_value(wrapped,effect))
    }
    pub fn text(&mut self,value:&mut Option<String>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError> {
        adopt(value,|text|Value::Atom(Atom::String(text)),|wrapped|match wrapped {Value::Atom(Atom::String(value))=>Some(value),_=>None},|wrapped|self.push_value(wrapped,effect))
    }
    pub fn push_owned<T:RetireOwned>(&mut self,value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError> {self.owners.admit_owned(value,effect)}
    /// 🎟️ Borrows the one selected quote of the next action for the given body width.
    fn selected(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {self.root.as_ref().map_or_else(||self.owners.next_demand(body),|owner|owner.next_demand(body))}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(self.selected(0)?.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {Ok(self.selected(copy)?.capacity_bytes)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.selected(0)?.release_bytes)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {let selected=self.selected(COLD_PAGE_BYTES)?.depth;let owner=self.root.as_ref().map_or_else(||self.owners.next_depth_demand(),ControlledRetirement::next_depth_demand)?;Ok(selected.max(owner))}
    /// 🧩️ Builds the one-item grant that exactly funds the selected action at the given page width.
    pub fn exact_grant(&self,page:usize)->Result<RetainedCloneGrant,ValueError> {
        let mut copy=page.max(1);let mut demand=self.selected(copy)?;
        if demand.copy_bytes>copy {copy=demand.copy_bytes;demand=self.selected(copy)?;}
        Ok(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(self.next_depth_demand()?)})
    }
    /// ⛽️ Quotes the selected inline root or queued frame action.
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError> {self.root.as_ref().map_or_else(||self.owners.next_computational_work(body),|owner|owner.next_computational_work(body))}
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        let grant=effect.grant();
        let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
            let Some(root)=self.root.as_mut() else {return RetirementPollExit::resume(self.owners.step(body,effect));};
            let step=RetirementPollExit::resume(root.step(body,effect))?;
            let progress=admit_retained_clone_close(grant,step,root.terminal_is_empty(),"Neural original inline root")?.progress();
            if root.terminal_is_empty() {self.root=None;}
            Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
        })();
        RetirementPollExit::settle(effect,result)
    }
}
impl Drop for ValueRetirement {fn drop(&mut self) {assert!(self.terminal_is_empty(),"neural values must finish explicit domain retirement before drop");}}
/// 📥️ Records one settled native close poll in the caller-owned reply slot of its original invocation.
pub fn close_reply<'reply>(poll:OriginalRetirementPoll,effect:&RetainedAllocationEffect<'_>,reply:CloseReply<'reply>)->Result<CloseComplete<'reply>,ValueError> {
    if effect.invocation().is_none() {return Err(ValueError::invariant("native retirement requires its original producer invocation"));}
    Ok(match cursor_step(poll) {CursorStep::Failure(error,_)=>reply.complete(Err(error)),step=>reply.complete(Ok(step))})
}
/// 📨️ Names the reply a native close step records.
pub type CloseReply<'reply>=semio_framework_value::retirement::__control::OriginalEffectReply<'reply,CursorStep,semio_framework_value::OriginalRetirementFailure>;
/// 📬️ Names the completion token of a recorded native close step.
pub type CloseComplete<'reply>=semio_framework_value::retirement::__control::OriginalEffectReplyComplete<'reply,CursorStep,semio_framework_value::OriginalRetirementFailure>;
/// 🔁️ Maps a settled original close poll onto the cursor step vocabulary, keeping a suspended effect pending.
pub fn cursor_step(poll:OriginalRetirementPoll)->CursorStep {
    match poll {
        RetainedWorkPoll::Pending{work}=>CursorStep::Pending(work),
        RetainedWorkPoll::Complete(turn)=>match turn.result {
            Err(error)=>CursorStep::Failure(error,turn.work),
            Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>CursorStep::Complete(turn.work),
            Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)) if turn.work!=0||progress!=Default::default()=>CursorStep::Progress(progress,turn.work),
            Ok(_)=>CursorStep::BudgetExhausted,
        },
    }
}
impl RetireOwned for ValueRetirement {
    type RetirementCursor=Self;
    fn retirement_seed(self)->Self {self}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool {true}
}
impl RetirementCursor for ValueRetirement {
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError> {Self::next_computational_work(self,body)}
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{self.root.as_ref().map_or_else(||self.owners.next_demand(body),|owner|owner.next_demand(body))}
    fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:CloseReply<'reply>)->Result<CloseComplete<'reply>,ValueError> {close_reply(Self::close_step(self,body,effect),effect,reply)}
    fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError> {self.next_copy_byte_demand()}
    fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
    fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Self::next_depth_demand(self)}
    fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(size_of::<Self>())}
}
semio_framework_value::artifact_retire_struct!(FieldSpec {key,value,default,label});
semio_framework_value::artifact_retire_struct!(Schema {id,module,name,icon,summary,fields});
semio_framework_value::artifact_retire_struct!(ChannelSpec {code,abbreviation,name,full_name,operators,value_types,item_types,default,label,cardinality});
semio_framework_value::artifact_retire_struct!(OperatorInfo {id,extension,name,abbreviation,icon,summary,inputs,outputs,variadic_input,variadic_output,group});
semio_framework_value::artifact_retire_struct!(super::Tree {neurons,synapses});
semio_framework_value::artifact_retire_struct!(super::Neuron {id,kind,params,tree});
semio_framework_value::artifact_retire_struct!(super::Synapse {id,from,to,from_port,to_port});
/// 🌳️ Pairs the two shared snapshot originals so a snapshot retires through one derived seed.
#[derive(semio_framework_value::RetireOwned)]
pub struct SnapshotSources{tree:SharedOwner<super::Tree>,seeds:SharedOwner<HistoryFoldIndex<String,Dictionary>>}
impl RetireOwned for TreeSnapshot{type RetirementCursor=<SnapshotSources as RetireOwned>::RetirementCursor;fn retirement_seed(self)->Self::RetirementCursor{SnapshotSources{tree:self.tree,seeds:self.seeds}.retirement_seed()}fn controlled_retirement_supported()->bool{true}}
semio_framework_value::artifact_retire_struct!(EvalChannels {outputs,inputs});
impl RetireOwned for super::VariadicSpec {
    type RetirementCursor=<String as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor {self.slot_key.retirement_seed()}
    fn controlled_retirement_supported()->bool {true}
}
impl RetireOwned for super::Cardinality {
    type RetirementCursor=<() as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor {().retirement_seed()}
    fn controlled_retirement_supported()->bool {true}
}
//#endregion 🧵️DomainRetirement
//#region 🧊️ColdOwners
/// 🧊️ Names the bounded work one synchronous cold step may spend; each step opens a fresh finite wallet.
const COLD_STEP_WORK:u64=1<<10;
/// 🧊️ Names the page width of every cold copy grant.
const COLD_PAGE_BYTES:usize=4096;
/// 🧊️ Runs one cold step inside an explicit authoring stock because no turn funds it.
pub fn cold_effect<R>(grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<R,ValueError> {
    let (stock,work)=invocation_extent(grant,COLD_STEP_WORK)?;
    let mut wallet=AuthoringWallet::new(stock,u128::from(work));
    wallet.run_invoked(grant,COLD_STEP_WORK,operation)?.map(|authored|authored.output).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"cold Neural step work quote was not admitted"))
}
/// 🧊️ A synchronous boundary can never await a suspended original effect.
pub(crate) fn settled(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError> {
    match poll {
        RetainedWorkPoll::Pending{..}=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"synchronous Neural retirement cannot suspend")),
        RetainedWorkPoll::Complete(turn)=>turn.result.map_err(|failure|failure.into_value_error()),
    }
}
fn retire_grant(owner:&ValueRetirement)->Result<RetainedCloneGrant,ValueError> {owner.exact_grant(COLD_PAGE_BYTES)}
/// 🧊️ A cold step that moves nothing would loop forever, so it is refused typed.
fn moving(poll:OriginalRetirementPoll)->Result<(),ValueError> {
    match poll {
        RetainedWorkPoll::Pending{..}=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"synchronous Neural retirement cannot suspend")),
        RetainedWorkPoll::Complete(turn)=>{let step=turn.result.map_err(|failure|failure.into_value_error())?;if step.progress()==RetainedCloneProgress::default()&&turn.work==0&&!matches!(step,RetainedCloneStep::Complete(_)) {return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"cold Neural retirement step made no progress"))}Ok(())},
    }
}
fn cold_retire_step(owner:&mut ValueRetirement)->Result<(),ValueError> {
    let grant=retire_grant(owner)?;
    let body=grant.maximum_copy_bytes;moving(cold_effect(grant,|effect|owner.close_step(body,effect))?)
}
fn controlled_retire_step(owner:&mut ValueRetirement,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError> {
    let grant=retire_grant(owner)?;let quote=owner.next_computational_work(grant.maximum_copy_bytes)?;
    let body=grant.maximum_copy_bytes;let poll=control.with_effect(grant,quote,|_,effect|owner.close_step(body,effect))?.ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native Neural retirement work quote was not admitted"))?;
    moving(poll)
}
/// 🧊️ Drives one typed input binding phase under an exact cold grant.
pub(crate) fn cold_input_step(input:&mut RetainedDictionaryInput)->Result<DictionaryInputStep,ValueError> {
    let copy=4096.max(input.next_copy_byte_demand()?);
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:input.next_capacity_byte_demand(copy)?,maximum_release_bytes:input.next_release_byte_demand()?,maximum_depth:input.next_depth_demand()?};
    cold_effect(grant,|effect|input.step(effect))?
}
/// 🧊️ Drives one typed input closure phase under an exact cold grant.
pub(crate) fn cold_input_close(input:&mut RetainedDictionaryInput)->Result<(),ValueError> {
    let copy=4096.max(input.next_close_copy_byte_demand()?);
    let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:input.next_close_capacity_byte_demand(copy)?,maximum_release_bytes:input.next_close_release_byte_demand()?,maximum_depth:input.next_close_depth_demand()?};
    moving(cold_effect(grant,|effect|input.close_step(copy,effect))?)
}
/// 🧊️ Explicit synchronous boundary; never used by retained advance or close operations.
pub fn retire_value_cold(mut owner: ValueRetirement) {
    while !owner.terminal_is_empty() {
        if let Err(error)=cold_retire_step(&mut owner) {panic!("cold Neural retirement refused: {error}");}
    }
}
/// 🛬️ Retires through the receiving turn's own funded effect; cleanup that a refused turn cannot fund completes at the cold boundary instead of aborting.
pub fn retire_value_controlled(mut owner: ValueRetirement, control: &mut NativeDecodeControl<'_>) {
    while !owner.terminal_is_empty() {
        if controlled_retire_step(&mut owner,control).is_err() {retire_value_cold(owner);return;}
    }
}
fn cold_advance(update:&mut UpdateCursor<Value>)->Result<(),ValueError> {
    while !update.is_complete() {
        let capacity=update.next_capacity_byte_demand()?;
        update.advance(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: capacity, maximum_release_bytes: 0, maximum_depth: update.next_depth_demand() },u64::MAX).result?;
    }
    Ok(())
}
fn close_update(update:&mut UpdateCursor<Value>,retire:&mut dyn FnMut(Value))->Result<(),ValueError> {
    update.begin_close();
    loop {
        let grant=RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: update.next_close_byte_demand()?, maximum_depth: update.next_close_depth_demand() };
        match update.close_step(grant,u64::MAX) {
            RetirementStep::OwnedValue(value) => retire(value),
            RetirementStep::Complete => break,
            RetirementStep::Blocked => return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"positive cold dictionary close grant was blocked")),
            RetirementStep::Progress { .. } | RetirementStep::ProcessedBytes(_) => {}
            RetirementStep::Failure(error) => return Err(error),
        }
    }
    if update.terminal_is_empty() {Ok(())} else {Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"cold dictionary update closed with live ownership"))}
}

/// 🧊️ Cold construction owns replacement and error cleanup; its name makes unbounded work explicit.
pub struct ColdDictionaryBuilder { dictionary: Dictionary }
impl Default for ColdDictionaryBuilder { fn default() -> Self { Self { dictionary: Dictionary::new() } } }
impl ColdDictionaryBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn from_dictionary(dictionary: Dictionary) -> Self { Self { dictionary } }
    pub fn dictionary(&self) -> &Dictionary { &self.dictionary }
    pub fn insert(&mut self, key: String, value: Value) {
        if let Err(error)=self.cold_insert(key,value) {panic!("cold dictionary insert refused: {error}");}
    }
    fn cold_insert(&mut self, key: String, value: Value) -> Result<(), ValueError> {
        let mut update = self.dictionary.pairs.begin_set(key, value);
        if cold_advance(&mut update).is_err() { update.begin_close(); retire_value_cold(ValueRetirement::from_update(update)); return Ok(()); }
        let outcome = match update.take_result() {
            Some(result) => { let displaced = std::mem::replace(&mut self.dictionary.pairs, result); retire_value_cold(ValueRetirement::from_map(displaced)); Ok(()) }
            None => Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"cold dictionary update produced no result")),
        };
        close_update(&mut update,&mut |value|retire_value_cold(ValueRetirement::from_value(value)))?;
        outcome
    }
    /// 🛬️ Constructs one canonical dictionary entry with admitted metadata and resumable key comparisons; every retirement runs on the receiving turn's own effect.
    pub fn insert_controlled(&mut self, key: String, value: Value, control: &mut NativeDecodeControl<'_>) -> Result<(), ValueError> {
        if let Err(error) = control.charge(size_of::<String>() + size_of::<Value>() + 4 * size_of::<usize>()) { retire_value_controlled(ValueRetirement::from_value(value), control); return Err(error); }
        let mut update = self.dictionary.pairs.begin_set(key, value);
        let dictionary = &mut self.dictionary;
        let result = (|| {
            while !update.is_complete() {
                let demand = update.native_demands(COLD_PAGE_BYTES)?;let quote = update.native_work_demand(COLD_PAGE_BYTES)?;
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(update.next_depth_demand()) };
                control.with_effect(grant, quote, |_, effect| update.advance_native(COLD_PAGE_BYTES, effect))?.ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"native dictionary update work quote was not admitted"))?.result?;
            }
            let result = update.take_result().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native dictionary update produced no result"))?;
            let displaced = std::mem::replace(&mut dictionary.pairs, result);
            retire_value_controlled(ValueRetirement::from_map(displaced), control);
            Ok(())
        })();
        let closed = close_update(&mut update, &mut |value| retire_value_controlled(ValueRetirement::from_value(value), control));
        result.and(closed)
    }
    pub fn finish(mut self) -> Dictionary { std::mem::take(&mut self.dictionary) }
}
impl Drop for ColdDictionaryBuilder {
    fn drop(&mut self) { let dictionary = std::mem::take(&mut self.dictionary); if !dictionary.is_empty() { retire_value_cold(ValueRetirement::from_dictionary(dictionary)); } }
}

/// 🧊️ Explicit cold value scope for batch evaluation, decoding, and tests; retained owners use ValueRetirement.
pub struct ColdValueOwner { value: Value }
impl ColdValueOwner {
    pub fn new(value: Value) -> Self { Self { value } }
    pub fn value(&self) -> &Value { &self.value }
    pub fn into_value(mut self) -> Value { std::mem::replace(&mut self.value, Value::null()) }
}
impl Drop for ColdValueOwner {
    fn drop(&mut self) { let value = std::mem::replace(&mut self.value, Value::null()); if !value.is_null() { retire_value_cold(ValueRetirement::from_value(value)); } }
}
//#endregion 🧊️ColdOwners

#[derive(semio_framework_value::RetireOwned)]
struct DictionaryInputFrame {entries:VecDeque<(String,protocol::value::DslValue)>,dictionary:Dictionary,key:Option<String>}

/// 📤️ Preserves the full physical receipt of one original input phase.
pub struct DictionaryInputStep {pub progress:RetainedCloneProgress,pub dictionary:Option<Dictionary>}

/// 🧾️ Records work a grant-shaped child reported into the caller's effect; a synchronous owner cannot suspend.
fn pay_child_work(effect:&mut RetainedAllocationEffect<'_>,work:u128)->Result<(),ValueError> {
    RetirementPollExit::pay(effect,work).map_err(|exit|match exit {RetirementPollExit::Failed(failure)=>failure.into_value_error(),RetirementPollExit::Pending(_)=>ValueError::literal(ValueRefusalKind::InvariantViolated,"synchronous Neural input work cannot suspend")})
}
/// 🎟️ Maps one page reservation receipt onto the physical currencies, keeping the allocation refusal.
fn reservation_progress(outcome:Result<PagedListReservationOutcome,PagedListReservationFailure>)->Result<RetainedCloneProgress,ValueError> {
    match outcome {
        Ok(PagedListReservationOutcome::Denied)=>Ok(Default::default()),
        Ok(PagedListReservationOutcome::Ready(receipt))=>Ok(RetainedCloneProgress {copied_items:usize::from(receipt.progressed),retained_capacity_bytes:receipt.allocated_bytes,..Default::default()}),
        Err(PagedListReservationFailure::Work(error))=>Err(error),
        Err(PagedListReservationFailure::Allocation{error,copied_bytes})=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason).with_retained_progress(RetainedCloneProgress {copied_items:usize::from(error.allocated_bytes!=0||copied_bytes!=0),copied_bytes,retained_capacity_bytes:error.allocated_bytes,..Default::default()})),
    }
}
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
        if self.update.is_none()&&self.value.is_some()&&!self.frames.is_empty() {
            if self.shared_key.is_none() {return Ok(SharedOwner::<String>::source_admission_demand()?.copy_bytes);}
            if self.shared_value.is_none() {return Ok(SharedOwner::<Value>::source_admission_demand()?.copy_bytes);}
        }
        if self.reserving_frame() {return self.frames.next_reserve_copy_byte_demand().map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))}
        let Some(update)=self.update.as_ref().filter(|_|!self.closing_update) else {return Ok(0)};
        Ok(update.next_copy_byte_demand())
    }
    fn reserving_frame(&self)->bool {self.update.is_none()&&self.value.is_none()&&self.shared_value.is_none()&&matches!(self.pending,Some(protocol::value::DslValue::Object(_)))&&self.frames.len()<128&&!self.frames.has_reserved_slot()}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {
        if !self.retirement.terminal_is_empty() {return self.retirement.next_capacity_byte_demand(copy);}
        if let Some(update)=self.update.as_ref() {return if self.closing_update {Ok(0)}else {update.next_capacity_byte_demand()};}
        if self.value.is_some()&&!self.frames.is_empty() {
            return Ok(if self.shared_key.is_none() {SharedOwner::<String>::source_admission_demand()?.capacity_bytes}else if self.shared_value.is_none() {SharedOwner::<Value>::source_admission_demand()?.capacity_bytes}else {0});
        }
        if matches!(self.pending,Some(protocol::value::DslValue::Object(_))) {
            if self.frames.len()>=128 {return Ok(0);}
            return self.frames.next_allocation_bytes().map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason));
        }
        Ok(0)
    }
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {if self.retirement.terminal_is_empty(){Ok(0)}else{self.retirement.next_release_byte_demand()}}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {
        if !self.retirement.terminal_is_empty() {return self.retirement.next_depth_demand();}
        if self.reserving_frame() {return self.frames.next_reserve_depth_demand().map_err(|error|ValueError::literal(ValueRefusalKind::OwnershipLimit,error.reason))}
        Ok(self.update.as_ref().filter(|_|!self.closing_update).map_or(usize::from(!self.done),UpdateCursor::next_depth_demand))
    }
    /// ⏱️ Performs one admitted binding phase and preserves every independent receipt currency.
    pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<DictionaryInputStep,ValueError> {
        use protocol::value::{DslValue,FromValue};
        let grant=effect.grant();
        if self.cancelled {return Err(ValueError::literal(ValueRefusalKind::Canceled,"typed input binding canceled"));}
        if grant.maximum_items==0 {return Ok(DictionaryInputStep {progress:Default::default(),dictionary:None});}
        if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed input phase requires admitted depth"));}
        if !self.retirement.terminal_is_empty() {
            self.phase="input-bind-retire";
            let step=settled(self.retirement.close_step(grant.maximum_copy_bytes,effect))?;
            let progress=admit_retained_clone_close(grant,step,self.retirement.terminal_is_empty(),"Neural input child")?.progress();
            return Ok(DictionaryInputStep {progress,dictionary:None});
        }
        let mut progress=RetainedCloneProgress {copied_items:1,..Default::default()};
        if self.update.is_some() {
            if self.closing_update {
                let update=self.update.take().ok_or_else(||ValueError::invariant("typed input update absent"))?;
                self.phase="input-bind-retire";self.retirement=ValueRetirement::from_update(update);self.closing_update=false;
            } else {
                self.phase="input-bind-update";
                let update=self.update.as_mut().ok_or_else(||ValueError::invariant("typed input update absent"))?;
                let turn=update.advance(grant,effect.remaining_work_units()?);
                pay_child_work(effect,turn.work)?;
                let step=turn.result?;
                progress=admit_retained_clone_close(grant,step,update.is_complete(),"Neural input update")?.progress();
                if update.is_complete() {
                    let frame=self.frames.last_mut().ok_or_else(||ValueError::invariant("typed input frame absent"))?;
                    let result=update.take_result().ok_or_else(||ValueError::invariant("typed input update produced no result"))?;
                    self.retirement=ValueRetirement::from_map(std::mem::replace(&mut frame.dictionary.pairs,result));
                    self.closing_update=true;
                }
            }
        } else if self.value.is_some() {
            self.phase="input-bind-value";
            if let Some(frame)=self.frames.last_mut() {
                if self.shared_key.is_none() {
                    let key=frame.key.take().ok_or_else(||ValueError::invariant("key absent"))?;
                    let mut source=Some(key);let turn=SharedOwner::admit_from(&mut source,&mut self.shared_key,effect);
                    frame.key=source;progress=turn.result?.progress();
                } else if self.shared_value.is_none() {
                    if self.value.is_none() {return Err(ValueError::invariant("value absent"));}
                    let mut source=self.value.take();let turn=SharedOwner::admit_from(&mut source,&mut self.shared_value,effect);
                    self.value=source;progress=turn.result?.progress();
                } else {
                    let (key,value)=(self.shared_key.take(),self.shared_value.take());
                    let (Some(key),Some(value))=(key,value) else {return Err(ValueError::invariant("typed input shared owners absent"));};
                    self.update=Some(frame.dictionary.pairs.begin_set_shared(key,value));
                }
            } else if !self.frames.terminal_is_empty() {
                self.retirement=ValueRetirement::from_input_frames(std::mem::take(&mut self.frames));
            } else {
                let Some(Value::Dictionary(dictionary))=self.value.take() else {return Err(ValueError::invariant("typed input finished without a dictionary"));};
                self.units=self.units.saturating_add(1);self.done=true;return Ok(DictionaryInputStep {progress,dictionary:Some(dictionary)});
            }
        } else if self.shared_value.is_some() {
            self.phase="input-bind-update";
            let (key,value)=(self.shared_key.take(),self.shared_value.take());
            let (Some(key),Some(value))=(key,value) else {return Err(ValueError::invariant("typed input shared owners absent"));};
            let frame=self.frames.last().ok_or_else(||ValueError::invariant("typed input frame absent"))?;
            self.update=Some(frame.dictionary.pairs.begin_set_shared(key,value));
        } else if let Some(value)=self.pending.as_ref() {
            self.phase="input-bind-value";
            if self.frames.is_empty()&&!matches!(value,DslValue::Object(_)) {return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"expected an object for Dictionary"));}
            match value {
                DslValue::Object(_)=>{
                    if self.frames.len()>=128 {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed input nesting limit exceeded"));}
                    if !self.frames.has_reserved_slot() {
                        progress=reservation_progress(self.frames.reserve_one(effect).result)?;
                    } else {
                        let Some(DslValue::Object(entries))=self.pending.take() else {return Err(ValueError::invariant("typed input object absent"));};
                        if self.frames.push_reserved(DictionaryInputFrame {entries:VecDeque::from(entries),dictionary:Dictionary::new(),key:None}).is_err() {return Err(ValueError::invariant("admitted original input frame was refused"));}
                    }
                },
                DslValue::Array(_)|DslValue::Bytes(_)=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"expected an atom, found an array or bytes")),
                DslValue::Number(protocol::value::Number::UInt(value)) if *value>i64::MAX as u64=>return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"unsigned atom exceeds the signed integer range")),
                _=>{let atom=self.pending.take().ok_or_else(||ValueError::invariant("typed input atom absent"))?;self.value=Some(Value::Atom(Atom::from_value(atom)?));},
            }
        } else if let Some(frame)=self.frames.last_mut() {
            self.phase="input-bind-value";
            if let Some((key,value))=frame.entries.pop_front() {frame.key=Some(key);self.pending=Some(value);}
            else {
                let Some(mut frame)=self.frames.pop() else {return Err(ValueError::invariant("typed input frame absent"));};
                self.value=Some(Value::Dictionary(std::mem::replace(&mut frame.dictionary,Dictionary::new())));
                self.retirement=ValueRetirement::from_input_frame(frame);
            }
        } else {progress=Default::default();}
        if effect.performed_work()==0&&progress!=RetainedCloneProgress::default() {pay_child_work(effect,1)?;}
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
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        self.cancelled=true;let grant=effect.grant();
        let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
            if !self.retirement.terminal_is_empty() {let step=RetirementPollExit::resume(self.retirement.close_step(body,effect))?;return Ok(if self.terminal_is_empty(){step}else{RetainedCloneStep::Progress(step.progress())});}
            if self.terminal_is_empty() {self.done=true;return Ok(RetainedCloneStep::Complete(Default::default()));}
            if grant.maximum_items==0 {return Ok(RetainedCloneStep::Progress(Default::default()));}
            if grant.maximum_depth==0 {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"typed input handoff requires admitted depth").into());}
            self.retirement=if let Some(update)=self.update.take() {self.closing_update=false;ValueRetirement::from_update(update)}
            else if let Some(value)=self.pending.take() {ValueRetirement::from_input(value)}
            else if let Some(value)=self.value.take() {ValueRetirement::from_value(value)}
            else if let Some(value)=self.shared_key.take() {ValueRetirement::from_owner(Owner::InputSharedKey(value))}
            else if let Some(value)=self.shared_value.take() {ValueRetirement::from_owner(Owner::InputSharedValue(value))}
            else {ValueRetirement::from_input_frames(std::mem::take(&mut self.frames))};
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}))
        })();
        RetirementPollExit::settle(effect,result)
    }
    /// 🔒️ Confirms that all private input ownership has drained or moved to its caller.
    pub fn terminal_is_empty(&self)->bool {self.pending.is_none()&&self.value.is_none()&&self.frames.terminal_is_empty()&&self.update.is_none()&&self.shared_key.is_none()&&self.shared_value.is_none()&&self.retirement.terminal_is_empty()}
}
impl Drop for RetainedDictionaryInput {fn drop(&mut self) {assert!(std::thread::panicking()||self.terminal_is_empty(),"typed input owner dropped before terminal-empty");}}

#[cfg(test)]
#[path = "🧪️tests/🧵️retirement/🦀️.rs"]
mod tests;

//#region 🧵️OperatorRetirement
/// 🧹️ Implements the retirement cursor protocol over a type's inherent retirement methods.
#[macro_export]
macro_rules! original_retirement_cursor {
    ($type:ty) => {impl ::semio_framework_value::retirement::RetirementCursor for $type {
        fn next_demand(&self,body:usize)->Result<::semio_framework_value::RetirementDemand,::semio_framework_value::ValueError>{Self::demands(self,body)}
        fn next_computational_work(&self,body:usize)->Result<u64,::semio_framework_value::ValueError>{Self::next_computational_work(self,body)}
        fn close_into<'reply>(&mut self,body:usize,effect:&mut ::semio_framework_value::RetainedAllocationEffect<'_>,reply:$crate::retirement::CloseReply<'reply>)->Result<$crate::retirement::CloseComplete<'reply>,::semio_framework_value::ValueError>{$crate::retirement::close_reply(Self::close_step(self,body,effect),effect,reply)}
        fn terminal_is_empty(&self)->bool {Self::terminal_is_empty(self)}
        fn next_work_byte_demand(&self)->Result<usize,::semio_framework_value::ValueError> {self.next_copy_byte_demand()}
        fn next_birth_bytes(&self,copy:usize)->Option<usize> {self.next_capacity_byte_demand(copy).ok()}
        fn next_close_byte_demand(&self)->Option<usize> {self.next_release_byte_demand().ok()}
        fn next_depth_demand(&self)->Result<usize,::semio_framework_value::ValueError> {Self::next_depth_demand(self)}
        fn terminal_release_bytes(&self)->Option<usize> {self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
    }};
}
/// ⛽️ Bounds one operator field admission: a root handoff performs one unit, a queued frame two.
const OPERATOR_FIELD_ADMISSION_WORK:u64=2;
/// 🪪️ Closes one boxed operator's retained fields through admitted retirement grants.
pub struct OperatorRetirement {pub operator:Option<Box<dyn Operator>>,pub values:ValueRetirement}
impl OperatorRetirement {
    pub fn demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if !self.values.terminal_is_empty(){return RetirementCursor::next_demand(&self.values,body);}let Some(value)=self.operator.as_ref()else{return Ok(Default::default())};if value.retirement_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:size_of_val(value.as_ref()),depth:1,..Default::default()});}Ok(semio_framework_value::RetirementDemand{copy_bytes:value.next_retire_copy_byte_demand()?,capacity_bytes:value.next_retire_capacity_byte_demand(body)?,release_bytes:value.next_retire_release_byte_demand()?,depth:value.next_retire_depth_demand()?})}

    pub fn terminal_is_empty(&self)->bool {self.operator.is_none()&&self.values.terminal_is_empty()}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){self.values.next_copy_byte_demand()}else{self.operator.as_ref().map_or(Ok(0),|value|value.next_retire_copy_byte_demand())}}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){self.values.next_capacity_byte_demand(copy)}else{self.operator.as_ref().map_or(Ok(0),|value|value.next_retire_capacity_byte_demand(copy))}}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        if !self.values.terminal_is_empty(){return self.values.next_release_byte_demand();}
        self.operator.as_ref().map_or(Ok(0),|value|if value.retirement_is_empty(){Ok(size_of_val(value.as_ref()))}else{value.next_retire_release_byte_demand()})
    }
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {if !self.values.terminal_is_empty(){self.values.next_depth_demand()}else{self.operator.as_ref().map_or(Ok(0),|value|if value.retirement_is_empty(){Ok(1)}else{value.next_retire_depth_demand()})}}
    /// ⛽️ Quotes the pending field handoff or shell release; a field admission performs at most two units.
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError> {
        if !self.values.terminal_is_empty() {return self.values.next_computational_work(body);}
        Ok(match self.operator.as_ref() {None=>0,Some(value) if value.retirement_is_empty()=>1,Some(_)=>OPERATOR_FIELD_ADMISSION_WORK})
    }
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        let grant=effect.grant();let empty=RetainedCloneProgress::default();
        let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
            if self.terminal_is_empty() {return Ok(RetainedCloneStep::Complete(empty));}
            if !self.values.terminal_is_empty() {let step=RetirementPollExit::resume(self.values.close_step(body,effect))?;return Ok(RetainedCloneStep::Progress(step.progress()));}
            if grant.maximum_items==0 {return Ok(RetainedCloneStep::Progress(empty));}
            if grant.maximum_depth<self.next_depth_demand()? {return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"operator retained ownership exceeds admitted depth").into());}
            let Some(value)=self.operator.as_mut() else {return Err(ValueError::invariant("operator shell absent").into());};
            if value.retirement_is_empty() {
                let bytes=size_of_val(value.as_ref());
                if bytes>grant.maximum_release_bytes {return Ok(RetainedCloneStep::Progress(empty));}
                if !effect.performed(1)? {return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"operator shell release exceeds its work ceiling").into());}
                drop(self.operator.take());
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty}));
            }
            let step=value.retire_step(effect,&mut self.values)?;
            Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant,step,value.retirement_is_empty(),"original operator fields")?.progress()))
        })();
        RetirementPollExit::settle(effect,result)
    }
}
impl Drop for OperatorRetirement {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"operator abandoned before physical shell closure");}}
impl RetireOwned for Box<dyn Operator> {
    type RetirementCursor=OperatorRetirement;
    fn retirement_seed(self)->OperatorRetirement {OperatorRetirement {operator:Some(self),values:ValueRetirement::default()}}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<OperatorRetirement>())}
    fn controlled_retirement_supported()->bool {true}
}
original_retirement_cursor!(OperatorRetirement);
//#endregion 🧵️OperatorRetirement
