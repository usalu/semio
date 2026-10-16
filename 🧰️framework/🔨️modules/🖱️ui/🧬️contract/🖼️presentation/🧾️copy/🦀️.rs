//! 🧬️ Canonical presentation copy traversal over actual fields.
use crate::*;
use semio_framework_ui_contract_values::{ui_copy_fields,ui_copy_scalar};
use semio_framework_ui_contract_values::copy_protocol::*;
ui_copy_scalar!(Activity,TransitionHint,StyleSpec,SurfaceKind,Liveness,GridTrack,SpaceToken,Align,Justify,EdgeSpace,Axis,Anchor,ScrollAxes,Sizing,UiPublicationRevision);
crate::ui_presentation_field_catalog!(ui_copy_fields);
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
variants!(LayoutSpec { Leaf: LeafLayout, Stack: StackLayout, Grid: GridLayout, Overlay: OverlayLayout, Scroll: ScrollLayout, Absolute: AbsoluteLayout });
macro_rules! wrapper {
    ($($type:ty),*) => {$(impl TypedCopy for $type {
        fn empty_like(&self) -> Self { Self(self.0.empty_like()) }
        fn allocation(&self, candidate: &Self, path: &[usize]) -> Result<usize, PagedListError> { self.0.allocation(&candidate.0, path) }
        fn copy_one(&self, candidate: &mut Self, path: &mut [usize], byte_candidate: &mut Vec<u8>, allocation: usize, work: usize) -> Result<UiComponentCopyProgress, PagedListAllocationError> { self.0.copy_one(&mut candidate.0, path, byte_candidate, allocation, work) }
    })*};
}
wrapper!(Label);
