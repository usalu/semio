//! 🧐️ ZipIso21320Analyzer (2.0/✳️iso21320) — real ISO/IEC 21320-1:2015 (Document Container
//! File, Part 1: Core) conformance checks against the retained `ZipSnapshot.entries` (per-entry
//! `flags`, `method`, `version_needed` -- all persisted, D2 ground rule). ISO/IEC 21320-1
//! restricts the general ZIP APPNOTE format to a small interoperable core used by container
//! formats like OPC (docx/xlsx/pptx) and EPUB.
//!
//! Checks implemented as real, honest scans (never fabricated against fields the engine doesn't
//! parse):
//! - HARD (blocks the `iso21320` dialect stamp): general-purpose bit 0 (`0x0001`, entry
//!   encryption) set on any entry -- ISO/IEC 21320-1 §4.1 forbids encrypted entries entirely.
//! - HARD: general-purpose bit 6 (`0x0040`, Strong Encryption) and/or bit 13 (`0x2000`,
//!   central-directory-encrypted / masked local-header values) set -- the APPNOTE Strong
//!   Encryption extension is forbidden outright, not merely restricted.
//! - SOFT: general-purpose bit 3 (`0x0008`, trailing data descriptor) set -- an interoperability
//!   warning, not a hard violation: APPNOTE itself permits streamed sizes, but not every
//!   ISO/IEC 21320-1 reader (this repo's own writer included, see `⚙️engine::encode_zip`) is
//!   willing to trust a size it can't seek back and verify.
//! - SOFT: `version_needed` (version-needed-to-extract) greater than 45 -- APPNOTE 4.4.3.2's
//!   ZIP64 threshold; a value this high signals a feature (ZIP64, strong encryption, ...)
//!   ISO/IEC 21320-1's restricted Stored/Deflate-only profile has no legitimate reason to need.
//!
//! Deliberately NOT a runtime check (tautology guard, per ticket
//! 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES's roster brief): "`method` outside
//! `{Stored, Deflate}`". `ZipCompressionMethod` (the retained field's own type, see
//! `✳️any/🧬️schema/📸️snapshot`) is a closed two-variant enum, and `ZipEngine::decode_zip`
//! already rejects any on-disk method code that isn't 0 (Stored) or 8 (Deflate) with
//! `ZipError::UnsupportedMethod` before a `ZipEntry` is ever constructed -- there is no reachable
//! `ZipEntry` whose `method` could fail this check, so emitting a diagnostic for it would be
//! fabricating a finding against a condition that can never occur.
//!
//! HONEST SCHEMA GAP (ticket roster): ISO/IEC 21320-1 §4.2 also constrains split/spanned
//! archives (the EOCD "number of this disk" / "disk where central directory starts" fields must
//! both be zero). `ZipSnapshot`/`ZipEntry` never retain a disk-number field at all --
//! `ZipEngine::decode_zip` already unconditionally rejects any non-zero disk-start it encounters
//! (`ZipError::UnsupportedMultiDisk`) before a snapshot is ever produced, so by the time a
//! `ZipSnapshot` exists to check, every archive it could have come from was already single-disk.
//! There is no field here to check post-hoc, and no honest way to check split/spanned-ness from
//! this schema -- this check is skipped rather than fabricated; noted in this ticket's
//! `w3-zip-report.json`'s `schema_gaps`, not emitted as a runtime diagnostic (unlike the PDF 1.4
//! `✳️a`/`✳️x` pass-through case, ISO/IEC 21320-1 is NOT broadly unverifiable from this
//! schema -- every other check above is real, so a subset-wide "unverifiable" diagnostic would
//! itself be dishonest).

use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalyzer, Dialect, IoConfidence, StandardId, SubsetId};
use crate::artifacts::zip::standards::v2_0::subsets::any::analyzer::{ZipAnalyzer as ZipAnyAnalyzer, ZipParts};
use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipSnapshot;

/// 🎯️ This subset's dialect coordinate.
pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("iso21320") };

//#region 🔖️Flags
/// 🚩️ General-purpose bit 0 -- entry is encrypted (APPNOTE 4.4.4).
pub const FLAG_ENCRYPTED: u16 = 0x0001;
/// 🚩️ General-purpose bit 3 -- sizes/CRC live in a trailing data descriptor (APPNOTE 4.4.4).
pub const FLAG_DATA_DESCRIPTOR: u16 = 0x0008;
/// 🚩️ General-purpose bit 6 -- Strong Encryption extension in use (APPNOTE 4.4.4).
pub const FLAG_STRONG_ENCRYPTION: u16 = 0x0040;
/// 🚩️ General-purpose bit 13 -- central directory encrypted / local header values masked
/// (APPNOTE 4.4.4, paired with bit 6's Strong Encryption extension).
pub const FLAG_MASKED_LOCAL_HEADERS: u16 = 0x2000;
/// 🔢️ APPNOTE 4.4.3.2's ZIP64 version-needed threshold -- above this, an entry is declaring a
/// feature ISO/IEC 21320-1's restricted profile has no honest reason to need.
pub const VERSION_NEEDED_SOFT_CEILING: u16 = 45;
//#endregion 🔖️Flags

//#region 🔖️Conformance
pub const CODE_ENCRYPTED: &str = "stdio.zip.iso21320.entry-encrypted";
pub const CODE_STRONG_ENCRYPTION: &str = "stdio.zip.iso21320.strong-encryption-or-masked-headers";
pub const CODE_DATA_DESCRIPTOR: &str = "stdio.zip.iso21320.data-descriptor-present";
pub const CODE_VERSION_NEEDED: &str = "stdio.zip.iso21320.version-needed-high";

fn hard(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

fn soft(code: &'static str, message: String) -> Diagnostic {
    Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
}

/// 🛡️ Real ISO/IEC 21320-1:2015 Core conformance checks against one already-decoded
/// `ZipSnapshot`. Shared single source of truth: `ZipIso21320Composer::compose` normalizes
/// against this same shape (see that module), `ZipIso21320Builder::build` hard-gates on it, and
/// the registered `SubsetValidator` re-runs it post-hoc against the wire payload.
pub fn check_iso21320_conformance(snapshot: &ZipSnapshot) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for (index, entry) in snapshot.entries.iter().enumerate() {
        if entry.flags & FLAG_ENCRYPTED != 0 {
            out.push(hard(
                CODE_ENCRYPTED,
                format!("entry {index} ({:?}) has general-purpose bit 0 (encryption) set -- ISO/IEC 21320-1 §4.1 forbids encrypted entries", entry.name),
            ));
        }
        if entry.flags & (FLAG_STRONG_ENCRYPTION | FLAG_MASKED_LOCAL_HEADERS) != 0 {
            out.push(hard(
                CODE_STRONG_ENCRYPTION,
                format!(
                    "entry {index} ({:?}) has general-purpose bit 6 and/or bit 13 (Strong Encryption / masked local header values) set -- ISO/IEC 21320-1 forbids the Strong Encryption extension entirely",
                    entry.name
                ),
            ));
        }
        if entry.flags & FLAG_DATA_DESCRIPTOR != 0 {
            out.push(soft(
                CODE_DATA_DESCRIPTOR,
                format!(
                    "entry {index} ({:?}) has general-purpose bit 3 (trailing data descriptor) set -- interoperability warning: not every ISO/IEC 21320-1 reader trusts streamed sizes",
                    entry.name
                ),
            ));
        }
        if entry.version_needed > VERSION_NEEDED_SOFT_CEILING {
            out.push(soft(
                CODE_VERSION_NEEDED,
                format!(
                    "entry {index} ({:?}) declares version-needed-to-extract {} > {VERSION_NEEDED_SOFT_CEILING} -- signals a feature ISO/IEC 21320-1's restricted Stored/Deflate profile shouldn't require",
                    entry.name, entry.version_needed
                ),
            ));
        }
        // 🛡️ Tautology guard: `entry.method` cannot legally hold any value outside
        // {Stored, Deflate} -- see module doc comment. Deliberately no diagnostic here.
    }
    out
}
//#endregion 🔖️Conformance

//#region 🔖️Analyzer
/// 🧐️ Analyzes `stdio.zip` (2.0/✳️iso21320): delegates the real parse to the ✳️any subset's
/// analyzer (same `ZipSnapshot`), then folds real ISO/IEC 21320-1 conformance diagnostics on top.
/// `sniff` also delegates -- a subset-level sniff for `iso21320` is "is this recognizable as a
/// ZIP container at all", the same magic/EOCD probe every 2.0 dialect shares; conformance is a
/// separate, heavier question answered by `analyze`/`check_iso21320_conformance`.
pub struct ZipIso21320Analyzer;

impl ArtifactAnalyzer for ZipIso21320Analyzer {
    type Parts = ZipParts;
    const DIALECT: Dialect = DIALECT;

    fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
        ZipAnyAnalyzer::sniff(source)
    }

    fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
        let inner = ZipAnyAnalyzer::analyze(sources);
        let mut diagnostics = inner.diagnostics.clone();
        let mut confidence = inner.confidence;
        if let Some(snapshot) = &inner.parts.snapshot {
            let checks = check_iso21320_conformance(snapshot);
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
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::{ZipCompressionMethod, ZipEntry};

    fn entry(name: &str) -> ZipEntry {
        ZipEntry { name: name.into(), data: b"payload".to_vec(), method: ZipCompressionMethod::Stored, ..Default::default() }
    }

    #[test]
    fn conforming_snapshot_has_no_diagnostics() {
        let snapshot = ZipSnapshot { entries: vec![entry("a.txt")], ..ZipSnapshot::default() };
        let diagnostics = check_iso21320_conformance(&snapshot);
        assert!(diagnostics.is_empty(), "got {diagnostics:?}");
    }

    #[test]
    fn encrypted_entry_is_hard() {
        let mut e = entry("secret.bin");
        e.flags |= FLAG_ENCRYPTED;
        let snapshot = ZipSnapshot { entries: vec![e], ..ZipSnapshot::default() };
        let diagnostics = check_iso21320_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_ENCRYPTED && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn strong_encryption_bit_is_hard() {
        let mut e = entry("strong.bin");
        e.flags |= FLAG_STRONG_ENCRYPTION;
        let snapshot = ZipSnapshot { entries: vec![e], ..ZipSnapshot::default() };
        let diagnostics = check_iso21320_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRONG_ENCRYPTION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn masked_local_header_bit_is_hard() {
        let mut e = entry("masked.bin");
        e.flags |= FLAG_MASKED_LOCAL_HEADERS;
        let snapshot = ZipSnapshot { entries: vec![e], ..ZipSnapshot::default() };
        let diagnostics = check_iso21320_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRONG_ENCRYPTION && d.severity == Severity::Error), "got {diagnostics:?}");
    }

    #[test]
    fn data_descriptor_bit_is_soft() {
        let mut e = entry("streamed.bin");
        e.flags |= FLAG_DATA_DESCRIPTOR;
        let snapshot = ZipSnapshot { entries: vec![e], ..ZipSnapshot::default() };
        let diagnostics = check_iso21320_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DATA_DESCRIPTOR && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn high_version_needed_is_soft() {
        let mut e = entry("zip64.bin");
        e.version_needed = 63;
        let snapshot = ZipSnapshot { entries: vec![e], ..ZipSnapshot::default() };
        let diagnostics = check_iso21320_conformance(&snapshot);
        assert!(diagnostics.iter().any(|d| d.code.0 == CODE_VERSION_NEEDED && d.severity == Severity::Warning), "got {diagnostics:?}");
    }

    #[test]
    fn version_needed_at_ceiling_is_clean() {
        let mut e = entry("boundary.bin");
        e.version_needed = VERSION_NEEDED_SOFT_CEILING;
        let snapshot = ZipSnapshot { entries: vec![e], ..ZipSnapshot::default() };
        assert!(check_iso21320_conformance(&snapshot).is_empty());
    }
}
