//! 🧬️ DocxArtifact schema — full artifact state.

use crate::artifacts::docx::schema::snapshot::DocxDocument;
use crate::artifacts::docx::DocxSnapshot;
use crate::artifacts::zip::opc::OpcPackage;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.docx` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.docx")]
pub struct DocxArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub opc: OpcPackage,
    #[state(persistent)]
    #[serde(default)]
    pub document: DocxDocument,
}
//#endregion Artifact

//#region Conversions
impl Default for DocxArtifact {
    fn default() -> Self {
        Self::from_snapshot(DocxSnapshot::default())
    }
}

impl DocxArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> DocxSnapshot {
        DocxSnapshot { schema: self.schema.clone(), opc: self.opc.clone(), document: self.document.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: DocxSnapshot) -> Self {
        Self { schema: snapshot.schema, opc: snapshot.opc, document: snapshot.document }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: DocxSnapshot) {
        self.schema = snapshot.schema;
        self.opc = snapshot.opc;
        self.document = snapshot.document;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.docx`.
pub fn docx_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.docx",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable};
    use crate::artifacts::docx::{DocxDiff, DocxMutation, DocxSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.docx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DocxBuilderConstruction {
        snapshot: DocxSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DocxBuilderConstruction {
        type Snapshot = DocxSnapshot;
        type Mutation = DocxMutation;
        type Diff = DocxDiff;
        fn empty() -> Self {
            Self { snapshot: DocxSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DocxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::docx::schema::mutations::apply_docx_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <DocxDiff as protocol::MutationDiff<DocxSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    /// 🧱️ Typed content constructors — build a `word/document.xml` document from paragraphs/runs
    /// with basic formatting (bold/italic), mirroring the svg artifact's "builder builds a full
    /// standard document" reference shape. Chainable; `build()` (from `ArtifactBuilder`) produces
    /// the final `DocxSnapshot`, whose OPC container is assembled fresh (see `io::export::serializers::build_minimal_docx`)
    /// the first time a paragraph is added to an otherwise-empty builder.
    impl DocxBuilderConstruction {
        /// ➕️ Appends a paragraph.
        pub fn add_paragraph(mut self, paragraph: DocxParagraph) -> Self {
            self.snapshot.document.body.push(DocxBlock::Paragraph(paragraph));
            self.snapshot = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(self.snapshot.document);
            self
        }

        /// ➕️ Appends a single-run plain-text paragraph.
        pub fn add_text_paragraph(self, text: impl Into<String>) -> Self {
            self.add_paragraph(DocxParagraph::text(text.into()))
        }

        /// ➕️ Appends a paragraph made of the given runs (basic bold/italic/underline formatting).
        pub fn add_runs(self, runs: Vec<DocxRun>) -> Self {
            self.add_paragraph(DocxParagraph { runs, style: None, extra_paragraph_properties: Vec::new() })
        }

        /// ➕️ Appends a table.
        pub fn add_table(mut self, table: DocxTable) -> Self {
            self.snapshot.document.body.push(DocxBlock::Table(table));
            self.snapshot = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(self.snapshot.document);
            self
        }

        /// ➕️ Appends (or replaces, by `id`) a named style.
        pub fn add_style(mut self, style: DocxStyle) -> Self {
            if let Some(existing) = self.snapshot.document.styles.iter_mut().find(|s| s.id == style.id) {
                *existing = style;
            } else {
                self.snapshot.document.styles.push(style);
            }
            self.snapshot = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(self.snapshot.document);
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::docx::DocxSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.docx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DocxParts {
        pub snapshot: Option<DocxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.docx` (ecma-376/✳️any) sources.
    pub struct DocxAnalyzerAnalysis;

    impl ArtifactAnalysis for DocxAnalyzerAnalysis {
        type Parts = DocxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.docx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: OPC-shaped bytes (real `[Content_Types].xml`) whose root officeDocument
            // relationship resolves under `word/` — disambiguates from xlsx/pptx, which share the
            // same zip magic and OPC shape but resolve under `xl/`/`ppt/` instead.
            match source {
                AnalyzeSource::Binary(bytes) if crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::import::deserializers::sniff_docx_bytes(bytes) => IoConfidence::High,
                AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DocxParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DocxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.text",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <DocxSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.binary",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
pub fn empty_docx_snapshot() -> DocxSnapshot { DocxSnapshot::default() }

/// 📄️ FG-wave: the demo `stdio.docx` document — a genuinely non-trivial `DocxSnapshot` exercising
/// a styled heading paragraph, a mixed-formatting run (bold/italic/plain), a 2x2 table (recursing
/// through `Table -> row -> cell -> Paragraph`), two named styles (one `based_on` the other), and
/// one unmodeled raw OPC part (`word/numbering.xml`, verbatim-retained). The single source of
/// truth for `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`/`🎒️example.pack.semio` (both are
/// literally this snapshot's `print_dsl`/`encode_pack` output, asserted equal by
/// `fixture_honesty_law` below) — same shape `📷️png/…/⚙️engine/🦀️component.rs`'s own
/// `demo_png_snapshot()` establishes.
pub fn demo_docx_snapshot() -> DocxSnapshot {
    use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
    let document = DocxDocument {
        body: vec![
            DocxBlock::Paragraph(DocxParagraph { style: Some("Heading1".into()), ..DocxParagraph::text("Semio Demo") }),
            DocxBlock::Paragraph(DocxParagraph {
                runs: vec![
                    DocxRun { text: "Bold and ".into(), bold: true, ..Default::default() },
                    DocxRun { text: "italic".into(), italic: true, ..Default::default() },
                    DocxRun { text: " text".into(), ..Default::default() },
                ],
                style: None,
                extra_paragraph_properties: Vec::new(),
            }),
            DocxBlock::Table(DocxTable {
                rows: vec![
                    DocxTableRow {
                        cells: vec![
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C1")], ..Default::default() },
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C2")], ..Default::default() },
                        ],
                        ..Default::default()
                    },
                    DocxTableRow {
                        cells: vec![
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R2C1")], ..Default::default() },
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R2C2")], ..Default::default() },
                        ],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }),
        ],
        styles: vec![
            DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None },
            DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) },
        ],
    };
    let mut snap = crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_docx(document);
    snap.opc.set_part(
        "word/numbering.xml",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
        b"<w:numbering/>".to_vec(),
    );
    snap
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec DocxBuilderFacets {
        construction: derived_construction::DocxBuilderConstruction,
        analysis: derived_analysis::DocxAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DocxComposerComposition,
    }
    builder: DocxBuilder,
    analyzer: DocxAnalyzer,
    composer: DocxComposer,
);
//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::docx::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
    use crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::{build_minimal_docx, document_to_xml, encode_docx};
    use crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::import::deserializers::{decode_docx, sniff_docx_bytes};
    use crate::artifacts::docx::standards::v_ecma_376::subsets::any::io::DocxError;
    use crate::artifacts::xml::schema::snapshot::{xml_document_to_text, XmlAttr, XmlNode};
    use crate::artifacts::zip::opc::{OpcPackage, RELS_CONTENT_TYPE};

    fn sample_document() -> DocxDocument {
        DocxDocument {
            body: vec![
                DocxBlock::Paragraph(DocxParagraph {
                    runs: vec![
                        DocxRun { text: "Hello, ".into(), bold: true, ..Default::default() },
                        DocxRun { text: "world!".into(), italic: true, ..Default::default() },
                    ],
                    style: None,
                    extra_paragraph_properties: Vec::new(),
                }),
                DocxBlock::paragraph("Second paragraph, plain."),
            ],
            styles: Vec::new(),
        }
    }

    fn sample_document_with_table_and_styles() -> DocxDocument {
        DocxDocument {
            body: vec![
                DocxBlock::Paragraph(DocxParagraph { style: Some("Heading1".into()), ..DocxParagraph::text("Title") }),
                DocxBlock::Table(DocxTable {
                    rows: vec![DocxTableRow {
                        cells: vec![
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C1")], ..Default::default() },
                            DocxTableCell { blocks: vec![DocxBlock::paragraph("R1C2")], ..Default::default() },
                        ],
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            ],
            styles: vec![
                DocxStyle { id: "Normal".into(), name: "Normal".into(), based_on: None },
                DocxStyle { id: "Heading1".into(), name: "heading 1".into(), based_on: Some("Normal".into()) },
            ],
        }
    }

    #[test]
    fn builder_produces_minimal_valid_package_that_decodes_back() {
        let snap = build_minimal_docx(sample_document());
        let bytes = encode_docx(&snap).expect("encode minimal package");
        assert!(crate::artifacts::zip::opc::sniff_opc_bytes(&bytes));
        assert!(sniff_docx_bytes(&bytes));
        let decoded = decode_docx(&bytes).expect("decode minimal package");
        assert_eq!(decoded.document, sample_document());
    }

    #[test]
    fn tables_and_styles_round_trip() {
        let snap = build_minimal_docx(sample_document_with_table_and_styles());
        let bytes = encode_docx(&snap).expect("encode");
        let decoded = decode_docx(&bytes).expect("decode");
        assert_eq!(decoded.document, sample_document_with_table_and_styles());
        let DocxBlock::Table(table) = &decoded.document.body[1] else { panic!("expected table") };
        assert_eq!(table.rows[0].cells.len(), 2);
    }

    #[test]
    fn decode_resolves_real_hand_built_package_with_formatting() {
        // Hand-built OOXML: correct Content_Types/.rels/part structure, not just "a zip with xml".
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        let xml = concat!(
            r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">"#,
            "<w:body>",
            r#"<w:p><w:r><w:rPr><w:b/></w:rPr><w:t xml:space="preserve">Bold run</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:rPr><w:i/></w:rPr><w:t xml:space="preserve">Italic run</w:t></w:r></w:p>"#,
            r#"<w:p><w:r><w:t xml:space="preserve">Plain &amp; escaped</w:t></w:r></w:p>"#,
            "</w:body>",
            "</w:document>",
        );
        const MAIN_DOCUMENT_PART: &str = "word/document.xml";
        const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
        const REL_TYPE_OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
        opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml.as_bytes().to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode opc");

        let decoded = decode_docx(&bytes).expect("decode hand-built docx");
        assert_eq!(decoded.document.body.len(), 3);
        let DocxBlock::Paragraph(p0) = &decoded.document.body[0] else { panic!("paragraph") };
        assert!(p0.runs[0].bold);
        let DocxBlock::Paragraph(p1) = &decoded.document.body[1] else { panic!("paragraph") };
        assert!(p1.runs[0].italic);
        let DocxBlock::Paragraph(p2) = &decoded.document.body[2] else { panic!("paragraph") };
        assert_eq!(p2.runs[0].text, "Plain & escaped");
    }

    #[test]
    fn unmodeled_parts_survive_decode_encode_verbatim() {
        const MAIN_DOCUMENT_PART: &str = "word/document.xml";
        const MAIN_DOCUMENT_CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
        const REL_TYPE_OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.set_part(MAIN_DOCUMENT_PART, MAIN_DOCUMENT_CONTENT_TYPE, xml_document_to_text(&document_to_xml(&sample_document())).into_bytes());
        opc.set_part("word/numbering.xml", "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml", b"<w:numbering/>".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, MAIN_DOCUMENT_PART);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode");

        let decoded = decode_docx(&bytes).expect("decode");
        assert_eq!(decoded.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
        let re_encoded = encode_docx(&decoded).expect("re-encode");
        let re_decoded = decode_docx(&re_encoded).expect("re-decode");
        assert_eq!(re_decoded.opc.part_bytes("word/numbering.xml"), Some(b"<w:numbering/>".as_slice()));
        assert_eq!(re_decoded.document, sample_document());
    }

    #[test]
    fn unmodeled_run_properties_survive_round_trip() {
        let mut run = DocxRun { text: "colored".into(), ..Default::default() };
        run.extra_run_properties.push(XmlNode::Element { name: "w:color".into(), attrs: vec![XmlAttr { name: "w:val".into(), value: "FF0000".into() }], children: vec![] });
        let doc = DocxDocument { body: vec![DocxBlock::Paragraph(DocxParagraph { runs: vec![run], style: None, extra_paragraph_properties: Vec::new() })], styles: Vec::new() };
        let snap = build_minimal_docx(doc.clone());
        let bytes = encode_docx(&snap).expect("encode");
        let decoded = decode_docx(&bytes).expect("decode");
        assert_eq!(decoded.document, doc);
    }

    #[test]
    fn decode_rejects_missing_main_document_relationship() {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        let bytes = crate::artifacts::zip::opc::encode_opc(&opc).expect("encode");
        let err = decode_docx(&bytes).expect_err("must reject a package with no officeDocument relationship");
        assert_eq!(err, DocxError::MissingMainDocumentRelationship);
    }

    #[test]
    fn analyzer_builder_round_trip() {
        let original = build_minimal_docx(sample_document_with_table_and_styles());
        // Analyzer: real decode of the encoded bytes.
        let bytes = encode_docx(&original).expect("encode");
        let analyzed = decode_docx(&bytes).expect("decode");
        // Builder: reconstruct an equivalent document from the analyzed parts.
        let rebuilt = build_minimal_docx(analyzed.document.clone());
        let rebuilt_bytes = encode_docx(&rebuilt).expect("encode rebuilt");
        let reanalyzed = decode_docx(&rebuilt_bytes).expect("decode rebuilt");
        assert_eq!(reanalyzed.document, analyzed.document);
    }

    //#region 🔖️ConformanceLaws
    /// 🧪️ FG-wave: per-artifact conformance laws (`📖️grammar-recipe.md` §4's checklist item) --
    /// grammar/protocol parseability, `Recognizer` against real fixtures AND real `print_op`/
    /// `print_diff` output, `walk_protocol` against real `encode_pack`/`encode_op`/`encode_diff`
    /// bytes, and the fixture-honesty round-trip. Lives beside the rest of this artifact's schema
    /// tests (moved out of `⚙️engine`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) --
    /// this artifact's OWN early-warning, plus direct coverage of the mutations/diff facets the
    /// framework's `m5` auto-discovery does not reach at all.
    mod conformance_laws {
        use super::*;
        use crate::artifacts::docx::schema::{diff, mutations, snapshot};
        use protocol::{DiffCodec, OpBinary, OpText};

        /// ✅️ "committed files parse": all 6 handcrafted `.grammar.semio`/`.protocol.semio` files
        /// parse under the real dialect -- independent of, and cheaper than, the two
        /// `recognize`/`walk_protocol` laws below (a parse failure here fails fast with a clearer
        /// message).
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

        /// ✅️ `grammar_conformance_law`: the snapshot grammar models the real TEXT syntax of the
        /// XML parts a docx OPC package carries (`📸️snapshot/📝️text/📖️component.grammar.semio`'s
        /// own doc comment explains why -- this artifact's `ArtifactDsl::print_dsl` hex-dumps the
        /// WHOLE binary OPC package, matching this facet's SIBLING binary protocol, not this text
        /// grammar; the two facets describe different LAYERS of the same real artifact, same as
        /// every OPC-family member's own container/contained-parts split). So, UNLIKE a
        /// binary-native pilot's `grammar_conformance_law` (which feeds `print_dsl` output
        /// straight to the recognizer), this law decodes the REAL zip entries `encode_docx`
        /// genuinely produces (via `zip::engine::decode_zip`, the same real codec `opc::decode_opc`
        /// itself delegates to) and recognizes EACH real part's own text against the grammar --
        /// direct proof the grammar matches this artifact's own real per-part XML bytes, not an
        /// invented approximation.
        #[test]
        fn grammar_conformance_law() {
            let grammar = dsl::parse_grammar(snapshot::text::COMPONENT_GRAMMAR_SEMIO).expect("parse snapshot grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);

            let demo = demo_docx_snapshot();
            let bytes = encode_docx(&demo).expect("encode demo docx");
            let zip = crate::artifacts::zip::engine::decode_zip(&bytes).expect("decode zip");

            let modeled_parts = ["[Content_Types].xml", "_rels/.rels", "word/document.xml", "word/styles.xml"];
            let mut checked = 0;
            for entry in &zip.entries {
                if !modeled_parts.contains(&entry.name.as_str()) {
                    continue;
                }
                let text = String::from_utf8(entry.data.clone()).unwrap_or_else(|e| panic!("part {:?}: not valid utf-8: {e}", entry.name));
                assert!(recognizer.recognize(&text).unwrap_or(false), "grammar did not recognize real part {:?}:\n{text}", entry.name);
                checked += 1;
            }
            assert_eq!(checked, modeled_parts.len(), "not every modeled part was present in the real zip entries");
        }

        /// ✅️ `ops_grammar_conformance_law`: the mutations grammar recognizes real `print_op`
        /// output for every `DocxMutation` variant (`mutations::demo_mutation_cases()`).
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
        /// for every representative `DocxDiff` (`diff::demo_diff_cases()`).
        #[test]
        fn diff_grammar_conformance_law() {
            let grammar = dsl::parse_grammar(diff::text::COMPONENT_GRAMMAR_SEMIO).expect("parse diff grammar");
            let recognizer = dsl::Recognizer::compile(&grammar);
            for d in diff::demo_diff_cases() {
                let printed = d.print_diff();
                assert!(recognizer.recognize(&printed).unwrap_or(false), "diff grammar did not recognize {printed:?} (from {d:?})");
            }
        }

        /// ✅️ `protocol_walk_law`: `walk_protocol` against REAL bytes for all three facets --
        /// snapshot pack (`encode_pack`, envelope-unwrapped first, matching how
        /// `m5_handcrafted_protocol_conformance` itself feeds `walk_protocol`), every demo
        /// mutation's `encode_op`, and every demo diff's `encode_diff`. The snapshot protocol
        /// declares `backward`/`jump` (restated from zip's own real ZIP layout), so `walk_protocol`
        /// correctly does NOT require landing on exactly `bytes.len()` (M2's own documented
        /// exception, `📖️grammar-recipe.md` §2.3) -- assert a sane in-range `consumed` there
        /// instead, same as zip's own `protocol_walk_law` does; the op/diff protocols have no such
        /// exception and must consume every byte.
        #[test]
        fn protocol_walk_law() {
            let pack_spec = dsl::parse_protocol(snapshot::binary::COMPONENT_PROTOCOL_SEMIO).expect("parse snapshot protocol");
            let demo = demo_docx_snapshot();
            let packed = store::ArtifactPack::encode_pack(&demo);
            let (_, inner) = store::semio_format::unwrap_binary(&packed).expect("unwrap semio envelope");
            let trace = dsl::walk_protocol(&pack_spec, &inner).unwrap_or_else(|e| panic!("walk_protocol(pack) failed @{}: {}", e.offset, e.message));
            assert!(trace.consumed > 0 && trace.consumed <= inner.len(), "pack walk consumed an out-of-range span");

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
        /// `print_dsl`/`encode_pack` output of `demo_docx_snapshot()` -- `parse_dsl(fixture) ==
        /// demo()`, `print_dsl(demo()) == fixture` (byte-for-byte), and the pack twin -- so the
        /// fixtures can never silently drift back to a fake `"68656c6c6f"`-style placeholder again
        /// (see this ticket's own recon note on the pre-FG-wave state of these two files).
        #[test]
        fn fixture_honesty_law() {
            const FIXTURE_DSL: &str = include_str!("../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");
            const FIXTURE_PACK: &[u8] = include_bytes!("../📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio");

            let demo = demo_docx_snapshot();

            let parsed = <DocxSnapshot as store::ArtifactDsl>::parse_dsl(FIXTURE_DSL).expect("parse shipped .dsl.semio fixture");
            assert_eq!(parsed, demo, "shipped .dsl.semio fixture does not parse back to demo_docx_snapshot()");
            assert_eq!(store::ArtifactDsl::print_dsl(&demo), FIXTURE_DSL, "print_dsl(demo_docx_snapshot()) drifted from the shipped .dsl.semio fixture");

            let decoded = <DocxSnapshot as store::ArtifactPack>::decode_pack(FIXTURE_PACK).expect("decode shipped .pack.semio fixture");
            assert_eq!(decoded, demo, "shipped .pack.semio fixture does not decode back to demo_docx_snapshot()");
            assert_eq!(store::ArtifactPack::encode_pack(&demo), FIXTURE_PACK, "encode_pack(demo_docx_snapshot()) drifted from the shipped .pack.semio fixture");

            let native = encode_docx(&demo).expect("encode native docx");
            assert_eq!(native.as_slice(), include_bytes!("../📚️examples/🎬️demo/🖼️assets/📜️example.docx"), "encode_docx(demo) drifted from 📜️example.docx");
        }

        #[test]
        #[ignore]
        fn zzz_write_native_docx_fixture() {
            let demo = demo_docx_snapshot();
            let native = encode_docx(&demo).expect("encode");
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/📜️example.docx");
            std::fs::write(path, native).expect("write 📜️example.docx");
        }
    }
    //#endregion 🔖️ConformanceLaws
}
//#endregion 🧪️Tests
