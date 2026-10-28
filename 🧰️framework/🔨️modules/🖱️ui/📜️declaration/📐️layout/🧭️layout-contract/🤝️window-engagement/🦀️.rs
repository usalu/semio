//! 🤝️ Window engagement: the engagement options, inputs, status, possible values, controls and slots, the viewport engagement default and the window options. See [`super`].

use super::*;

// #region layout
//#region 🔖️WindowEngagement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagementOption {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<IconName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub pressed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagementInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub on_change: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub on_submit: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub on_repeat_last: Option<ActionDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub on_abort: Option<ActionDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagementStatus {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagementPossible {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagementRingOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagementToggleGroupOption {
    pub id: String,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagementSelectItem {
    pub id: String,
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum WindowEngagementControl {
    Slider { id: Option<String>, label: Option<String>, value: f64, min: f64, max: f64, step: Option<f64>, unit: Option<String>, disabled: Option<bool>, on_change: Option<ActionDescriptor>, on_commit: Option<ActionDescriptor> },
    Stepper { id: Option<String>, label: Option<String>, value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>, unit: Option<String>, disabled: Option<bool>, on_change: Option<ActionDescriptor>, on_commit: Option<ActionDescriptor> },
    Ring { id: Option<String>, label: Option<String>, value: Option<String>, options: Vec<WindowEngagementRingOption>, disabled: Option<bool>, on_select: Option<ActionDescriptor> },
    ToggleGroup { id: Option<String>, label: Option<String>, value: Option<String>, options: Vec<WindowEngagementToggleGroupOption>, disabled: Option<bool>, on_select: Option<ActionDescriptor> },
    Select { id: Option<String>, label: Option<String>, value: Option<String>, placeholder: Option<String>, items: Vec<WindowEngagementSelectItem>, disabled: Option<bool>, on_change: Option<ActionDescriptor> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowEngagement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub session_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<WindowEngagementOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub input: Option<WindowEngagementInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub control: Option<WindowEngagementControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<WindowEngagementControl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<WindowEngagementStatus>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    pub possible_engagements: Option<Vec<WindowEngagementPossible>>,
}

/// 🔌️ Closed replacement for `Option<WindowEngagement>` — makes "this window kind never engages" a
/// named variant instead of `None`, so absence is an explicit, typed state rather than an implicit gap.
/// ⚠️ `WindowEngagement` is a wide variant (nested `Vec`/`Option` fields), making `Some` far
/// larger than `None` — boxing it would be a breaking public-API change (every construction/match
/// site across ~30 plugins would need `Box::new`/deref updates), out of scope for a mechanical pass.
#[allow(clippy::large_enum_variant, reason = "boxing is a breaking public API change, out of T1 scope")]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
#[value(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum WindowEngagementSlot {
    #[default]
    None,
    Some(WindowEngagement),
}

impl WindowEngagementSlot {
    pub fn as_option(&self) -> Option<&WindowEngagement> {
        match self {
            WindowEngagementSlot::None => None,
            WindowEngagementSlot::Some(engagement) => Some(engagement),
        }
    }
}

pub fn default_viewport_engagement() -> WindowEngagement {
    WindowEngagement {
        session_active: Some(true),
        options: None,
        input: None,
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: "framework.viewport.status".into(), text: "Viewport".into() }]),
        possible_engagements: None,
    }
}

/// 🎛️ Everything a window kind can expose beyond its rendered body — always present as a shape,
/// empty collections/`WindowEngagementSlot::None` for windows that don't use a given facet.
/// Replaces the previously separately-optional `measures`/`engagement` pair on `WindowKindDefinition`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WindowOptions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub measures: Vec<WindowMeasure>,
    #[serde(default)]
    #[value(default)]
    pub engagement: WindowEngagementSlot,
}
//#endregion 🔖️WindowEngagement
// #endregion layout
