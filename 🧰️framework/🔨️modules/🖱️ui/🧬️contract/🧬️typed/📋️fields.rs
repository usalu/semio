//#region 🧬️FieldRoster
/// 🧬️ Single exact field roster for typed UI ownership operations.
macro_rules! ui_typed_field_catalog {
    ($visitor:ident) => {
        $visitor!(ActionId { 0 => scope: UiText, 1 => name: UiText, 2 => version: u16 });
        $visitor!(ActionBinding { 0 => trigger: Trigger, 1 => action: ActionId, 2 => args: Option<UiValue>, 3 => capability: Option<UiText> });
        $visitor!(MenuRef { 0 => id: UiText, 1 => args: Option<UiValue> });
        $visitor!(UiIntent { 0 => surface: SurfaceId, 1 => revision: UiRevision, 2 => node: UiNodeId, 3 => node_key: UiText, 4 => trigger: Trigger, 5 => action: ActionId, 6 => args: Option<UiValue>, 7 => input: Option<UiValue>, 8 => seq: u64 });
        $visitor!(AccessibilitySpec { 0 => label: Option<Label>, 1 => description: Option<Label>, 2 => live: Liveness, 3 => shortcut: Option<UiText>, 4 => hidden: bool });
        $visitor!(DropOverlaySpec { 0 => title: Label, 1 => hint: Label, 2 => accept: Option<UiText> });
        $visitor!(SelectItem { 0 => value: UiText, 1 => label: Label });
        $visitor!(KeyValueEntry { 0 => label: Label, 1 => value: UiText });
        $visitor!(RowAction { 0 => icon: UiText, 1 => label: Option<Label>, 2 => action: ActionBinding, 3 => placement: RowActionPlacement });
        $visitor!(ContainerProps { 0 => role: ContainerRole, 1 => label: Option<Label>, 2 => description: Option<UiText>, 3 => required: Option<bool>, 4 => error: Option<UiText>, 5 => default_open: Option<bool>, 6 => drop_overlay: Option<DropOverlaySpec> });
        $visitor!(TextProps { 0 => value: Label, 1 => emphasize: Option<bool>, 2 => data_attributes: Option<UiFixedMap<UiText>> });
        $visitor!(ButtonProps { 0 => icon: UiText, 1 => label: Label });
        $visitor!(SeparatorProps {});
        $visitor!(InputProps { 0 => kind: InputKind, 1 => value: UiText, 2 => placeholder: Option<Label>, 3 => commit: Option<UiText>, 4 => min: Option<f64>, 5 => max: Option<f64>, 6 => step: Option<f64>, 7 => accept: Option<UiText> });
        $visitor!(SelectProps { 0 => value: UiText, 1 => items: UiFixedList<SelectItem>, 2 => placeholder: Option<Label> });
        $visitor!(ToggleProps { 0 => on: bool, 1 => icon: UiText, 2 => text: Option<Label> });
        $visitor!(KeyValueListProps { 0 => entries: UiFixedList<KeyValueEntry> });
        $visitor!(SliderProps { 0 => value: f64, 1 => min: f64, 2 => max: f64, 3 => step: f64, 4 => unit: Option<UiText> });
        $visitor!(NumberStepperProps { 0 => value: f64, 1 => step: f64, 2 => uniform: bool });
        $visitor!(RingProps { 0 => orb_id: UiText, 1 => t: f64 });
        $visitor!(IconSelectProps { 0 => value: UiText, 1 => uniform: bool, 2 => classifier_kind: UiText });
        $visitor!(TreeProps { 0 => interaction_domain: Option<UiText> });
        $visitor!(TreeSectionProps { 0 => label: Option<Label>, 1 => default_open: Option<bool> });
        $visitor!(TreeItemProps { 0 => label: Label, 1 => description: Option<UiText>, 2 => icon: Option<UiText>, 3 => default_open: Option<bool>, 4 => draggable: Option<bool>, 5 => drag_data: Option<UiFixedMap<UiText>>, 6 => dimmed: Option<bool>, 7 => row_actions: UiFixedList<RowAction> });
        $visitor!(ImageProps { 0 => src: UiText, 1 => alt: Option<Label> });
        $visitor!(ExtensionProps { 0 => extension: UiText, 1 => props: UiValue });
        $visitor!(SurfaceProps { 0 => kind: SurfaceKind, 1 => doc_schema: UiText, 2 => doc: SurfaceDoc, 3 => bindings: UiNodeBindings });
        $visitor!(SurfaceDoc { 0 => bytes: UiFixedBytes });
        $visitor!(GridLayout { 0 => columns: UiGridTracks, 1 => rows: UiGridTracks, 2 => column_gap: SpaceToken, 3 => row_gap: SpaceToken, 4 => padding: EdgeSpace, 5 => align: Align, 6 => justify: Justify });
        $visitor!(StackLayout { 0 => axis: Axis, 1 => gap: SpaceToken, 2 => padding: EdgeSpace, 3 => align: Align, 4 => justify: Justify, 5 => grow: bool, 6 => wrap: bool });
        $visitor!(OverlayLayout { 0 => anchor: Anchor, 1 => inset: EdgeSpace, 2 => dismissible: bool });
        $visitor!(ScrollLayout { 0 => axes: ScrollAxes, 1 => padding: EdgeSpace, 2 => sizing: Sizing });
        $visitor!(AbsoluteLayout { 0 => sizing_width: Sizing, 1 => sizing_height: Sizing });
        $visitor!(LeafLayout { 0 => width: Sizing, 1 => height: Sizing });
        $visitor!(UiNodeRecord { 0 => id: UiNodeId, 1 => key: UiText, 2 => component: Component, 3 => layout: LayoutSpec, 4 => style: StyleSpec, 5 => activity: Activity, 6 => disabled: bool, 7 => transition: Option<TransitionHint>, 8 => accessibility: AccessibilitySpec, 9 => bindings: UiNodeBindings, 10 => menu: Option<MenuRef>, 11 => children: UiNodeChildren });
        $visitor!(UiSnapshot { 0 => surface: SurfaceId, 1 => revision: UiRevision, 2 => root: UiNodeId, 3 => nodes: UiSnapshotNodes, 4 => layout_epoch: u64 });
    };
}
//#endregion 🧬️FieldRoster
