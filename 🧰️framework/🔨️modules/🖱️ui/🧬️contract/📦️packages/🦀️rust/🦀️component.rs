//! @emoji 🧩️ The semantic `Component` enum and its per-component prop structs — the closed set of
//! things a [`crate::UiNodeRecord`] can render. Every prop struct carries only the data specific to
//! that component: identity lives on the record (`key`), actions live on the record (`bindings`),
//! visual state lives on the record (`activity`/`disabled`/`transition`) or its `layout`/`style`. A
//! prop struct that inlined any of those would be reintroducing the implicit coupling this contract
//! replaces.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.

use serde::{Deserialize, Serialize};

//#region 🔖️Component

//#region 🏷️Label
/// 🏷️ Display-ready UI text carried on the wire.
///
/// ⚠️ Decision (flagged per packet brief): the old `UiNode`'s `Label` (`crate::wgpu::Label` in
/// `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs`) is NOT reused here. It
/// is defined inside the old wgpu-target UI package, imports that package's own `Locale`/
/// `Terminology` axes, and its whole point (`From<LabelText>` wired to the `app_labels!` macro,
/// no `From<&str>`) is a compile-time-checked-label enforcement mechanism that lives at the
/// authoring boundary, not the wire boundary — and this crate must not depend on that package at
/// all (no engine, no wgpu; see `📦️glue.rs`). This is therefore a minimal, independent transparent
/// string. The localization/terminology resolution that used to happen via `LabelText::fill` still
/// happens upstream of the runtime (manifest/host), before a `Label` ever reaches this contract.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
/// literals (`"text"`, `"textarea"`, `"number"`, `"file"`) plus the values the React `Interpreter`
/// already branches on for `inputKind` (`"number"`, `"longText"`, `"date"`, `"color"`, `"file"`,
/// default `"text"`) — the union of both sides, since the renderer supports more kinds than any
/// current plugin emits yet. Note the old Rust/TS spelling mismatch this closes: Rust emitted
/// `"textarea"`, TS checked `"longText"`; this enum has exactly one spelling (`LongText`) for that
/// kind. No `Search` kind exists anywhere in the fleet today, so it is not included (see the
/// packet's own report for the grep evidence — adding it back is a one-variant change, not a design
/// change, if a plugin needs it later).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RowActionPlacement {
    #[default]
    Row,
    Menu,
}
//#endregion 🎛️Enums

//#region 🧱️Nested
/// 📥️ Hover-state copy for a [`ContainerProps::drop_overlay`] — shown while a drag is over the
/// container, ahead of its `Drop`-triggered [`crate::ActionBinding`] firing on release.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropOverlaySpec {
    pub title: Label,
    pub hint: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<crate::UiText>,
}

/// 🔽️ One option of a [`Component::Select`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectItem {
    pub value: crate::UiText,
    pub label: Label,
}

/// 🗝️ One row of a [`Component::KeyValueList`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValueEntry {
    pub label: Label,
    pub value: crate::UiText,
}

/// 🎬️ One action affordance painted on (or reachable from) a [`Component::TreeItem`] row —
/// `action` reuses [`crate::ActionBinding`] rather than a second parallel action-id type, since a row
/// action is exactly a binding fired unconditionally on click (no `Trigger` ambiguity to add here).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RowAction {
    /// 🖼️ Icon key. See [`ButtonProps::icon`] for why this is a plain `String`, not a closed enum.
    pub icon: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    pub action: crate::ActionBinding,
    #[serde(default, skip_serializing_if = "is_default_row_action_placement")]
    pub placement: RowActionPlacement,
}

impl RowAction {
    pub fn credited_clone(&self) -> Option<Self> {
        Some(Self { icon: self.icon.clone(), label: self.label.clone(), action: self.action.credited_clone()?, placement: self.placement })
    }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_row_action_placement(value: &RowActionPlacement) -> bool {
    *value == RowActionPlacement::default()
}
//#endregion 🧱️Nested

//#region 🎨️Props
/// 🌿️ Props for `Component::Container` — the box-with-children component every old `Stack`/
/// `Section`/`Group`/`Field` collapses into (see [`ContainerRole`]). `direction`/`gap`/`padding` do
/// NOT live here — they are `crate::LayoutSpec`, on the record. The old `Field`'s single
/// `child: Box<UiNode>` is simply `children[0]` on the record; there is nothing left to special-case.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerProps {
    #[serde(default, skip_serializing_if = "is_default_container_role")]
    pub role: ContainerRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop_overlay: Option<DropOverlaySpec>,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_container_role(value: &ContainerRole) -> bool {
    *value == ContainerRole::default()
}

/// 📝️ Props for `Component::Text`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextProps {
    pub value: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emphasize: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<crate::UiFixedMap<crate::UiText>>,
}

/// 🔘️ Props for `Component::Button`. `action` moved to the record's `bindings` (keyed by
/// `Trigger::Activate`); `style` moved to the record's `crate::StyleSpec`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonProps {
    /// 🖼️ Icon key. The old `IconName` is generated per-consuming-crate via a `#[path]` mount (see
    /// `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs`), not a publishable
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SeparatorProps {}

/// ⌨️ Props for `Component::Input`. `on_change` moved to the record's `bindings`
/// (`Trigger::Change`/`Trigger::Commit`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputProps {
    #[serde(default, skip_serializing_if = "is_default_input_kind")]
    pub kind: InputKind,
    pub value: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    /// 🫳️ Commit convention string carried verbatim from the old wire shape (e.g. `"blur"`) — no
    /// closed set of these was found in the fleet, unlike `input_kind`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<crate::UiText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<crate::UiText>,
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn is_default_input_kind(value: &InputKind) -> bool {
    *value == InputKind::default()
}

/// 🔽️ Props for `Component::Select`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectProps {
    pub value: crate::UiText,
    pub items: crate::UiFixedList<SelectItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
}

/// 🔀️ Props for `Component::Toggle`. `on` is the explicit state this contract adds — the old
/// `UiToggleNode` smuggled it through `presence.selected`, exactly the implicit coupling this
/// contract exists to remove. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToggleProps {
    pub on: bool,
    pub icon: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Label>,
}

/// 🗝️ Props for `Component::KeyValueList`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyValueListProps {
    pub entries: crate::UiFixedList<KeyValueEntry>,
}

/// 🎚️ Props for `Component::Slider`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SliderProps {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<crate::UiText>,
}

/// 🔢️ Props for `Component::NumberStepper`. `on_absolute`/`on_delta` both moved to the record's
/// `bindings`, distinguished by `Trigger`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberStepperProps {
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
}

/// 💍️ Props for `Component::Ring`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RingProps {
    pub orb_id: crate::UiText,
    pub t: f64,
}

/// 🖼️ Props for `Component::IconSelect`. `on_change` moved to the record's `bindings`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconSelectProps {
    pub value: crate::UiText,
    pub uniform: bool,
    pub classifier_kind: crate::UiText,
}

/// 🌲️ Props for `Component::Tree` — the tree's own binding, nothing else. Sections and items are no
/// longer inline (`sections: Vec<UiTreeSectionNode>`); they are ordinary child nodes
/// (`Component::TreeSection` / `Component::TreeItem`) reached through the record's `children`.
/// `drop_action` moved to the record's `bindings` (`Trigger::Drop`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeProps {
    /// 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain — selection/hover for
    /// bound items is owned by the framework's presence channel, not by per-item props.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<crate::UiText>,
}

/// 🌲️ Props for `Component::TreeSection` — a labeled, collapsible grouping of `TreeItem` children.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeSectionProps {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
}

/// 🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and
/// the old inline `control: Option<UiControlNode>` are now ordinary children on the record (the
/// `UiControlNode` enum does not get ported — every one of its old variants is already a
/// [`Component`] variant in its own right, so a control-as-child-node needs no separate wrapper
/// type). The row's primary click action (old `action: Option<ActionDescriptor>`) moved to the
/// record's `bindings` (`Trigger::Activate`).
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
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
}

impl TreeItemProps {
    fn credited_clone(&self) -> Option<Self> {
        let mut row_actions = crate::UiFixedList::default();
        for action in self.row_actions.iter() {
            row_actions.try_push(action.credited_clone()?).ok()?;
        }
        Some(Self {
            label: self.label.clone(),
            description: self.description.clone(),
            icon: self.icon.clone(),
            default_open: self.default_open,
            draggable: self.draggable,
            drag_data: self.drag_data.clone(),
            dimmed: self.dimmed,
            row_actions,
        })
    }
}

/// 🖼️ Props for `Component::Image`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageProps {
    pub src: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<Label>,
}

/// 🧩️ Props for `Component::Extension` — the old `ExternalSlot`. `params_json: String` becomes
/// structured `crate::UiValue`; `plugin_id`/`app_id`/`body_key` collapse into one opaque `extension`
/// address string (the old three-part addressing is a concern of whatever resolves `extension` to a
/// slot, not of this contract).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionProps {
    pub extension: crate::UiText,
    /// ⚠️ Decision: `crate::UiValue` is referenced, not defined, here. The packet brief's explicit
    /// "leave unresolved" list (`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`/`ActionBinding`/
    /// `MenuRef`/`Activity`/`SurfaceProps`) does not name `UiValue`, but `📋️master.md`'s "1. Contract
    /// crate" section places `UiValue` right beside `ActionId`/`Trigger`/`UiIntent` — the action
    /// model, owned by packet `contract-action`'s `🦀️action.rs`, not this file. Defining it here
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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
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
    Tree(TreeProps),
    TreeSection(TreeSectionProps),
    TreeItem(TreeItemProps),
    Image(ImageProps),
    Surface(crate::SurfaceProps),
    Extension(ExtensionProps),
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
            Self::Tree(value) => Self::Tree(value.clone()),
            Self::TreeSection(value) => Self::TreeSection(value.clone()),
            Self::TreeItem(value) => Self::TreeItem(value.credited_clone()?),
            Self::Image(value) => Self::Image(value.clone()),
            Self::Surface(value) => Self::Surface(value.credited_clone()?),
            Self::Extension(value) => Self::Extension(ExtensionProps { extension: value.extension.clone(), props: value.props.credited_clone()? }),
        })
    }
}
//#endregion 🧩️Enum

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn ui_text(value: &str) -> crate::UiText {
        crate::UiText::try_from_str(value).expect("bounded fixture text")
    }

    fn label(value: &str) -> Label {
        Label::try_from(value).expect("bounded fixture label")
    }

    #[allow(clippy::needless_pass_by_value)]
    fn component_round_trips(component: Component) {
        let first = serde_json::to_string(&component).expect("serialize");
        let deserialized: Component = serde_json::from_str(&first).expect("deserialize");
        let second = serde_json::to_string(&deserialized).expect("re-serialize");
        assert_eq!(first, second);
        assert_eq!(component, deserialized);
    }

    #[test]
    fn every_component_variant_round_trips() {
        component_round_trips(Component::Container(ContainerProps {
            role: ContainerRole::Section,
            label: Some(label("Section")),
            description: Some(ui_text("desc")),
            required: Some(true),
            error: None,
            default_open: Some(false),
            drop_overlay: Some(DropOverlaySpec { title: label("Drop"), hint: label("here"), accept: Some(ui_text("image/*")) }),
        }));
        component_round_trips(Component::Text(TextProps { value: label("hi"), emphasize: Some(true), data_attributes: None }));
        component_round_trips(Component::Button(ButtonProps { icon: ui_text("plus"), label: label("Add") }));
        component_round_trips(Component::Separator(SeparatorProps {}));
        component_round_trips(Component::Input(InputProps { kind: InputKind::Number, value: ui_text("3"), placeholder: None, commit: Some(ui_text("blur")), min: Some(0.0), max: Some(10.0), step: Some(1.0), accept: None }));
        component_round_trips(Component::Select(SelectProps { value: ui_text("a"), items: crate::UiFixedList::default(), placeholder: None }));
        component_round_trips(Component::Toggle(ToggleProps { on: true, icon: ui_text("toggle-left"), text: Some(label("Enabled")) }));
        component_round_trips(Component::KeyValueList(KeyValueListProps { entries: crate::UiFixedList::default() }));
        component_round_trips(Component::Slider(SliderProps { value: 0.5, min: 0.0, max: 1.0, step: 0.1, unit: Some(ui_text("m")) }));
        component_round_trips(Component::NumberStepper(NumberStepperProps { value: 2.0, step: 1.0, uniform: false }));
        component_round_trips(Component::Ring(RingProps { orb_id: ui_text("orb-1"), t: 0.25 }));
        component_round_trips(Component::IconSelect(IconSelectProps { value: ui_text("circle"), uniform: true, classifier_kind: ui_text("shape") }));
        component_round_trips(Component::Tree(TreeProps { interaction_domain: Some(ui_text("selection")) }));
        component_round_trips(Component::TreeSection(TreeSectionProps { label: Some(label("Section")), default_open: Some(true) }));
        component_round_trips(Component::TreeItem(TreeItemProps {
            label: label("Item"),
            description: None,
            icon: Some(ui_text("file")),
            default_open: None,
            draggable: Some(true),
            drag_data: None,
            dimmed: Some(false),
            row_actions: crate::UiFixedList::default(),
        }));
        component_round_trips(Component::Image(ImageProps { src: ui_text("atlas://x"), alt: Some(label("alt")) }));
        component_round_trips(Component::Surface(Default::default()));
        component_round_trips(Component::Extension(ExtensionProps { extension: ui_text("plugin.app.slot"), props: Default::default() }));
    }

    #[test]
    fn label_conversions_and_display() {
        let from_str = Label::try_from("hello").expect("bounded fixture label");
        let from_string = Label::try_from(String::from("hello")).expect("bounded fixture label");
        assert_eq!(from_str, from_string);
        assert_eq!(from_str.to_string(), "hello");
    }
}
//#endregion 🧪️Tests

//#endregion 🔖️Component
