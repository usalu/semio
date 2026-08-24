//! 🦀️ Semio GRAPH exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-graph-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️graph/🧪️oracle/🔣️component.json`): `s.stdio.semio.graph` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed, independently
//! handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/
//! 🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_semio_graph_mutation` over the full 11-kind `SemioGraphMutation` vocabulary.
//!
//! What distinguishes this subset is that identity is a NEWTYPE on the wire: a node is addressed
//! as `{"value": "n1"}`, not as a bare string, and an edge's `source`/`target` carry the same
//! wrapper. Ports and properties are POSITIONAL collections nested one level inside a node, so
//! `add-node-port`/`add-node-property` address `(node, index)` while `create-node`/`delete-node`
//! address the outer node set — and `delete-node` additionally severs every edge incident to the
//! node it removes, which is the one kind here whose effect reaches a collection it was not
//! addressed against.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never gets two sides to compare. Every law this case claims is therefore
//! asserted INSIDE the subject handler: `mutate-<kind>` checks the applied snapshot IS the committed
//! after-snapshot, `inverse-<kind>` checks the mutation's own computed inverse restores the
//! committed before-snapshot, and `identity-round-trip` crosses the two committed encodings of the
//! real artifact against each other. A handler that merely returned `Ok` would report a pass having
//! checked nothing at all.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this subset's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`), so neither
//! `protocol::Mutation` nor a `serde` derive is nameable from here. An earlier draft of this adapter
//! answered that by transcribing every fixture into a Rust literal beside it, which is exactly the
//! drift a specification-vector substitute cannot afford. The subset's own production code now
//! exports the bridges instead, whose signatures name only reachable types: `decode_semio_graph_snapshot_json`/
//! `encode_semio_graph_snapshot_json` and the DSL/pack pass-throughs (`../../🏅️standards/🔖️v1/🪆️subsets/
//! ✳️graph/🧬️schema/📸️snapshot/🦀️component.rs`), `decode_semio_graph_mutation_json`/`inverse_semio_graph_mutation`
//! (`…/🧬️mutations/🦀️component.rs`). Both roles read the SAME committed bytes — the oracle role via
//! `include_str!`, the subject role by decoding that same text. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never compiles the local implementation;
//! the Rust SUBJECT phase is blocked this wave by concurrent framework refactors (see
//! 📓️w7-fleet-brief.md), so it is written and gated but not run.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioGraphMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-node", "delete-node", "change-node-kind", "change-node-label", "move-node", "add-node-port", "remove-node-port", "add-node-property", "remove-node-property", "create-edge", "delete-edge"];

/// 🗣️ The real committed artifact — two nodes joined by one `out`→`in` edge, one carrying an integer property at the canvas origin and one carrying a string property at a negative coordinate.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/📚️examples/🕸️wires/🖼️assets/🗣️example.dsl.semio";
/// 🎒️ The same document in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️graph/📚️examples/🕸️wires/🖼️assets/🎒️example.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after)` fixture TEXT for one kind, read literally via
/// `include_str!` — this IS the independently handcrafted specification vector the no-oracle
/// decision rests on, never recomputed. One `include_str!` per file for the whole adapter: `oracle`
/// answers with `before`/`after`, `subject` decodes all three.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "create-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/🧪️tests/appends-a-filter-node-to-the-end-of-the-node-set/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/🧪️tests/appends-a-filter-node-to-the-end-of-the-node-set/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🏗️create-node/🧪️tests/appends-a-filter-node-to-the-end-of-the-node-set/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/removes-the-sink-node-and-severs-the-edge-into-it/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/removes-the-sink-node-and-severs-the-edge-into-it/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🗑️delete-node/🧪️tests/removes-the-sink-node-and-severs-the-edge-into-it/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-node-kind" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🧪️tests/retypes-the-source-node-without-relabelling-it/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🧪️tests/retypes-the-source-node-without-relabelling-it/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔧change-node-kind/🧪️tests/retypes-the-source-node-without-relabelling-it/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-node-label" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🧪️tests/relabels-the-source-node-without-retyping-it/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🧪️tests/relabels-the-source-node-without-retyping-it/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🖍️change-node-label/🧪️tests/relabels-the-source-node-without-retyping-it/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "move-node" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-sink-node-to-a-new-canvas-position/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-sink-node-to-a-new-canvas-position/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/📍move-node/🧪️tests/moves-the-sink-node-to-a-new-canvas-position/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-node-port" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/🧪️tests/inserts-an-in-port-ahead-of-the-existing-out-port/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/🧪️tests/inserts-an-in-port-ahead-of-the-existing-out-port/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔌add-node-port/🧪️tests/inserts-an-in-port-ahead-of-the-existing-out-port/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-node-port" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🧪️tests/detaches-the-trailing-out-port-from-the-source-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🧪️tests/detaches-the-trailing-out-port-from-the-source-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔚remove-node-port/🧪️tests/detaches-the-trailing-out-port-from-the-source-node/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-node-property" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/🧪️tests/inserts-a-weight-property-ahead-of-the-colour-property/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/🧪️tests/inserts-a-weight-property-ahead-of-the-colour-property/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➕add-node-property/🧪️tests/inserts-a-weight-property-ahead-of-the-colour-property/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-node-property" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/🧪️tests/detaches-the-trailing-weight-property-from-the-source-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/🧪️tests/detaches-the-trailing-weight-property-from-the-source-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/➖remove-node-property/🧪️tests/detaches-the-trailing-weight-property-from-the-source-node/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-edge" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/🧪️tests/connects-the-source-node-to-the-sink-node/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/🧪️tests/connects-the-source-node-to-the-sink-node/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/🔗create-edge/🧪️tests/connects-the-source-node-to-the-sink-node/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-edge" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/removes-the-feedback-edge-and-keeps-both-endpoints/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/removes-the-feedback-edge-and-keeps-both-endpoints/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️graph/🧬️schema/🧬️mutations/✂️delete-edge/🧪️tests/removes-the-feedback-edge-and-keeps-both-endpoints/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-graph: no fixture registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::mutations::{apply_semio_graph_mutation, decode_semio_graph_mutation_json, inverse_semio_graph_mutation, SemioGraphMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{decode_semio_graph_pack, decode_semio_graph_snapshot_json, encode_semio_graph_pack, encode_semio_graph_snapshot_json, parse_semio_graph_dsl, print_semio_graph_dsl, SemioGraphSnapshot};

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridges — real deserialization of the committed bytes,
    /// never a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<SemioGraphSnapshot, String> {
        decode_semio_graph_snapshot_json(text).map_err(|error| format!("mutate-semio-graph: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<SemioGraphMutation, String> {
        decode_semio_graph_mutation_json(text).map_err(|error| format!("mutate-semio-graph: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &SemioGraphSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_graph_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both fixtures are written
    /// in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioGraphSnapshot, expected: &SemioGraphSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_graph_snapshot_json(got), encode_semio_graph_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot. The assertion lives here rather than in the comparison because a recorded
    /// no-oracle case runs no oracle role: a handler that merely returned `Ok` would report a pass
    /// having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after) = super::fixture_text(kind);
            let mut current = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let outcome = apply_semio_graph_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("mutate-{kind}: the mutation was rejected: {:?}", outcome.messages()));
            }
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &current, &expected));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly — collection POSITION included, not merely
    /// membership, which is what a delete/create pair has to rebuild rather than re-append.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let outcome = apply_semio_graph_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {:?}", outcome.messages()));
            }
            for step in inverse_semio_graph_mutation(&mutation, &base) {
                let step_outcome = apply_semio_graph_mutation(&mut current, &step);
                if !step_outcome.messages().is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {:?}", step_outcome.messages()));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse-{kind}: undoing the mutation did not restore the before-snapshot"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed artifact through both of its committed encodings. The DSL text and the
    /// pack envelope are separate committed files produced by separate codecs, so agreeing on one
    /// snapshot cannot be achieved by smuggling bytes from either. Byte-identical re-emission IS
    /// expected here — the committed text is this codec's own output, not a foreign writer's — so
    /// the wave's usual "output must not equal input" tripwire does not apply and the text/binary
    /// cross-check carries that evidence instead.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed wires graph artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_graph_dsl(&text)?;
        if parsed.nodes.len() != 2 || parsed.edges.len() != 1 {
            return Err(format!("identity-round-trip: the committed wires graph is the two-node, one-edge fixture this case describes, but parsed {} node(s) and {} edge(s)", parsed.nodes.len(), parsed.edges.len()));
        }
        let reparsed = parse_semio_graph_dsl(&print_semio_graph_dsl(&parsed))?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let unpacked = decode_semio_graph_pack(&ctx.fixture_bytes(super::PACK_ASSET)?)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different document than the committed text artifact", &unpacked, &parsed));
        }
        let repacked = decode_semio_graph_pack(&encode_semio_graph_pack(&parsed))?;
        if repacked != parsed {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &parsed));
        }
        let projection = projection(&parsed)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. `identity-round-trip` is
/// deliberately subject-only: the reference answer for the other scenarios is a committed JSON
/// snapshot the oracle role can read literally, but the real artifact is committed as DSL and pack
/// bytes ONLY, and turning those into a snapshot needs this subset's own codec — which the
/// oracle-only build must not link. Inventing a JSON transcription of it here would be a second,
/// drifting copy of the artifact, so that scenario asserts entirely in-role instead.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
