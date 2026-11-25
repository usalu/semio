//! 🧊️ An interpreter-issued image owns only quiescent initialization resources.
use super::*;
use std::sync::atomic::{AtomicU64,Ordering};

#[path = "🧵️image-wire/🦀️.rs"]
mod image_wire;

pub struct PreparedCoreInstance{
 pub(super) identity:u64,
 module:Arc<CoreModule>,
 pub(super) memories:Vec<MemoryState>,
 pub(super) tables:Vec<TableState>,
 pub(super) globals:Vec<GlobalState>,
 pub(super) data:Vec<Option<Vec<u8>>>,
 pub(super) elements:Vec<Option<Vec<Option<u32>>>>,
}

impl PreparedCoreInstance{
 pub(crate) fn seal(core:CoreInstance)->Result<Self,CoreError>{
  Self::seal_checked(core, &mut || true)
 }

 fn seal_checked(core:CoreInstance, checkpoint:&mut impl FnMut()->bool)->Result<Self,CoreError>{
  let CoreInstance{module,memories,tables,globals,data,elements,machine,..}=core;
  if machine.is_some(){return Err(CoreError::State("prepared Core requires a quiescent machine".into()))}
  if memories.len()!=module.memories.len()||tables.len()!=module.tables.len()||globals.len()!=module.globals.len()||data.len()!=module.data.len()||elements.len()!=module.elements.len(){
   return Err(CoreError::State("prepared Core resources differ from the exact module shape".into()))
  }
  let function_exists=|id:u64|usize::try_from(id).is_ok_and(|index|index<module.functions.len());
  for (table,limits) in tables.iter().zip(&module.tables){
   let len=table.values.len()as u64;
   if table.element!=limits.element||table.maximum!=limits.maximum||len<limits.minimum||limits.maximum.is_some_and(|maximum|len>maximum){
    return Err(CoreError::State("prepared Core table carries invalid or external references".into()))
   }
   for values in table.values.chunks(4096) {
    check_prepared_checkpoint(checkpoint)?;
    if values.iter().flatten().any(|id|table.element!=ValueType::FuncRef||!function_exists(*id)){
     return Err(CoreError::State("prepared Core table carries invalid or external references".into()))
    }
   }
  }
  for (global,declared) in globals.iter().zip(&module.globals){
   check_prepared_checkpoint(checkpoint)?;
   if global.mutable!=declared.mutable||global.value.value_type()!=declared.value_type||matches!(global.value,Value::ExternRef(Some(_)))
    ||matches!(global.value,Value::FuncRef(Some(id)) if !function_exists(u64::from(id))){
    return Err(CoreError::State("prepared Core global carries a foreign type or live reference".into()))
   }
  }
  for (memory,limits) in memories.iter().zip(&module.memories){
   check_prepared_checkpoint(checkpoint)?;
   let pages=(memory.bytes.len()/WASM_PAGE_BYTES)as u64;
   if memory.memory64!=limits.memory64||memory.maximum_pages!=limits.maximum_pages||memory.bytes.len()%WASM_PAGE_BYTES!=0||pages<limits.minimum_pages||limits.maximum_pages.is_some_and(|maximum|pages>maximum){
    return Err(CoreError::State("prepared Core memory differs from its declared extent".into()))
   }
  }
  for (value,declared) in data.iter().zip(&module.data) {
   validate_segment(value.as_deref(), declared.mode, &declared.bytes, checkpoint)?;
  }
  for (value,declared) in elements.iter().zip(&module.elements) {
   validate_segment(value.as_deref(), declared.mode, &declared.values, checkpoint)?;
  }
  check_prepared_checkpoint(checkpoint)?;
  static NEXT:AtomicU64=AtomicU64::new(1);
  let identity=NEXT.try_update(Ordering::AcqRel,Ordering::Acquire,|value|value.checked_add(1)).map_err(|_|CoreError::State("prepared Core identity space exhausted".into()))?;
  Ok(Self{identity,module,memories,tables,globals,data,elements})
 }

 pub(super) fn matches_module(&self,module:&Arc<CoreModule>)->bool{Arc::ptr_eq(&self.module,module)}
}

fn validate_segment<T:PartialEq>(value:Option<&[T]>,mode:SegmentMode,declared:&[T],checkpoint:&mut impl FnMut()->bool)->Result<(),CoreError>{
 check_prepared_checkpoint(checkpoint)?;
 let Some(value)=value else{return Ok(())};
 let error=||CoreError::State("prepared Core passive segments differ from their module".into());
 if mode!=SegmentMode::Passive||value.len()!=declared.len(){return Err(error())}
 let chunk=(65_536/size_of::<T>().max(1)).max(1);
 for (actual,expected) in value.chunks(chunk).zip(declared.chunks(chunk)){
  check_prepared_checkpoint(checkpoint)?;
  if actual!=expected{return Err(error())}
 }
 Ok(())
}

fn check_prepared_checkpoint(checkpoint:&mut impl FnMut()->bool)->Result<(),CoreError>{
 if checkpoint(){Ok(())}else{Err(CoreError::State("prepared image restoration cancelled".into()))}
}

pub(super) fn copy_image<T:Copy>(target:&mut Vec<T>,source:&[T],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 let offset=target.len();
 if offset==source.len(){return Ok(true)}
 if target.capacity()<source.len(){reserve(target,source.len(),cx)?;return Ok(false)}
 let count=copy_count(cx,source.len()-offset,size_of::<T>());
 if count>0&&charge_in(RetainedCategory::CheckpointImage,cx,bytes_for::<T>(count)?,0,0)?{target.extend_from_slice(&source[offset..offset+count]);}
 Ok(false)
}

pub(super) fn copy_image_segment<T:Copy>(target:&mut Vec<Option<Vec<T>>>,index:usize,source:Option<&[T]>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
 if target.len()==index{
  if charge_in(RetainedCategory::CheckpointImage,cx,size_of::<Option<Vec<T>>>(),0,0)?{target.push(source.map(|_|Vec::new()));}
  return Ok(false)
 }
 let Some(source)=source else{return Ok(true)};
 let destination=target[index].as_mut().ok_or_else(||refused("prepared Core passive segment lost its paid destination"))?;
 copy_image(destination,source,cx)
}
