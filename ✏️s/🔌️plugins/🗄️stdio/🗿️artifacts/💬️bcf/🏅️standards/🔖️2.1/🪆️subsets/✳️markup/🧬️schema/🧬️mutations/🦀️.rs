//! 🧬️ BcfMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) via the diff module's `wrap_*_diff` helpers; every variant's `inverse()`
//! looks up prior state from `base` and constructs the exact undoing mutation (guid-aware,
//! matching svg/docx precedent). `SetVersion`/`SetViewpointSnapshot` extend the brief's literal
//! mutation list (`SetSnapshot, InsertTopic/RemoveTopic/SetTopicMarkup,
//! InsertComment/RemoveComment/SetComment, InsertViewpoint/RemoveViewpoint/SetViewpointCamera/
//! SetViewpointComponents`) — `version` and a viewpoint's `snapshot` bytes are real independently
//! mutable snapshot fields the target completeness table lists, so a complete mutation API needs
//! a setter for each (see report deviations).
//!
//! 🧬️ Mutation-leaf migration (ticket 26/08/29/S-END-TO-END): `NoMutation` is dropped —
//! `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one leaf payload, and a unit
//! variant wraps none. Every former `None => vec![BcfMutation::NoMutation]` inverse fallback below
//! is now `None => Vec::new()` (no inverse steps needed for a mutation that never found its
//! target), mirroring `tiff`'s own `RemoveTileTags`/`RemoveStripOffsets` "_ => return Vec::new()"
//! precedent (`../../../../🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations/🦀️.rs`).

use crate::artifacts::bcf::schema::diff::{
    dec_bcf_snapshot_bin, dec_camera_bin, dec_comment_bin, dec_components_bin, dec_topic_bin, dec_viewpoint_bin, enc_bcf_snapshot_bin, enc_camera_bin, enc_comment_bin, enc_components_bin, enc_topic_bin, enc_viewpoint_bin, read_bytes_lp, read_str_lp,
    write_bytes_lp, write_str_lp,
};
use crate::artifacts::bcf::schema::diff::{
    dec_bytes, dec_camera, dec_comment, dec_components, dec_list, dec_part, dec_str, dec_topic, dec_viewpoint, decode_option, enc_bytes, enc_camera, enc_comment, enc_components, enc_list, enc_part, enc_str, enc_topic, enc_viewpoint, encode_option,
    split_top_level, strip_brackets,
};
use crate::artifacts::bcf::schema::diff::{diff_set_snapshot, wrap_comment_diff, wrap_topic_diff, wrap_viewpoint_diff, BcfCommentDiff, BcfCommentsDiff, BcfDiff, BcfTopicDiff, BcfTopicsDiff, BcfViewpointDiff, BcfViewpointsDiff};
use crate::artifacts::bcf::schema::snapshot::{BcfCamera, BcfComment, BcfComponents, BcfTopic, BcfViewpoint};
use crate::artifacts::bcf::BcfSnapshot;
use protocol::Mutation;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.bcf`.
/// 🧪️ F6 CONFIRMED (real `cargo check` error, `dsl::DslOps` attempted and reverted): fails for
/// the mutation-side twin of the diff's blockers — `SetSnapshot{snapshot: BcfSnapshot}`
/// recursively contains `BcfCamera` (`error[E0277]: the trait bound v2_1::...::BcfCamera:
/// DslField is not satisfied`) via `topics -> viewpoints -> camera`, `InsertTopic`/
/// `InsertComment`/`InsertViewpoint` each carry a whole `BcfTopic`/`BcfComment`/`BcfViewpoint`
/// (none of which derive `DslField` — none are `DslRecord`-derived), and
/// `SetViewpointCamera{camera: Option<BcfCamera>}` carries the enum DIRECTLY as a variant field
/// (`error[E0277]` at this exact line, not just via the nested snapshot) — the mutation-side
/// mirror of `SvgMutation`'s `InsertElement{node: XmlNode}` finding (`f6-recon-report.md` §3a).
/// `SetComment{viewpoint_ref: Option<Option<String>>}` also independently fails the tri-state
/// check (§3b). `OpText`/`OpBinary` hand-rolled below, reusing the diff module's `pub(crate)`
/// grammar primitives.
//#region 🔖️Leaves
#[path = "🟤️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔢set-version/🦀️.rs"]
pub mod set_version;
#[path = "📌insert-topic/🦀️.rs"]
pub mod insert_topic;
#[path = "🗑remove-topic/🦀️.rs"]
pub mod remove_topic;
#[path = "🖊set-topic-markup/🦀️.rs"]
pub mod set_topic_markup;
#[path = "🗨insert-comment/🦀️.rs"]
pub mod insert_comment;
#[path = "🧹remove-comment/🦀️.rs"]
pub mod remove_comment;
#[path = "✏set-comment/🦀️.rs"]
pub mod set_comment;
#[path = "👁insert-viewpoint/🦀️.rs"]
pub mod insert_viewpoint;
#[path = "🙈remove-viewpoint/🦀️.rs"]
pub mod remove_viewpoint;
#[path = "📷set-viewpoint-camera/🦀️.rs"]
pub mod set_viewpoint_camera;
#[path = "🧱set-viewpoint-components/🦀️.rs"]
pub mod set_viewpoint_components;
#[path = "📸set-viewpoint-snapshot/🦀️.rs"]
pub mod set_viewpoint_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for `stdio.bcf`. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = BcfSnapshot, diff = BcfDiff, schema = "BcfMutation")]
pub enum BcfMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetVersion(set_version::SetVersion),
    InsertTopic(insert_topic::InsertTopic),
    RemoveTopic(remove_topic::RemoveTopic),
    SetTopicMarkup(set_topic_markup::SetTopicMarkup),
    InsertComment(insert_comment::InsertComment),
    RemoveComment(remove_comment::RemoveComment),
    SetComment(set_comment::SetComment),
    InsertViewpoint(insert_viewpoint::InsertViewpoint),
    RemoveViewpoint(remove_viewpoint::RemoveViewpoint),
    SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera),
    SetViewpointComponents(set_viewpoint_components::SetViewpointComponents),
    SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot),
}

/// 📇️ Kebab-case spelling of every `BcfMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself). Mirrors `print_bcf_mutation`'s own keyword match
/// entry-for-entry, so `KINDS[i]` is exactly what `print_op()` emits for the enum's `i`-th variant.
pub const KINDS: &[&str] = &[
    "set-snapshot",
    "set-version",
    "insert-topic",
    "remove-topic",
    "set-topic-markup",
    "insert-comment",
    "remove-comment",
    "set-comment",
    "insert-viewpoint",
    "remove-viewpoint",
    "set-viewpoint-camera",
    "set-viewpoint-components",
    "set-viewpoint-snapshot",
];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Single semantics source: the returned diff IS what gets
/// applied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_bcf_mutation(snapshot: &mut BcfSnapshot, mutation: &BcfMutation) -> protocol::MutationOutcome<BcfDiff> {
    let outcome = <BcfMutation as Mutation<BcfSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &BcfMutation, base: &BcfSnapshot) -> protocol::MutationOutcome<BcfDiff> {
    protocol::MutationOutcome::new(match this {
        BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        BcfMutation::SetVersion(set_version::SetVersion { version }) => BcfDiff { version: Some(version.clone()), topics: None, parts: None },
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic }) => BcfDiff { version: None, topics: Some(BcfTopicsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![topic.clone()] }), parts: None },
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid }) => BcfDiff { version: None, topics: Some(BcfTopicsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }), parts: None },
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }) => wrap_topic_diff(
            guid,
            BcfTopicDiff {
                title: title.clone(),
                description: description.clone(),
                status: status.clone(),
                priority: priority.clone(),
                labels: labels.clone(),
                creation_date: creation_date.clone(),
                creation_author: creation_author.clone(),
                comments: None,
                viewpoints: None,
            },
        ),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment }) => wrap_topic_diff(topic_guid, BcfTopicDiff { comments: Some(BcfCommentsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![comment.clone()] }), ..Default::default() }),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }) => wrap_topic_diff(topic_guid, BcfTopicDiff { comments: Some(BcfCommentsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }), ..Default::default() }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }) => {
            wrap_comment_diff(topic_guid, guid, BcfCommentDiff { date: date.clone(), author: author.clone(), text: text.clone(), viewpoint_ref: viewpoint_ref.clone() })
        }
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint }) => {
            wrap_topic_diff(topic_guid, BcfTopicDiff { viewpoints: Some(BcfViewpointsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![viewpoint.clone()] }), ..Default::default() })
        }
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }) => wrap_topic_diff(topic_guid, BcfTopicDiff { viewpoints: Some(BcfViewpointsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }), ..Default::default() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }) => wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: Some(camera.clone()), components: None, snapshot: None }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }) => wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: None, components: Some(components.clone()), snapshot: None }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }) => wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: None, components: None, snapshot: Some(snapshot.clone()) }),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &BcfMutation, base: &BcfSnapshot) -> Vec<BcfMutation> {
    match this {
        BcfMutation::SetSnapshot(_) => vec![BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        BcfMutation::SetVersion(_) => vec![BcfMutation::SetVersion(set_version::SetVersion { version: base.version.clone() })],
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic }) => vec![BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: topic.guid.clone() })],
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid }) => match find_topic(base, guid) {
            Some(t) => vec![BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: t.clone() })],
            None => Vec::new(),
        },
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }) => match find_topic(base, guid) {
            Some(t) => vec![BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
                guid: guid.clone(),
                title: title.as_ref().map(|_| t.title.clone()),
                description: description.as_ref().map(|_| t.description.clone()),
                status: status.as_ref().map(|_| t.status.clone()),
                priority: priority.as_ref().map(|_| t.priority.clone()),
                labels: labels.as_ref().map(|_| t.labels.clone()),
                creation_date: creation_date.as_ref().map(|_| t.creation_date.clone()),
                creation_author: creation_author.as_ref().map(|_| t.creation_author.clone()),
            })],
            None => Vec::new(),
        },
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment }) => {
            vec![BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: topic_guid.clone(), guid: comment.guid.clone() })]
        }
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }) => match find_comment(base, topic_guid, guid) {
            Some(c) => vec![BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: topic_guid.clone(), comment: c.clone() })],
            None => Vec::new(),
        },
        BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }) => match find_comment(base, topic_guid, guid) {
            Some(c) => vec![BcfMutation::SetComment(set_comment::SetComment {
                topic_guid: topic_guid.clone(),
                guid: guid.clone(),
                date: date.as_ref().map(|_| c.date.clone()),
                author: author.as_ref().map(|_| c.author.clone()),
                text: text.as_ref().map(|_| c.text.clone()),
                viewpoint_ref: viewpoint_ref.as_ref().map(|_| c.viewpoint_ref.clone()),
            })],
            None => Vec::new(),
        },
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint }) => {
            vec![BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: topic_guid.clone(), guid: viewpoint.guid.clone() })]
        }
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }) => match find_viewpoint(base, topic_guid, guid) {
            Some(v) => vec![BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: topic_guid.clone(), viewpoint: v.clone() })],
            None => Vec::new(),
        },
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, .. }) => match find_viewpoint(base, topic_guid, guid) {
            Some(v) => vec![BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: topic_guid.clone(), guid: guid.clone(), camera: v.camera.clone() })],
            None => Vec::new(),
        },
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, .. }) => match find_viewpoint(base, topic_guid, guid) {
            Some(v) => vec![BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: topic_guid.clone(), guid: guid.clone(), components: v.components.clone() })],
            None => Vec::new(),
        },
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, .. }) => match find_viewpoint(base, topic_guid, guid) {
            Some(v) => vec![BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: topic_guid.clone(), guid: guid.clone(), snapshot: v.snapshot.clone() })],
            None => Vec::new(),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_topic<'a>(base: &'a BcfSnapshot, guid: &str) -> Option<&'a BcfTopic> {
    base.topics.iter().find(|t| t.guid == guid)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_comment<'a>(base: &'a BcfSnapshot, topic_guid: &str, guid: &str) -> Option<&'a BcfComment> {
    find_topic(base, topic_guid)?.comments.iter().find(|c| c.guid == guid)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_viewpoint<'a>(base: &'a BcfSnapshot, topic_guid: &str, guid: &str) -> Option<&'a BcfViewpoint> {
    find_topic(base, topic_guid)?.viewpoints.iter().find(|v| v.guid == guid)
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `BcfMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above via a real `cargo check` error) — reuses the diff module's `pub(crate)` grammar
/// primitives (`enc_str`/`enc_camera`/`enc_topic`/`encode_option`/...) rather than duplicating them
/// a second time in this file, same pattern `SvgMutation` established. Grammar: `keyword arg=value
/// ...` (space-separated), one match arm per variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_bcf_snapshot(s: &BcfSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_str(&s.version), enc_list(&s.topics, enc_topic), enc_list(&s.parts, enc_part))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_bcf_snapshot(s: &str) -> Result<BcfSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, version, topics, parts_field] = parts.as_slice() else { return Err(format!("bcf snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(BcfSnapshot { schema: dec_str(schema)?, version: dec_str(version)?, topics: dec_list(topics, dec_topic)?, parts: dec_list(parts_field, dec_part)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_bcf_mutation(m: &BcfMutation) -> String {
    match m {
        BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_bcf_snapshot(snapshot)),
        BcfMutation::SetVersion(set_version::SetVersion { version }) => format!("set-version version={}", enc_str(version)),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic }) => format!("insert-topic topic={}", enc_topic(topic)),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid }) => format!("remove-topic guid={}", enc_str(guid)),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }) => format!(
            "set-topic-markup guid={} title={} description={} status={} priority={} labels={} creation-date={} creation-author={}",
            enc_str(guid),
            encode_option(title, |v: &String| enc_str(v)),
            encode_option(description, |v: &String| enc_str(v)),
            encode_option(status, |v: &String| enc_str(v)),
            encode_option(priority, |v: &String| enc_str(v)),
            encode_option(labels, |v: &Vec<String>| enc_list(v, |s| enc_str(s))),
            encode_option(creation_date, |v: &String| enc_str(v)),
            encode_option(creation_author, |v: &String| enc_str(v)),
        ),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment }) => format!("insert-comment topic-guid={} comment={}", enc_str(topic_guid), enc_comment(comment)),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }) => format!("remove-comment topic-guid={} guid={}", enc_str(topic_guid), enc_str(guid)),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }) => format!(
            "set-comment topic-guid={} guid={} date={} author={} text={} viewpoint-ref={}",
            enc_str(topic_guid),
            enc_str(guid),
            encode_option(date, |v: &String| enc_str(v)),
            encode_option(author, |v: &String| enc_str(v)),
            encode_option(text, |v: &String| enc_str(v)),
            encode_option(viewpoint_ref, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
        ),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint }) => format!("insert-viewpoint topic-guid={} viewpoint={}", enc_str(topic_guid), enc_viewpoint(viewpoint)),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }) => format!("remove-viewpoint topic-guid={} guid={}", enc_str(topic_guid), enc_str(guid)),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }) => format!("set-viewpoint-camera topic-guid={} guid={} camera={}", enc_str(topic_guid), enc_str(guid), encode_option(camera, enc_camera)),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }) => {
            format!("set-viewpoint-components topic-guid={} guid={} components={}", enc_str(topic_guid), enc_str(guid), encode_option(components, enc_components))
        }
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }) => {
            format!("set-viewpoint-snapshot topic-guid={} guid={} snapshot={}", enc_str(topic_guid), enc_str(guid), encode_option(snapshot, |b: &Vec<u8>| enc_bytes(b)))
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_bcf_mutation(line: &str) -> Result<BcfMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("bcf mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("bcf mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_bcf_snapshot(arg("snapshot")?)? })),
        "set-version" => Ok(BcfMutation::SetVersion(set_version::SetVersion { version: dec_str(arg("version")?)? })),
        "insert-topic" => Ok(BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: dec_topic(arg("topic")?)? })),
        "remove-topic" => Ok(BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: dec_str(arg("guid")?)? })),
        "set-topic-markup" => Ok(BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
            guid: dec_str(arg("guid")?)?,
            title: decode_option(arg("title")?, dec_str)?,
            description: decode_option(arg("description")?, dec_str)?,
            status: decode_option(arg("status")?, dec_str)?,
            priority: decode_option(arg("priority")?, dec_str)?,
            labels: decode_option(arg("labels")?, |s| dec_list(s, dec_str))?,
            creation_date: decode_option(arg("creation-date")?, dec_str)?,
            creation_author: decode_option(arg("creation-author")?, dec_str)?,
        })),
        "insert-comment" => Ok(BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: dec_str(arg("topic-guid")?)?, comment: dec_comment(arg("comment")?)? })),
        "remove-comment" => Ok(BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)? })),
        "set-comment" => Ok(BcfMutation::SetComment(set_comment::SetComment {
            topic_guid: dec_str(arg("topic-guid")?)?,
            guid: dec_str(arg("guid")?)?,
            date: decode_option(arg("date")?, dec_str)?,
            author: decode_option(arg("author")?, dec_str)?,
            text: decode_option(arg("text")?, dec_str)?,
            viewpoint_ref: decode_option(arg("viewpoint-ref")?, |s| decode_option(s, dec_str))?,
        })),
        "insert-viewpoint" => Ok(BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: dec_str(arg("topic-guid")?)?, viewpoint: dec_viewpoint(arg("viewpoint")?)? })),
        "remove-viewpoint" => Ok(BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)? })),
        "set-viewpoint-camera" => Ok(BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, camera: decode_option(arg("camera")?, dec_camera)? })),
        "set-viewpoint-components" => Ok(BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, components: decode_option(arg("components")?, dec_components)? })),
        "set-viewpoint-snapshot" => Ok(BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, snapshot: decode_option(arg("snapshot")?, dec_bytes)? })),
        other => Err(format!("bcf mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for BcfMutation {
    fn print_op(&self) -> String {
        print_bcf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_bcf_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ FG-wave: real recursive BINARY primitives for `BcfMutation`'s own variant-specific fields
/// (`Option<String>`/`Option<Option<String>>` tri-states, `Option<BcfCamera>`/
/// `Option<BcfComponents>`/`Option<Vec<u8>>`) -- everything ELSE (whole `BcfSnapshot`/`BcfTopic`/
/// `BcfComment`/`BcfViewpoint`/`BcfCamera`/`BcfComponents`) reuses `../🔺️diff/🦀️.rs`'s
/// own `pub(crate)` binary primitives directly (imported above), same intra-artifact reuse pattern
/// this file's text-form `OpText` impl already established for the string-grammar codecs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_opt_str_bin(out: &mut Vec<u8>, opt: &Option<String>) {
    out.push(if opt.is_some() { 1 } else { 0 });
    if let Some(v) = opt {
        write_str_lp(out, v);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_opt_str_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    Ok(if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_str_list_bin(out: &mut Vec<u8>, items: &[String]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for s in items {
        write_str_lp(out, s);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_str_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<String>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(read_str_lp(reader)?);
    }
    Ok(out)
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `BcfMutation` variant ordinal; tag 0 (formerly `NoMutation`) is retired rather than reused, so a
/// stray zero tag on the wire fails `decode_op` instead of silently resurrecting a dropped variant.
impl protocol::OpBinary for BcfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            BcfMutation::SetSnapshot(_) => 1,
            BcfMutation::SetVersion(_) => 2,
            BcfMutation::InsertTopic(_) => 3,
            BcfMutation::RemoveTopic(_) => 4,
            BcfMutation::SetTopicMarkup(_) => 5,
            BcfMutation::InsertComment(_) => 6,
            BcfMutation::RemoveComment(_) => 7,
            BcfMutation::SetComment(_) => 8,
            BcfMutation::InsertViewpoint(_) => 9,
            BcfMutation::RemoveViewpoint(_) => 10,
            BcfMutation::SetViewpointCamera(_) => 11,
            BcfMutation::SetViewpointComponents(_) => 12,
            BcfMutation::SetViewpointSnapshot(_) => 13,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_bcf_snapshot_bin(snapshot, &mut out),
            BcfMutation::SetVersion(set_version::SetVersion { version }) => write_str_lp(&mut out, version),
            BcfMutation::InsertTopic(insert_topic::InsertTopic { topic }) => enc_topic_bin(topic, &mut out),
            BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid }) => write_str_lp(&mut out, guid),
            BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }) => {
                write_str_lp(&mut out, guid);
                write_opt_str_bin(&mut out, title);
                write_opt_str_bin(&mut out, description);
                write_opt_str_bin(&mut out, status);
                write_opt_str_bin(&mut out, priority);
                out.push(if labels.is_some() { 1 } else { 0 });
                if let Some(v) = labels {
                    write_str_list_bin(&mut out, v);
                }
                write_opt_str_bin(&mut out, creation_date);
                write_opt_str_bin(&mut out, creation_author);
            }
            BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment }) => {
                write_str_lp(&mut out, topic_guid);
                enc_comment_bin(comment, &mut out);
            }
            BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
            }
            BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                write_opt_str_bin(&mut out, date);
                write_opt_str_bin(&mut out, author);
                write_opt_str_bin(&mut out, text);
                out.push(if viewpoint_ref.is_some() { 1 } else { 0 });
                if let Some(inner) = viewpoint_ref {
                    write_opt_str_bin(&mut out, inner);
                }
            }
            BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint }) => {
                write_str_lp(&mut out, topic_guid);
                enc_viewpoint_bin(viewpoint, &mut out);
            }
            BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
            }
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                out.push(if camera.is_some() { 1 } else { 0 });
                if let Some(c) = camera {
                    enc_camera_bin(c, &mut out);
                }
            }
            BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                out.push(if components.is_some() { 1 } else { 0 });
                if let Some(c) = components {
                    enc_components_bin(c, &mut out);
                }
            }
            BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }) => {
                write_str_lp(&mut out, topic_guid);
                write_str_lp(&mut out, guid);
                out.push(if snapshot.is_some() { 1 } else { 0 });
                if let Some(b) = snapshot {
                    write_bytes_lp(&mut out, b);
                }
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            1 => Ok(BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_bcf_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))? })),
            2 => Ok(BcfMutation::SetVersion(set_version::SetVersion { version: read_str_lp(&mut reader).map_err(|e| malformed("op version", reader.position(), e))? })),
            3 => Ok(BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: dec_topic_bin(&mut reader).map_err(|e| malformed("op topic", reader.position(), e))? })),
            4 => Ok(BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))? })),
            5 => {
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let title = read_opt_str_bin(&mut reader).map_err(|e| malformed("op title", reader.position(), e))?;
                let description = read_opt_str_bin(&mut reader).map_err(|e| malformed("op description", reader.position(), e))?;
                let status = read_opt_str_bin(&mut reader).map_err(|e| malformed("op status", reader.position(), e))?;
                let priority = read_opt_str_bin(&mut reader).map_err(|e| malformed("op priority", reader.position(), e))?;
                let labels = if reader.read_u8().map_err(|e| malformed("op labels presence", reader.position(), e.to_string()))? != 0 { Some(read_str_list_bin(&mut reader).map_err(|e| malformed("op labels", reader.position(), e))?) } else { None };
                let creation_date = read_opt_str_bin(&mut reader).map_err(|e| malformed("op creation_date", reader.position(), e))?;
                let creation_author = read_opt_str_bin(&mut reader).map_err(|e| malformed("op creation_author", reader.position(), e))?;
                Ok(BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author }))
            }
            6 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let comment = dec_comment_bin(&mut reader).map_err(|e| malformed("op comment", reader.position(), e))?;
                Ok(BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment }))
            }
            7 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                Ok(BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }))
            }
            8 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let date = read_opt_str_bin(&mut reader).map_err(|e| malformed("op date", reader.position(), e))?;
                let author = read_opt_str_bin(&mut reader).map_err(|e| malformed("op author", reader.position(), e))?;
                let text = read_opt_str_bin(&mut reader).map_err(|e| malformed("op text", reader.position(), e))?;
                let viewpoint_ref = if reader.read_u8().map_err(|e| malformed("op viewpoint_ref presence", reader.position(), e.to_string()))? != 0 {
                    Some(read_opt_str_bin(&mut reader).map_err(|e| malformed("op viewpoint_ref", reader.position(), e))?)
                } else {
                    None
                };
                Ok(BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }))
            }
            9 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let viewpoint = dec_viewpoint_bin(&mut reader).map_err(|e| malformed("op viewpoint", reader.position(), e))?;
                Ok(BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint }))
            }
            10 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                Ok(BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }))
            }
            11 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let camera = if reader.read_u8().map_err(|e| malformed("op camera presence", reader.position(), e.to_string()))? != 0 { Some(dec_camera_bin(&mut reader).map_err(|e| malformed("op camera", reader.position(), e))?) } else { None };
                Ok(BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }))
            }
            12 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let components =
                    if reader.read_u8().map_err(|e| malformed("op components presence", reader.position(), e.to_string()))? != 0 { Some(dec_components_bin(&mut reader).map_err(|e| malformed("op components", reader.position(), e))?) } else { None };
                Ok(BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }))
            }
            13 => {
                let topic_guid = read_str_lp(&mut reader).map_err(|e| malformed("op topic_guid", reader.position(), e))?;
                let guid = read_str_lp(&mut reader).map_err(|e| malformed("op guid", reader.position(), e))?;
                let snapshot = if reader.read_u8().map_err(|e| malformed("op snapshot presence", reader.position(), e.to_string()))? != 0 { Some(read_bytes_lp(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?) } else { None };
                Ok(BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }))
            }
            other => Err(malformed("op tag", 1, format!("unknown BcfMutation tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `BcfMutation` values -- one per variant -- the single source of
/// truth reused by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📜️docx/…/🧬️mutations/🦀️.rs`'s own
/// `demo_mutation_cases()` establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<BcfMutation> {
    use crate::artifacts::bcf::schema::diff::{demo_snapshot_a, demo_snapshot_b};
    let base = demo_snapshot_a();
    let snapshot = demo_snapshot_b();
    vec![
        BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }),
        BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
        BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: base.topics[0].clone() }),
        BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "keep".into() }),
        BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup {
            guid: "keep".into(),
            title: Some("Renamed".into()),
            description: None,
            status: Some("Closed".into()),
            priority: None,
            labels: Some(vec!["Renamed".into(), "Second".into()]),
            creation_date: None,
            creation_author: None,
        }),
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "keep".into(), comment: base.topics[0].comments[0].clone() }),
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "keep".into(), guid: "c-keep".into() }),
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "keep".into(), guid: "c-keep".into(), date: None, author: None, text: Some("Updated".into()), viewpoint_ref: Some(None) }),
        BcfMutation::SetComment(set_comment::SetComment {
            topic_guid: "keep".into(),
            guid: "c-keep".into(),
            date: Some("2025-01-01T00:00:00+00:00".into()),
            author: Some("a@example.com".into()),
            text: None,
            viewpoint_ref: Some(Some("vp2".into())),
        }),
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "keep".into(), viewpoint: base.topics[0].viewpoints[0].clone() }),
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "keep".into(), guid: "vp-keep".into() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "keep".into(), guid: "vp-keep".into(), camera: base.topics[0].viewpoints[0].camera.clone() }),
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "keep".into(), guid: "vp-keep".into(), camera: None }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "keep".into(), guid: "vp-keep".into(), components: base.topics[0].viewpoints[0].components.clone() }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "keep".into(), guid: "vp-keep".into(), components: None }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "keep".into(), guid: "vp-keep".into(), snapshot: Some(vec![1, 2, 3]) }),
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "keep".into(), guid: "vp-keep".into(), snapshot: None }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️KindsLaw
/// 🧪️ Keeps `KINDS` honest against the enum it claims to spell: every variant's
/// `print_bcf_mutation` keyword, in the SAME declaration order `OpBinary`'s own tag match uses,
/// must equal `KINDS` entry-for-entry -- the framework never parses Rust to check this itself (see
/// `KINDS`'s own doc comment), so this test is the one thing that does. `KINDS` is also kept
/// textually identical, by hand, to `../../🔣️oracle.json`'s own `kinds` array.
#[cfg(test)]
mod kinds_tests {
    use super::*;

    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let one_per_variant = vec![
            BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: BcfSnapshot::default() }),
            BcfMutation::SetVersion(set_version::SetVersion { version: "2.2".into() }),
            BcfMutation::InsertTopic(insert_topic::InsertTopic { topic: BcfTopic::default() }),
            BcfMutation::RemoveTopic(remove_topic::RemoveTopic { guid: "t".into() }),
            BcfMutation::SetTopicMarkup(set_topic_markup::SetTopicMarkup { guid: "t".into(), title: None, description: None, status: None, priority: None, labels: None, creation_date: None, creation_author: None }),
            BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid: "t".into(), comment: BcfComment::default() }),
            BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid: "t".into(), guid: "c".into() }),
            BcfMutation::SetComment(set_comment::SetComment { topic_guid: "t".into(), guid: "c".into(), date: None, author: None, text: None, viewpoint_ref: None }),
            BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid: "t".into(), viewpoint: BcfViewpoint::default() }),
            BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid: "t".into(), guid: "v".into() }),
            BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid: "t".into(), guid: "v".into(), camera: None }),
            BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid: "t".into(), guid: "v".into(), components: None }),
            BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid: "t".into(), guid: "v".into(), snapshot: None }),
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            let printed = print_bcf_mutation(mutation);
            let keyword = printed.split(' ').next().unwrap_or(&printed);
            assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
        }
    }
}
//#endregion 🧪️KindsLaw

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🟤️set-snapshot/🧪️tests/🛟️closes-the-clash-topic-and-answers-its-comment/🦀️.rs"]
    mod tests_set_snapshot_closes_the_clash_topic_and_answers_its_comment;
}
//#endregion 🧪️FixtureTests
