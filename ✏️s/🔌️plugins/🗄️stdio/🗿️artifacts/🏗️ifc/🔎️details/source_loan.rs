//! 🪢️ One original-funded projection leases the render host's actual immutable source Arc.
use super::{IfcSnapshot,wire::SnapshotSource};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneProgress,RetainedCloneGrant,retained_clone_grant_admits}};
use std::{mem::size_of,sync::Arc};

#[derive(Clone,Copy)]
struct Binding{original:OriginalSourceBinding,pointer:usize,revision:[u8;32]}
semio_framework_value::artifact_retire_leaf!(Binding);
#[derive(Default,semio_framework_value::RetireOwned)]
pub(super) struct SourceLoan{source:Option<SnapshotSource>,alias:Option<Arc<IfcSnapshot>>,binding:Option<Binding>}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"staging.original-source-loan")}
impl SourceLoan{
    fn metadata_bytes()->usize{2*size_of::<Self>()+size_of::<Arc<IfcSnapshot>>()}
    pub(super) fn demands(&self)->Result<RetirementDemand,ValueError>{
        if self.source.is_some(){return Ok(Default::default());}
        let mut demand=RetirementDemand{copy_bytes:Self::metadata_bytes(),depth:1,..Default::default()};
        if self.alias.is_some(){let child=SnapshotSource::borrowed_constructor_demand::<Arc<IfcSnapshot>>();demand.capacity_bytes=child.capacity_bytes;demand.depth=demand.depth.max(child.depth);}
        Ok(demand)
    }
    pub(super) fn source(&self)->Option<&SnapshotSource>{self.source.as_ref()}
    pub(super) fn advance(&mut self,owner:&Arc<IfcSnapshot>,expected:&IfcSnapshot,revision:[u8;32],cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        let pointer=expected as *const _ as usize;
        if !std::ptr::eq(owner.as_ref(),expected)||self.binding.is_some_and(|binding|!binding.original.matches(cx)||binding.pointer!=pointer||binding.revision!=revision){return Err(invalid().into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"staging source loan cancelled").into());}
        if self.source.is_some(){return Ok(true);}
        let grant=cx.retained_grant();let demand=self.demands()?;
        if cx.should_yield()||!retained_clone_grant_admits(grant,demand){return Ok(false);}
        let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:Self::metadata_bytes(),..Default::default()};
        let result=if self.alias.is_none(){
            self.alias=Some(Arc::clone(owner));self.binding=Some(Binding{original:OriginalSourceBinding::borrow(cx),pointer,revision});Ok(false)
        }else{
            let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::metadata_bytes(),..grant};
            match SnapshotSource::admit_borrowed(self.alias.take().ok_or_else(invalid)?,Arc::as_ref,child_grant){
                Ok((source,child))=>{self.source=Some(source);progress.retained_capacity_bytes=child.retained_capacity_bytes;progress.released_bytes=child.released_bytes;progress.copied_bytes+=child.copied_bytes;Ok(true)},
                Err((error,alias))=>{self.alias=Some(alias);Err(error)},
            }
        };
        let posted=cx.consume_retained(progress);cx.consume_fuel(1);
        match(result,posted){(Ok(ready),Ok(()))=>Ok(ready),(Err(error),posted)=>{let mut failure=OriginalTurnFailure::from(error);if let Err(error)=posted{failure.context_refusal=Some(error);}Err(failure)},(Ok(_),Err(error))=>Err(error.into())}
    }
}
