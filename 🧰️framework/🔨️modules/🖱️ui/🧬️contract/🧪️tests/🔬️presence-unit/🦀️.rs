use super::*;

use crate::schema_metadata::{ContractFixture, ContractFixtureFlags};

/// 🧫 Decodes the overlay fixture through this scope's `ContractFixture` export — the Rust half
/// `🧬️schema/🦀️.rs` declares — and still compares the re-serialized update with the committed wire text,
/// which is what proves the three own-presence flags stay separate rather than collapsing into one.
#[test]
fn presence_overlay_fixture_preserves_separate_own_flags() {
    const FIXTURE: &str = include_str!("../../🧫️fixtures/👥️presence-overlay.json");
    let fixture: ContractFixture = serde_json::from_str(FIXTURE).unwrap();
    let wire: serde_json::Value = serde_json::from_str(FIXTURE).unwrap();
    assert_eq!(fixture.cases.len(), 4);
    for (case, wire_case) in fixture.cases.iter().zip(wire["cases"].as_array().unwrap()) {
        assert_eq!(serde_json::to_value(&case.update).unwrap(), wire_case["update"]);
        assert_eq!(ContractFixtureFlags { selected: case.update.own.selected, hovered: case.update.own.hovered, previewed: case.update.own.previewed }, case.expected);
        assert_eq!(case.update.node_key, "item:根,1");
    }
}

#[test]
fn activity_defaults_to_idle_and_round_trips() {
    assert_eq!(Activity::default(), Activity::Idle);
    for activity in [Activity::Waiting, Activity::Loading, Activity::Idle, Activity::Finished] {
        let json = serde_json::to_string(&activity).expect("serialize");
        let back: Activity = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(activity, back);
    }
}

#[test]
fn peer_mark_round_trips_and_omits_false_flags() {
    let mark = PeerMark { actor: "actor-1".into(), color: Some(3), hovered: true, selected: false, label: "AS".into() };
    let json = serde_json::to_value(&mark).expect("serialize");
    assert_eq!(json.get("hovered").and_then(|v| v.as_bool()), Some(true));
    assert!(json.get("selected").is_none());
    let back: PeerMark = serde_json::from_value(json).expect("deserialize");
    assert_eq!(mark, back);
}

#[test]
fn own_presence_default_serializes_to_empty_object() {
    let json = serde_json::to_value(OwnPresence::default()).expect("serialize");
    assert_eq!(json, serde_json::json!({}));
}

#[test]
fn presence_update_round_trips_with_peers() {
    let update = PresenceUpdate {
        surface: crate::SurfaceId::try_from("note.play.navigator").expect("bounded surface id"),
        node_key: "row-9".into(),
        own: OwnPresence { hovered: true, selected: true, previewed: false, color: Some(2) },
        peers: vec![PeerMark { actor: "a".into(), color: Some(1), hovered: true, selected: false, label: "A".into() }, PeerMark { actor: "b".into(), color: None, hovered: false, selected: true, label: "B".into() }],
        ttl_ms: 4_000,
    };
    let first = serde_json::to_string(&update).expect("serialize");
    let deserialized: PresenceUpdate = serde_json::from_str(&first).expect("deserialize");
    let second = serde_json::to_string(&deserialized).expect("re-serialize");
    assert_eq!(first, second);
    assert_eq!(update, deserialized);
}

#[test]
fn presence_update_omits_empty_peers() {
    let update = PresenceUpdate { surface: crate::SurfaceId::try_from("s").expect("bounded surface id"), node_key: "k".into(), own: OwnPresence::default(), peers: Vec::new(), ttl_ms: 1_000 };
    let json = serde_json::to_value(&update).expect("serialize");
    assert!(json.get("peers").is_none());
}
