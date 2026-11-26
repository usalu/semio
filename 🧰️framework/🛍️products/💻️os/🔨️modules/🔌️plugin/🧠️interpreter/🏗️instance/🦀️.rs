//! 🏗️ Original caller phases reserve, initialize and retire Core resource storage before any guest instruction starts.
use super::*;
use semio_framework_job::{OriginalCallerSourceLoan,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize,Ordering};
#[path="🧊️prepared/🦀️.rs"]
mod prepared_instance;
pub use prepared_instance::PreparedCoreInstance;
use prepared_instance::{copy_image,copy_image_segment};

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Vectors(u8),Memories(usize),Tables(usize),Globals(usize),Data(usize,usize),Elements(usize,usize),Ready,Closing}
/// 🗝️ The embedding that opens a Core owns its separate canceled-input retirement authority.
pub struct CoreInputRetirementAuthority(NonZeroUsize);
impl CoreInputRetirementAuthority{
 pub fn new()->Self{
  static NEXT:AtomicUsize=AtomicUsize::new(1);
  let identity=NEXT.try_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).expect("Core input retirement identity exhausted");
  Self(NonZeroUsize::new(identity).unwrap())
 }
}
/// 🧱️ Holds the actual partial instance and initiating source until its resource opening or paid closure completes.
pub struct CoreInstanceOpening{core:ManuallyDrop<Option<CoreInstance>>,source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,epoch:u64,prepared:Option<u64>,phase:Phase,invocation_complete:bool,returning_frame:bool,control:Option<original_invocation::OriginalControlPlan>,memory_work:Option<original_invocation::OriginalMemoryWork>,host_call:Option<original_invocation::OriginalHostCall>,input_retirement_authority:Option<NonZeroUsize>,input_retirement_epoch:Option<u64>,actor_execution:Option<std::time::Duration>,ledger:RetainedLedger}
impl CoreInstanceOpening{
 /// 📒️ This instance's retained ledger counters, copied without formatting.
 pub fn retained_ledger_snapshot(&self)->[u64;20]{self.ledger.snapshot()}
 /// 📒️ This instance's retained ledger by category as one line.
 pub fn retained_ledger_text(&self)->String{self.ledger.to_string()}
 /// 📒️ Directs this thread's retained bookings to this instance while the guard lives.
 pub fn ledger_scope(&self)->LedgerScope{LedgerScope::enter(&self.ledger)}
 pub fn begin(module:&Arc<CoreModule>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
  Self::begin_actor(module,None,None,cx)
 }
 /// 🏠️ Opens private actor storage under the actual caller while borrowing a sealed initialization source.
 pub fn begin_prepared(module:&Arc<CoreModule>,prepared:&PreparedCoreInstance,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
  Self::begin_actor(module,Some(prepared),None,cx)
 }
 /// 🧷️ Seals the opening embedding's authority into its actual fresh or prepared Core.
 pub fn begin_actor(module:&Arc<CoreModule>,prepared:Option<&PreparedCoreInstance>,input_retirement:Option<&CoreInputRetirementAuthority>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
  if prepared.is_some_and(|image|!image.matches_module(module)){return Err(refused("prepared Core belongs to another compiled module").into())}
  let epoch=cx.original_source_epoch().ok_or_else(||refused("Core opening requires the initiating input epoch"))?;
  let ledger=RetainedLedger::default();let scope=LedgerScope::enter(&ledger);
  if !charge(cx,size_of::<Self>(),0,0)?{return Ok(None)}
  let parent=cx.original_source_parent()?;let mut loan=None;if !parent.admit_loan(&mut loan,cx)?{return Ok(None)}let Some(source)=loan else{return Ok(None)};
  Ok(Some(Self{core:ManuallyDrop::new(Some(CoreInstance{module:Arc::clone(module),memories:Vec::new(),tables:Vec::new(),globals:Vec::new(),data:Vec::new(),elements:Vec::new(),machine:None,pool:FramePool::default(),next_host_call:1,preparation_storage:None})),source:ManuallyDrop::new(Some(source)),epoch,prepared:prepared.map(|image|image.identity),phase:Phase::Vectors(0),invocation_complete:false,returning_frame:false,control:None,memory_work:None,host_call:None,input_retirement_authority:input_retirement.map(|authority|authority.0),input_retirement_epoch:None,actor_execution:None,ledger:{drop(scope);ledger}}))
 }
 fn validate(&self,module:&Arc<CoreModule>,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  self.source.as_ref().ok_or_else(||refused("Core opening source has already returned"))?.validate_context(cx)?;
  if !self.core.as_ref().is_some_and(|core|Arc::ptr_eq(&core.module,module))||cx.original_source_epoch().is_none_or(|epoch|epoch<self.epoch){return Err(refused("Core opening requires its same module and original causal source").into())}Ok(())
 }
 pub fn validate_original_context(&self,module:&Arc<CoreModule>,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.validate(module,cx)}
 pub fn resources_ready(&self)->bool{self.phase==Phase::Ready}
 pub fn resources(&self)->Option<&CoreInstance>{self.resources_ready().then(||self.core.as_ref()).flatten()}
 pub fn advance(&mut self,module:&Arc<CoreModule>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let _ledger=self.ledger_scope();
  self.advance_from(module,None,cx)
 }
 /// 🧾️ Copies the exact sealed preparation in paid slices into this actor's own instance.
 pub fn advance_prepared(&mut self,module:&Arc<CoreModule>,prepared:&PreparedCoreInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let _ledger=self.ledger_scope();
  self.advance_from(module,Some(prepared),cx)
 }
 fn advance_from(&mut self,module:&Arc<CoreModule>,prepared:Option<&PreparedCoreInstance>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.prepared!=prepared.map(|image|image.identity)||prepared.is_some_and(|image|!image.matches_module(module)){return Err(refused("Core opening cannot change its sealed initialization source").into())}
  self.validate(module,cx)?;if self.phase==Phase::Closing{return Err(refused("closing Core opening cannot allocate another phase").into())}if self.resources_ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let core=self.core.as_mut().unwrap();match self.phase{
   Phase::Memories(index) if prepared.is_some()=>{
    let image=prepared.unwrap();
    if index==image.memories.len(){self.phase=Phase::Tables(0)}
    else if core.memories.len()==index{
     let limits=&module.memories[index];
     if charge(cx,size_of::<MemoryState>(),0,0)?{core.memories.push(MemoryState{bytes:Vec::new(),maximum_pages:limits.maximum_pages,memory64:limits.memory64});}
    }else if copy_image(&mut core.memories[index].bytes,&image.memories[index].bytes,cx)?{self.phase=Phase::Memories(index+1)}
   },
   Phase::Tables(index) if prepared.is_some()=>{
    let image=prepared.unwrap();
    if index==image.tables.len(){self.phase=Phase::Globals(0)}
    else if core.tables.len()==index{
     let limits=&module.tables[index];
     if charge(cx,size_of::<TableState>(),0,0)?{core.tables.push(TableState{values:Vec::new(),element:limits.element,maximum:limits.maximum});}
    }else if copy_image(&mut core.tables[index].values,&image.tables[index].values,cx)?{self.phase=Phase::Tables(index+1)}
   },
   Phase::Data(index,_) if prepared.is_some()=>{
    let image=prepared.unwrap();
    if index==image.data.len(){self.phase=Phase::Elements(0,0)}
    else if copy_image_segment(&mut core.data,index,image.data[index].as_deref(),cx)?{self.phase=Phase::Data(index+1,0)}
   },
   Phase::Elements(index,_) if prepared.is_some()=>{
    let image=prepared.unwrap();
    if index==image.elements.len(){self.phase=Phase::Ready}
    else if copy_image_segment(&mut core.elements,index,image.elements[index].as_deref(),cx)?{self.phase=Phase::Elements(index+1,0)}
   },
   Phase::Vectors(index)=>{let done=match index{0=>reserve(&mut core.memories,module.memories.len(),cx)?,1=>reserve(&mut core.tables,module.tables.len(),cx)?,2=>reserve(&mut core.globals,module.globals.len(),cx)?,3=>reserve(&mut core.data,module.data.len(),cx)?,_=>reserve(&mut core.elements,module.elements.len(),cx)?};if done{self.phase=if index==4{Phase::Memories(0)}else{Phase::Vectors(index+1)}}}
   Phase::Memories(index)=>{if index==module.memories.len(){self.phase=Phase::Tables(0)}else{let limits=&module.memories[index];let length=usize::try_from(limits.minimum_pages).ok().and_then(|pages|pages.checked_mul(WASM_PAGE_BYTES)).ok_or_else(||refused("Core memory size exceeds original host storage"))?;
    if core.memories.len()==index{if charge(cx,size_of::<MemoryState>(),0,0)?{core.memories.push(MemoryState{bytes:Vec::new(),maximum_pages:limits.maximum_pages,memory64:limits.memory64});}}
    else{let bytes=&mut core.memories[index].bytes;if bytes.capacity()<length{reserve(bytes,length,cx)?;}else if bytes.len()<length{let count=copy_count(cx,length-bytes.len(),1);if count>0&&charge(cx,count,0,0)?{bytes.resize(bytes.len()+count,0);}}else{self.phase=Phase::Memories(index+1)}}}}
   Phase::Tables(index)=>{if index==module.tables.len(){self.phase=Phase::Globals(0)}else{let limits=&module.tables[index];let length=usize::try_from(limits.minimum).map_err(|_|refused("Core table size exceeds original host storage"))?;
    if core.tables.len()==index{if charge(cx,size_of::<TableState>(),0,0)?{core.tables.push(TableState{values:Vec::new(),element:limits.element,maximum:limits.maximum});}}
    else{let values=&mut core.tables[index].values;if values.capacity()<length{reserve(values,length,cx)?;}else if values.len()<length{let count=copy_count(cx,length-values.len(),size_of::<Option<u64>>());if count>0&&charge(cx,bytes_for::<Option<u64>>(count)?,0,0)?{values.resize(values.len()+count,None);}}else{self.phase=Phase::Tables(index+1)}}}}
   Phase::Globals(index)=>{if index==module.globals.len(){self.phase=Phase::Data(0,0)}else{let declaration=&module.globals[index];let value=if let Some(image)=prepared{image.globals[index].value}else{constant(core,&declaration.initializer)?};if value.value_type()!=declaration.value_type{return Err(refused("Core constant has a different declared type").into())}if charge(cx,size_of::<GlobalState>(),0,0)?{core.globals.push(GlobalState{value,mutable:declaration.mutable});self.phase=Phase::Globals(index+1)}}}
   Phase::Data(index,copied)=>{if index==module.data.len(){self.phase=Phase::Elements(0,0)}else{let segment=&module.data[index];if core.data.len()==index{if charge(cx,size_of::<Option<Vec<u8>>>(),0,0)?{core.data.push((segment.mode==SegmentMode::Passive).then(Vec::new));}}
    else if copied==segment.bytes.len(){self.phase=Phase::Data(index+1,0)}else if segment.mode==SegmentMode::Active{let offset=offset(core,segment.offset.as_ref())?;let target=core.memories.get_mut(segment.memory as usize).ok_or_else(||refused("Core data segment requires its declared memory"))?;let end=offset.checked_add(segment.bytes.len()).filter(|end|*end<=target.bytes.len()).ok_or_else(||refused("Core active data exceeds original memory"))?;let count=copy_count(cx,end-offset-copied,1);if count>0&&charge(cx,count,0,0)?{target.bytes[offset+copied..offset+copied+count].copy_from_slice(&segment.bytes[copied..copied+count]);self.phase=Phase::Data(index,copied+count)}}
    else{let target=core.data[index].as_mut().ok_or_else(||refused("Core passive data has no retained storage"))?;if target.capacity()<segment.bytes.len(){reserve(target,segment.bytes.len(),cx)?;}else{let count=copy_count(cx,segment.bytes.len()-copied,1);if count>0&&charge(cx,count,0,0)?{target.extend_from_slice(&segment.bytes[copied..copied+count]);self.phase=Phase::Data(index,copied+count)}}}}}
   Phase::Elements(index,copied)=>{if index==module.elements.len(){self.phase=Phase::Ready}else{let segment=&module.elements[index];if core.elements.len()==index{if charge(cx,size_of::<Option<Vec<Option<u32>>>>(),0,0)?{core.elements.push((segment.mode==SegmentMode::Passive).then(Vec::new));}}
    else if segment.mode==SegmentMode::Declarative||copied==segment.values.len(){self.phase=Phase::Elements(index+1,0)}else if segment.mode==SegmentMode::Active{let offset=offset(core,segment.offset.as_ref())?;let target=core.tables.get_mut(segment.table as usize).ok_or_else(||refused("Core element segment requires its declared table"))?;offset.checked_add(segment.values.len()).filter(|end|*end<=target.values.len()).ok_or_else(||refused("Core active elements exceed original table"))?;let count=copy_count(cx,segment.values.len()-copied,size_of::<Option<u64>>());if count>0&&charge(cx,bytes_for::<Option<u64>>(count)?,0,0)?{for item in 0..count{target.values[offset+copied+item]=segment.values[copied+item].map(u64::from)}self.phase=Phase::Elements(index,copied+count)}}
    else{let target=core.elements[index].as_mut().ok_or_else(||refused("Core passive elements have no retained storage"))?;if target.capacity()<segment.values.len(){reserve(target,segment.values.len(),cx)?;}else{let count=copy_count(cx,segment.values.len()-copied,size_of::<Option<u32>>());if count>0&&charge(cx,bytes_for::<Option<u32>>(count)?,0,0)?{target.extend_from_slice(&segment.values[copied..copied+count]);self.phase=Phase::Elements(index,copied+count)}}}}}
   Phase::Ready|Phase::Closing=>{}
  }Ok(self.resources_ready())
 }

 pub fn write_original_memory_step(&mut self,module:&Arc<CoreModule>,memory:u32,offset:usize,source:&[u8],copied:&mut usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let _ledger=self.ledger_scope();
  self.write_original_memory(module,memory,offset,source,copied,cx,false)
 }
 /// 🕰️ Only the actual cleanup clock import can receive its bounded prepared reply after cancellation.
 pub fn write_original_cleanup_clock_step(&mut self,module:&Arc<CoreModule>,memory:u32,offset:usize,source:&[u8],copied:&mut usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(module,cx)?;if !cx.original_retirement_active(){return Err(refused("Core cleanup clock requires its original retirement loan").into())}self.validate_retirement_root(module,cx)?;let call=self.original_host_call(module,cx)?.ok_or_else(||refused("Core cleanup clock has no actual host call"))?;let valid=call.module=="semio:framework/pure@1.0.0"&&call.name=="operation-clock-us"&&matches!(call.arguments,[Value::I64(binding),Value::I32(pointer)]if Some(*binding as u64)==cx.original_clock_binding()&&*pointer as u32 as usize==offset)&&matches!(module.export("memory"),Some(ExportKind::Memory(index))if *index==memory)&&source.len()==16;if !valid{return Err(refused("Core cleanup clock reply changes its actual import or destination").into())}self.write_original_memory(module,memory,offset,source,copied,cx,true)
 }
 fn write_original_memory(&mut self,module:&Arc<CoreModule>,memory:u32,offset:usize,source:&[u8],copied:&mut usize,cx:&mut StepContext<'_>,retirement:bool)->Result<bool,OriginalTurnFailure>{
  self.validate(module,cx)?;if !self.resources_ready()||self.phase==Phase::Closing||*copied>source.len(){return Err(refused("Core original input requires ready retained memory and its actual copy cursor").into())}let target=self.core.as_mut().unwrap().memories.get_mut(memory as usize).ok_or_else(||refused("Core original input memory is absent"))?;offset.checked_add(source.len()).filter(|end|*end<=target.bytes.len()).ok_or_else(||refused("Core original input exceeds its admitted memory extent"))?;if *copied==source.len(){return Ok(true)}if cx.is_cancelled()&&!retirement{return Ok(false)}let count=(source.len()-*copied).min(cx.retained_grant().maximum_copy_bytes.saturating_sub(size_of::<usize>()));if count==0||!charge_in(RetainedCategory::HostResults,cx,count+size_of::<usize>(),0,0)?{return Ok(false)}target.bytes[offset+*copied..offset+*copied+count].copy_from_slice(&source[*copied..*copied+count]);*copied+=count;Ok(*copied==source.len())
 }
 pub fn terminal_is_empty(&self)->bool{self.core.is_none()&&self.source.is_none()}
 pub fn close_step(&mut self,module:&Arc<CoreModule>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let _ledger=self.ledger_scope();
  if self.terminal_is_empty(){return Ok(true)}if self.core.is_some(){self.validate(module,cx)?;}self.phase=Phase::Closing;if cx.retirement_should_yield(){return Ok(false)}
  if self.core.as_ref().is_some_and(|core|core.machine.is_some()){self.close_invocation_step(module,cx)?;return Ok(false)}
  if let Some(core)=self.core.as_mut(){
   if release_pool(&mut core.pool,cx)?{return Ok(false)}
   if let Some(memory)=core.memories.last(){let release=bytes_for::<u8>(memory.bytes.capacity())?;if retirement_charge(cx,size_of::<MemoryState>(),0,release)?{core.memories.pop();}return Ok(false)}
   if let Some(table)=core.tables.last(){let release=bytes_for::<Option<u64>>(table.values.capacity())?;if retirement_charge(cx,size_of::<TableState>(),0,release)?{core.tables.pop();}return Ok(false)}
   if let Some(segment)=core.data.last(){let release=segment.as_ref().map_or(0,Vec::capacity);if retirement_charge(cx,size_of::<Option<Vec<u8>>>(),0,release)?{core.data.pop();}return Ok(false)}
   if let Some(segment)=core.elements.last(){let release=bytes_for::<Option<u32>>(segment.as_ref().map_or(0,Vec::capacity))?;if retirement_charge(cx,size_of::<Option<Vec<Option<u32>>>>(),0,release)?{core.elements.pop();}return Ok(false)}
   if release_vector(&mut core.memories,cx)?||release_vector(&mut core.tables,cx)?||release_vector(&mut core.globals,cx)?||release_vector(&mut core.data,cx)?||release_vector(&mut core.elements,cx)?{return Ok(false)}
   if !retirement_charge(cx,size_of::<CoreInstance>(),0,0)?{return Ok(false)}self.core.take();return Ok(false)
  }
  let parent=cx.original_source_parent()?;self.source.as_mut().unwrap().close_step(parent,cx)?;if self.source.as_ref().unwrap().terminal_is_empty(){self.source.take();}Ok(self.terminal_is_empty())
 }
}
#[path="▶️invocation/🦀️.rs"]
mod original_invocation;
pub use original_invocation::{CoreOriginalInvocationCall,CoreOriginalInvocationStatus,CoreOriginalHostCall};

impl Drop for CoreInstanceOpening{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Core resources reached Drop before paid retirement or custody transfer")}}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}
fn bytes_for<T>(count:usize)->Result<usize,ValueError>{count.checked_mul(size_of::<T>()).ok_or_else(||refused("Core resource byte count exceeds original host storage"))}
/// 💳️ One retained item for a real copy, birth or release.
fn charge_in(category:RetainedCategory,cx:&mut StepContext<'_>,copy:usize,capacity:usize,release:usize)->Result<bool,OriginalTurnFailure>{let paid=cx.charge_retained_item(copy,capacity,release)?;if paid{tally(category,copy)}Ok(paid)}
fn charge(cx:&mut StepContext<'_>,copy:usize,capacity:usize,release:usize)->Result<bool,OriginalTurnFailure>{charge_in(RetainedCategory::DataInstantiate,cx,copy,capacity,release)}
/// ⚙️ One work unit, no retained item.
fn pay_work(cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{Ok(cx.charge_work_unit())}
/// 📍️ Retained plus work progress.
fn progress(cx:&StepContext<'_>)->(RetainedCloneProgress,u128){(cx.retained_progress(),cx.original_work_units_spent())}
fn retirement_charge(cx:&mut StepContext<'_>,copy:usize,capacity:usize,release:usize)->Result<bool,OriginalTurnFailure>{let paid=cx.charge_retirement_retained_item(copy,capacity,release)?;if paid{tally(RetainedCategory::Retirement,copy)}Ok(paid)}
fn reserve<T>(vector:&mut Vec<T>,length:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if vector.capacity()>=length{return Ok(true)}if !vector.is_empty()||vector.capacity()!=0{return Err(refused("Core resource capacity must be admitted once from its actual zero owner").into())}let capacity=bytes_for::<T>(length)?;if !charge(cx,size_of::<Vec<T>>(),capacity,0)?{return Ok(false)}*vector=Vec::with_capacity(length);Ok(true)}
fn copy_count(cx:&StepContext<'_>,remaining:usize,width:usize)->usize{remaining.min(cx.retained_grant().maximum_copy_bytes/width)}
fn constant(core:&CoreInstance,value:&ConstExpr)->Result<Value,ValueError>{match value{ConstExpr::Value(value)=>Ok(*value),ConstExpr::Global(index)=>core.globals.get(*index as usize).map(|state|state.value).ok_or_else(||refused("Core constant requires an initialized global")),ConstExpr::RefFunction(index)=>Ok(Value::FuncRef(Some(*index)))}}
fn offset(core:&CoreInstance,value:Option<&ConstExpr>)->Result<usize,ValueError>{match constant(core,value.ok_or_else(||refused("Core active segment requires its original offset"))?)?{Value::I32(offset)=>Ok(offset as u32 as usize),_=>Err(refused("Core active segment offset must be i32"))}}
/// 📏️ Declared work divisor: one work unit per this many guest-internal bytes moved, at least one.
const WORK_BYTES_PER_UNIT:usize=64;
/// 📏️ Declared bound of one guest-internal move between cadence checks.
const WORK_CHUNK_BYTES:usize=1<<20;
/// 📏️ Work units for moving `bytes` inside guest storage.
fn work_units(bytes:usize)->u64{u64::try_from(bytes.div_ceil(WORK_BYTES_PER_UNIT).max(1)).unwrap_or(u64::MAX)}
/// 📏️ The bytes one move may span with the work this turn still holds.
fn work_chunk(cx:&StepContext<'_>,remaining:usize)->usize{remaining.min(WORK_CHUNK_BYTES).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).saturating_mul(WORK_BYTES_PER_UNIT))}
/// 📏️ Pays work proportional to guest-internal bytes moved, capped at the fuel left so a small turn still progresses.
fn pay_work_bytes(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{Ok(cx.charge_work_units(work_units(bytes).min(cx.fuel_remaining().max(1))))}
/// 🏊️ Size class that serves `need` elements, if pooled.
fn class_of(need:usize)->Option<usize>{(need!=0&&need<=1<<(POOL_CLASSES-1)).then(||need.next_power_of_two().trailing_zeros() as usize)}
/// 🏊️ Size class a vector of this capacity returns to.
fn floor_class(capacity:usize)->Option<usize>{(capacity!=0).then(||((usize::BITS-1-capacity.leading_zeros()) as usize).min(POOL_CLASSES-1))}
/// 🏊️ Capacity a newborn vector gets so that it fits its whole class.
fn class_capacity(need:usize)->usize{class_of(need).map_or(need,|class|1<<class)}
/// 🏊️ The smallest class that still holds a pooled vector for `need` elements, one vector of `claimed` already being spoken for: a vector that grew while lent returns to a larger class and must still serve the next small request.
fn pooled<T>(stacks:&[Vec<Vec<T>>],need:usize,claimed:Option<usize>)->Option<usize>{(class_of(need)?..POOL_CLASSES).find(|class|stacks[*class].len()>usize::from(claimed==Some(*class)))}
/// 🏊️ Pops the last pooled vector of the claimed class, or births one of the class capacity.
fn lend<T>(stacks:&mut [Vec<Vec<T>>],need:usize,claimed:Option<usize>)->Vec<T>{claimed.and_then(|class|stacks[class].pop()).unwrap_or_else(||Vec::with_capacity(class_capacity(need)))}
/// 🏊️ Birth bytes; none when pooled.
fn born<T>(need:usize,claimed:Option<usize>)->Result<usize,ValueError>{if need==0||claimed.is_some(){Ok(0)}else{bytes_for::<T>(class_capacity(need))}}
/// 🏊️ What happens to a returned vector: recycled for work, a paid growth of the pool stack first, or released when the class is full.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum PoolMove{Recycle,Grow{old:usize,new:usize,count:usize},Release(usize)}
fn pool_move<T>(capacity:usize,stacks:&[Vec<Vec<T>>])->Result<PoolMove,ValueError>{
 let Some(class)=floor_class(capacity)else{return Ok(PoolMove::Recycle)};let stack=&stacks[class];
 if stack.len()>=POOL_LIMIT{return Ok(PoolMove::Release(bytes_for::<T>(capacity)?))}
 if stack.len()==stack.capacity(){let count=(2*stack.len()).max(4).min(POOL_LIMIT);return Ok(PoolMove::Grow{old:bytes_for::<Vec<T>>(stack.len())?,new:bytes_for::<Vec<T>>(count)?,count})}
 Ok(PoolMove::Recycle)
}
/// 🏊️ Replaces a full pool stack by a larger backing.
fn grow_container<T>(stack:&mut Vec<Vec<T>>,count:usize){let mut grown=Vec::with_capacity(count);grown.append(stack);*stack=grown;}
/// 🏊️ Empties a vector into the last slot of its class.
fn stow<T>(stacks:&mut [Vec<Vec<T>>],vector:&mut Vec<T>){if let Some(class)=floor_class(vector.capacity()){vector.clear();stacks[class].push(std::mem::take(vector))}}
/// ♻️ Recycles a vector for one work unit, grows the pool stack through one paid step, or releases it when its class is full.
fn recycle_vector<T>(vector:&mut Vec<T>,stacks:&mut [Vec<Vec<T>>],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if vector.capacity()==0{return Ok(false)}
 match pool_move(vector.capacity(),stacks)?{
  PoolMove::Recycle=>{if !pay_work(cx)?{return Ok(true)}stow(stacks,vector)}
  PoolMove::Grow{old,new,count}=>{if !charge_in(RetainedCategory::PoolBirth,cx,old,new,old)?{return Ok(true)}grow_container(&mut stacks[floor_class(vector.capacity()).unwrap()],count)}
  PoolMove::Release(_)=>return release_vector(vector,cx)
 }
 Ok(true)
}
/// 🏊️ Releases one pooled vector or stack backing per retirement step.
fn release_stacks<T>(stacks:&mut [Vec<Vec<T>>],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 for stack in stacks{
  if let Some(last)=stack.last_mut(){let released=release_vector(last,cx)?;if last.capacity()==0{stack.pop();}return Ok(released)}
  if stack.capacity()!=0{return release_vector(stack,cx)}
 }
 Ok(false)
}
/// 🏊️ Releases the pool one vector per retirement step.
fn release_pool(pool:&mut FramePool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if release_stacks(&mut pool.locals,cx)?||release_stacks(&mut pool.controls,cx)?{return Ok(true)}
 release_stacks(&mut pool.types,cx)
}
fn release_vector<T>(vector:&mut Vec<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if vector.capacity()==0{return Ok(false)}let release=bytes_for::<T>(vector.capacity())?;if !retirement_charge(cx,size_of::<Vec<T>>(),0,release)?{return Ok(true)}drop(std::mem::take(vector));Ok(true)}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
