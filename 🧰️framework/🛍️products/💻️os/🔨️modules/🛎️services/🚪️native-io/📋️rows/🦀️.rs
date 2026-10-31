//! 📋️ Native path results retain only actual initialized rows and release each original on its own turn.
use semio_framework_value_native_storage::NativeOwnedPrefix;
use semio_framework_value_control_contract::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind};
pub const NATIVE_IO_PATH_CAPACITY:usize=256;
use std::{mem::ManuallyDrop,path::PathBuf};
macro_rules! native_rows{
 ($name:ident,$item:ty,$row:ident=>$path:expr)=>{
  pub struct $name{entries:NativeOwnedPrefix<$item,NATIVE_IO_PATH_CAPACITY>,retiring:ManuallyDrop<Option<$item>>}
  impl $name{
   pub const fn new()->Self{Self{entries:NativeOwnedPrefix::empty(),retiring:ManuallyDrop::new(None)}}
   pub fn len(&self)->usize{self.entries.len()}
   pub fn is_empty(&self)->bool{self.entries.is_empty()&&self.retiring.is_none()}
   pub fn as_slice(&self)->&[$item]{self.entries.as_slice()}
   pub fn push_demand()->RetirementDemand{NativeOwnedPrefix::<$item,NATIVE_IO_PATH_CAPACITY>::push_demand()}
   pub fn push_from(&mut self,input:&mut Option<$item>,g:RetainedCloneGrant)->RetainedWorkOutcome{if self.retiring.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing native rows cannot accept another original")),work:0}}self.entries.push_from(input,g)}
   pub fn pop_into(&mut self,output:&mut Option<$item>,g:RetainedCloneGrant)->RetainedWorkOutcome{if self.retiring.is_some(){return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"closing native rows retain an earlier original")),work:0}}self.entries.pop_into(output,g)}
   pub fn close_demand(&self)->RetirementDemand{if let Some($row)=self.retiring.as_ref(){RetirementDemand{release_bytes:($path).capacity(),depth:1,..Default::default()}}else if self.entries.is_empty(){Default::default()}else{RetirementDemand{depth:1,..Default::default()}}}
   pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome{
    if self.is_empty(){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
    if self.retiring.is_none(){return self.entries.pop_into(&mut self.retiring,g)}
    let d=self.close_demand();if !semio_framework_value_control_contract::retained_clone_grant_admits(g,d){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
    drop(self.retiring.take());let p=RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()};RetainedWorkOutcome{result:Ok(if self.is_empty(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}),work:1}
   }
  }
  impl Default for $name{fn default()->Self{Self::new()}}
  impl std::fmt::Debug for $name{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct(stringify!($name)).field("rows",&self.entries.as_slice()).field("retiring",&self.retiring.as_ref()).finish()}}
  impl PartialEq for $name{fn eq(&self,other:&Self)->bool{self.entries.as_slice()==other.entries.as_slice()&&*self.retiring==*other.retiring}}
  impl Eq for $name{}
  impl Drop for $name{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"native rows require paid original pop and release");if self.retiring.is_none(){unsafe{ManuallyDrop::drop(&mut self.retiring)}}}}
 };
}
native_rows!(NativePathSet,PathBuf,row=>row);
/// 🧺️ A single original path enters directly; larger inputs retain their already granted bounded rows.
#[derive(Debug)]
pub enum NativePathInput{Single(PathBuf),Rows(NativePathSet)}
native_rows!(NativeModifiedSet,(PathBuf,std::time::SystemTime),row=>&row.0);
#[cfg(test)]#[path="tests.rs"]mod tests;
