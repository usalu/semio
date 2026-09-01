//! 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `semio.migrate` — the cold job kind
//! design-abi.md §2 names as the replacement for the deleted `migrate-artifact` export: a versioned
//! re-encode of one pack from an old `ArtifactDialect` coordinate to a new one. `input` bundles the
//! three former export parameters into one JSON `{from, to, pack}` triple, the SAME "tuple-of-what-
//! used-to-be-separate-params" idiom `job_io_run`/`job_io_sniff`'s own `IoRunInput` already
//! established (a job only ever carries one opaque `list<u8>`). Dispatch goes through
//! `store::migrate_document`, the process-global `DialectMigration` registry every plugin's own
//! `PluginBuilder::migrations(...)` declarations already populate at build time (`🏗️builder/
//! 🦀️component.rs`'s `migrations` field, folded into `store::register_dialect_migrations` at
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
mod tests {
    use super::super::*;

    fn append_job_test_marker(bytes: &[u8]) -> Result<Vec<u8>, String> {
        let mut out = bytes.to_vec();
        out.push(0xAB);
        Ok(out)
    }

    async fn register_job_test_migration() -> (String, String) {
        let from = semio_framework::io_schema::ArtifactDialect { artifact_kind: "s.jobtest.migrate".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let to = semio_framework::io_schema::ArtifactDialect { artifact_kind: "s.jobtest.migrate".to_string(), standard: "2".to_string(), subset: "*".to_string() };
        let migration = store::DialectMigration { from: from.clone(), to: to.clone(), lossless: true, migrate_pack: append_job_test_marker };
        // 🧬️ Idempotent on purpose: several tests in this module register the SAME (from, to) pair —
        // `register_dialect_migration` treats a byte-identical re-registration as `Ok(())`, matching
        // `📓️terra-jobs-runtime-report.md`'s own "last-writer overwrites, identical is not a conflict"
        // convention one layer up in `register_job_kind`.
        let _ = store::register_dialect_migration(migration).await;
        (from.to_coordinate(), to.to_coordinate())
    }

    async fn input_bytes(from: &str, to: &str, pack: Vec<u8>) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({ "from": from, "to": to, "pack": pack })).expect("test input encodes")
    }

    /// 🔀️ Registers a real `DialectMigration` (not mocked away) and drives `semio.migrate` through
    /// two real `step_job` slices to `Done`, proving `job_migrate` really reaches
    /// `store::migrate_document` and runs the registered re-encode, not just `job.unknown-kind`.
    #[semio_framework_async_macros::async_test]
    async fn a_two_slice_migrate_job_decodes_then_dispatches_to_the_registered_migration() {
        let (from, to) = register_job_test_migration().await;
        let input = input_bytes(&from, &to, vec![1, 2, 3]).await;
        start_job(400, JOB_KIND_MIGRATE, &input).await;

        match step_job(400, JobBudget { fuel: 1, deadline_ms: 1 }).await {
            JobStep::Running(Some(progress)) => {
                assert_eq!(progress, format!("{from}->{to}").into_bytes());
            }
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                panic!("slice 1 must be Running(Some(coordinates)), not fail: {} {}", fault.code.0, fault.message);
            }
            JobStep::Done(_) => panic!("slice 1 must not finish in one tick"),
            JobStep::Running(None) => panic!("slice 1 must be Running(Some(coordinates)), not a bare Running(None)"),
        }
        match step_job(400, JobBudget { fuel: 1, deadline_ms: 1 }).await {
            JobStep::Done(bytes) => assert_eq!(bytes, vec![1, 2, 3, 0xAB]),
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                panic!("slice 2 must dispatch to the registered migration, not fail: {} {}", fault.code.0, fault.message);
            }
            JobStep::Running(_) => panic!("slice 2 must finish Done, the native migration call is atomic"),
        }
    }

    /// 📸️ Interrupts after slice 1 (decode only), checkpoints, cancels, restores, and confirms the
    /// resumed run reaches the SAME `Done` output as an uninterrupted run.
    #[semio_framework_async_macros::async_test]
    async fn migrate_job_checkpoint_restore_matches_an_uninterrupted_run() {
        let (from, to) = register_job_test_migration().await;
        let input = input_bytes(&from, &to, vec![9, 9]).await;

        start_job(401, JOB_KIND_MIGRATE, &input).await;
        step_job(401, JobBudget::default()).await;
        let baseline = match step_job(401, JobBudget::default()).await {
            JobStep::Done(bytes) => bytes,
            _ => panic!("uninterrupted run must finish Done within 2 slices"),
        };

        start_job(402, JOB_KIND_MIGRATE, &input).await;
        step_job(402, JobBudget::default()).await;
        let entries = checkpoint_jobs().await;
        let entry = entries.iter().find(|entry| entry.job == 402).expect("job 402 must appear in checkpoint_jobs()");
        assert_eq!(entry.checkpoint.as_deref(), Some(PHASE_DECODED));
        let checkpoint = entry.checkpoint.clone();
        cancel_job(402).await;

        restore_job(402, JOB_KIND_MIGRATE, &input, checkpoint).await;
        let restored_final = match step_job(402, JobBudget::default()).await {
            JobStep::Done(bytes) => bytes,
            _ => panic!("a restore from PHASE_DECODED must finish Done on its FIRST step_job call"),
        };
        assert_eq!(restored_final, baseline, "checkpoint/restore must produce the identical final output");
    }

    #[semio_framework_async_macros::async_test]
    async fn migrate_job_reports_a_named_fault_when_no_migration_is_registered() {
        let from = semio_framework::io_schema::ArtifactDialect { artifact_kind: "s.jobtest.migrate-missing".to_string(), standard: "1".to_string(), subset: "*".to_string() }.to_coordinate();
        let to = semio_framework::io_schema::ArtifactDialect { artifact_kind: "s.jobtest.migrate-missing".to_string(), standard: "2".to_string(), subset: "*".to_string() }.to_coordinate();
        let input = input_bytes(&from, &to, vec![1]).await;
        start_job(403, JOB_KIND_MIGRATE, &input).await;
        step_job(403, JobBudget::default()).await;
        match step_job(403, JobBudget::default()).await {
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                assert_eq!(fault.code.0, "job.migrate");
            }
            _ => panic!("a migration nobody registered must fail on slice 2, not succeed"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn migrate_job_reports_a_named_decode_fault_on_garbage_input() {
        start_job(404, JOB_KIND_MIGRATE, b"not json").await;
        match step_job(404, JobBudget::default()).await {
            JobStep::Failed(bytes) => {
                let fault = dsl::decode_fault_bytes(&bytes);
                assert_eq!(fault.code.0, "job.migrate.decode");
            }
            _ => panic!("garbage migrate input must fail on slice 1"),
        }
    }
}
