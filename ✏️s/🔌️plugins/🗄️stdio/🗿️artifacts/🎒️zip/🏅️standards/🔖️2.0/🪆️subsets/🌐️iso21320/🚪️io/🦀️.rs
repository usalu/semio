//! 🚪️ IO stdio.zip (2.0/🌐️iso21320) — reuses the 🧱️base subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `ZipSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `🧱️base/🚪️io` already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2_0::subsets::base::schema::snapshot::{ZipEntry, ZipSnapshot};
    use crate::standards::v2_0::subsets::base::schema::ZipComposer as ZipAnyComposer;
    use crate::standards::v2_0::subsets::iso21320::schema::check_iso21320_conformance;
    use crate::standards::v2_0::subsets::iso21320::schema::check_iso21320_wire_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_ISO21320: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("iso21320") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Normalize
    /// 🧹 Logical snapshots carry no native header fields — canonical serialization policy already
    /// emits conforming Stored/Deflate headers. Retained as the composer's normalization hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn normalize_entry_for_iso21320(entry: &mut ZipEntry) {
        let _ = entry;
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn zip_wire_bytes_from_payload(payload: &IoPayload) -> Option<Vec<u8>> {
        match payload {
            IoPayload::Binary(bytes) => {
                if let Ok((_, inner)) = store::semio_format::unwrap_binary(bytes) {
                    Some(inner.to_vec())
                } else if matches!(crate::standards::v2_0::subsets::base::io::sniff_zip_bytes(bytes), crate::standards::v2_0::subsets::base::io::SniffConfidence::High | crate::standards::v2_0::subsets::base::io::SniffConfidence::Medium) {
                    Some(bytes.to_vec())
                } else {
                    None
                }
            }
            IoPayload::Text(text) => <ZipSnapshot as store::ArtifactDsl>::parse_dsl(text).ok().and_then(|snapshot| crate::standards::v2_0::subsets::base::io::encode_zip(&snapshot).ok()),
        }
    }
    //#endregion 🔖️Normalize

    //#region 🔖️Composer
    pub struct ZipIso21320ComposerComposition;

    impl ArtifactComposition for ZipIso21320ComposerComposition {
        type Snapshot = ZipSnapshot;
        const WRITES: Dialect = DIALECT_ISO21320;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_ISO21320, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = ZipAnyComposer::compose(sources)?;
            let mut snapshot = inner.snapshot;
            for entry in &mut snapshot.entries {
                normalize_entry_for_iso21320(entry);
            }
            let checks = check_iso21320_conformance(&snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                // 🛡️ Defensive logical gate; native constraints are enforced at the IO boundary.
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("ISO/IEC 21320-1 normalization left {} hard issue(s) -- not stamping iso21320", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `2.0/iso21320` -- see the module doc comment for how
    /// this (a raw, non-normalizing recheck) honestly differs from the composer's own
    /// normalize-then-defensively-gate path above.
    pub struct ZipIso21320Validator;

    impl SubsetValidator for ZipIso21320Validator {
        const DIALECT: Dialect = DIALECT_ISO21320;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            match zip_wire_bytes_from_payload(payload) {
                Some(bytes) => check_iso21320_wire_conformance(&bytes),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.zip.iso21320.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "ISO/IEC 21320-1 SubsetValidator: payload did not decode as a ZipSnapshot -- skipped".into(),
                    expected: None,
                    scope: dsl::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<ZipIso21320Validator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Formerly called from the 2.0 standard's own `⚙️engine::register()`
    /// (dissolved, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); `zip::declaration()`
    /// now re-derives the same `SubsetValidatorEntry` directly via `subset_validator_entry_of::<
    /// ZipIso21320Validator>()` instead of calling this `register()`. The `ComposerEntry` itself is
    /// registered separately by the standard-level composer aggregator
    /// (`crate::standards::v2_0::subsets::base::io::io_registry::entries()`), matching
    /// how `🧱️base`'s own entry is registered.
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
