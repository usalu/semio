//! 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `semio.migrate` — the cold job kind
//! design-abi.md §2 names as the replacement for the deleted `migrate-artifact` export: a versioned
//! re-encode of one pack from an old `ArtifactDialect` coordinate to a new one. `input` bundles the
//! three former export parameters into one JSON `{from, to, pack}` triple, the SAME "tuple-of-what-
//! used-to-be-separate-params" idiom `job_io_run`/`job_io_sniff`'s own `IoRunInput` already
//! established (a job only ever carries one opaque `list<u8>`). Dispatch goes through
//! `store::migrate_document`, the process-global `DialectMigration` registry every plugin's own
//! `PluginBuilder::migrations(...)` declarations already populate at build time (`🏗️builder/
//! 🦀️.rs`'s `migrations` field, folded into `store::register_dialect_migrations` at
//! `try_build()` — outside this packet's owned paths, already wired by an earlier packet) — the
//! SAME kind of process-global registry `job_io_run`/`job_io_sniff`/`💡️infer`/`🧬️mutation-plan`
//! all read from, not a new mechanism.
//!
//! Sliced across `super::TwoPhaseBoundedJob`'s two explicit states exactly like its siblings:
//! `Decode` validates `input` (parsing both dialect coordinates, reporting `"{from}->{to}"` as
//! progress) and makes `PHASE_DECODED` its checkpoint; `Execute` runs the real `migrate_document`
//! re-encode.

use super::{BoundedJob, TwoPhaseBoundedJob};
use semio_framework_value_derive::FromValue;

/// 🌉️ Mirrors `job_io_run`'s own `IoRunInput`: what used to be `migrate-artifact`'s three separate
/// export parameters, bundled into one JSON tuple a `Vec<u8>`-only job can carry.
#[derive(serde::Deserialize, FromValue)]
struct MigrateInput {
    from: String,
    to: String,
    pack: Vec<u8>,
}

// 🚫️async: E4 fn-pointer slot — registered into `BoundedJobFactory` (see
// `⚛️reactor/💼️jobs/🦀️.rs`'s `builtin_registry`); the admission body is a pure constructor call.
pub(super) fn job_migrate(_job: u64, input: &[u8], restored: Option<&[u8]>) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    Ok(Box::new(TwoPhaseBoundedJob::admit("job.migrate", input, restored, decode_phase, execute_phase)))
}

// 🚫️async: E4 phase slot — `BuiltinPhaseFn` is synchronous by contract; `decode` itself has no
// suspension point, so `settle_in_step` resolves it inside this state action.
fn decode_phase(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    super::settle_in_step("job.migrate", decode(input))
}

// 🚫️async: E4 phase slot — see `decode_phase`; `store::migrate_document` is the unchunked native
// call this state action declares `WORK_UNITS_EXECUTE` for.
fn execute_phase(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    super::settle_in_step("job.migrate", execute(input))
}

async fn parse_dialects(input: &MigrateInput) -> Result<(semio_framework::io_schema::ArtifactDialect, semio_framework::io_schema::ArtifactDialect), semio_framework::Fault> {
    let from = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&input.from).map_err(|message| super::fault("job.migrate", message))?;
    let to = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&input.to).map_err(|message| super::fault("job.migrate", message))?;
    Ok((from, to))
}

/// 🔎️ Validates `input` decodes as `{from, to, pack}` with two parseable dialect coordinates, and
/// reports `"{from}->{to}"` as the first slice's progress bytes.
async fn decode(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|error| super::fault("job.migrate.decode", format!("invalid {} input: {error}", super::JOB_KIND_MIGRATE)))?;
    let parsed: MigrateInput = dsl::os_pack::json::from_json_str(input_text).map_err(|error| super::fault("job.migrate.decode", format!("invalid {} input: {error}", super::JOB_KIND_MIGRATE)))?;
    let (from, to) = parse_dialects(&parsed).await?;
    Ok(format!("{}->{}", from.to_coordinate(), to.to_coordinate()).into_bytes())
}

async fn execute(input: &[u8]) -> Result<Vec<u8>, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|error| super::fault("job.migrate.decode", format!("invalid {} input: {error}", super::JOB_KIND_MIGRATE)))?;
    let parsed: MigrateInput = dsl::os_pack::json::from_json_str(input_text).map_err(|error| super::fault("job.migrate.decode", format!("invalid {} input: {error}", super::JOB_KIND_MIGRATE)))?;
    let (from, to) = parse_dialects(&parsed).await?;
    store::migrate_document(&from, &to, &parsed.pack).await.map_err(|error| super::fault("job.migrate", format!("{error:?}")))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
