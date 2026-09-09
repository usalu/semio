//! 🚪️ IO — composer + subset validator registration for `s.stdio.semio.kit`, mirroring every
//! other semio subset's convention. Registration flows through `register()`, called from this
//! standard's `⚙️engine::register()`.
//!
//! ⚠️ OUT OF SCOPE for this wave (deliberately, per this ticket's brief, same as `🔤️text`/`📦️object`):
//! the `📥️import`/`📤️export` leaves bridging `kit` to any format artifact, and dissolving puzzle/
//! three-block's separately-declared `kit.catalog` artifact kind into this subset (a later wave's
//! concern — repointing those apps' `AppSchema::artifact_kind()` registrations). `io_entries()` is
//! empty; `reads()` only advertises this subset's own native dialect.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
    use crate::standards::v1::subsets::kit::schema::SemioKitAnalyzer;
    use semio_framework_plugin::{
        register_composer_entries, register_subset_validator, subset_validator_entry_of, AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, ComposerEntry, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator,
        SubsetValidatorEntry,
    };

    const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("kit") };

    //#region 🔖️Composer
    pub struct SemioKitComposerComposition;

    impl ArtifactComposition for SemioKitComposerComposition {
        type Snapshot = SemioKitSnapshot;
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
                return Err(ComposeError { message: "SemioKitComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() });
            }
            let analysis = SemioKitAnalyzer::analyze(&native);
            let snapshot = analysis.parts.snapshot.ok_or_else(|| ComposeError { message: "SemioKitComposerComposition: analysis produced no snapshot".into(), diagnostics: analysis.diagnostics.clone() })?;
            Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ Decode PLUS a real referential-invariant check on the owned CHILD slots: every
    /// `objects`/`models`/`properties` handle's `target` dialect must name the kind the slot
    /// declares — same convention `📦️object`'s own validator (this ticket's first composite subset)
    /// established. The LINK pool (`representations`) is intentionally NOT kind-checked here — a
    /// link may legitimately point at any independent artifact kind (image/mesh/whatever a
    /// catalog's representation happens to be), so there is no single expected kind to assert.
    pub struct SemioKitValidator;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn wrong_kind(field: &str, expected_subset: &str, target: &store::os_io::ArtifactRef) -> Option<dsl::Diagnostic> {
        if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.subset != expected_subset {
            Some(dsl::Diagnostic::error(
                "stdio.semio_kit.validate-child-kind-mismatch",
                dsl::TextSpan::at(1, 1),
                format!("SemioKitValidator: `{field}` handle targets {}@{}/{}, expected kind s.stdio.semio subset {expected_subset}", target.dialect.artifact_kind, target.dialect.standard, target.dialect.subset),
            ))
        } else {
            None
        }
    }

    impl SubsetValidator for SemioKitValidator {
        const DIALECT: Dialect = DIALECT;
        async fn validate(payload: &IoPayload) -> Vec<dsl::Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            let Some(snapshot) = decoded else {
                return vec![dsl::Diagnostic::error("stdio.semio_kit.validate-decode-failed", dsl::TextSpan::at(1, 1), "SemioKitValidator: payload did not decode as a SemioKitSnapshot".to_string())];
            };
            let mut diagnostics = Vec::new();
            for object in &snapshot.objects {
                diagnostics.extend(wrong_kind("objects", "object", &object.target));
            }
            for model in &snapshot.models {
                diagnostics.extend(wrong_kind("models", "model", &model.target));
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
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<SemioKitValidator>)
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
        ::framework_schema::register_artifact_schema_descriptor(crate::standards::v1::subsets::kit::schema::semio_kit_artifact_schema_descriptor());
        store::register_document_codec(store::ArtifactCodec::of::<SemioKitSnapshot, crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation>(crate::standards::v1::subsets::kit::schema::snapshot::STDIO_SEMIOKIT_DOCUMENT_SCHEMA))
            .expect("static Stdio registration must be available and conflict-free");
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
        register_composer_entries(io_entries()).expect("static Stdio registration must be available and conflict-free");
        register_artifact_inferences();
    }

    /// 💡️ Registers `s.stdio.semio.kit.inference`'s facet leaves into the OS-wide inference
    /// catalog — sibling to `register_artifact_schema_descriptor` above (separate registry,
    /// ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register_artifact_inferences() {
        ::framework_schema::register_artifact_inference_descriptor(crate::standards::v1::subsets::kit::schema::inferences::semio_kit_artifact_inference_descriptor());
    }
    //#endregion 🔖️Register

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
