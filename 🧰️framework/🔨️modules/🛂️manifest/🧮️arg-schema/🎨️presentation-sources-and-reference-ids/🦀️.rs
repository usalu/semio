//! 🎤️ Argument presentation, snap and option sources, number scale and the reference id type with its text form. See [`super`].

use super::*;

// #region 🛂️Manifest
//#region 🔖️Manifest
//#region 🔖️ArgSchema
/// 🖼️ How to WIDGET-render an argument beyond what its `ArgSchema` alone implies — consumed by
/// `ActionArgDef::control()` (e.g. a bounded `Number` still renders `Slider` without this, but a
/// single-bound one needs it to opt in).
// 🚧️ Needed in serde form too: referenced (directly or transitively) by a `🚧️ BLOCKED` serde-only manifest type above/below — see that type's own docstring.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ArgPresentation {
    Slider,
    Stepper,
    Dial,
    Segmented,
    IconSelect { classifier_kind: String },
    Multiline,
    Hidden,
    /// 🎨️ An sRGB colour: a [`ArgSchema::Vector`] of 3 (RGB) or 4 (RGBA, straight alpha) components within `0..=1`.
    /// The one canonical colour mapping — a hex string stays `text`.
    Color,
}

/// 🧲️ Where a number's snap points come from beyond its static `snaps`: every multiple of its `step`, a window
/// config value named `key` (the grid spacing a tool snaps to), or the document value at JSON `pointer`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SnapSource {
    Step,
    Config { key: String },
    Snapshot { pointer: String },
}

/// 🗝️ Where a string choice's options come from (`x-semio-ui.optionSource`): the KEYS of the object `pointer` leads to in
/// the document the editor previews — an RFC 6901 template whose `{field}` segments take the edited payload's top-level
/// member: an object's key, or, on an array, the record whose own `field` equals it (`/hostSnapshot/widgets/{id}/params`).
/// TypeScript twin: `OptionSource`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum OptionSource {
    Snapshot { pointer: String },
}

/// 📈️ How a slider or dial maps its travel onto the value range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum NumberScale {
    Linear,
    Log,
}

/// 📘️ The JSON type of a reference's ids: text ids (the default, left off the wire) or integer ids. A selection id is
/// always text, so an integer reference stages the integer its selected text spells ([`ReferenceIdType::id_value`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ReferenceIdType {
    #[default]
    String,
    Integer,
}

impl ReferenceIdType {
    /// 🔤️ Whether the ids are text — the default.
    pub fn is_string(&self) -> bool {
        *self == Self::String
    }

    /// 🎯️ The payload value of one selected id: the text itself, or the integer it spells — `None` for empty text or,
    /// for an integer reference, text that is not a decimal integer within ±(2^53 − 1). TS twin: `referenceIdValue`.
    pub fn id_value(self, id: &str) -> Option<DslValue> {
        if id.is_empty() {
            return None;
        }
        match self {
            Self::String => Some(DslValue::String(id.to_string())),
            Self::Integer => {
                let digits = id.strip_prefix('-').unwrap_or(id);
                if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
                    return None;
                }
                let value = id.parse::<i64>().ok().filter(|value| value.unsigned_abs() <= REFERENCE_ID_INTEGER_MAX)?;
                Some(if value < 0 { DslValue::int(value) } else { DslValue::uint(value as u64) })
            }
        }
    }
}

/// 🔢️ The largest integer id magnitude a reference admits — the exact-integer range every host's numbers share.
pub const REFERENCE_ID_INTEGER_MAX: u64 = (1 << 53) - 1;

/// 🏷️ The text of one reference id value — a non-empty string id as it is, an integer id in decimal (the spelling a
/// selection carries); `None` for anything else. TS twin: `referenceIdText`.
pub fn reference_id_text(value: &DslValue) -> Option<String> {
    match value {
        DslValue::String(id) if !id.is_empty() => Some(id.clone()),
        DslValue::Number(number) => match (number.as_i64(), number.as_u64()) {
            (Some(value), _) if value.unsigned_abs() <= REFERENCE_ID_INTEGER_MAX => Some(value.to_string()),
            (None, Some(value)) if value <= REFERENCE_ID_INTEGER_MAX => Some(value.to_string()),
            _ => None,
        },
        _ => None,
    }
}
//#endregion 🔖️ArgSchema
//#endregion 🔖️Manifest
// #endregion 🛂️Manifest
