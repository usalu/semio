//! 🧬️ Authoritative change-re-encode-quality mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeReEncodeQualityMutation {
    pub quality: Option<u8>,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ChangeReEncodeQualityMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "re-encode-quality", kind: "change-re-encode-quality", record: "ChangeReEncodeQuality" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { quality } = self;
        protocol::MutationOutcome::new(contribute(base, *quality))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Result<Vec<JpgMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![JpgMutation::ChangeReEncodeQuality(ChangeReEncodeQualityMutation { quality: base.re_encode_quality })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change re-encode quality", "Qualität der Neukodierung ändern")
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
#[path = "🧪️tests/🎯️direct/🦀️.rs"]
mod tests_direct_behavior;
