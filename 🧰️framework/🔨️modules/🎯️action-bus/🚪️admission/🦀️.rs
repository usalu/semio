//! 🚪️ Original tool factories borrow native construction authority and preserve caller custody through refusal.
use super::RetainedToolWireInput;
use semio_framework_job::{Generation, InteractiveJob, Operation, OperationId, OriginalTurnFailure, StepContext};
use semio_framework_job_payload::{OriginalPayloadSourceIdentity};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;

/// 📏️ One concrete factory declares the physical effects of its next original construction turn.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ToolJobAdmissionDemand {pub items:usize,pub copy_bytes:usize,pub capacity_bytes:usize,pub release_bytes:usize,pub depth:usize}
impl ToolJobAdmissionDemand {
    /// 📦️ A new typed frame adopts the original payload value and releases its original Box backing.
    pub fn boxed_job<P,J>()->Self{Self{items:1,copy_bytes:0,capacity_bytes:size_of::<J>(),release_bytes:size_of::<P>(),depth:1}}
    /// 🔗️ A new typed frame and shared payload allocate their actual declared native layouts.
    pub fn shared_payload<P,J>()->Result<Self,ValueError>{let mut demand=Self::boxed_job::<P,J>();demand.capacity_bytes=demand.capacity_bytes.checked_add(semio_framework_value::retirement::arc_custody::arc_bytes::<P>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool factory allocation extent overflow"))?;Ok(demand)}
    /// 🎒️ An already born job changes custody without allocating or releasing a replacement frame.
    pub fn reused_job()->Self{Self{items:1,copy_bytes:0,capacity_bytes:0,release_bytes:0,depth:1}}
    /// 🎟️ Every native axis must fit the actual original remaining turn before any slot transfers.
    pub fn fits(self,grant:RetainedCloneGrant)->bool{self.items==1&&self.depth>0&&self.items<=grant.maximum_items&&self.copy_bytes<=grant.maximum_copy_bytes&&self.capacity_bytes<=grant.maximum_capacity_bytes&&self.release_bytes<=grant.maximum_release_bytes&&self.depth<=grant.maximum_depth}
    fn physical(self)->RetirementDemand{RetirementDemand{copy_bytes:self.copy_bytes,capacity_bytes:self.capacity_bytes,release_bytes:self.release_bytes,depth:self.depth}}
    fn progress(self)->RetainedCloneProgress{RetainedCloneProgress{copied_items:self.items,copied_bytes:self.copy_bytes,retained_capacity_bytes:self.capacity_bytes,released_bytes:self.release_bytes}}
}

pub use semio_framework_job::OriginalSourceBinding as OriginalToolSourceBinding;
/// 🪪️ Construction receives an identity issued only inside the same admitted original source turn.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalToolJobIdentity{source:OriginalToolSourceBinding}
impl OriginalToolJobIdentity{
 pub fn operation(self)->OperationId{self.source.operation()}
 pub fn generation(self)->Generation{self.source.generation()}
 pub fn matches(self,context:&StepContext<'_>)->bool{self.source.matches(context)}
 pub fn source_binding(self)->OriginalToolSourceBinding{self.source}
}

/// 🫴️ Original slots transfer only inside the original decoder's admitted construction callback.
pub fn admit_original_tool_job<P:Send+'static,J:InteractiveJob+Send+'static>(
    context:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission,operation:&Operation,payload:&mut Option<Box<P>>,
    input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<J>>,
    demand:ToolJobAdmissionDemand,
    construct:impl FnOnce(OriginalToolJobIdentity,Box<P>,Option<RetainedToolWireInput>,Option<RetainedToolWireInput>)->Box<J>,
)->Result<semio_framework_job::OriginalConstructionStatus,OriginalTurnFailure>{
    admit_original_tool_job_measured(context,admission,operation,payload,input,checkpoint,output,demand,|identity,payload,input,checkpoint|(construct(identity,payload,input,checkpoint),demand.progress()))
}

/// 🧮️ Actual construction effects accompany retained output before original receipt acceptance.
pub fn admit_original_tool_job_measured<P:Send+'static,J:InteractiveJob+Send+'static>(
    context:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission,operation:&Operation,payload:&mut Option<Box<P>>,
    input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<J>>,
    demand:ToolJobAdmissionDemand,
    construct:impl FnOnce(OriginalToolJobIdentity,Box<P>,Option<RetainedToolWireInput>,Option<RetainedToolWireInput>)->(Box<J>,RetainedCloneProgress),
)->Result<semio_framework_job::OriginalConstructionStatus,OriginalTurnFailure>{
    admit_original_tool_job_controlled(context,admission,operation,payload,input,checkpoint,output,demand,|identity,payload,input,checkpoint|{let owned=payload.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"admitted original payload is absent"))?;Ok(construct(identity,owned,input.take(),checkpoint.take()))})
}

/// 🔐️ Fallible source preparation retains every original slot before a construction candidate exists.
pub fn admit_original_tool_job_controlled<P:Send+'static,J:InteractiveJob+Send+'static>(
    context:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission,_operation:&Operation,payload:&mut Option<Box<P>>,
    input:&mut Option<RetainedToolWireInput>,checkpoint:&mut Option<RetainedToolWireInput>,output:&mut Option<Box<J>>,
    demand:ToolJobAdmissionDemand,
    construct:impl FnOnce(OriginalToolJobIdentity,&mut Option<Box<P>>,&mut Option<RetainedToolWireInput>,&mut Option<RetainedToolWireInput>)->Result<(Box<J>,RetainedCloneProgress),ValueError>,
)->Result<semio_framework_job::OriginalConstructionStatus,OriginalTurnFailure>{
    if output.is_none()&&payload.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tool factory requires its original payload").into());}
    admit_original_tool_construction(context,admission,output,demand,|identity|construct(identity,payload,input,checkpoint))
}

/// 🏭️ A non-cloneable loan of an already admitted construction quote exists only inside its original callback.
pub struct OriginalToolConstructionScope<'scope>{identity:OriginalToolJobIdentity,remaining:RetirementDemand,_loan:std::marker::PhantomData<&'scope mut ()>}
impl<'scope> OriginalToolConstructionScope<'scope>{
 fn loan(_original:&'scope mut semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'_,'_>,identity:OriginalToolJobIdentity,remaining:RetirementDemand)->Self{Self{identity,remaining,_loan:std::marker::PhantomData}}
 pub fn identity(&self)->OriginalToolJobIdentity{self.identity}
 /// 🪙️ Reserves only the still-available original owner quote before any physical source is born.
 pub fn admit_owner(&mut self,demand:RetirementDemand)->Result<(),ValueError>{if demand.depth>self.remaining.depth||demand.copy_bytes>self.remaining.copy_bytes||demand.capacity_bytes>self.remaining.capacity_bytes||demand.release_bytes>self.remaining.release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original construction owner exceeds its remaining admitted quote"))}self.remaining.copy_bytes-=demand.copy_bytes;self.remaining.capacity_bytes-=demand.capacity_bytes;self.remaining.release_bytes-=demand.release_bytes;Ok(())}
}

/// 🏗️ One original operation funds construction while its caller retains any born candidate through refusal.
pub fn admit_original_tool_construction<O>(context:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission,output:&mut Option<O>,demand:ToolJobAdmissionDemand,construct:impl FnOnce(OriginalToolJobIdentity)->Result<(O,RetainedCloneProgress),ValueError>)->Result<semio_framework_job::OriginalConstructionStatus,OriginalTurnFailure>{
    admit_original_tool_construction_scoped(context,admission,output,demand,|scope|construct(scope.identity()))
}

/// 🔧️ Original scope custody funds nested owner births before one actual candidate reaches its receipt boundary.
pub fn admit_original_tool_construction_scoped<O>(context:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission,output:&mut Option<O>,demand:ToolJobAdmissionDemand,construct:impl FnOnce(&mut OriginalToolConstructionScope<'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<semio_framework_job::OriginalConstructionStatus,OriginalTurnFailure>{
    let source=OriginalToolSourceBinding::borrow(context);
    semio_framework_job::admit_original_construction(context,output,admission,semio_framework_job::OriginalConstructionDemand{items:demand.items,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},1,|control,actual|{
        let identity=OriginalToolJobIdentity{source};
        let mut scope=OriginalToolConstructionScope::loan(control,identity,demand.physical());
        let result=construct(&mut scope);
        let progress=match &result{Ok((_,progress))=>*progress,Err(error)=>error.retained_progress()};if progress.retained_capacity_bytes!=0&&actual.is_none(){*actual=Some(progress.retained_capacity_bytes)}
        semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}
    })
}

/// 🧭️ Drives bind, construct and settle as far as this turn allows over the caller's resident admission and output: `Ok(true)` when settled, `Ok(false)` when the next turn must continue with the same admission and output; a quote above the operation ceiling is a typed refusal. A turn commits only when it funds the exact quote of the constructor plus its settle metadata, so both then finish in the same turn.
pub fn admit_original_construction_settled<O>(context:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission,output:&mut Option<O>,demand:semio_framework_job::OriginalConstructionDemand,work:u64,construct:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'_,'_>,&mut Option<usize>)->semio_framework_value::RetainedWorkOutcome<(O,RetainedCloneProgress)>)->Result<bool,OriginalTurnFailure>{
    let mut construct=Some(construct);
    let(complete,total)=semio_framework_job::original_construction_reserve::<O>(demand,work)?;
    let(overhead_items,overhead_copy,overhead_fuel)=semio_framework_job::original_construction_settle_overhead();let settle=semio_framework_job::OriginalConstructionDemand{items:complete.items+overhead_items,copy_bytes:complete.copy_bytes+overhead_copy,..complete};let fuel_floor=total+overhead_fuel;
    let(ceiling,ceiling_fuel)=context.original_turn_ceiling();
    if !settle.fits(ceiling)||fuel_floor>ceiling_fuel{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original construction quote exceeds the largest grant any turn can have").into())}
    let mut committed=false;
    loop{
        if output.is_some()&&admission.terminal_is_empty(){return Ok(true)}
        if output.is_none()&&!admission.terminal_is_empty(){
            if !committed&&(context.retirement_should_yield()||context.fuel_remaining()<fuel_floor||!settle.fits(context.retained_grant())){return Ok(false)}
            committed=true;
        }
        let fuel=context.fuel_remaining();
        let status=semio_framework_job::admit_original_construction(context,output,admission,demand,work,|scope,actual|match construct.take(){
            Some(construct)=>construct(scope,actual),
            None=>semio_framework_value::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original constructor ran twice")),work:0},
        })?;
        match status{
            semio_framework_job::OriginalConstructionStatus::Ready=>{admission.reset_step(context)?;return Ok(true)}
            semio_framework_job::OriginalConstructionStatus::Failed=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original construction failed; its complete causes remain in the admission").into()),
            semio_framework_job::OriginalConstructionStatus::Pending=>if context.fuel_remaining()==fuel{return Ok(false)},
        }
    }
}

/// 🏛️ A tool job keeps its construction admission and candidate resident, so a turn that cannot commit yet continues next turn over the same bind and scope.
pub struct ResidentConstruction<O>{admission:semio_framework_job::OriginalConstructionAdmission,output:Option<O>}
impl<O> Default for ResidentConstruction<O>{fn default()->Self{Self{admission:Default::default(),output:None}}}
impl<O> ResidentConstruction<O>{
    pub fn drive(&mut self,context:&mut StepContext<'_>,demand:semio_framework_job::OriginalConstructionDemand,work:u64,construct:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'_,'_>,&mut Option<usize>)->semio_framework_value::RetainedWorkOutcome<(O,RetainedCloneProgress)>)->Result<Option<O>,OriginalTurnFailure>{
        Ok(if admit_original_construction_settled(context,&mut self.admission,&mut self.output,demand,work,construct)?{self.output.take()}else{None})
    }
    pub fn drive_tool(&mut self,context:&mut StepContext<'_>,demand:ToolJobAdmissionDemand,construct:impl FnOnce(&mut OriginalToolConstructionScope<'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<Option<O>,OriginalTurnFailure>{
        Ok(if admit_original_tool_construction_settled(context,&mut self.admission,&mut self.output,demand,construct)?{self.output.take()}else{None})
    }
    /// 🏦️ The constructed output with the scope credit its consumed payload held, moved to a carrier that settles when the output retires.
    pub fn drive_tool_credited(&mut self,context:&mut StepContext<'_>,demand:ToolJobAdmissionDemand,construct:impl FnOnce(&mut OriginalToolConstructionScope<'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<Option<std::result::Result<semio_framework_job::OriginalCreditedOutput<O>,O>>,OriginalTurnFailure>{
        match self.drive_tool(context,demand,construct)?{Some(output)=>Ok(Some(context.adopt_original_credit(output)?)),None=>Ok(None)}
    }
    pub fn terminal_is_empty(&self)->bool{self.output.is_none()&&self.admission.terminal_is_empty()}
}

/// 🧭️ The tool-shaped form of [`admit_original_construction_settled`] over its scoped identity and quote.
pub fn admit_original_tool_construction_settled<O>(context:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission,output:&mut Option<O>,demand:ToolJobAdmissionDemand,construct:impl FnOnce(&mut OriginalToolConstructionScope<'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{
    let source=OriginalToolSourceBinding::borrow(context);
    admit_original_construction_settled(context,admission,output,semio_framework_job::OriginalConstructionDemand{items:demand.items,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth},1,|control,actual|{
        let identity=OriginalToolJobIdentity{source};
        let mut scope=OriginalToolConstructionScope::loan(control,identity,demand.physical());
        let result=construct(&mut scope);
        let progress=match &result{Ok((_,progress))=>*progress,Err(error)=>error.retained_progress()};if progress.retained_capacity_bytes!=0&&actual.is_none(){*actual=Some(progress.retained_capacity_bytes)}
        semio_framework_value::RetainedWorkOutcome{work:u128::from(result.is_ok()),result}
    })
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
pub(crate) mod tests;
