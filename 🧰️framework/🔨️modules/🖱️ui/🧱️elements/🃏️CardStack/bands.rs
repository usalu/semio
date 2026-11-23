//! 🃏️ Chip seams and height snapping of a card stack, matching `bands.ts` and the language-neutral `bands.cases.json`.

/// 📏️ Chips closer than this (but not abutting) read as two lines with a gap between them; mirrors `CARD_BAND_MIN_GAP`.
pub const CARD_BAND_MIN_GAP: f64 = 40.0;
const ABUT: f64 = 0.25;

/// 🫸️ Whether two chips of one row separated by `gap` read as two lines with a small gap (closer than `CARD_BAND_MIN_GAP`, not abutting); mirrors `chipGapReadsAsSliver`.
///
/// See `bands.cases.json`.
pub fn chip_gap_reads_as_sliver(gap: f64) -> bool {
    gap > ABUT && gap < CARD_BAND_MIN_GAP
}

/// 📐️ The smallest height at or above `height` that is a whole number of device pixels (`pixel` is one device pixel in logical px); mirrors `snapCardHeight`.
///
/// See `bands.cases.json`.
pub fn snap_card_height(height: f64, pixel: f64) -> f64 {
    (height / pixel - 1e-4).ceil() * pixel
}

#[cfg(test)]
#[path = "bands.test.rs"]
mod tests;
