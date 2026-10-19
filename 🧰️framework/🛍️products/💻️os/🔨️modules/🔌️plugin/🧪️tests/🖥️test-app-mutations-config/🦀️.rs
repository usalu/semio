use protocol::MutationDiff;
use protocol::ColdMutationDiff;
use serde::{Deserialize, Serialize};
use pack::snapshot::ArtifactPack;

//#region 🧫️Snapshot
#[doc(hidden)]
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, Serialize, semio_framework_value_derive::ToValue, Deserialize, semio_framework_value_derive::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = store)]
#[artifact(extension = "testkit-macro-cfg")]
pub struct TestConfig {
    pub selected: Option<String>,
}
semio_framework_value::artifact_retire_struct!(TestConfig { selected });
#[doc(hidden)]
#[path="🧬️preparation/🦀️.rs"]
pub mod preparation;

impl pack::snapshot::ArtifactDsl for TestConfig {
    const EXTENSION: &'static str = "testkit-macro-cfg";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl ArtifactPack for TestConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for TestConfig {

}

impl store::ConfigRecord for TestConfig {}
//#endregion 🧫️Snapshot

//#region 🧬️Mutations
#[path = "../../🧪️testing/🖥️test-app-mutations/🎚️config/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use crate::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
pub use crate::test_app_mutation_fixture::config::mutations::TestConfigMutation;
//#endregion 🧬️Mutations

//#region 🔺️Diff
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(tag = "state", content = "value", rename_all = "camelCase")]
#[value(tag = "state", content = "value", rename_all = "camelCase")]
pub(crate) enum TestConfigDiff {
    Identity,
    Clear,
    Set(String),
}
impl Default for TestConfigDiff {
    fn default() -> Self {
        Self::Identity
    }
}
impl protocol::DiffAlgebra<TestConfig> for TestConfigDiff {

    fn is_empty(&self) -> bool {
        matches!(self, Self::Identity)
    }
}

impl protocol::ColdDiffAlgebra<TestConfig> for TestConfigDiff {
    fn inverse(&self, base: &TestConfig) -> Self {
        match (self, &base.selected) {
            (Self::Identity, _) => Self::Identity,
            (_, None) => Self::Clear,
            (_, Some(value)) => Self::Set(value.clone()),
        }
    }
}
impl MutationDiff<TestConfig> for TestConfigDiff {


}

impl ColdMutationDiff<TestConfig> for TestConfigDiff {
    fn absorb(&mut self, other: Self) {
        if !matches!(other, Self::Identity) {
            *self = other;
        }
    }

    fn apply(&self, base: &mut TestConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        match self {
            Self::Identity => {},
            Self::Clear => base.selected = None,
            Self::Set(value) => base.selected = Some(value.clone()),
        }
        Ok(())
    }
}
//#endregion 🔺️Diff
