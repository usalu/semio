//! 🎠️ Shared native activation handoff for headless and rendered hosts.

use super::shard::executor::{RegistrationAdmission, ShardExecutor};
use super::{CompiledHandle, GuestRuntime, GuestRuntimes};
use semio_framework::kernel::{ActivationEvent, BrokerCapabilityGrant, Budget, Event};
use semio_framework_actor::activation::KernelActivationReservation;
use semio_framework_actor::{ActorId, Kernel};
use std::sync::Arc;

/// 🎬️ The declared activation event an app id justifies — the native twin of the kernel's own
/// `activationReasonForAppId` (`🎠️kernel/🟦️.ts`), reading the SAME canonical surface grammar
/// `surface_app_id` mints. A role-suffixed surface id carries the artifact kind its actor is being
/// activated for, so it activates `OnArtifactKind` with that kind; a bare landing/host app id names
/// no kind and answers `None`, because the host's own trigger is not a declared activation event and
/// `📜️.wit`'s `activation-event` variant has no case for it.
pub fn activation_event_for_app_id(app_id: &str) -> Option<ActivationEvent> {
    semio_framework::parse_surface_app_id(app_id).ok().map(|(dialect, _)| ActivationEvent::OnArtifactKind { kind: dialect.artifact_kind })
}

/// 🎬️ The `Event::Activate` a freshly activated instance must receive before its own
/// `Event::InstanceOpen`, for an app id that names a declared activation event. This is the one
/// producer of that event in the tree: `kernel_event_to_wit` already marshals it onto the guest's
/// `activate(activate-event)`, so prepending this to an instance's first turn is exactly what makes
/// the guest see WHY it was woken; that marshal stamps the turn's own instance id onto the record.
pub fn activation_turn_event(app_id: &str) -> Option<Event> {
    activation_event_for_app_id(app_id).map(|reason| Event::Activate { reason })
}

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
