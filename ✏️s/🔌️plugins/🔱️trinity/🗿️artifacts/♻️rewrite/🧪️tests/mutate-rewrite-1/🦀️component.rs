//! 🦀️ Rewrite-rule exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `rewrite-1-graph-rewrite-rule-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): `s.trinity.rewrite` is a
//! semio-NATIVE graph-rewrite rule with no third-party reader or writer, so `oracle` here reads the
//! committed, independently handcrafted per-kind specification fixtures
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/<slug>/🧪️tests/<fixture>/`)
//! literally — no recomputation, no reimplementation of mutation semantics. `subject` drives this
//! repository's own `apply_rewrite_mutation_reporting` over the full seven-kind
//! `RewriteRuleMutation` vocabulary.
//!
//! **The two disjoint shapes this case holds apart.** Three kinds replace an opaque JSON body
//! atomically; four edit one key of one of the two key-addressed maps. The feature's `moves` column
//! names the single projection member each kind may touch, and `only_named_member_moved` asserts
//! both halves — the named member moved AND every other member is byte-identical — so a body edit
//! that also touched a binding fails on the neighbour rather than passing on its own target.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role, so every law this
//! case claims is asserted INSIDE the subject handler. A handler that merely returned `Ok` would
//! report a pass having checked nothing at all.
//!
//! **Why the shared `⚖️law` module is not used here.** `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` is
//! reachable only where the stdio oracle crate is linked into the generated host, which happens for
//! a case whose owner sits under `✏️s/🔌️plugins/🗄️stdio`. This case's owner does not, and declaring
//! stdio's contribution directory as a host package for the rewrite artifact would make one
//! plugin's test tree a build dependency of another's. The laws are stated inline, in the same words
//! and with the same strictness.
//!
//! **How the fixture reaches typed values.** The generated test host links only
//! `semio-repo-test-host` and, behind `sut`, this plugin's own crate — no `serde`, no `serde_json`,
//! and this crate's `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`). The subset's
//! own production code exports the bridges instead: `decode_rewrite_snapshot_json`/
//! `encode_rewrite_snapshot_json`/`parse_rewrite_dsl`/`print_rewrite_dsl`/`rewrite_rule_summary`
//! (`…/🧬️schema/📸️snapshot/🦀️component.rs`) and `decode_rewrite_mutation_json`/
//! `apply_rewrite_mutation_reporting`/`inverse_rewrite_mutation_steps`
//! (`…/🧬️schema/🧬️mutations/🦀️component.rs`).

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `RewriteRuleMutation::KINDS` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
const KINDS: &[&str] = &["edit-before-fixture", "edit-lhs", "edit-rhs", "change-parameter-binding", "remove-parameter-binding", "change-rule-layout-point", "remove-rule-layout-point"];

/// 🗣️ The real committed rule — a two-piece Nakagin ground-floor before-fixture, a neighbour pattern
/// with a where-clause, one `label` binding and two layout points.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind,
/// read literally via `include_str!` — this IS the independently handcrafted vector the no-oracle
/// decision rests on, never recomputed.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "edit-before-fixture" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🧪️tests/swaps-in-a-two-node-before-graph/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🧪️tests/swaps-in-a-two-node-before-graph/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🧪️tests/swaps-in-a-two-node-before-graph/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️edit-before-fixture/🧪️tests/swaps-in-a-two-node-before-graph/🎯️outcome/🔣️component.json"),
        ),
        "edit-lhs" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🧪️tests/narrows-the-lhs-pattern-to-a-shaft-neighbour/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🧪️tests/narrows-the-lhs-pattern-to-a-shaft-neighbour/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🧪️tests/narrows-the-lhs-pattern-to-a-shaft-neighbour/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️edit-lhs/🧪️tests/narrows-the-lhs-pattern-to-a-shaft-neighbour/🎯️outcome/🔣️component.json"),
        ),
        "edit-rhs" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🧪️tests/rewrites-the-rhs-to-set-a-second-property/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🧪️tests/rewrites-the-rhs-to-set-a-second-property/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🧪️tests/rewrites-the-rhs-to-set-a-second-property/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️edit-rhs/🧪️tests/rewrites-the-rhs-to-set-a-second-property/🎯️outcome/🔣️component.json"),
        ),
        "change-parameter-binding" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🧪️tests/retitles-the-caption-binding/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🧪️tests/retitles-the-caption-binding/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🧪️tests/retitles-the-caption-binding/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-parameter-binding/🧪️tests/retitles-the-caption-binding/🎯️outcome/🔣️component.json"),
        ),
        "remove-parameter-binding" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🧪️tests/drops-the-repeat-binding/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🧪️tests/drops-the-repeat-binding/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🧪️tests/drops-the-repeat-binding/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️remove-parameter-binding/🧪️tests/drops-the-repeat-binding/🎯️outcome/🔣️component.json"),
        ),
        "change-rule-layout-point" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🧪️tests/nudges-the-capsule-var-off-the-shaft/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🧪️tests/nudges-the-capsule-var-off-the-shaft/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🧪️tests/nudges-the-capsule-var-off-the-shaft/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-rule-layout-point/🧪️tests/nudges-the-capsule-var-off-the-shaft/🎯️outcome/🔣️component.json"),
        ),
        "remove-rule-layout-point" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🧪️tests/clears-the-shaft-layout-point/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🧪️tests/clears-the-shaft-layout-point/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🧪️tests/clears-the-shaft-layout-point/📸️snapshot/➡️after/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️remove-rule-layout-point/🧪️tests/clears-the-shaft-layout-point/🎯️outcome/🔣️component.json"),
        ),
        other => panic!("mutate-rewrite-1: no specification vector registered for kind {other:?}"),
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
    use semio_s_plugin_trinity::artifacts::rewrite::standards::v1::subsets::any::schema::mutations::{apply_rewrite_mutation_reporting, decode_rewrite_mutation_json, inverse_rewrite_mutation_steps, RewriteRuleMutation};
    use semio_s_plugin_trinity::artifacts::rewrite::standards::v1::subsets::any::schema::snapshot::{decode_rewrite_snapshot_json, encode_rewrite_snapshot_json, parse_rewrite_dsl, print_rewrite_dsl, rewrite_rule_summary, RewriteSnapshot};

    //#region 🔖️FixtureDecode
    fn snapshot_of(text: &str, label: &str, kind: &str) -> Result<RewriteSnapshot, String> {
        decode_rewrite_snapshot_json(text).map_err(|error| format!("mutate-rewrite-1: the committed {label}-snapshot for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<RewriteRuleMutation, String> {
        decode_rewrite_mutation_json(text).map_err(|error| format!("mutate-rewrite-1: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &RewriteSnapshot) -> Result<Json, String> {
        parse_json(&encode_rewrite_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &RewriteSnapshot, expected: &RewriteSnapshot) -> String {
        format!("{what}\n     got: {} {}\nexpected: {} {}", rewrite_rule_summary(got), encode_rewrite_snapshot_json(got), rewrite_rule_summary(expected), encode_rewrite_snapshot_json(expected))
    }

    /// 🔢️ The projection's members as `(key, rendered value)` pairs — the granularity the feature's
    /// `moves` column is stated at.
    fn members(projection: &Json) -> Vec<(String, String)> {
        match projection {
            Json::Object(entries) => entries.iter().map(|(key, value)| (key.clone(), value.to_string())).collect(),
            other => vec![(String::new(), other.to_string())],
        }
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law, in the exact form this subset can state it: the ONE member the
    /// feature's `moves` column names must have moved, and every other member must be untouched.
    /// This is what holds the vocabulary's two disjoint shapes apart — a body edit that also
    /// rewrote a keyed map, or a keyed-map edit that reached its sibling map, fails here even though
    /// a whole-document comparison against a fixture written from the same bug would not.
    fn only_named_member_moved(kind: &str, moves: &str, base: &Json, mutated: &Json) -> Result<(), String> {
        let (before, after) = (members(base), members(mutated));
        if before.len() != after.len() {
            return Err(format!("mutate-{kind}: the mutation changed the projection's member SET ({} members in, {} out) — a rule mutation edits fields, it never adds or drops them", before.len(), after.len()));
        }
        let mut moved = Vec::new();
        for ((key, was), (other, now)) in before.iter().zip(after.iter()) {
            if key != other {
                return Err(format!("mutate-{kind}: the mutation reordered the projection ({key:?} became {other:?})"));
            }
            if was != now {
                moved.push(key.clone());
            }
        }
        if !moved.iter().any(|key| key == moves) {
            return Err(format!("mutate-{kind}: the feature declares this kind moves {moves:?}, but that member is unchanged — the scenario would report a pass for a mutation it never observed (moved: {moved:?})"));
        }
        if moved.len() != 1 {
            return Err(format!("mutate-{kind}: the feature declares this kind moves ONLY {moves:?}, but {} member(s) moved: {moved:?}", moved.len()));
        }
        Ok(())
    }

    /// 🎯️ The committed outcome claim: all seven vectors are clean `applied` vectors, so any
    /// diagnostic at all is a divergence.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("mutate-{kind}: every committed rewrite vector is a clean applied vector, but its outcome declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-snapshot and asserts, in role, that the result IS
    /// the committed after-snapshot, that only the declared member moved, and that the reported
    /// diagnostics are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let expected = snapshot_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_rewrite_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied snapshot does not match the committed after-snapshot"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            let (was, now) = (projection(&base)?, projection(&current)?);
            only_named_member_moved(kind, &ctx.doc_json()?.str("moves"), &was, &now)?;
            let projection = now;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must
    /// restore the committed before-snapshot exactly. The two `remove` kinds carry the weight here —
    /// their payload carries only a KEY, so the undo has to restore a value the mutation never
    /// travelled with.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = snapshot_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_rewrite_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the rule untouched, so restoring it proves nothing"));
            }
            for step in inverse_rewrite_mutation_steps(&mutation, &base) {
                let undone = apply_rewrite_mutation_reporting(&mut current, &step);
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

    /// 🔁️ The real committed rule through its own DSL carrier. `.rewrite.dsl.semio` is a
    /// fixed-layout record grammar with a handcrafted codec — quoted body strings, a
    /// `parameter-bindings` block, a `rule-layout` block and a fenced `rhs-json` code block — and the
    /// committed example is this codec's own output, committed as such. The wave's usual "output
    /// must not equal input" tripwire therefore does not apply and would be the wrong law here; the
    /// byte-exact law is asserted instead, together with a content check a parser returning an empty
    /// rule cannot satisfy.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(super::DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed rule artifact is not UTF-8: {error}"))?;
        let parsed = parse_rewrite_dsl(&text)?;
        if parsed.parameter_bindings.len() != 1 || parsed.rule_layout.len() != 2 || !parsed.before_fixture_json.contains("trinity.graph") || !parsed.lhs_json.contains("whereClause") {
            return Err(format!("identity-round-trip: the committed rule carries one binding, two layout points, a `trinity.graph` before-fixture and a pattern with a where-clause, but parsed {}", rewrite_rule_summary(&parsed)));
        }
        let printed = print_rewrite_dsl(&parsed);
        let reparsed = parse_rewrite_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement("identity-round-trip: printing the rule back to DSL and reparsing it lost content", &reparsed, &parsed));
        }
        if printed != text {
            let at = printed.as_bytes().iter().zip(text.as_bytes().iter()).position(|(one, other)| one != other);
            return Err(format!(
                "exact-bytes law violated: `.rewrite.dsl.semio` is a fixed-layout record grammar and the committed example is this codec's own output, so the re-printed text was required to reproduce it — {} byte(s) out against {} byte(s) in{}",
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
/// snapshot the oracle role can read literally, but the real rule is committed as `.dsl.semio` text
/// ONLY and turning that into a document needs this subset's own codec, which the oracle-only build
/// must not link.
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
