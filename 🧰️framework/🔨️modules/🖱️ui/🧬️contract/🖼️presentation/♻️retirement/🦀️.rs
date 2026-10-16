//! ♻️ Exact presentation record descendant retirement.
use crate::*;
use semio_framework_ui_contract_values::{ui_typed_fields,ui_typed_scalar};
use semio_framework_ui_contract_values::retirement_protocol::{maximum_depth,done};
ui_typed_scalar!(TransitionHint,Activity,StyleSpec,SurfaceKind,Liveness,GridTrack,SpaceToken,Align,Justify,EdgeSpace,Axis,Anchor,ScrollAxes,Sizing,UiPublicationRevision);
impl UiTypedRetire for Label {
    fn original_release_demand(&self, _: &[u8], _: &Option<UiValueRetirement>) -> Result<usize, &'static str> { Ok(0) }
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        self.0.retire_typed(path, value, bytes)
    }
}
crate::ui_presentation_field_catalog!(ui_typed_fields);
impl UiTypedRetire for LayoutSpec {
    const DEPTH: usize = maximum_depth(&[LeafLayout::DEPTH, StackLayout::DEPTH, GridLayout::DEPTH, OverlayLayout::DEPTH, ScrollLayout::DEPTH, AbsoluteLayout::DEPTH]);
    fn original_release_demand(&self,path:&[u8],value:&Option<UiValueRetirement>)->Result<usize,&'static str>{
        match self {
            Self::Leaf(field) => field.original_release_demand(path, value),
            Self::Stack(field) => field.original_release_demand(path, value),
            Self::Grid(field) => field.original_release_demand(path, value),
            Self::Overlay(field) => field.original_release_demand(path, value),
            Self::Scroll(field) => field.original_release_demand(path, value),
            Self::Absolute(field) => field.original_release_demand(path, value),
        }
    }
    fn retire_typed(&mut self, path: &mut [u8], value: &mut Option<UiValueRetirement>, bytes: usize) -> Result<UiValueRetirementStep, &'static str> {
        match self {
            Self::Leaf(field) => field.retire_typed(path, value, bytes),
            Self::Stack(field) => field.retire_typed(path, value, bytes),
            Self::Grid(field) => field.retire_typed(path, value, bytes),
            Self::Overlay(field) => field.retire_typed(path, value, bytes),
            Self::Scroll(field) => field.retire_typed(path, value, bytes),
            Self::Absolute(field) => field.retire_typed(path, value, bytes),
        }
    }
}
