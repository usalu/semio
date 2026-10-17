import {readNativeStorageReservationPolicyV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {readViteRuntimeChannelsV1,viteRuntimeEnvironmentV1,withoutViteRuntimeEnvironmentV1,type ViteRuntimeChannelV1} from "./📥️environment/🏃️runtime/🟦️.ts";
import {withViteOptimizerDrainV1} from "./🏗️compiler/💧️drain/🟦️.ts";
import {projectViteCommandEnvironmentV1,viteCommandChildEnvironmentV1} from "./📥️environment/🟦️.ts";
import { compilerAdmissionBudgetV1, withCompilerCapacityScopeV1, type CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {scriptResourceProcessEnvironmentV1,requireIssuedResourceControlInvocationV1,createRequestedIssuedResourceControlV1,withRequestedIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { buildModeEnvironment, readBuildMode, type BuildMode } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { withCompilerState } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗃️state/🟦️.ts";
import {withViteStorageV1,viteWriterResourceV1} from "./💾️storage/🟦️.ts";
import {acquireResourceLease,checkResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {withRepositoryCacheStoreV1} from "../💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import storageSchema from "./💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import {repositoryCacheBudgetV1} from "../💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import { readFileSync, existsSync } from "node:fs";
import { rm, cp, lstat, mkdir } from "node:fs/promises";
import { createServer as createHttpServer } from "node:http";
import { createRequire } from "node:module";
import type { Socket } from "node:net";
import { dirname, join, resolve } from "node:path";
import { stageRepositoryArtifacts,withRepositoryArtifactDestinationV1 } from "../📦️artifacts/🟦️.ts";
import { collectArtifactFiles } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { parseServiceSession, SERVICE_READY_ENDPOINT, type ServiceSession } from "./🧾️session/🟦️.ts";
import {withViteArtifactInputsV1,readViteArtifactInputsV1} from "./📥️inputs/🟦️.ts";
import type {RepositoryArtifactRequestV1} from "../📦️artifacts/👓️reading/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";

/** 🎚️ Binds the selected intent and workspace to the actual caller before opening any cache. */
function requireViteBuildIntentV1(invocation:ScriptInvocation,mode:BuildMode,workspace?:string):string{const authority=readNativeCallerAuthorityV1(invocation.capabilities);if(mode!==authority.original.buildMode||workspace!==undefined&&resolve(workspace)!==authority.repositoryRoot)throw Error("Vite intent must match its Original mode and workspace");return authority.repositoryRoot;}

export {viteCacheDirectory} from "./📂️paths/🟦️.ts";

export type ViteArtifactBuild = {
  readonly invocation: ScriptInvocation;
  readonly control: CompilerAdmissionControl;
  readonly artifactInputs?: readonly RepositoryArtifactRequestV1[];
  readonly root: string;
  readonly workspace: string;
  readonly config: string;
  readonly owner: string;
  readonly mode?: BuildMode;
  readonly args?: readonly string[];
  readonly timeoutMs?: number;
  readonly signal?: AbortSignal;
  readonly environment?: Readonly<Record<string, string | undefined>>;
};

/** 🌐️ Bundles one site and publishes its complete deliverable only after Vite succeeds. */
export async function buildViteArtifact(options: ViteArtifactBuild): Promise<string> {
  requireIssuedResourceControlInvocationV1(options.control,options.invocation);const authoredEnvironment=withoutViteRuntimeEnvironmentV1({...options.environment},process.platform),phase = createRequestedIssuedResourceControlV1(options.control,options.timeoutMs ?? 600_000,options.signal);
  phase.signal.throwIfAborted();
  if(options.args?.length)throw Error("Vite build arguments must be authored in the owned configuration");
  const mode = options.mode ?? readBuildMode(options.environment?.SEMIO_BUILD_MODE ?? readNativeCallerAuthorityV1(options.invocation.capabilities).original.buildMode);requireViteBuildIntentV1(options.invocation,mode,options.workspace);const require = createRequire(join(options.workspace, "package.json"));
  return withRepositoryCacheStoreV1(options.workspace,"dependencies",options.invocation,phase,(_dependencies,dependencyPhase)=>withViteStorageV1(options.workspace,mode,options.owner,options.invocation,dependencyPhase,async(store,layout,temporary,storePhase)=>{
    if(options.artifactInputs?.some(request=>request.path===layout.output))throw Error("Vite output cannot also be an artifact input");
    return withViteArtifactInputsV1({workspace:options.workspace,invocation:options.invocation,store,control:storePhase,requests:options.artifactInputs??[]},async control=>{
    const state = { root: layout.state, owner: options.owner, compiler: `vite:${require("vite/package.json").version}:bun:${process.versions.bun}`, configuration: JSON.stringify([options.config, readFileSync(options.config, "utf8")]), mode };
    await withCompilerState({capacity:{kind:"whole-host"}, ...state, environment:{...withoutViteRuntimeEnvironmentV1({...store.environment,...authoredEnvironment},process.platform),TMPDIR:store.environment.TMPDIR,TMP:store.environment.TMP,TEMP:store.environment.TEMP}, control, storage:store.storage,storageControl:store.control, files: ["deps/_metadata.json"] }, async (directory, compilerEnvironment,control) => {
      const writer=await acquireResourceLease({directory:readNativeStorageReservationPolicyV1(store.storage,store.control).leaseDirectory,resource:viteWriterResourceV1(options.owner),mode:"exclusive",signal:control.signal,control});try{
      requireNativeStoragePathV1(store.root,directory);await store.recheck();const cli = join(dirname(require.resolve("vite/package.json")), "bin/vite.js");
      console.log(`Bundling ${options.owner}`);
      try {
        await runOwnedCommand(process.execPath, [cli, "build", "--mode", mode, "--minify", mode === "release" ? "esbuild" : "false", ...(mode === "prototype" ? ["--sourcemap"] : []), "--config", options.config, "--configLoader", "runner", "--outDir", temporary, "--emptyOutDir", ...(options.args ?? [])], options.root, `vite:${options.owner}`, compilerAdmissionBudgetV1(control), { env: { ...buildModeEnvironment(mode, compilerEnvironment), SEMIO_VITE_CACHE_DIRECTORY: directory }, signal: control.signal });
      } catch (error) { control.signal.throwIfAborted(); throw new Error(`Vite build failed for ${options.owner}`, { cause: error }); }
      const files = await collectArtifactFiles(temporary, control.signal);let bytes=0;for(const path of files.values()){requireNativeStoragePathV1(store.root,path);bytes+=(await lstat(path)).size;}if(files.size>storageSchema.definitions.Protocol.const.maximumOutputFiles||bytes>storageSchema.definitions.Protocol.const.maximumOutputBytes)throw Error("Vite deliverable exceeds finite output boundary");await store.recheck();
      await withRepositoryArtifactDestinationV1({workspace:options.workspace,invocation:options.invocation,storage:store.storage,control:store.control,phaseControl:control,directory:layout.output},destination=>stageRepositoryArtifacts(destination,options.owner,files));
      }finally{writer.release();}
    });return layout.output;
    });
  }));
}

export type ViteService = {
  readonly invocation: ScriptInvocation;
  readonly control: CompilerAdmissionControl;
  readonly artifactInputs?: readonly RepositoryArtifactRequestV1[];
  readonly root: string;
  readonly config: string;
  readonly configLoader?: "runner" | "native";
  readonly host: string;
  readonly port: number;
  readonly signal: AbortSignal;
  readonly ready: (url: string) => void | Promise<void>;
  readonly session?: ServiceSession;
  readonly mode?: BuildMode;
};

/** 🖥️ Describes one isolated compiler process without mutating another live service's environment. */
export interface ViteCommandServiceV1{readonly runtimeChannels:readonly ViteRuntimeChannelV1[];readonly invocation:ScriptInvocation;readonly control:CompilerAdmissionControl;readonly root:string;readonly config:string;readonly mode:BuildMode;readonly host:string;readonly port:number;readonly signal:AbortSignal;readonly environment:Readonly<Record<string,string|undefined>>;readonly artifactInputs?:readonly RepositoryArtifactRequestV1[];readonly strictPort?:boolean}

/** 🧵️ Holds paid compiler, exact input readers and bounded cache until its isolated Vite child joins. */
export async function serveViteCommandV1(options:ViteCommandServiceV1):Promise<void>{
 requireIssuedResourceControlInvocationV1(options.control,options.invocation);const invocation=options.invocation,signal=options.signal,phase=createRequestedIssuedResourceControlV1(options.control,0,signal),mode=options.mode,root=options.root,config=options.config,host=options.host,port=options.port,strictPort=options.strictPort!==false,workspace=requireViteBuildIntentV1(invocation,mode),authority=readNativeCallerAuthorityV1(invocation.capabilities);if(!Number.isSafeInteger(port)||port<1||port>65535||!host||host.length>253||host.includes("\0"))throw Error("Explicit Vite command listener required");const owner="vite-command-service:"+root,artifactInputs=readViteArtifactInputsV1(options.artifactInputs??[]),runtimeChannels=readViteRuntimeChannelsV1(options.runtimeChannels),projection=projectViteCommandEnvironmentV1({version:1,mode,platform:process.platform,layers:[{...process.env},{...options.environment}]}),environment=projection.environment;
 return withRepositoryCacheStoreV1(workspace,"dependencies",invocation,phase,(_dependencies,dependencyPhase)=>withViteStorageV1(workspace,mode,owner,invocation,dependencyPhase,(store,layout,_temporary,storePhase)=>withViteArtifactInputsV1({workspace,invocation,store,control:storePhase,requests:artifactInputs},async control=>{
  const require=createRequire(join(workspace,"package.json")),cli=join(dirname(require.resolve("vite/package.json")),"bin/vite.js"),state={root:layout.state,owner,compiler:`vite:${require("vite/package.json").version}:bun:${process.versions.bun}`,configuration:JSON.stringify([config,readFileSync(config,"utf8"),projection.digest]),mode};
  await withCompilerState({capacity:{kind:"whole-host"},...state,environment,control,storage:store.storage,storageControl:store.control,files:["deps/_metadata.json"]},async(directory,compilerEnvironment,control)=>{requireNativeStoragePathV1(store.root,directory);const child=viteCommandChildEnvironmentV1({version:1,mode,platform:process.platform,directory,environment:compilerEnvironment});requireNativeStoragePathV1(store.root,child.temporary);await store.recheck();await mkdir(child.temporary,{recursive:true});requireNativeStoragePathV1(store.root,child.temporary);await store.recheck();await runOwnedCommand(process.execPath,[cli,"--mode",mode,"--config",config,"--configLoader","runner","--host",host,"--port",String(port),...(strictPort?["--strictPort"]:[]),"--clearScreen","false"],root,owner,compilerAdmissionBudgetV1(control),{env:scriptResourceProcessEnvironmentV1(control,{...child.environment,...viteRuntimeEnvironmentV1(runtimeChannels)}),signal:control.signal,output:authority.transport,onProgress:async()=>{await control.publish({resource:owner,mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}});await store.recheck();});
 })));
}

/** 🖥️ Owns one Vite listener through readiness and cancellation without reusing ambient services. */
export async function serveVite(options: ViteService): Promise<void> {
  requireIssuedResourceControlInvocationV1(options.control,options.invocation);const phase = createRequestedIssuedResourceControlV1(options.control,0,options.signal);
  phase.signal.throwIfAborted();
  if (!Number.isInteger(options.port) || options.port < 0 || options.port > 65535) throw new Error("Invalid Vite service port");
  if(options.configLoader!==undefined&&!["runner","native"].includes(options.configLoader))throw Error("Vite service config loader must preserve bounded storage");
  const session = options.session ? parseServiceSession(options.session) : undefined;
  const mode=readBuildMode(options.mode??readNativeCallerAuthorityV1(options.invocation.capabilities).original.buildMode),workspace=requireViteBuildIntentV1(options.invocation,mode),owner=`vite-service:${options.root}`;
  return withRepositoryCacheStoreV1(workspace,"dependencies",options.invocation,phase,(_dependencies,dependencyPhase)=>withViteStorageV1(workspace,mode,owner,options.invocation,dependencyPhase,async(store,layout,cache,storePhase)=>withViteArtifactInputsV1({workspace,invocation:options.invocation,store,control:storePhase,requests:options.artifactInputs??[]},async control=>{
  const { createServer } = await import("vite");
  const require = createRequire(join(workspace,"package.json"));
  const state = { root: layout.state, owner, compiler: `vite:${require("vite/package.json").version}:bun:${process.versions.bun}`, configuration: JSON.stringify([options.config, readFileSync(options.config, "utf8")]), mode };
  try { await withCompilerState({ ...state, environment:store.environment, capacity: null, control, storage:store.storage,storageControl:store.control, files: ["snapshot/.semio-artifact.json"] }, async directory => { if (existsSync(join(directory, "snapshot"))) await cp(join(directory, "snapshot"), cache, { recursive: true }); await rm(join(cache, ".semio-artifact.json"), { force: true }); }); }
  catch (error) { await rm(cache, { recursive: true, force: true }); throw error; }
  const listener = createHttpServer(), sockets = new Set<Socket>();
  listener.on("connection", socket => { sockets.add(socket); socket.once("close", () => sockets.delete(socket)); });
  let server: Awaited<ReturnType<typeof createServer>> | undefined;
  const failures:unknown[]=[],pending=new Set<Promise<unknown>>();let accepting=true,interrupt!:()=>void;const interrupted=new Promise<void>(done=>{interrupt=done;});
  try {
    await withCompilerState({capacity:{kind:"whole-host"}, ...state, environment:store.environment, control, storage:store.storage,storageControl:store.control, files: ["snapshot/.semio-artifact.json"] }, async (_directory,_environment,control) => {
    control.signal.throwIfAborted();server = await createServer({ mode, dev: { preTransformRequests: false }, cacheDir: cache, root: options.root, configFile: options.config, configLoader: options.configLoader ?? "runner", server: { preTransformRequests: false, host: options.host, port: options.port, strictPort: true, open: false, middlewareMode: { server: listener }, hmr: { server: listener } }, plugins: session ? [{
    name: "semio-service-readiness",
    configureServer(server) {
      server.middlewares.use((request, response, next) => {
        if (request.url !== SERVICE_READY_ENDPOINT || request.method !== "GET") return next();
        response.setHeader("content-type", "application/json");
        response.setHeader("cache-control", "no-store");
        response.end(JSON.stringify(session));
      });
    },
    }] : [] });
    await Promise.all(Object.values(server.environments).map(environment => environment.depsOptimizer?.scanProcessing));checkResourceLeaseControl(control);
    });
    if (!server) throw Error("Vite service initialization did not produce a server");
    for (const environment of Object.values(server.environments)) {
      environment.config.dev.preTransformRequests = false;
      const transform = environment.transformRequest.bind(environment), warmup = environment.warmupRequest.bind(environment);
      const admitted = <T>(operation: () => Promise<T>): Promise<T> => {
        if(!accepting||pending.size>=storageSchema.definitions.Protocol.const.maximumPendingTransforms)throw Error("Vite accepted transform admission closed or capacity exceeded");
        const task=withRequestedIssuedResourceControlV1(control,600_000,async phase=>{
        return withCompilerCapacityScopeV1({request:{kind:"whole-host"},directory:readNativeStorageReservationPolicyV1(store.storage,store.control).leaseDirectory,control:phase,environment:store.environment},async(_,phase)=>withViteOptimizerDrainV1(async()=>{phase.signal.throwIfAborted();return operation();},()=>Object.values(environment.depsOptimizer?.metadata.discovered ?? {}).map(dependency=>dependency.processing),()=>{checkResourceLeaseControl(phase);}));
        });pending.add(task);void task.then(()=>{pending.delete(task);},error=>{pending.delete(task);accepting=false;if(!failures.includes(error))failures.push(error);interrupt();});return task;
      };
      environment.transformRequest = (...args) => {
        if(!accepting)throw Error("Vite accepted transform admission closed");
        const cached = environment.moduleGraph.urlToModuleMap.get(args[0]);
        control.signal.throwIfAborted();
        return cached?.transformResult ? Promise.resolve(cached.transformResult) : admitted(() => transform(...args));
      };
      environment.warmupRequest = (...args) => admitted(() => warmup(...args));
    }
    control.signal.throwIfAborted();
    listener.on("request", (request, response) => server!.middlewares(request, response));
    await new Promise<void>((resolve, reject) => {
      listener.once("error", reject);
      listener.listen(options.port, options.host, () => { listener.removeListener("error", reject); resolve(); });
    });
    control.signal.throwIfAborted();
    const address = listener.address();
    if (!address || typeof address === "string") throw new Error("Vite did not bind a TCP listener");
    const host = options.host === "0.0.0.0" ? "127.0.0.1" : options.host.includes(":") ? `[${options.host}]` : options.host;
    await options.ready(`http://${host}:${address.port}/`);
    await Promise.race([interrupted,new Promise<void>(resolve => {
      if (control.signal.aborted) resolve();
      else control.signal.addEventListener("abort", () => resolve(), { once: true });
    })]);
  }catch(error){failures.push(error);}finally{
    accepting=false;
    listener.closeAllConnections();
    const closed = new Promise<void>(resolve => listener.close(() => resolve()));
    for (const socket of sockets) socket.destroy();
    while(pending.size)await Promise.allSettled([...pending]);
    if(server){const drained=await Promise.allSettled(Object.values(server.environments).map(async environment=>{await environment.waitForRequestsIdle();await environment.depsOptimizer?.scanProcessing;}));for(const row of drained)if(row.status==="rejected")failures.push(row.reason);}
    try {
      const stopped=await Promise.allSettled([closed,Promise.resolve().then(()=>server?.close())]);for(const row of stopped)if(row.status==="rejected")failures.push(row.reason);
      if (!failures.length && server && !control.signal.aborted) await withRequestedIssuedResourceControlV1(control,30000,snapshotControl=>withCompilerState({ ...state, capacity: null, control:snapshotControl, storage:store.storage,storageControl:store.control, files: ["snapshot/.semio-artifact.json"] }, async (directory,_environment,actualPhase) => {
        const files = await collectArtifactFiles(cache,actualPhase.signal);
        requireNativeStoragePathV1(store.root,directory);await store.recheck();
        await withRepositoryArtifactDestinationV1({workspace,invocation:options.invocation,storage:store.storage,control:store.control,phaseControl:actualPhase,directory:join(directory,"snapshot")},destination=>stageRepositoryArtifacts(destination,"vite-service:optimizer",files));
      }));
    }catch(error){failures.push(error);}finally{try{await rm(cache,{recursive:true,force:true});}catch(error){failures.push(error);}}
    if(failures.length>1)throw new AggregateError(failures,"Vite request drainage, listener close or cache cleanup failed");if(failures.length)throw failures[0];
  }
  })));
}
