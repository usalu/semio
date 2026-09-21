// #region flex
//! 📏️ The production flex/grid layout engine for the retained tree (`tree`/`reconcile`), driven one
//! bounded step at a time by `mounted_layout::MountedLayoutJob`. Taffy's own types (`taffy::TaffyTree`,
//! `taffy::NodeId`, `taffy::Style`, …) are fully wrapped by [`FlexTree`] and never appear in any item
//! visible outside this file — [`FlexRect`]/[`MeasureConstraint`] are this crate's own vocabulary.
//!
//! **Two layout dialects meet here, and the difference is deliberate.** A node whose `LayoutSpec`
//! was AUTHORED (a `UiNodeRecord` carrying `record.layout`, the same value React's `layoutSpecStyle`
//! reads — `os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`) is laid out by
//! plain CSS rules: direction/align/justify/wrap/gap/padding/grow/basis, real grid tracks, in-flow
//! overlay positioning contexts, and out-of-flow absolute positioning. A node that came off the LEGACY declarative `UiNode`
//! path (in-crate chrome: `shell`, the wgpu `Interpreter`/`Shell` elements — content with no React
//! counterpart at all) keeps the pre-parity rule that every child of a `Stack`/`Field` grows into the
//! leftover main-axis space, because that chrome carries no `grow` information to replace it with.
//! [`FlexTree::push`] picks the dialect from whether the arena node carries `Node::layout_spec`.
//!
//! Composite wgpu widgets (`Field`/`Section`/`Tree`/`TreeSection`/`TreeRow`/`Control`) keep their own
//! hand-rolled geometry, expressed as flex styles here rather than as a second arrange pass: the
//! painter and `events::hit_test` derive their rows from the same `layout`-region constants, so an
//! authored spec on such a node must NOT move them (see [`style_for`]'s own arms).

use ui_contract::{Align, Axis, EdgePx, GridTrack, Justify, LayoutSpec, Sizing};

use crate::wgpu::layout::TreeRowMetrics;

//#region 🎚️LayoutNodeKind

/// 🧩️ What one retained node contributes to layout, classified once during admission (which is the
/// only phase that holds the `UiTree`) and kept for the whole job — the worker phases run on a pool
/// thread with no tree access.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum LayoutNodeKind {
    Text,
    /// 📚️ A generic container. The three metrics are the LEGACY declarative dialect's own
    /// (`UiStackNode`'s `direction`/`gap`/`padding` strings, already resolved through the shared
    /// `SpaceToken` ramp by `layout::gap_for_token`/`padding_for_token`); an authored `LayoutSpec`
    /// supersedes all three.
    Stack {
        horizontal: bool,
        gap: f32,
        padding: f32,
    },
    Field {
        top: f32,
    },
    Section {
        gap: f32,
    },
    /// 🌳️ A `Tree`, measured from its own spec through `layout`'s shared row geometry rather than
    /// from arena children — a tree's rows carry no children of their own, so aggregating them
    /// measured a whole tree as the sum of its rows' padding.
    Tree {
        height: f32,
        reversed: bool,
    },
    TreeSection {
        header: f32,
        height: f32,
        expanded: bool,
        reversed: bool,
    },
    /// 🌳️ `row` is this row's own band (the y a nested row starts at) and `expanded` says whether
    /// its nested rows are REACHED at all — the arena mounts a collapsed branch's children, and the
    /// painter draws none of them, so an unreached row must resolve to nothing rather than overlap
    /// the row that visually follows it.
    TreeRow {
        row: f32,
        height: f32,
        expanded: bool,
        reversed: bool,
    },
    /// 🎛️ A value-carrying control: one control row tall on its own, so a container that sizes its
    /// children by intrinsic height (a `Section`) never collapses it to zero.
    ///
    /// 🏷️ `label_padding` is `Some(px)` for a control whose own LABEL is its content — a `Button`,
    /// whose text is a field of the node and not an arena child. Such a control is measured through
    /// the same glyph lane a `Text` is, with that padding on each side, which is where `paint_button`
    /// starts its label.
    ///
    /// 🩸️ Without it a hug-width button solved to width ZERO: nothing measured it, so every retained
    /// button published a zero-area rect, `retained_hit_registration` refused it, and the surface's
    /// whole pointer registry came back empty while the panel painted and kept keyboard focus
    /// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, the ToolRun panel's `[STATS] … targets []`).
    Control {
        height: f32,
        label_padding: Option<f32>,
    },
    /// 🔽️ A synthesized Select option whose rect is owned by popup placement, outside document
    /// flow. The retained interaction sync writes this parent-relative physical rect before a
    /// refreshed document can schedule layout; carrying it through the solver prevents the option
    /// from reflowing as an ordinary Button child while it owns pointer capture.
    OverlayRow {
        rect: FlexRect,
    },
    /// 🧩️ HOST-PROVIDED content: a leaf whose pixels this engine does not author at all. It reserves
    /// the box its host declared and fills its parent's width, so an arbitrary host-painted surface
    /// (React's `Tree` `emptyState` escape hatch — an agent chat transcript, a marketplace list) can
    /// live inside an otherwise declarative document without the host having to express it as
    /// `UiNode`s. Unlike [`LayoutNodeKind::Leaf`] it is never measured from children (it has none) and
    /// never collapses to zero; unlike [`LayoutNodeKind::Control`] its height is the HOST's number, not
    /// the theme's control row.
    HostContent {
        height: f32,
    },
    /// 🎞️ An ENGINE surface (`UiNode::ComponentScene`): a leaf whose pixels an engine paints into the
    /// box this solver hands it. It has no content to measure and no intrinsic size, so as a plain
    /// [`LayoutNodeKind::Leaf`] it solved to height ZERO inside every authored container — an
    /// authored parent grows no child by itself, and a document that does not spell `grow` on its
    /// scene child is ordinary (React's surface host fills its parent whatever the record declares).
    /// Measured on the live generation3d flow window as `componentScene#procedural-main` at
    /// `[0, 0, 974.8, 0.0]`, which collapsed the whole node graph onto a one-pixel band.
    EngineSurface,
    Leaf,
}

//#endregion 🎚️LayoutNodeKind

//#region 📐️Geometry

/// 📐️ One node's solved box, parent-relative — taffy's own `Layout::location`/`Layout::size`
/// semantics, which is exactly what `tree::LayoutBucket` stores and `paint` accumulates down.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct FlexRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 📏️ The available-space question taffy asks a measured leaf, in this crate's own vocabulary.
/// `MinContent`/`MaxContent` are CSS's intrinsic sizes: the widest single word, and the whole run on
/// one line — the two numbers a browser resolves text against inside a flex item.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum MeasureConstraint {
    Definite(f32),
    MinContent,
    MaxContent,
}

//#endregion 📐️Geometry

//#region 🌊️FlowStyle

/// 📏️ One axis's sizing rule in this engine's own vocabulary — the closed reading of
/// [`ui_contract::Sizing`] plus the fixed pixel bands the composite wgpu widgets carry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Dim {
    Auto,
    Fill,
    Length(f32),
}

impl Dim {
    fn resolve(self, available: f32) -> Option<f32> {
        match self {
            Self::Auto => None,
            Self::Fill => Some(available),
            Self::Length(value) => Some(value),
        }
    }
}

fn dim_of(sizing: Sizing) -> Dim {
    match sizing {
        Sizing::Hug => Dim::Auto,
        Sizing::Fill => Dim::Fill,
        Sizing::Fixed(token) => Dim::Length(token.px()),
    }
}

/// 🌊️ How a container arranges its own children. `Flow` is plain single-line flex, which this engine
/// resolves arithmetically (see [`FlexTree::measure_one`]); `Wrapped` and `Grid` are handed to the
/// taffy solver instead — both are rare and small in practice (a toolbar, a property grid), so the
/// bound "one container per grant" still holds for them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FlowKind {
    Flow,
    Wrapped,
    Grid,
}

/// 🌊️ Everything the arithmetic path needs about one node, in this crate's own vocabulary. It is the
/// PRIMARY style: [`taffy_style`] derives the taffy `Style` from it (plus the authored grid tracks),
/// so the two readings cannot drift — there is one constructor, [`flow_for`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FlowStyle {
    pub kind: FlowKind,
    pub row: bool,
    pub gap_main: f32,
    pub gap_cross: f32,
    pub padding: EdgePx,
    pub align: Align,
    pub justify: Justify,
    pub grow: f32,
    pub shrink: f32,
    pub width: Dim,
    pub height: Dim,
    pub min_width: f32,
    pub min_height: f32,
    pub absolute: bool,
    pub inset: [Option<f32>; 4],
    pub clips: bool,
    pub text: bool,
    /// 🌱️ Legacy dialect only: every child of this container grows into the leftover main axis (see
    /// this file's header). An authored `LayoutSpec` carries each child's own `grow` instead.
    pub grows_children: bool,
    pub reverse: bool,
}

impl Default for FlowStyle {
    fn default() -> Self {
        Self {
            kind: FlowKind::Flow,
            row: false,
            gap_main: 0.0,
            gap_cross: 0.0,
            padding: EdgePx::default(),
            align: Align::Stretch,
            justify: Justify::Start,
            grow: 0.0,
            shrink: 1.0,
            width: Dim::Auto,
            height: Dim::Auto,
            min_width: 0.0,
            min_height: 0.0,
            absolute: false,
            inset: [None; 4],
            clips: false,
            text: false,
            grows_children: false,
            reverse: false,
        }
    }
}

impl FlowStyle {
    fn padding_main(&self) -> (f32, f32) {
        if self.row {
            (self.padding.left, self.padding.right)
        } else {
            (self.padding.top, self.padding.bottom)
        }
    }

    fn padding_cross(&self) -> (f32, f32) {
        if self.row {
            (self.padding.top, self.padding.bottom)
        } else {
            (self.padding.left, self.padding.right)
        }
    }
}

/// 🌊️ The authored `LayoutSpec` in flow vocabulary, field for field against React's
/// `layoutSpecStyle`. React leaves `leaf`/`overlay`/`scroll`/`absolute` as CSS BLOCK containers (it
/// sets no `display`), which is a column flow whose children span the inline size — the same box a
/// column flex with the default `Stretch` alignment produces for a vocabulary with no margins.
fn flow_from_spec(spec: &LayoutSpec) -> FlowStyle {
    match spec {
        LayoutSpec::Leaf(leaf) => FlowStyle { width: dim_of(leaf.width), height: dim_of(leaf.height), ..FlowStyle::default() },
        LayoutSpec::Stack(stack) => FlowStyle {
            kind: if stack.wrap { FlowKind::Wrapped } else { FlowKind::Flow },
            row: matches!(stack.axis, Axis::Horizontal),
            gap_main: stack.gap.px(),
            gap_cross: stack.gap.px(),
            padding: stack.padding.px(),
            align: stack.align,
            justify: stack.justify,
            grow: if stack.grow { 1.0 } else { 0.0 },
            ..FlowStyle::default()
        },
        LayoutSpec::Grid(grid) => FlowStyle { kind: FlowKind::Grid, gap_main: grid.row_gap.px(), gap_cross: grid.column_gap.px(), padding: grid.padding.px(), align: grid.align, justify: grid.justify, ..FlowStyle::default() },
        LayoutSpec::Overlay(overlay) => FlowStyle { padding: overlay.inset.px(), ..FlowStyle::default() },
        LayoutSpec::Scroll(scroll) => FlowStyle { padding: scroll.padding.px(), width: dim_of(scroll.sizing), clips: true, ..FlowStyle::default() },
        LayoutSpec::Absolute(absolute) => FlowStyle { absolute: true, width: dim_of(absolute.sizing_width), height: dim_of(absolute.sizing_height), ..FlowStyle::default() },
    }
}

/// 🌊️ One retained node's flow style. `authored` is the node's own `LayoutSpec` when the producer
/// published one (the React-parity dialect); the composite wgpu kinds ignore it on purpose — their
/// geometry is what `paint`/`events` already derive their rows from.
///
/// 🎛️ One control row tall AT LEAST, never exactly: a `Select`'s synthesized option rows
/// are real arena children, and the popup grows past the control band to hold them.
///
/// 🧩️ EXACTLY the host's band, clipped, filling the parent's cross axis: the host paints inside
/// the solved rect and this engine reserves it without measuring anything of its own.
fn flow_for(kind: LayoutNodeKind, parent_kind: Option<LayoutNodeKind>, authored: Option<&LayoutSpec>, metrics: &TreeRowMetrics) -> FlowStyle {
    if matches!(parent_kind, Some(LayoutNodeKind::TreeRow { .. })) && !matches!(kind, LayoutNodeKind::TreeRow { .. }) {
        let control_height = match kind {
            LayoutNodeKind::Control { height, .. } => height,
            _ => metrics.control_height,
        };
        let rect = crate::wgpu::layout::tree_row_control_rect_with_height(0.0, control_height, metrics);
        let inset = if metrics.inline.is_rtl() { [Some(rect.y), None, None, Some(metrics.gap)] } else { [Some(rect.y), Some(metrics.gap), None, None] };
        return FlowStyle { absolute: true, inset, width: Dim::Length(metrics.control_width), height: Dim::Length(control_height), ..FlowStyle::default() };
    }
    let band = |height: f32, header: f32, reverse: bool| FlowStyle {
        height: Dim::Length(height),
        shrink: 0.0,
        padding: if reverse { EdgePx { bottom: header, ..EdgePx::default() } } else { EdgePx { top: header, ..EdgePx::default() } },
        clips: true,
        reverse,
        ..FlowStyle::default()
    };
    match kind {
        LayoutNodeKind::Text => FlowStyle { text: true, ..authored.map_or_else(FlowStyle::default, flow_from_spec) },
        LayoutNodeKind::Leaf => authored.map_or_else(FlowStyle::default, flow_from_spec),
        LayoutNodeKind::Stack { horizontal, gap, padding } => match authored {
            Some(spec) => flow_from_spec(spec),
            None => FlowStyle { row: horizontal, gap_main: gap, gap_cross: gap, padding: EdgePx { top: padding, right: padding, bottom: padding, left: padding }, ..FlowStyle::default() },
        },
        LayoutNodeKind::Field { top } => FlowStyle { padding: EdgePx { top, ..EdgePx::default() }, ..FlowStyle::default() },
        LayoutNodeKind::Section { gap } => FlowStyle { gap_main: gap, padding: EdgePx { top: SECTION_HEADER_HEIGHT, ..EdgePx::default() }, ..FlowStyle::default() },
        LayoutNodeKind::Tree { height, reversed } => band(height, 0.0, reversed),
        LayoutNodeKind::TreeSection { header, height, reversed, .. } => band(height, header, reversed),
        LayoutNodeKind::TreeRow { row, height, reversed, .. } => band(height, row, reversed),
        LayoutNodeKind::Control { height, label_padding } => {
            let mut flow = authored.map_or_else(FlowStyle::default, flow_from_spec);
            flow.min_height = height;
            if let Some(padding) = label_padding {
                flow.text = true;
                flow.padding = EdgePx { top: flow.padding.top, right: flow.padding.right.max(padding), bottom: flow.padding.bottom, left: flow.padding.left.max(padding) };
            }
            flow
        }
        LayoutNodeKind::OverlayRow { rect } => FlowStyle {
            absolute: true,
            inset: [Some(rect.y), None, None, Some(rect.x)],
            width: Dim::Length(rect.width),
            height: Dim::Length(rect.height),
            shrink: 0.0,
            text: true,
            ..FlowStyle::default()
        },
        LayoutNodeKind::HostContent { height } => FlowStyle { height: Dim::Length(height), min_height: height, clips: true, ..authored.map_or_else(FlowStyle::default, flow_from_spec) },
        LayoutNodeKind::EngineSurface => {
            let mut flow = authored.map_or_else(FlowStyle::default, flow_from_spec);
            flow.clips = true;
            if matches!(flow.height, Dim::Auto) && flow.grow <= 0.0 {
                flow.grow = 1.0;
            }
            flow
        }
    }
}

//#endregion 🌊️FlowStyle

//#region 🎨️StyleMapping

fn align_items(align: Align) -> taffy::style::AlignItems {
    match align {
        Align::Start => taffy::style::AlignItems::Start,
        Align::Center => taffy::style::AlignItems::Center,
        Align::End => taffy::style::AlignItems::End,
        Align::Stretch => taffy::style::AlignItems::Stretch,
        Align::Baseline => taffy::style::AlignItems::Baseline,
    }
}

fn justify_content(justify: Justify) -> taffy::style::JustifyContent {
    match justify {
        Justify::Start => taffy::style::JustifyContent::Start,
        Justify::Center => taffy::style::JustifyContent::Center,
        Justify::End => taffy::style::JustifyContent::End,
        Justify::SpaceBetween => taffy::style::JustifyContent::SpaceBetween,
        Justify::SpaceAround => taffy::style::JustifyContent::SpaceAround,
        Justify::SpaceEvenly => taffy::style::JustifyContent::SpaceEvenly,
    }
}

/// 🔲️ One grid track, as taffy 0.9's `GridTemplateComponent` (a `Single | Repeat` union, not a bare
/// `TrackSizingFunction`) — the generic `style_helpers` constructors resolve to whichever of the two
/// the field asks for, so no manual conversion step is needed.
fn grid_track(track: GridTrack) -> taffy::style::GridTemplateComponent<String> {
    use taffy::style_helpers::{auto, fr, length, max_content, min_content};
    match track {
        GridTrack::Auto => auto(),
        GridTrack::Fraction(count) => fr(f32::from(count)),
        GridTrack::Fixed(token) => length(token.px()),
        GridTrack::MinContent => min_content(),
        GridTrack::MaxContent => max_content(),
    }
}

fn gap_size(column: f32, row: f32) -> taffy::geometry::Size<taffy::style::LengthPercentage> {
    taffy::geometry::Size { width: taffy::style::LengthPercentage::length(column), height: taffy::style::LengthPercentage::length(row) }
}

/// 🧬️ The taffy reading of a [`FlowStyle`], used only where the arithmetic path hands over: a
/// `Wrapped` or `Grid` container. Derived from the flow style rather than built beside it, so the two
/// readings of one node cannot drift; `tracks` is the authored grid track list, the one piece of the
/// vocabulary a [`FlowStyle`] has no field for.
fn taffy_style(flow: &FlowStyle, tracks: Option<&ui_contract::GridLayout>) -> taffy::Style {
    let grid = matches!(flow.kind, FlowKind::Grid);
    let inset = |value: Option<f32>| value.map_or_else(taffy::style::LengthPercentageAuto::auto, taffy::style::LengthPercentageAuto::length);
    let mut style = taffy::Style {
        display: if grid { taffy::style::Display::Grid } else { taffy::style::Display::Flex },
        flex_direction: if flow.row { taffy::style::FlexDirection::Row } else { taffy::style::FlexDirection::Column },
        flex_wrap: if matches!(flow.kind, FlowKind::Wrapped) { taffy::style::FlexWrap::Wrap } else { taffy::style::FlexWrap::NoWrap },
        flex_grow: flow.grow,
        flex_shrink: flow.shrink,
        gap: if grid { gap_size(flow.gap_cross, flow.gap_main) } else { gap_size(flow.gap_main, flow.gap_main) },
        padding: taffy::geometry::Rect {
            left: taffy::style::LengthPercentage::length(flow.padding.left),
            right: taffy::style::LengthPercentage::length(flow.padding.right),
            top: taffy::style::LengthPercentage::length(flow.padding.top),
            bottom: taffy::style::LengthPercentage::length(flow.padding.bottom),
        },
        align_items: Some(align_items(flow.align)),
        justify_content: Some(justify_content(flow.justify)),
        size: taffy::geometry::Size { width: taffy_dim(flow.width), height: taffy_dim(flow.height) },
        min_size: taffy::geometry::Size { width: taffy::style::Dimension::length(flow.min_width), height: taffy::style::Dimension::length(flow.min_height) },
        position: if flow.absolute { taffy::style::Position::Absolute } else { taffy::style::Position::Relative },
        inset: taffy::geometry::Rect { top: inset(flow.inset[0]), right: inset(flow.inset[1]), bottom: inset(flow.inset[2]), left: inset(flow.inset[3]) },
        ..Default::default()
    };
    if flow.clips {
        style.overflow = taffy::geometry::Point { x: taffy::style::Overflow::Hidden, y: taffy::style::Overflow::Hidden };
    }
    if let Some(tracks) = tracks {
        style.grid_template_columns = tracks.columns.iter().copied().map(grid_track).collect();
        style.grid_template_rows = tracks.rows.iter().copied().map(grid_track).collect();
    }
    style
}

fn taffy_dim(dim: Dim) -> taffy::style::Dimension {
    match dim {
        Dim::Auto => taffy::style::Dimension::auto(),
        Dim::Fill => taffy::style::Dimension::percent(1.0),
        Dim::Length(value) => taffy::style::Dimension::length(value),
    }
}

/// 🔖️ Mirrors `widgets`'/`paint`'s own `PANEL_HEADER` constant: the header-row height a
/// `Section` reserves for its content unconditionally (only the header's chevron+text *paint* is
/// gated on `label.is_some()`, not this offset).
pub(crate) const SECTION_HEADER_HEIGHT: f32 = 24.0;

/// 🌱️ Whether `kind`'s children grow into leftover main-axis space by virtue of the PARENT's kind
/// alone. True only for the legacy declarative dialect (see this file's header): an authored spec
/// carries each child's own `StackLayout::grow`, exactly like React's `flex: grow ? "1 1 auto" : undefined`.
fn grows_children(kind: LayoutNodeKind, authored: bool) -> bool {
    !authored && matches!(kind, LayoutNodeKind::Stack { .. } | LayoutNodeKind::Field { .. })
}

//#endregion 🎨️StyleMapping

//#region 🧮️FlexTree

/// 🍃️ What one scratch leaf answers the taffy fallback with: a text run re-measured against the
/// width it is actually offered (so it wraps the way CSS does), or an already-measured box.
#[derive(Clone, Copy)]
struct ScratchLeaf {
    node: usize,
    text: bool,
    intrinsic: (f32, f32),
}

/// 🧮️ Owns one bounded layout pass's styles and its solved boxes.
///
/// **The pass is per CONTAINER, never global, and plain flex is resolved arithmetically.** A single
/// whole-tree solve is not resumable, and even a single taffy solve of ONE 1,024-child container
/// measured ~11 ms, which starves every other interactive job on the lane (`engine`'s own
/// eight-millisecond slice law). So layout runs the way CSS itself resolves a box — a bottom-up
/// intrinsic (max-content) measurement, then a top-down arrange — and each grant handles exactly ONE
/// container against its own already-resolved box. For a plain single-line flex container (the
/// overwhelming majority, and the only shape the legacy chrome ever builds) that grant is pure
/// arithmetic over its direct children: flex-basis, grow/shrink, justify distribution, per-child
/// cross alignment, out-of-flow placement. `Wrapped` and `Grid` containers hand the same one-container
/// grant to taffy instead, which keeps multi-line reflow and real grid track sizing exact; those two
/// shapes are small in practice (a toolbar, a property grid), so the grant stays bounded either way.
/// Taffy also remains the third-party oracle the arithmetic path is validated against in `🧪️tests`.
///
/// No taffy value is RETAINED here — a local `taffy::TaffyTree` lives inside the two fallback methods
/// and dies with them. That is what keeps this type `Send` without an `unsafe impl`, which matters
/// because `mounted_layout::MountedLayoutJob` owns one and runs on a shared worker thread
/// (`taffy::style::CompactLength` packs its payload into a `*const ()`, so a retained taffy tree is
/// not `Send`).
pub(crate) struct FlexTree {
    flows: Vec<FlowStyle>,
    grids: Vec<Option<Box<ui_contract::GridLayout>>>,
    intrinsic: Vec<(f32, f32)>,
    resolved: Vec<FlexRect>,
    main: Vec<f32>,
    cross: Vec<f32>,
}

impl Default for FlexTree {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexTree {
    pub(crate) fn new() -> Self {
        Self { flows: Vec::new(), grids: Vec::new(), intrinsic: Vec::new(), resolved: Vec::new(), main: Vec::new(), cross: Vec::new() }
    }

    pub(crate) fn len(&self) -> usize {
        self.flows.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.flows.is_empty()
    }

    /// 🌱️ Admits the job's next node (nodes arrive in the admission walk's depth-first preorder).
    /// Only a `Grid` retains its authored track lists — everything else layout needs lives in the
    /// [`FlowStyle`], so the common node costs one small `Copy` record and no allocation at all.
    pub(crate) fn push(&mut self, kind: LayoutNodeKind, parent: Option<usize>, authored: Option<&LayoutSpec>, metrics: &TreeRowMetrics, parent_kind: Option<LayoutNodeKind>) -> bool {
        let mut flow = flow_for(kind, parent_kind, authored, metrics);
        if parent.is_some_and(|parent| self.flows.get(parent).is_some_and(|owner| owner.grows_children)) && !flow.absolute {
            flow.grow = 1.0;
        }
        flow.grows_children = grows_children(kind, authored.is_some());
        let grid = match authored {
            Some(LayoutSpec::Grid(tracks)) if matches!(flow.kind, FlowKind::Grid) => Some(Box::new(tracks.clone())),
            _ => None,
        };
        self.flows.push(flow);
        self.grids.push(grid);
        self.intrinsic.push((0.0, 0.0));
        self.resolved.push(FlexRect::default());
        true
    }

    /// 🪟️ Seeds the root's own box — the viewport it is laid out against.
    pub(crate) fn set_root_box(&mut self, index: usize, rect: FlexRect) -> bool {
        match self.resolved.get_mut(index) {
            Some(slot) => {
                *slot = rect;
                true
            }
            None => false,
        }
    }

    /// 📐️ The solved, parent-relative box for the job's node `index`.
    pub(crate) fn rect(&self, index: usize) -> Option<FlexRect> {
        self.resolved.get(index).copied()
    }

    pub(crate) fn intrinsic(&self, index: usize) -> Option<(f32, f32)> {
        self.intrinsic.get(index).copied()
    }

    /// 📏️ Node `index`'s intrinsic (max-content) size, from its `children`'s already-measured intrinsic
    /// sizes — the bottom-up half. `children` must be in document order, and every one of them must be
    /// measured already (reverse admission order guarantees it).
    pub(crate) fn measure_one(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let Some(flow) = self.flows.get(index).copied() else { return false };
        if !matches!(flow.kind, FlowKind::Flow) {
            return self.measure_via_taffy(index, children, measure);
        }
        let (main_start, main_end) = flow.padding_main();
        let (cross_start, cross_end) = flow.padding_cross();
        let (mut main_sum, mut cross_max, mut count) = (0.0_f32, 0.0_f32, 0_usize);
        for &child in children {
            let Some(child_flow) = self.flows.get(child).copied() else { return false };
            if child_flow.absolute {
                continue;
            }
            let Some(&(width, height)) = self.intrinsic.get(child) else { return false };
            let (main, cross) = if flow.row { (width, height) } else { (height, width) };
            main_sum += main;
            cross_max = cross_max.max(cross);
            count += 1;
        }
        let mut main = main_sum + flow.gap_main * count.saturating_sub(1) as f32 + main_start + main_end;
        let mut cross = cross_max + cross_start + cross_end;
        if flow.text {
            let (width, height) = measure(index, MeasureConstraint::MaxContent);
            let (own_main, own_cross) = if flow.row { (width, height) } else { (height, width) };
            main = main.max(own_main + main_start + main_end);
            cross = cross.max(own_cross + cross_start + cross_end);
        }
        let (mut width, mut height) = if flow.row { (main, cross) } else { (cross, main) };
        if let Dim::Length(value) = flow.width {
            width = value;
        }
        if let Dim::Length(value) = flow.height {
            height = value;
        }
        match self.intrinsic.get_mut(index) {
            Some(slot) => *slot = (width.max(flow.min_width), height.max(flow.min_height)),
            None => return false,
        }
        true
    }

    /// 📐️ Arranges `children` inside node `index`'s own resolved box — the top-down half. Each child's
    /// parent-relative rect is written into this tree, so a later [`Self::rect`] reads it back.
    pub(crate) fn arrange_one(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let (Some(flow), Some(&owner)) = (self.flows.get(index).copied(), self.resolved.get(index)) else { return false };
        if children.is_empty() {
            return true;
        }
        if !matches!(flow.kind, FlowKind::Flow) {
            return self.arrange_via_taffy(index, children, measure);
        }
        let (main_start, main_end) = flow.padding_main();
        let (cross_start, cross_end) = flow.padding_cross();
        let (outer_main, outer_cross) = if flow.row { (owner.width, owner.height) } else { (owner.height, owner.width) };
        let content_main = (outer_main - main_start - main_end).max(0.0);
        let content_cross = (outer_cross - cross_start - cross_end).max(0.0);

        self.main.clear();
        self.cross.clear();
        let (mut base_sum, mut grow_sum, mut shrink_sum, mut count) = (0.0_f32, 0.0_f32, 0.0_f32, 0_usize);
        for &child in children {
            let (Some(child_flow), Some(&intrinsic)) = (self.flows.get(child).copied(), self.intrinsic.get(child)) else { return false };
            if child_flow.absolute {
                self.main.push(0.0);
                self.cross.push(0.0);
                continue;
            }
            let (main, cross) = child_flow.flow_size(flow, content_main, content_cross, intrinsic, child, measure);
            base_sum += main;
            grow_sum += child_flow.grow;
            shrink_sum += child_flow.shrink * main;
            count += 1;
            self.main.push(main);
            self.cross.push(cross);
        }

        let gaps = flow.gap_main * count.saturating_sub(1) as f32;
        let free = content_main - base_sum - gaps;
        if free > 0.0 && grow_sum > 0.0 {
            for (slot, &child) in self.main.iter_mut().zip(children) {
                let Some(child_flow) = self.flows.get(child) else { return false };
                if !child_flow.absolute {
                    *slot += free * child_flow.grow / grow_sum;
                }
            }
        } else if free < 0.0 && shrink_sum > 0.0 {
            for (slot, &child) in self.main.iter_mut().zip(children) {
                let Some(child_flow) = self.flows.get(child) else { return false };
                if !child_flow.absolute {
                    *slot = (*slot + free * (child_flow.shrink * *slot) / shrink_sum).max(0.0);
                }
            }
        }

        let placed: f32 = self.main.iter().sum();
        let remaining = (content_main - placed - gaps).max(0.0);
        let (lead, between) = distribute(flow.justify, remaining, count);
        let mut cursor = if flow.reverse { outer_main - main_end - lead } else { main_start + lead };
        for (ordinal, &child) in children.iter().enumerate() {
            let (Some(child_flow), Some(&main), Some(&cross)) = (self.flows.get(child).copied(), self.main.get(ordinal), self.cross.get(ordinal)) else { return false };
            let rect = if child_flow.absolute {
                child_flow.absolute_rect(owner, self.intrinsic.get(child).copied().unwrap_or((0.0, 0.0)))
            } else {
                let offset = match flow.align {
                    Align::Center => (content_cross - cross) * 0.5,
                    Align::End => content_cross - cross,
                    Align::Start | Align::Stretch | Align::Baseline => 0.0,
                };
                if flow.reverse {
                    cursor -= main;
                }
                let (x, y, width, height) = if flow.row { (cursor, cross_start + offset, main, cross) } else { (cross_start + offset, cursor, cross, main) };
                if flow.reverse {
                    cursor -= flow.gap_main + between;
                } else {
                    cursor += main + flow.gap_main + between;
                }
                FlexRect { x, y, width, height }
            };
            match self.resolved.get_mut(child) {
                Some(slot) => *slot = rect,
                None => return false,
            }
        }
        true
    }

    fn taffy_style_of(&self, index: usize) -> Option<taffy::Style> {
        Some(taffy_style(self.flows.get(index)?, self.grids.get(index)?.as_deref()))
    }

    fn scratch_child(&self, scratch: &mut taffy::TaffyTree<ScratchLeaf>, index: usize) -> Option<taffy::NodeId> {
        let style = self.taffy_style_of(index)?;
        let leaf = ScratchLeaf { node: index, text: self.flows.get(index)?.text, intrinsic: *self.intrinsic.get(index)? };
        scratch.new_leaf_with_context(style, leaf).ok()
    }

    /// 📦️ The container's own taffy style for a local solve: it IS the root of that solve, so its own
    /// out-of-flow placement and grow factor (its parent's business, already settled) are cleared.
    fn own_style(&self, index: usize) -> Option<taffy::Style> {
        let mut style = self.taffy_style_of(index)?;
        style.position = taffy::style::Position::Relative;
        style.inset = taffy::geometry::Rect::auto();
        style.flex_grow = 0.0;
        Some(style)
    }

    /// 📏️ The `Wrapped`/`Grid` intrinsic measurement: one local taffy solve over this container and
    /// its already-measured children, at max-content.
    fn measure_via_taffy(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let Some(style) = self.own_style(index) else { return false };
        let mut scratch: taffy::TaffyTree<ScratchLeaf> = taffy::TaffyTree::new();
        scratch.disable_rounding();
        let mut kids = Vec::with_capacity(children.len());
        for &child in children {
            let Some(node) = self.scratch_child(&mut scratch, child) else { return false };
            kids.push(node);
        }
        let Ok(root) = scratch.new_with_children(style, &kids) else { return false };
        let available = taffy::geometry::Size { width: taffy::AvailableSpace::MaxContent, height: taffy::AvailableSpace::MaxContent };
        if solve_scratch(&mut scratch, root, available, measure).is_none() {
            return false;
        }
        let Ok(layout) = scratch.layout(root) else { return false };
        match self.intrinsic.get_mut(index) {
            Some(slot) => *slot = (layout.size.width, layout.size.height),
            None => return false,
        }
        true
    }

    /// 📐️ The `Wrapped`/`Grid` arrange: one local taffy solve against this container's resolved box.
    fn arrange_via_taffy(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let (Some(mut style), Some(&owner)) = (self.own_style(index), self.resolved.get(index)) else { return false };
        style.size = taffy::geometry::Size { width: taffy::style::Dimension::length(owner.width), height: taffy::style::Dimension::length(owner.height) };
        style.min_size = taffy::geometry::Size { width: taffy::style::Dimension::length(0.0), height: taffy::style::Dimension::length(0.0) };
        style.max_size = taffy::geometry::Size { width: taffy::style::Dimension::auto(), height: taffy::style::Dimension::auto() };
        let mut scratch: taffy::TaffyTree<ScratchLeaf> = taffy::TaffyTree::new();
        scratch.disable_rounding();
        let mut kids = Vec::with_capacity(children.len());
        for &child in children {
            let Some(node) = self.scratch_child(&mut scratch, child) else { return false };
            kids.push(node);
        }
        let Ok(root) = scratch.new_with_children(style, &kids) else { return false };
        let available = taffy::geometry::Size { width: taffy::AvailableSpace::Definite(owner.width), height: taffy::AvailableSpace::Definite(owner.height) };
        if solve_scratch(&mut scratch, root, available, measure).is_none() {
            return false;
        }
        for (&child, node) in children.iter().zip(kids) {
            let Ok(layout) = scratch.layout(node) else { return false };
            match self.resolved.get_mut(child) {
                Some(slot) => *slot = FlexRect { x: layout.location.x, y: layout.location.y, width: layout.size.width, height: layout.size.height },
                None => return false,
            }
        }
        true
    }

    /// 🧹️ Releases exactly ONE node, newest first — the same one-owner-per-grant close discipline
    /// every other bounded list in `mounted_layout` follows. Returns `true` once already empty.
    pub(crate) fn release_one(&mut self) -> bool {
        let Some(_) = self.flows.pop() else {
            self.main = Vec::new();
            self.cross = Vec::new();
            return true;
        };
        self.grids.pop();
        self.intrinsic.pop();
        self.resolved.pop();
        false
    }
}

impl FlowStyle {
    /// 📏️ One in-flow child's `(main, cross)` base size inside `parent`'s content box. A measured text
    /// run is re-measured against the size its container actually offers it, so it wraps where CSS
    /// would: in a column its width is settled first and the wrapped height becomes its main size; in
    /// a row its width is the main size and the wrapped height becomes its cross size.
    ///
    /// 🧭️ A child's own axes are read along its PARENT's main axis, never its own `row` flag —
    /// that flag describes how IT arranges ITS children, which says nothing about how it is placed.
    fn flow_size(self, parent: FlowStyle, content_main: f32, content_cross: f32, intrinsic: (f32, f32), node: usize, measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> (f32, f32) {
        let (own_main, own_cross, intrinsic_main, intrinsic_cross, min_main, min_cross) =
            if parent.row { (self.width, self.height, intrinsic.0, intrinsic.1, self.min_width, self.min_height) } else { (self.height, self.width, intrinsic.1, intrinsic.0, self.min_height, self.min_width) };
        let stretched = matches!(parent.align, Align::Stretch) && matches!(own_cross, Dim::Auto);
        let (mut main, mut cross) = if parent.row {
            let main = own_main.resolve(content_main).unwrap_or(intrinsic_main);
            let cross = if stretched {
                content_cross
            } else if self.text {
                own_cross.resolve(content_cross).unwrap_or_else(|| measure(node, MeasureConstraint::Definite(main)).1)
            } else {
                own_cross.resolve(content_cross).unwrap_or(intrinsic_cross)
            };
            (main, cross)
        } else {
            let cross = if stretched { content_cross } else { own_cross.resolve(content_cross).unwrap_or(intrinsic_cross) };
            let main = match own_main {
                Dim::Auto if self.text => measure(node, MeasureConstraint::Definite(cross)).1,
                dim => dim.resolve(content_main).unwrap_or(intrinsic_main),
            };
            (main, cross)
        };
        main = main.max(min_main);
        cross = cross.max(min_cross);
        (main, cross)
    }

    /// 📌️ An out-of-flow child's rect inside `owner`: each side comes from its own inset, its size
    /// from its own sizing, and — when both insets on an axis are given — from the span between them.
    fn absolute_rect(self, owner: FlexRect, intrinsic: (f32, f32)) -> FlexRect {
        let span = |size: Dim, start: Option<f32>, end: Option<f32>, available: f32, fallback: f32| {
            size.resolve(available).unwrap_or_else(|| match (start, end) {
                (Some(start), Some(end)) => (available - start - end).max(0.0),
                _ => fallback,
            })
        };
        let width = span(self.width, self.inset[3], self.inset[1], owner.width, intrinsic.0);
        let height = span(self.height, self.inset[0], self.inset[2], owner.height, intrinsic.1);
        let x = self.inset[3].unwrap_or_else(|| self.inset[1].map_or(0.0, |right| owner.width - right - width));
        let y = self.inset[0].unwrap_or_else(|| self.inset[2].map_or(0.0, |bottom| owner.height - bottom - height));
        FlexRect { x, y, width, height }
    }
}

/// ↔️ CSS `justify-content`: how leftover main-axis space is spent once no child can grow into it —
/// as a lead offset, as spacing between children, or both.
fn distribute(justify: Justify, remaining: f32, count: usize) -> (f32, f32) {
    let count = count as f32;
    match justify {
        Justify::Start => (0.0, 0.0),
        Justify::Center => (remaining * 0.5, 0.0),
        Justify::End => (remaining, 0.0),
        Justify::SpaceBetween if count > 1.0 => (0.0, remaining / (count - 1.0)),
        Justify::SpaceBetween => (0.0, 0.0),
        Justify::SpaceAround if count > 0.0 => (remaining / (count * 2.0), remaining / count),
        Justify::SpaceEvenly if count > 0.0 => (remaining / (count + 1.0), remaining / (count + 1.0)),
        Justify::SpaceAround | Justify::SpaceEvenly => (0.0, 0.0),
    }
}

/// 🏁️ Runs one local taffy solve, answering every scratch leaf from its own measurement rule.
fn solve_scratch(scratch: &mut taffy::TaffyTree<ScratchLeaf>, root: taffy::NodeId, available: taffy::geometry::Size<taffy::AvailableSpace>, measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> Option<()> {
    scratch
        .compute_layout_with_measure(root, available, |known, space, _node, context, _style| {
            if let (Some(width), Some(height)) = (known.width, known.height) {
                return taffy::geometry::Size { width, height };
            }
            let Some(leaf) = context else { return taffy::geometry::Size::ZERO };
            let (measured_width, measured_height) = if leaf.text {
                let constraint = match known.width {
                    Some(width) => MeasureConstraint::Definite(width),
                    None => match space.width {
                        taffy::AvailableSpace::Definite(width) => MeasureConstraint::Definite(width),
                        taffy::AvailableSpace::MinContent => MeasureConstraint::MinContent,
                        taffy::AvailableSpace::MaxContent => MeasureConstraint::MaxContent,
                    },
                };
                measure(leaf.node, constraint)
            } else {
                leaf.intrinsic
            };
            taffy::geometry::Size { width: known.width.unwrap_or(measured_width), height: known.height.unwrap_or(measured_height) }
        })
        .ok()
}

//#endregion 🧮️FlexTree

//#region 🚦️JobStages

/// 🚦️ The bounded layout job's stages, in the order `mounted_layout::MountedLayoutJob` walks them:
/// admit the arena walk (`CollectNodes`), shape every text scalar (`ShapeText`), measure intrinsic
/// sizes bottom-up (`MeasureLayout`), arrange each container against
/// its own resolved box top-down (`SolveLayout`), read every box back (`CollectResults`), then publish
/// into the inactive layout generation one node at a time (`PublishResults`). Every one of them
/// advances exactly one owner — see [`FlexTree`] for why the arrange is per container, not global.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LayoutJobStage {
    CollectNodes,
    ShapeText,
    MeasureLayout,
    SolveLayout,
    CollectResults,
    PublishResults,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum LayoutJobStep {
    Yield { stage: LayoutJobStage, nodes: usize, glyphs: usize },
    Complete,
    Cancelled,
    Fault(&'static str),
}

//#endregion 🚦️JobStages

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs"]
mod tests;
// #endregion flex
