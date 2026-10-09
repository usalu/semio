//! 🧮️ Actual property constructor requests and exact cumulative admission.
use super::*;
use std::alloc::{GlobalAlloc,Layout,System};
std::thread_local!{static REQUEST_BYTES:std::cell::Cell<Option<usize>>=const{std::cell::Cell::new(None)};static SLOT_REQUESTS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
fn record(size:usize,alignment:usize){REQUEST_BYTES.with(|value|{if let Some(bytes)=value.get(){value.set(Some(bytes.checked_add(size).expect("property request observation overflow")));if alignment==std::mem::align_of::<(String,graph::manifest::PropertyValue)>(){SLOT_REQUESTS.with(|count|count.set(count.get()+1));}}});}
std::thread_local!{static HEAP:std::cell::Cell<Option<(usize,usize)>>=const{std::cell::Cell::new(None)};}
fn heap_event(born:usize,released:usize){HEAP.with(|value|{if let Some((a,b))=value.get(){value.set(Some((a+born,b+released)));}});}
pub(crate) fn heap<T>(operation:impl FnOnce()->T)->(T,usize,usize){let prior=HEAP.with(|value|value.replace(Some((0,0))));let result=operation();let(a,b)=HEAP.with(|value|value.replace(prior).unwrap());(result,a,b)}
struct PropertyAllocator;
unsafe impl GlobalAlloc for PropertyAllocator {
 unsafe fn alloc(&self,layout:Layout)->*mut u8{heap_event(layout.size(),0);record(layout.size(),layout.align());unsafe{System.alloc(layout)}}
 unsafe fn alloc_zeroed(&self,layout:Layout)->*mut u8{heap_event(layout.size(),0);record(layout.size(),layout.align());unsafe{System.alloc_zeroed(layout)}}
 unsafe fn realloc(&self,pointer:*mut u8,layout:Layout,size:usize)->*mut u8{heap_event(size,layout.size());record(size,layout.align());unsafe{System.realloc(pointer,layout,size)}}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout){heap_event(0,layout.size());unsafe{System.dealloc(pointer,layout)}}
}
#[global_allocator]
static PROPERTY_ALLOCATOR:PropertyAllocator=PropertyAllocator;
struct Restore(Option<usize>,usize);
impl Drop for Restore{fn drop(&mut self){REQUEST_BYTES.with(|value|value.set(self.0));SLOT_REQUESTS.with(|value|value.set(self.1));}}
fn requests<T>(operation:impl FnOnce()->T)->(T,usize,usize){let restore=Restore(REQUEST_BYTES.with(|value|value.replace(Some(0))),SLOT_REQUESTS.with(|value|value.replace(0)));let result=operation();let bytes=REQUEST_BYTES.with(|value|value.get().unwrap());let slots=SLOT_REQUESTS.with(std::cell::Cell::get);drop(restore);(result,bytes,slots)}
#[test]
fn sqlite_snapshot_framework_dag_property_backing_is_admitted_before_actual_requests(){
 use graph::manifest::PropertyBag;
 use semio_framework_value::{NativeDecodeControl,ValueRefusalKind};
 for cardinality in laws()["propertyMembers"]["cardinalities"].as_array().unwrap(){
  let count=cardinality.as_u64().unwrap()as usize;
  let value=DslValue::Object((0..count).map(|index|("\0".repeat(index),DslValue::float(index as f64))).collect());
  let mut yes=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut yes);
  let(result,bytes,_)=requests(||PropertyBag::from_value_controlled(&value,&mut control));
  let owner=result.unwrap();assert_eq!(owner.len(),count);
  assert!(control.owned_bytes()>=bytes,"owned {} < actual request aggregate {bytes}",control.owned_bytes());
  if count==0{assert_eq!(bytes,0);assert_eq!(control.owned_bytes(),laws()["propertyMembers"]["emptyOwnedBytes"].as_u64().unwrap()as usize);}
  let exact=control.owned_bytes();PropertyBag::retire_decoded(owner);
  let mut yes=|_|true;let mut control=NativeDecodeControl::new(exact,&mut yes);
  let(result,_,_)=requests(||PropertyBag::from_value_controlled(&value,&mut control));PropertyBag::retire_decoded(result.unwrap());assert_eq!(control.owned_bytes(),exact);
  if bytes>0{let mut yes=|_|true;let mut control=NativeDecodeControl::new(0,&mut yes);let(result,_,denied_slots)=requests(||PropertyBag::from_value_controlled(&value,&mut control));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(denied_slots,0);assert_eq!(control.owned_bytes(),0);}
  if exact>0{let mut yes=|_|true;let mut control=NativeDecodeControl::new(exact-1,&mut yes);let error=PropertyBag::from_value_controlled(&value,&mut control).unwrap_err();assert_eq!(error.kind,ValueRefusalKind::OwnershipLimit);}
 }
}
