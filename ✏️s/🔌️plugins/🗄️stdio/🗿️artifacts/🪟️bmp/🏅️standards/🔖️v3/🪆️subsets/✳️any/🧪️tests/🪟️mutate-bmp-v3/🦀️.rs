//! 🦀️ Exhaustive owned BMP mutation, inverse, and logical identity adapter.

use semio_repo_test_host::{Adapter, Context, Outcome, law};
use semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::{oracle_apply_mutation, oracle_identity_round_trip, oracle_undo_mutation, project_bmp_mutation};

/// 🧫️ Copies the scenario's committed native fixture before exercising the model.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let input = ctx.step_input_uris().into_iter().next().ok_or_else(|| format!("scenario {} names no input", ctx.scenario.id))?;
    let copy = ctx.copy_input(&input, Some("input.bmp"))?;
    std::fs::read(copy).map_err(|error| error.to_string())
}

fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_bmp_mutation(&input)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_bmp_mutation(&bytes)?;
    law::mutation_is_observable(&spec.str("kind"), &projection, &before, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let before = project_bmp_mutation(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let bytes = oracle_undo_mutation(&input, &spec, &mutated)?;
    let projection = project_bmp_mutation(&bytes)?;
    law::inverse_restores(&spec.str("kind"), &projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔁️ The native carrier reconstructs all owned fields while row padding remains physical.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_identity_round_trip(&input)?;
    let before = project_bmp_mutation(&input)?;
    let projection = project_bmp_mutation(&bytes)?;
    law::round_trip_preserves(&projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}

#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome, law};
    use semio_s_artifact_stdio_bmp_test_oracle::standards::v_v3::subsets::any::project_bmp_mutation;
    use semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::io::{decode_bmp, encode_bmp};
    use semio_s_artifact_stdio_bmp::{mutation_from_payload_json, mutation_inverse, mutation_payload_json, BmpSnapshot, ArtifactDsl};
    use semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::schema::mutations::{apply_bmp_mutation, BmpMutation};

    fn mutation_from_spec(spec: &Json) -> Result<BmpMutation, String> {
        law::wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_bmp(&mutable_input(ctx)?)?;
        let _ = apply_bmp_mutation(&mut snapshot, &mutation_from_spec(&ctx.doc_json()?)?);
        let bytes = encode_bmp(&snapshot)?;
        let projection = project_bmp_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn undo(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let base = decode_bmp(&input)?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        let mut snapshot = base.clone();
        let _ = apply_bmp_mutation(&mut snapshot, &mutation);
        for inverse in mutation_inverse(&mutation, &base).map_err(|error| format!("{error:?}"))? {
            let _ = apply_bmp_mutation(&mut snapshot, &inverse);
        }
        if snapshot != base { return Err("algebraic inverse did not restore the complete owned snapshot".into()); }
        let bytes = encode_bmp(&snapshot)?;
        let projection = project_bmp_mutation(&bytes)?;
        law::inverse_restores(&spec.str("kind"), &projection, &project_bmp_mutation(&input)?)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🧬️ The logical carrier contains an owned image block and native component statements.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_bmp(&input)?;
        let text = <BmpSnapshot as ArtifactDsl>::print_dsl(&snapshot);
        let reparsed = <BmpSnapshot as ArtifactDsl>::parse_dsl(&text).map_err(|error| format!("{error:?}"))?;
        if reparsed != snapshot { return Err("logical carrier changed the complete owned image".into()); }
        let output = encode_bmp(&reparsed)?;
        let projection = project_bmp_mutation(&output)?;
        law::round_trip_preserves(&projection, &project_bmp_mutation(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }
}

/// 🧭️ Registers the shared scenario IDs for the independent and subject implementations.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    { built = built.subject("mutate", subject::mutate).subject("inverse", subject::undo).subject("identity-round-trip", subject::identity_round_trip); }
    built
}
