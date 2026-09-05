//! 🧬️ `ZipIso21320Mutation` — the ISO/IEC 21320-1:2015 mutation vocabulary. Handcrafted for THIS
//! subset, not inherited from `🧱️base`.
//!
//! ISO/IEC 21320-1 (Document Container File — Part 1: Core) is a RESTRICTION of the ZIP 2.0
//! container over the same `ZipSnapshot`. Its §4.4 admits exactly two compression methods, Stored
//! (0) and Deflate (8), out of the twenty-odd APPNOTE defines. That restriction is the profile, and
//! it is what this vocabulary makes representable: `🧱️base`'s `AddEntry` declares no method at all —
//! whichever one a member ends up with on the wire is a consequence of the canonical serializer —
//! whereas [`ZipIso21320Mutation::AddStoredEntry`] and [`ZipIso21320Mutation::AddDeflatedEntry`]
//! name it, and [`ZipIso21320Method`] makes every other ZIP method unrepresentable by construction.
//! The subset's own builder already declares that distinction as `with_stored_entry` /
//! `with_deflate_entry`; this is the mutation-side counterpart.
//!
//! ⚠️ HONEST LIMIT, recorded rather than papered over. The shared `ZipSnapshot` models a member as
//! `{name, data}` and nothing else — no compression method, no general-purpose flag bits, no
//! version-needed field. So the method this vocabulary declares is authoritative for a writer that
//! can honour it (the registered `zip` reference implementation does) and advisory for this
//! repository's own serializer, whose `canonical_compression_method` derives the method from the
//! member's filename extension instead. Every ISO/IEC 21320-1 constraint the subset's own
//! `check_iso21320_conformance` checks — the encryption bit, the Strong Encryption bit, the trailing
//! data descriptor, the version-needed ceiling — is likewise a WIRE property that no snapshot
//! mutation can address. Closing that gap means giving `ZipEntry` a native-header facet, which is a
//! schema change well outside this vocabulary.
//!
//! @see ../../🔣️oracle.json — the catalog `KINDS` below must match exactly.
//! @see ../../../../../../🧪️tests/🔀️mutate-zip-2-0-iso21320/🥒️.feature — the case that exercises it.

use crate::artifacts::zip::schema::diff::{self, ZipDiff};
use crate::artifacts::zip::schema::snapshot::ZipEntry;
use crate::artifacts::zip::ZipSnapshot;

//#region 🔖️Model
/// 🗜️ The two compression methods ISO/IEC 21320-1 §4.4 admits. Every other APPNOTE method is
/// unrepresentable here, which is precisely the profile this subset stamps.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum ZipIso21320Method {
    #[default]
    Stored,
    Deflate,
}

impl ZipIso21320Method {
    /// 🔢️ The APPNOTE 4.4.5 compression-method code this profile member carries on the wire.
    pub fn wire_code(&self) -> u16 {
        match self {
            ZipIso21320Method::Stored => 0,
            ZipIso21320Method::Deflate => 8,
        }
    }
}

//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "💬set-archive-comment/🦀️.rs"]
pub mod set_archive_comment;
#[path = "📦add-stored-entry/🦀️.rs"]
pub mod add_stored_entry;
#[path = "🗜️add-deflated-entry/🦀️.rs"]
pub mod add_deflated_entry;
#[path = "➖remove-entry/🦀️.rs"]
pub mod remove_entry;
#[path = "🏷️rename-entry/🦀️.rs"]
pub mod rename_entry;
#[path = "✍️set-entry-data/🦀️.rs"]
pub mod set_entry_data;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.zip` 2.0/🌐️iso21320. `NoMutation` was dropped:
/// `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one leaf payload and a unit
/// variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = ZipSnapshot, diff = ZipDiff, schema = "ZipIso21320Mutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum ZipIso21320Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 💬️ Sets the archive-level (EOCD) comment.
    SetArchiveComment(set_archive_comment::SetArchiveComment),
    /// ➕️ Adds a member the profile declares uncompressed (method 0).
    AddStoredEntry(add_stored_entry::AddStoredEntry),
    /// ➕️ Adds a member the profile declares Deflate-compressed (method 8).
    AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry),
    /// ➖️ Removes the member named `name`.
    RemoveEntry(remove_entry::RemoveEntry),
    /// 🏷️ Renames the member named `name`.
    RenameEntry(rename_entry::RenameEntry),
    /// ✍️ Replaces the decompressed payload of the member named `name`.
    SetEntryData(set_entry_data::SetEntryData),
}

/// 📇️ Kebab-case spelling of every `ZipIso21320Mutation` variant, in declaration order — the exact
/// `kinds` list `../../🔣️oracle.json`'s `mutationCatalogs` entry declares. The framework
/// never parses this enum; `kinds_matches_enum_variants_and_manifest` below is what keeps the two
/// declarations honest against each other.
pub const KINDS: &[&str] = &["set-snapshot", "set-archive-comment", "add-stored-entry", "add-deflated-entry", "remove-entry", "rename-entry", "set-entry-data"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant, exhaustively matched.
pub fn kind_of(mutation: &ZipIso21320Mutation) -> &'static str {
    match mutation {
        ZipIso21320Mutation::SetSnapshot(_) => "set-snapshot",
        ZipIso21320Mutation::SetArchiveComment(_) => "set-archive-comment",
        ZipIso21320Mutation::AddStoredEntry(_) => "add-stored-entry",
        ZipIso21320Mutation::AddDeflatedEntry(_) => "add-deflated-entry",
        ZipIso21320Mutation::RemoveEntry(_) => "remove-entry",
        ZipIso21320Mutation::RenameEntry(_) => "rename-entry",
        ZipIso21320Mutation::SetEntryData(_) => "set-entry-data",
    }
}

/// 🗜️ The method one authoring mutation declares for the member it adds, or `None` for the kinds
/// that add nothing.
pub fn declared_method(mutation: &ZipIso21320Mutation) -> Option<ZipIso21320Method> {
    match mutation {
        ZipIso21320Mutation::AddStoredEntry(_) => Some(ZipIso21320Method::Stored),
        ZipIso21320Mutation::AddDeflatedEntry(_) => Some(ZipIso21320Method::Deflate),
        _ => None,
    }
}
//#endregion 🔖️Model

//#region 🔖️Apply
const CODE_REJECTED: &str = "stdio.zip.iso21320.mutation-outside-profile";

/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source, never a separate
/// imperative apply path.
pub fn apply_zip_iso21320_mutation(snapshot: &mut ZipSnapshot, mutation: &ZipIso21320Mutation) -> protocol::MutationOutcome<ZipDiff> {
    let outcome = <ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// ↩️ This subset's own inverse algebra as a free function, so a caller that legitimately drives the
/// vocabulary from outside the crate can reach it without naming the `protocol::Mutation` trait.
pub fn inverse_zip_iso21320_mutation(mutation: &ZipIso21320Mutation, base: &ZipSnapshot) -> Vec<ZipIso21320Mutation> {
    <ZipIso21320Mutation as protocol::Mutation<ZipSnapshot>>::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &ZipIso21320Mutation, base: &ZipSnapshot) -> protocol::MutationOutcome<ZipDiff> {
    match this {
        ZipIso21320Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => protocol::MutationOutcome::new(diff::diff_set_snapshot(base, snapshot)),
        ZipIso21320Mutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment }) => protocol::MutationOutcome::new(diff::diff_set_archive_comment(comment)),
        ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry }) | ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry }) => {
            if base.entries.iter().any(|existing| existing.name == entry.name) {
                return protocol::MutationOutcome::error(CODE_REJECTED, format!("a member named {:?} already exists -- ISO/IEC 21320-1 containers address members by name", entry.name), [entry.name.clone()]);
            }
            protocol::MutationOutcome::new(diff::diff_add_entry(entry.clone()))
        }
        ZipIso21320Mutation::RemoveEntry(remove_entry::RemoveEntry { name }) => protocol::MutationOutcome::new(diff::diff_remove_entry(name)),
        ZipIso21320Mutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => {
            if base.entries.iter().any(|existing| existing.name == *new_name) {
                return protocol::MutationOutcome::error(CODE_REJECTED, format!("a member named {new_name:?} already exists"), [new_name.clone()]);
            }
            protocol::MutationOutcome::new(diff::diff_rename_entry(name, new_name))
        }
        ZipIso21320Mutation::SetEntryData(set_entry_data::SetEntryData { name, data }) => protocol::MutationOutcome::new(diff::diff_set_entry_data(name, data.clone())),
    }
}

/// ↩️ An added member is undone by removing it; which of the two profile methods declared it is
/// irrelevant to the undo, so both add kinds share one inverse.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &ZipIso21320Mutation, base: &ZipSnapshot) -> Vec<ZipIso21320Mutation> {
    match this {
        ZipIso21320Mutation::SetSnapshot(_) => vec![ZipIso21320Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        ZipIso21320Mutation::SetArchiveComment(_) => vec![ZipIso21320Mutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: base.comment.clone() })],
        ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry }) => vec![ZipIso21320Mutation::RemoveEntry(remove_entry::RemoveEntry { name: entry.name.clone() })],
        ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry }) => vec![ZipIso21320Mutation::RemoveEntry(remove_entry::RemoveEntry { name: entry.name.clone() })],
        ZipIso21320Mutation::RemoveEntry(remove_entry::RemoveEntry { name }) => base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry: entry.clone() })]).unwrap_or_default(),
        ZipIso21320Mutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => vec![ZipIso21320Mutation::RenameEntry(rename_entry::RenameEntry { name: new_name.clone(), new_name: name.clone() })],
        ZipIso21320Mutation::SetEntryData(set_entry_data::SetEntryData { name, .. }) => base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![ZipIso21320Mutation::SetEntryData(set_entry_data::SetEntryData { name: name.clone(), data: entry.data.clone() })]).unwrap_or_default(),
    }
}
//#endregion 🔖️MutationTrait

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation as _;

    fn entry(name: &str, data: &[u8]) -> ZipEntry {
        ZipEntry { name: name.into(), data: data.to_vec() }
    }

    fn base_snapshot() -> ZipSnapshot {
        ZipSnapshot { schema: "stdio.zip".into(), entries: vec![entry("bild.jpg", b"jpegbytes"), entry("notiz.txt", b"text")], comment: "Bestand".into() }
    }

    fn every_kind() -> Vec<ZipIso21320Mutation> {
        vec![
            ZipIso21320Mutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base_snapshot() }),
            ZipIso21320Mutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "geaendert".into() }),
            ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry: entry("beleg.png", b"png") }),
            ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry: entry("beleg.txt", b"text") }),
            ZipIso21320Mutation::RemoveEntry(remove_entry::RemoveEntry { name: "notiz.txt".into() }),
            ZipIso21320Mutation::RenameEntry(rename_entry::RenameEntry { name: "notiz.txt".into(), new_name: "notiz2.txt".into() }),
            ZipIso21320Mutation::SetEntryData(set_entry_data::SetEntryData { name: "notiz.txt".into(), data: b"anders".to_vec() }),
        ]
    }

    /// 📇️ The one test that keeps `KINDS` honest against the enum it claims to spell. The framework
    /// never parses Rust, so this is the only thing standing between a renamed variant and a catalog
    /// that silently measures the wrong vocabulary.
    #[test]
    fn kinds_matches_enum_variants_and_manifest() {
        let spelled: Vec<&'static str> = every_kind().iter().map(kind_of).collect();
        assert_eq!(spelled, KINDS.to_vec(), "KINDS must spell every variant, in declaration order");

        let manifest = include_str!("../../🔮️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle manifest's catalog does not declare {kind:?}");
        }
    }

    #[test]
    fn only_the_two_iso_methods_exist_and_carry_their_wire_codes() {
        assert_eq!(ZipIso21320Method::Stored.wire_code(), 0);
        assert_eq!(ZipIso21320Method::Deflate.wire_code(), 8);
        assert_eq!(declared_method(&ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry: entry("a.png", b"") })), Some(ZipIso21320Method::Stored));
        assert_eq!(declared_method(&ZipIso21320Mutation::AddDeflatedEntry(add_deflated_entry::AddDeflatedEntry { entry: entry("a.txt", b"") })), Some(ZipIso21320Method::Deflate));
    }

    #[test]
    fn every_declared_kind_is_invertible_against_the_real_base() {
        for mutation in every_kind() {
            let base = base_snapshot();
            let mut snapshot = base.clone();
            let undo = mutation.inverse(&base);
            apply_zip_iso21320_mutation(&mut snapshot, &mutation);
            for step in &undo {
                apply_zip_iso21320_mutation(&mut snapshot, step);
            }
            let mut restored = snapshot.entries.clone();
            let mut original = base.entries.clone();
            restored.sort_by(|a, b| a.name.cmp(&b.name));
            original.sort_by(|a, b| a.name.cmp(&b.name));
            assert_eq!(restored, original, "{} is not invertible", kind_of(&mutation));
            assert_eq!(snapshot.comment, base.comment, "{} did not restore the archive comment", kind_of(&mutation));
        }
    }

    #[test]
    fn adding_a_member_that_already_exists_is_rejected() {
        let mut snapshot = base_snapshot();
        let outcome = apply_zip_iso21320_mutation(&mut snapshot, &ZipIso21320Mutation::AddStoredEntry(add_stored_entry::AddStoredEntry { entry: entry("bild.jpg", b"other") }));
        assert!(!outcome.messages().is_empty(), "a duplicate member name must be rejected");
        assert_eq!(snapshot, base_snapshot(), "the archive must be untouched");
    }
}
