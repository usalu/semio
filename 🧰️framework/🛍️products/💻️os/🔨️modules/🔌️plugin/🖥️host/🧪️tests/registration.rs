//! 🪪️ Admits the compiled mock parent through the original native registry for routing laws.
use super::PluginRegistration;
use std::sync::Arc;
use semio_framework_actor::{ActorId as RuntimeActorId,PackageHash,PackageId};
use semio_plugin_host_guest_runtime::{GuestRuntime as _,GuestRuntimes,MockGuestRuntime,PackageRef};

pub(super) async fn mock_registration(mock: &Arc<MockGuestRuntime>, actor: RuntimeActorId, package: &str, hash: u8) -> PluginRegistration {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
    use semio_plugin_host_post_turn_relay::origin_registry;
    let compiled = mock.compile(&PackageRef { package: PackageId(package.to_string()), hash: PackageHash([hash; 32]) }, &[]).await.expect("mock compile");
    let runtime = Arc::new(GuestRuntimes::Mock(mock.clone()));
    let stock = semio_framework_value::RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 4_000_000, maximum_capacity_bytes: 16_000_000, maximum_release_bytes: 16_000_000, maximum_depth: 64 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(98), Generation(1), stock, || Some(1000), 100_000_000, 1_000_000);
    let identity = origin_registry::next_identity();
    let mut origin = None;
    for _ in 0..4096 {
        if caller.turn(stock, |cx| origin_registry::register_step(identity, &runtime, &compiled, &mut origin, cx)).unwrap() {
            break;
        }
    }
    caller.retire(stock);
    PluginRegistration { actor, origin: origin.expect("fixture registry admits the original compiled parent") }
}
