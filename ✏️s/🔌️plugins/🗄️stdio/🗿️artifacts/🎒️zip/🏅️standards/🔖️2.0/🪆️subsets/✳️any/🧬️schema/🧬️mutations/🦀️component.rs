//! 🧬️ Logical ZIP mutations over member names, decompressed payloads, ordering, and archive comment.

use crate::artifacts::zip::schema::diff::{self, ZipDiff};
use crate::artifacts::zip::schema::snapshot::ZipEntry;
use crate::artifacts::zip::ZipSnapshot;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Model
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum ZipMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: ZipSnapshot,
    },
    SetArchiveComment {
        comment: String,
    },
    AddEntry {
        #[dsl(block)]
        entry: ZipEntry,
    },
    RemoveEntry {
        name: String,
    },
    RenameEntry {
        name: String,
        new_name: String,
    },
    SetEntryData {
        name: String,
        #[dsl(base64)]
        data: Vec<u8>,
    },
}
//#endregion 🔖️Model

//#region 🔖️Kinds
/// 🦠️ Kebab-case spelling of every `ZipMutation` variant, in declaration order — the exact `kinds`
/// list `../../🧪️oracle/🔣️.json`'s `mutationCatalogs` entry must declare. The framework
/// never parses this enum; `kinds_matches_enum_variants_and_manifest` below is what keeps the two
/// declarations honest against each other.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-archive-comment", "add-entry", "remove-entry", "rename-entry", "set-entry-data"];

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm), so
/// a new variant that forgets its kebab spelling here fails to compile rather than failing silently.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn kind_of(mutation: &ZipMutation) -> &'static str {
    match mutation {
        ZipMutation::NoMutation => "no-mutation",
        ZipMutation::SetSnapshot { .. } => "set-snapshot",
        ZipMutation::SetArchiveComment { .. } => "set-archive-comment",
        ZipMutation::AddEntry { .. } => "add-entry",
        ZipMutation::RemoveEntry { .. } => "remove-entry",
        ZipMutation::RenameEntry { .. } => "rename-entry",
        ZipMutation::SetEntryData { .. } => "set-entry-data",
    }
}
//#endregion 🔖️Kinds

//#region 🔖️Algebra
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_zip_mutation(snapshot: &mut ZipSnapshot, mutation: &ZipMutation) -> protocol::MutationOutcome<ZipDiff> {
    let outcome = mutation.diff(snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

impl Mutation<ZipSnapshot> for ZipMutation {
    type Diff = ZipDiff;

    fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            Self::NoMutation => ZipDiff::default(),
            Self::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            Self::SetArchiveComment { comment } => diff::diff_set_archive_comment(comment),
            Self::AddEntry { entry } => diff::diff_add_entry(entry.clone()),
            Self::RemoveEntry { name } => diff::diff_remove_entry(name),
            Self::RenameEntry { name, new_name } => diff::diff_rename_entry(name, new_name),
            Self::SetEntryData { name, data } => diff::diff_set_entry_data(name, data.clone()),
        })
    }

    fn inverse(&self, base: &ZipSnapshot) -> Vec<Self> {
        match self {
            Self::NoMutation => vec![Self::NoMutation],
            Self::SetSnapshot { .. } => vec![Self::SetSnapshot { snapshot: base.clone() }],
            Self::SetArchiveComment { .. } => vec![Self::SetArchiveComment { comment: base.comment.clone() }],
            Self::AddEntry { entry, .. } => vec![Self::RemoveEntry { name: entry.name.clone() }],
            Self::RemoveEntry { name } => base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![Self::AddEntry { entry: entry.clone() }]).unwrap_or_else(|| vec![Self::NoMutation]),
            Self::RenameEntry { name, new_name } => vec![Self::RenameEntry { name: new_name.clone(), new_name: name.clone() }],
            Self::SetEntryData { name, .. } => base.entries.iter().find(|entry| entry.name == *name).map(|entry| vec![Self::SetEntryData { name: name.clone(), data: entry.data.clone() }]).unwrap_or_else(|| vec![Self::NoMutation]),
        }
    }
}
//#endregion 🔖️Algebra

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
        ZipMutation::NoMutation,
        ZipMutation::SetSnapshot { snapshot: base_snapshot() },
        ZipMutation::SetArchiveComment { comment: "new".into() },
        ZipMutation::AddEntry { entry: entry("x.bin", b"xxx") },
        ZipMutation::RemoveEntry { name: "a.txt".into() },
        ZipMutation::RenameEntry { name: "a.txt".into(), new_name: "renamed.txt".into() },
        ZipMutation::SetEntryData { name: "a.txt".into(), data: b"changed".to_vec() },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{MutationDiff as _, OpBinary as _, OpText as _};

    #[semio_framework_async_macros::async_test]
    async fn logical_mutations_diff_and_codecs_round_trip() {
        let base = base_snapshot();
        for mutation in demo_mutation_cases() {
            let text = mutation.print_op();
            assert_eq!(ZipMutation::parse_op(&text).expect("text operation"), mutation);
            let bytes = mutation.encode_op().expect("binary operation");
            assert_eq!(ZipMutation::decode_op(&bytes).expect("binary operation"), mutation);
            assert_eq!(mutation.diff(&base).diff().apply(&base).unwrap(), {
                let mut next = base.clone();
                apply_zip_mutation(&mut next, &mutation);
                next
            });
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn kinds_matches_enum_variants_and_manifest() {
        let observed: std::collections::BTreeSet<&str> = demo_mutation_cases().iter().map(kind_of).collect();
        let declared: std::collections::BTreeSet<&str> = KINDS.iter().copied().collect();
        assert_eq!(observed, declared, "KINDS must list exactly the kebab-case spelling of every ZipMutation variant");
        assert_eq!(KINDS.len(), demo_mutation_cases().len(), "KINDS must cover every variant exactly once, with no duplicates");

        let manifest: serde_json::Value = serde_json::from_str(include_str!("../../🧪️oracle/🔣️.json")).expect("valid oracle manifest JSON");
        let catalog_kinds: std::collections::BTreeSet<String> = manifest["mutationCatalogs"][0]["kinds"].as_array().expect("mutationCatalogs[0].kinds array").iter().map(|value| value.as_str().expect("kind is a string").to_string()).collect();
        let declared_owned: std::collections::BTreeSet<String> = KINDS.iter().map(|kind| kind.to_string()).collect();
        assert_eq!(catalog_kinds, declared_owned, "the oracle manifest's mutationCatalogs[0].kinds must match KINDS exactly");
    }
}

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/extends-the-readme-and-adds-a-version-member/🦀️component.rs"]
    mod tests_set_snapshot_extends_the_readme_and_adds_a_version_member;
}
//#endregion 🧪️FixtureTests
