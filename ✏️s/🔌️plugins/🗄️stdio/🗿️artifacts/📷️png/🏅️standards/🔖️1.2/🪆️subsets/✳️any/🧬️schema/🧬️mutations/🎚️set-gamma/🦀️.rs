//! 🎚️ Unguarded gAMA setter: the concrete undo of every gamma change.

use crate::schema::diff::PngDiff;
use crate::schema::mutations::PngMutation;
use crate::schema::snapshot::PngGammaValue;
use crate::PngSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetGamma {
    pub gama: Option<u32>,
}

impl protocol::MutationKind<PngSnapshot, PngMutation> for SetGamma {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "gamma", kind: "set-gamma", record: "SetGamma" };

    fn diff(&self, base: &PngSnapshot) -> protocol::MutationOutcome<PngDiff> {
        match self.gama {
            Some(0) => protocol::MutationOutcome::refuse(protocol::OutcomeCode::TargetMismatch, "png: gamma must be positive", ["gAMA"]),
            gama => protocol::MutationOutcome::new(PngDiff { gamma: (base.image.gamma != gama).then_some(PngGammaValue { gama }), ..PngDiff::default() }),
        }
    }

    fn inverse(&self, base: &PngSnapshot) -> Result<Vec<PngMutation>, semio_framework_value::ValueError> {
        Ok((base.image.gamma != self.gama).then(|| PngMutation::SetGamma(SetGamma { gama: base.image.gamma })).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set gamma", "Gamma setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["chunk:gAMA".into()]
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> PngMutation {
    PngMutation::SetGamma(SetGamma { gama: Some(45_455) })
}
