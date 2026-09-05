//! 🧬️ Authoritative change-header-fields mutation.
use crate::artifacts::bmp::schema::diff::{self, *};
use crate::artifacts::bmp::schema::mutations::BmpMutation;
use crate::artifacts::bmp::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeHeaderFieldsMutation {
    pub header_size: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub row_order: Option<BmpRowOrder>,
    pub planes: Option<u16>,
    pub bits_per_pixel: Option<u16>,
    pub compression: Option<u32>,
    pub image_size: Option<u32>,
    pub x_pixels_per_meter: Option<i32>,
    pub y_pixels_per_meter: Option<i32>,
    pub colors_used: Option<u32>,
    pub colors_important: Option<u32>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<BmpSnapshot, BmpMutation> for ChangeHeaderFieldsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "header-fields", kind: "change-header-fields", record: "ChangeHeaderFields" };
    fn diff(&self, base: &BmpSnapshot) -> protocol::MutationOutcome<BmpDiff> {
        let Self { header_size, width, height, row_order, planes, bits_per_pixel, compression, image_size, x_pixels_per_meter, y_pixels_per_meter, colors_used, colors_important } = self;
        protocol::MutationOutcome::new(BmpDiff {
            header_size: *header_size,
            width: *width,
            height: *height,
            row_order: *row_order,
            planes: *planes,
            bits_per_pixel: *bits_per_pixel,
            compression: *compression,
            image_size: *image_size,
            x_pixels_per_meter: *x_pixels_per_meter,
            y_pixels_per_meter: *y_pixels_per_meter,
            colors_used: *colors_used,
            colors_important: *colors_important,
            ..Default::default()
        })
    }
    fn inverse(&self, base: &BmpSnapshot) -> Vec<BmpMutation> {
        let Self { header_size, width, height, row_order, planes, bits_per_pixel, compression, image_size, x_pixels_per_meter, y_pixels_per_meter, colors_used, colors_important } = self;
        let outcome = <Self as protocol::MutationKind<BmpSnapshot, BmpMutation>>::diff(self, base);
        if <BmpDiff as protocol::DiffAlgebra<BmpSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![BmpMutation::ChangeHeaderFields(crate::artifacts::bmp::schema::mutations::ChangeHeaderFieldsMutation {
            header_size: header_size.map(|_| base.header_size),
            width: width.map(|_| base.width),
            height: height.map(|_| base.height),
            row_order: row_order.map(|_| base.row_order),
            planes: planes.map(|_| base.planes),
            bits_per_pixel: bits_per_pixel.map(|_| base.bits_per_pixel),
            compression: compression.map(|_| base.compression),
            image_size: image_size.map(|_| base.image_size),
            x_pixels_per_meter: x_pixels_per_meter.map(|_| base.x_pixels_per_meter),
            y_pixels_per_meter: y_pixels_per_meter.map(|_| base.y_pixels_per_meter),
            colors_used: colors_used.map(|_| base.colors_used),
            colors_important: colors_important.map(|_| base.colors_important),
        })]
    }
    fn label(&self) -> String {
        "change header fields".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-header-fields".into()]
    }
}

//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> BmpMutation {
    serde_json::from_str(include_str!("🧪️tests/🎯️direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-header-fields payload")
}
#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
