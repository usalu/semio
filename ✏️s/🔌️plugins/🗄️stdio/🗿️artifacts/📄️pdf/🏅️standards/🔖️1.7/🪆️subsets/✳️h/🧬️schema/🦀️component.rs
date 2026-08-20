//! 🧬️ PdfSnapshot schema (1.7/✳️h) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.pdf.1.7` schema id). PDF/H (AIIM/ASTM PDF Healthcare Best Practices Guide) is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️h/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition. Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
    use crate::artifacts::pdf::standards::v1_7::subsets::h::schema::check_h_conformance;
    use dsl::Diagnostic;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfHBuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfHBuilderConstruction {
        pub async fn new() -> Self {
            Self { snapshot: PdfSnapshot::default() }
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

    impl Default for PdfHBuilderConstruction {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ArtifactBuilder for PdfHBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        async fn empty() -> Self {
            Self::new().await
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await)
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await)
        }

        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff.await)
        }

        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot).await?;
            Ok(self)
        }

        /// ✅ Always `Ok` -- `check_h_conformance` is ALL-SOFT, so the hard-filter below is never
        /// non-empty. Still runs the real check (not skipped) so a future hard check added here would
        /// correctly start gating without any other code change.
        async fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_h_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal)).collect();
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
        async fn build_always_succeeds() {
            let snapshot = PdfHBuilderConstruction::new().add_page(PdfPage::new(200.0, 200.0)).build().expect("PDF/H build() never fails");
            assert_eq!(snapshot.pages.len(), 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn set_info_clears_title_author_advisory() {
            let snapshot = PdfHBuilderConstruction::new().set_info(PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..PdfInfo::default() }).build().unwrap();
            let diagnostics = check_h_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != crate::artifacts::pdf::standards::v1_7::subsets::h::schema::CODE_INFO_TITLE_OR_AUTHOR), "got {diagnostics:?}");
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
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("h") };

    //#region 🔖️Conformance
    pub const CODE_INFO_TITLE_OR_AUTHOR: &str = "stdio.pdf.h.missing-info-title-or-author";
    pub const CODE_JAVASCRIPT: &str = "stdio.pdf.h.javascript-action";
    pub const CODE_LAUNCH: &str = "stdio.pdf.h.launch-action";
    pub const CODE_SIGNATURE_FIELD: &str = "stdio.pdf.h.missing-signature-field";
    pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.h.font-not-embedded";

    async fn dict_name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
        dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
    }

    async fn resolve_ref<'a>(objects: &'a [PdfIndirectObject], r: ObjRef) -> Option<&'a PdfObject> {
        objects.iter().find(|o| o.id == r).map(|o| &o.value)
    }

    async fn resolve_item<'a>(objects: &'a [PdfIndirectObject], item: &'a PdfObject) -> Option<&'a PdfObject> {
        match item {
            PdfObject::Ref(r) => resolve_ref(objects, *r).await,
            other => Some(other),
        }
    }

    async fn find_catalog(objects: &[PdfIndirectObject]) -> Option<&PdfObject> {
        objects.iter().find(|o| semio_framework_plugin::resolve_ready(o.value.as_dict()).map(|d| dict_name(d, "Type") == Some("Catalog")).unwrap_or(false)).map(|o| &o.value)
    }

    async fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
        objects.iter().filter(|o| semio_framework_plugin::resolve_ready(o.value.as_dict()).map(|d| dict_name(d, "S") == Some(subtype)).unwrap_or(false)).map(|o| o.id).collect()
    }

    async fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
        objects.iter().filter(|o| !already.contains(&o.id) && semio_framework_plugin::resolve_ready(o.value.as_dict()).map(|d| d.iter().any(|e| e.key == "JS")).unwrap_or(false)).map(|o| o.id).collect()
    }

    /// ✍️ Real check: `/Root/AcroForm/Fields` contains a resolved entry with `/FT /Sig`.
    async fn has_signature_field(objects: &[PdfIndirectObject]) -> bool {
        let Some(catalog) = find_catalog(objects).await else { return false };
        let Some(acroform) = catalog.dict_get("AcroForm").await.and_then(|v| resolve_item(objects, v)) else { return false };
        let Some(fields) = acroform.dict_get("Fields").and_then(|v| v.as_array()) else { return false };
        fields.iter().any(|item| semio_framework_plugin::resolve_ready(resolve_item(objects, item)).and_then(|f| f.as_dict()).map(|d| dict_name(d, "FT") == Some("Sig")).unwrap_or(false))
    }

    async fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
        resolve_ref(objects, desc_ref).await.and_then(|o| o.as_dict()).map(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3")).unwrap_or(false)
    }

    async fn non_embedded_fonts(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        let mut out = Vec::new();
        for o in objects {
            let Some(d) = o.value.as_dict().await else { continue };
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
                        semio_framework_plugin::resolve_ready(resolve_item(objects, item)).and_then(|desc| desc.as_dict()).and_then(|dd| dd.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref())).map(|r| descriptor_has_embedded_file(objects, r)).unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            if !direct && !via_descendants {
                out.push(o.id);
            }
        }
        out
    }

    async fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real AIIM/ASTM PDF Healthcare Best Practices Guide (2008) checks against one
    /// already-decoded `PdfSnapshot`. ALL-SOFT by design (industry best-practice guide, never an ISO
    /// standard, no enforcement mechanism) -- never returns a `Severity::Error`/`Fatal` diagnostic.
    /// Shared single source of truth used by `PdfHComposer` and `PdfHValidator` (both pass-through,
    /// per the roster's "never hard-gates").
    pub async fn check_h_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let objects = &snapshot.objects;
        let mut out = Vec::new();
        let title_ok = snapshot.info.title.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
        let author_ok = snapshot.info.author.as_deref().map(|s| !s.is_empty()).unwrap_or(false);
        if !title_ok || !author_ok {
            out.push(soft(CODE_INFO_TITLE_OR_AUTHOR, "document Info.title and/or Info.author is absent or empty -- the PDF Healthcare Best Practices Guide recommends both be populated".into()));
        }
        let js_actions = scan_action_subtype(objects, "JavaScript");
        for r in &js_actions {
            out.push(soft(CODE_JAVASCRIPT, format!("object {} {} R is an /S /JavaScript action -- the PDF Healthcare Best Practices Guide discourages embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_js_key_only(objects, &js_actions) {
            out.push(soft(CODE_JAVASCRIPT, format!("object {} {} R carries a /JS key -- the PDF Healthcare Best Practices Guide discourages embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_action_subtype(objects, "Launch") {
            out.push(soft(CODE_LAUNCH, format!("object {} {} R is an /S /Launch action -- the PDF Healthcare Best Practices Guide discourages launch actions", r.num, r.gen)));
        }
        if !has_signature_field(objects) {
            out.push(soft(CODE_SIGNATURE_FIELD, "no /AcroForm field with /FT /Sig found -- the PDF Healthcare Best Practices Guide recommends a signature flow".into()));
        }
        for r in non_embedded_fonts(objects) {
            out.push(soft(CODE_FONT_NOT_EMBEDDED, format!("font object {} {} R has no FontFile/FontFile2/FontFile3 reachable from its FontDescriptor -- the Guide recommends embedded fonts", r.num, r.gen)));
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    pub struct PdfHAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfHAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source).await
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources).await;
            let mut diagnostics = inner.diagnostics.clone();
            // ℹ️ ALL-SOFT profile -- confidence is never downgraded by check_h_conformance's output
            // since it never returns Error/Fatal.
            let confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                diagnostics.extend(check_h_conformance(snapshot));
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfInfo;

        #[semio_framework_async_macros::async_test]
        async fn empty_snapshot_reports_only_soft_findings() {
            let snapshot = PdfSnapshot::default();
            let diagnostics = check_h_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.severity != Severity::Error && d.severity != Severity::Fatal), "PDF/H must never emit a hard diagnostic: got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_INFO_TITLE_OR_AUTHOR));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_SIGNATURE_FIELD));
        }

        #[semio_framework_async_macros::async_test]
        async fn title_and_author_present_clears_that_finding() {
            let snapshot = PdfSnapshot { info: PdfInfo { title: Some("A Chart".into()), author: Some("Dr. X".into()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
            let diagnostics = check_h_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_INFO_TITLE_OR_AUTHOR), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn javascript_action_is_soft_never_hard() {
            let objects = vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }]) }];
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_h_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn signature_field_present_clears_that_finding() {
            let objects = vec![
                PdfIndirectObject {
                    id: ObjRef { num: 1, gen: 0 },
                    value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "AcroForm".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) }]),
                },
                PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Fields".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 3, gen: 0 })]) }]) },
                PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "FT".into(), value: PdfObject::Name("Sig".into()) }]) },
            ];
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_h_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_SIGNATURE_FIELD), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfHBuilderFacets {
        construction: PdfHBuilderConstruction,
        analysis: PdfHAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfHComposerComposition,
    }
    builder: PdfHBuilder,
    analyzer: PdfHAnalyzer,
    composer: PdfHComposer,
);
//#endregion 🧬️DerivedArtifactFacets
