//! 🦀️ Semio FLOW exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-flow-mutation-semantics` (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️flow/🧪️oracle/🔣️component.json`): `s.stdio.semio.flow` is a semio-NATIVE format with
//! no third-party reader or writer in any ecosystem, so the `oracle` role here reads the committed
//! `(before, mutation, after)` specification vectors LITERALLY — no recomputation, no
//! reimplementation of mutation semantics — while `subject` decodes the same committed bytes into
//! real `SemioFlowSnapshot`/`SemioFlowMutation` values and drives this repository's own
//! `apply_semio_flow_mutation`. Both halves read one physical source, so nothing can drift between
//! them; that is why nothing is hand-transcribed into a Rust literal here the way earlier semio
//! cases had to.
//!
//! The vectors were derived by an INDEPENDENT Python implementation of both the committed DSL
//! grammar and this vocabulary's specification (see the ticket's `🐍️semio-pattern-a-vectors.py`),
//! starting from this standard's own committed real artifact
//! `📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio` — never by running this repository's Rust. The
//! `identity-round-trip` scenario is what keeps that honest: it asserts production's own `parse_dsl`
//! of the same real artifact equals the vectors' `before` snapshot, and crosses the committed text
//! artifact against its committed binary twin.
//!
//! The oracle-only build must never link the subject crate (fleet brief §5.3), so every `subject`
//! item is gated behind the generated host's `sut` feature. The Rust SUBJECT phase is blocked this
//! wave by a concurrent os-kernel/job refactor, so it is written and gated but not run.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioFlowMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
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

//#region 🔖️Vectors
/// 🧫️ The committed specification vector for one kind, read as the framework's own dependency-free
/// `Json` — this IS the independently derived evidence the no-oracle decision rests on.
fn vector(ctx: &Context, kind: &str) -> Result<Json, String> {
    ctx.fixture_json(&format!("local://🦠️{kind}.json"))
}

/// 🔎️ One named member of a committed vector, or a loud error naming the file that lacks it.
fn member(vector: &Json, name: &str) -> Result<Json, String> {
    vector.get(name).cloned().ok_or_else(|| format!("committed specification vector carries no {name:?} member"))
}
//#endregion 🔖️Vectors

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let after = member(&vector(ctx, kind)?, "after")?;
        Ok(Outcome::with_raw(after.to_string().into_bytes(), after))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must land
/// back exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let before = member(&vector(ctx, kind)?, "before")?;
        Ok(Outcome::with_raw(before.to_string().into_bytes(), before))
    }
}

/// 🔮️ The round-trip reference answer: the same committed BEFORE snapshot, which is what the real
/// committed artifact decodes to through either of its two committed encodings.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let before = member(&vector(ctx, "no-mutation")?, "before")?;
    Ok(Outcome::with_raw(before.to_string().into_bytes(), before))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::mutations::{apply_semio_flow_mutation, decode_semio_flow_mutation_json, inverse_semio_flow_mutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{
        decode_semio_flow_pack, decode_semio_flow_snapshot_json, encode_semio_flow_pack, encode_semio_flow_snapshot_json, parse_semio_flow_dsl, print_semio_flow_dsl, SemioFlowSnapshot,
    };

    const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🗣️example.dsl.semio";
    const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🌊️pipeline/🖼️assets/🎒️example.pack.semio";

    //#region 🔖️Fixtures
    /// 🧫️ The SAME committed vector `../🦀️component.rs::vector` reads, decoded into real values
    /// through this subset's own `serde_json` bridges rather than transcribed into Rust literals.
    fn vector(ctx: &Context, kind: &str) -> Result<Json, String> {
        ctx.fixture_json(&format!("local://🦠️{kind}.json"))
    }

    fn snapshot_of(vector: &Json, name: &str) -> Result<SemioFlowSnapshot, String> {
        let member = vector.get(name).ok_or_else(|| format!("committed specification vector carries no {name:?} member"))?;
        decode_semio_flow_snapshot_json(&member.to_string())
    }

    fn projection(snapshot: &SemioFlowSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_flow_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &SemioFlowSnapshot, expected: &SemioFlowSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_flow_snapshot_json(got), encode_semio_flow_snapshot_json(expected))
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts the result IS the committed
    /// after-snapshot. The assertion lives here rather than in the comparison because a recorded
    /// no-oracle case runs no oracle role: a handler that merely returned `Ok` would report a pass
    /// having checked nothing.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let vector = vector(ctx, kind)?;
            let mut current = snapshot_of(&vector, "before")?;
            let expected = snapshot_of(&vector, "after")?;
            let mutation = decode_semio_flow_mutation_json(&member_text(&vector, "mutation")?)?;
            let outcome = apply_semio_flow_mutation(&mut current, &mutation);
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
    /// restore the committed before-snapshot exactly.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let vector = vector(ctx, kind)?;
            let base = snapshot_of(&vector, "before")?;
            let mutation = decode_semio_flow_mutation_json(&member_text(&vector, "mutation")?)?;
            let mut current = base.clone();
            let outcome = apply_semio_flow_mutation(&mut current, &mutation);
            if !outcome.messages().is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {:?}", outcome.messages()));
            }
            for step in inverse_semio_flow_mutation(&mutation, &base) {
                let step_outcome = apply_semio_flow_mutation(&mut current, &step);
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

    /// 🔁️ The real artifact through both of its committed encodings. The committed text is this
    /// codec's own output, so byte-identical re-emission is the EXPECTED result here and the wave's
    /// usual pass-through tripwire does not apply; what carries that evidence instead is the binary
    /// twin, a separate committed file produced by a separate codec, which cannot agree by accident.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let expected = snapshot_of(&vector(ctx, "no-mutation")?, "before")?;
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_flow_dsl(&text)?;
        if parsed != expected {
            return Err(disagreement("identity-round-trip: parse_dsl of the real artifact does not match the committed before-snapshot the specification vectors start from", &parsed, &expected));
        }
        let reparsed = parse_semio_flow_dsl(&print_semio_flow_dsl(&parsed))?;
        if reparsed != expected {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &expected));
        }
        let unpacked = decode_semio_flow_pack(&ctx.fixture_bytes(PACK_ASSET)?)?;
        if unpacked != expected {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different snapshot than the committed text artifact", &unpacked, &expected));
        }
        let repacked = decode_semio_flow_pack(&encode_semio_flow_pack(&parsed))?;
        if repacked != expected {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &expected));
        }
        let projection = projection(&parsed)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    fn member_text(vector: &Json, name: &str) -> Result<String, String> {
        Ok(vector.get(name).ok_or_else(|| format!("committed specification vector carries no {name:?} member"))?.to_string())
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
