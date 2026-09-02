//! 🦀️ ZIP 2.0/✳️iso21320 exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR.
//!
//! Every scenario copies the immutable real-world fixture into the case work directory first; the
//! committed archive is never written to. `oracle` drives the registered `zip` reference
//! implementation through this subset's own oracle module
//! (`../../🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🦀️oracle.rs`), which is the only
//! producer in the comparison that can honour a per-member compression method; `subject` drives this
//! repository's own decode/mutate/encode round trip through the real `ZipIso21320Mutation`
//! vocabulary. Both results are read back by the INDEPENDENT `zip` reader before the
//! `semantic-zip-iso21320-v1` profile compares them. The subject half is gated behind the generated
//! host's `sut` feature so the oracle-only run never links `semio-s-plugin-stdio`.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::zip::standards::v2_0::subsets::iso21320::{oracle_apply_inverse, oracle_apply_mutation, oracle_round_trip, project_zip_iso21320};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, mutation_is_observable, round_trip_preserves, unordered};

//#region 🔖️Input
/// 🦠️ Every declared `ZipIso21320Mutation` variant, kebab-case — mirrors
/// `../../🏅️standards/🔖️2.0/🪆️subsets/✳️iso21320/🧬️schema/🧬️mutations/🦀️.rs`'s `KINDS` and
/// that subset's `🔣️oracle.json` catalog. Declared locally rather than imported so the
/// oracle-only role's registration loop never has to link `semio-s-plugin-stdio`.
const KINDS: &[&str] = &["set-snapshot", "set-archive-comment", "add-stored-entry", "add-deflated-entry", "remove-entry", "rename-entry", "set-entry-data"];

const INPUT: &str = "shared://🎒️zwischenbericht-projekte.zip";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.zip"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with observability asserted in role: the reference applies the kind to the
/// real container and the result has to differ from the untouched one under the very profile the
/// case is measured by — member ORDER excepted, since `semantic-zip-iso21320-v1` declares
/// `arrays: "set"` and a reordering is not a change the comparison would ever see.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_zip_iso21320(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &unordered(&projection, &["entries"]), &unordered(&project_zip_iso21320(&input)?, &["entries"]), &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The inverse law, checked on the ORACLE side rather than deferred to the parity phase: the
/// reference implementation applies the kind and then its own computed inverse, and the restored
/// container's independent projection must equal the REAL original's own. Member ORDER is writer
/// freedom under `semantic-zip-iso21320-v1` (`arrays: "set"`), so both sides go through
/// [`unordered`] first — the profile's own tolerance, never stricter. Without this the scenario
/// would only prove that the inverse ran without erroring, which is not what `@mode-property`
/// claims — and with the subject phase blocked, it is the only place the property can be checked
/// today.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_inverse(&input, &mutated, &spec)?;
    let projection = project_zip_iso21320(&restored)?;
    let original = project_zip_iso21320(&input)?;
    inverse_restores(&spec.str("kind"), &unordered(&projection, &["entries"]), &unordered(&original, &["entries"]))?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The identity law, asserted in role — but NOT through the must-differ tripwire this module's
/// doc comment used to claim, because for THIS fixture that tripwire is a fabricated law and was
/// measured to be false. `read_archive` genuinely inflates every member and `write_archive`
/// genuinely re-deflates it under the method the profile declared, yet the output is bit-identical
/// to the 1,605,927-byte input — because the fixture itself was authored ONCE by this same `zip`
/// reference writer with these same default `FileOptions` (the archive's `1980-01-01` timestamps
/// and version-20/Unix headers are that writer's defaults). Bit-stability is what this pairing can
/// honestly claim, so it is what is asserted: the exact-bytes law plus preservation of the semantic
/// projection, both of which fail loudly if reader, writer, method policy or entry order drift.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    carrier_is_exact(&bytes, &input)?;
    let projection = project_zip_iso21320(&bytes)?;
    round_trip_preserves(&unordered(&projection, &["entries"]), &unordered(&project_zip_iso21320(&input)?, &["entries"]))?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::io::{decode_zip, encode_zip};
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::schema::snapshot::ZipEntry;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::add_deflated_entry::AddDeflatedEntry;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::add_stored_entry::AddStoredEntry;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::remove_entry::RemoveEntry;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::rename_entry::RenameEntry;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::set_archive_comment::SetArchiveComment;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::set_entry_data::SetEntryData;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::set_snapshot::SetSnapshot;
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::iso21320::schema::mutations::{apply_zip_iso21320_mutation, inverse_zip_iso21320_mutation, ZipIso21320Mutation};
    use semio_s_plugin_stdio::artifacts::zip::{ZipSnapshot, STDIO_ZIP_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::artifacts::zip::standards::v2_0::subsets::iso21320::project_zip_iso21320;

    //#region 🔖️Spec
    /// 🦠️ Builds the real typed `ZipIso21320Mutation` this scenario's `{"kind", "params"}` spec
    /// describes — the same 7 kinds the mutations file's own `KINDS` declares. An undeclared kind is
    /// an error, never a silent no-op.
    fn mutation_from_spec(value: &Json) -> Result<ZipIso21320Mutation, String> {
        let params = value.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        Ok(match value.str("kind").as_str() {
            "set-snapshot" => ZipIso21320Mutation::SetSnapshot(SetSnapshot {
                snapshot: ZipSnapshot {
                    schema: STDIO_ZIP_DOCUMENT_SCHEMA.to_string(),
                    entries: params.array("entries").iter().map(|entry| ZipEntry { name: entry.str("name"), data: entry.str("content").into_bytes() }).collect(),
                    comment: params.str("comment"),
                },
            }),
            "set-archive-comment" => ZipIso21320Mutation::SetArchiveComment(SetArchiveComment { comment: params.str("comment") }),
            "add-stored-entry" => ZipIso21320Mutation::AddStoredEntry(AddStoredEntry { entry: ZipEntry { name: params.str("name"), data: params.str("content").into_bytes() } }),
            "add-deflated-entry" => ZipIso21320Mutation::AddDeflatedEntry(AddDeflatedEntry { entry: ZipEntry { name: params.str("name"), data: params.str("content").into_bytes() } }),
            "remove-entry" => ZipIso21320Mutation::RemoveEntry(RemoveEntry { name: params.str("name") }),
            "rename-entry" => ZipIso21320Mutation::RenameEntry(RenameEntry { name: params.str("name"), new_name: params.str("newName") }),
            "set-entry-data" => ZipIso21320Mutation::SetEntryData(SetEntryData { name: params.str("name"), data: params.str("content").into_bytes() }),
            other => return Err(format!("mutation kind {other:?} has no subject implementation")),
        })
    }
    //#endregion 🔖️Spec

    //#region 🔖️Handlers
    fn base_snapshot(ctx: &Context) -> Result<ZipSnapshot, String> {
        decode_zip(&mutable_input(ctx)?).map_err(|error| format!("decode_zip failed: {error}"))
    }

    fn outcome_of(snapshot: &ZipSnapshot) -> Result<Outcome, String> {
        let bytes = encode_zip(snapshot).map_err(|error| format!("encode_zip failed: {error}"))?;
        let projection = project_zip_iso21320(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = base_snapshot(ctx)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_zip_iso21320_mutation(&mut snapshot, &mutation);
        outcome_of(&snapshot)
    }

    /// ↩️ The subset's OWN inverse algebra, reached through its typed vocabulary rather than
    /// re-derived here, so the property under test is the implementation's algebra and not a
    /// transcription of it.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = base_snapshot(ctx)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        let undo = inverse_zip_iso21320_mutation(&mutation, &base);
        let mut snapshot = base;
        apply_zip_iso21320_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_zip_iso21320_mutation(&mut snapshot, step);
        }
        outcome_of(&snapshot)
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real container into its
    /// typed snapshot and re-serialize from the model alone.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_zip(&input).map_err(|error| format!("decode_zip failed: {error}"))?;
        let output = encode_zip(&snapshot).map_err(|error| format!("encode_zip failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_zip_iso21320(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 7 kinds — the scenario id only selects which Examples row's
/// `<id>`/`<params>` doc string the shared handler reads.
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
