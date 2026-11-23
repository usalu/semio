//! 🃏️ Native card stack: every card is one rectangular frame and frames abut on shared edges (no card is lifted onto a neighbour). A card's title and footer chips are glass overlays inside the frame; the frame outline and each chip's inner edges are painted as shared-border segments, so every coincident line is one line.
//! @see `🧱️elements/🃏️CardStack/🟦️.tsx`
//! @see `🧱️elements/🔲️WindowFrame/🟦️.tsx`
//! @see `🧱️elements/🃏️CardStack/stack.cases.json`

use semio_ui_draw_contract::{DrawList, SharedBorderState};
use semio_ui_theme::geometry::Rect;
use semio_ui_theme::{GlassStyle, Rgba};

const EPSILON: f64 = 1e-6;

/// 🃏️ A card of a stack: its full height, the height of its chip rows and the widths of its title (top, left) and footer (bottom, right) chips; a width of zero means no chip.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardStackCard {
    pub height: f64,
    pub row: f64,
    pub top_chip: f64,
    pub bottom_chip: f64,
}

/// 📍️ Which frame edge a chip is anchored to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardStackDock {
    Top,
    Bottom,
}

/// 💠️ A chip overlay inside a frame as `[x, y, width, height]`; `inner_edges` are its `[top, right, bottom, left]` edges that do not lie on the frame outline.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardStackChip {
    pub rect: [f64; 4],
    pub dock: CardStackDock,
    pub inner_edges: [bool; 4],
}

/// 🔲️ One card as the rectangle `rect` (`[x, y, width, height]`) and the chips inside it.
#[derive(Clone, Debug, PartialEq)]
pub struct CardStackFrame {
    pub rect: [f64; 4],
    pub chips: Vec<CardStackChip>,
}

fn chip(frame: [f64; 4], dock: CardStackDock, width: f64, row: f64) -> Option<CardStackChip> {
    let width = width.min(frame[2]);
    let height = row.min(frame[3]);
    if width <= EPSILON || height <= EPSILON {
        return None;
    }
    let (x, y) = match dock {
        CardStackDock::Top => (frame[0], frame[1]),
        CardStackDock::Bottom => (frame[0] + frame[2] - width, frame[1] + frame[3] - height),
    };
    let touches_right = x + width >= frame[0] + frame[2] - EPSILON;
    let touches_left = x <= frame[0] + EPSILON;
    let spans_height = height >= frame[3] - EPSILON;
    let inner_edges = match dock {
        CardStackDock::Top => [false, !touches_right, !spans_height, !touches_left],
        CardStackDock::Bottom => [!spans_height, !touches_right, false, !touches_left],
    };
    Some(CardStackChip { rect: [x, y, width, height], dock, inner_edges })
}

/// 🧱️ Lays out `cards` in one column whose first frame sits at `left`/`top` with `width`; frame `i + 1` starts at the bottom edge of frame `i`.
pub fn card_stack_frames(left: f64, top: f64, width: f64, cards: &[CardStackCard]) -> Vec<CardStackFrame> {
    let mut y = top;
    cards
        .iter()
        .map(|card| {
            let rect = [left, y, width, card.height];
            y += card.height;
            let chips = [chip(rect, CardStackDock::Top, card.top_chip, card.row), chip(rect, CardStackDock::Bottom, card.bottom_chip, card.row)].into_iter().flatten().collect();
            CardStackFrame { rect, chips }
        })
        .collect()
}

/// 📍️ Whether the point lies inside the frame rectangle; the space beside a chip is content.
pub fn card_frame_contains(frame: &CardStackFrame, x: f64, y: f64) -> bool {
    x >= frame.rect[0] && x <= frame.rect[0] + frame.rect[2] && y >= frame.rect[1] && y <= frame.rect[1] + frame.rect[3]
}

/// 🔢️ The retained output items [`paint_card_stack`] records for `frames`: four lines per frame, one per chip inner edge and two per chip glass.
pub fn card_stack_paint_items(frames: &[CardStackFrame]) -> usize {
    frames.iter().map(|frame| 4 + frame.chips.iter().map(|chip| chip.inner_edges.iter().filter(|edge| **edge).count() + 2).sum::<usize>()).sum()
}

/// 🖌️ Records every frame outline as a shared border (the seam of two abutting frames resolves to ONE painted line), then for every chip its glass surface and its inner edges.
pub fn paint_card_stack(draw: &mut DrawList, frames: &[CardStackFrame], stroke: f32, chip_glass: GlassStyle, border: impl Fn(usize) -> (Rgba, SharedBorderState)) {
    for (index, frame) in frames.iter().enumerate() {
        let (color, state) = border(index);
        let [x, y, w, h] = frame.rect.map(|value| value as f32);
        draw.push_shared_border(Rect::new(x, y, w, h), stroke, color, state);
        for chip in &frame.chips {
            let [x, y, w, h] = chip.rect.map(|value| value as f32);
            draw.push_glass([x, y, w, h], 0.0, chip_glass);
            draw.push_shared_border_edges(Rect::new(x, y, w, h), stroke, color, state, chip.inner_edges);
        }
    }
}

#[cfg(test)]
#[path = "../../stack.test.rs"]
mod tests;
