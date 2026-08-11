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
use crate::artifacts::bcf::schema::snapshot::{BcfCamera, BcfComment, BcfComponents, BcfTopic, BcfViewpoint};
use crate::artifacts::bcf::BcfSnapshot;
use protocol::Mutation;
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
impl protocol::OpText for BcfMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for BcfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs
