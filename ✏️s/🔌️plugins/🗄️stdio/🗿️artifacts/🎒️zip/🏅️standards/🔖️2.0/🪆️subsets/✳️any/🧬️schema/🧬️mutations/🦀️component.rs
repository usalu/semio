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

//#region 🔖️Algebra
pub async fn apply_zip_mutation(snapshot: &mut ZipSnapshot, mutation: &ZipMutation) -> protocol::MutationOutcome<ZipDiff> {
    let outcome = mutation.diff(snapshot).await;
    match protocol::MutationDiff::apply(outcome.diff().await, snapshot).await {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).await.absorb_messages(outcome.messages().await.to_vec()).await,
    }
}

impl Mutation<ZipSnapshot> for ZipMutation {
    type Diff = ZipDiff;

    async fn diff(&self, base: &ZipSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            Self::NoMutation => ZipDiff::default(),
            Self::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot).await,
            Self::SetArchiveComment { comment } => diff::diff_set_archive_comment(comment).await,
            Self::AddEntry { entry } => diff::diff_add_entry(entry.clone()).await,
            Self::RemoveEntry { name } => diff::diff_remove_entry(name).await,
            Self::RenameEntry { name, new_name } => diff::diff_rename_entry(name, new_name).await,
            Self::SetEntryData { name, data } => diff::diff_set_entry_data(name, data.clone()).await,
        })
    }

    async fn inverse(&self, base: &ZipSnapshot) -> Vec<Self> {
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
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec) in &variants {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Inline }).await?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record).await;
            }
        }
        Err(dsl::__rt::field_error(format!("unknown ZIP operation '{line}'")))
    }

    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as dsl::DslVariants>::variants();
        let spec = variants.iter().find(|(name, _)| name == &keyword).map(|(_, spec)| *spec).expect("ZIP operation spec");
        dsl::print(&record, &spec(), dsl::JoinMode::Inline).await
    }
}

impl protocol::OpBinary for ZipMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self).await
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes).await
    }
}
//#endregion 🔖️Codecs

#[cfg(test)]
pub(crate) async fn entry(name: &str, data: &[u8]) -> ZipEntry {
    ZipEntry { name: name.into(), data: data.to_vec() }
}

#[cfg(test)]
pub(crate) async fn base_snapshot() -> ZipSnapshot {
    ZipSnapshot { schema: "stdio.zip".into(), entries: vec![entry("a.txt", b"aaa"), entry("b.txt", b"bbb")], comment: "archive".into() }
}

#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<ZipMutation> {
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
}
