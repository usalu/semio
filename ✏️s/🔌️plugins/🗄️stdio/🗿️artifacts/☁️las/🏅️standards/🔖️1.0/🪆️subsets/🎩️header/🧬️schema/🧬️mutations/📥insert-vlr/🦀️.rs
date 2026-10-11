//! 📥️ `insert-vlr` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! ➕️ Inserts a fully-specified VLR at `index` (final position, clamped to `len`).
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertVlr {
    pub index: usize,
    pub vlr: LasVlr,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for InsertVlr {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "vlr", kind: "insert-vlr", record: "InsertVlr" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { index, vlr } = self;
        protocol::MutationOutcome::new(diff::diff_insert_vlr(base, *index, vlr.clone()))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({ vec![LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: (*index).min(base.vlrs.len()) })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert VLR", "VLR einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
