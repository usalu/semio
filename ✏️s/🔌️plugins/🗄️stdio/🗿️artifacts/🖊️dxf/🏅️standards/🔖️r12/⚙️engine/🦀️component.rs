//! ⚙️ DxfEngine — owns a real `DxfArtifact`.

use crate::artifacts::dxf::{DxfArtifact, DxfDiff, DxfMutation, DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_dxf_snapshot() -> DxfSnapshot {
    DxfSnapshot::default()
}

/// 🧬️ Genuinely 2-level-nested (a `BLOCK` with a nested entity), every-section demo snapshot —
/// the single source of truth for `fixture_honesty_law`'s shipped `🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` fixtures AND `grammar_conformance_law`/`protocol_walk_law` below, same
/// role `demo_json_snapshot()` plays for `stdio.json` (also reused by `codec_retention_law`'s own
/// `snap1` literal, which predates this helper and stays inline there for that test's own
/// generation-2-fixed-point narrative — not duplicated here to avoid two competing "canonical
/// demo" literals).
pub fn demo_dxf_snapshot() -> DxfSnapshot {
    use crate::artifacts::dxf::schema::snapshot::{
        DxfBlock, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue,
    };
    DxfSnapshot {
        schema: STDIO_DXF_DOCUMENT_SCHEMA.into(),
        header_vars: vec![
            DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] },
            DxfHeaderVar { name: "$INSBASE".into(), group_code: 10, value: DxfValue::Point { value: [1.0, 2.0, 3.0] }, extra_group_codes: vec![] },
        ],
        tables: DxfTables {
            layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }],
            styles: vec![DxfStyle { name: "STANDARD".into(), flags: 0, font_name: "txt".into(), unknown_group_codes: vec![] }],
            linetypes: vec![DxfLinetype { name: "CONTINUOUS".into(), flags: 0, description: "Solid".into(), unknown_group_codes: vec![] }],
        },
        other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
        blocks: vec![DxfBlock {
            name: "MYBLOCK".into(),
            base_point: [0.0, 0.0, 0.0],
            entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }],
            unknown_group_codes: vec![],
        }],
        entities: vec![
            DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Circle { center: [1.0, 1.0, 0.0], radius: 2.0, layer: "0".into(), unknown_group_codes: vec![] },
            DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] },
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::dxf::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<DxfSnapshot, DxfMutation>(STDIO_DXF_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) — the full 5-role
/// `LanguageSpec` scheme (Document/Ops/Diff, `Pack`/`Spr` roles reuse the Document facet's own
/// grammar/protocol per the recipe's exemplar, `stdio.json`'s own `register_pilot_languages`).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dxf",
        extension: Some("dxf"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::dxf::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dxf::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dxf"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dxf.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::dxf::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dxf::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dxf::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dxf::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dxf.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dxf.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::dxf::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dxf::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        // 🧭️ The 5-role scheme has no dedicated "diff binary" role even when a real diff
        // protocol file exists (`../🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/📡️component.protocol.semio`) —
        // `stdio.json`'s own `stdio.json.diff` registration leaves `protocol: None` for the same
        // reason, per the recipe's own checklist.
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("stdio.dxf.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dxf.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dxf.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dxf.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::dxf::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dxf::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dxf.spr"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.dxf`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::dxf::schema::dxf_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.dxf` artifact engine.
pub struct DxfEngine {
    artifact_state: DxfArtifact,
    snapshot_state: DxfSnapshot,
}

impl DxfEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: DxfSnapshot) -> Self {
        let artifact_state = DxfArtifact::from_snapshot(snapshot.clone());
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
        let snapshot = empty_dxf_snapshot();
        assert_eq!(snapshot.schema, STDIO_DXF_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_dxf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DxfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode retains every field across every section (header vars incl. a point-
    /// component var, all three typed table kinds, an unmodeled table kind, a block with a
    /// nested entity, and every typed entity kind plus one raw-retained unmodeled kind) —
    /// documented NORMAL FORM (see `📸️snapshot` module docs): from the SECOND generation onward
    /// decode/encode is a true fixed point (source float/whitespace formatting isn't preserved,
    /// only semantic content).
    #[test]
    fn codec_retention_law() {
        use crate::artifacts::dxf::schema::snapshot::{
            parse_dxf_document, print_dxf_document, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype,
            DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue,
        };
        let snap1 = DxfSnapshot {
            schema: STDIO_DXF_DOCUMENT_SCHEMA.into(),
            header_vars: vec![
                DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] },
                DxfHeaderVar { name: "$INSBASE".into(), group_code: 10, value: DxfValue::Point { value: [1.0, 2.0, 3.0] }, extra_group_codes: vec![] },
            ],
            tables: DxfTables {
                layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }],
                styles: vec![DxfStyle { name: "STANDARD".into(), flags: 0, font_name: "txt".into(), unknown_group_codes: vec![] }],
                linetypes: vec![DxfLinetype { name: "CONTINUOUS".into(), flags: 0, description: "Solid".into(), unknown_group_codes: vec![] }],
            },
            other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
            blocks: vec![crate::artifacts::dxf::schema::snapshot::DxfBlock {
                name: "MYBLOCK".into(),
                base_point: [0.0, 0.0, 0.0],
                entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }],
                unknown_group_codes: vec![],
            }],
            entities: vec![
                DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Circle { center: [1.0, 1.0, 0.0], radius: 2.0, layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] },
            ],
        };
        let text2 = print_dxf_document(&snap1);
        let snap2 = parse_dxf_document(&text2).expect("re-parse");
        assert_eq!(snap1, snap2, "decode(encode(snap)) must be a fixed point");

        let text3 = print_dxf_document(&snap2);
        assert_eq!(text2, text3, "from generation 2 onward, print(parse(text)) must be a true text fixed point too");
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG1: per-artifact conformance laws (recipe §4's deliverable checklist item 6) —
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives here (the engine's own test region), not
    /// any framework file — same shape every pilot's `conformance_laws` module uses
    /// (`🔣️json/…/⚙️engine/🦀️component.rs` is the copy-pasteable exemplar this module mirrors).
    mod conformance_laws {
        use super::*;
        use crate::artifacts::dxf::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
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
        /// for the demo (2-level: HEADER incl. a point-component var, all 3 typed table kinds, a
        /// raw-retained unmodeled table, a block with a nested entity, and every typed entity kind
        /// plus one raw-retained unmodeled kind) snapshot — same preamble-stripped body
        /// reconstruction `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_dxf_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `DxfMutation` variant (`mutations::demo_mutation_cases()`), not just
        /// one trivial case — incl. `SetSnapshot`'s whole-snapshot payload and every typed entity
        /// kind (LINE/CIRCLE/ARC/POLYLINE/TEXT/SOLID/INSERT/Other).
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in mutations::demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff`
        /// output for every representative `DxfDiff` (`diff::demo_diff_cases()`), incl. the
        /// empty-line diff, a single-collection sparse diff, and the rich multi-collection case
        /// (name-keyed + index-keyed triples, `Replace` AND non-`Replace` entity diffs, a nested
        /// block-level `entities` sub-diff).
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
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff` — asserting `consumed ==
        /// bytes.len()`.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_dxf_snapshot());
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

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_dxf_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../../../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_dxf_snapshot();

            let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_dxf_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_dxf_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <DxfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_dxf_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_dxf_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
