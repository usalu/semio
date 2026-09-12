#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-async` task router: `bun ./📜️script.ts <test|typegen>`. */
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative } from "node:path";
import assert from "node:assert/strict";
import Ajv from "ajv";
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmdStatus, resolveTestLevel, runExactCargoLaws } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

function exactCargoStageEnvironments() {
  return {
    env: { ...process.env, RUST_MIN_STACK: process.env.SEMIO_BUILD_RUST_MIN_STACK ?? "33554432" },
    nativeEnv: { RUST_MIN_STACK: "268435456" },
  };
}

/** 🔔️ Neutral lifecycle and actual native/cooperative idle wake acceptance. */
class WorkerMaintenanceCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("worker-maintenance-check accepts only --native");
    const owner = join(this.root, "../../🔔️maintenance"), export_ = "MaintenanceFixture";
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const module_ = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).addSchema(module_).getSchema(`${module_.$id}#/$defs/${export_}`)!;
    assert(validate(fixture), JSON.stringify(validate.errors));
    const owners = new Map<string, { requested: boolean; running: boolean; closing: boolean }>();
    for (const step of fixture.lifecycle) {
      const entry = owners.get(step.owner);
      let actual: string;
      if (step.action === "install") { owners.set(step.owner, { requested: false, running: false, closing: false }); actual = "installed"; }
      else if (!entry) actual = "stale";
      else if (step.action === "request") { actual = entry.closing ? "closed" : entry.requested ? "coalesced" : "requested"; if (!entry.closing) entry.requested = true; }
      else if (step.action === "take") { assert(entry.requested && !entry.running); entry.requested = false; entry.running = true; actual = "running"; }
      else if (step.action === "remove") { entry.closing = true; entry.requested = false; if (entry.running) actual = "pending"; else { owners.delete(step.owner); actual = "removed"; } }
      else { assert(entry.running); entry.running = false; if (entry.closing) entry.requested = false; else if (step.action === "finish-more") entry.requested = true; actual = entry.requested ? "requested" : "idle"; }
      assert.equal(actual, step.expected, JSON.stringify(step));
    }
    assert.equal(owners.size, 0);
    assert.deepEqual(fixture.competingWork.order, fixture.competingWork.jobs.flatMap((job: number, index: number) => [job, fixture.competingWork.hooks[index]]));
    assert.equal(fixture.selfRetire.cycles, fixture.capacity);
    assert(fixture.selfRetire.requestWhileRunning && fixture.selfRetire.reusesCapacity);
    assert.equal(fixture.selfRetire.laterRequest, "stale");
    console.log(`worker-maintenance-independent-oracle: AJV=1 lifecycle=${fixture.lifecycle.length} capacity=${fixture.capacity} native=1 cooperative=1`);
    assert(existsSync(join(owner, "🦀️.rs")), "missing fixed maintenance-hook implementation");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WorkerMaintenanceTicket", "struct WorkerMaintenanceRegistry", "enum PoolWork", "closed: bool", "entry.running", "entry.closing", "WorkerMaintenanceStep::Retire", "checked_add(1)", "fn shutdown(", "fn finish("]) assert(source.includes(marker), `missing maintenance owner primitive: ${marker}`);
    const pool = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    for (const api of ["install_maintenance_hook", "request_maintenance", "remove_maintenance_hook"]) assert.equal(pool.match(new RegExp(`pub fn ${api}\\(`, "g"))?.length, 2, `native/cooperative API mismatch: ${api}`);
    assert(pool.includes("job.run(&inner.maintenance)") && pool.includes("job.run(&self.inner.maintenance)"), "both pool schedulers must run fixed work under their existing permits");
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [{ package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: ["worker_maintenance_matches_neutral_retention_and_aba_lifecycle", "worker_maintenance_capacity_and_pool_identity_are_exact", "worker_maintenance_running_callback_retires_requested_generation_exactly_once", "worker_maintenance_native_idle_wake_uses_no_queued_job", "worker_maintenance_native_self_retire_reuses_all_fixed_slots", "worker_maintenance_cooperative_wake_obeys_pump_and_drr", "worker_maintenance_native_running_close_and_shutdown_keep_exact_invocation", "worker_maintenance_native_interleaves_io_jobs_and_rotating_hooks", "worker_maintenance_cooperative_interleaves_io_jobs_and_rotating_hooks", "native_drr_finishes_eligible_deficit_frontier_before_idle", "cooperative_maintenance_retains_deficit_until_later_host_turn", "cooperative_maintenance_snapshot_contention_preserves_queued_job", "cooperative_maintenance_live_host_revisits_queued_owner"] }], artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listBudgetMs: 60_000, lawBudgetMs: 120_000, progress(event) { console.log(`worker-maintenance-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
    for (const receipt of receipts) console.log(`worker-maintenance-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 💤️ Proves the fixed deferred-waker runtime and hostile retry admission fence. */
class WorkerDeferredWakeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("worker-deferred-wake-check accepts only --native");
    const owner = join(this.root, "../../🔔️deferred-wake"), export_ = "DeferredWakeFixture";
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const module_ = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).addSchema(module_).getSchema(`${module_.$id}#/$defs/${export_}`)!;
    assert(validate(fixture), JSON.stringify(validate.errors));
    const capacity = fixture.capacity;
    assert.equal(capacity.partitions, capacity.backendControls);
    assert.equal(capacity.slotsPerPartition, capacity.writersPerBackend);
    assert.equal(capacity.totalWaiters, capacity.backendControls * capacity.writersPerBackend * capacity.waitersPerWriter);
    assert.equal(capacity.totalWaiters, capacity.partitions * capacity.slotsPerPartition);
    let queuedOwner = true;
    let faulted = true;
    let retryEpoch = 0;
    let queuedOwners = 1;
    let maximumQueuedOwners = queuedOwners;
    assert.equal(faulted && queuedOwner ? "pending" : "ready", fixture.retryEpoch.hostileTrace[2]);
    assert.equal(fixture.retryEpoch.readyBeforeOldSlotDrains, false);
    queuedOwner = false;
    assert.equal(faulted && queuedOwner ? "pending" : "fault-ready", fixture.retryEpoch.hostileTrace[4]);
    faulted = false;
    retryEpoch += 1;
    queuedOwner = true;
    queuedOwners = Number(queuedOwner);
    maximumQueuedOwners = Math.max(maximumQueuedOwners, queuedOwners);
    assert.equal(retryEpoch, 1);
    assert.equal(maximumQueuedOwners, fixture.retryEpoch.maximumQueuedOwnersPerSignal);
    assert.equal(fixture.retryEpoch.admission, "fault-ready-after-exact-slot-drain");
    assert.equal(new Set(fixture.cases.map((row: { id: string }) => row.id)).size, fixture.cases.length);
    for (const row of fixture.cases) {
      const transferable = row.parkedWaiters - row.supersededWaiters;
      assert.equal(row.transferredWaiters + row.restoredWaiters, transferable, row.id);
      assert(row.transferredWaiters <= capacity.totalWaiters, row.id);
      const ring = Array<string | undefined>(capacity.totalWaiters);
      for (let index = 0; index < row.transferredWaiters; index += 1) ring[index] = `${row.id}:${index}`;
      assert.equal(ring.filter(Boolean).length, row.transferredWaiters, row.id);
      assert.equal(row.expected.inlineWakes, 0, row.id);
      let drained = 0;
      for (let index = 0; index < ring.length; index += fixture.dispatch.wakesPerTurn) {
        if (ring[index] !== undefined) { ring[index] = undefined; drained += 1; }
      }
      assert.equal(drained, row.expected.drainedWakes, row.id);
      assert.equal(row.expected.retainedFaults, row.activeRequested, row.id);
      assert.equal(row.expected.retainedGuards, row.activeRequested, row.id);
      assert.equal(row.expected.terminalEpochs, 0, row.id);
    }
    const asyncSource = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    const storageSource = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs"), "utf8");
    const writerSource = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs"), "utf8");
    const releaseSource = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs"), "utf8");
    assert(storageSource.includes(`const DB_IO_BACKEND_CONTROLS: usize = ${capacity.backendControls};`));
    assert(writerSource.includes(`const WAL_WRITER_CAPACITY: usize = ${capacity.writersPerBackend};`));
    assert(releaseSource.includes("fn request_controller(") && releaseSource.includes("fn notify_faults("));
    const missingAsync = fixture.runtimeMarkers.async.filter((marker: string) => !asyncSource.includes(marker));
    const missingWriter = fixture.runtimeMarkers.writer.filter((marker: string) => !releaseSource.includes(marker));
    assert.deepEqual(missingAsync, [], `missing async runtime markers: ${missingAsync.join(", ")}`);
    assert.deepEqual(missingWriter, [], `missing writer runtime markers: ${missingWriter.join(", ")}`);
    const markerCount = fixture.runtimeMarkers.async.length + fixture.runtimeMarkers.writer.length;
    console.log(`worker-deferred-wake-independent-oracle: AJV=1 cases=${fixture.cases.length} fixed-waiters=${capacity.totalWaiters} retry-epochs=${retryEpoch} max-queued-per-signal=${maximumQueuedOwners} inline-wakes=0 runtime-markers=${markerCount}/${markerCount}`);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-async",
        target: { kind: "lib", name: "semio_framework_async" },
        laws: [
          "deferred_wake::tests::worker_deferred_wake_matches_neutral_capacity_generation_and_shutdown_drain",
          "native_pool::tests::worker_deferred_wake_native_never_runs_inline_and_shutdown_drains_accepted_owner",
          "wasm_pool::cooperative_tests::worker_deferred_wake_cooperative_shutdown_requires_later_pump_to_drain",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`worker-deferred-wake-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`worker-deferred-wake-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🔐️ Proves the cold pool-use lifecycle fence for native and cooperative schedulers. */
class WorkerPoolUseCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("worker-pool-use-check accepts only --native");
    const owner = join(this.root, "../../🔐️use"), export_ = "UseFixture";
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const module_ = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).addSchema(module_).getSchema(`${module_.$id}#/$defs/${export_}`)!;
    assert(validate(fixture), JSON.stringify(validate.errors));
    for (const row of fixture.cases) {
      let state = "open";
      let uses = 0;
      let cells = 0;
      for (const step of row.steps) {
        if (step === "acquire") { assert.equal(state, "open", row.id); uses += 1; cells += 1; }
        else if (step === "clone") { assert(uses > 0, row.id); cells += 1; }
        else if (step === "drop") { assert(cells > 0, row.id); cells -= 1; if (cells === 0) uses -= 1; }
        else if (step === "shutdown-busy") { assert(uses > 0, row.id); assert.equal(state, "open", row.id); }
        else if (step === "shutdown") { assert.equal(uses, 0, row.id); assert.equal(state, "open", row.id); state = "stopped"; }
        else if (step === "acquire-rejected") assert.notEqual(state, "open", row.id);
        else if (step === "shutdown-idempotent") assert.equal(state, "stopped", row.id);
      }
      assert.equal(state, row.expected.state, row.id);
      assert.equal(uses, row.expected.retainedUses, row.id);
      assert.equal(state === "open", row.expected.executable, row.id);
    }
    for (const row of fixture.mountedCases) {
      const retainedUses = row.externalUses + (row.pool === "open" && (row.database === "open" || row.database === "opening-non-runnable") ? 1 : 0);
      const shutdown = retainedUses ? `busy-${retainedUses}` : "stopped";
      assert.equal(shutdown, row.expectedShutdown, row.id);
      if (row.authority === "ready") assert.equal(row.database, "open", row.id);
      if (row.database === "terminal" || row.database === "absent") assert.equal(row.databaseActivities, "closed", row.id);
    }
    const source = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    for (const marker of ["pub struct WorkerPoolUse", "pub enum WorkerPoolShutdownError", "pub enum WorkerPoolUseError", "pub fn acquire_use(&self)", "retained_uses", "PoolLifecycleState::Closing", "PoolLifecycleState::Stopped"]) assert(source.includes(marker), `missing pool-use marker ${marker}`);
    assert.equal(source.match(/pub fn shutdown\(&self\) -> Result<\(\), WorkerPoolShutdownError>/g)?.length, 2);
    for (const law of ["worker_pool_use_native_busy_keeps_executor_running_until_final_release", "worker_pool_use_acquire_and_shutdown_linearize_exactly_once", "worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release"]) assert(source.includes(`fn ${law}(`), `missing exact pool-use law ${law}`);
    const engine = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs"), "utf8");
    const artifact = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs"), "utf8");
    const sync = readFileSync(join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"), "utf8");
    for (const marker of ["pool_use: Option<Arc<WorkerPoolUse>>", "let pool_use = pool.acquire_use()", "fn require_open_use(&self)", "self.pool_use.take()", "DatabaseRetainedActivityRejected::Closed", "DatabaseDocumentMountDriver::NonRunnable", "DatabaseShutdownBlock::Executor(kind)"]) assert(engine.includes(marker), `missing mounted pool-use marker ${marker}`);
    assert.equal(engine.match(/_pool_use: Arc<WorkerPoolUse>/g)?.length, 4, "every retained Database capability/catalog state must own the use cell");
    for (const marker of ["_pool_use: Arc<semio_framework_async::WorkerPoolUse>", "spawn_with_pool_use", "pool.acquire_use()"] ) assert(artifact.includes(marker), `missing authority pool-use marker ${marker}`);
    for (const marker of ["_pool_use: std::sync::Arc<semio_framework_async::WorkerPoolUse>", "let pool_use = match pool.acquire_use()"] ) assert(sync.includes(marker), `missing sync-hello pool-use marker ${marker}`);
    for (const law of ["database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack", "database_worker_pool_use_is_admitted_before_the_first_storage_probe", "database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer"]) assert(engine.includes(`fn ${law}(`), `missing mounted pool-use law ${law}`);
    console.log(`worker-pool-use-independent-oracle: AJV=1 cases=${fixture.cases.length} mounted=${fixture.mountedCases.length} native=1 cooperative=1`);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{ package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: [
        "native_pool::tests::worker_pool_use_native_busy_keeps_executor_running_until_final_release",
        "native_pool::tests::worker_pool_use_acquire_and_shutdown_linearize_exactly_once",
        "wasm_pool::cooperative_tests::worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release",
      ] }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`worker-pool-use-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`worker-pool-use-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

//#region 🦀️Checks
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "--manifest-path", "Cargo.toml", ...segments], this.root);
  }
}
//#endregion 🦀️Checks

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-framework-async"], this.repoRoot, rest);
  }
}

//#region 🔖️Typegen
/** 🧬️ Name of the versioned owned-schema export test in `🦀️.rs`. */
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

/** 🎯️ The mirror lives at `<owner>/🤖️generated/⏳️async/🟦️.ts`, a sibling of `📦️packages`. */
function generatedBindingsPath(root: string): string {
  return join(root, "..", "..", "🤖️generated", "⏳️async", "🟦️.ts");
}

function runTypegenExportTest(root: string, outPath: string): void {
  const env = { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  const status = runCmdStatus("cargo", ["test", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: root, env, budgetMs: buildBudgetMs() });
  if (status !== 0) {
    console.error("framework-async typegen: `cargo test --features typegen` failed — see output above.");
    process.exit(status);
  }
}

class TypegenScript extends BundleScript {
  run(): void {
    const outPath = generatedBindingsPath(this.root);
    mkdirSync(dirname(outPath), { recursive: true });
    runTypegenExportTest(this.root, outPath);
    for (const name of readdirSync(dirname(outPath))) if (name !== basename(outPath)) rmSync(join(dirname(outPath), name), { recursive: true, force: true });
    console.log(`framework-async typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🧾️ Runs the exact exporter against isolated output/target directories and emits only canonical JSON. */
class PreviewGeneratedScript extends BundleScript {
  run(): void {
    const targetPath = generatedBindingsPath(this.root);
    const temp = mkdtempSync(join(tmpdir(), "semio-async-typegen-"));
    let content: Buffer;
    try {
      const outPath = join(temp, basename(targetPath));
      const result = Bun.spawnSync(["cargo", "test", "--locked", "--features", "typegen", TYPEGEN_TEST_FILTER], { cwd: this.root, env: { ...process.env, CARGO_TARGET_DIR: join(temp, "target"), SEMIO_TYPEGEN_OUT: outPath }, stderr: "pipe", stdout: "pipe" });
      if (result.exitCode !== 0) throw new Error(`framework-async preview export failed: ${result.stderr.toString()}`);
      content = readFileSync(outPath);
    } finally {
      rmSync(temp, { recursive: true, force: true });
    }
    const rootPath = relative(this.repoRoot, dirname(targetPath)).replaceAll("\\", "/").normalize("NFC");
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      { bytesBase64: content.toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/${basename(targetPath).normalize("NFC")}` },
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const staleRemovals = (existsSync(dirname(targetPath)) ? readdirSync(dirname(targetPath)) : []).filter((name) => name !== basename(targetPath)).map((name) => `${rootPath}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
    process.stdout.write(`${JSON.stringify({ contractId: "async-typegen", nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}
//#endregion 🔖️Typegen

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript).register("worker-maintenance-check", WorkerMaintenanceCheckScript).register("worker-deferred-wake-check", WorkerDeferredWakeCheckScript).register("worker-pool-use-check", WorkerPoolUseCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
