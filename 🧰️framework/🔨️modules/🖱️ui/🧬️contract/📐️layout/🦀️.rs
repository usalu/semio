//! @emoji 📐️ The renderer-neutral `LayoutSpec` vocabulary and the `WindowLayout` shell model.
//!
//! ⚠️ SCAFFOLD — owned by packet `contract-layout`. Replace this placeholder wholesale; keep the region
//! structure and the U1 sync rule (no `async fn` in this crate).

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Layout

/// 📐️ Closed spacing scale a renderer resolves against the active theme's spacing ramp — never a raw
/// `f32`/px. tokens.json's `spacing` table today only names `compact`/`touch` (see [`crate::Density`]);
/// no full ramp exists there yet, so this scale is the shape this packet's own brief specifies
/// verbatim (`None,Xs,Sm,Md,Lg,Xl,…`) pending a registrar-added token set — flagged in the packet report.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum SpaceToken {
    #[default]
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

/// 📐️ The ONE spacing ramp every renderer resolves a [`SpaceToken`] against — the multiplier table
/// React's own `SPACE_TOKEN_MULTIPLIER`
/// (`os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`) carries, times
/// `ui_styling`'s generated `--ui-spacing` compact step. Renderers call [`SpaceToken::px`]/
/// [`EdgeSpace::px`]; none of them owns a private table, so a DOM rect and a GPU rect can never drift.
const SPACE_TOKEN_MULTIPLIER: [f32; 7] = [0.0, 1.0, 2.0, 4.0, 6.0, 8.0, 12.0];

impl SpaceToken {
    /// 📐️ This token's multiple of the compact `--ui-spacing` step — React's `SPACE_TOKEN_MULTIPLIER`.
    pub const fn multiplier(self) -> f32 {
        SPACE_TOKEN_MULTIPLIER[self as usize]
    }

    /// 📐️ This token in logical pixels at the compact reference root, e.g. `Md → 12.8`.
    pub fn px(self) -> f32 {
        self.multiplier() * ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX as f32
    }
}

/// 📐️ A resolved [`EdgeSpace`] in logical pixels — the four independent CSS values React writes as a
/// `padding`/`inset` shorthand, never one representative side applied to all four.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EdgePx {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeSpace {
    /// 📐️ Resolves all four sides through [`SpaceToken::px`].
    pub fn px(self) -> EdgePx {
        match self {
            Self::All(token) => {
                let value = token.px();
                EdgePx { top: value, right: value, bottom: value, left: value }
            }
            Self::Symmetric { vertical, horizontal } => EdgePx { top: vertical.px(), right: horizontal.px(), bottom: vertical.px(), left: horizontal.px() },
            Self::Each { top, right, bottom, left } => EdgePx { top: top.px(), right: right.px(), bottom: bottom.px(), left: left.px() },
        }
    }
}

/// 📏️ How a node sizes itself along one axis relative to its parent's flow — `Fixed` still names a
/// [`SpaceToken`], never a pixel value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Sizing {
    #[default]
    Hug,
    Fill,
    Fixed(SpaceToken),
}

/// ↔️ The main axis a [`StackLayout`] or [`WindowLayoutNode::Split`] lays its children along.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Axis {
    #[default]
    Horizontal,
    Vertical,
}

/// ↕️ Cross-axis alignment — the CSS `align-items` equivalent, `Stretch` default so a node fills its
/// cross axis unless it opts out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Align {
    Start,
    Center,
    End,
    #[default]
    Stretch,
    Baseline,
}

/// ↔️ Main-axis distribution — the CSS `justify-content` equivalent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Justify {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// 🔲️ One grid track's sizing rule — `Fraction` is a proportion count, never a pixel width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum GridTrack {
    #[default]
    Auto,
    Fraction(u8),
    Fixed(SpaceToken),
    MinContent,
    MaxContent,
}

/// 🖱️ Which axes a [`ScrollLayout`] permits overflow scrolling on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum ScrollAxes {
    #[default]
    None,
    Horizontal,
    Vertical,
    Both,
}

/// 🧭️ A logical 9-point placement, `Start`/`End` rather than `Left`/`Right` so it stays correct under
/// RTL locales without a renderer-side flip (CLAUDE.md's multi-language accessibility mandate).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Anchor {
    TopStart,
    Top,
    TopEnd,
    Start,
    #[default]
    Center,
    End,
    BottomStart,
    Bottom,
    BottomEnd,
}

/// 📐️ Per-side padding that costs one [`SpaceToken`] on the wire in the common uniform case, instead
/// of four always-present fields — mirrors CSS shorthand's 1/2/4-value forms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum EdgeSpace {
    All(SpaceToken),
    Symmetric { vertical: SpaceToken, horizontal: SpaceToken },
    Each { top: SpaceToken, right: SpaceToken, bottom: SpaceToken, left: SpaceToken },
}

impl Default for EdgeSpace {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn default() -> Self {
        Self::All(SpaceToken::None)
    }
}

/// 📚️ A one-axis flex-like arrangement — expressible by CSS flex, a taffy tree, or a native stack.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct StackLayout {
    pub axis: Axis,
    pub gap: SpaceToken,
    pub padding: EdgeSpace,
    pub align: Align,
    pub justify: Justify,
    pub grow: bool,
    pub wrap: bool,
}

/// 🔲️ A two-dimensional track arrangement — expressible by CSS grid or a taffy grid tree.
pub const UI_GRID_TRACKS: usize = 32;
pub type UiGridTracks = crate::UiFixedList<GridTrack, UI_GRID_TRACKS>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct GridLayout {
    pub columns: UiGridTracks,
    pub rows: UiGridTracks,
    pub column_gap: SpaceToken,
    pub row_gap: SpaceToken,
    pub padding: EdgeSpace,
    pub align: Align,
    pub justify: Justify,
}

impl GridLayout {
    pub fn try_push_column(&mut self, track: GridTrack) -> Result<(), GridTrack> {
        self.columns.try_push(track)
    }

    pub fn try_push_row(&mut self, track: GridTrack) -> Result<(), GridTrack> {
        self.rows.try_push(track)
    }
}

/// 🪟️ A positioning context whose children stack on top of one another anchored to the box —
/// modals, popovers, tooltips.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct OverlayLayout {
    pub anchor: Anchor,
    pub inset: EdgeSpace,
    pub dismissible: bool,
}

/// 🖱️ A viewport clipping its content and permitting overflow scroll on the named axes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ScrollLayout {
    pub axes: ScrollAxes,
    pub padding: EdgeSpace,
    pub sizing: Sizing,
}

/// 📌️ A freeform positioning context — children carry their own placement outside normal flow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct AbsoluteLayout {
    pub sizing_width: Sizing,
    pub sizing_height: Sizing,
}

/// 🍃️ A childless terminal node's own box sizing — text, image, and other atomic components.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct LeafLayout {
    pub width: Sizing,
    pub height: Sizing,
}

/// 🧬️ The renderer-neutral layout vocabulary a [`crate::UiNodeRecord`] carries — expressible by CSS
/// flex/grid, by a taffy tree, and by native stacks alike. No CSS strings, no taffy types, no pixel
/// geometry: every metric is a closed enum over [`SpaceToken`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase")]
pub enum LayoutSpec {
    /// 🍃️ A node that participates in its parent's layout but imposes none of its own.
    Leaf(LeafLayout),
    Stack(StackLayout),
    Grid(GridLayout),
    Overlay(OverlayLayout),
    Scroll(ScrollLayout),
    Absolute(AbsoluteLayout),
}

/// 🍃️ A record whose layout was never set must not silently become a container, so the default is the
/// terminal one. `#[derive(Default)]` cannot express this — the attribute only accepts unit variants.
impl Default for LayoutSpec {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn default() -> Self {
        Self::Leaf(LeafLayout::default())
    }
}
//#endregion 🔖️Layout

//#region 🧭️Flow

/// 🧭️ Horizontal reading direction — `Rtl` mirrors inline chrome. The renderer-neutral twin of
/// React's `FlowInline` (`🔨️modules/🧭️flow-direction-context/🟦️.tsx:14`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum FlowInline {
    #[default]
    Ltr,
    Rtl,
}

impl FlowInline {
    /// ↔️ Whether inline `Start` means the RIGHT edge — the single predicate every mirrored formula
    /// branches on, so no call site repeats the comparison React writes as `flow.inline === "rtl"`.
    pub const fn is_rtl(self) -> bool {
        matches!(self, FlowInline::Rtl)
    }

    /// ↔️ `+1.0` under `Ltr`, `-1.0` under `Rtl` — the sign an inline delta (a horizontal scroll, a
    /// slider's `ArrowRight`) carries once mirrored.
    pub const fn inline_sign(self) -> f32 {
        match self {
            FlowInline::Ltr => 1.0,
            FlowInline::Rtl => -1.0,
        }
    }
}

/// 🧭️ Vertical stacking direction — `Up` grows content toward the display centre, which is what a
/// bottom-docked panel does. Twin of React's `FlowBlock`
/// (`🔨️modules/🧭️flow-direction-context/🟦️.tsx:17`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum FlowBlock {
    #[default]
    Down,
    Up,
}

impl FlowBlock {
    /// ⬆️ Whether a stack in this flow paints its children bottom-up — React's
    /// `flow.block === "up" ? "flex-col-reverse" : "flex-col"` (`🧱️elements/🖼️Panel/🟦️.tsx:513`).
    pub const fn is_reversed(self) -> bool {
        matches!(self, FlowBlock::Up)
    }
}

/// 🧭️ The logical flow a subtree inherits — twin of React's `Flow`
/// (`🔨️modules/🧭️flow-direction-context/🟦️.tsx:20-23`), whose default is `{ ltr, down }`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct UiFlow {
    pub inline: FlowInline,
    pub block: FlowBlock,
}

impl UiFlow {
    /// 🧭️ React's `DEFAULT_FLOW` (`🧭️flow-direction-context/🟦️.tsx:35`).
    pub const DEFAULT: Self = Self { inline: FlowInline::Ltr, block: FlowBlock::Down };

    /// 🧭️ React's `FlowProvider` merge rule (`🧭️flow-direction-context/🟦️.tsx:41-44`): a `None`
    /// override inherits the parent axis, so a provider that sets only `block` keeps the inherited
    /// `inline`.
    pub const fn merged(self, inline: Option<FlowInline>, block: Option<FlowBlock>) -> Self {
        Self { inline: match inline { Some(value) => value, None => self.inline }, block: match block { Some(value) => value, None => self.block } }
    }

    /// 🧭️ React's `flowFromAnchor` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6859-6860`): the mirrored flow a
    /// `Panel`/`Pane` grows into. A RIGHT anchor flips inline, a BOTTOM anchor flips block, and a
    /// middle anchor on either axis never mirrors. The one production origin of a non-default flow
    /// on BOTH renderers — flow is dock geometry, not language.
    pub const fn for_anchor(anchor: Anchor) -> Self {
        let inline = match anchor {
            Anchor::TopEnd | Anchor::End | Anchor::BottomEnd => FlowInline::Rtl,
            _ => FlowInline::Ltr,
        };
        let block = match anchor {
            Anchor::BottomStart | Anchor::Bottom | Anchor::BottomEnd => FlowBlock::Up,
            _ => FlowBlock::Down,
        };
        Self { inline, block }
    }
}

//#endregion 🧭️Flow

//#region 🔖️WindowLayout

/// 🪟️ Corner of a window stack where a tab chip docks. Ported verbatim from the wgpu target's
/// `WindowStackCorner`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum WindowStackCorner {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// 🪟️ The window-shell tree: a single recursive, internally-tagged enum replacing the old
/// `WindowLayoutWindowNode`/`WindowLayoutStackNode`/`WindowLayoutAxisNode` trio and their
/// `kind: String` + `#[serde(untagged)]` scheme. `size` stays an `Option<f64>` fraction of the parent
/// split (a ratio, not a pixel measurement, so it is exempt from the [`SpaceToken`] rule). The
/// `alias = "activeId"` serde alias on the old stack node is dropped — greenfield, fixtures
/// re-handcrafted, no compatibility requirement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(crate = "::protocol::value", tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum WindowLayoutNode {
    Window {
        window_kind_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        instance_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        template_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        corner: Option<WindowStackCorner>,
    },
    Stack {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        size: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        active_window_kind_id: Option<String>,
        children: Vec<WindowLayoutNode>,
    },
    Split {
        axis: Axis,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        size: Option<f64>,
        children: Vec<WindowLayoutNode>,
    },
}

/// 🪟️ The window-shell root. Moved here from the wgpu target's `WindowLayout` — same name, one
/// recursive `WindowLayoutNode` root instead of the old `WindowLayoutRoot` `Axis`/`Stack` union.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct WindowLayout {
    pub root: WindowLayoutNode,
}
//#endregion 🔖️WindowLayout

#[cfg(test)]
#[path = "../🧪️tests/🔬️layout-unit/🦀️.rs"]
mod tests;
