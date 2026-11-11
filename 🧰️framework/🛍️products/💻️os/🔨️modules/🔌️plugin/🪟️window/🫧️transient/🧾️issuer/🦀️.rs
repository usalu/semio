//! 🧾️ Captured Window mutations retain the exact registered typed retirement issuer through original publication.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_job::{StepContext,OriginalTurnFailure,OriginalConstructionDemand};
use std::any::Any;

trait ErasedWindowMutationIssuer:semio_framework_value::FactoryRetirement{
    fn as_any(&self)->&dyn Any;
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RegisteredWindowMutationIssuer<M:'static>{
    #[factory_child]
    factory:Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>,
}
impl<M:Send+Sync+'static> ErasedWindowMutationIssuer for RegisteredWindowMutationIssuer<M>{fn as_any(&self)->&dyn Any{self}}

#[derive(Clone)]
pub struct CapturedWindowMutationIssuer{source:Arc<dyn ErasedWindowMutationIssuer>}
impl CapturedWindowMutationIssuer{
    pub fn birth_bytes<M:Send+Sync+'static>()->usize{semio_framework_value::factory_arc_birth_bytes::<RegisteredWindowMutationIssuer<M>>()}
    pub fn capture<M:Send+Sync+'static>(factory:Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>)->Self{Self{source:Arc::new(RegisteredWindowMutationIssuer{factory})}}
    fn factory<M:Send+Sync+'static>(&self)->Result<&Arc<dyn store::ArtifactOwnedValueRetirementFactory<M>>,ValueError>{self.source.as_any().downcast_ref::<RegisteredWindowMutationIssuer<M>>().map(|issuer|&issuer.factory).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"captured Window mutation differs from its registered typed issuer"))}
}
impl RetireOwned for CapturedWindowMutationIssuer{
    type RetirementCursor=semio_framework_value::FactoryAuthority;
    fn retirement_seed(self)->Self::RetirementCursor{let factory:Arc<dyn semio_framework_value::FactoryRetirement>=self.source;semio_framework_value::FactoryAuthority::new(factory)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<semio_framework_value::FactoryAuthority>())}
    fn controlled_retirement_supported()->bool{true}
}

impl WindowTransientSnapshot{
    pub fn original_mutation_demands<O:WindowTransientOwner>(&self)->Result<RetirementDemand,ValueError>{
        if self.window_kind_id!=O::WINDOW_KIND_ID{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"captured Window kind differs from the requested transient owner"));}
        self.mutation_issuer.factory::<O::Mutation>()?;
        Ok(RetirementDemand{copy_bytes:2*size_of::<WindowTransientMutation>()+size_of::<O::Mutation>()+size_of::<FactoryBoxedValue<O::Mutation>>()+self.window_id.len(),capacity_bytes:size_of::<O::Mutation>()+size_of::<FactoryBoxedValue<O::Mutation>>()+self.window_id.len(),depth:1,..Default::default()})
    }
    pub fn admit_original_mutation<O:WindowTransientOwner>(&self,input:&mut Option<O::Mutation>,output:&mut Option<WindowTransientMutation>,cx:&mut StepContext<'_>,admission:&mut semio_framework_job::OriginalConstructionAdmission)->Result<bool,OriginalTurnFailure>{
        let demand=self.original_mutation_demands::<O>()?;
        if input.is_none()||output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window mutation requires its retained input and empty destination").into());}
        let factory=self.mutation_issuer.factory::<O::Mutation>()?;
        semio_framework_job::admit_original_construction_settled(cx,admission,output,OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:0,depth:demand.depth},1,|_,actual|{
            let value=WindowTransientMutation::from_issued(self.window_id.clone(),self.window_kind_id,input.take().expect("original Window mutation remains owned"),factory.clone());
            let capacity=value.window_id.capacity()+size_of::<O::Mutation>()+size_of::<FactoryBoxedValue<O::Mutation>>();
            if capacity!=0&&actual.is_none(){*actual=Some(capacity)}
            semio_framework_value::RetainedWorkOutcome{result:Ok((value,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:capacity,..Default::default()})),work:1}
        })?;
        Ok(output.is_some())
    }
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
