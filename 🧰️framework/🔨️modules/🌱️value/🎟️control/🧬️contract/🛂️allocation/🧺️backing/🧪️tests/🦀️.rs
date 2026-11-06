//! 🧪️ Exact native backing layouts preserve neutral widths, empty ownership and aligned vector capacity.
use super::*;
use std::mem::size_of;
use semio_framework_trace_memory_testing::{observe_backing,refuse_one_allocation_on_this_thread,RequestedAllocator};
#[global_allocator]
static ALLOCATOR:RequestedAllocator=RequestedAllocator;
#[repr(align(64))]
#[derive(Default)]
struct Wide{_byte:u8}
fn vector<T:Default>(count:usize,bytes:usize){assert_eq!(exact_vector_bytes::<T>(count).unwrap(),bytes);let(mut value,born,released)=observe_backing(||allocate_exact_vector::<T>(count).unwrap());assert_eq!((born,released),(bytes,0));assert!(value.is_empty());if size_of::<T>()!=0{assert_eq!(value.capacity(),count);assert_eq!(value.capacity()*size_of::<T>(),bytes)}let pointer=value.as_ptr();let((),born,released)=observe_backing(||{for _ in 0..count{value.push(T::default())}});assert_eq!((born,released),(0,0));assert_eq!(value.as_ptr(),pointer);assert_eq!(value.len(),count);let((),born,released)=observe_backing(||drop(value));assert_eq!((born,released),(0,bytes));}
fn string(bytes:usize){let(mut text,born,released)=observe_backing(||allocate_exact_string(bytes).unwrap());assert_eq!((born,released),(bytes,0));assert!(text.is_empty());assert_eq!(text.capacity(),bytes);let pointer=text.as_ptr();let((),born,released)=observe_backing(||{if bytes>=2{text.push_str("é")}});assert_eq!((born,released),(0,0));assert_eq!(text.as_ptr(),pointer);if bytes>=2{assert_eq!(text,"é")}let((),born,released)=observe_backing(||drop(text));assert_eq!((born,released),(0,bytes));}
#[test]
fn exact_backings_match_neutral_capacity_and_empty_utf8(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in fixture["vectors"].as_array().unwrap(){let n=row["count"].as_u64().unwrap()as usize;let bytes=row["bytes"].as_u64().unwrap()as usize;match row["kind"].as_str().unwrap(){"byte"=>vector::<u8>(n,bytes),"word"=>vector::<u64>(n,bytes),"wide"=>vector::<Wide>(n,bytes),"zero"=>vector::<()>(n,bytes),_=>unreachable!()}}for bytes in fixture["strings"].as_array().unwrap(){string(bytes.as_u64().unwrap()as usize);}}
#[test]
fn overflowing_exact_layouts_refuse_before_allocation(){assert_eq!(exact_vector_bytes::<u64>(usize::MAX).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(allocate_exact_vector::<u64>(usize::MAX).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(allocate_exact_string(usize::MAX).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(exact_vector_bytes::<()>(usize::MAX).unwrap(),0);assert!(allocate_exact_vector::<()>(usize::MAX).unwrap().is_empty());}

#[test]
fn exact_backing_allocation_failure_has_no_retained_owner(){let((result,requested,released),refused)=refuse_one_allocation_on_this_thread(||observe_backing(||allocate_exact_vector::<u64>(3)));assert_eq!(refused,1);assert_eq!((requested,released),(24,0));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::AllocationFailed);let((result,requested,released),refused)=refuse_one_allocation_on_this_thread(||observe_backing(||allocate_exact_string(7)));assert_eq!(refused,1);assert_eq!((requested,released),(7,0));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::AllocationFailed);}
