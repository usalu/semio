//! 🧬️ Authoritative replace-pixels mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplacePixelsMutation {
    pub pixels: Vec<u8>,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<JpgSnapshot, JpgMutation> for ReplacePixelsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "pixels", kind: "replace-pixels", record: "ReplacePixels" };
    fn diff(&self, base: &JpgSnapshot) -> protocol::MutationOutcome<JpgDiff> {
        let Self { pixels } = self;
        protocol::MutationOutcome::new(contribute(base, pixels.clone()))
    }
    fn inverse(&self, base: &JpgSnapshot) -> Result<Vec<JpgMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let outcome = <Self as protocol::MutationKind<JpgSnapshot, JpgMutation>>::diff(self, base);
        if <JpgDiff as protocol::DiffAlgebra<JpgSnapshot>>::is_empty(outcome.diff()) {
            return Vec::new();
        }
        vec![JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels: base.pixels.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace pixels", "Pixel ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-pixels".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, pixels: Vec<u8>) -> JpgDiff {
    JpgDiff { pixels: (base.pixels != pixels).then_some(pixels), ..Default::default() }
}
//#endregion Semantics


