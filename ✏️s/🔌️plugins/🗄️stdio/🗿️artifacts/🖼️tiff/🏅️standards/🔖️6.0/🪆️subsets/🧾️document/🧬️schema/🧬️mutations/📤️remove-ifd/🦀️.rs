//! 🧬️ Authoritative remove-ifd mutation.
use crate::schema::diff::*;
use crate::schema::mutations::TiffMutation;
use crate::schema::snapshot::*;

//#region Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RemoveIfdMutation {
    pub index: usize,
}
//#endregion Payload

//#region Facets
//#endregion Facets

//#region Semantics
impl protocol::MutationKind<TiffSnapshot, TiffMutation> for RemoveIfdMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "ifd", kind: "remove-ifd", record: "RemoveIfd" };
    fn diff(&self, base: &TiffSnapshot) -> protocol::MutationOutcome<TiffDiff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(contribute(base, *index))
    }
    fn inverse(&self, base: &TiffSnapshot) -> Result<Vec<TiffMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok(base.ifds.get(*index).map(|ifd| TiffMutation::InsertIfd(crate::schema::mutations::InsertIfdMutation { index: *index, ifd: ifd.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove IFD", "IFD entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["remove-ifd".into()]
    }
}
pub fn contribute(base: &TiffSnapshot, index: usize) -> TiffDiff {
    if index >= base.ifds.len() {
        return TiffDiff::default();
    }
    TiffDiff { ifds: Some(TiffIfdsDiff { removed: vec![index], modified: vec![], added: vec![] }), ..Default::default() }
}
//#endregion Semantics


#[cfg(test)]
#[path = "🧪️tests/🎯️direct-behavior/🦀️.rs"]
mod tests_direct_behavior;
