//! 🧹️ Exact turn-patch retirement keeps every original page and reservation until its quoted close succeeds.
use super::*;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,record_retirement_work};

pub struct OriginalPatches(UiTurnPatches);

impl OriginalPatches {
    fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{
        let grant=effect.grant();
        let result=(||{
            let demand=self.demand(body)?;
            if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth||effect.maximum_work_units()==0{return Ok(RetirementStep::BudgetExhausted)}
            for entry in &mut self.0.entries[..self.0.length]{
                if !entry.contents.terminal_is_empty(){let step=entry.contents.pending.close_original_step(grant);return Ok(match step.result{Ok(progress)=>paid(effect,progress.progress(),u64::try_from(step.work).unwrap_or(u64::MAX)),Err(error)=>RetirementStep::Failure(error.into(),step.work)})}
                if let Some(key)=entry.retirement{
                    let mut arena=match UI_TURN_PATCH_RETIRE_ARENA.try_lock(){Ok(arena)=>arena,Err(std::sync::TryLockError::WouldBlock)=>return Ok(RetirementStep::BudgetExhausted),Err(std::sync::TryLockError::Poisoned(_))=>return Err(invalid("turn patch retirement arena is poisoned"))};
                    if !arena.release_empty(key){return Err(invalid("original turn patch reservation differs from its owner"))}
                    entry.retirement=None;
                    return Ok(paid(effect,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1))
                }
            }
            self.0.length=0;self.0.cursor=0;
            Ok(paid(effect,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1))
        })();
        result.unwrap_or_else(|error|RetirementStep::Failure(error.into(),0))
    }
    fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        for entry in &self.0.entries[..self.0.length]{
            if !entry.contents.terminal_is_empty(){return entry.contents.pending.original_retirement_demand(body)}
            if entry.retirement.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<UiTurnPatchRetireKey>>()+size_of::<bool>(),depth:1,..Default::default()})}
        }
        Ok(RetirementDemand{copy_bytes:2*size_of::<usize>(),depth:usize::from(self.0.length!=0||self.0.cursor!=0),..Default::default()})
    }
}

impl RetirementCursor for OriginalPatches {
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}
    fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(1)}
    semio_framework_value::native_retirement_reply!();
    fn terminal_is_empty(&self)->bool{self.0.length==0&&self.0.cursor==0}
    fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}

impl RetireOwned for UiTurnPatches {
    type RetirementCursor=OriginalPatches;
    fn retirement_seed(self)->OriginalPatches{OriginalPatches(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalPatches>())}
    fn controlled_retirement_supported()->bool{true}
}

fn paid(effect:&mut RetainedAllocationEffect<'_>,progress:RetainedCloneProgress,work:u64)->RetirementStep{
    match record_retirement_work(effect,work){Ok(work)=>RetirementStep::Progress(progress,work),Err((error,work))=>RetirementStep::Failure(error.into(),work)}
}

fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
