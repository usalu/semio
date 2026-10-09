//! 🖼️ Packet asset entries use original paged backing and explicit candidate admission.
use super::{DrawingImageAsset,MAX_NODES,MAX_ID_BYTES};
use semio_framework_value::{DslValue,ToValue,FromValue,ValueError,ValueRefusalKind,NativeDecodeControl,NativeEncodeControl,list::PagedList};
#[derive(Clone,Debug,Default,PartialEq,semio_framework_value::RetainedClone,semio_framework_value::RetireOwned)]
pub struct DrawingClipboardAssets{entries:PagedList<(String,DrawingImageAsset),MAX_NODES>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Invalid drawing clipboard asset entry")}
impl DrawingClipboardAssets{
 pub fn len(&self)->usize{self.entries.len()}
 pub fn is_empty(&self)->bool{self.entries.is_empty()}
 pub fn iter(&self)->impl DoubleEndedIterator<Item=(&String,&DrawingImageAsset)>+ExactSizeIterator{self.entries.iter().map(|(key,value)|(key,value))}
 pub fn keys(&self)->impl DoubleEndedIterator<Item=&String>+ExactSizeIterator{self.iter().map(|(key,_)|key)}
 pub fn values(&self)->impl DoubleEndedIterator<Item=&DrawingImageAsset>+ExactSizeIterator{self.iter().map(|(_,value)|value)}
 pub fn entry_at(&self,index:usize)->Option<(&String,&DrawingImageAsset)>{self.entries.get(index).map(|(key,value)|(key,value))}
 pub fn entry_at_mut(&mut self,index:usize)->Option<(&String,&mut DrawingImageAsset)>{self.entries.get_mut(index).map(|(key,value)|(&*key,value))}
 pub fn get(&self,key:&str)->Option<&DrawingImageAsset>{self.iter().find(|(candidate,_)|candidate.as_str()==key).map(|(_,value)|value)}
 pub fn get_mut(&mut self,key:&str)->Option<&mut DrawingImageAsset>{self.entries.iter_mut().find(|(candidate,_)|candidate.as_str()==key).map(|(_,value)|value)}
 pub fn contains_key(&self,key:&str)->bool{self.get(key).is_some()}
 pub fn append_candidate(&mut self,candidate:&mut Option<(String,DrawingImageAsset)>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  let mut ordinal=0;while !self.append_candidate_step(candidate,&mut ordinal,control)?{}Ok(())
 }
 /// 🔎️ Advances one bounded key comparison while the candidate and existing entries remain immutable.
 pub(crate) fn append_candidate_step(&mut self,candidate:&mut Option<(String,DrawingImageAsset)>,ordinal:&mut usize,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{
  control.checkpoint()?;let (key,_)=candidate.as_ref().ok_or_else(invalid)?;if key.is_empty()||key.len()>MAX_ID_BYTES||self.len()==MAX_NODES||*ordinal>self.len(){return Err(invalid());}
  if let Some((existing,_))=self.entry_at(*ordinal){if existing==key{return Err(invalid());}*ordinal+=1;return Ok(false);}
  while !self.entries.has_reserved_slot(){let bytes=self.entries.next_allocation_bytes()?;control.charge(bytes)?;self.entries.reserve_one(bytes).map_err(|error|ValueError::from(error.refusal()))?;}
  self.entries.push_reserved(candidate.take().unwrap()).map_err(|original|{*candidate=Some(original);invalid()})?;*ordinal=0;Ok(true)
 }
 pub fn insert(&mut self,key:String,value:DrawingImageAsset)->Option<DrawingImageAsset>{
  if let Some(original)=self.get_mut(&key){return Some(std::mem::replace(original,value));}
  let mut candidate=Some((key,value));let mut accepted=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut accepted);self.append_candidate(&mut candidate,&mut control).expect("cold packet asset construction");None
 }
}
impl IntoIterator for DrawingClipboardAssets{type Item=(String,DrawingImageAsset);type IntoIter=<PagedList<Self::Item,MAX_NODES>as IntoIterator>::IntoIter;fn into_iter(self)->Self::IntoIter{self.entries.into_iter()}}
impl<'a> IntoIterator for &'a DrawingClipboardAssets{type Item=(&'a String,&'a DrawingImageAsset);type IntoIter=std::iter::Map<<&'a PagedList<(String,DrawingImageAsset),MAX_NODES>as IntoIterator>::IntoIter,fn(&'a(String,DrawingImageAsset))->Self::Item>;fn into_iter(self)->Self::IntoIter{(&self.entries).into_iter().map(|(key,value)|(key,value))}}
impl ToValue for DrawingClipboardAssets{
 fn to_value(&self)->DslValue{DslValue::Object(self.iter().map(|(key,value)|(key.clone(),value.to_value())).collect())}
 fn to_value_controlled(&self,control:&mut NativeEncodeControl<'_>)->Result<DslValue,ValueError>{let mut entries=control.allocate_vec(self.len())?;for(key,value)in self.iter(){entries.push((control.copy_text(key)?,value.to_value_controlled(control)?));}Ok(DslValue::Object(entries))}
}
impl FromValue for DrawingClipboardAssets{
 fn from_value(value:DslValue)->Result<Self,ValueError>{let DslValue::Object(entries)=value else{return Err(invalid());};if entries.len()>MAX_NODES{return Err(invalid());}let mut output=Self::default();for(key,value)in entries{if key.is_empty()||key.len()>MAX_ID_BYTES||output.contains_key(&key){return Err(invalid());}output.insert(key,DrawingImageAsset::from_value(value)?);}Ok(output)}
 fn from_value_controlled(value:&DslValue,_control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let DslValue::Object(entries)=value else{return Err(invalid());};if entries.is_empty(){Ok(Self::default())}else{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"nonempty packet assets require retained fragment hydration"))}}
}
#[cfg(test)]
impl serde::Serialize for DrawingClipboardAssets{fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeMap;let mut map=serializer.serialize_map(Some(self.len()))?;for(key,value)in self.iter(){map.serialize_entry(key,value)?;}map.end()}}
#[cfg(test)]
impl<'de> serde::Deserialize<'de> for DrawingClipboardAssets{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let entries=<std::collections::BTreeMap<String,DrawingImageAsset>as serde::Deserialize>::deserialize(deserializer)?;let mut output=Self::default();for(key,value)in entries{if key.is_empty()||key.len()>MAX_ID_BYTES||output.len()==MAX_NODES{return Err(serde::de::Error::custom("invalid packet asset capacity or identity"));}output.insert(key,value);}Ok(output)}}
