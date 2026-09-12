use super::reactor_driver::*;
use semio_framework_ui_contract as ui_contract;
use semio_framework_ui_runtime::{ComponentTree, TreeNode};

fn leaf(key: &str) -> ComponentTree {
    ComponentTree { root: TreeNode::try_new(key, ui_contract::Component::Separator(ui_contract::SeparatorProps {})).unwrap_or_else(|_| panic!("bounded test tree")) }
}

/// 🎯️ M1: `PATCHES.revision` reads 0 for a surface `poll` has never rendered, and
/// `ui_runtime::is_stale_intent` correctly classifies an intent at/behind/beyond the tolerance
/// against it — the exact two calls `poll`'s `Event::UiIntent` arm chains together.
#[semio_framework_async_macros::async_test]
async fn revision_guard_never_rejects_an_intent_at_the_never_rendered_default() {
    let current = patches_revision("never-rendered").await;
    assert_eq!(current, ui_contract::UiRevision(0));
    assert!(!semio_framework_ui_runtime::is_stale_intent(ui_contract::UiRevision(0), current, semio_framework_ui_runtime::DEFAULT_REVISION_TOLERANCE));
}

/// 🎯️ M1: after one real render bumps `PATCHES`'s revision to 1, an intent stamped at revision
/// 0 (trailing by exactly the default tolerance of 1) is NOT stale — but one that trails by 2
/// (as if two more renders had happened since the client last saw the surface) IS. This is
/// exactly the acceptance criterion "an intent whose revision trails by 2 produces no patch and
/// no command" reduced to the guard `poll` evaluates before ever reaching dispatch.
#[semio_framework_async_macros::async_test]
async fn revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance() {
    patches_diff("s", leaf("root")).await;
    patches_diff("s", leaf("root2")).await;
    patches_diff("s", leaf("root3")).await;
    let current = patches_revision("s").await;
    assert_eq!(current, ui_contract::UiRevision(3));
    assert!(!semio_framework_ui_runtime::is_stale_intent(ui_contract::UiRevision(2), current, semio_framework_ui_runtime::DEFAULT_REVISION_TOLERANCE), "trailing by exactly the tolerance must still dispatch");
    assert!(semio_framework_ui_runtime::is_stale_intent(ui_contract::UiRevision(1), current, semio_framework_ui_runtime::DEFAULT_REVISION_TOLERANCE), "trailing by 2 must be rejected — no patch, no command");
}

/// 👥️ M2 acceptance: a turn where only presence changed touches `PRESENCE` and leaves `PATCHES`
/// completely untouched — no revision bump, no patch, because the two channels share no code
/// path (by construction: `stamp_and_cache_interaction_ui` writes `pending_presence`, never
/// `PENDING_PATCHES`/`PATCHES`).
#[semio_framework_async_macros::async_test]
async fn a_presence_only_turn_emits_presence_and_zero_patches() {
    let before = patches_revision("presence-only").await;
    presence_record_own("presence-only", "row-1", ui_contract::OwnPresence { selected: true, ..Default::default() }, 4_000).await;
    let updates = presence_expire_and_flush(0).await;
    assert_eq!(updates.len(), 1);
    assert!(updates[0].own.selected);
    // 🩹️ Zero ui_patches: the surface's revision is EXACTLY what it was before — nothing was ever
    // diffed against `PATCHES` for it, so there is nothing for a subsequent `poll` to have sent.
    assert_eq!(patches_revision("presence-only").await, before);
}

/// 👥️ M2 acceptance: a burst of same-key presence writes between two flushes coalesces to ONE
/// update per `(surface, node_key)` — the property `PresenceHub` itself guarantees, exercised
/// here through the reactor's own `PRESENCE` wiring rather than the hub in isolation.
#[semio_framework_async_macros::async_test]
async fn a_burst_of_same_key_presence_writes_between_polls_coalesces_to_one_update() {
    let mark = |selected: bool, hovered: bool| ui_contract::PeerMark { actor: "user:bob#s1".into(), color: Some(2), hovered, selected, label: "user:bob#s1".into() };
    presence_record_peer("burst", "row-1", mark(false, true), 4_000, 0).await;
    presence_record_peer("burst", "row-1", mark(true, true), 4_000, 10).await;
    presence_record_peer("burst", "row-1", mark(true, false), 4_000, 20).await;
    let updates = presence_expire_and_flush(20).await;
    assert_eq!(updates.len(), 1, "a burst on one key must cost exactly one update, got {updates:?}");
    assert_eq!(updates[0].peers.len(), 1);
    assert!(updates[0].peers[0].selected);
    assert!(!updates[0].peers[0].hovered, "must reflect the LAST write, not the first");
}

/// 👥️ M2 acceptance: a peer mark not refreshed within its TTL ages out with no goodbye message —
/// the flush after expiry reports the now-empty peer list once, then the slot is forgotten.
#[semio_framework_async_macros::async_test]
async fn ttl_expiry_drops_a_peer_mark_with_no_goodbye_message() {
    let mark = ui_contract::PeerMark { actor: "user:carol#s1".into(), color: Some(4), hovered: true, selected: false, label: "user:carol#s1".into() };
    presence_record_peer("ttl", "row-1", mark, 1_000, 0).await;
    let first = presence_expire_and_flush(0).await;
    assert_eq!(first[0].peers.len(), 1);
    let after_expiry = presence_expire_and_flush(1_000).await;
    assert_eq!(after_expiry.len(), 1, "expiry at the TTL boundary must surface as one more update");
    assert!(after_expiry[0].peers.is_empty(), "the expired peer must be omitted, not sent as a goodbye");
    assert!(presence_expire_and_flush(2_000).await.is_empty(), "the now-empty slot was garbage-collected");
}
