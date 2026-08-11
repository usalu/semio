//! 🧬️ BcfMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) via the diff module's `wrap_*_diff` helpers; every variant's `inverse()`
//! looks up prior state from `base` and constructs the exact undoing mutation (guid-aware,
//! matching svg/docx precedent). `SetVersion`/`SetViewpointSnapshot` extend the brief's literal
//! mutation list (`SetSnapshot, InsertTopic/RemoveTopic/SetTopicMarkup,
//! InsertComment/RemoveComment/SetComment, InsertViewpoint/RemoveViewpoint/SetViewpointCamera/
//! SetViewpointComponents`) — `version` and a viewpoint's `snapshot` bytes are real independently
//! mutable snapshot fields the target completeness table lists, so a complete mutation API needs
//! a setter for each (see report deviations).

use crate::artifacts::bcf::schema::diff::{
    diff_set_snapshot, wrap_comment_diff, wrap_topic_diff, wrap_viewpoint_diff,
    BcfCommentDiff, BcfCommentsDiff, BcfDiff, BcfTopicDiff, BcfTopicsDiff, BcfViewpointDiff, BcfViewpointsDiff,
};
use crate::artifacts::bcf::schema::diff::{
    dec_bytes, dec_camera, dec_comment, dec_components, dec_list, dec_part, dec_str, dec_topic, dec_viewpoint,
    decode_option, enc_bytes, enc_camera, enc_comment, enc_components, enc_list, enc_part, enc_str, enc_topic,
    enc_viewpoint, encode_option, split_top_level, strip_brackets,
};
use crate::artifacts::bcf::schema::snapshot::{BcfCamera, BcfComment, BcfComponents, BcfTopic, BcfViewpoint};
use crate::artifacts::bcf::BcfSnapshot;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum BcfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: BcfSnapshot,
    },
    SetVersion {
        version: String,
    },
    InsertTopic {
        topic: BcfTopic,
    },
    RemoveTopic {
        guid: String,
    },
    SetTopicMarkup {
        guid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        priority: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        labels: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        creation_date: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        creation_author: Option<String>,
    },
    InsertComment {
        topic_guid: String,
        comment: BcfComment,
    },
    RemoveComment {
        topic_guid: String,
        guid: String,
    },
    SetComment {
        topic_guid: String,
        guid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        date: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        author: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        viewpoint_ref: Option<Option<String>>,
    },
    InsertViewpoint {
        topic_guid: String,
        viewpoint: BcfViewpoint,
    },
    RemoveViewpoint {
        topic_guid: String,
        guid: String,
    },
    SetViewpointCamera {
        topic_guid: String,
        guid: String,
        camera: Option<BcfCamera>,
    },
    SetViewpointComponents {
        topic_guid: String,
        guid: String,
        components: Option<BcfComponents>,
    },
    SetViewpointSnapshot {
        topic_guid: String,
        guid: String,
        snapshot: Option<Vec<u8>>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Single semantics source: the returned diff IS what gets
/// applied.
pub fn apply_bcf_mutation(snapshot: &mut BcfSnapshot, mutation: &BcfMutation) -> BcfDiff {
    let diff = <BcfMutation as Mutation<BcfSnapshot>>::diff(mutation, snapshot);
    *snapshot = <BcfDiff as protocol::MutationDiff<BcfSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<BcfSnapshot> for BcfMutation {
    type Diff = BcfDiff;

    fn diff(&self, base: &BcfSnapshot) -> Self::Diff {
        match self {
            BcfMutation::NoMutation => BcfDiff::default(),
            BcfMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            BcfMutation::SetVersion { version } => BcfDiff { version: Some(version.clone()), topics: None, parts: None },
            BcfMutation::InsertTopic { topic } => BcfDiff {
                version: None,
                topics: Some(BcfTopicsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![topic.clone()] }),
                parts: None,
            },
            BcfMutation::RemoveTopic { guid } => BcfDiff {
                version: None,
                topics: Some(BcfTopicsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }),
                parts: None,
            },
            BcfMutation::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author } => {
                wrap_topic_diff(guid, BcfTopicDiff {
                    title: title.clone(),
                    description: description.clone(),
                    status: status.clone(),
                    priority: priority.clone(),
                    labels: labels.clone(),
                    creation_date: creation_date.clone(),
                    creation_author: creation_author.clone(),
                    comments: None,
                    viewpoints: None,
                })
            }
            BcfMutation::InsertComment { topic_guid, comment } => wrap_topic_diff(topic_guid, BcfTopicDiff {
                comments: Some(BcfCommentsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![comment.clone()] }),
                ..Default::default()
            }),
            BcfMutation::RemoveComment { topic_guid, guid } => wrap_topic_diff(topic_guid, BcfTopicDiff {
                comments: Some(BcfCommentsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }),
                ..Default::default()
            }),
            BcfMutation::SetComment { topic_guid, guid, date, author, text, viewpoint_ref } => {
                wrap_comment_diff(topic_guid, guid, BcfCommentDiff {
                    date: date.clone(),
                    author: author.clone(),
                    text: text.clone(),
                    viewpoint_ref: viewpoint_ref.clone(),
                })
            }
            BcfMutation::InsertViewpoint { topic_guid, viewpoint } => wrap_topic_diff(topic_guid, BcfTopicDiff {
                viewpoints: Some(BcfViewpointsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![viewpoint.clone()] }),
                ..Default::default()
            }),
            BcfMutation::RemoveViewpoint { topic_guid, guid } => wrap_topic_diff(topic_guid, BcfTopicDiff {
                viewpoints: Some(BcfViewpointsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }),
                ..Default::default()
            }),
            BcfMutation::SetViewpointCamera { topic_guid, guid, camera } => {
                wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: Some(camera.clone()), components: None, snapshot: None })
            }
            BcfMutation::SetViewpointComponents { topic_guid, guid, components } => {
                wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: None, components: Some(components.clone()), snapshot: None })
            }
            BcfMutation::SetViewpointSnapshot { topic_guid, guid, snapshot } => {
                wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: None, components: None, snapshot: Some(snapshot.clone()) })
            }
        }
    }

    fn inverse(&self, base: &BcfSnapshot) -> Vec<Self> {
        match self {
            BcfMutation::NoMutation => vec![BcfMutation::NoMutation],
            BcfMutation::SetSnapshot { .. } => vec![BcfMutation::SetSnapshot { snapshot: base.clone() }],
            BcfMutation::SetVersion { .. } => vec![BcfMutation::SetVersion { version: base.version.clone() }],
            BcfMutation::InsertTopic { topic } => vec![BcfMutation::RemoveTopic { guid: topic.guid.clone() }],
            BcfMutation::RemoveTopic { guid } => match find_topic(base, guid) {
                Some(t) => vec![BcfMutation::InsertTopic { topic: t.clone() }],
                None => vec![BcfMutation::NoMutation],
            },
            BcfMutation::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author } => {
                match find_topic(base, guid) {
                    Some(t) => vec![BcfMutation::SetTopicMarkup {
                        guid: guid.clone(),
                        title: title.as_ref().map(|_| t.title.clone()),
                        description: description.as_ref().map(|_| t.description.clone()),
                        status: status.as_ref().map(|_| t.status.clone()),
                        priority: priority.as_ref().map(|_| t.priority.clone()),
                        labels: labels.as_ref().map(|_| t.labels.clone()),
                        creation_date: creation_date.as_ref().map(|_| t.creation_date.clone()),
                        creation_author: creation_author.as_ref().map(|_| t.creation_author.clone()),
                    }],
                    None => vec![BcfMutation::NoMutation],
                }
            }
            BcfMutation::InsertComment { topic_guid, comment } => {
                vec![BcfMutation::RemoveComment { topic_guid: topic_guid.clone(), guid: comment.guid.clone() }]
            }
            BcfMutation::RemoveComment { topic_guid, guid } => match find_comment(base, topic_guid, guid) {
                Some(c) => vec![BcfMutation::InsertComment { topic_guid: topic_guid.clone(), comment: c.clone() }],
                None => vec![BcfMutation::NoMutation],
            },
            BcfMutation::SetComment { topic_guid, guid, date, author, text, viewpoint_ref } => {
                match find_comment(base, topic_guid, guid) {
                    Some(c) => vec![BcfMutation::SetComment {
                        topic_guid: topic_guid.clone(),
                        guid: guid.clone(),
                        date: date.as_ref().map(|_| c.date.clone()),
                        author: author.as_ref().map(|_| c.author.clone()),
                        text: text.as_ref().map(|_| c.text.clone()),
                        viewpoint_ref: viewpoint_ref.as_ref().map(|_| c.viewpoint_ref.clone()),
                    }],
                    None => vec![BcfMutation::NoMutation],
                }
            }
            BcfMutation::InsertViewpoint { topic_guid, viewpoint } => {
                vec![BcfMutation::RemoveViewpoint { topic_guid: topic_guid.clone(), guid: viewpoint.guid.clone() }]
            }
            BcfMutation::RemoveViewpoint { topic_guid, guid } => match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![BcfMutation::InsertViewpoint { topic_guid: topic_guid.clone(), viewpoint: v.clone() }],
                None => vec![BcfMutation::NoMutation],
            },
            BcfMutation::SetViewpointCamera { topic_guid, guid, .. } => match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![BcfMutation::SetViewpointCamera { topic_guid: topic_guid.clone(), guid: guid.clone(), camera: v.camera.clone() }],
                None => vec![BcfMutation::NoMutation],
            },
            BcfMutation::SetViewpointComponents { topic_guid, guid, .. } => match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![BcfMutation::SetViewpointComponents { topic_guid: topic_guid.clone(), guid: guid.clone(), components: v.components.clone() }],
                None => vec![BcfMutation::NoMutation],
            },
            BcfMutation::SetViewpointSnapshot { topic_guid, guid, .. } => match find_viewpoint(base, topic_guid, guid) {
                Some(v) => vec![BcfMutation::SetViewpointSnapshot { topic_guid: topic_guid.clone(), guid: guid.clone(), snapshot: v.snapshot.clone() }],
                None => vec![BcfMutation::NoMutation],
            },
        }
    }
}

fn find_topic<'a>(base: &'a BcfSnapshot, guid: &str) -> Option<&'a BcfTopic> {
    base.topics.iter().find(|t| t.guid == guid)
}

fn find_comment<'a>(base: &'a BcfSnapshot, topic_guid: &str, guid: &str) -> Option<&'a BcfComment> {
    find_topic(base, topic_guid)?.comments.iter().find(|c| c.guid == guid)
}

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
fn enc_bcf_snapshot(s: &BcfSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_str(&s.version), enc_list(&s.topics, enc_topic), enc_list(&s.parts, enc_part))
}
fn dec_bcf_snapshot(s: &str) -> Result<BcfSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, version, topics, parts_field] = parts.as_slice() else { return Err(format!("bcf snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(BcfSnapshot { schema: dec_str(schema)?, version: dec_str(version)?, topics: dec_list(topics, dec_topic)?, parts: dec_list(parts_field, dec_part)? })
}

fn print_bcf_mutation(m: &BcfMutation) -> String {
    match m {
        BcfMutation::NoMutation => "no-mutation".to_string(),
        BcfMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_bcf_snapshot(snapshot)),
        BcfMutation::SetVersion { version } => format!("set-version version={}", enc_str(version)),
        BcfMutation::InsertTopic { topic } => format!("insert-topic topic={}", enc_topic(topic)),
        BcfMutation::RemoveTopic { guid } => format!("remove-topic guid={}", enc_str(guid)),
        BcfMutation::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author } => format!(
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
        BcfMutation::InsertComment { topic_guid, comment } => format!("insert-comment topic-guid={} comment={}", enc_str(topic_guid), enc_comment(comment)),
        BcfMutation::RemoveComment { topic_guid, guid } => format!("remove-comment topic-guid={} guid={}", enc_str(topic_guid), enc_str(guid)),
        BcfMutation::SetComment { topic_guid, guid, date, author, text, viewpoint_ref } => format!(
            "set-comment topic-guid={} guid={} date={} author={} text={} viewpoint-ref={}",
            enc_str(topic_guid), enc_str(guid),
            encode_option(date, |v: &String| enc_str(v)),
            encode_option(author, |v: &String| enc_str(v)),
            encode_option(text, |v: &String| enc_str(v)),
            encode_option(viewpoint_ref, |inner: &Option<String>| encode_option(inner, |v: &String| enc_str(v))),
        ),
        BcfMutation::InsertViewpoint { topic_guid, viewpoint } => format!("insert-viewpoint topic-guid={} viewpoint={}", enc_str(topic_guid), enc_viewpoint(viewpoint)),
        BcfMutation::RemoveViewpoint { topic_guid, guid } => format!("remove-viewpoint topic-guid={} guid={}", enc_str(topic_guid), enc_str(guid)),
        BcfMutation::SetViewpointCamera { topic_guid, guid, camera } => format!("set-viewpoint-camera topic-guid={} guid={} camera={}", enc_str(topic_guid), enc_str(guid), encode_option(camera, enc_camera)),
        BcfMutation::SetViewpointComponents { topic_guid, guid, components } => format!("set-viewpoint-components topic-guid={} guid={} components={}", enc_str(topic_guid), enc_str(guid), encode_option(components, enc_components)),
        BcfMutation::SetViewpointSnapshot { topic_guid, guid, snapshot } => format!("set-viewpoint-snapshot topic-guid={} guid={} snapshot={}", enc_str(topic_guid), enc_str(guid), encode_option(snapshot, |b: &Vec<u8>| enc_bytes(b))),
    }
}
fn parse_bcf_mutation(line: &str) -> Result<BcfMutation, String> {
    if line == "no-mutation" {
        return Ok(BcfMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("bcf mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("bcf mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(BcfMutation::SetSnapshot { snapshot: dec_bcf_snapshot(arg("snapshot")?)? }),
        "set-version" => Ok(BcfMutation::SetVersion { version: dec_str(arg("version")?)? }),
        "insert-topic" => Ok(BcfMutation::InsertTopic { topic: dec_topic(arg("topic")?)? }),
        "remove-topic" => Ok(BcfMutation::RemoveTopic { guid: dec_str(arg("guid")?)? }),
        "set-topic-markup" => Ok(BcfMutation::SetTopicMarkup {
            guid: dec_str(arg("guid")?)?,
            title: decode_option(arg("title")?, dec_str)?,
            description: decode_option(arg("description")?, dec_str)?,
            status: decode_option(arg("status")?, dec_str)?,
            priority: decode_option(arg("priority")?, dec_str)?,
            labels: decode_option(arg("labels")?, |s| dec_list(s, dec_str))?,
            creation_date: decode_option(arg("creation-date")?, dec_str)?,
            creation_author: decode_option(arg("creation-author")?, dec_str)?,
        }),
        "insert-comment" => Ok(BcfMutation::InsertComment { topic_guid: dec_str(arg("topic-guid")?)?, comment: dec_comment(arg("comment")?)? }),
        "remove-comment" => Ok(BcfMutation::RemoveComment { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)? }),
        "set-comment" => Ok(BcfMutation::SetComment {
            topic_guid: dec_str(arg("topic-guid")?)?,
            guid: dec_str(arg("guid")?)?,
            date: decode_option(arg("date")?, dec_str)?,
            author: decode_option(arg("author")?, dec_str)?,
            text: decode_option(arg("text")?, dec_str)?,
            viewpoint_ref: decode_option(arg("viewpoint-ref")?, |s| decode_option(s, dec_str))?,
        }),
        "insert-viewpoint" => Ok(BcfMutation::InsertViewpoint { topic_guid: dec_str(arg("topic-guid")?)?, viewpoint: dec_viewpoint(arg("viewpoint")?)? }),
        "remove-viewpoint" => Ok(BcfMutation::RemoveViewpoint { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)? }),
        "set-viewpoint-camera" => Ok(BcfMutation::SetViewpointCamera { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, camera: decode_option(arg("camera")?, dec_camera)? }),
        "set-viewpoint-components" => Ok(BcfMutation::SetViewpointComponents { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, components: decode_option(arg("components")?, dec_components)? }),
        "set-viewpoint-snapshot" => Ok(BcfMutation::SetViewpointSnapshot { topic_guid: dec_str(arg("topic-guid")?)?, guid: dec_str(arg("guid")?)?, snapshot: decode_option(arg("snapshot")?, dec_bytes)? }),
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

/// ⚡️ Binary = the text bytes verbatim, same simplification as `BcfDiff`'s hand-rolled codec.
impl protocol::OpBinary for BcfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_op().into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 0, detail: e.to_string() })?;
        Self::parse_op(line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 0, detail: e.to_string() })
    }
}
//#endregion OpCodecs
