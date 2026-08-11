//! 🎹️ ZipIso21320Composer (2.0/✳️iso21320) — reads the same sources the ✳️any subset does
//! (native `stdio.zip` 2.0, plus its `binary`/`deflate` DAG deps), delegates the actual parse to
//! the ✳️any composer, then fulfils this ticket's roster composer duty: NORMALIZES every entry to
//! ISO/IEC 21320-1 Core conformance by construction -- clears every forbidden general-purpose
//! flag bit and caps `version_needed` at 20 -- rather than rejecting non-conforming input
//! outright. This differs from the PDF `✳️a` family's pure hard-gate shape because every forbidden
//! bit here is genuinely metadata this writer controls (real ZIP encryption/streaming behavior
//! lives in the compressed payload bytes + flag bit together, and this artifact's own engine
//! already never round-trips ciphertext -- `⚙️engine::decode_zip` has no decryption path at all),
//! so silently-fabricated non-conformant metadata can always be corrected without touching entry
//! payload bytes. `check_iso21320_conformance` is still run AFTER normalization as a defensive
//! gate (should always come back clean; see `compose`'s doc comment on the `Err` branch) --
//! keeping the same Result-typed hard/soft shape every other real subset's composer uses.
//!
//! Also registers this dialect's `SubsetValidator` (D5's generic validate-on-build hook) — the
//! SAME `check_iso21320_conformance` function backs both: the defensive gate here runs
//! post-normalization against the typed `ZipSnapshot`, while the registered validator re-runs it
//! post-hoc, WITHOUT normalizing, against the wire `IoPayload` for the generic
//! `io_dispatch`/`wire_artifact_compose` hook -- so an archive that reaches the validator without
//! ever having gone through this composer (e.g. a `iso21320`-labeled payload authored elsewhere)
//! is still honestly flagged if it carries forbidden bits.

use std::sync::OnceLock;
use dsl::{Diagnostic, FaultCode, Severity, TextSpan};
use semio_framework_plugin::{
    ArtifactComposer, ComposeError, ComposeSource, Composition, Dialect, IoPayload, StandardId, SubsetId, SubsetValidator, SubsetValidatorEntry,
    register_subset_validator, subset_validator_entry_of,
};
use crate::artifacts::zip::standards::v2_0::subsets::any::composer::ZipComposer as ZipAnyComposer;
use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::{ZipEntry, ZipSnapshot};
use crate::artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::{
    check_iso21320_conformance, FLAG_DATA_DESCRIPTOR, FLAG_ENCRYPTED, FLAG_MASKED_LOCAL_HEADERS, FLAG_STRONG_ENCRYPTION,
};

const DIALECT_ISO21320: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("iso21320") };
const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

/// 🔢️ The version-needed ceiling this composer normalizes down to -- APPNOTE's classic-zip
/// baseline, always sufficient for a Stored/Deflate-only, non-encrypted archive.
const NORMALIZED_VERSION_NEEDED: u16 = 20;

//#region 🔖️Normalize
/// 🧹 Normalizes one entry to ISO/IEC 21320-1 Core conformance BY CONSTRUCTION: clears every
/// forbidden general-purpose flag bit, and caps `version_needed` at 20. `method` is untouched --
/// it's already constrained to `{Stored, Deflate}` by `ZipCompressionMethod`'s own type, see the
/// analyzer module doc comment's tautology-guard note.
fn normalize_entry_for_iso21320(entry: &mut ZipEntry) {
    entry.flags &= !(FLAG_ENCRYPTED | FLAG_DATA_DESCRIPTOR | FLAG_STRONG_ENCRYPTION | FLAG_MASKED_LOCAL_HEADERS);
    if entry.version_needed > NORMALIZED_VERSION_NEEDED {
        entry.version_needed = NORMALIZED_VERSION_NEEDED;
    }
}
//#endregion 🔖️Normalize

//#region 🔖️Composer
pub struct ZipIso21320Composer;

impl ArtifactComposer for ZipIso21320Composer {
    type Snapshot = ZipSnapshot;
    const WRITES: Dialect = DIALECT_ISO21320;

    fn reads() -> &'static [Dialect] {
        &[DIALECT_ANY, DIALECT_ISO21320, DEP_BINARY, DEP_DEFLATE]
    }

    fn compose(sources: &[ComposeSource]) -> Result<Composition<Self::Snapshot>, ComposeError> {
        let inner = ZipAnyComposer::compose(sources)?;
        let mut snapshot = inner.snapshot;
        for entry in &mut snapshot.entries {
            normalize_entry_for_iso21320(entry);
        }
        let checks = check_iso21320_conformance(&snapshot);
        let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
        if !hard.is_empty() {
            // 🛡️ Defensive only: `normalize_entry_for_iso21320` clears every bit
            // `check_iso21320_conformance` treats as HARD and caps `version_needed` well under
            // the SOFT check's threshold too, so this branch should be unreachable in practice.
            // Kept so a future normalization bug fails loudly (a real `ComposeError`) instead of
            // silently stamping a non-conforming dialect.
            let mut all = hard.clone();
            all.extend(soft);
            return Err(ComposeError {
                message: format!("ISO/IEC 21320-1 normalization left {} hard issue(s) -- not stamping iso21320", hard.len()),
                diagnostics: all,
            });
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
/// validate-on-build hook). Called from the 2.0 standard's own `⚙️engine::register()`. The
/// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
/// (`crate::artifacts::zip::standards::v2_0::composer::entries()`), matching how `✳️any`'s own
/// entry is registered.
pub fn register() {
    register_subset_validator(validator_entry());
}
//#endregion 🔖️SubsetValidator

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::AnalyzeSource;
    use crate::artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::CODE_ENCRYPTED;
    use crate::artifacts::zip::standards::v2_0::subsets::iso21320::builder::ZipIso21320Builder;
    use semio_framework_plugin::ArtifactBuilder as _;

    #[test]
    fn clean_snapshot_composes_and_stamps_iso21320() {
        let snapshot = ZipIso21320Builder::new().with_stored_entry("a.txt", b"hello".to_vec()).build().unwrap();
        let bytes = <ZipSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = ZipIso21320Composer::compose(&sources).expect("clean archive must compose to iso21320");
        assert!(composed.diagnostics.iter().all(|d| d.severity != Severity::Error), "no hard diagnostics expected: {:?}", composed.diagnostics);
    }

    #[test]
    fn encrypted_entry_gets_normalized_away_and_still_composes() {
        let mut snapshot = ZipIso21320Builder::new().with_stored_entry("secret.bin", b"payload".to_vec()).build().unwrap();
        snapshot.entries[0].flags |= FLAG_ENCRYPTED;
        let bytes = <ZipSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = ZipIso21320Composer::compose(&sources).expect("normalization must clear the forbidden bit before the defensive gate runs");
        assert_eq!(composed.snapshot.entries[0].flags & FLAG_ENCRYPTED, 0, "composer must clear the encryption bit");
        assert!(composed.diagnostics.iter().all(|d| d.code.0 != CODE_ENCRYPTED), "normalized output must not still report the cleared violation");
    }

    #[test]
    fn high_version_needed_gets_capped() {
        let mut snapshot = ZipIso21320Builder::new().with_stored_entry("f.bin", b"x".to_vec()).build().unwrap();
        snapshot.entries[0].version_needed = 63;
        let bytes = <ZipSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let sources = vec![ComposeSource { dialect: DIALECT_ANY, payload: AnalyzeSource::Binary(&bytes) }];
        let composed = ZipIso21320Composer::compose(&sources).expect("capping version_needed must not fail composition");
        assert_eq!(composed.snapshot.entries[0].version_needed, NORMALIZED_VERSION_NEEDED);
    }

    #[test]
    fn subset_validator_flags_real_violations_without_normalizing() {
        // The validator, called directly (same fn the generic io hook calls): unlike the
        // composer, it never normalizes -- a wire payload with a forbidden bit set is honestly
        // reported, not silently repaired.
        let mut snapshot = ZipIso21320Builder::new().with_stored_entry("secret.bin", b"payload".to_vec()).build().unwrap();
        snapshot.entries[0].flags |= FLAG_ENCRYPTED;
        let bytes = <ZipSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
        let diagnostics = ZipIso21320Validator::validate(&IoPayload::Binary(bytes));
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPTED && d.severity == Severity::Error), "got {diagnostics:?}");
    }
}
