#!/usr/bin/env bun
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { EventEmitter } from "node:events";
import { chmodSync, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createCachePolicyTests } from "./🧪️tests/⚡️cache-contracts/🟦️.ts";
import { ArtifactPackageContractScript } from "./📦️artifacts/📋️package-orchestration/🟦️.ts";
import { AuditScript, PolicyScript } from "./📇️inventory/📋️orchestration/🟦️.ts";
import { GraphScript } from "./🕸️graph/✅️verification/🟦️.ts";
import { DiskScript } from "./💾️storage/📊️report/🟦️.ts";
import { DoctorScript } from "./🩺️environment/📋️inspection/🟦️.ts";
import { CacheVerifyScript } from "./🔁️verification/📋️orchestration/🟦️.ts";
import { CachePruneScript, CacheReportScript } from "./🧹️pruning/📋️orchestration/🟦️.ts";
import {
  BundleScript,
  ScriptRouter,
  devToolingEnv,
  getWorkspaceRoot,
  orchestratorBudgetOpts,
  runBundleScriptMain,
  runCmd,
  wasmBindgenVersion,
  wasmBuildArguments,
  wasmBuildEnvironment,
} from "../📦️packages/🟦️typescript/🟦️.ts";
import plugin, { cacheInternals } from "../🟨️.mjs";
import { stageArtifacts } from "./📦️artifacts/🟦️.ts";

const SCRIPT_ROOT = dirname(fileURLToPath(import.meta.url));
const createCachePolicyTestsInstance = createCachePolicyTests(
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
    stageArtifacts,
    utimesSync,
    wasmBindgenVersion,
    wasmBuildArguments,
    wasmBuildEnvironment,
    writeFileSync,
  },
  { directory: import.meta.dir, url: import.meta.url },
);
export const testCacheContracts = createCachePolicyTestsInstance.testCacheContracts;

/** 🧪️ Routes the full cache suite or the focused portable command-source contract. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args[0] === "cache-command-source") {
      if (args.length !== 1) throw new Error("Expected test cache-command-source");
      runCmd(process.execPath, ["test", join(SCRIPT_ROOT, "🧪️tests", "🧱️command-source", "🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
      return;
    }
    await testCacheContracts();
  }
}

const router = new ScriptRouter(SCRIPT_ROOT)
  .register("test", TestScript)
  .register("audit", AuditScript)
  .register("policy-check", PolicyScript)
  .register("artifact-check", PolicyScript)
  .register("artifact-package-contract", ArtifactPackageContractScript)
  .register("graph-check", GraphScript)
  .register("doctor", DoctorScript)
  .register("disk-report", DiskScript)
  .register("cache-verify", CacheVerifyScript)
  .register("cache-report", CacheReportScript)
  .register("cache-prune", CachePruneScript);

if (import.meta.main) await runBundleScriptMain(router, import.meta.url);
