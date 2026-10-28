//! 🛬️ The projection vocabulary borrows every defining construction cause without a diagnostic mirror.
use super::{FieldProjectionSource,FieldProjectionView as V,projection_path_error};
use semio_framework_value::{ValueError,RetainedCloneProgress,Number};
use semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary;
#[derive(Clone,Copy)]
enum Node<'a>{Receiving(&'a OriginalReceivingPrimary),Value(&'a ValueError),Progress(RetainedCloneProgress),Text(&'a str),UInt(usize)}
impl<'a> Node<'a>{
 fn key(self,index:usize)->Result<&'static str,ValueError>{match self{
  Self::Receiving(value)=>[value.primary.as_ref().map(|_|"primary"),value.quote_refusal.as_ref().map(|_|"quoteRefusal"),value.allocation_refusal.as_ref().map(|_|"allocationRefusal"),Some("retainedProgress")].into_iter().flatten().nth(index).ok_or_else(projection_path_error),
  Self::Value(_)=>["kind","message","retainedProgress"].get(index).copied().ok_or_else(projection_path_error),
  Self::Progress(_)=>["copiedItems","copiedBytes","retainedCapacityBytes","releasedBytes"].get(index).copied().ok_or_else(projection_path_error),_=>Err(projection_path_error())
 }}
 fn child(self,index:usize)->Result<Self,ValueError>{Ok(match self{
  Self::Receiving(value)=>match self.key(index)?{"primary"=>Self::Value(value.primary.as_ref().unwrap()),"quoteRefusal"=>Self::Value(value.quote_refusal.as_ref().unwrap()),"allocationRefusal"=>Self::Value(value.allocation_refusal.as_ref().unwrap()),"retainedProgress"=>Self::Progress(value.progress),_=>return Err(projection_path_error())},
  Self::Value(value)=>match index{0=>Self::Text(value.kind.as_str()),1=>Self::Text(&value.message),2=>Self::Progress(value.retained_progress()),_=>return Err(projection_path_error())},
  Self::Progress(value)=>Self::UInt(*[value.copied_items,value.copied_bytes,value.retained_capacity_bytes,value.released_bytes].get(index).ok_or_else(projection_path_error)?),_=>return Err(projection_path_error())
 })}
 fn view(self)->V<'a>{match self{Self::Receiving(value)=>V::IntrinsicObject(1+usize::from(value.primary.is_some())+usize::from(value.quote_refusal.is_some())+usize::from(value.allocation_refusal.is_some())),Self::Value(_)=>V::IntrinsicObject(3),Self::Progress(_)=>V::IntrinsicObject(4),Self::Text(value)=>V::IntrinsicText(value),Self::UInt(value)=>V::IntrinsicNumber(Number::UInt(value as u64))}}
}
fn locate<'a>(value:&'a OriginalReceivingPrimary,path:&[usize])->Result<Node<'a>,ValueError>{let mut node=Node::Receiving(value);for &index in path{node=node.child(index)?}Ok(node)}
impl FieldProjectionSource for OriginalReceivingPrimary{
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{Ok(locate(self,path)?.view())}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{locate(self,path)?.key(index)}
}

#[cfg(test)]
mod tests{
 use super::*;
 fn progress(value:&serde_json::Value)->RetainedCloneProgress{RetainedCloneProgress{copied_items:value["copiedItems"].as_u64().unwrap()as usize,copied_bytes:value["copiedBytes"].as_u64().unwrap()as usize,retained_capacity_bytes:value["retainedCapacityBytes"].as_u64().unwrap()as usize,released_bytes:value["releasedBytes"].as_u64().unwrap()as usize}}
 fn json(source:&impl FieldProjectionSource,path:&mut Vec<usize>)->serde_json::Value{match source.projection_view(path).unwrap(){V::IntrinsicObject(length)=>{let mut object=serde_json::Map::new();for index in 0..length{let key=source.projection_key(path,index).unwrap().to_owned();path.push(index);let value=json(source,path);path.pop();object.insert(key,value);}object.into()},V::IntrinsicText(value)=>value.into(),V::IntrinsicNumber(Number::UInt(value))=>value.into(),_=>panic!("unexpected receiving fixture node")}}
 #[test]
 fn defining_receiving_projection_matches_neutral_causes_and_borrows_original_messages(){
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧵️job/⚠️failure/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let row=&fixture["receivingWire"]["primary"]["original"];
  let cause=|name:&str|{let value=&row[name];ValueError::new(semio_framework_value::ValueRefusalKind::from_wire(value["kind"].as_str().unwrap()).unwrap(),value["message"].as_str().unwrap()).with_retained_progress(progress(&value["retainedProgress"]))};
  let source=OriginalReceivingPrimary{primary:Some(cause("primary")),quote_refusal:Some(cause("quoteRefusal")),allocation_refusal:Some(cause("allocationRefusal")),progress:progress(&row["retainedProgress"])};
  assert_eq!(json(&source,&mut Vec::new()),*row);
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{for(index,value)in[&source.primary,&source.quote_refusal,&source.allocation_refusal].iter().enumerate(){assert!(matches!(source.projection_view(&[index,1]).unwrap(),V::IntrinsicText(message)if message.as_ptr()==value.as_ref().unwrap().message.as_ptr()));}assert!(source.projection_view(&[99]).is_err());});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }
}
