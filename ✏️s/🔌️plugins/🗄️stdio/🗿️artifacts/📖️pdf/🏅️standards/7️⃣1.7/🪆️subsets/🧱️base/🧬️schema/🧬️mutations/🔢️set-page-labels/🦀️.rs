//! 🔢️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-labels`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageLabels {
    pub labels: Vec<PdfPageLabelRange>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetPageLabels
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetPageLabels>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-labels", kind: "set-page-labels", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_page_labels(base, &self.labels))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        vec![Op::from(SetPageLabels { labels: base.page_labels.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set {} page label ranges", self.labels.len()), &format!("{} Seitenbeschriftungsbereiche setzen", self.labels.len()))
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

