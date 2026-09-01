//! 📏️ `set-scale-and-offset` — its own mutation leaf. The aggregate's original `diff`/`inverse`
//! bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its
//! aggregate value and delegates, so the semantics are preserved by construction rather than
//! re-derived.
//!
//! 📏️ Sets X/Y/Z scale factors and offsets — the two header fields that jointly
//! reconstruct real-world coordinates from the on-disk integer point records — and
//! nothing else. The records keep the exact integers they carry, so every coordinate
//! is re-read under the new parameters (`coordinate = record * scale + offset`) rather
//! than held fixed while the records are silently re-quantized. Lossless and exactly
//! invertible in either direction; see
//! `../../🔺️diff/🦀️component.rs::diff_set_scale_and_offset` for the reproduction that
//! settled it.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetScaleAndOffset {
    pub scale: (f64, f64, f64),
    pub offset: (f64, f64, f64),
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetScaleAndOffset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "scale-and-offset", kind: "set-scale-and-offset", record: "SetScaleAndOffset" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as protocol::Mutation<LasSnapshot>>::Diff> {
        agg_diff(&LasMutation::SetScaleAndOffset(self.clone()), base)
    }
    fn inverse(&self, base: &LasSnapshot) -> Vec<LasMutation> {
        agg_inverse(&LasMutation::SetScaleAndOffset(self.clone()), base)
    }
    fn label(&self) -> String {
        "set-scale-and-offset".to_string()
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
