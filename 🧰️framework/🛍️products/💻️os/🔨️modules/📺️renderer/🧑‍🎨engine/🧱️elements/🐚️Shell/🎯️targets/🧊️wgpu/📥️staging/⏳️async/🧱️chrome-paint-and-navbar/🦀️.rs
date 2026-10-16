//! ⏳️ `shell-async` staging of `chrome-paint-and-navbar`: RENDERER-INTEGRATION-2, the items the direction graph homes in this crate (they were in a higher crate or in the old chunk file).

use super::*;

#[cfg(any(test, feature = "testkit"))]
pub fn chrome_group_border(draw: &mut DrawList, rect: Rect, theme: &Theme) {
    push_chrome_group_border(draw, rect, theme);
}
