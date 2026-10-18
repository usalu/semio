//! 🎚️ Slider, stepper and control nodes: the slider, number stepper, ring, icon select and progress nodes and the `UiControlNode` union. See [`super`].

use super::*;

// #region ui
//#region 🔖Primitives
/// 🧿️ A retained slider or dial — the contract's `SliderProps`: values are stored values, the readout, the spoken value
/// and a typed value are display values (`display_factor`, `precision`, `display_unit` else `unit`), a typed value is
/// refused by `limits` (the travel itself when absent).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiSliderNode {
    pub id: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 🧲️ Detents — the contract's `SliderProps.snaps`, painted as ticks and applied by the shared law.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub snaps: Vec<f64>,
    /// 🎡️ A straight track or a rotary dial — the contract's `SliderProps.appearance`.
    #[serde(default, skip_serializing_if = "ui_contract::SliderAppearance::is_track")]
    #[value(default, skip_serializing_if = "ui_contract::SliderAppearance::is_track")]
    pub appearance: ui_contract::SliderAppearance,
    /// 📈️ A linear or log axis — the contract's `SliderProps.scale`.
    #[serde(default, skip_serializing_if = "ui_contract::UiNumberScale::is_linear")]
    #[value(default, skip_serializing_if = "ui_contract::UiNumberScale::is_linear")]
    pub scale: ui_contract::UiNumberScale,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ui_contract::UiNumberLimits>,
    pub on_change: ActionDescriptor,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

impl UiSliderNode {
    /// 🔤️ The readout of a stored `value`: its display text (`stored × display_factor` at `precision`, else twelve digits).
    pub fn readout(&self, value: f64) -> String {
        ui_contract::ui_number_display_text(value, self.display_factor, self.precision)
    }

    /// 🏷️ The unit the slider shows — `display_unit`, else `unit`.
    pub fn shown_unit(&self) -> Option<&str> {
        self.display_unit.as_deref().or(self.unit.as_deref()).filter(|unit| !unit.is_empty())
    }

    /// 🪧️ React's external readout beside a slider with a unit: the declared value's display text and the shown unit.
    pub fn unit_label(&self) -> Option<String> {
        self.shown_unit().map(|unit| format!("{} {unit}", self.readout(self.value)))
    }

    /// 🪐️ The travel's axis — linear or log — as the contract's axis law reads it.
    pub fn axis_position(&self, value: f64) -> f64 {
        ui_contract::slider_axis_position(value, self.min, self.max, self.scale)
    }
}

/// 🪜️ A retained number stepper — the contract's `NumberStepperProps`, its detents, shown unit (`display_unit` else
/// `unit`), display factor and limits included.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiNumberStepperNode {
    pub id: String,
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /// 🎯️ Fraction digits it shows and commits — the contract's `NumberStepperProps.precision`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    /// 📍️ Detents its page keys stop on — the contract's `NumberStepperProps.snaps`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub snaps: Vec<f64>,
    /// 🫧️ The unit shown beside the value (the contract's `display_unit`, else `unit`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ui_contract::UiNumberLimits>,
    pub on_absolute: ActionDescriptor,
    pub on_delta: ActionDescriptor,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

impl UiNumberStepperNode {
    /// 🛰️ The text a stored `value` shows: its display text, and the shown unit when the stepper has one.
    pub fn value_text(&self, value: f64) -> String {
        let text = ui_contract::ui_number_display_text(value, self.display_factor, self.precision);
        self.unit.as_deref().filter(|unit| !unit.is_empty()).map_or_else(|| text.clone(), |unit| format!("{text} {unit}"))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiRingNode {
    pub id: String,
    pub orb_id: String,
    pub t: f64,
    pub on_change: ActionDescriptor,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiIconSelectNode {
    pub id: String,
    pub value: String,
    pub uniform: bool,
    pub classifier_kind: String,
    pub on_change: ActionDescriptor,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

/// 📶️ The retained twin of `ui_contract::Component::Progress`: a read-only bar, determinate on
/// `0..=total` or an indeterminate sweep while `total` is `None`. `value_text` is the localized spoken
/// form the accessibility projection announces; paint never invents a percentage string.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UiProgressNode {
    pub id: String,
    pub completed: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    pub value_text: Label,
    #[serde(default, skip_serializing_if = "UiPresence::is_default")]
    #[value(default, skip_serializing_if = "UiPresence::is_default")]
    pub presence: UiPresence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<UiMenuRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "type", rename_all = "camelCase")]
#[value(tag = "type", rename_all = "camelCase")]
pub enum UiControlNode {
    Input(UiInputNode),
    Select(UiSelectNode),
    Toggle(UiToggleNode),
    Button(UiButtonNode),
    KeyValue(UiKeyValueNode),
    Slider(UiSliderNode),
    NumberStepper(UiNumberStepperNode),
    Ring(UiRingNode),
    IconSelect(UiIconSelectNode),
}

impl UiControlNode {
    /// 🧭️ Exhaustive accessor — a new control variant fails to compile here until wired.
    /// `&UiPresence` (ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-
    /// CREATION C7.6): `UiPresence` gained `peers: Vec<UiPeerMark>` and lost `Copy`.
    pub fn presence(&self) -> &UiPresence {
        match self {
            UiControlNode::Input(n) => &n.presence,
            UiControlNode::Select(n) => &n.presence,
            UiControlNode::Toggle(n) => &n.presence,
            UiControlNode::Button(n) => &n.presence,
            UiControlNode::KeyValue(n) => &n.presence,
            UiControlNode::Slider(n) => &n.presence,
            UiControlNode::NumberStepper(n) => &n.presence,
            UiControlNode::Ring(n) => &n.presence,
            UiControlNode::IconSelect(n) => &n.presence,
        }
    }
    pub fn presence_mut(&mut self) -> &mut UiPresence {
        match self {
            UiControlNode::Input(n) => &mut n.presence,
            UiControlNode::Select(n) => &mut n.presence,
            UiControlNode::Toggle(n) => &mut n.presence,
            UiControlNode::Button(n) => &mut n.presence,
            UiControlNode::KeyValue(n) => &mut n.presence,
            UiControlNode::Slider(n) => &mut n.presence,
            UiControlNode::NumberStepper(n) => &mut n.presence,
            UiControlNode::Ring(n) => &mut n.presence,
            UiControlNode::IconSelect(n) => &mut n.presence,
        }
    }
}
//#endregion 🔖️Primitives
// #endregion ui
