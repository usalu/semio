//! 🚪️ IO — composer + subset validator registration for `s.stdio.semio.object`, mirroring every
//! other semio subset's convention. Registration flows through `register()`, called from this
//! standard's `⚙️engine::register()`.
//!
//! ⚠️ OUT OF SCOPE for this wave (deliberately, per this ticket's brief, same as `🔤️text`): the
//! `📥️import`/`📤️export` leaves bridging `object` to any format artifact. `io_entries()` is empty;
//! `reads()` only advertises this subset's own native dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
    use crate::standards::v1::subsets::object::schema::SemioObjectAnalyzer;
    use semio_framework_plugin::{
        register_composer_entries, register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator,
        SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

    //#region 🔖️Composer
    pub struct SemioObjectComposerComposition;

    impl ArtifactComposition for SemioObjectComposerComposition {
        type Snapshot = SemioObjectSnapshot;
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
                return Err(ComposeError { message: "SemioObjectComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioObjectAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioObjectComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decode PLUS a real referential-invariant check: every present child handle's `target`
    /// dialect must name the kind the slot declares (`brep`→`s.stdio.semio`/`v1`/`brep`, etc.) —
    /// `object` is the first COMPOSITE subset, so unlike every leaf's decode-only validator, there
    /// is now something genuinely cross-referential to check at this level (the handle's own
    /// declared kind, not the child DOCUMENT's content — dereferencing the child is a host-level
    /// concern, out of scope for a pure decode-time validator).
    pub struct SemioObjectValidator;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn wrong_kind(field: &str, expected_subset: &str, target: &store::os_io::ArtifactRef) -> Option<dsl::Diagnostic> {
        if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.subset != expected_subset {
            Some(dsl::Diagnostic::error(
                "stdio.semio_object.validate-child-kind-mismatch",
                dsl::TextSpan::at(1, 1),
                format!("SemioObjectValidator: `{field}` handle targets {}@{}/{}, expected kind s.stdio.semio subset {expected_subset}", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
            ))
        } else {
            None
        }
    }

    impl SubsetValidator for SemioObjectValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let Some(snapshot) = decoded else {
                return vec![dsl::Diagnostic::error("stdio.semio_object.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioObjectValidator: payload did not decode as a SemioObjectSnapshot".to_string())];
            };
            let mut diagnostics = Vec::new();
            if let Some(brep) = &snapshot.brep {
                diagnostics.extend(wrong_kind("brep", "brep", &brep.target));
            }
            if let Some(mesh) = &snapshot.mesh {
                diagnostics.extend(wrong_kind("mesh", "mesh", &mesh.target));
            }
            if let Some(properties) = &snapshot.properties {
                diagnostics.extend(wrong_kind("properties", "value", &properties.target));
            }
            diagnostics
        }
    }

    static VALIDATOR_ENTRY: std::sync::OnceLock<SubsetValidatorEntry> = std::sync::OnceLock::new();
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioObjectValidator>)
    }
    //#endregion 🔖️SubsetValidator

    //#region 🔖️IoEntries
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn io_entries() -> &'static [ComposerEntry] {
        &[]
    }
    //#endregion 🔖️IoEntries

    //#region 🔖️Register
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::object::schema::semio_object_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioObjectSnapshot, crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation>(
            crate::standards::v1::subsets::object::schema::snapshot::STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA,
        ))
        .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.object.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::object::schema::inferences::semio_object_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
