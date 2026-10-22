//! 📏️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-page-user-unit`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPageUserUnit {
    pub index: usize,
    pub user_unit: Option<f64>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetPageUserUnit
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetPageUserUnit>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "page-user-unit", kind: "set-page-user-unit", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set page {} user unit", self.index), &format!("Benutzereinheit von Seite {} setzen", self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetPageUserUnit
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetPageUserUnit>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_page_user_unit(self.index, self.user_unit))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.pages.get(self.index).map(|page| Op::from(SetPageUserUnit { index: self.index, user_unit: page.user_unit })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

