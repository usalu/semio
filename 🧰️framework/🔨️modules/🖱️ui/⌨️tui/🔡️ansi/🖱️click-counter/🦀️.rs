//! 🖱️ Click counter: the multi-click window and the `ClickCounter`. See [`super`].

use super::*;

//#region 🖱️Clicks
/// 🖱️ Longest gap between two presses that still count as a double or triple click.
pub const CLICK_WINDOW_MS: u64 = 400;

/// 🔢️ Counts consecutive presses of one button on one cell and stamps `MouseEvent::clicks`.
#[derive(Default)]
pub struct ClickCounter {
    pub(super) last: Option<(MouseButton, Pos, u64)>,
    pub(super) count: u8,
}

impl ClickCounter {
    /// 🏷️ Sets `clicks` on one mouse event observed at `now_ms` (monotonic milliseconds).
    pub fn stamp(&mut self, event: &mut MouseEvent, now_ms: u64) {
        match event.kind {
            MouseKind::Down(button) => {
                let repeated = self.last.is_some_and(|(last_button, last_pos, last_ms)| last_button == button && last_pos == event.pos && now_ms.saturating_sub(last_ms) <= CLICK_WINDOW_MS);
                self.count = if repeated && self.count < 3 { self.count + 1 } else { 1 };
                self.last = Some((button, event.pos, now_ms));
                event.clicks = self.count;
            }
            MouseKind::Up(_) | MouseKind::Drag(_) => event.clicks = self.count.max(1),
            MouseKind::Move | MouseKind::Scroll { .. } => event.clicks = 1,
        }
    }

    /// 🧷️ Stamps every mouse event of a decoded batch.
    pub fn stamp_all(&mut self, events: &mut [Event], now_ms: u64) {
        for event in events {
            if let Event::Mouse(mouse) = event {
                self.stamp(mouse, now_ms);
            }
        }
    }
}
//#endregion 🖱️Clicks
