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

use crate::schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility};
use crate::BcfSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

//#region 🔖️GenericNamedEngine
/// 🏷️ Name/key-keyed collection triple, generic over key `K`, item `T`, and per-field diff `D`.
/// `added` carries the full item (which already contains its own key). Mirrors docx's
/// `NamedTripleDiff` verbatim (see that module's doc comment for the `bound(...)` rationale — a
/// known serde_derive limitation where `#[value(default)]` on a `Vec<T>` field spuriously infers
/// `T: Default`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct NamedTripleDiff<K, D, T> {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<K>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<NamedModified<K, D>>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<T>,
}

impl<K, D, T> Default for NamedTripleDiff<K, D, T> {
    fn default() -> Self {
        Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
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
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
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
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf.diff")]
pub struct BcfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<BcfTopicsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<BcfPartsDiff>,
}

/// 🔺️ Per-topic sparse diff. `title`/`description`/`status`/`priority`/`creation_date`/
/// `creation_author` are scalar patches; `labels` is whole-value replaced (not itself a keyed
/// collection per the completeness target); `comments`/`viewpoints` are recursive guid-keyed
/// triples.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfTopicDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<BcfCommentsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub viewpoints: Option<BcfViewpointsDiff>,
}

/// 🔺️ Per-comment sparse diff. `viewpoint_ref` is tri-state (`Some(None)` = reference cleared).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfCommentDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub viewpoint_ref: Option<Option<String>>,
}

/// 🔺️ Per-viewpoint sparse diff. `camera`/`components`/`snapshot` are all weak (whole-value
/// replaced, per the recipe — never sub-diffed) and tri-state nullable.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfViewpointDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub camera: Option<Option<BcfCamera>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Option<BcfComponents>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Option<Vec<u8>>>,
}

/// 🔺️ Per-raw-part sparse diff — `name` is the key, so only `data` can change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BcfPartDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
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
    fn apply(&self, base: &BcfSnapshot) -> MutationApplyResult<BcfSnapshot> {
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

    fn absorb(&mut self, other: Self) {
        if other.version.is_some() {
            self.version = other.version;
        }
        self.topics = match (self.topics.take(), other.topics) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |t| t.guid.clone(), absorb_topic_diff, apply_topic)),
        };
        self.parts = match (self.parts.take(), other.parts) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |p| p.name.clone(), absorb_part_diff, apply_part)),
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
    fn inverse(&self, base: &BcfSnapshot) -> Self {
        BcfDiff {
            version: self.version.as_ref().map(|_| base.version.clone()),
            topics: self.topics.as_ref().map(|d| inverse_named(&base.topics, d, |t| t.guid.clone(), inverse_topic)),
            parts: self.parts.as_ref().map(|d| inverse_named(&base.parts, d, |p| p.name.clone(), inverse_part)),
        }
    }

    fn between(base: &BcfSnapshot, other: &BcfSnapshot) -> Self {
        BcfDiff {
            version: if base.version != other.version { Some(other.version.clone()) } else { None },
            topics: between_named(&base.topics, &other.topics, |t| t.guid.clone(), between_topic),
            parts: between_named(&base.parts, &other.parts, |p| p.name.clone(), between_part),
        }
    }

    fn is_empty(&self) -> bool {
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
        (Some(x), Some(y)) => Some(absorb_named(x, &y, |c| c.guid.clone(), absorb_comment_diff, apply_comment)),
    };
    a.viewpoints = match (a.viewpoints.take(), b.viewpoints) {
        (None, x) => x,
        (x, None) => x,
        (Some(x), Some(y)) => Some(absorb_named(x, &y, |v| v.guid.clone(), absorb_viewpoint_diff, apply_viewpoint)),
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














//#endregion 🔖️Primitives

//#region 🔖️NamedTripleCodec


//#endregion 🔖️NamedTripleCodec

//#region 🔖️ValueCodecs


























//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs











//#endregion 🔖️DiffValueCodecs

//#region 🔖️BinaryCodecs
/// 🧪️ FG-wave: real recursive BINARY twins of every text-form codec above, backing the upgraded
/// `DiffBinary::encode_diff`/`decode_diff` below (and, via re-export, `../🧬️mutations/🦀️.rs`'s
/// own upgraded `OpBinary`) -- replaces F6's `print_diff().into_bytes()` text-as-binary shortcut.
/// Real LEB128-varint-framed length-prefixed strings/bytes (`store::pack_rt::write_varint_u64` +
/// `store::ByteReader`), 1-byte tri-state presence tags, and 1-byte enum-variant tags -- genuinely
/// structured binary, never hex-ASCII text reused as "binary". Same shape
/// `📜️docx/…/🔺️diff/🦀️.rs`'s own `BinaryPrimitives`/`ValueBinaryCodecs`/
/// `GenericTripleBinaryCodecs`/`DiffValueBinaryCodecs` regions establish; duplicated here (not
/// imported) per this repo's per-artifact hand-roll convention.
//#region 🔖️BinaryPrimitives






//#endregion 🔖️BinaryPrimitives

//#region 🔖️ValueBinaryCodecs
































//#endregion 🔖️ValueBinaryCodecs

//#region 🔖️GenericNamedTripleBinaryCodecs


//#endregion 🔖️GenericNamedTripleBinaryCodecs

//#region 🔖️DiffValueBinaryCodecs























//#endregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️BinaryCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `BcfSnapshot` pair (self-contained, NOT `⚙️engine`'s own
/// `#[cfg(test)]`-gated `sample_snapshot`/`sweep_a`/`sweep_b` -- those live behind `#[cfg(test)]`
/// in a DIFFERENT file and are not reachable from here) -- every mutable field differs (incl. one
/// removed/one modified-in-every-field/one added topic/comment/viewpoint, both `BcfCamera`
/// variants, and every tri-state field's `Some(None)` transition), the single source of truth
/// reused by `demo_diff_cases()` below AND by `⚙️engine/🦀️.rs`'s own
/// `diff_grammar_conformance_law`/`protocol_walk_law` conformance tests, same shape
/// `📜️docx/…/🔺️diff/🦀️.rs`'s own `snapshot_a()`/`snapshot_b()` establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_snapshot_a() -> BcfSnapshot {
    BcfSnapshot {
        schema: crate::STDIO_BCF_DOCUMENT_SCHEMA.into(),
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
        schema: crate::STDIO_BCF_DOCUMENT_SCHEMA.into(),
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
