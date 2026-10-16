//! 🎛️ `shell-prefs` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🎨️ThemeDocumentModel

/// 🖌️ `parseHex6` — React accepts `#rgb` and `#rrggbb` alike.
pub fn theme_parse_hex6(hex: &str) -> [u8; 3] {
    let raw = hex.trim().trim_start_matches('#');
    if raw.len() == 3 {
        let expand = |index: usize| u8::from_str_radix(&raw[index..index + 1].repeat(2), 16).unwrap_or(0);
        return [expand(0), expand(1), expand(2)];
    }
    let value = u32::from_str_radix(raw, 16).unwrap_or(0);
    [((value >> 16) & 0xff) as u8, ((value >> 8) & 0xff) as u8, (value & 0xff) as u8]
}

/// 🖌️ `blendHex` — a straight sRGB channel lerp, NOT an oklab mix: React's paint refs blend in the
/// authored space and only the LEVEL ramp below is oklab.
fn theme_blend_hex(a: &str, b: &str, ratio_a: f64) -> [u8; 3] {
    let left = theme_parse_hex6(a);
    let right = theme_parse_hex6(b);
    let t = ratio_a.clamp(0.0, 1.0);
    std::array::from_fn(|channel| (f64::from(left[channel]) * t + f64::from(right[channel]) * (1.0 - t)).round() as u8)
}

/// 🖌️ The Rust twin of `resolveThemePaint` (`🖱️ui/🎨️styling/🌓️theme/🟦️.ts:109`), including its
/// `"transparent"` mix special case (a mix against transparent carries `1 - ratio` as its alpha
/// unless the ref states one). An unresolvable ref answers `None` rather than inventing a colour.
pub fn theme_resolve_paint(colors: &BTreeMap<String, String>, paint: &ThemePaintRef) -> Option<[u8; 4]> {
    let mut alpha = paint.alpha.unwrap_or(1.0);
    let rgb = if let Some((left, right, ratio)) = paint.mix.as_ref() {
        let left_hex = colors.get(left)?;
        let right_hex = if right == "transparent" { "#000000".to_string() } else { colors.get(right)?.clone() };
        if right == "transparent" && paint.alpha.is_none() {
            alpha = 1.0 - ratio;
        }
        theme_blend_hex(left_hex, &right_hex, *ratio)
    } else if let Some(hex) = paint.hex.as_ref() {
        theme_parse_hex6(hex)
    } else {
        theme_parse_hex6(colors.get(paint.token.as_deref()?)?)
    };
    Some([rgb[0], rgb[1], rgb[2], (alpha.clamp(0.0, 1.0) * 255.0).round() as u8])
}

//#endregion 🎨️ThemeDocumentModel
//#region 🎨️ThemeEditorPanel
impl PrefsState {
    /// 🎨️ The theme document this session is editing — the live draft, else the selected custom
    /// theme's own persisted document, else the authored base. React's
    /// `uiThemeBase = uiThemeDraft ?? activeUiTheme()`, verbatim.
    pub fn theme_document(&self, chrome: &ChromeState) -> ThemeDocument {
        if let Some(draft) = self.theme_draft.as_ref() {
            return draft.clone();
        }
        if let Some(document) = chrome.chrome_build.preferences.custom_themes.get(&chrome.chrome_build.preferences.theme_id).and_then(|raw| ThemeDocument::parse(raw)) {
            return document;
        }
        shell_theme_document_base().clone()
    }

    /// 🎨️ The draft, seeded from the active document on first write — React's `draftThemePatch`,
    /// which `structuredClone`s `uiThemeBase` before every patch.
    pub fn theme_draft_patch(&mut self, patch: impl FnOnce(&mut ThemeDocument), chrome: &ChromeState) {
        let mut draft = self.theme_draft.take().unwrap_or_else(|| self.theme_document(chrome));
        patch(&mut draft);
        set_active_theme_document(Some(draft.clone()));
        self.theme_draft = Some(draft);
    }

    /// 🛡️ Stages a geometry edit and publishes only a fully resolvable document.
    pub fn theme_geometry_draft_patch(&mut self, patch: impl FnOnce(&mut ThemeDocument), chrome: &ChromeState) {
        let mut draft = self.theme_document(chrome);
        patch(&mut draft);
        if theme_document_geometry(&draft).is_some() {
            set_active_theme_document(Some(draft.clone()));
            self.theme_draft = Some(draft);
        }
    }

    /// 🎨️ Drops the draft and the present-side re-tokenisation with it — React's
    /// `dispatch({ type: "SET_UI_THEME_DRAFT", value: null })`.
    pub fn clear_theme_draft(&mut self) {
        self.theme_draft = None;
        set_active_theme_document(None);
    }
}

/// 🖌️ What React's appearance-paint row shows: the resolved `#rrggbb` from `rgba8ToHex` and the
/// resolved alpha as `rgba[3] / 255` (`buildThemeAppearanceGroupItems`), so the boxes read the same
/// on both renderers even for a `mix`/`token` ref that carries neither literally.
pub fn theme_paint_editor_values(colors: &BTreeMap<String, String>, reference: &ThemePaintRef) -> (String, f64) {
    let [r, g, b, a] = theme_resolve_paint(colors, reference).unwrap_or([0, 0, 0, 255]);
    (format!("#{r:02x}{g:02x}{b:02x}"), f64::from(a) / 255.0)
}

//#endregion 🎨️ThemeEditorPanel
//#region 🎨️ThemeDocumentDoors
/// 🎨️ Publishes (or clears) the DRAFT document the present side re-tokenises from. The draft lives
/// beside `theme_id` in the same owned preference snapshot `frame()`'s `resolve_theme_for_ids` reads,
/// which is what makes an edit in the Settings panel repaint the chrome on the very next frame —
/// React's `applyUiThemeToRoot(uiThemeDraft)`.
pub fn set_active_theme_document(document: Option<ThemeDocument>) {
    with_chrome_prefs(|prefs| {
        prefs.theme_document_draft = document;
        prefs.theme_document_generation = prefs.theme_document_generation.wrapping_add(1);
        prefs.theme_document_resolved = None;
    });
}

//#endregion 🎨️ThemeDocumentDoors
//#region ⚖️MergePolicyAndThemeCommits
impl PrefsState {
    /// 🎨️ One theme-editor commit — the eight `draftThemePatch` mutators of React's `ShellHost`
    /// (`🏛️ShellHost/🟦️.tsx:8248-8301`), dispatched by verb over the row's own coordinates. A value
    /// that does not parse commits NOTHING, exactly as React's `onBlur` returns without calling
    /// `onCommit`.
    ///
    /// 🖌️ React edits a paint through ONE call carrying both channels: the colour input sends
    /// the new hex with the row's current alpha, the alpha box sends the row's current hex with
    /// the new alpha. Either way the ref becomes a literal `{ hex, alpha }` — a token/mix ref
    /// the user has overridden is no longer that token, exactly as React's
    /// `next.appearances[a][g][k] = { hex, alpha }` replaces it outright.
    pub fn apply_theme_editor_commit(&mut self, verb: &str, args: &Value, chrome: &ChromeState) {
        let text = args.get("value").and_then(Value::as_str).unwrap_or_default().to_string();
        let key = args.get("key").and_then(Value::as_str).unwrap_or_default().to_string();
        match verb {
            "setThemeColor" => self.theme_draft_patch(move |document| {
                document.colors.insert(key, text);
            }, chrome),
            "setThemeSpacing" => self.theme_geometry_draft_patch(move |document| {
                document.spacing.insert(key, text);
            }, chrome),
            "setThemeFontStack" => self.theme_draft_patch(move |document| {
                document.font_stacks.insert(key, text);
            }, chrome),
            "setThemeStroke" => {
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_draft_patch(move |document| {
                        document.strokes.insert(key, number);
                    }, chrome);
                }
            }
            "setThemeRadius" => {
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_draft_patch(move |document| {
                        document.radii.insert(key, number);
                    }, chrome);
                }
            }
            "setThemeOpacity" => {
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_draft_patch(move |document| {
                        document.opacities.insert(key, number);
                    }, chrome);
                }
            }
            "setThemeMetric" => {
                let section = args.get("section").and_then(Value::as_str).unwrap_or_default().to_string();
                if let Some(number) = ThemeNumber::parse(&text) {
                    self.theme_geometry_draft_patch(move |document| {
                        document.metrics.entry(section).or_default().insert(key, number);
                    }, chrome);
                }
            }
            "setThemeAppearancePaint" => {
                let appearance = args.get("appearance").and_then(Value::as_str).unwrap_or_default().to_string();
                let group = args.get("group").and_then(Value::as_str).unwrap_or_default().to_string();
                let paint = args.get("paint").and_then(Value::as_str).unwrap_or_default().to_string();
                let channel = args.get("channel").and_then(Value::as_str).unwrap_or("hex").to_string();
                let document = self.theme_document(chrome);
                let current = document.appearances.get(&appearance).and_then(|groups| groups.get(&group)).and_then(|paints| paints.get(&paint)).cloned().unwrap_or_default();
                let (current_hex, current_alpha) = theme_paint_editor_values(&document.colors, &current);
                let reference = if channel == "alpha" {
                    let Some(alpha) = text.trim().parse::<f64>().ok() else { return };
                    ThemePaintRef { hex: Some(current_hex), alpha: Some(alpha.clamp(0.0, 1.0)), ..ThemePaintRef::default() }
                } else {
                    ThemePaintRef { hex: Some(text), alpha: Some(current_alpha), ..ThemePaintRef::default() }
                };
                self.theme_draft_patch(move |document| {
                    document.appearances.entry(appearance).or_default().entry(group).or_default().insert(paint, reference);
                }, chrome);
            }
            _ => {}
        }
    }
}

//#endregion ⚖️MergePolicyAndThemeCommits

//#region 🎨️ThemeDocumentModel
/// 🖌️ One resolved paint as this target's linear-float [`Rgba`], through the same
/// `ui_styling::color::rgba8_to_linear` decode every generated palette constant went through.
pub fn theme_paint_rgba(colors: &BTreeMap<String, String>, group: &BTreeMap<String, ThemePaintRef>, key: &str, fallback: Rgba) -> Rgba {
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
pub fn theme_from_document(document: &ThemeDocument, dark: bool) -> Theme {
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
//#endregion 🎨️ThemeDocumentModel
