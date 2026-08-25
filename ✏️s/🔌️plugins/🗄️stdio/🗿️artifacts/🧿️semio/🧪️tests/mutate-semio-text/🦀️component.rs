//! 🦀️ Semio TEXT exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `semio-text-mutation-semantics` (`../../🏅️standards/
//! 🔖️v1/🪆️subsets/✳️text/🧪️oracle/🔣️component.json`): `s.stdio.semio.text` is a semio-NATIVE
//! format with no third-party reader or writer, so `oracle` here reads the committed, independently
//! handcrafted per-kind specification fixtures (`../../🏅️standards/🔖️v1/🪆️subsets/✳️text/
//! 🧬️schema/🧬️mutations/<kind>/🧪️tests/<fixture>/`) literally — no recomputation, no
//! reimplementation of mutation semantics. `subject` drives this repository's own
//! `apply_semio_text_mutation` over the full 7-kind `SemioTextMutation` vocabulary.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none, so
//! the comparison profile never gets two sides to compare. Every law this case claims is therefore
//! asserted INSIDE the subject handler: `mutate-<kind>` checks the applied snapshot IS the committed
//! after-snapshot, `inverse-<kind>` checks the mutation's own computed inverse restores the
//! committed before-snapshot, and `identity-round-trip` crosses the two committed encodings of the
//! real note artifact against each other. A handler that merely returned `Ok` would report a pass
//! having checked nothing at all.
//!
//! **How the fixture reaches typed values.** The generated test host links only `semio-repo-test-host`
//! and, behind `sut`, this subset's own crate — no `serde`, no `serde_json`, and this crate's
//! `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`), so neither `protocol::Mutation` nor
//! a `serde` derive is nameable from here. The subset's own production code therefore exports the bridges
//! this adapter needs, whose signatures name only reachable types:
//! `decode_semio_text_snapshot_json`/`encode_semio_text_snapshot_json` (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️text/🧬️schema/📸️snapshot/🦀️component.rs`), `decode_semio_text_mutation_json`/
//! `inverse_semio_text_mutation` (`…/🧬️mutations/🦀️component.rs`) and the DSL/pack pass-throughs
//! `parse_semio_text_dsl`/`print_semio_text_dsl`/`encode_semio_text_pack`/`decode_semio_text_pack`. Both
//! roles read the SAME committed bytes — the oracle role via `include_str!`, the subject role by decoding
//! that same text — so no hand-transcribed Rust literal exists here to drift away from the fixture it
//! claims to mirror. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation; the Rust SUBJECT phase RUNS. The os-kernel
//! blocker earlier waves recorded here was cleared on 2026-08-24 — `cargo check -p
//! semio-framework-os-kernel --lib` exits 0 and `semio-s-plugin-stdio` builds — so `bun ./📜️script.ts
//! subject exhaustive --owner 🗄️stdio --case mutate-semio-text` really executes every scenario below. The
//! gate keeps the two BUILDS apart; it has never been a reason the subject half goes unmeasured, and for
//! this recorded no-oracle case the subject phase is the only phase that runs at all.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioTextMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["insert-run", "remove-run", "edit-run", "change-run-language", "reorder-runs", "add-mark", "remove-mark"];

/// 🗣️ The real committed note — three runs that between them cover every field this subset has: an
/// unmarked English run, an English run carrying a `bold` mark, and a German run carrying a `link`
/// mark with a non-empty `href`.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🗣️example.dsl.semio";
/// 🎒️ The same note in its binary envelope, written by a separate codec from the DSL text.
#[cfg(feature = "sut")]
const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🎒️example.pack.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after)` fixture TEXT for one kind, read literally via
/// `include_str!` — this IS the independently handcrafted specification vector the no-oracle
/// decision rests on, never recomputed. One `include_str!` per file for the whole adapter: `oracle`
/// answers with `before`/`after`, `subject` decodes all three.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "insert-run" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/🧪️tests/inserts-a-german-run-between-two-english-runs/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/🧪️tests/inserts-a-german-run-between-two-english-runs/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/📥insert-run/🧪️tests/inserts-a-german-run-between-two-english-runs/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-run" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/🧪️tests/removes-the-middle-run/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/🧪️tests/removes-the-middle-run/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🗑️remove-run/🧪️tests/removes-the-middle-run/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "edit-run" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/🧪️tests/rewrites-the-marked-runs-content/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/🧪️tests/rewrites-the-marked-runs-content/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/✏️edit-run/🧪️tests/rewrites-the-marked-runs-content/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "change-run-language" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/🧪️tests/retags-the-second-run-as-german/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/🧪️tests/retags-the-second-run-as-german/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🌐change-run-language/🧪️tests/retags-the-second-run-as-german/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "reorder-runs" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/🧪️tests/moves-the-first-run-to-the-end/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/🧪️tests/moves-the-first-run-to-the-end/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/🔀reorder-runs/🧪️tests/moves-the-first-run-to-the-end/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "add-mark" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/🧪️tests/adds-a-link-mark-ahead-of-the-bold-mark/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/🧪️tests/adds-a-link-mark-ahead-of-the-bold-mark/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➕add-mark/🧪️tests/adds-a-link-mark-ahead-of-the-bold-mark/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "remove-mark" => (
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/🧪️tests/detaches-the-italic-mark-from-the-run/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/🧪️tests/detaches-the-italic-mark-from-the-run/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/🧬️mutations/➖remove-mark/🧪️tests/detaches-the-italic-mark-from-the-run/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-semio-text: no fixture registered for kind {other:?}"),
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
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::mutations::{apply_semio_text_mutation, decode_semio_text_mutation_json, inverse_semio_text_mutation, SemioTextMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{decode_semio_text_pack, decode_semio_text_snapshot_json, encode_semio_text_pack, encode_semio_text_snapshot_json, parse_semio_text_dsl, print_semio_text_dsl, SemioTextSnapshot};

    //#region 🔖️FixtureDecode
    /// 🧫️ Decodes the SAME committed fixture text `../🦀️component.rs::fixture_text` embeds, through
    /// this subset's own production JSON bridges — real deserialization of the committed bytes,
    /// never a Rust literal transcribed beside them.
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<SemioTextSnapshot, String> {
        decode_semio_text_snapshot_json(text).map_err(|error| format!("mutate-semio-text: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<SemioTextMutation, String> {
        decode_semio_text_mutation_json(text).map_err(|error| format!("mutate-semio-text: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &SemioTextSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_text_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both fixtures are written
    /// in, so a red scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioTextSnapshot, expected: &SemioTextSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_text_snapshot_json(got), encode_semio_text_snapshot_json(expected))
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
            let outcome = apply_semio_text_mutation(&mut current, &mutation);
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
    /// restore the committed before-snapshot exactly — run addressing, nested mark indices and the
    /// `href` a `link` mark carries included.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let outcome = apply_semio_text_mutation(&mut current, &mutation);
            if !semio_mutation_refusals(&outcome).is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {:?}", semio_mutation_refusals(&outcome)));
            }
            for step in inverse_semio_text_mutation(&mutation, &base) {
                let step_outcome = apply_semio_text_mutation(&mut current, &step);
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

    /// 🔁️ The real committed note through both of its committed encodings. The DSL text and the
    /// pack envelope are separate committed files produced by separate codecs, so agreeing on one
    /// snapshot cannot be achieved by smuggling bytes from either. Byte-identical re-emission IS
    /// expected here — the committed text is this codec's own output, not a foreign writer's — so
    /// the wave's usual "output must not equal input" tripwire does not apply, and its MIRROR law is
    /// asserted below in its place: `carrier_is_exact` on both committed files, with the text/binary
    /// cross-check keeping that from being a self-comparison.
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
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed note artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_text_dsl(&text)?;
        if parsed.runs.len() != 3 {
            return Err(format!("identity-round-trip: the committed note is the three-run fixture this case describes, but parsed {} run(s)", parsed.runs.len()));
        }
        let printed = print_semio_text_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), text.as_bytes())?;
        let reparsed = parse_semio_text_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(super::PACK_ASSET)?;
        let unpacked = decode_semio_text_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different note than the committed text artifact", &unpacked, &parsed));
        }
        let repacked_bytes = encode_semio_text_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_text_pack(&repacked_bytes)?;
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
/// snapshot the oracle role can read literally, but the note artifact is committed as DSL and pack
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
