//! 🦀️ BCF 2.1 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `wellness-center-coordination-review.bcf` fixture
//! (derived once from a real IFC2X3 model plus a real committed floor plan PNG — see the feature
//! file's own header) into the case work directory first; the committed fixture is never written
//! to. `oracle` drives the registered independent `zip`+`quick-xml` composition
//! (`../../🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_bcf`/`encode_bcf`/`apply_bcf_mutation` over the full 14-kind `BcfMutation` vocabulary.
//! Both results are read back by the SAME independent `project_bcf_2_1` before the `semantic-bcf-v1`
//! profile compares them. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::bcf::standards::v2_1::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_bcf_2_1};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `BcfMutation` variant, mirrored from
/// `../../🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own `KINDS` --
/// duplicated rather than imported because the ORACLE-only build of this adapter must never link
/// `semio-s-plugin-stdio`.
const KINDS: &[&str] = &[
    "no-mutation",
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
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://wellness-center-coordination-review.bcf";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("coordination-review.bcf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Laws
/// 🔍️ First point at which two projections disagree, as a `path: expected != read` sentence -- a law
/// violation must name the field that broke it rather than dump two whole documents at the reader.
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    match (expected, actual) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in left {
                match right.iter().find(|(name, _)| name == key) {
                    Some((_, other)) => {
                        if let Some(found) = first_divergence(&format!("{path}.{key}"), value, other) {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{path}.{key} is absent from the result")),
                }
            }
            right.iter().find(|(key, _)| !left.iter().any(|(name, _)| name == key)).map(|(key, _)| format!("{path}.{key} appeared in the result out of nowhere"))
        }
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{path} holds {} member(s), expected {}", right.len(), left.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&format!("{path}[{index}]"), value, other))
        }
        _ if expected == actual => None,
        _ => Some(format!("{path}: expected {} but read {}", expected.to_string(), actual.to_string())),
    }
}

/// ⚖️ Turns a projection law into a real verdict: `Ok` only when the two projections agree, otherwise
/// an `Err` naming the FIRST field that diverged. Without this an oracle handler asserts nothing and
/// its scenario passes whenever the reference library merely declined to error.
fn assert_same_projection(law: &str, expected: &Json, actual: &Json) -> Result<(), String> {
    match first_divergence("projection", expected, actual) {
        Some(divergence) => Err(format!("{law}: {divergence}")),
        None => Ok(()),
    }
}
//#endregion 🔖️Laws

//#region 🔖️Oracle
/// 🧾️ The `no-mutation` spec, spelled once -- both laws below need a baseline that has been through
/// exactly the same number of unzip/rezip cycles as the archive they judge, so that a divergence
/// names the mutation pair and never the reference writer's own normal form.
fn no_mutation() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))])
}

/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to. It asserts ONE thing in
/// role, before any parity comparison exists: every kind other than `no-mutation` must MOVE the
/// semantic projection. A row whose parameters make the mutation a no-op is not a test -- it passes
/// whenever the reference library declined to error, which is exactly the failure this platform
/// exists to prevent. The baseline runs one `no-mutation` cycle so the comparison isolates the
/// mutation rather than the writer's own normal form.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_bcf_2_1(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_bcf_2_1(&bytes)?;
    if kind != "no-mutation" && projection == baseline {
        return Err(format!("{kind:?} left the semantic projection of the coordination review unchanged -- a mutation that is not observable proves nothing, so this row's parameters do not exercise the kind they name"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id, and the ORACLE side of the inverse
/// law -- a law that is checkable in-role, without a subject: the independent `zip`+`quick-xml`
/// composition applies the forward mutation and then its own base-relative inverse
/// (`oracle_apply_mutation_inverse`), and the restored archive MUST project exactly as the untouched
/// coordination review does. The baseline is taken through one `no-mutation` cycle so both sides
/// carry the same re-serialisation and the comparison isolates the mutation pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let baseline = project_bcf_2_1(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_bcf_2_1(&bytes)?;
    assert_same_projection(&format!("inverse law violated for {:?} -- undoing it did not restore the coordination review", spec.str("kind")), &baseline, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity round trip, asserted in-role: the independent `zip`+`quick-xml`
/// composition fully parses the real coordination review and re-serializes it from its own model
/// alone, so the re-encoded archive MUST carry the same semantic projection as the input AND MUST NOT
/// be bit-identical to it. A `.bcf` is not a byte-preserving carrier -- every markup part is re-written
/// by the XML writer and every entry re-deflated -- so the byte tripwire is real evidence that the
/// archive was parsed rather than copied.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_bcf_2_1(&input)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;
    if bytes == input {
        return Err("byte pass-through: the re-encoded output is bit-identical to the input, so nothing here proves the archive was parsed".to_string());
    }
    let projection = project_bcf_2_1(&bytes)?;
    assert_same_projection("identity round trip is not semantics-preserving", &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::bcf::schema::mutations::{apply_bcf_mutation, BcfMutation};
    use semio_s_plugin_stdio::artifacts::bcf::schema::snapshot::{BcfCamera, BcfColoring, BcfComment, BcfComponents, BcfPoint3, BcfRawPart, BcfTopic, BcfViewpoint, BcfVisibility};
    use semio_s_plugin_stdio::artifacts::bcf::standards::v2_1::subsets::any::io::{decode_bcf, encode_bcf};
    use semio_s_plugin_stdio::artifacts::bcf::BcfSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::bcf::standards::v2_1::subsets::any::project_bcf_2_1;

    //#region 🔖️Hex
    /// 🔤️ The same lowercase-hex binary-in-text convention the oracle side uses for `BcfSnapshot`'s
    /// own DSL form -- duplicated rather than imported since this module must not depend on the
    /// oracle crate's private `hex_decode`.
    fn hex_decode(text: &str) -> Result<Vec<u8>, String> {
        if text.len() % 2 != 0 {
            return Err(format!("odd hex length ({} chars)", text.len()));
        }
        (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|error| format!("invalid hex {:?}: {error}", &text[i..i + 2]))).collect()
    }
    //#endregion 🔖️Hex

    //#region 🔖️SpecCodec
    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn strings(items: Vec<Json>) -> Vec<String> {
        items.into_iter().filter_map(|item| if let Json::String(text) = item { Some(text) } else { None }).collect()
    }

    fn point_from_json(value: &Json) -> BcfPoint3 {
        BcfPoint3 { x: number_field(value, "x"), y: number_field(value, "y"), z: number_field(value, "z") }
    }

    /// 📄️ The same owned camera-spec JSON grammar the oracle side speaks
    /// (`{"kind":"perspective"|"orthogonal", ...}`), decoded into the PRODUCTION `BcfCamera` here
    /// instead of the oracle's own independent type.
    fn camera_from_json(value: &Json) -> Result<BcfCamera, String> {
        let view_point = value.get("viewPoint").map(point_from_json).unwrap_or_default();
        let direction = value.get("direction").map(point_from_json).unwrap_or_default();
        let up_vector = value.get("upVector").map(point_from_json).unwrap_or_default();
        match value.str("kind").as_str() {
            "perspective" => Ok(BcfCamera::Perspective { view_point, direction, up_vector, field_of_view: number_field(value, "fieldOfView") }),
            "orthogonal" => Ok(BcfCamera::Orthogonal { view_point, direction, up_vector, view_to_world_scale: number_field(value, "viewToWorldScale") }),
            other => Err(format!("unknown camera kind {other:?}")),
        }
    }

    fn components_from_json(value: &Json) -> BcfComponents {
        let visibility = match value.get("visibility") {
            Some(node) => BcfVisibility { default_visibility: match node.get("defaultVisibility") { Some(Json::Bool(flag)) => *flag, _ => true }, exceptions: strings(node.array("exceptions")) },
            None => BcfVisibility { default_visibility: true, exceptions: Vec::new() },
        };
        BcfComponents { selection: strings(value.array("selection")), visibility, coloring: value.array("coloring").iter().map(|entry| BcfColoring { color: entry.str("color"), components: strings(entry.array("components")) }).collect() }
    }

    fn comment_from_json(value: &Json) -> BcfComment {
        BcfComment {
            guid: value.str("guid"),
            date: value.str("date"),
            author: value.str("author"),
            text: value.str("text"),
            viewpoint_ref: match value.get("viewpointRef") {
                Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
                _ => None,
            },
        }
    }

    fn viewpoint_from_json(value: &Json) -> Result<BcfViewpoint, String> {
        let camera = match value.get("camera") {
            Some(Json::Null) | None => None,
            Some(node) => Some(camera_from_json(node)?),
        };
        let components = match value.get("components") {
            Some(Json::Null) | None => None,
            Some(node) => Some(components_from_json(node)),
        };
        let snapshot = match value.get("snapshot") {
            Some(Json::String(hex)) if !hex.is_empty() => Some(hex_decode(hex)?),
            _ => None,
        };
        Ok(BcfViewpoint { guid: value.str("guid"), camera, components, snapshot })
    }

    fn topic_from_json(value: &Json) -> Result<BcfTopic, String> {
        Ok(BcfTopic {
            guid: value.str("guid"),
            title: value.str("title"),
            description: value.str("description"),
            status: value.str("status"),
            priority: value.str("priority"),
            labels: strings(value.array("labels")),
            creation_date: value.str("creationDate"),
            creation_author: value.str("creationAuthor"),
            comments: value.array("comments").iter().map(comment_from_json).collect(),
            viewpoints: value.array("viewpoints").iter().map(viewpoint_from_json).collect::<Result<_, _>>()?,
        })
    }

    fn snapshot_from_json(value: &Json) -> Result<BcfSnapshot, String> {
        Ok(BcfSnapshot {
            schema: semio_s_plugin_stdio::artifacts::bcf::STDIO_BCF_DOCUMENT_SCHEMA.to_string(),
            version: value.str("version"),
            topics: value.array("topics").iter().map(topic_from_json).collect::<Result<_, _>>()?,
            parts: value
                .array("parts")
                .iter()
                .map(|entry| {
                    let data = match entry.get("content") {
                        Some(Json::String(hex)) if !hex.is_empty() => hex_decode(hex)?,
                        _ => Vec::new(),
                    };
                    Ok(BcfRawPart { name: entry.str("name"), data })
                })
                .collect::<Result<_, String>>()?,
        })
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `BcfMutation` this subset
    /// declares for it.
    fn mutation_from_spec(spec: &Json) -> Result<BcfMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(BcfMutation::NoMutation),
            "set-snapshot" => Ok(BcfMutation::SetSnapshot { snapshot: snapshot_from_json(&params)? }),
            "set-version" => Ok(BcfMutation::SetVersion { version: params.str("version") }),
            "insert-topic" => Ok(BcfMutation::InsertTopic { topic: topic_from_json(&params.get("topic").cloned().unwrap_or(Json::Null))? }),
            "remove-topic" => Ok(BcfMutation::RemoveTopic { guid: params.str("guid") }),
            "set-topic-markup" => Ok(BcfMutation::SetTopicMarkup {
                guid: params.str("guid"),
                title: match params.get("title") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                description: match params.get("description") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                status: match params.get("status") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                priority: match params.get("priority") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                labels: match params.get("labels") { Some(Json::Array(_)) => Some(strings(params.array("labels"))), _ => None },
                creation_date: match params.get("creationDate") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                creation_author: match params.get("creationAuthor") { Some(Json::String(value)) => Some(value.clone()), _ => None },
            }),
            "insert-comment" => Ok(BcfMutation::InsertComment { topic_guid: params.str("topicGuid"), comment: comment_from_json(&params.get("comment").cloned().unwrap_or(Json::Null)) }),
            "remove-comment" => Ok(BcfMutation::RemoveComment { topic_guid: params.str("topicGuid"), guid: params.str("guid") }),
            "set-comment" => Ok(BcfMutation::SetComment {
                topic_guid: params.str("topicGuid"),
                guid: params.str("guid"),
                date: match params.get("date") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                author: match params.get("author") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                text: match params.get("text") { Some(Json::String(value)) => Some(value.clone()), _ => None },
                viewpoint_ref: match params.get("viewpointRef") {
                    Some(Json::String(reference)) if !reference.is_empty() => Some(Some(reference.clone())),
                    Some(Json::Null) => Some(None),
                    _ => None,
                },
            }),
            "insert-viewpoint" => Ok(BcfMutation::InsertViewpoint { topic_guid: params.str("topicGuid"), viewpoint: viewpoint_from_json(&params.get("viewpoint").cloned().unwrap_or(Json::Null))? }),
            "remove-viewpoint" => Ok(BcfMutation::RemoveViewpoint { topic_guid: params.str("topicGuid"), guid: params.str("guid") }),
            "set-viewpoint-camera" => Ok(BcfMutation::SetViewpointCamera {
                topic_guid: params.str("topicGuid"),
                guid: params.str("guid"),
                camera: match params.get("camera") {
                    Some(Json::Null) | None => None,
                    Some(node) => Some(camera_from_json(node)?),
                },
            }),
            "set-viewpoint-components" => Ok(BcfMutation::SetViewpointComponents {
                topic_guid: params.str("topicGuid"),
                guid: params.str("guid"),
                components: match params.get("components") {
                    Some(Json::Null) | None => None,
                    Some(node) => Some(components_from_json(node)),
                },
            }),
            "set-viewpoint-snapshot" => Ok(BcfMutation::SetViewpointSnapshot {
                topic_guid: params.str("topicGuid"),
                guid: params.str("guid"),
                snapshot: match params.get("snapshot") {
                    Some(Json::String(hex)) if !hex.is_empty() => Some(hex_decode(hex)?),
                    _ => None,
                },
            }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    fn find_topic<'a>(base: &'a BcfSnapshot, guid: &str) -> Option<&'a BcfTopic> {
        base.topics.iter().find(|topic| topic.guid == guid)
    }

    fn find_comment<'a>(base: &'a BcfSnapshot, topic_guid: &str, guid: &str) -> Option<&'a BcfComment> {
        find_topic(base, topic_guid)?.comments.iter().find(|comment| comment.guid == guid)
    }

    fn find_viewpoint<'a>(base: &'a BcfSnapshot, topic_guid: &str, guid: &str) -> Option<&'a BcfViewpoint> {
        find_topic(base, topic_guid)?.viewpoints.iter().find(|viewpoint| viewpoint.guid == guid)
    }

    /// ↩️ `BcfMutation::inverse` in closed form -- every variant's own `Mutation::inverse` arm,
    /// transplanted rather than called through the trait, same precedent `mutate-pdf-1-7`'s own
    /// `inverse_of` gives: written in closed form so this adapter needs no extra crate dependency
    /// beyond `semio-s-plugin-stdio` itself.
    fn inverse_of(mutation: &BcfMutation, base: &BcfSnapshot) -> BcfMutation {
        match mutation {
            BcfMutation::NoMutation => BcfMutation::NoMutation,
            BcfMutation::SetSnapshot { .. } => BcfMutation::SetSnapshot { snapshot: base.clone() },
            BcfMutation::SetVersion { .. } => BcfMutation::SetVersion { version: base.version.clone() },
            BcfMutation::InsertTopic { topic } => BcfMutation::RemoveTopic { guid: topic.guid.clone() },
            BcfMutation::RemoveTopic { guid } => match find_topic(base, guid) {
                Some(topic) => BcfMutation::InsertTopic { topic: topic.clone() },
                None => BcfMutation::NoMutation,
            },
            BcfMutation::SetTopicMarkup { guid, title, description, status, priority, labels, creation_date, creation_author } => match find_topic(base, guid) {
                Some(topic) => BcfMutation::SetTopicMarkup {
                    guid: guid.clone(),
                    title: title.as_ref().map(|_| topic.title.clone()),
                    description: description.as_ref().map(|_| topic.description.clone()),
                    status: status.as_ref().map(|_| topic.status.clone()),
                    priority: priority.as_ref().map(|_| topic.priority.clone()),
                    labels: labels.as_ref().map(|_| topic.labels.clone()),
                    creation_date: creation_date.as_ref().map(|_| topic.creation_date.clone()),
                    creation_author: creation_author.as_ref().map(|_| topic.creation_author.clone()),
                },
                None => BcfMutation::NoMutation,
            },
            BcfMutation::InsertComment { topic_guid, comment } => BcfMutation::RemoveComment { topic_guid: topic_guid.clone(), guid: comment.guid.clone() },
            BcfMutation::RemoveComment { topic_guid, guid } => match find_comment(base, topic_guid, guid) {
                Some(comment) => BcfMutation::InsertComment { topic_guid: topic_guid.clone(), comment: comment.clone() },
                None => BcfMutation::NoMutation,
            },
            BcfMutation::SetComment { topic_guid, guid, date, author, text, viewpoint_ref } => match find_comment(base, topic_guid, guid) {
                Some(comment) => BcfMutation::SetComment {
                    topic_guid: topic_guid.clone(),
                    guid: guid.clone(),
                    date: date.as_ref().map(|_| comment.date.clone()),
                    author: author.as_ref().map(|_| comment.author.clone()),
                    text: text.as_ref().map(|_| comment.text.clone()),
                    viewpoint_ref: viewpoint_ref.as_ref().map(|_| comment.viewpoint_ref.clone()),
                },
                None => BcfMutation::NoMutation,
            },
            BcfMutation::InsertViewpoint { topic_guid, viewpoint } => BcfMutation::RemoveViewpoint { topic_guid: topic_guid.clone(), guid: viewpoint.guid.clone() },
            BcfMutation::RemoveViewpoint { topic_guid, guid } => match find_viewpoint(base, topic_guid, guid) {
                Some(viewpoint) => BcfMutation::InsertViewpoint { topic_guid: topic_guid.clone(), viewpoint: viewpoint.clone() },
                None => BcfMutation::NoMutation,
            },
            BcfMutation::SetViewpointCamera { topic_guid, guid, .. } => match find_viewpoint(base, topic_guid, guid) {
                Some(viewpoint) => BcfMutation::SetViewpointCamera { topic_guid: topic_guid.clone(), guid: guid.clone(), camera: viewpoint.camera.clone() },
                None => BcfMutation::NoMutation,
            },
            BcfMutation::SetViewpointComponents { topic_guid, guid, .. } => match find_viewpoint(base, topic_guid, guid) {
                Some(viewpoint) => BcfMutation::SetViewpointComponents { topic_guid: topic_guid.clone(), guid: guid.clone(), components: viewpoint.components.clone() },
                None => BcfMutation::NoMutation,
            },
            BcfMutation::SetViewpointSnapshot { topic_guid, guid, .. } => match find_viewpoint(base, topic_guid, guid) {
                Some(viewpoint) => BcfMutation::SetViewpointSnapshot { topic_guid: topic_guid.clone(), guid: guid.clone(), snapshot: viewpoint.snapshot.clone() },
                None => BcfMutation::NoMutation,
            },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_bcf(&mutable_input(ctx)?).map_err(|error| format!("decode_bcf failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let mut snapshot = base;
        apply_bcf_mutation(&mut snapshot, &mutation);
        let bytes = encode_bcf(&snapshot).map_err(|error| format!("encode_bcf failed: {error}"))?;
        let projection = project_bcf_2_1(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_bcf(&mutable_input(ctx)?).map_err(|error| format!("decode_bcf failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_bcf_mutation(&mut snapshot, &mutation);
        apply_bcf_mutation(&mut snapshot, &undo);
        let bytes = encode_bcf(&snapshot).map_err(|error| format!("encode_bcf failed: {error}"))?;
        let projection = project_bcf_2_1(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_bcf`/`encode_bcf` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_bcf(&input).map_err(|error| format!("decode_bcf failed: {error}"))?;
        let output = encode_bcf(&snapshot).map_err(|error| format!("encode_bcf failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_bcf_2_1(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 14-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 14 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
