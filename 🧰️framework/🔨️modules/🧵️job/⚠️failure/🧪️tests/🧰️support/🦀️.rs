//! 🧰️ Shared real payloads, borrowed projections and paid-close witnesses for owning Original laws.
use crate::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V,close_original_retained_step,close_original_effect_step,project_original_fault,key_original_fault};
use semio_framework_diagnostic::FaultMessageOwner;
use semio_framework_value::{ValueError,ValueRefusalKind,Number,RetainedCloneProgress as P,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use serde_json::{Value,json};
pub(crate) fn clock()->Option<u64>{Some(0)}
#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct BodyFailure{pub(crate) ordinal:u64,pub(crate) owner:FaultMessageOwner,pub(crate) progress:P}
impl std::fmt::Debug for BodyFailure{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("BodyFailure").field("ordinal",&self.ordinal).finish_non_exhaustive()}}
impl OriginalDomainFailure for BodyFailure{
 fn domain_code(&self)->&str{"fixture.body"}
 fn domain_message(&self)->&str{&self.owner.fault().unwrap().message}
 fn domain_retained_progress(&self)->P{self.progress}
}
pub(crate) fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"fixture body projection changed")}
impl OriginalDomainProjection for BodyFailure{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::IntrinsicObject(2)),[0]=>Ok(V::IntrinsicNumber(Number::UInt(self.ordinal))),[1,rest @ ..]=>project_original_fault(self.owner.fault().unwrap(),rest),_=>Err(invalid())}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{[]=>["ordinal","fault"].get(index).copied().ok_or_else(invalid),[1,rest @ ..]=>key_original_fault(self.owner.fault().unwrap(),rest,index),_=>Err(invalid())}}
}
pub(crate) fn delta(before:P,after:P)->P{P{copied_items:after.copied_items.checked_sub(before.copied_items).unwrap(),copied_bytes:after.copied_bytes.checked_sub(before.copied_bytes).unwrap(),retained_capacity_bytes:after.retained_capacity_bytes.checked_sub(before.retained_capacity_bytes).unwrap(),released_bytes:after.released_bytes.checked_sub(before.released_bytes).unwrap()}}
pub(crate) fn wire_progress(p:P)->Value{json!({"copiedItems":p.copied_items,"copiedBytes":p.copied_bytes,"retainedCapacityBytes":p.retained_capacity_bytes,"releasedBytes":p.released_bytes})}
pub(crate) fn wire_value(kind:&str,message:&str,progress:P)->Value{json!({"kind":kind,"message":message,"retainedProgress":wire_progress(progress)})}
pub(crate) fn independent_body(ordinal:u64,message:&str)->Value{json!({"ordinal":ordinal,"fault":{"origin":"app","code":"app.message","severity":"error","message":message,"scope":{},"retainedProgress":wire_progress(P::default()),"retryable":false}})}
pub(crate) fn projected(source:&dyn OriginalDomainProjection,path:&mut Vec<usize>)->Value{
 match source.projection_view(path).unwrap(){
  V::IntrinsicObject(len)=>{let mut object=serde_json::Map::new();for index in 0..len{let key=source.projection_key(path,index).unwrap().to_owned();path.push(index);let child=projected(source,path);path.pop();object.insert(key,child);}Value::Object(object)},
  V::IntrinsicArray(len)=>{let mut values=Vec::new();for index in 0..len{path.push(index);values.push(projected(source,path));path.pop();}Value::Array(values)},
  V::IntrinsicText(text)=>json!(text),V::IntrinsicBool(value)=>json!(value),V::IntrinsicNumber(Number::UInt(value))=>json!(value),_=>panic!("unexpected fixture projection")
 }
}
pub(crate) fn close<T:RetireOwned>(value:T,cx:&mut crate::StepContext<'_>,bound:usize){
 let(mut owner,heap)=observe(||ControlledRetirement::new(value).unwrap_or_else(|_|panic!("fixture owner must support controlled close")));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 for _ in 0..bound{if owner.terminal_is_empty(){break}let grant=cx.retained_grant();let demand=owner.selected_demand(grant.maximum_copy_bytes).unwrap();let(result,heap)=observe(||close_original_effect_step(cx,demand,|effect|owner.step(grant.maximum_copy_bytes,effect)));let progress=result.unwrap().progress();assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));}
 assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
