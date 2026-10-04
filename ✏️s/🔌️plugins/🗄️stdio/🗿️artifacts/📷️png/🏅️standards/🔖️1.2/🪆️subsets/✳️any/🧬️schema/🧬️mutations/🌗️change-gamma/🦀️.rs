//! 🌗️ Revision-guarded exact gAMA chunk edit.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::{PngMutation, SetSnapshot};
use crate::PngSnapshot;
use protocol::DiffAlgebra;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ChangeGammaMutation {
    pub revision: String,
    pub gama: Option<u32>,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<PngSnapshot, PngMutation> for ChangeGammaMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "gamma", kind: "change-gamma", record: "ChangeGamma" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        match crate::io::set_gamma_chunk_controlled(base, &self.revision, self.gama, &mut |_, _| true) {
            Ok(next) => protocol::MutationOutcome::new(PngDiff::between(base, &next)),
            Err(message) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, message, ["gAMA"]),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Result<Vec<PngMutation>, semio_framework_value::ValueError> {
        Ok(vec![PngMutation::SetSnapshot(SetSnapshot { snapshot: base.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change gamma", "Gamma ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["chunk:gAMA".into()]
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    let base = PngSnapshot::default();
    PngMutation::ChangeGamma(ChangeGammaMutation { revision: crate::io::png_revision(&base), gama: Some(45_455) })
}
