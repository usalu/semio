#[path = "📝️set-dummy-count/🦀️.rs"]
pub mod set_dummy_count;
pub(crate) use set_dummy_count::SetDummyCount;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::Mutations)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = super::DummySnapshot, diff = super::DummyDiff, schema = "plugin.testkit.dummy")]
pub(crate) enum DummyMutation { SetDummyCount(SetDummyCount) }
impl protocol::OpText for DummyMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { Ok(SetDummyCount::parse_op(line)?.into()) } fn print_op(&self) -> String { match self { Self::SetDummyCount(value) => value.print_op() } } }
impl protocol::OpBinary for DummyMutation { fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { match self { Self::SetDummyCount(value) => value.encode_op() } } fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { Ok(SetDummyCount::decode_op(bytes)?.into()) } }
