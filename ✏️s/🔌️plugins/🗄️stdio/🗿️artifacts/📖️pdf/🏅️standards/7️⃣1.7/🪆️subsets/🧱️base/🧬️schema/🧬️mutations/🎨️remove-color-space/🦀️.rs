//! 🎨️ Authoritative PDF mutation payload, diff, inverse, and tests for `remove-color-space`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveColorSpace {
    pub name: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveColorSpace
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveColorSpace> + From<crate::standards::v1_7::subsets::base::schema::mutations::set_color_space::SetColorSpace>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "color-space", kind: "remove-color-space", record: "Remove" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_remove_color_space(base, &self.name))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        base.color_spaces.iter().position(|item| item.name == self.name).map(|index| Op::from(crate::standards::v1_7::subsets::base::schema::mutations::set_color_space::SetColorSpace { color_space: base.color_spaces[index].clone(), index: Some(index) })).into_iter().collect()
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove color-space {}", self.name), &format!("Farbraum {} entfernen", self.name))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

