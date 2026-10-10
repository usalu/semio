//! 🚪️ IO stdio.zip (2.0/🌐️iso21320) — reuses the 🧱️base subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `ZipSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! `🧱️base/🚪️io` already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v2_0::subsets::base::schema::snapshot::{ZipEntry, ZipSnapshot};
    use crate::standards::v2_0::subsets::base::io::ZipComposer as ZipAnyComposer;
    use crate::standards::v2_0::subsets::iso21320::io::check_iso21320_conformance;
    use crate::standards::v2_0::subsets::iso21320::io::check_iso21320_wire_conformance;
    use semio_framework_diagnostic::Diagnostic;

use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_ISO21320: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("iso21320") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Normalize
    /// 🧹 Retains authored member headers for the named semantic conformance gate.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn normalize_entry_for_iso21320(entry: &mut ZipEntry) {
        let _ = entry;
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
            let typed=match payload{IoPayload::Binary(bytes)=><ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),IoPayload::Text(text)=><ZipSnapshot as store::ArtifactDsl>::parse_dsl(text).ok()};
            if let Some(snapshot)=typed{return check_iso21320_conformance(&snapshot);}
            match payload{
                IoPayload::Binary(bytes) if matches!(crate::standards::v2_0::subsets::base::io::sniff_zip_bytes(bytes),crate::standards::v2_0::subsets::base::io::SniffConfidence::High|crate::standards::v2_0::subsets::base::io::SniffConfidence::Medium)=>check_iso21320_wire_conformance(bytes),
                _ => vec![Diagnostic {
                    code: FaultCode::new("stdio.zip.iso21320.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "ISO/IEC 21320-1 SubsetValidator: payload did not decode as a ZipSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
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

pub mod derived_construction {
    use crate::apply_mutation;
    use crate::standards::v2_0::subsets::base::schema::diff::ZipDiff;
    use crate::standards::v2_0::subsets::base::schema::snapshot::{ZipEntry, ZipSnapshot};
    use crate::standards::v2_0::subsets::iso21320::io::check_iso21320_conformance;
    use crate::standards::v2_0::subsets::iso21320::schema::mutations::{add_deflated_entry, add_stored_entry, ZipIso21320Mutation};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct ZipIso21320BuilderConstruction {
        snapshot: ZipSnapshot,
    }

    impl ZipIso21320BuilderConstruction {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: ZipSnapshot::default() }
        }

        /// ➕️ Adds a member this profile declares uncompressed (ISO/IEC 21320-1 §4.4 method 0).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            apply_mutation(&mut self.snapshot, &ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry: ZipEntry { name: name.into(), data, ..Default::default() }, before: None }));
            self
        }

        /// ➕️ Adds a member this profile declares Deflate-compressed (ISO/IEC 21320-1 §4.4 method 8).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            apply_mutation(&mut self.snapshot, &ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry: ZipEntry { name: name.into(), data, ..Default::default() }, before: None }));
            self
        }

        /// 💬️ Sets the archive-level (EOCD) comment.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.snapshot.comment = comment.into();
            self
        }
    }

    impl ArtifactBuilder for ZipIso21320BuilderConstruction {
        type Snapshot = ZipSnapshot;
        type Mutation = ZipIso21320Mutation;
        type Diff = ZipDiff;

        fn empty() -> Self {
            Self::new()
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
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
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v2_0::subsets::base::schema::snapshot::ZipSnapshot;
    use crate::standards::v2_0::subsets::base::io::{ZipAnalyzer as ZipAnyAnalyzer, ZipParts};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

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
    pub const CODE_COMPRESSION_METHOD:&str="stdio.zip.iso21320.compression-method-unsupported";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn check_iso21320_header(index:usize,name:&str,flags:u16,version_needed:u16,compression_method:u16,out:&mut Vec<Diagnostic>){
    if flags & FLAG_ENCRYPTED != 0 {
        out.push(hard(CODE_ENCRYPTED, format!("entry {index} ({:?}) has general-purpose bit 0 (encryption) set -- ISO/IEC 21320-1 §4.1 forbids encrypted entries", name)));
    }
    if flags & (FLAG_STRONG_ENCRYPTION | FLAG_MASKED_LOCAL_HEADERS) != 0 {
        out.push(hard(
            CODE_STRONG_ENCRYPTION,
            format!("entry {index} ({:?}) has general-purpose bit 6 and/or bit 13 (Strong Encryption / masked local header values) set -- ISO/IEC 21320-1 forbids the Strong Encryption extension entirely", name),
        ));
    }
    if flags & FLAG_DATA_DESCRIPTOR != 0 {
        out.push(soft(CODE_DATA_DESCRIPTOR, format!("entry {index} ({:?}) has general-purpose bit 3 (trailing data descriptor) set -- interoperability warning: not every ISO/IEC 21320-1 reader trusts streamed sizes", name)));
    }
    if version_needed > VERSION_NEEDED_SOFT_CEILING {
        out.push(soft(
            CODE_VERSION_NEEDED,
            format!("entry {index} ({:?}) declares version-needed-to-extract {} > {VERSION_NEEDED_SOFT_CEILING} -- signals a feature ISO/IEC 21320-1's restricted Stored/Deflate profile shouldn't require", name, version_needed),
        ));
    }
        if !matches!(compression_method,0|8){out.push(hard(CODE_COMPRESSION_METHOD,format!("entry {index} ({name:?}) declares compression method {compression_method} -- ISO/IEC 21320-1 §4.4 admits only Stored (0) and Deflate (8)")));}
    }

    fn check_iso21320_entry_headers(entries:&[crate::standards::v2_0::subsets::base::io::ZipCentralEntryHeader])->Vec<Diagnostic>{
        let mut out=Vec::new();
        for(index,entry)in entries.iter().enumerate(){check_iso21320_header(index,&entry.name,entry.flags,entry.version_needed,entry.compression_method,&mut out);}
        out
    }

    /// 🛡️ Checks ISO/IEC 21320-1 header policy against raw ZIP container bytes.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_iso21320_wire_conformance(data: &[u8]) -> Vec<Diagnostic> {
        match crate::standards::v2_0::subsets::base::io::inspect_zip_central_entry_headers(data) {
            Ok(headers) => check_iso21320_entry_headers(&headers),
            Err(err) => vec![hard("stdio.zip.iso21320.wire-inspect-failed", format!("ISO/IEC 21320-1 wire inspection failed: {err}"))],
        }
    }

    /// 🛡️ Checks both independently owned member headers without wire encoding.
    pub fn check_iso21320_conformance(snapshot:&ZipSnapshot)->Vec<Diagnostic>{
        let mut out=Vec::new();
        for(index,entry)in snapshot.entries.iter().enumerate(){check_iso21320_header(index,&entry.name,entry.metadata.local.flags|entry.metadata.central.flags,entry.metadata.local.version_needed.max(entry.metadata.central.version_needed),entry.metadata.compression_method,&mut out);}
        out
    }

    /// 🛡️ Bounds cancellation while checking explicit typed header policy.
    pub fn check_iso21320_conformance_controlled(snapshot:&ZipSnapshot,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,store::sqlite_snapshot::ValueError>{
        use store::sqlite_snapshot::SqliteSnapshotPhase;
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,snapshot.entries.len())?;
        let mut out=Vec::new();
        for(index,entry)in snapshot.entries.iter().enumerate(){
            check_iso21320_header(index,&entry.name,entry.metadata.local.flags|entry.metadata.central.flags,entry.metadata.local.version_needed.max(entry.metadata.central.version_needed),entry.metadata.compression_method,&mut out);
            if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,index+1,snapshot.entries.len())?;}
        }
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,snapshot.entries.len(),snapshot.entries.len())?;
        Ok(out)
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.zip` (2.0/🌐️iso21320): delegates the real parse to the 🧱️base subset's
    /// analyzer (same `ZipSnapshot`), then folds real ISO/IEC 21320-1 conformance diagnostics on top.
    /// `sniff` also delegates -- a subset-level sniff for `iso21320` is "is this recognizable as a
    /// ZIP container at all", the same magic/EOCD probe every 2.0 dialect shares; conformance is a
    /// separate, heavier question answered by `analyze`/`check_iso21320_conformance`.
    pub struct ZipIso21320AnalyzerAnalysis;

    impl ArtifactAnalysis for ZipIso21320AnalyzerAnalysis {
        type Parts = ZipParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            ZipAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = ZipAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            let mut wire_checked = false;
            for source in sources {
                if let AnalyzeSource::Binary(bytes) = source {
                    let checks = check_iso21320_wire_conformance(bytes);
                    if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                        confidence = semio_framework_plugin::io::Confidence::Low;
                    }
                    diagnostics.extend(checks);
                    wire_checked = true;
                }
            }
            if !wire_checked {
                if let Some(snapshot) = &inner.parts.snapshot {
                    let checks = check_iso21320_conformance(snapshot);
                    if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                        confidence = semio_framework_plugin::io::Confidence::Low;
                    }
                    diagnostics.extend(checks);
                }
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec ZipIso21320BuilderFacets {
        construction: ZipIso21320BuilderConstruction,
        analysis: ZipIso21320AnalyzerAnalysis,
        composition: crate::standards::v2_0::subsets::iso21320::io::derived_composition::ZipIso21320ComposerComposition,
    }
    builder: ZipIso21320Builder,
    analyzer: ZipIso21320Analyzer,
    composer: ZipIso21320Composer,
);
