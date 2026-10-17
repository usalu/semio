//! 🏳️ Authoritative PDF mutation payload, diff, inverse, and tests for `set-output-intents`.

use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetOutputIntents {
    pub intents: Vec<PdfOutputIntent>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetOutputIntents
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetOutputIntents>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "output-intents", kind: "set-output-intents", record: "Set" };

    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let _ = base;
        MutationOutcome::new(diff::diff_set_output_intents(base, &self.intents))
    }

    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        let _ = base;
        vec![Op::from(SetOutputIntents { intents: base.output_intents.clone() })]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set {} output intents", self.intents.len()), &format!("{} Ausgabebedingungen setzen", self.intents.len()))
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

