#!/usr/bin/env bun
/** 🦀️ `@semio-tech/framework-async` task router: `bun ./📜️script.ts <test|typegen>`. */
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative } from "node:path";
import assert from "node:assert/strict";
import Ajv2020 from "ajv/dist/2020.js";
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmdStatus, resolveTestLevel, runExactCargoLaws } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🔔️ Neutral lifecycle and actual native/cooperative idle wake acceptance. */
class WorkerMaintenanceCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length && segments[0] !== "--native")) throw new Error("worker-maintenance-check accepts only --native");
    const owner = join(this.root, "../../🔔️maintenance");
    const fixture = JSON.parse(readFileSync(join(owner, "🧪️fixtures/🔣️.json"), "utf8"));
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(owner, "🧪️fixtures/🧬️.schema.json"), "utf8")));
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
    console.log(`worker-maintenance-independent-oracle: AJV=1 lifecycle=${fixture.lifecycle.length} capacity=${fixture.capacity} native=1 cooperative=1`);
    assert(existsSync(join(owner, "🦀️.rs")), "missing fixed maintenance-hook implementation");
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    for (const marker of ["struct WorkerMaintenanceTicket", "struct WorkerMaintenanceRegistry", "enum PoolWork", "closed: bool", "entry.running", "entry.closing", "checked_add(1)", "fn shutdown(", "fn finish("]) assert(source.includes(marker), `missing maintenance owner primitive: ${marker}`);
    const pool = readFileSync(join(owner, "../🦀️.rs"), "utf8");
    for (const api of ["install_maintenance_hook", "request_maintenance", "remove_maintenance_hook"]) assert.equal(pool.match(new RegExp(`pub fn ${api}\\(`, "g"))?.length, 2, `native/cooperative API mismatch: ${api}`);
    assert(pool.includes("job.run(&inner.maintenance)") && pool.includes("job.run(&self.inner.maintenance)"), "both pool schedulers must run fixed work under their existing permits");
    if (segments[0] !== "--native") return;
    const receipts = await runExactCargoLaws({ cwd: this.repoRoot, env: { ...process.env, RUST_MIN_STACK: "268435456" }, groups: [{ package: "semio-framework-async", target: { kind: "lib", name: "semio_framework_async" }, laws: ["worker_maintenance_matches_neutral_retention_and_aba_lifecycle", "worker_maintenance_capacity_and_pool_identity_are_exact", "worker_maintenance_native_idle_wake_uses_no_queued_job", "worker_maintenance_cooperative_wake_obeys_pump_and_drr", "worker_maintenance_native_running_close_and_shutdown_keep_exact_invocation", "worker_maintenance_native_interleaves_io_jobs_and_rotating_hooks", "worker_maintenance_cooperative_interleaves_io_jobs_and_rotating_hooks", "native_drr_finishes_eligible_deficit_frontier_before_idle", "cooperative_maintenance_retains_deficit_until_later_host_turn", "cooperative_maintenance_snapshot_contention_preserves_queued_job", "cooperative_maintenance_live_host_revisits_queued_owner"] }], artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, buildBudgetMs: Number(process.env.SEMIO_BUILD_BUDGET_MS ?? 3_600_000), listBudgetMs: 60_000, lawBudgetMs: 120_000, progress(event) { console.log(`worker-maintenance-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`); } });
    for (const receipt of receipts) console.log(`worker-maintenance-native-receipt: ${JSON.stringify(receipt)}`);
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

/** 🎯️ The mirror lives at `<owner>/🤖️generated/🟦️async.ts`, a sibling of `📦️packages`. */
function generatedBindingsPath(root: string): string {
  return join(root, "..", "..", "🤖️generated", "🟦️async.ts");
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

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript).register("worker-maintenance-check", WorkerMaintenanceCheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
