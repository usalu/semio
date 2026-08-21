//! 🧬️ PdfSnapshot schema (1.7/✳️ua) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.pdf.1.7` schema id). PDF/UA-1 (ISO 14289-1:2014) is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️ua/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition. Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfInfo, PdfObject, PdfPage, PdfSnapshot};
    use crate::artifacts::pdf::standards::v1_7::subsets::ua::schema::check_ua_conformance;
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Seed
    /// 🌱️ Seeds a fresh snapshot with a real tagged-PDF Catalog: `/MarkInfo/Marked true`, a
    /// (minimal) `/StructTreeRoot`, `/Lang`, and `/ViewerPreferences/DisplayDocTitle true`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn seeded_snapshot(lang: String) -> PdfSnapshot {
        let objects = vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                    PdfDictEntry { key: "MarkInfo".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) },
                    PdfDictEntry { key: "StructTreeRoot".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
                    PdfDictEntry { key: "Lang".into(), value: PdfObject::Str(lang.into_bytes()) },
                    PdfDictEntry { key: "ViewerPreferences".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                ]),
            },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Marked".into(), value: PdfObject::Bool(true) }]) },
            PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("StructTreeRoot".into()) }]) },
            PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "DisplayDocTitle".into(), value: PdfObject::Bool(true) }]) },
        ];
        PdfSnapshot { objects, ..PdfSnapshot::default() }
    }
    //#endregion 🔖️Seed

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfUaBuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfUaBuilderConstruction {
        /// ➕ The recommended entry point: REQUIRES a language tag (e.g. `"en-US"`) up front.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(lang: impl Into<String>) -> Self {
            Self { snapshot: seeded_snapshot(lang.into()) }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_page(mut self, page: PdfPage) -> Self {
            let index = self.snapshot.pages.len();
            apply_pdf_mutation(&mut self.snapshot, &PdfMutation::InsertPage { index, page });
            self
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_info(mut self, info: PdfInfo) -> Self {
            apply_pdf_mutation(&mut self.snapshot, &PdfMutation::SetInfo { info });
            self
        }
    }

    impl ArtifactBuilder for PdfUaBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        async fn empty() -> Self {
            Self::new("en")
        }

        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text)?).await)
        }

        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes)?).await)
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
            let hard: Vec<Diagnostic> = check_ua_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
        async fn new_requires_lang_and_builds_clean() {
            let snapshot = PdfUaBuilderConstruction::new("en-US")
                .await
                .add_page(PdfPage::new(200.0, 200.0))
                .await
                .set_info(PdfInfo { title: Some("An Accessible Doc".into()), ..PdfInfo::default() })
                .await
                .build()
                .await
                .expect("conforming construction must build");
            assert_eq!(snapshot.pages.len(), 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let mut snapshot = PdfUaBuilderConstruction::new("en-US").await.add_page(PdfPage::new(100.0, 100.0)).await.build().await.unwrap();
            // Strip the seeded /StructTreeRoot to simulate a stripped-down document reaching the
            // builder via the generic `SetSnapshot` escape hatch.
            if let Some(catalog_obj) = snapshot.objects.iter_mut().find(|o| o.id.num == 1) {
                if let PdfObject::Dict(d) = &mut catalog_obj.value {
                    d.retain(|e| e.key != "StructTreeRoot");
                }
            }
            let (mutated, _diff) = PdfUaBuilderConstruction::from_snapshot(PdfSnapshot::default()).await.mutate(PdfMutation::SetSnapshot { snapshot }).await;
            let err = mutated.build().expect_err("a Catalog missing /StructTreeRoot must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::ua::schema::CODE_STRUCT_TREE_ROOT));
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
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("ua") };

    //#region 🔖️Conformance
    pub const CODE_MARKINFO: &str = "stdio.pdf.ua.missing-markinfo-marked";
    pub const CODE_STRUCT_TREE_ROOT: &str = "stdio.pdf.ua.missing-structtreeroot";
    pub const CODE_LANG: &str = "stdio.pdf.ua.missing-lang";
    pub const CODE_DISPLAY_DOC_TITLE: &str = "stdio.pdf.ua.missing-displaydoctitle";
    pub const CODE_INFO_TITLE: &str = "stdio.pdf.ua.missing-info-title";
    pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.ua.font-not-embedded";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dict_name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
        dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolve_ref<'a>(objects: &'a [PdfIndirectObject], r: ObjRef) -> Option<&'a PdfObject> {
        objects.iter().find(|o| o.id == r).map(|o| &o.value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolve_item<'a>(objects: &'a [PdfIndirectObject], item: &'a PdfObject) -> Option<&'a PdfObject> {
        match item {
            PdfObject::Ref(r) => resolve_ref(objects, *r),
            other => Some(other),
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn find_catalog(objects: &[PdfIndirectObject]) -> Option<&PdfObject> {
        objects.iter().find(|o| o.value.as_dict().map(|d| dict_name(d, "Type") == Some("Catalog")).unwrap_or(false)).map(|o| &o.value)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolved_dict_entry<'a>(objects: &'a [PdfIndirectObject], catalog: &'a PdfObject, key: &str) -> Option<&'a PdfObject> {
        catalog.dict_get(key).and_then(|v| resolve_item(objects, v))
    }

    /// ✅ Real check: `/Root/MarkInfo` resolves to a dict carrying `/Marked true`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_marked_true(objects: &[PdfIndirectObject], catalog: &PdfObject) -> bool {
        resolved_dict_entry(objects, catalog, "MarkInfo").and_then(|v| v.dict_get("Marked")).map(|v| matches!(v, PdfObject::Bool(true))).unwrap_or(false)
    }

    /// 🌳️ Real check: `/Root` carries a `/StructTreeRoot` key at all (any value -- presence is what
    /// PDF/UA requires; deep structure-tree content validation is out of this schema's reach).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_struct_tree_root(catalog: &PdfObject) -> bool {
        catalog.dict_get("StructTreeRoot").is_some()
    }

    /// 🗣️ Real check: `/Root/Lang` is a non-empty text string.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_nonempty_lang(catalog: &PdfObject) -> bool {
        catalog.dict_get("Lang").map(|v| matches!(v, PdfObject::Str(s) if !s.is_empty())).unwrap_or(false)
    }

    /// 🏷️ Real check: `/Root/ViewerPreferences` resolves to a dict carrying `/DisplayDocTitle true`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_display_doc_title(objects: &[PdfIndirectObject], catalog: &PdfObject) -> bool {
        resolved_dict_entry(objects, catalog, "ViewerPreferences").and_then(|v| v.dict_get("DisplayDocTitle")).map(|v| matches!(v, PdfObject::Bool(true))).unwrap_or(false)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
        resolve_ref(objects, desc_ref).and_then(|o| o.as_dict()).map(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3")).unwrap_or(false)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn non_embedded_fonts(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 14289-1:2014 (PDF/UA-1) conformance checks against one already-decoded
    /// `PdfSnapshot`. Shared single source of truth used by `PdfUaComposer`, `PdfUaBuilder`, and
    /// `PdfUaValidator`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_ua_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let objects = &snapshot.objects;
        let mut out = Vec::new();
        let catalog = find_catalog(objects);
        match catalog {
            Some(catalog) => {
                if !has_marked_true(objects, catalog) {
                    out.push(hard(CODE_MARKINFO, "/Root/MarkInfo is absent or lacks /Marked true -- ISO 14289-1 requires the document be marked as tagged".into()));
                }
                if !has_struct_tree_root(catalog) {
                    out.push(hard(CODE_STRUCT_TREE_ROOT, "/Root carries no /StructTreeRoot key -- PDF/UA's structure tree is entirely absent".into()));
                }
                if !has_nonempty_lang(catalog) {
                    out.push(soft(CODE_LANG, "/Root/Lang is absent or empty -- PDF/UA expects a document language".into()));
                }
                if !has_display_doc_title(objects, catalog) {
                    out.push(soft(CODE_DISPLAY_DOC_TITLE, "/Root/ViewerPreferences lacks /DisplayDocTitle true -- PDF/UA recommends showing the document title, not the filename".into()));
                }
            }
            None => {
                out.push(hard(CODE_MARKINFO, "no /Type /Catalog object found -- cannot verify /MarkInfo/Marked".into()));
                out.push(hard(CODE_STRUCT_TREE_ROOT, "no /Type /Catalog object found -- cannot verify /StructTreeRoot".into()));
                out.push(soft(CODE_LANG, "no /Type /Catalog object found -- cannot verify /Lang".into()));
                out.push(soft(CODE_DISPLAY_DOC_TITLE, "no /Type /Catalog object found -- cannot verify /ViewerPreferences/DisplayDocTitle".into()));
            }
        }
        if snapshot.info.title.as_deref().unwrap_or("").is_empty() {
            out.push(soft(CODE_INFO_TITLE, "document Info.title is absent or empty -- PDF/UA expects a real title, not a filename fallback".into()));
        }
        for r in non_embedded_fonts(objects) {
            out.push(soft(CODE_FONT_NOT_EMBEDDED, format!("font object {} {} R has no FontFile/FontFile2/FontFile3 reachable from its FontDescriptor -- PDF/UA requires embedded fonts", r.num, r.gen)));
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    pub struct PdfUaAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfUaAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source).await
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources).await;
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_ua_conformance(snapshot);
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
        use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfInfo;

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn tagged_catalog_objects() -> Vec<PdfIndirectObject> {
            vec![
                PdfIndirectObject {
                    id: ObjRef { num: 1, gen: 0 },
                    value: PdfObject::Dict(vec![
                        PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                        PdfDictEntry { key: "MarkInfo".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) },
                        PdfDictEntry { key: "StructTreeRoot".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
                        PdfDictEntry { key: "Lang".into(), value: PdfObject::Str(b"en-US".to_vec()) },
                        PdfDictEntry { key: "ViewerPreferences".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                    ]),
                },
                PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Marked".into(), value: PdfObject::Bool(true) }]) },
                PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("StructTreeRoot".into()) }]) },
                PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "DisplayDocTitle".into(), value: PdfObject::Bool(true) }]) },
            ]
        }

        #[semio_framework_async_macros::async_test]
        async fn fully_tagged_conforming_document_has_no_diagnostics() {
            let snapshot = PdfSnapshot { objects: tagged_catalog_objects(), info: PdfInfo { title: Some("A Title".into()), ..PdfInfo::default() }, ..PdfSnapshot::default() };
            let diagnostics = check_ua_conformance(&snapshot);
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_markinfo_and_structtreeroot_are_hard() {
            let objects = vec![PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }]) }];
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_ua_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MARKINFO && d.severity == Severity::Error), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRUCT_TREE_ROOT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn marked_false_is_still_hard() {
            let objects = vec![
                PdfIndirectObject {
                    id: ObjRef { num: 1, gen: 0 },
                    value: PdfObject::Dict(vec![
                        PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                        PdfDictEntry { key: "MarkInfo".into(), value: PdfObject::Ref(ObjRef { num: 2, gen: 0 }) },
                        PdfDictEntry { key: "StructTreeRoot".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
                    ]),
                },
                PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Marked".into(), value: PdfObject::Bool(false) }]) },
                PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Dict(vec![]) },
            ];
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_ua_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_MARKINFO && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_lang_title_and_displaydoctitle_are_soft() {
            let snapshot = PdfSnapshot { objects: tagged_catalog_objects().into_iter().filter(|o| o.id.num != 4).collect(), ..PdfSnapshot::default() };
            let mut objects = snapshot.objects.clone();
            if let Some(cat) = objects.iter_mut().find(|o| o.id.num == 1) {
                if let PdfObject::Dict(d) = &mut cat.value {
                    d.retain(|e| e.key != "Lang" && e.key != "ViewerPreferences");
                }
            }
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_ua_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LANG && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DISPLAY_DOC_TITLE && d.severity == Severity::Warning), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_INFO_TITLE && d.severity == Severity::Warning), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfUaBuilderFacets {
        construction: PdfUaBuilderConstruction,
        analysis: PdfUaAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfUaComposerComposition,
    }
    builder: PdfUaBuilder,
    analyzer: PdfUaAnalyzer,
    composer: PdfUaComposer,
);
//#endregion 🧬️DerivedArtifactFacets
