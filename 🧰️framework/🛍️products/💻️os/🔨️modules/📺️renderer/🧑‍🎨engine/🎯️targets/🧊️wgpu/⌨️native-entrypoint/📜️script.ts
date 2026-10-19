#!/usr/bin/env bun
import { readNativeCallerAuthorityV1, withRepositoryExecutablePinV1 } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import {withPlaygroundSessionV1} from "../../../../../🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts";
import {createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { createAssetHttpServerV1 } from "../../../../../../../../🔨️modules/🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🧭️dispatch/🟦️.ts";
import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "../../../../../🔌️plugin/📇️registry/🎮️playground/🖼️assets/🧩️composition/🟦️.ts";
import { playgroundNativeHostArtifactV1 } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import {compilerAdmissionBudgetV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { once } from "node:events";
import nativeLaunchDeclaration from "./🎟️launch/🔣️.json";
import { nativeLaunchEnvironmentV1, readNativeLaunchDeclarationV1 } from "./🎟️launch/🟦️.ts";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import type { Server } from "node:http";
import { basename, dirname, isAbsolute, join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

import { nativeRendererBinary } from "../🏗️compiler/🦀️native/📜️script.ts";
import { nativeRuntimeDirectory,withNativeRuntimeV1 } from "./📦️modules/🟦️.ts";

const ownerPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript";
const registryPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";

/** 🔐️ Excludes parent credentials from ordinary native renderer processes. */
export function nativeRunnerEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  return Object.fromEntries(Object.entries(source).filter(([key]) => {
    const normalized = key.toUpperCase();
    return !["S_USER", "VITE_S_USER", "S_HUB_URL"].includes(normalized) && !/TOKEN|SESSION|CREDENTIAL|BEARER|CAPABILITY|AUTHORIZATION|COOKIE/.test(normalized);
  }).concat([["SEMIO_DIRECT_CHILD_BENIGN", "preserved"]]));
}

/** 🏃️ Carries actual phase authority through immutable acquisition and the complete owned child lifetime. */
export async function runNativeBinary(executable:string,args:readonly string[],environment:NodeJS.ProcessEnv,cwd:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);const capturedArgs=Object.freeze([...args]),capturedEnvironment=Object.freeze({...environment});const execute=async(binary:string,control:ResourceLeaseControl)=>{requireIssuedResourceControlInvocationV1(control,invocation);requireIssuedResourceControlRefinementV1(phaseControl,control);const budget=compilerAdmissionBudgetV1(control);await runOwnedCommand(binary,[...capturedArgs],cwd,"native:execution",budget,{env:nativeRunnerEnvironment(capturedEnvironment),signal:control.signal,onProgress:async()=>{checkResourceLeaseControl(control);await control.publish({resource:"native:execution",mode:"shared",elapsedMs:0});checkResourceLeaseControl(control);await control.yieldContinuation();checkResourceLeaseControl(control);}});checkResourceLeaseControl(control);};
 if(isAbsolute(executable))return withRepositoryExecutablePinV1({workspace:readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot,invocation,phaseControl,source:executable},execute);return execute(executable,phaseControl);
}

/** 🌐️ Starts one ready asset listener, runs the native consumer and closes all owned connections. */
export async function runNativeSession(executable: string, args: readonly string[], environment: NodeJS.ProcessEnv, cwd:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,startAssets?: () => Server): Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);const signal=phaseControl.signal;
  const server=startAssets?.();let failed=false;
  try {
    if(server&&!server.listening){const stop=new AbortController(),remaining=checkResourceLeaseControl(phaseControl),timer=remaining===null?undefined:setTimeout(()=>stop.abort(Error("Native asset readiness phase exhausted")),remaining);try{await once(server,"listening",{signal:AbortSignal.any([signal,stop.signal])});checkResourceLeaseControl(phaseControl);}finally{if(timer)clearTimeout(timer);}}
    const address = server?.address();
    if (server && (!address || typeof address === "string")) throw new Error("Native asset server needs a TCP address");
    const env = { ...environment, ...(address && typeof address !== "string" ? { SEMIO_ASSET_BASE_URL: `http://127.0.0.1:${address.port}` } : {}) };
    await runNativeBinary(executable,args,env,cwd,invocation,phaseControl);
  }catch(error){failed=true;throw error;}finally{
    const failures:unknown[]=[];if(server){try{server.closeAllConnections();}catch(error){failures.push(error);}try{await new Promise<void>((accept,refuse)=>server.close(error=>error?refuse(error):accept()));}catch(error){failures.push(error);}}if(!failed&&failures.length)throw failures[0];
  }
}

/** 🧭️ The per-navigation boot axes the native binary reads, the CLI twin of the browser's
 * `?app=&role=&mode=&example=&hub=&user=&dataDir=` (`../🧭️boot-descriptor/🟦️.ts`). `--brand` is here
 * too so a dev can override the playground row's own brand; `--plugin` is not, because the variant
 * positional already names it. */
const BOOT_AXIS_FLAGS = ["--app", "--role", "--mode", "--example", "--brand", "--hub", "--user", "--data-dir"] as const;

/** 🎛️ Rejects undeclared profiles and variants before reading completed artifacts, and admits the
 * boot axes as `--flag value` pairs so one invocation can open the document a url opens. */
function selection(args: string[], smoke = false): { variant: string; profile: "prototype" | "release"; axes: string[] } {
  const [variant, profile, ...rest] = args;
  if(!variant||variant.length>64||!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant)||!(profile==="prototype"||profile==="release"))throw Error("Exact Native variant and mode required");
  const axes: string[] = [];
  for (let index = 0; index < rest.length; index++) {
    const flag = rest[index]!;
    if (smoke && flag === "--smoke") continue;
    if (!(BOOT_AXIS_FLAGS as readonly string[]).includes(flag) || rest[index + 1] === undefined) throw new Error(`Native runtime accepts a variant, profile, optional --smoke and ${BOOT_AXIS_FLAGS.join("/")} value pairs`);
    axes.push(flag, rest[++index]!);
  }
  return { variant, profile: profile as "prototype" | "release", axes };
}

/** 🖥️ Runs completed native artifacts without compiling or invoking another build graph. */
class RunScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const {variant,profile,axes}=selection(args,true),repo=this.repoRoot;if(profile!==readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode)throw Error("Native run must match actual Original mode");
    await withPlaygroundSessionV1(repo,this.invocation,variant,async(_session,projection,control)=>{
    return withNativeRuntimeV1(repo,this.invocation,variant,control,async(runtime,_manifest,runtimeControl)=>{
    const catalog=projection.playgrounds;
    const row = catalog.find((entry: { variant: string }) => entry.variant === variant);
    if (!row) throw new Error(`Unknown native playground: ${variant}`);
    const signal = this.invocation.control.signal;
    {
      const binary=playgroundNativeHostArtifactV1(repo,row.nativeHost,profile)??nativeRendererBinary(resolve(import.meta.dir,"../📦️packages/🦀️rust"),profile);
      if(!existsSync(binary))throw new Error(`Missing native host artifact: ${binary}`);
      // 🧭️ An explicit `--flag` is spelled FIRST, because the binary's `arg_value` reads the first
      // occurrence; the playground row then supplies the per-server defaults (`app`, `brand`) the wgpu
      // serve injects as `<meta name="semio-*">` for the browser.
      await runNativeSession(binary, ["--plugin", variant, ...axes, ...(row.app ? ["--app", row.app] : []), ...(row.brand ? ["--brand", row.brand] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], nativeLaunchEnvironmentV1(this.invocation, { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_PLUGIN_MODULES: runtime, SEMIO_RUNTIME_ASSETS: repo }, readNativeLaunchDeclarationV1(nativeLaunchDeclaration), Date.now()), repo,this.invocation,createRequestedIssuedResourceControlV1(runtimeControl,0,signal),row.assets?.length ? () => createAssetHttpServerV1(repo, 0, row.assets, PLAYGROUND_ASSET_PROVIDERS_V1) : undefined);
    }
    });
    });
  }
}

/** 🌉 Runs the exact caller-declared MCP producer with its authored service inventory. */
class McpScript extends BundleScript {
  async run([variant,profile,transport,...args]:string[]):Promise<void> {
    if(!variant||(profile!=="prototype"&&profile!=="release")||(transport!=="stdio"&&transport!=="http"))throw new Error("MCP owner selection requires a variant, exact profile and transport");
    const catalog=JSON.parse(readFileSync(join(this.repoRoot,registryPath,"🤖️generated/🎠️playgrounds.json"),"utf8"));
    const row=catalog.find((entry:{variant:string})=>entry.variant===variant);
    if(!row?.mcpHost)throw new Error(`Playground does not declare an MCP producer: ${variant}`);
    const binary=playgroundNativeHostArtifactV1(this.repoRoot,row.mcpHost,profile as "prototype"|"release");
    if(!binary || !existsSync(binary))throw new Error("Missing MCP owner artifact");
    const controller=new AbortController(),cancel=()=>controller.abort();
    const phaseControl=createIssuedResourceControlV1(this.invocation,"native:mcp",0,controller.signal);
    process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
    try {await withRepositoryExecutablePinV1({workspace:this.repoRoot,invocation:this.invocation,phaseControl,source:binary},async(executable,actualControl)=>{await runOwnedCommand(executable,[transport,...args],this.repoRoot,"native:mcp",compilerAdmissionBudgetV1(actualControl),{env:process.env,signal:actualControl.signal});checkResourceLeaseControl(actualControl);});}
    finally {process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);}
  }
}

/** ⚖️ Runs the native scale consumer with only its binary and fixture prerequisites. */
class ScaleScript extends BundleScript {
  async run([profile, ...args]:string[]):Promise<void> {
    const phaseControl=createIssuedResourceControlV1(this.invocation,"native:scale",0);
    if(profile!=="prototype"&&profile!=="release")throw Error("Exact Native scale mode required");nativeRuntimeDirectory(this.repoRoot,"scale",profile);
    if (!args.length || args.length % 2 !== 0 || args.some((value, index) => index % 2 === 0 && !["--scale", "--scale-wasm", "--shards", "--report"].includes(value))) throw new Error("Native scale accepts --scale, --scale-wasm, --shards and --report value pairs");
    for (const flag of ["--scale", "--scale-wasm", "--report"]) if (args.filter(value => value === flag).length !== 1) throw new Error(`Native scale requires exactly one ${flag}`);
    const report=args[args.indexOf("--report")+1];if(!report)throw Error("Finite scale report required");mkdirSync(dirname(resolve(this.repoRoot,report)),{recursive:true});
    const signal = this.invocation.control.signal;
    await runNativeBinary(nativeRendererBinary(resolve(import.meta.dir, "../📦️packages/🦀️rust"), profile), args, nativeLaunchEnvironmentV1(this.invocation, process.env, readNativeLaunchDeclarationV1(nativeLaunchDeclaration), Date.now()),this.repoRoot,this.invocation,createRequestedIssuedResourceControlV1(phaseControl,0,signal));
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("run", RunScript).register("scale", ScaleScript).register("mcp",McpScript)).run(process.argv.slice(2), original));
