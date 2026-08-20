//! @emoji 🎨️ Design-token `StyleSpec` — closed enums over ui_styling tokens, never raw values.
//!
//! ⚠️ SCAFFOLD — owned by packet `contract-layout`. Replace this placeholder wholesale; keep the region
//! structure and the U1 sync rule (no `async fn` in this crate).

use serde::{Deserialize, Serialize};

//#region 🔖️Style

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// 🖌️ The chrome treatment a renderer paints a component with — independent of [`Tone`] (which color
/// role) and [`Emphasis`] (how prominent).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Variant {
    #[default]
    Solid,
    Outline,
    Ghost,
    Plain,
}

/// 📏️ A component's t-shirt size. Mirrors the one real precedent in the wgpu target's old
/// `StyleSpec.size` (`"md"`); no dedicated component-size ramp exists in tokens.json yet — see the
/// packet report.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum SizeToken {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// 📐️ Named directly from tokens.json's `spacing` table, the only two spacing tokens the styling
/// package actually ships (`compact`, `touch`). `Standard` is the deliberate default occupying the gap
/// between them — no dedicated token for the middle case exists yet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Density {
    Compact,
    #[default]
    Standard,
    Touch,
}

/// 🎨️ The semantic color role a renderer resolves against the active theme — named after
/// tokens.json's `colors` table's semantic entries (`primary`, `secondary`, `tertiary`, `danger`,
/// `warning`, `info`, `success`). `Neutral` is the default: no explicit accent, inherit the
/// surrounding surface/text color.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Tone {
    #[default]
    Neutral,
    Primary,
    Secondary,
    Tertiary,
    Info,
    Success,
    Warning,
    Danger,
}

/// 🔆️ Visual prominence, orthogonal to [`Variant`] and [`Tone`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Emphasis {
    Subtle,
    #[default]
    Regular,
    Strong,
}

/// 🎨️ A node's design-token styling — five closed enums, never a raw color or a raw pixel value. A
/// renderer resolves every field against the active theme; this struct only names the tokens. Each
/// field is omitted from the wire at its default, so a default-styled node costs nothing to encode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct StyleSpec {
    #[serde(default, skip_serializing_if = "is_default")]
    pub variant: Variant,
    #[serde(default, skip_serializing_if = "is_default")]
    pub size: SizeToken,
    #[serde(default, skip_serializing_if = "is_default")]
    pub density: Density,
    #[serde(default, skip_serializing_if = "is_default")]
    pub tone: Tone,
    #[serde(default, skip_serializing_if = "is_default")]
    pub emphasis: Emphasis,
}
//#endregion 🔖️Style

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_style_spec_serializes_to_empty_object() {
        let json = serde_json::to_value(StyleSpec::default()).expect("serialize");
        assert_eq!(json, serde_json::json!({}));
    }

    #[test]
    fn variant_field_omitted_at_default() {
        let json = serde_json::to_value(StyleSpec { variant: Variant::Outline, ..StyleSpec::default() }).expect("serialize");
        assert_eq!(json, serde_json::json!({ "variant": "outline" }));
    }

    #[test]
    fn size_field_omitted_at_default() {
        let json = serde_json::to_value(StyleSpec { size: SizeToken::Lg, ..StyleSpec::default() }).expect("serialize");
        assert_eq!(json, serde_json::json!({ "size": "lg" }));
    }

    #[test]
    fn density_field_omitted_at_default() {
        let json = serde_json::to_value(StyleSpec { density: Density::Touch, ..StyleSpec::default() }).expect("serialize");
        assert_eq!(json, serde_json::json!({ "density": "touch" }));
    }

    #[test]
    fn tone_field_omitted_at_default() {
        let json = serde_json::to_value(StyleSpec { tone: Tone::Danger, ..StyleSpec::default() }).expect("serialize");
        assert_eq!(json, serde_json::json!({ "tone": "danger" }));
    }

    #[test]
    fn emphasis_field_omitted_at_default() {
        let json = serde_json::to_value(StyleSpec { emphasis: Emphasis::Strong, ..StyleSpec::default() }).expect("serialize");
        assert_eq!(json, serde_json::json!({ "emphasis": "strong" }));
    }

    #[test]
    fn fully_styled_spec_roundtrips() {
        let spec = StyleSpec { variant: Variant::Ghost, size: SizeToken::Xs, density: Density::Compact, tone: Tone::Success, emphasis: Emphasis::Subtle };
        let json = serde_json::to_string(&spec).expect("serialize");
        let back: StyleSpec = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(spec, back);
    }
}
