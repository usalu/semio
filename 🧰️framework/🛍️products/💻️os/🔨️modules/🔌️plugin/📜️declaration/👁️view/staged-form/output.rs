//! 📦️ A private completed native body stays retained until original publication or paid close.
use super::StagedScalarCursor;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_job::{StepContext,OriginalTurnFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use std::mem::{ManuallyDrop,size_of};
pub struct StagedBodyOutput{node:ManuallyDrop<Option<BuiltNode>>,close:StagedScalarCursor,closing:bool}
impl Default for StagedBodyOutput{fn default()->Self{Self{node:ManuallyDrop::new(None),close:Default::default(),closing:false}}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"staging.body-output")}
fn metadata()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<StagedScalarCursor>()+2*size_of::<BuiltNode>()+size_of::<ComponentTree>(),depth:4,..Default::default()}}
fn fits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
impl StagedBodyOutput{
    pub fn donor(&mut self)->Result<&mut Option<BuiltNode>,ValueError>{if self.closing||!self.close.terminal_is_empty(){return Err(invalid());}Ok(&mut self.node)}
    pub fn ready(&self)->bool{!self.closing&&self.node.is_some()}
    /// 🎟️ The paid move creates no backing and leaves the existing original root stationary on refusal.
    pub fn take_tree(&mut self,cx:&mut StepContext<'_>)->Result<Option<ComponentTree>,OriginalTurnFailure>{
        if self.closing||self.node.is_none(){return Err(invalid().into());}cx.original_source_parent()?;if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"io.cancelled").into());}
        let demand=metadata();if cx.should_yield()||!fits(cx.retained_grant(),demand){return Ok(None);}
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);
        Ok(Some(ComponentTree::new(self.node.take().unwrap())))
    }
    pub fn terminal_is_empty(&self)->bool{self.node.is_none()&&self.close.terminal_is_empty()}
    fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.close.terminal_is_empty(){self.close.close_demand(body)}else{Ok(if self.node.is_some(){metadata()}else{Default::default()})}}
    fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=self.demand(grant.maximum_copy_bytes)?;if !fits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}self.closing=true;
        if !self.close.terminal_is_empty(){return self.close.close(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(node)=self.node.take(){self.close.adopt_for_close(node);}Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
}
impl Drop for StagedBodyOutput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"staged body output requires original paid retirement");}}
impl RetireOwned for StagedBodyOutput{fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}fn controlled_retirement_supported()->bool{true}}
impl RetirementCursor for StagedBodyOutput{
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete;}match self.step(grant){Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool{StagedBodyOutput::terminal_is_empty(self)}
    fn next_close_byte_demand(&self)->Option<usize>{Some(0)}fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(usize::MAX)?.copy_bytes)}fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demand(body)?.copy_bytes)}
}
