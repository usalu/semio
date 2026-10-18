//! 🧠️ Owned runtime adapter: the compiled and instance accessors of the owned interpreter runtime, the component snapshot transport and the guest runtime implementation for it. See [`super`].

use super::*;

//#region 🎭️GuestRuntime
//#region 🧠️OwnedRuntime
/// 🧠️ The owned compiled guest of `compiled`; a handle another runtime compiled has none.
pub(crate) fn owned_compiled(compiled: &CompiledHandle) -> Result<&OwnedCompiledGuest, PluginHostError> {
    compiled.owned.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no repository-owned actor artifact".to_string()))
}

/// 🧠️ The owned instance of `inst`; an instance another runtime built has none.
fn owned_instance_mut(inst: &mut GuestInstance) -> Result<&mut OwnedGuestInstance, TurnFault> {
    match &mut inst.state {
        GuestInstanceState::Owned(state) => Ok(state),
        _ => Err(TurnFault::Trapped("owned runtime called on a non-owned GuestInstance".to_string())),
    }
}

/// 🧳️ Copies a component snapshot's dialect coordinate and payload into storage the caller's original authority admitted,
/// for the Wasmtime codec exports that take both by value.
pub(crate) fn snapshot_component_transport<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(dialect:&str,payload:&[u8],identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>)->Result<(String,Vec<u8>),TurnFault>{
    identity.encode(|control|{
        control.checkpoint()?;
        control.charge(dialect.len().checked_add(payload.len()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"component snapshot transport length overflow"))?)?;
        let mut coordinate=String::new();
        coordinate.try_reserve_exact(dialect.len()).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"component snapshot dialect storage refused"))?;
        coordinate.push_str(dialect);
        let mut bytes=Vec::new();
        bytes.try_reserve_exact(payload.len()).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"component snapshot payload storage refused"))?;
        bytes.extend_from_slice(payload);
        Ok::<_,semio_framework_value::ValueError>((coordinate,bytes))
    }).map_err(identity_turn_fault)
}

impl GuestRuntime for OwnedRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{self.admit_context(owned_instance_mut(inst)?,input,budget,identity)}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{self.close_context(owned_instance_mut(inst)?,input,budget,identity)}

    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        Ok(CompiledHandle { package_hash: package.hash.0, component: None, owned: Some(self.compile_component(bytes)?) })
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        Ok(GuestInstance { actor, state: GuestInstanceState::Owned(self.instantiate_actor(owned_compiled(compiled)?)?) })
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        self.execute_actor_turn(owned_instance_mut(inst)?, events, budget, identity)
    }

    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        self.start_actor_job(owned_instance_mut(inst)?, job, kind, input)
    }

    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        self.step_actor_job(owned_instance_mut(inst)?, job, budget)
    }

    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault> {
        self.cancel_actor_job(owned_instance_mut(inst)?, job)
    }

    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Owned(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("checkpoint called on a non-owned GuestInstance".to_string()));
        };
        self.checkpoint_actor(state, identity)
    }

    async fn restore(&self, inst: &mut GuestInstance, checkpoint: &[u8], _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        let GuestInstanceState::Owned(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("restore called on a non-owned GuestInstance".to_string()));
        };
        self.restore_actor(state, checkpoint)
    }

    async fn drop_instance(&self, _inst: GuestInstance) {}
}
//#endregion 🧠️OwnedRuntime
//#endregion 🎭️GuestRuntime
