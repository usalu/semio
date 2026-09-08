//! 🎠️ Shared native activation handoff for headless and rendered hosts.

use super::shard::executor::{RegistrationAdmission, ShardExecutor};
use super::{CompiledHandle, GuestRuntime, GuestRuntimes};
use semio_framework::kernel::{BrokerCapabilityGrant, Budget};
use semio_framework_actor::activation::KernelActivationReservation;
use semio_framework_actor::{ActorId, Kernel};
use std::sync::Arc;

async fn retire_failed_activation(kernel: &mut Kernel, reservation: KernelActivationReservation, reason: String) -> String {
    match kernel.abort_activation(reservation).await {
        Ok(_) => reason,
        Err(error) => format!("{reason}; kernel activation retirement failed: {:?}", error.reason),
    }
}

/// 🎟️ Transfers an existing kernel activation into its pinned shard, retiring failures exactly.
#[allow(clippy::too_many_arguments)]
pub async fn install_actor(
    kernel: &mut Kernel,
    runtime: &Arc<GuestRuntimes>,
    shards: &[Arc<ShardExecutor>],
    reservation: KernelActivationReservation,
    compiled: &CompiledHandle,
    caps: &[BrokerCapabilityGrant],
    budget: &Budget,
) -> Result<ActorId, String> {
    if !kernel.reservation_is_current(&reservation) {
        return Err("kernel activation reservation is not current".into());
    }
    let actor = reservation.actor();
    if kernel.actor_status(actor).await != Some(&semio_framework_actor::ActorStatus::Activating) {
        return Err(retire_failed_activation(kernel, reservation, "kernel activation is no longer activating".into()).await);
    }
    let Some(shard) = shards.get(reservation.shard().0 as usize) else {
        let reason = format!("kernel assigned shard {} but only {} shards exist", reservation.shard().0, shards.len());
        return Err(retire_failed_activation(kernel, reservation, reason).await);
    };
    let instance = match runtime.instantiate(compiled, actor, caps, budget).await {
        Ok(instance) => instance,
        Err(error) => return Err(retire_failed_activation(kernel, reservation, error.to_string()).await),
    };
    let reason = match shard.register(actor, instance).await {
        RegistrationAdmission::Admitted(allocation) => {
            let key = allocation.key(reservation.shard());
            return Ok(kernel.bind_activation(reservation, key).await.expect("exclusive Kernel reservation matches the admitted physical shard allocation"));
        }
        RegistrationAdmission::Refused(rejected) => {
            runtime.drop_instance(rejected.instance).await;
            format!("shard registration refused: {:?}", rejected.reason)
        }
        RegistrationAdmission::Rejected { instance, limit, .. } => {
            runtime.drop_instance(instance).await;
            format!("shard registration capacity: {limit:?}")
        }
        RegistrationAdmission::Stopped => "shard stopped before registration acknowledgement".into(),
    };
    Err(retire_failed_activation(kernel, reservation, reason).await)
}

#[cfg(test)]
#[path = "🧪️tests/🎠️activation/🦀️.rs"]
mod tests;
