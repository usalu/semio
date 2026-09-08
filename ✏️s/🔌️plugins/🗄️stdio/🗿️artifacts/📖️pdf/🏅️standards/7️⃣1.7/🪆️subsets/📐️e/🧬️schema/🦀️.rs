//! 🧬️ PdfSnapshot schema (1.7/📐️e) — reuses the 🧱️base subset's `PdfSnapshot` verbatim (the
//! SAME Rust type, same `s.stdio.pdf.1.7` schema id). PDF/E-1 (ISO 24517-1:2008) is a validation-gated dialect
//! STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot type,
//! subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/📐️e/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition. Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES W3.

pub use crate::standards::v1_7::subsets::base::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// 🧱️base subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of 🧱️base's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::<name>::schema::mutations` while 🧱️base's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_7::subsets::base::schema::mutations::{apply_pdf_mutation, InsertPage, PdfMutation, SetInfo};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
    use crate::standards::v1_7::subsets::e::schema::check_e_conformance;
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfEBuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfEBuilderConstruction {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: PdfSnapshot::default() }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_page(mut self, page: PdfPage) -> Self {
            let index = self.snapshot.pages.len();
            apply_pdf_mutation(&mut self.snapshot, &PdfMutation::InsertPage(InsertPage { index, page }));
            self
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_info(mut self, info: PdfInfo) -> Self {
            apply_pdf_mutation(&mut self.snapshot, &PdfMutation::SetInfo(SetInfo { info }));
            self
        }
    }

    impl Default for PdfEBuilderConstruction {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ArtifactBuilder for PdfEBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        fn empty() -> Self {
            Self::new()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
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

        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_e_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};
    use crate::standards::v1_7::subsets::base::schema::PdfAnalyzer as PdfAnyAnalyzer;
    pub use crate::standards::v1_7::subsets::base::schema::PdfParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("e") };

    //#region 🔖️Conformance
    pub const CODE_ENCRYPT: &str = "stdio.pdf.e.encrypt-present";
    pub const CODE_JAVASCRIPT: &str = "stdio.pdf.e.javascript-action";
    pub const CODE_LAUNCH: &str = "stdio.pdf.e.launch-action";
    pub const CODE_MOVIE_OR_SOUND: &str = "stdio.pdf.e.movie-or-sound-annotation";
    pub const CODE_OUTPUT_INTENT: &str = "stdio.pdf.e.missing-output-intent";
    pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.e.font-not-embedded";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dict_name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
        dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn resolve_ref(objects: &[PdfIndirectObject], r: ObjRef) -> Option<&PdfObject> {
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
        objects.iter().find(|o| o.value.as_dict().is_some_and(|d| dict_name(d, "Type") == Some("Catalog"))).map(|o| &o.value)
    }

    /// 🔒️ Real scan: Standard Security Handler encryption dictionary shape.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_encryption(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects
            .iter()
            .filter(|o| {
                let Some(d) = o.value.as_dict() else { return false };
                dict_name(d, "Filter") == Some("Standard") && d.iter().any(|e| e.key == "V") && d.iter().any(|e| e.key == "R") && d.iter().any(|e| e.key == "O") && d.iter().any(|e| e.key == "U")
            })
            .map(|o| o.id)
            .collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().is_some_and(|d| dict_name(d, "S") == Some(subtype))).map(|o| o.id).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
        objects.iter().filter(|o| !already.contains(&o.id) && o.value.as_dict().is_some_and(|d| d.iter().any(|e| e.key == "JS"))).map(|o| o.id).collect()
    }

    /// 🎬️ Real scan: `/Subtype /Movie` or `/Subtype /Sound` annotation dicts. `/Subtype /3D` is a
    /// distinct, explicitly allowed name -- this filter never matches it.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn movie_or_sound_annotations(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().is_some_and(|d| matches!(dict_name(d, "Subtype"), Some("Movie") | Some("Sound")))).map(|o| o.id).collect()
    }

    /// 🏳️ Real check: `/Root/OutputIntents` is a non-empty array (any `/S` marker counts for PDF/E,
    /// unlike PDF/A's/PDF/X's specific `GTS_*` requirement).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_any_output_intent(objects: &[PdfIndirectObject]) -> bool {
        let Some(catalog) = find_catalog(objects) else { return false };
        catalog.dict_get("OutputIntents").and_then(|v| v.as_array()).is_some_and(|a| !a.is_empty())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
        resolve_ref(objects, desc_ref).and_then(|o| o.as_dict()).is_some_and(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3"))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn non_embedded_fonts(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        let mut out = Vec::new();
        for o in objects {
            let Some(d) = o.value.as_dict() else { continue };
            if dict_name(d, "Type") != Some("Font") {
                continue;
            }
            let direct = d.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref()).is_some_and(|r| descriptor_has_embedded_file(objects, r));
            let via_descendants = d
                .iter()
                .find(|e| e.key == "DescendantFonts")
                .and_then(|e| e.value.as_array())
                .is_some_and(|arr| {
                    arr.iter().any(|item| {
                        resolve_item(objects, item).and_then(|desc| desc.as_dict()).and_then(|dd| dd.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref())).is_some_and(|r| descriptor_has_embedded_file(objects, r))
                    })
                });
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

    /// 🛡️ Real ISO 24517-1:2008 (PDF/E-1) conformance checks against one already-decoded
    /// `PdfSnapshot`. Shared single source of truth used by `PdfEComposer`, `PdfEBuilder`, and
    /// `PdfEValidator`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_e_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let objects = &snapshot.objects;
        let mut out = Vec::new();
        for r in scan_encryption(objects) {
            out.push(hard(CODE_ENCRYPT, format!("object {} {} R looks like a Standard Security Handler encryption dictionary -- PDF/E forbids /Encrypt", r.num, r.gen)));
        }
        let js_actions = scan_action_subtype(objects, "JavaScript");
        for r in &js_actions {
            out.push(hard(CODE_JAVASCRIPT, format!("object {} {} R is an /S /JavaScript action -- PDF/E forbids embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_js_key_only(objects, &js_actions) {
            out.push(hard(CODE_JAVASCRIPT, format!("object {} {} R carries a /JS key -- PDF/E forbids embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_action_subtype(objects, "Launch") {
            out.push(hard(CODE_LAUNCH, format!("object {} {} R is an /S /Launch action -- PDF/E forbids launch actions", r.num, r.gen)));
        }
        for r in movie_or_sound_annotations(objects) {
            out.push(hard(CODE_MOVIE_OR_SOUND, format!("annotation object {} {} R is /Subtype /Movie or /Sound -- PDF/E forbids these (unlike /Subtype /3D, which is explicitly allowed)", r.num, r.gen)));
        }
        if !has_any_output_intent(objects) {
            out.push(soft(CODE_OUTPUT_INTENT, "no OutputIntent reachable from /Root/OutputIntents -- ISO 24517-1 expects one when color-managed output matters".into()));
        }
        for r in non_embedded_fonts(objects) {
            out.push(soft(CODE_FONT_NOT_EMBEDDED, format!("font object {} {} R has no FontFile/FontFile2/FontFile3 reachable from its FontDescriptor -- PDF/E requires embedded fonts", r.num, r.gen)));
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    pub struct PdfEAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfEAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_e_conformance(snapshot);
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfEBuilderFacets {
        construction: PdfEBuilderConstruction,
        analysis: PdfEAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfEComposerComposition,
    }
    builder: PdfEBuilder,
    analyzer: PdfEAnalyzer,
    composer: PdfEComposer,
);
//#endregion 🧬️DerivedArtifactFacets
