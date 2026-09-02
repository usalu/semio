//! 🗂️ Curation exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the `sourcing.curate` document and all three typed mutations, written
//! in Python from this subset's committed snapshot schema, mutation schema, mutation grammar and
//! specification vectors. This adapter registers the SUBJECT half only: keeping oracle
//! registrations here would put this repository's answer on both sides of the comparison.
//!
//! **What the positional claim adds.** The three committed vectors all run against the same
//! two-entry curation and the derived real document is a three-entry one, so a membership-only
//! comparison would pass an implementation that rebuilt or re-sorted `curated` on every edit. The
//! feature's `effect` column states the positional claim per row — `append`, `detach`, `retune` —
//! and both implementations check it in role.
//!
//! **A limit of the vocabulary, and why the two tables differ.** `create-curated-item` carries an
//! `objectId` and a `count` and no index, so a created item can only land at the end and the inverse
//! of `delete-curated-item` is exact only for a TRAILING item. Both implementations share that
//! limit, so a differential alone would report a green over a violated law; it is caught because
//! both sides assert the restoring law in role, position for position. The feature therefore deletes
//! the LEADING entry in `mutate-` and the TRAILING one in `inverse-`, on purpose and stated there.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this plugin's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`🦀️.rs`). The subset's
//! own production code exports the bridges instead: `decode_curate_snapshot_json`/
//! `encode_curate_snapshot_json`/`parse_curate_dsl`/`print_curate_dsl`/`curate_selection_summary`
//! (`…/🧬️schema/📸️snapshot/🦀️.rs`) and `decode_sourcing_mutation_json`/
//! `apply_sourcing_mutation_reporting`/`inverse_sourcing_mutation_steps`
//! (`…/🧬️schema/🧬️mutations/🦀️.rs`).
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and declaring
//! stdio's contribution directory as a host package for the curate artifact would make one plugin's
//! test tree a build dependency of another's. The laws are stated inline, in the same words and with
//! the same strictness.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SourcingMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["create-curated-item", "delete-curated-item", "change-curated-item-count"];

//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_sourcing::artifacts::curate::standards::v1::subsets::any::schema::mutations::{apply_sourcing_mutation_reporting, decode_sourcing_mutation_json, inverse_sourcing_mutation_steps, SourcingMutation};
    use semio_s_plugin_sourcing::artifacts::curate::standards::v1::subsets::any::schema::snapshot::{curate_selection_summary, decode_curate_snapshot_json, encode_curate_snapshot_json, parse_curate_dsl, print_curate_dsl, CurateSnapshot};

    //#region 🔖️Plan
    /// 🧫️ The one declared fixture URI of this scenario's steps containing `needle`.
    fn uri_in(ctx: &Context, needle: &str) -> Result<String, String> {
        ctx.scenario
            .steps
            .iter()
            .flat_map(|(_, text)| text.split_whitespace())
            .find(|token| (token.starts_with("asset://") || token.starts_with("local://") || token.starts_with("shared://")) && token.contains(needle))
            .map(|token| token.to_string())
            .ok_or_else(|| format!("scenario {} declares no fixture URI containing {needle:?}", ctx.scenario.id))
    }

    /// 🧫️ The declared fixture's bytes as UTF-8 text.
    fn fixture_text(ctx: &Context, uri: &str) -> Result<String, String> {
        String::from_utf8(ctx.fixture_bytes(uri)?).map_err(|error| format!("the declared fixture is not UTF-8: {error}"))
    }

    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<CurateSnapshot, String> {
        decode_curate_snapshot_json(text).map_err(|error| format!("mutate-curate-1: the {label} snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<SourcingMutation, String> {
        decode_sourcing_mutation_json(text).map_err(|error| format!("mutate-curate-1: the mutation payload for {kind:?} must decode: {error}"))
    }

    /// 📤️ What parity compares: the whole document this subset's own JSON codec writes. Every member
    /// of `CurateSnapshot` is plain content — this artifact composes no digest-derived child — so
    /// nothing has to be held back from the comparison.
    fn projection(snapshot: &CurateSnapshot) -> Result<Json, String> {
        parse_json(&encode_curate_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &CurateSnapshot, expected: &CurateSnapshot) -> String {
        format!("{what}\n     got: [{}] {}\nexpected: [{}] {}", curate_selection_summary(got), encode_curate_snapshot_json(got), curate_selection_summary(expected), encode_curate_snapshot_json(expected))
    }
    //#endregion 🔖️Plan

    //#region 🔖️Laws
    /// 👁️ The positional observability law, per kind. `append` requires exactly one more entry with
    /// every member already present still at its own index; `detach` requires exactly one fewer with
    /// the survivors in their original relative order; `retune` requires the same length, the same
    /// ids at the same indices, and exactly one count moved. All three fail an implementation that
    /// rebuilt or re-sorted the curation, which a membership comparison would let through.
    fn effect_holds(scenario: &str, effect: &str, before: &CurateSnapshot, after: &CurateSnapshot) -> Result<(), String> {
        let entries = |snapshot: &CurateSnapshot| snapshot.curated.iter().map(|item| (item.object_id.clone(), item.count)).collect::<Vec<_>>();
        let (was, now) = (entries(before), entries(after));
        match effect {
            "append" => {
                if now.len() != was.len() + 1 {
                    return Err(format!("{scenario}: an append must leave exactly one more curated entry, went from {} to {}", was.len(), now.len()));
                }
                if now[..was.len()] != was[..] {
                    return Err(format!("{scenario}: an append must land AFTER the members already present, but the leading entries changed: {was:?} -> {now:?}"));
                }
            }
            "detach" => {
                if now.len() + 1 != was.len() {
                    return Err(format!("{scenario}: a detach must leave exactly one fewer curated entry, went from {} to {}", was.len(), now.len()));
                }
                let survivors: Vec<_> = was.iter().filter(|entry| now.contains(entry)).cloned().collect();
                if survivors != now {
                    return Err(format!("{scenario}: a detach must leave the survivors in their original order, got {now:?} from {was:?}"));
                }
            }
            "retune" => {
                if now.len() != was.len() {
                    return Err(format!("{scenario}: a count change must not add or drop an entry, went from {} to {}", was.len(), now.len()));
                }
                let moved: Vec<usize> = (0..was.len()).filter(|&at| was[at] != now[at]).collect();
                if moved.len() != 1 {
                    return Err(format!("{scenario}: a count change must move exactly one entry, moved {}: {was:?} -> {now:?}", moved.len()));
                }
                if was[moved[0]].0 != now[moved[0]].0 {
                    return Err(format!("{scenario}: a count change must keep the entry at its own index, but {:?} became {:?}", was[moved[0]].0, now[moved[0]].0));
                }
            }
            other => return Err(format!("{scenario}: the feature declares an unknown effect {other:?}")),
        }
        if before.catalog != after.catalog || before.stock_extra != after.stock_extra {
            return Err(format!("{scenario}: the vocabulary edits `curated` only, but the catalogue or the stock table moved too"));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: all three vectors are clean `applied` vectors, so any
    /// diagnostic at all is a divergence. Checking the snapshot alone would let a warned degenerate
    /// application pass as a real one.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("spec-vector-{kind}: every committed curate vector is a clean applied vector, but its outcome declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("spec-vector-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL derived timber-kit curation with the parameters the feature
    /// states, and asserts in role that the curation moved the way the `effect` column claims.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let plan = ctx.doc_json()?;
            let base = snapshot_of(&fixture_text(ctx, &uri_in(ctx, "timber-kit")?)?, "derived", kind)?;
            let mutation = mutation_of(&plan.get("mutation").ok_or("the feature states no mutation payload")?.to_string(), kind)?;
            let mut current = base.clone();
            let raised = apply_sourcing_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters were rejected: {raised:?}"));
            }
            effect_holds(&format!("mutate-{kind}"), &plan.str("effect"), &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ Applies one kind to the REAL derived curation and then its OWN computed inverse steps. The
    /// projection carries BOTH documents: projecting only the restored one would make all three rows
    /// project the same value and the differential would be vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let plan = ctx.doc_json()?;
            let base = snapshot_of(&fixture_text(ctx, &uri_in(ctx, "timber-kit")?)?, "derived", kind)?;
            let mutation = mutation_of(&plan.get("mutation").ok_or("the feature states no mutation payload")?.to_string(), kind)?;
            let mut current = base.clone();
            let raised = apply_sourcing_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            effect_holds(&format!("inverse-{kind}"), &plan.str("effect"), &base, &current)?;
            let mutated = projection(&current)?;
            for step in inverse_sourcing_mutation_steps(&mutation, &base) {
                let undone = apply_sourcing_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), projection(&current)?)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📐️ Replays one committed handcrafted specification vector, read through the plan's declared
    /// fixtures — the same three files the Python reference reads. This is where the evidence the
    /// case carried before the conversion still lives: the applied document is held to the committed
    /// after-snapshot, the positional claim to the feature's `effect` column, and the reported
    /// diagnostics to the committed `🎯️outcome`.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = snapshot_of(&fixture_text(ctx, &uri_in(ctx, "⬅️before")?)?, "committed before", kind)?;
            let expected = snapshot_of(&fixture_text(ctx, &uri_in(ctx, "➡️after")?)?, "committed after", kind)?;
            let mutation = mutation_of(&fixture_text(ctx, &uri_in(ctx, "🦠️mutation")?)?, kind)?;
            let mut current = base.clone();
            let raised = apply_sourcing_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("spec-vector-{kind}: the applied snapshot does not match the committed after-snapshot"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(OUTCOME)?, &raised)?;
            effect_holds(&format!("spec-vector-{kind}"), &ctx.doc_json()?.str("effect"), &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🎯️ Every committed curate vector declares the same clean applied outcome, so the claim is
    /// stated once rather than read three times from three identical files.
    const OUTCOME: &str = "{\"status\":\"applied\"}";

    /// 🔁️ Two identities in one scenario, because they can only be asserted in two different places.
    ///
    /// The CARRIER identity is Rust-only and asserted here in role, on the artifact's own committed
    /// example: `.curate.dsl.semio` is a fixed-layout document with no writer freedom and the
    /// committed example is this codec's own output, so the re-printed text is required to reproduce
    /// it byte for byte. THIS CURRENTLY FAILS — `parse_curate_dsl` rejects the committed example —
    /// and the failure is kept rather than routed around; see the feature description.
    ///
    /// The DOCUMENT identity is what the Python reference can also produce: the document this
    /// subset's own JSON codec reads out of the derived real kit.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let derived = snapshot_of(&fixture_text(ctx, &uri_in(ctx, "timber-kit")?)?, "derived", "identity-round-trip")?;
        if derived.stock_extra.len() != 10 {
            return Err(format!("identity-round-trip: the committed kit carries ten stock entries, the derived document has {}", derived.stock_extra.len()));
        }
        let reread = snapshot_of(&encode_curate_snapshot_json(&derived), "re-encoded", "identity-round-trip")?;
        if reread != derived {
            return Err(disagreement("identity-round-trip: encoding the document to JSON and decoding it again moved it", &reread, &derived));
        }
        let projection = projection(&derived)?;
        let text = fixture_text(ctx, &uri_in(ctx, "📚️examples")?)?;
        let parsed = parse_curate_dsl(&text)?;
        if parsed.stock_extra.len() != 10 || !parsed.curated.is_empty() {
            return Err(format!("identity-round-trip: the committed carrier carries ten stock entries against an empty curation, but parsed {} stock entr(ies) and [{}]", parsed.stock_extra.len(), curate_selection_summary(&parsed)));
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
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id. SUBJECT only:
/// the reference for every scenario here is the Python implementation beside this file, and
/// registering an oracle handler as well would put this repository's answer on both sides.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        let mut built = built;
        for kind in KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind));
            built = built.subject(&format!("inverse-{kind}"), subject::inverse(kind));
            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        return built.subject("identity-round-trip", subject::round_trip);
    }
    #[cfg(not(feature = "sut"))]
    {
        let _ = KINDS;
        built
    }
}
//#endregion 🔖️Registration
