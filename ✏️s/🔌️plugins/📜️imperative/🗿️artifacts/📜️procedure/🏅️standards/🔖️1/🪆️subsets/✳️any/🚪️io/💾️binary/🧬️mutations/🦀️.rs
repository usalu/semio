//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::ProcedureDiff;
use crate::ProcedureSnapshot;

/// 💾️ No parent operation record exists.
impl protocol::OpBinary for ProcedureMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "procedure-mutation", offset: 0, detail: "a procedure has no parent-lane mutation; content edits are child-lane leaves".into() })
    }
}
}
pub use mutations_codec::*;
