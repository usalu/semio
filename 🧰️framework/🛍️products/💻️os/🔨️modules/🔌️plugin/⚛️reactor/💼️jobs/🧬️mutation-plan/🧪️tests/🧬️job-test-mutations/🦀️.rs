//#region 🧬️JobTestMutations
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactPack;

#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct JobTestSnapshot {
    pub(crate) value: i32,
}

impl ArtifactPack for JobTestSnapshot {
    fn encode_pack_with(&self, _: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }
    fn decode_pack_with(bytes: &[u8], _: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })
    }
}

/// 🧮️ Ordered checked additions preserve intermediate rejection during structural composition.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub(crate) struct JobTestDiff {
    pub(crate) deltas: Vec<i32>,
}

impl protocol::DiffAlgebra<JobTestSnapshot> for JobTestDiff {
    fn inverse(&self, _base: &JobTestSnapshot) -> Self {
        Self { deltas: self.deltas.iter().rev().map(|delta| delta.saturating_neg()).collect() }
    }
    fn between(base: &JobTestSnapshot, other: &JobTestSnapshot) -> Self {
        Self { deltas: if base.value == other.value { Vec::new() } else { vec![other.value.wrapping_sub(base.value)] } }
    }
    fn is_empty(&self) -> bool {
        self.deltas.iter().all(|delta| *delta == 0)
    }
}

impl protocol::MutationDiff<JobTestSnapshot> for JobTestDiff {
    fn apply(&self, base: &JobTestSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<JobTestSnapshot> {
        let value = self.deltas.iter().try_fold(base.value, |value, delta| value.checked_add(*delta).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.value-overflow", "job fixture value addition exceeds i32").at(["value"])))?;
        Ok(JobTestSnapshot { value })
    }
    fn absorb(&mut self, other: Self) {
        self.deltas.extend(other.deltas);
    }
}

#[path = "../../🧪️testing/🧬️job-test-mutations/🧬️mutations/🦀️.rs"]
mod mutations;
pub(crate) use mutations::{AddValue, JobTestOp};

#[path = "../🧬️job-test-mutations-unit/🦀️.rs"]
mod tests;
//#endregion 🧬️JobTestMutations
