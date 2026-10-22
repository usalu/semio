//! 🗞️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-form`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveForm {
    pub id: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveForm
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveForm> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_form::SetForm>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "form", kind: "remove-form", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove form {}", self.id), &format!("Formular {} entfernen", self.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveForm
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveForm> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_form::SetForm>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_form(base, &self.id))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.forms.iter().position(|item| item.id == self.id).map(|index| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::set_form::SetForm { form: base.forms[index].clone(), index: Some(index) })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

