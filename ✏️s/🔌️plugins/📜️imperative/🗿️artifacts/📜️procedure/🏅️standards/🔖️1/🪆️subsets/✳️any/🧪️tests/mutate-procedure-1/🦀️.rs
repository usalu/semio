//! 🦀️ Imperative-program exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR. Recorded no-oracle decision
//! `procedure-1-nested-step-list-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): `procedure.document` is a
//! semio-NATIVE program document with no third-party reader or writer, so `oracle` here reads the
//! committed, independently handcrafted per-kind specification fixtures
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_procedure_mutation_reporting` over the full four-kind
//! `ProcedureMutation` vocabulary.
//!
//! **Why the feature carries a `program` column.** The document persists only content-addressed
//! child handles; the program lives in a working scene keyed by the flow handle, so a decoded
//! `⬅️before` stands for no program at all until one is cached against it. Inside this crate each
//! triad leaf does that with its own `cached_program()` Rust literal. From outside neither `Path`
//! nor its `Dictionary`/`Value` members can be constructed, so the program travels as JSON in the
//! feature's own `Examples` row and is decoded by `seed_procedure_flow_json`. That restatement is
//! self-checking rather than a second source of truth: a program that drifted from the leaf's would
//! stop producing the committed diagnostic, and the scenario would fail on the `code` column.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and declaring
//! stdio's contribution directory as a host package for the imperative artifact would make one
//! plugin's test tree a build dependency of another's. The laws are stated inline, in the same words
//! and with the same strictness.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this plugin's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`🦀️.rs`). The subset's
//! own production code exports the bridges instead: `decode_procedure_snapshot_json`/
//! `encode_procedure_snapshot_json`/`parse_procedure_dsl`/`print_procedure_dsl`
//! (`…/🧬️schema/📸️snapshot/🦀️.rs`) and `decode_procedure_mutation_json`/
//! `seed_procedure_flow_json`/`apply_procedure_mutation_reporting`/
//! `inverse_procedure_mutation_steps`/`procedure_program_summary`
//! (`…/🧬️schema/🧬️mutations/🦀️.rs`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `ProcedureMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-step", "delete-step", "reorder-steps", "edit-step-params"];

/// 🗣️ The real committed program document — one `s.stdio.semio@v1/flow` child and one
/// `s.stdio.semio@v1/text` child, both carried as hex-encoded handles.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind,
/// read literally via `include_str!` — this IS the independently handcrafted vector the no-oracle
/// decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "create-step" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/rejects-a-duplicate-step-id-at-the-root-path/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/rejects-a-duplicate-step-id-at-the-root-path/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/rejects-a-duplicate-step-id-at-the-root-path/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-step/🧪️tests/rejects-a-duplicate-step-id-at-the-root-path/🎯️outcome/🔣️.json"),
        ),
        "delete-step" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/rejects-a-root-step-id-addressed-inside-a-branch-body/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/rejects-a-root-step-id-addressed-inside-a-branch-body/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/rejects-a-root-step-id-addressed-inside-a-branch-body/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-step/🧪️tests/rejects-a-root-step-id-addressed-inside-a-branch-body/🎯️outcome/🔣️.json"),
        ),
        "reorder-steps" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-steps/🧪️tests/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place/🎯️outcome/🔣️.json"),
        ),
        "edit-step-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/warns-that-step-1-already-carries-the-requested-params/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/warns-that-step-1-already-carries-the-requested-params/🦠️mutation/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/warns-that-step-1-already-carries-the-requested-params/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/warns-that-step-1-already-carries-the-requested-params/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-procedure-1: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER snapshot, read literally. For every kind in
/// this vocabulary that is byte-identical to the BEFORE snapshot — two refusals and two degenerate
/// applications — which is why the `(code, severity)` pair rather than the document is what the
/// subject handler holds each vector to.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_imperative::artifacts::procedure::standards::v1::subsets::any::schema::mutations::{apply_procedure_mutation_reporting, decode_procedure_mutation_json, procedure_program_summary, inverse_procedure_mutation_steps, seed_procedure_flow_json, ProcedureMutation};
    use semio_s_plugin_imperative::artifacts::procedure::standards::v1::subsets::any::schema::snapshot::{decode_procedure_snapshot_json, encode_procedure_snapshot_json, parse_procedure_dsl, print_procedure_dsl, ProcedureSnapshot};

    //#region 🔖️FixtureDecode
    /// 🧫️ The committed before-snapshot for one kind, with its composed flow child resolved to the
    /// program the feature's own row declares — the two together ARE the before-state.
    fn seeded_before(kind: &str, row: &Json) -> Result<ProcedureSnapshot, String> {
        let (before, _, _, _) = super::fixture_text(kind);
        let mut snapshot = decode_procedure_snapshot_json(before).map_err(|error| format!("mutate-procedure-1: the committed before-snapshot for {kind:?} must decode: {error}"))?;
        let program = row.get("program").map(Json::to_string).unwrap_or_default();
        seed_procedure_flow_json(&mut snapshot, &program).map_err(|error| format!("mutate-procedure-1: the feature's declared program for {kind:?} must decode: {error}"))?;
        Ok(snapshot)
    }

    fn mutation_of(text: &str, label: &str, kind: &str) -> Result<ProcedureMutation, String> {
        decode_procedure_mutation_json(text).map_err(|error| format!("mutate-procedure-1: the {label} payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &ProcedureSnapshot) -> Result<Json, String> {
        parse_json(&encode_procedure_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &ProcedureSnapshot, expected: &ProcedureSnapshot) -> String {
        format!("{what}\n     got: {} [{}]\nexpected: {} [{}]", encode_procedure_snapshot_json(got), procedure_program_summary(got), encode_procedure_snapshot_json(expected), procedure_program_summary(expected))
    }

    fn params_text(row: &Json) -> String {
        row.get("params").map(Json::to_string).unwrap_or_default()
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 🎯️ The committed vector's own claim, in full: the declared diagnostic code, its declared
    /// SEVERITY, and that the content-addressed flow handle was not re-minted. All four vectors
    /// leave the document byte-identical, so without the severity a refusal and a degenerate
    /// application are indistinguishable, and without the handle check a refusal that quietly
    /// rebuilt the child would look clean.
    fn vector_reports(kind: &str, code: &str, level: &str, declared: &Json, raised: &[(String, String)], before: &ProcedureSnapshot, after: &ProcedureSnapshot, expected: &ProcedureSnapshot) -> Result<(), String> {
        let declared_code = if declared.str("code").is_empty() { declared.array("messages").first().map(|message| message.str("code")).unwrap_or_default() } else { declared.str("code") };
        if declared_code != code {
            return Err(format!("mutate-{kind}: the feature's Examples row names code {code:?} but the committed outcome declares {declared_code:?} — the two declarations of the same vector have drifted"));
        }
        let Some((raised_code, raised_level)) = raised.first() else {
            return Err(format!("mutate-{kind}: the committed vector must report {code:?} at {level}, but the implementation raised nothing at all — most likely the program the feature declared is not the one this vector was authored against"));
        };
        if raised_code != code || raised_level != level {
            return Err(format!("mutate-{kind}: the committed vector must report {code:?} at {level}, but the implementation raised {raised_code:?} at {raised_level} — all four vectors here leave the document byte-identical, so this pair is the only thing that tells a refusal from a degenerate application"));
        }
        if after != expected {
            return Err(disagreement(&format!("mutate-{kind}: the vector must leave the document at the committed after-snapshot"), after, expected));
        }
        if after.flow != before.flow {
            return Err(format!("mutate-{kind}: the vector re-minted the composed flow handle ({} -> {}) — the program may be unchanged, but the document is no longer the same document", before.flow.child_id, after.flow.child_id));
        }
        Ok(())
    }

    /// 👁️ The observability law, in the exact form this artifact can state it: an APPLIED mutation
    /// must move the content-addressed flow handle, which is a digest of the child and therefore
    /// moves if and only if the program moved.
    fn application_is_observable(kind: &str, raised: &[(String, String)], base: &ProcedureSnapshot, mutated: &ProcedureSnapshot) -> Result<(), String> {
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the real-effect payload was meant to APPLY to the seeded program, but the implementation raised {raised:?}"));
        }
        if mutated.flow.child_id == base.flow.child_id {
            return Err(format!("mutate-{kind}: applying this kind left the content-addressed flow handle at {} — the mutation never reached the program, so the scenario would report a pass for a mutation it never observed ([{}])", base.flow.child_id, procedure_program_summary(mutated)));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Both halves in one scenario: the committed vector for its exact `(code, severity)` pair
    /// with the flow handle required to stay put, then the feature's own real-effect payload against
    /// the same seeded program for its effect.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (_, vector, after, outcome) = super::fixture_text(kind);
            let base = seeded_before(kind, &row)?;
            let expected = decode_procedure_snapshot_json(after).map_err(|error| format!("mutate-procedure-1: the committed after-snapshot for {kind:?} must decode: {error}"))?;
            let vector = mutation_of(vector, "committed vector", kind)?;
            let mut replayed = base.clone();
            let raised = apply_procedure_mutation_reporting(&mut replayed, &vector);
            vector_reports(kind, &row.str("code"), &row.str("level"), &parse_json(outcome)?, &raised, &base, &replayed, &expected)?;

            let reseeded = seeded_before(kind, &row)?;
            let payload = mutation_of(&params_text(&row), "feature real-effect", kind)?;
            let mut applied = reseeded.clone();
            let applied_messages = apply_procedure_mutation_reporting(&mut applied, &payload);
            application_is_observable(kind, &applied_messages, &reseeded, &applied)?;
            let projection = projection(&applied)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law over the seeded program: applying the kind and then its OWN
    /// computed inverse must restore the document exactly, flow handle included. Because that handle
    /// is a digest of the child, restoring it is the strongest available statement that the whole
    /// program came back — step ORDER and nested body membership included, which is what a
    /// `reorder-steps` undo and a branch-scoped `delete-step` undo respectively have to rebuild.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let base = seeded_before(kind, &row)?;
            let payload = mutation_of(&params_text(&row), "feature real-effect", kind)?;
            let mut current = base.clone();
            let raised = apply_procedure_mutation_reporting(&mut current, &payload);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current.flow.child_id == base.flow.child_id {
                return Err(format!("inverse-{kind}: the forward mutation left the flow handle untouched, so restoring it proves nothing ([{}])", procedure_program_summary(&current)));
            }
            for step in inverse_procedure_mutation_steps(&payload, &base) {
                let undone = apply_procedure_mutation_reporting(&mut current, &step);
                if undone.iter().any(|(code, _)| code != "mutation.no-op") {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real committed program document through its own DSL carrier. `.imperative.dsl.semio`
    /// is a fixed-layout record grammar whose two child-handle lines are hex-encoded, with no writer
    /// freedom at all, and the committed example is this codec's own output, committed as such. The
    /// wave's usual "output must not equal input" tripwire therefore does not apply and would be the
    /// wrong law here; the byte-exact law is asserted instead, together with a check on both child
    /// dialects that a parser returning `ProcedureSnapshot::default()` cannot satisfy.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed program artifact is not UTF-8: {error}"))?;
        let parsed = parse_procedure_dsl(&text)?;
        let rendered = encode_procedure_snapshot_json(&parsed);
        if !rendered.contains("\"flow\"") || !rendered.contains("\"text\"") || !rendered.contains("imperative-flow-") || !rendered.contains("imperative-text-") {
            return Err(format!("identity-round-trip: the committed document composes one flow child and one text child, but parsed {rendered}"));
        }
        let printed = print_procedure_dsl(&parsed);
        let reparsed = parse_procedure_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        if printed != text {
            let at = printed.as_bytes().iter().zip(text.as_bytes().iter()).position(|(one, other)| one != other);
            return Err(format!(
                "exact-bytes law violated: `.imperative.dsl.semio` is a fixed-layout hex-encoded record grammar and the committed example is this codec's own output, so the re-printed text was required to reproduce it — {} byte(s) out against {} byte(s) in{}",
                printed.len(),
                text.len(),
                match at {
                    Some(offset) => format!(" (first at byte {offset})"),
                    None => String::new(),
                }
            ));
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
/// deliberately subject-only: the reference answer for every other scenario is a committed JSON
/// snapshot the oracle role can read literally, but the real document is committed as `.dsl.semio`
/// text ONLY and turning that into a document needs this subset's own codec, which the oracle-only
/// build must not link.
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
