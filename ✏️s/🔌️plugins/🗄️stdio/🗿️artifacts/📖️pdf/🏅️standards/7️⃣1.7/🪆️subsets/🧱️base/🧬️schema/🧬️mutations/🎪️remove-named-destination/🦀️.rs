//! 🎪️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-named-destination`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveNamedDestination {
    pub name: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveNamedDestination
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveNamedDestination> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_named_destination::SetNamedDestination>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "named-destination", kind: "remove-named-destination", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove named destination {}", self.name), &format!("Benanntes Ziel {} entfernen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveNamedDestination
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveNamedDestination> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_named_destination::SetNamedDestination>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_named_destination(base, &self.name))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.named_destinations.iter().position(|item| item.name == self.name).map(|index| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::set_named_destination::SetNamedDestination { destination: base.named_destinations[index].clone(), index: Some(index) })).into_iter().collect()
    
    })())
}
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

