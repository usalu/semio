#!/usr/bin/env bun
import { wasmBuildArguments } from "../../../../../🔨️modules/🏃️process/📦️artifacts/🕸️wasm-build/🟦️.ts";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { EventEmitter } from "node:events";
import { chmodSync, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { AuditScript, PolicyScript } from "./📇️inventory/📋️orchestration/🟦️.ts";
import { GraphScript } from "./🕸️graph/✅️verification/🟦️.ts";
import { DiskScript } from "./💾️storage/📊️report/🟦️.ts";
import { DoctorScript } from "./🩺️environment/📋️inspection/🟦️.ts";
import { CacheVerifyScript } from "./🔁️verification/📋️orchestration/🟦️.ts";
import { CachePruneScript, CacheReportScript } from "./🧹️pruning/📋️orchestration/🟦️.ts";
import { CargoProvenanceScript } from "./🦀️cargo/🧾️provenance/🟦️.ts";
import { devToolingEnv, getWorkspaceRoot, orchestratorBudgetOpts, runCmd, wasmBindgenVersion, wasmBuildEnvironment } from "../📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import plugin, { cacheInternals } from "../🟨️.mjs";
import { stageRepositoryArtifacts } from "./📦️artifacts/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { repoCacheDirectory } from "./🟦️.ts";

const SCRIPT_ROOT = dirname(fileURLToPath(import.meta.url));
export async function testCacheContracts(): Promise<void> {
  const { createCachePolicyTests } = await import("./🧪️tests/⚡️cache-contracts/🟦️.ts");
  const tests = createCachePolicyTests(
  {
    assert,
    cacheInternals,
    chmodSync,
    copyFileSync,
    createRequire,
    devToolingEnv,
    dirname,
    EventEmitter,
    existsSync,
    getWorkspaceRoot,
    join,
    lstatSync,
    mkdirSync,
    mkdtempSync,
    plugin,
    readFileSync,
    relative,
    resolve,
    rmSync,
    SCRIPT_ROOT,
    spawn,
    stageRepositoryArtifacts,
    utimesSync,
    wasmBindgenVersion,
    wasmBuildArguments,
    wasmBuildEnvironment,
    writeFileSync,
  },
  { directory: import.meta.dir, url: import.meta.url },
);
  await tests.testCacheContracts();
}

/** 🧪️ Routes the full cache suite, the focused portable command-source contract or the build-dir provenance laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] === "cache-policy") {
      if (args.length !== 1) throw new Error("Expected test cache-policy");
      const { proveCachePolicy } = await import("./🧪️tests/⚡️cache-contracts/🟦️.ts");
      proveCachePolicy(this.repoRoot, cacheInternals);
      return;
    }
    if (args[0] === "cargo-build-lease") {
      if (args.length !== 1) throw Error("Expected test cargo-build-lease");
      const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
      if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
      await (await import("../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts")).proveCargoBuildLeasesV1(output);
      return;
    }
    if (args[0] === "graph-revision") {
      if (args.length !== 1) throw new Error("Expected test graph-revision");
      const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
      if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
      mkdirSync(output, { recursive: true });
      await (await import("./🧪️tests/🔁️graph-revision/🟦️.ts")).testGraphRevision(this.repoRoot, output);
      return;
    }
    if (args[0] === "native-dependencies") {
      if (args.length !== 1) throw new Error("Expected test native-dependencies");
      const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
      if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
      mkdirSync(output, { recursive: true });
      await (await import("./🧪️tests/📦️native-dependencies/🟦️.ts")).testNativeDependencies(this.repoRoot, output);
      return;
    }
    if (args[0] === "artifact-source") {
      if (args.length !== 1) throw new Error("Expected test artifact-source");
      runCmd(process.execPath, ["test", join(SCRIPT_ROOT, "🧪️tests", "🗿️artifact-source", "🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
      return;
    }
    if (args[0] === "native-input-vocabulary") {
      if (args.length !== 1) throw Error("Expected test native-input-vocabulary");
      runCmd(process.execPath, ["test", join(SCRIPT_ROOT, "../🧪️tests/⚡️production-cache-input-boundary/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 15000 });
      return;
    }
    if (args[0] === "cache-command-source") {
      if (args.length !== 1) throw new Error("Expected test cache-command-source");
      runCmd(process.execPath, ["test", join(SCRIPT_ROOT, "🧪️tests", "🧱️command-source", "🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
      return;
    }
    if (args[0] === "cargo-provenance") {
      if (args.length !== 1) throw new Error("Expected test cargo-provenance");
      runCmd(process.execPath, ["test", join(SCRIPT_ROOT, "🧪️tests", "🧾️cargo-provenance", "🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
      return;
    }
    await testCacheContracts();
  }
}

/** 🚦️ `lease <exclusive|shared> <resource> <owner> -- <command…>`: runs one command while holding a queued lease on a
 * repository resource (arrival order, crashed waiters swept, released on every exit), so a shell caller serializes on
 * exactly the lease the product's own chains take — `wasm-build` for every all-plugin wasm build. */
class LeaseScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [mode, resource, owner, separator, command, ...rest] = args;
    if ((mode !== "exclusive" && mode !== "shared") || !resource || !owner || separator !== "--" || !command) throw new Error("usage: lease <exclusive|shared> <resource> <owner> -- <command…>");
    const controller = new AbortController(), abort = () => controller.abort();
    process.once("SIGINT", abort);
    process.once("SIGTERM", abort);
    const lease = await acquireQueuedResourceLease({ directory: repoCacheDirectory(this.repoRoot, "agents", "resource-leases"), resource, mode, owner, signal: controller.signal });
    try {
      const child = spawn(command, rest, { cwd: process.cwd(), stdio: "inherit" });
      process.on("SIGINT", () => child.kill("SIGINT"));
      process.on("SIGTERM", () => child.kill("SIGTERM"));
      process.exitCode = await new Promise<number>((accept) => child.once("exit", (code, signal) => accept(code ?? (signal ? 1 : 0))));
    } finally {
      lease.release();
    }
  }
}

/** 🧪️ Loads the artifact package proof only for its explicit verification command. */
class ArtifactPackageVerificationScript extends BundleScript {
  async run(): Promise<void> {
    const { ArtifactPackageContractScript } = await import("./🧪️tests/📦️artifact-packages/🟦️.ts");
    await new ArtifactPackageContractScript(this.root, this.repoRoot).run();
  }
}

const router = new ScriptRouter(SCRIPT_ROOT)
  .register("test", TestScript)
  .register("lease", LeaseScript)
  .register("audit", AuditScript)
  .register("policy-check", PolicyScript)
  .register("artifact-check", PolicyScript)
  .register("artifact-package-contract", ArtifactPackageVerificationScript)
  .register("graph-check", GraphScript)
  .register("doctor", DoctorScript)
  .register("disk-report", DiskScript)
  .register("cache-verify", CacheVerifyScript)
  .register("cache-report", CacheReportScript)
  .register("cache-prune", CachePruneScript)
  .register("cargo-provenance", CargoProvenanceScript);

if (import.meta.main) await runScriptMain(router);
