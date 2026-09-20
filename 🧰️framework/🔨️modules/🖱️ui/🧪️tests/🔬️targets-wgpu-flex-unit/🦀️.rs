use super::*;
use ui_contract::{AbsoluteLayout, Align, Axis, EdgeSpace, GridLayout, GridTrack, Justify, LayoutSpec, LeafLayout, ScrollAxes, ScrollLayout, Sizing, SpaceToken, StackLayout};

//#region 🧪️Fixtures

/// 🧪️ Drives the real two-pass ladder `mounted_layout::MountedLayoutJob` walks — measure every node
/// bottom-up, seed the root's viewport box, arrange every container top-down — so a fixture can never
/// assert against geometry the shipped renderer would not produce.
struct Fixture {
    flex: FlexTree,
    metrics: TreeRowMetrics,
    children: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>,
    kinds: Vec<LayoutNodeKind>,
    sizes: Vec<(f32, f32)>,
}

impl Fixture {
    fn new() -> Self {
        Self { flex: FlexTree::new(), metrics: TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default()), children: Vec::new(), parents: Vec::new(), kinds: Vec::new(), sizes: Vec::new() }
    }

    fn push(&mut self, kind: LayoutNodeKind, parent: Option<usize>, authored: Option<&LayoutSpec>) -> usize {
        let index = self.children.len();
        let parent_kind = parent.map(|parent| self.kinds[parent]);
        assert!(self.flex.push(kind, parent, authored, &self.metrics, parent_kind), "push {index}");
        self.children.push(Vec::new());
        self.parents.push(parent);
        self.kinds.push(kind);
        self.sizes.push((0.0, 0.0));
        if let Some(parent) = parent {
            self.children[parent].push(index);
        }
        index
    }

    /// 📏️ Gives node `index` a measured intrinsic size, standing in for a shaped text run.
    fn measured(&mut self, index: usize, size: (f32, f32)) {
        self.sizes[index] = size;
    }

    fn solve(&mut self, width: f32, height: f32) {
        let sizes = self.sizes.clone();
        let mut measure = |node: usize, _constraint: MeasureConstraint| sizes[node];
        for index in (0..self.children.len()).rev() {
            assert!(self.flex.measure_one(index, &self.children[index], &mut measure), "measure {index}");
        }
        assert!(self.flex.set_root_box(0, FlexRect { x: 0.0, y: 0.0, width, height }), "root box");
        for index in 0..self.children.len() {
            assert!(self.flex.arrange_one(index, &self.children[index], &mut measure), "arrange {index}");
        }
    }

    fn rect(&self, index: usize) -> FlexRect {
        self.flex.rect(index).unwrap_or_else(|| panic!("rect {index}"))
    }
}

fn stack(axis: Axis, align: Align, justify: Justify, wrap: bool) -> LayoutSpec {
    LayoutSpec::Stack(StackLayout { axis, gap: SpaceToken::None, padding: EdgeSpace::default(), align, justify, grow: false, wrap })
}

fn fixed_leaf() -> LayoutSpec {
    LayoutSpec::Leaf(LeafLayout { width: Sizing::Fixed(SpaceToken::Xxl), height: Sizing::Fixed(SpaceToken::Lg) })
}

/// 🧪️ Builds a root container with `count` identical `Fixed(Xxl) x Fixed(Lg)` children — the shape
/// most align/justify/wrap fixtures need. Xxl is 38.4px, Lg is 19.2px on the shared ramp.
fn fixed_children(root: LayoutSpec, count: usize, width: f32, height: f32) -> Fixture {
    let mut fixture = Fixture::new();
    let parent = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&root));
    let leaf = fixed_leaf();
    for _ in 0..count {
        fixture.push(LayoutNodeKind::Leaf, Some(parent), Some(&leaf));
    }
    fixture.solve(width, height);
    fixture
}

fn close(left: f32, right: f32) -> bool {
    (left - right).abs() < 0.01
}

fn overlay_flow_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📐️overlay-flow/🔣️.json")).expect("overlay flow fixture")
}

fn fixture_layout(fixture: &serde_json::Value, node: &str) -> LayoutSpec {
    serde_json::from_value(fixture[node]["layout"].clone()).unwrap_or_else(|error| panic!("{node} layout: {error}"))
}

fn fixture_rect(fixture: &serde_json::Value, node: &str) -> FlexRect {
    let rect = &fixture[node]["expectedRect"];
    let number = |field: &str| rect[field].as_f64().unwrap_or_else(|| panic!("{node}.{field}")) as f32;
    FlexRect { x: number("x"), y: number("y"), width: number("width"), height: number("height") }
}

fn assert_rect(actual: FlexRect, expected: FlexRect) {
    assert!(close(actual.x, expected.x) && close(actual.y, expected.y) && close(actual.width, expected.width) && close(actual.height, expected.height), "expected {expected:?}, got {actual:?}");
}

//#endregion 🧪️Fixtures

//#region 🚫️NoTestGating

/// 🚫️ The layout engine is production code. Before ticket 26/09/17/WGPU-RENDERER-REACT-PARITY every
/// item that performed the flex computation here was `#[cfg(test)]`, so the shipped renderer ran a
/// hand-rolled "stretch everything equally" fallback instead — the single largest source of "elements
/// placed totally different" against React. This law fails the moment any of it is test-gated again.
#[test]
fn the_flex_engine_carries_no_test_gating() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/📐️flex/🦀️.rs");
    let gated: Vec<&str> = source.lines().map(str::trim).filter(|line| line.starts_with("#[cfg(test)]") || line.starts_with("#[cfg(any(test")).collect();
    assert_eq!(gated.len(), 1, "only the trailing `mod tests` declaration may be test-gated, found {gated:?}");
    let tail: Vec<&str> = source.lines().map(str::trim).skip_while(|line| !line.starts_with("#[cfg(test)]")).take(3).collect();
    assert!(tail.iter().any(|line| line.starts_with("mod tests")), "the one gate must be the test module, found {tail:?}");
}

//#endregion 🚫️NoTestGating

//#region 📏️SpaceTokenRamp

/// 📏️ The seven-value ramp React resolves through `spaceTokenRem`/`SPACE_TOKEN_MULTIPLIER` —
/// `0, 1, 2, 4, 6, 8, 12` steps of the compact `--ui-spacing` (3.2px). The pre-parity wgpu target
/// collapsed these onto three buckets (`none`/`tight`/`loose`), rendering `Xs` and `Sm` identically
/// and shrinking `Md` fourfold.
#[test]
fn every_space_token_resolves_to_reacts_own_pixel_ramp() {
    let ramp: Vec<f32> = [SpaceToken::None, SpaceToken::Xs, SpaceToken::Sm, SpaceToken::Md, SpaceToken::Lg, SpaceToken::Xl, SpaceToken::Xxl].into_iter().map(SpaceToken::px).collect();
    for (resolved, expected) in ramp.iter().zip([0.0, 3.2, 6.4, 12.8, 19.2, 25.6, 38.4]) {
        assert!(close(*resolved, expected), "expected {expected}, got {resolved}");
    }
    let distinct: std::collections::BTreeSet<u32> = ramp.iter().map(|value| value.to_bits()).collect();
    assert_eq!(distinct.len(), 7, "all seven tokens must be visibly distinct");
}

#[test]
fn an_asymmetric_edge_space_keeps_all_four_sides() {
    let edge = EdgeSpace::Each { top: SpaceToken::Xs, right: SpaceToken::Sm, bottom: SpaceToken::Md, left: SpaceToken::Lg };
    let px = edge.px();
    assert!(close(px.top, 3.2) && close(px.right, 6.4) && close(px.bottom, 12.8) && close(px.left, 19.2), "got {px:?}");
}

#[test]
fn a_stacks_padding_is_applied_per_side_not_from_one_sampled_side() {
    let root = LayoutSpec::Stack(StackLayout {
        axis: Axis::Vertical,
        gap: SpaceToken::None,
        padding: EdgeSpace::Each { top: SpaceToken::Md, right: SpaceToken::None, bottom: SpaceToken::None, left: SpaceToken::Xl },
        align: Align::Start,
        justify: Justify::Start,
        grow: false,
        wrap: false,
    });
    let fixture = fixed_children(root, 1, 200.0, 200.0);
    let child = fixture.rect(1);
    assert!(close(child.x, 25.6) && close(child.y, 12.8), "left/top padding resolve independently, got {child:?}");
}

//#endregion 📏️SpaceTokenRamp

//#region ↕️AlignJustifyWrapGrow

#[test]
fn justify_space_between_pushes_children_to_both_ends_instead_of_growing_them() {
    let fixture = fixed_children(stack(Axis::Horizontal, Align::Start, Justify::SpaceBetween, false), 3, 300.0, 100.0);
    assert!(close(fixture.rect(1).x, 0.0));
    assert!(close(fixture.rect(2).x, 130.8), "got {}", fixture.rect(2).x);
    assert!(close(fixture.rect(3).x, 261.6), "got {}", fixture.rect(3).x);
    for index in 1..=3 {
        assert!(close(fixture.rect(index).width, 38.4), "space-between distributes BETWEEN children, never into them");
    }
}

#[test]
fn justify_center_and_end_place_the_run_without_resizing_it() {
    let centered = fixed_children(stack(Axis::Horizontal, Align::Start, Justify::Center, false), 2, 200.0, 100.0);
    assert!(close(centered.rect(1).x, 61.6), "got {}", centered.rect(1).x);
    let ended = fixed_children(stack(Axis::Horizontal, Align::Start, Justify::End, false), 2, 200.0, 100.0);
    assert!(close(ended.rect(1).x, 123.2), "got {}", ended.rect(1).x);
}

#[test]
fn align_items_positions_the_cross_axis_instead_of_always_stretching() {
    let start = fixed_children(stack(Axis::Horizontal, Align::Start, Justify::Start, false), 1, 200.0, 100.0);
    assert!(close(start.rect(1).y, 0.0) && close(start.rect(1).height, 19.2));
    let center = fixed_children(stack(Axis::Horizontal, Align::Center, Justify::Start, false), 1, 200.0, 100.0);
    assert!(close(center.rect(1).y, 40.4), "got {}", center.rect(1).y);
    let end = fixed_children(stack(Axis::Horizontal, Align::End, Justify::Start, false), 1, 200.0, 100.0);
    assert!(close(end.rect(1).y, 80.8), "got {}", end.rect(1).y);
}

#[test]
fn wrap_reflows_an_overflowing_row_onto_a_second_line() {
    let wrapped = fixed_children(stack(Axis::Horizontal, Align::Start, Justify::Start, true), 3, 100.0, 100.0);
    assert!(close(wrapped.rect(1).y, 0.0));
    assert!(close(wrapped.rect(2).y, 0.0));
    assert!(wrapped.rect(3).y > 0.0, "a third 38.4px child cannot fit on a 100px line and must wrap");
    let nowrap = fixed_children(stack(Axis::Horizontal, Align::Start, Justify::Start, false), 3, 100.0, 100.0);
    assert!(close(nowrap.rect(3).y, 0.0));
}

#[test]
fn only_a_grow_marked_child_absorbs_leftover_space() {
    let root = stack(Axis::Vertical, Align::Stretch, Justify::Start, false);
    let grower = LayoutSpec::Stack(StackLayout { axis: Axis::Vertical, gap: SpaceToken::None, padding: EdgeSpace::default(), align: Align::Stretch, justify: Justify::Start, grow: true, wrap: false });
    let steady = fixed_leaf();
    let mut fixture = Fixture::new();
    let parent = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&root));
    let first = fixture.push(LayoutNodeKind::Leaf, Some(parent), Some(&steady));
    let second = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, Some(parent), Some(&grower));
    fixture.solve(200.0, 100.0);
    assert!(close(fixture.rect(first).height, 19.2), "a grow:false child keeps its intrinsic height, got {}", fixture.rect(first).height);
    assert!(close(fixture.rect(second).height, 80.8), "the grow:true sibling takes the whole remainder, got {}", fixture.rect(second).height);
}

//#endregion ↕️AlignJustifyWrapGrow

//#region 🔲️GridScrollOverlayAbsolute

#[test]
fn a_two_column_grid_renders_two_columns_not_two_rows() {
    let mut grid = GridLayout::default();
    assert_eq!(grid.try_push_column(GridTrack::Fraction(1)), Ok(()));
    assert_eq!(grid.try_push_column(GridTrack::Fraction(1)), Ok(()));
    assert_eq!(grid.try_push_row(GridTrack::Auto), Ok(()));
    let fixture = fixed_children(LayoutSpec::Grid(grid), 2, 200.0, 100.0);
    assert!(close(fixture.rect(1).x, 0.0));
    assert!(close(fixture.rect(2).x, 100.0), "the second cell sits in the second COLUMN, not on a second row, got {}", fixture.rect(2).x);
    assert!(close(fixture.rect(1).y, fixture.rect(2).y));
}

#[test]
fn an_overlay_is_an_in_flow_positioning_context_and_absolute_stays_out_of_flow() {
    let source = overlay_flow_fixture();
    let mut fixture = Fixture::new();
    let root_layout = fixture_layout(&source, "root");
    let overlay_layout = fixture_layout(&source, "overlay");
    let content_layout = fixture_layout(&source, "content");
    let absolute_layout = fixture_layout(&source, "absolute");
    let following_layout = fixture_layout(&source, "following");
    let parent = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&root_layout));
    let overlay = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, Some(parent), Some(&overlay_layout));
    let content = fixture.push(LayoutNodeKind::Leaf, Some(overlay), Some(&content_layout));
    let absolute = fixture.push(LayoutNodeKind::Leaf, Some(parent), Some(&absolute_layout));
    let following = fixture.push(LayoutNodeKind::Leaf, Some(parent), Some(&following_layout));
    fixture.solve(source["viewport"]["width"].as_f64().expect("viewport width") as f32, source["viewport"]["height"].as_f64().expect("viewport height") as f32);
    assert_rect(fixture.rect(parent), fixture_rect(&source, "root"));
    assert_rect(fixture.rect(overlay), fixture_rect(&source, "overlay"));
    assert_rect(fixture.rect(content), fixture_rect(&source, "content"));
    assert_rect(fixture.rect(absolute), fixture_rect(&source, "absolute"));
    assert_rect(fixture.rect(following), fixture_rect(&source, "following"));
}

#[test]
fn taffy_confirms_the_shared_overlay_flow_geometry() {
    let source = overlay_flow_fixture();
    let LayoutSpec::Stack(root) = fixture_layout(&source, "root") else { panic!("root stack") };
    let LayoutSpec::Overlay(overlay) = fixture_layout(&source, "overlay") else { panic!("overlay") };
    let LayoutSpec::Leaf(content) = fixture_layout(&source, "content") else { panic!("content leaf") };
    let LayoutSpec::Absolute(absolute) = fixture_layout(&source, "absolute") else { panic!("absolute") };
    let LayoutSpec::Leaf(following) = fixture_layout(&source, "following") else { panic!("following leaf") };
    let dimension = |sizing: Sizing| match sizing {
        Sizing::Hug => taffy::style::Dimension::auto(),
        Sizing::Fill => taffy::style::Dimension::percent(1.0),
        Sizing::Fixed(token) => taffy::style::Dimension::length(token.px()),
    };
    let edge = overlay.inset.px();
    let mut oracle: taffy::TaffyTree<()> = taffy::TaffyTree::new();
    oracle.disable_rounding();
    let content_node = oracle.new_leaf_with_context(taffy::Style { size: taffy::geometry::Size { width: dimension(content.width), height: dimension(content.height) }, ..Default::default() }, ()).expect("taffy content");
    let overlay_node = oracle
        .new_with_children(
            taffy::Style {
                display: taffy::style::Display::Flex,
                position: taffy::style::Position::Relative,
                flex_direction: taffy::style::FlexDirection::Column,
                padding: taffy::geometry::Rect {
                    top: taffy::style::LengthPercentage::length(edge.top),
                    right: taffy::style::LengthPercentage::length(edge.right),
                    bottom: taffy::style::LengthPercentage::length(edge.bottom),
                    left: taffy::style::LengthPercentage::length(edge.left),
                },
                ..Default::default()
            },
            &[content_node],
        )
        .expect("taffy overlay");
    let absolute_node = oracle
        .new_leaf_with_context(taffy::Style { position: taffy::style::Position::Absolute, size: taffy::geometry::Size { width: dimension(absolute.sizing_width), height: dimension(absolute.sizing_height) }, ..Default::default() }, ())
        .expect("taffy absolute");
    let following_node = oracle.new_leaf_with_context(taffy::Style { size: taffy::geometry::Size { width: dimension(following.width), height: dimension(following.height) }, ..Default::default() }, ()).expect("taffy following");
    let width = source["viewport"]["width"].as_f64().expect("viewport width") as f32;
    let height = source["viewport"]["height"].as_f64().expect("viewport height") as f32;
    let root_node = oracle
        .new_with_children(
            taffy::Style {
                display: taffy::style::Display::Flex,
                flex_direction: match root.axis {
                    Axis::Horizontal => taffy::style::FlexDirection::Row,
                    Axis::Vertical => taffy::style::FlexDirection::Column,
                },
                align_items: Some(taffy::style::AlignItems::Stretch),
                size: taffy::geometry::Size { width: taffy::style::Dimension::length(width), height: taffy::style::Dimension::length(height) },
                ..Default::default()
            },
            &[overlay_node, absolute_node, following_node],
        )
        .expect("taffy root");
    oracle
        .compute_layout_with_measure(root_node, taffy::geometry::Size { width: taffy::AvailableSpace::Definite(width), height: taffy::AvailableSpace::Definite(height) }, |known, _, _, _, _| taffy::geometry::Size {
            width: known.width.unwrap_or(0.0),
            height: known.height.unwrap_or(0.0),
        })
        .expect("taffy solve");
    let solved = |node| {
        let layout = oracle.layout(node).expect("taffy layout");
        FlexRect { x: layout.location.x, y: layout.location.y, width: layout.size.width, height: layout.size.height }
    };
    assert_rect(solved(root_node), fixture_rect(&source, "root"));
    assert_rect(solved(overlay_node), fixture_rect(&source, "overlay"));
    assert_rect(solved(content_node), fixture_rect(&source, "content"));
    assert_rect(solved(absolute_node), fixture_rect(&source, "absolute"));
    assert_rect(solved(following_node), fixture_rect(&source, "following"));
}

#[test]
fn an_absolute_child_is_out_of_flow_with_its_own_size() {
    let root = stack(Axis::Vertical, Align::Stretch, Justify::Start, false);
    let absolute = LayoutSpec::Absolute(AbsoluteLayout { sizing_width: Sizing::Fixed(SpaceToken::Xxl), sizing_height: Sizing::Fixed(SpaceToken::Xl) });
    let mut fixture = Fixture::new();
    let parent = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&root));
    let pinned = fixture.push(LayoutNodeKind::Leaf, Some(parent), Some(&absolute));
    fixture.solve(200.0, 100.0);
    let rect = fixture.rect(pinned);
    assert!(close(rect.width, 38.4) && close(rect.height, 25.6), "got {rect:?}");
}

#[test]
fn a_scroll_container_spans_its_parent_and_keeps_its_child_in_flow() {
    let scroll = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, padding: EdgeSpace::default(), sizing: Sizing::Fill });
    let fixture = fixed_children(scroll, 1, 200.0, 100.0);
    assert!(close(fixture.rect(0).width, 200.0), "a Fill-sized scroll viewport spans its parent");
    assert!(close(fixture.rect(1).y, 0.0));
}

//#endregion 🔲️GridScrollOverlayAbsolute

//#region 🧱️LegacyDialect

/// 🧱️ The in-crate declarative chrome (`shell`, the wgpu `Interpreter`/`Shell` elements) carries no
/// `grow` vocabulary at all, so its stacks keep the pre-parity rule that every child takes an equal
/// share of the leftover main axis. This is the ONLY place that rule survives.
#[test]
fn a_legacy_declarative_stack_still_grows_every_child_equally() {
    let mut fixture = Fixture::new();
    let parent = fixture.push(LayoutNodeKind::Stack { horizontal: true, gap: 0.0, padding: 0.0 }, None, None);
    for _ in 0..3 {
        fixture.push(LayoutNodeKind::Leaf, Some(parent), None);
    }
    fixture.solve(300.0, 100.0);
    for index in 1..=3 {
        assert!(close(fixture.rect(index).width, 100.0), "child {index} takes an equal third, got {}", fixture.rect(index).width);
    }
}

#[test]
fn a_field_reserves_its_label_band_and_its_child_fills_the_remainder() {
    let theme = crate::wgpu::theme::Theme::default();
    let top = theme.font_size_small + theme.gap_standard;
    let mut fixture = Fixture::new();
    let parent = fixture.push(LayoutNodeKind::Field { top }, None, None);
    let child = fixture.push(LayoutNodeKind::Leaf, Some(parent), None);
    fixture.solve(200.0, 100.0);
    let rect = fixture.rect(child);
    assert!(close(rect.y, top), "child starts below the label band, got {}", rect.y);
    assert!(close(rect.height, 100.0 - top), "child fills the label-adjusted remainder, got {}", rect.height);
    assert!(close(rect.width, 200.0));
}

#[test]
fn a_section_stacks_children_below_its_header_at_their_own_height() {
    let theme = crate::wgpu::theme::Theme::default();
    let mut fixture = Fixture::new();
    let parent = fixture.push(LayoutNodeKind::Section { gap: theme.gap_standard }, None, None);
    let first = fixture.push(LayoutNodeKind::Control { height: 12.0, label_padding: None }, Some(parent), None);
    let second = fixture.push(LayoutNodeKind::Control { height: 12.0, label_padding: None }, Some(parent), None);
    fixture.solve(200.0, 200.0);
    let first_rect = fixture.rect(first);
    let second_rect = fixture.rect(second);
    assert!(close(first_rect.y, SECTION_HEADER_HEIGHT), "got {}", first_rect.y);
    assert!(close(first_rect.height, 12.0), "a section's children keep their intrinsic height, never grow");
    assert!(close(second_rect.y, first_rect.y + first_rect.height + theme.gap_standard), "second sits one gap below, got {}", second_rect.y);
}

#[test]
fn fixed_tree_bands_keep_their_intrinsic_pitch_inside_a_short_scroll_viewport() {
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
    let row = metrics.row_height;
    let action_count = 55;
    let intrinsic = row * (action_count + 1) as f32;
    let scroll = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, padding: EdgeSpace::default(), sizing: Sizing::Fill });
    let mut fixture = Fixture::new();
    let viewport = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&scroll));
    let tree = fixture.push(LayoutNodeKind::Tree { height: intrinsic, reversed: false }, Some(viewport), None);
    let section = fixture.push(LayoutNodeKind::TreeSection { header: row, height: intrinsic, expanded: true, reversed: false }, Some(tree), None);
    let mut rows = Vec::new();
    for _ in 0..action_count {
        rows.push(fixture.push(LayoutNodeKind::TreeRow { row, height: row, expanded: false, reversed: false }, Some(section), None));
    }
    fixture.solve(300.0, 480.0);

    assert!(close(fixture.rect(viewport).height, 480.0));
    assert!(close(fixture.rect(tree).height, intrinsic), "the intrinsic Tree band was squeezed to {:?}", fixture.rect(tree));
    assert!(close(fixture.rect(section).height, intrinsic), "the intrinsic section band was squeezed to {:?}", fixture.rect(section));
    assert!(close(fixture.rect(rows[0]).height, row) && close(fixture.rect(rows[1]).height, row));
    assert!(close(fixture.rect(rows[1]).y - fixture.rect(rows[0]).y, row), "successive Actions rows keep React's 24px pitch");
}

//#endregion 🧱️LegacyDialect

//#region 🧹️CloseDiscipline

#[test]
fn the_flex_tree_releases_exactly_one_node_per_close_grant() {
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
    let mut flex = FlexTree::new();
    for index in 0..4 {
        assert!(flex.push(LayoutNodeKind::Leaf, if index == 0 { None } else { Some(0) }, None, &metrics, None));
    }
    for step in (1..=4).rev() {
        assert_eq!(flex.len(), step);
        assert!(!flex.release_one(), "a grant that released an owner is never terminal");
    }
    assert!(flex.is_empty());
    assert!(flex.release_one(), "an already-empty tree reports terminal without releasing anything");
}

#[test]
fn a_solve_against_a_node_that_was_never_admitted_is_refused_rather_than_guessed() {
    let mut flex = FlexTree::new();
    let mut measure = |_node: usize, _constraint: MeasureConstraint| (0.0_f32, 0.0_f32);
    assert!(!flex.measure_one(0, &[], &mut measure));
    assert!(!flex.arrange_one(0, &[], &mut measure));
    assert!(!flex.set_root_box(0, FlexRect::default()));
    assert_eq!(flex.rect(0), None);
}

//#endregion 🧹️CloseDiscipline

#[test]
fn a_leaf_spec_sizes_itself_from_its_own_sizing_vocabulary() {
    let fixture = fixed_children(stack(Axis::Vertical, Align::Start, Justify::Start, false), 1, 200.0, 200.0);
    let rect = fixture.rect(1);
    assert!(close(rect.width, 38.4) && close(rect.height, 19.2), "got {rect:?}");
}

/// ✍️ A measured text leaf is re-measured against the width its container actually offers it, so a
/// narrow column wraps exactly where CSS would — the `Definite` arm of [`MeasureConstraint`].
#[test]
fn a_text_leaf_is_remeasured_against_the_width_its_container_offers() {
    let root = stack(Axis::Vertical, Align::Stretch, Justify::Start, false);
    let mut fixture = Fixture::new();
    let parent = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&root));
    let text = fixture.push(LayoutNodeKind::Text, Some(parent), None);
    fixture.measured(text, (500.0, 14.0));
    fixture.solve(120.0, 400.0);
    assert!(fixture.rect(text).width <= 120.0, "a stretched text child is bounded by its container, got {}", fixture.rect(text).width);
}

/// 🎞️ LAW: an engine surface fills the authored container that mounts it. The record a plugin
/// publishes for a scene carries the terminal `LayoutSpec::Leaf` by default and an authored parent
/// grows no child by itself, so as a plain leaf the surface solved to height ZERO — the live
/// generation3d flow window laid `componentScene#procedural-main` out at `[0, 0, 974.8, 0.0]` and
/// painted no node of its graph at all.
#[test]
fn an_engine_surface_fills_the_authored_container_that_mounts_it() {
    let body = LayoutSpec::Stack(StackLayout { axis: Axis::Vertical, gap: SpaceToken::None, padding: EdgeSpace::default(), align: Align::Stretch, justify: Justify::Start, grow: true, wrap: false });
    let declared = LayoutSpec::Leaf(LeafLayout::default());
    let mut fixture = Fixture::new();
    let column = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&body));
    let scene = fixture.push(LayoutNodeKind::EngineSurface, Some(column), Some(&declared));
    fixture.solve(974.848, 813.6);
    let rect = fixture.rect(scene);
    assert!(close(rect.width, 974.848), "the surface spans its container, got {rect:?}");
    assert!(close(rect.height, 813.6), "the surface fills its container, got {rect:?}");
}

/// 📐️ …and a surface whose record DOES size itself keeps that size: filling is the fallback for a
/// scene nothing else measures, never an override of an authored box.
#[test]
fn an_engine_surface_that_declares_its_own_height_keeps_it() {
    let body = LayoutSpec::Stack(StackLayout { axis: Axis::Vertical, gap: SpaceToken::None, padding: EdgeSpace::default(), align: Align::Stretch, justify: Justify::Start, grow: true, wrap: false });
    let declared = LayoutSpec::Leaf(LeafLayout { width: Sizing::Fill, height: Sizing::Fixed(SpaceToken::Xxl) });
    let mut fixture = Fixture::new();
    let column = fixture.push(LayoutNodeKind::Stack { horizontal: false, gap: 0.0, padding: 0.0 }, None, Some(&body));
    let scene = fixture.push(LayoutNodeKind::EngineSurface, Some(column), Some(&declared));
    fixture.solve(600.0, 800.0);
    assert!(close(fixture.rect(scene).height, SpaceToken::Xxl.px()), "got {:?}", fixture.rect(scene));
}

/// 🗺️ …and a document whose ROOT is the engine surface — a map playground publishes exactly one node
/// — solves against the viewport box without a second pass over itself.
#[test]
fn an_engine_surface_root_solves_against_the_viewport_it_is_given() {
    let declared = LayoutSpec::Leaf(LeafLayout::default());
    let mut fixture = Fixture::new();
    let scene = fixture.push(LayoutNodeKind::EngineSurface, None, Some(&declared));
    fixture.solve(1433.6, 813.6);
    let rect = fixture.rect(scene);
    assert!(close(rect.width, 1433.6) && close(rect.height, 813.6), "got {rect:?}");
}
