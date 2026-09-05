//! ♻️ `s.trinity.rewriting` exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the rule document and all seven typed mutations, written in Python
//! from this subset's committed snapshot schema, mutation grammar and specification vectors. This
//! adapter registers the SUBJECT half only: keeping oracle registrations here would put this
//! repository's answer on both sides of the comparison.
//!
//! A rewrite rule is five members: three whole JSON DOCUMENTS carried as strings — the before-fixture
//! graph, the left-hand pattern and the right-hand side — plus a map of parameter bindings and a map
//! of layout points. Seven verbs: three whole-value setters and a set/remove pair over each map.
//!
//! **What the two roles each hold.** The cross-language projection is the whole document; this
//! artifact composes no digest-derived child, so nothing has to be held back. The `.dsl.semio`
//! carrier's byte-exact law is asserted HERE, in role, on the artifact's committed example, because
//! the carrier mixes three value encodings with nothing stating which member gets which — a reading
//! the Python reference deliberately does not guess at.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `RewriteRuleMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["edit-before-fixture", "edit-lhs", "edit-rhs", "change-parameter-binding", "remove-parameter-binding", "change-rule-layout-point", "remove-rule-layout-point"];

//#endregion 🔖️Kinds


//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_trinity::artifacts::rewriting::standards::v1::subsets::any::schema::mutations::text::{apply_rewriting_mutation_reporting, decode_rewriting_mutation_json, inverse_rewriting_mutation_steps};
    use semio_s_plugin_trinity::artifacts::rewriting::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
    use semio_s_plugin_trinity::artifacts::rewriting::standards::v1::subsets::any::schema::snapshot::{decode_rewriting_snapshot_json, encode_rewriting_snapshot_json, parse_rewriting_dsl, print_rewriting_dsl, rewrite_rule_summary, RewritingSnapshot};

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
    fn fixture_text(ctx: &Context, needle: &str) -> Result<String, String> {
        let uri = uri_in(ctx, needle)?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the declared fixture {uri} is not UTF-8: {error}"))
    }

    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<RewritingSnapshot, String> {
        decode_rewriting_snapshot_json(text).map_err(|error| format!("mutate-rewriting-1: the {label} rule for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, label: &str, kind: &str) -> Result<RewriteRuleMutation, String> {
        decode_rewriting_mutation_json(text).map_err(|error| format!("mutate-rewriting-1: the {label} payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &RewritingSnapshot) -> Result<Json, String> {
        parse_json(&encode_rewriting_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &RewritingSnapshot, expected: &RewritingSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", rewrite_rule_summary(got), rewrite_rule_summary(expected))
    }
    //#endregion 🔖️Plan

    //#region 🔖️Laws
    /// 🔀️ Each verb writes exactly ONE of the five members. That is the check an after-snapshot
    /// comparison cannot make on its own: an implementation that re-serialized a JSON string member
    /// on every edit — reordering its keys or changing its whitespace — would still land on the right
    /// document for the member it meant to write while silently rewriting the other two.
    fn touches_one(scenario: &str, kind: &str, before: &RewritingSnapshot, after: &RewritingSnapshot) -> Result<(), String> {
        let written = match kind {
            "edit-before-fixture" => "beforeFixtureJson",
            "edit-lhs" => "lhsJson",
            "edit-rhs" => "rhsJson",
            "change-parameter-binding" | "remove-parameter-binding" => "parameterBindings",
            _ => "ruleLayout",
        };
        let (was, now) = (projection(before)?, projection(after)?);
        let moved: Vec<String> = ["beforeFixtureJson", "lhsJson", "rhsJson", "parameterBindings", "ruleLayout"].iter().filter(|name| was.get(name) != now.get(name)).map(|name| (*name).to_string()).collect();
        if moved != vec![written.to_string()] {
            return Err(format!("{scenario}: this verb writes {written} and nothing else, but {moved:?} moved"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL derived Nakagin ground-floor rule with the parameters the
    /// feature states, and asserts in role that it moved the rule and wrote exactly one member.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = snapshot_of(&fixture_text(ctx, "nakagin-capsule-tower")?, "derived", kind)?;
            let payload = mutation_of(ctx.doc_string()?, "feature", kind)?;
            let mut current = base.clone();
            let raised = apply_rewriting_mutation_reporting(&mut current, &payload);
            if !raised.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters were rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("mutate-{kind}: the forward mutation left the rule untouched, so nothing was proved"));
            }
            touches_one(&format!("mutate-{kind}"), kind, &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ Applies one kind to the REAL derived rule and then EVERY step of its OWN computed inverse.
    /// The projection carries BOTH rules: projecting only the restored one would make all seven rows
    /// project the same value and the differential would be vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = snapshot_of(&fixture_text(ctx, "nakagin-capsule-tower")?, "derived", kind)?;
            let payload = mutation_of(ctx.doc_string()?, "feature", kind)?;
            let mut current = base.clone();
            let raised = apply_rewriting_mutation_reporting(&mut current, &payload);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the rule untouched, so restoring it proves nothing"));
            }
            let mutated = projection(&current)?;
            for step in inverse_rewriting_mutation_steps(&payload, &base) {
                let undone = apply_rewriting_mutation_reporting(&mut current, &step);
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
    /// fixtures — the same three files the Python reference reads. All seven are accepting, so each
    /// must reach the committed after-rule, move it, write exactly one member and invert cleanly.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let base = snapshot_of(&fixture_text(ctx, "⬅️before")?, "committed before", kind)?;
            let expected = snapshot_of(&fixture_text(ctx, "➡️after")?, "committed after", kind)?;
            let vector = mutation_of(&fixture_text(ctx, "🦠️mutation")?, "committed vector", kind)?;
            let mut current = base.clone();
            let raised = apply_rewriting_mutation_reporting(&mut current, &vector);
            if !raised.is_empty() {
                return Err(format!("spec-vector-{kind}: every committed rewriting vector is a clean applied vector, but the implementation raised {raised:?}"));
            }
            if current != expected {
                return Err(disagreement(&format!("spec-vector-{kind}: the applied rule does not match the committed after-rule"), &current, &expected));
            }
            if current == base {
                return Err(format!("spec-vector-{kind}: the committed vector left the rule untouched, so nothing was proved"));
            }
            touches_one(&format!("spec-vector-{kind}"), kind, &base, &current)?;
            for step in inverse_rewriting_mutation_steps(&vector, &base) {
                let undone = apply_rewriting_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("spec-vector-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("spec-vector-{kind}: applying the vector and then its own inverse did not restore the committed before-rule"), &current, &base));
            }
            let projection = projection(&expected)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ Two identities in one scenario, because they can only be asserted in two different places.
    ///
    /// The CARRIER identity is Rust-only and asserted here in role, on the artifact's own committed
    /// example: `.rewriting.dsl.semio` is a fixed-layout document and the committed example is this
    /// codec's own output, committed as such, so the re-printed text is required to reproduce it byte
    /// for byte. The DOCUMENT identity is what the Python reference can also produce: the five
    /// members this subset's own JSON codec reads out of the derived real rule.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = fixture_text(ctx, "📚️examples")?;
        let parsed = parse_rewriting_dsl(&text)?;
        let printed = print_rewriting_dsl(&parsed);
        let reparsed = parse_rewriting_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the rule back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        if printed != text {
            let at = printed.as_bytes().iter().zip(text.as_bytes().iter()).position(|(one, other)| one != other);
            return Err(format!(
                "exact-bytes law violated: `.rewriting.dsl.semio` is a fixed-layout document and the committed example is this codec's own output, so the re-printed text was required to reproduce it — {} byte(s) out against {} byte(s) in{}",
                printed.len(),
                text.len(),
                match at {
                    Some(offset) => format!(" (first at byte {offset})"),
                    None => String::new(),
                }
            ));
        }
        let ground_floor = snapshot_of(&fixture_text(ctx, "nakagin-ground-floor")?, "ground floor", "identity-round-trip")?;
        let derived = snapshot_of(&fixture_text(ctx, "nakagin-capsule-tower")?, "capsule tower", "identity-round-trip")?;
        for (rule, what) in [(&ground_floor, "ground floor"), (&derived, "capsule tower")] {
            let reread = snapshot_of(&encode_rewriting_snapshot_json(rule), "re-encoded", "identity-round-trip")?;
            if &reread != rule {
                return Err(disagreement(&format!("identity-round-trip: encoding the {what} rule to JSON and decoding it again moved it"), &reread, rule));
            }
        }
        let projection = Json::Object(vec![("groundFloor".to_string(), projection(&ground_floor)?), ("capsuleTower".to_string(), projection(&derived)?)]);
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
