//! 🏗️ Original caller phases reserve, initialize and retire Core resource storage before any guest instruction starts.
use super::*;
use semio_framework_job::{OriginalCallerSourceLoan,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::mem::{ManuallyDrop,size_of};

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Vectors(u8),Memories(usize),Tables(usize),Globals(usize),Data(usize,usize),Elements(usize,usize),Ready,Closing}
/// 🧱️ Holds the actual partial instance and initiating source until its resource opening or paid closure completes.
pub struct CoreInstanceOpening{core:ManuallyDrop<Option<CoreInstance>>,source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,epoch:u64,phase:Phase,invocation_complete:bool,returning_frame:bool}
impl CoreInstanceOpening{
 pub fn begin(module:&Arc<CoreModule>,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
  let epoch=cx.original_source_epoch().ok_or_else(||refused("Core opening requires the initiating input epoch"))?;
  if !charge(cx,size_of::<Self>(),0,0)?{return Ok(None)}
  let parent=cx.original_source_parent()?;let Some(source)=parent.admit_loan(cx)?else{return Ok(None)};
  Ok(Some(Self{core:ManuallyDrop::new(Some(CoreInstance{module:Arc::clone(module),memories:Vec::new(),tables:Vec::new(),globals:Vec::new(),data:Vec::new(),elements:Vec::new(),machine:None,next_host_call:1})),source:ManuallyDrop::new(Some(source)),epoch,phase:Phase::Vectors(0),invocation_complete:false,returning_frame:false}))
 }
 fn validate(&self,module:&Arc<CoreModule>,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  self.source.as_ref().ok_or_else(||refused("Core opening source has already returned"))?.validate_context(cx)?;
  if !self.core.as_ref().is_some_and(|core|Arc::ptr_eq(&core.module,module))||cx.original_source_epoch().is_none_or(|epoch|epoch<self.epoch){return Err(refused("Core opening requires its same module and original causal source").into())}Ok(())
 }
 pub fn validate_original_context(&self,module:&Arc<CoreModule>,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{self.validate(module,cx)}
 pub fn resources_ready(&self)->bool{self.phase==Phase::Ready}
 pub fn resources(&self)->Option<&CoreInstance>{self.resources_ready().then(||self.core.as_ref()).flatten()}
 pub fn advance(&mut self,module:&Arc<CoreModule>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(module,cx)?;if self.phase==Phase::Closing{return Err(refused("closing Core opening cannot allocate another phase").into())}if self.resources_ready(){return Ok(true)}if cx.is_cancelled()||cx.should_yield(){return Ok(false)}
  let core=self.core.as_mut().unwrap();match self.phase{
   Phase::Vectors(index)=>{let done=match index{0=>reserve(&mut core.memories,module.memories.len(),cx)?,1=>reserve(&mut core.tables,module.tables.len(),cx)?,2=>reserve(&mut core.globals,module.globals.len(),cx)?,3=>reserve(&mut core.data,module.data.len(),cx)?,_=>reserve(&mut core.elements,module.elements.len(),cx)?};if done{self.phase=if index==4{Phase::Memories(0)}else{Phase::Vectors(index+1)}}}
   Phase::Memories(index)=>{if index==module.memories.len(){self.phase=Phase::Tables(0)}else{let limits=&module.memories[index];let length=usize::try_from(limits.minimum_pages).ok().and_then(|pages|pages.checked_mul(WASM_PAGE_BYTES)).ok_or_else(||refused("Core memory size exceeds original host storage"))?;
    if core.memories.len()==index{if charge(cx,size_of::<MemoryState>(),0,0)?{core.memories.push(MemoryState{bytes:Vec::new(),maximum_pages:limits.maximum_pages,memory64:limits.memory64});}}
    else{let bytes=&mut core.memories[index].bytes;if bytes.capacity()<length{reserve(bytes,length,cx)?;}else if bytes.len()<length{let count=copy_count(cx,length-bytes.len(),1);if count>0&&charge(cx,count,0,0)?{bytes.resize(bytes.len()+count,0);}}else{self.phase=Phase::Memories(index+1)}}}}
   Phase::Tables(index)=>{if index==module.tables.len(){self.phase=Phase::Globals(0)}else{let limits=&module.tables[index];let length=usize::try_from(limits.minimum).map_err(|_|refused("Core table size exceeds original host storage"))?;
    if core.tables.len()==index{if charge(cx,size_of::<TableState>(),0,0)?{core.tables.push(TableState{values:Vec::new(),element:limits.element,maximum:limits.maximum});}}
    else{let values=&mut core.tables[index].values;if values.capacity()<length{reserve(values,length,cx)?;}else if values.len()<length{let count=copy_count(cx,length-values.len(),size_of::<Option<u64>>());if count>0&&charge(cx,bytes_for::<Option<u64>>(count)?,0,0)?{values.resize(values.len()+count,None);}}else{self.phase=Phase::Tables(index+1)}}}}
   Phase::Globals(index)=>{if index==module.globals.len(){self.phase=Phase::Data(0,0)}else{let declaration=&module.globals[index];let value=constant(core,&declaration.initializer)?;if value.value_type()!=declaration.value_type{return Err(refused("Core constant has a different declared type").into())}if charge(cx,size_of::<GlobalState>(),0,0)?{core.globals.push(GlobalState{value,mutable:declaration.mutable});self.phase=Phase::Globals(index+1)}}}
   Phase::Data(index,copied)=>{if index==module.data.len(){self.phase=Phase::Elements(0,0)}else{let segment=&module.data[index];if core.data.len()==index{if charge(cx,size_of::<Option<Vec<u8>>>(),0,0)?{core.data.push((segment.mode==SegmentMode::Passive).then(Vec::new));}}
    else if copied==segment.bytes.len(){self.phase=Phase::Data(index+1,0)}else if segment.mode==SegmentMode::Active{let offset=offset(core,segment.offset.as_ref())?;let target=core.memories.get_mut(segment.memory as usize).ok_or_else(||refused("Core data segment requires its declared memory"))?;let end=offset.checked_add(segment.bytes.len()).filter(|end|*end<=target.bytes.len()).ok_or_else(||refused("Core active data exceeds original memory"))?;let count=copy_count(cx,end-offset-copied,1);if count>0&&charge(cx,count,0,0)?{target.bytes[offset+copied..offset+copied+count].copy_from_slice(&segment.bytes[copied..copied+count]);self.phase=Phase::Data(index,copied+count)}}
    else{let target=core.data[index].as_mut().ok_or_else(||refused("Core passive data has no retained storage"))?;if target.capacity()<segment.bytes.len(){reserve(target,segment.bytes.len(),cx)?;}else{let count=copy_count(cx,segment.bytes.len()-copied,1);if count>0&&charge(cx,count,0,0)?{target.extend_from_slice(&segment.bytes[copied..copied+count]);self.phase=Phase::Data(index,copied+count)}}}}}
   Phase::Elements(index,copied)=>{if index==module.elements.len(){self.phase=Phase::Ready}else{let segment=&module.elements[index];if core.elements.len()==index{if charge(cx,size_of::<Option<Vec<Option<u32>>>>(),0,0)?{core.elements.push((segment.mode==SegmentMode::Passive).then(Vec::new));}}
    else if segment.mode==SegmentMode::Declarative||copied==segment.values.len(){self.phase=Phase::Elements(index+1,0)}else if segment.mode==SegmentMode::Active{let offset=offset(core,segment.offset.as_ref())?;let target=core.tables.get_mut(segment.table as usize).ok_or_else(||refused("Core element segment requires its declared table"))?;offset.checked_add(segment.values.len()).filter(|end|*end<=target.values.len()).ok_or_else(||refused("Core active elements exceed original table"))?;let count=copy_count(cx,segment.values.len()-copied,size_of::<Option<u64>>());if count>0&&charge(cx,bytes_for::<Option<u64>>(count)?,0,0)?{for item in 0..count{target.values[offset+copied+item]=segment.values[copied+item].map(u64::from)}self.phase=Phase::Elements(index,copied+count)}}
    else{let target=core.elements[index].as_mut().ok_or_else(||refused("Core passive elements have no retained storage"))?;if target.capacity()<segment.values.len(){reserve(target,segment.values.len(),cx)?;}else{let count=copy_count(cx,segment.values.len()-copied,size_of::<Option<u32>>());if count>0&&charge(cx,bytes_for::<Option<u32>>(count)?,0,0)?{target.extend_from_slice(&segment.values[copied..copied+count]);self.phase=Phase::Elements(index,copied+count)}}}}}
   Phase::Ready|Phase::Closing=>{}
  }Ok(self.resources_ready())
 }

 pub fn write_original_memory_step(&mut self,module:&Arc<CoreModule>,memory:u32,offset:usize,source:&[u8],copied:&mut usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(module,cx)?;if !self.resources_ready()||self.phase==Phase::Closing||*copied>source.len(){return Err(refused("Core original input requires ready retained memory and its actual copy cursor").into())}let target=self.core.as_mut().unwrap().memories.get_mut(memory as usize).ok_or_else(||refused("Core original input memory is absent"))?;offset.checked_add(source.len()).filter(|end|*end<=target.bytes.len()).ok_or_else(||refused("Core original input exceeds its admitted memory extent"))?;if *copied==source.len(){return Ok(true)}if cx.is_cancelled(){return Ok(false)}let count=(source.len()-*copied).min(cx.retained_grant().maximum_copy_bytes.saturating_sub(size_of::<usize>()));if count==0||!charge(cx,count+size_of::<usize>(),0,0)?{return Ok(false)}target.bytes[offset+*copied..offset+*copied+count].copy_from_slice(&source[*copied..*copied+count]);*copied+=count;Ok(*copied==source.len())
 }
 pub fn terminal_is_empty(&self)->bool{self.core.is_none()&&self.source.is_none()}
 pub fn close_step(&mut self,module:&Arc<CoreModule>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(true)}if self.core.is_some(){self.validate(module,cx)?;}self.phase=Phase::Closing;if cx.should_yield(){return Ok(false)}
  if self.core.as_ref().is_some_and(|core|core.machine.is_some()){self.close_invocation_step(module,cx)?;return Ok(false)}
  if let Some(core)=self.core.as_mut(){
   if let Some(memory)=core.memories.last(){let release=bytes_for::<u8>(memory.bytes.capacity())?;if charge(cx,size_of::<MemoryState>(),0,release)?{core.memories.pop();}return Ok(false)}
   if let Some(table)=core.tables.last(){let release=bytes_for::<Option<u64>>(table.values.capacity())?;if charge(cx,size_of::<TableState>(),0,release)?{core.tables.pop();}return Ok(false)}
   if let Some(segment)=core.data.last(){let release=segment.as_ref().map_or(0,Vec::capacity);if charge(cx,size_of::<Option<Vec<u8>>>(),0,release)?{core.data.pop();}return Ok(false)}
   if let Some(segment)=core.elements.last(){let release=bytes_for::<Option<u32>>(segment.as_ref().map_or(0,Vec::capacity))?;if charge(cx,size_of::<Option<Vec<Option<u32>>>>(),0,release)?{core.elements.pop();}return Ok(false)}
   if !core.globals.is_empty(){if charge(cx,size_of::<GlobalState>(),0,0)?{core.globals.pop();}return Ok(false)}
   if release_vector(&mut core.memories,cx)?||release_vector(&mut core.tables,cx)?||release_vector(&mut core.globals,cx)?||release_vector(&mut core.data,cx)?||release_vector(&mut core.elements,cx)?{return Ok(false)}
   if !charge(cx,size_of::<CoreInstance>(),0,0)?{return Ok(false)}self.core.take();return Ok(false)
  }
  let parent=cx.original_source_parent()?;self.source.as_mut().unwrap().close_step(parent,cx)?;if self.source.as_ref().unwrap().terminal_is_empty(){self.source.take();}Ok(self.terminal_is_empty())
 }
}
#[path="▶️invocation/🦀️.rs"]
mod original_invocation;
pub use original_invocation::{CoreOriginalInvocationCall,CoreOriginalInvocationStatus};

impl Drop for CoreInstanceOpening{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Core resources reached Drop before paid retirement or custody transfer")}}
fn refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}
fn bytes_for<T>(count:usize)->Result<usize,ValueError>{count.checked_mul(size_of::<T>()).ok_or_else(||refused("Core resource byte count exceeds original host storage"))}
fn charge(cx:&mut StepContext<'_>,copy:usize,capacity:usize,release:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy||grant.maximum_capacity_bytes<capacity||grant.maximum_release_bytes<release{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,released_bytes:release})?;cx.consume_fuel(1);Ok(true)}
fn reserve<T>(vector:&mut Vec<T>,length:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if vector.capacity()>=length{return Ok(true)}if !vector.is_empty()||vector.capacity()!=0{return Err(refused("Core resource capacity must be admitted once from its actual zero owner").into())}let capacity=bytes_for::<T>(length)?;if !charge(cx,size_of::<Vec<T>>(),capacity,0)?{return Ok(false)}*vector=Vec::with_capacity(length);Ok(true)}
fn copy_count(cx:&StepContext<'_>,remaining:usize,width:usize)->usize{remaining.min(cx.retained_grant().maximum_copy_bytes/width)}
fn constant(core:&CoreInstance,value:&ConstExpr)->Result<Value,ValueError>{match value{ConstExpr::Value(value)=>Ok(*value),ConstExpr::Global(index)=>core.globals.get(*index as usize).map(|state|state.value).ok_or_else(||refused("Core constant requires an initialized global")),ConstExpr::RefFunction(index)=>Ok(Value::FuncRef(Some(*index)))}}
fn offset(core:&CoreInstance,value:Option<&ConstExpr>)->Result<usize,ValueError>{match constant(core,value.ok_or_else(||refused("Core active segment requires its original offset"))?)?{Value::I32(offset)=>Ok(offset as u32 as usize),_=>Err(refused("Core active segment offset must be i32"))}}
fn release_vector<T>(vector:&mut Vec<T>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if vector.capacity()==0{return Ok(false)}let release=bytes_for::<T>(vector.capacity())?;if !charge(cx,size_of::<Vec<T>>(),0,release)?{return Ok(true)}drop(std::mem::take(vector));Ok(true)}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
