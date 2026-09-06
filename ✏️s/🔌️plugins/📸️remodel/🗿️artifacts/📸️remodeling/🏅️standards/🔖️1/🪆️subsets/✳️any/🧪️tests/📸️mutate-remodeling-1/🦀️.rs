//! 🦀️ Remodeling-scene exhaustive mutation case — Rust adapter, the SUBJECT half of a
//! cross-language differential. The reference is `🐍️component.py` beside this file, registered as
//! oracle `remodeling-1-python-independent`; this half drives the production bridges
//! `apply_remodeling_mutation_json`, `undo_remodeling_mutation_json` and `round_trip_remodeling_dsl`
//! that `../../🧬️schema/🧬️mutations/🦀️.rs` exports, and asserts in role every law a byte comparison
//! against the reference cannot reach.
//!
//! **Where a vector lives is the FEATURE's answer, not this file's.** Every scenario carries a doc
//! string naming its `(before, mutation, after)` triple as `asset://` (or, for
//! `commit-reconstruction`, `local://`) URIs, and they are resolved through the test context at RUN
//! time. Nothing here transcribes a fixture path: the 2026-09-05 repo-wide path-shortening pass
//! renamed every case directory under this subset and left 107 compile-time `include_str!` literals
//! addressing names that no longer existed, which is exactly the drift runtime resolution cannot
//! repeat. The plan pins each file's digest, so a silently edited vector changes the plan rather
//! than the answer.
//!
//! **What is asserted, through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module.**
//! `law::divergence` names the first divergence by JSON path, `law::mutation_is_observable` refuses
//! a kind that moved nothing, `law::inverse_restores` is the inverse law itself, and
//! `law::round_trip_preserves` plus `law::carrier_is_exact` are the identity law's two halves.
//!
//! **`commit-reconstruction` is the one kind with no committed leaf triple**, because its diff reads
//! process-global staging state (`commit_staged_remodeling_reconstruction`) that a triple cannot
//! carry. Its vector lives in this case's own `🧫️fixtures/`, its provenance is written into
//! `component.feature`, and it exercises the kind's documented refusal path: the doc string names
//! the diagnostic the vector declares, so a vector that stopped raising it fails here rather than
//! passing as a mutation that quietly did nothing.
//!
//! **How the fixture reaches typed values.** The generated host links only `semio_repo_test_host`,
//! the law module and — behind `sut` — this plugin's crate, whose `protocol`/`store` extern-crate
//! aliases are private. The whole adapter is therefore gated on `sut`: this half runs in the SUBJECT
//! role only, and the runner turns that feature on for exactly that role.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in `../../🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not imported, because
/// the host may not reach into the subject crate outside the `sut` feature. The contract's
/// mutation-coverage gate keeps this list honest against the catalog and
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "create-stream",
    "delete-stream",
    "change-stream-sync",
    "add-stream-frame",
    "remove-stream-frame",
    "replace-stream-source",
    "create-asset",
    "delete-asset",
    "create-camera-calibration",
    "update-camera-calibration",
    "delete-camera-calibration",
    "create-rig-extrinsic",
    "delete-rig-extrinsic",
    "update-rig-extrinsic",
    "create-gcp",
    "delete-gcp",
    "add-gcp-observation",
    "remove-gcp-observation",
    "update-ingest-params",
    "update-feature-params",
    "update-match-params",
    "update-sfm-params",
    "update-dense-params",
    "update-mesh-params",
    "update-motion-params",
    "update-geo-params",
    "replace-job",
    "replace-sparse",
    "replace-dense",
    "replace-mesh-result",
    "replace-trajectory",
    "replace-tracks",
    "replace-geo-products",
    "replace-qc",
    "commit-reconstruction",
];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_remodeling::artifacts::remodeling::standards::v1::subsets::any::schema::mutations::{apply_remodeling_mutation_json, round_trip_remodeling_dsl, undo_remodeling_mutation_json};

    /// 🧫️ One specification vector, addressed entirely by the scenario's own doc string. The three
    /// documents are kept as TEXT as well as parsed: the production bridges take the committed bytes,
    /// so re-serializing a parsed copy would hand them a document this repository wrote rather than
    /// the one the fixture commits.
    pub struct Vector {
        pub kind: String,
        pub before_text: String,
        pub mutation_text: String,
        pub before: Json,
        pub after: Json,
        pub code: Option<String>,
    }

    /// 🧫️ A declared fixture's committed bytes as UTF-8 text.
    fn text(ctx: &Context, uri: &str) -> Result<String, String> {
        String::from_utf8(ctx.fixture_bytes(uri)?).map_err(|error| format!("the committed fixture {uri} is not UTF-8: {error}"))
    }

    /// 📜️ The vector the scenario's doc string addresses. A `code` member marks a vector whose
    /// documented answer is a REFUSAL, which inverts what the observability law may demand.
    pub fn vector(ctx: &Context) -> Result<Vector, String> {
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        if !super::KINDS.contains(&kind.as_str()) {
            return Err(format!("the scenario doc string names {kind:?}, which is not a declared RemodelingMutation kind"));
        }
        let code = match spec.get("code") {
            Some(Json::String(declared)) => Some(declared.clone()),
            _ => None,
        };
        let before_text = text(ctx, &spec.str("before"))?;
        let mutation_text = text(ctx, &spec.str("mutation"))?;
        let after_text = text(ctx, &spec.str("after"))?;
        Ok(Vector { kind, before: parse_json(&before_text)?, after: parse_json(&after_text)?, before_text, mutation_text, code })
    }

    /// 📥️ Splits a bridge answer into the resulting document and the diagnostic codes it raised.
    fn answer(text: &str) -> Result<(Json, Vec<String>), String> {
        let value = parse_json(text)?;
        let document = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let codes = value
            .array("messages")
            .iter()
            .map(|code| match code {
                Json::String(text) => text.clone(),
                other => other.to_string(),
            })
            .collect();
        Ok((document, codes))
    }

    /// 🚨️ A vector that declares a refusal must raise exactly the diagnostic it names.
    fn raised(vector: &Vector, codes: &[String]) -> Result<(), String> {
        match &vector.code {
            None => Ok(()),
            Some(code) if codes.iter().any(|actual| actual == code) => Ok(()),
            Some(code) => Err(format!("mutate-{}: the vector declares the diagnostic {code:?}, but applying it raised {codes:?}", vector.kind)),
        }
    }

    /// 🎯️ Applies the kind to its committed before-document and asserts the result IS the committed
    /// after-document, that the mutation moved the compared projection unless its own vector declares
    /// a refusal, and that a declared refusal really was refused.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let vector = vector(ctx)?;
        let (document, codes) = answer(&apply_remodeling_mutation_json(&vector.before_text, &vector.mutation_text)?)?;
        if let Some(first) = law::divergence(&document, &vector.after) {
            return Err(format!("mutate-{}: the applied document does not match the committed after-document — {first}", vector.kind));
        }
        raised(&vector, &codes)?;
        if vector.code.is_none() {
            law::mutation_is_observable(&vector.kind, &document, &vector.before, &[])?;
        } else if law::divergence(&document, &vector.before).is_some() {
            return Err(format!("mutate-{}: the vector declares a refusal, so the document must be left untouched", vector.kind));
        }
        Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
    }

    /// ↩️ The inverse law in role: applying the kind and then EVERY step of its own computed inverse
    /// must restore the committed before-document — member positions included, which is what a
    /// delete undone by re-appending would fail.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let vector = vector(ctx)?;
        let (document, _codes) = answer(&undo_remodeling_mutation_json(&vector.before_text, &vector.mutation_text)?)?;
        law::inverse_restores(&vector.kind, &document, &vector.before)?;
        Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
    }

    /// 🔁️ The identity law in role, on the real committed example the scenario's doc string names.
    /// Its two halves are asserted separately: the reparsed document must agree with the first parse
    /// — a document comparison the reference cannot make, because this subset's committed text
    /// grammar is the repository-wide placeholder — and the reprinted text must reproduce the
    /// committed bytes, which is the projection the reference answers with. The byte half is
    /// `carrier_is_exact` rather than the wave's usual no-pass-through tripwire because the committed
    /// `🗣️.dsl.semio` is this codec's OWN canonical output: reproducing it exactly is the correct
    /// answer, and any divergence is codec drift this case exists to catch.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let carrier = ctx.doc_json()?.str("carrier");
        let input = ctx.fixture_bytes(&carrier)?;
        let source = String::from_utf8(input.clone()).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let value = parse_json(&round_trip_remodeling_dsl(&source)?)?;
        let parsed = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let reparsed = value.get("reparsed").cloned().ok_or_else(|| "the bridge answer carries no reparsed document".to_string())?;
        law::round_trip_preserves(&reparsed, &parsed)?;
        let printed = value.str("printed");
        law::carrier_is_exact(printed.as_bytes(), &input)?;
        Ok(Outcome::with_raw(printed.clone().into_bytes(), Json::String(printed)))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id, so the loop
/// mirrors `component.feature`'s `Examples` tables exactly. Every handler is registered in the
/// SUBJECT role alone: the oracle role belongs to the registered Python reference, and a Rust
/// handler placed there would compare this codec with itself. The registrations are gated on `sut`
/// because they all reach production bridges, and the runner enables that feature for exactly the
/// role that needs them.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
