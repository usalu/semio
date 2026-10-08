//! 🧬️ Logical ZIP mutations over member names, decompressed payloads, ordering, and archive comment.

use crate::schema::diff::{self, ZipDiff};
use crate::schema::snapshot::ZipEntry;
use crate::ZipSnapshot;

//#region 🔖️Model
//#region 🔖️Leaves
#[path = "➕add-entry/🦀️.rs"]
pub mod add_entry;
#[path = "➖remove-entry/🦀️.rs"]
pub mod remove_entry;
#[path = "🏷️rename-entry/🦀️.rs"]
pub mod rename_entry;
#[path = "💬set-archive-comment/🦀️.rs"]
pub mod set_archive_comment;
#[path = "✍️set-entry-data/🦀️.rs"]
pub mod set_entry_data;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = ZipSnapshot, diff = ZipDiff, schema = "ZipMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum ZipMutation {
    SetArchiveComment(set_archive_comment::SetArchiveComment),
    AddEntry(add_entry::AddEntry),
    RemoveEntry(remove_entry::RemoveEntry),
    RenameEntry(rename_entry::RenameEntry),
    SetEntryData(set_entry_data::SetEntryData),
}
//#endregion 🔖️Model

//#region 🔖️Kinds
/// 🦠️ Kebab-case spelling of every `ZipMutation` variant, in declaration order — the exact `kinds`
/// list `../../🔣️oracle.json`'s `mutationCatalogs` entry must declare. The framework
/// never parses this enum; `kinds_matches_enum_variants_and_manifest` below is what keeps the two
/// declarations honest against each other.
pub const KINDS: &[&str] = &["set-archive-comment", "add-entry", "remove-entry", "rename-entry", "set-entry-data"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm), so
/// a new variant that forgets its kebab spelling here fails to compile rather than failing silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &ZipMutation) -> &'static str {
    match mutation {
        ZipMutation::SetArchiveComment(_) => "set-archive-comment",
        ZipMutation::AddEntry(_) => "add-entry",
        ZipMutation::RemoveEntry(_) => "remove-entry",
        ZipMutation::RenameEntry(_) => "rename-entry",
        ZipMutation::SetEntryData(_) => "set-entry-data",
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Apply
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_zip_mutation(snapshot: &mut ZipSnapshot, mutation: &ZipMutation) -> protocol::MutationOutcome<ZipDiff> {
    let outcome = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the archive comment, then the members that vanished, then — in `next` order —
/// every changed payload and every new member inserted before the next member that already exists. The snapshot `schema` is a
/// constant of the artifact and never differs; a move or a metadata change is no leaf and is refused by the exact net.
pub fn net_mutations(base: &ZipSnapshot, next: &ZipSnapshot) -> Vec<ZipMutation> {
    let mut leaves = Vec::new();
    if (&base.comment, base.comment_utf8) != (&next.comment, next.comment_utf8) {
        leaves.push(ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: next.comment.clone(), comment_utf8: next.comment_utf8 }));
    }
    leaves.extend(base.entries.iter().filter(|entry| !next.entries.iter().any(|kept| kept.name == entry.name)).map(|entry| ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name: entry.name.clone() })));
    let mut present: Vec<&str> = base.entries.iter().filter(|entry| next.entries.iter().any(|kept| kept.name == entry.name)).map(|entry| entry.name.as_str()).collect();
    for (index, entry) in next.entries.iter().enumerate() {
        match base.entries.iter().find(|existing| existing.name == entry.name) {
            Some(existing) if existing.data != entry.data => leaves.push(ZipMutation::SetEntryData(set_entry_data::SetEntryData { name: entry.name.clone(), data: entry.data.clone() })),
            Some(_) => {}
            None => {
                let before = next.entries[index + 1..].iter().map(|following| following.name.clone()).find(|name| present.contains(&name.as_str()));
                leaves.push(ZipMutation::AddEntry(add_entry::AddEntry { entry: entry.clone(), before }));
                present.push(entry.name.as_str());
            }
        }
    }
    leaves
}
//#endregion 🔖️Net

//#region 🔖️Codecs



//#endregion 🔖️Codecs

//#endregion 🔖️MutationTrait

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn entry(name: &str, data: &[u8]) -> ZipEntry {
    ZipEntry { name: name.into(), data: data.to_vec(), ..Default::default() }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn base_snapshot() -> ZipSnapshot {
    ZipSnapshot { schema: "stdio.zip".into(), entries: vec![entry("a.txt", b"aaa"), entry("b.txt", b"bbb")], comment: "archive".into(), ..Default::default() }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<ZipMutation> {
    vec![
        ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "new".into(), comment_utf8: true }),
        ZipMutation::AddEntry(add_entry::AddEntry { entry: entry("x.bin", b"xxx"), before: None }),
        ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name: "a.txt".into() }),
        ZipMutation::RenameEntry(rename_entry::RenameEntry { name: "a.txt".into(), new_name: "renamed.txt".into() }),
        ZipMutation::SetEntryData(set_entry_data::SetEntryData { name: "a.txt".into(), data: b"changed".to_vec() }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
