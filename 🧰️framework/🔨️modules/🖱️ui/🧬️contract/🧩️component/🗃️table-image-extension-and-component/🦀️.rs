//! 🗃️ Table, image, extension and component: the table and table row props, the image and extension props and the `Component` enum with its accessors. See [`super`].

use super::*;

//#region 🎨️Props
/// 📊️ Props for `Component::Table` — a column-headed data table. The header lives HERE, as props, and
/// every [`TableRowProps`] child is one row, so a table costs one node record plus one per MATERIALISED
/// row however many columns and row actions it has. `window` is the same [`TreeWindow`] contract a tree
/// section carries: the children are the rows `[offset, offset + children.len())` of a logically
/// `total`-long row list, a renderer pitches the unmaterialised rows as spacers and asks for the rows its
/// viewport shows through `ViewModel::tree_windows` — one windowing mechanism and one body-wide node
/// ledger ([`TREE_WINDOW_BODY_NODE_BUDGET`]) for trees and tables alike.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
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
    /// 🍕️ The materialised slice of the logical column list. `columns` and every row's `cells`
    /// contain exactly this slice and retain their logical indices through `offset`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_window: Option<TreeWindow>,
}

/// 🏓️ Props for `Component::TableRow` — one row of a [`TableProps`] table: its cells in column order, its
/// row-scoped actions and its target, all as props, so a row is ONE node record. Actions and activation
/// are verbs on the row's ONE [`RowTarget`], exactly as a [`TreeItemProps`] row's are. No
/// `ToValue`/`FromValue`: [`RowTarget`] embeds `UiMap`, the same deliberate exception [`TreeItemProps`]
/// documents.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowProps {
    /// 📝️ The row's cells, positional to the table's `columns`.
    pub cells: crate::UiFixedList<crate::UiText>,
    /// 🔘️ Row-scoped actions, rendered in the table's trailing actions column.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}

impl TableRowProps {
    pub(super) fn credited_clone(&self) -> Option<Self> {
        Some(Self { cells: self.cells.clone(), row_actions: self.row_actions.clone(), target: credited_target(&self.target)? })
    }
}

/// 🖼️ Props for `Component::Image`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
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
/// 🧱️ The closed set of things a [`crate::UiNodeRecord`] can render.
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
