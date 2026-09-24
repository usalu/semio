//! 🦀️ Remodeling-scene exhaustive mutation case — Rust adapter, the SUBJECT half of a
//! cross-language differential. The reference is `🐍️component.py` beside this file, registered as
//! oracle `remodeling-1-python-independent`; this half drives the production bridges
//! `apply_remodeling_mutation_json`, `undo_remodeling_mutation_json` and `round_trip_remodeling_dsl`
//! that `../../🧬️schema/🧬️mutations/🦀️.rs` exports, and asserts in role every law a byte comparison
//! against the reference cannot reach.
//!
//! **Where a vector lives is the FEATURE's answer, not this file's.** Every scenario carries a doc
//! string naming its `(before, mutation, after)` triple as `shared://` URIs, and they are resolved
//! through the test context at RUN time. Nothing here transcribes a fixture path: the 2026-09-05 repo-wide path-shortening pass
//! renamed every case directory under this subset and left 107 compile-time `include_str!` literals
//! addressing names that no longer existed, which is exactly the drift runtime resolution cannot
//! repeat. The plan pins each file's digest, so a silently edited vector changes the plan rather
//! than the answer.
//!
//! **What is asserted, through the shared `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/⚖️law` module.**
//! `law::divergence` names the first divergence by JSON path, `law::mutation_is_observable` refuses
//! a kind that moved nothing, `law::inverse_restores` is the inverse law itself, and
//! `law::round_trip_preserves` plus `law::carrier_is_exact` are the identity law's two halves.
//!
//! **`commit-reconstruction` is an ordinary document kind.** It binds durable content the BASE
//! document already stores through `append-content`, so its leaf triples carry everything its diff
//! reads. It additionally keeps one shared vector in this owner's `🧫️fixtures/🏁️commit-reconstruction/`
//! — a sparse cloud naming unpublished content — whose doc string names the diagnostic it declares,
//! so a vector that stopped raising it fails here rather than passing as a mutation that quietly did
//! nothing.
//!
//! **How the fixture reaches typed values.** The generated host links only `semio_repo_test_host`,
//! the law module and — behind `sut` — this plugin's crate, whose `protocol`/`store` extern-crate
//! aliases are private. The whole adapter is therefore gated on `sut`: this half runs in the SUBJECT
//! role only, and the runner turns that feature on for exactly that role.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
/// 🏷️ Mirrors `KINDS` in `../../🧬️schema/🧬️mutations/🦀️.rs` — duplicated, not imported, because
/// the host may not reach into the subject crate outside the `sut` feature. The contract's
/// mutation-coverage gate keeps this list honest against the catalog.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &[
    "add-gcp-observation",
    "add-stream-frame",
    "append-content",
    "change-stream-sync",
    "commit-reconstruction",
    "create-asset",
    "create-camera-calibration",
    "create-gcp",
    "create-rig-extrinsic",
    "create-stream",
    "delete-asset",
    "delete-camera-calibration",
    "delete-gcp",
    "delete-rig-extrinsic",
    "delete-stream",
    "remove-content",
    "remove-gcp-observation",
    "remove-stream-frame",
    "replace-dense",
    "replace-geo-products",
    "replace-mesh-result",
    "replace-qc",
    "replace-sparse",
    "replace-stream-source",
    "replace-tracks",
    "replace-trajectory",
    "update-camera-calibration",
    "update-dense-params",
    "update-feature-params",
    "update-geo-params",
    "update-ingest-params",
    "update-match-params",
    "update-mesh-params",
    "update-motion-params",
    "update-rig-extrinsic",
    "update-sfm-params",
];

/// ▶️ Every `@id-mutate` row `🥒️.feature` plans, applied and refused alike — generated with the
/// feature itself, so a row that gains or loses a vector cannot leave this list behind.
#[cfg(feature = "sut")]
const MUTATE_SCENARIOS: &[&str] = &[
    "adds-a-control-298de4",
    "adds-a-fourth-97e912",
    "adds-a-quay-7569de",
    "adds-a-rig-2df5df",
    "adds-a-third-61fb5d",
    "adds-an-unbound-2b2373",
    "adds-gcp-tower-d71a54",
    "adds-stream-c-458900",
    "adds-the-cam-c-82c8fb",
    "adds-the-dtm-and-64d5bb",
    "adds-the-first-05b1b5",
    "appends-a-third-8ac259",
    "appends-an-0c2164",
    "appends-sparse-leaves",
    "attaches-a-607df8",
    "clears-every-760061",
    "clears-the-d2f81a",
    "clears-the-geo-f4886e",
    "clears-the-qc-1d2249",
    "clears-the-video-143f2b",
    "commit-reconstruction",
    "doubles-the-c245d5",
    "drops-the-6436a8",
    "drops-the-cam-a-a1f8a2",
    "drops-the-content",
    "drops-the-first-9ebf0b",
    "drops-the-first-d98a0f",
    "drops-the-middle-282fb7",
    "drops-the-middle-2d6d53",
    "drops-the-spare-c6ffb6",
    "enables-georefere-18a68a",
    "enables-motion-2444a3",
    "files-a-qc-report-64d222",
    "halves-the-002a17",
    "halves-the-voxel-21b53d",
    "moves-the-3621f6",
    "overwrites-an-a34b9d",
    "picks-the-south-eb0c4d",
    "places-the-0d0b8d",
    "raises-the-dense-ddb263",
    "records-a-dtm-6e132a",
    "records-a-qc-f5caf4",
    "refines-the-9fd25a",
    "refines-the-cam-0eaef0",
    "refuses-a-19c1ab",
    "refuses-a-48f3a6",
    "refuses-a-camera-e92a02",
    "refuses-a-d82e38",
    "refuses-a-frame-81beea",
    "refuses-a-frame-e7c374",
    "refuses-a-gap",
    "refuses-a-ratio-65dcb9",
    "refuses-a-rig-cb71ba",
    "refuses-a-second-95e04d",
    "refuses-a-zero-fa917f",
    "refuses-an-109cf1",
    "refuses-an-59752a",
    "refuses-an-asset-cb0d4b",
    "refuses-missing-content",
    "refuses-to-12366b",
    "refuses-to-1805df",
    "refuses-to-2cfb53",
    "refuses-to-3c20ff",
    "refuses-to-5c6f74",
    "refuses-to-73655a",
    "refuses-to-8095d3",
    "refuses-to-b60a39",
    "refuses-to-c4563a",
    "refuses-to-c93e98",
    "refuses-to-clear-30cbb5",
    "refuses-to-clear-524569",
    "refuses-to-clear-b8c54a",
    "refuses-to-f7f40d",
    "refuses-to-f9541f",
    "refuses-to-pick-3c0570",
    "refuses-to-remove-3c8f32",
    "refuses-to-remove-422a37",
    "reingests-the-311c32",
    "rejects-a-6b58da",
    "rejects-a-stream-aac5c2",
    "rejects-an-2e5568",
    "rejects-an-5d3a60",
    "rejects-an-e9fa51",
    "removes-an-8f3868",
    "removes-gcp-209b7d",
    "removes-the-40cba4",
    "removes-the-cam-f90b89",
    "removes-the-first-c0fc2a",
    "removes-the-last-304bdf",
    "removes-the-only-f82e64",
    "removes-the-south-42cd9e",
    "removes-the-spare-556d1d",
    "replaces-the-d40c68",
    "retimes-the-50dd75",
    "retunes-the-675f52",
    "retunes-the-cam-4ca5a2",
    "sharpens-the-25044c",
    "shifts-stream-a-5b442c",
    "stores-a-new-d56283",
    "stores-an-9f39e1",
    "swaps-in-a-denser-4174e1",
    "swaps-in-a-re-3cfa6d",
    "swaps-in-a-three-49b17f",
    "swaps-in-a-two-c688db",
    "swaps-in-an-6d9ae4",
    "swaps-in-an-f23e71",
    "swaps-in-two-166265",
    "swaps-the-c43d9c",
    "sweeps-the-503b27",
    "switches-the-423de9",
    "switches-the-652d03",
    "switches-the-7f0371",
    "switches-to-a-kd-d6fa4b",
    "tightens-the-499c47",
    "tightens-the-850036",
    "triples-the-4bb69f",
    "unplaces-the-a39356",
    "unplaces-the-f5b35e",
    "warns-that-the-1e8abe",
    "warns-that-the-414aae",
    "warns-that-the-4e65c8",
    "warns-that-the-56a3a9",
    "warns-that-the-675b6e",
    "warns-that-the-697b4f",
    "warns-that-the-79a92a",
    "warns-that-the-83ff67",
    "warns-that-the-887e9f",
    "warns-that-the-89422a",
    "warns-that-the-8dbf82",
    "warns-that-the-8eaad8",
    "warns-that-the-a98c13",
    "warns-that-the-b39bab",
    "warns-that-the-b6b7dc",
    "warns-that-the-efc6e8",
    "warns-that-the-leaves-exist",
    "warns-that-this-dca661",
    "widens-the-73f33e",
];

/// ↩️ Every `@id-inverse` row — one per committed vector. A refused or warned vector's inverse is
/// empty, which still restores: its forward step moved nothing.
#[cfg(feature = "sut")]
const INVERSE_SCENARIOS: &[&str] = &[
    "adds-a-control-298de4",
    "adds-a-fourth-97e912",
    "adds-a-quay-7569de",
    "adds-a-rig-2df5df",
    "adds-a-third-61fb5d",
    "adds-an-unbound-2b2373",
    "adds-gcp-tower-d71a54",
    "adds-stream-c-458900",
    "adds-the-cam-c-82c8fb",
    "adds-the-dtm-and-64d5bb",
    "adds-the-first-05b1b5",
    "appends-a-third-8ac259",
    "appends-an-0c2164",
    "appends-sparse-leaves",
    "attaches-a-607df8",
    "clears-every-760061",
    "clears-the-d2f81a",
    "clears-the-geo-f4886e",
    "clears-the-qc-1d2249",
    "clears-the-video-143f2b",
    "commit-reconstruction",
    "doubles-the-c245d5",
    "drops-the-6436a8",
    "drops-the-cam-a-a1f8a2",
    "drops-the-content",
    "drops-the-first-9ebf0b",
    "drops-the-first-d98a0f",
    "drops-the-middle-282fb7",
    "drops-the-middle-2d6d53",
    "drops-the-spare-c6ffb6",
    "enables-georefere-18a68a",
    "enables-motion-2444a3",
    "files-a-qc-report-64d222",
    "halves-the-002a17",
    "halves-the-voxel-21b53d",
    "moves-the-3621f6",
    "overwrites-an-a34b9d",
    "picks-the-south-eb0c4d",
    "places-the-0d0b8d",
    "raises-the-dense-ddb263",
    "records-a-dtm-6e132a",
    "records-a-qc-f5caf4",
    "refines-the-9fd25a",
    "refines-the-cam-0eaef0",
    "refuses-a-19c1ab",
    "refuses-a-48f3a6",
    "refuses-a-camera-e92a02",
    "refuses-a-d82e38",
    "refuses-a-frame-81beea",
    "refuses-a-frame-e7c374",
    "refuses-a-gap",
    "refuses-a-ratio-65dcb9",
    "refuses-a-rig-cb71ba",
    "refuses-a-second-95e04d",
    "refuses-a-zero-fa917f",
    "refuses-an-109cf1",
    "refuses-an-59752a",
    "refuses-an-asset-cb0d4b",
    "refuses-missing-content",
    "refuses-to-12366b",
    "refuses-to-1805df",
    "refuses-to-2cfb53",
    "refuses-to-3c20ff",
    "refuses-to-5c6f74",
    "refuses-to-73655a",
    "refuses-to-8095d3",
    "refuses-to-b60a39",
    "refuses-to-c4563a",
    "refuses-to-c93e98",
    "refuses-to-clear-30cbb5",
    "refuses-to-clear-524569",
    "refuses-to-clear-b8c54a",
    "refuses-to-f7f40d",
    "refuses-to-f9541f",
    "refuses-to-pick-3c0570",
    "refuses-to-remove-3c8f32",
    "refuses-to-remove-422a37",
    "reingests-the-311c32",
    "rejects-a-6b58da",
    "rejects-a-stream-aac5c2",
    "rejects-an-2e5568",
    "rejects-an-5d3a60",
    "rejects-an-e9fa51",
    "removes-an-8f3868",
    "removes-gcp-209b7d",
    "removes-the-40cba4",
    "removes-the-cam-f90b89",
    "removes-the-first-c0fc2a",
    "removes-the-last-304bdf",
    "removes-the-only-f82e64",
    "removes-the-south-42cd9e",
    "removes-the-spare-556d1d",
    "replaces-the-d40c68",
    "retimes-the-50dd75",
    "retunes-the-675f52",
    "retunes-the-cam-4ca5a2",
    "sharpens-the-25044c",
    "shifts-stream-a-5b442c",
    "stores-a-new-d56283",
    "stores-an-9f39e1",
    "swaps-in-a-denser-4174e1",
    "swaps-in-a-re-3cfa6d",
    "swaps-in-a-three-49b17f",
    "swaps-in-a-two-c688db",
    "swaps-in-an-6d9ae4",
    "swaps-in-an-f23e71",
    "swaps-in-two-166265",
    "swaps-the-c43d9c",
    "sweeps-the-503b27",
    "switches-the-423de9",
    "switches-the-652d03",
    "switches-the-7f0371",
    "switches-to-a-kd-d6fa4b",
    "tightens-the-499c47",
    "tightens-the-850036",
    "triples-the-4bb69f",
    "unplaces-the-a39356",
    "unplaces-the-f5b35e",
    "warns-that-the-1e8abe",
    "warns-that-the-414aae",
    "warns-that-the-4e65c8",
    "warns-that-the-56a3a9",
    "warns-that-the-675b6e",
    "warns-that-the-697b4f",
    "warns-that-the-79a92a",
    "warns-that-the-83ff67",
    "warns-that-the-887e9f",
    "warns-that-the-89422a",
    "warns-that-the-8dbf82",
    "warns-that-the-8eaad8",
    "warns-that-the-a98c13",
    "warns-that-the-b39bab",
    "warns-that-the-b6b7dc",
    "warns-that-the-efc6e8",
    "warns-that-the-leaves-exist",
    "warns-that-this-dca661",
    "widens-the-73f33e",
];
//#endregion 🔖️Scenarios

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_artifact_remodel_remodeling::standards::v1::subsets::any::schema::mutations::{apply_remodeling_mutation_json, round_trip_remodeling_dsl, undo_remodeling_mutation_json};

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
    /// 🏭️ Bridge version every scenario reports beside its operation. It is the `bridgeVersion` the
    /// oracle registry's own `productionDispatch` rows declare for all 36 kinds; a subject result
    /// without this record is treated as a vector replay and refused by `vectorReplayBreaches`.
    const PRODUCTION_BRIDGE_VERSION: u32 = 1;

    /// 🔁️ The operation `identity-round-trip` reports. It is not a mutation kind — the bridge it
    /// reaches is the codec's `round_trip_remodeling_dsl`, so it names that instead of borrowing a
    /// mutation's id.
    const ROUND_TRIP_BRIDGE_OPERATION: &str = "round-trip-remodeling-dsl";

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
        Ok(Outcome::with_raw(document.to_string().into_bytes(), document).dispatched(&vector.kind, PRODUCTION_BRIDGE_VERSION))
    }

    /// ↩️ The inverse law in role: applying the kind and then EVERY step of its own computed inverse
    /// must restore the committed before-document — member positions included, which is what a
    /// delete undone by re-appending would fail.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let vector = vector(ctx)?;
        let (document, _codes) = answer(&undo_remodeling_mutation_json(&vector.before_text, &vector.mutation_text)?)?;
        law::inverse_restores(&vector.kind, &document, &vector.before)?;
        Ok(Outcome::with_raw(document.to_string().into_bytes(), document).dispatched(&vector.kind, PRODUCTION_BRIDGE_VERSION))
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
        Ok(Outcome::with_raw(printed.clone().into_bytes(), Json::String(printed)).dispatched(ROUND_TRIP_BRIDGE_OPERATION, PRODUCTION_BRIDGE_VERSION))
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
    for scenario in MUTATE_SCENARIOS {
        built = built.subject(&format!("mutate-{scenario}"), subject::mutate);
    }
    #[cfg(feature = "sut")]
    for scenario in INVERSE_SCENARIOS {
        built = built.subject(&format!("inverse-{scenario}"), subject::inverse);
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
