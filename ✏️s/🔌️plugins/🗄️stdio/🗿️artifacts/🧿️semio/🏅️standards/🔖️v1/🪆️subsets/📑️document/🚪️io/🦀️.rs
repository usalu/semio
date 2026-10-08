//! 🚪️ IO — 🚧 scaffolded by W1b: structure only. Registration flows through
//! 🎹️composer::register (matching the repo-wide convention — see gif's own io leaf doc comment).
//! W4 adds the real import/export leaves under 📥️import/🧩️deserializers and
//! 📤️export/🧵️serializers.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    #[cfg(feature = "conversion-document")]
    use super::super::export::serializers::artifacts::docx::v_ecma_376::any::SemioDocumentToDocx;
    #[cfg(feature = "conversion-document")]
    use super::super::export::serializers::artifacts::md::v_commonmark::any::SemioDocumentToMd;
    #[cfg(feature = "conversion-document")]
    use super::super::export::serializers::artifacts::pdf::v1_7::any::SemioDocumentToPdf;
    #[cfg(feature = "conversion-document")]
    use super::super::export::serializers::artifacts::txt::v_utf_8::any::SemioDocumentToTxt;
    #[cfg(feature = "conversion-document")]
    use super::super::import::deserializers::artifacts::docx::v_ecma_376::any::SemioDocumentFromDocx;
    #[cfg(feature = "conversion-document")]
    use super::super::import::deserializers::artifacts::md::v_commonmark::any::SemioDocumentFromMd;
    #[cfg(feature = "conversion-document")]
    use super::super::import::deserializers::artifacts::pdf::v1_7::any::SemioDocumentFromPdf;
    #[cfg(feature = "conversion-document")]
    use super::super::import::deserializers::artifacts::txt::v_utf_8::any::SemioDocumentFromTxt;
    use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, SemioDocumentSnapshot};
    use crate::standards::v1::subsets::document::io::SemioDocumentAnalyzer;
    #[cfg(feature = "conversion-document")]
    use semio_framework_plugin::{deserializer_entry_of, io::register_composer_entries, serializer_entry_of, io::ComposerEntry};
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

    //#region 🔖️Composer
    pub struct SemioDocumentComposerComposition;

    impl ArtifactComposition for SemioDocumentComposerComposition {
        type Snapshot = SemioDocumentSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
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
            let analysis = SemioDocumentAnalyzer::analyze(&native);
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
    pub fn check_document_referential_integrity(snapshot: &SemioDocumentSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic> {
        let mut diagnostics = Vec::new();
        let known_images: std::collections::HashSet<&str> = snapshot.images.iter().map(|i| i.id.as_str()).collect();
        let known_styles: std::collections::HashSet<&str> = snapshot.styles.iter().map(|s| s.id.as_str()).collect();

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn walk(blocks: &[DocBlock], known_images: &std::collections::HashSet<&str>, known_styles: &std::collections::HashSet<&str>, out: &mut Vec<semio_framework_diagnostic::Diagnostic>) {
            for block in blocks {
                match block {
                    DocBlock::Paragraph { style_id: Some(id), .. } | DocBlock::Heading { style_id: Some(id), .. } if !known_styles.contains(id.as_str()) => {
                        out.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_document.unresolved-style-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioDocumentValidator: block references unknown style id {id:?}")));
                    }
                    DocBlock::Image { image_id, .. } if !known_images.contains(image_id.as_str()) => {
                        out.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_document.unresolved-image-id", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioDocumentValidator: Image block references unknown image id {image_id:?}")));
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
                    diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_document.based-on-cycle", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioDocumentValidator: style {:?} has a based_on cycle through {cursor:?}", style.id)));
                    break;
                }
                match snapshot.styles.iter().find(|s| s.id == cursor) {
                    Some(next) => match &next.based_on {
                        Some(v) => cursor = v.clone(),
                        None => break,
                    },
                    None => {
                        diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.semio_document.unresolved-based-on", semio_framework_diagnostic::TextSpan::at(1, 1), format!("SemioDocumentValidator: style {:?} has based_on {cursor:?} which does not resolve", style.id)));
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
        async fn validate(payload: &IoPayload) -> Vec<semio_framework_diagnostic::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_document_referential_integrity(&snapshot),
                None => vec![semio_framework_diagnostic::Diagnostic::error("stdio.semio_document.validate-decode-failed", semio_framework_diagnostic::TextSpan::at(1, 1), "SemioDocumentValidator: payload did not decode as a SemioDocumentSnapshot".to_string())],
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
    #[cfg(feature = "conversion-document")]
    static IO_ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    #[cfg(feature = "conversion-document")]
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
        ::semio_framework_schema_registry::register_artifact_schema_descriptor(crate::standards::v1::subsets::document::schema::semio_document_artifact_schema_descriptor()).expect("schema descriptor publication");
        semio_framework_plugin::io::register_native_document_codec(semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("document") }, store::ArtifactCodec::bare::<SemioDocumentSnapshot, crate::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation>(
            crate::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        #[cfg(feature = "conversion-document")]
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 🧾️ The declarative twin of [`register`]: this subset's schema, document codec, `SubsetValidator`, composers
    /// (those writing semio, [`crate::semio_written`]) and inference descriptor as rows of [`crate::declaration`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn declare(builder: semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady>) -> semio_framework_plugin::app::ArtifactDeclarationBuilder<semio_framework_plugin::app::DeclarationReady> {
        let builder = builder
            .schemas([crate::standards::v1::subsets::document::schema::semio_document_artifact_schema_descriptor()])
            .document_codec_bare::<SemioDocumentSnapshot, crate::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation>(crate::standards::v1::subsets::document::schema::snapshot::STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("document") })
            .subset_validators(std::slice::from_ref(validator_entry()))
            .inferences([crate::standards::v1::subsets::document::schema::inferences::semio_document_artifact_inference_descriptor()]);
        #[cfg(feature = "conversion-document")]
        let builder = {
            static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
            builder.composers(crate::semio_written(io_entries(), &COMPOSERS))
        };
        builder
    }

    /// 💡️ Registers `s.stdio.semio.document.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::semio_framework_schema_registry::register_artifact_inference_descriptor(crate::standards::v1::subsets::document::schema::inferences::semio_document_artifact_inference_descriptor()).expect("schema descriptor publication");
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(all(test, feature = "conversion-document"))]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::standards::v1::subsets::document::schema::diff::SemioDocumentDiff;
    #[cfg(test)]
    use crate::standards::v1::subsets::document::schema::mutations::insert_style;
    use crate::standards::v1::subsets::document::schema::mutations::{SemioDocumentMutation};
    use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocStyle, SemioDocumentSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioDocumentBuilderConstruction {
        snapshot: SemioDocumentSnapshot,
    }

    impl SemioDocumentBuilderConstruction {
        /// 🎨️ Fluent: appends a named style.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_style(mut self, style: DocStyle) -> Self {
            self.snapshot.styles.push(style);
            self
        }
        /// 🖼️ Fluent: appends a named image.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_image(mut self, image: DocImage) -> Self {
            self.snapshot.images.push(image);
            self
        }
        /// 🧱️ Fluent: appends a top-level block.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_block(mut self, block: DocBlock) -> Self {
            self.snapshot.blocks.push(block);
            self
        }
    }

    impl ArtifactBuilder for SemioDocumentBuilderConstruction {
        type Snapshot = SemioDocumentSnapshot;
        type Mutation = SemioDocumentMutation;
        type Diff = SemioDocumentDiff;
        fn empty() -> Self {
            Self { snapshot: SemioDocumentSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <SemioDocumentMutation as protocol::Mutation<SemioDocumentSnapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => (Self { snapshot, ..self }, outcome),
                Err(error) => (self, protocol::MutationOutcome::fatal(error.code, error.message, error.target)),
            }
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1::subsets::document::schema::snapshot::{SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioDocumentParts {
        pub snapshot: Option<SemioDocumentSnapshot>,
    }

    pub struct SemioDocumentAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioDocumentAnalyzerAnalysis {
        type Parts = SemioDocumentParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA) {
                        semio_framework_plugin::io::Confidence::High
                    } else {
                        semio_framework_plugin::io::Confidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioDocumentParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("stdio.analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioDocumentBuilderFacets {
        construction: SemioDocumentBuilderConstruction,
        analysis: SemioDocumentAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::document::io::derived_composition::SemioDocumentComposerComposition,
    }
    builder: SemioDocumentBuilder,
    analyzer: SemioDocumentAnalyzer,
    composer: SemioDocumentComposer,
);
