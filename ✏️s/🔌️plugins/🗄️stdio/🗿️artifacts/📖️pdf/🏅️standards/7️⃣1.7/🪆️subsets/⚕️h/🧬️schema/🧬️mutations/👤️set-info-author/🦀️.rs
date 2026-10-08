//! 👤️ Authoritative PDF/H mutation for setting the document author conformance axis.

use super::PdfHMutation;
use crate::standards::v1_7::subsets::base::schema::{diff::{PdfDiff, PdfInfoDiff, PdfSet}, snapshot::{PdfSnapshot}};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInfoAuthor {
    pub author: String,
}

impl MutationKind<PdfSnapshot, PdfHMutation> for SetInfoAuthor {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info-author", kind: "set-info-author", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let change = (base.info.author.as_deref() != Some(self.author.as_str())).then(|| PdfInfoDiff { author: Some(PdfSet::Set { value: self.author.clone() }), ..Default::default() });
        MutationOutcome::new(PdfDiff { info: change, ..Default::default() })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<PdfHMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![PdfHMutation::SetInfoAuthor(SetInfoAuthor { author: base.info.author.clone().unwrap_or_default() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/H author \"{}\"", self.author), &format!("PDF/H-Autor \"{}\" setzen", self.author))
    }

    fn target(&self) -> Vec<String> {
        vec!["Info.Author".to_string()]
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
