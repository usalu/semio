//! 🎯️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-named-destination`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetNamedDestination {
    pub destination: PdfNamedDestination,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetNamedDestination
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetNamedDestination> + From<crate::standards::v1_7::subsets::base::schema::mutations::remove_named_destination::RemoveNamedDestination>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "named-destination", kind: "set-named-destination", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_named_destination(base, self.destination.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.named_destinations.iter().find(|item| item.name == self.destination.name) { Some(previous) => vec![Op::from(SetNamedDestination { destination: previous.clone(), index: None })], None => vec![Op::from(crate::standards::v1_7::subsets::base::schema::mutations::remove_named_destination::RemoveNamedDestination { name: self.destination.name.clone() })] }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set named destination {}", self.destination.name), &format!("Benanntes Ziel {} setzen", self.destination.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.destination.name.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

