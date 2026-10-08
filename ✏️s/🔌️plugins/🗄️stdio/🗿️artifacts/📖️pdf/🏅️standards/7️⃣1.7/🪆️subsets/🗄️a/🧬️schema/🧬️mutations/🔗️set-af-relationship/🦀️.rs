//! 🔗️ Authoritative PDF/A mutation for setting an attached file relationship.

use super::remove_af_relationship::RemoveAfRelationship;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::{PdfObject, PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAfRelationship {
    pub file_name: String,
    pub relationship: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for SetAfRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "af-relationship", kind: "set-af-relationship", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::file_spec_named(base, &self.file_name).map_or_else(PdfDiff::default, |id| support::set_entry_rows(base, id, "AFRelationship", PdfObject::Name(self.relationship.clone())));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfAMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let Some(id) = support::file_spec_named(base, &self.file_name) else { return Vec::new() };
        match support::object(base, id).and_then(|value| support::dict_name(value, "AFRelationship")) {
            Some(previous) => vec![PdfAMutation::SetAfRelationship(SetAfRelationship { file_name: self.file_name.clone(), relationship: previous.to_string() })],
            None => vec![PdfAMutation::RemoveAfRelationship(RemoveAfRelationship { file_name: self.file_name.clone() })],
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set AF relationship for \"{}\"", self.file_name), &format!("AF-Beziehung für \"{}\" setzen", self.file_name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.file_name.clone(), self.relationship.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
