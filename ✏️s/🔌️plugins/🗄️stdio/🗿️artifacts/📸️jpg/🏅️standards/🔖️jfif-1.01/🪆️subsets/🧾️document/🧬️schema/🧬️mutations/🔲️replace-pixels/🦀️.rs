//! 🧬️ Authoritative replace-pixels mutation.
use crate::schema::diff::*;
use crate::schema::mutations::JpgMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
        Ok((base.image.pixels != self.pixels).then(|| JpgMutation::ReplacePixels(ReplacePixelsMutation { pixels: base.image.pixels.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace pixels", "Pixel ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["replace-pixels".into()]
    }
}
pub fn contribute(base: &JpgSnapshot, pixels: Vec<u8>) -> JpgDiff {
    JpgDiff { pixels: (base.image.pixels != pixels).then_some(pixels), ..Default::default() }
}
//#endregion Semantics
