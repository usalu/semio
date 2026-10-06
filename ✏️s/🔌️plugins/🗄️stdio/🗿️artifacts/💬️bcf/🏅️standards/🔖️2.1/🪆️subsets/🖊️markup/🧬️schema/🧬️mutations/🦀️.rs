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







































use crate::schema::diff::{diff_set_snapshot, wrap_comment_diff, wrap_topic_diff, wrap_viewpoint_diff, BcfCommentDiff, BcfCommentsDiff, BcfDiff, BcfTopicDiff, BcfTopicsDiff, BcfViewpointDiff, BcfViewpointsDiff};
use crate::schema::snapshot::{BcfCamera, BcfComment, BcfComponents, BcfTopic, BcfViewpoint};
use crate::BcfSnapshot;
use protocol::Mutation;

//#region 🔖️DoubleOption
/// 🪆️ Decodes a present key of an `Option<Option<T>>` field as `Some(inner)`, so a present `null` is `Some(None)` ("clear")
/// and only an absent key (the field's `default`) is `None` ("untouched") — the blanket `Option<T>` impl would collapse both
/// to `None`. Paired with `skip_serializing_if = "Option::is_none"`, `payload_value()` and `with_payload_value()` round-trip.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}
//#endregion 🔖️DoubleOption

//#region 🔖️Mutations
#[path = "🗨️insert-comment/🦀️.rs"]
pub mod insert_comment;
#[path = "📌insert-topic/🦀️.rs"]
pub mod insert_topic;
#[path = "👁️insert-viewpoint/🦀️.rs"]
pub mod insert_viewpoint;
#[path = "🧹remove-comment/🦀️.rs"]
pub mod remove_comment;
#[path = "🗑️remove-topic/🦀️.rs"]
pub mod remove_topic;
#[path = "🙈remove-viewpoint/🦀️.rs"]
pub mod remove_viewpoint;
#[path = "✏️set-comment/🦀️.rs"]
pub mod set_comment;
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
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "🗃️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🖊️set-topic-markup/🦀️.rs"]
pub mod set_topic_markup;
#[path = "🔢set-version/🦀️.rs"]
pub mod set_version;
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
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum BcfMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
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
    "set-snapshot", "patch-snapshot",
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
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply


//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &BcfMutation, base: &BcfSnapshot) -> protocol::MutationOutcome<BcfDiff> {
    protocol::MutationOutcome::new(match this {
        BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        BcfMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<BcfSnapshot, BcfMutation>>::diff(patch, base),
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
        BcfMutation::InsertComment(insert_comment::InsertComment { topic_guid, comment }) => {
            wrap_topic_diff(topic_guid, BcfTopicDiff { comments: Some(BcfCommentsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![comment.clone()] }), ..Default::default() })
        }
        BcfMutation::RemoveComment(remove_comment::RemoveComment { topic_guid, guid }) => {
            wrap_topic_diff(topic_guid, BcfTopicDiff { comments: Some(BcfCommentsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }), ..Default::default() })
        }
        BcfMutation::SetComment(set_comment::SetComment { topic_guid, guid, date, author, text, viewpoint_ref }) => {
            wrap_comment_diff(topic_guid, guid, BcfCommentDiff { date: date.clone(), author: author.clone(), text: text.clone(), viewpoint_ref: viewpoint_ref.clone() })
        }
        BcfMutation::InsertViewpoint(insert_viewpoint::InsertViewpoint { topic_guid, viewpoint }) => {
            wrap_topic_diff(topic_guid, BcfTopicDiff { viewpoints: Some(BcfViewpointsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![viewpoint.clone()] }), ..Default::default() })
        }
        BcfMutation::RemoveViewpoint(remove_viewpoint::RemoveViewpoint { topic_guid, guid }) => {
            wrap_topic_diff(topic_guid, BcfTopicDiff { viewpoints: Some(BcfViewpointsDiff { removed: vec![guid.clone()], modified: Vec::new(), added: Vec::new() }), ..Default::default() })
        }
        BcfMutation::SetViewpointCamera(set_viewpoint_camera::SetViewpointCamera { topic_guid, guid, camera }) => wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: Some(camera.clone()), components: None, snapshot: None }),
        BcfMutation::SetViewpointComponents(set_viewpoint_components::SetViewpointComponents { topic_guid, guid, components }) => {
            wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: None, components: Some(components.clone()), snapshot: None })
        }
        BcfMutation::SetViewpointSnapshot(set_viewpoint_snapshot::SetViewpointSnapshot { topic_guid, guid, snapshot }) => wrap_viewpoint_diff(topic_guid, guid, BcfViewpointDiff { camera: None, components: None, snapshot: Some(snapshot.clone()) }),
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &BcfMutation, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        BcfMutation::SetSnapshot(_) => vec![BcfMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        BcfMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<BcfSnapshot, BcfMutation>>::inverse(patch, base)?),
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

    })
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








//#region 🔖️OpBinaryCodec




//#endregion 🔖️OpBinaryCodec




//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `BcfMutation` values -- one per variant -- the single source of
/// truth reused by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📜️docx/…/🧬️mutations/🦀️.rs`'s own
/// `demo_mutation_cases()` establishes.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<BcfMutation> {
    use crate::schema::diff::{demo_snapshot_a, demo_snapshot_b};
    let base = demo_snapshot_a();
    let snapshot = demo_snapshot_b();
    vec![
        BcfMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
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
        BcfMutation::SetComment(set_comment::SetComment { topic_guid: "keep".into(), guid: "c-keep".into(), date: Some("2025-01-01T00:00:00+00:00".into()), author: Some("a@example.com".into()), text: None, viewpoint_ref: Some(Some("vp2".into())) }),
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
#[path = "🧪️tests/🔬️kinds/🦀️.rs"]
mod kinds_tests;
//#endregion 🧪️KindsLaw

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
