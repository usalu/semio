//! 🚪️ IO stdio.pptx (ecma-376/🧱️base) — registration flows through `pptx::declaration()`
//! (`🗄️stdio/🗿️artifacts/🎞️pptx/🦀️.rs`), not a side-effecting `register()`; `⚙️engine`
//! dissolved (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — `PptxEngine` (zero
//! construction sites) deleted outright; its orphaned `register()`/`register_artifact_inferences()`/
//! `register_pilot_languages()` (zero callers, superseded by `declaration()`) deleted outright too;
//! `PptxError` + shared OPC/XML constants + the minimal slideMaster/slideLayout/theme boilerplate
//! below (used by both `📥️import/🧩️deserializers` and `📤️export/🧵️serializers`); `io_registry`
//! moved here from `⚙️engine`, live (`pptx::declaration()`'s `.composers(...)` and this artifact's
//! own root `io_registry` both reach it).
use crate::standards::v_ecma_376::subsets::base::schema::{vocabulary::*,refusal::*};


/// 📐️ Minimal-but-schema-shaped `slideMaster1.xml` — synthesized once when a package has no
/// existing slide master, never regenerated over a decoded one.
pub const MINIMAL_SLIDE_MASTER_XML: &str = concat!(
    r#"<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
    "<p:cSld><p:spTree>",
    r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
    "</p:spTree></p:cSld>",
    r#"<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>"#,
    r#"<p:sldLayoutIdLst><p:sldLayoutId id="2147483649" r:id="rId1"/></p:sldLayoutIdLst>"#,
    "</p:sldMaster>",
);

/// 📐️ Minimal-but-schema-shaped `slideLayout1.xml`.
pub const MINIMAL_SLIDE_LAYOUT_XML: &str = concat!(
    r#"<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="blank" preserve="1">"#,
    "<p:cSld><p:spTree>",
    r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/>"#,
    "</p:spTree></p:cSld>",
    "<p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>",
    "</p:sldLayout>",
);

/// 🎨️ Minimal-but-schema-shaped `theme1.xml` (all required color/font/format-scheme slots).
pub const MINIMAL_THEME_XML: &str = concat!(
    r#"<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Minimal">"#,
    "<a:themeElements>",
    r#"<a:clrScheme name="Minimal">"#,
    r#"<a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>"#,
    r#"<a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>"#,
    r#"<a:dk2><a:srgbClr val="1F497D"/></a:dk2>"#,
    r#"<a:lt2><a:srgbClr val="EEECE1"/></a:lt2>"#,
    r#"<a:accent1><a:srgbClr val="4F81BD"/></a:accent1>"#,
    r#"<a:accent2><a:srgbClr val="C0504D"/></a:accent2>"#,
    r#"<a:accent3><a:srgbClr val="9BBB59"/></a:accent3>"#,
    r#"<a:accent4><a:srgbClr val="8064A2"/></a:accent4>"#,
    r#"<a:accent5><a:srgbClr val="4BACC6"/></a:accent5>"#,
    r#"<a:accent6><a:srgbClr val="F79646"/></a:accent6>"#,
    r#"<a:hlink><a:srgbClr val="0000FF"/></a:hlink>"#,
    r#"<a:folHlink><a:srgbClr val="800080"/></a:folHlink>"#,
    "</a:clrScheme>",
    r#"<a:fontScheme name="Minimal"><a:majorFont><a:latin typeface="Calibri"/></a:majorFont><a:minorFont><a:latin typeface="Calibri"/></a:minorFont></a:fontScheme>"#,
    r#"<a:fmtScheme name="Minimal">"#,
    r#"<a:fillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:fillStyleLst>"#,
    r#"<a:lnStyleLst><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln></a:lnStyleLst>"#,
    r#"<a:effectStyleLst><a:effectStyle><a:effectLst/></a:effectStyle><a:effectStyle><a:effectLst/></a:effectStyle><a:effectStyle><a:effectLst/></a:effectStyle></a:effectStyleLst>"#,
    r#"<a:bgFillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:bgFillStyleLst>"#,
    "</a:fmtScheme>",
    "</a:themeElements>",
    "</a:theme>",
);
//#endregion 🔖️Constants

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_ecma_376::subsets::base::io::PptxAnalyzer;
    use crate::PptxSnapshot;
    use {semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };
    const DEP_ZIP: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

    pub struct PptxComposerComposition;

    impl ArtifactComposition for PptxComposerComposition {
        type Snapshot = PptxSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_ZIP]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            // 🌱 Every listed read dialect's payload is raw text/bytes that this artifact's own
            // analyzer already round-trips through `store::Document{Dsl,Pack}` -- including bytes
            // claiming a dependency's dialect, since (for a single-standard DAG-adjacent dependency
            // like binary) that payload IS the same byte/text shape `analyze` already accepts.
            let native: Vec<AnalyzeSource<'_>> = sources
                .iter()
                .filter(|s| s.dialect == DIALECT || s.dialect == DEP_ZIP)
                .map(|s| match &s.payload {
                    AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                    AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                })
                .collect();
            if native.is_empty() {
                return Err(ComposeError { message: "PptxComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = PptxAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "PptxComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::standards::v_ecma_376::subsets::base::io::PptxComposer as PptxRawAnyComposer;
    use crate::standards::v_ecma_376::subsets::strict::io::PptxStrictComposer;
    use crate::standards::v_ecma_376::subsets::transitional::io::PptxTransitionalComposer;
    use semio_framework_plugin::{composer_entry_of, io::ComposerEntry};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    // 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9
    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES.get_or_init(|| vec![composer_entry_of::<PptxRawAnyComposer>(), composer_entry_of::<PptxStrictComposer>(), composer_entry_of::<PptxTransitionalComposer>()]).as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::schema::snapshot::{PptxParagraph, PptxRun};
    use crate::{PptxDiff, PptxMutation, PptxSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.pptx` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PptxBuilderConstruction {
        snapshot: PptxSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for PptxBuilderConstruction {
        type Snapshot = PptxSnapshot;
        type Mutation = PptxMutation;
        type Diff = PptxDiff;
        fn empty() -> Self {
            Self { snapshot: PptxSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            match snapshot.presentation() {
                Ok(_) => Self { snapshot, diagnostics: Vec::new() },
                Err(error) => Self { snapshot, diagnostics: vec![semio_framework_diagnostic::Diagnostic::error("stdio.pptx.builder.presentation", semio_framework_diagnostic::TextSpan::at(1, 1), error)] },
            }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<PptxSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PptxSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let (next, diff) = store::apply_outcome(&self.snapshot, protocol::Mutation::diff(&mutation, &self.snapshot));
            self.snapshot = next;
            (Self::from_snapshot(self.snapshot), diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(Self::from_snapshot(self.snapshot))
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    /// 🧱️ Typed content constructors — build a presentation from slides of paragraphs/runs with
    /// basic formatting (bold/italic), the same shape as `docx::DocxBuilder`'s constructors.
    impl PptxBuilderConstruction {
        /// ➕️ Appends a new (initially empty) slide and makes it the active slide for `add_paragraph`.
        pub async fn add_slide(mut self) -> Self {
            if self.diagnostics.is_empty() {
                if let Err(error) = crate::standards::v_ecma_376::subsets::base::schema::construction::append_slide(&mut self.snapshot) {
                    self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.pptx.builder.add-slide", semio_framework_diagnostic::TextSpan::at(1, 1), error));
                }
            }
            self
        }

        /// ➕️ Appends a paragraph to the active slide's active `TextBox` shape (the most recently
        /// added one), creating a fresh `TextBox` shape first if the slide has none yet or its last
        /// shape isn't one.
        pub async fn add_paragraph(mut self, paragraph: PptxParagraph) -> Self {
            if self.diagnostics.is_empty() {
                if let Err(error) = crate::standards::v_ecma_376::subsets::base::schema::construction::append_paragraph(&mut self.snapshot, paragraph) {
                    self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.pptx.builder.add-paragraph", semio_framework_diagnostic::TextSpan::at(1, 1), error));
                }
            }
            self
        }

        /// ➕️ Appends a single-run plain-text paragraph to the active slide.
        pub async fn add_text_paragraph(self, text: impl Into<String>) -> Self {
            self.add_paragraph(PptxParagraph::text(text.into())).await
        }

        /// ➕️ Appends a paragraph made of the given runs (basic bold/italic formatting).
        pub async fn add_runs(self, runs: Vec<PptxRun>) -> Self {
            self.add_paragraph(PptxParagraph { runs }).await
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::PptxSnapshot;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.pptx` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PptxParts {
        pub snapshot: Option<PptxSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pptx` (ecma-376/🧱️base) sources.
    pub struct PptxAnalyzerAnalysis;

    impl ArtifactAnalysis for PptxAnalyzerAnalysis {
        type Parts = PptxParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pptx", standard: StandardId("ecma-376"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            // 🕵️ Real sniff: OPC-shaped bytes whose root officeDocument relationship resolves under
            // `ppt/` — disambiguates from docx/xlsx, which share the same zip magic and OPC shape.
            match source {
                AnalyzeSource::Binary(bytes) if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_pptx_bytes(bytes) => semio_framework_plugin::io::Confidence::High,
                AnalyzeSource::Binary(_) | AnalyzeSource::Text(_) => semio_framework_plugin::io::Confidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PptxParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PptxSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => {
                        let result = if crate::standards::v_ecma_376::subsets::base::io::import::deserializers::sniff_pptx_bytes(bytes) {
                            crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(bytes).map_err(|err| err.to_string())
                        } else {
                            <PptxSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|err| err.to_string())
                        };
                        match result {
                            Ok(snapshot) => parts.snapshot = Some(snapshot),
                            Err(err) => {
                                confidence = semio_framework_plugin::io::Confidence::Low;
                                diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err));
                            }
                        }
                    }
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec PptxBuilderFacets {
        construction: PptxBuilderConstruction,
        analysis: PptxAnalyzerAnalysis,
        composition: crate::standards::v_ecma_376::subsets::base::io::derived_composition::PptxComposerComposition,
    }
    builder: PptxBuilder,
    analyzer: PptxAnalyzer,
    composer: PptxComposer,
);
