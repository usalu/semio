//! ⚙️ DwgEngine — owns a real `DwgArtifact`.

use crate::artifacts::dwg::DwgArtifact;
use crate::artifacts::dwg::DwgSnapshot as CanonicalDwgSnapshot;
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::{
    diff::DwgDiff, mutations::DwgMutation, snapshot::DwgSnapshot,
};

/// 🏷️ Document schema / DSL envelope id for ac1018.
pub const STDIO_DWG_AC1018_DOCUMENT_SCHEMA: &str = "stdio.dwg.ac1018";

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_dwg_snapshot() -> DwgSnapshot {
    DwgSnapshot::default()
}

/// 📄️ The demo `stdio.dwg` (ac1018) document — decodes the real, committed 22-byte AC1018 stub
/// (`📚️examples/🎬️demo/🖼️assets/example.dwg`, this standard's OWN dedicated fixture — NOT the
/// artifact-level `📚️examples/🎬️demo` demo, which is ac1024-shaped, the canonical standard, per
/// S-6/Decision #5) via ac1018's own real `decode_dwg`. The single source of truth for
/// `🏅️standards/🔖️ac1018/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio`
/// (both are literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `conformance_laws::fixture_honesty_law` below).
pub fn demo_dwg_snapshot() -> crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot {
    let stub = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::decode_dwg(stub).expect("decode ac1018 demo stub")
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::dwg::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    register_schema_specs();
    store::register_document_codec(store::ArtifactCodec::of::<DwgSnapshot, DwgMutation>(STDIO_DWG_AC1018_DOCUMENT_SCHEMA));
}

/// 📌️ P2-P3-style 5-role `LanguageSpec` registration (Document/Ops/Diff/Pack/Spr) for ac1018's
/// OWN real facet grammars/protocols — fully qualified to
/// `standards::v_ac1018::subsets::any::schema`, deliberately NOT the top-level
/// `crate::artifacts::dwg::schema` shim (aliased to the CANONICAL ac1024 standard, per S-6/
/// Decision #5 — the exact pitfall `🔺️diff/🦀️component.rs`'s own module doc warns about:
/// "NOT `crate::artifacts::dwg::DwgSnapshot`"). `diff`'s `protocol` slot stays `None` matching
/// every other pilot's own 5-role exemplar exactly (the role scheme has no dedicated "diff
/// binary" role even though `🔺️diff/💾️binary/📡️component.protocol.semio` is a real,
/// conformance-tested file — its binary form is exercised directly by `protocol_walk_law` below).
pub fn register_pilot_languages() {
    use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema as ac1018_schema;
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dwg.ac1018",
        extension: Some("bin"),
        role: dsl::LanguageRole::Document,
        grammar: Some(ac1018_schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(ac1018_schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(ac1018_schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(ac1018_schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dwg.ac1018"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dwg.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(ac1018_schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(ac1018_schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(ac1018_schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(ac1018_schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dwg.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dwg.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(ac1018_schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(ac1018_schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.dwg.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dwg.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(ac1018_schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(ac1018_schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dwg.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dwg.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(ac1018_schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(ac1018_schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dwg.spr"),
    });
}

/// 📇️ ac1018's own `DwgSnapshot`/`DwgDiff` both derive real `dsl::DslRecord`/`dsl::DslDiff`
/// (`../🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`,
/// `.../🔺️diff/🦀️component.rs`) — genuinely callable, same 2-call shape as
/// `stdio.binary`/`stdio.txt`'s own `register_schema_specs`. Registered under a
/// standard-qualified id (`"stdio.dwg.ac1018"`, not the bare `"stdio.dwg"` the LanguageSpec
/// above uses) so this never collides with ac1024's own identically-shaped call under the plain
/// `"stdio.dwg"` id — both standards derive real specs from DIFFERENT `RecordSpec`-shaped types
/// (ac1018's own 6-field `DwgSnapshot` vs. ac1024's richer 8-field one), so they cannot honestly
/// share one schema id. Per-mutation-variant specs are NOT registered — no single canonical id
/// exists for a `Mutation` enum's N independently-shaped variants (same documented scope boundary
/// every other pilot's own `register_schema_specs` observes).
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema as ac1018_schema;
    dsl::registry::register_schema_spec("stdio.dwg.ac1018", ac1018_schema::snapshot::DwgSnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.dwg.ac1018#diff", ac1018_schema::diff::DwgDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}

/// 📌️ Registers schema leaves for `s.stdio.dwg`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::dwg::schema::dwg_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.dwg` artifact engine.
pub struct DwgEngine {
    artifact_state: DwgArtifact,
    snapshot_state: CanonicalDwgSnapshot,
}

impl DwgEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: CanonicalDwgSnapshot) -> Self {
        let artifact_state = DwgArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_dwg_snapshot();
        assert_eq!(snapshot.schema, STDIO_DWG_AC1018_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let stub = b"AC1018\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let snap = crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::decode_dwg(stub).expect("decode stub");
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DwgSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.version, "AC1018");
        assert_eq!(parsed.bytes, stub);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DwgSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION FG2: per-standard
    /// conformance laws for ac1018's OWN real facets — grammar/protocol parseability, `Recognizer`
    /// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Lives
    /// here (the engine's own test region), not any framework file — mirrors
    /// `stdio.binary`/`stdio.txt`'s own `conformance_laws` module shape exactly, fully qualified
    /// to ac1018's own standard (never the top-level `crate::artifacts::dwg` shim, aliased to
    /// ac1024).
    mod conformance_laws {
        use super::*;
        use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect.
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [
                ("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO),
                ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO),
            ] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [
                ("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO),
            ] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the ac1018 demo snapshot AND the empty-bytes case (`hex` macro's zero-width match).
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_dwg_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `mutations::demo_mutation_cases()` variant.
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
        /// for every `diff::demo_diff_cases()`, incl. the empty (all-`None`) diff.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets —
        /// snapshot pack (`encode_pack`, envelope-unwrapped first), every demo mutation's
        /// `encode_op`, and every demo diff's `encode_diff` — asserting `consumed == bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_dwg_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in mutations::demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in diff::demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        #[test]
        #[ignore]
        fn zzz_generate_p2p1_fixtures() {
            let demo = demo_dwg_snapshot();
            let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/📚️examples/🎬️demo/🖼️assets");
            std::fs::write(dir.join("🗣️example.dsl.semio"), store::ArtifactDsl::print_dsl(&demo)).unwrap();
            std::fs::write(dir.join("🎒️example.pack.semio"), store::ArtifactPack::encode_pack(&demo)).unwrap();
        }

        /// ✅️ `fixture_honesty_law`: the shipped ac1018-own `.dsl.semio`/`.pack.semio` fixtures
        /// are GENUINE `print_dsl`/`encode_pack` output of `demo_dwg_snapshot()`.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_dwg_snapshot();

            let parsed = <snapshot::DwgSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_dwg_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_dwg_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <snapshot::DwgSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_dwg_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_dwg_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
