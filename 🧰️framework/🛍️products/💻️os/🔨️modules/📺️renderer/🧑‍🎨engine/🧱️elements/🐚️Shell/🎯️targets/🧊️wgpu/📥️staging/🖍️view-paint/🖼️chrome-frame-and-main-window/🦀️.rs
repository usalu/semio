//! 🖍 `shell-view-paint` staging of `chrome-frame-and-main-window`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
/// 🧭️ Fixed 64-deep footer-utility tree cursor path. A newtype rather than a bare `[u16; 64]`
/// field because std implements `Default` for arrays only up to length 32, and
/// `ShellChromeChildCursor` must stay derivable — the bound is on the array, not on the cursor.
pub(super) struct ShellUtilityPath([u16; 64]);

impl Default for ShellUtilityPath {
    fn default() -> Self {
        Self([0; 64])
    }
}
//#endregion ShellChrome
