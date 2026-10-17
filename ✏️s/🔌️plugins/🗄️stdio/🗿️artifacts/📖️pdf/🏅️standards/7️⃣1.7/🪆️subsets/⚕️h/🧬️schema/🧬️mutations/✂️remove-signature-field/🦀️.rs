//! ✂️ Authoritative PDF/H mutation for removing a matching signature field.

use crate::standards::v1_7::subsets::h::schema::mutations::insert_signature_field::InsertSignatureField;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveSignatureField {
    pub name: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveSignatureField
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveSignatureField> + From<crate::standards::v1_7::subsets::h::schema::mutations::insert_signature_field::InsertSignatureField>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "signature-field", kind: "remove-signature-field", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::graph_edit(support::remove_signature_field_rows(base, &self.name)))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::signature_field_named(base, &self.name).map(|field| {
                let fields = support::signature_fields(base);
                Op::from(InsertSignatureField { name: self.name.clone(), placements: support::placements_of(base, &[field]), field_index: fields.iter().position(|candidate| *candidate == field), entry_index: (fields.len() == 1).then(|| support::catalog_entry_position(base, "AcroForm")).flatten() })
            }).into_iter().collect()
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove signature field \"{}\"", self.name), &format!("Signaturfeld \"{}\" entfernen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
