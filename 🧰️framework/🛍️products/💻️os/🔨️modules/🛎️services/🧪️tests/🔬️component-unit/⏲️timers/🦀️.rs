//! ⏲️ Tests of the timer wheel: the pure arithmetic (ordering, boundaries, repeats, disarm, quota) and the finite driver posting through the completion sink.

use semio_framework_actor::PackageId;
use std::sync::Arc;
use std::time::Duration;
use super::super::completion_sink::{CompletionRecord, MockCompletionSink};
use super::super::timer_wheel::{TimerError, TimerId, TimerWheel, WheelCore};
use super::support::test_pool;

//#region ⏲️WheelCoreTests
#[semio_framework_async_macros::async_test]
async fn wheel_core_pop_expired_fires_in_expiry_order_not_arm_order() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    let late = wheel.arm(plugin.clone(), 1, 0, 0, 200, None).unwrap();
    let early = wheel.arm(plugin, 2, 0, 0, 100, None).unwrap();
    let fired: Vec<TimerId> = wheel.pop_expired(1_000).into_iter().map(|f| f.id).collect();
    assert_eq!(fired, vec![early, late], "expiry order must win over arm order");
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_pop_expired_respects_now_ms_boundary() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    wheel.arm(plugin, 1, 0, 0, 500, None).unwrap();
    assert!(wheel.pop_expired(400).is_empty(), "must not fire before its expiry");
    assert_eq!(wheel.pop_expired(500).len(), 1, "must fire once now_ms reaches the expiry");
}

/// ⏲️ Jumps far past several repeat periods: a naive `expiry += repeat` applied once would
/// still land before `now_ms` and fire again immediately on the next call; the catch-up loop
/// in `pop_expired` must land beyond `now_ms` instead.
#[semio_framework_async_macros::async_test]
async fn wheel_core_repeat_rearms_and_catches_up_without_drift_accumulation() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    let id = wheel.arm(plugin, 1, 0, 0, 100, Some(100)).unwrap();
    let first = wheel.pop_expired(100);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].id, id);
    let second = wheel.pop_expired(450);
    assert_eq!(second.len(), 1, "a repeating timer must fire exactly once per pop_expired call even after a large time jump");
    assert!(wheel.next_expiry_ms().unwrap() > 450);
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_disarm_prevents_a_future_fire() {
    let mut wheel = WheelCore::new(10).await;
    let plugin = PackageId("p".to_string());
    let id = wheel.arm(plugin, 1, 0, 0, 100, None).unwrap();
    assert!(wheel.disarm(id));
    assert!(wheel.pop_expired(1_000).is_empty());
    assert!(!wheel.disarm(id), "disarming twice must report false the second time");
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_rejects_arm_past_the_per_plugin_quota_with_a_typed_error() {
    let mut wheel = WheelCore::new(2).await;
    let plugin = PackageId("p".to_string());
    wheel.arm(plugin.clone(), 1, 0, 0, 100, None).unwrap();
    wheel.arm(plugin.clone(), 1, 0, 0, 200, None).unwrap();
    let result = wheel.arm(plugin.clone(), 1, 0, 0, 300, None);
    assert_eq!(result, Err(TimerError::QuotaExceeded { plugin: plugin.clone(), limit: 2 }));
    assert_eq!(wheel.armed_count(&plugin), 2, "a rejected arm must leave the wheel untouched");
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_disarm_frees_quota_for_a_new_arm() {
    let mut wheel = WheelCore::new(1).await;
    let plugin = PackageId("p".to_string());
    let id = wheel.arm(plugin.clone(), 1, 0, 0, 100, None).unwrap();
    assert!(wheel.arm(plugin.clone(), 1, 0, 0, 200, None).is_err());
    wheel.disarm(id);
    assert!(wheel.arm(plugin, 1, 0, 0, 200, None).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn wheel_core_expiry_batch_preserves_due_remainder() {
    let mut wheel = WheelCore::new(32).await;
    let plugin = PackageId("batch".to_string());
    for actor in 0..20 {
        wheel.arm(plugin.clone(), actor, 0, 0, 100, None).expect("timer arm");
    }
    assert_eq!(wheel.pop_expired_batch(100, 8).len(), 8);
    assert_eq!(wheel.pop_expired_batch(100, 8).len(), 8);
    assert_eq!(wheel.pop_expired_batch(100, 8).len(), 4);
    assert!(wheel.pop_expired_batch(100, 8).is_empty());
}
//#endregion ⏲️WheelCoreTests
//#region ⏲️TimerWheelDriverTests
/// ⏱️ Polls `sink.recorded()` on a short real-time tick until it is non-empty or `timeout`
/// elapses while finite timer turns use the pool's real monotonic clock.
async fn wait_for_completions(sink: &MockCompletionSink, timeout: Duration) -> Vec<CompletionRecord> {
    let start = std::time::Instant::now();
    loop {
        let recorded = sink.recorded().await;
        if !recorded.is_empty() || start.elapsed() >= timeout {
            return recorded;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[semio_framework_async_macros::async_test]
async fn timer_wheel_driver_posts_a_fired_timer_through_the_completion_sink() {
    let pool = test_pool(2);
    let wheel = TimerWheel::new(10).await;
    let sink = Arc::new(MockCompletionSink::new().await);
    wheel.spawn_driver(&pool, sink.clone());
    let now = pool.now_ms();
    wheel.arm(PackageId("plugin-a".to_string()), 42, 3, 1, now + 30, None).await.expect("arm should succeed");
    assert!(sink.recorded().await.is_empty(), "must not fire before its deadline");
    let recorded = wait_for_completions(&sink, Duration::from_secs(5)).await;
    assert_eq!(recorded.len(), 1, "the driver must post exactly one completion for the fired timer");
    assert_eq!(recorded[0].actor, 42);
    assert_eq!(recorded[0].generation, 3);
    assert_eq!(recorded[0].lane, 1);
    pool.shutdown();
}
//#endregion ⏲️TimerWheelDriverTests
