//! 🦀️ ZIP 2.0/✳️any mutation case — Rust adapter.
//!
//! Every scenario copies the immutable real-world fixture into the case work directory first; the
//! committed archive is never written to. `oracle` drives the registered `zip` reference
//! implementation (`../../🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`), `subject`
//! drives this repository's own decode/mutate/encode round trip through the real `ZipMutation`
//! vocabulary, and both results are read back by the INDEPENDENT `zip` reader before the
//! `semantic-archive-mutate-v1` profile compares them. The subject half is gated behind the
//! generated host's `sut` feature so the oracle-only run never links `semio-s-plugin-stdio`, whose
//! subject phase is peer-blocked right now (concurrent os-kernel refactor).

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::zip::standards::v2_0::subsets::any::{oracle_apply_inverse, oracle_apply_mutation, oracle_round_trip, project_zip_mutation};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, round_trip_preserves, unordered};

//#region 🔖️Input
/// 🦠️ Every declared `ZipMutation` variant, kebab-case — mirrors `../../🏅️standards/🔖️2.0/🪆️subsets/
/// ✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s `KINDS` and that same standard's
/// `🧪️oracle/🔣️component.json` catalog. Declared locally rather than imported so the oracle-only
/// role's registration loop never has to link `semio-s-plugin-stdio`.
const KINDS: [&str; 7] = ["no-mutation", "set-snapshot", "set-archive-comment", "add-entry", "remove-entry", "rename-entry", "set-entry-data"];

const INPUT: &str = "shared://🎒️zwischenbericht-projekte.zip";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.zip"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🧫️ The scenario's `{"kind": ..., "params": {...}}` spec, read from its doc string.
fn spec(ctx: &Context) -> Result<Json, String> {
    ctx.doc_json()
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &spec(ctx)?)?;
    let projection = project_zip_mutation(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, asserted HERE rather than deferred to the parity phase: the reference
/// applies the kind and then its own computed inverse, and the restored archive's independent
/// projection must equal the REAL original's own. `semantic-archive-mutate-v1` declares
/// `arrays: "set"`, so member ORDER is writer freedom and the two projections are compared through
/// [`unordered`] — exactly the profile's own tolerance, never stricter. Without this the scenario
/// would only prove the inverse ran without erroring, which is not what `@mode-property` claims.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let scenario_spec = spec(ctx)?;
    let mutated = oracle_apply_mutation(&input, &scenario_spec)?;
    let restored = oracle_apply_inverse(&input, &mutated, &scenario_spec)?;
    let projection = project_zip_mutation(&restored)?;
    let original = project_zip_mutation(&input)?;
    inverse_restores(&scenario_spec.str("kind"), &unordered(&projection, &["entries"]), &unordered(&original, &["entries"]))?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, asserted in role — but NOT through the must-differ tripwire every parsed
/// format in this wave uses, because for THIS fixture that tripwire is a fabricated law and was
/// measured to be false. `read_archive` genuinely inflates every member (`read_to_end` on a
/// `ZipFile`) and `write_archive` genuinely re-deflates it, yet the output is bit-identical to the
/// 1,605,927-byte input — because the fixture itself was authored ONCE by this same `zip` reference
/// writer with these same default `FileOptions` (see the feature file's provenance paragraph, and
/// the archive's own `1980-01-01` timestamps and version-20/Unix headers, which are that writer's
/// defaults). Bit-stability is what this pairing can honestly claim, so it is what is asserted:
/// the exact-bytes law plus preservation of the semantic projection. Both still fail loudly if the
/// reader, the writer, the compression defaults or the entry order ever drift.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    carrier_is_exact(&bytes, &input)?;
    let projection = project_zip_mutation(&bytes)?;
    round_trip_preserves(&unordered(&projection, &["entries"]), &unordered(&project_zip_mutation(&input)?, &["entries"]))?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, spec};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::io::{decode_zip, encode_zip};
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::schema::mutations::apply_zip_mutation;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipEntry;
    use semio_s_plugin_stdio::artifacts::zip::{ZipMutation, ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::artifacts::zip::standards::v2_0::subsets::any::project_zip_mutation;

    //#region 🔖️Spec
    /// 🦠️ Builds the real typed `ZipMutation` this scenario's `{"kind", "params"}` spec describes —
    /// the same 7 kinds the mutations file's own `KINDS` declares, kept honest against them by that
    /// file's `kinds_matches_enum_variants_and_manifest` test.
    fn zip_mutation_from_json(value: &Json) -> Result<ZipMutation, String> {
        let params = value.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        Ok(match value.str("kind").as_str() {
            "no-mutation" => ZipMutation::NoMutation,
            "set-snapshot" => ZipMutation::SetSnapshot {
                snapshot: ZipSnapshot {
                    schema: STDIO_ZIP_DOCUMENT_SCHEMA.to_string(),
                    entries: params.array("entries").iter().map(|entry| ZipEntry { name: entry.str("name"), data: entry.str("content").into_bytes() }).collect(),
                    comment: params.str("comment"),
                },
            },
            "set-archive-comment" => ZipMutation::SetArchiveComment { comment: params.str("comment") },
            "add-entry" => ZipMutation::AddEntry { entry: ZipEntry { name: params.str("name"), data: params.str("content").into_bytes() } },
            "remove-entry" => ZipMutation::RemoveEntry { name: params.str("name") },
            "rename-entry" => ZipMutation::RenameEntry { name: params.str("name"), new_name: params.str("newName") },
            "set-entry-data" => ZipMutation::SetEntryData { name: params.str("name"), data: params.str("content").into_bytes() },
            kind => return Err(format!("mutation kind {kind:?} has no subject implementation")),
        })
    }

    /// ↩️ This mutation's own inverse against `original` — the snapshot as it stood before the
    /// forward mutation ran. Mirrors `../../🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/
    /// 🦀️component.rs`'s `ZipMutation::inverse` algebra directly (not through the `protocol::
    /// Mutation` trait) so this adapter carries no dependency on that internal crate.
    fn invert_zip_mutation(original: &ZipSnapshot, mutation: &ZipMutation) -> ZipMutation {
        match mutation {
            ZipMutation::NoMutation => ZipMutation::NoMutation,
            ZipMutation::SetSnapshot { .. } => ZipMutation::SetSnapshot { snapshot: original.clone() },
            ZipMutation::SetArchiveComment { .. } => ZipMutation::SetArchiveComment { comment: original.comment.clone() },
            ZipMutation::AddEntry { entry } => ZipMutation::RemoveEntry { name: entry.name.clone() },
            ZipMutation::RemoveEntry { name } => original.entries.iter().find(|entry| entry.name == *name).map(|entry| ZipMutation::AddEntry { entry: entry.clone() }).unwrap_or(ZipMutation::NoMutation),
            ZipMutation::RenameEntry { name, new_name } => ZipMutation::RenameEntry { name: new_name.clone(), new_name: name.clone() },
            ZipMutation::SetEntryData { name, .. } => original.entries.iter().find(|entry| entry.name == *name).map(|entry| ZipMutation::SetEntryData { name: name.clone(), data: entry.data.clone() }).unwrap_or(ZipMutation::NoMutation),
        }
    }
    //#endregion 🔖️Spec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mutation = zip_mutation_from_json(&spec(ctx)?)?;
        let mut snapshot = decode_zip(&input).map_err(|error| format!("decode_zip failed: {error}"))?;
        apply_zip_mutation(&mut snapshot, &mutation);
        let bytes = encode_zip(&snapshot).map_err(|error| format!("encode_zip failed: {error}"))?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".into());
        }
        let projection = project_zip_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let original = decode_zip(&input).map_err(|error| format!("decode_zip failed: {error}"))?;
        let mutation = zip_mutation_from_json(&spec(ctx)?)?;
        let mut snapshot = original.clone();
        apply_zip_mutation(&mut snapshot, &mutation);
        let inverse_mutation = invert_zip_mutation(&original, &mutation);
        apply_zip_mutation(&mut snapshot, &inverse_mutation);
        let bytes = encode_zip(&snapshot).map_err(|error| format!("encode_zip failed: {error}"))?;
        let projection = project_zip_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_zip(&input).map_err(|error| format!("decode_zip failed: {error}"))?;
        let bytes = encode_zip(&snapshot).map_err(|error| format!("encode_zip failed: {error}"))?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".into());
        }
        let projection = project_zip_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
