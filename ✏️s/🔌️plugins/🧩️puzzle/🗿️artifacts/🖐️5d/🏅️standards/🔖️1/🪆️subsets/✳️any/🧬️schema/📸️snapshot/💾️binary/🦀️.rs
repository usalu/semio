//! 🎒️ Puzzle 5d artifact — the binary document surface and its laws: `encode`/`decode` over the
//! typed `Puzzle5dSnapshot`, agreeing byte-for-byte with what `🗣️dsl` prints and parses.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::Puzzle5dSnapshot;
use store::PackError;

/// 📦️ Encodes a `Puzzle5dSnapshot` to its binary pack form.
pub fn encode(document: &Puzzle5dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle5dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle5dSnapshot, PackError> {
    <Puzzle5dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
