//! 🚪️ IO stdio.zip (2.0/✳️iso21320) — reuses the ✳️any subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `ZipSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `✳️any/🚪️io` already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::{ZipEntry, ZipSnapshot};
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::ZipComposer as ZipAnyComposer;
    use crate::artifacts::zip::standards::v2_0::subsets::iso21320::schema::check_iso21320_conformance;
    use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
    use semio_framework_plugin::{register_subset_validator, subset_validator_entry_of, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_ISO21320: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("iso21320") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Normalize
    /// 🧹 Normalizes logical member state. Native compression and headers are fixed serializer policy.
    fn normalize_entry_for_iso21320(entry: &mut ZipEntry) {
        let _ = entry;
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

        fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <ZipSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_iso21320_conformance(&snapshot),
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

    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<ZipIso21320Validator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Formerly called from the 2.0 standard's own `⚙️engine::register()`
    /// (dissolved, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES); `zip::declaration()`
    /// now re-derives the same `SubsetValidatorEntry` directly via `subset_validator_entry_of::<
    /// ZipIso21320Validator>()` instead of calling this `register()`. The `ComposerEntry` itself is
    /// registered separately by the standard-level composer aggregator
    /// (`crate::artifacts::zip::standards::v2_0::subsets::any::io::io_registry::entries()`), matching
    /// how `✳️any`'s own entry is registered.
    pub fn register() {
        let _ = register_subset_validator(validator_entry());
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::zip::standards::v2_0::subsets::iso21320::schema::ZipIso21320BuilderConstruction as ZipIso21320Builder;
        
        use semio_framework_plugin::AnalyzeSource;
        use semio_framework_plugin::ArtifactBuilder as _;

        #[test]
        fn clean_snapshot_composes_and_stamps_iso21320() {
            let snapshot = ZipIso21320Builder::new().with_stored_entry("a.txt", b"hello".to_vec()).build().unwrap();
            let bytes = <ZipSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
            let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
            let composed = ZipIso21320ComposerComposition::compose(&sources).expect("clean archive must compose to iso21320");
            assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition
