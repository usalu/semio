//! 🔺️ BcfDiff — handcrafted sparse diff over `BcfSnapshot`. No `snapshot: Option<BcfSnapshot>`
//! full-replace slot — even `SetSnapshot`'s diff is the sparse field-by-field
//! `BcfDiff::between(base, next)`.
//!
//! `topics` (guid-keyed) and, within a modified topic, `comments`/`viewpoints` (also guid-keyed)
//! are diffed via a generic `NamedTripleDiff<K, D, T>` engine — the same shape docx's
//! `NamedTripleDiff` established (see `f4-docx-report.md` §3) for name/key-keyed collections.
//! This artifact defines its OWN copy rather than importing docx's (cross-artifact imports would
//! be architecturally wrong; docx's own report flags hoisting this engine into a shared location
//! as `glue_followup`, which this wave inherits). `parts` (unknown/unmodeled files) uses the same
//! engine, keyed by name.

use crate::artifacts::bcf::schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility};
use crate::artifacts::bcf::BcfSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️GenericNamedEngine
/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `added` carries the full item (which already contains its own key). Mirrors docx's
/// `NamedTripleDiff` verbatim (see that module's doc comment for the `bound(...)` rationale — a
/// known serde_derive limitation where `#[serde(default)]` on a `Vec<T>` field spuriously infers
/// `T: Default`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", bound(serialize = "K: Serialize, D: Serialize, T: Serialize", deserialize = "K: Deserialize<'de>, D: Deserialize<'de>, T: Deserialize<'de>"))]
pub struct NamedTripleDiff<K, D, T> {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    for item in &diff.added {
        items.push(item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_named<K, T, D>(items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K) -> MutationApplyResult<()>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let keys: Vec<K> = items.iter().map(&key_of).collect();
    for (position, key) in diff.removed.iter().enumerate() {
        if !keys.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist").at(["removed"]));
        }
        if diff.removed[..position].contains(key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated").at(["removed"]));
        }
    }
    for (position, modified) in diff.modified.iter().enumerate() {
        if !keys.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at(["modified"]));
        }
        if diff.removed.contains(&modified.key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item").at(["modified"]));
        }
        if diff.modified[..position].iter().any(|candidate| candidate.key == modified.key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated").at(["modified"]));
        }
    }
    let mut added_keys = Vec::new();
    for item in &diff.added {
        let key = key_of(item);
        if keys.contains(&key) || added_keys.contains(&key) || diff.removed.contains(&key) || diff.modified.iter().any(|modified| modified.key == key) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists or conflicts").at(["added"]));
        }
        added_keys.push(key);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) {
            added.push(original.clone());
        }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name-keyed absorb — identity is the KEY (not position): a `d2`-removal of a `d1`-added key
/// annihilates the add; a `d2`-modify of a `d1`-added key patches into the carried payload;
/// everything else composes directly on the shared key space.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️DiffTypes
pub type BcfTopicsDiff = NamedTripleDiff<String, BcfTopicDiff, BcfTopic>;
pub type BcfCommentsDiff = NamedTripleDiff<String, BcfCommentDiff, BcfComment>;
pub type BcfViewpointsDiff = NamedTripleDiff<String, BcfViewpointDiff, BcfViewpoint>;
pub type BcfPartsDiff = NamedTripleDiff<String, BcfPartDiff, BcfRawPart>;

/// 🔺️ Diff for `stdio.bcf`.
/// 🧪️ F6 CONFIRMED (real `cargo check` error, `dsl::DslDiff` attempted and reverted): fails on
/// BOTH independent blockers simultaneously — (1) `topics`/`parts`: `NamedTripleDiff<K,D,T>` has
/// no `DslField` impl (no blanket bridge from `DslVariants`/generic collection types into
/// `DslField` exists anywhere in the `dsl` crate, same root cause as every other collection-triple
/// artifact); (2) `viewpoint_ref`/`camera`/`components`/`snapshot` are tri-state
/// `Option<Option<T>>` — `classify_field` peels exactly one `Option` layer, and no
/// `impl<T: DslField> DslField for Option<T>` exists, so the remaining `Option<T>` never binds
/// (§3b of `f6-recon-report.md`); (3) `BcfCamera` is additionally a genuine data-carrying enum
/// reachable through `camera: Option<Option<BcfCamera>>` — `error[E0277]: the trait bound
/// v2_1::...::BcfCamera: DslField is not satisfied` (§3a) — so this diff hits BOTH documented
/// failure modes at once, not just the tri-state one the recon sweep's file-level grep guessed
/// (that sweep found "0 enums" only because it greps for `pub enum` declared IN the diff file
/// itself; `BcfCamera` lives in the snapshot module and is reached via `BcfViewpointDiff`).
/// `DiffCodec` is hand-rolled below.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf.diff")]
pub struct BcfDiff {
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<BcfTopicsDiff>,
    #[state(artifact)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<BcfPartsDiff>,
}

/// 🔺️ Per-topic sparse diff. `title`/`description`/`status`/`priority`/`creation_date`/
/// `creation_author` are scalar patches; `labels` is whole-value replaced (not itself a keyed
/// collection per the completeness target); `comments`/`viewpoints` are recursive guid-keyed
/// triples.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfTopicDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<BcfCommentsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewpoints: Option<BcfViewpointsDiff>,
}

/// 🔺️ Per-comment sparse diff. `viewpoint_ref` is tri-state (`Some(None)` = reference cleared).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfCommentDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewpoint_ref: Option<Option<String>>,
}

/// 🔺️ Per-viewpoint sparse diff. `camera`/`components`/`snapshot` are all weak (whole-value
/// replaced, per the recipe — never sub-diffed) and tri-state nullable.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfViewpointDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Option<BcfCamera>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Option<BcfComponents>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Option<Vec<u8>>>,
}

/// 🔺️ Per-raw-part sparse diff — `name` is the key, so only `data` can change.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BcfPartDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️WrapHelpers
/// 🧭️ Lowers a per-topic leaf diff into a full `BcfDiff` (mirrors svg's `diff_at_path` /
/// docx's `wrap_body_diff`, specialized to this artifact's fixed two-level guid nesting instead of
/// a generic path — bcf's tree never grows deeper than topic -> {comment,viewpoint}).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_topic_diff(guid: &str, diff: BcfTopicDiff) -> BcfDiff {
    BcfDiff { version: None, topics: Some(BcfTopicsDiff { removed: Vec::new(), modified: vec![NamedModified { key: guid.to_string(), diff }], added: Vec::new() }), parts: None }
}

/// 🧭️ Lowers a per-comment leaf diff (inside topic `topic_guid`) into a full `BcfDiff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_comment_diff(topic_guid: &str, comment_guid: &str, diff: BcfCommentDiff) -> BcfDiff {
    wrap_topic_diff(topic_guid, BcfTopicDiff { comments: Some(BcfCommentsDiff { removed: Vec::new(), modified: vec![NamedModified { key: comment_guid.to_string(), diff }], added: Vec::new() }), ..Default::default() })
}

/// 🧭️ Lowers a per-viewpoint leaf diff (inside topic `topic_guid`) into a full `BcfDiff`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn wrap_viewpoint_diff(topic_guid: &str, viewpoint_guid: &str, diff: BcfViewpointDiff) -> BcfDiff {
    wrap_topic_diff(topic_guid, BcfTopicDiff { viewpoints: Some(BcfViewpointsDiff { removed: Vec::new(), modified: vec![NamedModified { key: viewpoint_guid.to_string(), diff }], added: Vec::new() }), ..Default::default() })
}
//#endregion 🔖️WrapHelpers

//#region 🔖️Apply
impl MutationDiff<BcfSnapshot> for BcfDiff {
    async fn apply(&self, base: &BcfSnapshot) -> MutationApplyResult<BcfSnapshot> {
        validate_bcf_diff(self, base)?;
        let mut next = base.clone();
        if let Some(v) = &self.version {
            next.version = v.clone();
        }
        if let Some(td) = &self.topics {
            apply_named(&mut next.topics, td, |t| t.guid.clone(), apply_topic);
        }
        if let Some(pd) = &self.parts {
            apply_named(&mut next.parts, pd, |p| p.name.clone(), apply_part);
        }
        Ok(next)
    }

    async fn absorb(&mut self, other: Self) {
        if other.version.is_some() {
            self.version = other.version;
        }
        self.topics = match (self.topics.take(), other.topics) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |t| t.guid.clone(), absorb_topic_diff, apply_topic)),
        };
        self.parts = match (self.parts.take(), other.parts) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |p| p.name.clone(), absorb_part_diff, apply_part)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_bcf_diff(diff: &BcfDiff, base: &BcfSnapshot) -> MutationApplyResult<()> {
    if let Some(topics) = &diff.topics {
        validate_named(&base.topics, topics, |topic| topic.guid.clone())?;
        for modified in &topics.modified {
            if let Some(topic) = base.topics.iter().find(|topic| topic.guid == modified.key) {
                validate_topic_diff(topic, &modified.diff)?;
            }
        }
    }
    if let Some(parts) = &diff.parts {
        validate_named(&base.parts, parts, |part| part.name.clone())?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_topic_diff(base: &BcfTopic, diff: &BcfTopicDiff) -> MutationApplyResult<()> {
    if let Some(comments) = &diff.comments {
        validate_named(&base.comments, comments, |comment| comment.guid.clone())?;
    }
    if let Some(viewpoints) = &diff.viewpoints {
        validate_named(&base.viewpoints, viewpoints, |viewpoint| viewpoint.guid.clone())?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_topic(topic: &mut BcfTopic, diff: &BcfTopicDiff) {
    if let Some(v) = &diff.title {
        topic.title = v.clone();
    }
    if let Some(v) = &diff.description {
        topic.description = v.clone();
    }
    if let Some(v) = &diff.status {
        topic.status = v.clone();
    }
    if let Some(v) = &diff.priority {
        topic.priority = v.clone();
    }
    if let Some(v) = &diff.labels {
        topic.labels = v.clone();
    }
    if let Some(v) = &diff.creation_date {
        topic.creation_date = v.clone();
    }
    if let Some(v) = &diff.creation_author {
        topic.creation_author = v.clone();
    }
    if let Some(cd) = &diff.comments {
        apply_named(&mut topic.comments, cd, |c| c.guid.clone(), apply_comment);
    }
    if let Some(vd) = &diff.viewpoints {
        apply_named(&mut topic.viewpoints, vd, |v| v.guid.clone(), apply_viewpoint);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_comment(comment: &mut BcfComment, diff: &BcfCommentDiff) {
    if let Some(v) = &diff.date {
        comment.date = v.clone();
    }
    if let Some(v) = &diff.author {
        comment.author = v.clone();
    }
    if let Some(v) = &diff.text {
        comment.text = v.clone();
    }
    if let Some(v) = &diff.viewpoint_ref {
        comment.viewpoint_ref = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_viewpoint(vp: &mut BcfViewpoint, diff: &BcfViewpointDiff) {
    if let Some(v) = &diff.camera {
        vp.camera = v.clone();
    }
    if let Some(v) = &diff.components {
        vp.components = v.clone();
    }
    if let Some(v) = &diff.snapshot {
        vp.snapshot = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_part(part: &mut BcfRawPart, diff: &BcfPartDiff) {
    if let Some(v) = &diff.data {
        part.data = v.clone();
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<BcfSnapshot> for BcfDiff {
    async fn inverse(&self, base: &BcfSnapshot) -> Self {
        BcfDiff {
            version: self.version.as_ref().map(|_| base.version.clone()),
            topics: self.topics.as_ref().map(|d| inverse_named(&base.topics, d, |t| t.guid.clone(), inverse_topic)),
            parts: self.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.name.clone(), inverse_part)),
        }
    }

    async fn between(base: &BcfSnapshot, other: &BcfSnapshot) -> Self {
        BcfDiff {
            version: if base.version != other.version { Some(other.version.clone()) } else { None },
            topics: between_named(&base.topics, &other.topics, |t| t.guid.clone(), between_topic),
            parts: between_named(&base.parts, &other.parts, |p| p.name.clone(), between_part),
        }
    }

    async fn is_empty(&self) -> bool {
        self.version.is_none() && self.topics.is_none() && self.parts.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_topic(base: &BcfTopic, diff: &BcfTopicDiff) -> BcfTopicDiff {
    BcfTopicDiff {
        title: diff.title.as_ref().map(|_| base.title.clone()),
        description: diff.description.as_ref().map(|_| base.description.clone()),
        status: diff.status.as_ref().map(|_| base.status.clone()),
        priority: diff.priority.as_ref().map(|_| base.priority.clone()),
        labels: diff.labels.as_ref().map(|_| base.labels.clone()),
        creation_date: diff.creation_date.as_ref().map(|_| base.creation_date.clone()),
        creation_author: diff.creation_author.as_ref().map(|_| base.creation_author.clone()),
        comments: diff.comments.as_ref().map(|d| inverse_named(&base.comments, d, |c| c.guid.clone(), inverse_comment)),
        viewpoints: diff.viewpoints.as_ref().map(|d| inverse_named(&base.viewpoints, d, |v| v.guid.clone(), inverse_viewpoint)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_comment(base: &BcfComment, diff: &BcfCommentDiff) -> BcfCommentDiff {
    BcfCommentDiff {
        date: diff.date.as_ref().map(|_| base.date.clone()),
        author: diff.author.as_ref().map(|_| base.author.clone()),
        text: diff.text.as_ref().map(|_| base.text.clone()),
        viewpoint_ref: diff.viewpoint_ref.as_ref().map(|_| base.viewpoint_ref.clone()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_viewpoint(base: &BcfViewpoint, diff: &BcfViewpointDiff) -> BcfViewpointDiff {
    BcfViewpointDiff { camera: diff.camera.as_ref().map(|_| base.camera.clone()), components: diff.components.as_ref().map(|_| base.components.clone()), snapshot: diff.snapshot.as_ref().map(|_| base.snapshot.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_part(base: &BcfRawPart, diff: &BcfPartDiff) -> BcfPartDiff {
    BcfPartDiff { data: diff.data.as_ref().map(|_| base.data.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_topic(base: &BcfTopic, other: &BcfTopic) -> Option<BcfTopicDiff> {
    let title = if base.title != other.title { Some(other.title.clone()) } else { None };
    let description = if base.description != other.description { Some(other.description.clone()) } else { None };
    let status = if base.status != other.status { Some(other.status.clone()) } else { None };
    let priority = if base.priority != other.priority { Some(other.priority.clone()) } else { None };
    let labels = if base.labels != other.labels { Some(other.labels.clone()) } else { None };
    let creation_date = if base.creation_date != other.creation_date { Some(other.creation_date.clone()) } else { None };
    let creation_author = if base.creation_author != other.creation_author { Some(other.creation_author.clone()) } else { None };
    let comments = between_named(&base.comments, &other.comments, |c| c.guid.clone(), between_comment);
    let viewpoints = between_named(&base.viewpoints, &other.viewpoints, |v| v.guid.clone(), between_viewpoint);
    if title.is_none() && description.is_none() && status.is_none() && priority.is_none() && labels.is_none() && creation_date.is_none() && creation_author.is_none() && comments.is_none() && viewpoints.is_none() {
        None
    } else {
        Some(BcfTopicDiff { title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_comment(base: &BcfComment, other: &BcfComment) -> Option<BcfCommentDiff> {
    let date = if base.date != other.date { Some(other.date.clone()) } else { None };
    let author = if base.author != other.author { Some(other.author.clone()) } else { None };
    let text = if base.text != other.text { Some(other.text.clone()) } else { None };
    let viewpoint_ref = if base.viewpoint_ref != other.viewpoint_ref { Some(other.viewpoint_ref.clone()) } else { None };
    if date.is_none() && author.is_none() && text.is_none() && viewpoint_ref.is_none() {
        None
    } else {
        Some(BcfCommentDiff { date, author, text, viewpoint_ref })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_viewpoint(base: &BcfViewpoint, other: &BcfViewpoint) -> Option<BcfViewpointDiff> {
    let camera = if base.camera != other.camera { Some(other.camera.clone()) } else { None };
    let components = if base.components != other.components { Some(other.components.clone()) } else { None };
    let snapshot = if base.snapshot != other.snapshot { Some(other.snapshot.clone()) } else { None };
    if camera.is_none() && components.is_none() && snapshot.is_none() {
        None
    } else {
        Some(BcfViewpointDiff { camera, components, snapshot })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_part(base: &BcfRawPart, other: &BcfRawPart) -> Option<BcfPartDiff> {
    if base.data != other.data {
        Some(BcfPartDiff { data: Some(other.data.clone()) })
    } else {
        None
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_topic_diff(mut a: BcfTopicDiff, b: BcfTopicDiff) -> BcfTopicDiff {
    if b.title.is_some() {
        a.title = b.title;
    }
    if b.description.is_some() {
        a.description = b.description;
    }
    if b.status.is_some() {
        a.status = b.status;
    }
    if b.priority.is_some() {
        a.priority = b.priority;
    }
    if b.labels.is_some() {
        a.labels = b.labels;
    }
    if b.creation_date.is_some() {
        a.creation_date = b.creation_date;
    }
    if b.creation_author.is_some() {
        a.creation_author = b.creation_author;
    }
    a.comments = match (a.comments.take(), b.comments) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(absorb_named(x, y, |c| c.guid.clone(), absorb_comment_diff, apply_comment)),
    };
    a.viewpoints = match (a.viewpoints.take(), b.viewpoints) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(absorb_named(x, y, |v| v.guid.clone(), absorb_viewpoint_diff, apply_viewpoint)),
    };
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_comment_diff(mut a: BcfCommentDiff, b: BcfCommentDiff) -> BcfCommentDiff {
    if b.date.is_some() {
        a.date = b.date;
    }
    if b.author.is_some() {
        a.author = b.author;
    }
    if b.text.is_some() {
        a.text = b.text;
    }
    if b.viewpoint_ref.is_some() {
        a.viewpoint_ref = b.viewpoint_ref;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_viewpoint_diff(mut a: BcfViewpointDiff, b: BcfViewpointDiff) -> BcfViewpointDiff {
    if b.camera.is_some() {
        a.camera = b.camera;
    }
    if b.components.is_some() {
        a.components = b.components;
    }
    if b.snapshot.is_some() {
        a.snapshot = b.snapshot;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_part_diff(mut a: BcfPartDiff, b: BcfPartDiff) -> BcfPartDiff {
    if b.data.is_some() {
        a.data = b.data;
    }
    a
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No
/// `snapshot: Option<BcfSnapshot>` full-replace slot -- this IS `BcfDiff::between`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &BcfSnapshot, next: &BcfSnapshot) -> BcfDiff {
    BcfDiff::between(base, next)
}
//#endregion 🔖️SetSnapshot

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6: hand-rolled `protocol::DiffCodec` for `BcfDiff` — required per the doc comment on
/// `BcfDiff` above (both `NamedTripleDiff`'s missing `DslField` impl AND the tri-state/`BcfCamera`
/// enum blockers, confirmed via real `cargo check` errors). Same grammar style established by
/// `GifDiff`/`SvgDiff` (bracket-depth-aware split, hex for strings/bytes, `[0]`/`[1,x]` for
/// `Option<T>`, tag-prefixed single letters for data-carrying enums) — see `f6-recon-report.md`
/// §5 for the primitive rationale. This artifact's own copy of the small helper set (no shared
/// "hand-roll helpers" module exists yet); adds `enc_list`/`dec_list` and generic
/// `enc_named_triple`/`dec_named_triple` beyond the recon's base primitive set (a direct,
/// in-spirit extension: bcf's `NamedTripleDiff<K,D,T>` engine is itself generic, so its grammar
/// codec is written generically once and instantiated per collection, rather than copy-pasted
/// per collection the way svg's non-generic `SvgChildrenDiff`/`SvgAttributesDiff` needed).
//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => {
                out.push(&s[start..i]);
                start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
/// 📋️ Bracketed comma-joined list -- the un-keyed sibling of `NamedTripleDiff`'s codec below, for
/// plain `Vec<T>` fields (`labels`, `exceptions`, `selection`, `coloring`, ...) that are
/// whole-value replaced rather than key-diffed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️NamedTripleCodec
/// 🏷️ Generic codec for the `NamedTripleDiff<K,D,T>` engine (this file's own §GenericNamedEngine
/// region) -- `[removed];[modified];[added]`, semicolon-separated sections, each a comma-separated
/// list; `modified` entries are `key:diff` (colon-separated, unambiguous because every key here is
/// hex-encoded and hex never contains `:`). Written once, generically, and instantiated per
/// collection (`topics`/`comments`/`viewpoints`/`parts`) below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_triple<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = triple.removed.iter().map(|k| enc_k(k)).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_triple<K, D, T>(s: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_k(e)).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, rest) = entry.split_once(':').ok_or_else(|| format!("named triple modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_k(k)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_t(e)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️NamedTripleCodec

//#region 🔖️ValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &BcfPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<BcfPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(BcfPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

/// 📷 `P[view_point,direction,up_vector,field_of_view]` (Perspective) / `O[...,view_to_world_scale]`
/// (Orthogonal) -- single-letter tag prefix, the `xs:choice` made concrete (same convention as
/// `enc_xml_node`'s `E`/`T`/`D`/`M`/`P` tags in svg's hand-rolled codec).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_camera(c: &BcfCamera) -> String {
    match c {
        BcfCamera::Perspective { view_point, direction, up_vector, field_of_view } => {
            format!("P[{},{},{},{}]", enc_point3(view_point), enc_point3(direction), enc_point3(up_vector), field_of_view)
        }
        BcfCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => {
            format!("O[{},{},{},{}]", enc_point3(view_point), enc_point3(direction), enc_point3(up_vector), view_to_world_scale)
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_camera(s: &str) -> Result<BcfCamera, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    let [view_point, direction, up_vector, last] = parts.as_slice() else { return Err(format!("camera: expected 4 fields, got {}", parts.len())) };
    match tag {
        "P" => Ok(BcfCamera::Perspective { view_point: dec_point3(view_point)?, direction: dec_point3(direction)?, up_vector: dec_point3(up_vector)?, field_of_view: parse_f64(last)? }),
        "O" => Ok(BcfCamera::Orthogonal { view_point: dec_point3(view_point)?, direction: dec_point3(direction)?, up_vector: dec_point3(up_vector)?, view_to_world_scale: parse_f64(last)? }),
        other => Err(format!("camera: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_visibility(v: &BcfVisibility) -> String {
    format!("[{},{}]", if v.default_visibility { "1" } else { "0" }, enc_list(&v.exceptions, |s: &String| enc_str(s)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_visibility(s: &str) -> Result<BcfVisibility, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [default_visibility, exceptions] = parts.as_slice() else { return Err(format!("visibility: expected 2 fields, got {}", parts.len())) };
    Ok(BcfVisibility { default_visibility: *default_visibility == "1", exceptions: dec_list(exceptions, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_coloring(c: &BcfColoring) -> String {
    format!("[{},{}]", enc_str(&c.color), enc_list(&c.components, |s: &String| enc_str(s)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_coloring(s: &str) -> Result<BcfColoring, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color, components] = parts.as_slice() else { return Err(format!("coloring: expected 2 fields, got {}", parts.len())) };
    Ok(BcfColoring { color: dec_str(color)?, components: dec_list(components, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_components(c: &BcfComponents) -> String {
    format!("[{},{},{}]", enc_list(&c.selection, |s: &String| enc_str(s)), enc_visibility(&c.visibility), enc_list(&c.coloring, enc_coloring))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_components(s: &str) -> Result<BcfComponents, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [selection, visibility, coloring] = parts.as_slice() else { return Err(format!("components: expected 3 fields, got {}", parts.len())) };
    Ok(BcfComponents { selection: dec_list(selection, dec_str)?, visibility: dec_visibility(visibility)?, coloring: dec_list(coloring, dec_coloring)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment(c: &BcfComment) -> String {
    format!("[{},{},{},{},{}]", enc_str(&c.guid), enc_str(&c.date), enc_str(&c.author), enc_str(&c.text), encode_option(&c.viewpoint_ref, |v: &String| enc_str(v)),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment(s: &str) -> Result<BcfComment, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, date, author, text, viewpoint_ref] = parts.as_slice() else { return Err(format!("comment: expected 5 fields, got {}", parts.len())) };
    Ok(BcfComment { guid: dec_str(guid)?, date: dec_str(date)?, author: dec_str(author)?, text: dec_str(text)?, viewpoint_ref: decode_option(viewpoint_ref, dec_str)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint(v: &BcfViewpoint) -> String {
    format!("[{},{},{},{}]", enc_str(&v.guid), encode_option(&v.camera, enc_camera), encode_option(&v.components, enc_components), encode_option(&v.snapshot, |b: &Vec<u8>| enc_bytes(b)),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint(s: &str) -> Result<BcfViewpoint, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, camera, components, snapshot] = parts.as_slice() else { return Err(format!("viewpoint: expected 4 fields, got {}", parts.len())) };
    Ok(BcfViewpoint { guid: dec_str(guid)?, camera: decode_option(camera, dec_camera)?, components: decode_option(components, dec_components)?, snapshot: decode_option(snapshot, dec_bytes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic(t: &BcfTopic) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{}]",
        enc_str(&t.guid),
        enc_str(&t.title),
        enc_str(&t.description),
        enc_str(&t.status),
        enc_str(&t.priority),
        enc_list(&t.labels, |s: &String| enc_str(s)),
        enc_str(&t.creation_date),
        enc_str(&t.creation_author),
        enc_list(&t.comments, enc_comment),
        enc_list(&t.viewpoints, enc_viewpoint),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic(s: &str) -> Result<BcfTopic, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints] = parts.as_slice() else {
        return Err(format!("topic: expected 10 fields, got {}", parts.len()));
    };
    Ok(BcfTopic {
        guid: dec_str(guid)?,
        title: dec_str(title)?,
        description: dec_str(description)?,
        status: dec_str(status)?,
        priority: dec_str(priority)?,
        labels: dec_list(labels, dec_str)?,
        creation_date: dec_str(creation_date)?,
        creation_author: dec_str(creation_author)?,
        comments: dec_list(comments, dec_comment)?,
        viewpoints: dec_list(viewpoints, dec_viewpoint)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part(p: &BcfRawPart) -> String {
    format!("[{},{}]", enc_str(&p.name), enc_bytes(&p.data))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part(s: &str) -> Result<BcfRawPart, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, data] = parts.as_slice() else { return Err(format!("part: expected 2 fields, got {}", parts.len())) };
    Ok(BcfRawPart { name: dec_str(name)?, data: dec_bytes(data)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment_diff(d: &BcfCommentDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.date, |v: &String| enc_str(v)),
        encode_option(&d.author, |v: &String| enc_str(v)),
        encode_option(&d.text, |v: &String| enc_str(v)),
        encode_option(&d.viewpoint_ref, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment_diff(s: &str) -> Result<BcfCommentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [date, author, text, viewpoint_ref] = parts.as_slice() else { return Err(format!("comment diff: expected 4 fields, got {}", parts.len())) };
    Ok(BcfCommentDiff { date: decode_option(date, dec_str)?, author: decode_option(author, dec_str)?, text: decode_option(text, dec_str)?, viewpoint_ref: decode_option(viewpoint_ref, |s| decode_option(s, dec_str))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint_diff(d: &BcfViewpointDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.camera, |inner: &Option<BcfCamera>| encode_option(inner, enc_camera)),
        encode_option(&d.components, |inner: &Option<BcfComponents>| encode_option(inner, enc_components)),
        encode_option(&d.snapshot, |inner: &Option<Vec<u8>>| encode_option(inner, |b: &Vec<u8>| enc_bytes(b))),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint_diff(s: &str) -> Result<BcfViewpointDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [camera, components, snapshot] = parts.as_slice() else { return Err(format!("viewpoint diff: expected 3 fields, got {}", parts.len())) };
    Ok(BcfViewpointDiff { camera: decode_option(camera, |s| decode_option(s, dec_camera))?, components: decode_option(components, |s| decode_option(s, dec_components))?, snapshot: decode_option(snapshot, |s| decode_option(s, dec_bytes))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part_diff(d: &BcfPartDiff) -> String {
    format!("[{}]", encode_option(&d.data, |b: &Vec<u8>| enc_bytes(b)))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part_diff(s: &str) -> Result<BcfPartDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BcfPartDiff { data: decode_option(inner, dec_bytes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic_diff(d: &BcfTopicDiff) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{}]",
        encode_option(&d.title, |v: &String| enc_str(v)),
        encode_option(&d.description, |v: &String| enc_str(v)),
        encode_option(&d.status, |v: &String| enc_str(v)),
        encode_option(&d.priority, |v: &String| enc_str(v)),
        encode_option(&d.labels, |v: &Vec<String>| enc_list(v, |s| enc_str(s))),
        encode_option(&d.creation_date, |v: &String| enc_str(v)),
        encode_option(&d.creation_author, |v: &String| enc_str(v)),
        encode_option(&d.comments, |v: &BcfCommentsDiff| enc_named_triple(v, |k: &String| enc_str(k), enc_comment_diff, enc_comment)),
        encode_option(&d.viewpoints, |v: &BcfViewpointsDiff| enc_named_triple(v, |k: &String| enc_str(k), enc_viewpoint_diff, enc_viewpoint)),
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic_diff(s: &str) -> Result<BcfTopicDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints] = parts.as_slice() else {
        return Err(format!("topic diff: expected 9 fields, got {}", parts.len()));
    };
    Ok(BcfTopicDiff {
        title: decode_option(title, dec_str)?,
        description: decode_option(description, dec_str)?,
        status: decode_option(status, dec_str)?,
        priority: decode_option(priority, dec_str)?,
        labels: decode_option(labels, |s| dec_list(s, dec_str))?,
        creation_date: decode_option(creation_date, dec_str)?,
        creation_author: decode_option(creation_author, dec_str)?,
        comments: decode_option(comments, |s| dec_named_triple(s, dec_str, dec_comment_diff, dec_comment))?,
        viewpoints: decode_option(viewpoints, |s| dec_named_triple(s, dec_str, dec_viewpoint_diff, dec_viewpoint))?,
    })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffCodec::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️component.rs`'s
/// own upgraded `OpBinary`) -- replaces F6's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), 1-byte tri-state presence tags, and 1-byte enum-variant tags -- genuinely
/// structured binary, never hex-ASCII text reused as "binary". Same shape
/// `📜️docx/…/🔺️diff/🦀️component.rs`'s own `BinaryPrimitives`/`ValueBinaryCodecs`/
/// `GenericTripleBinaryCodecs`/`DiffValueBinaryCodecs` regions establish; duplicated here (not
/// imported) per this repo's per-artifact hand-roll convention.
//#region 🔖️BinaryPrimitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_opt_str(out: &mut Vec<u8>, opt: &Option<String>) {
    out.push(if opt.is_some() { 1 } else { 0 });
    if let Some(v) = opt {
        write_str_lp(out, v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_opt_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    Ok(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs
/// 🌳️ Full-item (non-diff) binary codecs, mirrored one-for-one against `../🔖️ValueCodecs`'s text
/// forms above. `pub(crate)` so `../🧬️mutations/🦀️component.rs` reuses these rather than
/// re-deriving its own copies (same intra-artifact reuse pattern the text codecs already use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3_bin(p: &BcfPoint3, out: &mut Vec<u8>) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfPoint3, String> {
    let x = reader.read_f64_le().map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().map_err(|e| e.to_string())?;
    let z = reader.read_f64_le().map_err(|e| e.to_string())?;
    Ok(BcfPoint3 { x, y, z })
}

/// 🌳️ `0`=Perspective / `1`=Orthogonal -- binary twin of `enc_camera`/`dec_camera`'s `P`/`O` tags.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_camera_bin(c: &BcfCamera, out: &mut Vec<u8>) {
    match c {
        BcfCamera::Perspective { view_point, direction, up_vector, field_of_view } => {
            out.push(0);
            enc_point3_bin(view_point, out);
            enc_point3_bin(direction, out);
            enc_point3_bin(up_vector, out);
            out.extend_from_slice(&field_of_view.to_le_bytes());
        }
        BcfCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale } => {
            out.push(1);
            enc_point3_bin(view_point, out);
            enc_point3_bin(direction, out);
            enc_point3_bin(up_vector, out);
            out.extend_from_slice(&view_to_world_scale.to_le_bytes());
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_camera_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfCamera, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(BcfCamera::Perspective { view_point: dec_point3_bin(reader)?, direction: dec_point3_bin(reader)?, up_vector: dec_point3_bin(reader)?, field_of_view: reader.read_f64_le().map_err(|e| e.to_string())? }),
        1 => Ok(BcfCamera::Orthogonal { view_point: dec_point3_bin(reader)?, direction: dec_point3_bin(reader)?, up_vector: dec_point3_bin(reader)?, view_to_world_scale: reader.read_f64_le().map_err(|e| e.to_string())? }),
        other => Err(format!("camera binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str_list_bin(items: &[String], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for s in items {
        write_str_lp(out, s);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<String>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(read_str_lp(reader)?);
    }
    Ok(out)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_visibility_bin(v: &BcfVisibility, out: &mut Vec<u8>) {
    out.push(v.default_visibility as u8);
    enc_str_list_bin(&v.exceptions, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_visibility_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfVisibility, String> {
    let default_visibility = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let exceptions = dec_str_list_bin(reader)?;
    Ok(BcfVisibility { default_visibility, exceptions })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_coloring_bin(c: &BcfColoring, out: &mut Vec<u8>) {
    write_str_lp(out, &c.color);
    enc_str_list_bin(&c.components, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_coloring_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfColoring, String> {
    let color = read_str_lp(reader)?;
    let components = dec_str_list_bin(reader)?;
    Ok(BcfColoring { color, components })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_components_bin(c: &BcfComponents, out: &mut Vec<u8>) {
    enc_str_list_bin(&c.selection, out);
    enc_visibility_bin(&c.visibility, out);
    store::pack_rt::write_varint_u64(out, c.coloring.len() as u64);
    for entry in &c.coloring {
        enc_coloring_bin(entry, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_components_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfComponents, String> {
    let selection = dec_str_list_bin(reader)?;
    let visibility = dec_visibility_bin(reader)?;
    let coloring_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut coloring = Vec::with_capacity(coloring_count as usize);
    for _ in 0..coloring_count {
        coloring.push(dec_coloring_bin(reader)?);
    }
    Ok(BcfComponents { selection, visibility, coloring })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment_bin(c: &BcfComment, out: &mut Vec<u8>) {
    write_str_lp(out, &c.guid);
    write_str_lp(out, &c.date);
    write_str_lp(out, &c.author);
    write_str_lp(out, &c.text);
    write_opt_str(out, &c.viewpoint_ref);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfComment, String> {
    let guid = read_str_lp(reader)?;
    let date = read_str_lp(reader)?;
    let author = read_str_lp(reader)?;
    let text = read_str_lp(reader)?;
    let viewpoint_ref = read_opt_str(reader)?;
    Ok(BcfComment { guid, date, author, text, viewpoint_ref })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint_bin(v: &BcfViewpoint, out: &mut Vec<u8>) {
    write_str_lp(out, &v.guid);
    out.push(if v.camera.is_some() { 1 } else { 0 });
    if let Some(camera) = &v.camera {
        enc_camera_bin(camera, out);
    }
    out.push(if v.components.is_some() { 1 } else { 0 });
    if let Some(components) = &v.components {
        enc_components_bin(components, out);
    }
    out.push(if v.snapshot.is_some() { 1 } else { 0 });
    if let Some(snapshot) = &v.snapshot {
        write_bytes_lp(out, snapshot);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfViewpoint, String> {
    let guid = read_str_lp(reader)?;
    let camera = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_camera_bin(reader)?) } else { None };
    let components = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_components_bin(reader)?) } else { None };
    let snapshot = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
    Ok(BcfViewpoint { guid, camera, components, snapshot })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic_bin(t: &BcfTopic, out: &mut Vec<u8>) {
    write_str_lp(out, &t.guid);
    write_str_lp(out, &t.title);
    write_str_lp(out, &t.description);
    write_str_lp(out, &t.status);
    write_str_lp(out, &t.priority);
    enc_str_list_bin(&t.labels, out);
    write_str_lp(out, &t.creation_date);
    write_str_lp(out, &t.creation_author);
    store::pack_rt::write_varint_u64(out, t.comments.len() as u64);
    for c in &t.comments {
        enc_comment_bin(c, out);
    }
    store::pack_rt::write_varint_u64(out, t.viewpoints.len() as u64);
    for v in &t.viewpoints {
        enc_viewpoint_bin(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfTopic, String> {
    let guid = read_str_lp(reader)?;
    let title = read_str_lp(reader)?;
    let description = read_str_lp(reader)?;
    let status = read_str_lp(reader)?;
    let priority = read_str_lp(reader)?;
    let labels = dec_str_list_bin(reader)?;
    let creation_date = read_str_lp(reader)?;
    let creation_author = read_str_lp(reader)?;
    let comment_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut comments = Vec::with_capacity(comment_count as usize);
    for _ in 0..comment_count {
        comments.push(dec_comment_bin(reader)?);
    }
    let viewpoint_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut viewpoints = Vec::with_capacity(viewpoint_count as usize);
    for _ in 0..viewpoint_count {
        viewpoints.push(dec_viewpoint_bin(reader)?);
    }
    Ok(BcfTopic { guid, title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part_bin(p: &BcfRawPart, out: &mut Vec<u8>) {
    write_str_lp(out, &p.name);
    write_bytes_lp(out, &p.data);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfRawPart, String> {
    let name = read_str_lp(reader)?;
    let data = read_bytes_lp(reader)?;
    Ok(BcfRawPart { name, data })
}

/// 🌱 Full (non-diff) `BcfSnapshot` binary codec -- only `SetSnapshot`'s whole-payload encoding
/// needs this, mirroring `enc_bcf_snapshot`'s text form above.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bcf_snapshot_bin(s: &BcfSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    write_str_lp(out, &s.version);
    store::pack_rt::write_varint_u64(out, s.topics.len() as u64);
    for t in &s.topics {
        enc_topic_bin(t, out);
    }
    store::pack_rt::write_varint_u64(out, s.parts.len() as u64);
    for p in &s.parts {
        enc_part_bin(p, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bcf_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let version = read_str_lp(reader)?;
    let topic_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut topics = Vec::with_capacity(topic_count as usize);
    for _ in 0..topic_count {
        topics.push(dec_topic_bin(reader)?);
    }
    let part_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut parts = Vec::with_capacity(part_count as usize);
    for _ in 0..part_count {
        parts.push(dec_part_bin(reader)?);
    }
    Ok(BcfSnapshot { schema, version, topics, parts })
}
//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️GenericNamedTripleBinaryCodecs
/// 🏷️ Binary twin of `enc_named_triple`/`dec_named_triple` -- three varint-counted sections
/// (removed keys / modified key+diff pairs / added whole items), generic over `K`/`D`/`T`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_named_triple_bin<K, D, T>(triple: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K, &mut Vec<u8>), enc_d: impl Fn(&D, &mut Vec<u8>), enc_t: impl Fn(&T, &mut Vec<u8>), out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, triple.removed.len() as u64);
    for k in &triple.removed {
        enc_k(k, out);
    }
    store::pack_rt::write_varint_u64(out, triple.modified.len() as u64);
    for m in &triple.modified {
        enc_k(&m.key, out);
        enc_d(&m.diff, out);
    }
    store::pack_rt::write_varint_u64(out, triple.added.len() as u64);
    for t in &triple.added {
        enc_t(t, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_named_triple_bin<K, D, T>(
    reader: &mut store::ByteReader<'_>,
    dec_k: impl Fn(&mut store::ByteReader<'_>) -> Result<K, String>,
    dec_d: impl Fn(&mut store::ByteReader<'_>) -> Result<D, String>,
    dec_t: impl Fn(&mut store::ByteReader<'_>) -> Result<T, String>,
) -> Result<NamedTripleDiff<K, D, T>, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(dec_k(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let key = dec_k(reader)?;
        let diff = dec_d(reader)?;
        modified.push(NamedModified { key, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        added.push(dec_t(reader)?);
    }
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️GenericNamedTripleBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_comment_diff_bin(d: &BcfCommentDiff, out: &mut Vec<u8>) {
    write_opt_str(out, &d.date);
    write_opt_str(out, &d.author);
    write_opt_str(out, &d.text);
    out.push(if d.viewpoint_ref.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.viewpoint_ref {
        write_opt_str(out, inner);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_comment_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfCommentDiff, String> {
    let date = read_opt_str(reader)?;
    let author = read_opt_str(reader)?;
    let text = read_opt_str(reader)?;
    let viewpoint_ref = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_opt_str(reader)?) } else { None };
    Ok(BcfCommentDiff { date, author, text, viewpoint_ref })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_viewpoint_diff_bin(d: &BcfViewpointDiff, out: &mut Vec<u8>) {
    out.push(if d.camera.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.camera {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(camera) = inner {
            enc_camera_bin(camera, out);
        }
    }
    out.push(if d.components.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.components {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(components) = inner {
            enc_components_bin(components, out);
        }
    }
    out.push(if d.snapshot.is_some() { 1 } else { 0 });
    if let Some(inner) = &d.snapshot {
        out.push(if inner.is_some() { 1 } else { 0 });
        if let Some(snapshot) = inner {
            write_bytes_lp(out, snapshot);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_viewpoint_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfViewpointDiff, String> {
    let camera = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_camera_bin(reader)?) } else { None }) } else { None };
    let components = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_components_bin(reader)?) } else { None }) } else { None };
    let snapshot = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None }) } else { None };
    Ok(BcfViewpointDiff { camera, components, snapshot })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_part_diff_bin(d: &BcfPartDiff, out: &mut Vec<u8>) {
    out.push(if d.data.is_some() { 1 } else { 0 });
    if let Some(v) = &d.data {
        write_bytes_lp(out, v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_part_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfPartDiff, String> {
    let data = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_bytes_lp(reader)?) } else { None };
    Ok(BcfPartDiff { data })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_comments_diff_bin(d: &BcfCommentsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_comment_diff_bin, enc_comment_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_comments_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfCommentsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_comment_diff_bin, dec_comment_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_viewpoints_diff_bin(d: &BcfViewpointsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_viewpoint_diff_bin, enc_viewpoint_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_viewpoints_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfViewpointsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_viewpoint_diff_bin, dec_viewpoint_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topic_diff_bin(d: &BcfTopicDiff, out: &mut Vec<u8>) {
    write_opt_str(out, &d.title);
    write_opt_str(out, &d.description);
    write_opt_str(out, &d.status);
    write_opt_str(out, &d.priority);
    out.push(if d.labels.is_some() { 1 } else { 0 });
    if let Some(v) = &d.labels {
        enc_str_list_bin(v, out);
    }
    write_opt_str(out, &d.creation_date);
    write_opt_str(out, &d.creation_author);
    out.push(if d.comments.is_some() { 1 } else { 0 });
    if let Some(v) = &d.comments {
        enc_comments_diff_bin(v, out);
    }
    out.push(if d.viewpoints.is_some() { 1 } else { 0 });
    if let Some(v) = &d.viewpoints {
        enc_viewpoints_diff_bin(v, out);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topic_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfTopicDiff, String> {
    let title = read_opt_str(reader)?;
    let description = read_opt_str(reader)?;
    let status = read_opt_str(reader)?;
    let priority = read_opt_str(reader)?;
    let labels = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_str_list_bin(reader)?) } else { None };
    let creation_date = read_opt_str(reader)?;
    let creation_author = read_opt_str(reader)?;
    let comments = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_comments_diff_bin(reader)?) } else { None };
    let viewpoints = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_viewpoints_diff_bin(reader)?) } else { None };
    Ok(BcfTopicDiff { title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_topics_diff_bin(d: &BcfTopicsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_topic_diff_bin, enc_topic_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_topics_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfTopicsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_topic_diff_bin, dec_topic_bin)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_parts_diff_bin(d: &BcfPartsDiff, out: &mut Vec<u8>) {
    enc_named_triple_bin(d, |k, out| write_str_lp(out, k), enc_part_diff_bin, enc_part_bin, out)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_parts_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<BcfPartsDiff, String> {
    dec_named_triple_bin(reader, |r| read_str_lp(r), dec_part_diff_bin, dec_part_bin)
}
//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_bcf_diff(d: &BcfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.version {
        tokens.push(format!("version={}", enc_str(v)));
    }
    if let Some(t) = &d.topics {
        tokens.push(format!("topics={}", enc_named_triple(t, |k: &String| enc_str(k), enc_topic_diff, enc_topic)));
    }
    if let Some(p) = &d.parts {
        tokens.push(format!("parts={}", enc_named_triple(p, |k: &String| enc_str(k), enc_part_diff, enc_part)));
    }
    tokens.join(" ")
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_bcf_diff(line: &str) -> Result<BcfDiff, String> {
    let mut d = BcfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("version=") {
            d.version = Some(dec_str(rest)?);
        } else if let Some(rest) = token.strip_prefix("topics=") {
            d.topics = Some(dec_named_triple(rest, dec_str, dec_topic_diff, dec_topic)?);
        } else if let Some(rest) = token.strip_prefix("parts=") {
            d.parts = Some(dec_named_triple(rest, dec_str, dec_part_diff, dec_part)?);
        } else {
            return Err(format!("bcf diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffCodec for BcfDiff {
    async fn print_diff(&self) -> String {
        print_bcf_diff(self)
    }
    async fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_bcf_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// 🧪️ FG-wave: REAL binary frame (`format u8 | flags u8 | [version][topics][parts]`), matching
    /// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
    /// upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (per this ticket's
    /// own `📖️grammar-recipe.md` §4/§6 census, 100% of stdio's `DiffCodec` impls were still on
    /// that shortcut before this pilot ladder). `flags` bit0/bit1/bit2 mark
    /// `version`/`topics`/`parts` presence; each present field's own binary payload follows in
    /// that fixed order (see `🔖️BinaryCodecs` above).
    async fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut flags: u8 = 0;
        if self.version.is_some() {
            flags |= 0b001;
        }
        if self.topics.is_some() {
            flags |= 0b010;
        }
        if self.parts.is_some() {
            flags |= 0b100;
        }
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
        if let Some(v) = &self.version {
            write_str_lp(&mut out, v);
        }
        if let Some(t) = &self.topics {
            enc_topics_diff_bin(t, &mut out);
        }
        if let Some(p) = &self.parts {
            enc_parts_diff_bin(p, &mut out);
        }
        Ok(out)
    }
    async fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().await.map_err(|e| malformed("diff format", 0, e.to_string()))?;
        let flags = reader.read_u8().await.map_err(|e| malformed("diff flags", 1, e.to_string()))?;
        let version = if flags & 0b001 != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("diff version", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let topics = if flags & 0b010 != 0 { Some(dec_topics_diff_bin(&mut reader).map_err(|e| malformed("diff topics", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        let parts = if flags & 0b100 != 0 { Some(dec_parts_diff_bin(&mut reader).map_err(|e| malformed("diff parts", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
        Ok(BcfDiff { version, topics, parts })
    }
}
//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `BcfSnapshot` pair (self-contained, NOT `⚙️engine`'s own
/// `#[cfg(test)]`-gated `sample_snapshot`/`sweep_a`/`sweep_b` -- those live behind `#[cfg(test)]`
/// in a DIFFERENT file and are not reachable from here) -- every mutable field differs (incl. one
/// removed/one modified-in-every-field/one added topic/comment/viewpoint, both `BcfCamera`
/// variants, and every tri-state field's `Some(None)` transition), the single source of truth
/// reused by `demo_diff_cases()` below AND by `⚙️engine/🦀️component.rs`'s own
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests, same shape
/// `📜️docx/…/🔺️diff/🦀️component.rs`'s own `snapshot_a()`/`snapshot_b()` establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_snapshot_a() -> BcfSnapshot {
    BcfSnapshot {
        schema: crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(),
        version: "2.1".into(),
        topics: vec![
            BcfTopic {
                guid: "keep".into(),
                title: "Keep-topic before".into(),
                description: "before desc".into(),
                status: "Open".into(),
                priority: "Low".into(),
                labels: vec!["before".into()],
                creation_date: "2024-01-01T00:00:00+00:00".into(),
                creation_author: "a@example.com".into(),
                comments: vec![
                    BcfComment { guid: "c-keep".into(), date: "2024-01-01T00:00:00+00:00".into(), author: "a@example.com".into(), text: "before text".into(), viewpoint_ref: Some("vp-remove".into()) },
                    BcfComment { guid: "c-remove".into(), date: "2024-01-01T00:00:00+00:00".into(), author: "a@example.com".into(), text: "will be removed".into(), viewpoint_ref: Some("vp-keep".into()) },
                ],
                viewpoints: vec![
                    BcfViewpoint {
                        guid: "vp-keep".into(),
                        camera: Some(BcfCamera::Perspective { view_point: BcfPoint3 { x: 1.0, y: 2.0, z: 3.0 }, direction: BcfPoint3 { x: 0.0, y: 0.0, z: -1.0 }, up_vector: BcfPoint3 { x: 0.0, y: 1.0, z: 0.0 }, field_of_view: 60.0 }),
                        components: Some(BcfComponents {
                            selection: vec!["2O2Fr$t4X7Zf8NOew3FLOH".into()],
                            visibility: BcfVisibility { default_visibility: false, exceptions: vec!["1yQBoo7d5EEBLiyMxGgTLc".into()] },
                            coloring: vec![BcfColoring { color: "FFFF0000".into(), components: vec!["0BTBFw6f90Nfh9rP1dl_3n".into()] }],
                        }),
                        snapshot: Some(vec![2]),
                    },
                    BcfViewpoint { guid: "vp-remove".into(), camera: None, components: None, snapshot: Some(vec![1]) },
                ],
            },
            BcfTopic {
                guid: "topic-remove".into(),
                title: "Will be removed".into(),
                description: String::new(),
                status: "Open".into(),
                priority: String::new(),
                labels: Vec::new(),
                creation_date: String::new(),
                creation_author: String::new(),
                comments: Vec::new(),
                viewpoints: Vec::new(),
            },
        ],
        parts: vec![BcfRawPart { name: "part-keep.txt".into(), data: b"before".to_vec() }, BcfRawPart { name: "part-remove.txt".into(), data: b"gone".to_vec() }],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_snapshot_b() -> BcfSnapshot {
    BcfSnapshot {
        schema: crate::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA.into(),
        version: "2.2".into(),
        topics: vec![
            BcfTopic {
                guid: "keep".into(),
                title: "Keep-topic after".into(),
                description: "after desc".into(),
                status: "Closed".into(),
                priority: "High".into(),
                labels: vec!["after".into(), "second".into()],
                creation_date: "2024-02-02T00:00:00+00:00".into(),
                creation_author: "b@example.com".into(),
                comments: vec![
                    BcfComment { guid: "c-keep".into(), date: "2024-02-02T00:00:00+00:00".into(), author: "b@example.com".into(), text: "after text".into(), viewpoint_ref: None },
                    BcfComment { guid: "c-add".into(), date: "2024-02-02T00:00:00+00:00".into(), author: "b@example.com".into(), text: "newly added".into(), viewpoint_ref: Some("vp-keep".into()) },
                ],
                viewpoints: vec![
                    BcfViewpoint {
                        guid: "vp-keep".into(),
                        camera: Some(BcfCamera::Orthogonal { view_point: BcfPoint3 { x: 4.0, y: 5.0, z: 6.0 }, direction: BcfPoint3 { x: 1.0, y: 0.0, z: 0.0 }, up_vector: BcfPoint3 { x: 0.0, y: 0.0, z: 1.0 }, view_to_world_scale: 2.5 }),
                        components: None,
                        snapshot: None,
                    },
                    BcfViewpoint { guid: "vp-add".into(), camera: None, components: Some(BcfComponents::default()), snapshot: Some(vec![9]) },
                ],
            },
            BcfTopic {
                guid: "topic-add".into(),
                title: "Freshly added".into(),
                description: "added desc".into(),
                status: "Open".into(),
                priority: "Medium".into(),
                labels: vec!["fresh".into()],
                creation_date: "2024-03-03T00:00:00+00:00".into(),
                creation_author: "c@example.com".into(),
                comments: Vec::new(),
                viewpoints: Vec::new(),
            },
        ],
        parts: vec![BcfRawPart { name: "part-keep.txt".into(), data: b"after".to_vec() }, BcfRawPart { name: "part-add.txt".into(), data: b"new".to_vec() }],
    }
}

/// 🧪️ The demo cases proper — `default()` (empty diff) plus every real `between()` shape (both
/// directions, and the trivially-empty self-diff).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<BcfDiff> {
    let a = demo_snapshot_a();
    let b = demo_snapshot_b();
    vec![BcfDiff::default(), BcfDiff::between(&a, &b), BcfDiff::between(&b, &a), BcfDiff::between(&a, &a)]
}
//#endregion 🔖️DemoCases
//#endregion 🔖️HandcraftedDiffCodec
