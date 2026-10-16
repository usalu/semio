//! 🔖️ Semantic labels and exact committed publication revisions.
use serde::{Serialize,Deserialize};
use semio_framework_value_derive::{ToValue,FromValue};
/// 🧾️ Exact native committed-document revision projected onto the u64 completion lane.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct UiPublicationRevision(pub u64);

impl UiPublicationRevision {
    pub fn decimal(self) -> String { self.0.to_string() }
}

impl Serialize for UiPublicationRevision {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> { serializer.serialize_str(&self.0.to_string()) }
}

impl<'de> Deserialize<'de> for UiPublicationRevision {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value == "0" || (!value.starts_with('0') && value.bytes().all(|byte| byte.is_ascii_digit())) {
            return value.parse::<u64>().map(Self).map_err(serde::de::Error::custom);
        }
        Err(serde::de::Error::custom("publication revision must be canonical decimal u64"))
    }
}

impl ::semio_framework_value::ToValue for UiPublicationRevision {
    fn to_value(&self) -> ::semio_framework_value::DslValue { ::semio_framework_value::DslValue::String(self.0.to_string()) }
}

impl ::semio_framework_value::FromValue for UiPublicationRevision {
    fn from_value(value: ::semio_framework_value::DslValue) -> Result<Self, ::semio_framework_value::ValueError> {
        let ::semio_framework_value::DslValue::String(value) = value else {
            return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "publication revision must be a decimal string"));
        };
        if value != "0" && (value.starts_with('0') || !value.bytes().all(|byte| byte.is_ascii_digit())) {
            return Err(::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "publication revision must be canonical decimal u64"));
        }
        value.parse::<u64>().map(Self).map_err(|_| ::semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue, "publication revision must fit u64"))
    }
}

//#region 🔖️Component

//#region 🏷️Label
/// 🏷️ Display-ready UI text carried on the wire.
///
/// ⚠️ Decision (flagged per packet brief): the old `UiNode`'s `Label` (`crate::wgpu::Label` in
/// `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs`) is NOT reused here. It
/// is defined inside the old wgpu-target UI package, imports that package's own `Locale`/
/// `Terminology` axes, and its whole point (`From<LabelText>` wired to the `app_labels!` macro,
/// no `From<&str>`) is a compile-time-checked-label enforcement mechanism that lives at the
/// authoring boundary, not the wire boundary — and this crate must not depend on that package at
/// all (no engine, no wgpu; see `🦀️.rs`). This is therefore a minimal, independent transparent
/// string. The localization/terminology resolution that used to happen via `LabelText::fill` still
/// happens upstream of the runtime (manifest/host), before a `Label` ever reaches this contract.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(crate = "::semio_framework_value", transparent)]
pub struct Label(pub crate::UiText);

impl TryFrom<String> for Label {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::UiText::try_from_string(value).map(Self)
    }
}

impl<'a> TryFrom<&'a str> for Label {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        crate::UiText::try_from_str(value).map(Self).ok_or(value)
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
//#endregion 🏷️Label

//#region 🎛️Enums
/// 🌿️ The closed set of roles a [`Component::Container`] plays — collapses the old `Stack`/
/// `Section`/`Group`/`Field` variants (all four were "a box with children plus optional chrome") into
/// one component, distinguished only by role. `Form`/`Toolbar` are new roles with no old-`UiNode`

/// 🎞️ The transient visual emphasis a node is entering — orthogonal to `activity`/`disabled`. A node
/// carrying neither is in its steady state; the renderer clears this once the transition has played.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum TransitionHint {
    Introducing,
    Celebrating,
}

