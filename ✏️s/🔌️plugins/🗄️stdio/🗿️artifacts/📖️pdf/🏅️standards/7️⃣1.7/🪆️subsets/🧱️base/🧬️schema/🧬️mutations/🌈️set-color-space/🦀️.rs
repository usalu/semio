//! 🌈️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-color-space`.

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
pub struct SetColorSpace {
    pub color_space: PdfNamedColorSpace,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}

impl MutationKind<PdfSnapshot, PdfMutation> for SetColorSpace {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "color-space", kind: "set-color-space", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_color_space(base, self.color_space.clone(), self.index))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        match base.color_spaces.iter().find(|item| item.name == self.color_space.name) { Some(previous) => vec![PdfMutation::SetColorSpace(SetColorSpace { color_space: previous.clone(), index: None })], None => vec![PdfMutation::RemoveColorSpace(super::remove_color_space::RemoveColorSpace { name: self.color_space.name.clone() })] }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set color-space {}", self.color_space.name), &format!("Farbraum {} setzen", self.color_space.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.color_space.name.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

