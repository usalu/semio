//! \u{1f52c}\uFE0F Workflow test requests are independently delegated to the system allocator.
use std::{alloc::{GlobalAlloc,Layout,System},cell::Cell};
std::thread_local!{static ACTIVE:Cell<bool>=const{Cell::new(false)};static REQUESTED:Cell<usize>=const{Cell::new(0)};}
fn request(bytes:usize){let _=ACTIVE.try_with(|active|{if active.get(){let _=REQUESTED.try_with(|requested|requested.set(requested.get().checked_add(bytes).expect("bounded Workflow test requests")));}});}
struct Allocator;
unsafe impl GlobalAlloc for Allocator{
 unsafe fn alloc(&self,layout:Layout)->*mut u8{request(layout.size());unsafe{System.alloc(layout)}}
 unsafe fn alloc_zeroed(&self,layout:Layout)->*mut u8{request(layout.size());unsafe{System.alloc_zeroed(layout)}}
 unsafe fn realloc(&self,pointer:*mut u8,layout:Layout,size:usize)->*mut u8{request(size);unsafe{System.realloc(pointer,layout,size)}}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout){unsafe{System.dealloc(pointer,layout)}}
}
#[global_allocator]static SYSTEM_REQUESTS:Allocator=Allocator;
struct Restore;
impl Drop for Restore{fn drop(&mut self){ACTIVE.with(|active|active.set(false));}}
pub(super) fn observe<T>(operation:impl FnOnce()->T)->(T,usize){ACTIVE.with(|active|assert!(!active.get(),"non-nested Workflow allocation observation"));REQUESTED.with(|requested|requested.set(0));ACTIVE.with(|active|active.set(true));let restore=Restore;let result=operation();drop(restore);(result,REQUESTED.with(Cell::get))}
