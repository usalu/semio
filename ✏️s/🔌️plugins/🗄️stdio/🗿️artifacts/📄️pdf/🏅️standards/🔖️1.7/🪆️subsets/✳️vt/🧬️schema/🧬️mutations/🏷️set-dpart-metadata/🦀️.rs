//! 🏷️ Authoritative PDF/VT mutation for set dpart metadata.

use super::remove_dpart_metadata::RemoveDpartMetadata;
use super::PdfVtMutation;
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::{conformance_support as support, diff::PdfDiff, snapshot::{PdfObject, PdfSnapshot}};
use protocol::command::DiffAlgebra;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct SetDpartMetadata {
    pub job: String,
}

impl MutationKind<PdfSnapshot, PdfVtMutation> for SetDpartMetadata {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "dpart-metadata", kind: "set-dpart-metadata", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let mut next = base.clone();
        support::set_dpart_job(&mut next, Some(&self.job));
        MutationOutcome::new(<PdfDiff as DiffAlgebra<PdfSnapshot>>::between(base, &next))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfVtMutation> {
        match support::dpart_job(base) {
            Some(job) => vec![PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job })],
            None => vec![PdfVtMutation::RemoveDpartMetadata(RemoveDpartMetadata {})],
        }
    }

    fn label(&self) -> String {
        format!("Set PDF/VT partition metadata {}", self.job)
    }

    fn target(&self) -> Vec<String> {
        vec!["DPartRoot.DPartRootNode.DPM".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[test]
    fn changes_the_owned_conformance_axis_and_plans_its_inverse() {
        let mut base = PdfSnapshot::default();
        support::insert_object(&mut base, support::dict(vec![("Type", PdfObject::Name("Catalog".to_string()))]));
        support::set_dpart_root(&mut base, "before");
        let mutation = SetDpartMetadata { job: "after".to_string() };
        let outcome = <SetDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::diff(&mutation, &base);
        let next = outcome.diff().apply(&base).unwrap();
        assert_eq!(support::dpart_job(&next).as_deref(), Some("after"));
        assert_eq!(<SetDpartMetadata as MutationKind<PdfSnapshot, PdfVtMutation>>::inverse(&mutation, &base), vec![PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job: "before".to_string() })]);
    }
}
//#endregion 🧪️Tests

//#region 🔖️Facets
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;
#[path = "📝️text/🦀️component.rs"]
pub mod text;
//#endregion 🔖️Facets
