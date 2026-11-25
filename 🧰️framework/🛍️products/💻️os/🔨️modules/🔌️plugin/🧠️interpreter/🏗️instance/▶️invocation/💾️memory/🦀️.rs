//! 💾️ Memory instructions retain their actual paid replacement or bounded copy cursor until the guest instruction commits.
use super::*;
pub(crate) enum OriginalMemoryWork{
 Grow{memory:usize,pc:usize,result:Value,old:usize,length:usize,new:Vec<u8>},
 Extend{memory:usize,pc:usize,result:Value,length:usize,initialized:usize},
 Copy{destination:usize,source:usize,to:usize,from:usize,length:usize,copied:usize,backward:bool,pc:usize},
 Fill{memory:usize,to:usize,value:u8,length:usize,copied:usize,pc:usize},
 Init{memory:usize,data:usize,to:usize,from:usize,length:usize,copied:usize,pc:usize},
}
pub(super) struct MemoryAdmission{pub executed:bool,pub work:Option<OriginalMemoryWork>}
impl OriginalMemoryWork{
 pub(super) fn advance(&mut self,core:&mut CoreInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  match self{
   Self::Grow{memory,pc,result,old,length,new}=>{
    if new.len()<*old{let offset=new.len();let count=chunk(cx,*old-offset);if count==0||!charge_in(RetainedCategory::MemoryGrow,cx,count,0,0)?{return Ok(false)}new.extend_from_slice(&core.memories[*memory].bytes[offset..offset+count]);return Ok(false)}
    if new.len()<*length{let count=chunk(cx,*length-new.len());if count==0||!charge_in(RetainedCategory::MemoryGrow,cx,count,0,0)?{return Ok(false)}new.resize(new.len()+count,0);return Ok(false)}
    let memory=&mut core.memories[*memory];if !charge_in(RetainedCategory::MemoryGrow,cx,0,0,memory.bytes.capacity())?{return Ok(false)}let old=std::mem::replace(&mut memory.bytes,std::mem::take(new));drop(old);finish(core,*pc,1,Some(*result));Ok(true)
   },
   Self::Extend{memory,pc,result,length,initialized}=>{
    let bytes=&mut core.memories[*memory].bytes;
    if *initialized<*length-bytes.len(){
     let count=chunk(cx,*length-bytes.len()-*initialized);
     if count==0||!charge_in(RetainedCategory::MemoryGrow,cx,count,0,0)?{return Ok(false)}
     bytes.spare_capacity_mut()[*initialized..*initialized+count].fill(std::mem::MaybeUninit::new(0));
     *initialized+=count;
     return Ok(false)
    }
    if !pay_work(cx)?{return Ok(false)}
    publish_initialized_pages(bytes,*length,*initialized);
    finish(core,*pc,1,Some(*result));Ok(true)
   },
   Self::Copy{destination,source,to,from,length,copied,backward,pc}=>{
    if *copied<*length{let count=work_chunk(cx,*length-*copied);if count==0||!pay_work_bytes(cx,count)?{return Ok(false)}let offset=if *backward{*length-*copied-count}else{*copied};if destination==source{core.memories[*destination].bytes.copy_within(*from+offset..*from+offset+count,*to+offset)}else if *source<*destination{let(left,right)=core.memories.split_at_mut(*destination);right[0].bytes[*to+offset..*to+offset+count].copy_from_slice(&left[*source].bytes[*from+offset..*from+offset+count])}else{let(left,right)=core.memories.split_at_mut(*source);left[*destination].bytes[*to+offset..*to+offset+count].copy_from_slice(&right[0].bytes[*from+offset..*from+offset+count])}*copied+=count;return Ok(false)}commit(core,*pc,cx)
   },
   Self::Fill{memory,to,value,length,copied,pc}=>{if *copied<*length{let count=work_chunk(cx,*length-*copied);if count==0||!pay_work_bytes(cx,count)?{return Ok(false)}core.memories[*memory].bytes[*to+*copied..*to+*copied+count].fill(*value);*copied+=count;return Ok(false)}commit(core,*pc,cx)},
   Self::Init{memory,data,to,from,length,copied,pc}=>{if *copied<*length{let count=work_chunk(cx,*length-*copied);if count==0||!pay_work_bytes(cx,count)?{return Ok(false)}let source=core.data[*data].as_deref().unwrap_or(&[]);core.memories[*memory].bytes[*to+*copied..*to+*copied+count].copy_from_slice(&source[*from+*copied..*from+*copied+count]);*copied+=count;return Ok(false)}commit(core,*pc,cx)}
  }
 }
 pub(super) fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{let release=match self{Self::Grow{new,..}=>new.capacity(),_=>0};if !retirement_charge(cx,size_of::<Self>(),0,release)?{return Ok(false)}if let Self::Grow{new,..}=self{drop(std::mem::take(new));}Ok(true)}
}
pub(super) fn admit(core:&mut CoreInstance,opcode:u8,body:&[u8],mut pc:usize,cx:&mut StepContext<'_>)->Result<Option<MemoryAdmission>,OriginalTurnFailure>{
 if opcode==0x40{
  let index=unsigned(body,&mut pc)?;let memory=core.memories.get(index).ok_or_else(||refused("Core original grow memory is absent"))?;let machine=core.machine.as_ref().unwrap();let value=*machine.values.last().ok_or_else(||refused("Core original grow has no delta"))?;let delta=match(value,memory.memory64){(Value::I32(value),false)=>u64::from(value as u32),(Value::I64(value),true)=>value as u64,_=>return Err(refused("Core original grow delta changes its declared type").into())};let old_pages=(memory.bytes.len()/WASM_PAGE_BYTES)as u64;let length=old_pages.checked_add(delta).filter(|pages|memory.maximum_pages.is_none_or(|maximum|*pages<=maximum)).and_then(|pages|pages.checked_mul(WASM_PAGE_BYTES as u64)).and_then(|length|usize::try_from(length).ok()).filter(|length|*length<=isize::MAX as usize);
  let result=if memory.memory64{Value::I64(old_pages as i64)}else{Value::I32(old_pages as i32)};if length.is_none()||delta==0{if !pay_work(cx)?{return Ok(None)}let value=if length.is_none(){if memory.memory64{Value::I64(-1)}else{Value::I32(-1)}}else{result};finish(core,pc,1,Some(value));return Ok(Some(MemoryAdmission{executed:true,work:None}))}
  let length=length.unwrap();
  let work=if length<=memory.bytes.capacity(){
   if !pay_work(cx)?{return Ok(None)}
   OriginalMemoryWork::Extend{memory:index,pc,result,length,initialized:0}
  }else{
   let capacity=growth_capacity(memory,length,cx.retained_grant().maximum_capacity_bytes);
   if !charge_in(RetainedCategory::MemoryGrow,cx,0,capacity,0)?{return Ok(None)}
   OriginalMemoryWork::Grow{memory:index,pc,result,old:memory.bytes.len(),length,new:Vec::with_capacity(capacity)}
  };
  return Ok(Some(MemoryAdmission{executed:false,work:Some(work)}))
 }
 let operation=unsigned(body,&mut pc)?;let machine=core.machine.as_ref().unwrap();
 if operation<=7{let value=*machine.values.last().ok_or_else(||refused("Core original saturating conversion has no operand"))?;let result=match(operation,value){(0|1,Value::F32(bits))=>Value::I32(saturating_i32_from_f32(f32::from_bits(bits),operation==0)),(2|3,Value::F64(bits))=>Value::I32(saturating_i32_from_f64(f64::from_bits(bits),operation==2)),(4|5,Value::F32(bits))=>Value::I64(saturating_i64_from_f32(f32::from_bits(bits),operation==4)),(6|7,Value::F64(bits))=>Value::I64(saturating_i64_from_f64(f64::from_bits(bits),operation==6)),_=>return Err(refused("Core original saturating conversion has the wrong operand type").into())};if !pay_work(cx)?{return Ok(None)}finish(core,pc,1,Some(result));return Ok(Some(MemoryAdmission{executed:true,work:None}))}
 if operation==9{let index=unsigned(body,&mut pc)?;let data=core.data.get(index).ok_or_else(||refused("Core original data drop index is absent"))?;let release=data.as_ref().map_or(0,Vec::capacity);if !charge_in(RetainedCategory::MemoryGrow,cx,0,0,release)?{return Ok(None)}core.data[index].take();finish(core,pc,0,None);return Ok(Some(MemoryAdmission{executed:true,work:None}))}
 let work=match operation{
  8=>{let data=unsigned(body,&mut pc)?;let memory=unsigned(body,&mut pc)?;let(to,from,length)=operands(machine)?;let source=core.data.get(data).ok_or_else(||refused("Core original memory init data is absent"))?.as_deref().unwrap_or(&[]);extent(from,length,source.len())?;extent(to,length,core.memories.get(memory).ok_or_else(||refused("Core original memory init destination is absent"))?.bytes.len())?;OriginalMemoryWork::Init{memory,data,to,from,length,copied:0,pc}},
  10=>{let destination=unsigned(body,&mut pc)?;let source=unsigned(body,&mut pc)?;let(to,from,length)=operands(machine)?;extent(to,length,core.memories.get(destination).ok_or_else(||refused("Core original memory copy destination is absent"))?.bytes.len())?;extent(from,length,core.memories.get(source).ok_or_else(||refused("Core original memory copy source is absent"))?.bytes.len())?;OriginalMemoryWork::Copy{destination,source,to,from,length,copied:0,backward:destination==source&&to>from&&to<from+length,pc}},
  11=>{let memory=unsigned(body,&mut pc)?;let(to,value,length)=operands(machine)?;extent(to,length,core.memories.get(memory).ok_or_else(||refused("Core original memory fill destination is absent"))?.bytes.len())?;OriginalMemoryWork::Fill{memory,to,value:value as u8,length,copied:0,pc}},
  _=>return Err(refused("Core original bulk instruction has no admitted physical plan").into())
 };if !pay_work(cx)?{return Ok(None)}Ok(Some(MemoryAdmission{executed:false,work:Some(work)}))
}
fn growth_capacity(memory:&MemoryState,length:usize,maximum_capacity_bytes:usize)->usize{
 let maximum=memory.maximum_pages.and_then(|pages|pages.checked_mul(WASM_PAGE_BYTES as u64)).and_then(|bytes|usize::try_from(bytes).ok()).unwrap_or(isize::MAX as usize).min(isize::MAX as usize);
 let available=maximum_capacity_bytes.min(maximum)/WASM_PAGE_BYTES*WASM_PAGE_BYTES;
 memory.bytes.capacity().saturating_mul(2).min(available).max(length)
}
/// 🌱️ Publishes only the spare page prefix initialized contiguously by the retained extension.
fn publish_initialized_pages(bytes:&mut Vec<u8>,length:usize,initialized:usize){
 assert_eq!(length-bytes.len(),initialized);
 assert!(length<=bytes.capacity());
 unsafe{bytes.set_len(length)}
}
fn chunk(cx:&StepContext<'_>,remaining:usize)->usize{remaining.min(cx.retained_grant().maximum_copy_bytes)}
fn operands(machine:&Machine)->Result<(usize,usize,usize),ValueError>{let start=machine.values.len().checked_sub(3).ok_or_else(||refused("Core original memory operation requires three operands"))?;match(&machine.values[start],&machine.values[start+1],&machine.values[start+2]){(Value::I32(to),Value::I32(from),Value::I32(length))=>Ok((*to as u32 as usize,*from as u32 as usize,*length as u32 as usize)),_=>Err(refused("Core original bulk operation requires its declared i32 operands"))}}
fn extent(start:usize,length:usize,bound:usize)->Result<(),ValueError>{start.checked_add(length).filter(|end|*end<=bound).map(|_|()).ok_or_else(||refused("Core original bulk operation exceeds its actual retained extent"))}
fn finish(core:&mut CoreInstance,pc:usize,count:usize,result:Option<Value>){let machine=core.machine.as_mut().unwrap();machine.values.truncate(machine.values.len()-count);if let Some(result)=result{machine.values.push(result)}machine.frames.last_mut().unwrap().pc=pc;}
fn commit(core:&mut CoreInstance,pc:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if !pay_work(cx)?{return Ok(false)}finish(core,pc,3,None);Ok(true)}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
