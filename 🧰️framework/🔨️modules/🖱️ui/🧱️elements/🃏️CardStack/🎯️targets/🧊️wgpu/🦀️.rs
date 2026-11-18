//! 🃏️ Native card stack: cards are placed with [`resolve_card_bands`] (the next card's cap row starts on the previous card's bottom line, sharing one band with its footer row) and every card's silhouette is painted as shared-border segments, so each coincident line is one line.
//! @see `🧱️elements/🃏️CardStack/🟦️.tsx`
//! @see `🧱️elements/🔲️WindowSilhouette/🟦️.tsx`

use crate::card_bands::{resolve_card_bands, CardBandCard, CardBandMode, CardBandSpan};
use semio_ui_draw_contract::{DrawList, SharedBorderState};
use semio_ui_theme::geometry::Rect;
use semio_ui_theme::Rgba;

const EPSILON: f64 = 1e-6;

/// 📏️ One axis-aligned piece of a card outline in the stack's coordinate space.
pub type CardStackSegment = ([f64; 2], [f64; 2]);

/// 🪢️ A card's cap or footer row: its natural height and the extents of its chips relative to the card's left edge.
#[derive(Clone, Debug, PartialEq)]
pub struct CardStackEdge {
    pub depth: f64,
    pub chips: Vec<CardBandSpan>,
}

/// 🃏️ A card of a stack: its full height and its cap (top) and footer (bottom) rows.
#[derive(Clone, Debug, PartialEq)]
pub struct CardStackCard {
    pub height: f64,
    pub top: CardStackEdge,
    pub bottom: CardStackEdge,
}

/// 📍️ Where a card sits: its top, and the depths its cap and footer rows end up with after the shared bands grew them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardStackPlacement {
    pub top: f64,
    pub height: f64,
    pub top_depth: f64,
    pub bottom_depth: f64,
}

/// 🧮️ Places `cards` in one column with no gap: card `i + 1` is lifted onto the bottom line of card `i` by its band, and a shared band grows the cap row below and the footer row above to the same height.
pub fn layout_card_stack(cards: &[CardStackCard]) -> Vec<CardStackPlacement> {
    let bands = resolve_card_bands(&cards.iter().map(|card| CardBandCard { cap: card.top.depth, footer: card.bottom.depth, cap_chips: card.top.chips.clone(), footer_chips: card.bottom.chips.clone() }).collect::<Vec<_>>());
    let mut placements: Vec<CardStackPlacement> = Vec::with_capacity(cards.len());
    for (index, card) in cards.iter().enumerate() {
        let top = placements.last().zip(bands.get(index.wrapping_sub(1))).map_or(0.0, |(above, band)| above.top + above.height - band.lift);
        let mut placement = CardStackPlacement { top, height: card.height, top_depth: card.top.depth, bottom_depth: card.bottom.depth };
        if let (Some(above), Some(band)) = (placements.last_mut(), bands.get(index.wrapping_sub(1))) {
            if band.mode == CardBandMode::Shared {
                above.bottom_depth = above.bottom_depth.max(band.footer);
                placement.top_depth = placement.top_depth.max(band.cap);
            }
        }
        placements.push(placement);
    }
    placements
}

fn normalized(chips: &[CardBandSpan], x0: f64, x1: f64) -> Vec<CardBandSpan> {
    let mut spans: Vec<CardBandSpan> = chips.iter().map(|chip| CardBandSpan { left: chip.left.max(x0), right: chip.right.min(x1) }).filter(|chip| chip.right - chip.left > EPSILON).collect();
    spans.sort_by(|a, b| a.left.total_cmp(&b.left));
    let mut merged: Vec<CardBandSpan> = Vec::with_capacity(spans.len());
    for span in spans {
        match merged.last_mut() {
            Some(last) if span.left <= last.right + EPSILON => last.right = last.right.max(span.right),
            _ => merged.push(span),
        }
    }
    merged
}

fn push_point(points: &mut Vec<[f64; 2]>, x: f64, y: f64) {
    if points.last().is_none_or(|last| (last[0] - x).abs() > EPSILON || (last[1] - y).abs() > EPSILON) {
        points.push([x, y]);
    }
}

fn edge_points(depth: f64, chips: &[CardBandSpan], x0: f64, x1: f64, outer: f64, inner: f64) -> Vec<[f64; 2]> {
    let chips = normalized(chips, x0, x1);
    if depth <= EPSILON {
        return vec![[x0, outer], [x1, outer]];
    }
    if chips.is_empty() {
        return vec![[x0, inner], [x1, inner]];
    }
    let starts_with_chip = chips[0].left <= x0 + EPSILON;
    let mut points = vec![[x0, if starts_with_chip { outer } else { inner }]];
    if !starts_with_chip {
        push_point(&mut points, chips[0].left, inner);
        push_point(&mut points, chips[0].left, outer);
    }
    for (index, chip) in chips.iter().enumerate() {
        push_point(&mut points, chip.right, outer);
        if let Some(next) = chips.get(index + 1) {
            push_point(&mut points, chip.right, inner);
            push_point(&mut points, next.left, inner);
            push_point(&mut points, next.left, outer);
        } else if chip.right < x1 - EPSILON {
            push_point(&mut points, chip.right, inner);
            push_point(&mut points, x1, inner);
        }
    }
    points
}

/// 🔲️ The stepped outline of one card at `left`/`placement.top`: its top edge dips to the cap depth between chips, its bottom edge rises to the footer depth between chips.
pub fn card_outline(left: f64, width: f64, placement: CardStackPlacement, card: &CardStackCard) -> Vec<CardStackSegment> {
    let top_depth = placement.top_depth.min(placement.height);
    let bottom_depth = placement.bottom_depth.min((placement.height - top_depth).max(0.0));
    let bottom = placement.height;
    let mut points = edge_points(top_depth, &card.top.chips, 0.0, width, 0.0, top_depth);
    let mut lower = edge_points(bottom_depth, &card.bottom.chips, 0.0, width, bottom, bottom - bottom_depth);
    lower.reverse();
    points.append(&mut lower);
    let moved: Vec<[f64; 2]> = points.iter().map(|point| [point[0] + left, point[1] + placement.top]).collect();
    moved.iter().zip(moved.iter().cycle().skip(1)).filter(|(from, to)| (from[0] - to[0]).abs() > EPSILON || (from[1] - to[1]).abs() > EPSILON).map(|(from, to)| (*from, *to)).collect()
}

/// 🧱️ Lays out `cards` in a column whose first card sits at `left`/`top` with `width`, one outline per card.
pub fn card_stack_outlines(left: f64, top: f64, width: f64, cards: &[CardStackCard]) -> Vec<Vec<CardStackSegment>> {
    layout_card_stack(cards).into_iter().zip(cards).map(|(placement, card)| card_outline(left, width, CardStackPlacement { top: placement.top + top, ..placement }, card)).collect()
}

fn ordered(from: [f64; 2], to: [f64; 2]) -> ([f64; 2], [f64; 2]) {
    if from[0] + from[1] <= to[0] + to[1] { (from, to) } else { (to, from) }
}

/// 🖌️ Records every outline segment as a shared border: a line lies inside the stack's bounds (its top and left lines on the first pixel row and column, every other on the one before its coordinate), so coincident lines of neighbouring cards resolve to ONE painted line.
pub fn paint_card_stack(draw: &mut DrawList, outlines: &[Vec<CardStackSegment>], stroke: f32, color: Rgba, state: SharedBorderState) {
    let all = || outlines.iter().flatten().flat_map(|(from, to)| [from, to]);
    let (top, left) = (all().map(|point| point[1]).fold(f64::INFINITY, f64::min), all().map(|point| point[0]).fold(f64::INFINITY, f64::min));
    for (from, to) in outlines.iter().flatten() {
        let (from, to) = ordered(*from, *to);
        let (x, y) = (from[0] as f32, from[1] as f32);
        if (from[1] - to[1]).abs() <= EPSILON {
            let width = (to[0] - from[0]) as f32;
            if from[1] - top <= EPSILON {
                draw.push_shared_border_edges(Rect::new(x, y, width, stroke), stroke, color, state, [true, false, false, false]);
            } else {
                draw.push_shared_border_edges(Rect::new(x, y - stroke, width, stroke), stroke, color, state, [false, false, true, false]);
            }
        } else {
            let height = (to[1] - from[1]) as f32;
            if from[0] - left <= EPSILON {
                draw.push_shared_border_edges(Rect::new(x, y, stroke, height), stroke, color, state, [false, false, false, true]);
            } else {
                draw.push_shared_border_edges(Rect::new(x - stroke, y, stroke, height), stroke, color, state, [false, true, false, false]);
            }
        }
    }
}

#[cfg(test)]
#[path = "../../stack.test.rs"]
mod tests;
