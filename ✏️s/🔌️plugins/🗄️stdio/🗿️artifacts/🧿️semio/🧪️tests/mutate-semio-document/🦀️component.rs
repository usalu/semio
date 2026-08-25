//! 🦀️ Semio DOCUMENT exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-document-mutation-semantics` (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️document/🧪️oracle/🔣️component.json`): `s.stdio.semio.document` is a semio-NATIVE format with
//! no third-party reader or writer in any ecosystem, so the `oracle` role here reads the committed
//! `(before, mutation, after)` specification vectors LITERALLY — no recomputation, no
//! reimplementation of mutation semantics — while `subject` decodes the same committed bytes into
//! real `SemioDocumentSnapshot`/`SemioDocumentMutation` values and drives this repository's own
//! `apply_semio_document_mutation`. Both halves read one physical source, so nothing can drift between
//! them; that is why nothing is hand-transcribed into a Rust literal here the way earlier semio
//! cases had to.
//!
//! The vectors were derived by an INDEPENDENT Python implementation of both the committed DSL
//! grammar and this vocabulary's specification (see the ticket's `🐍️semio-pattern-a-vectors.py`),
//! starting from this standard's own committed real artifact
//! `📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio` — never by running this repository's Rust. The
//! `identity-round-trip` scenario is what keeps that honest: it asserts production's own `parse_dsl`
//! of the same real artifact equals the vectors' `before` snapshot, and crosses the committed text
//! artifact against its committed binary twin.
//!
//! The oracle-only build must never link the subject crate (fleet brief §5.3), so every `subject` item is
//! gated behind the generated host's `sut` feature. The Rust SUBJECT phase RUNS. The os-kernel blocker
//! earlier waves recorded here was cleared on 2026-08-24 — `cargo check -p semio-framework-os-kernel
//! --lib` exits 0 and `semio-s-plugin-stdio` builds — so `bun ./📜️script.ts subject exhaustive --owner
//! 🗄️stdio --case mutate-semio-document` really executes every scenario below. The gate keeps the two
//! BUILDS apart; it has never been a reason the subject half goes unmeasured, and for this recorded
//! no-oracle case the subject phase is the only phase that runs at all.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioDocumentMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-block",
    "remove-block",
    "set-block-content",
    "set-paragraph-style",
    "set-heading-level",
    "set-list-ordered",
    "set-run-text",
    "set-run-style",
    "set-image-block",
    "insert-style",
    "remove-style",
    "set-style-name",
    "set-style-based-on",
    "insert-image",
    "remove-image",
    "set-image-bytes",
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
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::mutations::{apply_semio_document_mutation, decode_semio_document_mutation_json, inverse_semio_document_mutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{
        decode_semio_document_pack, decode_semio_document_snapshot_json, encode_semio_document_pack, encode_semio_document_snapshot_json, parse_semio_document_dsl, print_semio_document_dsl, SemioDocumentSnapshot,
    };

    const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio";
    const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🎒️example.pack.semio";

    //#region 🔖️Fixtures
    /// 🧫️ The SAME committed vector `../🦀️component.rs::vector` reads, decoded into real values
    /// through this subset's own `serde_json` bridges rather than transcribed into Rust literals.
    fn vector(ctx: &Context, kind: &str) -> Result<Json, String> {
        ctx.fixture_json(&format!("local://🦠️{kind}.json"))
    }

    fn snapshot_of(vector: &Json, name: &str) -> Result<SemioDocumentSnapshot, String> {
        let member = vector.get(name).ok_or_else(|| format!("committed specification vector carries no {name:?} member"))?;
        decode_semio_document_snapshot_json(&member.to_string())
    }

    fn projection(snapshot: &SemioDocumentSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_document_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &SemioDocumentSnapshot, expected: &SemioDocumentSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_document_snapshot_json(got), encode_semio_document_snapshot_json(expected))
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
            let mutation = decode_semio_document_mutation_json(&member_text(&vector, "mutation")?)?;
            let outcome = apply_semio_document_mutation(&mut current, &mutation);
            if !semio_mutation_refusals(&outcome).is_empty() {
                return Err(format!("mutate-{kind}: the mutation was rejected: {:?}", semio_mutation_refusals(&outcome)));
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
            let mutation = decode_semio_document_mutation_json(&member_text(&vector, "mutation")?)?;
            let mut current = base.clone();
            let outcome = apply_semio_document_mutation(&mut current, &mutation);
            if !semio_mutation_refusals(&outcome).is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {:?}", semio_mutation_refusals(&outcome)));
            }
            for step in inverse_semio_document_mutation(&mutation, &base) {
                let step_outcome = apply_semio_document_mutation(&mut current, &step);
                if !semio_mutation_refusals(&step_outcome).is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {:?}", semio_mutation_refusals(&step_outcome)));
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
    /// usual pass-through tripwire does not apply, and its MIRROR law is asserted below in its place:
    /// `carrier_is_exact` on both committed files, with the binary twin — a separate committed file
    /// produced by a separate codec, which cannot agree by accident — keeping that honest.
    /// 🔒️ **The byte half of the identity law — asserted, and asserted as `carrier_is_exact`.**
    /// `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin; the two
    /// committed example artifacts this scenario reads were produced by these very codecs, so
    /// reproducing them BYTE FOR BYTE is the correct answer here and `law::reparsed_not_copied`
    /// would be exactly backwards — the same reading `mutate-dag-1` records for `.dag.dsl.semio`
    /// and `mutate-bmp-v3` for its own reference-authored fixture. Saying so in prose alone would
    /// leave the claim an excuse; asserting it makes it checkable, and it fails with the offset of
    /// the first differing byte the moment the printer or the packer drifts. Nor is it a
    /// self-comparison: one side is a file committed to the repository, the other is computed now.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let expected = snapshot_of(&vector(ctx, "no-mutation")?, "before")?;
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_document_dsl(&text)?;
        if parsed != expected {
            return Err(disagreement("identity-round-trip: parse_dsl of the real artifact does not match the committed before-snapshot the specification vectors start from", &parsed, &expected));
        }
        let printed = print_semio_document_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), text.as_bytes())?;
        let reparsed = parse_semio_document_dsl(&printed)?;
        if reparsed != expected {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &expected));
        }
        let pack_bytes = ctx.fixture_bytes(PACK_ASSET)?;
        let unpacked = decode_semio_document_pack(&pack_bytes)?;
        if unpacked != expected {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different snapshot than the committed text artifact", &unpacked, &expected));
        }
        let repacked_bytes = encode_semio_document_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_document_pack(&repacked_bytes)?;
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
