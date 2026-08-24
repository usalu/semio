//! 🦀️ Curation exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `curate-1-curation-selection-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `sourcing.curate` is a
//! semio-NATIVE curation document with no third-party reader or writer, so `oracle` here reads the
//! committed, independently handcrafted per-kind specification fixtures
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_sourcing_mutation_reporting` over the full three-kind `SourcingMutation`
//! vocabulary.
//!
//! **What the positional claim adds.** All three vectors run against the same two-entry curation, so
//! a membership-only comparison would pass an implementation that rebuilt or re-sorted `curated` on
//! every edit. The feature's `effect` column states the positional claim per kind — `append`,
//! `detach`, `retune` — and the handler checks it against the committed after-snapshot rather than
//! against a recomputation, which is what keeps the specification vector the authority.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and declaring
//! stdio's contribution directory as a host package for the curate artifact would make one plugin's
//! test tree a build dependency of another's. The laws are stated inline, in the same words and with
//! the same strictness.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this plugin's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`). The subset's
//! own production code exports the bridges instead: `decode_curate_snapshot_json`/
//! `encode_curate_snapshot_json`/`parse_curate_dsl`/`print_curate_dsl`/`curate_selection_summary`
//! (`…/🧬️schema/📸️snapshot/🦀️component.rs`) and `decode_sourcing_mutation_json`/
//! `apply_sourcing_mutation_reporting`/`inverse_sourcing_mutation_steps`
//! (`…/🧬️schema/🧬️mutations/🦀️component.rs`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `SourcingMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-curated-item", "delete-curated-item", "change-curated-item-count"];

/// 🗣️ The real committed curation — ten stock entries across beams, windows and slabs, against an
/// empty curation.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind,
/// read literally via `include_str!` — this IS the independently handcrafted vector the no-oracle
/// decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "create-curated-item" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🧪️tests/appends-a-steel-plate-to-the-curation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🧪️tests/appends-a-steel-plate-to-the-curation/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🧪️tests/appends-a-steel-plate-to-the-curation/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🧪️tests/appends-a-steel-plate-to-the-curation/🎯️outcome/🔣️component.json"),
        ),
        "delete-curated-item" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🧪️tests/removes-the-clt-panel-from-the-curation/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🧪️tests/removes-the-clt-panel-from-the-curation/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🧪️tests/removes-the-clt-panel-from-the-curation/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🧪️tests/removes-the-clt-panel-from-the-curation/🎯️outcome/🔣️component.json"),
        ),
        "change-curated-item-count" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🧪️tests/raises-the-glulam-beam-count-to-20/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🧪️tests/raises-the-glulam-beam-count-to-20/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🧪️tests/raises-the-glulam-beam-count-to-20/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🧪️tests/raises-the-glulam-beam-count-to-20/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-curate-1: no specification vector registered for kind {other:?}"),
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
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE snapshot — undoing any mutation must
/// return to exactly where the specification vector started.
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
    use semio_s_plugin_sourcing::artifacts::curate::standards::v1::subsets::any::schema::mutations::{apply_sourcing_mutation_reporting, decode_sourcing_mutation_json, inverse_sourcing_mutation_steps, SourcingMutation};
    use semio_s_plugin_sourcing::artifacts::curate::standards::v1::subsets::any::schema::snapshot::{curate_selection_summary, decode_curate_snapshot_json, encode_curate_snapshot_json, parse_curate_dsl, print_curate_dsl, CurateSnapshot};

    //#region 🔖️FixtureDecode
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<CurateSnapshot, String> {
        decode_curate_snapshot_json(text).map_err(|error| format!("mutate-curate-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<SourcingMutation, String> {
        decode_sourcing_mutation_json(text).map_err(|error| format!("mutate-curate-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &CurateSnapshot) -> Result<Json, String> {
        parse_json(&encode_curate_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &CurateSnapshot, expected: &CurateSnapshot) -> String {
        format!("{what}\n     got: [{}] {}\nexpected: [{}] {}", curate_selection_summary(got), encode_curate_snapshot_json(got), curate_selection_summary(expected), encode_curate_snapshot_json(expected))
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The positional observability law, per kind. `append` requires exactly one more entry with
    /// every member already present still at its own index; `detach` requires exactly one fewer with
    /// the survivors in their original relative order; `retune` requires the same length, the same
    /// ids at the same indices, and exactly one count moved. All three fail an implementation that
    /// rebuilt or re-sorted the curation, which a membership comparison would let through.
    fn effect_holds(kind: &str, effect: &str, before: &CurateSnapshot, after: &CurateSnapshot) -> Result<(), String> {
        let was: Vec<(String, u64)> = before.curated.iter().map(|item| (item.object_id.clone(), item.count)).collect();
        let now: Vec<(String, u64)> = after.curated.iter().map(|item| (item.object_id.clone(), item.count)).collect();
        match effect {
            "append" => {
                if now.len() != was.len() + 1 {
                    return Err(format!("mutate-{kind}: an append must leave exactly one more curated entry, went from {} to {}", was.len(), now.len()));
                }
                if now[..was.len()] != was[..] {
                    return Err(format!("mutate-{kind}: an append must land AFTER the members already present, but the leading entries changed: {was:?} -> {now:?}"));
                }
                Ok(())
            }
            "detach" => {
                if now.len() + 1 != was.len() {
                    return Err(format!("mutate-{kind}: a detach must leave exactly one fewer curated entry, went from {} to {}", was.len(), now.len()));
                }
                let survivors: Vec<_> = was.iter().filter(|entry| now.contains(entry)).cloned().collect();
                if survivors != now {
                    return Err(format!("mutate-{kind}: a detach must leave the survivors in their original order, got {now:?} from {was:?}"));
                }
                Ok(())
            }
            "retune" => {
                if now.len() != was.len() {
                    return Err(format!("mutate-{kind}: a count change must not add or drop an entry, went from {} to {}", was.len(), now.len()));
                }
                let moved: Vec<_> = was.iter().zip(now.iter()).filter(|(one, other)| one != other).collect();
                if moved.len() != 1 {
                    return Err(format!("mutate-{kind}: a count change must move exactly one entry, moved {}: {was:?} -> {now:?}", moved.len()));
                }
                let ((was_id, _), (now_id, _)) = moved[0];
                if was_id != now_id {
                    return Err(format!("mutate-{kind}: a count change must keep the entry at its own index, but {was_id:?} became {now_id:?}"));
                }
                Ok(())
            }
            other => Err(format!("mutate-{kind}: the feature declares an unknown effect {other:?}")),
        }
    }

    /// 🎯️ The committed outcome claim: all three vectors are clean `applied` vectors, so any
    /// diagnostic at all is a divergence. Checking the snapshot alone would let a warned degenerate
    /// application pass as a real one.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("mutate-{kind}: every committed curate vector is a clean applied vector, but its outcome declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts, in role, that the result IS
    /// the committed after-snapshot, that the curation moved the way the feature's `effect` column
    /// claims, and that the reported diagnostics are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_sourcing_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            effect_holds(kind, &ctx.doc_json()?.str("effect"), &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly — entry POSITION included, which is what a
    /// delete/create pair has to rebuild rather than re-append.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_sourcing_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the curation untouched, so restoring it proves nothing"));
            }
            for step in inverse_sourcing_mutation_steps(&mutation, &base) {
                let undone = apply_sourcing_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
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

    /// 🔁️ The real committed curation through its own DSL carrier. `.curate.dsl.semio` is a
    /// fixed-layout document — a catalogue child handle, a stock list and a `curated` table with a
    /// declared column header — with no writer freedom, and the committed example is this codec's
    /// own output, committed as such. The wave's usual "output must not equal input" tripwire
    /// therefore does not apply and would be the wrong law here; the byte-exact law is asserted
    /// instead, together with a content check a parser returning `CurateSnapshot::default()` (an
    /// empty-stock handle and no stock entries at all) cannot satisfy.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed curation artifact is not UTF-8: {error}"))?;
        let parsed = parse_curate_dsl(&text)?;
        if parsed.stock_extra.len() != 10 || !parsed.curated.is_empty() {
            return Err(format!("identity-round-trip: the committed curation carries ten stock entries against an empty curation, but parsed {} stock entr(ies) and [{}]", parsed.stock_extra.len(), curate_selection_summary(&parsed)));
        }
        let printed = print_curate_dsl(&parsed);
        let reparsed = parse_curate_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the document back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        if printed != text {
            let at = printed.as_bytes().iter().zip(text.as_bytes().iter()).position(|(one, other)| one != other);
            return Err(format!(
                "exact-bytes law violated: `.curate.dsl.semio` is a fixed-layout document and the committed example is this codec's own output, so the re-printed text was required to reproduce it — {} byte(s) out against {} byte(s) in{}",
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
/// snapshot the oracle role can read literally, but the real curation is committed as `.dsl.semio`
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
