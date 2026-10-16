//! 🧬️ Canonical presentation compare traversal over actual fields.
use crate::*;
use semio_framework_ui_contract_values::{ui_compare_fields,ui_compare_scalar};
use semio_framework_ui_contract_values::compare_protocol::*;
ui_compare_scalar!(Activity,TransitionHint,StyleSpec,SurfaceKind,Liveness,GridTrack,SpaceToken,Align,Justify,EdgeSpace,Axis,Anchor,ScrollAxes,Sizing,UiPublicationRevision);
crate::ui_presentation_field_catalog!(ui_compare_fields);
macro_rules! variants {
    ($type:ty { $($variant:ident : $props:ty),* }) => {impl TypedCompare for $type {
        const DEPTH: usize = maximum(&[$(<$props as TypedCompare>::DEPTH),*]);
        fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> {
            match (self, right) { $((Self::$variant(left), Self::$variant(right)) => left.compare_one(right, path, values, bytes),)* _ => Ok(result(false)) }
        }
    }};
}
variants!(LayoutSpec { Leaf: LeafLayout, Stack: StackLayout, Grid: GridLayout, Overlay: OverlayLayout, Scroll: ScrollLayout, Absolute: AbsoluteLayout });
macro_rules! wrapper {
    ($($type:ty),*) => {$(impl TypedCompare for $type {
        const DEPTH: usize = <UiText as TypedCompare>::DEPTH;
        fn compare_one(&self, right: &Self, path: &mut [usize], values: &mut ValueComparison, bytes: usize) -> Result<UiComponentCompareProgress, &'static str> { self.0.compare_one(&right.0, path, values, bytes) }
    })*};
}
wrapper!(Label);
