//! 🦀️ Semio ENVELOPE exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-envelope-routing` (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️any/🧪️oracle/🔣️.json`): `s.stdio.semio` is the ENVELOPE union over all
//! eighteen semio subsets and has no third-party reader or writer, so `oracle` here answers with the
//! routing outcome the feature's own `Then` steps declare — envelope subset tag, fault codes and
//! whether the resulting document still equals the one the scenario started from — while `subject`
//! drives this repository's own `apply_semio_mutation`/`inverse_semio_mutation` and reports the same
//! four facts measured off real typed values. `ordered-json-v1` compares them.
//!
//! The projection is deliberately envelope-level. This subset does not own its eighteen arms'
//! payload semantics — those are handcrafted in each arm's own `🧬️mutations/<kind>/🧪️tests/` leaf
//! and measured by that arm's own case — it owns the ROUTING: match threads through, mismatch is
//! refused with `mutation.target-missing`, and only `set-snapshot` may retype the envelope. Reading
//! any arm's contents back would need a decoder for all eighteen snapshot types and would re-test
//! work that already has its own coverage; `matchesReference` (derived from `SemioSnapshot`'s own
//! derived `PartialEq`) is what proves the delegated verb genuinely reached the arm and changed it,
//! and its inverse genuinely put it back.
//!
//! The subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation; the Rust SUBJECT phase RUNS. The os-kernel blocker earlier waves
//! recorded here was cleared on 2026-08-24 — `cargo check -p semio-framework-os-kernel --lib` exits 0 and
//! `semio-s-plugin-stdio` builds — so `bun ./📜️script.ts subject exhaustive --owner 🗄️stdio --case
//! mutate-semio-any` really executes every scenario below. The gate keeps the two BUILDS apart; it has
//! never been a reason the subject half goes unmeasured, and for this recorded no-oracle case the subject
//! phase is the only phase that runs at all.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never receives two sides to compare and the `oracle` handlers below are
//! the written statement of the reference answer rather than a second running party. Every law this
//! case claims is therefore asserted INSIDE the subject handler, which fails with both documents
//! printed. A handler that merely ran the mutation and returned would report a pass having checked
//! nothing.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// both the enum and the envelope's own runtime subset tag.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "brep", "mesh", "model", "value", "document", "cad", "drawing", "image", "video", "audio", "animation", "presentation", "flow", "text", "table", "graph", "object", "kit"];

/// 🌐️ The envelope schema id every projection carries — the value `SemioSnapshot::default()` and the
/// committed leaf fixture both use.
const ENVELOPE_SCHEMA: &str = "stdio.semio";

/// 🌐️ The envelope's OWN committed real artifact, in both of its committed encodings — the byte
/// carriers `identity-round-trip` measures alongside the typed full-replace routing law.
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌐️envelope/🖼️assets/🗣️example.dsl.semio";
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌐️envelope/🖼️assets/🎒️example.pack.semio";

/// 📄️ The committed `(before, mutation, after, diff)` specification vector for the two
/// envelope-owned verbs. Its arm is the `value` subset.
const LEAF_DIR: &str = "🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset";
const LEAF_SUBSET: &str = "value";

fn leaf_before_uri() -> String {
    format!("asset://{LEAF_DIR}/📸️snapshot/⬅️before/🔣️component.json")
}
fn leaf_mutation_uri() -> String {
    format!("asset://{LEAF_DIR}/🦠️mutation/🔣️component.json")
}
//#endregion 🔖️Kinds

//#region 🔖️Projection
/// 🎯️ The one projection every scenario compares: the four envelope-level facts, in the shape the
/// feature's `Then` steps state in prose.
fn routing_json(subset: &str, diagnostics: &[String], matches_reference: bool) -> Json {
    Json::Object(vec![
        ("schema".to_string(), Json::String(ENVELOPE_SCHEMA.to_string())),
        ("subset".to_string(), Json::String(subset.to_string())),
        ("diagnostics".to_string(), Json::Array(diagnostics.iter().map(|code| Json::String(code.clone())).collect())),
        ("matchesReference".to_string(), Json::Bool(matches_reference)),
    ])
}
fn outcome_of(subset: &str, diagnostics: &[String], matches_reference: bool) -> Outcome {
    let projection = routing_json(subset, diagnostics, matches_reference);
    let bytes = projection.to_string().into_bytes();
    Outcome::with_raw(bytes, projection)
}
//#endregion 🔖️Projection

//#region 🔖️Law
/// 📐️ The routing law each scenario's own `Then` steps declare, stated ONCE for both roles: the
/// envelope's subset tag after routing, the fault codes raised, and whether the routed document
/// still equals the one the scenario started from. The `oracle` role answers with it; the `subject`
/// role is CHECKED against it. Because this case records a no-oracle decision the runner executes no
/// oracle role at all, so a subject handler that merely reported its measurements would report a
/// pass having compared them with nothing — that is why the law lives here rather than in the
/// comparison profile.
fn expected_routing(scenario: &str, kind: &str) -> (String, Vec<String>, bool) {
    match scenario {
        // ▶️ A routed forward verb reaches its arm and changes it, raising nothing. `no-mutation` is
        // the identity, so it alone still equals the document it started from. Only the two
        // envelope-owned verbs run against the committed `value` leaf; every delegated verb runs
        // against an envelope of its own subset.
        "mutate" => (envelope_arm(kind), Vec::new(), kind == "no-mutation"),
        // ↩️ The inverse law: whatever the forward verb did, its own computed inverse puts back,
        // leaving the subset tag untouched and raising nothing.
        "inverse" => (envelope_arm(kind), Vec::new(), true),
        // 🚫️ A wrapped `image` verb against a `value` envelope is refused with
        // `mutation.target-missing`, leaving the document exactly as it stood.
        "rejects-a-mismatched-arm" => (LEAF_SUBSET.to_string(), vec!["mutation.target-missing".to_string()], true),
        // 🔁️ `set-snapshot` is the ONE verb that may change the subset kind, so the envelope comes
        // back carrying `image` and no longer equals what it was.
        "set-snapshot-changes-the-subset-kind" => ("image".to_string(), Vec::new(), false),
        // 🔁️ Rebuilding the committed envelope from an empty one must land on that same envelope.
        "identity-round-trip" => (LEAF_SUBSET.to_string(), Vec::new(), true),
        other => panic!("mutate-semio-any: no declared routing law for scenario {other:?}"),
    }
}

/// 🏷️ The subset tag the envelope must carry after a scenario's verb. Only the two envelope-OWNED
/// verbs (`no-mutation`, `set-snapshot`) run against the committed `value` leaf; for every one of
/// the eighteen delegated verbs the kind name IS the arm name, which is the routing identity this
/// case exists to pin.
fn envelope_arm(kind: &str) -> String {
    if kind == "no-mutation" || kind == "set-snapshot" {
        LEAF_SUBSET.to_string()
    } else {
        kind.to_string()
    }
}
//#endregion 🔖️Law

//#region 🔖️Oracle
/// 🔮️ The reference answer for a routed forward mutation, read straight off the declared law.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (subset, raised, matches) = expected_routing("mutate", kind);
        Ok(outcome_of(&subset, &raised, matches))
    }
}

/// 🔮️ The reference answer for the inverse law, read straight off the declared law.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (subset, raised, matches) = expected_routing("inverse", kind);
        Ok(outcome_of(&subset, &raised, matches))
    }
}

/// 🔮️ The reference answer for the mismatch law.
fn mismatch_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (subset, raised, matches) = expected_routing("rejects-a-mismatched-arm", "");
    Ok(outcome_of(&subset, &raised, matches))
}

/// 🔮️ The reference answer for the retyping law.
fn retype_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (subset, raised, matches) = expected_routing("set-snapshot-changes-the-subset-kind", "");
    Ok(outcome_of(&subset, &raised, matches))
}

/// 🔮️ The reference answer for the completeness law.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (subset, raised, matches) = expected_routing("identity-round-trip", "");
    Ok(outcome_of(&subset, &raised, matches))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{expected_routing, leaf_before_uri, leaf_mutation_uri, outcome_of, routing_json};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{decode_semio_envelope_pack, encode_semio_envelope_pack, parse_semio_envelope_dsl, print_semio_envelope_dsl};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusal_codes;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint2, SemioPoint3};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::{
        apply_semio_mutation, apply_audio as any_audio, apply_brep as any_brep, apply_cad as any_cad, apply_document as any_document, apply_drawing as any_drawing, apply_flow as any_flow, apply_graph as any_graph, apply_image as any_image, inverse_semio_mutation, apply_kit as any_kit, apply_mesh as any_mesh,
        apply_model as any_model, apply_object as any_object, apply_presentation as any_presentation, semio_subset_tag, set_snapshot as any_set_snapshot, apply_table as any_table, apply_text as any_text, apply_value as any_value, apply_video as any_video, SemioMutation,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::apply_animation as any_animation;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};

    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::animation::schema::{mutations::{insert_timeline, SemioAnimationMutation}, snapshot::AnimTimeline};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_vertex, SemioBrepMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::cad::schema::{mutations::SemioCadMutation, snapshot::CadLayer};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::{mutations::SemioDocumentMutation, snapshot::DocStyle};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::schema::{
        mutations::{create_layer, SemioDrawingMutation},
        snapshot::DrawLayer,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::{mutations::{insert_node, SemioFlowMutation}, snapshot::FlowNode};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::{
        mutations::{create_node, SemioGraphMutation},
        snapshot::GraphNodeId,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{add_type, SemioKitMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::{
        mutations::{create_mesh, SemioMeshMutation},
        snapshot::SemioMesh,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::model::schema::{mutations::SemioModelMutation, snapshot::SpatialNode};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::object::schema::mutations::{move_object, SemioObjectMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::{mutations::{insert_slide, SemioPresentationMutation}, snapshot::Slide};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::{
        mutations::{create_column, SemioTableMutation},
        snapshot::SemioTableCellKind,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::{
        mutations::{insert_run, SemioTextMutation},
        snapshot::SemioTextRun,
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::{
        mutations::{set_node, SemioValueMutation},
        snapshot::{SemioValue, ValueId},
    };
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::video::schema::{mutations::SemioVideoMutation, snapshot::SemioVideoStream};

    //#region 🔖️Arms
    /// 🪆️ An envelope wrapping one arm's OWN empty snapshot — the document each delegating scenario
    /// starts from. No arm's contents are ever read back, so no snapshot decoder is needed.
    fn empty_envelope(subset: &str) -> SemioSnapshot {
        let arm = match subset {
            "brep" => SemioSubsetSnapshot::Brep(Default::default()),
            "mesh" => SemioSubsetSnapshot::Mesh(Default::default()),
            "model" => SemioSubsetSnapshot::Model(Default::default()),
            "value" => SemioSubsetSnapshot::Value(Default::default()),
            "document" => SemioSubsetSnapshot::Document(Default::default()),
            "cad" => SemioSubsetSnapshot::Cad(Default::default()),
            "drawing" => SemioSubsetSnapshot::Drawing(Default::default()),
            "image" => SemioSubsetSnapshot::Image(Default::default()),
            "video" => SemioSubsetSnapshot::Video(Default::default()),
            "audio" => SemioSubsetSnapshot::Audio(Default::default()),
            "animation" => SemioSubsetSnapshot::Animation(Default::default()),
            "presentation" => SemioSubsetSnapshot::Presentation(Default::default()),
            "flow" => SemioSubsetSnapshot::Flow(Default::default()),
            "text" => SemioSubsetSnapshot::Text(Default::default()),
            "table" => SemioSubsetSnapshot::Table(Default::default()),
            "graph" => SemioSubsetSnapshot::Graph(Default::default()),
            "object" => SemioSubsetSnapshot::Object(Default::default()),
            "kit" => SemioSubsetSnapshot::Kit(Default::default()),
            other => panic!("mutate-semio-any: no empty envelope for subset {other:?}"),
        };
        SemioSnapshot { subset: arm, ..Default::default() }
    }

    /// 🚚 One real, minimal verb per arm — the smallest mutation each subset's own vocabulary offers
    /// that genuinely changes an empty document of that subset, so `matchesReference` turning false
    /// is evidence the envelope routed it all the way in rather than swallowing it.
    fn delegated(subset: &str) -> SemioMutation {
        match subset {
            "brep" => SemioMutation::ApplyBrep(any_brep::ApplyBrep { mutation: SemioBrepMutation::CreateVertex(create_vertex::mutation::CreateVertex { id: "v-probe".into(), point: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } }) }),
            "mesh" => SemioMutation::ApplyMesh(any_mesh::ApplyMesh { mutation: SemioMeshMutation::CreateMesh(create_mesh::mutation::CreateMesh { mesh: SemioMesh { id: "m-probe".into(), ..Default::default() } }) }),
            "model" => SemioMutation::ApplyModel(any_model::ApplyModel { mutation: SemioModelMutation::InsertSpatialNode { node: SpatialNode { id: "s-probe".into(), ..Default::default() } } }),
            "value" => SemioMutation::ApplyValue(any_value::ApplyValue { mutation: SemioValueMutation::SetNode(set_node::SetNode { id: ValueId::new("n-probe"), value: SemioValue::Str { value: "probe".into() } }) }),
            "document" => SemioMutation::ApplyDocument(any_document::ApplyDocument { mutation: SemioDocumentMutation::InsertStyle { style: DocStyle { id: "st-probe".into(), ..Default::default() } } }),
            "cad" => SemioMutation::ApplyCad(any_cad::ApplyCad { mutation: SemioCadMutation::AddLayer { layer: CadLayer { name: "L-PROBE".into(), ..Default::default() } } }),
            "drawing" => SemioMutation::ApplyDrawing(any_drawing::ApplyDrawing { mutation: SemioDrawingMutation::CreateLayer(create_layer::mutation::CreateLayer { index: 0, layer: DrawLayer { id: "dl-probe".into(), ..Default::default() } }) }),
            "image" => SemioMutation::ApplyImage(any_image::ApplyImage { mutation: SemioImageMutation::SetDimensions { width: 4, height: 2 } }),
            "video" => SemioMutation::ApplyVideo(any_video::ApplyVideo { mutation: SemioVideoMutation::InsertStream { index: 0, stream: SemioVideoStream { codec: "probe".into(), ..Default::default() } } }),
            "audio" => SemioMutation::ApplyAudio(any_audio::ApplyAudio { mutation: SemioAudioMutation::SetSampleRate { sample_rate: 48_000 } }),
            "animation" => SemioMutation::ApplyAnimation(any_animation::ApplyAnimation { mutation: SemioAnimationMutation::InsertTimeline(insert_timeline::InsertTimeline { index: 0, timeline: AnimTimeline { name: Some("probe".into()), ..Default::default() } }) }),
            "presentation" => SemioMutation::ApplyPresentation(any_presentation::ApplyPresentation { mutation: SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: 0, slide: Slide { id: "sl-probe".into(), ..Default::default() } }) }),
            "flow" => SemioMutation::ApplyFlow(any_flow::ApplyFlow { mutation: SemioFlowMutation::InsertNode(insert_node::InsertNode { node: FlowNode { id: "fn-probe".into(), ..Default::default() } }) }),
            "text" => SemioMutation::ApplyText(any_text::ApplyText { mutation: SemioTextMutation::InsertRun(insert_run::mutation::InsertRun { index: 0, run: SemioTextRun { language: "en".into(), content: "probe".into(), marks: Vec::new() } }) }),
            "table" => SemioMutation::ApplyTable(any_table::ApplyTable { mutation: SemioTableMutation::CreateColumn(create_column::mutation::CreateColumn { name: "probe".into(), kind: SemioTableCellKind::Str, index: None }) }),
            "graph" => SemioMutation::ApplyGraph(any_graph::ApplyGraph {
                mutation: SemioGraphMutation::CreateNode(create_node::mutation::CreateNode {
                    id: GraphNodeId { value: "gn-probe".into() },
                    kind: "probe".into(),
                    label: "Probe".into(),
                    position: SemioPoint2 { x: 0.0, y: 0.0 },
                    ports: Vec::new(),
                    properties: Vec::new(),
                }),
            }),
            "object" => SemioMutation::ApplyObject(any_object::ApplyObject { mutation: SemioObjectMutation::MoveObject(move_object::mutation::MoveObject { translation: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }) }),
            "kit" => SemioMutation::ApplyKit(any_kit::ApplyKit { mutation: SemioKitMutation::AddType(add_type::mutation::AddType { id: "kt-probe".into(), name: "Probe".into(), category: "probe".into() }) }),
            other => panic!("mutate-semio-any: no delegated verb for subset {other:?}"),
        }
    }
    //#endregion 🔖️Arms

    //#region 🔖️Decode
    /// 🧫️ The committed leaf vector's envelope carries the `value` arm, so only that one arm needs a
    /// structural decoder — a mechanical field-by-field read of the committed file's own serde
    /// shape, never a reimplementation of mutation semantics.
    fn bytes_field(json: &Json, key: &str) -> Vec<u8> {
        json.array(key)
            .iter()
            .map(|entry| match entry {
                Json::Number(value) => *value as u8,
                other => panic!("mutate-semio-any: expected a byte number, found {other:?}"),
            })
            .collect()
    }
    fn decode_id(json: &Json) -> ValueId {
        ValueId::new(json.str("value"))
    }
    fn decode_value(json: &Json) -> SemioValue {
        use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;
        match json.str("kind").as_str() {
            "null" => SemioValue::Null,
            "bool" => SemioValue::Bool { value: matches!(json.get("value"), Some(Json::Bool(true))) },
            "int" => SemioValue::Int { lexeme: json.str("lexeme") },
            "float" => SemioValue::Float { lexeme: json.str("lexeme") },
            "str" => SemioValue::Str { value: json.str("value") },
            "bytes" => SemioValue::Bytes { value: bytes_field(json, "value") },
            "list" => SemioValue::List { items: json.array("items").iter().map(decode_value).collect() },
            "map" => SemioValue::Map {
                entries: json
                    .array("entries")
                    .iter()
                    .map(|entry| SemioValueEntry { key: entry.str("key"), value: decode_value(entry.get("value").expect("mutate-semio-any: a map entry must carry a value")) })
                    .collect(),
            },
            "ref" => SemioValue::Ref { id: decode_id(json.get("id").expect("mutate-semio-any: a ref value must carry an id")) },
            other => panic!("mutate-semio-any: unknown value kind {other:?}"),
        }
    }
    fn decode_value_snapshot(json: &Json) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot {
        use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValueNode, SemioValueSnapshot};
        SemioValueSnapshot {
            schema: json.str("schema"),
            root: decode_value(json.get("root").expect("mutate-semio-any: a value snapshot must carry a root")),
            nodes: json
                .array("nodes")
                .iter()
                .map(|node| SemioValueNode { id: decode_id(node.get("id").expect("mutate-semio-any: a graph node must carry an id")), value: decode_value(node.get("value").expect("mutate-semio-any: a graph node must carry a value")) })
                .collect(),
        }
    }
    fn decode_envelope(json: &Json) -> SemioSnapshot {
        let arm = json.get("subset").expect("mutate-semio-any: an envelope must carry a subset");
        let subset = match arm.str("subset").as_str() {
            "value" => SemioSubsetSnapshot::Value(decode_value_snapshot(arm)),
            other => panic!("mutate-semio-any: the committed leaf vector carries the value arm; no decoder for {other:?}"),
        };
        SemioSnapshot { schema: json.str("schema"), subset }
    }
    fn decode_envelope_mutation(json: &Json) -> SemioMutation {
        match json.str("mutation").as_str() {
            "setSnapshot" => SemioMutation::SetSnapshot(any_set_snapshot::SetSnapshot { snapshot: decode_envelope(json.get("payload").expect("mutate-semio-any: setSnapshot must carry a payload").get("snapshot").expect("mutate-semio-any: setSnapshot's payload must carry a snapshot")) }),
            other => panic!("mutate-semio-any: the committed leaf vector declares only the envelope-owned verb setSnapshot; no decoder for {other:?}"),
        }
    }
    //#endregion 🔖️Decode

    //#region 🔖️Apply
    /// ▶️ Applies `mutation` to `base`, returning the routed envelope and the fault codes raised.
    fn apply(base: &SemioSnapshot, mutation: &SemioMutation) -> (SemioSnapshot, Vec<String>) {
        let mut current = base.clone();
        let outcome = apply_semio_mutation(&mut current, mutation);
        let raised = semio_mutation_refusal_codes(&outcome);
        (current, raised)
    }
    //#endregion 🔖️Apply

    //#region 🔖️Handlers
    fn leaf_document(ctx: &Context) -> Result<SemioSnapshot, String> {
        Ok(decode_envelope(&ctx.fixture_json(&leaf_before_uri())?))
    }

    /// `"no-mutation"` maps to the identity mutation `SetSnapshot(set_snapshot::SetSnapshot {
    /// snapshot: base.clone() })` — the retained `no-mutation` scenario id's convention — rather
    /// than through `decode_envelope_mutation`, which no longer has an arm for it.
    fn envelope_verb(kind: &str, ctx: &Context) -> Result<(SemioSnapshot, SemioMutation), String> {
        let base = leaf_document(ctx)?;
        let mutation = if kind == "no-mutation" {
            SemioMutation::SetSnapshot(any_set_snapshot::SetSnapshot { snapshot: base.clone() })
        } else {
            decode_envelope_mutation(&ctx.fixture_json(&leaf_mutation_uri())?)
        };
        Ok((base, mutation))
    }

    fn scenario_input(kind: &str, ctx: &Context) -> Result<(SemioSnapshot, SemioMutation), String> {
        if kind == "no-mutation" || kind == "set-snapshot" {
            envelope_verb(kind, ctx)
        } else {
            Ok((empty_envelope(kind), delegated(kind)))
        }
    }

    /// ⚖️ Measures the four envelope-level facts and checks them against the routing law
    /// `../🦀️component.rs::expected_routing` states for this scenario, failing with both
    /// projections printed. Every handler below goes through here, so no scenario in this case can
    /// pass by producing a result nobody looked at.
    fn checked(scenario: &str, kind: &str, subset: &str, raised: &[String], matches: bool) -> Result<Outcome, String> {
        let (want_subset, want_raised, want_matches) = expected_routing(scenario, kind);
        if subset != want_subset.as_str() || raised != want_raised.as_slice() || matches != want_matches {
            return Err(format!(
                "{scenario}-{kind}: the envelope routed differently from the law this feature declares\n     got: {}\nexpected: {}",
                routing_json(subset, raised, matches).to_string(),
                routing_json(&want_subset, &want_raised, want_matches).to_string()
            ));
        }
        Ok(outcome_of(subset, raised, matches))
    }

    /// 🎯️ A routed forward verb must reach its arm, change it, raise nothing, and leave the
    /// envelope tagged with the arm it was routed to.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation) = scenario_input(kind, ctx)?;
            let (routed, raised) = apply(&base, &mutation);
            checked("mutate", kind, semio_subset_tag(&routed), &raised, routed == base)
        }
    }

    /// ↩️ The inverse law at envelope level: whatever the routed verb did to its arm, the mutation's
    /// own computed inverse must put back, restoring the whole envelope — arm contents included,
    /// since `matchesReference` is `SemioSnapshot`'s own derived `PartialEq`.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (base, mutation) = scenario_input(kind, ctx)?;
            let (mut current, mut raised) = apply(&base, &mutation);
            for step in &inverse_semio_mutation(&mutation, &base) {
                let (next, more) = apply(&current, step);
                current = next;
                raised.extend(more);
            }
            checked("inverse", kind, semio_subset_tag(&current), &raised, current == base)
        }
    }

    /// 🚫️ The mismatch law: a wrapped `image` verb against a `value` envelope must be refused with
    /// `mutation.target-missing`, leaving the document exactly as it stood.
    pub fn mismatch(ctx: &Context) -> Result<Outcome, String> {
        let base = leaf_document(ctx)?;
        let (routed, raised) = apply(&base, &SemioMutation::ApplyImage(any_image::ApplyImage { mutation: SemioImageMutation::SetDimensions { width: 4, height: 2 } }));
        checked("rejects-a-mismatched-arm", "", semio_subset_tag(&routed), &raised, routed == base)
    }

    /// 🔁️ The retyping law: `set-snapshot` is the only verb that may change the subset kind.
    pub fn retype(ctx: &Context) -> Result<Outcome, String> {
        let base = leaf_document(ctx)?;
        let (routed, raised) = apply(&base, &SemioMutation::SetSnapshot(any_set_snapshot::SetSnapshot { snapshot: empty_envelope("image") }));
        checked("set-snapshot-changes-the-subset-kind", "", semio_subset_tag(&routed), &raised, routed == base)
    }

    /// 🔁️ The completeness law: the envelope's own full-replace verb must carry an empty envelope all
    /// the way to the committed document.
    /// 🔒️ **The byte half of the identity law, and where it lives for the envelope.** The routing
    /// check above moves no bytes at all — it compares typed values — so this handler also reads
    /// the envelope's OWN committed real artifact in both of its committed encodings and asserts
    /// `carrier_is_exact` on each. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio`
    /// its binary twin; both committed files were produced by these very codecs, so reproducing
    /// them byte for byte is the CORRECT answer and `law::reparsed_not_copied` would be exactly
    /// backwards — the same reading `mutate-dag-1` records for `.dag.dsl.semio`. The two carriers
    /// also cross-check each other: the pack must decode to the same envelope the text does, which
    /// no single codec can arrange on its own, and the envelope carries a real ARM (the committed
    /// example wraps a `flow` document), so the delegation the rest of this case measures on typed
    /// values is exercised here on bytes.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = leaf_document(ctx)?;
        let (rebuilt, raised) = apply(&SemioSnapshot::default(), &SemioMutation::SetSnapshot(any_set_snapshot::SetSnapshot { snapshot: committed.clone() }));
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed envelope artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_envelope_dsl(&text)?;
        let printed = print_semio_envelope_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), text.as_bytes())?;
        let pack_bytes = ctx.fixture_bytes(super::PACK_ASSET)?;
        let unpacked = decode_semio_envelope_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(format!(
                "identity-round-trip: the committed binary twin decodes to a different envelope than the committed text artifact\n     got: {}\nexpected: {}",
                semio_subset_tag(&unpacked),
                semio_subset_tag(&parsed)
            ));
        }
        let repacked_bytes = encode_semio_envelope_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        checked("identity-round-trip", "", semio_subset_tag(&rebuilt), &raised, rebuilt == committed)
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so every kind is registered in a loop over `KINDS`.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("rejects-a-mismatched-arm", mismatch_oracle).oracle("set-snapshot-changes-the-subset-kind", retype_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("rejects-a-mismatched-arm", subject::mismatch).subject("set-snapshot-changes-the-subset-kind", subject::retype).subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
