//#region ➕️AddJobTestValue
use super::super::{JobTestDiff,JobTestOp,JobTestSnapshot};

#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize,dsl::MutationLeaf)]
#[mutation_leaf(contract=::protocol)]
#[serde(deny_unknown_fields)]
pub(crate) struct AddValue { pub(crate) delta:i32 }

impl protocol::MutationKind<JobTestSnapshot,JobTestOp> for AddValue {
    const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"add",entity:"value",kind:"add-value",record:"AddedValue"};
    fn diff(&self,_:&JobTestSnapshot)->protocol::MutationOutcome<JobTestDiff>{protocol::MutationOutcome::new(JobTestDiff{deltas:vec![self.delta]})}
    fn inverse(&self,_:&JobTestSnapshot)->Vec<JobTestOp>{
        match self.delta.checked_neg(){
            Some(delta)=>vec![JobTestOp::AddValue(Self{delta})],
            None=>vec![JobTestOp::AddValue(Self{delta:1}),JobTestOp::AddValue(Self{delta:i32::MAX})],
        }
    }
    fn label(&self)->String{format!("Add {} to value",self.delta)}
}

impl protocol::CompositeMutationKind<JobTestSnapshot,JobTestOp> for AddValue {
    const SEMANTICS:protocol::SemanticDescriptor=<Self as protocol::MutationKind<JobTestSnapshot,JobTestOp>>::SEMANTICS;
    fn plan(&self,_:&JobTestSnapshot,planner:&mut protocol::Planner<JobTestSnapshot,JobTestOp>)->Result<(),protocol::PlanError>{planner.call(JobTestOp::AddValue(self.clone()))}
    fn label(&self)->String{<Self as protocol::MutationKind<JobTestSnapshot,JobTestOp>>::label(self)}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
//#endregion ➕️AddJobTestValue
