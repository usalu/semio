//! 📡️ `trinity.rewrite.rule` artifact — binary command protocol surface + laws (constitutional:
//! spr, renamed from the old `📡️protocol` — no `📡️protocol` segment survives). `RewriteRuleOperation`
//! already derives `dsl::DslOps` directly (see `🔧️op`), so this file is a pure wrapper, unlike
//! `jack`'s `📡️spr` which needs a full DSL mirror.

use crate::artifacts::rewrite::op::RewriteRuleOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `RewriteRuleOperation` to its binary command form.
pub fn encode_op(operation: &RewriteRuleOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RewriteRuleOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RewriteRuleOperation, protocol::ProtocolError> {
    RewriteRuleOperation::decode_op(bytes)
}
