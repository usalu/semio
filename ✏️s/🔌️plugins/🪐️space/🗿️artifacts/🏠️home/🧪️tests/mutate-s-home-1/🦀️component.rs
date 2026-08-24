//! 🏠️ `s.space.home` exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR. Recorded no-oracle decision `s-home-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`): the launcher artifact is
//! semio-NATIVE, its `.shome.dsl.semio`/`.pack.semio` envelopes are read by nobody else, and its one
//! mutable artifact-lane field is a counter that no interchange format models — so the candidate
//! category is empty rather than unexplored.
//!
//! The whole vocabulary is ONE root-scalar setter, which is exactly why the inverse law is the
//! load-bearing check here: a setter cannot be inverted structurally from its own diff, so the leaf
//! has to read the previous counter out of BASE and re-pin it, and `inverse-change-catalog-generation`
//! is what notices when it does not. The committed vector pins 3 → 7 rather than 0 → 1, so an
//! implementation that incremented lands on 4 and `mutate-change-catalog-generation` fails.
//!
//! **Where the assertions live.** A recorded no-oracle case runs NO oracle role — the runner resolves an
//! oracle implementation from the feature's `@oracle-` tag and this feature has none — so every law this
//! case claims is asserted inside the SUBJECT handlers, through the shared law module
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️component.rs` that the stdio subsets use. The oracle handlers
//! below still answer with the committed vector read literally, so the reference side exists the moment a
//! second producer ever does. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog, and that file's own
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against both the enum and the manifest.
const KINDS: &[&str] = &[
    "change-catalog-generation",
];

/// 👁️ Kinds whose COMMITTED specification vector cannot exhibit a forward effect, so
/// [`law::mutation_is_observable`] must not demand one of them.
/// Empty: this subset's one committed vector moves the counter, so its forward effect is visible.
const UNOBSERVABLE: &[&str] = &[

];

/// 🗣️ The real committed document this artifact ships as its own example.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read
/// literally via `include_str!`. This IS the independently handcrafted evidence the no-oracle decision
/// rests on — never recomputed here, never restated as a Rust literal.
struct Vector {
    before: &'static str,
    mutation: &'static str,
    after: &'static str,
    diff: &'static str,
    outcome: &'static str,
}

fn vector(kind: &str) -> Vector {
    match kind {
        "change-catalog-generation" => Vector {
            before: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/bumps-the-catalog-generation-to-7/📸️snapshot/⬅️before/🔣️component.json"),
            mutation: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/bumps-the-catalog-generation-to-7/🦠️mutation/🔣️component.json"),
            after: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/bumps-the-catalog-generation-to-7/📸️snapshot/➡️after/🔣️component.json"),
            diff: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/bumps-the-catalog-generation-to-7/🔺️diff/🔣️component.json"),
            outcome: include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/bumps-the-catalog-generation-to-7/🎯️outcome/🔣️component.json"),
        },
        other => panic!("mutate-s-home-1: no committed specification vector is registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-s-home-1: a committed fixture must be valid JSON: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed after-snapshot, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let after = vector(kind).after;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed before-snapshot — undoing a mutation must land back
/// exactly where the specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let before = vector(kind).before;
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{canonical, vector, DSL_ASSET, UNOBSERVABLE};
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_space::artifacts::home::standards::v1::subsets::any::schema::mutations::s_home_mutation_report_json;
    use semio_s_plugin_space::artifacts::home::standards::v1::subsets::any::schema::snapshot::s_home_identity_report_json;

    //#region 🔖️Report
    /// 📋️ One member of the production bridge's report, named in the error when it is absent — never
    /// defaulted, because a silently missing member would turn every comparison below into a comparison
    /// of two empty values.
    fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
        report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
    }

    /// 📋️ An array member of the report, rejecting a present-but-wrong-shaped value rather than
    /// treating it as empty.
    fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
        match member(report, key)? {
            Json::Array(items) => Ok(items.clone()),
            other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
        }
    }

    /// 📋️ A string member of the report, rejecting a present-but-wrong-shaped value.
    fn text(report: &Json, key: &str) -> Result<String, String> {
        match member(report, key)? {
            Json::String(value) => Ok(value.clone()),
            other => Err(format!("the report's {key:?} member is {}, not a string", other.to_string())),
        }
    }

    /// 📋️ A string array read as owned `String`s — an address list, either declared by a committed
    /// outcome or reported by a diagnostic.
    fn strings(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .map(|segment| match segment {
                Json::String(text) => text.clone(),
                other => other.to_string(),
            })
            .collect()
    }

    /// 🚦️ Normalizes a declared severity word. The committed outcome vectors are not consistent — some
    /// write `warn` where the serialized `Severity` writes `warning` — so the level is normalized before
    /// comparison while the `code`, which is a frozen closed-set identifier, is compared verbatim.
    fn level_of(word: &str) -> String {
        if word == "warn" {
            "warning".to_string()
        } else {
            word.to_string()
        }
    }

    /// 🎯️ Checks the produced diagnostics against the ones the committed `🎯️outcome` vector declares.
    /// A `rejected` vector declares one fault code and the offending address; an `applied` vector
    /// declares an ordered (possibly empty) message list and forbids anything at error level or worse.
    fn declared_outcome_holds(kind: &str, produced: &[Json], outcome: &Json) -> Result<(), String> {
        let codes: Vec<String> = produced.iter().map(|message| message.str("code")).collect();
        let levels: Vec<String> = produced.iter().map(|message| level_of(&message.str("level"))).collect();
        if outcome.str("status") == "rejected" {
            let expected = outcome.str("code");
            if codes != vec![expected.clone()] {
                return Err(format!("mutate-{kind}: the vector declares a rejection with code {expected:?}, the implementation raised {codes:?}"));
            }
            if !levels.iter().any(|level| level == "error" || level == "fatal") {
                return Err(format!("mutate-{kind}: the vector declares a rejection, but the implementation raised it at {levels:?} — a rejection is at least an error"));
            }
            let path = strings(outcome, "path");
            let target = strings(&produced[0], "target");
            if !path.is_empty() && target != path {
                return Err(format!("mutate-{kind}: the vector declares the offending address {path:?}, the implementation reported {target:?}"));
            }
            return Ok(());
        }
        let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
        if codes != expected {
            return Err(format!("mutate-{kind}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
        }
        match levels.iter().find(|level| level.as_str() == "error" || level.as_str() == "fatal") {
            Some(level) => Err(format!("mutate-{kind}: the vector declares an applied outcome, but the implementation raised a {level}")),
            None => Ok(()),
        }
    }
    //#endregion 🔖️Report

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to its committed before-snapshot and asserts THREE things the vector commits
    /// to: the resulting document is the committed after-snapshot, the produced delta is the committed
    /// `🔺️diff` (which pins WHICH fields the mutation was allowed to touch, not merely where it ended
    /// up), and the diagnostics are the ones the committed `🎯️outcome` declares. A kind the vector shows
    /// moving is additionally held to the observability law, so a mutation that quietly did nothing
    /// cannot pass by agreeing with an unchanged document.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = parse_json(&s_home_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("mutate-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let applied = member(&report, "snapshot")?;
            let expected = member(&report, "expectedSnapshot")?;
            if let Some(first) = law::divergence(applied, expected) {
                return Err(format!("mutate-{kind}: the applied document is not the committed after-snapshot — {first}"));
            }
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {
                return Err(format!("mutate-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
            }
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse must restore
    /// the committed before-snapshot exactly. Asserted in role through `law::inverse_restores`, so a
    /// divergence is reported by JSON path rather than as a bare inequality, and an inverse step that
    /// was itself rejected fails here rather than silently leaving the document where it was.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let committed = vector(kind);
            let report = parse_json(&s_home_mutation_report_json(committed.before, committed.mutation, committed.after).map_err(|error| format!("inverse-{kind}: the committed vector did not reach this subset's own codec: {error}"))?)?;
            let faults: Vec<String> = members(&report, "inverseMessages")?.iter().filter(|message| { let level = message.str("level"); level == "error" || level == "fatal" }).map(|message| message.str("code")).collect();
            if !faults.is_empty() {
                return Err(format!("inverse-{kind}: an inverse step was rejected with {faults:?}, so the document never got the chance to return"));
            }
            let restored = member(&report, "inverseSnapshot")?;
            law::inverse_restores(kind, restored, member(&report, "base")?)?;
            Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored.clone()))
        }
    }

    /// 🔁️ The real committed document through this subset's own two codecs. The semantic half is
    /// `law::round_trip_preserves`: parsing, printing back and parsing again must not move the
    /// projection. The byte half is `law::carrier_is_exact` rather than the wave's usual
    /// no-pass-through tripwire, and deliberately so — `store::ArtifactDsl`'s own documented LAW is that
    /// canonical `print_dsl` output is a `parse_dsl` fixpoint, so the correct answer for a second
    /// printing IS byte identity and anything else is the defect. Neither printing is compared against
    /// the committed file, which the same law explicitly allows to normalize on the way in. The pack
    /// decoding is a separate binary codec, so agreeing on one snapshot cannot be reached by carrying
    /// text bytes across.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let report = parse_json(&s_home_identity_report_json(&text).map_err(|error| format!("identity-round-trip: the committed example did not reach this subset's own codec: {error}"))?)?;
        let parsed = member(&report, "parsed")?;
        law::round_trip_preserves(member(&report, "reparsed")?, parsed)?;
        law::carrier_is_exact(text(&report, "canonicalTextAgain")?.as_bytes(), text(&report, "canonicalText")?.as_bytes())?;
        if let Some(first) = law::divergence(member(&report, "packDecoded")?, parsed) {
            return Err(format!("identity-round-trip: the binary codec decodes to a different document than the text codec — {first}"));
        }
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed.clone()))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario id,
/// so the loop mirrors the feature's `Examples` tables exactly; `identity-round-trip` is subject-only
/// because turning the committed example's DSL bytes into a document needs this subset's own codec,
/// which the oracle-only build must not link.
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
