/** 📄️ Document commands: mount single flight, durable group decision and journal, document opening attempts, reopen storms and the document http check.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runRepositoryCargoTests, runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { exactCargoStageEnvironments } from "../🧱️script-bases/🟦️.ts";

/** 🪢️ Proves one generation-fenced retained mount owner serves every concurrent document opener. */
export class DocumentMountSingleFlightCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("document-mount-single-flight-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🚪️document-mount");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    
    const mountedPoolUse = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔐️pool-use/🔣️.json"), "utf8"));
    
    
    
    for (const row of mountedPoolUse.cases) {
      const retainedUses = row.externalUses + (row.pool === "open" && (row.database === "open" || row.database === "opening-non-runnable") ? 1 : 0);
      const shutdown = retainedUses ? `busy-${retainedUses}` : "stopped";
      assert.equal(shutdown, row.expectedShutdown, row.id);
      if (row.authority === "ready") assert.equal(row.database, "open", row.id);
      if (row.database === "terminal" || row.database === "absent") assert.equal(row.databaseActivities, "closed", row.id);
    }
    const mountedEngine = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs"), "utf8");
    const mountedArtifact = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"), "utf8");
    const mountedSync = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"), "utf8");
    assert(mountedEngine.includes('#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;'), "missing actual database pool-use test mount");
    const engineLaws = readFileSync(join(owner, "..", "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    for (const marker of ["pool_use: Option<Arc<WorkerPoolUse>>", "let pool_use = pool.acquire_use()", "fn require_open_use(&self)", "self.pool_use.take()", "DatabaseRetainedActivityRejected::Closed", "DatabaseDocumentMountDriver::NonRunnable", "DatabaseShutdownBlock::Executor(kind)"]) assert(mountedEngine.includes(marker), `missing mounted pool-use marker ${marker}`);
    const catalogStateStart = mountedEngine.indexOf("struct DatabaseCreateCatalogState {");
    const catalogState = mountedEngine.slice(catalogStateStart, mountedEngine.indexOf("\n}", catalogStateStart));
    assert.equal((mountedEngine.match(/_pool_use: Arc<WorkerPoolUse>/g)?.length ?? 0) + Number(catalogState.includes("pool_use: Mutex<Option<Arc<WorkerPoolUse>>>")), 4, "every retained Database capability/catalog state must own the use cell");
    assert(mountedEngine.includes("pool_use: Mutex::new(Some(pool_use))"), "catalog publication must retain its admitted use cell");
    assert(mountedEngine.includes("self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();"), "catalog terminal drain must release its exact admitted use cell");
    for (const marker of ["_pool_use: Arc<semio_framework_async::WorkerPoolUse>", "spawn_with_pool_use", "pool.acquire_use()"] ) assert(mountedArtifact.includes(marker), `missing authority pool-use marker ${marker}`);
    for (const marker of ["_pool_use: std::sync::Arc<semio_framework_async::WorkerPoolUse>", "let pool_use = match pool.acquire_use()"] ) assert(mountedSync.includes(marker), `missing sync-hello pool-use marker ${marker}`);
    for (const law of ["database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack", "database_worker_pool_use_is_admitted_before_the_first_storage_probe", "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer"]) assert(engineLaws.includes(`fn ${law}(`), `missing mounted pool-use law ${law}`);
    console.log(`[DEBUG] database-mounted-pool-use-independent-oracle: behavior-oracles=retained cases=${mountedPoolUse.cases.length} retained-source-owners=3`);
    for (const row of fixture.cases) {
      let activeGeneration: bigint | undefined;
      let waiters = 0;
      let ready = false;
      let terminalCleanup = true;
      let emitting = false;
      let events = 0;
      let fanoutPrepared = false;
      let registryUnlocked = true;
      let ownerPollActive = false;
      for (const step of row.steps) {
        const generation = BigInt(step.generation);
        if (step.action === "install") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          ready = false;
        } else if (step.action === "join") {
          assert.equal(generation, activeGeneration, row.name);
          waiters += 1;
        } else if (step.action === "cancel-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "catalog-published" || step.action === "shutdown-interrupted") {
          assert.equal(generation, activeGeneration, row.name);
        } else if (step.action === "emit-begin") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!emitting && events === 0, row.name);
          emitting = true;
        } else if (step.action === "emit-complete") {
          assert.equal(generation, activeGeneration, row.name);
          assert(events === 0, row.name);
          emitting = false;
          events = 1;
        } else if (step.action === "fill-waiters") {
          assert.equal(generation, activeGeneration, row.name);
          waiters = fixture.capacity.waitersPerDocument;
        } else if (step.action === "reject-over-capacity") {
          assert.equal(generation, activeGeneration, row.name);
          assert.equal(waiters, fixture.capacity.waitersPerDocument, row.name);
        } else if (step.action === "release-slot") {
          assert.equal(generation, activeGeneration, row.name);
          waiters -= 1;
        } else if (step.action === "close-fault") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = false;
        } else if (step.action === "resume-cleanup") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "explicit-retirement-retry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!terminalCleanup, row.name);
        } else if (step.action === "close-terminal") {
          assert.equal(generation, activeGeneration, row.name);
          terminalCleanup = true;
          activeGeneration = undefined;
        } else if (step.action === "prepare-fanout") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = true;
          registryUnlocked = false;
        } else if (step.action === "unlock-registry") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && !registryUnlocked, row.name);
          registryUnlocked = true;
        } else if (step.action === "wake-waiter") {
          assert.equal(generation, activeGeneration, row.name);
          assert(fanoutPrepared && registryUnlocked, row.name);
          fanoutPrepared = false;
        } else if (step.action === "owner-poll-active") {
          assert.equal(generation, activeGeneration, row.name);
          assert(!ownerPollActive, row.name);
          ownerPollActive = true;
        } else if (step.action === "request-drive") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
        } else if (step.action === "owner-poll-release") {
          assert.equal(generation, activeGeneration, row.name);
          assert(ownerPollActive, row.name);
          ownerPollActive = false;
        } else if (step.action === "retry") {
          assert.equal(activeGeneration, undefined, row.name);
          activeGeneration = generation;
          waiters = 1;
          events = 0;
        } else if (step.action === "ready") {
          assert.equal(generation, activeGeneration, row.name);
          assert(terminalCleanup && !emitting && events === 1, row.name);
          ready = true;
        }
      }
      assert.equal(ready ? 1 : 0, row.expected.actors, row.name);
      assert.equal(row.expected.owners, 1, row.name);
      assert.equal(events, row.expected.events, row.name);
      assert.equal(waiters, row.expected.waitersReleased, row.name);
      assert.equal(terminalCleanup, row.expected.terminalBeforeFanout, row.name);
      assert(!fanoutPrepared && registryUnlocked && !ownerPollActive, row.name);
    }
    for (const row of fixture.driverCases) {
      let state = "idle";
      let wakeRequested = false;
      let resumeRequested = false;
      let queued = 0;
      let maximumQueued = 0;
      let cleanupParked = false;
      let cleanupResumes = 0;
      for (const step of row.steps) {
        if (step === "request" || step === "request-resume") {
          wakeRequested = true;
          resumeRequested ||= step === "request-resume";
          if (state === "idle") {
            state = "queued";
            queued = 1;
            maximumQueued = Math.max(maximumQueued, queued);
          }
        } else if (step === "poll-start") {
          assert.equal(state, "queued", row.name);
          state = "polling";
          queued = 0;
          wakeRequested = false;
        } else if (step === "cleanup-fault") {
          assert.equal(state, "polling", row.name);
          cleanupParked = true;
        } else if (step === "cleanup-resume") {
          assert(cleanupParked && resumeRequested, row.name);
          cleanupParked = false;
          resumeRequested = false;
          cleanupResumes += 1;
        } else if (step === "hard-submit-rejected") {
          assert.equal(state, "queued", row.name);
          state = "non-runnable";
        } else if (step === "poll-pending") {
          assert.equal(state, "polling", row.name);
          if (wakeRequested) wakeRequested = false;
          else state = "idle";
        } else if (step === "complete") {
          state = "terminal";
        }
      }
      assert.equal(state, row.expected.state, row.name);
      assert.equal(maximumQueued, row.expected.maximumQueuedPollers, row.name);
      assert.equal(cleanupResumes, row.expected.cleanupResumes, row.name);
    }
    const source = readFileSync(join(owner, "..", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum DatabaseDocumentMountSlot",
      "Opening {",
      "struct DatabaseDocumentMountOwner",
      "DatabaseDocumentMountWork",
      "DATABASE_DOCUMENT_MOUNT_WAITERS",
      "fn take_generation",
      "pub async fn ensure_document",
      "retained_mount_rejection",
      "DatabaseDocumentMountWait",
      "impl Drop for DatabaseDocumentMountWait",
    ])
      assert(source.includes(marker), `missing retained document-mount marker: ${marker}`);
    assert(!source.includes("open_artifacts: Mutex<HashMap<String, Arc<db_artifact::ArtifactAuthority>>>") && source.includes("open_artifacts: Arc<Mutex<DatabaseDocumentMountRegistry>>"));
    const retainedMount = source.slice(source.indexOf("async fn run_document_mount("), source.indexOf("async fn mount_document("));
    assert(retainedMount.indexOf("emit.emit(") >= 0 && retainedMount.indexOf("emit.emit(") < retainedMount.indexOf("Ok(DatabaseDocumentMountReply"));
    assert(!source.slice(source.indexOf("struct DatabaseDocumentMountReply"), source.indexOf("struct DatabaseDocumentMountWaiter")).includes("emission:"));
    const complete = source.slice(source.indexOf("fn complete(&self, result: Result<DatabaseDocumentMountReply, DbError>)"), source.indexOf("//#region 🔖️Database"));
    assert(complete.includes("let mut fanout: [Option<(") && complete.includes("DATABASE_DOCUMENT_MOUNT_WAITERS"));
    assert(complete.indexOf("drop(authority);") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(complete.indexOf("self.terminal.store(true") < complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    const registryScope = complete.slice(complete.indexOf("let mut registry = registry.lock()"), complete.indexOf("for (reply, outcome) in fanout.into_iter().flatten()"));
    assert(registryScope.trimEnd().endsWith("self.terminal.store(true, Ordering::Release);\n        }"), "waiters must be woken only after the registry guard's scope has closed");
    const mountOwner = source.slice(source.indexOf("impl DatabaseDocumentMountOwner"), source.indexOf("//#region 🔖️Database"));
    assert(
      mountOwner.includes("DatabaseDocumentMountDriver::Idle") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Queued") &&
        mountOwner.includes("DatabaseDocumentMountDriver::Polling") &&
        mountOwner.includes("DatabaseDocumentMountDriver::NonRunnable"),
    );
    assert(mountOwner.includes("resume_requested") && mountOwner.includes("wake_requested"));
    assert(mountOwner.includes("Arc::downgrade(self)") && !mountOwner.includes("let owner = self.clone();\n        self.submit_exact"));
    const databaseOpen = source.slice(source.indexOf("async fn open_with("), source.indexOf("fn document_engine_config("));
    assert.equal(databaseOpen.match(/acquire_use\(\)/g)?.length, 1);
    for (const marker of ["DatabaseCapabilityOpenFuture::try_prepare_with_use", "DatabaseCatalogReadFuture::try_prepare_with_use", "DatabaseCatalogBootstrapFuture::try_prepare_with_use"])
      assert(databaseOpen.includes(marker), `database open minted an untracked pool use instead of retaining ${marker}`);
    const catalogPublication = source.slice(source.indexOf("async fn publish_mount_catalog("), source.indexOf("async fn run_open_document_mount("));
    assert(catalogPublication.includes("DatabaseCreateCatalogFuture::try_prepare_with_use(pool, pool_use"));
    const catalogDriveStart = source.indexOf("fn drive_one(self: Arc<Self>, generation: u64)", source.indexOf("//#region 🔖️CreateDocumentCatalogCas"));
    const catalogDrive = source.slice(catalogDriveStart, source.indexOf("fn drive_claimed(self: &Arc<Self>, generation: u64)", catalogDriveStart));
    assert(!catalogDrive.includes("release_success()"));
    const catalogTerminal = source.slice(source.indexOf("pub struct DatabaseCreateCatalogTerminalHandle"), source.indexOf("pub fn take_database_create_catalog_terminal"));
    assert(catalogTerminal.includes("drive_explicit_close_one()"), "explicit catalog terminal cleanup must make one bounded turn without a wall-clock callback");
    const helloStart = source.indexOf("pub fn hello_retained(");
    const hello = source.slice(helloStart, source.indexOf("pub async fn hello(", helloStart));
    assert(hello.includes("DatabaseSyncHelloFuture::try_submit_with_use") && !hello.includes("DatabaseSyncHelloFuture::try_submit("));
    const requestDrive = mountOwner.slice(mountOwner.indexOf("fn request_drive(self: &Arc<Self>"), mountOwner.indexOf("fn resume_parked("));
    assert(!requestDrive.includes("self.work.try_lock()") && !requestDrive.includes("self.work.lock()"));
    const artifact = readFileSync(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"), "utf8");
    for (const marker of [
      "enum ArtifactRunnerDriver",
      "RunnableIdle",
      "Queued",
      "PollingWake",
      "Parked",
      "ClosingReady",
      "ClosingPollingWake",
      "ClosingParked",
      "Terminal",
      "fn park_terminal_job",
      "struct ArtifactRunnerClosePoll",
      "struct ArtifactRunnerPoll",
      "struct ArtifactRunnerRetirementReservation",
      "WorkerMaintenanceStep::Retire",
      "impl Drop for ArtifactAuthority",
      "pub fn close(mut self) -> Result<(), Self>",
      "pub fn resume(mut self) -> Result<(), Self>",
    ]) {
      assert(artifact.includes(marker), `missing artifact terminal-authority marker: ${marker}`);
    }
    for (const marker of ["close_error", "WorkerMaintenanceStep::Fault"]) {
      assert(artifact.includes(marker), `missing retained artifact close-fault marker: ${marker}`);
    }
    const artifactLaws = readFileSync(join(owner, "..", "..", "🗿️artifact", "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    for (const law of [
      "artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal",
      "artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission",
      "artifact_engine_close_fault_cancel_stops_the_timer_until_readmission",
    ]) {
      assert(artifactLaws.includes(`fn ${law}(`), `missing retained artifact close-fault law ${law}`);
    }
    const artifactSchedule = artifact.slice(artifact.indexOf("fn schedule(self: &Arc<Self>)"), artifact.indexOf("fn submit_exact(self: &Arc<Self>"));
    assert(artifactSchedule.includes("compare_exchange") && artifactSchedule.includes("ArtifactRunnerDriver::RunnableIdle as u8") && artifactSchedule.includes("ArtifactRunnerDriver::Queued as u8"));
    assert(!artifactSchedule.includes("scheduled.compare_exchange"));
    const observe = readFileSync(join(owner, "..", "..", "👁️observe", "🦀️.rs"), "utf8");
    assert(observe.includes("fn emit(&self, event: EmitEvent) -> impl Future<Output = ()> + Send;"));
    for (const law of [
      "database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
      "database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
      "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
    ]) {
      assert(engineLaws.includes(`fn ${law}(`), `missing exact mount driver law ${law}`);
    }
    console.log(`document-mount-single-flight-independent-oracle: behavior-oracles=retained cases=${fixture.cases.length} waiters=${fixture.capacity.waitersPerDocument} owner-futures=${fixture.capacity.ownerFuturesPerDocument}`);
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 180_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_engine::tests::database_concurrent_ensure_mounts_one_actor_and_one_writer",
            "db_engine::tests::database_published_opening_joins_without_actor_overwrite",
            "db_engine::tests::database_cancelled_ensure_waiter_does_not_cancel_mount_owner",
            "db_engine::tests::database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation",
            "db_engine::tests::database_mount_owner_emits_before_ready_and_survives_elected_waiter_cancellation",
            "db_engine::tests::database_mount_waiter_capacity_rejects_33_and_reuses_one_cancelled_slot",
            "db_engine::tests::database_document_mount_fanout_wakes_only_after_registry_unlock_and_internal_owner_handoff",
            "db_engine::tests::database_shutdown_interrupt_retains_waiterless_opening_owner_until_ready",
            "db_engine::tests::database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume",
            "db_engine::tests::database_document_mount_coalesces_join_drives_without_shared_pool_starvation",
            "db_engine::tests::database_document_mount_cleanup_fault_consumes_racing_resume_request_exactly_once",
            "db_engine::tests::database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack",
            "db_engine::tests::database_worker_pool_use_is_admitted_before_the_first_storage_probe",
            "db_engine::tests::database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer",
            "db_engine::tests::database_create_catalog_resolved_drop_retains_use_until_terminal_drain",
            "db_artifact::tests::artifact_runner_terminal_authority_latch_preserves_external_job_and_one_resume",
            "db_artifact::tests::artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn",
            "db_artifact::tests::artifact_runner_terminal_close_returns_exact_cursor_until_retained_wake",
            "db_artifact::tests::artifact_runner_terminal_resume_refusal_returns_exact_cursor_for_close",
            "db_artifact::tests::artifact_authority_drop_transfers_parked_terminal_job_to_registered_close_owner",
            "db_artifact::tests::artifact_runner_retirement_panic_retains_exact_cursor_until_explicit_retry",
            "db_artifact::tests::artifact_engine_close_fault_retries_on_bounded_timer_backoff_until_terminal",
            "db_artifact::tests::artifact_engine_close_fault_exhausts_its_budget_then_polls_only_on_readmission",
            "db_artifact::tests::artifact_engine_close_fault_cancel_stops_the_timer_until_readmission",
            "db_artifact::tests::more_live_authorities_than_pool_maintenance_hooks_retire_through_one_shared_hook",
          ],
        },
      ],progress(event) {
        console.log(`document-mount-single-flight-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      }});
    console.log(`document-mount-single-flight-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🗺️ Proves the fixed owned Map decision envelope with independent AJV and SHA-256 oracles. */
export class DurableOwnedGroupDecisionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-owned-group-decision-check accepts only --native");
    const { testDurableOwnedGroupDecisionFixture } = await import("../../../../🔨️modules/🏪️store/🧪️tests/🗄️durable-owned-group/🟦️.ts");
    testDurableOwnedGroupDecisionFixture();
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-framework-os-store-durable-group-tests",
            target: { kind: "lib" },
            laws: [
              "durable_byte_carriers_preserve_octets_under_the_declared_ownership_ceiling",
              "durable_owned_group_decision_matches_neutral_canonical_hash_and_bounds",
              "durable_group_journal_record_projects_edits_only_after_all_three_bound_outcomes_verify",
              "durable_store_prepared_outcome_derives_and_verifies_exact_unbound_bytes",
              "durable_owned_group_decision_rejects_forged_identity_commitment_and_capacity",
              "durable_decision_rejects_deflate_expansion_before_document_body_allocation",
              "durable_store_owned_three_member_bind_and_base_recovery_retain_exact_private_owners",
              "durable_store_private_committed_record_recovers_all_three_stores_without_reappending_journal",
              "durable_json_carriers_preserve_numeric_kinds_and_reject_control_and_resource_excess",
              "durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once",
              "durable_store_group_cancellation_waits_for_trusted_absence_then_restores_all_old_roots",
              "durable_store_group_stage_error_retains_abort_owner_until_every_root_is_empty",
              "durable_store_group_uncertain_journal_error_retries_same_owner_without_rebegin_or_visibility_change",
              "durable_store_group_rejects_foreign_anchor_receipt_before_visibility_and_aborts_only_after_absence",
              "durable_map_fixed_host_slot_retains_every_live_owner_across_request_error_until_terminal_handoff",
              "durable_map_fixed_host_slot_cancellation_after_uncertain_io_waits_for_trusted_absence",
            ],
          },
        ],progress(event) {
          console.log(`durable-owned-group-decision-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`durable-owned-group-decision-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧾️ Proves the typed Store-decision journal boundary and exact physical WAL reservation. */
export class DurableGroupJournalCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("durable-group-journal-check accepts only --native");
    const artifactOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact");
    const fixtureOwner = join(artifactOwner, "🧪️tests/🧫️fixtures/📓️durable-group-journal");
    const fixture = JSON.parse(readFileSync(join(fixtureOwner, "🔣️.json"), "utf8"));
    
    
    assert.equal(new Set(fixture.cases.map((row: any) => row.id)).size, fixture.cases.length);
    for (const row of fixture.committedDecisionWitnessCases) {
      const events = row.recordKinds.filter((kind: string) => kind === "event").length;
      const expected = row.transaction === "aborted" || events === 0 ? "ignored" : row.replayDocument === "foreign" || events !== 1 || row.recordKinds.length !== 1 ? "rejected" : "witness";
      assert.equal(row.expected, expected);
    }
    assert.deepEqual(fixture.committedRecovery.authority, {
      consumer: "db-wal-witness",
      recordEscape: false,
      receiptSource: "derived",
      cancelAfterCommit: false,
      errorSurface: "retaining-fault",
      terminalHandoff: "stores-and-ack-once",
    });
    assert.equal(new Set(fixture.committedRecovery.cases.map((row: any) => row.id)).size, fixture.committedRecovery.cases.length);
    for (const row of fixture.committedRecovery.cases) {
      const expected = row.frontier === "base" ? ["complete", 3] : row.frontier === "post" ? ["already-applied", 0] : ["fault", 0];
      assert.deepEqual([row.expected, row.mutations], expected);
      assert.equal(row.returnedStoreOwners, 3);
    }
    const storeOwner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group");
    const storeFixture = JSON.parse(readFileSync(join(storeOwner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    assert.equal(createHash("sha256").update(storeFixture.expected.unsignedJson).digest("hex"), fixture.record.decisionSha256);
    assert.equal(storeFixture.expected.anchorSha256, fixture.record.anchorSha256);
    const varintBytes = (value: number) => {
      let bytes = 1;
      while (value >= 128) {
        value = Math.floor(value / 128);
        bytes += 1;
      }
      return bytes;
    };
    const frameBytes = (payload: number) => varintBytes(payload + 2) + payload + 10;
    const fieldBytes = (value: string) => varintBytes(Buffer.byteLength(value)) + Buffer.byteLength(value);
    const successorHeaderBytes = 32 + frameBytes(fieldBytes(fixture.record.document) + 41) + 75;
    const transactionBytes = (eventBytes: number) => frameBytes(8) + frameBytes(eventBytes) + frameBytes(12) + 75;
    assert.equal(successorHeaderBytes, fixture.limits.successorHeaderBytes);
    assert.equal(transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumTransactionBytes);
    assert.equal(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes), fixture.limits.storeMaximumSegmentBytes);
    let maximumEventBytes = fixture.limits.walSegmentBytes;
    while (successorHeaderBytes + transactionBytes(maximumEventBytes) > fixture.limits.walSegmentBytes) maximumEventBytes -= 1;
    assert.equal(maximumEventBytes, fixture.limits.maximumEventBytes);
    assert(successorHeaderBytes + transactionBytes(fixture.limits.storeEventBytes) <= fixture.limits.walSegmentBytes);
    assert(successorHeaderBytes + transactionBytes(maximumEventBytes + 1) > fixture.limits.walSegmentBytes);
    const storeSource = readFileSync(join(storeOwner, "🦀️.rs"), "utf8");
    const artifactSource = readFileSync(join(artifactOwner, "🦀️.rs"), "utf8");
    const walSource = readFileSync(join(artifactOwner, "../📝️wal/🦀️.rs"), "utf8");
    const engineSource = readFileSync(join(artifactOwner, "../⚙️engine/🦀️.rs"), "utf8");
    assert(storeSource.includes("pub struct DurableOwnedGroupJournalRecordV1") && storeSource.includes("DurableOwnedThreeMemberDecisionV1::decode_canonical_pack(&canonical_pack)"));
    assert(storeSource.includes("pub enum DurableOwnedGroupJournalAdvanceV1") && storeSource.includes("Rejected(String)"));
    const append = artifactSource.slice(artifactSource.indexOf("async fn append_durable_group_decision("), artifactSource.indexOf("pub(crate) async fn compact_retained", artifactSource.indexOf("async fn append_durable_group_decision(")));
    assert(append.includes("WalRecord::Event") && append.includes("DurabilityClass::Fsync"));
    assert(append.indexOf("preflight_submit") < append.indexOf("self.wal.submit"));
    assert(append.includes("ArtifactDurableGroupJournalAppendV1::Absent") && append.includes("ArtifactDurableGroupJournalAppendV1::Rejected"));
    const witness = artifactSource.slice(artifactSource.indexOf("struct ArtifactCommittedDurableGroupDecisionV1"), artifactSource.indexOf("//#endregion 🔖️Receipt"));
    const lowerRecovery = storeSource.slice(
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> DurableOwnedMapRecoveryHostV1"),
      storeSource.indexOf("impl<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation> Drop for DurableOwnedMapRecoveryHostV1"),
    );
    assert(witness.includes("WalCommittedTransaction") && witness.includes("record_count != 1") && witness.includes("transaction.finish()?"));
    for (const marker of [
      "ArtifactCommittedDurableGroupRecoveryV1",
      "ArtifactCommittedDurableGroupRecoveryStateV1::Admitting",
      "ArtifactCommittedDurableGroupRecoveryAdvanceV1::Fault",
      "into_store_owned_recovery",
      "restore_untrusted_committed_record",
      "acknowledge_restoration(&receipt)",
      "take_terminal",
      "take_rejected_terminal",
    ])
      assert(witness.includes(marker), `missing committed Store recovery marker ${marker}`);
    assert(!witness.includes("pub fn begin_store_owned_recovery"));
    assert(!witness.includes("fn into_record(") && !witness.includes("fn record(&self)"), "a committed WAL witness has no raw record escape");
    assert(!witness.includes("fn cancel("), "committed Store recovery remains non-cancellable after WAL visibility");
    assert(
      lowerRecovery.includes("pub fn advance(&mut self, grant: super::ArtifactStoreOneItemGrant) -> DurableOwnedMapRecoveryAdvanceV1") && lowerRecovery.includes("DurableOwnedMapRecoveryAdvanceV1::Fault(error)"),
      "lower Store recovery faults must retain their exact host instead of exposing Result/? owner loss",
    );
    assert(lowerRecovery.includes("pub fn capture_snapshot(&self) -> Option<"), "lower Store recovery observation must not expose a Result/? owner-loss path");
    const sink = artifactSource.slice(artifactSource.indexOf("struct ArtifactDurableGroupJournalSinkV1"), artifactSource.indexOf("type ArtifactBuildFuture"));
    assert(sink.includes("NotSubmitted") && sink.includes("Awaiting") && sink.includes("Failed") && sink.includes("Committed") && sink.includes("terminal_is_empty"));
    assert(!sink.includes("block_on"));
    assert(walSource.includes("pub(crate) fn preflight_submit(&self, commands: &[Vec<u8>], records: &WalRecordBatch)") && walSource.includes("wal transaction exceeds readable segment"));
    assert(engineSource.includes("pub fn durable_group_journal_sink(&self, now_ms: u64)"));
    const laws = [
      "db_artifact::tests::document_authority_durable_group_journal_commits_one_exact_fsync_event",
      "db_artifact::tests::committed_durable_group_decision_accepts_only_one_exact_event_transaction",
      "db_artifact::tests::committed_durable_group_recovery_consumes_wal_witness_and_returns_exact_three_stores_on_pre_mutation_rejection",
      "db_artifact::tests::document_authority_durable_group_journal_cancellation_before_handoff_is_absent",
      "db_artifact::tests::document_authority_durable_group_journal_rejects_hash_before_mailbox",
    ];
    const artifactLawsSource = readFileSync(join(artifactOwner, "🧪️tests/🔬️unit/🦀️.rs"), "utf8");
    for (const law of laws) assert(artifactLawsSource.includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    console.log(
      `durable-group-journal-independent-oracle: behavior-oracles=retained cases=${fixture.cases.length} witnesses=${fixture.committedDecisionWitnessCases.length} recovery=${fixture.committedRecovery.cases.length} max-event=${maximumEventBytes} store-margin=${fixture.limits.walSegmentBytes - fixture.limits.storeMaximumSegmentBytes}`,
    );
    if (segments[0] !== "--native") return;
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 180_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, cargoArgs: ["--all-features"], laws }],progress(event) {
        console.log(`durable-group-journal-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      }});
    console.log(`durable-group-journal-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

/** 🪪️ Executes the native outer opening-attempt wire law without broadening the browser patch contract. */
export class DocumentOpeningAttemptNativeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("document-opening-attempt-native-check accepts no arguments");
    const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
        {
          package: "semio-framework-os-store-sync-tests-1",
          target: { kind: "lib" },
          laws: ["sync_tests::document_opening_attempt_wire_preserves_outer_owner_without_widening_actor_messages"],
        },
      ],progress(event) {
        console.log(`document-opening-attempt-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      }});
    console.log(`document-opening-attempt-native-receipts: ${JSON.stringify(receipts)}`);
  }
}

//#region 🌪️ReopenStorm
/** 🌪️ The reopen-storm laws of the db engine: a hub restart's reconnect storm, measured where the storage lives. `unit`
 * greets two dozen grown documents at once after a reopen (bound 30 s, the growth e2e's reopen bound); `fs`, `sqlite`,
 * `postgres` and `neo4j` are the throughput laws that grow, reopen alone and reopen as a storm and bound the storm against
 * the solo reopen (`🛢️db/⚙️engine/🧫️fixtures/⏱️throughput`). `postgres`/`neo4j` run against the ONE shared development
 * server claimed by `os-hub-ts backend run <postgres|neo4j> -- …` (its `OS_HUB_*` environment selects it), so they are
 * selected only by name and `all` never includes them. */
const REOPEN_STORM_LAWS: Readonly<Record<string, Readonly<{ law: string; features: string; claimed?: string }>>> = {
  unit: { law: "db_engine::tests::long::two_dozen_grown_documents_greeted_at_once_after_a_reopen_are_welcomed_within_the_reopen_bound", features: "sqlite" },
  fs: { law: "db_engine::throughput_tests::fs_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite" },
  sqlite: { law: "db_engine::throughput_tests::sqlite_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite" },
  postgres: { law: "db_engine::throughput_tests::postgres_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite,postgres", claimed: "OS_HUB_DATABASE_URL" },
  neo4j: { law: "db_engine::throughput_tests::neo4j_commits_and_reopen_storms_stay_within_their_throughput_bounds", features: "sqlite,neo4j", claimed: "OS_HUB_NEO4J_URI" },
};

/** 🌪️ `reopen-storm-check [unit|fs|sqlite|all]…` — runs each selected law alone in its own cargo test process (in place,
 * `SEMIO_DB_ISOLATED_LAW`), streams its output, reads the storm welcome distribution it prints and publishes the
 * acceptance record. Ctrl-C stops the running law. */
export class ReopenStormCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const wanted = segments.length === 0 || segments.includes("all") ? Object.keys(REOPEN_STORM_LAWS).filter((name) => !REOPEN_STORM_LAWS[name]!.claimed) : segments;
    const unknown = wanted.filter((name) => !(name in REOPEN_STORM_LAWS));
    if (unknown.length) throw new Error(`reopen-storm-check accepts ${Object.keys(REOPEN_STORM_LAWS).join(" | ")} | all, got ${unknown.join(",")}`);
    const throughput = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🧫️fixtures/⏱️throughput/🔣️.json"), "utf8"));
    
    
    console.log(`[reopen-storm] throughput example: storm ${throughput.storm.documents}×${throughput.storm.batches}×${throughput.storm.batchEdits}, storm/serial ≤ ${throughput.bounds.stormToSerialRatioMax}`);
    const unclaimed = wanted.filter((name) => REOPEN_STORM_LAWS[name]!.claimed && !process.env[REOPEN_STORM_LAWS[name]!.claimed!]);
    if (unclaimed.length) throw new Error(`reopen-storm-check ${unclaimed.join(",")} needs the claimed shared server: run it under \`os-hub-ts backend run ${unclaimed[0]} -- …\``);
    const { acceptanceCheckResult, publishAcceptanceCheckResult, runLawProcess } = await import("../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts");
    const startedAt = new Date();
    const measured: Record<string, number | string | boolean> = {};
    const failed: string[] = [];
    let cancelled = false;
    for (const [index, name] of wanted.entries()) {
      if (cancelled) break;
      const { law, features, claimed } = REOPEN_STORM_LAWS[name]!;
      console.log(`[reopen-storm] ${index + 1}/${wanted.length} ${name}: ${law}`);
      const { status, lines } = await runLawProcess(
        "cargo",
        ["test", "-p", "semio-framework-os-kernel-db", "--features", features, "--lib", "--no-fail-fast", "--", "--exact", law, "--nocapture", "--test-threads=1", ...(claimed ? ["--include-ignored"] : [])],
        { cwd: this.repoRoot, env: { ...process.env, CARGO_INCREMENTAL: "0", RUST_MIN_STACK: "268435456", SEMIO_DB_ISOLATED_LAW: law } },
        () => {
          cancelled = true;
        },
      );
      const passed = status === 0 && lines.some((line) => line.includes("test result: ok. 1 passed"));
      measured[`${name}Pass`] = passed;
      const storm = lines.map((line) => /storm of (\d+) in (\d+) ms: welcome p50 ([\d.]+) ms .*? max ([\d.]+) ms(?:.*solo ([\d.]+) ms)?/u.exec(line)).find((match) => match !== null);
      if (storm) Object.assign(measured, { [`${name}Documents`]: Number(storm[1]), [`${name}StormMs`]: Number(storm[2]), [`${name}WelcomeP50Ms`]: Number(storm[3]), [`${name}WelcomeMaxMs`]: Number(storm[4]) }, storm[5] === undefined ? {} : { [`${name}SoloMs`]: Number(storm[5]) });
      if (!passed) failed.push(name);
    }
    const status = cancelled ? "skipped" : failed.length === 0 ? "pass" : "fail";
    publishAcceptanceCheckResult(
      this.repoRoot,
      acceptanceCheckResult({
        check: "hub-reopen-storm",
        status,
        startedAt,
        measured: { ...measured, laws: wanted.join(","), cancelled },
        summary: {
          en: `reopen storm laws ${wanted.length - failed.length}/${wanted.length} pass (${wanted.join(", ")})${failed.length ? `; failing: ${failed.join(", ")}` : ""}${cancelled ? "; cancelled" : ""}`,
          de: `Wiederöffnungssturm-Gesetze ${wanted.length - failed.length}/${wanted.length} bestanden (${wanted.join(", ")})${failed.length ? `; fehlgeschlagen: ${failed.join(", ")}` : ""}${cancelled ? "; abgebrochen" : ""}`,
        },
      }),
    );
    if (status !== "pass") process.exitCode = 1;
  }
}
//#endregion 🌪️ReopenStorm

/** 🌐️ Checks owner removal with neutral schema vectors and independent Ajv validation. */
export class DocumentHttpCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🌐️document-http/🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    const ajv = semioSchemaAjvV1({ strict: false });
    const declarationSchema = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧬️schema/🔣️.json"), "utf8"));
    const declared = ajv.compile(declarationSchema);
    assert(declared(fixture.neutral));
    assert(declared(fixture.secondary));
    const replyBounds=ajv.compile(JSON.parse(readFileSync(join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🌐️document-http/🧪️testing/📤️reply-node-bounds/🧬️schema/🔣️.json"),"utf8")).$defs.BoundedReply);
    for(const vector of fixture.replyNodeBounds.vectors) assert.equal(replyBounds(Array(vector.items).fill(null)),vector.valid);
    const validate = ajv.compile(JSON.parse(fixture.neutral.operations[0].inputSchema));
    for (const vector of fixture.vectors) assert.equal(validate(vector.value), vector.valid, vector.name);
    for (const law of ["os_directory::client::document_http::tests::owner_removal_preserves_neutral_document_transport", "os_directory::client::document_http::tests::schema_vectors_match_owned_validator", "os_directory::client::document_http::tests::decoded_replies_obey_the_same_node_bounds_as_owner_inputs"]) {
      await runRepositoryCargoTests(["semio-framework-os-kernel"], this.root, this.invocation, ["--lib", "--", law, "--exact", "--nocapture"]);
    }
    await runRepositoryCargoTests(["semio-framework-os-kernel"], this.root, this.invocation, ["--lib", "--", "os_directory::client::tests::document_http_transport_preserves_scope_bounds_and_owner_decode", "--exact"]);
    console.log("document-http: Ajv vectors, removal/reinstall, scope, bounds and owner decoding passed");
  }
}
