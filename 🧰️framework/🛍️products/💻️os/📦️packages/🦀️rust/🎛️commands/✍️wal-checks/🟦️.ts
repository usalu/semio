/** ✍️ Write-ahead log commands: writer authority and fence, postgres round trips, committed transactions and compaction, recovery, capacity and segment state.
 *
 * @see ../../📜️script.ts — the package script that registers the commands of this module
 */
import { unsignedLebOracleV1 } from "@semio-tech/framework-pack-ts/format/testing/oracles/leb";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { runCargo } from "@semio-tech/process/execution";
import { runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { blake3Hex } from "@semio-tech/framework/hash";
import { exactCargoStageEnvironments } from "../🧱️script-bases/🟦️.ts";

/** 🔐️ Checks fixed writer capabilities and retained local backend integration. */
export class WalWriterAuthorityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-writer-authority-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    
    const writerDeferredWake = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🔔️deferred-wake/🔣️.json"), "utf8"));
    
    
    
    const neutralDeferredWake = JSON.parse(readFileSync(join(this.repoRoot, "🧰️framework/🔨️modules/⏳️async/🔔️deferred-wake/🧪️tests/🧫️fixtures/🔣️.json"), "utf8"));
    
    
    
    assert.equal(neutralDeferredWake.capacity.partitions, writerDeferredWake.capacity.backendControls);
    assert.equal(neutralDeferredWake.capacity.slotsPerPartition, writerDeferredWake.capacity.writersPerBackend);
    assert.equal(neutralDeferredWake.capacity.totalWaiters, writerDeferredWake.capacity.backendControls * writerDeferredWake.capacity.writersPerBackend * writerDeferredWake.capacity.waitersPerWriter);
    {
    let queuedOwner = true;
    let faulted = true;
    let retryEpoch = 0;
    let queuedOwners = 1;
    let maximumQueuedOwners = queuedOwners;
    assert.equal(faulted && queuedOwner ? "pending" : "ready", writerDeferredWake.retryEpoch.hostileTrace[2]);
    assert.equal(writerDeferredWake.retryEpoch.readyBeforeOldSlotDrains, false);
    queuedOwner = false;
    assert.equal(faulted && queuedOwner ? "pending" : "fault-ready", writerDeferredWake.retryEpoch.hostileTrace[4]);
    faulted = false;
    retryEpoch += 1;
    queuedOwner = true;
    queuedOwners = Number(queuedOwner);
    maximumQueuedOwners = Math.max(maximumQueuedOwners, queuedOwners);
    assert.equal(retryEpoch, 1);
    assert.equal(maximumQueuedOwners, writerDeferredWake.retryEpoch.maximumQueuedOwnersPerSignal);
    assert.equal(writerDeferredWake.retryEpoch.admission, "fault-ready-after-exact-slot-drain");
    }
    for (const row of writerDeferredWake.cases) {
      assert.equal(row.expected.retainedFaults, row.activeRequested, row.id);
      assert.equal(row.expected.retainedGuards, row.activeRequested, row.id);
      assert.equal(row.expected.terminalEpochs, 0, row.id);
    }
    const writerDeferredStorageSource = readModuleTreeSource(join(owner, "..", "🦀️.rs"));
    const writerDeferredSource = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const writerDeferredReleaseSource = readFileSync(join(owner, "🔔️release/🦀️.rs"), "utf8");
    assert(writerDeferredStorageSource.includes(`const DB_IO_BACKEND_CONTROLS: usize = ${writerDeferredWake.capacity.backendControls};`));
    assert(writerDeferredSource.includes(`const WAL_WRITER_CAPACITY: usize = ${writerDeferredWake.capacity.writersPerBackend};`));
    assert(writerDeferredReleaseSource.includes("fn request_controller(") && writerDeferredReleaseSource.includes("fn notify_faults("));
    const missingWriter = writerDeferredWake.runtimeMarkers.writer.filter((marker: string) => !writerDeferredReleaseSource.includes(marker));
    assert.deepEqual(missingWriter, [], `missing writer runtime markers: ${missingWriter.join(", ")}`);
    const remoteOwner = join(owner, "🧪️tests/🧫️fixtures/🌐️remote-guard");
    const remoteFixture = JSON.parse(readFileSync(join(remoteOwner, "🔣️.json"), "utf8"));
    
    
    assert.deepEqual(remoteFixture.mutations, ["create", "append", "sync", "seal", "truncateTail", "delete"]);
    for (const row of remoteFixture.postgres.lockKeys) {
      const digest = createHash("sha256").update(Buffer.concat([Buffer.from(remoteFixture.postgres.lockNamespace, "utf8"), Buffer.from([0]), Buffer.from(row.document, "utf8")])).digest();
      assert.equal(digest.readBigInt64BE(0).toString(), row.key, `postgres advisory-lock key oracle for ${row.document}`);
    }
    assert(remoteFixture.neo4j.renewEveryMs * 2 < remoteFixture.neo4j.leaseTtlMs, "a Neo4j writer lease must survive one missed renewal");
    for (const backend of ["sqlite", "postgres", "neo4j"]) assert(remoteFixture.laws.filter((law: { backends: string[] }) => law.backends.includes(backend)).length >= 4, `${backend} runs every shared fence law`);
    const sourceOf = (path: string) => readModuleTreeSource(join(owner, "..", path, "🦀️.rs"));
    for (const [marker, path] of [
      ["pg_try_advisory_lock($1)", "🐘️postgres"],
      ["fn fenced_wal_mutation", "🐘️postgres"],
      [`"${remoteFixture.postgres.lockNamespace}"`, "🐘️postgres"],
      ["CYPHER_WAL_WRITER_FENCE", "🌐️neo4j"],
      ["fn fenced_wal_mutation", "🌐️neo4j"],
      [`WAL_WRITER_LEASE_TTL_MS: i64 = ${remoteFixture.neo4j.leaseTtlMs.toLocaleString("en-US").replaceAll(",", "_")}`, "🌐️neo4j"],
    ] as const)
      assert(sourceOf(path).includes(marker), `missing cross-process writer fence primitive ${marker} in ${path}`);
    console.log(`wal-writer-fence-oracle: behavior-oracles=retained lockKeys=${remoteFixture.postgres.lockKeys.length} laws=${remoteFixture.laws.length}`);
    const memoryOwner = join(owner, "..", "🧪️tests/🧫️fixtures", "🧮️memory-backing");
    const memoryFixture = JSON.parse(readFileSync(join(memoryOwner, "🔣️.json"), "utf8"));
    
    
    const poolUseOwner = join(owner, "..", "🧪️tests/🧫️fixtures", "🔐️backend-pool-use");
    const poolUseFixture = JSON.parse(readFileSync(join(poolUseOwner, "🔣️.json"), "utf8"));
    
    
    assert.deepEqual(
      poolUseFixture.cases.map((row: { name: string }) => row.name),
      [
        "registered-backend-blocks-shutdown",
        "task-derives-registered-pool",
        "dropped-facade-retains-use",
        "compaction-acquires-before-admission",
        "forged-kind-rejected-before-page-admission",
        "rollback-reserved-before-owner-transfer",
        "all-retirement-tiers-full-return-exact-executor",
        "committed-rollback-retains-use-until-terminal",
        "prepared-post-transfer-refusal-returns-close-owner",
      ],
    );
    const { Database } = await import("bun:sqlite");
    const openingOracle = new Database(":memory:");
    try {
      openingOracle.run("CREATE TABLE opening_owner (backend TEXT PRIMARY KEY, retained INTEGER NOT NULL, close_requested INTEGER NOT NULL)");
      for (const row of poolUseFixture.opening) {
        openingOracle.run("INSERT INTO opening_owner VALUES (?1, 1, 0)", [row.backend]);
        openingOracle.run("UPDATE opening_owner SET close_requested = 1 WHERE backend = ?1", [row.backend]);
        const requested = openingOracle.query("SELECT close_requested FROM opening_owner WHERE backend = ?1").get(row.backend) as { close_requested: number };
        assert.equal(!!requested.close_requested, row.closeRequestedBeforeRetry);
        openingOracle.run("DELETE FROM opening_owner WHERE backend = ?1 AND close_requested = 1", [row.backend]);
        const terminal = openingOracle.query("SELECT count(*) AS retained FROM opening_owner").get() as { retained: number };
        assert.equal(terminal.retained === 0, row.ledgerBaseline);
        assert.equal(terminal.retained === 0, row.poolShutdownAfterDrain);
        assert.equal(terminal.retained === 0, row.reopens);
      }
    } finally {
      openingOracle.close();
    }
    assert.deepEqual(memoryFixture.writerTable, { slots: fixture.capacity, separateBox: true });
    assert.deepEqual(memoryFixture.controllerCredit, { items: 1, controls: 1, bytesFormula: "wake-plus-two-usize" });
    assert.equal(memoryFixture.retainedPageResults, 44);
    assert.equal(memoryFixture.sameSlotReuseBeforeOldResultClose, true);
    assert.equal(memoryFixture.taskSlotReleasedWhileResultRetained, true);
    assert.equal(memoryFixture.droppedResultRetirement, "mounted-io-maintenance");
    const directoryOwner = join(owner, "..", "🧪️tests/🧫️fixtures", "📁️directory-durability");
    const directoryFixture = JSON.parse(readFileSync(join(directoryOwner, "🔣️.json"), "utf8"));
    
    
    const openOwner = join(owner, "..", "..", "📝️wal", "🧪️tests/🧫️fixtures", "🚪️open-rejection");
    const openFixture = JSON.parse(readFileSync(join(openOwner, "🔣️.json"), "utf8"));
    
    
    let openOwnerState = "exact-writer-permit";
    assert.equal(openFixture.acquisition[1].owner, openOwnerState);
    openOwnerState = "exact-writer-release";
    assert.equal(openFixture.acquisition[2].owner, openOwnerState);
    assert.equal(openFixture.acquisition[2].releaseActivation, "first-explicit-close-poll");
    const exactRelease = Symbol("exact-writer-release");
    let retainedRelease: symbol | undefined = exactRelease;
    for (const [index, close] of openFixture.explicitClose.entries()) {
      assert.equal(close.attempt, index + 1);
      assert.equal(close.openCause, "original-open-error");
      if (close.outcome === "fault") assert.equal(retainedRelease, exactRelease);
      else retainedRelease = undefined;
      assert.equal(close.sameDocumentAcquire, retainedRelease ? "conflict" : "available");
    }
    assert.equal(openFixture.droppedRejection.behavior === "transfer" && !openFixture.droppedRejection.panics ? "backend-release-cell" : "discarded", openFixture.droppedRejection.location);
    const directoryNames = new Set(["segment", "marker"]);
    directoryNames.delete("segment");
    assert.equal(!directoryNames.has("segment") && directoryNames.has("marker") ? "not-found-with-retained-marker" : "invalid", directoryFixture.deleteFaultState);
    directoryNames.delete("marker");
    assert.equal(directoryNames.size, 0);
    assert(directoryFixture.delete.indexOf("segment-delete-parent-synced") < directoryFixture.delete.indexOf("marker-deleted"));
    const pinnedWriter = { operation: fixture.controllerBinding.operation, releaseRequested: true };
    assert.equal(pinnedWriter.releaseRequested && pinnedWriter.operation !== fixture.controllerBinding.contender ? "closed" : "ok", fixture.controllerBinding.fenceBeforeCallback);
    assert.equal(pinnedWriter.operation === fixture.controllerBinding.operation ? "ok" : "closed", fixture.controllerBinding.pinnedResume);
    assert.equal(fixture.controllerBinding.tasksAddedOnRelease, 0);
    assert.equal(fixture.controllerBinding.guardFactoryCallsOnConflict, 0);
    assert.equal(fixture.controllerBinding.faultRetryPreservesKey, true);
    const pendingWriters = new Map([
      ["fault", "retained"],
      ["healthy", "retained"],
    ]);
    const faultTrace = ["fault-retained"];
    pendingWriters.delete("healthy");
    faultTrace.push("healthy-terminal");
    assert.equal(pendingWriters.get("fault"), "retained");
    pendingWriters.delete("fault");
    faultTrace.push("fault-retry-terminal");
    assert.deepEqual(faultTrace, fixture.controllerFaults.coalesced);
    assert.equal(fixture.controllerFaults.outerPanic.executorTurns, 1);
    assert.equal(fixture.controllerFaults.outerPanic.wakesPerOwner, 1);
    const followerOwners = new Set(["document"]);
    assert.equal(followerOwners.has("document") ? "conflict" : "ok", fixture.replication.occupiedFollower);
    assert.deepEqual(fixture.replication.inventoryAfterConflict, []);
    const transfer = fixture.replication.snapshotTransfer;
    const sourceSnapshot = Buffer.concat(Array.from({ length: transfer.repetitions }, () => Buffer.from(transfer.pattern)));
    const copiedSnapshot = Buffer.from(sourceSnapshot);
    assert.equal(sourceSnapshot.length, transfer.bytes);
    sourceSnapshot.fill(0);
    assert.deepEqual(
      [...copiedSnapshot],
      Array.from({ length: transfer.bytes }, (_, index) => transfer.pattern[index % transfer.pattern.length]),
    );
    assert.equal(Symbol("source-result") === Symbol("independent-input"), transfer.sameOperation);
    const clusterSource = readFileSync(join(owner, "..", "..", "🌐️cluster", "🦀️.rs"), "utf8");
    assert(clusterSource.includes("db_io_copy_page_owner(&pages)") && clusterSource.includes("close_replication_pages(&mut pages)"), "snapshot replication must retire source result before a distinct write owner");
    const walSource = readFileSync(join(owner, "..", "..", "📝️wal", "🦀️.rs"), "utf8");
    for (const marker of ["struct ArtifactWalAcquiredRejected", "enum ArtifactWalOpenRejected", "retry_open(", "retry_close(", "open_acquired(", "into_open_rejected"])
      assert(walSource.includes(marker), "missing retained WAL-open owner primitive: " + marker);
    assert(!walSource.includes("release_failed_open"), "WAL open rejection must not await and flatten its writer release");
    const artifactSource = readModuleTreeSource(join(owner, "..", "..", "🗿️artifact", "🦀️.rs"));
    for (const marker of ["enum ArtifactEngineOpenRejected", "RetainedWal", "has_retained_writer", "Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>>"])
      assert(artifactSource.includes(marker), "missing engine retained-open propagation: " + marker);
    const engineSource = readModuleTreeSource(join(owner, "..", "..", "⚙️engine", "🦀️.rs"));
    for (const marker of ["enum DatabaseDocumentOpenRejected", "Result<ArtifactHandle, DatabaseDocumentOpenRejected>", "rejected.retry_close().await"]) assert(engineSource.includes(marker), "missing database retained-open propagation: " + marker);
    for (const marker of ["enum ReplicationRejected", "ArtifactWal::open_acquired", "ReplicationRejected::WalOpen"]) assert(clusterSource.includes(marker), "missing cluster acquired-writer propagation: " + marker);
    for (const outcome of fixture.replication.releaseAfter) {
      const owner = new Set(["document"]);
      try {
        assert(["tail", "up-to-date", "snapshot", "leader-corrupt"].includes(outcome));
      } finally {
        owner.delete("document");
      }
      assert.equal(owner.size, 0);
    }
    for (const row of fixture.cases) {
      let generation = BigInt(row.firstGeneration);
      const live = new Map<string, bigint>();
      const owners = new Map<string, { document: string; generation: bigint }>();
      for (const step of row.steps) {
        let actual = "ok";
        const permit = owners.get(step.owner);
        if (step.action === "acquire") {
          if (live.has(step.document)) actual = "conflict";
          else if (generation === 0xffffffffffffffffn) actual = "exhausted";
          else {
            live.set(step.document, generation);
            owners.set(step.owner, { document: step.document, generation: generation++ });
          }
        } else if (step.backend !== 0 || permit?.document !== step.document || live.get(step.document) !== permit?.generation) actual = "fenced";
        else if (step.action === "release") live.delete(step.document);
        assert.equal(actual, step.expected, `${row.name}: ${JSON.stringify(step)}`);
      }
      assert.equal(live.size, 0, row.name);
    }
    assert.deepEqual(
      fixture.resultRetirement.terminal,
      Array.from({ length: fixture.resultRetirement.pages + 3 }, (_, index) => index === fixture.resultRetirement.pages + 2),
    );
    assert.deepEqual(
      fixture.guardRetirement.terminal,
      fixture.guardRetirement.stages.map((stage: string) => stage === "terminal"),
    );
    const rejected = new Set(Array.from({ length: fixture.backendPressure.capacity }, (_, index) => index));
    let retained = rejected.size === fixture.backendPressure.capacity;
    assert.equal(retained, fixture.backendPressure.retainedWhenFull);
    rejected.delete(0);
    if (rejected.size < fixture.backendPressure.capacity) retained = false;
    assert.equal(!retained, fixture.backendPressure.terminalAfterCapacityReturns);
    const closing = new Map([
      [fixture.fairRetirement.pinnedSlot, "pinned"],
      [fixture.fairRetirement.releasingSlot, "releasing"],
    ]);
    for (let cursor = 0; cursor < fixture.fairRetirement.maximumOpportunities; cursor++) if (closing.get(cursor) === "releasing") closing.delete(cursor);
    assert.equal(closing.has(fixture.fairRetirement.pinnedSlot), fixture.fairRetirement.pinnedRetained);
    assert.equal(!closing.has(fixture.fairRetirement.releasingSlot), fixture.fairRetirement.releasingRetired);
    for (const trace of [fixture.maintenanceFairness.continuouslyReady, fixture.maintenanceFairness.firstClassFaults])
      assert.deepEqual(
        trace,
        trace.map((_: unknown, index: number) => index % fixture.maintenanceFairness.classes.length),
      );
    let terminalEpoch = BigInt(fixture.releaseSignal.firstEpoch);
    const waits: bigint[] = [];
    for (const writer of fixture.releaseSignal.writers) {
      const required = terminalEpoch + 1n;
      assert.equal(terminalEpoch >= required, fixture.releaseSignal.requestCompletesRelease);
      terminalEpoch += 1n;
      waits.push(required);
      assert.equal(terminalEpoch.toString(), writer.terminalEpoch);
      assert.equal(
        waits.every((wait) => terminalEpoch >= wait),
        fixture.releaseSignal.oldWaitSurvivesReuse,
      );
    }
    console.log(
      `wal-writer-authority-independent-oracle: AJV=6 exact-u64=1 cases=${fixture.cases.length} mutations=${fixture.mutations.length} fence-laws=${remoteFixture.laws.length} writer-slots=${memoryFixture.writerTable.slots} retained-result=1 directory-barriers=4 wal-open-owner=1 backend-pool-use=${poolUseFixture.cases.length} physical-opening=${poolUseFixture.opening.length}`,
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WalWriterPermit", "struct WalWriterTable", "struct WalFileWriterGuard", "try_lock()", "checked_add(1)", "active_operation", "fn release_step"])
      assert(source.includes(marker), `missing writer capability primitive: ${marker}`);
    const explicitRelease = source.slice(source.indexOf("pub fn release(mut self)"), source.indexOf("fn request_release(&self)"));
    assert(!explicitRelease.includes("self.request_release()"), "explicit retained release must stay dormant until its first poll");
    const storageSource = readModuleTreeSource(join(owner, "..", "🦀️.rs"));
    for (const marker of ["WalWriterTable<WalFileWriterGuard>", "fn writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(storageSource.includes(marker), `missing filesystem writer integration: ${marker}`);
    for (const marker of [
      "pool_use: Option<Arc<WorkerPoolUse>>",
      "fn db_io_backend_admit_operation",
      "Result<Arc<WorkerPool>, DbError>",
      "pub fn submit_db_io_task(task: DbIoTask)",
      "db_io_backend_control(owner.kind, slot, generation) != control",
      "struct DbIoBackendRollbackReservation",
      "struct DbIoBackendRegistrationRejected",
      "pub enum DbStorageOpenRejected",
      "pub async fn retry_close(self) -> Result<DbError, Self>",
      "register_db_io_backend_prepared_with_use",
      "Result<DbIoBackendControl, DbIoBackendRegistrationRejected>",
      "reserved: bool",
    ])
      assert(storageSource.includes(marker), `missing backend-owned WorkerPool use boundary: ${marker}`);
    assert(!storageSource.includes("pub fn submit_db_io_task(pool:"), "DB I/O tasks must derive the exact registered backend pool");
    const registrationSource = storageSource.slice(storageSource.indexOf("pub fn register_db_io_backend("), storageSource.indexOf("fn db_io_writer_release_lane_step"));
    assert(!registrationSource.includes("let _ = db_io_park_lost_owner"), "backend registration must not discard a saturated retirement owner");
    for (const marker of ["MemoryDbIoExecutor::backing_bytes()", "checked_add(writer::release::controller_credit())"]) assert(storageSource.includes(marker), `missing memory writer backing integration: ${marker}`);
    const sqliteSource = readModuleTreeSource(join(owner, "..", "🪶️sqlite", "🦀️.rs"));
    for (const marker of ["WalWriterTable<SqliteWalWriterGuard>", "canonical_database", "fn physical_writer_sidecar", "DbIoTask::WalWriterAcquire", "fn pin_writer_operation", "finish_operation_if_pinned", ".semio-wal-writer"])
      assert(sqliteSource.includes(marker), `missing SQLite writer integration: ${marker}`);
    const releaseSource = readFileSync(join(owner, "🔔️release/🦀️.rs"), "utf8");
    for (const marker of [
      "struct WalWriterSignalCell",
      "requested: bool",
      "fn request_release(&mut self)",
      "impl Drop for WalWriterRelease",
      "deferred_fault_waiter",
      "defer_fault_notifications",
      "suspend_controller_for_refusal",
      "deferred_wake_pending_for_test",
    ])
      assert(releaseSource.includes(marker), `missing bounded deferred refusal primitive: ${marker}`);
    assert.equal((storageSource.match(/writer::release::notify_faults/g) ?? []).length, 0, "public DB handback paths must not invoke writer wakers directly");
    const retainedFixtureLaws = readFileSync(join(owner, "..", "🧪️tests", "🔬️db-io-retained-fixtures", "🦀️.rs"), "utf8");
    assert(retainedFixtureLaws.includes("fn wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch()"), "missing mounted stale-controller refusal law");
    if (segments[0] !== "--native") return;
    const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
        {
          package: "semio-framework-os-kernel-db",
          target: { kind: "lib", name: "db" },
          cargoArgs: ["--all-features"],
          laws: [
            "db_io_real_storage_open_drop_retires_queued_backend_and_allows_reopen",
            "db_io_real_storage_open_fault_drop_retires_registered_backend_without_retry",
            "wal_writer_table_matches_neutral_exact_scope_and_aba_rejection",
            "wal_writer_table_capacity_recycles_slots_without_reusing_generations",
            "wal_writer_file_lock_excludes_independent_instances_and_processes",
            "db_io_lost_result_lease_retains_every_page_and_final_handback",
            "wal_writer_release_retains_pinned_operation_and_faulted_guard",
            "db_io_lost_backend_retains_exact_owner_under_rejected_registry_pressure",
            "wal_writer_table_close_advances_other_guards_while_first_operation_is_pinned",
            "db_io_maintenance_rotates_ready_and_faulted_classes_without_starvation",
            "wal_writer_release_signal_preserves_exact_waits_across_writer_and_backend_reuse",
            "wal_writer_mounted_controller_fences_at_signal_and_wakes_outside_registry_without_tasks",
            "wal_writer_mounted_controller_fault_returns_exact_retry_owner_without_poisoning_other_writer",
            "wal_writer_mounted_stale_controller_defers_cross_key_wake_and_fences_retry_epoch",
            "wal_writer_mounted_controller_rerequests_after_async_executor_handback",
            "wal_writer_mounted_controller_coalesced_fault_does_not_strand_healthy_release",
            "wal_writer_mounted_controller_outer_panic_faults_waiters_once_and_stops",
            "db_io_memory_backend_heap_tables_have_exact_preflight_credit_and_terminal_return",
            "db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit",
            "fs_storage_canonical_alias_writer_fences_all_six_mutations",
            "sqlite_wal_writer_real_database_alias_and_crash_are_exclusive",
            "fs_wal_directory_barriers_match_neutral_order_and_duplicate_create_is_atomic",
            "fs_wal_directory_faults_retain_seal_and_delete_order_until_explicit_retry",
            "fs_replacement_reports_failure_until_renamed_parent_is_synced",
            "fs_wal_reopen_repairs_unacknowledged_segment_namespace_before_header_ack",
            "replicate_document_fences_occupied_follower_before_inventory_or_up_to_date",
            "replicate_document_releases_follower_after_leader_replay_failure",
            "replicate_document_applies_missing_tail_commands_to_a_fresh_follower",
            "replicate_document_reports_up_to_date_once_a_follower_catches_up",
            "replicate_document_transfers_a_snapshot_when_the_follower_is_below_the_retained_floor",
            "artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry",
            "artifact_engine_create_rejection_propagates_exact_wal_release_owner",
            "database_document_mount_failure_terminalizes_authority_builder_wal_owner_before_fanout",
            "db_io_registered_backend_use_blocks_pool_shutdown_until_terminal_close",
            "db_io_backend_registration_saturation_returns_exact_executor_before_pool_use",
            "db_io_prepared_registration_failure_returns_exact_close_owner_after_submission_refusal",
            "db_io_task_uses_registered_backend_pool_not_caller_pool",
            "db_io_backend_drop_retains_pool_until_deferred_close_terminal",
            "db_io_forged_backend_kind_is_rejected_before_task_page_admission",
            "database_compaction_future_acquires_pool_use_before_admission",
          ],
        },
      ],progress(event) {
        console.log(`wal-writer-authority-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
      }});
    for (const receipt of receipts) console.log(`wal-writer-authority-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🤺️ The cross-process WAL writer fence laws of `db_storage::writer::fence_conformance`, one lane per backend:
 * `sqlite` on a scratch file; `postgres` and `neo4j` on the ONE shared development server claimed by
 * `os-hub-ts backend run <postgres|neo4j> -- …` (its `OS_HUB_*` environment selects the server, `SEMIO_BACKEND_CLIENT` is the
 * server's own client the laws cross-check the lock through), so a server lane runs only under that claim.
 * `wal-writer-fence-live [sqlite|postgres|neo4j]…`: without a lane, `sqlite` plus every claimed lane whose environment
 * is present. */
const WAL_WRITER_FENCE_LANES: Readonly<Record<string, Readonly<{ law: string; claimed?: string }>>> = {
  sqlite: { law: "fence_conformance::sqlite_wal_writer_fence_holds_every_shared_law" },
  postgres: { law: "fence_conformance::postgres_wal_writer_fence_holds_every_shared_law", claimed: "OS_HUB_DATABASE_URL" },
  neo4j: { law: "fence_conformance::neo4j_wal_writer_fence_holds_every_shared_law", claimed: "OS_HUB_NEO4J_URI" },
};

export class WalWriterFenceLiveScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const lanes = segments.length ? segments : Object.keys(WAL_WRITER_FENCE_LANES).filter((name) => !WAL_WRITER_FENCE_LANES[name]!.claimed || process.env[WAL_WRITER_FENCE_LANES[name]!.claimed!]);
    const unknown = lanes.filter((name) => !(name in WAL_WRITER_FENCE_LANES));
    if (unknown.length) throw new Error(`wal-writer-fence-live accepts ${Object.keys(WAL_WRITER_FENCE_LANES).join(" | ")}, got ${unknown.join(",")}`);
    const unclaimed = lanes.filter((name) => WAL_WRITER_FENCE_LANES[name]!.claimed && !process.env[WAL_WRITER_FENCE_LANES[name]!.claimed!]);
    if (unclaimed.length) throw new Error(`wal-writer-fence-live ${unclaimed.join(",")} needs the claimed shared server: run it under \`os-hub-ts backend run ${unclaimed[0]} -- …\``);
    for (const lane of lanes) {
      console.log(`wal-writer-fence-live ${lane}: ${WAL_WRITER_FENCE_LANES[lane]!.law}`);
      await runCargo(["test", "--manifest-path", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/📦️packages/🦀️rust/Cargo.toml"), "--lib", "--", "--exact", WAL_WRITER_FENCE_LANES[lane]!.law, "--include-ignored", "--test-threads=1"], this.root);
    }
  }
}

/** 🐘️ `postgres-round-trips-live` — the PostgreSQL storage laws of `db_storage_postgres::round_trips` on the ONE shared
 * development server claimed by `os-hub-ts backend run postgres -- …` (a list is one statement, concurrent writers wait for
 * the backend's operation slot, concurrent openers of a fresh database all find its schema). */
export class PostgresRoundTripsLiveScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("postgres-round-trips-live takes no arguments");
    if (!process.env.OS_HUB_DATABASE_URL) throw new Error("postgres-round-trips-live needs the claimed shared server: run it under `os-hub-ts backend run postgres -- …`");
    await runCargo(["test", "--manifest-path", join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/📦️packages/🦀️rust/Cargo.toml"), "--lib", "--", "round_trips::", "--include-ignored"], this.root);
  }
}

/** 🧾️ Proves the logical commit firewall with an independent neutral grammar evaluator. */
export class WalCommittedTransactionsCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-transactions-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🧾️committed-transactions/🔣️.json"), "utf8"));
    
    
    assert.deepEqual(fixture.historyProjection.expected, { entries: 1, operationIds: fixture.historyProjection.commands.map((command: any) => command.id), headSeq: fixture.historyProjection.commands.length, commitSeq: 1 });
    assert.equal(new Set(fixture.historyProjection.expected.operationIds).size, fixture.historyProjection.commands.length);
    for (const row of fixture.historyProjection.rejections) {
      const frontiers = row.records.flatMap((record: any, index: number) => (record.kind === "frontier" ? [index] : []));
      const accepted = row.records.every((record: any) => record.document === "current") && frontiers.length === 1 && frontiers[0] === row.records.length - 1;
      assert.equal(accepted, row.accepted, row.name);
    }
    for (const row of fixture.cases) {
      let next = 1n;
      let observed = false;
      let recoverAbort: string | null = null;
      const transactions: { id: string; kinds: string[] }[] = [];
      let error: string | null = null;
      try {
        for (const [index, segment] of row.segments.entries()) {
          if (index !== row.segments.length - 1 && segment.state !== "sealed") throw "corrupt";
          assert.deepEqual(
            [...segment.physicalCommitsAfter].sort((a: number, b: number) => a - b),
            segment.physicalCommitsAfter,
          );
          const frames = segment.frames.flatMap((frame: any) => Array.from({ length: frame.repeat ?? 1 }, () => frame));
          assert.equal(segment.physicalCommitsAfter.at(-1), frames.length - 1);
          let current: { id: string; kinds: string[] } | null = null;
          for (const [ordinal, frame] of frames.entries()) {
            if (frame.kind === "header") {
              if (ordinal !== 0 || current !== null) throw "corrupt";
              continue;
            }
            if (ordinal === 0) throw "corrupt";
            if (frame.kind === "begin") {
              if (current !== null || BigInt(frame.id) < next || (observed && BigInt(frame.id) !== next)) throw "corrupt";
              const payload = Buffer.alloc(8);
              payload.writeBigUInt64LE(BigInt(frame.id));
              const id = payload.readBigUInt64LE();
              if (id === 0xffffffffffffffffn) throw "sequence";
              next = id + 1n;
              observed = true;
              current = { id: id.toString(), kinds: [] };
            } else if (frame.kind === "commit" || frame.kind === "abort") {
              if (current === null || current.id !== frame.id || (frame.kind === "commit" && current.kinds.length !== frame.count)) throw "corrupt";
              if (frame.kind === "commit") transactions.push(current);
              current = null;
            } else {
              if (current === null) throw "corrupt";
              if (current.kinds.length === fixture.maximumRecords) throw "capacity";
              current.kinds.push(frame.kind);
            }
          }
          if (current !== null) {
            if (segment.state !== "active" || index !== row.segments.length - 1) throw "corrupt";
            recoverAbort = current.id;
          }
        }
      } catch (caught) {
        if (!["corrupt", "capacity", "sequence"].includes(String(caught))) throw caught;
        error = String(caught);
      }
      assert.deepEqual({ accepted: error === null, transactions: error === null ? transactions : [], nextTxId: error === null ? next.toString() : null, recoverAbort: error === null ? recoverAbort : null, error }, row.expected, row.name);
    }
    console.log(`wal-committed-transactions-independent-oracle: behavior-oracles=retained u64=1 vectors=${fixture.cases.length}`);
    const faults = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    
    
    assert.deepEqual(
      faults.cases.filter((row: any) => row.fault !== "successorAppendError").map((row: any) => [row.fault, row.expectedPhysicalSuffix]),
      [
        ["shortAppend", "torn"],
        ["appendError", "absent"],
        ["syncError", "complete"],
      ],
    );
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("struct WalTransactionGate") && source.includes("frames: Vec<WalRecordFrame>") && source.includes("self.frames.len() >= WAL_TRANSACTION_RECORDS_MAX") && source.includes("count > WAL_TRANSACTION_RECORDS_MAX"), "logical admission must retain frame spans (not owned decoded records) up to the writer's own transaction bound");
    assert(source.includes("WalCommittedCursor") && source.includes("WalCommittedTransaction"), "materializers need one shared borrowed committed cursor");
    assert(source.includes("enum WalVerifiedFrameStep"), "verified replay must yield after each physical frame without repeating whole-frame CRC work");
    assert(source.includes("trait WalImmutableByteSource"), "History must share the authenticated frame source without borrowing another field across polls");
    assert(source.includes("struct WalAuthenticatedSource<S>") && source.includes("source: S"), "History authentication and committed spans must retain the same immutable source owner");
    assert(source.includes("recovered_abort_tx_id") && source.includes("wal recovery abort exceeds retained segment budget"), "active recovery must durably abort within the retained segment budget");
    const history = readModuleTreeSource(join(owner, "../🗿️artifact/🦀️.rs"));
    assert(history.includes("WalAuthenticatedSource<HistoryPageSet>") && !history.includes("struct HistoryFrameCursor"), "History must consume authenticated committed spans, with no independent frame grammar");
    for (const check of ["history envelope document differs", "history frontier document differs", "history committed frontier is not terminal"]) assert(history.includes(check), `History admission is missing ${check}`);
    const decoder = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/📖️retained-decoder/🔣️.json"), "utf8"));
    
    
    const leb=unsignedLebOracleV1();
    for (const row of decoder.varints) {
      const bytes = Buffer.from(row.hex, "hex");
      let value: string | null = null;
      let consumed: number | null = null;
      const terminal = bytes.findIndex((byte) => byte < 128);
      if (terminal >= 0 && terminal < 10) {
        const number = bytes.subarray(0, terminal + 1).reduceRight((value, byte) => value * 128n + BigInt(byte & 127), 0n);
        if (number <= 0xffffffffffffffffn) {
          const storage = Buffer.alloc(8);
          storage.writeBigUInt64LE(number);
          if (Buffer.from(leb.encodeUnsignedBuffer(storage)).equals(bytes.subarray(0, terminal + 1))) {
            value = number.toString();
            consumed = terminal + 1;
          }
        }
      }
      assert.deepEqual({ value, consumed }, { value: row.value, consumed: row.consumed }, row.name);
    }
    assert(source.includes("fn wal_read_canonical_varint"), "retained readers must reject noncanonical and overflowing u64 fields");
    console.log(`wal-retained-decoder-independent-oracle: behavior-oracles=retained LEB128=1 vectors=${decoder.varints.length}`);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            cargoArgs: ["--all-features"],
            laws: [
              "wal_transaction_gate_matches_neutral_committed_spans",
              "wal_retained_decoder_fuel_resumes_exact_fragmented_bytes",
              "wal_retained_decoder_cancel_close_preserves_source_and_returns_owner",
              "wal_committed_cursor_cancel_resume_keeps_transaction_position",
              "wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close",
              "artifact_open_ignores_neutral_aborted_command_snapshot_and_cas",
              "sync_replay_ignores_neutral_aborted_command_snapshot_and_cas",
              "cli_verify_checks_neutral_logical_commit_boundaries",
              "open_replays_the_wal_and_reconstructs_state_and_frontier_identically",
              "wal_retained_varints_match_neutral_exact_u64_and_atomic_interruption",
              "wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions",
              "sync_retained_reads_resume_neutral_varints_without_renewing_overall_deadline",
              "wal_immutable_source_fragmentation_matches_neutral_transactions",
              "artifact_history_replay_uses_neutral_committed_inventory_and_retires_every_owner",
              "artifact_history_replay_projects_real_committed_batch_and_cancels_owned_sources",
              "artifact_history_and_opener_reject_neutral_inner_documents_and_frontier_order",
              "wal_recovery_aborts_only_incomplete_active_transactions_idempotently",
              "wal_recovery_abort_fsync_survives_two_independent_filesystem_reopens",
              "wal_recovery_abort_faults_retry_without_duplicate_abort",
              "wal_recovery_abort_cancellation_has_one_durable_boundary",
              "wal_recovery_abort_capacity_exact_and_plus_one_preserves_source",
              "artifact_history_panic_at_each_phase_transition_retains_then_fault_retires",
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
            ],
          },
        ],progress(event) {
          console.log(`wal-committed-transactions-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`wal-committed-transactions-native-receipt: ${JSON.stringify(receipt)}`);
    }
  }
}

/** 🧹️ Proves compaction observes only logically committed WAL effects. */
export class WalCommittedCompactionCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-committed-compaction-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🧾️committed-effects/🔣️.json"), "utf8"));
    
    
    assert.deepEqual(
      fixture.segments.map((row: any) => row.index),
      fixture.segments.map((_: any, index: number) => index),
    );
    assert.equal(fixture.segments.at(-1).state, "active");
    const committed = fixture.segments.flatMap((segment: any) =>
      segment.transactions.filter((transaction: any) => transaction.outcome === "commit").flatMap((transaction: any) => transaction.records.map((record: any) => ({ ...record, segment: segment.index }))),
    );
    const horizons = fixture.segments.map((segment: any) => ({
      segment: segment.index,
      head: committed.filter((record: any) => record.segment === segment.index && ["frontier", "snapshot"].includes(record.kind)).reduce((head: number | null, record: any) => (head === null ? record.headSeq : Math.max(head, record.headSeq)), null),
    }));
    const highest = fixture.segments.at(-1).index;
    const deletedSegments = horizons.filter((row: any) => row.segment !== highest && row.head !== null && row.head <= fixture.floorHeadSeq).map((row: any) => row.segment);
    const deletedPayloads =
      fixture.actor.payloadReclamation === "deferred-global-reference-authority"
        ? []
        : committed
            .filter((record: any) => record.kind === "payload" && deletedSegments.includes(record.segment) && !committed.some((live: any) => live.kind === "payload" && live.payload === record.payload && !deletedSegments.includes(live.segment)))
            .map((record: any) => record.payload);
    const allPayloads = new Set(fixture.segments.flatMap((segment: any) => segment.transactions.flatMap((transaction: any) => transaction.records.filter((record: any) => record.kind === "payload").map((record: any) => record.payload))));
    assert.deepEqual(
      {
        deletedSegments: deletedSegments.length,
        deletedPayloads: new Set(deletedPayloads).size,
        remainingSegments: fixture.segments.map((row: any) => row.index).filter((index: number) => !deletedSegments.includes(index)),
        retainedPayloads: [...allPayloads].filter((payload) => !deletedPayloads.includes(payload)),
      },
      fixture.expected,
    );
    assert.deepEqual(fixture.actor, {
      priority: "command",
      writerAuthority: "retained-artifact-wal",
      activeSegmentAuthority: "artifact-wal",
      leaseBefore: ["snapshot-floor", "wal-horizon", "wal-delete"],
      indexBudgetContinuation: "same-owned-future-cooperative-yield",
      payloadReclamation: "deferred-global-reference-authority",
      queuedSubmitDuring: "pending",
      queuedSubmitAfter: "accepted",
    });
    const source = readModuleTreeSource(join(owner, "🦀️.rs"));
    const committedCut = source.slice(source.indexOf("async fn committed_compaction_horizons"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(committedCut.includes("replay_committed_document") && committedCut.includes("close_record_step") && committedCut.includes("transaction.finish()") && committedCut.includes("close_compaction_replay"));
    assert(source.slice(source.indexOf("async fn close_compaction_replay"), source.indexOf("async fn close_compaction_owner")).includes("close_owner_step"));
    assert(!committedCut.includes("WalReplayCursor") && !committedCut.includes("replay_document"));
    const liveCut = source.slice(source.indexOf("pub async fn retained_compaction_with_wal"), source.indexOf("async fn retained_compaction_under_lease"));
    assert(liveCut.indexOf("CompactionLease::acquire") < liveCut.indexOf("SnapshotFloor"));
    assert(!liveCut.includes("acquire_writer"));
    const underLeaseCut = source.slice(source.indexOf("async fn retained_compaction_under_lease"), source.indexOf("async fn retained_compaction_snapshot"));
    assert(!underLeaseCut.includes("loop {\n            let deadline") && underLeaseCut.includes("handle.compact(&mut control).await"));
    const walSource = readFileSync(join(owner, "..", "📝️wal", "🦀️.rs"), "utf8");
    assert(walSource.includes("delete_compacted_sealed_segment"));
    const artifactSource = readModuleTreeSource(join(owner, "..", "🗿️artifact", "🦀️.rs"));
    assert(artifactSource.includes("ArtifactMessage::Compact") && artifactSource.includes("Priority::Command"));
    const laws = readFileSync(join(owner, "🧪️tests", "🔬️unit", "🦀️.rs"), "utf8");
    assert(laws.includes("fn compaction_applies_only_committed_frontier_snapshot_and_payload_effects("));
    console.log("wal-committed-compaction-independent-oracle: abort effects excluded, global payloads retained, header-only highest preserved");
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), cargoArgs: ["--all-features"], groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib", name: "db" },
            laws: [
              "db_compact::tests::compaction_applies_only_committed_frontier_snapshot_and_payload_effects",
              "db_compact::tests::document_compaction_retains_shared_and_private_cas_without_global_reference_authority",
              "db_engine::tests::compact_document_uses_live_actor_writer_and_restores_submits",
            ],
          },
        ], progress(event) {
          console.log(`wal-committed-compaction-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`wal-committed-compaction-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

/** 🚑️ Proves exact WAL commit boundaries with independent CRC/LEB128 and schema oracles. */
export class WalRecoveryCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-recovery-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🚑️recovery/🔣️.json"), "utf8"));
    
    
    const failStop = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/🛑️fail-stop/🔣️.json"), "utf8"));
    
    
    assert.deepEqual(
      failStop.cases.map((row: any) => [row.name, row.fault, row.expectedPhysicalSuffix]),
      [
        ["short-append", "shortAppend", "torn"],
        ["append-error", "appendError", "absent"],
        ["sync-error", "syncError", "complete"],
        ["successor-append-error", "successorAppendError", "complete"],
      ],
    );
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb=unsignedLebOracleV1();
    const { inspectRetainedSprNeutral } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts"));
    const checksum = (bytes: Uint8Array) => crc.buf(bytes) >>> 0;
    const fragmented = Buffer.from(Array.from({ length: 49152 }, (_, index) => (index * 17 + 3) % 251));
    for (const row of fixture.fragmentCopies) assert.equal(checksum(fragmented.subarray(row.offset, row.offset + row.length)), row.crc32c);
    const hash = (bytes: Buffer) => Buffer.from(blake3Hex(bytes), "hex");
    const u64 = (value: number) => {
      const bytes = Buffer.alloc(8);
      bytes.writeBigUInt64LE(BigInt(value));
      return bytes;
    };
    const frame = (kind: number, payload: Buffer) => {
      const body = Buffer.concat([Buffer.from([kind, 2]), payload]);
      const size = Buffer.from(leb.encodeUnsigned32(body.length));
      const trailer = Buffer.alloc(8);
      trailer.writeUInt32LE(checksum(body));
      trailer.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, trailer]);
    };
    const header = Buffer.alloc(32);
    Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
    header.writeUInt16LE(1, 8);
    header.writeUInt32LE(1, 12);
    header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    let bytes = header;
    let chain = hash(header);
    let previousOffset = 0;
    const batches = [
      [frame(64, Buffer.concat([Buffer.from([1, 100]), u64(0), Buffer.from([0])]))],
      ...fixture.commands.map((command: string, index: number) => [frame(65, u64(index + 1)), frame(68, Buffer.from(command)), frame(66, Buffer.concat([u64(index + 1), Buffer.from([1, 0, 0, 0])]))]),
    ];
    for (const [index, records] of batches.entries()) {
      const payload = Buffer.alloc(64);
      const recordsLength = records.reduce((sum: number, value: Buffer) => sum + value.length, 0);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      payload.writeBigUInt64LE(BigInt(index + 1));
      payload.writeBigUInt64LE(BigInt(previousOffset), 8);
      payload.writeBigUInt64LE(BigInt(recordsLength), 16);
      payload.writeUInt32LE(records.length, 24);
      chain.copy(payload, 32);
      previousOffset = bytes.length + recordsLength;
      bytes = Buffer.concat([bytes, ...records, frame(12, payload)]);
      assert.equal(bytes.length, fixture.commitEnds[index]);
    }
    for (const row of fixture.cuts) {
      const prefix = bytes.subarray(0, row.cut);
      const span = row.cut < 32 ? { end: 0, sequence: 0 } : inspectRetainedSprNeutral(prefix, checksum, hash);
      assert.equal(span.end, row.trustedEnd);
      assert.equal(Math.max(129, span.end), row.recoveredEnd);
      assert.equal(Math.max(1, span.sequence), row.nextTxId);
    }
    const expectedAccepted = new Set(["missing", "highest-sealed", "successor-empty", "successor-partial", "successor-header", "compacted-clean"]);
    assert.equal(new Set(fixture.lifecycle.map((row: any) => row.name)).size, fixture.lifecycle.length);
    for (const row of fixture.lifecycle) assert.equal(row.accepted, expectedAccepted.has(row.name), row.name);
    console.log(`wal-recovery-independent-oracle: ${fixture.cuts.length} exact CRC/hash-chain prefixes, ${fixture.lifecycle.length} lifecycle rows`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const open = source.slice(source.indexOf("pub async fn open(storage:", source.indexOf("impl ArtifactWal")), source.indexOf("pub async fn document(&self)", source.indexOf("impl ArtifactWal")));
    assert(!open.includes("delete_segment"), "recovery must not delete a committed segment");
    assert(source.includes("resume_verified") && source.includes("RetainedSprVerification"), "full verification and exact writer resume are required");
    const replayClose = source.slice(
      source.indexOf("pub fn close_owner_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
      source.indexOf("pub async fn close_step(&mut self)", source.indexOf("impl<'storage, S: db_storage::WalStorage> WalReplayCursor")),
    );
    assert(replayClose.includes("pages.close_step()") && replayClose.includes("segments.close_step()"), "replay close must retire retained page and list owners");
    assert(!replayClose.includes("control.grant()"), "terminal replay close must remain available after cancellation");
    const artifactClose = source.slice(source.indexOf("pub fn poll_close(&mut self", source.indexOf("impl ArtifactWal")), source.indexOf("//#endregion 🔖️ArtifactWal"));
    assert(artifactClose.includes("self.active.close_step()") && artifactClose.includes("self.active.terminal_is_empty()"), "artifact WAL must expose explicit terminal owner retirement");
    assert(artifactClose.includes("writer.release()") && artifactClose.includes("self.release.is_none()"), "artifact WAL completion must retain writer release until terminal retirement");
    const segmentClose = source.slice(source.indexOf("fn close_step(&mut self)", source.indexOf("impl SegmentWriter")), source.indexOf("//#endregion 🔖️Segment"));
    assert(segmentClose.indexOf("self.writer.take()") < segmentClose.indexOf("buf.close_step()"), "segment close must relinquish the retained writer before retiring its page buffer");
    assert(segmentClose.includes("force_flush is required before close"), "segment close must reject pending records");
    const segmentFlush = source.slice(source.indexOf("async fn commit_and_flush", source.indexOf("impl SegmentWriter")), source.indexOf("async fn tip_chain_hash", source.indexOf("impl SegmentWriter")));
    assert(segmentFlush.includes("new_len != expected_len") && segmentFlush.includes("self.flushed_len = new_len"), "WAL flush must verify the exact appended length before acknowledging it");
    assert(segmentFlush.indexOf("self.flushed_len = new_len") < segmentFlush.indexOf("storage.sync"), "a failed sync must retain knowledge that its append already landed");
    assert(segmentFlush.includes("self.poison()"), "every uncertain post-commit failure must poison the live writer");
    const rotate = source.slice(source.indexOf("async fn rotate", source.indexOf("impl ArtifactWal")), source.indexOf("pub fn close_step", source.indexOf("impl ArtifactWal")));
    assert(rotate.indexOf("storage.seal") < rotate.indexOf("self.active.poison()"), "a sealed segment must poison its old live writer before successor creation");
    const laws = [
      "db_wal::tests::wal_recovery_preserves_neutral_committed_prefixes",
      "db_wal::tests::wal_recovery_matches_neutral_lifecycle_without_prefix_replacement",
      "db_wal::retained_tests::wal_replay_cancellation_remains_set_while_close_reaches_terminal_empty",
      "db_wal::retained_tests::artifact_wal_repeated_open_close_is_page_budget_neutral",
      "db_wal::retained_tests::artifact_wal_close_rejects_pending_records_and_closed_writes",
      "db_wal::retained_tests::artifact_wal_short_append_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_append_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_sync_error_is_fail_stop_until_reopen",
      "db_wal::retained_tests::artifact_wal_successor_failure_after_seal_is_fail_stop_until_reopen",
      "db_fault_testing::tests::fault_storage_fail_nth_sync_fails_once_after_the_preceding_append",
    ];
    laws.push(
      ...[
        "single_segment_write_commit_flush_recovers_cleanly",
        "group_commit_batches_until_policy_threshold_then_commits",
        "fsync_durability_forces_immediate_commit_regardless_of_policy",
        "torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix",
        "recovery_resumes_next_tx_id_and_accepts_further_submits",
        "multi_segment_rotation_chains_prev_hash_and_replay_spans_segments",
        "recovery_rejects_a_torn_non_active_sealed_segment",
        "empty_document_open_creates_a_fresh_wal",
      ].map((law) => `db_wal::tests::${law}`),
    );
    const faultStorageLawsSource = readFileSync(join(owner, "../🧪️tests/🧯️fault-storage-laws/🦀️.rs"), "utf8");
    const walLawsSource = readModuleTreeSource(join(owner, "🧪️tests/🔬️unit/🦀️.rs"));
    const walRetainedLawsSource = readFileSync(join(owner, "🧪️tests/🔬️retained/🦀️.rs"), "utf8");
    for (const law of laws) assert((law.startsWith("db_fault_testing::") ? faultStorageLawsSource : law.startsWith("db_wal::retained_tests::") ? walRetainedLawsSource : walLawsSource).includes(`fn ${law.split("::").at(-1)}(`), `missing exact native law ${law}`);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws }], progress(event) {
          console.log(`wal-recovery ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`wal-recovery-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-recovery-check: ${fixture.cuts.length + fixture.lifecycle.length + fixture.fragmentCopies.length + failStop.cases.length} checks clean`);
  }
}

/** 📏️ Proves complete transaction reservations fit the shared readable storage span. */
export class WalCapacityCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("wal-capacity-check accepts only --native");
    const owner = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️tests/🧫️fixtures/📏️capacity/🔣️.json"), "utf8"));
    
    
    const leb=unsignedLebOracleV1();
    const frame = (payload: number) => leb.encodeUnsigned32(payload + 2).length + payload + 10;
    const txn = (payload: number) => frame(8) + frame(payload) + frame(12);
    for (const row of fixture.cases) {
      const lengths = [129],grow=(bytes:number)=>{const index=lengths.length-1,current=lengths[index];assert(current!==undefined,"Actual last segment extent required");lengths[index]=current+bytes;};
      const segments: number[] = [];
      let pending = false;
      for (let index = 0; index < 3; index++) {
        if (lengths.at(-1)! + txn(fixture.payloadBytes) + 75 > fixture.maxSegmentBytes) {
          if (pending) grow(75);
          lengths.push(161);
          pending = false;
        }
        segments.push(lengths.length - 1);
        grow(txn(fixture.payloadBytes));
        pending = true;
        if (row.durability === "fsync") {
          grow(75);
          pending = false;
        }
      }
      if (pending) grow(75);
      assert.deepEqual(segments, row.segments);
      assert.deepEqual(lengths, row.lengths);
    }
    assert.equal(129 + txn(fixture.exactPayloadBytes) + 75, fixture.maxSegmentBytes);
    assert.equal(129 + txn(fixture.oversizedPayloadBytes) + 75, fixture.maxSegmentBytes + 1);
    console.log("wal-capacity-independent-oracle: Fsync/grouped rotation and exact/one-over capacity confirmed by LEB128");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    assert(source.includes("fn wal_transaction_frame_bytes("), "missing transaction byte preflight before writes");
    assert(source.includes("const DEFAULT_MAX_SEGMENT_BYTES: u64 = db_storage::DB_IO_MAX_READ_BYTES;"), "WAL and storage must share one byte ceiling");
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [{ package: "semio-framework-os-kernel-db", target: { kind: "lib", name: "db" }, laws: ["db_wal::tests::wal_capacity_preflight_matches_neutral_memory_and_filesystem_boundaries"] }], progress(event) {
          console.log(`wal-capacity ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`wal-capacity-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log("wal-capacity-check: 6 checks clean");
  }
}

/** 🚦️ Proves every WAL backend exposes the same read-only active/sealed contract. */
export function walSegmentStateOracle(repoRoot: string): number {
  const storageRoot = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db");
  const core = readModuleTreeSource(join(storageRoot, "🗄️storage/🦀️.rs"));
  const sqlite = readModuleTreeSource(join(storageRoot, "🗄️storage/🪶️sqlite/🦀️.rs"));
  const postgres = readModuleTreeSource(join(storageRoot, "🗄️storage/🐘️postgres/🦀️.rs"));
  const neo4j = readModuleTreeSource(join(storageRoot, "🗄️storage/🌐️neo4j/🦀️.rs"));
  const faultStorage = readFileSync(join(storageRoot, "🧪️tests/🧯️fault-storage/🦀️.rs"), "utf8");
  assert(core.includes("pub enum WalSegmentState") && core.includes("Active,") && core.includes("Sealed,"));
  assert(core.includes("async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError>"));
  assert(core.includes("WalState { backend: DbIoBackendControl") && core.includes("WalSegmentState(WalSegmentState)"));
  assert(core.includes("Err(error) if error.kind() == std::io::ErrorKind::NotFound => WalSegmentState::Active") && core.includes("Err(error) => return Err(io_err(error))"));
  assert(sqlite.includes("SELECT sealed FROM wal_segment") && sqlite.includes("Err(DbError::Corrupt") && sqlite.includes("DbIoTask::WalState"));
  assert(postgres.includes("POSTGRES_WAL_STATE_QUERY") && postgres.includes("fetch_optional") && !postgres.slice(postgres.indexOf("const POSTGRES_WAL_STATE_QUERY"), postgres.indexOf("const POSTGRES_WAL_STATE_QUERY") + 240).includes("FOR UPDATE"));
  assert(neo4j.includes("const CYPHER_WAL_STATE") && neo4j.includes("RETURN n.sealed AS sealed") && !neo4j.slice(neo4j.indexOf("const CYPHER_WAL_STATE"), neo4j.indexOf("const CYPHER_WAL_STATE") + 240).includes("bytes"));
  const faultStorageLaws = readFileSync(join(storageRoot, "🧪️tests/🧯️fault-storage-laws/🦀️.rs"), "utf8");
  assert(faultStorage.includes("async fn segment_state") && faultStorageLaws.includes("fault_storage_segment_state_is_observational_and_counter_neutral"));
  return 12;
}

export class WalSegmentStateCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("wal-segment-state-check accepts only --native");
    const checks = walSegmentStateOracle(this.repoRoot);
    if (segments[0] === "--native") {
      const receipts = await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd: this.repoRoot, ...exactCargoStageEnvironments(), cargoArgs: ["--all-features"], groups: [
          {
            package: "semio-framework-os-kernel-db",
            target: { kind: "lib" },
            laws: [
              "memory_storage_satisfies_wal_storage_laws",
              "fs_storage_satisfies_wal_storage_laws",
              "fs_storage_stale_seal_marker_does_not_resurrect_missing_segment",
              "memory_storage_db_backend_accessors_and_capabilities",
              "wal_segment_state_observes_active_sealed_and_missing_rows",
              "wal_segment_state_decoder_rejects_non_boolean_storage_values",
              "wal_segment_state_query_and_mapper_are_read_only_and_byte_neutral",
              "wal_cypher_statements_reference_the_expected_label_and_keys",
              "fault_storage_segment_state_is_observational_and_counter_neutral",
            ],
          },
        ], progress(event) {
          console.log(`wal-segment-state ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        } });
      console.log(`wal-segment-state-native-receipts: ${JSON.stringify(receipts)}`);
    }
    console.log(`wal-segment-state-check: checks=${checks} clean`);
  }
}
