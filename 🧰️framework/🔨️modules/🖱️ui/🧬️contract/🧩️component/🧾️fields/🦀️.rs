//! 🧬️ Canonical component field order for typed ownership operations.
#[macro_export]
macro_rules! ui_component_field_catalog {
    ($visitor:ident) => {
        $visitor!(DropOverlaySpec { 0 => title: Label, 1 => hint: Label, 2 => accept: Option<UiText> });
        $visitor!(SelectItem { 0 => value: UiText, 1 => label: Label });
        $visitor!(KeyValueEntry { 0 => label: Label, 1 => value: UiText });
        $visitor!(RowAction { 0 => icon: UiText, 1 => label: Option<Label>, 2 => verb: UiText, 3 => placement: RowActionPlacement, 4 => disabled: bool, 5 => reason: Option<Label> });
        $visitor!(RowTarget { 0 => scope: UiText, 1 => version: u16, 2 => args: Option<UiMap>, 3 => activation: Option<UiText> });
        $visitor!(ContainerProps { 0 => role: ContainerRole, 1 => label: Option<Label>, 2 => description: Option<UiText>, 3 => required: Option<bool>, 4 => error: Option<UiText>, 5 => default_open: Option<bool>, 6 => drop_overlay: Option<DropOverlaySpec> });
        $visitor!(TextProps { 0 => value: Label, 1 => emphasize: Option<bool>, 2 => data_attributes: Option<UiFixedMap<UiText>> });
        $visitor!(ButtonProps { 0 => icon: UiText, 1 => label: Label });
        $visitor!(SeparatorProps {});
        $visitor!(InputProps { 0 => kind: InputKind, 1 => value: UiText, 2 => placeholder: Option<Label>, 3 => commit: Option<UiText>, 4 => min: Option<f64>, 5 => max: Option<f64>, 6 => step: Option<f64>, 7 => accept: Option<UiText>, 8 => precision: Option<u16>, 9 => snaps: UiFixedList<f64>, 10 => display_factor: Option<f64>, 11 => limits: Option<UiNumberLimits>, 12 => draft_target: Option<UiText>, 13 => publication_revision: Option<UiPublicationRevision> });
        $visitor!(SelectProps { 0 => value: UiText, 1 => items: UiFixedList<SelectItem>, 2 => placeholder: Option<Label>, 3 => appearance: SelectAppearance });
        $visitor!(ToggleProps { 0 => on: bool, 1 => icon: UiText, 2 => text: Option<Label>, 3 => appearance: ToggleAppearance });
        $visitor!(KeyValueListProps { 0 => entries: UiFixedList<KeyValueEntry> });
        $visitor!(UiNumberBound { 0 => value: f64, 1 => exclusive: bool, 2 => refusal: Option<Label> });
        $visitor!(UiNumberLimits { 0 => min: Option<UiNumberBound>, 1 => max: Option<UiNumberBound> });
        $visitor!(SliderProps { 0 => value: f64, 1 => min: f64, 2 => max: f64, 3 => step: f64, 4 => unit: Option<UiText>, 5 => snaps: UiFixedList<f64>, 6 => appearance: SliderAppearance, 7 => scale: UiNumberScale, 8 => precision: Option<u16>, 9 => display_unit: Option<UiText>, 10 => display_factor: Option<f64>, 11 => limits: Option<UiNumberLimits> });
        $visitor!(NumberStepperProps { 0 => value: f64, 1 => step: f64, 2 => uniform: bool, 3 => min: Option<f64>, 4 => max: Option<f64>, 5 => precision: Option<u16>, 6 => snaps: UiFixedList<f64>, 7 => unit: Option<UiText>, 8 => display_unit: Option<UiText>, 9 => display_factor: Option<f64>, 10 => limits: Option<UiNumberLimits> });
        $visitor!(RingProps { 0 => orb_id: UiText, 1 => t: f64 });
        $visitor!(IconSelectProps { 0 => value: UiText, 1 => uniform: bool, 2 => classifier_kind: UiText });
        $visitor!(ProgressProps { 0 => completed: f64, 1 => total: Option<f64>, 2 => value_text: Label });
        $visitor!(TreeWindow { 0 => total: u32, 1 => offset: u32, 2 => row_extent: TreeWindowRowExtent });
        $visitor!(TreeProps { 0 => interaction_domain: Option<UiText>, 1 => presentation: TreePresentation });
        $visitor!(TreeSectionProps { 0 => label: Option<Label>, 1 => default_open: Option<bool>, 2 => header_toolbar: Option<UiNodeId>, 3 => window: Option<TreeWindow> });
        $visitor!(TreeItemProps { 0 => label: Label, 1 => description: Option<UiText>, 2 => icon: Option<UiText>, 3 => default_open: Option<bool>, 4 => draggable: Option<bool>, 5 => drag_data: Option<UiFixedMap<UiText>>, 6 => dimmed: Option<bool>, 7 => window: Option<TreeWindow>, 8 => granularity: Option<UiText>, 9 => inline_toolbar: Option<UiNodeId>, 10 => row_actions: UiFixedList<RowAction>, 11 => detail: Option<UiNodeId>, 12 => target: Option<RowTarget>, 13 => selected: Option<bool> });
        $visitor!(TableProps { 0 => label: Label, 1 => columns: UiFixedList<Label>, 2 => actions_label: Option<Label>, 3 => window: Option<TreeWindow>, 4 => row_label: Option<Label>, 5 => column_label: Option<Label>, 6 => column_window: Option<TreeWindow> });
        $visitor!(TableRowProps { 0 => cells: UiFixedList<UiText>, 1 => row_actions: UiFixedList<RowAction>, 2 => target: Option<RowTarget> });
        $visitor!(ImageProps { 0 => src: UiText, 1 => alt: Option<Label> });
        $visitor!(ExtensionProps { 0 => extension: UiText, 1 => props: UiValue });
    };
}

#[cfg(test)]
mod tests{macro_rules! ordinal{(InputProps{$($ordinal:literal=>$field:ident:$ty:ty),*$(,)?})=>{{let roster=[$(($ordinal,stringify!($field))),*];assert_eq!(roster.last(),Some(&(13,"publication_revision")));}};($name:ident{$($fields:tt)*})=>{};}#[test]fn publication_revision_remains_field_thirteen(){crate::ui_component_field_catalog!(ordinal);}}
