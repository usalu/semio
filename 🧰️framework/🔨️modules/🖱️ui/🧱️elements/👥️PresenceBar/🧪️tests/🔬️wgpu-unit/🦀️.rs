
use super::*;

fn peer(actor: &str, label: &str, role: Option<PresenceRole>) -> PresencePeerRow {
    PresencePeerRow { actor: actor.into(), user_id: None, label: label.into(), role, connected_at_ms: None, color: None }
}

#[semio_framework_async_macros::async_test]
async fn presence_color_wraps_after_twelve_with_lightness_then_saturation_shift() {
    let base = presence_color(0, PresenceAppearance::Light);
    let cycle_one = presence_color(12, PresenceAppearance::Light); // k=1: same hue, lighter (odd k)
    let cycle_two = presence_color(24, PresenceAppearance::Light); // k=2: same hue, desaturated, lightness back to base
    assert_eq!(base.h, cycle_one.h);
    assert_eq!(base.h, cycle_two.h);
    assert_eq!(base.s, cycle_one.s, "k=1 stays under the k>=2 desaturation threshold");
    assert!(cycle_one.l > base.l, "light appearance shifts lightness UP on odd cycles");
    assert!((cycle_two.s - (base.s - 0.25)).abs() < 1e-9, "k=2 desaturates by 0.25");
    assert_eq!(cycle_two.l, base.l, "k=2 is even, so no lightness shift");

    let dark_base = presence_color(0, PresenceAppearance::Dark);
    let dark_cycle_one = presence_color(12, PresenceAppearance::Dark);
    assert!(dark_cycle_one.l < dark_base.l, "dark appearance shifts lightness DOWN on odd cycles");
    assert_eq!(dark_base.s, dark_cycle_one.s);
}

#[semio_framework_async_macros::async_test]
async fn presence_css_var_only_addresses_the_base_cycle() {
    assert_eq!(presence_css_var(0), "var(--presence-0)");
    assert_eq!(presence_css_var(11), "var(--presence-11)");
    assert_eq!(presence_css_var(12), "var(--presence-0)", "wraps modulo 12 — callers past k=0 must render inline HSL instead");
}

#[semio_framework_async_macros::async_test]
async fn build_presence_bar_renders_one_stack_child_per_peer_under_max() {
    let peers = vec![peer("user:a#1", "Alice", Some(PresenceRole::Author)), peer("user:b#1", "Bob", Some(PresenceRole::Spectator))];
    let node = build_presence_bar("s-presence-peers", &peers, None);
    let UiNode::Stack(root) = node else { panic!("expected a Stack root") };
    assert_eq!(root.id.as_deref(), Some("s-presence-peers"));
    assert_eq!(root.children.len(), 2);
    for (child, expected_actor) in root.children.iter().zip(["user:a#1", "user:b#1"]) {
        let UiNode::Stack(peer_stack) = child else { panic!("expected each peer to be a Stack") };
        assert_eq!(peer_stack.id.as_deref(), Some(format!("peer:{expected_actor}").as_str()));
    }
}

#[semio_framework_async_macros::async_test]
async fn build_presence_bar_collapses_past_max_into_one_overflow_node() {
    let mut peers: Vec<PresencePeerRow> = Vec::with_capacity(7);
    for i in 0..7 {
        peers.push(peer(&format!("user:{i}#1"), &format!("Peer {i}"), None));
    }
    let node = build_presence_bar("s-presence-peers", &peers, Some(5));
    let UiNode::Stack(root) = node else { panic!("expected a Stack root") };
    // 5 visible peers + 1 overflow node.
    assert_eq!(root.children.len(), 6);
    let UiNode::Stack(overflow) = root.children.last().unwrap() else { panic!("expected overflow Stack") };
    assert_eq!(overflow.id.as_deref(), Some("peer:overflow"));
}

#[semio_framework_async_macros::async_test]
async fn build_presence_bar_empty_peers_renders_localized_empty_text() {
    let en = build_presence_bar_localized("s-presence-peers", &[], None, Locale::En);
    let de = build_presence_bar_localized("s-presence-peers", &[], None, Locale::De);
    for (node, expected) in [(en, "No one else is here"), (de, "Niemand sonst ist hier")] {
        let UiNode::Stack(root) = node else { panic!("expected a Stack root") };
        assert_eq!(root.children.len(), 1);
        let UiNode::Text(text) = &root.children[0] else { panic!("expected a Text child") };
        assert_eq!(text.value.as_str(), expected);
    }
}
