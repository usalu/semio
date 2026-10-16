//! 🧬️ Canonical component copy traversal over actual fields.
use crate::*;
use semio_framework_ui_contract_values::{ui_copy_fields,ui_copy_scalar};
use semio_framework_ui_contract_values::copy_protocol::*;
ui_copy_scalar!(ContainerRole,InputKind,SelectAppearance,ToggleAppearance,SliderAppearance,UiNumberScale,TreePresentation,TreeWindowRowExtent,RowActionPlacement);
crate::ui_component_field_catalog!(ui_copy_fields);
macro_rules! variants {
    ($type:ty { $($variant:ident : $props:ty),* }) => {
        impl TypedCopy for $type {
            const DEPTH: usize = maximum(&[$(<$props as TypedCopy>::DEPTH),*]);
            fn empty_like(&self) -> Self { match self { $(Self::$variant(source) => Self::$variant(source.empty_like()),)* } }
            fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError> {
                match (self, candidate) { $((Self::$variant(source), Self::$variant(target)) => source.allocation(target, path),)* _ => Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "typed copy candidate variant differs" }) }
            }
            fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> {
                match (self, candidate) { $((Self::$variant(source), Self::$variant(target)) => source.copy_one(target, path, byte_candidate, allocation, work),)* _ => Err(error("typed copy candidate variant differs")) }
            }
        }
    };
}
variants!(Component {
    Container: ContainerProps,
    Text: TextProps,
    Button: ButtonProps,
    Separator: SeparatorProps,
    Input: InputProps,
    Select: SelectProps,
    Toggle: ToggleProps,
    KeyValueList: KeyValueListProps,
    Slider: SliderProps,
    NumberStepper: NumberStepperProps,
    Ring: RingProps,
    IconSelect: IconSelectProps,
    Progress: ProgressProps,
    Tree: TreeProps,
    TreeSection: TreeSectionProps,
    TreeItem: TreeItemProps,
    Image: ImageProps,
    Surface: SurfaceProps,
    Extension: ExtensionProps,
    Table: TableProps,
    TableRow: TableRowProps
});
