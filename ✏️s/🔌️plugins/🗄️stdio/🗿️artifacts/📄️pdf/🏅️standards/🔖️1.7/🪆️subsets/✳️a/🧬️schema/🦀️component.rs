//! 🧬️ PdfSnapshot schema (1.7/✳️a) — reuses the ✳️any subset's `PdfSnapshot` verbatim (the SAME
//! Rust type, same `s.stdio.pdf.1.7` schema id). PDF/A (ISO 19005 parts 2 and 3) is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/✳️a/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`,
//! without duplicating the schema definition.
//!
//! W2 restructure (ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES): this subset was
//! previously `✳️a-2b`, conflating the PDF/A *family* ("a") with a specific conformance *level*
//! ("2b"). The subset id is now just `a`; the level (2b/2u/3b/3u) is analyzer-DETECTED DATA
//! reported as a diagnostic (`stdio.pdf.a.level`), never part of the dialect id -- see
//! `🧐️analyzer`'s `detect_pdfa_level`.

pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::*;
//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `📦️glue.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// ✳️any subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of ✳️any's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::<name>::schema::mutations` while ✳️any's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️component.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::pdf::standards::v1_7::subsets::a::schema::check_pdf_a_conformance;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfInfo, PdfObject, PdfPage, PdfSnapshot};
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Seed
    /// 🌱️ Seeds a fresh snapshot with a real `/Root /OutputIntents` → `OutputIntent` object pair
    /// (`/S /GTS_PDFA1`, ISO 19005-2/-3's own conformance marker) -- a genuine, well-formed PDF/A
    /// OutputIntent, not a placeholder value that merely satisfies string equality.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn seeded_snapshot(output_intent_condition: String) -> PdfSnapshot {
        let objects = vec![
            PdfIndirectObject {
                id: ObjRef { num: 1, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) }]),
            },
            PdfIndirectObject {
                id: ObjRef { num: 2, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                    PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFA1".into()) },
                    PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(output_intent_condition.into_bytes()) },
                ]),
            },
        ];
        PdfSnapshot { objects, ..PdfSnapshot::default() }
    }
    //#endregion 🔖️Seed

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfABuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfABuilderConstruction {
        /// ➕ The recommended entry point: REQUIRES an OutputIntent condition identifier
        /// (e.g. `"sRGB IEC61966-2.1"`) up front -- there is no variant of `new` that omits it.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(output_intent_condition: impl Into<String>) -> Self {
            Self { snapshot: seeded_snapshot(output_intent_condition.into()) }
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

    impl ArtifactBuilder for PdfABuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        /// ⚠️ `ArtifactBuilder::empty()` is mandated no-arg by the SDK trait (generic UI/mutation
        /// dispatch needs every builder facet uniform) -- it falls back to a generic sRGB condition
        /// rather than omitting the OutputIntent entirely, since `build()` requires one to pass clean
        /// regardless. Prefer `PdfABuilderConstruction::new(condition)` directly wherever the real condition is
        /// known.
        async fn empty() -> Self {
            Self::new("sRGB IEC61966-2.1")
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

        /// 🛡️ The real construction gate: however `self.snapshot` got here (`new`+`add_page`,
        /// `from_binary`, a raw `mutate(SetSnapshot { .. })`), a hard PDF/A violation fails
        /// `build()` -- soft/info diagnostics (missing OutputIntent, non-embedded font, the detected
        /// level) pass through as advisory `Diagnostic`s; the `Err` path is NOT taken for those, only
        /// hard ones block.
        async fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_pdf_a_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
        async fn new_requires_output_intent_and_builds_clean() {
            let snapshot = PdfABuilderConstruction::new("sRGB IEC61966-2.1")
                
                .add_page(PdfPage::new(200.0, 200.0))
                
                .set_info(PdfInfo { title: Some("A Test".into()), ..PdfInfo::default() })
                
                .build()
                
                .await.expect("conforming construction must build");
            assert_eq!(snapshot.pages.len(), 1);
            assert_eq!(snapshot.info.title.as_deref(), Some("A Test"));
        }

        #[semio_framework_async_macros::async_test]
        async fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let violating = PdfIndirectObject { id: ObjRef { num: 99, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("Launch".into()) }]) };
            let mut snapshot = PdfABuilderConstruction::new("sRGB IEC61966-2.1").add_page(PdfPage::new(100.0, 100.0)).build().await.unwrap();
            snapshot.objects.push(violating);
            // Even routed back in via the generic `SetSnapshot` escape hatch, `build()` still catches it.
            let (mutated, _diff) = PdfABuilderConstruction::from_snapshot(PdfSnapshot::default()).await.mutate(PdfMutation::SetSnapshot { snapshot }).await;
            let err = mutated.build().await.expect_err("a /Launch action must fail build()");
            assert!(err.iter().any(|d| d.code.0 == crate::artifacts::pdf::standards::v1_7::subsets::a::schema::CODE_LAUNCH));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfIndirectObject, PdfObject, PdfSnapshot};
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfAnalyzer as PdfAnyAnalyzer;
    pub use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::PdfParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("a") };

    //#region 🔖️Level
    /// 🔢️ The four ISO 19005 PDF/A conformance levels. Never fabricated as a whole: see
    /// `detect_pdfa_level` for exactly what is and isn't determinable from `PdfSnapshot.objects`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PdfALevel {
        L2b,
        L2u,
        L3b,
        L3u,
    }

    impl PdfALevel {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn as_str(self) -> &'static str {
            match self {
                PdfALevel::L2b => "2b",
                PdfALevel::L2u => "2u",
                PdfALevel::L3b => "3b",
                PdfALevel::L3u => "3u",
            }
        }
    }

    /// 🔍️ Real, honestly-scoped PDF/A conformance LEVEL detector. What the retained object graph
    /// genuinely lets us tell apart:
    /// - PART (2 vs 3): ISO 19005-3 introduced `/AFRelationship` on embedded-file `Filespec` objects;
    ///   ISO 19005-2 has no legitimate use for it (generic embedded files aren't allowed at all under
    ///   Part 2). So "any Filespec object with `/EF` + `/AFRelationship`" is a real, non-fabricated
    ///   signal that this document targets Part 3.
    /// - Conformance LETTER (b vs u): NOT determinable from this schema. The `u` suffix means every
    ///   text string in the document is genuine Unicode content (vs. `b`'s weaker "just don't corrupt
    ///   bytes" bar) -- telling that apart needs per-string script/encoding analysis this object model
    ///   doesn't retain a basis for. Rather than guess, this always defaults to `b`.
    /// Returns `None` when the document doesn't even carry a `GTS_PDFA1` OutputIntent -- there is no
    /// honest basis for reporting *any* PDF/A level on a document that doesn't claim to be one.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn detect_pdfa_level(snapshot: &PdfSnapshot) -> Option<PdfALevel> {
        let objects = &snapshot.objects;
        if !has_pdfa_output_intent(objects) {
            return None;
        }
        if !embedded_files_with_afrelationship(objects).is_empty() {
            Some(PdfALevel::L3b)
        } else {
            Some(PdfALevel::L2b)
        }
    }
    //#endregion 🔖️Level

    //#region 🔖️Conformance
    pub const CODE_ENCRYPT: &str = "stdio.pdf.a.encrypt-present";
    pub const CODE_JAVASCRIPT: &str = "stdio.pdf.a.javascript-action";
    pub const CODE_LAUNCH: &str = "stdio.pdf.a.launch-action";
    pub const CODE_OUTPUT_INTENT: &str = "stdio.pdf.a.missing-output-intent";
    pub const CODE_FONT_NOT_EMBEDDED: &str = "stdio.pdf.a.font-not-embedded";
    pub const CODE_EMBEDDED_FILE_AFRELATIONSHIP: &str = "stdio.pdf.a.embedded-file-missing-afrelationship";
    pub const CODE_LEVEL: &str = "stdio.pdf.a.level";

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
    fn dict_name<'a>(dict: &'a [crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfDictEntry], key: &str) -> Option<&'a str> {
        dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
    }

    /// 🔒️ Real, independent-of-decode scan: does any retained object look like a Standard Security
    /// Handler encryption dictionary (`/Filter /Standard` + `/V`/`/R`/`/O`/`/U`)?
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

    /// 📜️ Real scan for `/S /<subtype>` action dictionaries anywhere in the retained object graph.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_action_subtype(objects: &[PdfIndirectObject], subtype: &str) -> Vec<ObjRef> {
        objects.iter().filter(|o| o.value.as_dict().map(|d| dict_name(d, "S") == Some(subtype)).unwrap_or(false)).map(|o| o.id).collect()
    }

    /// 📜️ Real scan for a bare `/JS` key not already caught by `/S /JavaScript` (some JS action
    /// dicts carry `/JS` without a matching `/S` when malformed/hand-authored -- PDF/A forbids the
    /// key itself, not just the well-formed `/S /JavaScript` shape).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_js_key_only(objects: &[PdfIndirectObject], already: &[ObjRef]) -> Vec<ObjRef> {
        objects.iter().filter(|o| !already.contains(&o.id) && o.value.as_dict().map(|d| d.iter().any(|e| e.key == "JS")).unwrap_or(false)).map(|o| o.id).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn find_catalog(objects: &[PdfIndirectObject]) -> Option<&PdfObject> {
        objects.iter().find(|o| o.value.as_dict().map(|d| dict_name(d, "Type") == Some("Catalog")).unwrap_or(false)).map(|o| &o.value)
    }

    /// 🏳️ Real check: `/Root`'s `/OutputIntents` array contains an intent with `/S /GTS_PDFA1`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn has_pdfa_output_intent(objects: &[PdfIndirectObject]) -> bool {
        let Some(catalog) = find_catalog(objects) else { return false };
        let Some(intents) = catalog.dict_get("OutputIntents").and_then(|v| v.as_array()) else { return false };
        intents.iter().any(|item| resolve_item(objects, item).and_then(|o| o.as_dict()).map(|d| dict_name(d, "S") == Some("GTS_PDFA1")).unwrap_or(false))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn descriptor_has_embedded_file(objects: &[PdfIndirectObject], desc_ref: ObjRef) -> bool {
        resolve_ref(objects, desc_ref).and_then(|o| o.as_dict()).map(|d| d.iter().any(|e| e.key == "FontFile" || e.key == "FontFile2" || e.key == "FontFile3")).unwrap_or(false)
    }

    /// 🔤️ Real check: every `/Type /Font` object (simple or `/DescendantFonts` composite) resolves
    /// to a `/FontDescriptor` carrying an embedded font program. Real because `objects` retains the
    /// full logical indirect-object graph -- font dicts are genuinely present
    /// here, this is not fabricated against a field the engine doesn't parse.
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

    /// 📎️ Real scan: `/Type /Filespec` objects that carry an `/EF` entry (an actual attached file
    /// stream dict, as opposed to a bare external-file reference) AND a non-empty `/AFRelationship`
    /// name. This is the A-3-only, genuinely-inspectable signal `detect_pdfa_level` keys off of.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn embedded_files_with_afrelationship(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects
            .iter()
            .filter(|o| {
                let Some(d) = o.value.as_dict() else { return false };
                dict_name(d, "Type") == Some("Filespec") && d.iter().any(|e| e.key == "EF") && d.iter().any(|e| e.key == "AFRelationship")
            })
            .map(|o| o.id)
            .collect()
    }

    /// 📎️ Real scan, the A-2/A-3 differentiator: `/Type /Filespec` objects that carry an `/EF`
    /// (actual attached file) but NO `/AFRelationship`. ISO 19005-3 requires the relationship key on
    /// every such association; ISO 19005-2 doesn't permit generic embedded files at all, so this
    /// shape is non-conformant regardless of which part the rest of the document targets -- a real,
    /// level-independent hard check rather than one gated on `detect_pdfa_level`'s (necessarily
    /// incomplete) part guess.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn embedded_files_missing_afrelationship(objects: &[PdfIndirectObject]) -> Vec<ObjRef> {
        objects
            .iter()
            .filter(|o| {
                let Some(d) = o.value.as_dict() else { return false };
                dict_name(d, "Type") == Some("Filespec") && d.iter().any(|e| e.key == "EF") && !d.iter().any(|e| e.key == "AFRelationship")
            })
            .map(|o| o.id)
            .collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// ℹ️ Informational-only diagnostic (`detect_pdfa_level`'s report) -- `Severity::Info`,
    /// the softest severity this fault model has.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn info(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Info, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 19005-2/-3 (PDF/A-2, PDF/A-3) conformance checks against one already-decoded
    /// `PdfSnapshot`. Shared single source of truth: `PdfAComposer::compose` hard-gates on this
    /// (pre-serialization, authoritative), `PdfABuilder::build` hard-gates on this too, and the
    /// generic `SubsetValidator` (registered from `🎹️composer::register`) re-runs it post-hoc against
    /// the wire payload for the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_pdf_a_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let objects = &snapshot.objects;
        let mut out = Vec::new();
        for r in scan_encryption(objects) {
            out.push(hard(CODE_ENCRYPT, format!("object {} {} R looks like a Standard Security Handler encryption dictionary -- PDF/A forbids /Encrypt", r.num, r.gen)));
        }
        let js_actions = scan_action_subtype(objects, "JavaScript");
        for r in &js_actions {
            out.push(hard(CODE_JAVASCRIPT, format!("object {} {} R is an /S /JavaScript action -- PDF/A forbids embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_js_key_only(objects, &js_actions) {
            out.push(hard(CODE_JAVASCRIPT, format!("object {} {} R carries a /JS key -- PDF/A forbids embedded JavaScript", r.num, r.gen)));
        }
        for r in scan_action_subtype(objects, "Launch") {
            out.push(hard(CODE_LAUNCH, format!("object {} {} R is an /S /Launch action -- PDF/A forbids launch actions", r.num, r.gen)));
        }
        for r in embedded_files_missing_afrelationship(objects) {
            out.push(hard(
                CODE_EMBEDDED_FILE_AFRELATIONSHIP,
                format!("Filespec object {} {} R carries /EF (an attached file) but no /AFRelationship -- ISO 19005-3 requires it on every embedded-file association, and ISO 19005-2 forbids generic embedded files entirely", r.num, r.gen),
            ));
        }
        if !has_pdfa_output_intent(objects) {
            out.push(soft(CODE_OUTPUT_INTENT, "no OutputIntent with /S /GTS_PDFA1 reachable from /Root/OutputIntents -- real PDF/A files declare one".into()));
        }
        for r in non_embedded_fonts(objects) {
            out.push(soft(CODE_FONT_NOT_EMBEDDED, format!("font object {} {} R has no FontFile/FontFile2/FontFile3 reachable from its FontDescriptor -- PDF/A requires embedded fonts", r.num, r.gen)));
        }
        if let Some(level) = detect_pdfa_level(snapshot) {
            out.push(info(
                CODE_LEVEL,
                format!(
                    "detected PDF/A conformance level {} -- part ({}) inferred from {}; letter ('b' vs 'u') defaulted to 'b' since Unicode-string usage isn't distinguishable from this object graph",
                    level.as_str(),
                    if level == PdfALevel::L3b { "3" } else { "2" },
                    if level == PdfALevel::L3b { "a Filespec object with /EF + /AFRelationship" } else { "the absence of any such Filespec object" },
                ),
            ));
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.7/✳️a): delegates the real parse to the ✳️any subset's analyzer
    /// (same `PdfSnapshot`), then folds real PDF/A conformance diagnostics on top. `sniff` also
    /// delegates -- a subset-level sniff for `a` is "is this recognizable as a PDF at all", the same
    /// magic-byte probe every 1.7 dialect shares; conformance is a separate, heavier question answered
    /// by `analyze`/`check_pdf_a_conformance`, not by `sniff`.
    pub struct PdfAAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfAAnalyzerAnalysis {
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
                let checks = check_pdf_a_conformance(snapshot);
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

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn output_intent_objects(condition: &str) -> Vec<PdfIndirectObject> {
            vec![
                PdfIndirectObject {
                    id: ObjRef { num: 1, gen: 0 },
                    value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }, PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) }]),
                },
                PdfIndirectObject {
                    id: ObjRef { num: 2, gen: 0 },
                    value: PdfObject::Dict(vec![
                        PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                        PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFA1".into()) },
                        PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(condition.as_bytes().to_vec()) },
                    ]),
                },
            ]
        }

        #[semio_framework_async_macros::async_test]
        async fn conforming_snapshot_with_output_intent_reports_only_level_info() {
            let snapshot = PdfSnapshot { objects: output_intent_objects("sRGB IEC61966-2.1"), ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert_eq!(diagnostics.len(), 1, "expected exactly the level-detection Info diagnostic, got {diagnostics:?}");
            assert_eq!(diagnostics[0].code.0, CODE_LEVEL);
            assert_eq!(diagnostics[0].severity, Severity::Info);
        }

        #[semio_framework_async_macros::async_test]
        async fn missing_output_intent_is_soft_and_reports_no_level() {
            let snapshot = PdfSnapshot::default();
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert_eq!(diagnostics.len(), 1);
            assert_eq!(diagnostics[0].code.0, CODE_OUTPUT_INTENT);
            assert_eq!(diagnostics[0].severity, Severity::Warning);
            assert!(detect_pdfa_level(&snapshot).is_none());
        }

        #[semio_framework_async_macros::async_test]
        async fn encryption_dict_shape_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Filter".into(), value: PdfObject::Name("Standard".into()) },
                    PdfDictEntry { key: "V".into(), value: PdfObject::Int(2) },
                    PdfDictEntry { key: "R".into(), value: PdfObject::Int(3) },
                    PdfDictEntry { key: "O".into(), value: PdfObject::Str(vec![0u8; 32]) },
                    PdfDictEntry { key: "U".into(), value: PdfObject::Str(vec![0u8; 32]) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPT && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn javascript_action_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("JavaScript".into()) }, PdfDictEntry { key: "JS".into(), value: PdfObject::Str(b"app.alert(1)".to_vec()) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert_eq!(diagnostics.iter().filter(|d| d.code.0 == CODE_JAVASCRIPT).count(), 1, "must not double-report S=JavaScript + JS key on the same object: got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_JAVASCRIPT && d.severity == Severity::Error));
        }

        #[semio_framework_async_macros::async_test]
        async fn launch_action_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "S".into(), value: PdfObject::Name("Launch".into()) }, PdfDictEntry { key: "F".into(), value: PdfObject::Str(b"calc.exe".to_vec()) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LAUNCH && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn non_embedded_font_is_soft() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) },
                    PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("Type1".into()) },
                    PdfDictEntry { key: "BaseFont".into(), value: PdfObject::Name("Helvetica".into()) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_FONT_NOT_EMBEDDED && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn embedded_font_via_descriptor_has_no_diagnostic() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Font".into()) },
                    PdfDictEntry { key: "Subtype".into(), value: PdfObject::Name("TrueType".into()) },
                    PdfDictEntry { key: "FontDescriptor".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                ]),
            });
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 4, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("FontDescriptor".into()) }, PdfDictEntry { key: "FontFile2".into(), value: PdfObject::Ref(ObjRef { num: 5, gen: 0 }) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_FONT_NOT_EMBEDDED), "got {diagnostics:?}");
        }

        #[semio_framework_async_macros::async_test]
        async fn embedded_file_missing_afrelationship_is_hard() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Filespec".into()) }, PdfDictEntry { key: "EF".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) }]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_EMBEDDED_FILE_AFRELATIONSHIP && d.severity == Severity::Error), "got {diagnostics:?}");
            // No genuine part-3 signal (missing /AFRelationship is exactly the violation), so the
            // level detector must not credit this document with Part 3.
            assert_eq!(detect_pdfa_level(&snapshot), Some(PdfALevel::L2b));
        }

        #[semio_framework_async_macros::async_test]
        async fn embedded_file_with_afrelationship_detects_level_3b_and_is_clean() {
            let mut objects = output_intent_objects("sRGB IEC61966-2.1");
            objects.push(PdfIndirectObject {
                id: ObjRef { num: 3, gen: 0 },
                value: PdfObject::Dict(vec![
                    PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Filespec".into()) },
                    PdfDictEntry { key: "EF".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                    PdfDictEntry { key: "AFRelationship".into(), value: PdfObject::Name("Data".into()) },
                ]),
            });
            let snapshot = PdfSnapshot { objects, ..PdfSnapshot::default() };
            assert_eq!(detect_pdfa_level(&snapshot), Some(PdfALevel::L3b));
            let diagnostics = check_pdf_a_conformance(&snapshot);
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_EMBEDDED_FILE_AFRELATIONSHIP), "got {diagnostics:?}");
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_LEVEL && d.message.contains("3b")), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfABuilderFacets {
        construction: PdfABuilderConstruction,
        analysis: PdfAAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfAComposerComposition,
    }
    builder: PdfABuilder,
    analyzer: PdfAAnalyzer,
    composer: PdfAComposer,
);
//#endregion 🧬️DerivedArtifactFacets
