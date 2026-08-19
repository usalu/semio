//! 🔺️ Sparse logical ZIP diffs over member names, decompressed payloads, ordering, and archive comment.

use std::collections::{HashMap, HashSet};

use crate::artifacts::zip::schema::snapshot::ZipEntry;
use crate::artifacts::zip::ZipSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Model
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntryDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntryModified {
    pub name: String,
    pub diff: ZipEntryDiff,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZipEntriesDiff {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ZipEntryModified>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ZipEntry>,
}

impl ZipEntriesDiff {
    async fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip.diff")]
pub struct ZipDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<ZipEntriesDiff>,
}
//#endregion 🔖️Model

//#region 🔖️EntryLogic
async fn apply_entry_diff(entry: &mut ZipEntry, diff: &ZipEntryDiff) {
    if let Some(name) = &diff.name {
        entry.name = name.clone();
    }
    if let Some(data) = &diff.data {
        entry.data = data.clone();
    }
}

async fn entry_between(base: &ZipEntry, other: &ZipEntry) -> ZipEntryDiff {
    ZipEntryDiff { name: (base.name != other.name).then(|| other.name.clone()), data: (base.data != other.data).then(|| other.data.clone()) }
}

async fn absorb_entry_diff(base: &mut ZipEntryDiff, other: ZipEntryDiff) {
    if other.name.is_some() {
        base.name = other.name;
    }
    if other.data.is_some() {
        base.data = other.data;
    }
}

async fn absorb_entries(first: Option<ZipEntriesDiff>, second: Option<ZipEntriesDiff>) -> Option<ZipEntriesDiff> {
    let (mut first, second) = match (first, second) {
        (None, None) => return None,
        (Some(value), None) | (None, Some(value)) => return Some(value),
        (Some(first), Some(second)) => (first, second),
    };
    let renamed: HashMap<String, String> = first.modified.iter().filter_map(|item| item.diff.name.as_ref().map(|name| (item.name.clone(), name.clone()))).collect();
    let reverse: HashMap<&str, &str> = renamed.iter().map(|(base, current)| (current.as_str(), base.as_str())).collect();
    let added_names: HashSet<String> = first.added.iter().map(|item| item.name.clone()).collect();
    let mut removed = first.removed;
    let mut annihilated = HashSet::new();
    for name in &second.removed {
        if added_names.contains(name) {
            annihilated.insert(name.clone());
        } else {
            let base_name = reverse.get(name.as_str()).copied().unwrap_or(name).to_string();
            if !removed.contains(&base_name) {
                removed.push(base_name.clone());
            }
            first.modified.retain(|item| item.name != base_name);
        }
    }
    let mut modified = first.modified;
    let mut added: Vec<ZipEntry> = first.added.into_iter().filter(|item| !annihilated.contains(&item.name)).collect();
    for item in second.modified {
        if added_names.contains(&item.name) {
            if let Some(entry) = added.iter_mut().find(|entry| entry.name == item.name) {
                apply_entry_diff(entry, &item.diff);
            }
        } else {
            let base_name = reverse.get(item.name.as_str()).copied().unwrap_or(item.name.as_str()).to_string();
            if removed.contains(&base_name) {
                continue;
            }
            if let Some(existing) = modified.iter_mut().find(|existing| existing.name == base_name) {
                absorb_entry_diff(&mut existing.diff, item.diff);
            } else {
                modified.push(ZipEntryModified { name: base_name, diff: item.diff });
            }
        }
    }
    added.extend(second.added);
    let result = ZipEntriesDiff { removed, modified, added };
    (!result.is_empty()).then_some(result)
}
//#endregion 🔖️EntryLogic

//#region 🔖️Algebra
impl MutationDiff<ZipSnapshot> for ZipDiff {
    async fn apply(&self, base: &ZipSnapshot) -> MutationApplyResult<ZipSnapshot> {
        if let Some(entries) = &self.entries {
            validate_zip_entries(&base.entries, entries)?;
        }
        let mut next = base.clone();
        if let Some(comment) = &self.comment {
            next.comment = comment.clone();
        }
        if let Some(diff) = &self.entries {
            let removed: HashSet<&str> = diff.removed.iter().map(String::as_str).collect();
            next.entries.retain(|entry| !removed.contains(entry.name.as_str()));
            for modified in &diff.modified {
                if let Some(entry) = next.entries.iter_mut().find(|entry| entry.name == modified.name) {
                    apply_entry_diff(entry, &modified.diff);
                }
            }
            next.entries.extend(diff.added.iter().cloned());
        }
        next.entries.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        if other.comment.is_some() {
            self.comment = other.comment;
        }
        self.entries = absorb_entries(self.entries.take(), other.entries);
    }
}

async fn validate_zip_entries(base: &[ZipEntry], diff: &ZipEntriesDiff) -> MutationApplyResult<()> {
    let base_names: HashSet<&str> = base.iter().map(|entry| entry.name.as_str()).collect();
    if base_names.len() != base.len() {
        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "ZIP snapshot contains duplicate entry names").at(["entries"]));
    }
    let mut removed = HashSet::new();
    for name in &diff.removed {
        if !base_names.contains(name.as_str()) || !removed.insert(name.as_str()) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "ZIP entry removal is missing or duplicated").at(["entries", "removed"]));
        }
    }
    let mut modified = HashSet::new();
    let mut occupied: HashSet<&str> = base_names.iter().copied().filter(|name| !removed.contains(name)).collect();
    let mut renamed = HashSet::new();
    for entry in &diff.modified {
        if !base_names.contains(entry.name.as_str()) || !modified.insert(entry.name.as_str()) || removed.contains(entry.name.as_str()) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "ZIP entry modification is missing, duplicated, or removed").at(["entries", "modified"]));
        }
        if let Some(name) = &entry.diff.name {
            if name.is_empty() || (name != &entry.name && occupied.contains(name.as_str())) || !renamed.insert(name.as_str()) {
                return Err(MutationApplyError::new("mutation.apply.duplicate-target", "ZIP entry rename conflicts with an existing or repeated name").at(["entries", "modified"]));
            }
            occupied.remove(entry.name.as_str());
            occupied.insert(name.as_str());
        }
    }
    for entry in &diff.added {
        if entry.name.is_empty() || occupied.contains(entry.name.as_str()) || !occupied.insert(entry.name.as_str()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "ZIP entry addition conflicts with the target archive").at(["entries", "added"]));
        }
    }
    Ok(())
}

impl DiffAlgebra<ZipSnapshot> for ZipDiff {
    async fn inverse(&self, base: &ZipSnapshot) -> Self {
        Self::between(&self.apply(base).unwrap(), base)
    }

    async fn between(base: &ZipSnapshot, other: &ZipSnapshot) -> Self {
        let comment = (base.comment != other.comment).then(|| other.comment.clone());
        let base_names: HashSet<&str> = base.entries.iter().map(|entry| entry.name.as_str()).collect();
        let other_names: HashSet<&str> = other.entries.iter().map(|entry| entry.name.as_str()).collect();
        let removed = base.entries.iter().filter(|entry| !other_names.contains(entry.name.as_str())).map(|entry| entry.name.clone()).collect();
        let modified = base
            .entries
            .iter()
            .filter_map(|entry| {
                let other_entry = other.entries.iter().find(|candidate| candidate.name == entry.name)?;
                let diff = entry_between(entry, other_entry);
                (diff != ZipEntryDiff::default()).then_some(ZipEntryModified { name: entry.name.clone(), diff })
            })
            .collect();
        let added = other.entries.iter().filter(|entry| !base_names.contains(entry.name.as_str())).cloned().collect();
        let entries = ZipEntriesDiff { removed, modified, added };
        Self { comment, entries: (!entries.is_empty()).then_some(entries) }
    }

    async fn is_empty(&self) -> bool {
        self.comment.is_none() && self.entries.as_ref().map_or(true, ZipEntriesDiff::is_empty)
    }
}
//#endregion 🔖️Algebra

//#region 🔖️Builders
pub async fn diff_set_snapshot(base: &ZipSnapshot, next: &ZipSnapshot) -> ZipDiff {
    ZipDiff::between(base, next)
}

pub async fn diff_set_archive_comment(comment: &str) -> ZipDiff {
    ZipDiff { comment: Some(comment.into()), entries: None }
}

pub async fn diff_add_entry(entry: ZipEntry) -> ZipDiff {
    ZipDiff { comment: None, entries: Some(ZipEntriesDiff { added: vec![entry], ..Default::default() }) }
}

pub async fn diff_remove_entry(name: &str) -> ZipDiff {
    ZipDiff { comment: None, entries: Some(ZipEntriesDiff { removed: vec![name.into()], ..Default::default() }) }
}

async fn diff_entry_field(name: &str, diff: ZipEntryDiff) -> ZipDiff {
    ZipDiff { comment: None, entries: Some(ZipEntriesDiff { modified: vec![ZipEntryModified { name: name.into(), diff }], ..Default::default() }) }
}

pub async fn diff_rename_entry(name: &str, new_name: &str) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { name: Some(new_name.into()), data: None })
}

pub async fn diff_set_entry_data(name: &str, data: Vec<u8>) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { name: None, data: Some(data) })
}
//#endregion 🔖️Builders

//#region 🔖️Codec
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct ZipDiffRecord {
    value: dsl::DslValue,
}

impl protocol::DiffCodec for ZipDiff {
    async fn print_diff(&self) -> String {
        let model = ZipDiffRecord { value: dsl::to_dsl_value(self).expect("serializable logical ZIP diff") };
        dsl::print(&model.__dsl_to_record(), &ZipDiffRecord::__dsl_spec(), dsl::JoinMode::Document)
    }

    async fn parse_diff(text: &str) -> Result<Self, store::TextError> {
        let record = dsl::parse(text, &ZipDiffRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits { max_bytes: 64 * 1024 * 1024, ..dsl::Limits::default() }, mode: dsl::SourceMode::Document })?;
        let model = ZipDiffRecord::__dsl_from_record(&record)?;
        dsl::from_dsl_value(model.value).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }

    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let value = dsl::to_dsl_value(self).map_err(|detail| protocol::ProtocolError::Malformed { what: "zip diff", offset: 0, detail })?;
        Ok(store::pack_rt::encode_wire_value(&value))
    }

    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "zip diff", offset: 0, detail: error.to_string() })?;
        dsl::from_dsl_value(value).map_err(|detail| protocol::ProtocolError::Malformed { what: "zip diff", offset: 0, detail })
    }
}
//#endregion 🔖️Codec

#[cfg(test)]
pub(crate) async fn demo_diff_cases() -> Vec<ZipDiff> {
    let base = ZipSnapshot { entries: vec![ZipEntry { name: "before.txt".into(), data: b"before".to_vec() }], ..Default::default() };
    let other = ZipSnapshot { entries: vec![ZipEntry { name: "after.txt".into(), data: b"after".to_vec() }], comment: "archive".into(), ..Default::default() };
    vec![ZipDiff::default(), ZipDiff::between(&base, &other), ZipDiff::between(&other, &base)]
}
