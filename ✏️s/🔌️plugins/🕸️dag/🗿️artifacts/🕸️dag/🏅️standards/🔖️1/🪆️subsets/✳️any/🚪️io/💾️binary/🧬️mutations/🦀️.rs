//! 🚪️ Artifact representation codecs.

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::DagDiff;
use crate::DagSnapshot;

/// 💾️ No parent operation record exists.
impl protocol::OpBinary for DagMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Err(protocol::ProtocolError::Malformed { what: "dag-mutation", offset: 0, detail: "a DAG has no parent-lane mutation; content edits are child-lane leaves".into() })
    }
}
}
pub use mutations_codec::*;
