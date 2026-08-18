// #region theme
//! 🎨️ Theme colors and metrics for wgpu UI rendering.

use crate::wgpu::geometry::Rect;
use crate::wgpu::presence_bar::{presence_color, PresenceAppearance, PresenceHsl};
use ui_styling::appearance::AppearanceName;
use ui_styling::{
    levels,
    metrics::{chrome as chrome_metrics, dom, typography},
    radii, strokes, ChromePalette, CHROME_DARK, CHROME_LIGHT,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_srgb8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let [lr, lg, lb, la] = ui_styling::color::rgba8_to_linear(r, g, b, a);
        Self::new(lr, lg, lb, la)
    }

    fn from_chrome(c: &[f32; 4]) -> Self {
        Self::new(c[0], c[1], c[2], c[3])
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }
}

//#region 🔖️Level
/// 🪜️ The unified 6-level UI surface axis (base..menu, both z-order and glass/shade formula input)
/// — see `ui/styling/🔣️tokens.json`'s `levels` block and `.🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`.
/// Replaces the old unlinked level-name axis (canvas/window/panel/overlay/temporary) plus a
/// separate glass-tier axis (panel/ribbon/menu/windowOptions) with one formula-derived enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Base,
    Window,
    Pane,
    Panel,
    Dialog,
    Menu,
}

impl Level {
    /// 🔢️ Ordinal step `k` (0..=5) every formula-derived value (`Theme::surface`/`glass`)
    /// is computed from — mirrors `ui/styling/rs/🤖️generated.rs`'s `levels::NAMES` ordering.
    pub const fn index(self) -> usize {
        match self {
            Level::Base => 0,
            Level::Window => 1,
            Level::Pane => 2,
            Level::Panel => 3,
            Level::Dialog => 4,
            Level::Menu => 5,
        }
    }
}
//#endregion 🔖️Level

#[derive(Clone, Copy, Debug)]
pub struct GlassStyle {
    pub tint: Rgba,
    pub alpha: f32,
    pub blur_px: f32,
    pub saturate: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub background: Rgba,
    pub panel: Rgba,
    pub panel_border: Rgba,
    pub navbar: Rgba,
    pub text: Rgba,
    pub text_muted: Rgba,
    pub accent: Rgba,
    pub accent_hover: Rgba,
    pub active_foreground: Rgba,
    pub button: Rgba,
    pub button_hover: Rgba,
    pub input_bg: Rgba,
    pub separator: Rgba,
    pub selected: Rgba,
    pub canvas_clear: Rgba,
    pub temporary: Rgba,
    pub gap_standard: f32,
    pub padding_standard: f32,
    pub navbar_height: f32,
    pub panel_header_height: f32,
    pub control_height: f32,
    pub control_height_small: f32,
    pub glass_saturate: f32,
    pub font_size_body: f32,
    pub font_size_small: f32,
    pub font_size_emphasized: f32,
    pub footer_height: f32,
    pub panel_inset: f32,
    pub panel_min_width: f32,
    pub panel_max_width: f32,
    pub window_measures_default_width: f32,
    pub window_engagement_max_width: f32,
    pub overlay_shadow: Rgba,
    pub focus_ring: Rgba,
    pub row_hover: Rgba,
    pub border_radius: f32,
    pub border_normal: Rgba,
    pub border_emphasized: Rgba,
    pub text_element: Rgba,
    pub stroke_hairline: f32,
    pub checker_light: Rgba,
    pub checker_dark: Rgba,
    pub diagram_stroke: Rgba,
    pub diagram_seam: Rgba,
    pub diagram_accent: Rgba,
    pub diagram_accent_fill: Rgba,
    pub error: Rgba,
    /// 🪜️ Plain per-level fill, indexed by `Level::index` — `ui-surface`'s wgpu counterpart, backing
    /// `Theme::surface`/`glass`. Populated from the generated `levelBase..levelMenu`
    /// chrome paints (see `from_chrome` below).
    pub level_bg: [Rgba; 6],
    /// 🎨️ The 12 base-cycle (`k = index / 12 == 0`) session-color swatches for this theme's appearance
    /// (contract freeze §C7.5), indexed by `index % 12` — filled from `ui_styling::presence` via
    /// `presence_bar::presence_color`. See [`Theme::presence_color`].
    pub presence: [Rgba; 12],
    /// 🎨️ The local user's own hub-assigned palette index, if any — `None` before the hub's
    /// `ServerFrame::Session` handshake or for a folder-only session with no hub connection.
    pub local_presence: Option<u8>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

fn chrome_px(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

fn panel_width(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

//#region 🔖️Presence
/// 🎨️ HSL (`h` degrees, `s`/`l` `[0, 1]`) → sRGB8888, standard sector conversion.
fn hsl_to_srgb8(h: u16, s: f64, l: f64) -> (u8, u8, u8) {
    let h = f64::from(h).rem_euclid(360.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r1, g1, b1) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let to_u8 = |v: f64| ((v + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    (to_u8(r1), to_u8(g1), to_u8(b1))
}

fn presence_rgba(hsl: PresenceHsl) -> Rgba {
    let (r, g, b) = hsl_to_srgb8(hsl.h, hsl.s, hsl.l);
    Rgba::from_srgb8(r, g, b, 255)
}
//#endregion 🔖️Presence

fn from_chrome(chrome: &ChromePalette, presence_appearance: PresenceAppearance) -> Theme {
    Theme {
        background: Rgba::from_chrome(&chrome.base),
        panel: Rgba::from_chrome(&chrome.level_panel),
        panel_border: Rgba::from_chrome(&chrome.border_normal),
        navbar: Rgba::from_chrome(&chrome.level_window),
        text: Rgba::from_chrome(&chrome.foreground),
        text_muted: Rgba::from_chrome(&chrome.muted_foreground),
        accent: Rgba::from_chrome(&chrome.accent),
        accent_hover: Rgba::from_chrome(&chrome.active_hover),
        active_foreground: Rgba::from_chrome(&chrome.active_foreground),
        button: Rgba::from_chrome(&chrome.level_window),
        button_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
        input_bg: Rgba::from_chrome(&chrome.base),
        separator: Rgba::from_chrome(&chrome.border_normal),
        selected: Rgba::from_chrome(&chrome.active_base),
        canvas_clear: Rgba::from_chrome(&chrome.base),
        temporary: Rgba::from_chrome(&chrome.level_menu),
        gap_standard: chrome_px(chrome_metrics::GAP_STANDARD_UI_SPACING),
        padding_standard: chrome_px(chrome_metrics::PADDING_STANDARD_UI_SPACING),
        navbar_height: chrome_px(chrome_metrics::NAVBAR_HEIGHT_UI_SPACING),
        panel_header_height: chrome_px(chrome_metrics::PANEL_HEADER_HEIGHT_UI_SPACING),
        control_height: chrome_px(chrome_metrics::CONTROL_HEIGHT_UI_SPACING),
        control_height_small: chrome_px(5.0),
        glass_saturate: levels::GLASS_SATURATE as f32,
        font_size_body: typography::TEXT_SM_PX as f32,
        font_size_small: typography::TEXT_XS_PX as f32,
        font_size_emphasized: typography::TEXT_BASE_PX as f32,
        footer_height: chrome_px(chrome_metrics::FOOTER_HEIGHT_UI_SPACING),
        panel_inset: chrome_px(chrome_metrics::PANEL_INSET_UI_SPACING),
        panel_min_width: panel_width(dom::LAYOUT_PANEL_MIN_UI_SPACING),
        panel_max_width: panel_width(dom::LAYOUT_PANEL_MAX_UI_SPACING),
        window_measures_default_width: chrome_px(dom::LAYOUT_PANEL_RAIL_UI_SPACING),
        window_engagement_max_width: chrome_px(dom::LAYOUT_ENGAGEMENT_MAX_UI_SPACING),
        overlay_shadow: Rgba::new(0.0, 0.0, 0.0, 0.0),
        focus_ring: Rgba::from_chrome(&chrome.accent).with_alpha(0.6),
        row_hover: Rgba::from_chrome(&chrome.hover_interactive_fill),
        border_radius: radii::CHROME as f32,
        border_normal: Rgba::from_chrome(&chrome.border_normal),
        border_emphasized: Rgba::from_chrome(&chrome.border_emphasized),
        text_element: Rgba::from_chrome(&chrome.border_element),
        stroke_hairline: strokes::CHROME_BORDER_HAIRLINE as f32,
        checker_light: Rgba::new(0.85, 0.85, 0.85, 1.0),
        checker_dark: Rgba::new(0.72, 0.72, 0.72, 1.0),
        diagram_stroke: Rgba::new(0.2, 0.55, 0.95, 0.95),
        diagram_seam: Rgba::new(0.95, 0.45, 0.2, 0.95),
        diagram_accent: Rgba::new(0.25, 0.45, 0.65, 0.9),
        diagram_accent_fill: Rgba::new(0.25, 0.35, 0.55, 0.8),
        error: Rgba::new(0.95, 0.35, 0.35, 1.0),
        level_bg: [
            Rgba::from_chrome(&chrome.level_base),
            Rgba::from_chrome(&chrome.level_window),
            Rgba::from_chrome(&chrome.level_pane),
            Rgba::from_chrome(&chrome.level_panel),
            Rgba::from_chrome(&chrome.level_dialog),
            Rgba::from_chrome(&chrome.level_menu),
        ],
        presence: std::array::from_fn(|i| presence_rgba(presence_color(i as u8, presence_appearance))),
        local_presence: None,
    }
}

impl Theme {
    pub fn light() -> Self {
        from_chrome(&CHROME_LIGHT, PresenceAppearance::Light)
    }

    pub fn dark() -> Self {
        from_chrome(&CHROME_DARK, PresenceAppearance::Dark)
    }

    pub fn for_name(name: AppearanceName) -> Self {
        match name {
            AppearanceName::Light => Self::light(),
            AppearanceName::Dark => Self::dark(),
        }
    }

    //#region 🔖️LevelSurfaces
    /// 🪜️ Plain per-level fill (no blur/alpha) — `ui-surface`'s wgpu counterpart.
    pub fn surface(&self, level: Level) -> Rgba {
        self.level_bg[level.index()]
    }

    /// 🧊️ Formula-derived glass style for `level` — `ui-glass`'s wgpu counterpart. Alpha steps down
    /// and blur steps up per level index (`ui/styling/🔣️tokens.json`'s `levels` block:
    /// `alpha(k) = 1 - k * glassAlphaStep`, `blur(k) = k * glassBlurStepPx`), read from
    /// `ui_styling::levels` constants — never a per-tier lookup table.
    pub fn glass(&self, level: Level) -> GlassStyle {
        let k = level.index() as f32;
        GlassStyle { tint: self.level_bg[level.index()], alpha: 1.0 - k * levels::GLASS_ALPHA_STEP as f32, blur_px: k * levels::GLASS_BLUR_STEP_PX as f32, saturate: self.glass_saturate }
    }

    //#endregion 🔖️LevelSurfaces

    //#region 🔖️Presence
    /// 🎨️ Resolves a peer's palette index to this theme's appearance (contract freeze §C7.5), by
    /// `index % 12` into the pre-resolved base-cycle swatches. `Theme` is appearance-specific
    /// (`light()`/`dark()`), so the swatches are already correct for whichever `self` is; the full
    /// per-cycle desaturate/lighten shift for `index / 12 >= 1` is `presence_bar::presence_color`'s job
    /// for callers that need it directly.
    pub fn presence_color(&self, index: u8) -> Rgba {
        self.presence[(index % 12) as usize]
    }
    //#endregion 🔖️Presence

    pub fn glass_mip_level(blur_px: f32, max_mip: u32) -> f32 {
        (blur_px / 4.0).log2().max(0.0).min(max_mip as f32)
    }
}

pub type ThemedRect = Rect;

// #endregion theme
