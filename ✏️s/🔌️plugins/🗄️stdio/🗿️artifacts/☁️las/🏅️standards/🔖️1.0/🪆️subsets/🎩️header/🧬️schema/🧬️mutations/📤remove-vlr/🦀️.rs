//! 📤️ `remove-vlr` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! ➖️ Removes the VLR at `index` — an out-of-range index is rejected by apply with the snapshot untouched.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveVlr {
    pub index: usize,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for RemoveVlr {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "vlr", kind: "remove-vlr", record: "RemoveVlr" };

    fn diff(&self, base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as Mutation<LasSnapshot>>::Diff> {
        let Self { index } = self;
        protocol::MutationOutcome::new(diff::diff_remove_vlr(base, *index))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        let Self { index } = self;
        Ok({
            match base.vlrs.get(*index) {
                Some(v) => vec![LasMutation::InsertVlr(insert_vlr::InsertVlr { index: *index, vlr: v.clone() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove VLR", "VLR entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
