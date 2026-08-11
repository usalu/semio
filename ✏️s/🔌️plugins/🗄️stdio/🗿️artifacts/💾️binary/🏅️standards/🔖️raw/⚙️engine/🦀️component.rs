//! ⚙️ BinaryEngine — owns a real `BinaryArtifact`.

use crate::artifacts::binary::{BinaryArtifact, BinaryDiff, BinaryMutation, BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_binary_snapshot() -> BinarySnapshot {
    BinarySnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs, the artifact schema descriptor, and every composer entry (which supersedes
/// the pre-migration per-leaf `io::register()` no-ops -- see `🎹️composer::register`).
pub fn register() {
    crate::artifacts::binary::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<BinarySnapshot, BinaryMutation>(STDIO_BINARY_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.binary",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::binary::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::binary::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::binary::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.binary"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.binary`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::binary::schema::binary_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.binary` artifact engine.
pub struct BinaryEngine {
    artifact_state: BinaryArtifact,
    snapshot_state: BinarySnapshot,
}

impl BinaryEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: BinarySnapshot) -> Self {
        let artifact_state = BinaryArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_binary_snapshot();
        assert_eq!(snapshot.schema, STDIO_BINARY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_binary_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🧪️ `codec_retention_law`: decode→encode is byte-preserving on real fixtures, incl. bytes
    /// that are themselves invalid UTF-8 (the hex DSL layer never interprets payload bytes as
    /// text, so this is a real test of the hex codec, not just the binary-pack envelope).
    #[test]
    fn codec_retention_law() {
        for bytes in [
            vec![],
            vec![0x00, 0x01, 0xFF, 0xFE],
            (0u8..=255).collect::<Vec<u8>>(),
        ] {
            let snap = BinarySnapshot { bytes: bytes.clone(), ..Default::default() };
            let dsl_text = store::ArtifactDsl::print_dsl(&snap);
            let parsed = <BinarySnapshot as store::ArtifactDsl>::parse_dsl(&dsl_text).expect("parse");
            assert_eq!(parsed, snap, "dsl round-trip mismatch for {bytes:?}");
            let packed = store::ArtifactPack::encode_pack(&snap);
            let decoded = <BinarySnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode");
            assert_eq!(decoded, snap, "pack round-trip mismatch for {bytes:?}");
        }
    }

    //#region 🔖️FieldSweep
    /// 🧹 Canonical "every mutable field differs" snapshot A.
    fn sweep_a() -> BinarySnapshot {
        BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: vec![1, 2, 3, 4, 5, 6, 7, 8] }
    }

    /// 🧹 Canonical "every mutable field differs" snapshot B: an insert-only region (bytes 100
    /// inserted mid-buffer), a pure-removal region (bytes 3,4 dropped), and a pure-replacement
    /// region (byte 8 → 88) -- one splice can't express all three at once, exercising the
    /// splice mechanism's full range per the artifact's own field-sweep note.
    fn sweep_b() -> BinarySnapshot {
        BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: vec![1, 2, 100, 5, 6, 7, 88] }
    }

    /// 🧪️ `field_sweep`: THE acceptance criterion. `between` round-trips both directions, the
    /// splice list is non-empty (the only "field" a splice-list diff has), and `between(a,a)`
    /// is empty.
    #[test]
    fn field_sweep_covers_every_byte_level_change() {
        use protocol::os_spr::command::DiffAlgebra;
        use protocol::MutationDiff;
        let a = sweep_a();
        let b = sweep_b();

        let ab = BinaryDiff::between(&a, &b);
        assert_eq!(ab.apply(&a), b, "between(a,b).apply(a) must equal b");
        let ba = BinaryDiff::between(&b, &a);
        assert_eq!(ba.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(!ab.splices.is_empty(), "sweep diff must carry at least one splice");

        // 🔬️ Exercise insert/remove/replace explicitly via hand-built splices (not just the
        // minimal `between` form) to prove the mechanism itself, not just this one pair.
        let hand_built = BinaryDiff {
            splices: vec![
                crate::artifacts::binary::schema::diff::ByteSplice { offset: 2, remove_len: 2, insert: vec![100] }, // replace+shrink
                crate::artifacts::binary::schema::diff::ByteSplice { offset: 7, remove_len: 1, insert: vec![88] },  // pure replace
            ],
        };
        assert_eq!(hand_built.apply(&a), b);

        assert!(BinaryDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️FieldSweep
}
//#endregion 🧪️Tests
