//! 📥️ A typed original snapshot owner retains its source through bounded intake and close.
use crate::{OriginalTurnFailure,RetainedCloneGrant,RetainedCloneStep,RetirementDemand,StepContext};
use semio_framework_value::ValueError;

/// 🎟️ Concrete producers retain source custody and publish only their admitted typed snapshot.
pub trait OriginalSnapshotIntake<S>:Send {
 fn next_normal_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,revision:[u8;32],cx:&mut StepContext<'_>)->Result<Option<usize>,OriginalTurnFailure>;
 fn complete(&self)->bool;
 fn take(&mut self)->Option<S>;
 fn begin_close(&mut self);
 fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>;
 fn terminal_is_empty(&self)->bool;
}

/// 🏭️ The selected producer admits the original source and keeps its concrete refusal custody.
pub trait OriginalSnapshotIntakeProducer<S,Source> {
 type Intake:OriginalSnapshotIntake<S>+'static;
 type Rejected;
 fn source_demands(&self)->Result<RetirementDemand,ValueError>;
 fn preserve_source(&self,source:Source)->Self::Rejected;
 fn admit(&self,source:Source,revision:[u8;32],source_generation:u64,cx:&mut StepContext<'_>)->Result<Self::Intake,(OriginalTurnFailure,Self::Rejected)>;
}
