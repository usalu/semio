//! 🚪️ IO stdio.pdf (1.4/✳️any) — minimal PDF 1.4 with a FlateDecode content stream. 🦑 Codec +
//! `register_schema_specs` dissolved out of the former `⚙️engine` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); registration otherwise flows through
//! `crate::artifacts::pdf::declaration_1_4()` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
//!
//! 🔀️ S-6 twin: `crate::artifacts::pdf::schema` shims to 1.7 (canonical) -- 1.4's own codec
//! uses its own standard-local schema path directly rather than the shared root re-export.

use crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA;
use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{diff::PdfDiff, mutations::PdfMutation, snapshot::{PageDoc, PdfSnapshot}};

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use semio_framework_plugin::{ArtifactComposition, Dialect, StandardId, SubsetId, Composition, ComposeError, ComposeSource, AnalyzeSource};
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::PdfSnapshot;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::PdfAnalyzer;
    use semio_framework_plugin::ArtifactAnalyzer as _;

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };


    pub struct PdfComposerComposition;

    impl ArtifactComposition for PdfComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_BINARY || s.dialect == DEP_DEFLATE)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "PdfComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PdfAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError {
                message: "PdfComposerComposition: analysis produced no snapshot".into(),
                diagnostics: analysis.diagnostics.clone(),
            })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🔖️Codec
pub fn encode_pdf(snap: &PdfSnapshot) -> Result<Vec<u8>, String> {
    let page = &snap.page;
    let w = page.width.max(1.0);
    let h = page.height.max(1.0);
    let stream = format!("BT /F1 12 Tf 72 {} Td ({}) Tj ET", h - 72.0, escape_pdf(&page.text));
    let compressed = crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::zlib_compress(stream.as_bytes())?;
    let mut body: Vec<u8> = Vec::new();
    body.extend_from_slice(b"%PDF-1.4\n");
    let o1 = body.len();
    body.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");
    let o2 = body.len();
    body.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");
    let o3 = body.len();
    body.extend_from_slice(format!("3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {w} {h}] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n").as_bytes());
    let o4 = body.len();
    body.extend_from_slice(format!("4 0 obj\n<< /Length {} /Filter /FlateDecode >>\nstream\n", compressed.len()).as_bytes());
    body.extend_from_slice(&compressed);
    body.extend_from_slice(b"\nendstream\nendobj\n");
    let o5 = body.len();
    body.extend_from_slice(b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n");
    let xref = body.len();
    body.extend_from_slice(b"xref\n0 6\n0000000000 65535 f \n");
    for off in [o1, o2, o3, o4, o5] {
        body.extend_from_slice(format!("{:010} 00000 n \n", off).as_bytes());
    }
    body.extend_from_slice(format!("trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n").as_bytes());
    Ok(body)
}

fn escape_pdf(s: &str) -> String {
    s.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)")
}

/// 🔍️ Byte-level (never char-boundary-assuming) subslice search -- `decode_pdf` must locate the
/// `stream`/`endstream` markers in the RAW bytes, not in a lossy UTF-8 string conversion of them
/// (the deflate-compressed stream payload is essentially never valid UTF-8, so slicing a
/// `String::from_utf8_lossy` copy corrupts the very bytes being extracted -- `codec_retention_law`
/// caught this as a real, pre-existing decode bug).
fn find_subslice(data: &[u8], needle: &[u8]) -> Option<usize> {
    data.windows(needle.len()).position(|w| w == needle)
}

pub fn decode_pdf(data: &[u8]) -> Result<PdfSnapshot, String> {
    if !data.starts_with(b"%PDF") { return Err("not pdf".into()); }
    let w = 612.0f64;
    let h = 792.0f64;
    let mut content = String::new();
    if let Some(i) = find_subslice(data, b"stream") {
        let rest = &data[i + 6..];
        if let Some(j) = find_subslice(rest, b"endstream") {
            let raw_slice = &rest[..j];
            // `stream` is followed by an EOL before the real payload begins (ISO 32000-1
            // §7.3.8.1) and the payload itself may carry a trailing EOL before `endstream` --
            // trim only ASCII whitespace bytes at each end, never the lossy-string `.trim()`.
            let start = raw_slice.iter().position(|b| !b.is_ascii_whitespace()).unwrap_or(raw_slice.len());
            let end = raw_slice.iter().rposition(|b| !b.is_ascii_whitespace()).map(|p| p + 1).unwrap_or(start);
            let raw = &raw_slice[start..end];
            if let Ok(dec) = crate::artifacts::deflate::standards::v_rfc1950::subsets::any::io::zlib_decompress(raw) {
                content = String::from_utf8_lossy(&dec).into_owned();
            }
        }
    }
    let label = content.split('(').nth(1).and_then(|s| s.split(')').next()).unwrap_or("").to_string();
    Ok(PdfSnapshot {
        schema: STDIO_PDF_DOCUMENT_SCHEMA.into(),
        page: PageDoc { width: w, height: h, text: label },
    })
}
//#endregion 🔖️Codec

//#region 🔖️SchemaSpecs
/// 📇️ P2-FG3: `dsl::registry::register_schema_spec` -- genuinely callable here: `PdfSnapshot`/
/// `PageDoc` both derive `dsl::DslRecord` and `PdfDiff` derives `dsl::DslDiff` (S-6/F6's own real
/// derive path for this standard, confirmed by `cargo check`, see `🔺️diff/🦀️component.rs`'s own
/// doc comment), so both `__dsl_spec`/`__dsl_diff_spec` genuinely exist. `PdfMutation`'s own
/// per-variant specs are NOT registered here -- same scope boundary binary/raw's and txt's own
/// identical registration functions document: `register_schema_spec` registers one spec under one
/// schema id, and there is no single canonical id for a Mutation enum's per-variant shapes.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_schema_specs() {
    dsl::registry::register_schema_spec("stdio.pdf", PdfSnapshot::__dsl_spec);
    dsl::registry::register_schema_spec("stdio.pdf#diff", PdfDiff::__dsl_diff_spec);
}

#[cfg(target_arch = "wasm32")]
pub fn register_schema_specs() {}
//#endregion 🔖️SchemaSpecs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::snapshot::demo_pdf_snapshot;

    /// 🧪️ `codec_retention_law`: decode→encode→decode is stable on the field this stub actually
    /// round-trips (`text`, threaded through the FlateDecode content stream) -- `width`/`height`
    /// are NOT retained by `decode_pdf` (documented pre-real-codec scope boundary, W0 recon: 1.4
    /// stays a frozen stub, no decode enrichment), so only `text` is asserted here.
    #[test]
    fn codec_retention_law_text_round_trips_through_encode_decode() {
        let original = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 612.0, height: 792.0, text: "Hello Semio".into() } };
        let bytes = encode_pdf(&original).expect("encode");
        let redecoded = decode_pdf(&bytes).expect("decode");
        assert_eq!(redecoded.page.text, original.page.text);
    }

    #[test]
    fn demo_snapshot_round_trip() {
        let snap = demo_pdf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <PdfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed, snap);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ P2-FG3: per-artifact conformance laws — grammar/protocol parseability, `Recognizer`
    /// against real fixtures AND real `print_op`/`print_diff` output, `walk_protocol` against real
    /// `encode_pack`/`encode_op`/`encode_diff` bytes, and the fixture-honesty round-trip. Lives
    /// here (the engine's own test region), not any framework file.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::{diff, mutations, snapshot};
        use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::mutations::PdfMutation;
        use protocol::{DiffCodec, OpBinary, OpText};

        fn demo_mutation_cases() -> Vec<PdfMutation> {
            vec![
                PdfMutation::NoMutation,
                PdfMutation::SetSnapshot { snapshot: demo_pdf_snapshot() },
                PdfMutation::SetSnapshot { snapshot: PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 612.0, height: 792.0, text: "hello world".into() } } },
            ]
        }

        fn demo_diff_cases() -> Vec<PdfDiff> {
            let a = demo_pdf_snapshot();
            let b = PdfSnapshot { schema: STDIO_PDF_DOCUMENT_SCHEMA.into(), page: PageDoc { width: 300.5, height: 400.25, text: "changed text".into() } };
            vec![<PdfDiff as protocol::command::DiffAlgebra<PdfSnapshot>>::between(&a, &b), PdfDiff::default()]
        }

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

        /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            let text = store::ArtifactDsl::print_dsl(&demo_pdf_snapshot());
            let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
            let reconstructed = format!("{}\n{body}", envelope.envelope_id());
            assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every demo `PdfMutation`.
        #[test]
        fn ops_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for mutation in demo_mutation_cases() {
                let printed = mutation.print_op();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
            }
        }

        /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output.
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let packed = store::ArtifactPack::encode_pack(&demo_pdf_snapshot());
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert_eq!(trace.consumed, inner.len(), "pack walk did not consume every byte");

            let op_spec = dsl::parse_protocol(mutations::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse mutations protocol");
            for mutation in demo_mutation_cases() {
                let bytes = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed for {mutation:?}: {e:?}"));
                let trace = dsl::walk_protocol(&op_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(op) failed for {mutation:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "op walk did not consume every byte for {mutation:?}");
            }

            let diff_spec = dsl::parse_protocol(diff::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse diff protocol");
            for d in demo_diff_cases() {
                let bytes = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed for {d:?}: {e:?}"));
                let trace = dsl::walk_protocol(&diff_spec, &bytes).unwrap_or_else(|e| panic!("walk_protocol(diff) failed for {d:?} @{}: {}", e.offset, e.message));
                assert_eq!(trace.consumed, bytes.len(), "diff walk did not consume every byte for {d:?}");
            }
        }

        /// ✅️ `fixture_honesty_law`: the shipped `.dsl.semio`/`.pack.semio` fixtures are GENUINE
        /// `print_dsl`/`encode_pack` output of `demo_pdf_snapshot()`.
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_pdf_snapshot();

            let parsed = <PdfSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_pdf_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_pdf_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <PdfSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_pdf_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_pdf_snapshot()) drifted from the shipped .pack.semio fixture");
        }
    }
    //#endregion 🔖️ConformanceLaws
}

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, composer_entry_of};
    use crate::artifacts::pdf::standards::v1_4::subsets::any::schema::PdfComposer as PdfRawAnyComposer;
    use crate::artifacts::pdf::standards::v1_4::subsets::a::schema::PdfAComposer;
    use crate::artifacts::pdf::standards::v1_4::subsets::x::schema::PdfXComposer;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PdfRawAnyComposer>(), composer_entry_of::<PdfAComposer>(), composer_entry_of::<PdfXComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry
