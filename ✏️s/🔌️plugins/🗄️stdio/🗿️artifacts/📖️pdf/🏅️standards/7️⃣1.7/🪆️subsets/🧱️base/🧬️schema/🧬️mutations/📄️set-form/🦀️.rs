//! 📄️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-form`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetForm {
    pub form: PdfFormXObject,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetForm
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetForm> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_form::RemoveForm>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "form", kind: "set-form", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_form(base, self.form.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.forms.iter().find(|item| item.id == self.form.id) { Some(previous) => vec![Op::from(SetForm { form: previous.clone(), index: None })], None => vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_form::RemoveForm { id: self.form.id.clone() })] }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set form {}", self.form.id), &format!("Formular {} setzen", self.form.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.form.id.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

