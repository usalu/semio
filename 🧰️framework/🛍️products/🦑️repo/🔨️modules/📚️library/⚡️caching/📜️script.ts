#!/usr/bin/env bun
import{createIssuedResourceControlV1,createRequestedIssuedResourceControlV1}from"../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
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
import {devToolingEnv} from "../🏃️process/🌿️environment/🟦️.ts";
import {getWorkspaceRoot} from "../🗂️workspaces/🟦️.ts";
import {orchestratorBudgetOpts,runCmd} from "../🏃️process/🟦️.ts";
import { wasmBindgenVersion } from "./🦀️cargo/🕸️wasm-bindgen/🟦️.ts";
import {wasmBuildEnvironment} from "../🏃️process/🕸️wasm/🔣️policy/🟦️.ts";
import { BundleScript, ScriptRouter, checkScriptInvocation, scriptInvocationBudget, scriptInvocationEnvironment, type ScriptInvocation } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { captureOwnedCommandV1, terminateOwnedChildTree, resolveTypescriptCompilerV1 } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runScriptMain } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import plugin, { cacheInternals } from "../🏗️build/🕸️graph/📥️inference/🟨️.mjs";
import { stageRepositoryArtifacts } from "./📦️artifacts/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { repoCacheDirectory } from "./🟦️.ts";

import {withRepositoryTestArtifactsV1,repositoryTestArtifactEnvironmentV1,repositoryTestArtifactRootV1} from "@semio-tech/repo-test/storage";

import type{ResourceLeaseControl}from"@semio-tech/process/leases";
const SCRIPT_ROOT = dirname(fileURLToPath(import.meta.url));
export async function testCacheContracts(invocation: ScriptInvocation, phaseControl: ResourceLeaseControl): Promise<void> {
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
  await tests.testCacheContracts(invocation,phaseControl);
}

/** 🧪️ Runs the native workspace laws inside the same original command cancellation and deadline. */
async function testWorkspaceOwnership(invocation: ScriptInvocation, root: string, artifacts: string, phaseControl:ResourceLeaseControl, entrypoint = join(SCRIPT_ROOT, "../🗂️workspaces/🟦️bun/🧪️tests/🟦️.ts")): Promise<void> {
  checkScriptInvocation(invocation);if(artifacts!==repositoryTestArtifactRootV1(root))throw new Error("Original finite workspace test output required");
  const environment=repositoryTestArtifactEnvironmentV1(root,scriptInvocationEnvironment(invocation,process.env)),options=orchestratorBudgetOpts(environment),control=createRequestedIssuedResourceControlV1(phaseControl,options.budgetMs!),transcript=await captureOwnedCommandV1(process.execPath,["test",entrypoint],root,{control,env:environment,output:{maximumBytes:16777216,maximumLines:65536}});
  checkScriptInvocation(invocation);process.stdout.write(transcript);writeFileSync(join(repositoryTestArtifactRootV1(root),"bun-test-"+createHash("sha256").update(entrypoint).digest("hex").slice(0,16)+".log"),transcript);
  if(!/\b[1-9][0-9]* pass\b/u.test(transcript))throw new Error("Original Bun test owner did not report completed tests");
}

/** 🧪️ Routes the full cache suite, the focused portable command-source contract or the build-dir provenance laws. */
class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await withRepositoryTestArtifactsV1(this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cache:tests",0),actualPhase=>this.runTests(args,repositoryTestArtifactEnvironmentV1(this.repoRoot,scriptInvocationEnvironment(this.invocation,process.env)),actualPhase));
  }
  private async runBunTests(args:readonly string[],budgetMs:number,environment:NodeJS.ProcessEnv,phaseControl:ResourceLeaseControl):Promise<void>{
    process.stdout.write(await captureOwnedCommandV1(process.execPath,args,this.repoRoot,{control:createRequestedIssuedResourceControlV1(phaseControl,budgetMs),env:environment,output:{maximumBytes:16777216,maximumLines:65536}}));
  }
  private async runTests(args:string[],environment:NodeJS.ProcessEnv,phaseControl:ResourceLeaseControl):Promise<void>{
    if(args[0]==="playground-preparation-source"){
      if(args.length!==1)throw new Error("Expected test playground-preparation-source");
      const source=join(SCRIPT_ROOT,"🧪️tests/🎮️playground-preparation/🧪️tests/🟦️.ts"),budget=scriptInvocationBudget(this.invocation,120000);
      await this.runBunTests([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowJs","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",source],budget,environment,phaseControl);
      await this.runBunTests(["test","--timeout",String(scriptInvocationBudget(this.invocation,budget)),source],scriptInvocationBudget(this.invocation,budget),environment,phaseControl);return;
    }
    if(args[0]==="bootstrap-original-invocation"){
      if(args.length!==1)throw new Error("Expected test bootstrap-original-invocation");
      const source=join(SCRIPT_ROOT,"🚀️bootstrap/📥️invocation/🧪️tests/🟦️.ts"),budget=scriptInvocationBudget(this.invocation,120000);
      await this.runBunTests([resolveTypescriptCompilerV1(this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],budget,environment,phaseControl);
      await this.runBunTests(["test","--timeout",String(scriptInvocationBudget(this.invocation,budget)),source],scriptInvocationBudget(this.invocation,budget),environment,phaseControl);return;
    }
    if(args[0]==="pruning"){
      if(args.length!==1)throw new Error("Expected test pruning");
      const owner=join(SCRIPT_ROOT,"🧹️pruning"),artifacts=environment.SEMIO_TEST_ARTIFACT_DIR;
      if(!artifacts)throw new Error("Original ticket artifact directory required");
      await testWorkspaceOwnership(this.invocation,this.repoRoot,artifacts,phaseControl,join(owner,"⏱️continuation/🧪️tests/🟦️.ts"));
      await testWorkspaceOwnership(this.invocation,this.repoRoot,artifacts,phaseControl,join(owner,"🔒️store/🧪️tests/🟦️.ts"));
      await testWorkspaceOwnership(this.invocation,this.repoRoot,artifacts,phaseControl,join(owner,"🔒️incremental/🧪️tests/🟦️.ts"));
      const control=createRequestedIssuedResourceControlV1(phaseControl,120000);
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(owner,"🟦️.ts")],this.repoRoot,{control,env:environment});
      return;
    }
    if(args[0]==="sdk-snapshot"){
      if(args.length!==1)throw new Error("Expected test sdk-snapshot");
      const owner=join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools/📦️sdk/📥️snapshot");
      await testWorkspaceOwnership(this.invocation,this.repoRoot,repositoryTestArtifactRootV1(this.repoRoot),phaseControl,join(owner,"🧪️tests/🟦️.ts"));
      const control=createRequestedIssuedResourceControlV1(phaseControl,120000);
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(owner,"🟦️.ts")],this.repoRoot,{control,env:environment});
      const material=join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools/📦️sdk/📊️material");await testWorkspaceOwnership(this.invocation,this.repoRoot,repositoryTestArtifactRootV1(this.repoRoot),phaseControl,join(material,"🧪️tests/🟦️.ts"));await testWorkspaceOwnership(this.invocation,this.repoRoot,repositoryTestArtifactRootV1(this.repoRoot),phaseControl,join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/🟦️.ts"));const {withNxSdkMaterialReportV1,proveNxSdkMaterialConsumptionV1}=await import("./🚀️bootstrap/🛠️tools/📦️sdk/📊️material/🟦️.ts");await withNxSdkMaterialReportV1(this.repoRoot,this.invocation,phaseControl,async(report,store,materialPhase)=>{await proveNxSdkMaterialConsumptionV1(report.generation,report.generationMaterial,store,materialPhase);await proveNxSdkMaterialConsumptionV1(report.snapshot,report.snapshotMaterial,store,materialPhase);const {proveNxSdkBinaryPublicationV1}=await import("./🚀️bootstrap/🛠️tools/📦️sdk/🧊️binary/🧪️tests/📥️runtime/🟦️.ts");await proveNxSdkBinaryPublicationV1(store,materialPhase);writeFileSync(join(repositoryTestArtifactRootV1(this.repoRoot),"sdk-material.json"),JSON.stringify(report,null,2)+"\n");});return;
    }
    if(args[0]==="journal-source"){
      if(args.length!==1)throw new Error("Expected test journal-source");
      const owner=join(SCRIPT_ROOT,"🕸️graph/🔏️seal/📥️membership/📥️journal");
      await testWorkspaceOwnership(this.invocation,this.repoRoot,repositoryTestArtifactRootV1(this.repoRoot),phaseControl,join(owner,"🧪️tests/🟦️.ts"));
      const control=createRequestedIssuedResourceControlV1(phaseControl,120000);
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(owner,"🟦️.ts")],this.repoRoot,{control,env:environment});
      return;
    }
    if(args[0]==="authorizers"){
      if(args.length!==1)throw new Error("Expected test authorizers");
      await testWorkspaceOwnership(this.invocation,this.repoRoot,repositoryTestArtifactRootV1(this.repoRoot),phaseControl,join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/🟦️.ts"));
      const source=join(SCRIPT_ROOT,"🚀️bootstrap/🛠️tools"),control=createRequestedIssuedResourceControlV1(phaseControl,120000);
      await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(source,"🔏️authorizers/🟦️.ts"),join(source,"🔏️authorizers/📜️script.ts"),join(source,"🔏️authorizers/📤️receipt/🟦️.ts"),join(source,"📜️script.ts")],this.repoRoot,{control,env:environment});
      return;
    }
    if (args[0] === "daemon-journal") {
      if (args.length !== 1 && (args.length !== 2 || !["transport", "workspace"].includes(args[1]!))) throw new Error("Expected test daemon-journal [transport|workspace]");
      const output = repositoryTestArtifactRootV1(this.repoRoot);
      await (await import("./🧪️tests/🕸️daemon/📥️journal/🟦️.ts")).proveDaemonJournal(this.repoRoot, output, this.invocation, args[1] === "transport", args[1] === "workspace");
      return;
    }
    if(args[0]==="graph-cache-waiter"){if(args.length!==1)throw new Error("Expected test graph-cache-waiter");await testWorkspaceOwnership(this.invocation,this.repoRoot,repositoryTestArtifactRootV1(this.repoRoot),phaseControl,join(SCRIPT_ROOT,"🕸️graph/🏇️waiter/🧪️tests/🟦️.ts"));return;}
    if(args[0]==="binaryen-partition"){if(args.length!==1)throw new Error("Expected test binaryen-partition");const {withRepositoryToolingStoreV1}=await import("./💾️storage/🔒️enforcement/🛠️tools/🟦️.ts"),{prepareBinaryen}=await import("./🚀️bootstrap/🛠️tools/🕸️wasm/📜️script.ts");await withRepositoryToolingStoreV1(this.repoRoot,this.invocation,createIssuedResourceControlV1(this.invocation,"cache:entry",0),async store=>{const executable=await prepareBinaryen(this.repoRoot,store);const version=await captureOwnedCommandV1(executable,["--version"],this.repoRoot,{control:store.control,env:scriptInvocationEnvironment(this.invocation,store.environment)});if(version.trim()!=="wasm-opt version 130 (version_130)")throw new Error("Original bounded Binaryen version refused");});return;}
    if(args[0]==="bazel-storage-runtime"){if(args.length!==1)throw new Error("Expected test bazel-storage-runtime");await (await import("../🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/📥️runtime/🟦️.ts")).proveBazelStorageRuntimeV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="runtime-verification-storage-runtime"){if(args.length!==1)throw new Error("Expected one verifier storage proof");await(await import("../🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/📥️runtime/🟦️.ts")).proveRuntimeVerificationStorageV1(this.repoRoot,this.invocation);await(await import("./💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/⚙️configuration/🧪️tests/📥️runtime/🟦️.ts")).proveCargoCompilerConfigurationAdmissionV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="javascript-partition"){if(args.length!==1)throw new Error("Expected test javascript-partition");await (await import("./🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts")).proveJavascriptDependencyPartitionV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="lease-queue"){if(args.length!==1)throw new Error("Expected test lease-queue");await (await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🧪️testing/🔒️resource-leases/🟦️.ts")).testResourceLeases(environment.SEMIO_TEST_ARTIFACT_DIR!,createRequestedIssuedResourceControlV1(phaseControl,0),this.invocation);return;}
    if(args[0]==="leases-partition"){if(args.length!==1)throw new Error("Expected test leases-partition");const proof=await import("./💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🟦️.ts"),control=createRequestedIssuedResourceControlV1(phaseControl,120000);await proof.proveRepositoryLeasePartitionV1(this.repoRoot,control);await proof.proveRepositoryQueueMetadataHeadroomV1(this.repoRoot,this.invocation,control);return;}
    if(args[0]==="rustup-toolchain"){if(args.length!==1)throw new Error("Expected test rustup-toolchain");const proof=await import("./🚀️bootstrap/🛠️tools/🦀️rustup/🧪️tests/🏗️installation/🟦️.ts");await proof.proveRepositoryRustToolchainV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="rustup-generation"){if(args.length!==1)throw new Error("Expected test rustup-generation");const proof=await import("./🚀️bootstrap/🛠️tools/🦀️rustup/🧪️tests/📥️runtime/🟦️.ts");await proof.proveRustupGenerationCaptureV1(this.repoRoot,this.invocation);return;}
    if(args[0]==="tools-partition"){if(args.length!==1)throw new Error("Expected test tools-partition");const proof=await import("./💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🟦️.ts");await proof.proveRepositoryToolsPartitionV1(this.repoRoot,createRequestedIssuedResourceControlV1(phaseControl,120000));await proof.proveRepositoryToolingChargeV1(this.repoRoot,this.invocation);await proof.proveRepositoryCacheScopeLifetimeV1(this.repoRoot,this.invocation);return;}
    const inferenceTests:Record<string,string>={"rustup-protocol":"🚀️bootstrap/🛠️tools/🦀️rustup/🧪️tests/🟦️.ts","cargo-ingress":"💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🧪️tests/🟦️.ts","cargo-launcher":"../🏃️process/🦀️cargo/🚪️launcher/🧪️tests/🟦️.ts","runtime-verification-storage":"../🔍️discovery/🕸️runtime/🔎️verification/💾️storage/🧪️tests/🟦️.ts","cache-scope":"💾️storage/🔒️enforcement/📦️budget/📥️opening/🧪️tests/🏪️scope.ts","javascript-storage":"🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts","lease-bootstrap":"../../../../../🔨️modules/🏃️process/🔒️leases/📦️bounded/🧪️tests/🟦️.ts","toolkit-preflight":"🚀️bootstrap/🛠️tools/🔏️authorizers/📥️loading/🧪️tests/🟦️.ts","storage-budget":"💾️storage/🔒️enforcement/📦️budget/🧪️tests/🟦️.ts","graph-bounded":"💾️storage/🔒️enforcement/📦️budget/🕸️graph/🧪️tests/🟦️.ts","bazel-storage":"../🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests/🟦️.ts","repository-exact":"../🏃️process/🦀️cargo/🎯️exact/🧪️tests/🍎️image/🟦️.ts","nx-bounded-image":"📦️artifacts/🔒️bounded/🧪️tests/🍎️image.ts","nx-bounded":"📦️artifacts/🔒️bounded/🧪️tests/🟦️.ts","rust-units":"🦀️cargo/📦️units/🧪️tests/🟦️.ts","caller-environment":"🧪️tests/🌍️caller-environment/🟦️.ts","toolkit-outputs":"💾️storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts","toolkit-retention":"🚀️bootstrap/🛠️tools/🧹️retention/🧪️tests/🟦️.ts","graph-publication":"🕸️graph/📤️publication/🧪️tests/🟦️.ts","import-edges":"🧪️tests/🔗️import-edges","task-child":"🚀️bootstrap/🛠️tools/📥️use/🧪️tests/📬️task-child/🟦️.ts","toolkit-use":"🚀️bootstrap/🛠️tools/📥️use/🧪️tests/🟦️.ts","project-admission":"../🏗️build/👁️observation/🧪️tests/📦️projects/🟦️.ts","inference-observation":"../🏗️build/👁️observation/🧪️tests/🟦️.ts"};
    if(args[0]&&inferenceTests[args[0]]){if(args.length!==1)throw new Error("Expected one inference test command");const selected=args[0];await withRepositoryTestArtifactsV1(this.repoRoot,this.invocation,phaseControl,async phaseControl=>{process.stdout.write(await captureOwnedCommandV1(process.execPath,["test",...(selected==="import-edges"?["🧪️tests/🔗️import-edges/🟦️.ts","🧪️tests/🔗️import-edges/📁️paths/🟦️.ts","🧪️tests/🔗️import-edges/🔁️context/🟦️.ts","🧪️tests/🔗️import-edges/🏛️graph/🔑️authority/🟦️.ts"].map(path=>join(SCRIPT_ROOT,path)):(selected==="nx-bounded"?[join(SCRIPT_ROOT,inferenceTests[selected]!),join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🟦️.ts")]:selected==="repository-exact"?[join(SCRIPT_ROOT,inferenceTests[selected]!),join(SCRIPT_ROOT,"../🏃️process/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts")]:selected==="rust-units"?[join(SCRIPT_ROOT,inferenceTests[selected]!),join(SCRIPT_ROOT,"🦀️cargo/📦️units/🧪️tests/📥️source/🟦️.ts"),join(this.repoRoot,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🧪️tests/🟦️.ts"),join(this.repoRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🕸️units/🟦️.ts")]:selected==="cargo-ingress"?[join(SCRIPT_ROOT,inferenceTests[selected]!),join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/🧪️tests/🟦️.ts"),join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/📤️forwarding/🧪️tests/🟦️.ts"),join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/🌍️environment/🧪️tests/🟦️.ts"),join(this.repoRoot,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🧪️tests/🟦️.ts"),join(this.repoRoot,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/⚙️configuration/🎟️admission/🧪️tests/🟦️.ts")]:[join(SCRIPT_ROOT,inferenceTests[selected]!)])),],this.repoRoot,{control:createRequestedIssuedResourceControlV1(phaseControl,selected==="rust-units"?600000:120000),env:environment}));if(["nx-bounded","toolkit-use","graph-publication","toolkit-outputs"].includes(selected)){const strictOwner=selected==="toolkit-outputs"?"💾️storage/🔒️enforcement/🔌️nx/📦️outputs":selected==="nx-bounded"?"📦️artifacts/🔒️bounded":selected==="toolkit-use"?"🚀️bootstrap/🛠️tools/📥️use":"🕸️graph/📤️publication";await captureOwnedCommandV1(process.execPath,[join(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop",join(SCRIPT_ROOT,strictOwner,"🟦️.ts"),join(SCRIPT_ROOT,strictOwner,"🧪️tests/🟦️.ts"),...(selected==="nx-bounded"?[join(SCRIPT_ROOT,"💾️storage/🔒️enforcement/🏗️compiler/🧪️tests/🟦️.ts")]:[])],this.repoRoot,{control:createRequestedIssuedResourceControlV1(phaseControl,120000),env:environment});}});return;}
    if(args[0]==="selected-graph"){if(args.length!==1)throw new Error("Expected test selected-graph");await testWorkspaceOwnership(this.invocation,this.repoRoot,repositoryTestArtifactRootV1(this.repoRoot),phaseControl,join(SCRIPT_ROOT,"🕸️graph/🔏️seal/🎯️selection/🧪️tests/🟦️.ts"));return;}
    if(args[0]==="graph-seal"){if(args.length!==1)throw new Error("Expected test graph-seal");await this.runBunTests(["test",join(SCRIPT_ROOT,"🕸️graph/🔏️seal/🧪️tests/🟦️.ts")],orchestratorBudgetOpts(environment).budgetMs!,environment,phaseControl);return;}
    if (args[0] === "build-mode") {
      if (args.length !== 1) throw new Error("Expected test build-mode");
      const { testNxBuildModes } = await import("./🧪️tests/🎚️build-mode/🟦️.ts");
      await testNxBuildModes(this.repoRoot, environment.SEMIO_TEST_ARTIFACT_DIR,this.invocation,phaseControl);
      return;
    }
    if (args[0] === "next-runtime") {
      if (args.length > 2 || (args.length === 2 && args[1] !== "dev")) throw new Error("Expected test next-runtime [dev]");
      const output = repositoryTestArtifactRootV1(this.repoRoot);
      const runtime = await import("./🧪️tests/🌐️next-runtime/🟦️.ts");
      await (args[1] === "dev" ? runtime.proveNextDevRuntime : runtime.proveNextRuntime)(this.repoRoot, output, this.invocation);
      return;
    }
    if (args[0] === "web-build") {
      if (args.length !== 1) throw new Error("Expected test web-build");
      const output = repositoryTestArtifactRootV1(this.repoRoot);
      await (await import("./🧪️tests/🏗️web-build/🟦️.ts")).proveWebBuild(this.repoRoot, output, this.invocation,phaseControl);
      return;
    }
    if (args[0] === "cache-policy") {
      if (args.length !== 1) throw new Error("Expected test cache-policy");
      const { proveCachePolicy } = await import("./🧪️tests/⚡️cache-contracts/🟦️.ts");
      proveCachePolicy(this.repoRoot, cacheInternals);
      return;
    }
    if (args[0] === "cargo-build-lease") {
      if (args.length !== 1) throw new Error("Expected test cargo-build-lease");
      const output = environment.SEMIO_TEST_ARTIFACT_DIR;
      if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
      const {runProcessTestOwner}=await import("./🧪️tests/🧭️ownership/🟦️.ts");
      await runProcessTestOwner(this.invocation, this.repoRoot, output, "cargo-build-lease");
      return;
    }
    if (args[0] === "graph-revision") {
      if (args.length !== 1) throw new Error("Expected test graph-revision");
      const output = environment.SEMIO_TEST_ARTIFACT_DIR;
      if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
      mkdirSync(output, { recursive: true });
      await (await import("./🧪️tests/🔁️graph-revision/🟦️.ts")).testGraphRevision(this.repoRoot, output);
      return;
    }
    if(args[0]==="runtime-bootstrap"){
      if(args.length!==1)throw new Error("Expected test runtime-bootstrap");
      await (await import("./📦️artifacts/🐳️containers/🧪️tests/🚀️runtime-bootstrap/🟦️.ts")).testContainerRuntimeBootstrap(this.repoRoot,this.invocation);
      return;
    }
    if (args[0] === "native-dependencies") {
      if (args.length !== 1) throw new Error("Expected test native-dependencies");
      const output = environment.SEMIO_TEST_ARTIFACT_DIR;
      if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
      mkdirSync(output, { recursive: true });
      await (await import("./🧪️tests/📦️native-dependencies/🟦️.ts")).testNativeDependencies(this.repoRoot, output,this.invocation,phaseControl);
      return;
    }
    if (args[0] === "artifact-source") {
      if (args.length !== 1) throw new Error("Expected test artifact-source");
      await this.runBunTests(["test", join(SCRIPT_ROOT, "🧪️tests", "🗿️artifact-source", "🟦️.ts")],orchestratorBudgetOpts(environment).budgetMs!,environment,phaseControl);
      return;
    }
    if (args[0] === "workspace-ownership") {
      if (args.length !== 1) throw new Error("Expected test workspace-ownership");
      const artifacts = repositoryTestArtifactRootV1(this.repoRoot);
      mkdirSync(artifacts, { recursive: true });
      await testWorkspaceOwnership(this.invocation, this.repoRoot, artifacts,phaseControl);
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
      return;
    }
    if (args[0] === "native-input-vocabulary") {
      if (args.length !== 1) throw new Error("Expected test native-input-vocabulary");
      await this.runBunTests(["test",join(SCRIPT_ROOT,"../🧪️tests/⚡️production-cache-input-boundary/🟦️.ts")],15000,environment,phaseControl);
      return;
    }
    if (args[0] === "cache-command-source") {
      if (args.length !== 1) throw new Error("Expected test cache-command-source");
      await this.runBunTests(["test", join(SCRIPT_ROOT, "🧪️tests", "🧱️command-source", "🟦️.ts")],orchestratorBudgetOpts(environment).budgetMs!,environment,phaseControl);
      return;
    }
    if (args[0] === "cargo-provenance") {
      if (args.length !== 1) throw new Error("Expected test cargo-provenance");
      await this.runBunTests(["test", join(SCRIPT_ROOT, "🧪️tests", "🧾️cargo-provenance", "🟦️.ts")],orchestratorBudgetOpts(environment).budgetMs!,environment,phaseControl);
      return;
    }
    await testCacheContracts(this.invocation,phaseControl);
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
