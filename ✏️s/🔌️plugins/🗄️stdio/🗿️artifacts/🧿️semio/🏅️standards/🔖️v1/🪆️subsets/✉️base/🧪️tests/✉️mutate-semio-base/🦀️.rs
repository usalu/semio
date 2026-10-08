//! 🦀️ Semio ENVELOPE exhaustive mutation case — Rust adapter.
//!
//! Both roles read the same committed JSON carrier vectors. The `oracle` role reads them through
//! json-rust (`semio_s_artifact_stdio_semio_test_oracle::standards::v1::subsets::base`,
//! registered as `json-rust-semio-envelope-carrier-reader` in `../../🔮️oracles/🔣️.json`) and routes
//! them by the envelope's published law; it never links the subject crate. The `subject` role, gated
//! behind the generated host's `sut` feature, decodes the same files through this subset's
//! schema-derived JSON bridge, drives `diff_semio_mutation`/`inverse_semio_mutation` and the central `apply_diff`, and encodes the
//! result back through the same bridge. `ordered-json-v1` then compares
//! `{schema, subset, diagnostics, matchesReference, envelopeDigest}`, where `envelopeDigest` digests
//! the complete resulting envelope with its keys ordered.
//!
//! @see ../🥒️.feature
//! @see ../../../🧬️schema/📸️snapshot/🔣️.json
//! @see ../../../🧬️schema/🧬️mutations/🔣️.json

use semio_repo_test_host::{digest, Adapter, Json, Outcome};

use semio_s_artifact_stdio_semio::apply_diff;

//#region 🔖️Vectors
/// 🧫️ The committed wrapped-arm vector each delegated kind is measured on — before-envelope, wrapped
/// mutation and the arm's own committed result, under this subset's `🧫️fixtures/`.
const ARM_VECTORS: &[(&str, &str)] = &[
    ("apply-brep", "🧊️apply-brep-applied"),
    ("apply-mesh", "🔺️apply-mesh-applied"),
    ("apply-model", "🏛️apply-model-applied"),
    ("apply-value", "🔢️apply-value-applied"),
    ("apply-document", "📑️apply-document-applied"),
    ("apply-cad", "📐️apply-cad-applied"),
    ("apply-drawing", "🖊️apply-drawing-applied"),
    ("apply-image", "🖼️apply-image-applied"),
    ("apply-video", "🎬️apply-video-applied"),
    ("apply-audio", "🔊️apply-audio-applied"),
    ("apply-animation", "🎞️apply-animation-applied"),
    ("apply-presentation", "📽️apply-presentation-applied"),
    ("apply-flow", "🌊️apply-flow-applied"),
    ("apply-text", "🔤️apply-text-applied"),
    ("apply-table", "🗂️apply-table-applied"),
    ("apply-graph", "🕸️apply-graph-applied"),
    ("apply-object", "📦️apply-object-applied"),
    ("apply-kit", "🧰️apply-kit-applied"),
];

/// 🧫️ One committed vector: where its before-envelope, mutation and expected result live.
struct Vector {
    before: String,
    mutation: String,
    after: String,
}

/// 🧫️ A catalog-registered `🧬️mutations/` vector, addressed by its feature-declared `shared://` URIs.
fn catalog_vector(leaf: &str, scenario: &str) -> Vector {
    let root = format!("shared://🧬️mutations/{leaf}/{scenario}");
    Vector { before: format!("{root}/📸️snapshot/⬅️before/🔣️.json"), mutation: format!("{root}/🦠️mutation/🔣️.json"), after: format!("{root}/📸️snapshot/➡️after/🔣️.json") }
}

fn refuses() -> Vector {
    catalog_vector("🖼️apply-image", "🚫️refuses")
}

/// 🧫️ The vector a `mutate-<kind>`/`inverse-<kind>` scenario is measured on: the wrapped arm's committed
/// before/mutation/result triple.
fn kind_vector(kind: &str) -> Vector {
    let root = ARM_VECTORS.iter().find(|(arm, _)| *arm == kind).map(|(_, vector)| *vector).unwrap_or_else(|| panic!("mutate-semio-base: no committed vector for arm {kind:?}"));
    Vector { before: format!("shared://{root}/⬅️before.json"), mutation: format!("shared://{root}/🦠️mutation.json"), after: format!("shared://{root}/➡️after.json") }
}

/// 🌐️ The envelope's own committed real artifact, in both committed encodings.
const DSL_ASSET: &str = "asset://🌐️envelope/🗣️.dsl.semio";
const PACK_ASSET: &str = "asset://🌐️envelope/🎒️.pack.semio";
//#endregion 🔖️Vectors

//#region 🔖️Projection
/// 🔣️ Orders every object's keys so equal envelopes print, and therefore digest, identically.
fn ordered(json: &Json) -> Json {
    match json {
        Json::Object(entries) => {
            let mut sorted: Vec<(String, Json)> = entries.iter().map(|(key, value)| (key.clone(), ordered(value))).collect();
            sorted.sort_by(|left, right| left.0.cmp(&right.0));
            Json::Object(sorted)
        }
        Json::Array(items) => Json::Array(items.iter().map(ordered).collect()),
        other => other.clone(),
    }
}

/// 🏷️ The envelope's `subset.subset` discriminator as the projection reports it.
fn arm_of(envelope: &Json) -> String {
    envelope.get("subset").map(|subset| subset.str("subset")).unwrap_or_default()
}

/// 🎯️ The one projection both roles report for a routed envelope.
fn outcome_of(envelope: &Json, diagnostics: &[String], matches_reference: bool) -> Outcome {
    let projection = Json::Object(vec![
        ("schema".to_string(), Json::String(envelope.str("schema"))),
        ("subset".to_string(), Json::String(arm_of(envelope))),
        ("diagnostics".to_string(), Json::Array(diagnostics.iter().map(|code| Json::String(code.clone())).collect())),
        ("matchesReference".to_string(), Json::Bool(matches_reference)),
        ("envelopeDigest".to_string(), Json::String(digest(ordered(envelope).to_string().as_bytes()))),
    ]);
    let bytes = projection.to_string().into_bytes();
    Outcome::with_raw(bytes, projection)
}
//#endregion 🔖️Projection

//#region 🔖️Oracle
mod oracle {
    use super::{kind_vector, outcome_of, refuses, Vector};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_semio_test_oracle::standards::v1::subsets::base::{read_carrier, restore, route, Routed};

    fn carrier(ctx: &Context, uri: &str) -> Result<Json, String> {
        read_carrier(&ctx.input_bytes(uri)?)
    }

    fn reported(routed: &Routed, before: &Json) -> Outcome {
        outcome_of(&routed.envelope, &routed.refused, routed.envelope == *before)
    }

    /// 🔮️ A committed vector routed forward; the committed after-envelope must be what the law yields.
    fn forward(ctx: &Context, vector: Vector) -> Result<Outcome, String> {
        let before = carrier(ctx, &vector.before)?;
        let after = carrier(ctx, &vector.after)?;
        let routed = route(&before, &carrier(ctx, &vector.mutation)?, Some(&after))?;
        if routed.envelope != after {
            return Err(format!("the committed vector {} disagrees with the routing law it records", vector.after));
        }
        Ok(reported(&routed, &before))
    }

    /// 🔮️ A committed vector's inverse law: the envelope it started from.
    fn backward(ctx: &Context, vector: Vector) -> Result<Outcome, String> {
        let before = carrier(ctx, &vector.before)?;
        route(&before, &carrier(ctx, &vector.mutation)?, Some(&carrier(ctx, &vector.after)?))?;
        Ok(reported(&restore(&before), &before))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, kind_vector(ctx.row()?))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        backward(ctx, kind_vector(ctx.row()?))
    }

    pub fn mismatch(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, refuses())
    }

    /// 🔮️ The committed envelope is its own answer: the carrier law has nothing to route, so it reports the
    /// committed envelope unchanged.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = carrier(ctx, &kind_vector("apply-value").before)?;
        Ok(outcome_of(&committed, &[], true))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{kind_vector, outcome_of, refuses, Vector, DSL_ASSET, PACK_ASSET};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::mutations::{diff_semio_mutation, inverse_semio_mutation, semio_mutation_refusal_codes, SemioMutation};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::io::text::mutations::{decode_semio_mutation_json};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::io::binary::snapshot::{encode_semio_envelope_pack};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::io::binary::snapshot::{decode_semio_envelope_pack};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::io::text::snapshot::{decode_semio_snapshot_json};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::io::text::snapshot::{encode_semio_snapshot_json};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::io::text::snapshot::{print_semio_envelope_dsl};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::io::text::snapshot::{parse_semio_envelope_dsl};
    use semio_repo_test_host::law::carrier_is_exact;

    fn text(ctx: &Context, uri: &str) -> Result<String, String> {
        String::from_utf8(ctx.input_bytes(uri)?).map_err(|error| format!("{uri} is not UTF-8: {error}"))
    }

    fn envelope(ctx: &Context, uri: &str) -> Result<SemioSnapshot, String> {
        decode_semio_snapshot_json(&text(ctx, uri)?).map_err(|error| format!("{uri}: {error}"))
    }

    fn mutation(ctx: &Context, uri: &str) -> Result<SemioMutation, String> {
        decode_semio_mutation_json(&text(ctx, uri)?).map_err(|error| format!("{uri}: {error}"))
    }

    fn apply(base: &SemioSnapshot, mutation: &SemioMutation) -> (SemioSnapshot, Vec<String>) {
        let outcome = diff_semio_mutation(mutation, base);
        let refusals = semio_mutation_refusal_codes(&outcome);
        if refusals.is_empty() {
            return (apply_diff(outcome.diff(), base).expect("a refusal-free envelope diff must apply to the envelope it was computed from"), refusals);
        }
        (base.clone(), refusals)
    }

    fn reported(routed: &SemioSnapshot, raised: &[String], base: &SemioSnapshot) -> Result<Outcome, String> {
        Ok(outcome_of(&parse_json(&encode_semio_snapshot_json(routed))?, raised, routed == base))
    }

    fn forward(ctx: &Context, vector: Vector) -> Result<Outcome, String> {
        let base = envelope(ctx, &vector.before)?;
        let (routed, raised) = apply(&base, &mutation(ctx, &vector.mutation)?);
        reported(&routed, &raised, &base)
    }

    fn backward(ctx: &Context, vector: Vector) -> Result<Outcome, String> {
        let base = envelope(ctx, &vector.before)?;
        let forward = mutation(ctx, &vector.mutation)?;
        let (mut current, mut raised) = apply(&base, &forward);
        for step in &inverse_semio_mutation(&forward, &base).expect("valid retained mutation inverse fixture") {
            let (next, more) = apply(&current, step);
            current = next;
            raised.extend(more);
        }
        reported(&current, &raised, &base)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, kind_vector(ctx.row()?))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        backward(ctx, kind_vector(ctx.row()?))
    }

    pub fn mismatch(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, refuses())
    }

    /// 🔁️ Holds both committed encodings of the envelope's own example artifact to `carrier_is_exact` and to each
    /// other, and reports the committed envelope the text decodes to.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = envelope(ctx, &kind_vector("apply-value").before)?;
        let dsl = text(ctx, DSL_ASSET)?;
        let parsed = parse_semio_envelope_dsl(&dsl)?;
        carrier_is_exact(print_semio_envelope_dsl(&parsed).as_bytes(), dsl.as_bytes())?;
        let pack = ctx.input_bytes(PACK_ASSET)?;
        if decode_semio_envelope_pack(&pack)? != parsed {
            return Err("identity-round-trip: the committed binary twin decodes to a different envelope than the committed text artifact".to_string());
        }
        carrier_is_exact(&encode_semio_envelope_pack(&parsed), &pack)?;
        reported(&committed, &[], &committed)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built
        .oracle("mutate", oracle::mutate)
        .oracle("inverse", oracle::inverse)
        .oracle("rejects-a-mismatched-arm", oracle::mismatch)
        .oracle("identity-round-trip", oracle::round_trip);
    #[cfg(feature = "sut")]
    {
        built = built
            .subject("mutate", subject::mutate)
            .subject("inverse", subject::inverse)
            .subject("rejects-a-mismatched-arm", subject::mismatch)
            .subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
