//! 🕹️ Shared retained wire drivers expose one parser event and its funded closure.
use crate::RetainedValueToken as Token;
use semio_framework_replication_codec::PackLimits;
use semio_framework_pack_error::PackRefusal;
use semio_framework_value::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
/// 🪶️ Body and Document parsers share the same explicit retained source and closure protocol.
pub trait RetainedValueDriver:Sized{
 fn new(limits:PackLimits,length:usize,maximum:usize)->Result<Self,PackRefusal>;
 fn one(&mut self,input:&[u8],maximum:usize)->Result<Option<Token>,PackRefusal>;
 fn allocated_bytes(&self)->usize;
 fn input_bytes(&self)->usize;
 fn consumed_bytes(&self)->u64;
 fn body_bytes(&self,input:&[u8])->u64;
 fn symbol_utf8_bytes(&self,symbol:u64)->Result<usize,PackRefusal>;
 fn symbol_chars(&self,symbol:u64)->Result<usize,PackRefusal>;
 fn symbol_char(&self,symbol:u64,index:usize)->Result<Option<char>,PackRefusal>;
 fn normal_demands(&self)->Result<semio_framework_value::RetirementDemand,PackRefusal>{Err(PackRefusal::RetainedMalformed{kind:semio_framework_value::ValueRefusalKind::InvalidValue,what:"retained intrinsic Body",offset:0,detail:"original turn allocation quote is unavailable for this intrinsic driver"})}
 fn next_copy_demand(&self)->usize{0}
 fn next_capacity_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(1)}
 fn next_release_demand(&self)->Result<usize,PackRefusal>;
 fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,PackRefusal>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>;
 fn terminal_is_empty(&self)->bool;
}
/// 🧮️ Converts the physical format close receipt without changing its paid currencies.
pub fn retained_pack_close_progress(step:semio_framework_pack_format_retained::RetainedPackCloseStep)->RetainedCloneStep{match step{semio_framework_pack_format_retained::RetainedPackCloseStep::Complete=>RetainedCloneStep::Complete(Default::default()),semio_framework_pack_format_retained::RetainedPackCloseStep::Pending{released_items,released_bytes}=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,released_bytes,..Default::default()})}}
