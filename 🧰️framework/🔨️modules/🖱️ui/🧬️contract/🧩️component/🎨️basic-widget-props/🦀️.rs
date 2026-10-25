//! 🎨️ Basic widget props: the container, text, button, separator, input, select, toggle, key value list, ring, icon select and progress props with their appearances. See [`super`].

use super::*;

//#region 🎨️Props
/// 🌿️ Props for `Component::Container` — the box-with-children component every old `Stack`/
/// `Section`/`Group`/`Field` collapses into (see [`ContainerRole`]). `direction`/`gap`/`padding` do
/// NOT live here — they are `crate::LayoutSpec`, on the record. The old `Field`'s single
/// `child: Box<UiNode>` is simply `children[0]` on the record; there is nothing left to special-case.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ContainerProps {
    #[serde(default, skip_serializing_if = "is_default_container_role")]
    #[value(default, skip_serializing_if = "is_default_container_role")]
    pub role: ContainerRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drop_overlay: Option<DropOverlaySpec>,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_container_role(value: &ContainerRole) -> bool {
    *value == ContainerRole::default()
}

/// 📝️ Props for `Component::Text`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct TextProps {
    pub value: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emphasize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<crate::UiFixedMap<crate::UiText>>,
}

/// 🧩️ Concatenates one packed text leaf: `value` then attribute values in the caller's key order.
pub fn packed_text_leaf(value: &str, attribute_values: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut payload = String::from(value);
    for attribute in attribute_values {
        payload.push_str(attribute.as_ref());
    }
    payload
}

impl TextProps {
    /// 🎒️ Inverse of the 33-slice pack: `value` then ascending `data_attributes`.
    pub fn packed_payload(&self) -> String {
        match &self.data_attributes {
            Some(attributes) => packed_text_leaf(self.value.0.as_str(), attributes.iter().map(|(_, value)| value.as_str())),
            None => packed_text_leaf(self.value.0.as_str(), std::iter::empty::<&str>()),
        }
    }
}

/// 🔘️ Props for `Component::Button`. `action` moved to the record's `bindings` (keyed by
/// `Trigger::Activate`); `style` moved to the record's `crate::StyleSpec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ButtonProps {
    /// 🖼️ Icon key. The old `IconName` is generated per-consuming-crate via a `#[path]` mount (see
    /// `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`), not a publishable
    /// dependency this crate's `Cargo.toml` — which this packet is forbidden from editing — could
    /// take on. A plain `String` icon key is the only viable choice here; flagged as a
    /// registrar-request in `📓️terra-contract-doc-report.md` in case a shared icon crate should
    /// exist instead.
    pub icon: crate::UiText,
    pub label: Label,
}

/// ➖️ Props for `Component::Separator`. Every field the old `UiSeparatorNode` carried
/// (`presence`, `menu`) now lives on the record, so this is intentionally empty — kept as its own
/// struct (rather than a unit variant) purely for structural symmetry with every other component.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SeparatorProps {}

/// ⌨️ Props for `Component::Input`. `on_change` moved to the record's `bindings`
/// (`Trigger::Change`/`Trigger::Commit`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct InputProps {
    #[serde(default, skip_serializing_if = "is_default_input_kind")]
    #[value(default, skip_serializing_if = "is_default_input_kind")]
    pub kind: InputKind,
    pub value: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    /// 🫳️ Commit convention string carried verbatim from the old wire shape (e.g. `"blur"`) — no
    /// closed set of these was found in the fleet, unlike `input_kind`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<crate::UiText>,
    /// 🔣️ Fraction digits a `InputKind::Number` field shows and commits — see [`round_ui_number`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    /// 📌️ Detents of a `InputKind::Number` field under the detent law of [`SliderProps::snaps`] against `min`/`max`
    /// (unbounded when absent): its page keys jump between them ([`ui_number_key_value`]), typing never snaps.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub snaps: crate::UiFixedList<f64>,
    /// 🔁️ A `InputKind::Number` field's `value` is the stored number; it shows and reads `stored × display_factor`
    /// ([`ui_number_display_text`]/[`ui_number_typed_value`]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    /// ⛔️ The hard range a typed number must keep (`min`/`max` themselves when absent), see [`UiNumberLimits`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
    /// 🎯️ Opaque producer-owned identity for retaining an in-progress draft across
    /// revisions while the same semantic edit target remains mounted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub draft_target: Option<crate::UiText>,
    /// 🧾️ Decimal u64 revision of the native document projection that authored this input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub publication_revision: Option<UiPublicationRevision>,
}

impl InputProps {
    /// 🧷️ Whether `snaps` satisfies the detent law against the optional bounds.
    pub fn snaps_are_valid(&self) -> bool {
        crate::snaps_are_valid(self.snaps.iter().copied(), self.min.unwrap_or(f64::NEG_INFINITY), self.max.unwrap_or(f64::INFINITY))
    }

    /// ⚖️ Whether the bounds, display factor and limits agree ([`crate::number_range_is_valid`]).
    pub fn number_range_is_valid(&self) -> bool {
        crate::number_range_is_valid(self.min, self.max, UiNumberScale::Linear, self.display_factor, self.limits.as_ref(), self.snaps.iter().copied())
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_input_kind(value: &InputKind) -> bool {
    *value == InputKind::default()
}

/// 🔽️ Props for `Component::Select`. `on_change` moved to the record's `bindings`. `appearance` only chooses how the one
/// choice is shown ([`SelectAppearance`]); value, items and `Trigger::Change` are the same in both.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct SelectProps {
    pub value: crate::UiText,
    pub items: crate::UiFixedList<SelectItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    #[serde(default, skip_serializing_if = "SelectAppearance::is_menu")]
    #[value(default, skip_serializing_if = "SelectAppearance::is_menu")]
    pub appearance: SelectAppearance,
}

/// 🗳️ How a select shows its one choice: a `menu` that opens its options on demand, or `segmented` — every option
/// visible at once as one row of mutually exclusive buttons (a radio group), for the few-option choices a descriptor
/// declares segmented (`x-semio-ui.widget: "segmented"`). Arrow keys move the choice along a segmented row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum SelectAppearance {
    #[default]
    Menu,
    Segmented,
}

impl SelectAppearance {
    pub fn is_menu(&self) -> bool {
        matches!(self, Self::Menu)
    }
}

/// 🔀️ Props for `Component::Toggle`. `on` is the explicit state this contract adds — the old
/// `UiToggleNode` smuggled it through `presence.selected`, exactly the implicit coupling this
/// contract exists to remove. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ToggleProps {
    #[serde(default, skip_serializing_if = "ToggleAppearance::is_button")]
    #[value(default, skip_serializing_if = "ToggleAppearance::is_button")]
    pub appearance: ToggleAppearance,
    pub on: bool,
    pub icon: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Label>,
}

/// ☑️ Closed binary-control presentation shared by every renderer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub enum ToggleAppearance {
    #[default]
    Button,
    Checkbox,
}

impl ToggleAppearance {
    pub fn is_button(&self) -> bool {
        matches!(self, Self::Button)
    }
}

/// 🗝️ Props for `Component::KeyValueList`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct KeyValueListProps {
    pub entries: crate::UiFixedList<KeyValueEntry>,
}

/// 💍️ Props for `Component::Ring`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct RingProps {
    pub orb_id: crate::UiText,
    pub t: f64,
}

/// 🎴️ Props for `Component::IconSelect`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct IconSelectProps {
    pub value: crate::UiText,
    pub uniform: bool,
    pub classifier_kind: crate::UiText,
}

/// 📶️ Props for `Component::Progress` — a read-only progress bar. `total` absent means indeterminate: a
/// renderer shows a busy sweep and announces no value; present means determinate on `0..=total`, and the
/// percentage is derived by the renderer, never sent. `value_text` is the already-localized spoken form
/// (`aria-valuetext`); plugins never send a percentage string of their own. See
/// `📋️tool-run-contract.md` §2.3 and §2.6 of ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
pub struct ProgressProps {
    pub completed: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    pub value_text: Label,
}

impl ProgressProps {
    /// 📏️ Whether a total is known — the one switch between value attributes and `aria-busy`.
    pub fn is_determinate(&self) -> bool {
        self.total.is_some()
    }
}

/// 📐️ The filled share in `0.0..=1.0` of a bar at `completed` of `total`; `None` while indeterminate. A
/// zero total fills nothing rather than dividing by zero, and an overshoot clamps the fill, never the
/// announced value. Every renderer fills through this one law.
pub fn progress_fraction(completed: f64, total: Option<f64>) -> Option<f64> {
    total.map(|total| if total > 0.0 { (completed / total).clamp(0.0, 1.0) } else { 0.0 })
}
//#endregion 🎨️Props
