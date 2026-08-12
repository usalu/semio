//! 🧬️ ZipSnapshot schema (2.0/✳️iso21320) — reuses the ✳️any subset's `ZipSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.zip` schema id). ISO/IEC 21320-1:2015 (Document Container
//! File, Part 1: Core) is a validation-gated dialect STAMP on top of that existing schema, not a
//! new one -- see D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️iso21320/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::zip::standards::v2_0::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::diff::ZipDiff;
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::mutations::{apply_zip_mutation, ZipMutation};
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::{ZipCompressionMethod, ZipEntry, ZipSnapshot};
    use crate::artifacts::zip::standards::v2_0::subsets::iso21320::schema::check_iso21320_conformance;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct ZipIso21320BuilderConstruction {
        snapshot: ZipSnapshot,
    }

    impl ZipIso21320BuilderConstruction {
        pub fn new() -> Self {
            Self { snapshot: ZipSnapshot::default() }
        }

        /// ➕️ Adds a member stored with no compression (method 0), no forbidden flags, no elevated
        /// version-needed -- conforming by construction.
        pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            let index = self.snapshot.entries.len();
            apply_zip_mutation(
                &mut self.snapshot,
                &ZipMutation::AddEntry { index, entry: ZipEntry { name: name.into(), data, method: ZipCompressionMethod::Stored, ..Default::default() } },
            );
            self
        }

        /// ➕️ Adds a member compressed via the real deflate codec (method 8), no forbidden flags, no
        /// elevated version-needed -- conforming by construction.
        pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            let index = self.snapshot.entries.len();
            apply_zip_mutation(
                &mut self.snapshot,
                &ZipMutation::AddEntry { index, entry: ZipEntry { name: name.into(), data, method: ZipCompressionMethod::Deflate, ..Default::default() } },
            );
            self
        }

        /// 💬️ Sets the archive-level (EOCD) comment.
        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.snapshot.comment = comment.into();
            self
        }
    }

    impl ArtifactBuilder for ZipIso21320BuilderConstruction {
        type Snapshot = ZipSnapshot;
        type Mutation = ZipMutation;
        type Diff = ZipDiff;

        fn empty() -> Self {
            Self::new()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_zip_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <ZipDiff as protocol::MutationDiff<ZipSnapshot>>::apply(&diff, &self.snapshot);
            self
        }

        /// 🛡️ The real construction gate: however `self.snapshot` got here (typed constructors,
        /// `from_binary`, a raw `mutate(SetSnapshot { .. })`), a hard ISO/IEC 21320-1 violation fails
        /// `build()` -- the soft diagnostics (data descriptor, high version-needed) pass through as
        /// advisory `Diagnostic`s, only hard ones block.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_iso21320_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
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
        use crate::artifacts::zip::standards::v2_0::subsets::iso21320::schema::{CODE_ENCRYPTED, FLAG_ENCRYPTED};

        #[test]
        fn typed_constructors_build_clean() {
            let snapshot = ZipIso21320BuilderConstruction::new()
                .with_stored_entry("a.txt", b"hello".to_vec())
                .with_deflate_entry("b.txt", b"world, compressed".to_vec())
                .with_comment("archive")
                .build()
                .expect("conforming construction must build");
            assert_eq!(snapshot.entries.len(), 2);
            assert_eq!(snapshot.comment, "archive");
        }

        #[test]
        fn hard_violation_injected_via_raw_mutate_still_fails_build() {
            let built = ZipIso21320BuilderConstruction::new().with_stored_entry("a.txt", b"hello".to_vec()).build().unwrap();
            let mut snapshot = built;
            snapshot.entries[0].flags |= FLAG_ENCRYPTED;
            // Even routed back in via the generic `SetSnapshot` escape hatch, `build()` still catches it.
            let (mutated, _diff) = ZipIso21320BuilderConstruction::from_snapshot(ZipSnapshot::default()).mutate(ZipMutation::SetSnapshot { snapshot });
            let err = mutated.build().expect_err("an encrypted entry must fail build()");
            assert!(err.iter().any(|d| d.code.0 == CODE_ENCRYPTED));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{AnalyzeSource, Analysis, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::{ZipAnalyzer as ZipAnyAnalyzer, ZipParts};
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
    pub struct ZipIso21320AnalyzerAnalysis;

    impl ArtifactAnalysis for ZipIso21320AnalyzerAnalysis {
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
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec ZipIso21320BuilderFacets {
        construction: derived_construction::ZipIso21320BuilderConstruction,
        analysis: derived_analysis::ZipIso21320AnalyzerAnalysis,
        composition: crate::artifacts::zip::standards::v2_0::subsets::iso21320::io::derived_composition::ZipIso21320ComposerComposition,
    }
    builder: ZipIso21320Builder,
    analyzer: ZipIso21320Analyzer,
    composer: ZipIso21320Composer,
);
//#endregion 🧬️DerivedArtifactFacets
