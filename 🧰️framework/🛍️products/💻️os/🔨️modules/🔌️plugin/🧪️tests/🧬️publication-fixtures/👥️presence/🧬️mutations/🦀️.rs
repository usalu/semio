//! 👥️ Transparent publication-presence mutation roster.

#[path = "📝️change-publication-presence/🦀️.rs"]
pub mod change_publication_presence;
pub use change_publication_presence::ChangePublicationPresence;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, ToValue, FromValue, dsl::Mutations)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = super::PublicationPresence, diff = super::PublicationPresenceDiff, schema = "plugin.test.publication-presence")]
pub enum PublicationPresenceMutation {
    ChangePublicationPresence(ChangePublicationPresence),
}

impl protocol::OpText for PublicationPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, crate::store::TextError> {
        Ok(<ChangePublicationPresence as protocol::OpText>::parse_op(line)?.into())
    }

    fn print_op(&self) -> String {
        match self {
            Self::ChangePublicationPresence(change) => <ChangePublicationPresence as protocol::OpText>::print_op(change),
        }
    }
}

impl protocol::OpBinary for PublicationPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        match self {
            Self::ChangePublicationPresence(change) => <ChangePublicationPresence as protocol::OpBinary>::encode_op(change),
        }
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(<ChangePublicationPresence as protocol::OpBinary>::decode_op(bytes)?.into())
    }
}
