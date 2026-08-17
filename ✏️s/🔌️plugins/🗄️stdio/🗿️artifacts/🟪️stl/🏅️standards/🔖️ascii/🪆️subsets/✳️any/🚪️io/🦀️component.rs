//! 🚪️ IO stdio.stl (ascii/✳️any) — registration now flows through the `s.stdio.stl`
//! `ArtifactDeclaration` (`crate::artifacts::stl::declaration`), not per-leaf register().
use crate::artifacts::stl::schema::snapshot::StlTriangle;
use crate::artifacts::stl::{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA};

//#region 🔖️Codec
//#region 🔖️Ascii
/// 📥 Parses `solid <name>`/`facet normal`/`outer loop`/`vertex`×3/`endloop`/`endfacet`/
/// `endsolid` ASCII STL. The facet normal is persisted exactly as written (real STL files often
/// carry a degenerate `facet normal 0 0 0` and rely on downstream tooling to recompute it — this
/// codec doesn't silently rewrite that on decode, matching the recipe's "nothing fabricated"
/// rule; `<StlTriangle as PartialEq>` sees whatever the file actually said).
pub fn decode_stl_ascii(text: &str) -> Result<StlSnapshot, String> {
    if !text.trim_start().starts_with("solid") {
        return Err("stl ascii: missing 'solid' header".into());
    }
    let mut lines = text.lines();
    let header = lines.next().unwrap_or("");
    let solid_name = header.trim().strip_prefix("solid").unwrap_or("").trim().to_string();

    let mut triangles = Vec::new();
    let mut normal: [f64; 3] = [0.0; 3];
    let mut verts: [Option<[f64; 3]>; 3] = [None, None, None];
    let mut slot = 0usize;
    let mut in_loop = false;
    for line in lines {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("facet normal") {
            let coords: Vec<f64> = rest.split_whitespace().map(|s| s.parse::<f64>().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?;
            if coords.len() < 3 {
                return Err("stl ascii: facet normal needs 3 coords".into());
            }
            normal = [coords[0], coords[1], coords[2]];
        } else if line.starts_with("outer loop") {
            in_loop = true;
            slot = 0;
            verts = [None, None, None];
        } else if line.starts_with("endloop") {
            if slot != 3 {
                return Err(format!("stl ascii: facet had {slot} vertices, expected 3"));
            }
            let vertices = [verts[0].unwrap(), verts[1].unwrap(), verts[2].unwrap()];
            triangles.push(StlTriangle { normal, vertices });
            in_loop = false;
        } else if in_loop && line.starts_with("vertex") {
            let coords: Vec<f64> = line.trim_start_matches("vertex").split_whitespace().map(|s| s.parse::<f64>().map_err(|e| e.to_string())).collect::<Result<Vec<_>, _>>()?;
            if coords.len() < 3 {
                return Err("stl ascii: vertex needs 3 coords".into());
            }
            if slot >= 3 {
                return Err("stl ascii: more than 3 vertices in one facet".into());
            }
            verts[slot] = Some([coords[0], coords[1], coords[2]]);
            slot += 1;
        }
    }
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name, triangles })
}

/// 📤 Writes real ASCII STL, round-tripping each facet's persisted normal exactly (never
/// recomputed from vertex winding — see `decode_stl_ascii`'s doc comment).
pub fn encode_stl_ascii(snap: &StlSnapshot) -> String {
    let mut out = format!("solid {}\n", snap.solid_name);
    for f in &snap.triangles {
        let [nx, ny, nz] = f.normal;
        out.push_str(&format!("  facet normal {nx} {ny} {nz}\n"));
        out.push_str("    outer loop\n");
        for [x, y, z] in f.vertices {
            out.push_str(&format!("      vertex {x} {y} {z}\n"));
        }
        out.push_str("    endloop\n  endfacet\n");
    }
    out.push_str(&format!("endsolid {}\n", snap.solid_name));
    out
}
//#endregion 🔖️Ascii

//#region 🔖️Binary
/// 📥 Parses binary STL: 80-byte header (used as `solid_name`, trimmed of trailing NULs/
/// whitespace — real-world binary STL writers commonly stash a comment/name there even though
/// the spec calls it opaque) + u32 triangle count + N × (12-byte normal [f32×3] + 3×12-byte
/// vertices [f32×3] + 2-byte attribute-byte-count [dropped: no attribute-byte-count usage is
/// specified by the base format]). Normals/vertices widen `f32` -> `f64` (see `StlTriangle`'s
/// doc comment on the ASCII/binary precision-normalization tradeoff).
pub fn decode_stl_binary(bytes: &[u8]) -> Result<StlSnapshot, String> {
    if bytes.len() < 84 {
        return Err("stl binary: header too short".into());
    }
    let solid_name = String::from_utf8_lossy(&bytes[0..80]).trim_end_matches('\0').trim().to_string();
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let mut triangles = Vec::with_capacity(count);
    let mut off = 84usize;
    for _ in 0..count {
        if off + 50 > bytes.len() {
            return Err("stl binary: truncated facet record".into());
        }
        let read_vec3 =
            |b: &[u8], at: usize| -> [f64; 3] { [f32::from_le_bytes(b[at..at + 4].try_into().unwrap()) as f64, f32::from_le_bytes(b[at + 4..at + 8].try_into().unwrap()) as f64, f32::from_le_bytes(b[at + 8..at + 12].try_into().unwrap()) as f64] };
        let normal = read_vec3(bytes, off);
        off += 12;
        let mut vertices = [[0.0; 3]; 3];
        for v in vertices.iter_mut() {
            *v = read_vec3(bytes, off);
            off += 12;
        }
        off += 2; // attribute byte count
        triangles.push(StlTriangle { normal, vertices });
    }
    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name, triangles })
}

/// 📤 Writes real binary STL: 80-byte header (`solid_name`, truncated to 80 bytes / zero-padded)
/// + u32 triangle count at offset 80..84 (the count belongs INSIDE the 84-byte header, not
/// appended after it — an 80-byte header vec here, not 84, is what makes the count land at the
/// right offset). Each facet's persisted `f64` normal/vertices narrow to `f32` (binary STL's
/// spec-mandated precision — a documented, lossy normalization, not fabrication).
pub fn encode_stl_binary(snap: &StlSnapshot) -> Vec<u8> {
    let mut out = vec![0u8; 80];
    let name_bytes = snap.solid_name.as_bytes();
    let n = name_bytes.len().min(80);
    out[..n].copy_from_slice(&name_bytes[..n]);
    out.extend_from_slice(&(snap.triangles.len() as u32).to_le_bytes());
    for f in &snap.triangles {
        for v in f.normal {
            out.extend_from_slice(&(v as f32).to_le_bytes());
        }
        for vertex in f.vertices {
            for v in vertex {
                out.extend_from_slice(&(v as f32).to_le_bytes());
            }
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}
//#endregion 🔖️Binary

//#region 🔖️AutoDetect
/// 🔍 Dispatches on the `solid` ASCII magic; anything else is treated as binary STL.
pub fn decode_stl_auto(bytes: &[u8]) -> Result<StlSnapshot, String> {
    if bytes.len() >= 5 && &bytes[0..5] == b"solid" {
        // A binary STL's 80-byte header can coincidentally start with "solid" too;
        // disambiguate by checking whether the binary triangle-count framing actually
        // matches the file length before trusting the ASCII path.
        if bytes.len() >= 84 {
            let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
            let expected_binary_len = 84 + count * 50;
            if expected_binary_len == bytes.len() {
                return decode_stl_binary(bytes);
            }
        }
        decode_stl_ascii(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)
    } else {
        decode_stl_binary(bytes)
    }
}
//#endregion 🔖️AutoDetect
//#endregion 🔖️Codec

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::stl::standards::v_ascii::subsets::any::schema::StlAnalyzer;
    use crate::artifacts::stl::StlSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

    pub struct StlComposerComposition;

    impl ArtifactComposition for StlComposerComposition {
        type Snapshot = StlSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_TXT, DEP_BINARY]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_TXT || s.dialect == DEP_BINARY)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "StlComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = StlAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "StlComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = crate::artifacts::stl::engine::empty_stl_snapshot();
        assert_eq!(snapshot.schema, STDIO_STL_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = crate::artifacts::stl::engine::empty_stl_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <StlSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <StlSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    /// 🔺 A real (non-degenerate) 4-triangle tetrahedron — enough structure to catch an
    /// off-by-one in facet/vertex slot tracking that a single-triangle fixture would miss. Each
    /// facet gets a distinct, non-zero normal to exercise the persisted-normal round trip.
    fn tetrahedron() -> StlSnapshot {
        let corners = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let faces: [(usize, usize, usize, [f64; 3]); 4] = [(0, 1, 2, [0.0, 0.0, -1.0]), (0, 1, 3, [0.0, -1.0, 0.0]), (1, 2, 3, [1.0, 1.0, 1.0]), (0, 2, 3, [-1.0, 0.0, 0.0])];
        let triangles = faces.iter().map(|&(a, b, c, normal)| StlTriangle { normal, vertices: [corners[a], corners[b], corners[c]] }).collect();
        StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), solid_name: "tetrahedron".into(), triangles }
    }

    #[test]
    fn ascii_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let text = encode_stl_ascii(&snap);
        assert!(text.starts_with("solid tetrahedron"));
        assert!(text.trim_end().ends_with("endsolid tetrahedron"));
        assert_eq!(text.matches("facet normal").count(), 4);
        let decoded = decode_stl_ascii(&text).expect("decode");
        assert_eq!(decoded, snap);
    }

    #[test]
    fn binary_tetrahedron_round_trip() {
        let snap = tetrahedron();
        let bytes = encode_stl_binary(&snap);
        assert_eq!(bytes.len(), 84 + 4 * 50);
        let decoded = decode_stl_binary(&bytes).expect("decode");
        assert_eq!(decoded.solid_name, snap.solid_name);
        assert_eq!(decoded.triangles.len(), snap.triangles.len());
        // f64 -> f32 -> f64 narrowing is lossy by spec; compare within tolerance.
        for (a, b) in decoded.triangles.iter().zip(snap.triangles.iter()) {
            for i in 0..3 {
                assert!((a.normal[i] - b.normal[i]).abs() < 1e-6);
                for j in 0..3 {
                    assert!((a.vertices[i][j] - b.vertices[i][j]).abs() < 1e-6);
                }
            }
        }
    }

    #[test]
    fn auto_detect_dispatches_ascii_vs_binary() {
        let snap = tetrahedron();
        let ascii_bytes = encode_stl_ascii(&snap).into_bytes();
        let binary_bytes = encode_stl_binary(&snap);
        assert_eq!(decode_stl_auto(&ascii_bytes).expect("ascii").triangles.len(), 4);
        assert_eq!(decode_stl_auto(&binary_bytes).expect("binary").triangles.len(), 4);
    }

    #[test]
    fn ascii_facet_normal_is_persisted_not_recomputed() {
        // A real-world "lazy writer" pattern: degenerate 0 0 0 facet normals that a naive
        // recompute-on-encode codec would silently overwrite. This codec must round-trip them
        // exactly as written.
        let text = "solid degenerate\n  facet normal 0 0 0\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid degenerate\n";
        let decoded = decode_stl_ascii(text).expect("decode");
        assert_eq!(decoded.triangles[0].normal, [0.0, 0.0, 0.0]);
        let reencoded = encode_stl_ascii(&decoded);
        let redecoded = decode_stl_ascii(&reencoded).expect("re-decode");
        assert_eq!(redecoded.triangles[0].normal, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn ascii_solid_name_round_trips_including_empty() {
        let text = "solid\n  facet normal 0 0 1\n    outer loop\n      vertex 0 0 0\n      vertex 1 0 0\n      vertex 0 1 0\n    endloop\n  endfacet\nendsolid\n";
        let decoded = decode_stl_ascii(text).expect("decode");
        assert_eq!(decoded.solid_name, "");
        let reencoded = encode_stl_ascii(&decoded);
        assert!(reencoded.starts_with("solid \n") || reencoded.starts_with("solid\n"));
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ FG1: per-artifact conformance laws — grammar/protocol parseability, `Recognizer` against
    /// real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Dissolved
    /// out of `⚙️engine`'s own test region (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
    mod conformance_laws {
        use super::*;
        use crate::artifacts::stl::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
        /// `walk_protocol` laws below (a parse failure here fails fast with a clearer message).
        #[test]
        fn committed_facet_files_parse() {
            for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
            }
            for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
            }
        }

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output
        /// for the demo snapshot — same preamble-stripped body reconstruction
        /// `m5_handcrafted_grammar_conformance`'s own `dsl_body_from_fixture` uses, so this is a
        /// direct proof this artifact will pass that harness once graduated, not merely an
        /// analogue.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&crate::artifacts::stl::engine::demo_stl_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).unwrap_or(false), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `StlMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// for every representative `StlDiff` (`diff::demo_diff_cases()`), incl. the empty diff and
        /// a full removed+modified+added triple.
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
        /// bytes.len()`. Mutations/diff `encode_op`/`encode_diff` are the raw hand-rolled text
        /// bytes verbatim (no SEMIO envelope of their own — see those protocol files' own doc
        /// comments), so they're walked directly, unlike the snapshot pack facet.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&crate::artifacts::stl::engine::demo_stl_snapshot());
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
        /// `print_dsl`/`encode_pack` output of `demo_stl_snapshot()` — `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin — so the
        /// fixtures can never silently drift back to a fake again.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = crate::artifacts::stl::engine::demo_stl_snapshot();

            let parsed = <StlSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_stl_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_stl_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <StlSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_stl_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_stl_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests

//#region 🚪️DerivedIoRegistry
/// 🚪️ Dissolved out of `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub mod io_registry {
    use crate::artifacts::stl::standards::v_ascii::subsets::any::schema::StlComposer as StlRawAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<StlRawAnyComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
