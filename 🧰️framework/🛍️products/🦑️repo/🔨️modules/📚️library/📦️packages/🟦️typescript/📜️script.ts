#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-lib` router: `bun ./📜️script.ts <lint|test [level]|workspaces <--write|--check>>`. */
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { spawn, spawnSync } from "node:child_process";
import { join, relative } from "node:path";
import { tmpdir } from "node:os";
import { BundleScript, ScriptRouter, computeWorkspaces, runBundleScriptMain, runBunx, resolveTestLevel, runTestBudgeted } from "./📦️index.ts";

class LintScript extends BundleScript {
  run(): void {
    runBunx(["tsc", "-p", "tsconfig.json", "--noEmit"], this.root);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments[0] === "transaction-v2") {
      const runId = `${process.pid}-${crypto.randomUUID()}`;
      const source = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️transaction-v2/🟦️.test.ts");
      const bundleRoot = join(this.repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️transaction-v2-bundle");
      const bundle = join(bundleRoot, "🟦️.test.js");
      const normalizationSource = join(this.repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts");
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
        "process-tree-killed|language-neutral|incomplete plans|rolls back after-regenerations|rejects stale generator",
        "rolls back after-(?:staging|embedded-root-staging|moves|relocations|symlink-retargeting|edits)|rolls back before-verify|parent-killed transaction-(?:attempt|initial)",
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
      const spawnFilter = (filter: string): ReturnType<typeof spawn> => {
        const concurrency = filter.includes("process-tree-killed") ? 5 : 6;
        const startedAt = performance.now(), child = spawn(process.execPath, ["test", `--max-concurrency=${concurrency}`, bundle, "-t", filter], { cwd: this.repoRoot, detached: process.platform !== "win32", env: { ...process.env, NX_DAEMON: "false", SEMIO_TRANSACTION_V2_BOUNDARY_REGISTRY: boundaryRegistry, SEMIO_TRANSACTION_V2_MODULE: normalizationBundle, SEMIO_TRANSACTION_V2_SCHEMA: schemaSnapshot, SEMIO_TRANSACTION_V2_PID_REGISTRY: registry, SEMIO_TRANSACTION_V2_RUN_ID: runId }, stdio: "inherit" });
        const ordinal = children.push(child);
        childOutcomes.set(child, new Promise<void>((resolveExit) => {
          child.once("error", (error) => { stop(error); resolveExit(); });
          child.once("exit", (code, signal) => { console.error(`[DEBUG] Transaction v2 shard ${ordinal} finished in ${((performance.now() - startedAt) / 1_000).toFixed(2)}s`); if (code !== 0 || signal) stop(new Error(`Transaction v2 shard ${ordinal} failed with ${signal ?? code}`)); resolveExit(); });
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
      for (const pid of registeredPids()) {
        try { process.kill(pid, 0); killTree(pid); failure ??= new Error(`Transaction v2 aggregate left child ${pid} alive`); }
        catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error; }
      }
      if (failure) throw failure;
      if (segments.length === 1) {
        const golden = JSON.parse(readFileSync(join(this.root, "🧫️fixtures/🧪️transaction-dispositions/🔣️.json"), "utf8")) as { boundaries: Record<string, unknown> };
        const expected = Object.keys(golden.boundaries).sort(), actual = readFileSync(boundaryRegistry, "utf8").split("\n").filter(Boolean).sort();
        if (JSON.stringify(actual) !== JSON.stringify(expected)) throw new Error(`Transaction v2 boundary coverage is not exact: ${actual.length}/${expected.length}`);
      }
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
