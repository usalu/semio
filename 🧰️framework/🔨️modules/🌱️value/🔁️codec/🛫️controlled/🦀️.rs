//! 🛫️ Borrowed projection into admitted intrinsic native output.
use super::{DslValue,ToValue,FromValue,ValueError,NativeEncodeControl,DecodedValue};

pub fn scalar(control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{control.scoped_stage(|control|{control.begin_stage(1)?;control.step()}).map_err(ValueError::new)}

pub fn sequence<'a,T:ToValue+'a>(items:impl ExactSizeIterator<Item=&'a T>,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
 control.scoped_depth(64,|control|control.scoped_stage(|control|{
  control.begin_stage(items.len()).map_err(ValueError::new)?;
  let mut output=Vec::<DslValue>::guard_decoded(control.allocate_vec(items.len()).map_err(ValueError::new)?);
  for(index,value)in items.enumerate(){output.get_mut().push(value.to_value_controlled(control).map_err(|error|error.under(index))?);control.step().map_err(ValueError::new)?;}
  Ok(DslValue::Array(output.take()))
 }))
}

pub fn map<'a,K:ToValue+'a,V:ToValue+'a>(items:impl ExactSizeIterator<Item=(&'a K,&'a V)>,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
 control.scoped_depth(64,|control|control.scoped_stage(|control|{
  control.begin_stage(items.len()).map_err(ValueError::new)?;
  let mut output=object(items.len(),control)?;
  for(key,value)in items{let key=key.to_object_key_controlled(control)?;let child=value.to_value_controlled(control)?;output.get_mut().push((key,child));control.step().map_err(ValueError::new)?;}
  Ok(DslValue::Object(output.take()))
 }))
}

pub fn object(count:usize,control:&mut NativeEncodeControl<'_>)->Result<DecodedValue<Vec<(String,DslValue)>>,ValueError>{Ok(DecodedValue::new(control.allocate_vec(count).map_err(ValueError::new)?,|items|{for(_,value)in items{super::controlled::retire_intrinsic(value)}}))}

struct StackText{bytes:[u8;4096],length:usize}
impl std::fmt::Write for StackText{fn write_str(&mut self,text:&str)->std::fmt::Result{let end=self.length.checked_add(text.len()).filter(|end|*end<=self.bytes.len()).ok_or(std::fmt::Error)?;self.bytes[self.length..end].copy_from_slice(text.as_bytes());self.length=end;Ok(())}}
impl StackText{fn new()->Self{Self{bytes:[0;4096],length:0}}fn text(&self)->&str{std::str::from_utf8(&self.bytes[..self.length]).expect("formatted UTF-8")}}

pub fn key(value:impl std::fmt::Display,control:&mut NativeEncodeControl<'_>)->Result<String,ValueError>{use std::fmt::Write;let mut text=StackText::new();control.checkpoint().map_err(ValueError::new)?;write!(text,"{value}").map_err(|_|ValueError::new("native numeric key exceeds bounded formatting"))?;control.copy_text(text.text()).map_err(ValueError::new)}

pub fn float32(value:f32,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
 use std::fmt::Write;scalar(control)?;let bits=value.to_bits();let payload=bits&0x007f_ffff;
 if bits&0x7f80_0000==0x7f80_0000&&payload!=0{return Ok(DslValue::Number(super::Number::Float(f64::from_bits((u64::from(bits&0x8000_0000)<<32)|0x7ff0_0000_0000_0000|(u64::from(payload)<<29)))))}
 let mut shortest=StackText::new();write!(shortest,"{value:e}").map_err(|_|ValueError::new("native float formatting overflow"))?;let precision=shortest.text().split('e').next().unwrap().bytes().filter(u8::is_ascii_digit).count().saturating_sub(1);let mut exact=StackText::new();write!(exact,"{value:.precision$e}").map_err(|_|ValueError::new("native float formatting overflow"))?;Ok(DslValue::Number(super::Number::Float(exact.text().parse().map_err(|_|ValueError::new("native float formatting failed"))?)))
}

fn push<T>(values:&mut Vec<T>,value:T,control:&mut NativeEncodeControl<'_>)->Result<(),(ValueError,T)>{
    if values.len()==values.capacity(){
        let next=values.capacity().max(1).checked_mul(2).and_then(|n|n.checked_mul(size_of::<T>())).filter(|bytes|*bytes<=isize::MAX as usize);
        let Some(bytes)=next else{return Err((ValueError::new("native frontier size overflow"),value))};
        if let Err(error)=control.charge(bytes){return Err((ValueError::new(error),value))}
        if values.try_reserve_exact(values.capacity().max(1)).is_err(){return Err((ValueError::new("native frontier allocation failed"),value))}
    }
    values.push(value);Ok(())
}

enum Task<'a>{Value(&'a DslValue),Array(usize),Object(&'a[(String,DslValue)])}

pub fn intrinsic(value:&DslValue,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
    match value{
     DslValue::Null=>{scalar(control)?;return Ok(DslValue::Null)},DslValue::Bool(value)=>{scalar(control)?;return Ok(DslValue::Bool(*value))},DslValue::Number(value)=>{scalar(control)?;return Ok(DslValue::Number(*value))},DslValue::String(value)=>return control.copy_text(value).map(DslValue::String).map_err(ValueError::new),DslValue::Bytes(value)=>return control.copy_bytes(value).map(DslValue::Bytes).map_err(ValueError::new),DslValue::Array(_)|DslValue::Object(_)=>{}
    }
    control.scoped_stage(|control|{
        control.begin_stage(0).map_err(ValueError::new)?;
        let mut pending=Vec::new();push(&mut pending,Task::Value(value),control).map_err(|(e,_)|e)?;
        let mut values=Vec::<DslValue>::new().guard_decoded();
        while let Some(task)=pending.pop(){
            control.step().map_err(ValueError::new)?;
            let result=match task{
                Task::Value(DslValue::Null)=>Some(DslValue::Null),
                Task::Value(DslValue::Bool(v))=>Some(DslValue::Bool(*v)),
                Task::Value(DslValue::Number(v))=>Some(DslValue::Number(*v)),
                Task::Value(DslValue::String(v))=>Some(DslValue::String(control.copy_text(v).map_err(ValueError::new)?)),
                Task::Value(DslValue::Bytes(v))=>Some(DslValue::Bytes(control.copy_bytes(v).map_err(ValueError::new)?)),
                Task::Value(DslValue::Array(items))=>{push(&mut pending,Task::Array(items.len()),control).map_err(|(e,_)|e)?;for item in items.iter().rev(){control.step().map_err(ValueError::new)?;push(&mut pending,Task::Value(item),control).map_err(|(e,_)|e)?;}None},
                Task::Value(DslValue::Object(entries))=>{push(&mut pending,Task::Object(entries),control).map_err(|(e,_)|e)?;for(_,item)in entries.iter().rev(){control.step().map_err(ValueError::new)?;push(&mut pending,Task::Value(item),control).map_err(|(e,_)|e)?;}None},
                Task::Array(count)=>{
                    let mut children=Vec::<DslValue>::guard_decoded(control.allocate_vec(count).map_err(ValueError::new)?);
                    let start=values.get().len()-count;for index in start..values.get().len(){control.step().map_err(ValueError::new)?;children.get_mut().push(std::mem::replace(&mut values.get_mut()[index],DslValue::Null));}values.get_mut().truncate(start);Some(DslValue::Array(children.take()))
                },
                Task::Object(entries)=>{
                    let mut children=DecodedValue::new(control.allocate_vec::<(String,DslValue)>(entries.len()).map_err(ValueError::new)?,|values|{for(_,v)in values{super::controlled::retire_intrinsic(v)}});
                    let start=values.get().len()-entries.len();
                    for(index,(key,_))in entries.iter().enumerate(){let key=control.copy_text(key).map_err(ValueError::new)?;let child=std::mem::replace(&mut values.get_mut()[start+index],DslValue::Null);children.get_mut().push((key,child));}
                    values.get_mut().truncate(start);Some(DslValue::Object(children.take()))
                }
            };
            if let Some(result)=result{if let Err((error,value))=push(values.get_mut(),result,control){super::controlled::retire_intrinsic(value);return Err(error)}}
        }
        Ok(values.get_mut().pop().expect("one completed intrinsic root"))
    })
}


impl DslValue{
 /// 🛡️ Holds produced native state until its parent publishes or retires it.
 pub fn guard_encoded(self)->DecodedValue<Self>{self.guard_decoded()}
 /// 🗂️ Admits a derived object's slots before any field is materialized.
 pub fn object_encoding_controlled(count:usize,control:&mut NativeEncodeControl<'_>)->Result<DecodedValue<Vec<(String,Self)>>,ValueError>{object(count,control)}
 /// 📎️ Transfers a completed field after admitting its key and any additional slots.
 pub fn push_encoding_controlled(entries:&mut Vec<(String,Self)>,key:&str,value:Self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  let value=value.guard_decoded();let key=control.copy_text(key).map_err(ValueError::new)?;
  if entries.len()==entries.capacity(){control.charge(entries.len().checked_add(1).and_then(|count|count.checked_mul(size_of::<(String,Self)>())).ok_or_else(||ValueError::new("native object size overflow"))?).map_err(ValueError::new)?;entries.try_reserve_exact(1).map_err(|_|ValueError::new("native object allocation failed"))?;}
  entries.push((key,value.take()));Ok(())
 }
 /// 🪗️ Transfers explicitly flattened object members with bounded slot ownership.
 pub fn flatten_encoding_controlled(entries:&mut Vec<(String,Self)>,value:Self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  let mut value=value.guard_decoded();let Self::Object(fields)=value.get_mut() else{return Ok(())};
  let additional=fields.len();let count=entries.len().checked_add(additional).ok_or_else(||ValueError::new("native object size overflow"))?;if count>entries.capacity(){control.charge(count.checked_mul(size_of::<(String,Self)>()).ok_or_else(||ValueError::new("native object size overflow"))?).map_err(ValueError::new)?;entries.try_reserve_exact(additional).map_err(|_|ValueError::new("native object allocation failed"))?;}
  control.scoped_stage(|control|{control.begin_stage(additional).map_err(ValueError::new)?;for(key,child)in fields.iter_mut(){entries.push((std::mem::take(key),std::mem::replace(child,Self::Null)));control.step().map_err(ValueError::new)?;}Ok(())})
 }
}


pub fn path(value:&std::path::Path,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{
 control.scoped_stage(|control|{
  control.begin_stage(0).map_err(ValueError::new)?;
  let mut size=0usize;path_text(value,|text|{size=size.checked_add(text.len()).filter(|size|*size<=control.maximum_bytes().saturating_sub(control.owned_bytes())).ok_or_else(||ValueError::new("native path ownership exceeds caller limit"))?;control.step().map_err(ValueError::new)})?;
  let mut output=control.allocate_vec::<u8>(size).map_err(ValueError::new)?;
  path_text(value,|text|{output.extend_from_slice(text.as_bytes());control.step().map_err(ValueError::new)})?;
  Ok(DslValue::String(String::from_utf8(output).map_err(|_|ValueError::new("native path UTF-8 construction failed"))?))
 })
}
#[cfg(unix)]
fn path_text(value:&std::path::Path,mut append:impl FnMut(&str)->Result<(),ValueError>)->Result<(),ValueError>{
 use std::os::unix::ffi::OsStrExt;let bytes=value.as_os_str().as_bytes();let mut position=0;
 while position<bytes.len(){let first=bytes[position];let width=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>{append("\u{fffd}")?;position+=1;continue}};let end=position.saturating_add(width).min(bytes.len());match std::str::from_utf8(&bytes[position..end]){Ok(text)=>{append(text)?;position=end},Err(error)=>{append("\u{fffd}")?;position+=error.error_len().unwrap_or(end-position);}}}Ok(())
}
#[cfg(windows)]
fn path_text(value:&std::path::Path,mut append:impl FnMut(&str)->Result<(),ValueError>)->Result<(),ValueError>{use std::os::windows::ffi::OsStrExt;for character in char::decode_utf16(value.as_os_str().encode_wide()){let mut buffer=[0;4];append(character.unwrap_or(char::REPLACEMENT_CHARACTER).encode_utf8(&mut buffer))?;}Ok(())}
#[cfg(not(any(unix,windows)))]
fn path_text(_value:&std::path::Path,_append:impl FnMut(&str)->Result<(),ValueError>)->Result<(),ValueError>{Err(ValueError::new("native path encoding is unavailable on this platform"))}
