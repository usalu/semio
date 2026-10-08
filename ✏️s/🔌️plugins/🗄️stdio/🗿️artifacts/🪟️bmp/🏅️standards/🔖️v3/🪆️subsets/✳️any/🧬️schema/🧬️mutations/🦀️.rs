//! 🧬️ Precise owned sample BMP mutation aggregate.

use crate::schema::diff::BmpDiff;
use crate::BmpSnapshot;

pub use super::paint_direct_region::PaintDirectRegion;
pub use super::paint_indexed_region::PaintIndexedRegion;
pub use super::replace_image::ReplaceImage;
pub use crate::schema::operations::apply_bmp_mutation;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case")]
#[mutations(snapshot = BmpSnapshot, diff = BmpDiff, schema = "s.stdio.bmp")]
pub enum BmpMutation {
    ReplaceImage(ReplaceImage),
    PaintIndexedRegion(PaintIndexedRegion),
    PaintDirectRegion(PaintDirectRegion),
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<BmpMutation> {
    let snapshot = BmpSnapshot::default();
    vec![
        BmpMutation::PaintIndexedRegion(PaintIndexedRegion { revision: "stale".into(), x: 0, y: 0, width: 1, height: 1, palette_index: 0 }),
        BmpMutation::PaintDirectRegion(PaintDirectRegion { revision: "stale".into(), x: 0, y: 0, width: 1, height: 1, red: 0, green: 0, blue: 0, alpha: 255 }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
