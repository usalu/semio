//! 🧾️ Exact native shapes retain their canonical serializer field declarations.
use super::*;
use semio_ui_contract::*;
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
#[value(deny_unknown_fields)]
pub struct PresenceUpdate {
    pub surface: semio_ui_contract::SurfaceId,
    pub node_key: String,
    pub own: OwnPresence,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub peers: Vec<PeerMark>,
    pub ttl_ms: u32,
}
}


semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct UiPatch {
    pub surface: SurfaceId,
    pub base_revision: UiRevision,
    pub revision: UiRevision,
    pub ops: UiPatchOps,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct SurfaceId(pub UiText);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
#[value(deny_unknown_fields)]
pub struct OwnPresence {
    #[value(default, skip_serializing_if = "is_false")]
    pub hovered: bool,
    #[value(default, skip_serializing_if = "is_false")]
    pub selected: bool,
    #[value(default, skip_serializing_if = "is_false")]
    pub previewed: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
#[value(deny_unknown_fields)]
pub struct PeerMark {
    pub actor: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u8>,
    #[value(default, skip_serializing_if = "is_false")]
    pub hovered: bool,
    #[value(default, skip_serializing_if = "is_false")]
    pub selected: bool,
    pub label: String,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct UiRevision(pub u64);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(tag = "type", rename_all = "camelCase")]
pub enum UiPatchOp {
    Upsert(UiNodeRecord),
    SetComponent {
        id: UiNodeId,
        component: semio_ui_contract::Component,
    },
    SetLayout {
        id: UiNodeId,
        layout: semio_ui_contract::LayoutSpec,
    },
    SetActivity {
        id: UiNodeId,
        activity: semio_ui_contract::Activity,
        disabled: bool,
    },
    SetChildren {
        id: UiNodeId,
        children: UiNodeChildren,
    },
    SetStyle {
        id: UiNodeId,
        style: semio_ui_contract::StyleSpec,
    },
    SetAccessibility {
        id: UiNodeId,
        accessibility: semio_ui_contract::AccessibilitySpec,
    },
    SetBindings {
        id: UiNodeId,
        bindings: UiNodeBindings,
    },
    SetMenu {
        id: UiNodeId,
        menu: Option<semio_ui_contract::MenuRef>,
    },
    Remove {
        id: UiNodeId,
    },
    SetRoot {
        id: UiNodeId,
    },
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct UiNodeRecord {
    pub id: UiNodeId,
    pub key: semio_ui_contract::UiText,
    pub component: semio_ui_contract::Component,
    pub layout: semio_ui_contract::LayoutSpec,
    pub style: semio_ui_contract::StyleSpec,
    pub activity: semio_ui_contract::Activity,
    #[value(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<TransitionHint>,
    pub accessibility: semio_ui_contract::AccessibilitySpec,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub bindings: UiNodeBindings,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<semio_ui_contract::MenuRef>,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub children: UiNodeChildren,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct UiNodeId(pub u64);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(tag = "type", rename_all = "camelCase")]
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
    Surface(semio_ui_contract::SurfaceProps),
    Extension(ExtensionProps),
    Table(TableProps),
    TableRow(TableRowProps),
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum LayoutSpec {
    Leaf(LeafLayout),
    Stack(StackLayout),
    Grid(GridLayout),
    Overlay(OverlayLayout),
    Scroll(ScrollLayout),
    Absolute(AbsoluteLayout),
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Activity {
    Waiting,
    Loading,
    Idle,
    Finished,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct StyleSpec {
    #[value(default, skip_serializing_if = "is_default")]
    pub variant: Variant,
    #[value(default, skip_serializing_if = "is_default")]
    pub size: SizeToken,
    #[value(default, skip_serializing_if = "is_default")]
    pub density: Density,
    #[value(default, skip_serializing_if = "is_default")]
    pub tone: Tone,
    #[value(default, skip_serializing_if = "is_default")]
    pub emphasis: Emphasis,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct AccessibilitySpec {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<semio_ui_contract::Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<semio_ui_contract::Label>,
    #[value(default, skip_serializing_if = "is_default")]
    pub live: Liveness,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "is_default")]
    pub hidden: bool,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct MenuRef {
    pub id: UiText,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum TransitionHint {
    Introducing,
    Celebrating,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ContainerProps {
    #[value(default, skip_serializing_if = "is_default")]
    pub role: ContainerRole,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drop_overlay: Option<DropOverlaySpec>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TextProps {
    pub value: Label,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub emphasize: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<semio_ui_contract::UiFixedMap<semio_ui_contract::UiText>>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ButtonProps {
    pub icon: semio_ui_contract::UiText,
    pub label: Label,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
pub struct SeparatorProps {}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct InputProps {
    #[value(default, skip_serializing_if = "is_default")]
    pub kind: InputKind,
    pub value: semio_ui_contract::UiText,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub snaps: semio_ui_contract::UiFixedList<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub draft_target: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub publication_revision: Option<UiPublicationRevision>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct SelectProps {
    pub value: semio_ui_contract::UiText,
    pub items: semio_ui_contract::UiFixedList<SelectItem>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<Label>,
    #[value(default, skip_serializing_if = "is_default")]
    pub appearance: SelectAppearance,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ToggleProps {
    #[value(default, skip_serializing_if = "is_default")]
    pub appearance: ToggleAppearance,
    pub on: bool,
    pub icon: semio_ui_contract::UiText,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Label>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct KeyValueListProps {
    pub entries: semio_ui_contract::UiFixedList<KeyValueEntry>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct SliderProps {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub snaps: semio_ui_contract::UiFixedList<f64>,
    #[value(default, skip_serializing_if = "is_default")]
    pub appearance: SliderAppearance,
    #[value(default, skip_serializing_if = "is_default")]
    pub scale: UiNumberScale,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_unit: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct NumberStepperProps {
    pub value: f64,
    pub step: f64,
    pub uniform: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<u16>,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub snaps: semio_ui_contract::UiFixedList<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_unit: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub display_factor: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<UiNumberLimits>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct RingProps {
    pub orb_id: semio_ui_contract::UiText,
    pub t: f64,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct IconSelectProps {
    pub value: semio_ui_contract::UiText,
    pub uniform: bool,
    pub classifier_kind: semio_ui_contract::UiText,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ProgressProps {
    pub completed: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    pub value_text: Label,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TreeProps {
    #[value(default, skip_serializing_if = "is_default")]
    pub presentation: TreePresentation,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<semio_ui_contract::UiText>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TreeSectionProps {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_toolbar: Option<semio_ui_contract::UiNodeId>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TreeItemProps {
    pub label: Label,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub default_open: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub drag_data: Option<semio_ui_contract::UiFixedMap<semio_ui_contract::UiText>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub dimmed: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub inline_toolbar: Option<semio_ui_contract::UiNodeId>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<semio_ui_contract::UiNodeId>,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub row_actions: semio_ui_contract::UiFixedList<RowAction>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ImageProps {
    pub src: semio_ui_contract::UiText,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<Label>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct SurfaceProps {
    pub kind: SurfaceKind,
    pub doc_schema: semio_ui_contract::UiText,
    pub doc: SurfaceDoc,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub bindings: semio_ui_contract::UiNodeBindings,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ExtensionProps {
    pub extension: semio_ui_contract::UiText,
    pub props: semio_ui_contract::UiValue,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TableProps {
    pub label: Label,
    pub columns: semio_ui_contract::UiFixedList<Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub row_label: Option<Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_label: Option<Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub actions_label: Option<Label>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<TreeWindow>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_window: Option<TreeWindow>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TableRowProps {
    pub cells: semio_ui_contract::UiFixedList<semio_ui_contract::UiText>,
    #[value(default, skip_serializing_if = "semio_ui_contract::UiFixedList::is_empty")]
    pub row_actions: semio_ui_contract::UiFixedList<RowAction>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RowTarget>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct LeafLayout {
    pub width: Sizing,
    pub height: Sizing,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct StackLayout {
    pub axis: Axis,
    pub gap: SpaceToken,
    pub padding: EdgeSpace,
    pub align: Align,
    pub justify: Justify,
    pub grow: bool,
    pub wrap: bool,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct GridLayout {
    pub columns: UiGridTracks,
    pub rows: UiGridTracks,
    pub column_gap: SpaceToken,
    pub row_gap: SpaceToken,
    pub padding: EdgeSpace,
    pub align: Align,
    pub justify: Justify,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct OverlayLayout {
    pub anchor: Anchor,
    pub inset: EdgeSpace,
    pub dismissible: bool,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ScrollLayout {
    pub axes: ScrollAxes,
    pub padding: EdgeSpace,
    pub sizing: Sizing,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct AbsoluteLayout {
    pub sizing_width: Sizing,
    pub sizing_height: Sizing,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Variant {
    Solid,
    Outline,
    Ghost,
    Plain,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum SizeToken {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Density {
    Compact,
    Standard,
    Touch,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Tone {
    Neutral,
    Primary,
    Secondary,
    Tertiary,
    Info,
    Success,
    Warning,
    Danger,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Emphasis {
    Subtle,
    Regular,
    Strong,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(transparent)]
pub struct Label(pub semio_ui_contract::UiText);
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Liveness {
    Off,
    Polite,
    Assertive,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ActionBinding {
    pub trigger: Trigger,
    pub action: ActionId,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<UiValue>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub capability: Option<UiText>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum ContainerRole {
    Plain,
    Section,
    Group,
    Field,
    Form,
    Toolbar,
    Sheet,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct DropOverlaySpec {
    pub title: Label,
    pub hint: Label,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<semio_ui_contract::UiText>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum InputKind {
    Text,
    LongText,
    Number,
    Date,
    Color,
    File,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct UiNumberLimits {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<UiNumberBound>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<UiNumberBound>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct SelectItem {
    pub value: semio_ui_contract::UiText,
    pub label: Label,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum SelectAppearance {
    Menu,
    Segmented,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum ToggleAppearance {
    Button,
    Checkbox,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct KeyValueEntry {
    pub label: Label,
    pub value: semio_ui_contract::UiText,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum SliderAppearance {
    Track,
    Dial,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum UiNumberScale {
    Linear,
    Log,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum TreePresentation {
    Standard,
    Compact,
    Touch,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct TreeWindow {
    pub total: u32,
    pub offset: u32,
    pub row_extent: TreeWindowRowExtent,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct RowAction {
    pub icon: semio_ui_contract::UiText,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    pub verb: semio_ui_contract::UiText,
    #[value(default, skip_serializing_if = "is_default")]
    pub placement: RowActionPlacement,
    #[value(default, skip_serializing_if = "is_false")]
    pub disabled: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Label>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct RowTarget {
    pub scope: semio_ui_contract::UiText,
    pub version: u16,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<semio_ui_contract::UiMap>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<semio_ui_contract::UiText>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
pub enum SurfaceKind {
    #[value(rename = "canvas-2d")]
    Canvas2d,
    #[value(rename = "world-3d")]
    World3d,
    #[value(rename = "node-graph")]
    NodeGraph,
    #[value(rename = "text-editor")]
    TextEditor,
    #[value(rename = "table")]
    Table,
    #[value(rename = "paint-2d")]
    Paint2d,
    #[value(rename = "virtual-file-system")]
    VirtualFileSystem,
    #[value(rename = "tiled-map")]
    TiledMap,
    #[value(rename = "board-2d")]
    Board2d,
    #[value(rename = "icon-render")]
    IconRender,
    #[value(rename = "ink-canvas")]
    InkCanvas,
    #[value(rename = "graph-timeline")]
    GraphTimeline,
    #[value(rename = "block-list")]
    BlockList,
    #[value(rename = "diff-view")]
    DiffView,
    #[value(rename = "event-feed")]
    EventFeed,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
pub struct SurfaceDoc {
    pub bytes: semio_ui_contract::UiFixedBytes,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Sizing {
    Hug,
    Fill,
    Fixed(SpaceToken),
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Axis {
    Horizontal,
    Vertical,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum SpaceToken {
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum EdgeSpace {
    All(SpaceToken),
    Symmetric { vertical: SpaceToken, horizontal: SpaceToken },
    Each { top: SpaceToken, right: SpaceToken, bottom: SpaceToken, left: SpaceToken },
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Anchor {
    TopStart,
    Top,
    TopEnd,
    Start,
    Center,
    End,
    BottomStart,
    Bottom,
    BottomEnd,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum ScrollAxes {
    None,
    Horizontal,
    Vertical,
    Both,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum Trigger {
    Activate,
    Change,
    Commit,
    Delta,
    Drop,
    Submit,
    Abort,
    RepeatLast,
    HoverPreview,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct ActionId {
    pub scope: UiText,
    pub name: UiText,
    pub version: u16,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub struct UiNumberBound {
    pub value: f64,
    #[value(default, skip_serializing_if = "is_false")]
    pub exclusive: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<Label>,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum TreeWindowRowExtent {
    Standard,
    CompactText,
    CompactSmallControl,
    CompactControl,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum RowActionPlacement {
    Row,
    Menu,
}
}
semio_framework_value::native_json_codec!{
#[borrowed_json(protocol="crate::projection::KernelJson")]
#[value(rename_all = "camelCase")]
pub enum GridTrack {
    Auto,
    Fraction(u8),
    Fixed(SpaceToken),
    MinContent,
    MaxContent,
}
}
