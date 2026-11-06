//! 🗃️ Table, image, extension and component: the table and table row props, the image and extension props and the `Component` enum with its accessors. See [`super`].

use super::*;

/// 📊️ Props for `Component::Table` — a column-headed data table. The header lives HERE, as props, and [Design notes](README.md#source-notes-5).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(crate = "::semio_framework_value", rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
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
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub struct TableRowProps {
    /// 📝️ The row's cells, positional to the table's `columns`.
    pub cells: crate::UiFixedList<crate::UiText>,
    /// 🔘️ Row-scoped actions, rendered in the table's trailing actions column.
    #[serde(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    #[value(default, skip_serializing_if = "crate::UiFixedList::is_empty")]
    pub row_actions: crate::UiFixedList<RowAction>,
    /// 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
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
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub struct ImageProps {
    pub src: crate::UiText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<Label>,
}

/// 🧩️ Props for `Component::Extension` — the old `ExternalSlot`. `params_json: String` becomes [Design notes](README.md#source-notes-85).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
pub struct ExtensionProps {
    pub extension: crate::UiText,
    /// ⚠️ Decision: `crate::UiValue` is referenced, not defined, here. The packet brief's explicit [Design notes](README.md#source-notes-97).
    pub props: crate::UiValue,
}
//#endregion 🎨️Props

/// 🧱️ The closed set of things a [`crate::UiNodeRecord`] can render. [Design notes](README.md#source-notes-107).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
#[value(tag = "type", rename_all = "camelCase")]
#[expect(clippy::large_enum_variant, reason = "Typed copy and retirement account for inline component bytes; boxing variants would require separate allocation credits.")]
#[derive(semio_framework_value::NativeReceive,semio_framework_value::RetireOwned)]
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
