import { readNativeCallerAuthorityV1, withRepositoryExecutablePinV1 } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { withPlaygroundSessionV1 } from "../../../../../🔌️plugin/📇️registry/🎮️playground/🧭️session/👓️reading/🟦️.ts";
import { createIssuedResourceControlV1, createRequestedIssuedResourceControlV1, requireIssuedResourceControlInvocationV1, requireIssuedResourceControlRefinementV1, type ScriptInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { createAssetHttpServerV1 } from "../../../../../../../../🔨️modules/🖼️assets/📦️packages/🟦️typescript/🔍️resolver/🧭️dispatch/🟦️.ts";
import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "../../../../../🔌️plugin/📇️registry/🎮️playground/🖼️assets/🧩️composition/🟦️.ts";
import { playgroundNativeHostArtifactV1 } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts";
import { compilerAdmissionBudgetV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { runOwnedCommand } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import { checkResourceLeaseControl, type ResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { once } from "node:events";
import nativeLaunchDeclaration from "./🎟️launch/🔣️.json" with { type: "json" };
import { nativeLaunchEnvironmentV1, readNativeLaunchDeclarationV1 } from "./🎟️launch/🟦️.ts";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { Server } from "node:http";
import { dirname, isAbsolute, join, resolve } from "node:path";
import { nativeRendererBinary } from "../🏗️compiler/🦀️native/📜️script.ts";
import { nativeRuntimeDirectory, withNativeRuntimeV1 } from "./📦️modules/🟦️.ts";

const registryPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
export function nativeRunnerEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  return Object.fromEntries(Object.entries(source).filter(([key]) => {
    const normalized = key.toUpperCase();
    return !["S_USER", "VITE_S_USER", "S_HUB_URL"].includes(normalized) && !/TOKEN|SESSION|CREDENTIAL|BEARER|CAPABILITY|AUTHORIZATION|COOKIE/.test(normalized);
  }).concat([["SEMIO_DIRECT_CHILD_BENIGN", "preserved"]]));
}
export async function runNativeBinary(executable:string,args:readonly string[],environment:NodeJS.ProcessEnv,cwd:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);const capturedArgs=Object.freeze([...args]),capturedEnvironment=Object.freeze({...environment});const execute=async(binary:string,control:ResourceLeaseControl)=>{requireIssuedResourceControlInvocationV1(control,invocation);requireIssuedResourceControlRefinementV1(phaseControl,control);const budget=compilerAdmissionBudgetV1(control);await runOwnedCommand(binary,[...capturedArgs],cwd,"native:execution",budget,{env:nativeRunnerEnvironment(capturedEnvironment),signal:control.signal,onProgress:async()=>{checkResourceLeaseControl(control);await control.publish({resource:"native:execution",mode:"shared",elapsedMs:0});checkResourceLeaseControl(control);await control.yieldContinuation();checkResourceLeaseControl(control);}});checkResourceLeaseControl(control);};
 if(isAbsolute(executable))return withRepositoryExecutablePinV1({workspace:readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot,invocation,phaseControl,source:executable},execute);return execute(executable,phaseControl);
}
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
const BOOT_AXIS_FLAGS = ["--app", "--role", "--mode", "--example", "--brand", "--hub", "--user", "--data-dir"] as const;
function selection(args: string[], smoke = false): { variant: string; profile: "prototype" | "release"; axes: string[] } {
  const [variant, profile, ...rest] = args;
  if(!variant||variant.length>64||!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant)||!(profile==="prototype"||profile==="release"))throw new Error("Exact Native variant and mode required");
  const axes: string[] = [];
  for (let index = 0; index < rest.length; index++) {
    const flag = rest[index]!;
    if (smoke && flag === "--smoke") continue;
    if (!(BOOT_AXIS_FLAGS as readonly string[]).includes(flag) || rest[index + 1] === undefined) throw new Error(`Native runtime accepts a variant, profile, optional --smoke and ${BOOT_AXIS_FLAGS.join("/")} value pairs`);
    axes.push(flag, rest[++index]!);
  }
  return { variant, profile: profile as "prototype" | "release", axes };
}

/** 🏃️ Runs the `RunScript` command under the caller's roots and invocation. */
export async function runRunScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  const {variant,profile,axes}=selection(args,true),repo=context.repoRoot;if(profile!==readNativeCallerAuthorityV1(context.invocation.capabilities).original.buildMode)throw new Error("Native run must match actual Original mode");
  await withPlaygroundSessionV1(repo,context.invocation,variant,async(_session,projection,control)=>{
  return withNativeRuntimeV1(repo,context.invocation,variant,control,async(runtime,_manifest,runtimeControl)=>{
  const catalog=projection.playgrounds;
  const row = catalog.find((entry: { variant: string }) => entry.variant === variant);
  if (!row) throw new Error(`Unknown native playground: ${variant}`);
  const signal = context.invocation.control.signal;
  {
    const binary=playgroundNativeHostArtifactV1(repo,row.nativeHost,profile)??nativeRendererBinary(resolve(context.root,"../📦️packages/🦀️rust"),profile);
    if(!existsSync(binary))throw new Error(`Missing native host artifact: ${binary}`);
    await runNativeSession(binary, ["--plugin", variant, ...axes, ...(row.app ? ["--app", row.app] : []), ...(row.brand ? ["--brand", row.brand] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], nativeLaunchEnvironmentV1(context.invocation, { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_PLUGIN_MODULES: runtime, SEMIO_RUNTIME_ASSETS: repo }, readNativeLaunchDeclarationV1(nativeLaunchDeclaration), Date.now()), repo,context.invocation,createRequestedIssuedResourceControlV1(runtimeControl,0,signal),row.assets?.length ? () => createAssetHttpServerV1(repo, 0, row.assets, PLAYGROUND_ASSET_PROVIDERS_V1) : undefined);
  }
  });
  });
}

/** 🏃️ Runs the `McpScript` command under the caller's roots and invocation. */
export async function runMcpScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, [variant,profile,transport,...args]:string[]): Promise<void> {
  if(!variant||(profile!=="prototype"&&profile!=="release")||(transport!=="stdio"&&transport!=="http"))throw new Error("MCP owner selection requires a variant, exact profile and transport");
  const catalog=JSON.parse(readFileSync(join(context.repoRoot,registryPath,"🤖️generated/🎠️playgrounds.json"),"utf8"));
  const row=catalog.find((entry:{variant:string})=>entry.variant===variant);
  if(!row?.mcpHost)throw new Error(`Playground does not declare an MCP producer: ${variant}`);
  const binary=playgroundNativeHostArtifactV1(context.repoRoot,row.mcpHost,profile as "prototype"|"release");
  if(!binary || !existsSync(binary))throw new Error("Missing MCP owner artifact");
  const controller=new AbortController(),cancel=()=>controller.abort();
  const phaseControl=createIssuedResourceControlV1(context.invocation,"native:mcp",0,controller.signal);
  process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
  try {await withRepositoryExecutablePinV1({workspace:context.repoRoot,invocation:context.invocation,phaseControl,source:binary},async(executable,actualControl)=>{await runOwnedCommand(executable,[transport,...args],context.repoRoot,"native:mcp",compilerAdmissionBudgetV1(actualControl),{env:process.env,signal:actualControl.signal});checkResourceLeaseControl(actualControl);});}
  finally {process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);}
}

/** 🏃️ Runs the `ScaleScript` command under the caller's roots and invocation. */
export async function runScaleScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, [profile, ...args]:string[]): Promise<void> {
  const phaseControl=createIssuedResourceControlV1(context.invocation,"native:scale",0);
  if(profile!=="prototype"&&profile!=="release")throw new Error("Exact Native scale mode required");nativeRuntimeDirectory(context.repoRoot,"scale",profile);
  if (!args.length || args.length % 2 !== 0 || args.some((value, index) => index % 2 === 0 && !["--scale", "--scale-wasm", "--shards", "--report"].includes(value))) throw new Error("Native scale accepts --scale, --scale-wasm, --shards and --report value pairs");
  for (const flag of ["--scale", "--scale-wasm", "--report"]) if (args.filter(value => value === flag).length !== 1) throw new Error(`Native scale requires exactly one ${flag}`);
  const report=args[args.indexOf("--report")+1];if(!report)throw new Error("Finite scale report required");mkdirSync(dirname(resolve(context.repoRoot,report)),{recursive:true});
  const signal = context.invocation.control.signal;
  await runNativeBinary(nativeRendererBinary(resolve(context.root, "../📦️packages/🦀️rust"), profile), args, nativeLaunchEnvironmentV1(context.invocation, process.env, readNativeLaunchDeclarationV1(nativeLaunchDeclaration), Date.now()),context.repoRoot,context.invocation,createRequestedIssuedResourceControlV1(phaseControl,0,signal));
}
