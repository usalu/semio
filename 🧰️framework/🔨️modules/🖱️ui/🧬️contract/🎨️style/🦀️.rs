//! @emoji 🎨️ Design-token `StyleSpec` — closed enums over ui_styling tokens, never raw values.
//!
//! ⚠️ SCAFFOLD — owned by packet `contract-layout`. Replace this placeholder wholesale; keep the region
//! structure and the U1 sync rule (no `async fn` in this crate).

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

//#region 🔖️Style

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// 🖌️ The chrome treatment a renderer paints a component with — independent of [`Tone`] (which color
/// role) and [`Emphasis`] (how prominent).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum Emphasis {
    Subtle,
    #[default]
    Regular,
    Strong,
}

/// 🎨️ A node's design-token styling — five closed enums, never a raw color or a raw pixel value. A
/// renderer resolves every field against the active theme; this struct only names the tokens. Each
/// field is omitted from the wire at its default, so a default-styled node costs nothing to encode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct StyleSpec {
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub variant: Variant,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub size: SizeToken,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub density: Density,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub tone: Tone,
    #[serde(default, skip_serializing_if = "is_default")]
    #[value(default, skip_serializing_if = "is_default")]
    pub emphasis: Emphasis,
}
//#endregion 🔖️Style

#[cfg(test)]
#[path = "../🧪️tests/🔬️style-unit/🦀️.rs"]
mod tests;
