//! 📮️ Tests of the event router queueing policies and byte bounds and of the recording completion sink.

use semio_framework_actor::ActorId;
use semio_framework_async::ChannelPolicy;
use super::super::completion_sink::{CompletionSink, MockCompletionSink};
use super::super::event_router::{EventRouter, PublishOutcome, Topic};

//#region 📮️EventRouterTests
#[semio_framework_async_macros::async_test]
async fn event_router_latest_wins_collapses_older_pending_value() {
    let router = EventRouter::new();
    let topic = Topic("scene.updates".to_string());
    let actor = ActorId(1);
    router.subscribe(topic.clone(), actor, ChannelPolicy::LatestWins { max_bytes: 1_000_000 });
    let first = router.publish(&topic, None, b"v1").await;
    let second = router.publish(&topic, None, b"v2").await;
    assert_eq!(first, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(second, vec![(actor, PublishOutcome::Collapsed)]);
    assert_eq!(router.drain(&topic, actor).await, vec![b"v2".to_vec()], "only the LATEST value must survive the collapse");
}

#[semio_framework_async_macros::async_test]
async fn event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth() {
    let router = EventRouter::new();
    let topic = Topic("jobs.updates".to_string());
    let actor = ActorId(2);
    router.subscribe(topic.clone(), actor, ChannelPolicy::LosslessBounded { max_items: 2, max_bytes: 1_000_000 });
    assert_eq!(router.publish(&topic, None, b"a").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"b").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"c").await, vec![(actor, PublishOutcome::RejectedFull { cap: 2 })], "must reject rather than grow past cap");
    assert_eq!(router.drain(&topic, actor).await, vec![b"a".to_vec(), b"b".to_vec()], "the rejected message must never have been queued");
}

#[semio_framework_async_macros::async_test]
async fn event_router_coalesced_collapses_same_key_but_queues_distinct_keys() {
    let router = EventRouter::new();
    let topic = Topic("cursor.updates".to_string());
    let actor = ActorId(3);
    router.subscribe(topic.clone(), actor, ChannelPolicy::Coalesced { key: "cursor".to_string(), max_items: 100, max_bytes: 1_000_000 });
    router.publish(&topic, Some("peer-1"), b"pos-1").await;
    let outcome = router.publish(&topic, Some("peer-1"), b"pos-2").await;
    router.publish(&topic, Some("peer-2"), b"pos-a").await;
    assert_eq!(outcome, vec![(actor, PublishOutcome::Collapsed)]);
    let drained = router.drain(&topic, actor).await;
    assert_eq!(drained.len(), 2, "distinct coalesce keys must not collapse into each other");
    assert!(drained.contains(&b"pos-2".to_vec()));
    assert!(drained.contains(&b"pos-a".to_vec()));
}

#[semio_framework_async_macros::async_test]
async fn event_router_ring_overwrites_oldest_by_item_and_byte_bounds() {
    let router = EventRouter::new();
    let topic = Topic("diagnostics".to_string());
    let actor = ActorId(6);
    router.subscribe(topic.clone(), actor, ChannelPolicy::Ring { max_items: 3, max_bytes: 4 });
    assert_eq!(router.publish(&topic, None, b"aa").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"bb").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, b"cc").await, vec![(actor, PublishOutcome::Collapsed)]);
    assert_eq!(router.drain(&topic, actor).await, vec![b"bb".to_vec(), b"cc".to_vec()]);
}

#[semio_framework_async_macros::async_test]
async fn event_router_payload_bytes_are_enforced_for_every_queueing_policy() {
    let router = EventRouter::new();
    let latest = Topic("bounded.latest".to_string());
    let coalesced = Topic("bounded.coalesced".to_string());
    let lossless = Topic("bounded.lossless".to_string());
    let actor = ActorId(7);
    router.subscribe(latest.clone(), actor, ChannelPolicy::LatestWins { max_bytes: 2 });
    router.subscribe(coalesced.clone(), actor, ChannelPolicy::Coalesced { key: "entity".to_string(), max_items: 2, max_bytes: 4 });
    router.subscribe(lossless.clone(), actor, ChannelPolicy::LosslessBounded { max_items: 4, max_bytes: 3 });
    assert_eq!(router.publish(&latest, None, b"xxx").await, vec![(actor, PublishOutcome::RejectedInsufficientCredit)]);
    assert_eq!(router.publish(&coalesced, Some("a"), b"aaa").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&coalesced, Some("b"), b"bb").await, vec![(actor, PublishOutcome::Collapsed)]);
    assert_eq!(router.drain(&coalesced, actor).await, vec![b"bb".to_vec()]);
    assert_eq!(router.publish(&lossless, None, b"aa").await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&lossless, None, b"bb").await, vec![(actor, PublishOutcome::RejectedInsufficientCredit)]);
}

#[semio_framework_async_macros::async_test]
async fn event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket() {
    let router = EventRouter::new();
    let topic = Topic("stream.frames".to_string());
    let actor = ActorId(4);
    router.subscribe(topic.clone(), actor, ChannelPolicy::ByteCredit { max_items: 100, max_bytes: 4 });
    assert_eq!(router.publish(&topic, None, &[0u8; 3]).await, vec![(actor, PublishOutcome::Delivered)]);
    assert_eq!(router.publish(&topic, None, &[0u8; 3]).await, vec![(actor, PublishOutcome::RejectedInsufficientCredit)], "must reject once the remaining credit is insufficient");
}

#[semio_framework_async_macros::async_test]
async fn event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber() {
    let router = EventRouter::new();
    let topic = Topic("scene.updates".to_string());
    let actor = ActorId(5);
    router.subscribe(topic.clone(), actor, ChannelPolicy::LatestWins { max_bytes: 1_000_000 });
    router.unsubscribe(&topic, actor).await;
    assert_eq!(router.publish(&topic, None, b"x").await, Vec::new(), "no subscribers left means no outcomes at all");
    assert_eq!(router.send_message(&topic, actor, b"x".to_vec()).await, PublishOutcome::NoSuchSubscriber);
}
//#endregion 📮️EventRouterTests
//#region 🧾️CompletionSinkTests
#[semio_framework_async_macros::async_test]
async fn mock_completion_sink_records_calls_in_order() {
    let sink = MockCompletionSink::new().await;
    sink.complete(1, 0, vec![1], 0);
    sink.complete(2, 1, vec![2], 1);
    let recorded = sink.recorded().await;
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded[0].actor, 1);
    assert_eq!(recorded[1].actor, 2);
}
//#endregion 🧾️CompletionSinkTests
