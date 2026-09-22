//! @emoji 🧩️ The semantic `Component` enum and its per-component prop structs — the closed set of
//! things a [`crate::UiNodeRecord`] can render. Every prop struct carries only the data specific to
//! that component: identity lives on the record (`key`), actions live on the record (`bindings`),
//! visual state lives on the record (`activity`/`disabled`/`transition`) or its `layout`/`style`. A
//! prop struct that inlined any of those would be reintroducing the implicit coupling this contract
//! replaces.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1.

// 🌱️ `ToValue`/`FromValue` here is the first-party analog of `Serialize`/`Deserialize` below, for
// ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS. `RowAction`,
// `TreeItemProps`, `ExtensionProps` and the `Component` enum itself are the deliberate exception —
// each embeds `crate::UiValue`/`crate::ActionBinding` (directly or via `RowAction`), which stay
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

/// 🎬️ One action affordance painted on (or reachable from) a [`Component::TreeItem`] row —
/// `action` reuses [`crate::ActionBinding`] rather than a second parallel action-id type, since a row
/// action is exactly a binding fired unconditionally on click (no `Trigger` ambiguity to add here).
// 🌱️ No `ToValue`/`FromValue` here: `action: crate::ActionBinding` embeds `UiValue`, the deliberate
// DslValue-free exception — see `UiValue`'s docstring in `🎬️action.rs`.
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

/// 🎚️ Props for `Component::Slider`. `on_change` moved to the record's `bindings`.
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
}

/// 🔢️ Props for `Component::NumberStepper`. `on_absolute`/`on_delta` both moved to the record's
/// `bindings`, distinguished by `Trigger`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub struct NumberStepperProps {
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
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
    /// 🪟️ The materialised slice of this section's logical child list — see [`TreeWindow`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
}

/// 🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and
/// the old inline `control: Option<UiControlNode>` are now ordinary children on the record (the
/// `UiControlNode` enum does not get ported — every one of its old variants is already a
/// [`Component`] variant in its own right, so a control-as-child-node needs no separate wrapper
/// type). The row's primary click action (old `action: Option<ActionDescriptor>`) moved to the
/// record's `bindings` (`Trigger::Activate`).
// 🌱️ No `ToValue`/`FromValue` here: `row_actions: UiFixedList<RowAction>` needs `RowAction: ToValue`,
// which RowAction deliberately does not implement (embeds `UiValue`) — see its own note above.
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
            window: self.window,
            granularity: self.granularity.clone(),
            row_actions,
        })
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
