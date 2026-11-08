//! 🔔️ A channel backbone whose actor installed its outbound wake retires completely through paid steps.
use semio_framework_value::{AuthoringWallet, RetainedCloneGrant};
use semio_os_store_backbone::{backbone_retirement::ArtifactStoreBackboneRetirement, Backbones, ChannelBackbone};
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use std::future::Future;
use std::task::{Context, Poll, Waker};

#[test]
fn installed_actor_wake_is_released_by_paid_steps_and_retirement_finishes() {
    let mut pair = std::pin::pin!(ChannelBackbone::pair("wake"));
    let Poll::Ready((channel, remote)) = pair.as_mut().poll(&mut Context::from_waker(Waker::noop())) else { panic!("pairing never suspends") };
    let calls = Arc::new(AtomicUsize::new(0));
    let probe = calls.clone();
    remote.set_outbound_wake(Arc::new(move || { probe.fetch_add(1, Ordering::SeqCst); }));
    drop(remote);
    let stock = RetainedCloneGrant { maximum_items: 1 << 16, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 1 << 20, maximum_depth: 128 };
    let mut wallet = AuthoringWallet::new(stock, 1 << 20);
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: ArtifactStoreBackboneRetirement::constructor_capacity_bytes(), maximum_depth: 1, ..Default::default() };
    let (mut queue, mut backbone) = (std::collections::VecDeque::new(), Some(Backbones::Channel(channel)));
    let (mut owner, _) = ArtifactStoreBackboneRetirement::admit_attached(&mut queue, &mut backbone, birth).unwrap();
    assert_eq!(Arc::strong_count(&calls), 2);
    for turn in 0..1000 {
        if owner.is_none() { break; }
        let grant = RetainedCloneGrant { maximum_items: 1, ..stock };
        let (outcome, _) = wallet.run_settled(grant, 64, |effect| semio_framework_value::close_factory_ticket(&mut owner, effect)).unwrap().expect("quote admitted");
        outcome.result.unwrap();
        assert!(turn < 999, "retirement never finished");
    }
    assert!(owner.is_none());
    assert_eq!(Arc::strong_count(&calls), 1);
}
