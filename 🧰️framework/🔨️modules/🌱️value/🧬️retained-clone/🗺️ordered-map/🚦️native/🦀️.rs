//! 🚦️ Dictionary wire encoding and cumulative native admission for fixed owned pages.
use super::{RetainedOrderedMap,RETAINED_ORDERED_MAP_PAGE_CAPACITY};
use crate::{DecodedValue,DslValue,FromValue,ToValue,ValueError,ValueRefusalKind,NativeDecodeControl,NativeEncodeControl,ValueShape};

impl<V:ToValue> ToValue for RetainedOrderedMap<String,V>{
 fn to_value(&self)->DslValue{DslValue::Object(self.iter().map(|(key,value)|(key.clone(),value.to_value())).collect())}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{
  control.begin_stage(self.len())?;let mut output=DecodedValue::new(control.allocate_vec::<(String,DslValue)>(self.len())?,|entries|{for(_,value)in entries{DslValue::retire_decoded(value)}});
  for(key,value)in self.iter(){let key=control.copy_text(key)?;let child=value.to_value_controlled(control)?;output.get_mut().push((key,child));control.step()?;}Ok(DslValue::Object(output.take()))
 }))}
 fn value_at_path(&self,path:&[&str])->Result<DslValue,ValueError>{let Some((segment,rest))=path.split_first()else{return Ok(self.to_value())};self.get(*segment).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"missing ordered-map key"))?.value_at_path(rest).map_err(|error|error.under(segment))}
 fn value_shape_at_path(&self,path:&[&str])->Result<ValueShape,ValueError>{let Some((segment,rest))=path.split_first()else{return Ok(ValueShape::Object{len:self.len()})};self.get(*segment).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"missing ordered-map key"))?.value_shape_at_path(rest).map_err(|error|error.under(segment))}
 fn value_key_at_path(&self,path:&[&str],index:usize)->Result<String,ValueError>{let Some((segment,rest))=path.split_first()else{return self.get_index(index).map(|(key,_)|key.clone()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"ordered-map key ordinal exceeds length"))};self.get(*segment).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"missing ordered-map key"))?.value_key_at_path(rest,index).map_err(|error|error.under(segment))}
}
impl<V:FromValue> FromValue for RetainedOrderedMap<String,V>{
 fn from_value(value:DslValue)->Result<Self,ValueError>{
  let DslValue::Object(entries)=value else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"ordered map requires an object"))};
  let mut map=Self::default();for(key,value)in entries{if map.get(key.as_str()).is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"duplicate ordered-map key"))};map.cold_insert(key,V::from_value(value)?);}Ok(map)
 }
 fn from_value_controlled(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.scoped_depth(64,|control|control.scoped_stage(|control|{
  let entries=value.object_controlled(control)?;control.begin_stage(entries.len())?;
  let mut order=control.allocate_vec::<usize>(entries.len())?;
  for index in 0..entries.len(){let mut position=order.len();while position>0&&compare(&entries[order[position-1]].0,&entries[index].0,control)?.is_gt(){position-=1;}order.insert(position,index);control.step()?;}
  let mut output=Self::default().guard_decoded();output.get_mut().pages=control.allocate_vec(entries.len().div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY))?;
  for index in order{
   if output.get().len%RETAINED_ORDERED_MAP_PAGE_CAPACITY==0{let page=control.allocate_vec::<(String,V)>(RETAINED_ORDERED_MAP_PAGE_CAPACITY)?;output.get_mut().pages.push(page);}
   let(key,value)=&entries[index];let key=String::from_object_key_controlled(key,control)?;let child=V::from_value_controlled(value,control)?;
   output.get_mut().pages.last_mut().expect("admitted page").push((key,child));output.get_mut().len+=1;control.step()?;
  }
  Ok(output.take())
 }))}
 fn default_value_controlled(control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;Ok(Self::default())}
 fn retire_decoded(self){for page in self.pages{for(key,value)in page{String::retire_decoded(key);V::retire_decoded(value);}}}
}
fn compare(left:&str,right:&str,control:&mut NativeDecodeControl<'_>)->Result<std::cmp::Ordering,ValueError>{control.scoped_stage(|control|{
 control.begin_stage(left.len().min(right.len()).saturating_add(1))?;
 for(left,right)in left.bytes().zip(right.bytes()){control.step()?;let order=left.cmp(&right);if !order.is_eq(){return Ok(order)}}control.step()?;Ok(left.len().cmp(&right.len()))
})}