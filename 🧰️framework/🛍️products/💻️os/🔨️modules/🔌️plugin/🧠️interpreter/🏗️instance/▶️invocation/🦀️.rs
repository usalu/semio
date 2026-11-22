//! ▶️ Each admitted scalar instruction preserves the original execution cap separately from physical work and storage.
use super::*;
#[path="🧭️control/🦀️.rs"]
mod control;
pub(super) use control::OriginalControlPlan;
#[path="💾️memory/🦀️.rs"]
mod memory;
pub(super) use memory::OriginalMemoryWork;
#[path="📞️host/🦀️.rs"]
mod host;
pub(super) use host::OriginalHostCall;
pub use host::CoreOriginalHostCall;
#[cfg(any(test, feature = "testkit"))]
#[path="📊️trace/🦀️.rs"]
mod trace;
#[cfg(any(test, feature = "testkit"))]
use trace::{Category, Slice};

macro_rules! trace_work {
 ($trace:expr, $category:ident, $cx:expr, $fuel:expr) => {
  #[cfg(any(test, feature = "testkit"))]
  if let Some(trace) = $trace.as_mut() {
   trace.mark(Category::$category, $cx, $fuel);
  }
 };
}

use semio_framework_actor::{OriginalGuestExecutionBudget,OriginalGuestExecutionReceipt,OriginalJobBudget};
use semio_framework_value::RetirementDemand;
static TRACE_CORE_FRAME:std::sync::OnceLock<bool>=std::sync::OnceLock::new();
thread_local!{static CORE_FRAME_STEPS:std::cell::Cell<u64>=const{std::cell::Cell::new(0)};}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CoreOriginalInvocationStatus{Pending,Complete,Host,Cancelled,Deadline}
pub struct CoreOriginalInvocationCall{pub result:Result<CoreOriginalInvocationStatus,OriginalTurnFailure>,pub execution:OriginalGuestExecutionReceipt}
pub(super) fn retirement_export(module:&CoreModule,function:u32)->bool{
 let mut allowed=false;
 for export in crate::OwnedSemioExport::ALL{if matches!(module.export(export.core_name()),Some(ExportKind::Function(index))if *index==function){if export.is_original_retirement(){allowed=true}else{return false}}}
 allowed
}
impl CoreInstanceOpening{
 /// 🧾️ Opens a canceled admission only through the embedding that verified the retained input receipt and handback.
 pub fn begin_input_retirement_step(&mut self,module:&Arc<CoreModule>,authority:&CoreInputRetirementAuthority,received_epoch:u64,arguments:&[Value],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(module,cx)?;
  if self.input_retirement_authority!=Some(authority.0)||!cx.original_retirement_active(){return Err(refused("Core input retirement requires its opening embedding and original retirement loan").into())}
  let export=OwnedSemioExport::AdmitActorInput.core_name();
  let Some(ExportKind::Function(function))=module.export(export)else{return Err(refused("Core input retirement requires its actual admission export").into())};
  if module.exports().any(|(name,kind)|name!=export&&matches!(kind,ExportKind::Function(index)if index==function)){return Err(refused("Core input retirement requires its unaliased admission export").into())}
  let [Value::I32(_),Value::I32(length),words @ ..]=arguments else{return Err(refused("Core input retirement requires its exact wire arguments").into())};
  if *length<0||words.len()!=OriginalJobBudget::SCALAR_ARGUMENT_COUNT{return Err(refused("Core input retirement requires its exact budget extent").into())}
  let mut scalars=[0;OriginalJobBudget::SCALAR_ARGUMENT_COUNT];
  for (word,scalar) in words.iter().zip(&mut scalars){
   let Value::I64(word)=word else{return Err(refused("Core input retirement requires its actual scalar budget").into())};
   *scalar=*word as u64;
  }
  let budget=OriginalJobBudget::from_scalar_arguments(scalars)?;
  let input=budget.input;
  let epoch=cx.original_source_epoch().ok_or_else(||refused("Core input retirement requires its actual initiating epoch"))?;
  if !budget.cancelled||received_epoch==0||received_epoch>=input.epoch||input.epoch>epoch
   ||(input.operation,input.generation)!=(cx.operation().0,cx.generation().0)
   ||Some(input.work.clock_binding)!=cx.original_clock_binding()||input.work.absolute_deadline_us!=cx.original_absolute_deadline_us(){
   return Err(refused("Core input retirement changed its canceled budget or original Source").into())
  }
  self.begin_function(module,*function,arguments,Some(epoch),cx)
 }
 pub(super) fn validate_retirement_root(&self,module:&CoreModule,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if !cx.original_retirement_active(){
   if self.input_retirement_epoch.is_some(){return Err(refused("Core input retirement cannot leave its original retirement loan").into())}
   return Ok(())
  }
  let root=self.core.as_ref().and_then(|core|core.machine.as_ref()).and_then(|machine|machine.frames.first()).ok_or_else(||refused("Core retirement requires its actual cleanup frame"))?;
  let authorized=if let Some(epoch)=self.input_retirement_epoch{
   cx.original_source_epoch().is_some_and(|current|current>=epoch)
    &&matches!(module.export(OwnedSemioExport::AdmitActorInput.core_name()),Some(ExportKind::Function(function))if *function==root.function)
  }else{
   retirement_export(module,root.function)
  };
  if !authorized{return Err(refused("Core retirement cannot resume a semantic export").into())}
  Ok(())
 }
 pub fn begin_function_step(&mut self,module:&Arc<CoreModule>,function:u32,arguments:&[Value],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.begin_function(module,function,arguments,None,cx)
 }
 fn begin_function(&mut self,module:&Arc<CoreModule>,function:u32,arguments:&[Value],input_retirement_epoch:Option<u64>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(module,cx)?;if !self.resources_ready(){return Err(refused("Core invocation requires its fully initialized original resources").into())}if self.core.as_ref().unwrap().machine.is_some(){return Err(refused("Core invocation must retire its preceding machine before admission").into())}if cx.original_retirement_active()&&input_retirement_epoch.is_none()&&!retirement_export(module,function){return Err(refused("Core retirement permits only unaliased original cleanup exports").into())}if (cx.is_cancelled()&&!cx.original_retirement_active())||cx.should_yield(){return Ok(false)}
  let (type_index,locals,body)=match module.functions.get(function as usize){Some(FunctionDecl::Defined{type_index,locals,body,..})=>(*type_index,locals,body),_=>return Err(refused("Core opening requires an owned defined function").into())};let signature=module.types.get(type_index as usize).ok_or_else(||refused("Core opening function type is absent"))?;if arguments.len()!=signature.parameters.len()||arguments.iter().zip(&signature.parameters).any(|(value,ty)|value.value_type()!=*ty){return Err(refused("Core opening arguments differ from their original function type").into())}let local_count=arguments.len().checked_add(locals.len()).ok_or_else(||refused("Core opening local count overflow"))?;let results=bytes_for::<ValueType>(signature.results.len())?;let local_bytes=bytes_for::<Value>(local_count)?;let capacity=size_of::<Frame>().checked_add(size_of::<ControlFrame>()).and_then(|n|n.checked_add(local_bytes)).and_then(|n|n.checked_add(results.checked_mul(2)?)).ok_or_else(||refused("Core machine storage size overflow"))?;let copy=capacity.checked_add(size_of::<Machine>()+if input_retirement_epoch.is_some(){size_of::<Option<u64>>()}else{0}).ok_or_else(||refused("Core machine copy size overflow"))?;let end_pc=body.len().checked_sub(1).ok_or_else(||refused("Core function has no terminating instruction"))?;
  #[cfg(any(test, feature = "testkit"))]
  let trace=Slice::begin(cx,Category::FrameBirth);
  let charged=charge(cx,copy,capacity,0);
  #[cfg(any(test, feature = "testkit"))]
  if let Some(trace)=trace{trace.finish(module,function,cx,0,matches!(&charged,Ok(false)));}
  if !charged?{return Ok(false)}
  #[cfg(any(test, feature = "testkit"))]
  trace::input_witness(module,function,arguments,self.core.as_ref().unwrap());
  let mut local_values=Vec::with_capacity(local_count);local_values.extend_from_slice(arguments);local_values.extend(locals.iter().copied().map(ValueType::zero));let mut branch_types=Vec::with_capacity(signature.results.len());branch_types.extend_from_slice(&signature.results);let mut result_types=Vec::with_capacity(signature.results.len());result_types.extend_from_slice(&signature.results);let mut controls=Vec::with_capacity(1);controls.push(ControlFrame{kind:ControlKind::Function,start_pc:0,end_pc,stack_height:0,branch_types,result_types});let mut frames=Vec::with_capacity(1);frames.push(Frame{function,pc:0,locals:local_values,stack_base:0,controls});self.core.as_mut().unwrap().machine=Some(Machine{values:Vec::new(),frames,pending_host:None});self.invocation_complete=false;self.returning_frame=false;self.input_retirement_epoch=input_retirement_epoch;Ok(true)
 }
 pub fn invocation_values(&self)->Option<&[Value]>{self.invocation_complete.then(||self.core.as_ref()?.machine.as_ref().map(|machine|machine.values.as_slice())).flatten()}
 /// 🧮️ Quotes the next atomic retirement of the actual returned callee.
 pub fn returned_frame_demand(&self,module:&Arc<CoreModule>,cx:&StepContext<'_>)->Result<Option<RetirementDemand>,OriginalTurnFailure>{
  self.validate(module,cx)?;
  if !self.returning_frame{return Ok(None)}
  let machine=self.core.as_ref().and_then(|core|core.machine.as_ref()).ok_or_else(||refused("Core returned callee has no actual machine"))?;
  Ok(Some(returned_frame_step(returned_callee(machine)?)?.1))
 }
 pub fn advance_scalar(&mut self,module:&Arc<CoreModule>,execution:OriginalGuestExecutionBudget,cx:&mut StepContext<'_>)->Result<CoreOriginalInvocationCall,OriginalTurnFailure>{
  self.validate(module,cx)?;self.validate_retirement_root(module,cx)?;let cleanup=cx.original_retirement_active();let input=execution.input;if input.operation!=cx.operation().0||input.generation!=cx.generation().0||Some(input.epoch)!=cx.original_source_epoch()||input.grant!=cx.retained_grant(){return Err(refused("Core execution requires the actual incoming input and unchanged physical ceiling").into())}execution.validate_original(input,cx.deadline_us())?;let mut used=0;
  #[cfg(any(test, feature = "testkit"))]
  let mut trace=Slice::begin(cx,Category::Scalar);
  let result=(||{if self.invocation_complete{return Ok(CoreOriginalInvocationStatus::Complete)}if self.host_call.is_some(){return Ok(CoreOriginalInvocationStatus::Host)}while used<execution.fuel{if cx.is_cancelled()&&!cleanup{return Ok(CoreOriginalInvocationStatus::Cancelled)}if cx.deadline_exceeded(){return Ok(CoreOriginalInvocationStatus::Deadline)}if cx.should_yield(){return Ok(CoreOriginalInvocationStatus::Pending)}let core=self.core.as_mut().unwrap();
   if let Some(work)=self.memory_work.as_mut(){
    trace_work!(trace,Memory,cx,used);
    let before=cx.retained_progress();
    if work.advance(core,cx)?{
     self.memory_work=None;
     used+=1;
     continue
    }
    if cx.retained_progress()!=before{continue}
    return Ok(CoreOriginalInvocationStatus::Pending)
   }
   let machine=core.machine.as_mut().ok_or_else(||refused("Core scalar execution has no admitted machine"))?;if let Some(plan)=self.control.as_mut(){trace_work!(trace,Control,cx,used);let before=cx.retained_progress();match plan.advance(machine,module,cx)?{control::ControlProgress::Pending if cx.retained_progress()!=before=>continue,control::ControlProgress::Pending=>return Ok(CoreOriginalInvocationStatus::Pending),control::ControlProgress::Executed=>{used+=1;continue},control::ControlProgress::Complete{parameter_return}=>{self.control=None;if parameter_return{if machine.frames.len()==1{self.invocation_complete=true;return Ok(CoreOriginalInvocationStatus::Complete)}self.returning_frame=true;}continue}}}if self.returning_frame{trace_work!(trace,FrameReturn,cx,used);let before=cx.retained_progress();if close_returned_frame(machine,cx)?{self.returning_frame=false;continue;}if cx.retained_progress()!=before{continue;}return Ok(CoreOriginalInvocationStatus::Pending)}let frame=machine.frames.last().ok_or_else(||refused("Core scalar execution has no retained frame"))?;let body=match module.functions.get(frame.function as usize){Some(FunctionDecl::Defined{body,..})=>body,_=>return Err(refused("Core scalar execution has no original body").into())};let mut pc=frame.pc;let opcode=*body.get(pc).ok_or_else(||refused("Core scalar instruction exceeds its original body"))?;pc+=1;
   if matches!(opcode,0x02..=0x05|0x0b..=0x0f|0x1b|0x1c){trace_work!(trace,Control,cx,used);if let Some(admitted)=control::admit(module,machine,opcode,pc,cx)?{self.control=admitted.plan;used+=u64::from(admitted.executed);continue}else{return Ok(CoreOriginalInvocationStatus::Pending)}}
   if matches!(opcode,0x40|0xfc){trace_work!(trace,Memory,cx,used);if let Some(admitted)=memory::admit(core,opcode,body,pc,cx)?{self.memory_work=admitted.work;used+=u64::from(admitted.executed);continue}else{return Ok(CoreOriginalInvocationStatus::Pending)}}
   let action=match opcode{0x01=>Action::Nop,0x10=>{let function=unsigned(body,&mut pc)?;Action::Call(u32::try_from(function).map_err(|_|refused("Core original call function exceeds its declared width"))?,false)},0x11=>{let expected=unsigned(body,&mut pc)?;let table=unsigned(body,&mut pc)?;let selector=match machine.values.last(){Some(Value::I32(value))=>*value as u32 as usize,_=>return Err(refused("Core original indirect call requires its i32 selector").into())};let table=core.tables.get(table).filter(|table|table.element==ValueType::FuncRef).ok_or_else(||refused("Core original indirect call requires its actual function table"))?;let function=table.values.get(selector).copied().flatten().and_then(|value|u32::try_from(value).ok()).ok_or_else(||refused("Core original indirect call targets an absent function"))?;let expected=module.types.get(expected).ok_or_else(||refused("Core original indirect call expected type is absent"))?;let actual=module.functions.get(function as usize).and_then(|function|module.types.get(function.type_index()as usize)).ok_or_else(||refused("Core original indirect call actual type is absent"))?;if expected!=actual{return Err(refused("Core original indirect call changes its declared function type").into())}Action::Call(function,true)},0x1a=>{if machine.values.is_empty(){return Err(refused("Core scalar drop requires its original operand").into())}Action::Drop},0x20=>{let index=unsigned(body,&mut pc)?;Action::Push(*frame.locals.get(index).ok_or_else(||refused("Core scalar local is absent"))?)},0x21|0x22=>{let index=unsigned(body,&mut pc)?;let target=frame.locals.get(index).ok_or_else(||refused("Core scalar local is absent"))?;let value=*machine.values.last().ok_or_else(||refused("Core scalar local assignment has no original operand"))?;if target.value_type()!=value.value_type(){return Err(refused("Core scalar local assignment changes its type").into())}Action::Local(index,value,opcode==0x21)},0x23=>{let index=unsigned(body,&mut pc)?;Action::Push(core.globals.get(index).ok_or_else(||refused("Core scalar global is absent"))?.value)},0x24=>{let index=unsigned(body,&mut pc)?;let target=core.globals.get(index).ok_or_else(||refused("Core scalar global is absent"))?;let value=*machine.values.last().ok_or_else(||refused("Core scalar global assignment has no original operand"))?;if !target.mutable||target.value.value_type()!=value.value_type(){return Err(refused("Core scalar global assignment changes immutable original storage").into())}Action::Global(index,value)},0x28..=0x3e=>{let alignment=unsigned(body,&mut pc)?;if alignment>=64{return Err(refused("Core original memory plan requires its declared default memory").into())}let offset=unsigned64(body,&mut pc)?;let memory=core.memories.first().ok_or_else(||refused("Core original memory instruction has no retained memory"))?;let(address,width)=plan_memory_address(memory,machine,opcode,offset)?;Action::Memory(opcode,address,width)},0x3f=>{let index=unsigned(body,&mut pc)?;let memory=core.memories.get(index).ok_or_else(||refused("Core original memory size index is absent"))?;let pages=memory.bytes.len()/WASM_PAGE_BYTES;Action::Push(if memory.memory64{Value::I64(pages as i64)}else{Value::I32(pages as i32)})},0x41=>Action::Push(Value::I32(signed(body,&mut pc,32)?as i32)),0x42=>Action::Push(Value::I64(signed(body,&mut pc,64)?)),0x43=>Action::Push(Value::F32(u32::from_le_bytes(fixed::<4>(body,&mut pc)?))),0x44=>Action::Push(Value::F64(u64::from_le_bytes(fixed::<8>(body,&mut pc)?))),0x45..=0xc4=>{let count=numeric_arity(opcode).map_err(|_|refused("Core numeric instruction has no declared scalar shape"))?;let start=machine.values.len().checked_sub(count).ok_or_else(||refused("Core numeric instruction has fewer original operands than declared"))?;let value=execute_numeric_fixed(opcode,&machine.values[start..]).map_err(|_|refused("Core numeric instruction refused its actual original operands"))?;Action::Numeric(value,count)},_=>return Err(refused("Core instruction has no admitted original physical execution plan").into())};
   if let Action::Memory(opcode,address,width)=action{trace_work!(trace,Memory,cx,used);if !admit_memory_instruction(&mut core.memories[0],machine,opcode,address,width,cx)?{return Ok(CoreOriginalInvocationStatus::Pending)}used+=1;machine.frames.last_mut().unwrap().pc=pc;continue}if let Action::Call(function,selector)=action{if matches!(module.functions.get(function as usize),Some(FunctionDecl::Import{..})){trace_work!(trace,HostCall,cx,used);if let Some(call)=host::admit(module,function,machine,selector,pc,cx)?{self.host_call=Some(call);used+=1;return Ok(CoreOriginalInvocationStatus::Host)}return Ok(CoreOriginalInvocationStatus::Pending)}trace_work!(trace,FrameBirth,cx,used);if !admit_defined_frame(module,function,machine,selector,cx)?{return Ok(CoreOriginalInvocationStatus::Pending)}used+=1;let caller=machine.frames.len()-2;machine.frames[caller].pc=pc;continue}if opcode==0x3f{trace_work!(trace,Memory,cx,used);}else{trace_work!(trace,Scalar,cx,used);}if matches!(action,Action::Push(_))&&machine.values.len()==machine.values.capacity(){if !grow_values(&mut machine.values,cx)?{return Ok(CoreOriginalInvocationStatus::Pending)}}if !charge(cx,size_of::<Value>()+size_of::<usize>(),0,0)?{return Ok(CoreOriginalInvocationStatus::Pending)}used+=1;match action{Action::Nop=>{},Action::Drop=>{machine.values.pop();},Action::Push(value)=>machine.values.push(value),Action::Local(index,value,pop)=>{machine.frames.last_mut().unwrap().locals[index]=value;if pop{machine.values.pop();}},Action::Global(index,value)=>{core.globals[index].value=value;machine.values.pop();},Action::Numeric(value,count)=>{machine.values.truncate(machine.values.len()-count);machine.values.push(value);},Action::Call(_,_)|Action::Memory(_,_,_)=>unreachable!()}machine.frames.last_mut().unwrap().pc=pc;
  }Ok(CoreOriginalInvocationStatus::Pending)})();
  #[cfg(any(test, feature = "testkit"))]
  if let Some(trace)=trace{
   if let Some(function)=self.core.as_ref().and_then(|core|core.machine.as_ref()).and_then(|machine|machine.frames.first()).map(|frame|frame.function){
    if self.returning_frame&&matches!(&result,Ok(CoreOriginalInvocationStatus::Pending))&&trace.zero_progress(cx,used){
     let machine=self.core.as_ref().unwrap().machine.as_ref().unwrap();
     trace::pending_frame_return(machine,cx,execution.fuel-used);
    }
    trace.finish(module,function,cx,used,matches!(&result,Ok(CoreOriginalInvocationStatus::Pending)));
   }
  }
  if !cleanup&&*TRACE_CORE_FRAME.get_or_init(||std::env::var_os("RUST_BACKTRACE").is_some()){
   CORE_FRAME_STEPS.with(|steps|{
    let step=steps.get().saturating_add(1);steps.set(step);
    if step.is_power_of_two(){
     if let Some(machine)=self.core.as_ref().and_then(|core|core.machine.as_ref()){
      let leaf: [Option<(u32,usize)>;4]=std::array::from_fn(|index|machine.frames.iter().rev().nth(index).map(|frame|(frame.function,frame.pc)));
      eprintln!("[DEBUG] Core scalar: step {step}, execution epoch {}, root function {:?}, frame depth {}, leaf functions and byte offsets {leaf:?}, VM fuel {used}, status {result:?}",input.epoch,machine.frames.first().map(|frame|frame.function),machine.frames.len());
     }
    }
   });
  }
  Ok(CoreOriginalInvocationCall{result,execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel:Some(used),submitted:true}})
 }
 pub fn close_invocation_step(&mut self,module:&Arc<CoreModule>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  self.validate(module,cx)?;
  #[cfg(any(test, feature = "testkit"))]
  if trace::ordinary_flush_enabled(cx){
   if let Some(function)=self.core.as_ref().and_then(|core|core.machine.as_ref()).and_then(|machine|machine.frames.first()).map(|frame|frame.function){
    trace::flush_ordinary(module,function,cx);
   }
  }
  if self.host_call.is_some(){if !retirement_charge(cx,size_of::<OriginalHostCall>(),0,0)?{return Ok(false)}self.host_call=None;}if let Some(work)=self.memory_work.as_mut(){if !work.close_step(cx)?{return Ok(false)}self.memory_work=None;}if self.control.is_some(){if !retirement_charge(cx,size_of::<OriginalControlPlan>(),0,0)?{return Ok(false)}self.control=None;}let Some(machine)=self.core.as_mut().unwrap().machine.as_mut()else{return Ok(true)};if machine.pending_host.is_some(){return Err(refused("Core scalar machine cannot discard a live original host call").into())}if let Some(frame)=machine.frames.last_mut(){if let Some(control)=frame.controls.last_mut(){if release_vector(&mut control.branch_types,cx)?||release_vector(&mut control.result_types,cx)?{return Ok(false)}if retirement_charge(cx,size_of::<ControlFrame>(),0,0)?{frame.controls.pop();}return Ok(false)}if release_vector(&mut frame.controls,cx)?||release_vector(&mut frame.locals,cx)?{return Ok(false)}if retirement_charge(cx,size_of::<Frame>(),0,0)?{machine.frames.pop();}return Ok(false)}if release_vector(&mut machine.frames,cx)?||release_vector(&mut machine.values,cx)?{return Ok(false)}if !retirement_charge(cx,size_of::<Machine>()+size_of::<bool>()+if self.input_retirement_epoch.is_some(){size_of::<Option<u64>>()}else{0},0,0)?{return Ok(false)}self.core.as_mut().unwrap().machine.take();self.invocation_complete=false;self.returning_frame=false;self.input_retirement_epoch=None;Ok(true)
 }
}
enum Action{Nop,Call(u32,bool),Memory(u8,usize,usize),Drop,Push(Value),Local(usize,Value,bool),Global(usize,Value),Numeric(Value,usize)}
fn unsigned(body:&[u8],pc:&mut usize)->Result<usize,ValueError>{let mut value=0usize;for shift in (0..35).step_by(7){let byte=*body.get(*pc).ok_or_else(||refused("Core unsigned operand exceeds original bytecode"))?;*pc+=1;value|=usize::from(byte&127).checked_shl(shift).ok_or_else(||refused("Core unsigned operand exceeds original host width"))?;if byte&128==0{return Ok(value)}}Err(refused("Core unsigned operand exceeds its original integer width"))}
fn signed(body:&[u8],pc:&mut usize,width:u32)->Result<i64,ValueError>{let mut value=0u64;let mut shift=0;loop{let byte=*body.get(*pc).ok_or_else(||refused("Core signed operand exceeds original bytecode"))?;*pc+=1;value|=u64::from(byte&127).checked_shl(shift).ok_or_else(||refused("Core signed operand exceeds original integer width"))?;shift+=7;if byte&128==0{if shift<64&&byte&64!=0{value|=u64::MAX<<shift}return Ok(value as i64)}if shift>=width+7{return Err(refused("Core signed operand exceeds original integer width"))}}}
fn fixed<const N:usize>(body:&[u8],pc:&mut usize)->Result<[u8;N],ValueError>{let end=pc.checked_add(N).ok_or_else(||refused("Core fixed operand position overflow"))?;let bytes=body.get(*pc..end).and_then(|bytes|bytes.try_into().ok()).ok_or_else(||refused("Core fixed operand exceeds original bytecode"))?;*pc=end;Ok(bytes)}
struct StackGrowth {
    capacity: usize,
    old_bytes: usize,
    new_bytes: usize,
}

impl StackGrowth {
    fn for_push<T>(stack: &Vec<T>, other_birth: usize, cx: &StepContext<'_>) -> Result<Option<Self>, ValueError> {
        let length = stack.len();
        if length < stack.capacity() {
            return Ok(None);
        }
        let host_limit = isize::MAX as usize / size_of::<T>();
        let required = length.checked_add(1).filter(|count| *count <= host_limit)
            .ok_or_else(|| refused("Core stack storage exceeds its host limit"))?;
        let grant = cx.retained_grant();
        let available = grant.maximum_capacity_bytes.saturating_sub(other_birth)
            .min(grant.maximum_release_bytes) / size_of::<T>();
        let capacity = length.saturating_mul(2).min(host_limit).min(available).max(required);
        Ok(Some(Self { capacity, old_bytes: bytes_for::<T>(length)?, new_bytes: bytes_for::<T>(capacity)? }))
    }

    fn apply<T>(self, stack: &mut Vec<T>) {
        let mut replacement = Vec::with_capacity(self.capacity);
        replacement.append(stack);
        *stack = replacement;
    }
}

fn grow_values(values: &mut Vec<Value>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    let Some(growth) = StackGrowth::for_push(values, 0, cx)? else { return Ok(true) };
    if !charge(cx, growth.old_bytes, growth.new_bytes, growth.old_bytes)? {
        return Ok(false);
    }
    growth.apply(values);
    Ok(true)
}

fn admit_defined_frame(module:&CoreModule,function:u32,machine:&mut Machine,selector:bool,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 let(type_index,locals,body)=match module.functions.get(function as usize){Some(FunctionDecl::Defined{type_index,locals,body,..})=>(*type_index,locals,body),_=>return Err(refused("Core original call requires a planned defined callee").into())};let signature=module.types.get(type_index as usize).ok_or_else(||refused("Core original call type is absent"))?;let start=machine.values.len().checked_sub(signature.parameters.len()+usize::from(selector)).ok_or_else(||refused("Core original call has fewer operands than its declared parameters"))?;if machine.values[start..start+signature.parameters.len()].iter().zip(&signature.parameters).any(|(value,ty)|value.value_type()!=*ty){return Err(refused("Core original call parameters differ from their actual types").into())}let count=signature.parameters.len().checked_add(locals.len()).ok_or_else(||refused("Core original callee local count overflow"))?;let local_bytes=bytes_for::<Value>(count)?;let type_bytes=bytes_for::<ValueType>(signature.results.len())?;let frame_bytes=local_bytes.checked_add(type_bytes.checked_mul(2).ok_or_else(||refused("Core original call type size overflow"))?).and_then(|bytes|bytes.checked_add(size_of::<ControlFrame>())).ok_or_else(||refused("Core original call storage size overflow"))?;let growth=StackGrowth::for_push(&machine.frames,frame_bytes,cx)?;let old_bytes=growth.as_ref().map_or(0,|growth|growth.old_bytes);let vector_birth=growth.as_ref().map_or(0,|growth|growth.new_bytes);let birth=frame_bytes.checked_add(vector_birth).ok_or_else(||refused("Core original call birth size overflow"))?;let copy=frame_bytes.checked_add(old_bytes).and_then(|bytes|bytes.checked_add(size_of::<Frame>()+size_of::<Value>()+2*size_of::<usize>())).ok_or_else(||refused("Core original call copy size overflow"))?;let end_pc=body.len().checked_sub(1).ok_or_else(||refused("Core original callee has no terminating instruction"))?;if !charge(cx,copy,birth,old_bytes)?{return Ok(false)}
 let mut local_values=Vec::with_capacity(count);local_values.extend_from_slice(&machine.values[start..start+signature.parameters.len()]);local_values.extend(locals.iter().copied().map(ValueType::zero));let mut branch_types=Vec::with_capacity(signature.results.len());branch_types.extend_from_slice(&signature.results);let mut result_types=Vec::with_capacity(signature.results.len());result_types.extend_from_slice(&signature.results);let mut controls=Vec::with_capacity(1);controls.push(ControlFrame{kind:ControlKind::Function,start_pc:0,end_pc,stack_height:start,branch_types,result_types});let frame=Frame{function,pc:0,locals:local_values,stack_base:start,controls};if let Some(growth)=growth{growth.apply(&mut machine.frames);}machine.values.truncate(start);machine.frames.push(frame);Ok(true)
}
enum ReturnedFrameStep{BranchTypes,ResultTypes,Control,Controls,Locals,PopFrame}
fn returned_callee(machine:&Machine)->Result<&Frame,ValueError>{
 if machine.frames.len()<2{return Err(refused("Core returned callee must keep its actual parent frame"))}
 Ok(machine.frames.last().unwrap())
}
fn returned_frame_step(frame:&Frame)->Result<(ReturnedFrameStep,RetirementDemand),ValueError>{
 use ReturnedFrameStep::*;
 let (step,copy_bytes,release_bytes)=if let Some(control)=frame.controls.last(){
  if control.branch_types.capacity()!=0{(BranchTypes,size_of::<Vec<ValueType>>(),bytes_for::<ValueType>(control.branch_types.capacity())?)}
  else if control.result_types.capacity()!=0{(ResultTypes,size_of::<Vec<ValueType>>(),bytes_for::<ValueType>(control.result_types.capacity())?)}
  else{(Control,size_of::<ControlFrame>(),0)}
 }else if frame.controls.capacity()!=0{(Controls,size_of::<Vec<ControlFrame>>(),bytes_for::<ControlFrame>(frame.controls.capacity())?)}
 else if frame.locals.capacity()!=0{(Locals,size_of::<Vec<Value>>(),bytes_for::<Value>(frame.locals.capacity())?)}
 else{(PopFrame,size_of::<Frame>()+size_of::<bool>(),0)};
 Ok((step,RetirementDemand{copy_bytes,release_bytes,depth:1,..Default::default()}))
}
fn close_returned_frame(machine:&mut Machine,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 use ReturnedFrameStep::*;
 let (step,demand)=returned_frame_step(returned_callee(machine)?)?;
 let pay=if demand.release_bytes==0{charge}else{retirement_charge};
 if !pay(cx,demand.copy_bytes,0,demand.release_bytes)?{return Ok(false)}
 let callee=machine.frames.last_mut().unwrap();
 match step{
  BranchTypes=>drop(std::mem::take(&mut callee.controls.last_mut().unwrap().branch_types)),
  ResultTypes=>drop(std::mem::take(&mut callee.controls.last_mut().unwrap().result_types)),
  Control=>{callee.controls.pop();},
  Controls=>drop(std::mem::take(&mut callee.controls)),
  Locals=>drop(std::mem::take(&mut callee.locals)),
  PopFrame=>{machine.frames.pop();return Ok(true)},
 }
 Ok(false)
}

fn unsigned64(body:&[u8],pc:&mut usize)->Result<u64,ValueError>{let mut value=0u64;for shift in (0..70).step_by(7){let byte=*body.get(*pc).ok_or_else(||refused("Core original memory offset exceeds its bytecode"))?;*pc+=1;if shift==63&&byte&126!=0{return Err(refused("Core original memory offset exceeds its integer width"))}value|=u64::from(byte&127)<<shift;if byte&128==0{return Ok(value)}}Err(refused("Core original memory offset exceeds its integer width"))}
fn plan_memory_address(memory:&MemoryState,machine:&Machine,opcode:u8,offset:u64)->Result<(usize,usize),ValueError>{
 let store=opcode>=0x36;let index=machine.values.len().checked_sub(if store{2}else{1}).ok_or_else(||refused("Core original memory instruction has no address operand"))?;let base=match(machine.values[index],memory.memory64){(Value::I32(value),false)=>u64::from(value as u32),(Value::I64(value),true)=>value as u64,_=>return Err(refused("Core original memory address differs from its declared width"))};let width=match opcode{0x28|0x2a|0x34|0x35|0x36|0x38|0x3e=>4,0x29|0x2b|0x37|0x39=>8,0x2c|0x2d|0x30|0x31|0x3a|0x3c=>1,_=>2};let address=base.checked_add(offset).and_then(|value|usize::try_from(value).ok()).ok_or_else(||refused("Core original memory address exceeds its host width"))?;address.checked_add(width).filter(|end|*end<=memory.bytes.len()).ok_or_else(||refused("Core original memory instruction exceeds its retained extent"))?;if store{let expected=match opcode{0x36|0x3a|0x3b=>ValueType::I32,0x38=>ValueType::F32,0x39=>ValueType::F64,_=>ValueType::I64};if machine.values.last().unwrap().value_type()!=expected{return Err(refused("Core original stored scalar differs from its declared type"))}}Ok((address,width))
}
fn admit_memory_instruction(memory:&mut MemoryState,machine:&mut Machine,opcode:u8,address:usize,width:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 let store=opcode>=0x36;let copy=width+size_of::<Value>()*if store{2}else{1}+size_of::<usize>();if !charge(cx,copy,0,0)?{return Ok(false)}if store{let value=*machine.values.last().unwrap();let bytes=match value{Value::I32(value)=>(value as u32 as u64).to_le_bytes(),Value::I64(value)=>(value as u64).to_le_bytes(),Value::F32(bits)=>u64::from(bits).to_le_bytes(),Value::F64(bits)=>bits.to_le_bytes(),_=>unreachable!()};memory.bytes[address..address+width].copy_from_slice(&bytes[..width]);machine.values.pop();machine.values.pop();}else{let mut bytes=[0u8;8];bytes[..width].copy_from_slice(&memory.bytes[address..address+width]);let value=u64::from_le_bytes(bytes);let scalar=match opcode{0x28=>Value::I32(value as u32 as i32),0x29=>Value::I64(value as i64),0x2a=>Value::F32(value as u32),0x2b=>Value::F64(value),0x2c=>Value::I32(value as u8 as i8 as i32),0x2d=>Value::I32(value as u8 as i32),0x2e=>Value::I32(value as u16 as i16 as i32),0x2f=>Value::I32(value as u16 as i32),0x30=>Value::I64(value as u8 as i8 as i64),0x31=>Value::I64(value as u8 as i64),0x32=>Value::I64(value as u16 as i16 as i64),0x33=>Value::I64(value as u16 as i64),0x34=>Value::I64(value as u32 as i32 as i64),0x35=>Value::I64(value as u32 as i64),_=>unreachable!()};*machine.values.last_mut().unwrap()=scalar;}Ok(true)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
