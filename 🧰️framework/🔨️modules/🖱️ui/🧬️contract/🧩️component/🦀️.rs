//! 🧩️ The semantic `Component` enum and its per-component prop structs — the closed set of
//! things a [`crate::UiNodeRecord`] can render. Every prop struct carries only the data specific to
//! that component: identity lives on the record (`key`), actions live on the record (`bindings`),
//! visual state lives on the record (`activity`/`disabled`/`transition`) or its `layout`/`style`. A
//! prop struct that inlined any of those would be reintroducing the implicit coupling this contract
//! replaces.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS. `RowTarget`,
// `TreeItemProps`, `TableRowProps`, `ExtensionProps` and the `Component` enum itself are the deliberate
// exception — each embeds `crate::UiValue`/`crate::UiMap` (directly or via `RowTarget`), which stay
// DslValue-free by construction (see `UiValue`'s own docstring in `🎬️action.rs`).
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

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
#[value(crate = "::protocol::value", transparent)]
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
/// counterpart, added per the packet brief for the layouts those two collapsed variants could not
/// previously express as a single node.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum ContainerRole {
    #[default]
    Plain,
    Section,
    Group,
    Field,
    Form,
    Toolbar,
}

/// ⌨️ The closed set of input kinds. Grepped from the fleet's actual `UiInputNode.input_kind` string
/// literals (`"text"`, `"textarea"`, `"number"`, `"file"`) plus the values the React `🟦️Interpreter`
/// already branches on for `inputKind` (`"number"`, `"longText"`, `"date"`, `"color"`, `"file"`,
/// default `"text"`) — the union of both sides, since the renderer supports more kinds than any
/// current plugin emits yet. Note the old Rust/TS spelling mismatch this closes: Rust emitted
/// `"textarea"`, TS checked `"longText"`; this enum has exactly one spelling (`LongText`) for that
/// kind. No `Search` kind exists anywhere in the fleet today, so it is not included (see the
/// packet's own report for the grep evidence — adding it back is a one-variant change, not a design
/// change, if a plugin needs it later).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum InputKind {
    #[default]
    Text,
    LongText,
    Number,
    Date,
    Color,
    File,
}

/// 📍️ Where a [`RowAction`] paints: on the tree row itself, or folded into the row's context menu.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum RowActionPlacement {
    #[default]
    Row,
    Menu,
}
//#endregion 🎛️Enums

//#region 🧱️Nested
/// 📥️ Hover-state copy for a [`ContainerProps::drop_overlay`] — shown while a drag is over the
/// container, ahead of its `Drop`-triggered [`crate::ActionBinding`] firing on release.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct DropOverlaySpec {
    pub title: Label,
    pub hint: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<crate::UiText>,
}

/// 🔽️ One option of a [`Component::Select`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct SelectItem {
    pub value: crate::UiText,
    pub label: Label,
}

/// 🗝️ One row of a [`Component::KeyValueList`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct KeyValueEntry {
    pub label: Label,
    pub value: crate::UiText,
}

/// 🎬️ One action affordance painted on (or reachable from) a [`Component::TreeItem`] or
/// [`Component::TableRow`] row. It names only its `verb`: the scope, version and argument map are the
/// row's ONE [`RowTarget`], which every row action and the row's activation inherit, so N actions cost N
/// verbs, never N argument maps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct RowAction {
    /// 🖼️ Icon key. See [`ButtonProps::icon`] for why this is a plain `String`, not a closed enum.
    pub icon: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    /// ▶️ The verb this action fires on the row's [`RowTarget`].
    pub verb: crate::UiText,
    #[serde(default, skip_serializing_if = "is_default_row_action_placement")]
    #[value(default, skip_serializing_if = "is_default_row_action_placement")]
    pub placement: RowActionPlacement,
    /// 🚫️ A disabled action paints and announces disabled and never dispatches ([`RowTarget::action_binding`] refuses it
    /// typed). Only a disabled action carries the flag, so an enabled row costs nothing for it.
    #[serde(default, skip_serializing_if = "is_enabled_row_action")]
    #[value(default, skip_serializing_if = "is_enabled_row_action")]
    pub disabled: bool,
}

impl RowAction {
    /// 🚫️ This action with its enabled state set — `true` paints it disabled and refuses its dispatch.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// 🚫️ Why a row action does not dispatch — the ONE typed refusal every host answers a row verb with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowActionRefusal {
    /// 🚫️ The action is disabled on this row.
    Disabled,
    /// 🧮️ The target's argument map could not be admitted into one more binding.
    Credit,
}

/// 🎯️ The ONE action target of a row: the controller `scope` at its contract `version` that every
/// [`RowAction::verb`] and the row's `activation` fire in, the ONE argument map they all inherit, and the
/// verb the row's primary activation (open, select) fires. A host dispatches a row verb as
/// [`RowTarget::binding`] — the same [`crate::ActionBinding`] shape any record binding has, so a tree row
/// and a table row with the same target dispatch identically. `args` is a [`crate::UiMap`] handle, not an
/// inline [`crate::UiValue`], so a targeted row costs one scope and one activation verb inline.
// 🌱️ No `ToValue`/`FromValue` here: `args` embeds `UiMap`, the deliberate DslValue-free exception —
// see `UiValue`'s docstring in `🎬️action.rs`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowTarget {
    pub scope: crate::UiText,
    pub version: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<crate::UiMap>,
    /// ▶️ The verb the row's primary activation fires on this target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<crate::UiText>,
}

impl RowTarget {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self { scope: self.scope.clone(), version: self.version, args: credited_args(&self.args)?, activation: self.activation.clone() })
    }

    /// 🔗️ The `Trigger::Activate` binding `verb` fires on this target: its scope and version, its argument map.
    pub fn binding(&self, verb: &crate::UiText) -> Option<crate::ActionBinding> {
        Some(crate::ActionBinding { trigger: crate::Trigger::Activate, action: crate::ActionId::new(self.scope.clone(), verb.clone(), self.version), args: credited_args(&self.args)?.map(crate::UiValue::Map), capability: None })
    }

    /// 🎬️ The versioned id one of the row's actions fires on this target — refused typed when the action is disabled, so a
    /// host never dispatches what the row paints disabled.
    pub fn action_id(&self, action: &RowAction) -> Result<crate::ActionId, RowActionRefusal> {
        if action.disabled {
            return Err(RowActionRefusal::Disabled);
        }
        Ok(crate::ActionId::new(self.scope.clone(), action.verb.clone(), self.version))
    }

    /// 🔗️ The binding one of the row's actions fires on this target: [`Self::action_id`] with the target's argument map.
    pub fn action_binding(&self, action: &RowAction) -> Result<crate::ActionBinding, RowActionRefusal> {
        let action = self.action_id(action)?;
        Ok(crate::ActionBinding { trigger: crate::Trigger::Activate, action, args: credited_args(&self.args).ok_or(RowActionRefusal::Credit)?.map(crate::UiValue::Map), capability: None })
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn credited_args(args: &Option<crate::UiMap>) -> Option<Option<crate::UiMap>> {
    match args {
        Some(args) => Some(Some(args.credited_clone()?)),
        None => Some(None),
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_row_action_placement(value: &RowActionPlacement) -> bool {
    *value == RowActionPlacement::default()
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_enabled_row_action(disabled: &bool) -> bool {
    !*disabled
}
//#endregion 🧱️Nested

//#region 🎨️Props
/// 🌿️ Props for `Component::Container` — the box-with-children component every old `Stack`/
/// `Section`/`Group`/`Field` collapses into (see [`ContainerRole`]). `direction`/`gap`/`padding` do
/// NOT live here — they are `crate::LayoutSpec`, on the record. The old `Field`'s single
/// `child: Box<UiNode>` is simply `children[0]` on the record; there is nothing left to special-case.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
    /// 🧩️ Inverse of the 33-slice pack: `value` then ascending `data_attributes`.
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
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[value(crate = "::protocol::value")]
pub struct SeparatorProps {}

/// ⌨️ Props for `Component::Input`. `on_change` moved to the record's `bindings`
/// (`Trigger::Change`/`Trigger::Commit`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
}

impl InputProps {
    /// 🧷️ Whether `snaps` satisfies the detent law against the optional bounds.
    pub fn snaps_are_valid(&self) -> bool {
        crate::snaps_are_valid(self.snaps.iter().copied(), self.min.unwrap_or(f64::NEG_INFINITY), self.max.unwrap_or(f64::INFINITY))
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_input_kind(value: &InputKind) -> bool {
    *value == InputKind::default()
}

/// 🔽️ Props for `Component::Select`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct SelectProps {
    pub value: crate::UiText,
    pub items: crate::UiFixedList<SelectItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
}

/// 🔀️ Props for `Component::Toggle`. `on` is the explicit state this contract adds — the old
/// `UiToggleNode` smuggled it through `presence.selected`, exactly the implicit coupling this
/// contract exists to remove. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct KeyValueListProps {
    pub entries: crate::UiFixedList<KeyValueEntry>,
}

/// 🎚️ Props for `Component::Slider`. `on_change` moved to the record's `bindings`. `snaps` are the
/// slider's detents: strictly ascending, finite, inside `min..=max`, at most
/// [`crate::UI_FIXED_LIST_ITEMS`] of them — every renderer paints one tick per snap and resolves a
/// pointer value through [`slider_pointer_value`] and a key through [`slider_key_value`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct SliderProps {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub snaps: crate::UiFixedList<f64>,
}

impl SliderProps {
    /// 🧲️ Whether `snaps` satisfies the detent law: finite, strictly ascending, inside the bounds.
    pub fn snaps_are_valid(&self) -> bool {
        crate::snaps_are_valid(self.snaps.iter().copied(), self.min, self.max)
    }
}

/// 🔢️ Props for `Component::NumberStepper`. `on_absolute`/`on_delta` both moved to the record's
/// `bindings`, distinguished by `Trigger`. `precision` is the fraction digits it shows and commits.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct NumberStepperProps {
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
}

/// 🧿️ Share of a slider's span within which a pointer value is pulled onto a detent. Keyboard steps
/// never snap (a stuck arrow key is worse than a missed detent); page keys jump between detents.
pub const SLIDER_SNAP_RADIUS: f64 = 0.03;

/// 🔬️ The largest [`InputProps::precision`]/[`NumberStepperProps::precision`] a renderer honours —
/// the fraction digits an `f64` still carries meaningfully.
pub const UI_NUMBER_PRECISION_MAX: u16 = 15;

/// 👆️ Resolves a raw pointer value: clamped to the bounds, quantized onto the `step` ladder from
/// `min`, then pulled onto the nearest snap when it lies within [`SLIDER_SNAP_RADIUS`] of the span.
pub fn slider_pointer_value(value: f64, min: f64, max: f64, step: f64, snaps: impl IntoIterator<Item = f64>) -> f64 {
    let clamped = value.clamp(min, max.max(min));
    let stepped = if step > 0.0 { (min + ((clamped - min) / step).round() * step).clamp(min, max.max(min)) } else { clamped };
    let radius = (max - min).abs() * SLIDER_SNAP_RADIUS;
    snaps.into_iter().map(|snap| (snap, (snap - clamped).abs())).filter(|(_, distance)| *distance <= radius).min_by(|left, right| left.1.total_cmp(&right.1)).map_or(stepped, |(snap, _)| snap)
}

/// ⏭️ The next detent strictly above (`forward`) or below `current` — what a page key jumps to.
pub fn slider_adjacent_snap(current: f64, snaps: impl IntoIterator<Item = f64>, forward: bool) -> Option<f64> {
    let candidates = snaps.into_iter().filter(|snap| if forward { *snap > current } else { *snap < current });
    if forward {
        candidates.min_by(f64::total_cmp)
    } else {
        candidates.max_by(f64::total_cmp)
    }
}

/// 📄️ How many ladder rungs a large arrow (`Shift`) or a page key with no detent ahead moves a slider.
pub const SLIDER_PAGE_STEPS: f64 = 10.0;

/// 🎹️ The keys a slider takes, once a renderer has mapped its physical key (direction, `dir`, inversion) onto
/// the value axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliderKey {
    Decrement,
    Increment,
    PageDown,
    PageUp,
    Home,
    End,
}

/// 🪜️ The keyboard law every renderer shares for a bounded slider — [`ui_number_key_value`] with both bounds.
pub fn slider_key_value(current: f64, min: f64, max: f64, step: f64, snaps: impl IntoIterator<Item = f64>, key: SliderKey, large: bool) -> f64 {
    ui_number_key_value(current, Some(min), Some(max), step, snaps, key, large)
}

/// ⌨️ The keyboard law of every numeric control. Arrows walk the step ladder from `min` (from 0 without one;
/// `large` walks [`SLIDER_PAGE_STEPS`] rungs) and never snap, so a detent never traps them; from an off-ladder
/// value the first rung beyond it is one rung. Page keys jump to the adjacent detent, else walk
/// [`SLIDER_PAGE_STEPS`] rungs. Home and End go to the bounds (and keep the value without one). An invalid step
/// walks rungs of one. The result is clamped to the bounds and cleaned to the decimals of the ladder origin and
/// `step`, so `0.2 + 0.1` lands on `0.3`.
pub fn ui_number_key_value(current: f64, min: Option<f64>, max: Option<f64>, step: f64, snaps: impl IntoIterator<Item = f64>, key: SliderKey, large: bool) -> f64 {
    let min = min.filter(|min| min.is_finite());
    let max = max.filter(|max| max.is_finite()).map(|max| min.map_or(max, |min| max.max(min)));
    let clamp = |value: f64| value.max(min.unwrap_or(f64::NEG_INFINITY)).min(max.unwrap_or(f64::INFINITY));
    let origin = min.unwrap_or(0.0);
    let step = if step.is_finite() && step > 0.0 { step } else { 1.0 };
    let digits = decimal_digits(origin).max(decimal_digits(step)).min(12);
    let walk = |rungs: f64, forward: bool| {
        let position = (current - origin) / step;
        let nearest = position.round();
        let base = if (position - nearest).abs() <= 1e-9 * nearest.abs().max(1.0) {
            nearest
        } else if forward {
            position.floor()
        } else {
            position.ceil()
        };
        clamp(crate::round_ui_number(origin + (if forward { base + rungs } else { base - rungs }) * step, digits))
    };
    let rungs = if large { SLIDER_PAGE_STEPS } else { 1.0 };
    match key {
        SliderKey::Increment => walk(rungs, true),
        SliderKey::Decrement => walk(rungs, false),
        SliderKey::PageUp => slider_adjacent_snap(current, snaps, true).unwrap_or_else(|| walk(SLIDER_PAGE_STEPS, true)),
        SliderKey::PageDown => slider_adjacent_snap(current, snaps, false).unwrap_or_else(|| walk(SLIDER_PAGE_STEPS, false)),
        SliderKey::Home => min.unwrap_or(current),
        SliderKey::End => max.unwrap_or(current),
    }
}

/// 🎨️ `rgba` — sRGB components in `0..=1` with straight alpha, a missing component 0 and a missing alpha 1, a
/// non-finite one 0 — as a lowercase `#rrggbb`, or `#rrggbbaa` with `alpha`; each channel is `round(c × 255)`.
pub fn ui_color_hex(rgba: &[f64], alpha: bool) -> String {
    let channel = |index: usize| {
        let component = rgba.get(index).copied().unwrap_or(if index == 3 { 1.0 } else { 0.0 });
        ((if component.is_finite() { component.clamp(0.0, 1.0) } else { 0.0 }) * 255.0).round() as u8
    };
    std::iter::once("#".to_string()).chain((0..if alpha { 4 } else { 3 }).map(|index| format!("{:02x}", channel(index)))).collect()
}

/// 🧪️ Reads a hex colour a user typed — an optional `#`, then 3, 4, 6 or 8 hex digits in any case, surrounding
/// space ignored, the short forms doubling each digit — as sRGB components in `0..=1` (`channel / 255`), alpha 1
/// when absent. `None` for anything else.
pub fn parse_ui_color_hex(text: &str) -> Option<[f64; 4]> {
    let trimmed = text.trim();
    let digits = trimmed.strip_prefix('#').unwrap_or(trimmed);
    let width = match digits.len() {
        3 | 4 => 1,
        6 | 8 => 2,
        _ => return None,
    };
    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }
    let mut rgba = [0.0, 0.0, 0.0, 1.0];
    for (index, slot) in rgba.iter_mut().enumerate().take(digits.len() / width) {
        let channel = u8::from_str_radix(&digits[index * width..(index + 1) * width], 16).ok()?;
        *slot = f64::from(if width == 1 { channel * 17 } else { channel }) / 255.0;
    }
    Some(rgba)
}

/// 🔟️ The fraction digits `value` prints with in the twelve-digit format (an `e-n` exponent adds `n`).
fn decimal_digits(value: f64) -> u16 {
    let text = crate::format_ui_number(value);
    let (mantissa, exponent) = text.split_once('e').unwrap_or((text.as_str(), "0"));
    let fraction = mantissa.split_once('.').map_or(0, |(_, fraction)| fraction.len() as i64);
    u16::try_from((fraction - exponent.parse::<i64>().unwrap_or(0)).max(0)).unwrap_or(u16::MAX)
}

/// 💍️ Props for `Component::Ring`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct RingProps {
    pub orb_id: crate::UiText,
    pub t: f64,
}

/// 🖼️ Props for `Component::IconSelect`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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
#[value(crate = "::protocol::value", rename_all = "camelCase")]
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

/// 🌲️ Props for `Component::Tree` — the tree's own binding, nothing else. Sections and items are no
/// longer inline (`sections: Vec<UiTreeSectionNode>`); they are ordinary child nodes
/// (`Component::TreeSection` / `Component::TreeItem`) reached through the record's `children`.
/// `drop_action` moved to the record's `bindings` (`Trigger::Drop`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TreeProps {
    #[serde(default, skip_serializing_if = "TreePresentation::is_standard")]
    #[value(default, skip_serializing_if = "TreePresentation::is_standard")]
    pub presentation: TreePresentation,
    /// 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain — selection/hover for
    /// bound items is owned by the framework's presence channel, not by per-item props.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<crate::UiText>,
}

/// 🌳️ A tree's inherited row and value-column presentation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum TreePresentation {
    #[default]
    Standard,
    Compact,
}

impl TreePresentation {
    pub fn is_standard(&self) -> bool {
        matches!(self, Self::Standard)
    }
}

/// 🪟️ Closed geometry token for one unmaterialised row in a virtual Tree window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum TreeWindowRowExtent {
    #[default]
    Standard,
    CompactText,
    CompactSmallControl,
    CompactControl,
}

/// 🪟️ The materialised slice of a logically `total`-long child list: the record's `children` are the
/// entries `[offset, offset + children.len())` of that list. `total > 0` with no materialised
/// children means expandable-but-not-yet-loaded, never "empty"; a renderer pitches the unmaterialised
/// rows as spacers so the scrollbar spans the whole document instead of the loaded window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TreeWindow {
    pub total: u32,
    pub offset: u32,
    pub row_extent: TreeWindowRowExtent,
}

/// 🧾️ Node records a presented panel body may spend OUTSIDE its windowed containers: the fixed rows a
/// panel still builds without a [`TreeWindow`] (an Actions section, a properties block, a nested
/// control child) carry no node key, so neither side can address or budget them per container. The
/// body budget holds this many records back for them.
///
/// 🧾️ The value is the fleet's MEASURED worst case, not a guess. The fattest shipped un-windowed block
/// is the energy inspector with a fenestration selected
/// (`✏️s/🔌️plugins/🔋️energy/…/📌️panels/🔍️inspection/🦀️.rs`): a `Window` section node plus the
/// **sixteen** field rows `fenestration_rows` pushes (id, name, surface, U-value, SHGC, VLT, area,
/// height, sill height, frame, divider, overhang depth/offset, fin depth/offset, glazing), plus an
/// `Actions` section node and its ≤3 rows = **21** records that carry no window and no node key, so
/// neither side can budget them per container. Next fattest: fem2d/fem3d's inspector with a solid
/// selected (1 + 10 + 1 + 3 = 15) and the framework's own history panel (1 + 5 + 1 = 7). 24 covers the
/// measured maximum with three records of margin, and `tree_window_headroom_covers_the_fattest_shipped_panel`
/// in the plugin SDK's `🔬️app-panel-kit` laws rebuilds that energy body shape and pins it.
///
/// ⚠️ A panel that builds MORE than this in fixed rows must move them onto a window: the ledger cannot
/// see them, so they are the one way a body can still outgrow [`crate::UI_DOCUMENT_NODES`].
pub const TREE_WINDOW_FIXED_NODE_HEADROOM: usize = 24;

/// 🧾️ The ONE body-wide node budget both sides of the streaming loop spend, in node records:
/// `UI_DOCUMENT_NODES` (the reconciler's per-surface record arena,
/// `SurfaceReconcileLimits::max_nodes`) less the tree root, less
/// [`TREE_WINDOW_FIXED_NODE_HEADROOM`].
///
/// 🧾️ **Cost model, identical on host and guest**: one windowed container costs
/// `1 (its own node) + its materialised rows`, nested containers included, and every node is charged
/// exactly ONCE — a nested container is one row of its parent, so its own `1` is the row its parent
/// already paid for. The host caps what it requests so `Σ(1 + rows) ≤ TREE_WINDOW_BODY_NODE_BUDGET`
/// (`🌳️Tree/🟦️.tsx`), and the guest's `semio_framework_plugin::TreeWindows` ledger spends the same
/// number, so a request the host is allowed to make is a request the guest can always honour in full.
pub const TREE_WINDOW_BODY_NODE_BUDGET: usize = crate::UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM;

/// 🔑️ Joins the segments of a windowed container's **window path** — the identity both sides of the
/// streaming loop address a container by (`ViewModel::tree_windows[].node_key`).
///
/// 🔑️ A container's path is the node keys of its enclosing windowed containers, outermost first, then
/// its own key. A top-level section's path is therefore just its own key, so every flat request stays
/// exactly what it was; a load case's loads nested under `fem3d-play-artifact.load-cases` are
/// `"fem3d-play-artifact.load-cases\u{1f}dead"`. This is what makes container identity collision-free
/// BY CONSTRUCTION while node keys and pick target ids stay untouched: a document whose load case and
/// whose combination are both called `uls` gives them different paths, because their parents differ.
/// Two containers can still collide only as true siblings under one parent — which the UI document
/// itself already refuses (`DuplicateSiblingKey`).
///
/// 🔑️ The separator is U+241F SYMBOL FOR UNIT SEPARATOR — the PRINTABLE glyph, not the C0 control
/// U+001F it depicts. A path crosses the process boundary inside `ViewModel::tree_windows`, and the
/// view-context admission both sides run
/// (`admitCrossingViewContext`/`parseResolvedPluginViewState`, `🛂️manifest/🟦️.ts`) refuses ANY
/// identifier carrying a C0 or DEL code point or exceeding 256 code points — a control separator made
/// every refresh, action and pick carrying a nested path throw `view context: invalid identifier` the
/// moment one nested container existed. A path is therefore a view-context identifier like any other:
/// printable, and at most 256 code points. A container whose own key already contains this glyph is
/// refused at assembly (`ui.tree-window.separator-in-key`) so a path can never be ambiguous.
pub const TREE_WINDOW_PATH_SEPARATOR: &str = "␟";

/// 🌲️ Props for `Component::TreeSection` — a labeled, collapsible grouping of `TreeItem` children.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TreeSectionProps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    /// 🎛️ The section's direct horizontal Toolbar child whose Button children render in the header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_toolbar: Option<crate::UiNodeId>,
    /// 🪟️ The materialised slice of this section's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
}

/// 🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and
/// the old inline `control: Option<UiControlNode>` are now ordinary children on the record (the
/// `UiControlNode` enum does not get ported — every one of its old variants is already a
/// [`Component`] variant in its own right, so a control-as-child-node needs no separate wrapper
/// type). [`TreeItemProps::inline_toolbar`] identifies the one direct horizontal Toolbar child whose
/// real Button children remain independently focusable and actionable inside the row, while
/// [`TreeItemProps::detail`] identifies one direct Surface child placed below the row. The row's
/// primary click action (old `action: Option<ActionDescriptor>`) is [`RowTarget::activation`], a verb on
/// the row's target — never a record `Trigger::Activate` binding.
// 🌱️ No `ToValue`/`FromValue` here: `target: Option<RowTarget>` embeds `UiValue` — see [`RowTarget`].
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeItemProps {
    pub label: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<crate::UiFixedMap<crate::UiText>>,
    /// 👁️ Domain "eye toggle": the row stays visible, dimmed, and clickable (to un-hide). NOT the
    /// same axis as the record's `activity`/`disabled` — a dimmed row is still fully interactive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimmed: Option<bool>,
    /// 🪟️ The materialised slice of this row's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    /// 🎯️ The interaction granularity this row picks when the tree carries the domain binding: the
    /// row is a pick target of the tree's `interaction_domain`, keyed by its own record key, so it
    /// needs no argument map and no per-row binding of its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<crate::UiText>,
    /// 🎛️ A direct `ContainerRole::Toolbar` child laid out horizontally inside this row. The
    /// document validator requires the target to be one of this record's direct children and every
    /// toolbar child to be a real [`Component::Button`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_toolbar: Option<crate::UiNodeId>,
    /// 🎞️ A direct Surface child placed in a bounded detail band below this row. It is neither an
    /// inline control nor a nested TreeItem, and remains a real independently hosted scene.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<crate::UiNodeId>,
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}

impl TreeItemProps {
    fn credited_clone(&self) -> Option<Self> {
        Some(Self {
            label: self.label.clone(),
            description: self.description.clone(),
            icon: self.icon.clone(),
            default_open: self.default_open,
            draggable: self.draggable,
            drag_data: self.drag_data.clone(),
            dimmed: self.dimmed,
            window: self.window,
            granularity: self.granularity.clone(),
            inline_toolbar: self.inline_toolbar,
            detail: self.detail,
            row_actions: self.row_actions.clone(),
            target: credited_target(&self.target)?,
        })
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn credited_target(target: &Option<RowTarget>) -> Option<Option<RowTarget>> {
    match target {
        Some(target) => Some(Some(target.credited_clone()?)),
        None => Some(None),
    }
}

/// 📊️ Props for `Component::Table` — a column-headed data table. The header lives HERE, as props, and
/// every [`TableRowProps`] child is one row, so a table costs one node record plus one per MATERIALISED
/// row however many columns and row actions it has. `window` is the same [`TreeWindow`] contract a tree
/// section carries: the children are the rows `[offset, offset + children.len())` of a logically
/// `total`-long row list, a renderer pitches the unmaterialised rows as spacers and asks for the rows its
/// viewport shows through `ViewModel::tree_windows` — one windowing mechanism and one body-wide node
/// ledger ([`TREE_WINDOW_BODY_NODE_BUDGET`]) for trees and tables alike.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct TableProps {
    /// 🏷️ The table's accessible name — what assistive technology announces on entering it.
    pub label: Label,
    /// 🗂️ The column headers, in cell order.
    pub columns: crate::UiFixedList<Label>,
    /// ↕️ Domain name for one logical row, localized by the artifact (for example Row or Frame).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub row_label: Option<Label>,
    /// ↔️ Domain name for one logical column, localized by the artifact (for example Column or Channel).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_label: Option<Label>,
    /// 🎬️ Header of the trailing actions column a renderer adds when any row carries row actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub actions_label: Option<Label>,
    /// 🪟️ The materialised slice of the logical row list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    /// ↔️ The materialised slice of the logical column list. `columns` and every row's `cells`
    /// contain exactly this slice and retain their logical indices through `offset`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_window: Option<TreeWindow>,
}

/// 📊️ Props for `Component::TableRow` — one row of a [`TableProps`] table: its cells in column order, its
/// row-scoped actions and its target, all as props, so a row is ONE node record. Actions and activation
/// are verbs on the row's ONE [`RowTarget`], exactly as a [`TreeItemProps`] row's are. No
/// `ToValue`/`FromValue`: [`RowTarget`] embeds `UiMap`, the same deliberate exception [`TreeItemProps`]
/// documents.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProps {
    /// 📝️ The row's cells, positional to the table's `columns`.
    pub cells: crate::UiFixedList<crate::UiText>,
    /// 🎬️ Row-scoped actions, rendered in the table's trailing actions column.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}

impl TableRowProps {
    fn credited_clone(&self) -> Option<Self> {
        Some(Self { cells: self.cells.clone(), row_actions: self.row_actions.clone(), target: credited_target(&self.target)? })
    }
}

/// 🖼️ Props for `Component::Image`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct ImageProps {
    pub src: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<Label>,
}

/// 🧩️ Props for `Component::Extension` — the old `ExternalSlot`. `params_json: String` becomes
/// structured `crate::UiValue`; `plugin_id`/`app_id`/`body_key` collapse into one opaque `extension`
/// address string (the old three-part addressing is a concern of whatever resolves `extension` to a
/// slot, not of this contract).
// 🌱️ No `ToValue`/`FromValue` here: `props: crate::UiValue` directly — the deliberate DslValue-free
// exception, see `UiValue`'s own docstring in `🎬️action.rs`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionProps {
    pub extension: crate::UiText,
    /// ⚠️ Decision: `crate::UiValue` is referenced, not defined, here. The packet brief's explicit
    /// "leave unresolved" list (`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`/`ActionBinding`/
    /// `MenuRef`/`Activity`/`SurfaceProps`) does not name `UiValue`, but `📋️master.md`'s "1. Contract
    /// crate" section places `UiValue` right beside `ActionId`/`Trigger`/`UiIntent` — the action
    /// model, owned by packet `contract-action`'s `🎬️action.rs`, not this file. Defining it here
    /// risks the exact duplicate-definition collision U2 calls out as worse than an unresolved name.
    pub props: crate::UiValue,
}
//#endregion 🎨️Props

//#region 🧩️Enum
/// 🧩️ The closed set of things a [`crate::UiNodeRecord`] can render.
///
/// ⚠️ Unlike the old `UiNode`, no `#[allow(clippy::large_enum_variant)]` is needed: the old
/// `ComponentScene` variant carried up to fifteen `Option<XxxScene>` fields inline, which is exactly
/// the size disparity that lint was suppressing. `Component::Surface` now carries one
/// `crate::SurfaceProps` (a single pack-encoded payload keyed by a `doc_schema` id), so the variants
/// are all comparably small.
// 🌱️ No `ToValue`/`FromValue` on the dispatch enum itself: the `TreeItem`/`Extension` variants carry
// `TreeItemProps`/`ExtensionProps`, both deliberately DslValue-free-exception types — see their own
// notes above.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
#[expect(clippy::large_enum_variant, reason = "Typed copy and retirement account for inline component bytes; boxing variants would require separate allocation credits.")]
pub enum Component {
    Container(ContainerProps),
    Text(TextProps),
    Button(ButtonProps),
    Separator(SeparatorProps),
    Input(InputProps),
    Select(SelectProps),
    Toggle(ToggleProps),
    KeyValueList(KeyValueListProps),
    Slider(SliderProps),
    NumberStepper(NumberStepperProps),
    Ring(RingProps),
    IconSelect(IconSelectProps),
    Progress(ProgressProps),
    Tree(TreeProps),
    TreeSection(TreeSectionProps),
    TreeItem(TreeItemProps),
    Image(ImageProps),
    Surface(crate::SurfaceProps),
    Extension(ExtensionProps),
    Table(TableProps),
    TableRow(TableRowProps),
}

impl Component {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(match self {
            Self::Container(value) => Self::Container(value.clone()),
            Self::Text(value) => Self::Text(value.clone()),
            Self::Button(value) => Self::Button(value.clone()),
            Self::Separator(value) => Self::Separator(value.clone()),
            Self::Input(value) => Self::Input(value.clone()),
            Self::Select(value) => Self::Select(value.clone()),
            Self::Toggle(value) => Self::Toggle(value.clone()),
            Self::KeyValueList(value) => Self::KeyValueList(value.clone()),
            Self::Slider(value) => Self::Slider(value.clone()),
            Self::NumberStepper(value) => Self::NumberStepper(value.clone()),
            Self::Ring(value) => Self::Ring(value.clone()),
            Self::IconSelect(value) => Self::IconSelect(value.clone()),
            Self::Progress(value) => Self::Progress(value.clone()),
            Self::Tree(value) => Self::Tree(value.clone()),
            Self::TreeSection(value) => Self::TreeSection(value.clone()),
            Self::TreeItem(value) => Self::TreeItem(value.credited_clone()?),
            Self::Image(value) => Self::Image(value.clone()),
            Self::Surface(value) => Self::Surface(value.credited_clone()?),
            Self::Extension(value) => Self::Extension(ExtensionProps { extension: value.extension.clone(), props: value.props.credited_clone()? }),
            Self::Table(value) => Self::Table(value.clone()),
            Self::TableRow(value) => Self::TableRow(value.credited_clone()?),
        })
    }
}
//#endregion 🧩️Enum

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️component-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#endregion 🔖️Component
