//! 🔓️ Authoritative PDF/E mutation for removing a matching encryption dictionary.

use crate::standards::v1_7::subsets::e::schema::mutations::insert_encryption_dictionary::InsertEncryptionDictionary;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveEncryptionDictionary {
    pub version: i64,
    pub revision: i64,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveEncryptionDictionary
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveEncryptionDictionary> + From<crate::standards::v1_7::subsets::e::schema::mutations::insert_encryption_dictionary::InsertEncryptionDictionary>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "encryption-dictionary", kind: "remove-encryption-dictionary", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove encryption dictionary V{} R{}", self.version, self.revision), &format!("Verschlüsselungswörterbuch V{} R{} entfernen", self.version, self.revision))
    }

    fn target(&self) -> Vec<String> {
        vec![self.version.to_string(), self.revision.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveEncryptionDictionary
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveEncryptionDictionary> + From<crate::standards::v1_7::subsets::e::schema::mutations::insert_encryption_dictionary::InsertEncryptionDictionary>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::encryption_dictionary_with(base, self.version, self.revision).map_or_else(PdfDiff::default, |id| support::remove_object_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::encryption_dictionary_with(base, self.version, self.revision).map(|id| Op::from(InsertEncryptionDictionary { version: self.version, revision: self.revision, placements: support::placements_of(base, &[id]) })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
