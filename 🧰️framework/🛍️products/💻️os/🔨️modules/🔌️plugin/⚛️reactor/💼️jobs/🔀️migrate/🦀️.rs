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
//! Sliced across `super::run_two_phase`'s two ticks exactly like its siblings: slice 1
//! decodes+validates `input` (parsing both dialect coordinates, reporting `"{from}->{to}"` as
//! progress) and checkpoints; slice 2 runs the real `migrate_document` re-encode.

use super::{run_two_phase, JobCtx};
use semio_framework_value_derive::FromValue;
use std::future::Future;
use std::pin::Pin;

/// 🌉️ Mirrors `job_io_run`'s own `IoRunInput`: what used to be `migrate-artifact`'s three separate
/// export parameters, bundled into one JSON tuple a `Vec<u8>`-only job can carry.
#[derive(serde::Deserialize, FromValue)]
struct MigrateInput {
    from: String,
    to: String,
    pack: Vec<u8>,
}

// 🚫️async: E4 fn-pointer slot — see `job_mutation_plan`'s own comment in the sibling `🧬️mutation-plan`
// module for the full explanation; same `JobFn` registry shape.
pub(super) fn job_migrate(ctx: JobCtx, input: Vec<u8>, restored: Option<Vec<u8>>) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, semio_framework::Fault>>>> {
    Box::pin(async move {
        let decode_input = input.clone();
        let execute_input = input;
        run_two_phase(ctx, restored, move || async move { decode(&decode_input).await }, move || async move { execute(&execute_input).await }).await
    })
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
