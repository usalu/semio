//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use super::super::export::serializers::artifacts::docx::v_ecma_376::any::SemioDocumentToDocx;
    use super::super::export::serializers::artifacts::md::v_commonmark::any::SemioDocumentToMd;
    use super::super::export::serializers::artifacts::pdf::v1_7::any::SemioDocumentToPdf;
    use super::super::export::serializers::artifacts::txt::v_utf_8::any::SemioDocumentToTxt;
    use super::super::import::deserializers::artifacts::docx::v_ecma_376::any::SemioDocumentFromDocx;
    use super::super::import::deserializers::artifacts::md::v_commonmark::any::SemioDocumentFromMd;
    use super::super::import::deserializers::artifacts::pdf::v1_7::any::SemioDocumentFromPdf;
    use super::super::import::deserializers::artifacts::txt::v_utf_8::any::SemioDocumentFromTxt;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, SemioDocumentSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::document::schema::SemioDocumentAnalyzer;
    use semio_framework_plugin::{
        deserializer_entry_of, register_composer_entries, register_subset_validator, serializer_entry_of, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition, Dialect, IoPayload,
        StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

    //#region 🔖️Composer
    pub struct SemioDocumentComposerComposition;

    impl ArtifactComposition for SemioDocumentComposerComposition {
        type Snapshot = SemioDocumentSnapshot;
        const WRITES: Dialect = DIALECT;

        async fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        async fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "SemioDocumentComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioDocumentAnalyzer::analyze(&native).await;
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioDocumentComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️ReferentialInvariants
    /// 🛡️ Real cross-reference checks over a decoded snapshot: unresolved `image_id`/`style_id`
    /// references and `based_on` cycles. Recurses through `List`/`Table`/`Quote` nesting so a
    /// reference buried in a table cell or list item is caught too.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_document_referential_integrity(snapshot: &SemioDocumentSnapshot) -> Vec<dsl::Diagnostic> {
        let mut diagnostics = Vec::new();
        let known_images: std::collections::HashSet<&str> = snapshot.images.iter().map(|i| i.id.as_str()).collect();
        let known_styles: std::collections::HashSet<&str> = snapshot.styles.iter().map(|s| s.id.as_str()).collect();

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn walk(blocks: &[DocBlock], known_images: &std::collections::HashSet<&str>, known_styles: &std::collections::HashSet<&str>, out: &mut Vec<dsl::Diagnostic>) {
            for block in blocks {
                match block {
                    DocBlock::Paragraph { style_id: Some(id), .. } | DocBlock::Heading { style_id: Some(id), .. } if !known_styles.contains(id.as_str()) => {
                        out.push(dsl::Diagnostic::error("stdio.semio_document.unresolved-style-id", dsl::TextSpan::at(1, 1), format!("SemioDocumentValidator: block references unknown style id {id:?}")));
                    }
                    DocBlock::Image { image_id, .. } if !known_images.contains(image_id.as_str()) => {
                        out.push(dsl::Diagnostic::error("stdio.semio_document.unresolved-image-id", dsl::TextSpan::at(1, 1), format!("SemioDocumentValidator: Image block references unknown image id {image_id:?}")));
                    }
                    _ => {}
                }
                match block {
                    DocBlock::List { items, .. } => {
                        for item in items {
                            walk(&item.blocks, known_images, known_styles, out);
                        }
                    }
                    DocBlock::Table { rows } => {
                        for row in rows {
                            for cell in &row.cells {
                                walk(&cell.blocks, known_images, known_styles, out);
                            }
                        }
                    }
                    DocBlock::Quote { blocks } => walk(blocks, known_images, known_styles, out),
                    _ => {}
                }
            }
        }
        walk(&snapshot.blocks, &known_images, &known_styles, &mut diagnostics);

        for style in &snapshot.styles {
            let Some(mut cursor) = style.based_on.clone() else { continue };
            let mut seen = std::collections::HashSet::new();
            seen.insert(style.id.clone());
            loop {
                if !seen.insert(cursor.clone()) {
                    diagnostics.push(dsl::Diagnostic::error("stdio.semio_document.based-on-cycle", dsl::TextSpan::at(1, 1), format!("SemioDocumentValidator: style {:?} has a based_on cycle through {cursor:?}", style.id)));
                    break;
                }
                match snapshot.styles.iter().find(|s| s.id == cursor) {
                    Some(next) => match &next.based_on {
                        Some(v) => cursor = v.clone(),
                        None => break,
                    },
                    None => {
                        diagnostics.push(dsl::Diagnostic::error("stdio.semio_document.unresolved-based-on", dsl::TextSpan::at(1, 1), format!("SemioDocumentValidator: style {:?} has based_on {cursor:?} which does not resolve", style.id)));
                        break;
                    }
                }
            }
        }
        diagnostics
    }
    //#endregion 🔖️ReferentialInvariants

    //#region 🔖️SubsetValidator
    pub struct SemioDocumentValidator;

    impl SubsetValidator for SemioDocumentValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_document_referential_integrity(&snapshot),
                None => vec![dsl::Diagnostic::error("stdio.semio_document.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioDocumentValidator: payload did not decode as a SemioDocumentSnapshot".to_string())],
            }
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioDocumentValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    /// 🚪️ document<->{docx,md,txt,pdf} bridge rows (W4 G6). Each pair contributes a
    /// `deserializer_entry_of` (format -> semio, real `ArtifactDeserializer` leaf under
    /// `🚪️io/📥️import/🧩️deserializers`) + a `serializer_entry_of` (semio -> format, real
    /// `ArtifactSerializer` leaf under `🚪️io/📤️export/🧵️serializers`) row; `register_composer_entries`
    /// derives all 4 `IoKey`s per pair (semio-Import/Export-format, format-Import/Export-semio) from
    /// these 2 rows, per `io_compose_via`'s own doc comment / `register_composer_entries`'s
    /// reads-derives-both-directions behavior.
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_entries() -> &'static [ComposerEntry] {
        IO_ENTRIES.get_or_init(|| {
            vec![
                deserializer_entry_of::<SemioDocumentFromDocx>(),
                serializer_entry_of::<SemioDocumentToDocx>(),
                deserializer_entry_of::<SemioDocumentFromMd>(),
                serializer_entry_of::<SemioDocumentToMd>(),
                deserializer_entry_of::<SemioDocumentFromTxt>(),
                serializer_entry_of::<SemioDocumentToTxt>(),
                deserializer_entry_of::<SemioDocumentFromPdf>(),
                serializer_entry_of::<SemioDocumentToPdf>(),
            ]
        })
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    /// 📌️ Registers this subset's schema descriptor, document codec, SubsetValidator, and the
    /// document<->{docx,md,txt,pdf} io bridge rows. Called from this artifact's standard-level
    /// `engine::register()`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::schema::register_artifact_schema_descriptor(crate::artifacts::semio::standards::v1::subsets::document::schema::semio_document_artifact_schema_descriptor());
        let _ = store::register_document_codec(store::ArtifactCodec::of::<SemioDocumentSnapshot, crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation>(
            crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA,
        ));
        let _ = register_subset_validator(validator_entry());
        let _ = register_composer_entries(io_entries());
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.document.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::schema::register_artifact_inference_descriptor(crate::artifacts::semio::standards::v1::subsets::document::schema::inferences::semio_document_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocImage, DocStyle};
        use semio_framework_plugin::{ArtifactDeserializer, ArtifactSerializer};

        #[semio_framework_async_macros::async_test]
        async fn clean_document_validates_with_no_diagnostics() {
            let snapshot = SemioDocumentSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
                styles: vec![DocStyle { id: "base".into(), name: "Base".into(), based_on: None }, DocStyle { id: "child".into(), name: "Child".into(), based_on: Some("base".into()) }],
                images: vec![DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1] }],
                blocks: vec![DocBlock::Paragraph { style_id: Some("child".into()), runs: Vec::new() }, DocBlock::Image { image_id: "img1".into(), alt: "alt".into(), width: None, height: None }],
            };
            let bytes = store::ArtifactPack::encode_pack(&snapshot);
            let diagnostics = SemioDocumentValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.is_empty(), "expected no diagnostics, got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn unresolved_image_and_style_references_are_flagged() {
            let snapshot = SemioDocumentSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
                styles: Vec::new(),
                images: Vec::new(),
                blocks: vec![DocBlock::Paragraph { style_id: Some("missing-style".into()), runs: Vec::new() }, DocBlock::Image { image_id: "missing-image".into(), alt: String::new(), width: None, height: None }],
            };
            let bytes = store::ArtifactPack::encode_pack(&snapshot);
            let diagnostics = SemioDocumentValidator::validate(&IoPayload::Binary(bytes));
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-style-id"), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-image-id"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn based_on_cycle_is_flagged() {
            let snapshot = SemioDocumentSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
                styles: vec![DocStyle { id: "a".into(), name: "A".into(), based_on: Some("b".into()) }, DocStyle { id: "b".into(), name: "B".into(), based_on: Some("a".into()) }],
                images: Vec::new(),
                blocks: Vec::new(),
            };
            let diagnostics = check_document_referential_integrity(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.based-on-cycle"), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn nested_table_cell_reference_is_checked() {
            let snapshot = SemioDocumentSnapshot {
                schema: crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
                styles: Vec::new(),
                images: Vec::new(),
                blocks: vec![DocBlock::Table {
                    rows: vec![crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocTableRow {
                        cells: vec![crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocTableCell { blocks: vec![DocBlock::Image { image_id: "nested-missing".into(), alt: String::new(), width: None, height: None }] }],
                    }],
                }],
            };
            let diagnostics = check_document_referential_integrity(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == "stdio.semio_document.unresolved-image-id"), "nested reference must be checked too: {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn composer_reads_own_dialect_pack() {
            let snapshot = SemioDocumentSnapshot::default();
            let bytes = store::ArtifactPack::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = SemioDocumentComposerComposition::compose(&sources).expect("compose from native dialect");
            assert_eq!(composed.snapshot, snapshot);
        }

        //#region 🔖️IoRoundTrips
        // 🔁️ W4 G6 fixture-backed round trips: format1 -(deserialize)-> semio1 -(serialize)->
        // format2 -(deserialize)-> semio2, asserting semio1 == semio2 — i.e. this pair's serializer is
        // a faithful inverse of what its deserializer captured (documented lossy fields, e.g. docx's
        // `extra_*_properties` or txt's formatting, never entering the comparison because
        // `SemioDocumentSnapshot` itself has no field for them).

        #[semio_framework_async_macros::async_test]
        async fn docx_round_trip_is_stable() {
            use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxDocument, DocxParagraph, DocxRun, DocxStyle};
            use crate::artifacts::docx::DocxSnapshot;
            use crate::artifacts::zip::opc::OpcPackage;

            let docx1 = DocxSnapshot::from_parts(
                OpcPackage::default(),
                DocxDocument {
                    styles: vec![DocxStyle { id: "Heading1".into(), name: "Heading 1".into(), based_on: None }],
                    body: vec![
                        DocxBlock::Paragraph(DocxParagraph {
                            runs: vec![DocxRun { text: "Title".into(), bold: true, italic: false, underline: false, extra_run_properties: Vec::new() }],
                            style: Some("Heading1".into()),
                            extra_paragraph_properties: Vec::new(),
                        }),
                        DocxBlock::paragraph("Body."),
                    ],
                },
            );
            let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromDocx::deserialize(&docx1)).expect("deserialize");
            let docx2 = semio_framework_plugin::resolve_ready(SemioDocumentToDocx::serialize(&semio1)).expect("serialize");
            let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromDocx::deserialize(&docx2)).expect("deserialize round 2");
            assert_eq!(semio1, semio2);
        }

        #[semio_framework_async_macros::async_test]
        async fn md_round_trip_is_stable() {
            use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};
            use crate::artifacts::md::MdSnapshot;

            let md1 = MdSnapshot {
                schema: crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.into(),
                blocks: vec![
                    MdBlock::Heading { level: 1, inlines: vec![MdInline::Strong { inlines: vec![MdInline::Text { text: "Title".into() }] }] },
                    MdBlock::Paragraph { inlines: vec![MdInline::Text { text: "Body".into() }] },
                    MdBlock::CodeBlock { info: Some("rust".into()), literal: "fn main() {}".into() },
                ],
            };
            let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromMd::deserialize(&md1)).expect("deserialize");
            let md2 = semio_framework_plugin::resolve_ready(SemioDocumentToMd::serialize(&semio1)).expect("serialize");
            let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromMd::deserialize(&md2)).expect("deserialize round 2");
            assert_eq!(semio1, semio2);
        }

        #[semio_framework_async_macros::async_test]
        async fn txt_round_trip_is_stable() {
            use crate::artifacts::txt::schema::snapshot::LineEnding;
            use crate::artifacts::txt::TxtSnapshot;

            let txt1 = TxtSnapshot { schema: crate::artifacts::txt::STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: vec!["First line.".into(), String::new(), "Third line.".into()], trailing_newline: true, line_ending: LineEnding::Lf };
            let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromTxt::deserialize(&txt1)).expect("deserialize");
            let txt2 = semio_framework_plugin::resolve_ready(SemioDocumentToTxt::serialize(&semio1)).expect("serialize");
            let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromTxt::deserialize(&txt2)).expect("deserialize round 2");
            assert_eq!(semio1, semio2);
        }

        #[semio_framework_async_macros::async_test]
        async fn pdf_round_trip_is_stable() {
            use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfPage, PdfSnapshot};

            let mut p1 = PdfPage::new(612.0, 792.0);
            p1.text = "Page one text.".into();
            let mut p2 = PdfPage::new(612.0, 792.0);
            p2.text = "Page two text.".into();
            let pdf1 = PdfSnapshot { pages: vec![p1, p2], ..Default::default() };
            let semio1 = semio_framework_plugin::resolve_ready(SemioDocumentFromPdf::deserialize(&pdf1)).expect("deserialize");
            let pdf2 = semio_framework_plugin::resolve_ready(SemioDocumentToPdf::serialize(&semio1)).expect("serialize");
            let semio2 = semio_framework_plugin::resolve_ready(SemioDocumentFromPdf::deserialize(&pdf2)).expect("deserialize round 2");
            assert_eq!(semio1, semio2);
        }
        //#endregion 🔖️IoRoundTrips

        //#region 🔖️ConformanceLaws
        /// 🧪️ Per-artifact conformance laws (grammar recipe §4 item 8) for `s.stdio.semio.document`'s
        /// three facets — ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave,
        /// following the flow/model/brep pilots' proven pattern. Lives in this composer's own test
        /// region: document has no per-standard `⚙️engine` dir the way json/csv/zip/png do, and v1's
        /// SHARED `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` only aggregates all 14 subsets'
        /// `register()` calls (no test module of its own, and out of this ticket's `✳️document/`-only
        /// edit scope anyway).
        mod conformance_laws {

            use crate::artifacts::semio::standards::v1::subsets::document::schema::{diff, mutations, snapshot};
            use protocol::{DiffCodec, OpBinary, OpText};

            /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
            /// parse under the real dialect — independent of, and cheaper than, the two `recognize`/
            /// `walk_protocol` laws below.
            #[semio_framework_async_macros::async_test]
            async fn committed_facet_files_parse() {
                for (label, text) in [("snapshot grammar", snapshot::text::COMPONENT_GRAMMAR_SEMIO), ("mutations grammar", mutations::text::COMPONENT_GRAMMAR_SEMIO), ("diff grammar", diff::text::COMPONENT_GRAMMAR_SEMIO)] {
                    let grammar = dsl::parse_grammar(text).unwrap_or_else(|e| panic!("{label}: parse_grammar failed: {e:?}"));
                    assert_eq!(grammar.dialect, dsl::SemioDialect::Grammar, "{label}: expected grammar dialect");
                }
                for (label, text) in [("snapshot protocol", snapshot::binary::COMPONENT_PROTOCOL_SEMIO), ("mutations protocol", mutations::binary::COMPONENT_PROTOCOL_SEMIO), ("diff protocol", diff::binary::COMPONENT_PROTOCOL_SEMIO)] {
                    dsl::parse_protocol(text).unwrap_or_else(|e| panic!("{label}: parse_protocol failed: {e:?}"));
                }
            }

            /// ✅️ `grammar_conformance_law`: the snapshot grammar recognizes real `print_dsl` output for
            /// the demo snapshot — same preamble-stripped body reconstruction the eventual
            /// `m5_handcrafted_grammar_conformance` harness uses (envelope id prepended as the bare
            /// `artifact-mark` token), so this is a direct proof this facet will pass that harness once
            /// graduated.
            #[semio_framework_async_macros::async_test]
            async fn grammar_conformance_law() {
                let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                let text = store::ArtifactDsl::print_dsl(&snapshot::demo_semio_document_snapshot());
                let (envelope, body) = store::semio_format::split_text_preamble(&text).expect("split preamble");
                let reconstructed = format!("{}\n{body}", envelope.envelope_id());
                assert!(recognizer.recognize(&reconstructed).expect("recognize"), "grammar did not recognize demo dsl body:\n{reconstructed}");
            }

            /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op` output
            /// for every `SemioDocumentMutation` variant (`mutations::demo_mutation_cases()`).
            #[semio_framework_async_macros::async_test]
            async fn ops_grammar_conformance_law() {
                let grammar = dsl::parse_grammar(mutations::text::COMPONENT_GRAMMAR_SEMIO).expect("parse mutations grammar");
                let recognizer = dsl::Recognizer::compile(&grammar);
                for mutation in mutations::demo_mutation_cases() {
                    let printed = mutation.print_op();
                    assert!(recognizer.recognize(&printed).unwrap_or(false), "mutations grammar did not recognize {printed:?} (from {mutation:?})");
                }
            }

            /// ✅️ `diff_grammar_conformance_law`: the diff grammar recognizes real `print_diff` output
            /// for every representative `SemioDocumentDiff` (`diff::demo_diff_cases()`), incl. the
            /// empty (no-op) diff.
            #[semio_framework_async_macros::async_test]
            async fn diff_grammar_conformance_law() {
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
            #[semio_framework_async_macros::async_test]
            async fn protocol_walk_law() {
                let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
                let packed = store::ArtifactPack::encode_pack(&snapshot::demo_semio_document_snapshot());
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
            /// `print_dsl`/`encode_pack` output of `snapshot::demo_semio_document_snapshot()` —
            /// `parse_dsl(fixture) == demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the
            /// pack twin — so the fixtures can never silently drift back to a fake.
            #[semio_framework_async_macros::async_test]
            async fn fixture_honesty_law() {
                const FIXTURE_DSL: &str = include_str!("../../✳️any/📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio");
                const FIXTURE_PACK: &[u8] = include_bytes!("../../✳️any/📚️examples/📄️memo/🖼️assets/🎒️example.pack.semio");

                let demo = snapshot::demo_semio_document_snapshot();

                let parsed = <snapshot::SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
                assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_semio_document_snapshot()");
                assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_semio_document_snapshot()) drifted from the shipped .dsl.semio fixture");

                let decoded = <snapshot::SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
                assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_semio_document_snapshot()");
                assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_semio_document_snapshot()) drifted from the shipped .pack.semio fixture");
            }
        }
        //#endregion 🔖️ConformanceLaws
    }
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
