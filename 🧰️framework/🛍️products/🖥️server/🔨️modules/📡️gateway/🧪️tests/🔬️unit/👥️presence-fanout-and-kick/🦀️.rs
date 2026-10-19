//! 👥️ Fanout lanes and the kick signal of the gateway unit laws. See [`super`].

use super::*;

//#region 🔖️Fanout
#[tokio::test]
async fn a_lane_reaches_every_subscriber_and_is_dropped_when_empty() {
    let fanout = Fanout::new();
    assert_eq!(fanout.publish("stream:a", vec![1]), 0);
    let mut first = fanout.subscribe("stream:a");
    let mut second = fanout.subscribe("stream:a");
    assert_eq!(fanout.lanes(), 1);
    assert_eq!(fanout.publish("stream:a", vec![9]), 2);
    assert_eq!(first.recv().await, Some(vec![9]));
    assert_eq!(second.recv().await, Some(vec![9]));
    drop(first);
    assert_eq!(fanout.lanes(), 1);
    drop(second);
    assert_eq!(fanout.lanes(), 0);
}

#[test]
fn lane_keys_keep_the_durable_and_ephemeral_lanes_apart() {
    let scope = Scope("space-1".into());
    assert_eq!(document_lane(&scope), "document:space-1");
    assert_eq!(ephemeral_lane(&scope), "ephemeral:space-1");
    assert_eq!(stream_lane(&actor("c1")), "stream:t1/counter/c1");
}
//#endregion 🔖️Fanout

//#region 🔖️Kick
#[tokio::test]
async fn a_kick_fired_before_the_wait_is_still_observed() {
    let kicks = KickMap::new();
    kicks.kick("session-1");
    tokio::time::timeout(Duration::from_millis(200), kicks.kicked("session-1")).await.expect("kick must be observed");
    assert_eq!(kicks.tracked(), 1);
    kicks.forget("session-1");
    assert_eq!(kicks.tracked(), 0);
    assert!(tokio::time::timeout(Duration::from_millis(20), kicks.kicked("session-2")).await.is_err());
}
//#endregion 🔖️Kick
