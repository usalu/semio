//! 🔀️ Authoritative PDF mutation payload, diff, inverse, and tests for `move-page`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MovePage {
    pub from: usize,
    pub to: usize,
}

impl<Op> MutationKind<PdfSnapshot, Op> for MovePage
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<MovePage>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "page", kind: "move-page", record: "Move" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        MutationOutcome::new(diff::diff_move_page(base, self.from, self.to))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        if base.pages.get(self.from).is_none() {
            Vec::new()
        } else {
            vec![Op::from(MovePage { from: self.to.min(base.pages.len().saturating_sub(1)), to: self.from })]
        }
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move page {} to {}", self.from, self.to), &format!("Seite {} nach {} verschieben", self.from, self.to))
    }

    fn target(&self) -> Vec<String> {
        vec![self.from.to_string(), self.to.to_string()]
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

