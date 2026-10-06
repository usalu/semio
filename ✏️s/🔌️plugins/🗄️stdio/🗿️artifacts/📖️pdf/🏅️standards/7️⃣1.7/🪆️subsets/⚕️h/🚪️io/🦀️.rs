//! 🚪️ IO stdio.pdf (1.7/⚕️h) — reuses the 🧱️base subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `🗄️a/🚪️io` and `🧱️base/🚪️io` for this artifact. AIIM/ASTM PDF Healthcare Best Practices Guide.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use crate::standards::v1_7::subsets::base::io::PdfComposer as PdfAnyComposer;
    use crate::standards::v1_7::subsets::h::schema::check_h_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_H: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("h") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfHComposerComposition;

    impl ArtifactComposition for PdfHComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_H;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_H, DEP_BINARY, DEP_DEFLATE]
        }

        /// ✅ Always `Ok` -- PDF/H has no hard checks to gate on (see module doc comment). Advisory
        /// diagnostics from `check_h_conformance` are folded onto the successful `Composition`.
        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PdfAnyComposer::compose(sources)?;
            let checks = check_h_conformance(&inner.snapshot);
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(checks);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfHValidator;

    impl SubsetValidator for PdfHValidator {
        const DIALECT: Dialect = DIALECT_H;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_h_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.h.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/H SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfHValidator>)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

pub mod derived_construction {
    use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_7::subsets::base::schema::mutations::{apply_pdf_mutation, InsertPage, PdfMutation, SetInfo};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
    use crate::standards::v1_7::subsets::h::schema::check_h_conformance;
    use semio_framework_diagnostic::Diagnostic;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfHBuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfHBuilderConstruction {
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

    impl Default for PdfHBuilderConstruction {
        fn default() -> Self {
            Self::new()
        }
    }

    impl ArtifactBuilder for PdfHBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        fn empty() -> Self {
            Self::new()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
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

        /// ✅ Always `Ok` -- `check_h_conformance` is ALL-SOFT, so the hard-filter below is never
        /// non-empty. Still runs the real check (not skipped) so a future hard check added here would
        /// correctly start gating without any other code change.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_h_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)).collect();
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

pub mod derived_analysis {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};
    use crate::standards::v1_7::subsets::base::io::PdfAnalyzer as PdfAnyAnalyzer;
    pub use crate::standards::v1_7::subsets::base::io::PdfParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("h") };

    //#region 🔖️Conformance
    pub const CODE_INFO_TITLE_OR_AUTHOR: &str = "stdio.pdf.h.missing-info-title-or-author";
    pub const CODE_JAVASCRIPT: &str = "stdio.pdf.h.javascript-action";
    pub const CODE_LAUNCH: &str = "stdio.pdf.h.launch-action";
    pub const CODE_SIGNATURE_FIELD: &str = "stdio.pdf.h.missing-signature-field";
    pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.h.font-not-embedded";

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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().is_some_and(|d| dict_name(d, "S") == Some(subtype))).map(|o| o.id).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
        objects.iter().filter(|o| !already.contains(&o.id) && o.value.as_dict().is_some_and(|d| d.iter().any(|e| e.key == "JS"))).map(|o| o.id).collect()
    }

    /// ✍️ Real check: `/Root/AcroForm/Fields` contains a resolved entry with `/FT /Sig`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_signature_field(objects: &[PdfIndirectObject]) -> bool {
        let Some(catalog) = find_catalog(objects) else { return false };
        let Some(acroform) = catalog.dict_get("AcroForm").and_then(|v| resolve_item(objects, v)) else { return false };
        let Some(fields) = acroform.dict_get("Fields").and_then(|v| v.as_array()) else { return false };
        fields.iter().any(|item| resolve_item(objects, item).and_then(|f| f.as_dict()).is_some_and(|d| dict_name(d, "FT") == Some("Sig")))
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
            let via_descendants = d.iter().find(|e| e.key == "DescendantFonts").and_then(|e| e.value.as_array()).is_some_and(|arr| {
                arr.iter().any(|item| resolve_item(objects, item).and_then(|desc| desc.as_dict()).and_then(|dd| dd.iter().find(|e| e.key == "FontDescriptor").and_then(|e| e.value.as_ref())).is_some_and(|r| descriptor_has_embedded_file(objects, r)))
            });
            if !direct && !via_descendants {
                out.push(o.id);
            }
        }
        out
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real AIIM/ASTM PDF Healthcare Best Practices Guide (2008) checks against one
    /// already-decoded `PdfSnapshot`. ALL-SOFT by design (industry best-practice guide, never an ISO
    /// standard, no enforcement mechanism) -- never returns a `Severity::Error`/`Fatal` diagnostic.
    /// Shared single source of truth used by `PdfHComposer` and `PdfHValidator` (both pass-through,
    /// per the roster's "never hard-gates").
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_h_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let objects = &snapshot.objects;
        let mut out = Vec::new();
        let title_ok = snapshot.info.title.as_deref().is_some_and(|s| !s.is_empty());
        let author_ok = snapshot.info.author.as_deref().is_some_and(|s| !s.is_empty());
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

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
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
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

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
