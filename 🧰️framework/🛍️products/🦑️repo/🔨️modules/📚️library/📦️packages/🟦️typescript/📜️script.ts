#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-lib` router: `bun ./📜️script.ts <lint|test [level]|workspaces <--write|--check>>`. */
import { appendFileSync, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { spawn, spawnSync } from "node:child_process";
import { dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";
import { tmpdir } from "node:os";
import { BundleScript, ScriptRouter, computeWorkspaces, runBundleScriptMain, runBunx, resolveTestLevel, runTestBudgeted } from "./📦️index.ts";

/** 🧫️ Allocates one exclusive no-follow semantic run owner and its bundle directory. */
export function transactionV2BundleRoot(repoRoot: string, runId: string): string {
  const identity = /^[1-9][0-9]*-([0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12})$/u.exec(runId);
  if (!identity || !isAbsolute(repoRoot) || resolve(repoRoot) !== repoRoot) throw new Error("Invalid transaction run allocation identity");
  const ticket = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION");
  const report = join(ticket, "📓️transaction-v2-current-readiness"), owner = join(report, "🧾️runs");
  let path = parse(owner).root;
  for (const part of relative(path, owner).split(sep)) {
    path = join(path, part);
    let stat;
    try { stat = lstatSync(path); }
    catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT" || path !== report && path !== owner) throw error;
      mkdirSync(path);
      stat = lstatSync(path);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink()) throw new Error("Transaction run ancestor must be a no-follow directory: " + path);
  }
  const root = join(owner, "🔖️" + identity[1]);
  mkdirSync(root);
  const bundle = join(root, "📦️bundle");
  mkdirSync(bundle);
  return bundle;
}

class LintScript extends BundleScript {
  run(): void {
    runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "artifact-support") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️artifact-support-leaf-authority/🟦️.test.ts");
      const { rest } = resolveTestLevel(segments.slice(1));
      await runTestBudgeted(process.execPath, ["test", source, ...rest], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "historical-package-owner-identity") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️historical-package-owner-identity/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-provider-binding-trace") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️cargo-provider-binding/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "metadata-source-provider") {
      const { rest } = resolveTestLevel(segments.slice(1));
      await runTestBudgeted(process.execPath, ["test", "./🧪️index.test.ts", "-t", "mutation metadata source provider", ...rest], { cwd: this.root });
      return;
    }
    if (segments[0] === "rust-physical-reference-context") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️rust-physical-reference-context/🟦️.test.ts");
      const { rest } = resolveTestLevel(segments.slice(1));
      await runTestBudgeted(process.execPath, ["test", source, ...rest], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "taxonomy-cli-cancellation") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️taxonomy-cli-cancellation/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "inventory-artifact-shards") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️inventory-artifact-shards/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "root-script-compiler") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️root-script-compiler/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "json-reference-owner-lookup") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️json-reference-owner-lookup/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "cargo-discovery-exclusions") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️cargo-discovery-exclusions/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "nested-cargo-collision-authority") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️nested-cargo-collision-authority/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "registry-import-language") {
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️registry-import-language/🟦️.test.ts");
      await runTestBudgeted(process.execPath, ["test", source, ...segments.slice(1)], { cwd: this.repoRoot });
      return;
    }
    if (segments[0] === "transaction-v2") {
      const invocationStartedAt = performance.now(), startedAt = new Date().toISOString();
      const runId = `${process.pid}-${crypto.randomUUID()}`;
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️transaction-v2/🟦️.test.ts");
      const bundleRoot = transactionV2BundleRoot(this.repoRoot, runId), runRoot = dirname(bundleRoot);
      console.error(`[DEBUG] Transaction v2 run owner ${runRoot}`);
      const bundle = join(bundleRoot, "🟦️.test.js");
      const normalizationSource = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
      const identityPaths = { router: join(this.root, "📜️script.ts"), test: source, normalization: normalizationSource, discovery: join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts"), schema: join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), golden: join(this.root, "🧫️fixtures/🧪️transaction-dispositions/🔣️.json"), harness: join(this.root, "🧫️fixtures/🧪️transaction-harness-retention/🔣️.json") };
      const identities = () => Object.fromEntries(Object.entries(identityPaths).map(([key, path]) => { const bytes = readFileSync(path); return [key, { path, bytes: bytes.length, sha256: new Bun.CryptoHasher("sha256").update(bytes).digest("hex") }]; }));
      const retainRecord = (kind: string, value: unknown): void => { const root = join(runRoot, kind); mkdirSync(root); writeFileSync(join(root, "🔣️.json"), `${JSON.stringify(value, null, 2)}\n`, { flag: "wx" }); };
      const beforeIdentities = identities();
      retainRecord("📷️before", { schemaVersion: 1, runId, runRoot, startedAt, inputs: beforeIdentities });
      const normalizationBundleRoot = join(bundleRoot, "🧹️normalization"), normalizationBundle = join(normalizationBundleRoot, "🟦️.js");
      const schemaSnapshot = join(bundleRoot, "🔣️.json");
      mkdirSync(bundleRoot, { recursive: true });
      copyFileSync(join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), schemaSnapshot);
      const built = await Bun.build({ entrypoints: [source], outdir: bundleRoot, naming: "🟦️.test.js", target: "bun", packages: "external" });
      if (!built.success) throw new AggregateError(built.logs, "Transaction v2 aggregate bundle failed");
      mkdirSync(normalizationBundleRoot, { recursive: true });
      const normalizationBuilt = await Bun.build({ entrypoints: [normalizationSource], outdir: normalizationBundleRoot, naming: "🟦️.js", target: "bun", packages: "external" });
      if (!normalizationBuilt.success) throw new AggregateError(normalizationBuilt.logs, "Transaction v2 normalization child bundle failed");
      const defaultFilterWaves = [[
        "process-tree-killed|language-neutral|incomplete plans|rolls back after-regenerations|rejects stale generator|parent-killed transaction-attempt-canonical-published$",
        "rolls back after-(?:staging|embedded-root-staging|moves|relocations|symlink-retargeting|edits)|rolls back before-verify|parent-killed transaction-(?:attempt-preparation-(?:mkdir|children)|initial)",
        "parent-killed transaction-(?:journal|wal|backup|edit)|rejects forged|rejects unreachable",
        "parent-killed transaction-(?:restore|lease)|keeps double-plan|recovers caught|committed and rolled-back|elects exactly|restores a quarantined|rejects stale (?!generator)|rejects ordinal",
      ]];
      const defaultFilters = defaultFilterWaves.flat(), filterWaves = segments.length > 1 ? [[segments.slice(1).join(" ")]] : defaultFilterWaves;
      const staticTitles = [...readFileSync(source, "utf8").matchAll(/\btest(?:\.concurrent)?\("([^"]+)"/gu)].map((match) => match[1]);
      if (staticTitles.length !== 14 || staticTitles.length + 48 !== 62) throw new Error(`Transaction v2 aggregate manifest count changed: ${staticTitles.length + 48}`);
      for (const title of staticTitles) {
        const selections = defaultFilters.filter((filter) => new RegExp(filter, "u").test(title));
        if (selections.length !== 1) throw new Error(`Transaction v2 static case must be selected exactly once (${selections.length}): ${title}`);
      }
      const registry = join(bundleRoot, `pids-${runId}.txt`), boundaryRegistry = join(bundleRoot, `boundaries-${runId}.txt`);
      writeFileSync(registry, "");
      writeFileSync(boundaryRegistry, "");
      const children: ReturnType<typeof spawn>[] = [];
      const childOutcomes = new Map<ReturnType<typeof spawn>, Promise<void>>();
      const closedStreams: Promise<void>[] = [], shardOutcomes: { ordinal: number; filter: string; concurrency: number; milliseconds: number; code: number | null; signal: NodeJS.Signals | null }[] = [];
      const killTree = (pid: number): void => {
        if (process.platform === "win32") { spawnSync("taskkill", ["/pid", String(pid), "/t", "/f"], { stdio: "ignore" }); return; }
        try { process.kill(-pid, "SIGKILL"); }
        catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") try { process.kill(pid, "SIGKILL"); } catch {} }
      };
      const registeredPids = (): number[] => [...new Set(readFileSync(registry, "utf8").split(/\s+/u).filter(Boolean).map(Number).filter((pid) => Number.isSafeInteger(pid) && pid > 0))];
      let failure: Error | undefined, stopped = false;
      const stop = (error: Error): void => {
        if (stopped) return;
        stopped = true;
        failure = error;
        for (const child of children) if (child.pid && child.exitCode === null && child.signalCode === null) killTree(child.pid);
        for (const pid of registeredPids()) killTree(pid);
      };
      process.stdout.on("error", stop);
      process.stderr.on("error", stop);
      const spawnFilter = (filter: string): ReturnType<typeof spawn> => {
        const concurrency = filter.includes("process-tree-killed") ? 5 : 6;
        const outputRoot = join(runRoot, "📓️shards", `🔢️${children.length + 1}`);
        const streams = ["stdout", "stderr"].map((kind) => { const root = join(outputRoot, kind); mkdirSync(root, { recursive: true }); const path = join(root, "🔤️.txt"); writeFileSync(path, "", { flag: "wx" }); return path; });
        const startedAt = performance.now(), child = spawn(process.execPath, ["test", `--max-concurrency=${concurrency}`, bundle, "-t", filter], { cwd: this.repoRoot, detached: process.platform !== "win32", env: { ...process.env, NX_DAEMON: "false", SEMIO_TRANSACTION_V2_BOUNDARY_REGISTRY: boundaryRegistry, SEMIO_TRANSACTION_V2_MODULE: normalizationBundle, SEMIO_TRANSACTION_V2_SCHEMA: schemaSnapshot, SEMIO_TRANSACTION_V2_PID_REGISTRY: registry, SEMIO_TRANSACTION_V2_RUN_ID: runId, SEMIO_TRANSACTION_V2_RUN_ROOT: runRoot }, stdio: ["ignore", "pipe", "pipe"] });
        child.stdout!.on("data", (bytes) => { try { appendFileSync(streams[0]!, bytes); process.stdout.write(bytes); } catch (error) { stop(error instanceof Error ? error : new Error(String(error))); } });
        child.stderr!.on("data", (bytes) => { try { appendFileSync(streams[1]!, bytes); process.stderr.write(bytes); } catch (error) { stop(error instanceof Error ? error : new Error(String(error))); } });
        closedStreams.push(new Promise((resolveClose) => child.once("close", () => resolveClose())));
        const ordinal = children.push(child);
        childOutcomes.set(child, new Promise<void>((resolveExit) => {
          child.once("error", (error) => { stop(error); resolveExit(); });
          child.once("exit", (code, signal) => { const milliseconds = performance.now() - startedAt; shardOutcomes.push({ ordinal, filter, concurrency, milliseconds, code, signal }); console.error(`[DEBUG] Transaction v2 shard ${ordinal} finished in ${(milliseconds / 1_000).toFixed(2)}s`); if (code !== 0 || signal) stop(new Error(`Transaction v2 shard ${ordinal} failed with ${signal ?? code}`)); resolveExit(); });
        }));
        return child;
      };
      const timer = setTimeout(() => stop(new Error("Transaction v2 complete aggregate exceeded 14 seconds")), 14_000);
      try {
        for (const wave of filterWaves) {
          const waveChildren: ReturnType<typeof spawn>[] = [];
          for (const filter of wave) {
            const child = spawnFilter(filter);
            waveChildren.push(child);
            if (stopped) break;
          }
          await Promise.all(waveChildren.map((child) => childOutcomes.get(child)!));
          if (stopped) break;
        }
      } finally { clearTimeout(timer); }
      await Promise.all(closedStreams);
      process.stdout.off("error", stop);
      process.stderr.off("error", stop);
      for (const pid of registeredPids()) {
        try { process.kill(pid, 0); killTree(pid); failure ??= new Error(`Transaction v2 aggregate left child ${pid} alive`); }
        catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error; }
      }
      const golden = JSON.parse(readFileSync(identityPaths.golden, "utf8")) as { boundaries: Record<string, unknown> };
      const expected = Object.keys(golden.boundaries).sort(), actual = readFileSync(boundaryRegistry, "utf8").split("\n").filter(Boolean).sort();
      if (!failure && segments.length === 1 && JSON.stringify(actual) !== JSON.stringify(expected)) failure = new Error(`Transaction v2 boundary coverage is not exact: ${actual.length}/${expected.length}`);
      const afterIdentities = identities();
      retainRecord("📷️after", { schemaVersion: 1, runId, inputs: afterIdentities });
      retainRecord("📊️outcome", { schemaVersion: 1, runId, runRoot, startedAt, finishedAt: new Date().toISOString(), milliseconds: performance.now() - invocationStartedAt, unfiltered: segments.length === 1, unchangedInputs: JSON.stringify(beforeIdentities) === JSON.stringify(afterIdentities), failure: failure?.message ?? null, shards: shardOutcomes.sort((left, right) => left.ordinal - right.ordinal), expectedBoundaryCount: expected.length, actualBoundaryCount: actual.length, missingBoundaries: expected.filter((key) => !actual.includes(key)), extraBoundaries: actual.filter((key) => !expected.includes(key)), duplicateBoundaries: actual.filter((key, index) => index > 0 && actual[index - 1] === key) });
      if (failure) throw failure;
      return;
    }
    const { rest } = resolveTestLevel(segments);
    await runTestBudgeted(process.execPath, ["test", "./🧪️index.test.ts", ...rest], { cwd: this.root });
  }
}

//#region 🔖️WorkspacesScript
/** 🗂️ `bun ./📜️script.ts workspaces --write` regenerates root `package.json`'s `workspaces` array from
 * `computeWorkspaces()`; `--check` verifies without writing (exits 1 when stale). Never touches any
 * other root `package.json` field — see `26/08/06/GENERATED-BUN-WORKSPACES-FROM-PACKAGE-CATALOG`. */
class WorkspacesScript extends BundleScript {
  run(segments: string[]): void {
    const write = segments.includes("--write");
    const check = segments.includes("--check");
    if (write === check) {
      console.error("usage: bun ./📜️script.ts workspaces <--write|--check>");
      process.exit(1);
    }
    const rootPkgPath = join(this.repoRoot, "package.json");
    const rootPkg = JSON.parse(readFileSync(rootPkgPath, "utf8")) as Record<string, unknown>;
    const current = Array.isArray(rootPkg.workspaces) ? (rootPkg.workspaces as string[]) : [];
    const expected = computeWorkspaces(this.repoRoot);
    const fresh = current.length === expected.length && current.every((entry, i) => entry === expected[i]);
    if (check) {
      if (!fresh) {
        const missing = expected.filter((entry) => !current.includes(entry));
        const stale = current.filter((entry) => !expected.includes(entry));
        console.error(`root package.json workspaces is stale (${expected.length} expected, ${current.length} current).`);
        if (missing.length > 0) console.error(`  missing: ${missing.join(", ")}`);
        if (stale.length > 0) console.error(`  stale:   ${stale.join(", ")}`);
        if (missing.length === 0 && stale.length === 0) console.error("  (same set, different order)");
        console.error("run `bun ./📜️script.ts workspaces --write` to refresh.");
        process.exit(1);
      }
      console.log(`root package.json workspaces is fresh (${expected.length} packages).`);
      return;
    }
    if (fresh) {
      console.log(`root package.json workspaces already fresh (${expected.length} packages) — no write needed.`);
      return;
    }
    rootPkg.workspaces = expected;
    writeFileSync(rootPkgPath, `${JSON.stringify(rootPkg, null, 2)}\n`);
    console.log(`root package.json workspaces regenerated -> ${expected.length} packages.`);
  }
}
//#endregion 🔖️WorkspacesScript

//#region 📌️TicketImportantFemHandoffGenerator
const TICKET_IMPORTANT_FEM_HANDOFF_CONTRACT = "ticket-important-fem-handoff";
const TICKET_IMPORTANT_FEM_OWNER = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION";
const TICKET_IMPORTANT_FEM_OUTPUT = `${TICKET_IMPORTANT_FEM_OWNER}/📋️registrar-handoff.json`;

function ticketImportantFemHandoffBytes(repoRoot: string): Buffer {
  const owner = join(repoRoot, TICKET_IMPORTANT_FEM_OWNER);
  const writeOwner = join(owner, "🔧️write-handoff.mjs");
  const updateOwner = join(owner, "🔧️update-handoff-tests.mjs");
  const sandbox = mkdtempSync(join(tmpdir(), "semio-ticket-important-fem-"));
  const runOwner = (path: string): void => {
    const result = spawnSync(process.execPath, [path, sandbox], { cwd: repoRoot, encoding: "utf8" });
    if (result.status !== 0 || result.signal || result.error) throw new Error(`FEM handoff owner failed: ${relative(repoRoot, path)} status=${result.status ?? -1}`);
  };
  try {
    mkdirSync(join(sandbox, "📓️important"), { recursive: true });
    runOwner(writeOwner);
    const updateSource = readFileSync(updateOwner, "utf8");
    const markdownInput = updateSource.match(/readFileSync\(join\(ticket, "([^"]*registrar-handoff\.md)"\)/u)?.[1];
    if (!markdownInput) throw new Error("FEM handoff update owner has no exact registrar Markdown input");
    if (markdownInput !== "📋️registrar-handoff.md") copyFileSync(join(sandbox, "📋️registrar-handoff.md"), join(sandbox, markdownInput));
    runOwner(updateOwner);
    const handoff = JSON.parse(readFileSync(join(sandbox, "📋️registrar-handoff.json"), "utf8")) as Record<string, unknown>;
    handoff.ticketPath = TICKET_IMPORTANT_FEM_OWNER;
    return Buffer.from(`${JSON.stringify(handoff, null, 2).replaceAll("📌️important.md", "📓️important/📝️.md")}\n`);
  } finally {
    rmSync(sandbox, { recursive: true, force: true });
  }
}

class TicketImportantFemHandoffGenerateScript extends BundleScript {
  run(): void {
    writeFileSync(join(this.repoRoot, TICKET_IMPORTANT_FEM_OUTPUT), ticketImportantFemHandoffBytes(this.repoRoot));
    console.log(`Generated ${TICKET_IMPORTANT_FEM_OUTPUT}.`);
  }
}

class TicketImportantFemHandoffPreviewScript extends BundleScript {
  run(): void {
    const bytes = ticketImportantFemHandoffBytes(this.repoRoot);
    process.stdout.write(`${JSON.stringify({ contractId: TICKET_IMPORTANT_FEM_HANDOFF_CONTRACT, nodes: [{ bytesBase64: bytes.toString("base64"), mode: 0o644, nodeKind: "file", path: TICKET_IMPORTANT_FEM_OUTPUT }], schemaVersion: 1, staleRemovals: [] })}\n`);
  }
}

class TicketImportantFemHandoffCheckScript extends BundleScript {
  run(): void {
    const path = join(this.repoRoot, TICKET_IMPORTANT_FEM_OUTPUT);
    if (!existsSync(path) || !readFileSync(path).equals(ticketImportantFemHandoffBytes(this.repoRoot))) throw new Error(`${TICKET_IMPORTANT_FEM_OUTPUT} is stale`);
    console.log(`${TICKET_IMPORTANT_FEM_OUTPUT} is fresh.`);
  }
}
//#endregion 📌️TicketImportantFemHandoffGenerator

const router = new ScriptRouter(import.meta.dir)
  .register("lint", LintScript)
  .register("test", TestScript)
  .register("workspaces", WorkspacesScript)
  .register("generate-ticket-important-fem-handoff", TicketImportantFemHandoffGenerateScript)
  .register("preview-generated", TicketImportantFemHandoffPreviewScript)
  .register("check-ticket-important-fem-handoff", TicketImportantFemHandoffCheckScript);

await runBundleScriptMain(router, import.meta.url);
