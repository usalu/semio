//! 📎️ Authoritative PDF/A mutation for inserting an attached file specification.

use super::remove_embedded_file::RemoveEmbeddedFile;
use super::PdfAMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::{conformance_support as support, diff::PdfDiff, snapshot::PdfSnapshot};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertEmbeddedFile {
    pub file_name: String,
}

impl MutationKind<PdfSnapshot, PdfAMutation> for InsertEmbeddedFile {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "embedded-file", kind: "insert-embedded-file", record: "Insert" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::insert_file_spec(&mut next, &self.file_name);
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, _base: &PdfSnapshot) -> Vec<PdfAMutation> {
        vec![PdfAMutation::RemoveEmbeddedFile(RemoveEmbeddedFile { file_name: self.file_name.clone() })]
    }

    fn label(&self) -> String {
        format!("Insert embedded file \"{}\"", self.file_name)
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
    fn inserts_the_named_file_specification() {
        let base = PdfSnapshot::default();
        let mutation = InsertEmbeddedFile { file_name: "measurements.csv".to_string() };
        let outcome = <InsertEmbeddedFile as MutationKind<PdfSnapshot, PdfAMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert!(support::file_spec_named(&next, &mutation.file_name).is_some());
    }
}
//#endregion 🧪️Tests
