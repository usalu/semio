import {existsSync} from "node:fs";
import {BundleScript,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1,type CompilerAdmissionControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts";
import { repositoryCargoNetworkV1 } from "@semio-tech/repo-lib/cargo";
import { semioBuildParallel } from "@semio-tech/process/execution";
import {resolvePluginBuildTargets} from "../📋️plan/🟦️.ts";
import {buildComponentClosureV1} from "../🧩️driver-stages/🟦️.ts";
import type {PluginBuildTargetV1} from "@semio-tech/plugin-registry/discovery";

function createConcurrencyLimiter(limit: number): { run: <T>(fn: () => Promise<T>) => Promise<T> } {
  let active = 0;
  const queue: Array<() => void> = [];
  async function acquire(): Promise<void> {
    if (active < limit) {
      active++;
      return;
    }
    await new Promise<void>((wake) => queue.push(wake));
    active++;
  }
  function release(): void {
    active--;
    queue.shift()?.();
  }
  return {
    async run<T>(fn: () => Promise<T>): Promise<T> {
      await acquire();
      try {
        return await fn();
      } finally {
        release();
      }
    },
  };
}

/** 🧵️ Concurrency cap for bounded-parallel plugin catalog stages — each materialize invocation is
 * a mostly-single-threaded subprocess with its own decoded wasm footprint, so the cap tracks machine
 * width via `semioBuildParallel()`. `SEMIO_MATERIALIZE_CONCURRENCY` overrides for measurement. */
function materializeConcurrencyLimit(): number {
  const override = process.env.SEMIO_MATERIALIZE_CONCURRENCY;
  if (override) {
    const parsed = Number.parseInt(override, 10);
    if (Number.isFinite(parsed) && parsed > 0) return parsed;
  }
  return semioBuildParallel();
}

/** 🧵️ Concurrency cap for the plugin catalog cargo stage (isolated `CARGO_TARGET_DIR` per build). */
function cargoConcurrencyLimit(): number {
  const override = process.env.SEMIO_CARGO_CONCURRENCY;
  if (override) {
    const parsed = Number.parseInt(override, 10);
    if (Number.isFinite(parsed) && parsed > 0) return parsed;
  }
  return semioBuildParallel();
}

/** 🛡️Rejects stale public output without deleting another task's retained files. */
function assertNoStalePublicPluginOutputs(path: string): void {
  if (existsSync(path)) throw new Error(`Unexpected public plugin output preserved at ${path}; inspect its owner before running @semio-tech/framework-os-dev:plugin.`);
}


/** 📋️ Resolves an authored component selection without writing or compiling anything. */
export async function preparePluginBuildTargets(filterPlugin?:string):Promise<readonly PluginBuildTargetV1[]>{return resolvePluginBuildTargets(filterPlugin);}
/** 🎯️ Builds one actual selected component through the canonical managed driver. */
export async function buildPlugin(invocation:ScriptInvocation,target:PluginBuildTargetV1,control:CompilerAdmissionControl):Promise<void>{
 const workspace=readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot;
 await buildComponentClosureV1(workspace,new Set([target.pluginId]),{facet:"browser",invocation,control,network:repositoryCargoNetworkV1(invocation,workspace)});
}
/** 🧩️ Selects the exact authored domain component set and retains its real driver through settlement. */
export async function buildPlugins(invocation:ScriptInvocation,control:CompilerAdmissionControl,filterPlugin?:string):Promise<void>{
 const workspace=readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot,targets=await preparePluginBuildTargets(filterPlugin);
 if(!targets.length)throw Error("Nonempty authored plugin build selection required");
 const outcome=await buildComponentClosureV1(workspace,new Set(targets.map(target=>target.pluginId)),{facet:"browser",invocation,control,network:repositoryCargoNetworkV1(invocation,workspace)});
 console.log(`Plugin catalog: ${targets.length} selected component(s); report ${outcome.report}`);
}
/** 🔌️ Runs the explicit domain selection under this entry's actual Original. */
class PluginBuildScript extends BundleScript{
 async run(args:string[]):Promise<void>{if(args.length>1)throw Error("plugin build [authored-filter]");await buildPlugins(this.invocation,scriptCompilerAdmissionControlV1(this.invocation,"plugins:build",0),args[0]);}
}
export {PluginBuildScript,assertNoStalePublicPluginOutputs,createConcurrencyLimiter,materializeConcurrencyLimit,cargoConcurrencyLimit};
