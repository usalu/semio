//! 🧬️ THE carrier law this whole pilot exists to prove (design.md §3, mission step 5):
//! `s.stdio.binary@raw/*`'s native `Binary` `IoPayload` is the raw external file content,
//! byte-for-byte — decode→encode must reproduce arbitrary bytes exactly, and the encoded
//! payload must NOT be a `.semio` pack container (must not start with
//! `store::semio_format::BINARY_MAGIC`, the 8-byte magic `ArtifactPack::encode_pack` used to
//! emit before this fix — see `📸️snapshot/🦀️.rs`).
use crate::BinarySnapshot;
use store::{ArtifactDsl, ArtifactPack};

#[semio_framework_async_macros::async_test]
async fn carrier_native_is_raw() {
    for bytes in [Vec::<u8>::new(), vec![0x00, 0x01, 0xFF], b"hello".to_vec(), (0u8..=255).collect::<Vec<u8>>()] {
        let decoded = BinarySnapshot::decode_pack(&bytes).expect("decode");
        let encoded = decoded.encode_pack();
        assert_eq!(encoded, bytes, "carrier round trip must be byte-identical for {bytes:?}");
        assert!(!encoded.starts_with(&store::semio_format::BINARY_MAGIC), "carrier payload must not be a pack container: {encoded:?}");
    }
    // 🌱 `parse_dsl`/`print_dsl` (the hex-text facet) is NOT law-bound — `CARRIER_BINARY`
    // only governs the `Binary` `IoPayload` variant of this dialect; the DSL hex form stays a
    // legitimate, separate native-text encoding untouched by this law.
    let _ = <BinarySnapshot as ArtifactDsl>::envelope_id();
}
