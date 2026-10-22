//! 🔏️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-mark-info`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetMarkInfo {
    pub info: Option<PdfMarkInfo>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetMarkInfo
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetMarkInfo>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "mark-info", kind: "set-mark-info", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set mark-info", "Markierungsinfo setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetMarkInfo
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetMarkInfo>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_mark_info(base, self.info.clone()))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        vec![Op::from(SetMarkInfo { info: base.mark_info.clone() })]
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

