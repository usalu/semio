//! 📦️ Playbook artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::PlaybookSnapshot;
use store::PackError;

/// 📦️ Encodes a `PlaybookSnapshot` to its binary pack form.
pub fn encode(document: &PlaybookSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `PlaybookSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<PlaybookSnapshot, PackError> {
    <PlaybookSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
