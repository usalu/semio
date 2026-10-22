//! 🔗️ Authoritative PDF/A mutation for setting an attached file relationship.

use crate::standards::v1_7::subsets::a::schema::mutations::remove_af_relationship::RemoveAfRelationship;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfObject, PdfSnapshot};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAfRelationship {
    pub file_name: String,
    pub relationship: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entry_index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetAfRelationship
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetAfRelationship> + From<crate::standards::v1_7::subsets::a::schema::mutations::remove_af_relationship::RemoveAfRelationship>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "af-relationship", kind: "set-af-relationship", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set AF relationship for \"{}\"", self.file_name), &format!("AF-Beziehung für \"{}\" setzen", self.file_name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.file_name.clone(), self.relationship.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetAfRelationship
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetAfRelationship> + From<crate::standards::v1_7::subsets::a::schema::mutations::remove_af_relationship::RemoveAfRelationship>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::file_spec_named(base, &self.file_name).map_or_else(PdfDiff::default, |id| support::set_entry_rows(base, id, "AFRelationship", PdfObject::Name(self.relationship.clone()), self.entry_index));
        MutationOutcome::new(diff::graph_edit(rows))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            let Some(id) = support::file_spec_named(base, &self.file_name) else { return Ok(Vec::new()) };
            match support::object(base, id).and_then(|value| support::dict_name(value, "AFRelationship")) {
                Some(previous) => vec![Op::from(SetAfRelationship { file_name: self.file_name.clone(), relationship: previous.to_string(), entry_index: None })],
                None => vec![Op::from(RemoveAfRelationship { file_name: self.file_name.clone() })],
            }
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests
