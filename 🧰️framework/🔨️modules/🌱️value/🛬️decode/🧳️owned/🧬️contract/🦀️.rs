//! 🎟️ Canonical native receiving contracts require actual bounded work and preserve recovery custody.
pub use semio_framework_value_control_contract::{RetainedWorkOutcome,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,ValueError};
pub use semio_framework_value_native_decoding::native_decoding::{NativeDecodeControl,OriginalReceivingPrimary,construction::NativeRecoveryLoan};
pub use semio_framework_value_retained_clone::retained_clone::RetainedCloneRef;
pub use semio_framework_value_retirement::OriginalRetirementOutcome;
/// 🔐️ A close admission is issued only after the actual receiver reports no pending recovery.
pub struct NativeClosePermit(());

pub trait NativeReceiver<I:?Sized,T>:Send+'static{
 /// 🎯️ Selects one retained effect boundary using only already-cached receiver state.
 fn capture_plan(&self)->Result<Option<semio_framework_value_native_decoding::native_decoding::construction::NativeEffectCapturePlan>,ValueError>;

 fn advance_work_demand(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<u64,ValueError>;
 fn take_work_demand(&self)->Result<u64,ValueError>;
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>;
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>;
 fn output_ready(&self)->bool;
 fn is_closing(&self)->bool;
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{if self.has_pending_recovery(){return Err(ValueError::invariant("native close intent requires recovery"))}Ok(if self.is_closing(){Default::default()}else{RetirementDemand{copy_bytes:std::mem::size_of::<bool>(),depth:1,..Default::default()}})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{self.begin_close_demand()?;Ok(u64::from(!self.is_closing()))}
 fn begin_close_step(&mut self,grant:RetainedCloneGrant,maximum:u64)->RetainedWorkOutcome<RetainedCloneStep>{let result=(||{let d=self.begin_close_demand()?;if self.is_closing(){return Ok(RetainedCloneStep::Complete(Default::default()))}if maximum<1||!semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}self.begin_close()?;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();let work=if matches!(&result,Ok(RetainedCloneStep::Complete(p))if *p!=Default::default()){1}else{0};RetainedWorkOutcome{result,work}}

 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(T,RetainedCloneProgress)>>;
 /// 🛟️ Loans the actual unsettled original effect, including one owned by a nested receiver.
 fn has_pending_recovery(&self)->bool;
 fn pending_recovery(&mut self)->Option<NativeRecoveryLoan<'_>>;
 fn resume_recovery(&mut self,control:&mut NativeDecodeControl<'_>)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>;
 /// 🛑️ Starts closure only after the original caller has resolved every pending callback.
 fn begin_close(&mut self)->Result<(),ValueError>{if self.has_pending_recovery(){return Err(ValueError::invariant("original native receiving effect requires recovery before close"))}self.begin_close_admitted(NativeClosePermit(()))}
 fn begin_close_admitted(&mut self,permit:NativeClosePermit)->Result<(),ValueError>;
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->OriginalRetirementOutcome;
 fn terminal_is_empty(&self)->bool;
}





/// ⛽️ Narrows caller DATA to the actual live native work loan before any receiving effect.
pub fn native_work_limit(control:&mut NativeDecodeControl<'_>,maximum:u64)->Result<u64,ValueError>{if maximum!=0{control.validate_work_recipient()?}Ok(maximum)}/// ➕️ Preserves checked actual parent and child work.
pub fn work_sum(left:u64,right:u64)->Result<u64,ValueError>{left.checked_add(right).ok_or_else(||ValueError::invariant("native receiving work overflow"))}
/// 🎯️ Returns a selected quote only when the existing scope can fund it.
pub fn selected_native_work(work:u64,maximum:u64)->u64{if work<=maximum{work}else{0}}

/// 🏭️ A first-party factory supplies the exact canonical receiver without owning or mirroring its source.
pub trait NativeReceiverFactory<I:?Sized,T>:Send+'static{type Receiver:NativeReceiver<I,T>;fn receiver()->Self::Receiver;fn record_subset_receiver()->Option<Self::Receiver>{None}}
