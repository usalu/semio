//! 📡️ Puzzle3d inference — the normative handcrafted binary protocol for this facet. Same
//! declaration-only shape as the sibling `📝️text` leaf: inference values are only ever computed
//! and (optionally) cached, never decoded from an authored binary document, so there is no
//! `encode`/`decode` pair here — just the protocol spec text every other representation leaf
//! declares for its own facet.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::standards::v1::subsets::any::schema::FillRunCheckpoint;
impl FillRunCheckpoint {
    pub const BYTES: usize = 68;

    pub fn encode(self) -> [u8; Self::BYTES] {
        let mut bytes = [0; Self::BYTES];
        bytes[..8].copy_from_slice(&self.requested.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.placements.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.provisional_ops.to_le_bytes());
        bytes[20..28].copy_from_slice(&self.tested.to_le_bytes());
        bytes[28..36].copy_from_slice(&self.next_key.to_le_bytes());
        bytes[36..].copy_from_slice(&self.inputs);
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        let bytes: &[u8; Self::BYTES] = bytes.try_into().ok()?;
        let u64_at = |at: usize| u64::from_le_bytes(bytes[at..at + 8].try_into().expect("eight bytes"));
        Some(Self { requested: u64_at(0), placements: u64_at(8), provisional_ops: u32::from_le_bytes(bytes[16..20].try_into().expect("four bytes")), tested: u64_at(20), next_key: u64_at(28), inputs: bytes[36..].try_into().expect("thirty-two bytes") })
    }
}
