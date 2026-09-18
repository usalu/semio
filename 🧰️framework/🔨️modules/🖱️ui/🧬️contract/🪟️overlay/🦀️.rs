//! @emoji 🪟️ The ONE anchored-overlay positioner every renderer resolves a floating surface with —
//! the renderer-neutral port of React's `resolvePopoverPlacement`
//! (`🧱️elements/🗨️Popover/🟦️.tsx:216-259`).
//!
//! Lives in the contract rather than in a target because THREE renderers need the same answer: the
//! DOM one computes it in `Popover`, the retained wgpu one in `🎯️targets/🧊️wgpu/⚡️events`, and the
//! backend-neutral `🖌️render/🖱️dispatch` one in its own overlay region. Before this module the last
//! two each carried a private copy and the two copies had already drifted apart (ticket
//! `26/09/17/WGPU-RENDERER-REACT-PARITY`, packet W1n gap 6: one had been re-ported to React's
//! model, the other still carried a three-variant `BelowAnchorWithFlip`/`AtPointer`/`Centered`
//! guess).
//!
//! Dependency-free by construction: plain `f32` rectangles, no `Rect` type from any target — a
//! target converts at its own boundary.

use crate::FlowInline;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🪟️OverlayPlacement

/// 🧭️ Which edge of the anchor a floating surface hangs off — React's `PopoverSide`
/// (`🧱️elements/🗨️Popover/🟦️.tsx:22`). PHYSICAL, exactly as React takes it: `resolvePopoverPlacement`
/// mirrors `align` under RTL and leaves `side` alone.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum OverlaySide {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl OverlaySide {
    /// 🔁️ The collision flip target — React's `opposite` record (`🗨️Popover/🟦️.tsx:236`).
    pub const fn opposite(self) -> Self {
        match self {
            OverlaySide::Top => OverlaySide::Bottom,
            OverlaySide::Right => OverlaySide::Left,
            OverlaySide::Bottom => OverlaySide::Top,
            OverlaySide::Left => OverlaySide::Right,
        }
    }

    /// 🧭️ React's `transformOrigin` for the settled side (`🗨️Popover/🟦️.tsx:257`) — what an arrow or
    /// a scale-in animation points at.
    pub const fn transform_origin(self) -> &'static str {
        match self {
            OverlaySide::Top => "center bottom",
            OverlaySide::Bottom => "center top",
            OverlaySide::Left => "right center",
            OverlaySide::Right => "left center",
        }
    }
}

/// ↔️ How an overlay lines up along the anchor's cross axis — React's `PopoverAlign`
/// (`🧱️elements/🗨️Popover/🟦️.tsx:23`). LOGICAL on the inline axis: `Start`/`End` swap under
/// [`FlowInline::Rtl`], `Center` does not. On a vertical cross axis (`side: Left`/`Right`) they are
/// physical top/bottom and carry no direction, which is exactly what React's `alignedTop` does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum OverlayAlign {
    Start,
    #[default]
    Center,
    End,
}

/// 📐️ The side/align/offset/collision tuple React's `PopoverContent` takes as props — the exact
/// input set [`resolve_anchored_placement`] consumes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnchoredPlacement {
    pub side: OverlaySide,
    pub align: OverlayAlign,
    pub side_offset: f32,
    pub align_offset: f32,
    pub collision_padding: f32,
    pub avoid_collisions: bool,
}

impl AnchoredPlacement {
    /// 🗨️ `PopoverContent`'s own prop defaults — `🧱️elements/🗨️Popover/🟦️.tsx:288-293`.
    pub const POPOVER: Self = Self { side: OverlaySide::Bottom, align: OverlayAlign::Center, side_offset: 4.0, align_offset: 0.0, collision_padding: 8.0, avoid_collisions: true };
    /// 💡️ `ChromeControlHint`'s tooltip call — `🧱️elements/💡️ChromeControlHint/🟦️.tsx:69`.
    pub const TOOLTIP: Self = Self { side: OverlaySide::Top, align: OverlayAlign::Center, side_offset: 8.0, align_offset: 0.0, collision_padding: 8.0, avoid_collisions: true };
    /// 🔽️ A `Select` popup / context menu: flush under the trigger's start edge, no side gap.
    pub const MENU: Self = Self { side: OverlaySide::Bottom, align: OverlayAlign::Start, side_offset: 0.0, align_offset: 0.0, collision_padding: 0.0, avoid_collisions: true };
}

impl Default for AnchoredPlacement {
    fn default() -> Self {
        Self::POPOVER
    }
}

/// 📐️ How an overlay's resolved position is computed from its anchor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OverlayPlacement {
    /// ⚓️ React's anchored positioner: main-axis flip plus viewport clamp.
    Anchored(AnchoredPlacement),
    /// 🎯️ A modal surface (`Dialog`/`CommandPalette`): viewport-centered, no anchor.
    Centered,
}

impl Default for OverlayPlacement {
    fn default() -> Self {
        Self::Anchored(AnchoredPlacement::POPOVER)
    }
}

/// 📐️ What [`resolve_anchored_placement`] answers: the origin plus the side actually used after a
/// collision flip, so a caller can point an arrow the same way React's `transformOrigin` does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedOverlayPlacement {
    pub side: OverlaySide,
    pub x: f32,
    pub y: f32,
}

/// 📐️ An anchor rectangle in the same logical pixels the viewport is measured in. Its own type
/// rather than any target's `Rect`, so this module stays renderer-neutral.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OverlayRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl OverlayRect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// ⚓️ The degenerate rect a POINT anchor (a right-click position, a pointer-anchored tooltip)
    /// stands for.
    pub const fn point(x: f32, y: f32) -> Self {
        Self { x, y, width: 0.0, height: 0.0 }
    }
}

/// 📐️ React's `resolvePopoverPlacement`, field for field: align the cross axis, place on the
/// requested side, flip to the opposite side when the main axis overflows *and* the flip does not,
/// then clamp both axes inside `collision_padding`.
///
/// `flow` is React's own `rtl` argument (`🗨️Popover/🟦️.tsx:326`, from `useFlow().inline`). It
/// touches exactly ONE line, the inline alignment: React's `align === (rtl ? "end" : "start")`
/// picks the anchor's LEFT edge, everything else the anchor's RIGHT edge minus the content width.
/// Side placement, the vertical alignment, the flip and the clamp are direction-invariant there and
/// here.
pub fn resolve_anchored_placement(anchor: OverlayRect, content_size: (f32, f32), viewport: (f32, f32), placement: AnchoredPlacement, flow: FlowInline) -> ResolvedOverlayPlacement {
    let (content_w, content_h) = content_size;
    let (viewport_w, viewport_h) = viewport;
    let AnchoredPlacement { side, align, side_offset, align_offset, collision_padding, avoid_collisions } = placement;
    let inline_start = match flow {
        FlowInline::Rtl => OverlayAlign::End,
        FlowInline::Ltr => OverlayAlign::Start,
    };
    let aligned_left = if matches!(align, OverlayAlign::Center) {
        anchor.x + (anchor.width - content_w) / 2.0 + align_offset
    } else if align == inline_start {
        anchor.x + align_offset
    } else {
        anchor.x + anchor.width - content_w + align_offset
    };
    let aligned_top = match align {
        OverlayAlign::Center => anchor.y + (anchor.height - content_h) / 2.0 + align_offset,
        OverlayAlign::Start => anchor.y + align_offset,
        OverlayAlign::End => anchor.y + anchor.height - content_h + align_offset,
    };
    let position = |candidate: OverlaySide| match candidate {
        OverlaySide::Top => (aligned_left, anchor.y - content_h - side_offset),
        OverlaySide::Bottom => (aligned_left, anchor.y + anchor.height + side_offset),
        OverlaySide::Left => (anchor.x - content_w - side_offset, aligned_top),
        OverlaySide::Right => (anchor.x + anchor.width + side_offset, aligned_top),
    };
    let overflows = |candidate: OverlaySide, (x, y): (f32, f32)| match candidate {
        OverlaySide::Top => y < collision_padding,
        OverlaySide::Bottom => y + content_h > viewport_h - collision_padding,
        OverlaySide::Left => x < collision_padding,
        OverlaySide::Right => x + content_w > viewport_w - collision_padding,
    };
    let mut resolved_side = side;
    let mut point = position(side);
    if avoid_collisions && overflows(side, point) {
        let flipped = side.opposite();
        let flipped_point = position(flipped);
        if !overflows(flipped, flipped_point) {
            resolved_side = flipped;
            point = flipped_point;
        }
    }
    if avoid_collisions {
        point.0 = point.0.max(collision_padding).min((viewport_w - collision_padding - content_w).max(collision_padding));
        point.1 = point.1.max(collision_padding).min((viewport_h - collision_padding - content_h).max(collision_padding));
    }
    ResolvedOverlayPlacement { side: resolved_side, x: point.0, y: point.1 }
}

/// 🎯️ The centered placement a modal surface resolves to — no anchor, and no flow direction (a
/// scrim's centre is direction-invariant).
pub fn resolve_centered_placement(content_size: (f32, f32), viewport: (f32, f32)) -> ResolvedOverlayPlacement {
    let (content_w, content_h) = content_size;
    let (viewport_w, viewport_h) = viewport;
    ResolvedOverlayPlacement { side: OverlaySide::Bottom, x: ((viewport_w - content_w) / 2.0).max(0.0), y: ((viewport_h - content_h) / 2.0).max(0.0) }
}

/// 📐️ [`resolve_anchored_placement`]/[`resolve_centered_placement`] behind the one
/// [`OverlayPlacement`] switch, so a caller holding a placement value never re-implements the match.
pub fn resolve_overlay_placement(anchor: OverlayRect, content_size: (f32, f32), viewport: (f32, f32), placement: OverlayPlacement, flow: FlowInline) -> ResolvedOverlayPlacement {
    match placement {
        OverlayPlacement::Anchored(anchored) => resolve_anchored_placement(anchor, content_size, viewport, anchored, flow),
        OverlayPlacement::Centered => resolve_centered_placement(content_size, viewport),
    }
}

/// 🔽️ `Select`'s own positioner (`🧱️elements/🔽️Select/🟦️.tsx:253-265`), which does NOT go through
/// `resolvePopoverPlacement`: it resolves the inline edge first (`inlineStart`/`inlineEnd`, both
/// mirrored under RTL) and then picks by `align`. Answers the popup's LEFT edge.
pub fn resolve_select_inline_left(trigger: OverlayRect, content_width: f32, align: OverlayAlign, flow: FlowInline) -> f32 {
    let rtl = matches!(flow, FlowInline::Rtl);
    let inline_start = if rtl { trigger.x + trigger.width - content_width } else { trigger.x };
    let inline_end = if rtl { trigger.x } else { trigger.x + trigger.width - content_width };
    match align {
        OverlayAlign::Center => trigger.x + (trigger.width - content_width) / 2.0,
        OverlayAlign::Start => inline_start,
        OverlayAlign::End => inline_end,
    }
}

//#endregion 🪟️OverlayPlacement

//#region 🪟️OverlayKind

/// ⏱️ Dwell before a hover tooltip opens — React's `CHROME_CONTROL_TOOLTIP_DELAY_MS` (400 ms,
/// `🧱️elements/💡️ChromeControlHint/🟦️.tsx:21`).
pub const TOOLTIP_DWELL_SECONDS: f32 = 0.4;

/// ⏱️ How long a tooltip lingers after the pointer leaves both its anchor and its own bounds.
pub const TOOLTIP_HOVER_OUT_SECONDS: f32 = 0.4;

/// 🏷️ Which of the six floating-surface use-cases is open — drives the default placement rule and
/// the dismissal policy. Renderer-neutral, so the DOM, retained-wgpu and backend-neutral dispatch
/// paths cannot disagree about what a `Tooltip` or a `Dialog` is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum OverlayKind {
    SelectPopup,
    ContextMenu,
    Tooltip,
    /// 🗨️ React's `🧱️elements/🗨️Popover/🟦️.tsx` — a non-modal anchored surface. `PopoverContent`'s
    /// own prop defaults are what [`AnchoredPlacement::POPOVER`] carries.
    #[default]
    Popover,
    Dialog,
    CommandPalette,
}

/// 🚪️ How an open overlay can be dismissed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DismissPolicy {
    /// 👆️ A press landing outside the overlay's subtree closes it and swallows the press (doesn't
    /// fall through to whatever is underneath) — standard popup click-outside semantics.
    pub outside_press_swallow: bool,
    /// ⎋️ `Escape` closes this overlay if it is the topmost open one.
    pub escape_closes: bool,
    /// ⏱️ Tooltip-specific: close this many seconds after the pointer leaves both the anchor and the
    /// overlay's own bounds.
    pub hover_out_delay_seconds: Option<f32>,
}

impl OverlayKind {
    /// 📐️ The placement rule this kind hangs off by default.
    pub const fn default_placement(self) -> OverlayPlacement {
        match self {
            OverlayKind::SelectPopup | OverlayKind::ContextMenu => OverlayPlacement::Anchored(AnchoredPlacement::MENU),
            OverlayKind::Popover => OverlayPlacement::Anchored(AnchoredPlacement::POPOVER),
            OverlayKind::Tooltip => OverlayPlacement::Anchored(AnchoredPlacement::TOOLTIP),
            OverlayKind::Dialog | OverlayKind::CommandPalette => OverlayPlacement::Centered,
        }
    }

    /// 🚪️ This kind's dismissal policy.
    pub const fn dismiss_policy(self) -> DismissPolicy {
        match self {
            OverlayKind::Tooltip => DismissPolicy { outside_press_swallow: false, escape_closes: true, hover_out_delay_seconds: Some(TOOLTIP_HOVER_OUT_SECONDS) },
            _ => DismissPolicy { outside_press_swallow: true, escape_closes: true, hover_out_delay_seconds: None },
        }
    }

    /// 🌫️ Whether React portals this kind behind a full-viewport scrim.
    pub const fn has_backdrop(self) -> bool {
        matches!(self, OverlayKind::Dialog | OverlayKind::CommandPalette)
    }
}

//#endregion 🪟️OverlayKind

#[cfg(test)]
#[path = "../🧪️tests/🔬️overlay-unit/🦀️.rs"]
mod tests;
