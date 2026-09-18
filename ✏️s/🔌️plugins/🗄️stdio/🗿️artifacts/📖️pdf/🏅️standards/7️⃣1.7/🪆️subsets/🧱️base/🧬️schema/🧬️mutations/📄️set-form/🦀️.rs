//! 📄️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-form`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetForm {
    pub form: PdfFormXObject,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetForm {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "form", kind: "set-form", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_form(base, self.form.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.forms.iter().find(|item| item.id == self.form.id) { Some(previous) => vec![PdfMutation::SetForm(SetForm { form: previous.clone() })], None => vec![PdfMutation::RemoveForm(super::remove_form::RemoveForm { id: self.form.id.clone() })] }
    }

    fn label(&self) -> String {
        format!("Set form {}", self.form.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.form.id.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
