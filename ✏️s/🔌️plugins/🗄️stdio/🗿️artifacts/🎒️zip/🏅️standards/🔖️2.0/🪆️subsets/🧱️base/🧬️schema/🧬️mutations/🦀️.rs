//! 🧬️ Logical ZIP mutations over member names, decompressed payloads, ordering, and archive comment.

use crate::schema::diff::{self, ZipDiff};
use crate::schema::snapshot::ZipEntry;
use crate::ZipSnapshot;

//#region 🔖️Model
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "💬set-archive-comment/🦀️.rs"]
pub mod set_archive_comment;
#[path = "➕add-entry/🦀️.rs"]
pub mod add_entry;
#[path = "➖remove-entry/🦀️.rs"]
pub mod remove_entry;
#[path = "🏷️rename-entry/🦀️.rs"]
pub mod rename_entry;
#[path = "✍️set-entry-data/🦀️.rs"]
pub mod set_entry_data;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[mutations(snapshot = ZipSnapshot, diff = ZipDiff, schema = "ZipMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum ZipMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
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
pub const KINDS: &[&str] = &["set-snapshot", "set-archive-comment", "add-entry", "remove-entry", "rename-entry", "set-entry-data"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm), so
/// a new variant that forgets its kebab spelling here fails to compile rather than failing silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &ZipMutation) -> &'static str {
    match mutation {
        ZipMutation::SetSnapshot(_) => "set-snapshot",
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
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️Codecs
impl protocol::OpText for ZipMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec) in &variants {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown ZIP operation '{line}'")))
    }

    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(name, _)| name == &keyword).map(|(_, spec)| *spec).expect("ZIP operation spec");
        dsl::print(&record, &spec(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for ZipMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Codecs

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &ZipMutation, base: &ZipSnapshot) -> protocol::MutationOutcome<ZipDiff> {
    protocol::MutationOutcome::new(match this {
        ZipMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment }) => diff::diff_set_archive_comment(comment),
        ZipMutation::AddEntry(add_entry::AddEntry { entry }) => diff::diff_add_entry(entry.clone()),
        ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name }) => diff::diff_remove_entry(name),
        ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => diff::diff_rename_entry(name, new_name),
        ZipMutation::SetEntryData(set_entry_data::SetEntryData { name, data }) => diff::diff_set_entry_data(name, data.clone()),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &ZipMutation, base: &ZipSnapshot) -> Vec<ZipMutation> {
    match this {
        ZipMutation::SetSnapshot(_) => vec![ZipMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        ZipMutation::SetArchiveComment(_) => vec![ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: base.comment.clone() })],
        ZipMutation::AddEntry(add_entry::AddEntry { entry, .. }) => vec![ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name: entry.name.clone() })],
        ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name }) => base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![ZipMutation::AddEntry(add_entry::AddEntry { entry: entry.clone() })]).unwrap_or_default(),
        ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => vec![ZipMutation::RenameEntry(rename_entry::RenameEntry { name: new_name.clone(), new_name: name.clone() })],
        ZipMutation::SetEntryData(set_entry_data::SetEntryData { name, .. }) => base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![ZipMutation::SetEntryData(set_entry_data::SetEntryData { name: name.clone(), data: entry.data.clone() })]).unwrap_or_default(),
    }
}
//#endregion 🔖️MutationTrait

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn entry(name: &str, data: &[u8]) -> ZipEntry {
    ZipEntry { name: name.into(), data: data.to_vec() }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn base_snapshot() -> ZipSnapshot {
    ZipSnapshot { schema: "stdio.zip".into(), entries: vec![entry("a.txt", b"aaa"), entry("b.txt", b"bbb")], comment: "archive".into() }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<ZipMutation> {
    vec![
        ZipMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base_snapshot() }),
        ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "new".into() }),
        ZipMutation::AddEntry(add_entry::AddEntry { entry: entry("x.bin", b"xxx") }),
        ZipMutation::RemoveEntry(remove_entry::RemoveEntry { name: "a.txt".into() }),
        ZipMutation::RenameEntry(rename_entry::RenameEntry { name: "a.txt".into(), new_name: "renamed.txt".into() }),
        ZipMutation::SetEntryData(set_entry_data::SetEntryData { name: "a.txt".into(), data: b"changed".to_vec() }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
