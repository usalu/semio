//! 🫧️ Transparent publication-transient mutation roster.

#[path = "📝️change-publication-transient/🦀️.rs"]
pub mod change_publication_transient;
pub use change_publication_transient::ChangePublicationTransient;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = super::PublicationTransient, diff = super::PublicationTransientDiff, schema = "plugin.test.publication-transient")]
pub enum PublicationTransientMutation {
    ChangePublicationTransient(ChangePublicationTransient),
}

impl protocol::OpText for PublicationTransientMutation {
    fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
        Ok(<ChangePublicationTransient as protocol::OpText>::parse_op(line)?.into())
    }

    fn print_op(&self) -> String {
        match self {
            Self::ChangePublicationTransient(change) => <ChangePublicationTransient as protocol::OpText>::print_op(change),
        }
    }
}

impl protocol::OpBinary for PublicationTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match self {
            Self::ChangePublicationTransient(change) => <ChangePublicationTransient as protocol::OpBinary>::encode_op(change),
        }
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(<ChangePublicationTransient as protocol::OpBinary>::decode_op(bytes)?.into())
    }
}
