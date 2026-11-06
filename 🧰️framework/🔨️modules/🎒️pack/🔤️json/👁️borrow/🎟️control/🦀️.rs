//! 🎟️ Canonical neutral grant and progress records borrow their actual scalar fields.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
macro_rules! record{($ty:ident;$($index:literal:$field:ident=>$key:literal),+)=>{
 const _:fn(&$ty)=|value|{let $ty{$($field,)+}=value;$(let _: &usize=$field;)+};
 impl JsonNativeBorrow for $ty{
  fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Object([$($key,)+].len()))}
  fn json_child<'a>(&'a self,mode:u8,index:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}match index{$($index=>Ok(json_borrow_native(&self.$field)),)+_=>Err(json_borrow_absent())}}
  fn json_key<'a>(&'a self,mode:u8,index:usize,_:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{if mode!=0{return Err(json_borrow_absent())}match index{$($index=>Ok($key),)+_=>Err(json_borrow_absent())}}
 }
}}
record!(RetainedCloneGrant;0:maximum_items=>"maximumItems",1:maximum_copy_bytes=>"maximumCopyBytes",2:maximum_capacity_bytes=>"maximumCapacityBytes",3:maximum_release_bytes=>"maximumReleaseBytes",4:maximum_depth=>"maximumDepth");
record!(RetainedCloneProgress;0:copied_items=>"copiedItems",1:copied_bytes=>"copiedBytes",2:retained_capacity_bytes=>"retainedCapacityBytes",3:released_bytes=>"releasedBytes");
