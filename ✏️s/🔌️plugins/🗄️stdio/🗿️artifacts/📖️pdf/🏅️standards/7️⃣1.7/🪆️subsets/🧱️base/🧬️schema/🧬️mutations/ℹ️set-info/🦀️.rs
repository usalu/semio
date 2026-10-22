//! ℹ️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-info`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInfo {
    pub info: PdfInfo,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetInfo
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetInfo>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info", kind: "set-info", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set document info", "Dokumentinfo setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["Info".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetInfo
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetInfo>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_set_info(base, &self.info))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Op::from(SetInfo { info: base.info.clone() })]
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

