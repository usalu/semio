//! 📄️ Fixed scalar buffers and explicit typed page metadata reserve for owned writers.
use semio_framework_value::list::PagedList;
pub(crate) const PAGE:usize=3072;
pub(crate) struct Fragment{pub bytes:[u8;4096],pub len:usize,pub at:usize}
impl Default for Fragment{fn default()->Self{Self{bytes:[0;4096],len:0,at:0}}}
impl std::fmt::Write for Fragment{fn write_str(&mut self,value:&str)->std::fmt::Result{self.write_bytes(value.as_bytes())}}
impl Fragment{
 pub fn reset(&mut self){self.len=0;self.at=0;}
 pub fn write_bytes(&mut self,value:&[u8])->std::fmt::Result{let end=self.len.checked_add(value.len()).ok_or(std::fmt::Error)?;if end>self.bytes.len(){return Err(std::fmt::Error);}self.bytes[self.len..end].copy_from_slice(value);self.len=end;Ok(())}
}
pub(crate) fn reserve<T>(values:&mut PagedList<T,{usize::MAX}>)->Result<bool,String>{if values.has_reserved_slot(){return Ok(true);}let bytes=values.next_allocation_bytes().map_err(|error|error.to_string())?;values.reserve_one(bytes).map_err(|error|semio_framework_value::ValueError::from(error.refusal()).into_message())?;Ok(false)}

#[cfg(test)]
pub(crate) fn physical_close<T:semio_framework_value::retirement::RetireOwned>(value:T){let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|panic!("owned writer close admission"));for _ in 0..2000000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};let(zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(zero.progress(),Default::default());let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(!heap.overflowed);assert!(step.progress().fits(grant));assert_eq!(heap.requested_bytes,step.progress().retained_capacity_bytes);assert_eq!(heap.released_bytes,step.progress().released_bytes);}assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
