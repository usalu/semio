//! 🌅️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-shading`.

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
pub struct SetShading {
    pub shading: PdfShading,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetShading {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "shading", kind: "set-shading", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_shading(base, self.shading.clone()))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        let _ = base;
        match base.shadings.iter().find(|item| item.id == self.shading.id) { Some(previous) => vec![PdfMutation::SetShading(SetShading { shading: previous.clone() })], None => vec![PdfMutation::RemoveShading(super::remove_shading::RemoveShading { id: self.shading.id.clone() })] }
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Set shading {}", self.shading.id), &format!("Verschattung {} setzen", self.shading.id))
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

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
