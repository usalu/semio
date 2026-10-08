//! 🔺️ Sparse logical ZIP diffs over member names, decompressed payloads, ordering, and archive comment.

use crate::ZipSnapshot;
use crate::schema::snapshot::ZipEntryMetadata;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use std::collections::{HashMap, HashSet};

//#region 🔖️Model
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ZipEntryDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ZipEntryMetadata>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ZipEntryModified {
    pub name: String,
    pub diff: ZipEntryDiff,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ZipEntriesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ZipEntryModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ZipEntry>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<String>>,
}

impl ZipEntriesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty() && self.order.is_none()
    }

    /// 🔁️ The negative rows against the base entries: added members are removed again, removed members return with their base
    /// content, every modified member restores its base fields under the name it carries afterwards, and an explicit `order`
    /// is emitted exactly when the natural order of those rows would not land on the base order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base: &[ZipEntry]) -> Self {
        let find = |name: &str| base.iter().find(|entry| entry.name == name);
        let removed: Vec<String> = self.added.iter().map(|entry| entry.name.clone()).collect();
        let modified = self
            .modified
            .iter()
            .filter_map(|item| {
                let entry = find(&item.name)?;
                let diff = ZipEntryDiff { name: item.diff.name.as_ref().map(|_| entry.name.clone()), data: item.diff.data.as_ref().map(|_| entry.data.clone()), metadata: item.diff.metadata.as_ref().map(|_| entry.metadata.clone()) };
                Some(ZipEntryModified { name: item.diff.name.clone().unwrap_or_else(|| item.name.clone()), diff })
            })
            .collect();
        let added: Vec<ZipEntry> = self.removed.iter().filter_map(|name| find(name).cloned()).collect();
        let renamed: HashMap<&str, &str> = self.modified.iter().filter_map(|item| item.diff.name.as_deref().map(|name| (name, item.name.as_str()))).collect();
        let dropped: HashSet<&str> = self.removed.iter().map(String::as_str).collect();
        let after_names: Vec<&str> = match &self.order {
            Some(order) => order.iter().map(String::as_str).collect(),
            None => base.iter().filter(|entry| !dropped.contains(entry.name.as_str())).map(|entry| self.modified.iter().find(|item| item.name == entry.name).and_then(|item| item.diff.name.as_deref()).unwrap_or(entry.name.as_str())).chain(self.added.iter().map(|entry| entry.name.as_str())).collect(),
        };
        let added_names: HashSet<&str> = removed.iter().map(String::as_str).collect();
        let natural: Vec<&str> = after_names.into_iter().filter(|name| !added_names.contains(name)).map(|name| renamed.get(name).copied().unwrap_or(name)).chain(added.iter().map(|entry| entry.name.as_str())).collect();
        let order = (!natural.iter().copied().eq(base.iter().map(|entry| entry.name.as_str()))).then(|| base.iter().map(|entry| entry.name.clone()).collect());
        Self { removed, modified, added, order }
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip.diff")]
pub struct ZipDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comment_utf8: Option<bool>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<ZipEntriesDiff>,
}
//#endregion 🔖️Model

//#region 🔖️EntryLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_entry_diff(entry: &mut ZipEntry, diff: &ZipEntryDiff) {
    if let Some(name) = &diff.name {
        entry.name = name.clone();
    }
    if let Some(data) = &diff.data {
        entry.data = data.clone();
    }
    if let Some(metadata) = &diff.metadata {
        entry.metadata = metadata.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_entry_rows(base: &mut ZipEntryDiff, other: ZipEntryDiff) {
    if other.name.is_some() {
        base.name = other.name;
    }
    if other.data.is_some() {
        base.data = other.data;
    }
    if other.metadata.is_some() {
        base.metadata = other.metadata;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_entries(first: Option<ZipEntriesDiff>, second: Option<ZipEntriesDiff>) -> Option<ZipEntriesDiff> {
    let (mut first, second) = match (first, second) {
        (None, None) => return None,
        (Some(value), None) | (None, Some(value)) => return Some(value),
        (Some(first), Some(second)) => (first, second),
    };
    let order = second.order.clone().or_else(|| first.order.take().map(|mut order| {
        let removed: HashSet<&str> = second.removed.iter().map(String::as_str).collect();
        order.retain(|name| !removed.contains(name.as_str()));
        let renamed: HashMap<&str, &str> = second.modified.iter().filter_map(|entry| entry.diff.name.as_ref().map(|name| (entry.name.as_str(), name.as_str()))).collect();
        for name in &mut order {
            if let Some(renamed) = renamed.get(name.as_str()) {
                *name = (*renamed).into();
            }
        }
        order.extend(second.added.iter().map(|entry| entry.name.clone()));
        order
    }));
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
                absorb_entry_rows(&mut existing.diff, item.diff);
            } else {
                modified.push(ZipEntryModified { name: base_name, diff: item.diff });
            }
        }
    }
    added.extend(second.added);
    let result = ZipEntriesDiff { removed, modified, added, order };
    (!result.is_empty()).then_some(result)
}
//#endregion 🔖️EntryLogic

//#region 🔖️Algebra
impl MutationDiff<ZipSnapshot> for ZipDiff {
    fn apply(&self, base: &ZipSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<ZipSnapshot> {
        if let Some(entries) = &self.entries {
            validate_zip_entries(&base.entries, entries)?;
        }
        let mut next = base.clone();
        if let Some(comment) = &self.comment {
            next.comment = comment.clone();
        }
        if let Some(comment_utf8) = self.comment_utf8 {
            next.comment_utf8 = comment_utf8;
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
            if let Some(order) = &diff.order {
                let mut entries: HashMap<String, ZipEntry> = next.entries.into_iter().map(|entry| (entry.name.clone(), entry)).collect();
                next.entries = order.iter().map(|name| entries.remove(name).expect("validated ZIP order")).collect();
            }
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.comment.is_some() {
            self.comment = other.comment;
        }
        if other.comment_utf8.is_some() {
            self.comment_utf8 = other.comment_utf8;
        }
        self.entries = absorb_entries(self.entries.take(), other.entries);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_zip_entries(base: &[ZipEntry], diff: &ZipEntriesDiff) -> MutationApplyResult<()> {
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
    if let Some(order) = &diff.order {
        let ordered: HashSet<&str> = order.iter().map(String::as_str).collect();
        if ordered.len() != order.len() || ordered != occupied {
            return Err(MutationApplyError::new("mutation.apply.invalid-order", "ZIP entry order must contain every resulting member exactly once").at(["entries", "order"]));
        }
    }
    Ok(())
}

impl DiffAlgebra<ZipSnapshot> for ZipDiff {
    fn inverse(&self, base: &ZipSnapshot) -> Self {
        Self { comment: self.comment.as_ref().map(|_| base.comment.clone()), comment_utf8: self.comment_utf8.map(|_| base.comment_utf8), entries: self.entries.as_ref().map(|entries| entries.inverse(&base.entries)) }
    }

    fn is_empty(&self) -> bool {
        self.comment.is_none() && self.comment_utf8.is_none() && self.entries.as_ref().is_none_or(ZipEntriesDiff::is_empty)
    }
}
//#endregion 🔖️Algebra

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_archive_comment(comment: &str, comment_utf8: bool) -> ZipDiff {
    ZipDiff { comment: Some(comment.into()), comment_utf8: Some(comment_utf8), entries: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_entry(base: &ZipSnapshot, entry: ZipEntry, before: Option<&str>) -> ZipDiff {
    let order = before.map(|before| {
        let mut names = Vec::with_capacity(base.entries.len() + 1);
        for existing in &base.entries {
            if existing.name == before {
                names.push(entry.name.clone());
            }
            names.push(existing.name.clone());
        }
        names
    });
    ZipDiff { comment: None, comment_utf8: None, entries: Some(ZipEntriesDiff { added: vec![entry], order, ..Default::default() }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_entry(name: &str) -> ZipDiff {
    ZipDiff { comment: None, comment_utf8: None, entries: Some(ZipEntriesDiff { removed: vec![name.into()], ..Default::default() }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_entry_field(name: &str, diff: ZipEntryDiff) -> ZipDiff {
    ZipDiff { comment: None, comment_utf8: None, entries: Some(ZipEntriesDiff { modified: vec![ZipEntryModified { name: name.into(), diff }], ..Default::default() }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_rename_entry(name: &str, new_name: &str) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { name: Some(new_name.into()), data: None, metadata: None })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_entry_data(name: &str, data: Vec<u8>) -> ZipDiff {
    diff_entry_field(name, ZipEntryDiff { name: None, data: Some(data), metadata: None })
}
//#endregion 🔖️Builders

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<ZipDiff> {
    let entries = ZipEntriesDiff {
        removed: vec!["before.txt".into()],
        modified: vec![ZipEntryModified { name: "keep.txt".into(), diff: ZipEntryDiff { data: Some(b"after".to_vec()), ..Default::default() } }],
        added: vec![ZipEntry { name: "after.txt".into(), data: b"after".to_vec(), ..Default::default() }],
        order: None,
    };
    vec![ZipDiff::default(), ZipDiff { comment: Some("archive".into()), comment_utf8: Some(true), entries: Some(entries) }]
}

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::ZipEntry;
//#endregion 🔁️Re-exports

#[cfg(test)]
#[test]
fn logical_diff_does_not_require_native_comment_encoding(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚪️logical-diff-independence/🔣️.json")).unwrap();
    let base=ZipSnapshot{comment_utf8:fixture["before"]["commentUtf8"].as_bool().unwrap(),..ZipSnapshot::default()};
    let diff=ZipDiff{comment:Some(fixture["mutation"]["comment"].as_str().unwrap().into()),..ZipDiff::default()};
    let after=protocol::apply_diff(&diff,&base).expect("logical comment edit must not materialize native ZIP");
    let actual=serde_json::json!({"comment":after.comment,"commentUtf8":after.comment_utf8});
    let mut reference=fixture["before"].clone();reference["comment"]=fixture["mutation"]["comment"].clone();
    assert_eq!(reference,fixture["after"]);assert_eq!(actual,reference);
    assert_eq!(crate::standards::v2_0::subsets::base::io::encode_zip(&after).is_ok(),fixture["nativeEncode"].as_bool().unwrap());
    eprintln!("[DEBUG] ZIP logical diff and separate native admission oracle=serde_json");
}
