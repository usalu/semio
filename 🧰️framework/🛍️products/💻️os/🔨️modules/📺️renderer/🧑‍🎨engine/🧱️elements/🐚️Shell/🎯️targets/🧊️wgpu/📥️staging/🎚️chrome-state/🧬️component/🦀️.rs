//! 🎚️ `shell-chrome-state` component structs: `ChromeState` (RENDERER-WP4 step T1).
//!
//! Fields are cut from the former `ShellState` in declaration order and keep their names.

use super::*;

/// 🎚️ `ChromeState`: transient chrome: errors, screen metrics, fullscreen, the per-frame build and present cursors.
///
/// Fields keep the names they had on the former `ShellState`; the composer owns one value of this struct as `chrome`.
pub struct ChromeState {
    pub error: Option<String>,
    /// 📐️ The content root, in LOGICAL (CSS) pixels — the SAME unit as every chrome constant
    /// (`chrome_px`, `navbar_height`, `footer_height`, `control_height`, the resize-handle hit
    /// widths, every font size), which are read verbatim from the generated `ui_styling` tokens
    /// React's DOM consumes. The surface scale factor never reaches this pair: it sizes the GPU
    /// surface, the projection divisor and the glyph/icon atlas raster, and nothing else. Sizing this
    /// in physical pixels while the constants stayed logical was the HiDPI placement bug — ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g (`📓️w1g-dpi-logical-units.md`).
    pub screen_w: f32,
    pub screen_h: f32,
    pub fullscreen_toggle_requested: bool,
    pub fullscreen_active: bool,
    pub presented_chrome_accessibility: Vec<ui_contract::AccessibilityProjectionNode>,
    /// ♿️ The presented epoch at which `presented_chrome_accessibility` last CHANGED — the generation the
    /// ARIA mirror addresses the chrome by. It advances with the projection, not with every presented
    /// frame, so a mirror that lags one idle frame still addresses the live chrome.
    pub presented_chrome_accessibility_generation: u64,
    /// 🏠️ The surface scope the chrome walk has established for this frame, read by
    /// [`Self::render_main_window_step`] through `ui_wgpu::wgpu::shell_floor_paints` — React's
    /// `shellFloorPaints` (`🔨️modules/🏠️shell-floor-presentation/🟦️.ts:14`). `FrameSetup` fills the
    /// whole surface at [`Level::Base`], which makes the main window's own identical base fill a
    /// second paint of the same colour over the same pixels; React suppresses exactly that.
    pub chrome_floor_scope: Option<semio_framework_ui_theme::SurfaceScope>,
    /// 🪪️ Who issued the dispatch currently crossing [`Self::dispatch_action`], for the chrome
    /// ledger's `origin` column — React's `InputOriginV1`. A chrome press, a keybinding, a palette
    /// entry and a panel row are all `user`; a pointer/camera stream an engine surface derived from
    /// raw input is `gesture`, which is the one origin React never turns into a notice
    /// (`🏛️ShellHost/🎯️input-ledger/🟦️.ts`: "a 60 Hz `setCamera` that the guest refuses would
    /// otherwise toast once per throttle window for as long as the user orbits"). Set by
    /// [`Self::dispatch_gesture_action`] for the frame's deferred surface actions and nothing else.
    pub dispatch_origin: &'static str,
    pub chrome_build: ShellChromeBuildState,
    pub chrome_present: ShellChromePresentState,
}
