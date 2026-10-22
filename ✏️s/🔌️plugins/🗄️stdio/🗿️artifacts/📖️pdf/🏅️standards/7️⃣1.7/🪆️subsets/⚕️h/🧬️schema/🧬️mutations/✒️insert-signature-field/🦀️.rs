//! ✒️ Authoritative PDF/H mutation for inserting a named signature field.

use crate::standards::v1_7::subsets::h::schema::mutations::remove_signature_field::RemoveSignatureField;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertSignatureField {
    pub name: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub field_index: Option<usize>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for InsertSignatureField
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<InsertSignatureField> + From<crate::standards::v1_7::subsets::h::schema::mutations::remove_signature_field::RemoveSignatureField>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "signature-field", kind: "insert-signature-field", record: "Insert" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert signature field \"{}\"", self.name), &format!("Signaturfeld \"{}\" einfügen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for InsertSignatureField
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<InsertSignatureField> + From<crate::standards::v1_7::subsets::h::schema::mutations::remove_signature_field::RemoveSignatureField>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::insert_signature_field_rows(base, &self.name, &self.placements, self.field_index, self.entry_index)))
    }
    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Op::from(RemoveSignatureField { name: self.name.clone() })]
    
    })())
}
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
