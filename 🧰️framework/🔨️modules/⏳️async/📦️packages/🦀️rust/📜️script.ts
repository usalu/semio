#!/usr/bin/env bun
import { runExactCargoLaws } from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { buildBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
/** 🦀️ `@semio-tech/framework-async` task router: `bun ./📜️script.ts <test|typegen>`. */
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative } from "node:path";
import assert from "node:assert/strict";

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

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
    const owner = join(this.root, "../../🔔️maintenance");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
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
    console.log(`[DEBUG] worker-maintenance-independent-oracle: lifecycle=${fixture.lifecycle.length} capacity=${fixture.capacity} native-source=1 cooperative-source=1`);
    assert(existsSync(join(owner, "🦀️.rs")), "missing fixed maintenance-hook implementation");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WorkerMaintenanceTicket", "struct WorkerMaintenanceRegistry", "enum PoolWork", "closed: bool", "entry.running", "entry.closing", "WorkerMaintenanceStep::Retire", "checked_add(1)", "fn shutdown(", "fn finish("]) assert(source.includes(marker), `missing maintenance owner primitive: ${marker}`);
    const pool = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    for (const api of ["install_maintenance_hook", "request_maintenance", "remove_maintenance_hook"]) assert.equal(pool.match(new RegExp(`pub fn ${api}\\(`, "g"))?.length, 2, `native/cooperative API mismatch: ${api}`);
    assert(pool.includes("job.run(&inner.maintenance)") && pool.includes("job.run(&self.inner.maintenance)"), "both pool schedulers must run fixed work under their existing permits");
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-async": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, ...exactCargoStageEnvironments(), groups: [{ package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: ["worker_maintenance_matches_neutral_retention_and_aba_lifecycle", "worker_maintenance_capacity_and_pool_identity_are_exact", "worker_maintenance_running_callback_retires_requested_generation_exactly_once", "worker_maintenance_native_idle_wake_uses_no_queued_job", "worker_maintenance_native_self_retire_reuses_all_fixed_slots", "worker_maintenance_cooperative_wake_obeys_pump_and_drr", "worker_maintenance_native_running_close_and_shutdown_keep_exact_invocation", "worker_maintenance_native_interleaves_io_jobs_and_rotating_hooks", "worker_maintenance_cooperative_interleaves_io_jobs_and_rotating_hooks", "native_drr_finishes_eligible_deficit_frontier_before_idle", "cooperative_maintenance_retains_deficit_until_later_host_turn", "cooperative_maintenance_snapshot_contention_preserves_queued_job"] }], artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listBudgetMs: 60_000, lawBudgetMs: 120_000, progress(event) { console.log(`worker-maintenance-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
    for (const receipt of receipts) console.log(`worker-maintenance-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 🛌️ Neutral worker-parking protocol and an independent JS model and, with --native, the Rust laws. */
class WorkerParkingCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("worker-parking-check accepts only --native");
    const owner = join(this.root, "../../🔔️worker-parking");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const model = (kase: string, timerDue: boolean): string => {
      let signal = 0, closed = false, keeper = false, sleepers = 0;
      const observed = () => signal;
      const park = (seen: number, due: boolean, wakeAfter?: () => void): string => {
        if (closed) return "closed";
        if (signal !== seen) return "signalled";
        if (due && !keeper) { keeper = true; if (!wakeAfter) return "timer-due"; wakeAfter(); keeper = false; return "woken"; }
        sleepers += 1; wakeAfter?.(); sleepers -= 1; return "woken";
      };
      const signalWork = () => { signal += 1; };
      if (kase === "signal-before-park") { const seen = observed(); signalWork(); return park(seen, timerDue); }
      if (kase === "close-before-park") { signal += 1; closed = true; return park(observed(), timerDue); }
      if (kase === "keeper-deadline-elapses") return park(observed(), timerDue);
      return park(observed(), timerDue, () => { assert(sleepers > 0 || keeper, `${kase}: nobody to notify`); });
    };
    for (const row of fixture.protocol) assert.equal(model(row.case, row.timerDue), row.expected, row.case);
    assert.equal(fixture.idle.maximumSleepsInQuietWindow, 0);
    assert.equal(fixture.periodicTimer.maximumSleepsPerTick, 1);
    assert(fixture.farKeeper.farDeadlineMs > fixture.farKeeper.chainBoundMs * 6, "a chain parked behind the far keeper must outlive the chain bound");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["fn signal_work(", "fn signal_timer(", "fn signal_timer_from_firing_worker(", "fn hand_off_timers(", "fn park(", "Ordering::SeqCst"]) assert(source.includes(marker), `missing parking primitive: ${marker}`);
    const crate = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    const pool = crate.slice(crate.indexOf("mod native_pool {"), crate.indexOf("//#endregion 🧵️WorkerPoolNative"));
    assert(pool.length > 0 && !pool.includes("wait_timeout(guard") && !pool.includes("notify_all"), "the native pool must not poll or broadcast");
    console.log(`[DEBUG] worker-parking-independent-oracle: protocol=${fixture.protocol.length} quiet-window=${fixture.idle.quietWindowMs}ms periodic-ticks=${fixture.periodicTimer.ticks} far-keeper-chains=${fixture.farKeeper.chains}`);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-async": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory,
      cwd: this.repoRoot,
      ...exactCargoStageEnvironments(),
      groups: [{
        package: "semio-framework-async",
        target: { kind: "lib", name: "semio_framework_async" },
        laws: [
          "worker_parking::tests::worker_parking_protocol_matches_the_neutral_fixture",
          "worker_parking::tests::an_ancestor_cancel_wakes_a_descendant_waiter_once_and_drop_retains_nothing",
          "native_pool::tests::an_idle_native_pool_sleeps_without_a_poll_interval",
          "native_pool::tests::a_timer_deadline_wakes_exactly_the_parked_keeper",
          "native_pool::tests::a_timer_re_armed_from_its_own_callback_wakes_only_the_keeper",
          "native_pool::tests::a_timer_re_armed_by_a_firing_callback_never_waits_behind_a_far_keeper",
        ],
      }],
      artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR,
      buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000),
      listBudgetMs: 60_000,
      lawBudgetMs: 120_000,
      progress(event) { console.log(`worker-parking-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); },
    });
    for (const receipt of receipts) console.log(`worker-parking-native-receipt: ${JSON.stringify(receipt)}`);
  }
}

/** 💤️ Proves the fixed neutral deferred-waker transfer, drain and terminal admission. */
class WorkerDeferredWakeCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("worker-deferred-wake-check accepts only --native");
    const owner = join(this.root, "../../🔔️deferred-wake");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const capacity = fixture.capacity;
    assert.equal(capacity.totalWaiters, capacity.partitions * capacity.slotsPerPartition);
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
    }
    const asyncSource = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    const missingAsync = fixture.runtimeMarkers.async.filter((marker: string) => !asyncSource.includes(marker));
    assert.deepEqual(missingAsync, [], `missing async runtime markers: ${missingAsync.join(", ")}`);
    const markerCount = fixture.runtimeMarkers.async.length;
    console.log(`[DEBUG] worker-deferred-wake-independent-oracle: cases=${fixture.cases.length} fixed-waiters=${capacity.totalWaiters} inline-wakes=0 runtime-markers=${markerCount}/${markerCount}`);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-async": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory,
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
    const owner = join(this.root, "../../🔐️use");
    const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
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
    const source = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    for (const marker of ["pub struct WorkerPoolUse", "pub enum WorkerPoolShutdownError", "pub enum WorkerPoolUseError", "pub fn acquire_use(&self)", "retained_uses", "PoolLifecycleState::Closing", "PoolLifecycleState::Stopped"]) assert(source.includes(marker), `missing pool-use marker ${marker}`);
    assert.equal(source.match(/pub fn shutdown\(&self\) -> Result<\(\), WorkerPoolShutdownError>/g)?.length, 2);
    for (const [path, laws] of [
      ["🧪️tests/🔬️native-pool-unit/🦀️.rs", ["worker_pool_use_native_busy_keeps_executor_running_until_final_release", "worker_pool_use_acquire_and_shutdown_linearize_exactly_once"]],
      ["🧪️tests/🔬️wasm-pool-cooperative/🦀️.rs", ["worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release"]],
    ] as const) {
      assert(source.includes(`#[cfg(test)]\n    include!("${path}")`), `missing actual pool-use test mount ${path}`);
      const tests = readFileSync(join(owner, "..", path), "utf8");
      for (const law of laws) assert(tests.includes(`fn ${law}(`), `missing exact pool-use law ${law}`);
    }
    console.log(`[DEBUG] worker-pool-use-independent-oracle: cases=${fixture.cases.length} native-source=1 cooperative-source=1`);
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-async": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory,
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
    await runOwnedCommand("cargo",["check","--manifest-path",resolve(this.root,"Cargo.toml"),...segments],this.root,"async:check",buildBudgetMs());
  }
}
//#endregion 🦀️Checks

/** 📚️ Compiles every external-owner law under the caller's explicit native policy. */
class PublicationDocsTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("test-publication-docs accepts no arguments");
    const policy = readCargoTestPolicyV1(process.env), manifest = resolve(this.root, "Cargo.toml");
    if (policy.manifestPath !== manifest) throw new Error("publication doctests require the actual Async manifest policy");
    mkdirSync(policy.artifactDirectory, { recursive: true });
    await runOwnedCommand("cargo", ["test", "--locked", "--doc", "--manifest-path", manifest, "-p", "semio-framework-async"], this.root, "async:publication-docs", policy.buildBudgetMs, {
      env: { ...process.env, CARGO_TARGET_DIR: policy.targetDirectory, RUST_MIN_STACK: policy.rustMinStack, TMPDIR: policy.artifactDirectory, TMP: policy.artifactDirectory, TEMP: policy.artifactDirectory },
    });
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath: resolve(this.root, "Cargo.toml"), packages: ["semio-framework-async"], cwd: this.root, extraArgs: rest }, readCargoTestPolicyV1(process.env));
  }
}

//#region 🔖️Typegen
/** 🧬️ Name of the versioned owned-schema export test in `🦀️.rs`. */
const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";

/** 🎯️ The mirror lives at `<owner>/🤖️generated/⏳️async/🟦️.ts`, a sibling of `📦️packages`. */
function generatedBindingsPath(root: string): string {
  return join(root, "..", "..", "🤖️generated", "⏳️async", "🟦️.ts");
}

async function runTypegenExportTest(root: string, outPath: string): Promise<void> {
  const env = { ...process.env, SEMIO_TYPEGEN_OUT: outPath };
  await runOwnedCommand("cargo",["test","--manifest-path",resolve(root,"Cargo.toml"),"--features","typegen",TYPEGEN_TEST_FILTER],root,"async:typegen",buildBudgetMs(),{env});
}

class TypegenScript extends BundleScript {
  async run(): Promise<void> {
    const outPath = generatedBindingsPath(this.root);
    mkdirSync(dirname(outPath), { recursive: true });
    await runTypegenExportTest(this.root, outPath);
    for (const name of readdirSync(dirname(outPath))) if (name !== basename(outPath)) rmSync(join(dirname(outPath), name), { recursive: true, force: true });
    console.log(`framework-async typescript mirror refreshed -> ${outPath}`);
  }
}

/** 🧾️ Runs the exact exporter against isolated output/target directories and emits only canonical JSON. */
class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
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

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-publication-docs", PublicationDocsTestScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript).register("worker-maintenance-check", WorkerMaintenanceCheckScript).register("worker-deferred-wake-check", WorkerDeferredWakeCheckScript).register("worker-parking-check", WorkerParkingCheckScript).register("worker-pool-use-check", WorkerPoolUseCheckScript);

await runScriptMain(router, { defaultCommand: "test" });
