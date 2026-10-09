//! 🛬️ Borrowed intrinsic construction and explicit partial-owner retirement.
use super::{DslValue, FromValue, NativeDecodeControl, ValueError, ControlledValueHasher};

/// 🛡️ Retires an admitted child if its parent cannot publish a complete owner.
pub struct DecodedValue<T> { value:Option<T>, retire:fn(T) }
impl<T> DecodedValue<T> {
    pub fn new(value:T,retire:fn(T))->Self{Self{value:Some(value),retire}}
    pub fn get(&self)->&T{self.value.as_ref().expect("owned value present")}
    pub fn get_mut(&mut self)->&mut T{self.value.as_mut().expect("owned value present")}
    pub fn take(mut self)->T{self.value.take().expect("owned value present")}
}
impl<T> Drop for DecodedValue<T>{fn drop(&mut self){if let Some(value)=self.value.take(){(self.retire)(value)}}}

pub fn scalar(control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;control.step()})}

pub fn sequence<T:FromValue>(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Vec<T>,ValueError>{
    let DslValue::Array(items)=value else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected array"))};
    control.scoped_stage(|control|{
        control.begin_stage(items.len())?;
        let mut output=Vec::<T>::guard_decoded(control.allocate_vec(items.len())?);
        for(index,value)in items.iter().enumerate(){let child=T::from_value_controlled(value,control).map_err(|e|e.under(index))?;output.get_mut().push(child);control.step()?;}
        Ok(output.take())
    })
}

pub fn set<T:FromValue+Ord>(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<std::collections::BTreeSet<T>,ValueError>{
    let DslValue::Array(items)=value else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected array"))};
    control.scoped_stage(|control|{
        control.begin_stage(items.len())?;
        let mut output=std::collections::BTreeSet::<T>::new().guard_decoded();
        for(index,value)in items.iter().enumerate(){
            control.charge(tree_allocation::<T>(output.get().len()).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "set allocation overflow"))?)?;
            let child=T::from_value_controlled(value,control).map_err(|e|e.under(index))?.guard_decoded();
            if !output.get().contains(child.get()){output.get_mut().insert(child.take());}
            control.step()?;
        }
        Ok(output.take())
    })
}

pub fn hash_set<T:FromValue+Eq+std::hash::Hash,S:ControlledValueHasher>(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<std::collections::HashSet<T,S>,ValueError>{
    let DslValue::Array(items)=value else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected array"))};
    control.scoped_stage(|control|{
        control.begin_stage(items.len())?;
        control.charge(hash_allocation::<T>(items.len()).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "hash set allocation overflow"))?)?;
        let mut set=std::collections::HashSet::<T,S>::with_hasher(S::from_value_controlled(control)?);
        set.try_reserve(items.len()).map_err(|_|ValueError::new(crate::ValueRefusalKind::AllocationFailed, "hash set allocation failed"))?;
        let mut output=set.guard_decoded();
        for(index,value)in items.iter().enumerate(){
            let child=T::from_value_controlled(value,control).map_err(|error|error.under(index))?.guard_decoded();
            if !output.get().contains(child.get()){output.get_mut().insert(child.take());}
            control.step()?;
        }
        Ok(output.take())
    })
}

pub fn map<T:FromValue>(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<std::collections::BTreeMap<String,T>,ValueError>{
    let DslValue::Object(entries)=value else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected object"))};
    control.scoped_stage(|control|{
        control.begin_stage(entries.len())?;
        let mut output=std::collections::BTreeMap::<String,T>::new().guard_decoded();
        for(index,(key,value))in entries.iter().enumerate(){
            control.charge(tree_allocation::<(String,T)>(output.get().len()).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "map allocation overflow"))?)?;
            let key=control.copy_text(key)?;
            let child=T::from_value_controlled(value,control).map_err(|e|e.under(index))?;
            if let Some(previous)=output.get_mut().insert(key,child){T::retire_decoded(previous);}
            control.step()?;
        }
        Ok(output.take())
    })
}

fn push<T>(values:&mut Vec<T>,value:T,control:&mut NativeDecodeControl<'_>)->Result<(),(ValueError,T)>{
    if values.len()==values.capacity(){
        let next=values.capacity().max(1).checked_mul(2).and_then(|n|n.checked_mul(size_of::<T>())).filter(|bytes|*bytes<=isize::MAX as usize);
        let Some(bytes)=next else{return Err((ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "native frontier size overflow"),value))};
        if let Err(error)=control.charge(bytes){return Err((error,value))}
        if values.try_reserve_exact(values.capacity().max(1)).is_err(){return Err((ValueError::new(crate::ValueRefusalKind::AllocationFailed, "native frontier allocation failed"),value))}
    }
    values.push(value);Ok(())
}

enum Task<'a>{Value(&'a DslValue),Array(usize),Object(&'a[(String,DslValue)])}

pub fn intrinsic(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DslValue,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut pending=Vec::new();push(&mut pending,Task::Value(value),control).map_err(|(e,_)|e)?;
        let mut values=Vec::<DslValue>::new().guard_decoded();
        while let Some(task)=pending.pop(){
            control.step()?;
            let result=match task{
                Task::Value(DslValue::Null)=>Some(DslValue::Null),
                Task::Value(DslValue::Bool(v))=>Some(DslValue::Bool(*v)),
                Task::Value(DslValue::Number(v))=>Some(DslValue::Number(*v)),
                Task::Value(DslValue::String(v))=>Some(DslValue::String(control.copy_text(v)?)),
                Task::Value(DslValue::Bytes(v))=>Some(DslValue::Bytes(control.copy_bytes(v)?)),
                Task::Value(DslValue::Array(items))=>{push(&mut pending,Task::Array(items.len()),control).map_err(|(e,_)|e)?;for item in items.iter().rev(){control.step()?;push(&mut pending,Task::Value(item),control).map_err(|(e,_)|e)?;}None},
                Task::Value(DslValue::Object(entries))=>{push(&mut pending,Task::Object(entries),control).map_err(|(e,_)|e)?;for(_,item)in entries.iter().rev(){control.step()?;push(&mut pending,Task::Value(item),control).map_err(|(e,_)|e)?;}None},
                Task::Array(count)=>{
                    let mut children=Vec::<DslValue>::guard_decoded(control.allocate_vec(count)?);
                    let start=values.get().len()-count;for index in start..values.get().len(){control.step()?;children.get_mut().push(std::mem::replace(&mut values.get_mut()[index],DslValue::Null));}values.get_mut().truncate(start);Some(DslValue::Array(children.take()))
                },
                Task::Object(entries)=>{
                    let mut children=DecodedValue::new(control.allocate_vec::<(String,DslValue)>(entries.len())?,|values|{for(_,v)in values{retire_intrinsic(v)}});
                    let start=values.get().len()-entries.len();
                    for(index,(key,_))in entries.iter().enumerate(){let key=control.copy_text(key)?;let child=std::mem::replace(&mut values.get_mut()[start+index],DslValue::Null);children.get_mut().push((key,child));}
                    values.get_mut().truncate(start);Some(DslValue::Object(children.take()))
                }
            };
            if let Some(result)=result{if let Err((error,value))=push(values.get_mut(),result,control){retire_intrinsic(value);return Err(error)}}
        }
        Ok(values.get_mut().pop().expect("one completed intrinsic root"))
    })
}

pub fn retire_intrinsic(value:DslValue){
    let mut retirement=super::IntrinsicRetirement::new(value);
    while !retirement.terminal_is_empty(){retirement.close_step(256);}
}

fn key_hash(key:&str,control:&mut NativeDecodeControl<'_>)->Result<u64,ValueError>{
    use std::hash::Hasher;
    control.scoped_stage(|control|{
        control.begin_stage(key.len())?;
        let mut hasher=std::collections::hash_map::DefaultHasher::new();
        for chunk in key.as_bytes().chunks(65536){hasher.write(chunk);control.advance(chunk.len())?;}
        hasher.write_u8(0xff);
        Ok(hasher.finish())
    })
}

fn key_equal(left:&str,right:&str,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
    if left.len()!=right.len(){return Ok(false)}
    control.scoped_stage(|control|{
        control.begin_stage(left.len())?;
        for(a,b)in left.as_bytes().chunks(65536).zip(right.as_bytes().chunks(65536)){
            if let Some(index)=a.iter().zip(b).position(|(a,b)|a!=b){control.advance(index+1)?;return Ok(false)}
            control.advance(a.len())?;
        }
        Ok(true)
    })
}

impl DslValue {
/// 🗂️ Validates full borrowed object keys in admitted indexed slots.
pub fn object_controlled<'v>(&'v self,control:&mut NativeDecodeControl<'_>)->Result<&'v[(String,DslValue)],ValueError>{
 let Self::Object(entries)=self else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue,"expected object"))};
 control.scoped_stage(|control|{
  control.begin_stage(entries.len())?;
  let count=if entries.is_empty(){0}else{entries.len().checked_mul(2).and_then(usize::checked_next_power_of_two).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"object key index size overflow"))?};
  let mut slots=control.allocate_vec::<Option<(u64,&str)>>(count)?;slots.resize(count,None);
  for(key,_)in entries{
   let hash=key_hash(key,control)?;let mask=count-1;let mut slot=usize::try_from(hash&u64::try_from(mask).map_err(|_|ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"object key index mask overflow"))?).map_err(|_|ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"object key index offset overflow"))?;
   control.scoped_stage(|control|{
    control.begin_stage(0)?;
    for _ in 0..count{
     control.step()?;
     match slots[slot]{
      None=>{slots[slot]=Some((hash,key));return Ok(());},
      Some((previous_hash,previous))=>{if hash==previous_hash&&key_equal(previous,key,control)?{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue,"duplicate object key"));}}
     }
     slot=(slot+1)&mask;
    }
    Err(ValueError::new(crate::ValueRefusalKind::InvariantViolated,"object key index has no vacant slot"))
   })?;
   control.step()?;
  }
  Ok(entries.as_slice())
 })
}
    /// 🔎️ Reads a field without owning a second native subtree.
    pub fn field_controlled<'v>(entries:&'v[(String,DslValue)],key:&str,control:&mut NativeDecodeControl<'_>)->Result<Option<&'v DslValue>,ValueError>{
        control.scoped_stage(|control|{control.begin_stage(entries.len())?;let mut found=None;for(k,value)in entries{if k==key{found=Some(value)}control.step()?;}Ok(found)})
    }
    /// 🪗️ Materializes only the explicit flattened or internally-tagged payload under admission.
    pub fn filtered_object_controlled(entries:&[(String,DslValue)],excluded:&[&str],control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
        control.scoped_stage(|control|{
            control.begin_stage(entries.len())?;
            let count=control.scoped_stage(|control|{control.begin_stage(entries.len())?;let mut count=0;for(key,_)in entries{if !excluded.contains(&key.as_str()){count+=1}control.step()?;}Ok::<_,ValueError>(count)})?;
            let mut output=DecodedValue::new(control.allocate_vec::<(String,DslValue)>(count)?,|entries|{for(_,v)in entries{retire_intrinsic(v)}});
            for(key,value)in entries{if !excluded.contains(&key.as_str()){let key=control.copy_text(key)?;let value=intrinsic(value,control)?;output.get_mut().push((key,value));}control.step()?;}
            Ok(Self::Object(output.take()))
        })
    }
    /// 🛡️ Rejects undeclared borrowed fields while retaining cancellation checkpoints.
    pub fn deny_fields_controlled(entries:&[(String,DslValue)],allowed:&[&str],control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        control.scoped_stage(|control|{control.begin_stage(entries.len())?;for(key,_)in entries{if !allowed.contains(&key.as_str()){return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "unknown field"))}control.step()?;}Ok(())})
    }
}

pub(crate) fn tree_allocation<T>(length:usize)->Option<usize>{size_of::<T>().checked_mul(11)?.checked_add(align_of::<T>().checked_mul(3)?)?.checked_add(256)?.checked_mul(length.checked_add(1)?.ilog2() as usize+3)}
fn hash_allocation<T>(length:usize)->Option<usize>{if length==0{return Some(0)}length.checked_mul(2)?.checked_next_power_of_two()?.max(4).checked_mul(size_of::<T>().checked_add(1)?)?.checked_add(128)}

pub fn hash_map<K,V>(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<std::collections::HashMap<K,V>,ValueError>
where K:FromValue+std::str::FromStr+std::hash::Hash+Eq,K::Err:std::fmt::Display,V:FromValue{
    let DslValue::Object(entries)=value else{return Err(ValueError::new(crate::ValueRefusalKind::InvalidValue, "expected object"))};
    control.scoped_stage(|control|{
        control.begin_stage(entries.len())?;
        control.charge(hash_allocation::<(K,V)>(entries.len()).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "hash map allocation overflow"))?)?;
        let mut map=std::collections::HashMap::<K,V>::new();map.try_reserve(entries.len()).map_err(|_|ValueError::new(crate::ValueRefusalKind::AllocationFailed, "hash map allocation failed"))?;let mut output=map.guard_decoded();
        for(key,value)in entries{
            let key=K::from_object_key_controlled(key,control)?.guard_decoded();let value=V::from_value_controlled(value,control)?.guard_decoded();
            if let Some(previous)=output.get_mut().get_mut(key.get()){V::retire_decoded(std::mem::replace(previous,value.take()));}else{output.get_mut().insert(key.take(),value.take());}
            control.step()?;
        }
        Ok(output.take())
    })
}
