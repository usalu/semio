//! 🦀️ Draw-document exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `draw-mutation-semantics` (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`):
//! `s.draw.draw` is a semio-NATIVE format with no third-party reader or writer, so `oracle` here
//! reads the committed, independently handcrafted per-kind specification vectors literally — no
//! recomputation, no second implementation of draw semantics — and `subject` drives this
//! repository's own vocabulary over all 14 `DrawMutation` variants.
//!
//! What this case has to hold on to is that a draw document is a RECURSIVE tree: `create-layer`,
//! `duplicate-layer` and `reorder-layer` address a parent plus an index, so the inverse law here is
//! about POSITION and not merely membership, and `law::divergence` reporting the first
//! JSON-path-qualified difference is what makes a wrong position readable instead of a bare
//! inequality.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law this case claims is asserted INSIDE the subject handler, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module: `law::divergence` names the first divergence by
//! JSON path, `law::mutation_is_observable` refuses a kind that moved nothing it is compared
//! through, `law::inverse_restores` is the inverse law itself, and `law::round_trip_preserves` plus
//! `law::carrier_is_exact` are the identity law's two halves. A handler that merely returned `Ok`
//! would report a pass having checked nothing at all.
//!
//! **How the fixture reaches typed values.** The generated host links only `semio-repo-test-host`,
//! the law module and — behind `sut` — this plugin's crate, whose `protocol`/`store`/`serde_json`
//! extern-crate aliases are private (`📦️glue.rs`). The oracle role therefore reads the committed
//! bytes with `include_str!` and the platform's own JSON reader, and the subject role hands the SAME
//! bytes to the production bridges `apply_draw_mutation_json`, `undo_draw_mutation_json` and
//! `round_trip_draw_dsl` that this subset's `🧬️schema/🧬️mutations/🦀️component.rs` exports for it.
//! The subject half is gated behind the generated host's `sut` feature so an oracle-only run never
//! compiles the local implementation.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in
/// `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog and
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "set-layer-visible",
    "set-layer-locked",
    "set-layer-opacity",
    "set-layer-blend-mode",
    "rename-layer",
    "update-layer-transform",
    "replace-layer-fill",
    "replace-layer-stroke",
    "set-layer-boolean-operation",
    "update-layer-trace-params",
    "create-layer",
    "duplicate-layer",
    "delete-layer",
    "reorder-layer",
];

/// 👁️ Kinds whose committed specification vector declares NO movement — a refusal or an accepted
/// no-op — so the observability law must not be claimed for them. Each one is named in
/// `component.feature`'s description with the reason, and each is still asserted, through
/// [`DECLARED_CODE`], to raise exactly the diagnostic its leaf declares.
const UNOBSERVABLE: &[&str] = &["duplicate-layer"];

/// 🚨️ The diagnostic code a declared no-op or refusal must raise, from the leaf's own committed
/// `🎯️outcome/🔣️component.json`. A vector that stopped raising it would otherwise be
/// indistinguishable from a mutation that quietly did nothing. Read only by the subject role —
/// the oracle role answers with the committed after-document, which already IS the declared outcome.
#[cfg(feature = "sut")]
const DECLARED_CODE: &[(&str, &str)] = &[("duplicate-layer", "mutation.target-missing")];

/// 🗣️ The real committed example this artifact ships — the identity law's input.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after)` specification vector for one kind, read literally
/// via `include_str!` — this IS the independently handcrafted evidence the no-oracle decision rests
/// on, never recomputed here.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "set-layer-visible" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/hides-shape-a/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/hides-shape-a/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/hides-shape-a/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "set-layer-locked" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/locks-shape-a/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/locks-shape-a/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/locks-shape-a/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "set-layer-opacity" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🧪️tests/dims-shape-a-to-half/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🧪️tests/dims-shape-a-to-half/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️set-layer-opacity/🧪️tests/dims-shape-a-to-half/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "set-layer-blend-mode" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/🧪️tests/normal-to-multiply/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/🧪️tests/normal-to-multiply/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-layer-blend-mode/🧪️tests/normal-to-multiply/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "rename-layer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/renames-shape-a-without-touching-its-id/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/renames-shape-a-without-touching-its-id/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/renames-shape-a-without-touching-its-id/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-layer-transform" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-layer-transform/🧪️tests/translates-and-scales-shape-a/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-layer-transform/🧪️tests/translates-and-scales-shape-a/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-layer-transform/🧪️tests/translates-and-scales-shape-a/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-layer-fill" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-layer-fill/🧪️tests/solid-to-linear-gradient/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-layer-fill/🧪️tests/solid-to-linear-gradient/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-layer-fill/🧪️tests/solid-to-linear-gradient/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "replace-layer-stroke" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-layer-stroke/🧪️tests/adds-a-dashed-stroke/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-layer-stroke/🧪️tests/adds-a-dashed-stroke/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/♻️replace-layer-stroke/🧪️tests/adds-a-dashed-stroke/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "set-layer-boolean-operation" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🧪️tests/union-to-subtract/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🧪️tests/union-to-subtract/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀set-layer-boolean-operation/🧪️tests/union-to-subtract/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "update-layer-trace-params" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧update-layer-trace-params/🧪️tests/sharpens-the-trace/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧update-layer-trace-params/🧪️tests/sharpens-the-trace/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧update-layer-trace-params/🧪️tests/sharpens-the-trace/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "create-layer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/appends-shape-b-at-the-root/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/appends-shape-b-at-the-root/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-layer/🧪️tests/appends-shape-b-at-the-root/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "duplicate-layer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧬️duplicate-layer/🧪️tests/rejects-a-missing-source-layer/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧬️duplicate-layer/🧪️tests/rejects-a-missing-source-layer/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧬️duplicate-layer/🧪️tests/rejects-a-missing-source-layer/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "delete-layer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/removes-group-a-with-its-child/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/removes-group-a-with-its-child/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-layer/🧪️tests/removes-group-a-with-its-child/📸️snapshot/➡️after/🔣️component.json"),
        ),
        "reorder-layer" => (
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-layer/🧪️tests/moves-shape-a-above-shape-b/📸️snapshot/⬅️before/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-layer/🧪️tests/moves-shape-a-above-shape-b/🦠️mutation/🔣️component.json"),
            include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃reorder-layer/🧪️tests/moves-shape-a-above-shape-b/📸️snapshot/➡️after/🔣️component.json"),
        ),
        other => panic!("mutate-draw-1: {other:?} is not a declared kind of this subset"),
    }
}

/// 🔣️ A committed fixture parsed through the platform's own JSON reader.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-draw-1: a committed fixture must be valid JSON: {error}"))
}

/// 🚨️ The diagnostic a kind's vector declares, if it declares one.
#[cfg(feature = "sut")]
fn declared_code(kind: &str) -> Option<&'static str> {
    DECLARED_CODE.iter().find(|(name, _)| *name == kind).map(|(_, code)| *code)
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER document, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, after) = fixture_text(kind);
        law::mutation_is_observable(kind, &canonical(after), &canonical(before), UNOBSERVABLE)?;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE document — undoing any mutation must land
/// exactly where its specification vector started.
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
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_draw::artifacts::draw::standards::v1::subsets::any::schema::mutations::{apply_draw_mutation_json, round_trip_draw_dsl, undo_draw_mutation_json};

    /// 📥️ Splits a bridge answer into the resulting document and the diagnostic codes it raised.
    fn answer(text: &str) -> Result<(Json, Vec<String>), String> {
        let value = parse_json(text)?;
        let document = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let codes = value.array("messages").iter().map(|code| match code {
            Json::String(text) => text.clone(),
            other => other.to_string(),
        }).collect();
        Ok((document, codes))
    }

    /// 🚨️ A declared no-op or refusal must raise exactly the code its leaf's committed outcome names.
    fn raised(kind: &str, codes: &[String]) -> Result<(), String> {
        match super::declared_code(kind) {
            None => Ok(()),
            Some(code) if codes.iter().any(|raised| raised == code) => Ok(()),
            Some(code) => Err(format!("mutate-{kind}: the committed vector declares the diagnostic {code:?}, but applying it raised {codes:?}")),
        }
    }

    /// 🎯️ Applies the kind to its committed before-document and asserts the result IS the committed
    /// after-document, that the mutation moved the compared projection unless its own vector declares
    /// otherwise, and that a declared refusal really was refused.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after) = super::fixture_text(kind);
            let (document, codes) = answer(&apply_draw_mutation_json(before, mutation)?)?;
            let expected = super::canonical(after);
            if let Some(first) = law::divergence(&document, &expected) {
                return Err(format!("mutate-{kind}: the applied document does not match the committed after-document — {first}"));
            }
            law::mutation_is_observable(kind, &document, &super::canonical(before), super::UNOBSERVABLE)?;
            raised(kind, &codes)?;
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }

    /// ↩️ The inverse law in role: applying the kind and then EVERY step of its own computed inverse
    /// must restore the committed before-document — member positions included, which is what a
    /// delete undone by re-appending would fail.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after) = super::fixture_text(kind);
            let (document, _codes) = answer(&undo_draw_mutation_json(before, mutation)?)?;
            law::inverse_restores(kind, &document, &super::canonical(before))?;
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }

    /// 🔁️ The identity law in role, on the real committed example. Its two halves are asserted
    /// separately: the reparsed document must agree with the first parse, and the reprinted text must
    /// reproduce the committed bytes. The byte half is `carrier_is_exact` rather than the wave's
    /// usual no-pass-through tripwire because the committed `🗣️example.dsl.semio` is this codec's OWN
    /// canonical output, committed as the artifact's example — reproducing it exactly is the correct
    /// answer here and any divergence is codec drift this case exists to catch.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(super::DSL_ASSET)?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let value = parse_json(&round_trip_draw_dsl(&text)?)?;
        let parsed = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let reparsed = value.get("reparsed").cloned().ok_or_else(|| "the bridge answer carries no reparsed document".to_string())?;
        law::round_trip_preserves(&reparsed, &parsed)?;
        law::carrier_is_exact(value.str("printed").as_bytes(), &input)?;
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors `component.feature`'s `Examples` tables exactly. `identity-round-trip` is
/// subject-only: the reference answer for every other scenario is a committed JSON document the
/// oracle role can read literally, but the real artifact is committed as DSL text ONLY and turning
/// that into a document needs this subset's own codec, which the oracle-only build must not link.
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
