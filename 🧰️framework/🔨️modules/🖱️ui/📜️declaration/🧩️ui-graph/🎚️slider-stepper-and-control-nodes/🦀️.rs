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

/// 🖼️ The editor tab an icon string belongs to — React's `IconSelectorMode` (`🔣️Icons/🟦️.tsx`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconSelectorMode {
    Url,
    Shortcode,
    Data,
    Emoji,
    Math,
    Text,
    Vector,
}

impl IconSelectorMode {
    /// 🗂️ The seven modes in React's select order.
    pub const ALL: [Self; 7] = [Self::Url, Self::Shortcode, Self::Math, Self::Data, Self::Emoji, Self::Text, Self::Vector];

    /// 🏷️ The mode a wire id names.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|mode| mode.id() == id)
    }

    /// 📏️ The editor's rows: React's `rows={data|vector ? 5 : 4}`.
    pub const fn editor_rows(self) -> u8 {
        match self {
            Self::Data | Self::Vector => 5,
            _ => 4,
        }
    }

    /// 🏷️ The mode's wire id (`url`, `shortcode`, `data`, `emoji`, `math`, `text`, `vector`).
    pub const fn id(self) -> &'static str {
        match self {
            Self::Url => "url",
            Self::Shortcode => "shortcode",
            Self::Data => "data",
            Self::Emoji => "emoji",
            Self::Math => "math",
            Self::Text => "text",
            Self::Vector => "vector",
        }
    }
}

impl UiIconSelectNode {
    /// 🔤️ React's `decodeIcon` reduced to what the selector shows: the mode a stored icon string belongs to and the text its editor holds
    /// (`innerFromIconForSelectorMode`); a catalog name is `isIconName`, an undecodable string is `Math` with an empty editor.
    pub fn decode_mode(raw: &str) -> (IconSelectorMode, String) {
        use IconSelectorMode::*;
        let t = raw.trim();
        let lower = t.to_lowercase();
        let rest = |prefix: &str| t[prefix.len()..].trim().to_string();
        let non_empty = |mode: IconSelectorMode, inner: String| if inner.is_empty() { (Math, String::new()) } else { (mode, inner) };
        if t.is_empty() {
            (Math, String::new())
        } else if t.starts_with("url:") {
            non_empty(Url, rest("url:"))
        } else if lower.starts_with("http://") || lower.starts_with("https://") {
            (Url, t.to_string())
        } else if t.len() >= 3 && t.starts_with(':') && t.ends_with(':') && t[1..t.len() - 1].chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '+' | '-')) {
            (Shortcode, t[1..t.len() - 1].to_string())
        } else if t.starts_with("typst:") {
            non_empty(Math, rest("typst:"))
        } else if t.starts_with('$') {
            (Math, t.to_string())
        } else if t.starts_with("emoji:") {
            non_empty(Emoji, rest("emoji:"))
        } else if t.starts_with("text:") {
            non_empty(Text, rest("text:"))
        } else if lower.starts_with("data:") {
            (Data, t.to_string())
        } else if lower.starts_with("<?xml") || lower.contains("<svg") {
            (Vector, t.to_string())
        } else if IconName::from_str(t).is_some() {
            (Vector, t.to_string())
        } else if t.chars().all(|c| c.is_alphanumeric() || matches!(c, '_' | '.' | '-')) && (t.chars().any(|c| matches!(c, '.' | '_' | '-')) || t.chars().count() > 48) {
            (Math, String::new())
        } else if t.chars().any(|c| matches!(c as u32, 0xA9 | 0xAE | 0x203C | 0x2049 | 0x2122 | 0x2139 | 0x2194..=0x2199 | 0x21A9 | 0x21AA | 0x231A | 0x231B | 0x23E9..=0x23F3 | 0x23F8..=0x23FA | 0x24C2 | 0x25AA..=0x25FE | 0x2600..=0x27BF | 0x2934 | 0x2935 | 0x2B1B | 0x2B1C | 0x2B50 | 0x2B55 | 0x3030 | 0x303D | 0x3297 | 0x3299 | 0x1F000..=0x1FAFF)) {
            (Emoji, t.to_string())
        } else if t.chars().count() <= 16 {
            (Text, t.to_string())
        } else {
            (Math, String::new())
        }
    }

    /// 🎚️ Whether the selector is locked: React's `disabled || !uniform`.
    pub fn locked(&self) -> bool {
        self.presence.state == UiState::Disabled || !self.uniform
    }

    /// ✏️ The text the editor shows: empty for a mixed selection, else the stored icon's editor text.
    pub fn editor_text(&self) -> String {
        if self.uniform { Self::decode_mode(&self.value).1 } else { String::new() }
    }

    /// 📡️ React's `emitIconKindFromSelectorMode`: the icon string an editor `inner` text stores under `mode`.
    pub fn encode_for_mode(mode: IconSelectorMode, inner: &str) -> String {
        let inner = inner.trim();
        match (inner.is_empty(), mode) {
            (true, _) => String::new(),
            (_, IconSelectorMode::Url) => format!("url:{inner}"),
            (_, IconSelectorMode::Shortcode) => format!(":{}:", inner.trim_matches(':')),
            (_, IconSelectorMode::Emoji) => format!("emoji:{inner}"),
            (_, IconSelectorMode::Math) if inner.starts_with('$') => inner.to_string(),
            (_, IconSelectorMode::Math) => format!("typst:{inner}"),
            (_, IconSelectorMode::Text) => format!("text:{inner}"),
            (_, IconSelectorMode::Data | IconSelectorMode::Vector) => inner.to_string(),
        }
    }

    /// ✏️ The icon string a typed editor `text` commits: encoded under the mode the stored value classifies as.
    pub fn commit_text(&self, text: &str) -> String {
        Self::encode_for_mode(Self::decode_mode(&self.value).0, text)
    }

    /// 🔀️ React's `migrateIconKindToIconSelectorMode`: what choosing `mode` in the select stores for the icon string `prev` — `prev`
    /// itself when it already is that mode, `""` for data/vector/url/shortcode (they cannot be derived), else the neutral text
    /// (typst/emoji prefix stripped) re-encoded under the mode.
    pub fn migrate_mode(prev: &str, mode: IconSelectorMode) -> String {
        if Self::decode_mode(prev).0 == mode {
            return prev.to_string();
        }
        match mode {
            IconSelectorMode::Data | IconSelectorMode::Vector | IconSelectorMode::Url | IconSelectorMode::Shortcode => String::new(),
            IconSelectorMode::Math | IconSelectorMode::Emoji | IconSelectorMode::Text => {
                let t = prev.trim();
                let neutral = t.strip_prefix("typst:").or_else(|| t.strip_prefix("emoji:")).unwrap_or(t).trim();
                Self::encode_for_mode(mode, neutral)
            }
        }
    }

    /// 📂️ The file types React's `IconSelector` file input accepts.
    pub const IMPORT_ACCEPT: &'static str = "image/png,image/jpeg,image/webp,image/gif,image/svg+xml,.svg,.png,.jpg,.jpeg,.webp,.gif";

    /// 📂️ The shell verb (on the `framework` controller) the Import button fires: it carries this selector's own change action as
    /// `controllerId`/`action`/`args`, so the shell can open its file picker and answer with `{value}` on that action.
    pub const IMPORT_ACTION: &'static str = "importIconFile";

    /// 📂️ The Import button's action: [`Self::IMPORT_ACTION`] wrapping this selector's change action.
    pub fn import_action(&self) -> ActionDescriptor {
        let mut fields = vec![("controllerId".to_string(), DslValue::String(self.on_change.controller_id.clone())), ("action".to_string(), DslValue::String(self.on_change.action.clone()))];
        if let Some(args) = &self.on_change.args {
            fields.push(("args".to_string(), args.clone()));
        }
        ActionDescriptor { controller_id: "framework".into(), action: Self::IMPORT_ACTION.into(), args: Some(DslValue::Object(fields)) }
    }

    /// 📂️ The change action an [`Self::import_action`] wraps, answered with the picked file's `value` merged over its authored args
    /// (React's `onChange(text)`); `None` for any other action.
    pub fn imported_value_action(import: &ActionDescriptor, value: &str) -> Option<ActionDescriptor> {
        let args = import.args.as_ref()?;
        let mut fields = match args.get("args") {
            Some(DslValue::Object(fields)) => fields.clone(),
            _ => Vec::new(),
        };
        fields.retain(|(key, _)| key != "value");
        fields.push(("value".to_string(), DslValue::String(value.to_string())));
        Some(ActionDescriptor { controller_id: args.get("controllerId")?.as_str()?.to_string(), action: args.get("action")?.as_str()?.to_string(), args: Some(DslValue::Object(fields)) })
    }
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

impl UiInputNode {
    /// 🔖️ The commit policy prefix of an icon selector's editor: its typed text is encoded under the named mode.
    pub const ICON_MODE_COMMIT: &'static str = "iconMode:";

    /// 🖼️ The mode an icon selector's editor encodes under, read from its commit policy.
    pub fn icon_mode(&self) -> Option<IconSelectorMode> {
        self.commit.as_deref()?.strip_prefix(Self::ICON_MODE_COMMIT).and_then(IconSelectorMode::from_id)
    }

    /// 🔠️ React's `font-mono` editor: the icon selector's textarea is set in the mono face.
    pub fn uses_mono_face(&self) -> bool {
        self.icon_mode().is_some()
    }

    /// 📎️ The shell verb (on the `framework` controller) a `file` input's press fires: it carries this input's own change action as `controllerId`/`action`/`args`, so the shell can open its file picker and answer with `{value}` — the picked file's NAME, as React's `InputView` commits `files[0].name` — on that action.
    pub const PICK_FILE_ACTION: &'static str = "pickInputFile";

    /// 📎️ [`Self::PICK_FILE_ACTION`] wrapping this input's change action and its `accept` filter; `None` for any kind but `file`.
    pub fn pick_file_action(&self) -> Option<ActionDescriptor> {
        if self.input_kind != "file" {
            return None;
        }
        let mut fields = vec![("controllerId".to_string(), DslValue::String(self.on_change.controller_id.clone())), ("action".to_string(), DslValue::String(self.on_change.action.clone())), ("accept".to_string(), DslValue::String(self.accept.clone().unwrap_or_default()))];
        if let Some(args) = &self.on_change.args {
            fields.push(("args".to_string(), args.clone()));
        }
        Some(ActionDescriptor { controller_id: "framework".into(), action: Self::PICK_FILE_ACTION.into(), args: Some(DslValue::Object(fields)) })
    }
}
