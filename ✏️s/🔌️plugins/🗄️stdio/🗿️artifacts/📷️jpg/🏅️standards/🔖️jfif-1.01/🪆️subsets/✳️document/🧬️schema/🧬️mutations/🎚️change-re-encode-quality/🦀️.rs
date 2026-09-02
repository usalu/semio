//! 🧬️ Authoritative change-re-encode-quality mutation.
use crate::artifacts::jpg::schema::diff::{self, *};
use crate::artifacts::jpg::schema::mutations::JpgMutation;
use crate::artifacts::jpg::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeReEncodeQualityMutation {
    pub quality: Option<u8>,
}
//#endregion Payload

//#region Facets
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ChangeReEncodeQualityMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "re-encode-quality", kind: "change-re-encode-quality", record: "ChangeReEncodeQuality" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { quality } = self;
        protocol::MutationOutcome::new(contribute(base, *quality))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Vec<JpgMutation> {
        let Self { quality } = self;
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![JpgMutation::ChangeReEncodeQuality(crate::artifacts::jpg::schema::mutations::ChangeReEncodeQualityMutation { quality: base.re_encode_quality })]
    }
    fn label(&self) -> String {
        "change re encode quality".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["change-re-encode-quality".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, quality: Option<u8>) -> JpgDiff {
    JpgDiff { re_encode_quality: (base.re_encode_quality != quality).then_some(quality), ..Default::default() }
}
//#endregion Semantics

#[cfg(test)]
pub(crate) fn test_case() -> JpgMutation {
    serde_json::from_str(include_str!("🧪️tests/direct-behavior/🦠️mutation/🔣️.json")).expect("committed change-re-encode-quality payload")
}
#[cfg(test)]
#[path = "🧪️tests/direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
