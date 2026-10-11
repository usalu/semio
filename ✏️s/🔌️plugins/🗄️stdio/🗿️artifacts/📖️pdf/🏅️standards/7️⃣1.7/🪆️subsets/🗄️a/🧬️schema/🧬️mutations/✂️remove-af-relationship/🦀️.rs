//! ✂️ Authoritative PDF/A mutation for removing an attached file relationship.

use super::set_af_relationship::SetAfRelationship;
use super::PdfAMutation;
use crate::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::{self, PdfDiff}, snapshot::PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveAfRelationship {
    pub file_name: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for RemoveAfRelationship {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "af-relationship", kind: "remove-af-relationship", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::file_spec_named(base, &self.file_name).map_or_else(PdfDiff::default, |id| support::remove_entry_rows(base, id, "AFRelationship"));
        MutationOutcome::new(diff::graph_edit(rows))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfAMutation>, semio_framework_value::ValueError> {
        Ok({
            let Some(id) = support::file_spec_named(base, &self.file_name) else { return Ok(Vec::new()) };
            support::object(base, id).and_then(|value| support::dict_name(value, "AFRelationship")).map(|relationship| PdfAMutation::SetAfRelationship(SetAfRelationship { file_name: self.file_name.clone(), relationship: relationship.to_string(), entry_index: support::entry_position(base, id, "AFRelationship") })).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove AF relationship from \"{}\"", self.file_name), &format!("AF-Beziehung aus \"{}\" entfernen", self.file_name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.file_name.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
