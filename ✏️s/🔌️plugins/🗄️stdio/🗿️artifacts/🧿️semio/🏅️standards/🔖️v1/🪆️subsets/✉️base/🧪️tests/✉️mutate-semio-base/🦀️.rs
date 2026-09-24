//! 🦀️ Semio ENVELOPE exhaustive mutation case — Rust adapter.
//!
//! Both roles read the same committed JSON carrier vectors. The `oracle` role reads them through
//! json-rust (`semio_s_plugin_stdio_test_oracle::artifacts::semio::standards::v1::subsets::base`,
//! registered as `json-rust-semio-envelope-carrier-reader` in `../../🔮️oracles/🔣️.json`) and routes
//! them by the envelope's published law; it never links the subject crate. The `subject` role, gated
//! behind the generated host's `sut` feature, decodes the same files through this subset's
//! schema-derived JSON bridge, drives `apply_semio_mutation`/`inverse_semio_mutation`, and encodes the
//! result back through the same bridge. `ordered-json-v1` then compares
//! `{schema, subset, diagnostics, matchesReference, envelopeDigest}`, where `envelopeDigest` digests
//! the complete resulting envelope with its keys ordered.
//!
//! @see ../🥒️.feature
//! @see ../../../🧬️schema/📸️snapshot/🔣️.json
//! @see ../../../🧬️schema/🧬️mutations/🔣️.json

use semio_repo_test_host::{digest, Adapter, Json, Outcome};

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

fn replaces() -> Vector {
    catalog_vector("📸️set-snapshot", "✉️replaces-the-envelope-wrapping-a-value-subset")
}

fn reasserts() -> Vector {
    catalog_vector("📸️set-snapshot", "🪞️reasserts-the-value-envelope-unchanged")
}

fn retypes() -> Vector {
    catalog_vector("📸️set-snapshot", "🔁️retypes-a-value-envelope-to-an-empty-image")
}

fn refuses() -> Vector {
    catalog_vector("🖼️apply-image", "🚫️refuses-a-value-envelope")
}

/// 🧫️ The vector a `mutate-<kind>`/`inverse-<kind>` scenario is measured on: the committed
/// `set-snapshot` vector, or the wrapped arm's committed before/mutation/result triple.
fn kind_vector(kind: &str) -> Vector {
    if kind == "set-snapshot" {
        return replaces();
    }
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
    use super::{kind_vector, outcome_of, reasserts, refuses, replaces, retypes, Vector};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::artifacts::semio::standards::v1::subsets::base::{read_carrier, restore, route, Routed};

    fn carrier(ctx: &Context, uri: &str) -> Result<Json, String> {
        read_carrier(&ctx.fixture_bytes(uri)?)
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

    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| forward(ctx, kind_vector(kind))
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| backward(ctx, kind_vector(kind))
    }

    pub fn reasserting(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, reasserts())
    }

    pub fn undoes_reasserting(ctx: &Context) -> Result<Outcome, String> {
        backward(ctx, reasserts())
    }

    pub fn mismatch(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, refuses())
    }

    pub fn retype(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, retypes())
    }

    /// 🔮️ Rebuilding from an empty envelope is a `setSnapshot` of the committed one; the carrier law
    /// answers with the committed envelope itself, whatever the empty one carried.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = carrier(ctx, &replaces().before)?;
        let rebuild = Json::Object(vec![("mutation".to_string(), Json::String("setSnapshot".to_string())), ("payload".to_string(), Json::Object(vec![("snapshot".to_string(), committed.clone())]))]);
        let routed = route(&Json::Null, &rebuild, None)?;
        Ok(outcome_of(&routed.envelope, &routed.refused, routed.envelope == committed))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{kind_vector, outcome_of, reasserts, refuses, replaces, retypes, Vector, DSL_ASSET, PACK_ASSET};
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::mutations::{apply_semio_mutation, decode_semio_mutation_json, inverse_semio_mutation, semio_mutation_refusal_codes, set_snapshot, SemioMutation};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::snapshot::{decode_semio_envelope_pack, decode_semio_snapshot_json, encode_semio_envelope_pack, encode_semio_snapshot_json, parse_semio_envelope_dsl, print_semio_envelope_dsl, SemioSnapshot};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;

    fn text(ctx: &Context, uri: &str) -> Result<String, String> {
        String::from_utf8(ctx.fixture_bytes(uri)?).map_err(|error| format!("{uri} is not UTF-8: {error}"))
    }

    fn envelope(ctx: &Context, uri: &str) -> Result<SemioSnapshot, String> {
        decode_semio_snapshot_json(&text(ctx, uri)?).map_err(|error| format!("{uri}: {error}"))
    }

    fn mutation(ctx: &Context, uri: &str) -> Result<SemioMutation, String> {
        decode_semio_mutation_json(&text(ctx, uri)?).map_err(|error| format!("{uri}: {error}"))
    }

    fn apply(base: &SemioSnapshot, mutation: &SemioMutation) -> (SemioSnapshot, Vec<String>) {
        let mut current = base.clone();
        let outcome = apply_semio_mutation(&mut current, mutation);
        (current, semio_mutation_refusal_codes(&outcome))
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
        for step in &inverse_semio_mutation(&forward, &base) {
            let (next, more) = apply(&current, step);
            current = next;
            raised.extend(more);
        }
        reported(&current, &raised, &base)
    }

    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| forward(ctx, kind_vector(kind))
    }

    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| backward(ctx, kind_vector(kind))
    }

    pub fn reasserting(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, reasserts())
    }

    pub fn undoes_reasserting(ctx: &Context) -> Result<Outcome, String> {
        backward(ctx, reasserts())
    }

    pub fn mismatch(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, refuses())
    }

    pub fn retype(ctx: &Context) -> Result<Outcome, String> {
        forward(ctx, retypes())
    }

    /// 🔁️ Rebuilds the committed envelope from an empty one, then holds both committed encodings of the
    /// envelope's own example artifact to `carrier_is_exact` and to each other.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = envelope(ctx, &replaces().before)?;
        let (rebuilt, raised) = apply(&SemioSnapshot::default(), &SemioMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: committed.clone() }));
        let dsl = text(ctx, DSL_ASSET)?;
        let parsed = parse_semio_envelope_dsl(&dsl)?;
        carrier_is_exact(print_semio_envelope_dsl(&parsed).as_bytes(), dsl.as_bytes())?;
        let pack = ctx.fixture_bytes(PACK_ASSET)?;
        if decode_semio_envelope_pack(&pack)? != parsed {
            return Err("identity-round-trip: the committed binary twin decodes to a different envelope than the committed text artifact".to_string());
        }
        carrier_is_exact(&encode_semio_envelope_pack(&parsed), &pack)?;
        reported(&rebuilt, &raised, &committed)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id.
pub fn adapter() -> Adapter {
    use semio_s_plugin_stdio_test_oracle::law::scenario_id;
    let mut built = Adapter::new("rust");
    for kind in std::iter::once("set-snapshot").chain(ARM_VECTORS.iter().map(|(arm, _)| *arm)) {
        built = built.oracle(&scenario_id(kind, "mutate"), oracle::mutate(kind)).oracle(&scenario_id(kind, "inverse"), oracle::inverse(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&scenario_id(kind, "mutate"), subject::mutate(kind)).subject(&scenario_id(kind, "inverse"), subject::inverse(kind));
        }
    }
    built = built
        .oracle("reasserts-the-envelope-unchanged", oracle::reasserting)
        .oracle("undoes-reasserting-the-envelope", oracle::undoes_reasserting)
        .oracle("rejects-a-mismatched-arm", oracle::mismatch)
        .oracle("set-snapshot-changes-the-subset-kind", oracle::retype)
        .oracle("identity-round-trip", oracle::round_trip);
    #[cfg(feature = "sut")]
    {
        built = built
            .subject("reasserts-the-envelope-unchanged", subject::reasserting)
            .subject("undoes-reasserting-the-envelope", subject::undoes_reasserting)
            .subject("rejects-a-mismatched-arm", subject::mismatch)
            .subject("set-snapshot-changes-the-subset-kind", subject::retype)
            .subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
