
use super::*;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(value: &T) {
    let json = serde_json::to_string(value).expect("serialize");
    let back: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(value, &back);
}

#[test]
fn layout_spec_stack_roundtrips() {
    roundtrip(&LayoutSpec::Stack(StackLayout { axis: Axis::Vertical, gap: SpaceToken::Md, padding: EdgeSpace::All(SpaceToken::Sm), align: Align::Center, justify: Justify::SpaceBetween, grow: true, wrap: false }));
}

#[test]
fn layout_spec_grid_roundtrips() {
    roundtrip(&LayoutSpec::Grid(GridLayout {
        column_gap: SpaceToken::Sm,
        row_gap: SpaceToken::None,
        padding: EdgeSpace::Each { top: SpaceToken::Xs, right: SpaceToken::Sm, bottom: SpaceToken::Xs, left: SpaceToken::Sm },
        align: Align::Stretch,
        justify: Justify::Start,
        ..GridLayout::default()
    }));
}

#[test]
fn grid_tracks_max_plus_one_returns_the_exact_track() {
    let mut grid = GridLayout::default();
    for _ in 0..UI_GRID_TRACKS {
        assert!(grid.try_push_column(GridTrack::Fraction(1)).is_ok());
    }
    assert_eq!(grid.try_push_column(GridTrack::Fixed(SpaceToken::Lg)), Err(GridTrack::Fixed(SpaceToken::Lg)));
    assert_eq!(grid.columns.len(), UI_GRID_TRACKS);
}

#[test]
fn layout_spec_overlay_roundtrips() {
    roundtrip(&LayoutSpec::Overlay(OverlayLayout { anchor: Anchor::BottomEnd, inset: EdgeSpace::Symmetric { vertical: SpaceToken::Md, horizontal: SpaceToken::Lg }, dismissible: true }));
}

#[test]
fn layout_spec_scroll_roundtrips() {
    roundtrip(&LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, padding: EdgeSpace::default(), sizing: Sizing::Fill }));
}

#[test]
fn layout_spec_absolute_roundtrips() {
    roundtrip(&LayoutSpec::Absolute(AbsoluteLayout { sizing_width: Sizing::Fixed(SpaceToken::Xl), sizing_height: Sizing::Hug }));
}

#[test]
fn layout_spec_leaf_roundtrips() {
    roundtrip(&LayoutSpec::Leaf(LeafLayout { width: Sizing::Hug, height: Sizing::Fill }));
}

#[test]
fn window_layout_node_tagged_roundtrips() {
    let tree = WindowLayoutNode::Split {
        axis: Axis::Horizontal,
        size: Some(0.5),
        children: vec![
            WindowLayoutNode::Stack {
                size: Some(0.5),
                active_window_kind_id: Some("editor".into()),
                children: vec![WindowLayoutNode::Window { window_kind_id: "editor".into(), title: Some("Editor".into()), instance_id: None, template_id: None, corner: Some(WindowStackCorner::TopRight) }],
            },
            WindowLayoutNode::Stack { size: None, active_window_kind_id: None, children: vec![] },
        ],
    };
    roundtrip(&tree);
    let json = serde_json::to_string(&tree).expect("serialize");
    assert!(json.contains("\"kind\":\"split\""));
}

#[test]
fn window_layout_roundtrips() {
    roundtrip(&WindowLayout { root: WindowLayoutNode::Stack { size: None, active_window_kind_id: None, children: vec![] } });
}
