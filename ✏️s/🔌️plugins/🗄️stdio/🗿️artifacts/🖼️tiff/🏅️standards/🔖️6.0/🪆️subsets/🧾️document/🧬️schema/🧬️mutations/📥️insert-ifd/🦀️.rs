//! 🧬️ Authoritative insert-ifd mutation.
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertIfdMutation {
    pub index: usize,
    pub ifd: TiffIfd,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for InsertIfdMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "ifd", kind: "insert-ifd", record: "InsertIfd" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { index, ifd } = self;
        protocol::MutationOutcome::new(contribute(base, *index, ifd.clone()))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok(vec![TiffMutation::RemoveIfd(crate::schema::mutations::RemoveIfdMutation { index: (*index).min(base.ifds.len()) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert IFD", "IFD einfügen")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-ifd".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, index: usize, ifd: TiffIfd) -> TiffDiff {
    let at = index.min(base.ifds.len());
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![], modified: vec![], added: vec![TiffIfdAdded { index: at, ifd }] }), ..Default::default() }
}
//#endregion Semantics


#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
