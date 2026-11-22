//! 🧭️ Structured instructions retain bounded planning and paid label retirement on the actual original Core.
use super::*;
pub(crate) enum OriginalControlPlan{Transfer(Transfer),Table{pc:usize,count:usize,index:usize,selector:usize,depth:usize}}
pub(super) struct ControlAdmission{pub executed:bool,pub plan:Option<OriginalControlPlan>}
pub(super) enum ControlProgress{Pending,Executed,Complete{parameter_return:bool}}
pub(crate) struct Transfer{keep:usize,height:usize,start:usize,count:usize,copied:usize,pc:usize,parameter_return:bool}
impl OriginalControlPlan{
 pub(super) fn advance(&mut self,machine:&mut Machine,module:&CoreModule,cx:&mut StepContext<'_>)->Result<ControlProgress,OriginalTurnFailure>{
  match self{
   Self::Table{pc,count,index,selector,depth}=>{
    let body=body(module,machine)?;while *index<=*count{let mut next=*pc;let value=unsigned(body,&mut next)?;if !charge(cx,next-*pc+4*size_of::<usize>(),0,0)?{return Ok(ControlProgress::Pending)}if *index==*selector||*index==*count&&*selector>=*count{*depth=value}*pc=next;*index+=1;}
    let plan=transfer_to(machine,branch_index(machine,*depth)?,true,true)?;
    if let Some(plan)=plan.retain(machine,true,cx)?{*self=Self::Transfer(plan);Ok(ControlProgress::Executed)}else{Ok(ControlProgress::Pending)}
   },
   Self::Transfer(plan)=>{
    if plan.copied<plan.count{let width=size_of::<Value>();let count=(plan.count-plan.copied).min(cx.retained_grant().maximum_copy_bytes.saturating_sub(size_of::<usize>())/width);if count==0||!charge(cx,count*width+size_of::<usize>(),0,0)?{return Ok(ControlProgress::Pending)}machine.values.copy_within(plan.start+plan.copied..plan.start+plan.copied+count,plan.height+plan.copied);plan.copied+=count;return Ok(ControlProgress::Pending)}
    let frame=machine.frames.last_mut().ok_or_else(||refused("Core original control retirement has no frame"))?;
    if frame.controls.len()>plan.keep{let control=frame.controls.last_mut().unwrap();if release_vector(&mut control.branch_types,cx)?||release_vector(&mut control.result_types,cx)?{return Ok(ControlProgress::Pending)}if charge(cx,size_of::<ControlFrame>(),0,0)?{frame.controls.pop();}return Ok(ControlProgress::Pending)}
    if !charge(cx,size_of::<Self>()+2*size_of::<usize>(),0,0)?{return Ok(ControlProgress::Pending)}machine.values.truncate(plan.height+plan.count);frame.pc=plan.pc;Ok(ControlProgress::Complete{parameter_return:plan.parameter_return})
   }
  }
 }
}
pub(super) fn admit(module:&CoreModule,machine:&mut Machine,opcode:u8,mut pc:usize,cx:&mut StepContext<'_>)->Result<Option<ControlAdmission>,OriginalTurnFailure>{
 let body=body(module,machine)?;let frame=machine.frames.last().unwrap();
 match opcode{
  0x02..=0x04=>{
   let shape=shape(body,&mut pc,&module.types)?;let parameters=shape.parameters();let results=shape.results();let conditional=opcode==0x04;let condition=if conditional{match machine.values.last(){Some(Value::I32(value))=>*value!=0,_=>return Err(refused("Core original if requires its i32 condition").into())}}else{true};let available=machine.values.len().checked_sub(usize::from(conditional)).ok_or_else(||refused("Core original block has no condition operand"))?;let height=available.checked_sub(parameters.len()).ok_or_else(||refused("Core original block parameters underflow its stack"))?;types(&machine.values[height..available],parameters)?;
   let bounds=match module.functions.get(frame.function as usize){Some(FunctionDecl::Defined{controls,..})=>controls.get(&(frame.pc)).copied(),_=>None}.ok_or_else(||refused("Core original block has no actual control bounds"))?;
   if !condition&&bounds.else_pc.is_none(){types(&machine.values[height..available],results)?;if !charge(cx,size_of::<Value>()+size_of::<usize>(),0,0)?{return Ok(None)}machine.values.truncate(available);machine.frames.last_mut().unwrap().pc=bounds.end_pc+1;return Ok(Some(ControlAdmission{executed:true,plan:None}))}
   let kind=match opcode{0x02=>ControlKind::Block,0x03=>ControlKind::Loop,_=>ControlKind::If};let branch=if kind==ControlKind::Loop{parameters}else{results};let type_bytes=bytes_for::<ValueType>(branch.len()+results.len())?;let growth=StackGrowth::for_push(&frame.controls,type_bytes,cx)?;
   let (relocation,birth)=growth.as_ref().map_or((0,0),|growth|(growth.old_bytes,growth.new_bytes));
   if !charge(cx,type_bytes+relocation+size_of::<ControlFrame>()+size_of::<Value>(),type_bytes+birth,relocation)?{return Ok(None)}
   let mut branch_types=Vec::with_capacity(branch.len());branch_types.extend_from_slice(branch);let mut result_types=Vec::with_capacity(results.len());result_types.extend_from_slice(results);if conditional{machine.values.pop();}let frame=machine.frames.last_mut().unwrap();if let Some(growth)=growth{growth.apply(&mut frame.controls);}frame.controls.push(ControlFrame{kind,start_pc:pc,end_pc:bounds.end_pc,stack_height:height,branch_types,result_types});frame.pc=if !condition{bounds.else_pc.unwrap()+1}else{pc};return Ok(Some(ControlAdmission{executed:true,plan:None}))
  },
  0x05|0x0b=>{let index=frame.controls.len().checked_sub(1).ok_or_else(||refused("Core original end requires an actual control label"))?;let control=&frame.controls[index];if opcode==0x05&&control.kind!=ControlKind::If{return Err(refused("Core original else requires its actual if label").into())}admit_transfer(machine,index,false,false,cx)},
  0x0f=>admit_transfer(machine,0,false,false,cx),
  0x0c|0x0d=>{let depth=unsigned(body,&mut pc)?;if opcode==0x0d{let condition=match machine.values.last(){Some(Value::I32(value))=>*value!=0,_=>return Err(refused("Core original branch requires its i32 condition").into())};if !condition{if !charge(cx,size_of::<Value>()+size_of::<usize>(),0,0)?{return Ok(None)}machine.values.pop();machine.frames.last_mut().unwrap().pc=pc;return Ok(Some(ControlAdmission{executed:true,plan:None}))}}let index=branch_index(machine,depth)?;admit_transfer(machine,index,true,opcode==0x0d,cx)},
  0x0e=>{let count=unsigned(body,&mut pc)?;let selector=match machine.values.last(){Some(Value::I32(value))=>*value as u32 as usize,_=>return Err(refused("Core original branch table requires its i32 selector").into())};if !charge(cx,size_of::<OriginalControlPlan>(),0,0)?{return Ok(None)}return Ok(Some(ControlAdmission{executed:false,plan:Some(OriginalControlPlan::Table{pc,count,index:0,selector,depth:usize::MAX})}))},
  0x1b|0x1c=>{
   let declared=if opcode==0x1c{if unsigned(body,&mut pc)?!=1{return Err(refused("Core original typed select requires one declared result").into())}let first=*body.get(pc).ok_or_else(||refused("Core original select type exceeds its bytecode"))?;pc+=1;Some(value_type(first)?)}else{None};let start=machine.values.len().checked_sub(3).ok_or_else(||refused("Core original select requires three operands"))?;let left=machine.values[start];let right=machine.values[start+1];let condition=match machine.values[start+2]{Value::I32(value)=>value,_=>return Err(refused("Core original select requires an i32 condition").into())};if left.value_type()!=right.value_type()||declared.is_some_and(|ty|ty!=left.value_type()){return Err(refused("Core original select operands differ from their declared type").into())}if !charge(cx,3*size_of::<Value>()+size_of::<usize>(),0,0)?{return Ok(None)}machine.values.truncate(start);machine.values.push(if condition!=0{left}else{right});machine.frames.last_mut().unwrap().pc=pc;return Ok(Some(ControlAdmission{executed:true,plan:None}))
  },
  _=>return Err(refused("Core original structured control instruction is not declared").into())
 }
}
fn branch_index(machine:&Machine,depth:usize)->Result<usize,ValueError>{let count=machine.frames.last().ok_or_else(||refused("Core original branch has no actual frame"))?.controls.len();count.checked_sub(depth.checked_add(1).ok_or_else(||refused("Core original branch depth overflow"))?).ok_or_else(||refused("Core original branch depth exceeds its labels"))}

fn admit_transfer(machine:&mut Machine,index:usize,branch:bool,pop:bool,cx:&mut StepContext<'_>)->Result<Option<ControlAdmission>,OriginalTurnFailure>{
 let plan=transfer_to(machine,index,branch,pop)?;
 let frame=machine.frames.last_mut().unwrap();
 let label=&frame.controls[index];
 let retire=plan.keep==index;
 if plan.count==0 && !plan.parameter_return && index+1==frame.controls.len()
  && (!retire || (label.branch_types.capacity()==0 && label.result_types.capacity()==0)){
  let copy=2*size_of::<usize>()+if retire{size_of::<ControlFrame>()}else{0};
  if charge(cx,copy,0,0)?{
   machine.values.truncate(plan.height);
   if retire{frame.controls.pop();}
   frame.pc=plan.pc;
   return Ok(Some(ControlAdmission{executed:true,plan:None}));
  }
 }
 Ok(plan.retain(machine,pop,cx)?.map(|plan|ControlAdmission{executed:true,plan:Some(OriginalControlPlan::Transfer(plan))}))
}

impl Transfer{
 fn retain(self,machine:&mut Machine,pop:bool,cx:&mut StepContext<'_>)->Result<Option<Self>,OriginalTurnFailure>{
  if !charge(cx,size_of::<Self>()+size_of::<Value>(),0,0)?{return Ok(None)}
  if pop{machine.values.pop();}
  Ok(Some(self))
 }
}

fn transfer_to(machine:&Machine,index:usize,branch:bool,pop:bool)->Result<Transfer,ValueError>{let frame=machine.frames.last().unwrap();let control=frame.controls.get(index).ok_or_else(||refused("Core original control target is absent"))?;let target=if branch{&control.branch_types}else{&control.result_types};let available=machine.values.len().checked_sub(usize::from(pop)).ok_or_else(||refused("Core original branch selector is absent"))?;let start=available.checked_sub(target.len()).filter(|start|*start>=control.stack_height).ok_or_else(||refused("Core original control results overlap outer operands"))?;types(&machine.values[start..available],target)?;let looping=branch&&control.kind==ControlKind::Loop;Ok(Transfer{keep:if looping{index+1}else{index},height:control.stack_height,start,count:target.len(),copied:0,pc:if looping{control.start_pc}else{control.end_pc+1},parameter_return:control.kind==ControlKind::Function})}
fn types(values:&[Value],types:&[ValueType])->Result<(),ValueError>{if values.len()!=types.len()||values.iter().zip(types).any(|(value,ty)|value.value_type()!=*ty){return Err(refused("Core original structured control values differ from their declared types"))}Ok(())}
fn body<'a>(module:&'a CoreModule,machine:&Machine)->Result<&'a[u8],ValueError>{match module.functions.get(machine.frames.last().ok_or_else(||refused("Core original control requires its current frame"))?.function as usize){Some(FunctionDecl::Defined{body,..})=>Ok(body),_=>Err(refused("Core original control requires its defined original body"))}}
enum Shape<'a>{Empty,Single(ValueType),Signature(&'a FunctionType)}
impl Shape<'_>{fn parameters(&self)->&[ValueType]{match self{Self::Signature(ty)=>&ty.parameters,_=>&[]}}fn results(&self)->&[ValueType]{match self{Self::Empty=>&[],Self::Single(ty)=>std::slice::from_ref(ty),Self::Signature(ty)=>&ty.results}}}
fn shape<'a>(body:&[u8],pc:&mut usize,types:&'a[FunctionType])->Result<Shape<'a>,ValueError>{let first=*body.get(*pc).ok_or_else(||refused("Core original block type exceeds its bytecode"))?;if first==0x40{*pc+=1;return Ok(Shape::Empty)}if matches!(first,0x7f|0x7e|0x7d|0x7c|0x70|0x6f){*pc+=1;return Ok(Shape::Single(value_type(first)?))}let index=unsigned(body,pc)?;types.get(index).map(Shape::Signature).ok_or_else(||refused("Core original block type index exceeds its module"))}
fn value_type(byte:u8)->Result<ValueType,ValueError>{match byte{0x7f=>Ok(ValueType::I32),0x7e=>Ok(ValueType::I64),0x7d=>Ok(ValueType::F32),0x7c=>Ok(ValueType::F64),0x70=>Ok(ValueType::FuncRef),0x6f=>Ok(ValueType::ExternRef),_=>Err(refused("Core original value type is not declared"))}}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(super) mod tests;
