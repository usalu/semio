#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {parseCargoPreparationStorageV1,type CargoPreparationStorageV1} from "../../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import {nativeCallerPhaseV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {OwnedCommandFailure} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { Script, ScriptRouter,type ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { discoverCargoWorkspaces } from "../../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { getWorkspaceRoot } from "../../../../🗂️workspaces/🟦️.ts";
import { repoToolCacheEnv } from "../../../../🏃️process/🌿️environment/🟦️.ts";
import { wasmBindgenVersion } from "../../../🦀️cargo/🟦️.ts";
import {runNativeDependencyToolV1,runPreparedCargoDependencyPairV1,type NativeDependencyToolRunnerV1 } from "../📜️script.ts";

export type DependencyToolRunner=NativeDependencyToolRunnerV1;

/** 🛠️ Synchronizes one native environment; Nx schedules every cross-environment prerequisite. */
export async function prepareDependencies(kind: string, workspace: string, invocation:ScriptInvocation, storage:CargoPreparationStorageV1,runner: DependencyToolRunner = runNativeDependencyToolV1): Promise<void> {
  const phase=nativeCallerPhaseV1(invocation,workspace),signal=invocation.control.signal;await phase.advance("native-dependencies:"+kind);
  const run = async (command: string, args: string[], capture = false, environment?: NodeJS.ProcessEnv): Promise<string> => {
    await phase.advance(command+":"+args[0]);
    const result = await runner(command, args, workspace, signal, capture, environment??process.env,storage,phase);
    phase.budget();
    return result;
  };
  const probe = async (command: string, args: string[]): Promise<string> => {
    try { return await run(command, args, true); }
    catch(error){signal.throwIfAborted();phase.budget();if(!(error instanceof OwnedCommandFailure))throw error;return "";}
  };
  const tool = async (crate: string, command: string[], version: string): Promise<void> => {
    if ((await probe(command[0]!, [...command.slice(1), "--version"])).split(/\s+/).includes(version)) return;
    await run("cargo", ["install", crate, "--version", version, "--locked"]);
  };
  const target = async (name: string): Promise<void> => {
    if (!(await probe("rustup", ["target", "list", "--installed"])).trim().split(/\r?\n/).includes(name))
      await run("rustup", ["target", "add", name]);
  };
  if (kind === "python") await run("uv", ["sync", "--locked", "--all-packages", "--all-groups"]);
  else if (kind === "cargo") { for (const owner of discoverCargoWorkspaces(workspace)) { await runPreparedCargoDependencyPairV1(workspace,owner.manifest,invocation,storage,runner,phase); } }
  else if (kind === "cargo-lock") { for (const owner of discoverCargoWorkspaces(workspace)) await run("cargo", ["update", "--workspace", "--manifest-path", owner.manifest]); }
  else if (kind === "go") await run("go", ["mod", "download"], false, { ...process.env, GOWORK: join(workspace, "go.work") });
  else if (kind === "dotnet") console.log("[deps-dotnet] Nx project restores completed");
  else if (kind === "cpp") console.log("[deps-cpp] Nx native tooling prerequisite completed");
  else if (kind === "browsers") await run("bun", [join(workspace, "node_modules/playwright/cli.js"), "install", "chromium"], false, repoToolCacheEnv(workspace));
  else if (kind === "wasm" || kind === "trunk") {
    const bindgen = wasmBindgenVersion(readFileSync(join(workspace, "Cargo.lock"), "utf8"));
    if (kind === "wasm") await tool("wasm-pack", ["wasm-pack"], "0.15.0");
    else await tool("trunk", ["trunk"], "0.21.14");
    await tool("wasm-bindgen-cli", ["wasm-bindgen"], bindgen);
    await target(kind === "wasm" ? "wasm32-wasip2" : "wasm32-unknown-unknown");
    console.log(`[deps-${kind}] Pinned tools and Rust WebAssembly target are ready`);
  } else if (kind === "tools") {
    await tool("cargo-nextest", ["cargo", "nextest"], "0.9.140");
    await tool("cargo-llvm-cov", ["cargo", "llvm-cov"], "0.8.7");
  } else throw new Error(`Unknown dependency environment: ${kind}`);
}

/** 🔌️ Receives the original native caller for the selected dependency environment. */
export class NativeDependenciesScript extends Script {
 async run(args:string[]):Promise<void>{
  if(!(args.length===1||args.length===3&&args[1]==="--storage"))throw Error("Native dependency synchronization requires one environment and accepts no installer overrides");
  try{await prepareDependencies(args[0]!,this.root,this.invocation,args.length===3?parseCargoPreparationStorageV1({version:1,directory:args[2],program:repositoryCargoPreparationStorageV1(process.cwd()).program}):repositoryCargoPreparationStorageV1(this.root));}
  catch(error){this.invocation.control.signal.throwIfAborted();throw error;}
 }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(getWorkspaceRoot()).register("sync", NativeDependenciesScript)).run(process.argv.slice(2), original));
