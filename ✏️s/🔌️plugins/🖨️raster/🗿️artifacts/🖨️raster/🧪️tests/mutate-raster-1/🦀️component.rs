//! 🖨️ `s.raster.raster` exhaustive mutation case — Rust SUBJECT adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! This case is a CROSS-LANGUAGE DIFFERENTIAL. The reference is `🐍️component.py` beside this file —
//! a second implementation of the layered document and all twelve typed mutations, written in Python
//! from this subset's committed mutation schema, mutation grammar and specification vectors. This
//! adapter registers the SUBJECT half only: keeping oracle registrations here would put this
//! repository's answer on both sides of the comparison.
//!
//! `s.raster.raster` is a LAYERED DOCUMENT, not an image file. Its pixels live behind ids in a root
//! `assets` pool and its vocabulary edits the layer tree around them, so the raster crates this
//! repository links (`png`, `image`, `tiff`) read a different artifact entirely and Pillow, which
//! does read the payloads, has no authority over a group's `children` or a blend mode.
//!
//! **What the two roles each hold.** The cross-language projection is the whole document, which for
//! this artifact composes no digest-derived child and therefore needs nothing held back. The
//! diagnostic CODES a declared no-op or refusal must raise are Rust-side report shapes rather than
//! parts of the document, so [`DECLARED_CODE`] is still asserted here, in role, exactly as before
//! the conversion — as is the `.dsl.semio` carrier's fixpoint law on the committed example.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `KINDS` in
/// `../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The contract's
/// mutation-coverage gate keeps this list honest against the catalog and
/// `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against the enum.
const KINDS: &[&str] = &[
    "create-layer",
    "delete-layer",
    "reorder-layers",
    "rename-layer",
    "change-layer-visible",
    "change-layer-opacity",
    "change-layer-blend-mode",
    "move-layer",
    "resize-layer",
    "change-layer-adjustment-kind",
    "add-layer-asset",
    "remove-layer-asset",
];

/// 👁️ Kinds whose committed specification vector declares NO movement — a refusal or an accepted
/// no-op — so the observability law must not be claimed for them. Each one is named in
/// `component.feature`'s description with the reason, and each is still asserted, through
/// [`DECLARED_CODE`], to raise exactly the diagnostic its leaf declares.
const UNOBSERVABLE: &[&str] = &["add-layer-asset", "remove-layer-asset"];

/// 🚨️ The diagnostic code a declared no-op or refusal must raise, from the leaf's own committed
/// `🎯️outcome/🔣️component.json`. A vector that stopped raising it would otherwise be
/// indistinguishable from a mutation that quietly did nothing. Read only by the subject role —
/// the oracle role answers with the committed after-document, which already IS the declared outcome.
#[cfg(feature = "sut")]
const DECLARED_CODE: &[(&str, &str)] = &[("remove-layer-asset", "mutation.target-missing"), ("add-layer-asset", "mutation.no-op")];

//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🚨️ The diagnostic code a declared no-op or refusal must raise, from the leaf's own committed
/// `🎯️outcome/🔣️component.json`. A vector that stopped raising it would otherwise be
/// indistinguishable from a mutation that quietly did nothing. The three snapshot and mutation files
/// themselves are read through the plan's declared fixtures instead, so both implementations read
/// the SAME bytes and neither holds a transcription that could drift from what the other one read.
#[cfg(feature = "sut")]
fn declared_code(kind: &str) -> Option<&'static str> {
    DECLARED_CODE.iter().find(|(name, _)| *name == kind).map(|(_, code)| *code)
}
//#endregion 🔖️Fixtures

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_raster::artifacts::raster::standards::v1::subsets::any::schema::mutations::{apply_raster_mutation_json, round_trip_raster_dsl, undo_raster_mutation_json};
    use semio_s_plugin_stdio_test_oracle::law;

    //#region 🔖️Plan
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

    /// 🚨️ A declared no-op or refusal must raise exactly the code its leaf's committed outcome names.
    fn raised(kind: &str, codes: &[String]) -> Result<(), String> {
        match super::declared_code(kind) {
            None => Ok(()),
            Some(code) if codes.iter().any(|raised| raised == code) => Ok(()),
            Some(code) => Err(format!("spec-vector-{kind}: the committed vector declares the diagnostic {code:?}, but applying it raised {codes:?}")),
        }
    }
    //#endregion 🔖️Plan

    //#region 🔖️Handlers
    /// 🎯️ Applies one kind to the REAL derived Semio demo board with the parameters the feature
    /// states. The observability law is asserted here in role: every real-document row edits the tree
    /// or the asset pool, so a mutation that quietly did nothing cannot pass by agreeing with an
    /// unchanged document.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let before = fixture_text(ctx, "semio-demo-board")?;
            let (document, codes) = answer(&apply_raster_mutation_json(&before, ctx.doc_string()?)?)?;
            if !codes.is_empty() {
                return Err(format!("mutate-{kind}: the feature's parameters raised {codes:?}"));
            }
            law::mutation_is_observable(kind, &document, &parse_json(&before)?, &[])?;
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }

    /// ↩️ Applies one kind to the REAL derived board and then EVERY step of its own computed inverse.
    /// The projection carries BOTH documents: projecting only the restored one would make all twelve
    /// rows project the same value and the differential would be vacuous.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let before = fixture_text(ctx, "semio-demo-board")?;
            let (mutated, codes) = answer(&apply_raster_mutation_json(&before, ctx.doc_string()?)?)?;
            if !codes.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation raised {codes:?}"));
            }
            law::mutation_is_observable(kind, &mutated, &parse_json(&before)?, &[])?;
            let (restored, _codes) = answer(&undo_raster_mutation_json(&before, ctx.doc_string()?)?)?;
            law::inverse_restores(kind, &restored, &parse_json(&before)?)?;
            let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), restored)]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 📐️ Replays one committed handcrafted specification vector, read through the plan's declared
    /// fixtures — the same three files the Python reference reads. The feature's `verdict` column
    /// states which of the three answers the vector commits to: `applied` must reach the committed
    /// after-document AND move it, `noop` must reach it WITHOUT moving it, and `refused` must leave
    /// the document alone. All three additionally require the diagnostic the leaf's own committed
    /// `🎯️outcome` declares, which is what keeps a declared no-op distinguishable from a mutation
    /// that quietly did nothing.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let before = fixture_text(ctx, "⬅️before")?;
            let mutation = fixture_text(ctx, "🦠️mutation")?;
            let after = fixture_text(ctx, "➡️after")?;
            let verdict = ctx.doc_json()?.str("verdict");
            let (document, codes) = answer(&apply_raster_mutation_json(&before, &mutation)?)?;
            let expected = parse_json(&after)?;
            if let Some(first) = law::divergence(&document, &expected) {
                return Err(format!("spec-vector-{kind}: the applied document does not match the committed after-document — {first}"));
            }
            raised(kind, &codes)?;
            match verdict.as_str() {
                "applied" => {
                    law::mutation_is_observable(kind, &document, &parse_json(&before)?, &[])?;
                    let (restored, _codes) = answer(&undo_raster_mutation_json(&before, &mutation)?)?;
                    law::inverse_restores(kind, &restored, &parse_json(&before)?)?;
                }
                "noop" | "refused" => {
                    if let Some(first) = law::divergence(&document, &parse_json(&before)?) {
                        return Err(format!("spec-vector-{kind}: the committed vector declares a {verdict}, but the document moved — {first}"));
                    }
                }
                other => return Err(format!("spec-vector-{kind}: the feature declares an unknown verdict {other:?}")),
            }
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }

    /// 🔁️ Two identities in one scenario, because they can only be asserted in two different places.
    ///
    /// The CARRIER identity is Rust-only and asserted here in role, on the artifact's own committed
    /// example: the reparsed document must agree with the first parse, and the reprinted text must
    /// reproduce the committed bytes. The byte half is `carrier_is_exact` rather than the wave's
    /// usual no-pass-through tripwire because the committed `🗣️example.dsl.semio` is this codec's OWN
    /// canonical output, committed as the artifact's example.
    ///
    /// The DOCUMENT identity is what the Python reference can also produce: the document this
    /// subset's own JSON codec reads out of the derived real board. The feature's doc string carries
    /// a `changeLayerVisible` payload naming the value the board ALREADY holds, which is the only way
    /// to reach the bridge's decode without applying an edit.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(&uri_in(ctx, "📚️examples")?)?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let value = parse_json(&round_trip_raster_dsl(&text)?)?;
        let parsed = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let reparsed = value.get("reparsed").cloned().ok_or_else(|| "the bridge answer carries no reparsed document".to_string())?;
        law::round_trip_preserves(&reparsed, &parsed)?;
        law::carrier_is_exact(value.str("printed").as_bytes(), &input)?;
        let derived = fixture_text(ctx, "semio-demo-board")?;
        let (document, codes) = answer(&apply_raster_mutation_json(&derived, ctx.doc_string()?)?)?;
        if let Some(first) = law::divergence(&document, &parse_json(&derived)?) {
            return Err(format!("identity-round-trip: the doc string must name the value the derived board already holds, but applying it moved the document — {first}"));
        }
        if !codes.is_empty() && super::declared_code("add-layer-asset") != Some(codes[0].as_str()) {
            return Err(format!("identity-round-trip: reading the derived board raised {codes:?}"));
        }
        Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
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
        let _ = (KINDS, UNOBSERVABLE);
        built
    }
}
//#endregion 🔖️Registration
