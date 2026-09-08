
use super::*;

fn surface(value: &str) -> SurfaceId {
    SurfaceId::try_from(value).expect("bounded fixture surface")
}

fn mark(actor: &str) -> PeerMark {
    PeerMark { actor: actor.into(), color: Some(1), hovered: true, selected: false, label: actor.to_uppercase() }
}

#[test]
fn presence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them() {
    let mut hub = PresenceHub::new();
    let surface = surface("note.play.navigator");
    hub.record_peer(surface, "row-1", mark("a"), 1_000, 0);

    let first = hub.flush();
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].peers, vec![mark("a")]);

    hub.expire(999);
    assert!(hub.flush().is_empty(), "not yet at the TTL boundary, nothing became dirty");

    hub.expire(1_000);
    let after_expiry = hub.flush();
    assert_eq!(after_expiry.len(), 1, "expiry at the exact TTL boundary must surface as one more update");
    assert!(after_expiry[0].peers.is_empty(), "the flush after expiry must omit the expired peer");

    assert!(hub.flush().is_empty(), "the now-empty slot was garbage-collected, nothing left to report");
}

#[test]
fn a_burst_of_same_key_peer_writes_coalesces_to_one_update() {
    let mut hub = PresenceHub::new();
    let surface = surface("note.play.navigator");
    hub.record_peer(surface.clone(), "row-1", PeerMark { hovered: true, ..mark("a") }, 4_000, 0);
    hub.record_peer(surface.clone(), "row-1", PeerMark { hovered: true, selected: true, ..mark("a") }, 4_000, 10);
    hub.record_peer(surface, "row-1", PeerMark { hovered: false, selected: true, ..mark("a") }, 4_000, 20);

    let updates = hub.flush();
    assert_eq!(updates.len(), 1, "a burst of pointer moves on one key must cost exactly one update");
    assert_eq!(updates[0].peers, vec![PeerMark { hovered: false, selected: true, ..mark("a") }]);
}

#[test]
fn a_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value() {
    let mut hub = PresenceHub::new();
    let surface = surface("note.play.navigator");
    hub.record_own(surface.clone(), "row-1", OwnPresence { hovered: true, ..Default::default() }, 1_000);
    hub.record_own(surface.clone(), "row-1", OwnPresence { hovered: true, selected: true, ..Default::default() }, 1_000);
    let newest = OwnPresence { hovered: false, selected: true, previewed: true, color: Some(2) };
    hub.record_own(surface, "row-1", newest, 1_000);

    let updates = hub.flush();
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].own, newest);
}

#[test]
fn own_presence_never_expires() {
    let mut hub = PresenceHub::new();
    let surface = surface("s");
    hub.record_own(surface, "row-1", OwnPresence { hovered: true, ..Default::default() }, 1_000);
    hub.flush();

    hub.expire(1_000_000);
    assert!(hub.flush().is_empty(), "expire() must never touch own presence, so nothing became dirty");
}

#[test]
fn distinct_peers_on_one_key_are_all_reported_and_expire_independently() {
    let mut hub = PresenceHub::new();
    let surface = surface("s");
    hub.record_peer(surface.clone(), "row-1", mark("a"), 1_000, 0);
    hub.record_peer(surface, "row-1", mark("b"), 5_000, 0);
    let first = hub.flush();
    assert_eq!(first[0].peers.len(), 2);

    hub.expire(1_000);
    let after = hub.flush();
    assert_eq!(after[0].peers, vec![mark("b")], "only the expired peer drops off");
}
