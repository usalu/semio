//#region 🔗️DependencyContributionFixture
//! 🔗️ Concrete value contribution used only by PluginBuilder dependency tests.
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct DependencyTestSnapshot {
    pub value: i32,
}

impl pack::snapshot::ArtifactPack for DependencyTestSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for DependencyTestSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(deny_unknown_fields)]
#[value(deny_unknown_fields)]
pub struct DependencyTestDiff {
    pub deltas: Vec<i32>,
}

impl protocol::DiffAlgebra<DependencyTestSnapshot> for DependencyTestDiff {

    fn is_empty(&self) -> bool {
        self.deltas.iter().all(|delta| *delta == 0)
    }
}

impl protocol::ColdDiffAlgebra<DependencyTestSnapshot> for DependencyTestDiff {
    fn inverse(&self, _base: &DependencyTestSnapshot) -> Self {
        Self { deltas: self.deltas.iter().rev().flat_map(|delta| if *delta == i32::MIN { vec![i32::MAX, 1] } else { vec![-delta] }).collect() }
    }
}

impl protocol::MutationDiff<DependencyTestSnapshot> for DependencyTestDiff {


}

impl protocol::ColdMutationDiff<DependencyTestSnapshot> for DependencyTestDiff {
    fn absorb(&mut self, other: Self) {
        self.deltas.extend(other.deltas);
    }

    fn apply(&self, base: &mut DependencyTestSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let mut value = base.value;
        for delta in &self.deltas {
            value = value.checked_add(*delta).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.overflow", "builder contribution exceeds i32").at(["value"]))?;
        }
                base.value = value;
        Ok(())
    }
}

#[doc(hidden)]
#[path = "../../🧪️testing/🔗️dependency-contribution/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use crate::builder_test_hosts::builder_unit_1::dependency_fixture::mutations::add_value::AddValue;
pub use crate::builder_test_hosts::builder_unit_1::dependency_fixture::mutations::DependencyTestOp;

#[doc(hidden)]
#[path = "../🔗️dependency-contribution-unit/🦀️.rs"]
pub mod tests;
//#endregion 🔗️DependencyContributionFixture
