//! 🏷️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-properties`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetProperties {
    pub properties: PdfNamedProperties,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetProperties
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetProperties> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_properties::RemoveProperties>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "properties", kind: "set-properties", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set properties {}", self.properties.name), &format!("Eigenschaften {} setzen", self.properties.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.properties.name.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetProperties
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetProperties> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_properties::RemoveProperties>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_properties(base, self.properties.clone(), self.index))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.properties.iter().find(|item| item.name == self.properties.name) { Some(previous) => vec![Op::from(SetProperties { properties: previous.clone(), index: None })], None => vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_properties::RemoveProperties { name: self.properties.name.clone() })] }
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

