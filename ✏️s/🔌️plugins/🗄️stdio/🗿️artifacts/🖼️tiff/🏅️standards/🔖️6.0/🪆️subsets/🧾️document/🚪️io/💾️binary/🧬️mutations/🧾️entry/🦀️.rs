//! 🧾️ Registry entry of the binary codecs of TiffMutation, shared by the aggregator and every direct codec leaf.
use crate::schema::mutations::TiffMutation;

/// 📦 Encodes a recognized mutation payload or declines another variant.
pub type TiffMutationPayloadEncoder = fn(&TiffMutation) -> Option<Result<Vec<u8>, protocol::ProtocolError>>;
pub struct Entry {
    pub tag: u8,
    pub encode: TiffMutationPayloadEncoder,
    pub decode: fn(&[u8]) -> Result<TiffMutation, protocol::ProtocolError>,
}
