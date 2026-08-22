//! @emoji 📐️ The renderer-neutral `LayoutSpec` vocabulary and the `WindowLayout` shell model.
//!
//! ⚠️ SCAFFOLD — owned by packet `contract-layout`. Replace this placeholder wholesale; keep the region
//! structure and the U1 sync rule (no `async fn` in this crate).

use serde::{Deserialize, Serialize};

//#region 🔖️Layout

/// 📐️ Closed spacing scale a renderer resolves against the active theme's spacing ramp — never a raw
/// `f32`/px. tokens.json's `spacing` table today only names `compact`/`touch` (see [`crate::Density`]);
/// no full ramp exists there yet, so this scale is the shape this packet's own brief specifies
/// verbatim (`None,Xs,Sm,Md,Lg,Xl,…`) pending a registrar-added token set — flagged in the packet report.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// 📏️ How a node sizes itself along one axis relative to its parent's flow — `Fixed` still names a
/// [`SpaceToken`], never a pixel value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Sizing {
    #[default]
    Hug,
    Fill,
    Fixed(SpaceToken),
}

/// ↔️ The main axis a [`StackLayout`] or [`WindowLayoutNode::Split`] lays its children along.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Axis {
    #[default]
    Horizontal,
    Vertical,
}

/// ↕️ Cross-axis alignment — the CSS `align-items` equivalent, `Stretch` default so a node fills its
/// cross axis unless it opts out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Align {
    Start,
    Center,
    End,
    #[default]
    Stretch,
    Baseline,
}

/// ↔️ Main-axis distribution — the CSS `justify-content` equivalent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GridTrack {
    #[default]
    Auto,
    Fraction(u8),
    Fixed(SpaceToken),
    MinContent,
    MaxContent,
}

/// 🖱️ Which axes a [`ScrollLayout`] permits overflow scrolling on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScrollAxes {
    #[default]
    None,
    Horizontal,
    Vertical,
    Both,
}

/// 🧭️ A logical 9-point placement, `Start`/`End` rather than `Left`/`Right` so it stays correct under
/// RTL locales without a renderer-side flip (CLAUDE.md's multi-language accessibility mandate).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridLayout {
    pub columns: Vec<GridTrack>,
    pub rows: Vec<GridTrack>,
    pub column_gap: SpaceToken,
    pub row_gap: SpaceToken,
    pub padding: EdgeSpace,
    pub align: Align,
    pub justify: Justify,
}

/// 🪟️ A positioning context whose children stack on top of one another anchored to the box —
/// modals, popovers, tooltips.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverlayLayout {
    pub anchor: Anchor,
    pub inset: EdgeSpace,
    pub dismissible: bool,
}

/// 🖱️ A viewport clipping its content and permitting overflow scroll on the named axes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrollLayout {
    pub axes: ScrollAxes,
    pub padding: EdgeSpace,
    pub sizing: Sizing,
}

/// 📌️ A freeform positioning context — children carry their own placement outside normal flow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbsoluteLayout {
    pub sizing_width: Sizing,
    pub sizing_height: Sizing,
}

/// 🍃️ A childless terminal node's own box sizing — text, image, and other atomic components.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeafLayout {
    pub width: Sizing,
    pub height: Sizing,
}

/// 🧬️ The renderer-neutral layout vocabulary a [`crate::UiNodeRecord`] carries — expressible by CSS
/// flex/grid, by a taffy tree, and by native stacks alike. No CSS strings, no taffy types, no pixel
/// geometry: every metric is a closed enum over [`SpaceToken`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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

//#region 🔖️WindowLayout

/// 🪟️ Corner of a window stack where a tab chip docks. Ported verbatim from the wgpu target's
/// `WindowStackCorner`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum WindowLayoutNode {
    Window {
        window_kind_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        instance_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        template_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        corner: Option<WindowStackCorner>,
    },
    Stack {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        size: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        active_window_kind_id: Option<String>,
        children: Vec<WindowLayoutNode>,
    },
    Split {
        axis: Axis,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        size: Option<f64>,
        children: Vec<WindowLayoutNode>,
    },
}

/// 🪟️ The window-shell root. Moved here from the wgpu target's `WindowLayout` — same name, one
/// recursive `WindowLayoutNode` root instead of the old `WindowLayoutRoot` `Axis`/`Stack` union.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowLayout {
    pub root: WindowLayoutNode,
}
//#endregion 🔖️WindowLayout

#[cfg(test)]
mod tests {
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
            columns: vec![GridTrack::Fraction(1), GridTrack::Fixed(SpaceToken::Lg)],
            rows: vec![GridTrack::Auto],
            column_gap: SpaceToken::Sm,
            row_gap: SpaceToken::None,
            padding: EdgeSpace::Each { top: SpaceToken::Xs, right: SpaceToken::Sm, bottom: SpaceToken::Xs, left: SpaceToken::Sm },
            align: Align::Stretch,
            justify: Justify::Start,
        }));
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
}
