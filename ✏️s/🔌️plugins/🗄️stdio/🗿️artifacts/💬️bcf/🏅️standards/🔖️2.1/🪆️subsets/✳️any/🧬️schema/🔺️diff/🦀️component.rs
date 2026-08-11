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
use protocol::MutationDiff;
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
    fn default() -> Self { Self { removed: Vec::new(), modified: Vec::new(), added: Vec::new() } }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedModified<K, D> {
    pub key: K,
    pub diff: D,
}

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
    if removed.is_empty() && modified.is_empty() && added.is_empty() { None } else { Some(NamedTripleDiff { removed, modified, added }) }
}

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
fn absorb_named<K, T, D>(
    d1: NamedTripleDiff<K, D, T>,
    d2: NamedTripleDiff<K, D, T>,
    key_of: impl Fn(&T) -> K,
    absorb_item: impl Fn(D, D) -> D,
    apply_item: impl Fn(&mut T, &D),
) -> NamedTripleDiff<K, D, T>
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
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<BcfTopicsDiff>,
    #[state(persistent)]
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
pub fn wrap_topic_diff(guid: &str, diff: BcfTopicDiff) -> BcfDiff {
    BcfDiff {
        version: None,
        topics: Some(BcfTopicsDiff { removed: Vec::new(), modified: vec![NamedModified { key: guid.to_string(), diff }], added: Vec::new() }),
        parts: None,
    }
}

/// 🧭️ Lowers a per-comment leaf diff (inside topic `topic_guid`) into a full `BcfDiff`.
pub fn wrap_comment_diff(topic_guid: &str, comment_guid: &str, diff: BcfCommentDiff) -> BcfDiff {
    wrap_topic_diff(topic_guid, BcfTopicDiff {
        comments: Some(BcfCommentsDiff { removed: Vec::new(), modified: vec![NamedModified { key: comment_guid.to_string(), diff }], added: Vec::new() }),
        ..Default::default()
    })
}

/// 🧭️ Lowers a per-viewpoint leaf diff (inside topic `topic_guid`) into a full `BcfDiff`.
pub fn wrap_viewpoint_diff(topic_guid: &str, viewpoint_guid: &str, diff: BcfViewpointDiff) -> BcfDiff {
    wrap_topic_diff(topic_guid, BcfTopicDiff {
        viewpoints: Some(BcfViewpointsDiff { removed: Vec::new(), modified: vec![NamedModified { key: viewpoint_guid.to_string(), diff }], added: Vec::new() }),
        ..Default::default()
    })
}
//#endregion 🔖️WrapHelpers

//#region 🔖️Apply
impl MutationDiff<BcfSnapshot> for BcfDiff {
    fn apply(&self, base: &BcfSnapshot) -> BcfSnapshot {
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
        next
    }

    fn absorb(&mut self, other: Self) {
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

fn apply_topic(topic: &mut BcfTopic, diff: &BcfTopicDiff) {
    if let Some(v) = &diff.title { topic.title = v.clone(); }
    if let Some(v) = &diff.description { topic.description = v.clone(); }
    if let Some(v) = &diff.status { topic.status = v.clone(); }
    if let Some(v) = &diff.priority { topic.priority = v.clone(); }
    if let Some(v) = &diff.labels { topic.labels = v.clone(); }
    if let Some(v) = &diff.creation_date { topic.creation_date = v.clone(); }
    if let Some(v) = &diff.creation_author { topic.creation_author = v.clone(); }
    if let Some(cd) = &diff.comments {
        apply_named(&mut topic.comments, cd, |c| c.guid.clone(), apply_comment);
    }
    if let Some(vd) = &diff.viewpoints {
        apply_named(&mut topic.viewpoints, vd, |v| v.guid.clone(), apply_viewpoint);
    }
}

fn apply_comment(comment: &mut BcfComment, diff: &BcfCommentDiff) {
    if let Some(v) = &diff.date { comment.date = v.clone(); }
    if let Some(v) = &diff.author { comment.author = v.clone(); }
    if let Some(v) = &diff.text { comment.text = v.clone(); }
    if let Some(v) = &diff.viewpoint_ref { comment.viewpoint_ref = v.clone(); }
}

fn apply_viewpoint(vp: &mut BcfViewpoint, diff: &BcfViewpointDiff) {
    if let Some(v) = &diff.camera { vp.camera = v.clone(); }
    if let Some(v) = &diff.components { vp.components = v.clone(); }
    if let Some(v) = &diff.snapshot { vp.snapshot = v.clone(); }
}

fn apply_part(part: &mut BcfRawPart, diff: &BcfPartDiff) {
    if let Some(v) = &diff.data { part.data = v.clone(); }
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

fn inverse_comment(base: &BcfComment, diff: &BcfCommentDiff) -> BcfCommentDiff {
    BcfCommentDiff {
        date: diff.date.as_ref().map(|_| base.date.clone()),
        author: diff.author.as_ref().map(|_| base.author.clone()),
        text: diff.text.as_ref().map(|_| base.text.clone()),
        viewpoint_ref: diff.viewpoint_ref.as_ref().map(|_| base.viewpoint_ref.clone()),
    }
}

fn inverse_viewpoint(base: &BcfViewpoint, diff: &BcfViewpointDiff) -> BcfViewpointDiff {
    BcfViewpointDiff {
        camera: diff.camera.as_ref().map(|_| base.camera.clone()),
        components: diff.components.as_ref().map(|_| base.components.clone()),
        snapshot: diff.snapshot.as_ref().map(|_| base.snapshot.clone()),
    }
}

fn inverse_part(base: &BcfRawPart, diff: &BcfPartDiff) -> BcfPartDiff {
    BcfPartDiff { data: diff.data.as_ref().map(|_| base.data.clone()) }
}

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
    if title.is_none() && description.is_none() && status.is_none() && priority.is_none() && labels.is_none()
        && creation_date.is_none() && creation_author.is_none() && comments.is_none() && viewpoints.is_none()
    {
        None
    } else {
        Some(BcfTopicDiff { title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints })
    }
}

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

fn between_part(base: &BcfRawPart, other: &BcfRawPart) -> Option<BcfPartDiff> {
    if base.data != other.data { Some(BcfPartDiff { data: Some(other.data.clone()) }) } else { None }
}

fn absorb_topic_diff(mut a: BcfTopicDiff, b: BcfTopicDiff) -> BcfTopicDiff {
    if b.title.is_some() { a.title = b.title; }
    if b.description.is_some() { a.description = b.description; }
    if b.status.is_some() { a.status = b.status; }
    if b.priority.is_some() { a.priority = b.priority; }
    if b.labels.is_some() { a.labels = b.labels; }
    if b.creation_date.is_some() { a.creation_date = b.creation_date; }
    if b.creation_author.is_some() { a.creation_author = b.creation_author; }
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

fn absorb_comment_diff(mut a: BcfCommentDiff, b: BcfCommentDiff) -> BcfCommentDiff {
    if b.date.is_some() { a.date = b.date; }
    if b.author.is_some() { a.author = b.author; }
    if b.text.is_some() { a.text = b.text; }
    if b.viewpoint_ref.is_some() { a.viewpoint_ref = b.viewpoint_ref; }
    a
}

fn absorb_viewpoint_diff(mut a: BcfViewpointDiff, b: BcfViewpointDiff) -> BcfViewpointDiff {
    if b.camera.is_some() { a.camera = b.camera; }
    if b.components.is_some() { a.components = b.components; }
    if b.snapshot.is_some() { a.snapshot = b.snapshot; }
    a
}

fn absorb_part_diff(mut a: BcfPartDiff, b: BcfPartDiff) -> BcfPartDiff {
    if b.data.is_some() { a.data = b.data; }
    a
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️SetSnapshot
/// 🧩️ Builds the sparse field-by-field diff for a `SetSnapshot` mutation. No
/// `snapshot: Option<BcfSnapshot>` full-replace slot -- this IS `BcfDiff::between`.
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
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    hex_encode(b)
}
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    hex_decode(s)
}
fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}

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
pub(crate) fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
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
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
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
pub(crate) fn enc_named_triple<K, D, T>(
    triple: &NamedTripleDiff<K, D, T>,
    enc_k: impl Fn(&K) -> String,
    enc_d: impl Fn(&D) -> String,
    enc_t: impl Fn(&T) -> String,
) -> String {
    let removed = triple.removed.iter().map(|k| enc_k(k)).collect::<Vec<_>>().join(",");
    let modified = triple.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = triple.added.iter().map(|t| enc_t(t)).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}
pub(crate) fn dec_named_triple<K, D, T>(
    s: &str,
    dec_k: impl Fn(&str) -> Result<K, String>,
    dec_d: impl Fn(&str) -> Result<D, String>,
    dec_t: impl Fn(&str) -> Result<T, String>,
) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_k(e)).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| {
        let (k, rest) = entry.split_once(':').ok_or_else(|| format!("named triple modified: bad entry {entry:?}"))?;
        Ok(NamedModified { key: dec_k(k)?, diff: dec_d(rest)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|e| dec_t(e)).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}
//#endregion 🔖️NamedTripleCodec

//#region 🔖️ValueCodecs
pub(crate) fn enc_point3(p: &BcfPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
pub(crate) fn dec_point3(s: &str) -> Result<BcfPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(BcfPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

/// 📷 `P[view_point,direction,up_vector,field_of_view]` (Perspective) / `O[...,view_to_world_scale]`
/// (Orthogonal) -- single-letter tag prefix, the `xs:choice` made concrete (same convention as
/// `enc_xml_node`'s `E`/`T`/`D`/`M`/`P` tags in svg's hand-rolled codec).
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

pub(crate) fn enc_visibility(v: &BcfVisibility) -> String {
    format!("[{},{}]", if v.default_visibility { "1" } else { "0" }, enc_list(&v.exceptions, |s: &String| enc_str(s)))
}
pub(crate) fn dec_visibility(s: &str) -> Result<BcfVisibility, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [default_visibility, exceptions] = parts.as_slice() else { return Err(format!("visibility: expected 2 fields, got {}", parts.len())) };
    Ok(BcfVisibility { default_visibility: *default_visibility == "1", exceptions: dec_list(exceptions, dec_str)? })
}

pub(crate) fn enc_coloring(c: &BcfColoring) -> String {
    format!("[{},{}]", enc_str(&c.color), enc_list(&c.components, |s: &String| enc_str(s)))
}
pub(crate) fn dec_coloring(s: &str) -> Result<BcfColoring, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [color, components] = parts.as_slice() else { return Err(format!("coloring: expected 2 fields, got {}", parts.len())) };
    Ok(BcfColoring { color: dec_str(color)?, components: dec_list(components, dec_str)? })
}

pub(crate) fn enc_components(c: &BcfComponents) -> String {
    format!("[{},{},{}]", enc_list(&c.selection, |s: &String| enc_str(s)), enc_visibility(&c.visibility), enc_list(&c.coloring, enc_coloring))
}
pub(crate) fn dec_components(s: &str) -> Result<BcfComponents, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [selection, visibility, coloring] = parts.as_slice() else { return Err(format!("components: expected 3 fields, got {}", parts.len())) };
    Ok(BcfComponents { selection: dec_list(selection, dec_str)?, visibility: dec_visibility(visibility)?, coloring: dec_list(coloring, dec_coloring)? })
}

pub(crate) fn enc_comment(c: &BcfComment) -> String {
    format!(
        "[{},{},{},{},{}]",
        enc_str(&c.guid), enc_str(&c.date), enc_str(&c.author), enc_str(&c.text),
        encode_option(&c.viewpoint_ref, |v: &String| enc_str(v)),
    )
}
pub(crate) fn dec_comment(s: &str) -> Result<BcfComment, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, date, author, text, viewpoint_ref] = parts.as_slice() else { return Err(format!("comment: expected 5 fields, got {}", parts.len())) };
    Ok(BcfComment { guid: dec_str(guid)?, date: dec_str(date)?, author: dec_str(author)?, text: dec_str(text)?, viewpoint_ref: decode_option(viewpoint_ref, dec_str)? })
}

pub(crate) fn enc_viewpoint(v: &BcfViewpoint) -> String {
    format!(
        "[{},{},{},{}]",
        enc_str(&v.guid),
        encode_option(&v.camera, enc_camera),
        encode_option(&v.components, enc_components),
        encode_option(&v.snapshot, |b: &Vec<u8>| enc_bytes(b)),
    )
}
pub(crate) fn dec_viewpoint(s: &str) -> Result<BcfViewpoint, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, camera, components, snapshot] = parts.as_slice() else { return Err(format!("viewpoint: expected 4 fields, got {}", parts.len())) };
    Ok(BcfViewpoint { guid: dec_str(guid)?, camera: decode_option(camera, dec_camera)?, components: decode_option(components, dec_components)?, snapshot: decode_option(snapshot, dec_bytes)? })
}

pub(crate) fn enc_topic(t: &BcfTopic) -> String {
    format!(
        "[{},{},{},{},{},{},{},{},{},{}]",
        enc_str(&t.guid), enc_str(&t.title), enc_str(&t.description), enc_str(&t.status), enc_str(&t.priority),
        enc_list(&t.labels, |s: &String| enc_str(s)), enc_str(&t.creation_date), enc_str(&t.creation_author),
        enc_list(&t.comments, enc_comment), enc_list(&t.viewpoints, enc_viewpoint),
    )
}
pub(crate) fn dec_topic(s: &str) -> Result<BcfTopic, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [guid, title, description, status, priority, labels, creation_date, creation_author, comments, viewpoints] = parts.as_slice() else {
        return Err(format!("topic: expected 10 fields, got {}", parts.len()));
    };
    Ok(BcfTopic {
        guid: dec_str(guid)?, title: dec_str(title)?, description: dec_str(description)?, status: dec_str(status)?, priority: dec_str(priority)?,
        labels: dec_list(labels, dec_str)?, creation_date: dec_str(creation_date)?, creation_author: dec_str(creation_author)?,
        comments: dec_list(comments, dec_comment)?, viewpoints: dec_list(viewpoints, dec_viewpoint)?,
    })
}

pub(crate) fn enc_part(p: &BcfRawPart) -> String {
    format!("[{},{}]", enc_str(&p.name), enc_bytes(&p.data))
}
pub(crate) fn dec_part(s: &str) -> Result<BcfRawPart, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, data] = parts.as_slice() else { return Err(format!("part: expected 2 fields, got {}", parts.len())) };
    Ok(BcfRawPart { name: dec_str(name)?, data: dec_bytes(data)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
pub(crate) fn enc_comment_diff(d: &BcfCommentDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.date, |v: &String| enc_str(v)),
        encode_option(&d.author, |v: &String| enc_str(v)),
        encode_option(&d.text, |v: &String| enc_str(v)),
        encode_option(&d.viewpoint_ref, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
    )
}
pub(crate) fn dec_comment_diff(s: &str) -> Result<BcfCommentDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [date, author, text, viewpoint_ref] = parts.as_slice() else { return Err(format!("comment diff: expected 4 fields, got {}", parts.len())) };
    Ok(BcfCommentDiff {
        date: decode_option(date, dec_str)?,
        author: decode_option(author, dec_str)?,
        text: decode_option(text, dec_str)?,
        viewpoint_ref: decode_option(viewpoint_ref, |s| decode_option(s, dec_str))?,
    })
}

pub(crate) fn enc_viewpoint_diff(d: &BcfViewpointDiff) -> String {
    format!(
        "[{},{},{}]",
        encode_option(&d.camera, |inner: &Option<BcfCamera>| encode_option(inner, enc_camera)),
        encode_option(&d.components, |inner: &Option<BcfComponents>| encode_option(inner, enc_components)),
        encode_option(&d.snapshot, |inner: &Option<Vec<u8>>| encode_option(inner, |b: &Vec<u8>| enc_bytes(b))),
    )
}
pub(crate) fn dec_viewpoint_diff(s: &str) -> Result<BcfViewpointDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [camera, components, snapshot] = parts.as_slice() else { return Err(format!("viewpoint diff: expected 3 fields, got {}", parts.len())) };
    Ok(BcfViewpointDiff {
        camera: decode_option(camera, |s| decode_option(s, dec_camera))?,
        components: decode_option(components, |s| decode_option(s, dec_components))?,
        snapshot: decode_option(snapshot, |s| decode_option(s, dec_bytes))?,
    })
}

pub(crate) fn enc_part_diff(d: &BcfPartDiff) -> String {
    format!("[{}]", encode_option(&d.data, |b: &Vec<u8>| enc_bytes(b)))
}
pub(crate) fn dec_part_diff(s: &str) -> Result<BcfPartDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BcfPartDiff { data: decode_option(inner, dec_bytes)? })
}

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

//#region 🔖️TopLevel
fn print_bcf_diff(d: &BcfDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.version { tokens.push(format!("version={}", enc_str(v))); }
    if let Some(t) = &d.topics { tokens.push(format!("topics={}", enc_named_triple(t, |k: &String| enc_str(k), enc_topic_diff, enc_topic))); }
    if let Some(p) = &d.parts { tokens.push(format!("parts={}", enc_named_triple(p, |k: &String| enc_str(k), enc_part_diff, enc_part))); }
    tokens.join(" ")
}
fn parse_bcf_diff(line: &str) -> Result<BcfDiff, String> {
    let mut d = BcfDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("version=") { d.version = Some(dec_str(rest)?); }
        else if let Some(rest) = token.strip_prefix("topics=") { d.topics = Some(dec_named_triple(rest, dec_str, dec_topic_diff, dec_topic)?); }
        else if let Some(rest) = token.strip_prefix("parts=") { d.parts = Some(dec_named_triple(rest, dec_str, dec_part_diff, dec_part)?); }
        else { return Err(format!("bcf diff: unknown token {token:?}")); }
    }
    Ok(d)
}

impl protocol::DiffCodec for BcfDiff {
    fn print_diff(&self) -> String {
        print_bcf_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_bcf_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Binary = the text bytes verbatim, same simplification `GifDiff`/`SvgDiff`/`WriterDiff`
    /// all use — satisfies every `DiffCodec` law without inventing a second wire format.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_diff(line).map_err(|e| protocol::ProtocolError::Malformed { what: "diff text", offset: 0, detail: e.to_string() })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec
