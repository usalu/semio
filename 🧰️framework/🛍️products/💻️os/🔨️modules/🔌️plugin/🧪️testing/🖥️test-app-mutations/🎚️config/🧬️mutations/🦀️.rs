#[path = "📝️change-test-config/🦀️.rs"]
pub mod change_test_config_selection;
pub(crate) use change_test_config_selection::ChangeTestConfigSelection;
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::Mutations, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = store)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot=super::TestConfig,diff=super::TestConfigDiff,schema="plugin.testkit.config")]
pub(crate) enum TestConfigMutation {
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
        match self {
            Self::ChangeTestConfigSelection(value) => value.encode_op(),
        }
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(ChangeTestConfigSelection::decode_op(bytes)?.into())
    }
}
