//! 🧪️ Window ownership for local hover clearing.
use super::{HoverSurface, window_hover_can_clear};

#[test]
fn window_leave_only_clears_the_current_hover_owner() {
    for row in include_str!("🧫️fixtures/hover-window.tsv").lines().skip(1) {
        let columns: Vec<_> = row.split('\t').collect();
        let [name, channel, owner, requested, window, clear] = columns.as_slice() else {
            panic!("invalid hover owner fixture row {row}");
        };
        let optional = |value: &'static str| (!value.is_empty()).then_some(value);
        let source = |value| optional(value).map(|value| (value != "@panel").then_some(value));
        let owner = source(owner).map(HoverSurface::from_window);
        assert_eq!(
            window_hover_can_clear(optional(channel), owner.as_ref(), requested, source(window)),
            clear.parse::<bool>().unwrap(),
            "{name}",
        );
    }
}

#[test]
fn hover_owner_projection_reads_only_the_current_channel_with_a_live_mark() {
    use semio_plugin_decl_views::{InteractionHoverState, InteractionView, PeerPresenceRoot};
    let state = protocol::InteractionState::default();
    let draft = std::collections::BTreeMap::new();
    let window = |value: &'static str| (value != "@panel").then_some(value);
    for row in include_str!("🧫️fixtures/hover-source-projection.tsv").lines().skip(1) {
        let columns: Vec<_> = row.split('\t').collect();
        let [name, owner, channel, ids, expected] = columns.as_slice() else {
            panic!("invalid hover projection fixture row {row}");
        };
        let marks = InteractionHoverState::from([("graph".into(), protocol::DomainHover {
            channel: (*channel).into(),
            ids: ids.split(',').filter(|id| !id.is_empty()).map(str::to_owned).collect(),
        })]);
        let owners = (*owner != "-").then(|| ("graph".into(), HoverSurface::from_window(window(owner)))).into_iter().collect();
        let view = InteractionView::new(&state, &marks, &PeerPresenceRoot::EMPTY, &draft).with_hover_owners(&owners);
        assert_eq!(view.hover_source_window("graph", "pointer"), (*expected != "-").then(|| window(expected)), "{name}");
    }
}
