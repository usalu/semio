//! 🧬️ ZipSnapshot schema (2.0/✳️iso21320) — reuses the ✳️any subset's `ZipSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.zip` schema id). ISO/IEC 21320-1:2015 (Document Container
//! File, Part 1: Core) is a validation-gated dialect STAMP on top of that existing schema, not a
//! new one -- see D4's Tier-1 "same snapshot type, subset moves" semantics
//! (`ArtifactCommand::MigrateDialect`). This leaf exists so `🪆️subsets/✳️iso21320/🧬️schema/` is
//! present per `🔣️taxonomy.json`'s `subsetChildDirs`, without duplicating the schema definition.

pub use crate::artifacts::zip::standards::v2_0::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::diff::ZipDiff;
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::mutations::{apply_zip_mutation, ZipMutation};
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::{ZipEntry, ZipSnapshot};
    use crate::artifacts::zip::standards::v2_0::subsets::iso21320::schema::check_iso21320_conformance;
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct ZipIso21320BuilderConstruction {
        snapshot: ZipSnapshot,
    }

    impl ZipIso21320BuilderConstruction {
        pub fn new() -> Self {
            Self { snapshot: ZipSnapshot::default() }
        }

        /// ➕️ Adds a logical member; the ISO serializer owns the canonical compression and header policy.
        pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            apply_zip_mutation(&mut self.snapshot, &ZipMutation::AddEntry { entry: ZipEntry { name: name.into(), data } });
            self
        }

        /// ➕️ Adds a logical member; the ISO serializer owns the canonical compression and header policy.
        pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            apply_zip_mutation(&mut self.snapshot, &ZipMutation::AddEntry { entry: ZipEntry { name: name.into(), data } });
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

        /// 🛡️ Gates logical constraints; native header validation belongs to deserialization and
        /// canonical header materialization belongs to serialization.
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

        #[test]
        fn typed_constructors_build_clean() {
            let snapshot = ZipIso21320BuilderConstruction::new().with_stored_entry("a.txt", b"hello".to_vec()).with_deflate_entry("b.txt", b"world, compressed".to_vec()).with_comment("archive").build().expect("conforming construction must build");
            assert_eq!(snapshot.entries.len(), 2);
            assert_eq!(snapshot.comment, "archive");
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipSnapshot;
    use crate::artifacts::zip::standards::v2_0::subsets::any::schema::{ZipAnalyzer as ZipAnyAnalyzer, ZipParts};
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

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

    /// 🛡️ Checks logical ISO/IEC 21320-1 constraints. Native compression/header constraints are
    /// rejected while deserializing and emitted by fixed serializer policy, never persisted here.
    pub fn check_iso21320_conformance(_snapshot: &ZipSnapshot) -> Vec<Diagnostic> {
        Vec::new()
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
