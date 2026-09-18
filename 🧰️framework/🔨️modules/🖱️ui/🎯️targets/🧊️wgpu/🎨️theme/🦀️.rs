// #region theme
//! 🎨️ Theme colors and metrics for wgpu UI rendering.

use crate::wgpu::geometry::Rect;
use crate::wgpu::presence_bar::{presence_color, PresenceAppearance, PresenceHsl};
use ui_styling::appearance::AppearanceName;
use ui_styling::{
    levels,
    metrics::{chrome as chrome_metrics, dom, typography},
    colors, opacities, radii, strokes, ChromePalette, DiagramPalette, OutcomePalette, CHROME_DARK, CHROME_LIGHT, CHROME_MONO_DARK, CHROME_MONO_LIGHT, DIAGRAM_DARK, DIAGRAM_LIGHT,
    DIAGRAM_MONO_DARK, DIAGRAM_MONO_LIGHT, OUTCOME_DARK, OUTCOME_LIGHT, OUTCOME_MONO_DARK, OUTCOME_MONO_LIGHT,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    /// 🫥️ The "paint nothing" identity — zero in every channel, so it is a structural constant
    /// rather than a color, and carries no token.
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn from_srgb8(r: u8, g: u8, b: u8, a: u8) -> Self {
        let [lr, lg, lb, la] = ui_styling::color::rgba8_to_linear(r, g, b, a);
        Self::new(lr, lg, lb, la)
    }

    /// 🎨️ Lifts one generated `ui_styling` paint (already linear-float RGBA, decoded from the
    /// authored sRGB hex by the styling codegen) into a `Rgba`. Every themed color in this target
    /// MUST come through here or [`Rgba::from_srgb8`] — a hand-written channel literal is a drift
    /// bug, and `🧪️tests/🔬️targets-wgpu-theme-token-parity` fails the build on one.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub const fn from_token(c: &[f32; 4]) -> Self {
        Self::new(c[0], c[1], c[2], c[3])
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn with_alpha(self, a: f32) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }
}

//#region 🔖️Level
/// 🪜️ The unified 6-level UI surface axis (base..menu, both z-order and glass/shade formula input)
/// — see `ui/styling/🔣️tokens.json`'s `levels` block and `.🧬semio/🦑️repo/🎫️tickets/26/07/27/UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM/contract.txt`.
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
    /// is computed from — mirrors `ui/styling generated Rust projection`'s `levels::NAMES` ordering.
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
    /// 🔇️ React's `--muted` FILL (`🎨️styling/🖌️ui/🎨️.css:96`/`:582`, `bg-muted`) — the low-emphasis
    /// surface a progress track, a focus-visible stepper segment or a table avatar sits on. Distinct
    /// from `text_muted`, which is `--muted-foreground`.
    pub muted: Rgba,
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
    /// 🌳️ The ONE row pitch every tree presentation lays out, paints and hit-tests on —
    /// `dom.treeRowUiSpacing` (7.5 × `--ui-spacing`), the same `--size-workbench` React's
    /// `Tree` rows carry as `h-workbench`. See `layout::TreeRowMetrics`.
    pub tree_row_height: f32,
    /// 🌳️ Per-level tree indent — `dom.treeIndentPerLevelUiSpacing`, React's own gutter step.
    pub tree_indent_per_level: f32,
    /// 🌳️ Tree expand/collapse gutter width — `dom.treeToggleUiSpacing`.
    pub tree_toggle_width: f32,
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
    /// 🎉️ React's `--stroke-focus` (`🎨️styling/🖌️ui/🎨️.css:771`, three hairlines) — the thick end of
    /// the `celebrate-border-burst` keyframe, and what a `focus-visible:ring-[length:var(--stroke-focus)]`
    /// control rings at.
    pub stroke_focus: f32,
    /// 🎉️ The three conic stops `celebrate-border-spin` cycles through — React's
    /// `--color-primary` → `--color-secondary` → `--color-tertiary` → primary
    /// (`🎨️styling/🖌️ui/🎨️.css:1239-1247`'s `--celebrate-conic`). Until ticket 26/09/17 packet W2k
    /// `Theme` had no triad at all, so a celebrating element painted a STATIC `accent` ring.
    pub celebrate: [Rgba; 3],
    /// 🎉️ React's `--celebrate-border-duration` (1.2 s) — one full spin AND one full thickness burst.
    pub celebrate_duration_seconds: f32,
    pub checker_light: Rgba,
    pub checker_dark: Rgba,
    pub diagram_stroke: Rgba,
    pub diagram_seam: Rgba,
    pub diagram_accent: Rgba,
    pub diagram_accent_fill: Rgba,
    /// ✏️ Default outline for a canvas2d shape layer that declares no stroke colour of its own.
    pub diagram_shape_outline: Rgba,
    /// 📐️ The three FEM result-field hues a canvas2d packet distinguishes (residual / reaction+load /
    /// displacement+mode), taken from the palette so they stay distinguishable in both appearances.
    pub diagram_field_residual: Rgba,
    pub diagram_field_reaction: Rgba,
    pub diagram_field_displacement: Rgba,
    pub error: Rgba,
    /// ✅️ Settled-and-accepted outcome paint (validated commit, passed check) — the positive
    /// counterpart of [`Theme::error`]; renderers take outcome colors from here, never inline.
    pub success: Rgba,
    /// ⏳️ Still-running outcome paint (queued, applying, awaiting a decision) — the neutral third
    /// state between [`Theme::success`] and [`Theme::error`].
    pub progress: Rgba,
    /// ⚠️ Degraded-but-not-failed outcome paint (stalled, retried, partially rejected) — the wgpu
    /// counterpart of the `--color-warning` palette token, sitting between [`Theme::progress`]
    /// and [`Theme::error`].
    pub warning: Rgba,
    /// 🪜️ Plain per-level fill, indexed by `Level::index` — `ui-surface`'s wgpu counterpart, backing
    /// `Theme::surface`/`glass`. Populated from the generated `levelBase..levelMenu`
    /// chrome paints (see `from_chrome` below).
    ///
    /// 🌫️ The fullscreen scrim behind a modal is [`Theme::veil`], not a field — React derives it the
    /// same way (`ui-veil` = `--surface-bg` of a `data-level="dialog"` host at `--veil-alpha`).
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

// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
fn chrome_px(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
fn panel_width(ui_spacing_mult: f64) -> f32 {
    (chrome_metrics::UI_SPACING_COMPACT_PX * ui_spacing_mult) as f32
}

//#region 🔖️Presence
/// 🎨️ HSL (`h` degrees, `s`/`l` `[0, 1]`) → sRGB8888, standard sector conversion.
// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
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

// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
fn presence_rgba(hsl: PresenceHsl) -> Rgba {
    let (r, g, b) = hsl_to_srgb8(hsl.h, hsl.s, hsl.l);
    Rgba::from_srgb8(r, g, b, 255)
}
//#endregion 🔖️Presence

/// 🎨️ Builds one appearance's `Theme` out of the generated `ui_styling` palettes only — the same
/// `🔣️.json` that emits React's `🎨️palette/🎨️.css`. Every field below is either a `Rgba::from_token`
/// lift of a generated paint or a `ui_styling::{metrics, levels, radii, strokes}` constant; nothing
/// here may be a hand-written channel or px literal (law:
/// `🧪️tests/🔬️targets-wgpu-theme-token-parity`).
// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
fn from_chrome(chrome: &ChromePalette, outcome: &OutcomePalette, diagram: &DiagramPalette, presence_appearance: PresenceAppearance) -> Theme {
    Theme {
        background: Rgba::from_token(&chrome.base),
        panel: Rgba::from_token(&chrome.level_panel),
        panel_border: Rgba::from_token(&chrome.border_normal),
        navbar: Rgba::from_token(&chrome.level_window),
        text: Rgba::from_token(&chrome.foreground),
        muted: Rgba::from_token(&chrome.muted),
        text_muted: Rgba::from_token(&chrome.muted_foreground),
        accent: Rgba::from_token(&chrome.accent),
        accent_hover: Rgba::from_token(&chrome.active_hover),
        active_foreground: Rgba::from_token(&chrome.active_foreground),
        button: Rgba::from_token(&chrome.level_window),
        button_hover: Rgba::from_token(&chrome.hover_interactive_fill),
        input_bg: Rgba::from_token(&chrome.base),
        separator: Rgba::from_token(&chrome.border_normal),
        selected: Rgba::from_token(&chrome.active_base),
        canvas_clear: Rgba::from_token(&chrome.base),
        temporary: Rgba::from_token(&chrome.level_menu),
        gap_standard: chrome_px(chrome_metrics::GAP_STANDARD_UI_SPACING),
        padding_standard: chrome_px(chrome_metrics::PADDING_STANDARD_UI_SPACING),
        navbar_height: chrome_px(chrome_metrics::NAVBAR_HEIGHT_UI_SPACING),
        panel_header_height: chrome_px(chrome_metrics::PANEL_HEADER_HEIGHT_UI_SPACING),
        control_height: chrome_px(chrome_metrics::CONTROL_HEIGHT_UI_SPACING),
        control_height_small: chrome_px(chrome_metrics::CONTROL_HEIGHT_SMALL_UI_SPACING),
        tree_row_height: chrome_px(dom::TREE_ROW_UI_SPACING),
        tree_indent_per_level: chrome_px(dom::TREE_INDENT_PER_LEVEL_UI_SPACING),
        tree_toggle_width: chrome_px(dom::TREE_TOGGLE_UI_SPACING),
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
        overlay_shadow: Rgba::TRANSPARENT,
        focus_ring: Rgba::from_token(&chrome.accent).with_alpha(opacities::CHROME_FOCUS_RING_ALPHA as f32),
        row_hover: Rgba::from_token(&chrome.hover_interactive_fill),
        border_radius: radii::CHROME as f32,
        border_normal: Rgba::from_token(&chrome.border_normal),
        border_emphasized: Rgba::from_token(&chrome.border_emphasized),
        text_element: Rgba::from_token(&chrome.border_element),
        stroke_hairline: strokes::CHROME_BORDER_HAIRLINE as f32,
        stroke_focus: strokes::CHROME_BORDER_FOCUS as f32,
        celebrate: [Rgba::from_token(&colors::PRIMARY), Rgba::from_token(&colors::SECONDARY), Rgba::from_token(&colors::TERTIARY)],
        celebrate_duration_seconds: chrome_metrics::CELEBRATE_BORDER_DURATION_SECONDS as f32,
        checker_light: Rgba::from_token(&diagram.checker_light),
        checker_dark: Rgba::from_token(&diagram.checker_dark),
        diagram_stroke: Rgba::from_token(&diagram.stroke),
        diagram_seam: Rgba::from_token(&diagram.seam),
        diagram_accent: Rgba::from_token(&diagram.accent),
        diagram_accent_fill: Rgba::from_token(&diagram.accent_fill),
        diagram_shape_outline: Rgba::from_token(&diagram.shape_outline),
        diagram_field_residual: Rgba::from_token(&diagram.field_residual),
        diagram_field_reaction: Rgba::from_token(&diagram.field_reaction),
        diagram_field_displacement: Rgba::from_token(&diagram.field_displacement),
        error: Rgba::from_token(&outcome.error),
        success: Rgba::from_token(&outcome.success),
        progress: Rgba::from_token(&outcome.progress),
        warning: Rgba::from_token(&outcome.warning),
        level_bg: [
            Rgba::from_token(&chrome.level_base),
            Rgba::from_token(&chrome.level_window),
            Rgba::from_token(&chrome.level_pane),
            Rgba::from_token(&chrome.level_panel),
            Rgba::from_token(&chrome.level_dialog),
            Rgba::from_token(&chrome.level_menu),
        ],
        presence: std::array::from_fn(|i| presence_rgba(presence_color(i as u8, presence_appearance))),
        local_presence: None,
    }
}

//#region 🏠️ShellFloor

/// 🌈️ What a surface scope paints for its level — React's `SurfaceScopeValue.fill`
/// (`🧱️elements/🌈️Surface/🟦️.tsx`). `None` means the scope declares a level but paints nothing, so a
/// descendant floor still has to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceFill {
    None,
    Surface,
    Glass,
    Veil,
}

/// 🌈️ One inherited surface scope: the level a subtree sits at plus what that scope paints. The
/// renderer twin of React's `SurfaceScopeValue`, which `useSurface()` hands to chrome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceScope {
    pub level: Level,
    pub fill: SurfaceFill,
}

/// 🏠️ Whether a base-level floor should paint its own opaque fill, given the scope it is nested in —
/// the port of React's `shellFloorPaints` (`🔨️modules/🏠️shell-floor-presentation/🟦️.ts:14`). A floor
/// enclosed by a scope that is ALREADY at [`Level::Base`] and already painting would paint the same
/// colour over the same pixels, so `Navbar`/`Footer`/`Canvas`/`Skeletons` drop to `bg-transparent`
/// there; a scope at a deeper level, a base scope with [`SurfaceFill::None`], or no scope at all
/// still paints.
// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
pub fn shell_floor_paints(parent: Option<SurfaceScope>) -> bool {
    !parent.is_some_and(|scope| scope.level == Level::Base && scope.fill != SurfaceFill::None)
}

//#endregion 🏠️ShellFloor

impl Theme {
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn light() -> Self {
        from_chrome(&CHROME_LIGHT, &OUTCOME_LIGHT, &DIAGRAM_LIGHT, PresenceAppearance::Light)
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn dark() -> Self {
        from_chrome(&CHROME_DARK, &OUTCOME_DARK, &DIAGRAM_DARK, PresenceAppearance::Dark)
    }

    /// ⚫️ The "mono" premade theme (`🎨️styling/🌓️theme/🔣️.json`), off its OWN generated palettes. It
    /// goes through the same [`from_chrome`] as `light()`/`dark()`, so every derived surface — the
    /// `level*`/`element*` ramp, the metrics, the fonts, the checker, the outcome and diagram hues —
    /// comes from tokens rather than a hand-resolved copy. Until ticket 26/09/17 packet W2k the os
    /// wgpu Shell hand-ported 20 `Rgba::from_srgb8` literals here, because mono's `chrome` group
    /// declared seven keys the default theme lacked and so could not share `ChromePalette`; the two
    /// key sets are now reconciled and the premade is projected like any other palette.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn mono(dark: bool) -> Self {
        if dark {
            from_chrome(&CHROME_MONO_DARK, &OUTCOME_MONO_DARK, &DIAGRAM_MONO_DARK, PresenceAppearance::Dark)
        } else {
            from_chrome(&CHROME_MONO_LIGHT, &OUTCOME_MONO_LIGHT, &DIAGRAM_MONO_LIGHT, PresenceAppearance::Light)
        }
    }

    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn for_name(name: AppearanceName) -> Self {
        match name {
            AppearanceName::Light => Self::light(),
            AppearanceName::Dark => Self::dark(),
        }
    }

    //#region 🔖️LevelSurfaces
    /// 🪜️ Plain per-level fill (no blur/alpha) — `ui-surface`'s wgpu counterpart.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn surface(&self, level: Level) -> Rgba {
        self.level_bg[level.index()]
    }

    /// 🧊️ Formula-derived glass style for `level` — `ui-glass`'s wgpu counterpart. Alpha steps down
    /// and blur steps up per level index (`ui/styling/🔣️tokens.json`'s `levels` block:
    /// `alpha(k) = 1 - k * glassAlphaStep`, `blur(k) = k * glassBlurStepPx`), read from
    /// `ui_styling::levels` constants — never a per-tier lookup table.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn glass(&self, level: Level) -> GlassStyle {
        let k = level.index() as f32;
        GlassStyle { tint: self.level_bg[level.index()], alpha: 1.0 - k * levels::GLASS_ALPHA_STEP as f32, blur_px: k * levels::GLASS_BLUR_STEP_PX as f32, saturate: self.glass_saturate }
    }

    /// 🌫️ Fullscreen modal scrim for `level` — `ui-veil`'s wgpu counterpart. React paints
    /// `color-mix(in srgb, var(--surface-bg) calc(var(--veil-alpha) * 100%), transparent)` over a
    /// host that must carry `data-level="dialog"`, so this is the level's own surface fill at
    /// `levels::VEIL_ALPHA` — never a theme-agnostic black.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn veil(&self, level: Level) -> Rgba {
        self.surface(level).with_alpha(levels::VEIL_ALPHA as f32)
    }

    /// 🌫️ Blur radius `ui-veil` applies behind the scrim — `levels::VEIL_BLUR_PX` (`--veil-blur`).
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn veil_blur_px() -> f32 {
        levels::VEIL_BLUR_PX as f32
    }

    /// 🌫️ `ui-veil` as a GLASS region — the whole utility, not just its fill. React's scrim is
    /// `backdrop-filter: blur(var(--veil-blur)) saturate(var(--glass-saturate))` UNDER
    /// `color-mix(… var(--surface-bg) calc(var(--veil-alpha) * 100%) …)`, so a scrim pushed as a
    /// plain [`Self::veil`] quad is only half of it: everything behind stayed razor sharp while the
    /// reference blurred the entire page. Pushing this through `DrawList::push_glass` is what gives
    /// [`Self::veil_blur_px`] its first production consumer (`📓️w4a`/`📓️w7a` hand-off 2).
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn veil_glass(&self, level: Level) -> GlassStyle {
        GlassStyle { tint: self.surface(level), alpha: levels::VEIL_ALPHA as f32, blur_px: Self::veil_blur_px(), saturate: self.glass_saturate }
    }

    //#endregion 🔖️LevelSurfaces

    //#region 🔖️Presence
    /// 🎨️ Resolves a peer's palette index to this theme's appearance (contract freeze §C7.5), by
    /// `index % 12` into the pre-resolved base-cycle swatches. `Theme` is appearance-specific
    /// (`light()`/`dark()`), so the swatches are already correct for whichever `self` is; the full
    /// per-cycle desaturate/lighten shift for `index / 12 >= 1` is `presence_bar::presence_color`'s job
    /// for callers that need it directly.
    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn presence_color(&self, index: u8) -> Rgba {
        self.presence[(index % 12) as usize]
    }
    //#endregion 🔖️Presence

    // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
    pub fn glass_mip_level(blur_px: f32, max_mip: u32) -> f32 {
        (blur_px / 4.0).log2().max(0.0).min(max_mip as f32)
    }
}

pub type ThemedRect = Rect;

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-theme-token-parity/🦀️.rs"]
mod tests;
// #endregion theme
