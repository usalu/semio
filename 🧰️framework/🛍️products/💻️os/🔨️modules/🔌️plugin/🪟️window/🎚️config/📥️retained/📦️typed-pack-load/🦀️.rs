//! 📦️ Original native Window loading preserves private registry custody until its typed Store is ready.
use super::*;
use crate::registry;
use crate::owner_contract_and_mutation::WindowConfigAuthorityIdentity;
#[path="🛬️native/🦀️.rs"]
mod native;
pub(super) use native::TypedWindowConfigPackLoad;
pub(in super::super) use native::admit_typed_window_config_pack_load;
impl<O:WindowConfigOwner> TypedWindowConfigPackLoad<O>{
    fn install_demands(&self,partitions:&WindowRegistry<semio_framework_value::SharedUtf8,WindowConfigPartition<O>>)->Result<RetirementDemand,ValueError>{
        let control=size_of::<WindowConfigPackLoadPhase>()+2*size_of::<bool>();
        if self.registry_inserting{let mut demand=partitions.insert_original_demands()?;demand.copy_bytes=demand.copy_bytes.checked_add(control).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native Window publication control extent overflow"))?;return Ok(demand)}
        let candidate=self.candidate.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Window publication candidate absent"))?;let address=candidate.address.as_ref().and_then(ControlledRetirement::original).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Window publication address absent"))?;
        Ok(if partitions.get(address.as_str()).is_some(){RetirementDemand{copy_bytes:2*size_of::<WindowConfigPartition<O>>()+control,depth:1,..Default::default()}}else if self.registry_address.is_none(){RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::SharedUtf8>>(),..semio_framework_value::SharedUtf8::lease_demand()}}else{RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}})
    }
    fn install(&mut self,partitions:&mut WindowRegistry<semio_framework_value::SharedUtf8,WindowConfigPartition<O>>,registry_lifetime:u64,grant:RetainedCloneGrant)->WindowConfigPackLoadStep{
        if self.phase!=WindowConfigPackLoadPhase::Ready||self.registry_lifetime!=registry_lifetime{return self.reject_stale()}
        if self.registry_inserting{
            let control=size_of::<WindowConfigPackLoadPhase>()+2*size_of::<bool>();let demand=match self.install_demands(partitions){Ok(value)=>value,Err(_)=>return self.reject(WindowConfigPackLoadDiagnostic::Capacity)};if !registry::fits(grant,demand){return self.pending()}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-control,..grant};
            return match partitions.insert_original(&mut self.registry_address,&mut self.candidate,child){Ok((ready,mut receipt))=>{if ready{self.committed=true;self.registry_inserting=false;self.phase=WindowConfigPackLoadPhase::RetiringDisplacedStore;receipt.copied_bytes+=control;}WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),receipt)},Err(_)=>self.reject(WindowConfigPackLoadDiagnostic::Capacity)};
        }
        let candidate=self.candidate.as_ref().expect("original ready Window candidate");let address=candidate.address.as_ref().and_then(ControlledRetirement::original).expect("original candidate address");let id=address.as_str();let live=partitions.get(id).map(|partition|partition.store.generation());if live!=self.partition_generation{return self.reject_stale()}
        if let Some(original)=partitions.get_mut(id){let copied=2*size_of::<WindowConfigPartition<O>>()+size_of::<WindowConfigPackLoadPhase>()+2*size_of::<bool>();let demand=RetirementDemand{copy_bytes:copied,depth:1,..Default::default()};if !registry::fits(grant,demand){return self.pending()}*self.displaced=Some(std::mem::replace(original,self.candidate.take().unwrap()));self.committed=true;self.phase=WindowConfigPackLoadPhase::RetiringDisplacedStore;return WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),RetainedCloneProgress{copied_items:1,copied_bytes:copied,..Default::default()})}
        if self.registry_address.is_none(){if !registry::fits(grant,RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::SharedUtf8>>(),..semio_framework_value::SharedUtf8::lease_demand()}){return self.pending()}return match address.admit_clone(grant){Ok((original,mut receipt))=>{self.registry_address=Some(original);receipt.copied_bytes+=size_of::<Option<semio_framework_value::SharedUtf8>>();WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),receipt)},Err(_)=>self.reject(WindowConfigPackLoadDiagnostic::Capacity)}}
        let demand=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};if !registry::fits(grant,demand){return self.pending()}self.registry_inserting=true;
        WindowConfigPackLoadStep::OwnershipProgress(self.progress_now(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
    }

}
pub(in super::super) fn commit_typed_window_config_pack_load<O: WindowConfigOwner>(
    registry_lifetime: u64,
    partitions: &mut WindowRegistry<semio_framework_value::SharedUtf8, WindowConfigPartition<O>>,
    load: &mut dyn ErasedWindowConfigPackLoad,
    grant:RetainedCloneGrant,
) -> Result<WindowConfigPackLoadStep, WindowConfigPackLoadDiagnostic> {
    let typed = load.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>().ok_or(WindowConfigPackLoadDiagnostic::Stale)?;
    Ok(typed.install(partitions, registry_lifetime,grant))
}

/// ↩️ Restores a staged registry hole before any cancelled load owner is retired.
pub(in super::super) fn cancel_typed_window_config_pack_load<O:WindowConfigOwner>(partitions:&mut WindowRegistry<semio_framework_value::SharedUtf8,WindowConfigPartition<O>>,load:&mut dyn ErasedWindowConfigPackLoad,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
 let typed=load.as_any_mut().downcast_mut::<TypedWindowConfigPackLoad<O>>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original cancelled Window load owner differs from its registered vocabulary"))?;if !typed.registry_inserting{return Ok(None)}
 let mut demand=partitions.cancel_insert_original_demands()?;let overhead=size_of::<bool>();demand.copy_bytes=demand.copy_bytes.checked_add(overhead).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Window load insertion cancellation extent overflow"))?;demand.depth=demand.depth.max(1);if !registry::fits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
 let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant};let(done,mut receipt)=partitions.cancel_insert_original(&mut typed.registry_address,&mut typed.candidate,child)?;if done{typed.registry_inserting=false;receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes+=overhead;}Ok(Some(RetainedCloneStep::Progress(receipt)))
}

/// 📏️ Quotes the exact next private registry event before original native publication.
pub(in super::super) fn typed_window_config_pack_load_commit_demands<O:WindowConfigOwner>(partitions:&WindowRegistry<semio_framework_value::SharedUtf8,WindowConfigPartition<O>>,load:&dyn ErasedWindowConfigPackLoad)->Result<RetirementDemand,ValueError>{load.as_any().downcast_ref::<TypedWindowConfigPackLoad<O>>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native Window commit vocabulary differs from its registered owner"))?.install_demands(partitions)}

#[cfg(test)]
pub(super) use native::tests::input as original_saved_input;

#[cfg(test)]
pub(super) use native::tests::diagnostic as original_saved_diagnostic;
