//! 🎚️ Layout node kind, geometry and job stages: the layout node kind catalogue, the flex rect and measure constraint and the layout job stage and step. See [`super`].

use super::*;

// #region flex
//#region 🎚️LayoutNodeKind
/// 🧩️ What one retained node contributes to layout, classified once during admission (which is the
/// only phase that holds the `UiTree`) and kept for the whole job — the worker phases run on a pool
/// thread with no tree access.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LayoutNodeKind {
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
        bottom: f32,
    },
    Section {
        gap: f32,
        top: f32,
        bottom: f32,
    },
    /// 🌳️ A `Tree`, measured from its own spec through `layout`'s shared row geometry rather than
    /// from arena children — a tree's rows carry no children of their own, so aggregating them
    /// measured a whole tree as the sum of its rows' padding. `header` is a table's column header
    /// band, reserved ahead of its rows (`0` for a tree).
    Tree {
        height: f32,
        header: f32,
        reversed: bool,
    },
    TreeSection {
        header: f32,
        height: f32,
        expanded: bool,
        reversed: bool,
    },
    /// 🎛️ A section header's related Toolbar, positioned at the logical end inside the header band.
    TreeHeaderToolbar {
        header: f32,
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
        /// 🪟️ The leading spacer of the windowed container this row is the FIRST materialised child of (else `0`).
        lead: f32,
    },
    /// 🎞️ One explicitly related TreeItem Surface, flowing below the row chrome and before nested
    /// rows in a bounded scene band.
    TreeDetail {
        height: f32,
    },
    /// 📊️ One table row: its materialised cell children flow along the inline axis in equal columns — the grid
    /// `layout::table_column_rect` gives the header and the painted cells — between the leading gap and the trailing
    /// `actions` column its row actions paint in.
    TableRow {
        height: f32,
        actions: f32,
        /// 🪟️ The windowed table's leading spacer when this row is its first materialised row (else `0`).
        lead: f32,
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
pub struct FlexRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 📏️ The available-space question taffy asks a measured leaf, in this crate's own vocabulary.
/// `MinContent`/`MaxContent` are CSS's intrinsic sizes: the widest single word, and the whole run on
/// one line — the two numbers a browser resolves text against inside a flex item.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MeasureConstraint {
    Definite(f32),
    MinContent,
    MaxContent,
}
//#endregion 📐️Geometry

//#region 🚦️JobStages
/// 🚦️ The bounded layout job's stages, in the order `mounted_layout::MountedLayoutJob` walks them:
/// admit the arena walk (`CollectNodes`), shape every text scalar (`ShapeText`), measure intrinsic
/// sizes bottom-up (`MeasureLayout`), arrange each container against
/// its own resolved box top-down (`SolveLayout`), read every box back (`CollectResults`), then publish
/// into the inactive layout generation one node at a time (`PublishResults`). Every one of them
/// advances exactly one owner — see [`FlexTree`] for why the arrange is per container, not global.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutJobStage {
    CollectNodes,
    ShapeText,
    MeasureLayout,
    SolveLayout,
    CollectResults,
    PublishResults,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayoutJobStep {
    Yield { stage: LayoutJobStage, nodes: usize, glyphs: usize },
    Complete,
    Cancelled,
    Fault(&'static str),
}
//#endregion 🚦️JobStages
// #endregion flex
