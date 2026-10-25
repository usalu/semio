//! 🖌️ Flow style: the dimension, the flow kind and style, the flow from the authored spec, the tree content flow and `flow_for`. See [`super`].

use super::*;

// #region flex
//#region 🌊️FlowStyle
/// 📏️ One axis's sizing rule in this engine's own vocabulary — the closed reading of
/// [`ui_contract::Sizing`] plus the fixed pixel bands the composite wgpu widgets carry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dim {
    Auto,
    Fill,
    Length(f32),
}

impl Dim {
    pub(super) fn resolve(self, available: f32) -> Option<f32> {
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

/// 🌊️ How a container arranges its own children. `Flow` is plain single-line flex, `Wrapped` is multi-line flex
/// and `Grid` is a track-sized grid; this engine resolves all three arithmetically, one container per grant
/// (see [`FlexTree::measure_one`] and [`FlexTree::arrange_one`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlowKind {
    Flow,
    Wrapped,
    Grid,
}

/// 🗺️ Everything the arithmetic solvers need about one node, in this crate's own vocabulary. There is one
/// constructor, [`flow_for`]; the oracle tests read taffy's `Style` from it, so the two cannot drift.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlowStyle {
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
    /// 🪟️ Main-axis empty pitch this child claims BEFORE itself in its parent's flow (after it, in a reversed flow) — a
    /// windowed container's leading spacer, carried by its first materialised row. `0` for every other node.
    pub lead: f32,
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
            lead: 0.0,
        }
    }
}

impl FlowStyle {
    pub(super) fn padding_main(&self) -> (f32, f32) {
        if self.row {
            (self.padding.left, self.padding.right)
        } else {
            (self.padding.top, self.padding.bottom)
        }
    }

    pub(super) fn padding_cross(&self) -> (f32, f32) {
        if self.row {
            (self.padding.top, self.padding.bottom)
        } else {
            (self.padding.left, self.padding.right)
        }
    }
}

/// 📕️ The authored `LayoutSpec` in flow vocabulary, field for field against React's
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

/// 🧺️ How a node inside a tree row's content lays out — the one geometry every renderer's row content shares here:
/// `Slot` is a direct content child, its box in the row's value column `top` below the row's top and `height` tall (a
/// `container` stacks its children between its label bands, `above` and `below` tall, a leaf fills the box); `Column`
/// is a nested container stacking its children between its label bands; `Line` is a content leaf, exactly one line tall.
/// Every content leaf and every label fills one line, so a row's content height is known before layout
/// (`layout::tree_content_height`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TreeContentFlow {
    Slot { top: f32, height: f32, above: f32, below: f32, container: bool },
    Column { above: f32, below: f32 },
    Line { height: f32 },
}

/// 🧵️ The flow a tree-row content node takes: its `natural` style keeps only what its own painting needs (a text
/// leaf's measuring, a button's label padding); the box and the stacking are [`TreeContentFlow`]'s.
pub(super) fn tree_content_flow(content: TreeContentFlow, natural: FlowStyle, metrics: &TreeRowMetrics) -> FlowStyle {
    let column = |above: f32, below: f32| FlowStyle { gap_main: metrics.gap, gap_cross: metrics.gap, padding: EdgePx { top: above, bottom: below, ..EdgePx::default() }, shrink: 0.0, ..FlowStyle::default() };
    match content {
        TreeContentFlow::Slot { top, height, above, below, container } => {
            let inset = if metrics.inline.is_rtl() { [Some(top), None, None, Some(metrics.gap)] } else { [Some(top), Some(metrics.gap), None, None] };
            let base = if container { column(above, below) } else { FlowStyle { text: natural.text, padding: natural.padding, shrink: 0.0, ..FlowStyle::default() } };
            FlowStyle { absolute: true, inset, width: Dim::Length(metrics.control_width), height: Dim::Length(height), clips: true, ..base }
        }
        TreeContentFlow::Column { above, below } => column(above, below),
        TreeContentFlow::Line { height } => FlowStyle { height: Dim::Length(height), min_height: height, shrink: 0.0, text: natural.text, padding: natural.padding, ..FlowStyle::default() },
    }
}

/// 🎨️ One retained node's flow style. `authored` is the node's own `LayoutSpec` when the producer
/// published one (the React-parity dialect); the composite wgpu kinds ignore it on purpose — their
/// geometry is what `paint`/`events` already derive their rows from.
///
/// 🎛️ One control row tall AT LEAST, never exactly: a `Select`'s synthesized option rows
/// are real arena children, and the popup grows past the control band to hold them.
///
/// 🧩️ EXACTLY the host's band, clipped, filling the parent's cross axis: the host paints inside
/// the solved rect and this engine reserves it without measuring anything of its own.
pub(super) fn flow_for(kind: LayoutNodeKind, parent_kind: Option<LayoutNodeKind>, authored: Option<&LayoutSpec>, metrics: &TreeRowMetrics) -> FlowStyle {
    if matches!(parent_kind, Some(LayoutNodeKind::TreeRow { .. })) && !matches!(kind, LayoutNodeKind::TreeRow { .. } | LayoutNodeKind::TreeDetail { .. }) {
        let control_height = match kind {
            LayoutNodeKind::Control { height, .. } => height,
            LayoutNodeKind::Stack { .. } => metrics.row_height,
            _ => metrics.control_height,
        };
        let rect = crate::layout::tree_row_control_rect_with_height(0.0, control_height, metrics);
        let inset = if metrics.inline.is_rtl() { [Some(rect.y), None, None, Some(metrics.gap)] } else { [Some(rect.y), Some(metrics.gap), None, None] };
        let content = if matches!(kind, LayoutNodeKind::Stack { .. }) { flow_for(kind, None, authored, metrics) } else { FlowStyle::default() };
        return FlowStyle { absolute: true, inset, width: Dim::Length(metrics.control_width), height: Dim::Length(control_height), ..content };
    }
    if matches!(parent_kind, Some(LayoutNodeKind::TableRow { .. })) {
        let mut cell = flow_for(kind, None, authored, metrics);
        cell.absolute = false;
        cell.grow = 1.0;
        cell.shrink = 1.0;
        cell.width = Dim::Length(0.0);
        cell.min_width = 0.0;
        cell.height = match kind {
            LayoutNodeKind::Control { height, .. } => Dim::Length(height),
            _ => Dim::Fill,
        };
        return cell;
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
        LayoutNodeKind::Field { top, bottom } => FlowStyle { padding: EdgePx { top, bottom, ..EdgePx::default() }, ..FlowStyle::default() },
        LayoutNodeKind::Section { gap, top, bottom } => FlowStyle { gap_main: gap, padding: EdgePx { top, bottom, ..EdgePx::default() }, ..FlowStyle::default() },
        LayoutNodeKind::Tree { height, header, reversed } => band(height, header, reversed),
        LayoutNodeKind::TreeSection { header, height, reversed, .. } => band(height, header, reversed),
        LayoutNodeKind::TreeHeaderToolbar { header, reversed } => {
            let inset = if reversed { [None, Some(metrics.gap), Some(0.0), None] } else { [Some(0.0), Some(metrics.gap), None, None] };
            let mut content = authored.map_or_else(FlowStyle::default, flow_from_spec);
            content.absolute = true;
            content.inset = inset;
            content.width = Dim::Auto;
            content.height = Dim::Length(header);
            content
        }
        LayoutNodeKind::TreeRow { row, height, reversed, lead, .. } => FlowStyle { lead, ..band(height, row, reversed) },
        LayoutNodeKind::TreeDetail { height } => FlowStyle { width: Dim::Fill, height: Dim::Length(height), shrink: 0.0, ..FlowStyle::default() },
        LayoutNodeKind::TableRow { height, actions, lead } => {
            let (left, right) = if metrics.inline.is_rtl() { (actions, metrics.gap) } else { (metrics.gap, actions) };
            FlowStyle {
                row: true,
                reverse: metrics.inline.is_rtl(),
                gap_main: metrics.gap,
                align: Align::Center,
                height: Dim::Length(height),
                shrink: 0.0,
                padding: EdgePx { left, right, ..EdgePx::default() },
                clips: true,
                lead,
                ..FlowStyle::default()
            }
        }
        LayoutNodeKind::Control { height, label_padding } => {
            let mut flow = authored.map_or_else(FlowStyle::default, flow_from_spec);
            flow.min_height = height;
            if let Some(padding) = label_padding {
                flow.text = true;
                flow.padding = EdgePx { top: flow.padding.top, right: flow.padding.right.max(padding), bottom: flow.padding.bottom, left: flow.padding.left.max(padding) };
            }
            flow
        }
        LayoutNodeKind::OverlayRow { rect } => FlowStyle { absolute: true, inset: [Some(rect.y), None, None, Some(rect.x)], width: Dim::Length(rect.width), height: Dim::Length(rect.height), shrink: 0.0, text: true, ..FlowStyle::default() },
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
// #endregion flex
