//! 📋️ Element-cursor effects preserve the original source, computational loan and allocation marker.
use semio_framework_value_control_contract::*;
use semio_framework_value_clone_source::RetainedCloneRef;
use semio_framework_value_retirement::{RetireOwned,OriginalRetirementOutcome};
pub trait PagedCloneElement<T:RetireOwned+Send+Sync+'static>:Send{fn new()->Self;
fn output_ready(&self)->bool;
fn advance_demands(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
fn advance_work_demand(&self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
fn advance(&mut self,source:RetainedCloneRef<'_,T>,body:usize,maximum_work_units:u64,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementOutcome;
fn take_demand(&self)->Result<RetirementDemand,ValueError>;
fn take_work_demand(&self)->Result<u64,ValueError>;
fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>;
fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>;
fn begin_close_work_demand(&self)->Result<u64,ValueError>;
fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress>;
fn close_started(&self)->bool;
fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->OriginalRetirementOutcome;
fn terminal_is_empty(&self)->bool;
}
