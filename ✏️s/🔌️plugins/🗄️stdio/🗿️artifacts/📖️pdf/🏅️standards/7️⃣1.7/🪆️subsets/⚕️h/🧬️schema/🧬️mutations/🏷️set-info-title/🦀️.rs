//! 🏷️ Authoritative PDF/H mutation for setting the document title conformance axis.

use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{diff::{PdfDiff, PdfInfoDiff, PdfSet}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInfoTitle {
    pub title: String,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for SetInfoTitle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info-title", kind: "set-info-title", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let change = (base.info.title.as_deref() != Some(self.title.as_str())).then(|| PdfInfoDiff { title: Some(PdfSet::Set { value: self.title.clone() }), ..Default::default() });
        MutationOutcome::new(PdfDiff { info: change, ..Default::default() })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfHMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfHMutation::SetInfoTitle(SetInfoTitle { title: base.info.title.clone().unwrap_or_default() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/H title \"{}\"", self.title), &format!("PDF/H-Titel \"{}\" setzen", self.title))
    }

    fn target(&self) -> Vec<String> {
        vec!["Info.Title".to_string()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
