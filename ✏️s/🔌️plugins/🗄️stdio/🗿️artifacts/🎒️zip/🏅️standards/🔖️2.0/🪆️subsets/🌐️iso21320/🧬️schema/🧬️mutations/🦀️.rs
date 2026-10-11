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
//! The subset's own builder and mutation algebra persist that choice in `ZipEntry.metadata`.
//!
//! @see ../../🔣️oracle.json — the catalog `KINDS` below must match exactly.
//! @see ../../../../../../🧪️tests/🔀️mutate-zip-2-0-iso21320/🥒️.feature — the case that exercises it.

use crate::schema::diff::{self, ZipDiff};
use crate::schema::snapshot::ZipEntry;
use crate::ZipSnapshot;

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
#[path = "🗜️add-deflated-entry/🦀️.rs"]
pub mod add_deflated_entry;
#[path = "📦add-stored-entry/🦀️.rs"]
pub mod add_stored_entry;
#[path = "➖remove-entry/🦀️.rs"]
pub mod remove_entry;
#[path = "🏷️rename-entry/🦀️.rs"]
pub mod rename_entry;
#[path = "💬set-archive-comment/🦀️.rs"]
pub mod set_archive_comment;
#[path = "✍️set-entry-data/🦀️.rs"]
pub mod set_entry_data;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.zip` 2.0/🌐️iso21320. `NoMutation` was dropped:
/// `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one leaf payload and a unit
/// variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = ZipSnapshot, diff = ZipDiff, schema = "ZipIso21320Mutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum ZipIso21320Mutation {
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
pub const KINDS: &[&str] = &["set-archive-comment", "add-stored-entry", "add-deflated-entry", "remove-entry", "rename-entry", "set-entry-data"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant, exhaustively matched.
pub fn kind_of(mutation: &ZipIso21320Mutation) -> &'static str {
    match mutation {
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


//#endregion 🔖️Apply


//#region 🔖️AddEntry
/// ➕️ The diff both authoring kinds share: the member lands under the compression method the kind declares, refused when its name
/// is taken or its insertion anchor is gone.
fn added_entry_diff(base: &ZipSnapshot, entry: &ZipEntry, before: Option<&str>, method: ZipIso21320Method) -> protocol::MutationOutcome<ZipDiff> {
    if base.entries.iter().any(|existing| existing.name == entry.name) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("a member named {:?} already exists -- ISO/IEC 21320-1 containers address members by name", entry.name), [entry.name.clone()]);
    }
    if before.is_some_and(|name| !base.entries.iter().any(|entry| entry.name == name)) {
        return protocol::MutationOutcome::error("mutation.target-missing", "ZIP insertion anchor no longer exists", ["entries"]);
    }
    let entry = ZipEntry { metadata: crate::schema::snapshot::ZipEntryMetadata { compression_method: method.wire_code(), ..entry.metadata.clone() }, ..entry.clone() };
    protocol::MutationOutcome::new(diff::diff_add_entry(base, entry, before))
}
//#endregion 🔖️AddEntry

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
