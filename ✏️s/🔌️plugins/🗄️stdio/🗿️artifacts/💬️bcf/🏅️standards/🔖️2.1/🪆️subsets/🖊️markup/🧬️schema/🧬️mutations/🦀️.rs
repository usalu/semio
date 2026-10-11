//! 🧬️ BcfMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) via the diff module's `wrap_*_diff` helpers; every variant's `inverse()`
//! looks up prior state from `base` and constructs the exact undoing mutation (guid-aware,
//! matching svg/docx precedent). `SetVersion`/`SetViewpointSnapshot` extend the brief's literal
//! mutation list (`InsertTopic/RemoveTopic/SetTopicMarkup,
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







































use crate::schema::diff::{comment_index, topic_index, viewpoint_index, wrap_comment_diff, wrap_topic_diff, wrap_viewpoint_diff, BcfCommentDiff, BcfCommentsDiff, BcfDiff, BcfPartsDiff, BcfTopicDiff, BcfTopicsDiff, BcfViewpointDiff, BcfViewpointsDiff, IndexedAdded};
use crate::schema::snapshot::{BcfCamera, BcfComment, BcfComponents, BcfRawPart, BcfTopic, BcfViewpoint};
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
#[path = "📎set-parts/🦀️.rs"]
pub mod set_parts;
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
/// the mutation-side twin of the diff's blockers — `InsertTopic`/
/// `InsertComment`/`InsertViewpoint` each carry a whole `BcfTopic`/`BcfComment`/`BcfViewpoint`
/// (none of which derive `DslField` — none are `DslRecord`-derived), and
/// `SetViewpointCamera{camera: Option<BcfCamera>}` carries the enum DIRECTLY as a variant field
/// (`error[E0277]` at this exact line, not just via the nested snapshot) — the mutation-side
/// mirror of `SvgMutation`'s `InsertElement{node: XmlNode}` finding (`f6-recon-report.md` §3a).
/// `SetComment{viewpoint_ref: Option<Option<String>>}` also independently fails the tri-state
/// check (§3b). `OpText`/`OpBinary` hand-rolled below, reusing the diff module's `pub(crate)`
/// grammar primitives.
//#region 🔖️Leaves
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
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = BcfSnapshot, diff = BcfDiff, schema = "BcfMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum BcfMutation {
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
    SetParts(set_parts::SetParts),
}

/// 📇️ Kebab-case spelling of every `BcfMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../../🔣️oracle.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself). Mirrors `print_bcf_mutation`'s own keyword match
/// entry-for-entry, so `KINDS[i]` is exactly what `print_op()` emits for the enum's `i`-th variant.
pub const KINDS: &[&str] = &[
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
    "set-parts",
];
//#endregion 🔖️Mutations


//#endregion 🔖️Apply


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

