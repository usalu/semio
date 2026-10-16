//! 🎚️ `shell-chrome-state` staging of `theme-file-import-and-accessibility`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region 🎨️ThemeDocumentModel
/// 🎨️ React's `UiTheme["strokes"][k]` — `number | number[]` (a dash pattern is a list).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ThemeNumber {
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
    pub(super) fn as_text(&self) -> String {
        match self {
            ThemeNumber::Scalar(value) => format_theme_scalar(*value),
            ThemeNumber::List(values) => values.iter().map(|value| format_theme_scalar(*value)).collect::<Vec<_>>().join(", "),
        }
    }

    /// 🔢️ React's `onBlur` parse: a comma anywhere makes it a list, otherwise one float; an
    /// unparseable buffer commits NOTHING (React returns without calling `onCommit`).
    pub(crate) fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        if raw.contains(',') {
            let parts: Vec<f64> = raw.split(',').filter_map(|part| part.trim().parse::<f64>().ok()).collect();
            return (!parts.is_empty()).then_some(ThemeNumber::List(parts));
        }
        raw.parse::<f64>().ok().map(ThemeNumber::Scalar)
    }

    /// 🔢️ The scalar a `Theme` field wants — a list answers its first entry, exactly as React's
    /// `setThemeRadius`/`setThemeOpacity` narrow `typeof value === "number" ? value : value[0]!`.
    pub(crate) fn scalar(&self) -> Option<f64> {
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
pub(crate) struct ThemePaintRef {
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
pub(crate) struct ThemeIcons {
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
pub(crate) struct ThemeDocument {
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
    pub(crate) fn parse(text: &str) -> Option<Self> {
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
    pub(crate) fn canonical_json(&self) -> Option<String> {
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
    pub(crate) fn chrome_control_label(&self, control_id: &str, label: &str) -> String {
        match shell_control_hotkey_badge(&self.shortcut_table(), control_id) {
            Some(badge) if !label.ends_with(&badge) => format!("{label} {badge}"),
            _ => label.to_string(),
        }
    }
}

//#endregion ♿️ChromeAccessibleNames

