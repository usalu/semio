//! 🃏️ Height snapping of a card stack, matching `bands.ts` and the language-neutral `bands.cases.json`.

/// 📐️ The smallest height at or above `height` that is a whole number of device pixels (`pixel` is one device pixel in logical px); mirrors `snapCardHeight`.
///
/// See `bands.cases.json`.
pub fn snap_card_height(height: f64, pixel: f64) -> f64 {
    (height / pixel - 1e-4).ceil() * pixel
}

#[cfg(test)]
#[path = "bands.test.rs"]
mod tests;
