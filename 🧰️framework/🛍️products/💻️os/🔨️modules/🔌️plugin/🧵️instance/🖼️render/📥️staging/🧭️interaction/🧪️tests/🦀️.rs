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
