//! 🏷️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-properties`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetProperties {
    pub properties: PdfNamedProperties,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetProperties {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "properties", kind: "set-properties", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_properties(base, self.properties.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.properties.iter().find(|item| item.name == self.properties.name) { Some(previous) => vec![PdfMutation::SetProperties(SetProperties { properties: previous.clone(), index: None })], None => vec![PdfMutation::RemoveProperties(super::remove_properties::RemoveProperties { name: self.properties.name.clone() })] }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set properties {}", self.properties.name), &format!("Eigenschaften {} setzen", self.properties.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.properties.name.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

