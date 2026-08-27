//! 🗑️ Authoritative PDF/A mutation for removing a named attached file specification.

use super::insert_embedded_file::InsertEmbeddedFile;
use super::PdfAMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveEmbeddedFile {
    pub file_name: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for RemoveEmbeddedFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "embedded-file", kind: "remove-embedded-file", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        if let Some(id) = support::file_spec_named(&next, &self.file_name) {
            support::remove_object(&mut next, id);
        }
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfAMutation> {
        support::file_spec_named(base, &self.file_name)
            .map(|_| PdfAMutation::InsertEmbeddedFile(InsertEmbeddedFile { file_name: self.file_name.clone() }))
            .into_iter()
            .collect()
    }

    fn label(&self) -> String {
        format!("Remove embedded file \"{}\"", self.file_name)
    }

    fn target(&self) -> Vec<String> {
        vec![self.file_name.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn removes_the_named_file_specification() {
        let mut base = PdfSnapshot::default();
        support::insert_file_spec(&mut base, "measurements.csv");
        let mutation = RemoveEmbeddedFile { file_name: "measurements.csv".to_string() };
        let outcome = <RemoveEmbeddedFile as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::file_spec_named(&next, &mutation.file_name).is_none());
    }
}
//#endregion 🧪️Tests
