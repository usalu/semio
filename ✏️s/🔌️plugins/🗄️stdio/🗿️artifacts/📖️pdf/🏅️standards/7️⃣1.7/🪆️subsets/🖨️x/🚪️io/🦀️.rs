//! 🚪️ IO stdio.pdf (1.7/🖨️x) — reuses the 🧱️base subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `🗄️a/🚪️io` and `🧱️base/🚪️io` for this artifact. ISO 15930-7:2010 (PDF/X-4).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use crate::standards::v1_7::subsets::base::io::PdfComposer as PdfAnyComposer;
    use crate::standards::v1_7::subsets::x::io::check_x_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    pub(crate) const DIALECT_X: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("x") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfXComposerComposition;

    impl ArtifactComposition for PdfXComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_X;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_X, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PdfAnyComposer::compose(sources)?;
            let checks = check_x_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("PDF/X-4 conformance violated: {} hard issue(s) -- not stamping the x dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfXValidator;

    impl SubsetValidator for PdfXValidator {
        const DIALECT: Dialect = DIALECT_X;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_x_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.x.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/X SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfXValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry. Called from the
    /// 1.7 standard's own `⚙️engine::register()`. The `ComposerEntry` itself is registered separately
    /// by the standard-level composer aggregator (`composer::entries()`).
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

/// 🌉️ Applies one x conformance mutation through its leaf-owned diff and the base bridge.
pub mod mutation_bridge {
    use crate::standards::v1_7::subsets::base::schema::{diff::PdfDiff, snapshot::PdfSnapshot};
    use crate::standards::v1_7::subsets::x::schema::mutations::PdfXMutation;

    /// ▶️ Applies the authoritative leaf diff.
    pub fn apply_x_conformance_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfXMutation) -> protocol::MutationOutcome<PdfDiff> {
        use protocol::Mutation;
        let outcome = mutation.diff(snapshot);
        crate::standards::v1_7::subsets::base::io::mutation_bridge::apply_outcome(outcome, snapshot)
    }
}

pub mod derived_construction {
    use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_7::subsets::base::io::mutation_bridge::apply_pdf_mutation;
    use crate::standards::v1_7::subsets::base::schema::mutations::{InsertPage, PdfMutation, SetInfo};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfInfo, PdfObject, PdfOutputIntent, PdfPage, PdfSnapshot};
    use crate::standards::v1_7::subsets::x::io::check_x_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Seed
    /// 🌱️ Seeds a fresh snapshot with a real `/Root /OutputIntents` → `OutputIntent` object pair
    /// (`/S /GTS_PDFX` + `/DestOutputProfile`, ISO 15930-7's own conformance marker).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn seeded_snapshot(output_condition: String) -> PdfSnapshot {
        let profile = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🖼️assets/🌈️icc/🌈️sRGB2014.icc")).to_vec();
        let condition_identifier = output_condition.clone();
        let objects = vec![
            PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                PdfDictEntry { key: "Pages".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) },
            ]) },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFX".into()) },
                PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Text(output_condition) },
                PdfDictEntry { key: "DestOutputProfile".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
            ]) },
            PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Stream { dict: vec![PdfDictEntry { key: "N".into(), value: PdfObject::Int(3) }], data: profile.clone(), filters: Vec::new() } },
            PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Pages".into()) }, PdfDictEntry { key: "Kids".into(), value: PdfObject::Array(Vec::new()) }, PdfDictEntry { key: "Count".into(), value: PdfObject::Int(0) }]) },
        ];
        let profile = { use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{NativePdfArtifactResources,PdfArtifactResourcePort}; NativePdfArtifactResources::default().admit("s.stdio.icc",objects[2].value.clone()).expect("seeded native ICC stream") };
        PdfSnapshot {
            admitted_stream_roles: vec![crate::standards::v1_7::subsets::base::schema::stream_roles::PdfAdmittedStreamRole { identity: crate::standards::v1_7::subsets::base::schema::stream_roles::PdfGraphIdentity { owner: ObjRef {num:3,gen:0}, path: Vec::new() }, dependencies: vec![crate::standards::v1_7::subsets::base::schema::stream_roles::PdfGraphIdentity { owner: ObjRef {num:3,gen:0}, path: Vec::new() }], value: crate::standards::v1_7::subsets::base::schema::stream_roles::PdfStreamRoleValue::ReferenceBody {reference:profile.clone()} }],
            output_intents: vec![PdfOutputIntent { subtype: "GTS_PDFX".into(), condition_identifier, condition: None, registry_name: None, info: None, profile: Some(profile) }],
            catalog_extra: Vec::new(),
            trailer: vec![PdfDictEntry { key: "Root".into(), value: PdfObject::Ref(ObjRef { num: 1, gen: 0 }) }],
            objects,
            ..PdfSnapshot::default()
        }
    }
    //#endregion 🔖️Seed

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfXBuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfXBuilderConstruction {
        /// ➕ The recommended entry point: REQUIRES an output-condition identifier up front.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(output_condition: impl Into<String>) -> Self {
            Self { snapshot: seeded_snapshot(output_condition.into()) }
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

    impl ArtifactBuilder for PdfXBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        fn empty() -> Self {
            Self::new("sRGB2014")
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
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }

        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
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
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
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
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn dict_name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
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

    /// 🔒️ Real scan: does any retained object look like a Standard Security Handler encryption
    /// dictionary (`/Filter /Standard` + `/V`/`/R`/`/O`/`/U`)?
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn scan_encryption(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().is_some_and(|d| dict_name(d, "S") == Some(subtype))).map(|o| o.id).collect()
    }

    /// 📜️ Real scan for a bare `/JS` key not already caught by `/S /JavaScript`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
        objects.iter().filter(|o| !already.contains(&o.id) && o.value.as_dict().is_some_and(|d| d.iter().any(|e| e.key == "JS"))).map(|o| o.id).collect()
    }

    /// 🏳️ Real check: `/Root`'s `/OutputIntents` array contains an intent with `/S /GTS_PDFX` AND a
    /// `/DestOutputProfile` key (the ICC profile PDF/X-4 requires alongside the marker).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_pdfx_output_intent(objects: &[PdfIndirectObject]) -> bool {
        let Some(catalog) = find_catalog(objects) else { return false };
        let Some(intents) = catalog.dict_get("OutputIntents").and_then(|v| v.as_array()) else { return false };
        intents.iter().any(|item| resolve_item(objects, item).and_then(|o| o.as_dict()).is_some_and(|d| dict_name(d, "S") == Some("GTS_PDFX") && d.iter().any(|e| e.key == "DestOutputProfile")))
    }

    /// 📄️ Real scan: every `/Type /Page` object carries a `/TrimBox` or `/ArtBox` key. Returns the
    /// refs of pages missing BOTH.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn pages_missing_trim_or_art_box(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects
            .iter()
            .filter(|o| {
                let Some(d) = o.value.as_dict() else { return false };
                dict_name(d, "Type") == Some("Page") && !d.iter().any(|e| e.key == "TrimBox") && !d.iter().any(|e| e.key == "ArtBox")
            })
            .map(|o| o.id)
            .collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
        resolve_ref(objects, desc_ref).and_then(|o| o.as_dict()).is_some_and(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3"))
    }

    /// 🔤️ Real check: every `/Type /Font` object (simple or `/DescendantFonts` composite) resolves
    /// to a `/FontDescriptor` carrying an embedded font program.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn non_embedded_fonts(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
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

    /// 🎬️ Real scan: `/Subtype /Movie` or `/Subtype /Sound` annotation dicts anywhere in the graph.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn movie_or_sound_annotations(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().is_some_and(|d| matches!(dict_name(d, "Subtype"), Some("Movie") | Some("Sound")))).map(|o| o.id).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 15930-7:2010 (PDF/X-4) conformance checks against one already-decoded
    /// `PdfSnapshot`. Shared single source of truth: `PdfXComposer::compose`, `PdfXBuilder::build`,
    /// `PdfXValidator::validate`, and (layered on top) `🧾️vt`'s own conformance fn all call this.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_x_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let objects = &snapshot.objects;
        let mut out = Vec::new();
        if snapshot.encryption.is_some() {
            out.push(hard(CODE_ENCRYPT, "PDF/X forbids retained document encryption".into()));
        }
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
    /// 🧐️ Analyzes `stdio.pdf` (1.7/🖨️x): delegates the real parse to the 🧱️base subset's analyzer,
    /// then folds real PDF/X-4 conformance diagnostics on top.
    pub struct PdfXAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfXAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            PdfAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_x_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = semio_framework_plugin::io::Confidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

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
