//! 🧬️ PdfSnapshot schema (1.7/✳️x) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.pdf.1.7` schema id). PDF/X-4 (ISO 15930-7:2010) is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️x/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition. Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfInfo, PdfObject, PdfPage, PdfSnapshot};
    use crate::artifacts::pdf::standards::v1_7::subsets::x::schema::check_x_conformance;
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Seed
    /// 🌱️ Seeds a fresh snapshot with a real `/Root /OutputIntents` → `OutputIntent` object pair
    /// (`/S /GTS_PDFX` + `/DestOutputProfile`, ISO 15930-7's own conformance marker).
    async fn seeded_snapshot(output_condition: String) -> PdfSnapshot {
        let objects = vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) }]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 2, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                    PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFX".into()) },
                    PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(output_condition.into_bytes()) },
                    PdfDictEntry { key: "DestOutputProfile".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
                ]),
            },
        ];
        PdfSnapshot { objects, ..PdfSnapshot::default() }
    }
    //#endregion 🔖️Seed

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfXBuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfXBuilderConstruction {
        /// ➕ The recommended entry point: REQUIRES an output-condition identifier up front.
        pub async fn new(output_condition: impl Into<String>) -> Self {
            Self { snapshot: seeded_snapshot(output_condition.into()) }
        }

        pub async fn add_page(mut self, page: PdfPage) -> Self {
            let index = self.snapshot.pages.len();
            apply_pdf_mutation(&mut self.snapshot, &PdfMutation::InsertPage { index, page });
            self
        }

        pub async fn set_info(mut self, info: PdfInfo) -> Self {
            apply_pdf_mutation(&mut self.snapshot, &PdfMutation::SetInfo { info });
            self
        }
    }

    impl ArtifactBuilder for PdfXBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        async fn empty() -> Self {
            Self::new("FOGRA39")
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        async fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_x_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn new_requires_output_condition_and_builds_clean() {
            let snapshot = PdfXBuilderConstruction::new("FOGRA39").add_page(PdfPage::new(200.0, 200.0)).set_info(PdfInfo { title: Some("An X Test".into()), ..PdfInfo::default() }).build().expect("conforming construction must build");
            assert_eq!(snapshot.pages.len(), 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let violating = PdfIndirectObject {
                id: ObjRef { num: 99, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                    PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                    PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                    PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                    PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
                ]),
            };
            let mut snapshot = PdfXBuilderConstruction::new("FOGRA39").add_page(PdfPage::new(100.0, 100.0)).build().unwrap();
            snapshot.objects.push(violating);
            let (mutated, _diff) = PdfXBuilderConstruction::from_snapshot(PdfSnapshot::default()).mutate(PdfMutation::SetSnapshot { snapshot });
            let err = mutated.build().expect_err("an /Encrypt dict must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::x::schema::CODE_ENCRYPT));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfAnalyzer as PdfAnyAnalyzer;
    pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("x") };

    //#region 🔖️Conformance
    pub const CODE_ENCRYPT: &str = "stdio.pdf.x.encrypt-present";
    pub const CODE_OUTPUT_INTENT: &str = "stdio.pdf.x.missing-output-intent";
    pub const CODE_TRIM_OR_ART_BOX: &str = "stdio.pdf.x.missing-trim-or-art-box";
    pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.x.font-not-embedded";
    pub const CODE_JAVASCRIPT: &str = "stdio.pdf.x.javascript-action";
    pub const CODE_LAUNCH: &str = "stdio.pdf.x.launch-action";
    pub const CODE_MOVIE_OR_SOUND: &str = "stdio.pdf.x.movie-or-sound-annotation";

    pub(crate) async fn dict_name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
        dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
    }

    async fn resolve_ref<'a>(objects: &'a [PdfIndirectObject], r: ObjRef) -> Option<&'a PdfObject> {
        objects.iter().find(|o| o.id == r).map(|o| &o.value)
    }

    async fn resolve_item<'a>(objects: &'a [PdfIndirectObject], item: &'a PdfObject) -> Option<&'a PdfObject> {
        match item {
            PdfObject::Ref(r) => resolve_ref(objects, *r),
            other => Some(other),
        }
    }

    async fn find_catalog(objects: &[PdfIndirectObject]) -> Option<&PdfObject> {
        objects.iter().find(|o| o.value.as_dict().map(|d| dict_name(d, "Type") == Some("Catalog")).unwrap_or(false)).map(|o| &o.value)
    }

    /// 🔒️ Real scan: does any retained object look like a Standard Security Handler encryption
    /// dictionary (`/Filter /Standard` + `/V`/`/R`/`/O`/`/U`)?
    pub(crate) async fn scan_encryption(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects
            .iter()
            .filter(|o| {
                let Some(d) = o.value.as_dict() else { return false };
                dict_name(d, "Filter") == Some("Standard") && d.iter().any(|e| e.key == "V") && d.iter().any(|e| e.key == "R") && d.iter().any(|e| e.key == "O") && d.iter().any(|e| e.key == "U")
            })
            .map(|o| o.id)
            .collect()
    }

    /// 📜️ Real scan for `/S /<subtype>` action dictionaries anywhere in the retained object graph.
    pub(crate) async fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().map(|d| dict_name(d, "S") == Some(subtype)).unwrap_or(false)).map(|o| o.id).collect()
    }

    /// 📜️ Real scan for a bare `/JS` key not already caught by `/S /JavaScript`.
    pub(crate) async fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
        objects.iter().filter(|o| !already.contains(&o.id) && o.value.as_dict().map(|d| d.iter().any(|e| e.key == "JS")).unwrap_or(false)).map(|o| o.id).collect()
    }

    /// 🏳️ Real check: `/Root`'s `/OutputIntents` array contains an intent with `/S /GTS_PDFX` AND a
    /// `/DestOutputProfile` key (the ICC profile PDF/X-4 requires alongside the marker).
    async fn has_pdfx_output_intent(objects: &[PdfIndirectObject]) -> bool {
        let Some(catalog) = find_catalog(objects) else { return false };
        let Some(intents) = catalog.dict_get("OutputIntents").and_then(|v| v.as_array()) else { return false };
        intents.iter().any(|item| resolve_item(objects, item).and_then(|o| o.as_dict()).map(|d| dict_name(d, "S") == Some("GTS_PDFX") && d.iter().any(|e| e.key == "DestOutputProfile")).unwrap_or(false))
    }

    /// 📄️ Real scan: every `/Type /Page` object carries a `/TrimBox` or `/ArtBox` key. Returns the
    /// refs of pages missing BOTH.
    async fn pages_missing_trim_or_art_box(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects
            .iter()
            .filter(|o| {
                let Some(d) = o.value.as_dict() else { return false };
                dict_name(d, "Type") == Some("Page") && !d.iter().any(|e| e.key == "TrimBox") && !d.iter().any(|e| e.key == "ArtBox")
            })
            .map(|o| o.id)
            .collect()
    }

    async fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
        resolve_ref(objects, desc_ref).and_then(|o| o.as_dict()).map(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3")).unwrap_or(false)
    }

    /// 🔤️ Real check: every `/Type /Font` object (simple or `/DescendantFonts` composite) resolves
    /// to a `/FontDescriptor` carrying an embedded font program.
    pub(crate) async fn non_embedded_fonts(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        let mut out = Vec::new();
        for o in objects {
            let Some(d) = o.value.as_dict() else { continue };
            if dict_name(d, "Type") != Some("Font") {
                continue;
            }
            let direct = d.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref()).map(|r| descriptor_has_embedded_file(objects, r)).unwrap_or(false);
            let via_descendants = d
                .iter()
                .find(|e| e.key == "DescendantFonts")
                .and_then(|e| e.value.as_array())
                .map(|arr| {
                    arr.iter().any(|item| {
                        resolve_item(objects, item).and_then(|desc| desc.as_dict()).and_then(|dd| dd.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref())).map(|r| descriptor_has_embedded_file(objects, r)).unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            if !direct && !via_descendants {
                out.push(o.id);
            }
        }
        out
    }

    /// 🎬️ Real scan: `/Subtype /Movie` or `/Subtype /Sound` annotation dicts anywhere in the graph.
    async fn movie_or_sound_annotations(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().map(|d| matches!(dict_name(d, "Subtype"), Some("Movie") | Some("Sound"))).unwrap_or(false)).map(|o| o.id).collect()
    }

    async fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    async fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 15930-7:2010 (PDF/X-4) conformance checks against one already-decoded
    /// `PdfSnapshot`. Shared single source of truth: `PdfXComposer::compose`, `PdfXBuilder::build`,
    /// `PdfXValidator::validate`, and (layered on top) `✳️vt`'s own conformance fn all call this.
    pub async fn check_x_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let objects = &snapshot.objects;
        let mut out = Vec::new();
        for r in scan_encryption(objects) {
            out.push(hard(CODE_ENCRYPT, format!("object {} {} R looks like a Standard Security Handler encryption dictionary -- PDF/X forbids /Encrypt", r.num, r.gen)));
        }
        if !has_pdfx_output_intent(objects) {
            out.push(hard(CODE_OUTPUT_INTENT, "no OutputIntent with /S /GTS_PDFX and /DestOutputProfile reachable from /Root/OutputIntents -- ISO 15930-7 requires it".into()));
        }
        for r in pages_missing_trim_or_art_box(objects) {
            out.push(hard(CODE_TRIM_OR_ART_BOX, format!("/Type /Page object {} {} R has neither /TrimBox nor /ArtBox -- ISO 15930-7 requires one", r.num, r.gen)));
        }
        for r in non_embedded_fonts(objects) {
            out.push(soft(CODE_FONT_NOT_EMBEDDED, format!("font object {} {} R has no FontFile/FontFile2/FontFile3 reachable from its FontDescriptor -- PDF/X requires embedded fonts", r.num, r.gen)));
        }
        let js_actions = scan_action_subtype(objects, "JavaScript");
        for r in &js_actions {
            out.push(soft(CODE_JAVASCRIPT, format!("object {} {} R is an /S /JavaScript action -- PDF/X forbids embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_js_key_only(objects, &js_actions) {
            out.push(soft(CODE_JAVASCRIPT, format!("object {} {} R carries a /JS key -- PDF/X forbids embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_action_subtype(objects, "Launch") {
            out.push(soft(CODE_LAUNCH, format!("object {} {} R is an /S /Launch action -- PDF/X forbids launch actions", r.num, r.gen)));
        }
        for r in movie_or_sound_annotations(objects) {
            out.push(soft(CODE_MOVIE_OR_SOUND, format!("annotation object {} {} R is /Subtype /Movie or /Sound -- PDF/X-4 discourages non-static media", r.num, r.gen)));
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.7/✳️x): delegates the real parse to the ✳️any subset's analyzer,
    /// then folds real PDF/X-4 conformance diagnostics on top.
    pub struct PdfXAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfXAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source)
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_x_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = IoConfidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfDictEntry;

        async fn conforming_objects() -> Vec<PdfIndirectObject> {
            vec![
                PdfIndirectObject {
                    id: ObjRef { num: 1, gen: 0 },
                    value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) }]),
                },
                PdfIndirectObject {
                    id: ObjRef { num: 2, gen: 0 },
                    value: PdfObject::Dict(vec![
                        PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                        PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFX".into()) },
                        PdfDictEntry { key: "DestOutputProfile".into(), value: PdfObject::Ref(ObjRef { num: 9, gen: 0 }) },
                    ]),
                },
                PdfIndirectObject {
                    id: ObjRef { num: 3, gen: 0 },
                    value: PdfObject::Dict(vec![
                        PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Page".into()) },
                        PdfDictEntry { key: "TrimBox".into(), value: PdfObject::Array(vec![PdfObject::Int(0), PdfObject::Int(0), PdfObject::Int(100), PdfObject::Int(100)]) },
                    ]),
                },
            ]
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_snapshot_has_no_hard_diagnostics() {
            let snapshot = PdfSnapshot { objects: conforming_objects(), ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_output_intent_is_hard() {
            let snapshot = PdfSnapshot::default();
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_OUTPUT_INTENT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn page_missing_trim_or_art_box_is_hard() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Page".into()) }]) });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TRIM_OR_ART_BOX && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn encryption_dict_shape_is_hard() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 5, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                    PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                    PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                    PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                    PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn non_embedded_font_is_soft() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 6, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) }, PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Type1".into()) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FONT_NOT_EMBEDDED && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn javascript_action_is_soft() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject { id: ObjRef { num: 7, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }]) });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn movie_annotation_is_soft() {
            let mut objects = conforming_objects();
            objects.push(PdfIndirectObject { id: ObjRef { num: 8, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Movie".into()) }]) });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_x_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MOVIE_OR_SOUND && d.severity == Severity::Warning), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfXBuilderFacets {
        construction: PdfXBuilderConstruction,
        analysis: PdfXAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfXComposerComposition,
    }
    builder: PdfXBuilder,
    analyzer: PdfXAnalyzer,
    composer: PdfXComposer,
);
//#endregion 🧬️DerivedArtifactFacets
