//! 📅️ Schedule collection changes adopt their sealed schedule set through the original output branch.
use super::*;
use crate::snapshot::state::{StateScheduleSet,ModelOutputState};
pub struct ScheduleBranchParent;
impl NestedBranchPath for ScheduleBranchParent{
 type Branch=StateScheduleSet;type Parent=ModelOutputState;type Output=DirectBranchOutput<ModelOutputState>;
 fn parent(o:RetainedCloneRef<'_,EnergyModelSnapshot>)->RetainedCloneRef<'_,Self::Parent>{o.project(0,|s|s.model.outputs.get())}
 fn select(p:&mut Self::Parent)->&mut SealedShared<Self::Branch>{&mut p.schedules}
 fn output()->Self::Output{DirectBranchOutput::new(|outputs|{let mut d=EnergyModelDiff::default();d.model.outputs=Some(outputs);d})}
}
pub type ScheduleBranchOutput=NestedBranchOutput<ScheduleBranchParent>;
