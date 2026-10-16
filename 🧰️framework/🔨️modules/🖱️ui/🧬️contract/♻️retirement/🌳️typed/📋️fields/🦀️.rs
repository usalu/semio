//! 📃️ Canonical document descendant traversal.
use crate::*;
use semio_framework_ui_contract_values::ui_typed_fields;
use semio_framework_ui_contract_values::retirement_protocol::{maximum_depth,done,UI_TYPED_RETIREMENT_DEPTH};
crate::ui_document_field_catalog!(ui_typed_fields);
ui_typed_fields!(UiPatch {0=>surface:SurfaceId,1=>base_revision:UiRevision,2=>revision:UiRevision,3=>ops:UiPatchOps});
impl UiTypedRetire for UiPatchOp {
    const DEPTH: usize = maximum_depth(&[UiNodeRecord::DEPTH, Component::DEPTH, LayoutSpec::DEPTH, UiNodeChildren::DEPTH, AccessibilitySpec::DEPTH, UiNodeBindings::DEPTH, <Option<MenuRef>>::DEPTH]);
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        match self {
            Self::Upsert(field) => field.retire_typed(path, value, bytes),
            Self::SetComponent { id: _, component } => component.retire_typed(path, value, bytes),
            Self::SetLayout { id: _, layout } => layout.retire_typed(path, value, bytes),
            Self::SetActivity { id: _, activity: _, disabled: _ } => Ok(done()),
            Self::SetChildren { id: _, children } => children.retire_typed(path, value, bytes),
            Self::SetStyle { id: _, style: _ } => Ok(done()),
            Self::SetAccessibility { id: _, accessibility } => accessibility.retire_typed(path, value, bytes),
            Self::SetBindings { id: _, bindings } => bindings.retire_typed(path, value, bytes),
            Self::SetMenu { id: _, menu } => menu.retire_typed(path, value, bytes),
            Self::Remove { id: _ } | Self::SetRoot { id: _ } => Ok(done()),
        }
    }
}

const _: () = assert!(UiSnapshot::DEPTH <= UI_TYPED_RETIREMENT_DEPTH);
const _: () = assert!(UiPatch::DEPTH <= UI_TYPED_RETIREMENT_DEPTH);

