//! 🧬️ Authoritative change-header mutation.
use crate::artifacts::png::schema::diff::{self, *};
use crate::artifacts::png::schema::mutations::PngMutation;
use crate::artifacts::png::schema::snapshot::*;
use serde::{Deserialize, Serialize};

//#region Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeHeaderMutation {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: PngColorType,
    pub interlace: bool,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangeHeaderMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "header", kind: "change-header", record: "ChangeHeader" };
    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        let Self { width, height, bit_depth, color_type, interlace } = self;
        protocol::MutationOutcome::new(contribute(base, *width, *height, *bit_depth, *color_type, *interlace))
    }
    fn inverse(&self, base: &PngSnapshot) -> Vec<PngMutation> {
        let Self { width, height, bit_depth, color_type, interlace } = self;
        let outcome = <Self as protocol::MutationKind<PngSnapshot, PngMutation>>::diff(self, base);
        if <PngDiff as protocol::DiffAlgebra<PngSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![PngMutation::ChangeHeader(crate::artifacts::png::schema::mutations::ChangeHeaderMutation { width: base.width, height: base.height, bit_depth: base.bit_depth, color_type: base.color_type, interlace: base.interlace })]
    }
    fn label(&self) -> String {
        "change header".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-header".into()]
    }
}
pub fn contribute(base: &PngSnapshot, width: u32, height: u32, bit_depth: u8, color_type: PngColorType, interlace: bool) -> PngDiff {
    PngDiff {
        width: (base.width != width).then_some(width),
        height: (base.height != height).then_some(height),
        bit_depth: (base.bit_depth != bit_depth).then_some(bit_depth),
        color_type: (base.color_type != color_type).then_some(color_type),
        interlace: (base.interlace != interlace).then_some(interlace),
        ..Default::default()
    }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️component.json")).expect("committed change-header payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️component.rs"]
mod tests_direct_behavior;
