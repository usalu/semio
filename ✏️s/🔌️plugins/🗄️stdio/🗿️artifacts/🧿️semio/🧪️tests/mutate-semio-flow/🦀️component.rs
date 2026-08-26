//! 🦀️ Semio FLOW exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-flow-mutate` is the
//! registered oracle `semio-flow-python-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️flow/
//! 🧪️oracle/🔣️component.json`) — an independent Python implementation of the semio flow carrier and
//! its thirteen verbs, written from the committed grammar, protocol and specification vectors,
//! living beside this file as `🐍️component.py`. The runner dispatches the oracle role to that adapter
//! and the subject role here, and compares the two projections under `@comparison-ordered-json-v1`.
//! Registering oracle handlers here as well would put this repository's own answer on both sides of
//! that comparison, which is the precise failure the platform exists to prevent.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! capsule network, `spec-vector-<kind>` requires the applied snapshot to be the vector's committed
//! after-snapshot AND the undone one to be its before-snapshot, and `identity-round-trip` requires
//! all four committed encodings to be reproduced byte for byte through `law::carrier_is_exact`.
//!
//! **How the fixtures reach typed values.** The generated test host links only `semio-repo-test-host`
//! and, behind `sut`, this subset's own crate — no `serde`, no `serde_json` — so the subset's own
//! production code exports the bridges this adapter needs, whose signatures name only reachable
//! types: `decode_semio_flow_snapshot_json`/`encode_semio_flow_snapshot_json`,
//! `decode_semio_flow_mutation_json`/`inverse_semio_flow_mutation`, and the DSL/pack pass-throughs.
//! Every input is read from a fixture the FEATURE declares — the mutation parameters from the
//! scenario's doc string, the specification vectors from the `local://` URI its step names — so
//! neither adapter holds a transcription that could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioFlowMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-node",
    "remove-node",
    "set-node-kind",
    "set-node-label",
    "set-node-position",
    "set-node-param",
    "remove-node-param",
    "insert-edge",
    "remove-edge",
    "set-edge-endpoints",
    "set-edge-kind",
];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{apply_semio_flow_mutation, decode_semio_flow_mutation_json, inverse_semio_flow_mutation, SemioFlowMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{
        decode_semio_flow_pack, decode_semio_flow_snapshot_json, encode_semio_flow_pack, encode_semio_flow_snapshot_json, parse_semio_flow_dsl, print_semio_flow_dsl, SemioFlowSnapshot,
    };
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

    //#region 🔖️Input
    /// 🌊️ The two-node demo pipeline, in both encodings the domain commits for it — small, but the
    /// only `stdio.semio.flow` bytes in this artifact a codec other than the Python one wrote.
    const PIPELINE_DSL: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio";
    const PIPELINE_PACK: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🎒️example.pack.semio";
    /// 🏗️ The real 180-node, 179-edge capsule connection network and its binary twin, derived once
    /// from the committed Nakagin Capsule Tower IFC with IfcOpenShell.
    const TOWER_DSL: &str = "local://🏗️nakagin-capsule-tower.dsl.semio";
    const TOWER_PACK: &str = "local://🏗️nakagin-capsule-tower.pack.semio";

    fn utf8(bytes: Vec<u8>, what: &str) -> Result<String, String> {
        String::from_utf8(bytes).map_err(|error| format!("{what} is not UTF-8: {error}"))
    }

    /// 🏗️ The real capsule network, parsed through this repository's own DSL codec.
    fn tower(ctx: &Context) -> Result<SemioFlowSnapshot, String> {
        parse_semio_flow_dsl(&utf8(ctx.fixture_bytes(TOWER_DSL)?, "the committed capsule network")?)
    }

    /// 📜️ The scenario's own committed mutation parameters — the feature owns the vector.
    fn mutation(ctx: &Context) -> Result<SemioFlowMutation, String> {
        decode_semio_flow_mutation_json(ctx.doc_string()?).map_err(|error| format!("{}: the scenario's mutation payload must decode: {error}", ctx.scenario.id))
    }

    /// 🧫️ The first `local://` URI the scenario's steps name. The feature is the single place a
    /// specification-vector path is written down; both adapters read it from there.
    fn step_vector(ctx: &Context) -> Result<Json, String> {
        for (_, text) in &ctx.scenario.steps {
            if let Some(at) = text.find("local://") {
                let tail = &text[at..];
                let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
                return ctx.fixture_json(&tail[..end]);
            }
        }
        Err(format!("{}: the scenario names no local:// specification vector", ctx.scenario.id))
    }

    fn member(vector: &Json, name: &str) -> Result<String, String> {
        Ok(vector.get(name).ok_or_else(|| format!("the committed specification vector carries no {name:?} member"))?.to_string())
    }

    fn apply(current: &mut SemioFlowSnapshot, step: &SemioFlowMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_flow_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }

    fn projection(snapshot: &SemioFlowSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_flow_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioFlowSnapshot, expected: &SemioFlowSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_flow_snapshot_json(got), encode_semio_flow_snapshot_json(expected))
    }
    //#endregion 🔖️Input

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real 180-node capsule network by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = tower(ctx)?;
        apply(&mut current, &mutation(ctx)?, &ctx.scenario.id)?;
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(print_semio_flow_dsl(&current).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law on the real network: applying the verb and then its OWN
    /// computed inverse must restore it exactly — node and edge ORDER, param order and the `f64`
    /// coordinates included, not merely a graph with the same members.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = tower(ctx)?;
        let step = mutation(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = projection(&current)?;
        for undo in inverse_semio_flow_mutation(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the capsule network", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), projection(&current)?)])))
    }

    /// 🧫️ The same verb on its committed `(before, mutation, after)` vector, whose before-snapshot is
    /// the real demo pipeline artifact decoded — a THIRD statement of what the verb means,
    /// independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let vector = step_vector(ctx)?;
        let base = decode_semio_flow_snapshot_json(&member(&vector, "before")?)?;
        let expected = decode_semio_flow_snapshot_json(&member(&vector, "after")?)?;
        let step = decode_semio_flow_mutation_json(&member(&vector, "mutation")?)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied flow does not match the vector's after-snapshot", ctx.scenario.id), &current, &expected));
        }
        let applied = projection(&current)?;
        for undo in inverse_semio_flow_mutation(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the vector's mutation did not restore its before-snapshot", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("applied".to_string(), applied), ("restored".to_string(), projection(&current)?)])))
    }

    /// 🔁️ All four committed encodings — the demo pipeline's two and the capsule network's two —
    /// each re-emitted from the parsed document.
    ///
    /// 🔒️ **The byte half of the identity law, asserted as `carrier_is_exact` and asserted in both
    /// directions.** `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary
    /// twin, so reproducing them BYTE FOR BYTE is the correct answer here and
    /// `law::reparsed_not_copied` would be exactly backwards. Nor is it a self-comparison: the demo
    /// pipeline's bytes were written by THIS codec and the Python oracle reproduces them from the
    /// grammar alone, while the capsule network's bytes were written by the PYTHON implementation and
    /// this codec has to reproduce THOSE — including 360 little-endian `f64` coordinates read from a
    /// real IFC model, which is the sharpest test of the pack frame in the case.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let pipeline_dsl = ctx.fixture_bytes(PIPELINE_DSL)?;
        let pipeline = parse_semio_flow_dsl(&utf8(pipeline_dsl.clone(), "the committed demo pipeline")?)?;
        let pipeline_printed = print_semio_flow_dsl(&pipeline);
        carrier_is_exact(pipeline_printed.as_bytes(), &pipeline_dsl)?;
        let pipeline_pack = ctx.fixture_bytes(PIPELINE_PACK)?;
        let pipeline_unpacked = decode_semio_flow_pack(&pipeline_pack)?;
        if pipeline_unpacked != pipeline {
            return Err(disagreement("identity-round-trip: the demo pipeline's binary twin decodes to a different flow than its text", &pipeline_unpacked, &pipeline));
        }
        let pipeline_repacked = encode_semio_flow_pack(&pipeline);
        carrier_is_exact(&pipeline_repacked, &pipeline_pack)?;
        let tower_dsl = ctx.fixture_bytes(TOWER_DSL)?;
        let network = parse_semio_flow_dsl(&utf8(tower_dsl.clone(), "the committed capsule network")?)?;
        let tower_printed = print_semio_flow_dsl(&network);
        carrier_is_exact(tower_printed.as_bytes(), &tower_dsl)?;
        let reparsed = parse_semio_flow_dsl(&tower_printed)?;
        if reparsed != network {
            return Err(disagreement("identity-round-trip: printing the capsule network back to DSL and reparsing it lost content", &reparsed, &network));
        }
        let tower_pack = ctx.fixture_bytes(TOWER_PACK)?;
        let tower_unpacked = decode_semio_flow_pack(&tower_pack)?;
        if tower_unpacked != network {
            return Err(disagreement("identity-round-trip: the capsule network's binary twin decodes to a different flow than its text", &tower_unpacked, &network));
        }
        let tower_repacked = encode_semio_flow_pack(&network);
        carrier_is_exact(&tower_repacked, &tower_pack)?;
        Ok(Outcome::projection(Json::Object(vec![
            ("pipeline".to_string(), projection(&pipeline)?),
            ("pipelineDslDigest".to_string(), Json::String(digest(pipeline_printed.as_bytes()))),
            ("pipelinePackDigest".to_string(), Json::String(digest(&pipeline_repacked))),
            ("towerDslDigest".to_string(), Json::String(digest(tower_printed.as_bytes()))),
            ("towerPackDigest".to_string(), Json::String(digest(&tower_repacked))),
            ("towerNodes".to_string(), Json::Number(network.nodes.len() as f64)),
            ("towerEdges".to_string(), Json::Number(network.edges.len() as f64)),
            ("towerDslLength".to_string(), Json::Number(tower_printed.len() as f64)),
            ("towerPackLength".to_string(), Json::Number(tower_repacked.len() as f64)),
        ])))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Only subject handlers are
/// registered: the oracle role belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
