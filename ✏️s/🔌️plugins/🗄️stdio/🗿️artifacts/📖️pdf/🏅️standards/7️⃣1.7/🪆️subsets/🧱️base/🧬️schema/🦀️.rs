//! 🧬️ PdfArtifact schema (1.7) — full artifact state.

use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;

//#region 🏅️ConformanceSupport
#[path = "🏅️conformance-support/🦀️.rs"]
pub mod conformance_support;
//#endregion 🏅️ConformanceSupport

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub declared_version: String,
    #[state(artifact)]
    #[value(default)]
    pub pages: Vec<PdfPage>,
    #[state(artifact)]
    #[value(default)]
    pub fonts: Vec<PdfFont>,
    #[state(artifact)]
    #[value(default)]
    pub images: Vec<PdfImage>,
    #[state(artifact)]
    #[value(default)]
    pub forms: Vec<PdfFormXObject>,
    #[state(artifact)]
    #[value(default)]
    pub ext_g_states: Vec<PdfExtGState>,
    #[state(artifact)]
    #[value(default)]
    pub shadings: Vec<PdfShading>,
    #[state(artifact)]
    #[value(default)]
    pub patterns: Vec<PdfPattern>,
    #[state(artifact)]
    #[value(default)]
    pub color_spaces: Vec<PdfNamedColorSpace>,
    #[state(artifact)]
    #[value(default)]
    pub properties: Vec<PdfNamedProperties>,
    #[state(artifact)]
    #[value(default)]
    pub outlines: Vec<PdfOutlineItem>,
    #[state(artifact)]
    #[value(default)]
    pub named_destinations: Vec<PdfNamedDestination>,
    #[state(artifact)]
    #[value(default)]
    pub page_labels: Vec<PdfPageLabelRange>,
    #[state(artifact)]
    #[value(default)]
    pub embedded_files: Vec<PdfEmbeddedFile>,
    #[state(artifact)]
    #[value(default)]
    pub output_intents: Vec<PdfOutputIntent>,
    #[state(artifact)]
    #[value(default)]
    pub acro_form: Option<PdfAcroForm>,
    #[state(artifact)]
    #[value(default)]
    pub optional_content: Option<PdfOptionalContent>,
    #[state(artifact)]
    #[value(default)]
    pub page_layout: Option<PdfPageLayout>,
    #[state(artifact)]
    #[value(default)]
    pub page_mode: Option<PdfPageMode>,
    #[state(artifact)]
    #[value(default)]
    pub viewer_preferences: Option<PdfViewerPreferences>,
    #[state(artifact)]
    #[value(default)]
    pub open_action: Option<PdfOpenAction>,
    #[state(artifact)]
    #[value(default)]
    pub language: Option<String>,
    #[state(artifact)]
    #[value(default)]
    pub mark_info: Option<PdfMarkInfo>,
    #[state(artifact)]
    #[value(default)]
    pub metadata: Option<String>,
    #[state(artifact)]
    #[value(default)]
    pub document_id: Option<[Vec<u8>; 2]>,
    #[state(artifact)]
    #[value(default)]
    pub encryption: Option<PdfEncryption>,
    #[state(artifact)]
    #[value(default)]
    pub info: PdfInfo,
    #[state(artifact)]
    #[value(default)]
    pub catalog_extra: Vec<PdfDictEntry>,
    #[state(artifact)]
    #[value(default)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(artifact)]
    #[value(default)]
    pub trailer: Vec<PdfDictEntry>,
}

impl Default for PdfArtifact {
    fn default() -> Self {
        Self::from_snapshot(PdfSnapshot::default())
    }
}

impl PdfArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> PdfSnapshot {
        PdfSnapshot { schema: self.schema.clone(), declared_version: self.declared_version.clone(), pages: self.pages.clone(), fonts: self.fonts.clone(), images: self.images.clone(), forms: self.forms.clone(), ext_g_states: self.ext_g_states.clone(), shadings: self.shadings.clone(), patterns: self.patterns.clone(), color_spaces: self.color_spaces.clone(), properties: self.properties.clone(), outlines: self.outlines.clone(), named_destinations: self.named_destinations.clone(), page_labels: self.page_labels.clone(), embedded_files: self.embedded_files.clone(), output_intents: self.output_intents.clone(), acro_form: self.acro_form.clone(), optional_content: self.optional_content.clone(), page_layout: self.page_layout.clone(), page_mode: self.page_mode.clone(), viewer_preferences: self.viewer_preferences.clone(), open_action: self.open_action.clone(), language: self.language.clone(), mark_info: self.mark_info.clone(), metadata: self.metadata.clone(), document_id: self.document_id.clone(), encryption: self.encryption.clone(), info: self.info.clone(), catalog_extra: self.catalog_extra.clone(), objects: self.objects.clone(), trailer: self.trailer.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: PdfSnapshot) -> Self {
        Self { schema: snapshot.schema, declared_version: snapshot.declared_version, pages: snapshot.pages, fonts: snapshot.fonts, images: snapshot.images, forms: snapshot.forms, ext_g_states: snapshot.ext_g_states, shadings: snapshot.shadings, patterns: snapshot.patterns, color_spaces: snapshot.color_spaces, properties: snapshot.properties, outlines: snapshot.outlines, named_destinations: snapshot.named_destinations, page_labels: snapshot.page_labels, embedded_files: snapshot.embedded_files, output_intents: snapshot.output_intents, acro_form: snapshot.acro_form, optional_content: snapshot.optional_content, page_layout: snapshot.page_layout, page_mode: snapshot.page_mode, viewer_preferences: snapshot.viewer_preferences, open_action: snapshot.open_action, language: snapshot.language, mark_info: snapshot.mark_info, metadata: snapshot.metadata, document_id: snapshot.document_id, encryption: snapshot.encryption, info: snapshot.info, catalog_extra: snapshot.catalog_extra, objects: snapshot.objects, trailer: snapshot.trailer }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: PdfSnapshot) {
        self.schema = snapshot.schema;
        self.declared_version = snapshot.declared_version;
        self.pages = snapshot.pages;
        self.fonts = snapshot.fonts;
        self.images = snapshot.images;
        self.forms = snapshot.forms;
        self.ext_g_states = snapshot.ext_g_states;
        self.shadings = snapshot.shadings;
        self.patterns = snapshot.patterns;
        self.color_spaces = snapshot.color_spaces;
        self.properties = snapshot.properties;
        self.outlines = snapshot.outlines;
        self.named_destinations = snapshot.named_destinations;
        self.page_labels = snapshot.page_labels;
        self.embedded_files = snapshot.embedded_files;
        self.output_intents = snapshot.output_intents;
        self.acro_form = snapshot.acro_form;
        self.optional_content = snapshot.optional_content;
        self.page_layout = snapshot.page_layout;
        self.page_mode = snapshot.page_mode;
        self.viewer_preferences = snapshot.viewer_preferences;
        self.open_action = snapshot.open_action;
        self.language = snapshot.language;
        self.mark_info = snapshot.mark_info;
        self.metadata = snapshot.metadata;
        self.document_id = snapshot.document_id;
        self.encryption = snapshot.encryption;
        self.info = snapshot.info;
        self.catalog_extra = snapshot.catalog_extra;
        self.objects = snapshot.objects;
        self.trailer = snapshot.trailer;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pdf_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.pdf.1.7",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_7::subsets::base::schema::mutations::{apply_pdf_mutation, InsertPage, PdfMutation, SetInfo};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.pdf.1.7` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PdfBuilderConstruction {
        snapshot: PdfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl PdfBuilderConstruction {
        /// ➕ Typed construction: appends a page (the analyzer→builder round-trip acceptance test's
        /// primary entry point -- requirement #8's `InsertPage`, exposed ergonomically).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_page(self, page: PdfPage) -> Self {
            let index = self.snapshot.pages.len();
            let (next, _diff) = self.mutate(PdfMutation::InsertPage(InsertPage { index, page }));
            next
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_info(self, info: PdfInfo) -> Self {
            let (next, _diff) = self.mutate(PdfMutation::SetInfo(SetInfo { info }));
            next
        }
    }

    impl ArtifactBuilder for PdfBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;
        fn empty() -> Self {
            Self { snapshot: PdfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.pdf.1.7` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PdfParts {
        pub snapshot: Option<PdfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.7/🧱️base) sources.
    pub struct PdfAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };

        /// 🔍️ Real sniff (requirement #9): inspects `%PDF-` magic + version probe via
        /// `engine::sniff_pdf`, does not discard its argument.
        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => match crate::standards::v1_7::subsets::base::io::sniff_pdf(bytes) {
                    Some(_version) => IoConfidence::High,
                    None => IoConfidence::Low,
                },
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(10).collect();
                    let magic: Vec<u8> = (0..hex.len().min(10)).step_by(2).filter_map(|i| hex.get(i..i + 2)).filter_map(|h| u8::from_str_radix(h, 16).ok()).collect();
                    match crate::standards::v1_7::subsets::base::io::sniff_pdf(&magic) {
                        Some(_) => IoConfidence::Medium,
                        None => IoConfidence::Low,
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PdfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.as_bytes().starts_with(b"%PDF-") {
                        crate::standards::v1_7::subsets::base::io::decode_pdf(text.as_bytes()).map_err(|error| format!("{error:?}"))
                    } else {
                        <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match if crate::standards::v1_7::subsets::base::io::sniff_pdf(bytes).is_some() {
                        crate::standards::v1_7::subsets::base::io::decode_pdf(bytes).map_err(|error| format!("{error:?}"))
                    } else {
                        <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err));
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

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfBuilderFacets {
        construction: PdfBuilderConstruction,
        analysis: PdfAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PdfComposerComposition,
    }
    builder: PdfBuilder,
    analyzer: PdfAnalyzer,
    composer: PdfComposer,
);
//#endregion 🧬️DerivedArtifactFacets
