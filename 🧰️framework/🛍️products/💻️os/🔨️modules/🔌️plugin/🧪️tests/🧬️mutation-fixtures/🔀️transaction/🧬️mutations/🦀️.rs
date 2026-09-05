#[path = "📝️set-transaction-count/🦀️.rs"] pub mod set_transaction_count;
#[path = "⏩️set-transaction-count-without-preflight/🦀️.rs"] pub mod set_transaction_count_without_preflight;
#[path = "📣️set-transaction-count-and-notify/🦀️.rs"] pub mod set_transaction_count_and_notify;
pub(crate) use set_transaction_count::SetTransactionCount;
pub(crate) use set_transaction_count_without_preflight::SetTransactionCountWithoutPreflight;
pub(crate) use set_transaction_count_and_notify::SetTransactionCountAndNotify;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::Mutations)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = super::TxnSnapshot, diff = super::TxnDiff, schema = "plugin.testkit.transaction")]
pub(crate) enum TxnMutation { SetTransactionCount(SetTransactionCount), SetTransactionCountWithoutPreflight(SetTransactionCountWithoutPreflight), SetTransactionCountAndNotify(SetTransactionCountAndNotify) }
impl protocol::OpText for TxnMutation { fn parse_op(line: &str) -> Result<Self, crate::store::TextError> { if line.starts_with("set-transaction-count-and-notify ") { Ok(SetTransactionCountAndNotify::parse_op(line)?.into()) } else if line.starts_with("set-transaction-count-without-preflight ") { Ok(SetTransactionCountWithoutPreflight::parse_op(line)?.into()) } else { Ok(SetTransactionCount::parse_op(line)?.into()) } } fn print_op(&self) -> String { match self { Self::SetTransactionCount(value) => value.print_op(), Self::SetTransactionCountWithoutPreflight(value) => value.print_op(), Self::SetTransactionCountAndNotify(value) => value.print_op() } } }
impl protocol::OpBinary for TxnMutation { fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { match self { Self::SetTransactionCount(value) => value.encode_op(), Self::SetTransactionCountWithoutPreflight(value) => value.encode_op(), Self::SetTransactionCountAndNotify(value) => value.encode_op() } } fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { match bytes.first() { Some(0x62) => Ok(SetTransactionCount::decode_op(bytes)?.into()), Some(0x63) => Ok(SetTransactionCountWithoutPreflight::decode_op(bytes)?.into()), Some(0x64) => Ok(SetTransactionCountAndNotify::decode_op(bytes)?.into()), _ => Err(protocol::ProtocolError::Malformed { what: "transaction-counter", offset: 0, detail: "unknown transaction counter tag".into() }) } } }
