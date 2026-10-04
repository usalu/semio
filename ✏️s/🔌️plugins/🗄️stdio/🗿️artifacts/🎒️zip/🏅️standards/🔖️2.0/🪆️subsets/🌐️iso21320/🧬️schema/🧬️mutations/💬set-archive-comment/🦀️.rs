//! 💬️ `set-archive-comment` — authored as its own mutation leaf. The aggregate's original
//! `diff`/`inverse` bodies were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf
//! reconstructs its aggregate value and delegates, so the semantics are preserved by
//! construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetArchiveComment {
    pub(crate) comment: String,
    pub(crate) comment_utf8: bool,
}

impl protocol::MutationKind<ZipSnapshot, ZipIso21320Mutation> for SetArchiveComment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "archive-comment", kind: "set-archive-comment", record: "SetArchiveComment" };

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<<ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::Diff> {
        agg_diff(&ZipIso21320Mutation::SetArchiveComment(self.clone()), base)
    }
    fn inverse(&self, base: &ZipSnapshot) -> Result<Vec<ZipIso21320Mutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&ZipIso21320Mutation::SetArchiveComment(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set archive comment", "Archivkommentar setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
