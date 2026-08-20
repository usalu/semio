//! @emoji 📏️ The taffy adapter: `LayoutCx` owns the per-frame `taffy::TaffyTree`, maps
//! [`ui_contract::LayoutSpec`] onto taffy `Style`, and resolves intrinsic measurement + pixel
//! snapping. **Taffy types never appear in a public signature here or anywhere else in this crate** —
//! [`LayoutNodeId`] wraps `taffy::NodeId` with a private field for exactly that reason, and
//! [`AvailableSpace`]/[`Measurement`] are this crate's own vocabulary, never taffy's. This is what
//! lets a different layout engine replace taffy later without touching `element.rs`/`frame.rs`.
//!
//! **The taffy tree is rebuilt from scratch every frame**, not retained and incrementally diffed the
//! way the wgpu-old target's `flex::LayoutEngine` did against its persistent `UiTree`. That
//! incremental-sync machinery (`prune_removed`/`sync`'s dirty-flag-gated reuse) does not port here:
//! per `element.rs`'s own docstring, the *element* tree itself is rebuilt from scratch every frame
//! (only per-`ElementId` retained state survives), so there is no persistent taffy tree left to
//! incrementally sync against — a fresh [`taffy::TaffyTree`] per [`LayoutCx`] is not a missed
//! optimization, it is the only tree that could exist under this frame model. Style-token mapping
//! (`gap_for_token`/`padding_for_token`'s role) is ported as [`space_token_px`], generalized from two
//! hand-picked tokens to the full [`ui_contract::SpaceToken`] ramp that packet `contract-layout`
//! defined after this file's wgpu-old ancestor was written.

use ui_contract::{AbsoluteLayout, Align, Axis, EdgeSpace, GridLayout, GridTrack, Justify, LayoutSpec, ScrollAxes, ScrollLayout, Sizing, SpaceToken, StackLayout};

//#region 🔖️Layout

//#region 🔑️LayoutNodeId

/// 🔑️ An opaque handle into one [`LayoutCx`]'s taffy tree. The `taffy::NodeId` field is private —
/// only this file may construct or unwrap one — so no public signature anywhere in this crate ever
/// names a taffy type, per this file's own docstring.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LayoutNodeId(taffy::NodeId);

//#endregion 🔑️LayoutNodeId

//#region 📏️Measurement

/// 📏️ This crate's own available-space vocabulary — never `taffy::AvailableSpace` (see this file's
/// docstring). Passed into a leaf's [`MeasureFn`] at layout time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AvailableSpace {
    Definite(f32),
    MinContent,
    MaxContent,
}

impl AvailableSpace {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn from_taffy(space: taffy::AvailableSpace) -> Self {
        match space {
            taffy::AvailableSpace::Definite(value) => Self::Definite(value),
            taffy::AvailableSpace::MinContent => Self::MinContent,
            taffy::AvailableSpace::MaxContent => Self::MaxContent,
        }
    }
}

/// 📏️ The outcome of measuring one leaf's intrinsic content against the available space taffy offers
/// it. `Pending`/`Failed` are how master.md's decision #1 ("a dependency that is not ready is reported
/// as a pending measurement with a placeholder size — never awaited") is expressed in this crate: an
/// unshaped `TextSystem` run or an undecoded image resource is a `Pending` result carrying whatever
/// placeholder size the caller judges reasonable (e.g. the last known size, or a fixed minimum), never
/// a blocked call. Whoever completes that dependency (packet `render-text`'s shaping, the resource
/// registry's decode) is responsible for invalidating the window once it lands — this type only
/// records that the frame just built proceeded on a placeholder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Measurement {
    Ready { width: f32, height: f32 },
    Pending { placeholder_width: f32, placeholder_height: f32 },
    Failed,
}

impl Measurement {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn size(self) -> taffy::geometry::Size<f32> {
        match self {
            Self::Ready { width, height } => taffy::geometry::Size { width, height },
            Self::Pending { placeholder_width, placeholder_height } => taffy::geometry::Size { width: placeholder_width, height: placeholder_height },
            Self::Failed => taffy::geometry::Size::ZERO,
        }
    }
}

/// 📏️ A leaf's intrinsic-measurement hook, supplied to [`LayoutCx::leaf`]. Boxed `dyn FnMut` — not a
/// vtable slot, since a measure closure is not a first-party trait object under U3's own table (only
/// `Element` is named there); `dyn Fn*` stays permitted per R1.
pub type MeasureFn = Box<dyn FnMut(AvailableSpace, AvailableSpace) -> Measurement>;

struct LeafContext {
    measure: Option<MeasureFn>,
}

//#endregion 📏️Measurement

//#region 🎨️StyleMapping

/// 📐️ Provisional px scale for [`SpaceToken`], mirroring the wgpu-old target's `gap_for_token`/
/// `padding_for_token` hand-picked values (`tight`→4, `loose`→12, `none`→0) but covering the full
/// `None..Xxl` ramp `contract-layout` actually shipped. `contract-layout`'s own docstring already
/// flags that tokens.json has no real spacing ramp yet — this is the same open item, not a new one;
/// see this packet's report for the registrar-request to replace this table once one lands.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn space_token_px(token: SpaceToken) -> f32 {
    match token {
        SpaceToken::None => 0.0,
        SpaceToken::Xs => 4.0,
        SpaceToken::Sm => 8.0,
        SpaceToken::Md => 12.0,
        SpaceToken::Lg => 16.0,
        SpaceToken::Xl => 24.0,
        SpaceToken::Xxl => 32.0,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn sizing_to_dimension(sizing: Sizing) -> taffy::style::Dimension {
    match sizing {
        Sizing::Hug => taffy::style::Dimension::auto(),
        Sizing::Fill => taffy::style::Dimension::percent(1.0),
        Sizing::Fixed(token) => taffy::style::Dimension::length(space_token_px(token)),
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn edge_space_to_rect(edge: EdgeSpace) -> taffy::geometry::Rect<taffy::style::LengthPercentage> {
    use taffy::style::LengthPercentage;
    let length = |token: SpaceToken| LengthPercentage::length(space_token_px(token));
    match edge {
        EdgeSpace::All(token) => taffy::geometry::Rect { left: length(token), right: length(token), top: length(token), bottom: length(token) },
        EdgeSpace::Symmetric { vertical, horizontal } => taffy::geometry::Rect { left: length(horizontal), right: length(horizontal), top: length(vertical), bottom: length(vertical) },
        EdgeSpace::Each { top, right, bottom, left } => taffy::geometry::Rect { left: length(left), right: length(right), top: length(top), bottom: length(bottom) },
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn axis_to_flex_direction(axis: Axis) -> taffy::style::FlexDirection {
    match axis {
        Axis::Horizontal => taffy::style::FlexDirection::Row,
        Axis::Vertical => taffy::style::FlexDirection::Column,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn align_to_taffy(align: Align) -> taffy::style::AlignItems {
    match align {
        Align::Start => taffy::style::AlignItems::Start,
        Align::Center => taffy::style::AlignItems::Center,
        Align::End => taffy::style::AlignItems::End,
        Align::Stretch => taffy::style::AlignItems::Stretch,
        Align::Baseline => taffy::style::AlignItems::Baseline,
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn justify_to_taffy(justify: Justify) -> taffy::style::JustifyContent {
    match justify {
        Justify::Start => taffy::style::JustifyContent::Start,
        Justify::Center => taffy::style::JustifyContent::Center,
        Justify::End => taffy::style::JustifyContent::End,
        Justify::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
        Justify::SpaceAround => taffy::style::JustifyContent::SpaceAround,
        Justify::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
    }
}

/// 🧮️ Returns [`taffy::style::GridTemplateComponent`], **not** `TrackSizingFunction` — `Style`'s
/// `grid_template_columns`/`grid_template_rows` fields are `Vec<GridTemplateComponent<String>>` in
/// taffy 0.9 (a `Single(TrackSizingFunction) | Repeat(..)` union, confirmed at
/// `taffy-0.9.2/src/style/grid.rs:1226`), not `Vec<TrackSizingFunction>` directly. The generic
/// `style_helpers` functions below (`auto`/`fr`/`length`/`min_content`/`max_content`) resolve to
/// whichever marker-trait-bound return type is asked for — `GridTemplateComponent<S>` implements all
/// five marker traits itself (`grid.rs:1250-1284`), so no manual `TrackSizingFunction → GridTemplateComponent`
/// conversion step is needed; this function just asks for the right `T`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn grid_track_to_taffy(track: &GridTrack) -> taffy::style::GridTemplateComponent<String> {
    use taffy::style_helpers::{auto, fr, length, max_content, min_content};
    match *track {
        GridTrack::Auto => auto(),
        GridTrack::Fraction(count) => fr(f32::from(count)),
        GridTrack::Fixed(token) => length(space_token_px(token)),
        GridTrack::MinContent => min_content(),
        GridTrack::MaxContent => max_content(),
    }
}

/// 📦️ Maps the contract's renderer-neutral [`LayoutSpec`] onto a taffy `Style`. One arm per
/// [`LayoutSpec`] variant; `Overlay`'s per-child anchor resolution is intentionally **not** attempted
/// here — see this packet's report for why that is a deviation, not an oversight.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn style_from_spec(spec: &LayoutSpec) -> taffy::Style {
    match spec {
        LayoutSpec::Leaf(leaf) => taffy::Style { size: taffy::geometry::Size { width: sizing_to_dimension(leaf.width), height: sizing_to_dimension(leaf.height) }, ..Default::default() },
        LayoutSpec::Stack(stack) => style_from_stack(stack),
        LayoutSpec::Grid(grid) => style_from_grid(grid),
        LayoutSpec::Overlay(overlay) => taffy::Style {
            display: taffy::style::Display::Flex,
            position: taffy::style::Position::Relative,
            padding: edge_space_to_rect(overlay.inset),
            ..Default::default()
        },
        LayoutSpec::Scroll(scroll) => style_from_scroll(scroll),
        LayoutSpec::Absolute(absolute) => style_from_absolute(absolute),
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn style_from_stack(stack: &StackLayout) -> taffy::Style {
    let gap = taffy::style::LengthPercentage::length(space_token_px(stack.gap));
    taffy::Style {
        display: taffy::style::Display::Flex,
        flex_direction: axis_to_flex_direction(stack.axis),
        flex_wrap: if stack.wrap { taffy::style::FlexWrap::Wrap } else { taffy::style::FlexWrap::NoWrap },
        flex_grow: if stack.grow { 1.0 } else { 0.0 },
        gap: taffy::geometry::Size { width: gap, height: gap },
        padding: edge_space_to_rect(stack.padding),
        align_items: Some(align_to_taffy(stack.align)),
        justify_content: Some(justify_to_taffy(stack.justify)),
        ..Default::default()
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn style_from_grid(grid: &GridLayout) -> taffy::Style {
    taffy::Style {
        display: taffy::style::Display::Grid,
        grid_template_columns: grid.columns.iter().map(grid_track_to_taffy).collect(),
        grid_template_rows: grid.rows.iter().map(grid_track_to_taffy).collect(),
        gap: taffy::geometry::Size {
            width: taffy::style::LengthPercentage::length(space_token_px(grid.column_gap)),
            height: taffy::style::LengthPercentage::length(space_token_px(grid.row_gap)),
        },
        padding: edge_space_to_rect(grid.padding),
        align_items: Some(align_to_taffy(grid.align)),
        justify_content: Some(justify_to_taffy(grid.justify)),
        ..Default::default()
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn style_from_scroll(scroll: &ScrollLayout) -> taffy::Style {
    let (x, y) = match scroll.axes {
        ScrollAxes::None => (taffy::style::Overflow::Visible, taffy::style::Overflow::Visible),
        ScrollAxes::Horizontal => (taffy::style::Overflow::Scroll, taffy::style::Overflow::Hidden),
        ScrollAxes::Vertical => (taffy::style::Overflow::Hidden, taffy::style::Overflow::Scroll),
        ScrollAxes::Both => (taffy::style::Overflow::Scroll, taffy::style::Overflow::Scroll),
    };
    let dimension = |sizing: Sizing| sizing_to_dimension(sizing);
    taffy::Style {
        display: taffy::style::Display::Flex,
        overflow: taffy::geometry::Point { x, y },
        padding: edge_space_to_rect(scroll.padding),
        size: taffy::geometry::Size { width: dimension(scroll.sizing), height: dimension(scroll.sizing) },
        ..Default::default()
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn style_from_absolute(absolute: &AbsoluteLayout) -> taffy::Style {
    taffy::Style {
        position: taffy::style::Position::Absolute,
        size: taffy::geometry::Size { width: sizing_to_dimension(absolute.sizing_width), height: sizing_to_dimension(absolute.sizing_height) },
        ..Default::default()
    }
}

//#endregion 🎨️StyleMapping

//#region 🖇️LayoutCx

/// 🎯️ Rounds a resolved logical-pixel coordinate to the nearest whole logical pixel. Distinct from
/// [`crate::tessellate::snap_to_device_pixels`]'s later, dpr-aware **physical**-pixel snap in
/// `Scene::finish` — this one exists because taffy's own flexbox remainder distribution can hand back
/// coordinates like `33.333336` from perfectly clean integer inputs, and rounding that noise out here,
/// once, ahead of every consumer, is cheaper and more predictable than every consumer re-deriving the
/// same rounding independently.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn snap_logical(value: f32) -> f32 {
    value.round()
}

/// 🖇️ Owns one frame's taffy tree. Built fresh per frame (see this file's docstring), driven by
/// [`Element::request_layout`](crate::element::Element::request_layout) calls that build the tree
/// top-down, then a single [`Self::compute`], then read back via [`Self::resolved`].
pub struct LayoutCx<'a> {
    pub shared: crate::element::SharedFrameCx<'a>,
    taffy: taffy::TaffyTree<LeafContext>,
    had_pending: bool,
}

impl<'a> LayoutCx<'a> {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(shared: crate::element::SharedFrameCx<'a>) -> Self {
        Self { shared, taffy: taffy::TaffyTree::new(), had_pending: false }
    }

    /// 🍃️ Inserts a childless node. `measure` is `None` for a node whose size comes entirely from
    /// `spec` (most containers' synthetic leaves); `Some` for a node with real intrinsic content
    /// (text, an image) that needs to answer taffy's measurement pass itself.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn leaf(&mut self, spec: &LayoutSpec, measure: Option<MeasureFn>) -> LayoutNodeId {
        let style = style_from_spec(spec);
        let node = self.taffy.new_leaf_with_context(style, LeafContext { measure }).expect("taffy leaf insert is infallible for a freshly built tree");
        LayoutNodeId(node)
    }

    /// 📦️ Inserts a node with already-built `children`, in order.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn container(&mut self, spec: &LayoutSpec, children: &[LayoutNodeId]) -> LayoutNodeId {
        let style = style_from_spec(spec);
        let taffy_children: Vec<taffy::NodeId> = children.iter().map(|child| child.0).collect();
        let node = self.taffy.new_with_children(style, &taffy_children).expect("taffy container insert is infallible for a freshly built tree");
        LayoutNodeId(node)
    }

    /// 🔁️ Replaces `id`'s children wholesale — for a container whose child list is only known after
    /// some of them were themselves inserted as containers (recursive construction).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn set_children(&mut self, id: LayoutNodeId, children: &[LayoutNodeId]) {
        let taffy_children: Vec<taffy::NodeId> = children.iter().map(|child| child.0).collect();
        let _ = self.taffy.set_children(id.0, &taffy_children);
    }

    /// 🏁️ Runs one taffy layout pass rooted at `root` against a definite `(available_width,
    /// available_height)` viewport. Every `Pending` measurement encountered along the way is recorded
    /// — see [`Self::had_pending_measurement`].
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn compute(&mut self, root: LayoutNodeId, available_width: f32, available_height: f32) {
        let available = taffy::geometry::Size { width: taffy::AvailableSpace::Definite(available_width), height: taffy::AvailableSpace::Definite(available_height) };
        let mut had_pending = false;
        let _ = self.taffy.compute_layout_with_measure(root.0, available, |known_dimensions, available_space, _node_id, node_context, _style| {
            if let (Some(width), Some(height)) = (known_dimensions.width, known_dimensions.height) {
                return taffy::geometry::Size { width, height };
            }
            let Some(context) = node_context else { return taffy::geometry::Size::ZERO };
            let Some(measure) = context.measure.as_mut() else { return taffy::geometry::Size::ZERO };
            let width_space = AvailableSpace::from_taffy(known_dimensions.width.map_or(available_space.width, taffy::AvailableSpace::Definite));
            let height_space = AvailableSpace::from_taffy(known_dimensions.height.map_or(available_space.height, taffy::AvailableSpace::Definite));
            let measurement = measure(width_space, height_space);
            had_pending |= matches!(measurement, Measurement::Pending { .. });
            measurement.size()
        });
        self.had_pending = had_pending;
    }

    /// 📐️ `id`'s resolved rect, in its **parent-relative** coordinate space (taffy's own
    /// `Layout::location` semantics — a caller accumulates ancestor offsets while walking down, the
    /// same way `paint_node`'s `origin_x + origin_y` accumulation did in the wgpu-old target), snapped
    /// to the nearest whole logical pixel.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn resolved(&self, id: LayoutNodeId) -> crate::scene::LayoutRect {
        let layout = self.taffy.layout(id.0).expect("resolved called with a LayoutNodeId that has not been computed");
        crate::scene::LayoutRect::new(snap_logical(layout.location.x), snap_logical(layout.location.y), snap_logical(layout.size.width), snap_logical(layout.size.height))
    }

    /// 🕳️ Whether [`Self::compute`] answered at least one leaf's measurement with `Measurement::Pending`
    /// — the frame proceeded on a placeholder size and the caller may want to record that fact (e.g.
    /// for diagnostics) even though, per master.md's decision #1, re-invalidating the window is the
    /// landing dependency's job, not this method's.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn had_pending_measurement(&self) -> bool {
        self.had_pending
    }
}

//#endregion 🖇️LayoutCx

//#endregion 🔖️Layout

#[cfg(test)]
mod tests {
    use super::*;
    use ui_contract::LeafLayout;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn shared<'a>(arena: &'a mut crate::element::FrameArena, resources: &'a mut crate::resource::ResourceRegistry, retained: &'a mut crate::element::RetainedStore) -> crate::element::SharedFrameCx<'a> {
        crate::element::SharedFrameCx { arena, resources, retained, time_seconds: 0.0 }
    }

    #[test]
    fn a_fixed_size_leaf_resolves_to_exactly_that_size() {
        let mut arena = crate::element::FrameArena::default();
        let mut resources = crate::resource::ResourceRegistry::default();
        let mut retained = crate::element::RetainedStore::default();
        let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

        let spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Fixed(SpaceToken::Lg), height: Sizing::Fixed(SpaceToken::Md) });
        let leaf = cx.leaf(&spec, None);
        cx.compute(leaf, 200.0, 200.0);

        let rect = cx.resolved(leaf);
        assert_eq!(rect.w, space_token_px(SpaceToken::Lg));
        assert_eq!(rect.h, space_token_px(SpaceToken::Md));
    }

    #[test]
    fn a_vertical_stack_places_its_second_child_below_the_first() {
        let mut arena = crate::element::FrameArena::default();
        let mut resources = crate::resource::ResourceRegistry::default();
        let mut retained = crate::element::RetainedStore::default();
        let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

        let leaf_spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Fixed(SpaceToken::Lg), height: Sizing::Fixed(SpaceToken::Lg) });
        let first = cx.leaf(&leaf_spec, None);
        let second = cx.leaf(&leaf_spec, None);
        let stack_spec = LayoutSpec::Stack(StackLayout { axis: Axis::Vertical, gap: SpaceToken::None, padding: EdgeSpace::default(), align: Align::Start, justify: Justify::Start, grow: false, wrap: false });
        let root = cx.container(&stack_spec, &[first, second]);
        cx.compute(root, 400.0, 400.0);

        let first_rect = cx.resolved(first);
        let second_rect = cx.resolved(second);
        assert_eq!(first_rect.y, 0.0);
        assert!(second_rect.y >= first_rect.y + first_rect.h);
    }

    #[test]
    fn a_pending_measurement_yields_the_placeholder_size_and_is_recorded() {
        let mut arena = crate::element::FrameArena::default();
        let mut resources = crate::resource::ResourceRegistry::default();
        let mut retained = crate::element::RetainedStore::default();
        let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

        let spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Hug, height: Sizing::Hug });
        let measure: MeasureFn = Box::new(|_w, _h| Measurement::Pending { placeholder_width: 42.0, placeholder_height: 17.0 });
        let leaf = cx.leaf(&spec, Some(measure));
        assert!(!cx.had_pending_measurement());
        cx.compute(leaf, 200.0, 200.0);

        let rect = cx.resolved(leaf);
        assert_eq!(rect.w, 42.0);
        assert_eq!(rect.h, 17.0);
        assert!(cx.had_pending_measurement());
    }

    #[test]
    fn a_ready_measurement_does_not_mark_pending() {
        let mut arena = crate::element::FrameArena::default();
        let mut resources = crate::resource::ResourceRegistry::default();
        let mut retained = crate::element::RetainedStore::default();
        let mut cx = LayoutCx::new(shared(&mut arena, &mut resources, &mut retained));

        let spec = LayoutSpec::Leaf(LeafLayout { width: Sizing::Hug, height: Sizing::Hug });
        let measure: MeasureFn = Box::new(|_w, _h| Measurement::Ready { width: 10.0, height: 10.0 });
        let leaf = cx.leaf(&spec, Some(measure));
        cx.compute(leaf, 200.0, 200.0);

        assert!(!cx.had_pending_measurement());
    }

    #[test]
    fn snap_logical_rounds_taffy_remainder_noise() {
        assert_eq!(snap_logical(33.333336), 33.0);
        assert_eq!(snap_logical(33.6), 34.0);
    }
}
