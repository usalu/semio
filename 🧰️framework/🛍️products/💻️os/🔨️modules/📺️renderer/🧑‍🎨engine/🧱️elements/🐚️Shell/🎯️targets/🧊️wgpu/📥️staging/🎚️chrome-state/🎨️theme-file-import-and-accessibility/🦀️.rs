//! 🎚️ `shell-chrome-state` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🎨️ThemeDocumentModel
/// 🎨️ React's `UiTheme["strokes"][k]` — `number | number[]` (a dash pattern is a list).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeNumber {
    Scalar(f64),
    List(Vec<f64>),
}

impl ToValue for ThemeNumber {
    fn to_value(&self)->DslValue{match self{Self::Scalar(value)=>value.to_value(),Self::List(values)=>values.to_value()}}
}

impl FromValue for ThemeNumber {
    fn from_value(value:DslValue)->Result<Self,semio_framework_value::ValueError>{match value{DslValue::Array(_)=>Vec::<f64>::from_value(value).map(Self::List),_=>f64::from_value(value).map(Self::Scalar)}}
}

impl ThemeNumber {
    /// 🔢️ React's `Array.isArray(value) ? value.join(", ") : String(value)` — the text
    /// `themeNumberInputRow` puts in the box.
    pub fn as_text(&self) -> String {
        match self {
            ThemeNumber::Scalar(value) => format_theme_scalar(*value),
            ThemeNumber::List(values) => values.iter().map(|value| format_theme_scalar(*value)).collect::<Vec<_>>().join(", "),
        }
    }

    /// 🔢️ React's `onBlur` parse: a comma anywhere makes it a list, otherwise one float; an
    /// unparseable buffer commits NOTHING (React returns without calling `onCommit`).
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.contains(',') {
            let parts: Vec<f64> = raw.split(',').filter_map(|part| part.trim().parse::<f64>().ok()).collect();
            return (!parts.is_empty()).then_some(ThemeNumber::List(parts));
        }
        raw.parse::<f64>().ok().map(ThemeNumber::Scalar)
    }

    /// 🔢️ The scalar a `Theme` field wants — a list answers its first entry, exactly as React's
    /// `setThemeRadius`/`setThemeOpacity` narrow `typeof value === "number" ? value : value[0]!`.
    pub fn scalar(&self) -> Option<f64> {
        match self {
            ThemeNumber::Scalar(value) => Some(*value),
            ThemeNumber::List(values) => values.first().copied(),
        }
    }
}

/// 🖌️ React's `ThemePaintRef` (`🖱️ui/🎨️styling/🌓️theme/🟦️.ts:33`) — a primitive token, a literal hex,
/// or an oklab-free channel blend of two tokens.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ThemePaintRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mix: Option<(String, String, f64)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ThemeIcons {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub aliases: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub variants: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub themed_aliases: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub themed_variants: BTreeMap<String, String>,
}

/// 🎨️ The wgpu twin of React's `UiTheme` (`🖱️ui/🎨️styling/🌓️theme/🟦️.ts:63`) — the document BOTH
/// renderers' theme editors mutate and BOTH persist into `os.config.ui-preferences`'
/// `customThemes[id].config`. Every map is a `BTreeMap`, so a section's rows enumerate in the same
/// ascending key order React's `Object.keys(...).sort()` produces and a saved document is byte-stable.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ThemeDocument {
    #[serde(default)]
    #[value(default)]
    pub id: String,
    #[serde(default)]
    #[value(default)]
    pub label: String,
    #[serde(default)]
    #[value(default)]
    pub colors: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub spacing: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub font_stacks: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub canvas_fonts: BTreeMap<String, String>,
    #[serde(default)]
    #[value(default)]
    pub strokes: BTreeMap<String, ThemeNumber>,
    #[serde(default)]
    #[value(default)]
    pub radii: BTreeMap<String, ThemeNumber>,
    #[serde(default)]
    #[value(default)]
    pub opacities: BTreeMap<String, ThemeNumber>,
    #[serde(default)]
    #[value(default)]
    pub metrics: BTreeMap<String, BTreeMap<String, ThemeNumber>>,
    #[serde(default)]
    #[value(default)]
    pub appearances: BTreeMap<String, BTreeMap<String, BTreeMap<String, ThemePaintRef>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icons: Option<ThemeIcons>,
}

impl ThemeDocument {
    /// 🧬️ Parses the one canonical theme schema shared by save, import, resolution and labels.
    pub fn parse(text: &str) -> Option<Self> {
        const FIELDS: &[&str] = &["id", "label", "colors", "spacing", "fontStacks", "canvasFonts", "strokes", "radii", "opacities", "metrics", "appearances"];
        const GROUPS: &[&str] = &["board", "map", "canvas", "chrome", "outcome", "diagram"];
        let value: serde_json::Value = serde_json::from_str(text).ok()?;
        let object = value.as_object()?;
        if FIELDS.iter().any(|field| !object.contains_key(*field)) || object.get("icons").is_some_and(|icons| !icons.is_object()) {
            return None;
        }
        let document: Self = serde_json::from_str(text).ok()?;
        let scalar_maps = document.radii.values().chain(document.opacities.values()).all(|value| matches!(value, ThemeNumber::Scalar(_)));
        let palettes = ["light", "dark"]
            .iter()
            .all(|appearance| document.appearances.get(*appearance).is_some_and(|groups| GROUPS.iter().all(|group| groups.get(*group).is_some_and(|paints| paints.values().all(|paint| theme_paint_ref_is_resolvable(&document.colors, paint))))));
        (scalar_maps && palettes && theme_document_geometry(&document).is_some()).then_some(document)
    }

    /// 💾️ Serializes the same validated document shape React stores and exports.
    pub fn canonical_json(&self) -> Option<String> {
        let text = serde_json::to_string_pretty(self).ok()?;
        Self::parse(&text).map(|_| text)
    }
}

//#endregion 🎨️ThemeDocumentModel
//#region ♿️ChromeAccessibleNames
impl ChromeState {
    /// ⌨️ One chrome control's visible label WITH its inline hotkey badge — React's `ButtonGroupItem`
    /// paints a `ControlHotkeyBadge` beside the label of ANY bound control while the driver's
    /// `hotkeys` axis is `"inline"` (the default driver's), not only beside the two role chips
    /// (audit W14 §B14). Idempotent: a builder that already appended the badge (the role chips do,
    /// for their own measured layout) is left alone rather than double-badged.
    pub fn chrome_control_label(&self, control_id: &str, label: &str) -> String {
        match shell_control_hotkey_badge(&self.shortcut_table(), control_id) {
            Some(badge) if !label.ends_with(&badge) => format!("{label} {badge}"),
            _ => label.to_string(),
        }
    }
}

//#endregion ♿️ChromeAccessibleNames

//#region 🎨️ThemeDocumentModel
/// 🎨️ The authored base document, parsed once. `id`/`label` are stamped the way the styling codegen
/// stamps them onto `STYLING_SEMIO_THEME`.
pub fn shell_theme_document_base() -> &'static ThemeDocument {
    static BASE: std::sync::OnceLock<ThemeDocument> = std::sync::OnceLock::new();
    BASE.get_or_init(|| {
        let mut document: ThemeDocument = serde_json::from_str(SHELL_THEME_DOCUMENT_JSON).unwrap_or_else(|_| ThemeDocument {
            id: String::new(),
            label: String::new(),
            colors: BTreeMap::new(),
            spacing: BTreeMap::new(),
            font_stacks: BTreeMap::new(),
            canvas_fonts: BTreeMap::new(),
            strokes: BTreeMap::new(),
            radii: BTreeMap::new(),
            opacities: BTreeMap::new(),
            metrics: BTreeMap::new(),
            appearances: BTreeMap::new(),
            icons: None,
        });
        document.id = "semio".to_string();
        document.label = "semio".to_string();
        document
    })
}

/// 🎨️ The AUTHORED design-token document — the very `🔣️.json` the styling codegen turns into
/// React's `STYLING_SEMIO_THEME` and into this target's compile-time `ui_styling` constants. Embedded
/// so the wgpu theme EDITOR can enumerate exactly the keys React's editor enumerates
/// (`host.theme.colors`/`spacing`/`fontStacks`/`strokes`/`radii`/`opacities`/`metrics`/`appearances`),
/// rather than a hand-kept second key list that would drift on the next regeneration.
///
/// ⚖️ The generated `STYLING_SEMIO_THEME` is this document minus `version`/`fontFaces`/`levels`/
/// `presence` plus `id`/`label`, which is exactly what [`ThemeDocument`]'s `#[serde(default)]` fields
/// keep and what [`shell_theme_document_base`] stamps back on.
pub const SHELL_THEME_DOCUMENT_JSON: &str = include_str!("../../../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🔣️.json");

/// 📐️ Resolves the shared named geometry bindings before any target scalar is cast to f32.
pub fn theme_document_geometry(document: &ThemeDocument) -> Option<BTreeMap<&'static str, f64>> {
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

pub fn theme_paint_ref_is_resolvable(colors: &BTreeMap<String, String>, paint: &ThemePaintRef) -> bool {
    let token_exists = |token: &str| colors.get(token).is_some_and(|value| !value.is_empty());
    if let Some((left, right, _)) = paint.mix.as_ref() {
        return token_exists(left) && (right == "transparent" || token_exists(right));
    }
    paint.hex.is_some() || paint.token.as_deref().is_some_and(token_exists)
}

#[path = "../../../../../../../../../../../../🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry/🦀️.rs"]
pub mod theme_geometry;

/// 🔢️ `String(n)` in JavaScript never prints a trailing `.0`, and these strings are compared against
/// React's boxes byte for byte in the parity probe.
pub fn format_theme_scalar(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}
//#endregion 🎨️ThemeDocumentModel

//#region 🎨️ThemeDocumentModel
/// 🎛️ Projects validated authored geometry into the typed renderer fields and retains direct duration units.
pub fn apply_theme_document_metrics(theme: &mut Theme, document: &ThemeDocument) {
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
