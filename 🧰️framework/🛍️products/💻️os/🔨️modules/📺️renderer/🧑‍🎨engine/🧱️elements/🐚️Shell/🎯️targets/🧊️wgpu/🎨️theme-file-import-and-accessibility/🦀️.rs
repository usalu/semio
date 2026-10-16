//! 🎨️ Media export, file import, theme documents and chrome accessibility.
//!
//! Media export, file open/import, host IO tracing, media frame requests, the theme document model and editor,
//! theme doors, merge policy and the chrome accessible names and nodes.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region 📤️FileOpenImport

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📤️wgpu-file-open-import/🦀️.rs"]
mod file_open_import_tests;
//#endregion 📤️FileOpenImport

//#region RequestMediaFrames

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-media-frames/🦀️.rs"]
mod media_frames_tests;
//#endregion RequestMediaFrames

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs"]
mod context_menu_keyboard_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/⬇️wgpu-media-export-encoding/🦀️.rs"]
mod media_export_encoding_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs"]
mod dpi_logical_units_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/💓️chrome-maintenance-pressure/🦀️.rs"]
mod chrome_maintenance_pressure_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎬️wgpu-plugin-install/🦀️.rs"]
mod plugin_install_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🔀️wgpu-document-relay/🦀️.rs"]
mod document_relay_tests;

//#region 🎨️ThemeDocumentModel

/// 🔢️ `String(n)` in JavaScript never prints a trailing `.0`, and these strings are compared against
/// React's boxes byte for byte in the parity probe.
pub(super) fn format_theme_scalar(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

#[path = "../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🦀️.rs"]
pub(super) mod theme_geometry;

pub(super) fn theme_paint_ref_is_resolvable(colors: &BTreeMap<String, String>, paint: &ThemePaintRef) -> bool {
    let token_exists = |token: &str| colors.get(token).is_some_and(|value| !value.is_empty());
    if let Some((left, right, _)) = paint.mix.as_ref() {
        return token_exists(left) && (right == "transparent" || token_exists(right));
    }
    paint.hex.is_some() || paint.token.as_deref().is_some_and(token_exists)
}

/// 🖌️ One resolved paint as this target's linear-float [`Rgba`], through the same
/// `ui_styling::color::rgba8_to_linear` decode every generated palette constant went through.
fn theme_paint_rgba(colors: &BTreeMap<String, String>, group: &BTreeMap<String, ThemePaintRef>, key: &str, fallback: Rgba) -> Rgba {
    group.get(key).and_then(|paint| theme_resolve_paint(colors, paint)).map(|[r, g, b, a]| Rgba::from_srgb8(r, g, b, a)).unwrap_or(fallback)
}

/// 🎨️ Re-tokenises one appearance of a `Theme` FROM a live document — the runtime twin of the ui
/// crate's compile-time `from_chrome`, field for field, so an edit in the settings panel repaints the
/// shell on the next frame exactly as React's `SET_UI_THEME_DRAFT` + `applyUiThemeToRoot` repaints
/// its DOM.
///
/// 🪜️ The six `level*` surfaces are NOT authored paints: the styling projection injects them as
/// `oklabMix(base, foreground, k · shadeStepPercent/100)` (`🖱️ui/🎨️styling/📽️projection/🟦️.ts:129`),
/// so they are recomputed here through `ui_styling::color::oklab_mix` — which is what makes editing
/// `chrome.base` move the navbar, the panels and the menu surface and not just the page background.
///
/// ⚖️ A key the document does not carry keeps the built-in value it had: this layers onto the
/// appearance's own generated `Theme`, it does not replace it, so a partial custom document can never
/// blank a surface.
///
/// 🪜️ The formula-derived ramp, recomputed off the two paints it is a function of.
pub(crate) fn theme_from_document(document: &ThemeDocument, dark: bool) -> Theme {
    let mut theme = if dark { Theme::dark() } else { Theme::light() };
    let colors = &document.colors;
    if let Some([r, g, b]) = colors.get("diff-added").map(|hex| theme_parse_hex6(hex)) {
        theme.diff_added = Rgba::from_srgb8(r, g, b, 255);
    }
    let appearance = document.appearances.get(if dark { "dark" } else { "light" });
    if let Some(chrome) = appearance.and_then(|groups| groups.get("chrome")) {
        let paint = |key: &str, fallback: Rgba| theme_paint_rgba(colors, chrome, key, fallback);
        let base = paint("base", theme.background);
        let foreground = paint("foreground", theme.text);
        theme.background = base;
        theme.input_bg = base;
        theme.canvas_clear = base;
        theme.text = foreground;
        theme.muted = paint("muted", theme.muted);
        theme.text_muted = paint("mutedForeground", theme.text_muted);
        theme.accent = paint("accent", theme.accent);
        theme.accent_hover = paint("activeHover", theme.accent_hover);
        theme.active_foreground = paint("activeForeground", theme.active_foreground);
        theme.selected = paint("activeBase", theme.selected);
        theme.button_hover = paint("hoverInteractiveFill", theme.button_hover);
        theme.row_hover = theme.button_hover;
        theme.separator = paint("borderNormal", theme.separator);
        theme.panel_border = theme.separator;
        theme.border_normal = theme.separator;
        theme.border_emphasized = paint("borderEmphasized", theme.border_emphasized);
        theme.text_element = paint("borderElement", theme.text_element);
        let focus_alpha = document.opacities.get("chromeFocusRingAlpha").and_then(ThemeNumber::scalar).unwrap_or(f64::from(theme.focus_ring.a));
        theme.focus_ring = theme.accent.with_alpha(focus_alpha as f32);
        let shade_step = ui_styling::levels::SHADE_STEP_PERCENT as f32 / 100.0;
        let base_linear = [base.r, base.g, base.b, base.a];
        let foreground_linear = [foreground.r, foreground.g, foreground.b, foreground.a];
        theme.level_bg = std::array::from_fn(|index| {
            let mixed = ui_styling::color::oklab_mix(base_linear, foreground_linear, index as f32 * shade_step);
            Rgba::new(mixed[0], mixed[1], mixed[2], mixed[3])
        });
        theme.navbar = theme.level_bg[1];
        theme.button = theme.level_bg[1];
        theme.panel = theme.level_bg[3];
        theme.temporary = theme.level_bg[5];
    }
    if let Some(outcome) = appearance.and_then(|groups| groups.get("outcome")) {
        theme.error = theme_paint_rgba(colors, outcome, "error", theme.error);
        theme.warning = theme_paint_rgba(colors, outcome, "warning", theme.warning);
        theme.success = theme_paint_rgba(colors, outcome, "success", theme.success);
        theme.progress = theme_paint_rgba(colors, outcome, "progress", theme.progress);
    }
    if let Some(diagram) = appearance.and_then(|groups| groups.get("diagram")) {
        theme.checker_light = theme_paint_rgba(colors, diagram, "checkerLight", theme.checker_light);
        theme.checker_dark = theme_paint_rgba(colors, diagram, "checkerDark", theme.checker_dark);
        theme.diagram_stroke = theme_paint_rgba(colors, diagram, "stroke", theme.diagram_stroke);
        theme.diagram_seam = theme_paint_rgba(colors, diagram, "seam", theme.diagram_seam);
        theme.diagram_accent = theme_paint_rgba(colors, diagram, "accent", theme.diagram_accent);
        theme.diagram_accent_fill = theme_paint_rgba(colors, diagram, "accentFill", theme.diagram_accent_fill);
        theme.diagram_shape_outline = theme_paint_rgba(colors, diagram, "shapeOutline", theme.diagram_shape_outline);
        theme.diagram_field_residual = theme_paint_rgba(colors, diagram, "fieldResidual", theme.diagram_field_residual);
        theme.diagram_field_reaction = theme_paint_rgba(colors, diagram, "fieldReaction", theme.diagram_field_reaction);
        theme.diagram_field_displacement = theme_paint_rgba(colors, diagram, "fieldDisplacement", theme.diagram_field_displacement);
    }
    theme.border_radius = document.radii.get("chrome").and_then(ThemeNumber::scalar).map_or(theme.border_radius, |value| value as f32);
    theme.stroke_hairline = document.strokes.get("chromeBorderHairline").and_then(ThemeNumber::scalar).map_or(theme.stroke_hairline, |value| value as f32);
    theme.stroke_focus = document.strokes.get("chromeBorderFocus").and_then(ThemeNumber::scalar).map_or(theme.stroke_focus, |value| value as f32);
    apply_theme_document_metrics(&mut theme, document);
    theme
}

/// 📐️ Resolves the shared named geometry bindings before any target scalar is cast to f32.
pub(super) fn theme_document_geometry(document: &ThemeDocument) -> Option<BTreeMap<&'static str, f64>> {
    let canonical = shell_theme_document_base();
    let scalar = |section: &str, key: &str| document.metrics.get(section).and_then(|rows| rows.get(key)).or_else(|| canonical.metrics.get(section).and_then(|rows| rows.get(key))).and_then(|number| match number { ThemeNumber::Scalar(value) => Some(*value), ThemeNumber::List(_) => None });
    let compact = document.spacing.get("compact").or_else(|| canonical.spacing.get("compact"))?;
    let spacing_px = theme_geometry::resolve_spacing_px(compact, scalar("dom", "rootRemPx")?)?;
    let limit = theme_geometry::contract()["x-semio-resolution"]["maxMagnitudePx"].as_f64()?;
    let mut values = BTreeMap::new();
    for binding in theme_geometry::bindings()["bindings"].as_array()? {
        let value = scalar(binding["section"].as_str()?, binding["key"].as_str()?)?;
        if !value.is_finite() || value < 0.0 { return None; }
        let pixels = value * if binding["unit"].as_str()? == "spacing" { spacing_px } else { 1.0 };
        if !pixels.is_finite() || pixels > limit { return None; }
        values.insert(binding["themeField"].as_str()?, pixels);
    }
    Some(values)
}

/// 🎛️ Projects validated authored geometry into the typed renderer fields and retains direct duration units.
fn apply_theme_document_metrics(theme: &mut Theme, document: &ThemeDocument) {
    let Some(values) = theme_document_geometry(document) else { return };
    theme.navbar_height = values["navbar_height"] as f32;
    theme.footer_height = values["footer_height"] as f32;
    theme.panel_header_height = values["panel_header_height"] as f32;
    theme.control_height = values["control_height"] as f32;
    theme.control_height_small = values["control_height_small"] as f32;
    theme.gap_standard = values["gap_standard"] as f32;
    theme.padding_standard = values["padding_standard"] as f32;
    theme.panel_inset = values["panel_inset"] as f32;
    theme.tree_row_height = values["tree_row_height"] as f32;
    theme.tree_indent_per_level = values["tree_indent_per_level"] as f32;
    theme.tree_toggle_width = values["tree_toggle_width"] as f32;
    theme.panel_min_width = values["panel_min_width"] as f32;
    theme.panel_max_width = values["panel_max_width"] as f32;
    theme.window_measures_default_width = values["window_measures_default_width"] as f32;
    theme.window_engagement_max_width = values["window_engagement_max_width"] as f32;
    theme.font_size_body = values["font_size_body"] as f32;
    theme.font_size_small = values["font_size_small"] as f32;
    theme.font_size_emphasized = values["font_size_emphasized"] as f32;
    if let Some(value) = document.metrics.get("chrome").and_then(|rows| rows.get("celebrateBorderDurationSeconds")).and_then(ThemeNumber::scalar) {
        theme.celebrate_duration_seconds = value as f32;
    }
}
//#endregion 🎨️ThemeDocumentModel


//#region 🎨️ThemeDocumentDoors

/// 🎨️ The resolved theme for a canonical document-backed selection or live draft.
///
/// 🧊️ Cached on `(theme id, appearance, draft generation)`: `resolve_theme_for_ids` runs once per
/// frame per appearance, and re-resolving ~450 document entries there would put a JSON walk in the
/// frame loop.
pub(crate) fn document_theme_for_ids(theme_id: &str, dark: bool) -> Option<Theme> {
    with_chrome_prefs(|prefs| {
        let key = (theme_id.to_string(), dark, prefs.theme_document_generation);
        if let Some((cached, theme)) = prefs.theme_document_resolved.as_ref() {
            if cached == &key {
                return Some(*theme);
            }
        }
        let document = match prefs.theme_document_draft.clone() {
            Some(draft) => draft,
            None => custom_theme_document_from(prefs, theme_id)?,
        };
        let theme = theme_from_document(&document, dark);
        prefs.theme_document_resolved = Some((key, theme));
        Some(theme)
    })
}

//#endregion 🎨️ThemeDocumentDoors


//#region ♿️ChromeAccessibleNames

/// 🪡️ Per thread on wasm and in tests — each test owns its shell, so a concurrent test's frame setup can never
/// clear the names another test is about to publish — and one worker-wide cell natively.
#[cfg(any(target_arch = "wasm32", test))]
thread_local! {
    pub(super) static CHROME_CONTROL_NAMES: std::cell::RefCell<BTreeMap<String, ChromeControlPresentation>> = std::cell::RefCell::new(BTreeMap::new());
}

//#endregion ♿️ChromeAccessibleNames

#[cfg(test)]
#[path = "../../../🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs"]
mod theme_editor_and_accessibility_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/⌨️native-keyboard-ring/🦀️.rs"]
mod native_keyboard_ring_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/🎯️presented-input-authority/🦀️.rs"]
mod presented_input_authority_tests;
