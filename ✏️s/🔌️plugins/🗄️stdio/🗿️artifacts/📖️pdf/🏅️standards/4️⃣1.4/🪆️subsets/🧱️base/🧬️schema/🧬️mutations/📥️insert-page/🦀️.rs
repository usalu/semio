//! 📥️ Direct insert-page payload, sparse diff, concrete inverse, and laws.

use super::PdfMutation;
use crate::standards::v1_4::subsets::base::schema::{
    diff::{PdfDiff, PdfPageAdded, PdfPagesDiff},
    snapshot::{PageDoc, PdfSnapshot},
};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct InsertPage {
    pub index: usize,
    #[value(deserialize_with = "deserialize_page")]
    pub page: PageDoc,
}

#[derive(value_derive::FromValue)]
#[value(deny_unknown_fields)]
struct PagePayload {
    width: f64,
    height: f64,
    text: String,
}

fn deserialize_page(value: dsl::DslValue) -> Result<PageDoc, dsl::ValueError> {
    let PagePayload { width, height, text } = dsl::FromValue::from_value(value)?;
    Ok(PageDoc { width, height, text })
}
//#endregion 🔖️Payload

//#region 🔖️Behavior
impl InsertPage {
    fn valid(&self, base: &PdfSnapshot) -> bool {
        !base.pages.is_empty() && self.index <= base.pages.len() && self.page.width.is_finite() && self.page.height.is_finite()
    }
}

impl MutationKind<PdfSnapshot, PdfMutation> for InsertPage {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "page", kind: "insert-page", record: "InsertedPage" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        if !self.valid(base) {
            return MutationOutcome::error("stdio.pdf.insert-page.invalid-target", "Page target or geometry is outside the PDF 1.4 domain", self.target());
        }
        MutationOutcome::new(PdfDiff { pages: Some(PdfPagesDiff { added: vec![PdfPageAdded { index: self.index, page: self.page.clone() }], ..Default::default() }) })
    }

    fn inverse(&self, base: &PdfSnapshot) -> Vec<PdfMutation> {
        if !self.valid(base) {
            return Vec::new();
        }
        vec![PdfMutation::RemovePage(super::RemovePage { index: self.index })]
    }

    fn label(&self) -> String {
        "insert page".into()
    }

    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Behavior

//#region 🔖️Codecs
#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;
//#endregion 🔖️Codecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔄️round-trips-the-concrete-inverse/🦀️.rs"]
mod tests_round_trips_the_concrete_inverse;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
