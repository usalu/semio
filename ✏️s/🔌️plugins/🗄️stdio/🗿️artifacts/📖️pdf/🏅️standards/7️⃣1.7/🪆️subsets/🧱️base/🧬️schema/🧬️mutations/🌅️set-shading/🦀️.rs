//! 🌅️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-shading`.

use super::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::{
    diff::{self, PdfDiff},
    snapshot::*,
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetShading {
    pub shading: PdfShading,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetShading {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "shading", kind: "set-shading", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_shading(base, self.shading.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.shadings.iter().find(|item| item.id == self.shading.id) { Some(previous) => vec![PdfMutation::SetShading(SetShading { shading: previous.clone(), index: None })], None => vec![PdfMutation::RemoveShading(super::remove_shading::RemoveShading { id: self.shading.id.clone() })] }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set shading {}", self.shading.id), &format!("Schattierung {} setzen", self.shading.id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.shading.id.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

