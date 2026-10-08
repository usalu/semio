//! 🦀️ PNG 1.2/any exhaustive mutation case — Rust adapter, structured like
//! `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🧪️tests/✏️edit-existing-pdf/🦀️.rs`: oracle
//! handlers at top level, subject handlers inside `#[cfg(feature = "sut")] mod subject`, both
//! projected through the same INDEPENDENT `png` reader (`project_png_mutation`) before comparison.
//!
//! Every `mutate-<kind>`/`inverse-<kind>` pair is registered from the ONE `KINDS` list this file,
//! the catalog manifest and the vocabulary's own `KINDS` constant all separately spell out —
//! `bun ./📜️script.ts contract` is what keeps all three honest against each other (the framework
//! never parses Rust to check it itself).
//!
//! The oracle side never touches this repository's own codec: `oracle_apply_mutation`/
//! `oracle_undo_mutation` (this subset's own `🦀️.rs`) perform every kind
//! independently against the registered `png` reference crate. The subject side fully parses the
//! real document into the typed `PngSnapshot` and re-serializes from it — never splices bytes.

use semio_s_artifact_stdio_png_test_oracle::standards::v1_2::subsets::any::oracle_identity_round_trip;
use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_artifact_stdio_png_test_oracle::standards::v1_2::subsets::any::{oracle_apply_mutation, oracle_undo_mutation, project_png_mutation};
use semio_repo_test_host::law;
use semio_repo_test_host::Json;
use semio_s_artifact_stdio_png_test_oracle::standards::v1_2::subsets::any::{oracle_encode_owned, project_png_owned, owned_png_revision};

fn owned_vectors() -> Result<Json, String> { semio_repo_test_host::parse_json(include_str!("../../🧫️fixtures/🧬️owned-native-samples/🔣️.json")) }
fn painted_vector(row: &Json) -> Result<Json, String> {
    let mut snapshot = row.get("snapshot").ok_or("neutral PNG snapshot missing")?.clone();
    let Json::Object(fields) = &mut snapshot else { return Err("neutral PNG snapshot object missing".into()) };
    let Json::Object(image) = &mut fields.iter_mut().find(|(key, _)| key == "image").ok_or("neutral PNG image missing")?.1 else { return Err("neutral PNG image object missing".into()) };
    image.iter_mut().find(|(key, _)| key == "samples").ok_or("neutral PNG samples missing")?.1 = row.get("expectedSamples").ok_or("neutral PNG expected samples missing")?.clone();
    Ok(snapshot)
}
fn vector_paint_spec(row: &Json, revision: String, result: Json) -> Result<Json, String> {
    let paint = row.get("paint").ok_or("neutral PNG paint missing")?;
    let region = Json::Object(["x", "y", "width", "height"].into_iter().map(|key| (key.into(), paint.get(key).cloned().unwrap_or(Json::Null))).collect());
    let paint = Json::Object(["profile", "first", "second", "third", "fourth"].into_iter().map(|key| (key.into(), paint.get(key).cloned().unwrap_or(Json::Null))).collect());
    Ok(Json::Object(vec![("kind".into(), Json::String("paint-native-samples".into())), ("params".into(), Json::Object(vec![("revision".into(), Json::String(revision)), ("region".into(), region), ("paint".into(), paint), ("result".into(), result)]))]))
}
/// 🧪️ Independently reads, paints and restores all neutral precise sample profiles.
fn owned_native_samples_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let mut observations = Vec::new();
    for row in owned_vectors()?.array("cases") {
        let original = row.get("snapshot").ok_or("neutral PNG snapshot missing")?;
        let bytes = oracle_encode_owned(original)?;
        law::round_trip_preserves(&project_png_owned(&bytes)?, original)?;
        let expected = painted_vector(&row)?;
        let spec = vector_paint_spec(&row, owned_png_revision(original)?, expected.clone())?;
        let painted = oracle_apply_mutation(&bytes, &spec)?;
        let observation = project_png_owned(&painted)?;
        law::round_trip_preserves(&observation, &expected)?;
        let restored = oracle_undo_mutation(&bytes, &spec, &painted)?;
        law::inverse_restores("neutral native paint", &project_png_owned(&restored)?, original)?;
        observations.push(Json::Object(vec![("name".into(), Json::String(row.str("name"))), ("painted".into(), observation)]));
    }
    eprintln!("[DEBUG] independent PNG owned vectors preserve 16-bit precision and interlaced duplicate palette indices");
    Ok(Outcome::projection(Json::Array(observations)))
}


//#region 🔖️Input
/// 🧫️ Copies the immutable document the scenario's own `Given` names into the work directory and returns the
/// mutable copy's bytes — the committed 250 KB, 2334x2560, 8-bit COLORMAP architectural floor plan
/// (`rathaus-ahlen-grundriss.png`), the small COLORMAP document, or the small RGBA swatch. None is ever written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let input = ctx.step_input_uris().into_iter().next().ok_or_else(|| format!("scenario {} names no input document", ctx.scenario.id))?;
    let copy = ctx.copy_input(&input, Some("input.png"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ `@id-mutate`: applies the row's kind with the reference `png` codec and ASSERTS the result is
/// distinguishable from its own pre-state. Without that assertion a kind whose effect lands outside
/// the projection passes exactly as an unchanged round trip does, which is the defect this case carried while
/// its projection reported geometry and a sample digest alone.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = mutable_input(ctx)?;
    let before = project_png_mutation(&base)?;
    let bytes = oracle_apply_mutation(&base, &spec)?;
    let projection = project_png_mutation(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted rather than assumed: the reference `png` codec applies the row's
/// kind, then its own computed inverse ON TOP OF that real forward result, and the outcome must
/// project back onto the pristine original. Returning `undo_mutation(original)` without ever
/// applying the forward mutation (what this used to do) asserted nothing — the scenario passed
/// whenever the reference crate re-encoded the untouched fixture without erroring.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = mutable_input(ctx)?;
    let before = project_png_mutation(&base)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let bytes = oracle_undo_mutation(&base, &spec, &mutated)?;
    let projection = project_png_mutation(&bytes)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔁️ The no-byte-pass-through law on the ORACLE side: the reference `png` codec decodes the real
/// document and re-encodes its precise samples and metadata. Native compression may change;
/// exact owned sample values, precision, profile and ancillary metadata must survive.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_identity_round_trip(&input)?;
    law::reparsed_not_copied(&bytes, &input)?;
    let before = project_png_mutation(&input)?;
    let projection = project_png_mutation(&bytes)?;
    law::round_trip_preserves(&projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, owned_vectors, painted_vector, vector_paint_spec};
    use semio_repo_test_host::law;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_png_test_oracle::standards::v1_2::subsets::any::{project_png_mutation, project_png_owned, owned_png_revision};
    use semio_s_artifact_stdio_png::ArtifactDsl;
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_png::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::{decode_png, encode_png};
    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::mutations::{apply_png_mutation,PngMutation};

    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot;

    //#region 🔖️MutationFromSpec
    /// 🦠️ Decodes the scenario's `{"kind", "params"}` doc string: `params` is the leaf's own wire payload, read
    /// through the vocabulary's derive-generated decoder rather than a params grammar written beside it.
    fn mutation_from_spec(spec: &Json) -> Result<PngMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    /// 🧬️ Exercises typed admission, native I/O, actual paint and algebraic inversion.
    pub fn owned_native_samples(_ctx: &Context) -> Result<Outcome, String> {
        let mut observations = Vec::new();
        for row in owned_vectors()?.array("cases") {
            let original = row.get("snapshot").ok_or("neutral PNG snapshot missing")?;
            let payload = Json::Object(vec![("image".into(),original.get("image").ok_or("native image missing")?.clone())]);
            let PngMutation::ReplaceImage(set) = wire_operation("replace-image", &payload, mutation_from_payload_json, mutation_payload_json)? else { return Err("neutral PNG snapshot admission changed mutation kind".into()) };
            let base = PngSnapshot{schema:semio_s_artifact_stdio_png::STDIO_PNG_DOCUMENT_SCHEMA.into(),image:set.image};
            let revision = semio_s_artifact_stdio_png::schema::operations::png_revision(&base);
            if revision != owned_png_revision(original)? { return Err("native sample structural revision differs from independent oracle".into()) }
            let text = <PngSnapshot as ArtifactDsl>::print_dsl(&base);
            let mut snapshot = <PngSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| format!("neutral owned PNG DSL: {error:?}"))?;
            law::round_trip_preserves(&project_png_owned(&encode_png(&snapshot).map_err(|error| error.to_string())?)?, original)?;
            let expected = painted_vector(&row)?;
            let mutation = mutation_from_spec(&vector_paint_spec(&row, revision, expected.clone())?)?;
            let outcome = apply_png_mutation(&mut snapshot, &mutation);
            if !outcome.is_applicable(Default::default()) { return Err(format!("neutral PNG paint refused: {:?}", outcome.messages())); }
            let observation = project_png_owned(&encode_png(&snapshot).map_err(|error| error.to_string())?)?;
            law::round_trip_preserves(&observation, &expected)?;
            for inverse in mutation_inverse(&mutation, &base).map_err(|error| error.to_string())? {
                let outcome = apply_png_mutation(&mut snapshot, &inverse);
                if !outcome.is_applicable(Default::default()) { return Err(format!("neutral PNG inverse refused: {:?}", outcome.messages())); }
            }
            law::inverse_restores("neutral native paint", &project_png_owned(&encode_png(&snapshot).map_err(|error| error.to_string())?)?, original)?;
            observations.push(Json::Object(vec![("name".into(), Json::String(row.str("name"))), ("painted".into(), observation)]));
        }
        eprintln!("[DEBUG] actual PNG typed mutations preserve precise native values through physical I/O");
        Ok(Outcome::projection(Json::Array(observations)))
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode_png(&mutable_input(ctx)?).map_err(|error| format!("decode_png failed: {error}"))?;
        let outcome = apply_png_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        if !outcome.is_applicable(Default::default()) { return Err(format!("PNG mutation refused: {:?}", outcome.messages())); }
        let bytes = encode_png(&snapshot).map_err(|error| format!("encode_png failed: {error}"))?;
        let projection = project_png_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ Applies the forward mutation, then applies EVERY mutation `PngMutation::inverse`
    /// returns (the vocabulary's own algebraic law, index-aware, computed against the pre-forward
    /// `base`) — the real production undo pipeline, not a hand-derived counter-mutation.
    pub fn undo(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = decode_png(&mutable_input(ctx)?).map_err(|error| format!("decode_png failed: {error}"))?;
        let mutation = mutation_from_spec(&spec)?;
        let mut snapshot = base.clone();
        let outcome = apply_png_mutation(&mut snapshot, &mutation);
        if !outcome.is_applicable(Default::default()) { return Err(format!("PNG forward mutation refused: {:?}", outcome.messages())); }
        for inverse in mutation_inverse(&mutation, &base).expect("valid retained mutation inverse fixture") {
            let outcome = apply_png_mutation(&mut snapshot, &inverse);
            if !outcome.is_applicable(Default::default()) { return Err(format!("PNG inverse mutation refused: {:?}", outcome.messages())); }
        }
        let bytes = encode_png(&snapshot).map_err(|error| format!("encode_png failed: {error}"))?;
        let projection = project_png_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔁️ `decode_png` → `print_dsl` (the subset's own text codec) → `parse_dsl` → `encode_png` is the ONLY channel
    /// from input to output. Exact native samples, profile and metadata must survive.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_png(&input).map_err(|error| format!("decode_png failed: {error}"))?;
        let text = <PngSnapshot as ArtifactDsl>::print_dsl(&snapshot);
        let reparsed = <PngSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| format!("parse_dsl failed: {error:?}"))?;
        let output = encode_png(&reparsed).map_err(|error| format!("encode_png failed: {error}"))?;
        let projection = project_png_mutation(&output)?;
        law::round_trip_preserves(&projection, &project_png_mutation(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle);
    built = built.oracle("inverse", inverse_oracle);
    built = built.oracle("owned-native-samples", owned_native_samples_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate);
        built = built.subject("inverse", subject::undo);
        built = built.subject("owned-native-samples", subject::owned_native_samples);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
