
use super::*;
use ui_contract::{ActionId, SurfaceId, Trigger, UiNodeId, UiRevision};

fn fake_intent(surface: &str) -> UiIntent {
    UiIntent {
        surface: SurfaceId::try_from(surface).expect("bounded fixture surface"),
        revision: UiRevision(1),
        seq: 1,
        node: UiNodeId(1),
        node_key: ui_contract::UiText::try_from_str("root").expect("bounded fixture node key"),
        trigger: Trigger::Activate,
        action: ActionId::default(),
        args: None,
        input: None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn echo_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome> + Send>> {
    Box::pin(async move { KernelOutcome { surface: intent.surface.0.to_string(), detail: Box::new(intent.node_key) } })
}

#[cfg(target_arch = "wasm32")]
fn echo_exchange(intent: UiIntent) -> Pin<Box<dyn Future<Output = KernelOutcome>>> {
    Box::pin(async move { KernelOutcome { surface: intent.surface.0.to_string(), detail: Box::new(intent.node_key) } })
}

#[test]
fn a_fake_seam_receives_submitted_intents_and_outcomes_reach_the_host_on_wake() {
    let seam = AppKernelSeam::new(echo_exchange);
    let (woken, completion) = std::sync::mpsc::sync_channel(1);
    seam.set_waker(HostWaker::new(move || {
        let _ = woken.try_send(());
    }));

    assert!(seam.submit_intents(vec![fake_intent("surface-a")]).is_empty());
    #[cfg(not(target_arch = "wasm32"))]
    completion.recv_timeout(std::time::Duration::from_secs(2)).expect("worker completion wake");

    #[cfg(not(target_arch = "wasm32"))]
    {
        let outcomes = seam.drain_outcomes();
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].surface, "surface-a");
        assert_eq!(seam.pending_len(), 0, "drain_outcomes empties the queue");
    }
}

#[test]
fn drain_outcomes_is_empty_with_nothing_submitted() {
    let seam = AppKernelSeam::new(default_intent_exchange);
    assert!(seam.drain_outcomes().is_empty());
}

#[test]
fn outcome_mailbox_is_fixed_capacity_and_returns_backpressure_without_eviction() {
    let mut mailbox = OutcomeMailbox::new();
    for _ in 0..OUTCOME_CAPACITY {
        assert!(mailbox.reserve());
    }
    assert!(!mailbox.reserve());
    mailbox.finish(KernelOutcome { surface: "surface-7".to_string(), detail: Box::new(999usize) });
    assert_eq!(mailbox.ready.len() + mailbox.in_flight, OUTCOME_CAPACITY);
    assert_eq!(*mailbox.ready.back().expect("lossless outcome").detail.downcast_ref::<usize>().expect("usize detail"), 999);
}
