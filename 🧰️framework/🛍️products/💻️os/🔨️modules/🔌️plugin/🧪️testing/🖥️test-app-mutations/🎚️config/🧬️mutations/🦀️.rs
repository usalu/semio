#[path = "📝️change-test-config/🦀️.rs"]
pub mod change_test_config_selection;
pub use crate::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, semio_framework_dsl_record_derive::DslReceiving, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = store)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot=crate::test_app_mutation_fixture::config::TestConfig,diff=crate::test_app_mutation_fixture::config::TestConfigDiff,schema="plugin.testkit.config", reference = crate::test_app_mutation_fixture::config::TestConfig, reference_diff = crate::test_app_mutation_fixture::config::TestConfigDiff)]
pub enum TestConfigMutation {
    ChangeTestConfigSelection(ChangeTestConfigSelection),
}
impl semio_framework_value::retirement::RetireOwned for TestConfigMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self { Self::ChangeTestConfigSelection(value) => value.retirement() }
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        match self { Self::ChangeTestConfigSelection(value) => value.retirement_birth_bytes() }
    }
    fn controlled_retirement_supported() -> bool { true }
}
impl protocol::OpText for TestConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        Ok(ChangeTestConfigSelection::parse_op(line)?.into())
    }
    fn print_op(&self) -> String {
        match self {
            Self::ChangeTestConfigSelection(value) => value.print_op(),
        }
    }
}
impl protocol::OpBinary for TestConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
semio_framework_os_kernel::original_ordinal_operation_receiving!(TestConfigMutation);
impl store::snapshot_clone_preparation::ConfigApplyMutation<crate::test_app_mutation_fixture::config::TestConfig> for TestConfigMutation {
    fn exchange(self, post: &mut crate::test_app_mutation_fixture::config::TestConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        let Self::ChangeTestConfigSelection(change) = self;
        let previous = std::mem::replace(&mut post.selected, change.selected);
        Ok(Self::ChangeTestConfigSelection(ChangeTestConfigSelection { selected: previous }))
    }
}
