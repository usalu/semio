//! 📦️ `trinity.graph` artifact — binary document surface + laws (constitutional: pack).
//!
//! 📌️ The `ArtifactPack` impl itself lives in `🗣️dsl/🦀️.rs`, next to the private
//! `JackSnapshotDsl` mirror it delegates through (same reason the DSL impl lives there too) — this
//! file only holds the public encode/decode entry points, matching the old bundle crate's shape.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::JackSnapshot;
use store::{ArtifactPack, PackError};

/// 📦️ Encodes a `JackSnapshot` to its binary pack form.
pub fn encode(document: &JackSnapshot) -> Vec<u8> {
    ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `JackSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<JackSnapshot, PackError> {
    <JackSnapshot as ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
