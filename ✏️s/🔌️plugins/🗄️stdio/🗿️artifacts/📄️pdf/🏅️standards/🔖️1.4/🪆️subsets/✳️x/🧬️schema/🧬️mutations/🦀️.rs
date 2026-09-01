//! 🧬️ Transparent PDF 1.4/X mutation registry and delegation.

use crate::artifacts::pdf::standards::v1_4::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};

//#region 🔖️Leaves
#[path = "📐️set-page-size/🦀️.rs"]
pub mod set_page_size;
pub use set_page_size::SetPageSize;
#[path = "📉️collapse-page-size/🦀️.rs"]
pub mod collapse_page_size;
pub use collapse_page_size::CollapsePageSize;
pub use set_page_size::{CONFORMANT_HEIGHT, CONFORMANT_WIDTH};
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", content = "payload", rename_all = "kebab-case", deny_unknown_fields)]
#[mutations(snapshot = PdfSnapshot, diff = PdfDiff, schema = "s.stdio.pdf.1.4.x")]
pub enum PdfX1Mutation {
    SetPageSize(SetPageSize),
    CollapsePageSize(CollapsePageSize),
}

//#endregion 🔖️Aggregate

//#region 🔖️Delegation
/// ▶️ Applies the authoritative leaf diff.
pub fn apply_x_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfX1Mutation) -> protocol::MutationOutcome<PdfDiff> {
    use protocol::Mutation;
    mutation.diff(snapshot).apply_to(snapshot)
}

/// ↩️ Returns concrete inverse operations owned by the selected leaf.
pub fn inverse_x_conformance_mutation(mutation: &PdfX1Mutation, base: &PdfSnapshot) -> Vec<PdfX1Mutation> {
    use protocol::Mutation;
    mutation.inverse(base)
}
//#endregion 🔖️Delegation

//#region 🧪️Structure
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_descriptor_and_catalog_bijection() {
        let kinds: Vec<_> = <PdfX1Mutation as protocol::SemanticMutation<PdfSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
        let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️x/🧬️schema/🧬️mutations");
        let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("../../🧪️oracle/🔣️.json")).unwrap()).unwrap();
        assert_eq!(catalog["mutationCatalogs"][0]["kinds"], serde_json::json!(kinds));
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📐️set-page-size").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[0]);
            assert!(source.join("📐️set-page-size").join("🦀️.rs").is_file());
        }
        {
            let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📉️collapse-page-size").join("🔣️component.json")).unwrap()).unwrap();
            assert_eq!(descriptor["semanticKind"], kinds[1]);
            assert!(source.join("📉️collapse-page-size").join("🦀️.rs").is_file());
        }
    }
}
//#endregion 🧪️Structure
