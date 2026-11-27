//! 📥️ A typed original snapshot owner retains its source through bounded intake and close.
use crate::{OriginalTurnFailure,RetirementDemand,StepContext};
use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect};
use semio_framework_value_retained_clone::ValueError;

/// 🎟️ Concrete producers retain source custody and publish only their admitted typed snapshot.
pub trait OriginalSnapshotIntake<S>:Send {
 fn next_normal_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn advance(&mut self,revision:[u8;32],cx:&mut StepContext<'_>)->Result<Option<usize>,OriginalTurnFailure>;
 fn complete(&self)->bool;
 fn take(&mut self)->Option<S>;
 fn begin_close(&mut self);
 fn next_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
 fn terminal_is_empty(&self)->bool;
}

/// 🏭️ The selected producer admits the original source and keeps its concrete refusal custody.
pub trait OriginalSnapshotIntakeProducer<S,Source> {
 type Intake:OriginalSnapshotIntake<S>+'static;
 type Rejected;
 /// 📐️ Quotes the copy, capacity and depth of the whole source admission the constructor drives.
 fn source_demands(&self)->Result<RetirementDemand,ValueError>;
 /// ⛽️ Quotes the items and Work of every stage of that admission beyond its physical demand.
 fn source_steps(&self)->Result<(usize,u64),ValueError>;
 fn preserve_source(&self,source:Source)->Self::Rejected;
 fn admit(&self,source:Source,revision:[u8;32],source_generation:u64,cx:&mut StepContext<'_>)->Result<Self::Intake,(OriginalTurnFailure,Self::Rejected)>;
}
