//! 🃏️ Interlock bands of a card stack, matching `bands.ts` and the language-neutral `bands.cases.json`.

/// 📏️ A horizontal extent of a chip in one coordinate space shared by the cards of a stack.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardBandSpan {
    pub left: f64,
    pub right: f64,
}

/// 🃏️ Natural heights of a card's title-chip row and footer-chip row and the extents of the chips in them.
#[derive(Clone, Debug, PartialEq)]
pub struct CardBandCard {
    pub cap: f64,
    pub footer: f64,
    pub cap_chips: Vec<CardBandSpan>,
    pub footer_chips: Vec<CardBandSpan>,
}

/// 🪢️ How a card meets the one before it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardBandMode {
    None,
    Shared,
    Stacked,
}

/// 🪢️ Lift of the lower card and minimum heights of its cap row and of the upper card's footer row.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CardBand {
    pub mode: CardBandMode,
    pub lift: f64,
    pub cap: f64,
    pub footer: f64,
}

/// 📏️ Chips closer than this (but not abutting) read as two lines with a gap between them, so they are stacked; mirrors `CARD_BAND_MIN_GAP`.
pub const CARD_BAND_MIN_GAP: f64 = 40.0;
const ABUT: f64 = 0.25;

fn touch(a: CardBandSpan, b: CardBandSpan) -> bool {
    let gap = (a.left - b.right).max(b.left - a.right);
    gap < -1e-6 || (gap > ABUT && gap < CARD_BAND_MIN_GAP)
}

/// 🧮️ Entry `i` describes card `i + 1` against card `i`: one shared band as high as the taller of footer row and cap row, or two stacked bands when chips would meet horizontally.
pub fn resolve_card_bands(cards: &[CardBandCard]) -> Vec<CardBand> {
    cards
        .windows(2)
        .map(|pair| {
            let (above, below) = (&pair[0], &pair[1]);
            if above.footer <= 0.0 {
                CardBand { mode: CardBandMode::None, lift: 0.0, cap: 0.0, footer: 0.0 }
            } else if above.footer_chips.iter().any(|footer| below.cap_chips.iter().any(|cap| touch(*footer, *cap))) {
                CardBand { mode: CardBandMode::Stacked, lift: 0.0, cap: 0.0, footer: 0.0 }
            } else {
                let band = above.footer.max(below.cap);
                CardBand { mode: CardBandMode::Shared, lift: band, cap: band, footer: band }
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "bands.test.rs"]
mod tests;
