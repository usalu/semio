//! 🏗️ ZipIso21320Builder (2.0/✳️iso21320) — a typed builder whose ergonomic path
//! (`with_stored_entry`/`with_deflate_entry`) can only produce entries with `flags: 0` and
//! `version_needed: 0` (defaulted), which already satisfies ISO/IEC 21320-1 Core -- there is no
//! `with_encrypted_entry`/`with_strong_encryption` method anywhere on this type. Unlike
//! `ZipIso21320Composer` (which actively normalizes whatever it's handed, see that module's doc
//! comment), `build()` here is a pure HARD GATE: it re-runs `check_iso21320_conformance`
//! unconditionally and rejects rather than repairs, so a hard violation injected via the general
//! escape hatches (`with_entry`, raw `mutate(SetSnapshot { .. })`) can never leave this builder
//! as an `Ok(ZipSnapshot)`.

use dsl::{Diagnostic, Severity};
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::zip::standards::v2_0::subsets::any::schema::diff::ZipDiff;
use crate::artifacts::zip::standards::v2_0::subsets::any::schema::mutations::{apply_zip_mutation, ZipMutation};
use crate::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::{ZipCompressionMethod, ZipEntry, ZipSnapshot};
use crate::artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::check_iso21320_conformance;

//#region 🔖️Builder
#[derive(Clone, Debug, Default)]
pub struct ZipIso21320Builder {
    snapshot: ZipSnapshot,
}

impl ZipIso21320Builder {
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

impl ArtifactBuilder for ZipIso21320Builder {
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
    use crate::artifacts::zip::standards::v2_0::subsets::iso21320::analyzer::{CODE_ENCRYPTED, FLAG_ENCRYPTED};

    #[test]
    fn typed_constructors_build_clean() {
        let snapshot = ZipIso21320Builder::new()
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
        let built = ZipIso21320Builder::new().with_stored_entry("a.txt", b"hello".to_vec()).build().unwrap();
        let mut snapshot = built;
        snapshot.entries[0].flags |= FLAG_ENCRYPTED;
        // Even routed back in via the generic `SetSnapshot` escape hatch, `build()` still catches it.
        let (mutated, _diff) = ZipIso21320Builder::from_snapshot(ZipSnapshot::default()).mutate(ZipMutation::SetSnapshot { snapshot });
        let err = mutated.build().expect_err("an encrypted entry must fail build()");
        assert!(err.iter().any(|d| d.code.0 == CODE_ENCRYPTED));
    }
}
