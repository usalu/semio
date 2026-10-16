#!/usr/bin/env bun
import {readNativeCallerAuthorityV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {withPlaygroundSessionV1} from "../../../../../🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts";
import {createRequestedIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readGeneratedCatalogProjection } from "../../../../../🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
import { createAssetHttpServerV1 } from "../../../../../../../../🔨️modules/🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts";
import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "../../../../../🔌️plugin/📇️registry/🎮️playground/🖼️assets/🧩️composition/🟦️.ts";
import {playgroundNativeHostArtifactV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🎮️playground/🖥️native-host/🟦️.ts";
import { pinExecutableArtifact } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import { once } from "node:events";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import type { Server } from "node:http";
import { basename, dirname, isAbsolute, join, resolve } from "node:path";
import { BundleScript, ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runTool} from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts";
import {repositoryCargoPreparationStorageV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";

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

/** 🧊️ Acquires the selected native host without locking its mutable producer output. */
async function nativeExecutable(executable: string, cwd: string, signal: AbortSignal): Promise<string> {
  if(!isAbsolute(executable))return executable;
  const workspace=resolve(cwd),cache=join(workspace,".🧬semio/🦑️repo/⚡️cache");
  return pinExecutableArtifact(resolve(executable),join(cache,"tools/native-executables",basename(executable)),"native-execution",{signal,leaseDirectory:join(workspace,".cache/framework/leases"),onWait:({elapsedMs})=>console.log(`[native] Waiting for executable publication (${elapsedMs}ms); Ctrl+C cancels`)});
}

/** 🏃️ Keeps the owning JavaScript event loop responsive while the native child runs. */
export async function runNativeBinary(executable: string, args: readonly string[], environment: NodeJS.ProcessEnv, cwd = getWorkspaceRoot(), signal: AbortSignal = new AbortController().signal): Promise<void> {
  const binary=await nativeExecutable(executable,cwd,signal);
  await runTool(binary, [...args], cwd, signal, false, nativeRunnerEnvironment(environment),repositoryCargoPreparationStorageV1(process.cwd()));
}

/** 🌐️ Starts one ready asset listener, runs the native consumer and closes all owned connections. */
export async function runNativeSession(executable: string, args: readonly string[], environment: NodeJS.ProcessEnv, cwd: string, signal: AbortSignal, startAssets?: () => Server): Promise<void> {
  signal.throwIfAborted();
  const server=startAssets?.();let failed=false;
  try {
    if (server && !server.listening) await once(server, "listening", { signal });
    const address = server?.address();
    if (server && (!address || typeof address === "string")) throw new Error("Native asset server needs a TCP address");
    const env = { ...environment, ...(address && typeof address !== "string" ? { SEMIO_ASSET_BASE_URL: `http://127.0.0.1:${address.port}` } : {}) };
    await runNativeBinary(executable, args, env, cwd, signal);
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

/** 🖥️ Runs completed native artifacts without compiling or invoking another Nx graph. */
class RunScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const {variant,profile,axes}=selection(args,true),repo=this.repoRoot;if(profile!==readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode)throw Error("Native run must match actual Original mode");
    await withPlaygroundSessionV1(repo,this.invocation,variant,async(_session,projection,control)=>{
    return withNativeRuntimeV1(repo,this.invocation,variant,control,async(runtime,_manifest,runtimeControl)=>{
    const catalog=projection.playgrounds;
    const row = catalog.find((entry: { variant: string }) => entry.variant === variant);
    if (!row) throw new Error(`Unknown native playground: ${variant}`);
    const controller = new AbortController(), cancel = () => controller.abort();
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try {
      const binary=playgroundNativeHostArtifactV1(repo,row.nativeHost,profile)??nativeRendererBinary(resolve(import.meta.dir,"../📦️packages/🦀️rust"),profile);
      if(!existsSync(binary))throw new Error(`Missing Nx native host artifact: ${binary}`);
      // 🧭️ An explicit `--flag` is spelled FIRST, because the binary's `arg_value` reads the first
      // occurrence; the playground row then supplies the per-server defaults (`app`, `brand`) the wgpu
      // serve injects as `<meta name="semio-*">` for the browser.
      await runNativeSession(binary, ["--plugin", variant, ...axes, ...(row.app ? ["--app", row.app] : []), ...(row.brand ? ["--brand", row.brand] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_PLUGIN_MODULES: runtime, SEMIO_RUNTIME_ASSETS: repo }, repo,createRequestedIssuedResourceControlV1(runtimeControl,0,controller.signal).signal,row.assets?.length ? () => createAssetHttpServerV1(repo, 0, row.assets, PLAYGROUND_ASSET_PROVIDERS_V1) : undefined);
    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
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
    if(!binary || !existsSync(binary))throw new Error("Missing Nx MCP owner artifact");
    const controller=new AbortController(),cancel=()=>controller.abort();
    process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
    try {await runTool(await nativeExecutable(binary,this.repoRoot,controller.signal),[transport,...args],this.repoRoot,controller.signal,false,process.env,repositoryCargoPreparationStorageV1(process.cwd()));}
    finally {process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);}
  }
}

/** ⚖️ Runs the native scale consumer with only its binary and fixture prerequisites. */
class ScaleScript extends BundleScript {
  async run([profile, ...args]: string[]): Promise<void> {
    if(profile!=="prototype"&&profile!=="release")throw Error("Exact Native scale mode required");nativeRuntimeDirectory(this.repoRoot,"scale",profile);
    if (!args.length || args.length % 2 !== 0 || args.some((value, index) => index % 2 === 0 && !["--scale", "--scale-wasm", "--shards", "--report"].includes(value))) throw new Error("Native scale accepts --scale, --scale-wasm, --shards and --report value pairs");
    for (const flag of ["--scale", "--scale-wasm", "--report"]) if (args.filter(value => value === flag).length !== 1) throw new Error(`Native scale requires exactly one ${flag}`);
    const report=args[args.indexOf("--report")+1];if(!report)throw Error("Finite scale report required");mkdirSync(dirname(resolve(this.repoRoot,report)),{recursive:true});
    const controller = new AbortController(), cancel = () => controller.abort();
    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
    try { await runNativeBinary(nativeRendererBinary(resolve(import.meta.dir, "../📦️packages/🦀️rust"), profile), args, process.env, this.repoRoot, controller.signal); }
    finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
  }
}

if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("run", RunScript).register("scale", ScaleScript).register("mcp",McpScript)).run(process.argv.slice(2), original));
