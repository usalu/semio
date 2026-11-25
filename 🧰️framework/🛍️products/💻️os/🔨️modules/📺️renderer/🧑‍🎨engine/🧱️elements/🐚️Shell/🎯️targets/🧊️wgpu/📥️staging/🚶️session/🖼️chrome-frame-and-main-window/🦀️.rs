//! 🚶️ `shell-session` staging of `chrome-frame-and-main-window`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
/// 🧱️ The rectangles of `shell-bands/v1` (`🔝️Navbar/bands.cases.json`): the canvas is the whole client, the navbar and footer are glass bands above it, the body is the canvas minus the bands' height, which docks and scrollers keep as safe clearance, and the line a band shares with the body belongs to the band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShellBandLayout {
    pub canvas: Rect,
    pub body: Rect,
    pub navbar: Rect,
    pub footer: Rect,
    pub navbar_line: Rect,
    pub footer_line: Rect,
}

/// ➖️ The navbar's bottom line: inside the band, painted by the band.
pub fn shell_navbar_line_rect(width: f32, theme: &Theme) -> Rect {
    Rect::new(0.0, theme.navbar_height - theme.stroke_hairline, width, theme.stroke_hairline)
}

/// ➖️ The footer's top line at the band's top edge `y`: inside the band, painted by the band.
pub fn shell_footer_line_rect(y: f32, width: f32, theme: &Theme) -> Rect {
    Rect::new(0.0, y, width, theme.stroke_hairline)
}

/// 🎯️ What a point of the client belongs to: a band wins every point inside it, the body is the rest of the safe area.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellBandHit {
    Navbar,
    Footer,
    Body,
}

/// 📐️ Solves the bands of one client from the theme's band heights, the tutorial bar above the body and the sub-footer under the footer.
pub fn shell_band_layout(width: f32, height: f32, navbar_height: f32, tutorial_reserve: f32, footer_height: f32, subfooter_height: f32, stroke: f32) -> ShellBandLayout {
    let top = navbar_height + tutorial_reserve;
    let footer_y = height - footer_height - subfooter_height;
    ShellBandLayout {
        canvas: Rect::new(0.0, 0.0, width, height),
        body: Rect::new(0.0, top, width, footer_y - top),
        navbar: Rect::new(0.0, 0.0, width, navbar_height),
        footer: Rect::new(0.0, footer_y, width, footer_height),
        navbar_line: Rect::new(0.0, navbar_height - stroke, width, stroke),
        footer_line: Rect::new(0.0, footer_y, width, stroke),
    }
}

/// 🎯️ The band-first hit rule: navbar, then footer, then the body.
pub fn shell_band_hit(layout: &ShellBandLayout, x: f32, y: f32) -> Option<ShellBandHit> {
    if layout.navbar.contains(x, y) {
        Some(ShellBandHit::Navbar)
    } else if layout.footer.contains(x, y) {
        Some(ShellBandHit::Footer)
    } else {
        layout.body.contains(x, y).then_some(ShellBandHit::Body)
    }
}

impl<'ctx> SessionCtx<'ctx> {
    pub fn band_layout(&self, theme: &Theme) -> ShellBandLayout {
        shell_band_layout(self.chrome.screen_w, self.chrome.screen_h, theme.navbar_height, self.tutorial.tutorial_bar_reserve(theme), theme.footer_height, self.subfooter_height(theme), theme.stroke_hairline)
    }

    pub fn canvas_rect(&self, theme: &Theme) -> Rect {
        self.band_layout(theme).canvas
    }

    pub fn body_rect(&self, theme: &Theme) -> Rect {
        self.band_layout(theme).body
    }
}

//#endregion ShellChrome

