#!/usr/bin/env bun
import {InactiveCargoRetirementScript} from "./🧹️pruning/🏪️budget/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { wasmBuildArguments } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
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
import { BundleScript, ScriptRouter, checkScriptInvocation, scriptInvocationBudget, scriptInvocationEnvironment, type ScriptInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { captureOwnedCommandV1,terminateOwnedChildTree } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import plugin, { cacheInternals } from "../🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import { stageRepositoryArtifacts } from "./📦️artifacts/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { repoCacheDirectory } from "./🟦️.ts";
import { runProcessTestOwner } from "./🧪️tests/🧭️ownership/🟦️.ts";

const SCRIPT_ROOT = dirname(fileURLToPath(import.meta.url));
export async function testCacheContracts(invocation: ScriptInvocation): Promise<void> {
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
  await tests.testCacheContracts(invocation);
}

/** 🧪️ Runs the native workspace laws inside the same original command cancellation and deadline. */
async function testWorkspaceOwnership(invocation: ScriptInvocation, root: string, artifacts: string, entrypoint = join(SCRIPT_ROOT, "../🗂️workspaces/🟦️bun/🧪️tests/🟦️.ts")): Promise<void> {
  checkScriptInvocation(invocation);
  const options = orchestratorBudgetOpts({ SEMIO_TEST_ARTIFACT_DIR: artifacts }), budget = scriptInvocationBudget(invocation, options.budgetMs!), child = spawn(process.execPath, ["test", entrypoint], { cwd: root, env: options.env, stdio: ["inherit","pipe","pipe"], detached: process.platform !== "win32" });
  let stopped: unknown, transcript = "";
  child.stdout!.on("data", chunk => {transcript += String(chunk);process.stdout.write(chunk);});child.stderr!.on("data", chunk => {transcript += String(chunk);process.stderr.write(chunk);});
  const stop = (reason: unknown): void => { stopped ??= reason; terminateOwnedChildTree(child); }, abort = (): void => stop(invocation.control.signal.reason);
  const closed = new Promise<number>((accept, reject) => { child.once("error", reject); child.once("close", (code, signal) => signal ? reject(Error("Original workspace tests stopped: " + signal)) : accept(code ?? 1)); });
  invocation.control.signal.addEventListener("abort", abort, { once: true });
  const timeout = budget === 0 ? undefined : setTimeout(() => stop(Error("Original workspace test deadline exhausted")), budget);
  try {
    if (invocation.control.signal.aborted) abort();
    const status = await closed;
    mkdirSync(artifacts,{recursive:true});writeFileSync(join(artifacts,"bun-test-"+createHash("sha256").update(entrypoint).digest("hex").slice(0,16)+".log"),transcript);
    if (stopped) throw stopped;
    checkScriptInvocation(invocation);
    if (status !== 0) throw Error("Original workspace tests exited with status " + status);
    if(!/\b[1-9][0-9]* pass\b/u.test(transcript))throw Error("Original Bun test owner did not report completed tests");
  } finally {
    if (timeout) clearTimeout(timeout);
    invocation.control.signal.removeEventListener("abort", abort);
    terminateOwnedChildTree(child);
  }
}

/** 🧪️ Routes the full cache suite, the focused portable command-source contract or the build-dir provenance laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if(args[0]==="pruning"){
      if(args.length!==1)throw Error("Expected test pruning");
      const owner=join(SCRIPT_ROOT,"🧹️pruning"),control=scriptCompilerAdmissionControlV1(this.invocation,"cache:pruning:typescript",120000),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
      if(!artifacts)throw Error("Original ticket artifact directory required");
      await testWorkspaceOwnership(this.invocation,this.repoRoot,artifacts,join(owner,"⏱️continuation/🧪️tests/🟦️.ts"));
      await testWorkspaceOwnership(this.invocation,this.repoRoot,artifacts,join(owner,"🔒️store/🧪️tests/🟦️.ts"));
      await testWorkspaceOwnership(this.invocation,this.repoRoot,artifacts,join(owner,"🔒️incremental/🧪️tests/🟦️.ts"));
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(owner,"🟦️.ts")],this.repoRoot,{control,env:process.env});
      console.log("[DEBUG] Shared cache pruning core strict TypeScript check passed");return;
    }
    if(args[0]==="sdk-snapshot"){
      if(args.length!==1)throw Error("Expected test sdk-snapshot");
      const owner=join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot"),control=scriptCompilerAdmissionControlV1(this.invocation,"nx:sdk-snapshot:typescript",120000);
      await testWorkspaceOwnership(this.invocation,this.repoRoot,process.env.SEMIO_TEST_ARTIFACT_DIR??repoCacheDirectory(this.repoRoot,"sdk-snapshot-probes"),join(owner,"🧪️tests/🟦️.ts"));
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(owner,"🟦️.ts")],this.repoRoot,{control,env:process.env});
      console.log("[DEBUG] SDK snapshot strict TypeScript check passed");return;
    }
    if(args[0]==="journal-source"){
      if(args.length!==1)throw Error("Expected test journal-source");
      const owner=join(SCRIPT_ROOT,"🕸️graph/🔏️seal/📥️membership/📥️journal"),control=scriptCompilerAdmissionControlV1(this.invocation,"nx:journal-source:typescript",120000);
      await testWorkspaceOwnership(this.invocation,this.repoRoot,process.env.SEMIO_TEST_ARTIFACT_DIR??repoCacheDirectory(this.repoRoot,"journal-source-probes"),join(owner,"🧪️tests/🟦️.ts"));
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(owner,"🟦️.ts")],this.repoRoot,{control,env:process.env});
      console.log("[DEBUG] repository journal source strict TypeScript check passed");return;
    }
    if(args[0]==="authorizers"){
      if(args.length!==1)throw Error("Expected test authorizers");
      await testWorkspaceOwnership(this.invocation,this.repoRoot,process.env.SEMIO_TEST_ARTIFACT_DIR??repoCacheDirectory(this.repoRoot,"authorizer-probes"),join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts"));
      const source=join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools"),control=scriptCompilerAdmissionControlV1(this.invocation,"nx:authorizers:typescript",120000);
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(source,"🔏️authorizers/🟦️.ts"),join(source,"🔏️authorizers/📜️script.ts"),join(source,"🔏️authorizers/📤️receipt/🟦️.ts"),join(source,"📜️script.ts")],this.repoRoot,{control,env:process.env});
      console.log("[DEBUG] immutable authorizer strict TypeScript check passed");return;
    }
    if (args[0] === "daemon-journal") {
      if (args.length !== 1 && (args.length !== 2 || !["transport", "workspace"].includes(args[1]!))) throw Error("Expected test daemon-journal [transport|workspace]");
      const output = process.env.SEMIO_TEST_ARTIFACT_DIR ?? repoCacheDirectory(this.repoRoot, "daemon-journal-probes");
      await (await import("./🧪️tests/🕸️daemon/📥️journal/🟦️.ts")).proveDaemonJournal(this.repoRoot, output, this.invocation, args[1] === "transport", args[1] === "workspace");
      return;
    }
    if(args[0]==="graph-cache-waiter"){if(args.length!==1)throw Error("Expected test graph-cache-waiter");await testWorkspaceOwnership(this.invocation,this.repoRoot,process.env.SEMIO_TEST_ARTIFACT_DIR??repoCacheDirectory(this.repoRoot,"graph-cache-waiter-probes"),join(SCRIPT_ROOT,"🕸️graph/🏇️waiter/🧪️tests/🟦️.ts"));return;}
    if(args[0]==="binaryen-partition"){if(args.length!==1)throw Error("Expected test binaryen-partition");const {withRepositoryToolingStoreV1}=await import("./💾️storage/🔒️enforcement/🛠️tools/🟦️.ts"),{prepareBinaryen}=await import("./🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts");await withRepositoryToolingStoreV1(this.repoRoot,this.invocation,async store=>{const executable=await prepareBinaryen(this.repoRoot,store);const version=await captureOwnedCommandV1(executable,["--version"],this.repoRoot,{control:store.control,env:scriptInvocationEnvironment(this.invocation,store.environment)});if(version.trim()!=="wasm-opt version 130 (version_130)")throw Error("Original bounded Binaryen version refused");console.log("[DEBUG] Actual bounded Binaryen runtime "+JSON.stringify({executable,version:version.trim(),root:store.root}));});return;}
    if(args[0]==="bazel-storage-runtime"){if(args.length!==1)throw Error("Expected test bazel-storage-runtime");await (await import("../🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/📥️runtime/🟦️.ts")).proveBazelStorageRuntimeV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="runtime-verification-storage-runtime"){if(args.length!==1)throw Error("Expected one verifier storage proof");await(await import("../🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/📥️runtime/🟦️.ts")).proveRuntimeVerificationStorageV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="javascript-partition"){if(args.length!==1)throw Error("Expected test javascript-partition");await (await import("./🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts")).proveJavascriptDependencyPartitionV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="lease-queue"){if(args.length!==1)throw Error("Expected test lease-queue");await (await import("../../../../../🔨️modules/🏃️process/🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts")).testResourceLeases(process.env.SEMIO_TEST_ARTIFACT_DIR!,scriptCompilerAdmissionControlV1(this.invocation,"test:lease-queue",0));return;}
    if(args[0]==="leases-partition"){if(args.length!==1)throw Error("Expected test leases-partition");const proof=await import("./💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🟦️.ts"),control=scriptCompilerAdmissionControlV1(this.invocation,"cache:leases:physical-proof",120000);await proof.proveRepositoryLeasePartitionV1(this.repoRoot,control);await proof.proveRepositoryQueueMetadataHeadroomV1(this.repoRoot,this.invocation,control);return;}
    if(args[0]==="rustup-toolchain"){if(args.length!==1)throw Error("Expected test rustup-toolchain");const proof=await import("./🚀️bootstrap/🛠️tools/🦀️rustup/🧪️tests/🏗️installation/🟦️.ts");await proof.proveRepositoryRustToolchainV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="rustup-generation"){if(args.length!==1)throw Error("Expected test rustup-generation");const proof=await import("./🚀️bootstrap/🛠️tools/🦀️rustup/🧪️tests/📥️runtime/🟦️.ts");await proof.proveRustupGenerationCaptureV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="tools-partition"){if(args.length!==1)throw Error("Expected test tools-partition");const proof=await import("./💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🟦️.ts");await proof.proveRepositoryToolsPartitionV1(this.repoRoot,scriptCompilerAdmissionControlV1(this.invocation,"cache:tools:physical-proof",120000));await proof.proveRepositoryToolingChargeV1(this.repoRoot,this.invocation);await proof.proveRepositoryCacheScopeLifetimeV1(this.repoRoot,this.invocation);return;}
    const inferenceTests:Record<string,string>={"rustup-protocol":"🚀️bootstrap/🛠️tools/🦀️rustup/🧪️tests/🟦️.ts","cargo-ingress":"💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🧪️tests/🟦️.ts","cargo-launcher":"../🏃️process/🦀️cargo/🚪️launcher/🧪️tests/🟦️.ts","runtime-verification-storage":"../🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/🟦️.ts","cache-scope":"💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🏪️scope.ts","javascript-storage":"🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts","lease-bootstrap":"../../../../../🔨️modules/🏃️process/🔒️leases/📦️bounded/🧪️tests/🟦️.ts","toolkit-preflight":"🚀️bootstrap/🛠️tools/🔏️authorizers/📥️loading/🧪️tests/🟦️.ts","storage-budget":"💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts","graph-bounded":"💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧪️tests/🟦️.ts","bazel-storage":"../🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts","repository-exact":"../🏃️process/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts","nx-bounded-image":"📦️artifacts/🔒️bounded/🧪️tests/🍎️image.ts","nx-bounded":"📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts","rust-units":"🦀️cargo/📦️units/🧪️tests/🟦️.ts","caller-environment":"🧪️tests/🌍️caller-environment/🟦️.ts","toolkit-retention":"🚀️bootstrap/🛠️tools/🧹️retention/🧪️tests/🟦️.ts","graph-publication":"🕸️graph/📤️publication/🧪️tests/🟦️.ts","import-edges":"🧪️tests/🔗️import-edges","task-child":"🧪️tests/📬️task-child/🟦️.ts","toolkit-use":"🚀️bootstrap/🛠️tools/📥️use/🧪️tests/🟦️.ts","project-admission":"../🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts","inference-observation":"../🏗️build/👁️observation/🧪️tests/🟦️.ts"};
    if(args[0]&&inferenceTests[args[0]]){if(args.length!==1)throw Error("Expected one inference test command");const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR??repoCacheDirectory(this.repoRoot,"inference-probes");mkdirSync(artifacts,{recursive:true});process.stdout.write(await captureOwnedCommandV1(process.execPath,["test",...(args[0]==="import-edges"?["🧪️tests/🔗️import-edges/🟦️.ts","🧪️tests/🔗️import-edges/📁️paths/🟦️.ts","🧪️tests/🔗️import-edges/🔁️context/🟦️.ts","🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts"].map(path=>join(SCRIPT_ROOT,path)):(args[0]==="nx-bounded"?[join(SCRIPT_ROOT,inferenceTests[args[0]]!),join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🟦️.ts")]:args[0]==="repository-exact"?[join(SCRIPT_ROOT,inferenceTests[args[0]]!),join(SCRIPT_ROOT,"../🏃️process/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts")]:args[0]==="rust-units"?[join(SCRIPT_ROOT,inferenceTests[args[0]]!),join(SCRIPT_ROOT,"🦀️cargo/📦️units/🧪️tests/📥️source/🟦️.ts"),join(this.repoRoot,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🧪️tests/🟦️.ts"),join(this.repoRoot,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🧪️tests/🟦️.ts")]:args[0]==="cargo-ingress"?[join(SCRIPT_ROOT,inferenceTests[args[0]]!),join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/🧪️tests/🟦️.ts"),join(this.repoRoot,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🧪️tests/🟦️.ts")]:[join(SCRIPT_ROOT,inferenceTests[args[0]]!)])),],this.repoRoot,{control:scriptCompilerAdmissionControlV1(this.invocation,"nx:"+args[0],args[0]==="rust-units"?600000:120000),env:{...scriptInvocationEnvironment(this.invocation,process.env),SEMIO_TEST_ARTIFACT_DIR:artifacts}}));if(["nx-bounded","toolkit-use","graph-publication"].includes(args[0])){const strictOwner=args[0]==="nx-bounded"?"📦️artifacts/🔒️bounded":args[0]==="toolkit-use"?"🚀️bootstrap/🛠️tools/📥️use":"🕸️graph/📤️publication";await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(SCRIPT_ROOT,strictOwner,"🟦️.ts"),join(SCRIPT_ROOT,strictOwner,"🧪️tests/🟦️.ts"),...(args[0]==="nx-bounded"?[join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🟦️.ts")]:[])],this.repoRoot,{control:scriptCompilerAdmissionControlV1(this.invocation,"nx:"+args[0]+":typescript",120000),env:scriptInvocationEnvironment(this.invocation,process.env)});console.log("[DEBUG] "+args[0]+" strict TypeScript check passed");}return;}
    if(args[0]==="selected-graph"){if(args.length!==1)throw Error("Expected test selected-graph");await testWorkspaceOwnership(this.invocation,this.repoRoot,process.env.SEMIO_TEST_ARTIFACT_DIR??repoCacheDirectory(this.repoRoot,"selected-graph-probes"),join(SCRIPT_ROOT,"🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts"));return;}
    if(args[0]==="graph-seal"){if(args.length!==1)throw Error("Expected test graph-seal");runCmd(process.execPath,["test",join(SCRIPT_ROOT,"🕸️graph/🔏️seal/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,...orchestratorBudgetOpts()});return;}
    if (args[0] === "build-mode") {
      if (args.length !== 1) throw Error("Expected test build-mode");
      const { testNxBuildModes } = await import("./🧪️tests/🎚️build-mode/🟦️.ts");
      await testNxBuildModes(this.repoRoot, process.env.SEMIO_TEST_ARTIFACT_DIR);
      return;
    }
    if (args[0] === "next-runtime") {
      if (args.length > 2 || (args.length === 2 && args[1] !== "dev")) throw Error("Expected test next-runtime [dev]");
      const output = process.env.SEMIO_TEST_ARTIFACT_DIR ?? repoCacheDirectory(this.repoRoot, "next-runtime-probes");
      const runtime = await import("./🧪️tests/🌐️next-runtime/🟦️.ts");
      await (args[1] === "dev" ? runtime.proveNextDevRuntime : runtime.proveNextRuntime)(this.repoRoot, output, this.invocation);
      return;
    }
    if (args[0] === "web-build") {
      if (args.length !== 1) throw Error("Expected test web-build");
      const output = process.env.SEMIO_TEST_ARTIFACT_DIR ?? repoCacheDirectory(this.repoRoot, "web-build-probes");
      await (await import("./🧪️tests/🏗️web-build/🟦️.ts")).proveWebBuild(this.repoRoot, output, this.invocation);
      return;
    }
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
      await runProcessTestOwner(this.invocation, this.repoRoot, output, "cargo-build-lease");
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
    if (args[0] === "workspace-ownership") {
      if (args.length !== 1) throw Error("Expected test workspace-ownership");
      const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR ?? join(this.repoRoot, ".🧬semio/🦑️repo/⚡️cache/tests/bun-workspace");
      mkdirSync(artifacts, { recursive: true });
      await testWorkspaceOwnership(this.invocation, this.repoRoot, artifacts);
      const ownership = await import("../🗂️workspaces/🟦️bun/🟦️.ts");
      const report = await ownership.inspectBunWorkspaceOwnership(this.repoRoot, {kind:"all"}, {
        signal: this.invocation.control.signal,
        advance: async progress => {
          checkScriptInvocation(this.invocation);
          if (progress.completedOperations % 128 === 0) {
            await this.invocation.control.publish({ owner: this.invocation.policy.owner, command: "workspace-ownership " + progress.operation + " " + progress.completedOperations, stage: "running" });
            await this.invocation.control.yieldContinuation();
          }
          checkScriptInvocation(this.invocation);
        },
      });
      checkScriptInvocation(this.invocation);
      writeFileSync(join(artifacts, "bun-workspace-ownership.json"), JSON.stringify(report, null, 2) + "\n");
      console.log("[DEBUG] Original current Bun publication and source ownership verified " + report.scopes.length + " installation scopes");
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
    await testCacheContracts(this.invocation);
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
    const lease = await acquireQueuedResourceLease({ directory: join(this.repoRoot,".cache/b/l/m/data"), resource, mode, owner, signal: controller.signal });
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
    await new ArtifactPackageContractScript(this.root, this.repoRoot, this.invocation).run();
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
  .register("cache-retire-inactive", InactiveCargoRetirementScript)
  .register("cargo-provenance", CargoProvenanceScript);

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
