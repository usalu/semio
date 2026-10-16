#!/usr/bin/env bun
import {selectedGraphSdkPortsV1} from "../🕸️graph/🔏️seal/🎯️selection/🔌️sdk/🟦️.ts";
import{createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNxBootstrapCapabilitiesV1,readNxBootstrapAuthoringCapabilitiesV1,nxBootstrapJavascriptEnvironmentV1} from "./📥️caller/🟦️.ts";
import{isToolingHeadroomRefusalV1,type ToolingHeadroomRefusalV1}from"./🛠️tools/🧹️retention/📏️headroom/🟦️.ts";
import {writeSelectedGraphHandoffV1} from "../🕸️graph/🔏️seal/🎯️selection/📥️handoff/🟦️.ts";
import {withNxCollectionAdmissionStoreV1,NX_COLLECTION_ADMISSION_MAXIMUM_BYTES_V1} from "./📥️admission/💾️storage/🟦️.ts";
import {repositoryCacheBudgetV1} from "../💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import {withRepositoryToolingStoreV1,type RepositoryToolingStoreV1} from "../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {nativeNxChildEnvironmentV1,issueSelectedNxCallerEnvelopeV1} from "../📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts";
import {resolveBuildSelectionV1} from "../../🏗️build/🧭️invocation/🟦️.ts";
import {withRepositoryNxStorageV1,boundedNxNamespaceV1,boundedNxSocketNamespaceV1,type BoundedNxNamespaceV1} from "../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { createScriptProcessEnvelope, readScriptProcessEnvelope, scriptProcessEnvironment, SCRIPT_PROCESS_INVOCATION_ENV, withScriptProcessEnvelope } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { spawn as spawnNxProcess, spawnSync as stopNxProcessTree } from "node:child_process";
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, rmSync, writeSync } from "node:fs";
import { lstat, mkdir, readFile, readdir, rename, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { createHash, randomUUID } from "node:crypto";
import { createInterface } from "node:readline";
import { basename, dirname, isAbsolute, join, resolve, relative, sep, win32, posix } from "node:path";
import { checkScriptInvocation, Script, ScriptRouter, readScriptPolicy, scriptInvocationBudget, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { orchestratorBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { devToolingEnv, semioBuildParallelFlag } from "../../🏃️process/🌿️environment/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

import { boundedAdmissionWalkV1 as boundedWalk, gitCollectionManifests, moduleCollectionAncestry } from "./📥️admission/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {readBuildMode} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { resolveNxBuildMode } from "./🎚️mode/🟦️.ts";
import { nxCacheEnvironmentV1 } from "./💾️budget/🟦️.ts";
import {withNxToolingUseV1,nxToolingUseStorageV1} from "./🛠️tools/📥️use/🟦️.ts";

const WORKSPACE_ROOT = getWorkspaceRoot();

/** 🥖️ Carries the acquired Bun identity into every Nx producer and source watcher. */
export function pinnedNxRuntimeEnvironment(source:NodeJS.ProcessEnv,bun:string,platform:NodeJS.Platform=process.platform):NodeJS.ProcessEnv {
  const paths=platform==="win32"?win32:posix;
  if(!paths.isAbsolute(bun))throw new Error("Nx Bun runtime must be absolute");
  const environment={...source},keys=Object.keys(source).filter(key=>platform==="win32"?key.toUpperCase()==="PATH":key==="PATH"),key=keys.find(value=>value==="PATH")??keys[0]??"PATH",previous=source[key]??"";
  for(const candidate of keys)delete environment[candidate];
  const directory=paths.dirname(bun),entries=previous.split(paths.delimiter).filter(entry=>entry && (platform==="win32"?entry.toLowerCase()!==directory.toLowerCase():entry!==directory));
  environment[key]=[directory,...entries].join(paths.delimiter);
  return environment;
}

/** 🛠️ Loads the owner of the current immutable Nx recipe and patch identity. */
function nxBootstrapServices(): typeof import("./🛠️tools/📜️script.ts") {
  return createRequire(import.meta.url)("./🛠️tools/📜️script.ts");
}

export {publishBuildBootstrapSourcesV1 as publishBootstrapSources} from "../../🏗️build/🚀️bootstrap/📜️script.ts";
import {publishBuildBootstrapSourcesV1 as publishBootstrapSources} from "../../🏗️build/🚀️bootstrap/📜️script.ts";

/** 📂️ One directory the admission walk descended into: its identity stamp, the child directories it queues and the admitted manifest names it holds. */
type NxAdmissionDirectory = { readonly dev: number; readonly ino: number; readonly mtimeMs: number; readonly ctimeMs: number; readonly children: readonly string[]; readonly manifests: readonly string[] };

/** 🗄️ The admission snapshot persisted between invocations, keyed by the policy that produced it. */
type NxAdmissionSnapshot = { readonly policy: string; readonly directories: Readonly<Record<string, NxAdmissionDirectory>> };

/** ⏱️ Directory stamps younger than this at snapshot time are re-read next time: a later entry change within one coarse timestamp tick would otherwise keep the same mtime (racy-git rule). */
const NX_ADMISSION_RACY_MS = 2000;

/** 🚦️ Concurrent directory reads in flight during the admission walk. */
const NX_ADMISSION_CONCURRENCY = 64;

/** 🗂️ Where one workspace's admission snapshot lives: inside the excluded repository cache, so writing it never perturbs the walk. */
function nxAdmissionSnapshotPath(root: string,directory:string): string {
  return join(directory, `${createHash("sha256").update(root).digest("hex").slice(0, 16)}.json`);
}

/** 📖️ Reads the previous admission snapshot for `policy`, or none when absent, unreadable or produced by another policy. */
async function readNxAdmissionSnapshot(path: string, policy: string): Promise<NxAdmissionSnapshot["directories"]> {
  try {
    const state=await lstat(path);if(!state.isFile()||state.isSymbolicLink()||state.nlink!==1||state.size>NX_COLLECTION_ADMISSION_MAXIMUM_BYTES_V1)throw Error("Bounded collection admission snapshot required");
    const snapshot = JSON.parse(await readFile(path, "utf8")) as NxAdmissionSnapshot;
    return snapshot.policy === policy && snapshot.directories && typeof snapshot.directories === "object" ? snapshot.directories : {};
  } catch { return {}; }
}

/** 💾️ Atomically replaces the admission snapshot; a concurrent invocation's write wins whole or loses whole. */
async function writeNxAdmissionSnapshot(path: string, snapshot: NxAdmissionSnapshot): Promise<void> {
  await mkdir(dirname(path), { recursive: true });
  const temporary = `${path}.${process.pid}.${randomUUID()}.tmp`;
  const source=JSON.stringify(snapshot);if(Buffer.byteLength(source)>NX_COLLECTION_ADMISSION_MAXIMUM_BYTES_V1)throw Error("Bounded collection admission snapshot required");
  try{await writeFile(temporary,source,{flag:"wx"});await rename(temporary,path);}finally{rmSync(temporary,{force:true});}
}

/** 🫳️ Re-proves actual directory identity before the original manifest inventory is published. */
export async function verifyNxAdmissionDirectoriesV1(root:string,visited:readonly {path:string;device:number;inode:number}[],admitted:readonly {path:string}[],current:Record<string,NxAdmissionDirectory>,signal?:AbortSignal):Promise<void>{
  await boundedWalk(visited, NX_ADMISSION_CONCURRENCY, async (directory) => {
    signal?.throwIfAborted();
    const now = await lstat(directory.path).catch(error=>{
      if((error as NodeJS.ErrnoException).code!=="ENOENT"||directory.path===root)throw error;
      if(admitted.some(file=>{const path=relative(directory.path,resolve(root,file.path));return path!==".."&&!path.startsWith(".."+sep)&&!isAbsolute(path);}))throw error;
      delete current[relative(root,directory.path).split(sep).join("/")];
      return undefined;
    });
    if(now===undefined)return;
    if (!now.isDirectory() || now.isSymbolicLink() || now.dev !== directory.device || now.ino !== directory.inode) throw new Error("Nx current admission requires unchanged real directory custody");
  });
}

/** 🛡️ Publishes exact real project exceptions before Nx core can admit testing collections. The walk is parallel and incremental: a directory
 * whose identity stamp (dev, inode, mtime, ctime) matches the previous snapshot reuses its recorded children and manifests instead of being
 * re-read, because adding, removing or renaming an entry always restamps its directory; every visited directory and admitted manifest is
 * still re-proven real and unchanged before publication. */
export async function publishNxCollectionBoundary(root:string,invocation:ScriptInvocation,progress?:(directories:number)=>void):Promise<void>{
 checkScriptInvocation(invocation);return withNxCollectionAdmissionStoreV1(root,invocation,directory=>publishNxCollectionBoundaryV1(root,directory,invocation.control.signal,progress));
}

async function publishNxCollectionBoundaryV1(root: string,directory:string, signal?: AbortSignal, progress?: (directories: number) => void): Promise<void> {
  const profileStarted=performance.now(),profile=process.env.SEMIO_BUILD_PROFILE_INPUTS==="1";
  signal?.throwIfAborted();
  if (!isAbsolute(root) || root.split(/[\\/]/u).some(part => part === "." || part === "..")) throw new Error("Nx collection publication requires raw absolute real ancestry");
  const real = async (path: string): Promise<void> => {
    for (let current = path; ; current = dirname(current)) {
      const value = await lstat(current);
      if (!value.isDirectory() || value.isSymbolicLink()) throw new Error("Nx collection publication requires real directory ancestry");
      if (current === dirname(current)) return;
    }
  };
  await real(root);
  const policyText = await readFile(join(import.meta.dirname, "../🔣️policy.json"), "utf8"), ownershipText = await readFile(join(import.meta.dirname, "../../🗂️workspaces/🔣️policy.json"), "utf8");
  const policyRead=performance.now();
  const policy = JSON.parse(policyText) as { generatedDirectories: string[]; manifestBasenames: string[] };
  const ownership = JSON.parse(ownershipText) as { collections: { names: string[]; moduleMember: string } };
  const { names, moduleMember } = ownership.collections;
  const { manifestBasenames } = policy;
  const target = join(root, ".nxignore");
  const original = async (): Promise<string> => {
    try { const value = await lstat(target); if (!value.isFile() || value.isSymbolicLink()) throw new Error("Nx ignore authority must be a real file"); return await readFile(target, "utf8"); }
    catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return ""; throw error; }
  };
  const before = await original(), admitted: { path: string; device: number; inode: number }[] = [];
  const visited: { path: string; device: number; inode: number }[] = [];
  const excluded = new Set([...policy.generatedDirectories, ".🧬semio", "🤖️generated", "pkg"]);
  const snapshotPath = nxAdmissionSnapshotPath(root,directory), snapshotPolicy = createHash("sha256").update(policyText).update("\0").update(ownershipText).digest("hex");
  const policyParsed=performance.now();
  const current: Record<string, NxAdmissionDirectory> = {}, startedMs = Date.now();
  const insideCollection = (parts: readonly string[]): boolean => moduleCollectionAncestry(parts, ownership.collections).inside;
  let directories = 0, reused = 0;
  const inventoryStarted=performance.now();
  const inventory = await gitCollectionManifests(root, policy, ownership.collections, signal),inventoryRead=performance.now();
  const previous=inventory===undefined?await readNxAdmissionSnapshot(snapshotPath,snapshotPolicy):{},snapshotRead=performance.now();
  if (inventory !== undefined) {
    for (const path of inventory) {
      signal?.throwIfAborted();
      await real(dirname(join(root, path)));
      const file = await lstat(join(root, path));
      if (!file.isFile() || file.isSymbolicLink()) throw new Error("Nx admitted manifest requires a real file");
      admitted.push({ path, device: file.dev, inode: file.ino });
    }
    console.log(`[nx] Admitted ${inventory.length} structural collection manifests`);
  } else await boundedWalk<readonly string[]>([[]], NX_ADMISSION_CONCURRENCY, async (parts, enqueue) => {
    signal?.throwIfAborted();
    progress?.(++directories);
    signal?.throwIfAborted();
    const directory = join(root, ...parts), stamp = await lstat(directory), key = parts.join("/");
    if (!stamp.isDirectory() || stamp.isSymbolicLink()) throw new Error("Nx queued owner requires a real directory");
    visited.push({ path: directory, device: stamp.dev, inode: stamp.ino });
    const cached = previous[key];
    let children: string[], manifests: string[];
    if (cached && cached.dev === stamp.dev && cached.ino === stamp.ino && cached.mtimeMs === stamp.mtimeMs && cached.ctimeMs === stamp.ctimeMs) {
      ({ children, manifests } = { children: [...cached.children], manifests: [...cached.manifests] });
      reused++;
    } else {
      children = []; manifests = [];
      const collection = insideCollection(parts);
      for (const entry of await readdir(directory, { withFileTypes: true })) {
        signal?.throwIfAborted();
        if (entry.isSymbolicLink()) continue;
        const next = [...parts, entry.name];
        if (moduleCollectionAncestry(next, ownership.collections).invalid) continue;
        if (entry.isDirectory()) { if (!excluded.has(entry.name)) children.push(entry.name); continue; }
        if (entry.isFile() && collection && manifestBasenames.some(name => name === "*.patch" ? entry.name.endsWith(".patch") : entry.name === name)) manifests.push(entry.name);
      }
    }
    if (stamp.mtimeMs < startedMs - NX_ADMISSION_RACY_MS && stamp.ctimeMs < startedMs - NX_ADMISSION_RACY_MS) current[key] = { dev: stamp.dev, ino: stamp.ino, mtimeMs: stamp.mtimeMs, ctimeMs: stamp.ctimeMs, children, manifests };
    for (const name of manifests) {
      const file = await lstat(join(directory, name));
      if (!file.isFile() || file.isSymbolicLink()) throw new Error("Nx admitted manifest requires a real file");
      admitted.push({ path: [...parts, name].join("/"), device: file.dev, inode: file.ino });
    }
    for (const name of children) enqueue([...parts, name]);
  });
  const discoveryFinished=performance.now();
  await verifyNxAdmissionDirectoriesV1(root,visited,admitted,current,signal);
  for (const file of admitted) {
    signal?.throwIfAborted();
    await real(dirname(join(root, file.path)));
    const now = await lstat(join(root, file.path));
    if (!now.isFile() || now.isSymbolicLink() || now.dev !== file.device || now.ino !== file.inode) throw new Error("Nx current admission requires unchanged real manifest custody");
  }
  const custodyFinished=performance.now();
  const begin = "# 🧫️ Canonical Example Collection Admission", end = "# 🧫️ End Example Collection Admission";
  const start = before.indexOf(begin), finish = before.indexOf(end);
  if ((start < 0) !== (finish < 0) || (start >= 0 && finish < start)) throw new Error("Nx ignore publication requires a complete owned section");
  const escape = (path: string): string => path.replace(/[\\*?\[\]#!]/g, "\\$&");
  const masks = names.flatMap(name => manifestBasenames.map(file => `**/${name}/**/${file}`));
  const section = [begin, ...masks, ...admitted.sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path))).map(file => `!/${escape(file.path)}`), end].join("\n") + "\n";
  const result = start < 0 ? before + (before && !before.endsWith("\n") ? "\n" : "") + section : before.slice(0, start) + section + before.slice(finish + end.length).replace(/^\r?\n/, "");
  signal?.throwIfAborted();
  await real(root);
  if (await original() !== before) throw new Error("Nx ignore authority changed during current owner admission");
  signal?.throwIfAborted();
  if (result !== before) {
    if(Buffer.byteLength(result)>524288)throw Error("Nx source configuration exceeds finite publication capacity");
    const temporary=join(repositoryCacheBudgetV1(root).controlDirectory,"nxignore.tmp");
    if(existsSync(temporary)){const stale=await lstat(temporary);if(!stale.isFile()||stale.isSymbolicLink()||stale.nlink!==1||stale.size>524288)throw Error("Owned Nx configuration staging required");rmSync(temporary);}
    await mkdir(dirname(temporary), { recursive: true });
    try {
      if(Buffer.byteLength(result)>524288)throw Error("Bounded Nx ignore control file required");
      await writeFile(temporary, result,{flag:"wx"});
      signal?.throwIfAborted();
      if (await original() !== before) throw new Error("Nx ignore authority changed before atomic publication");
      await rename(temporary, target);
    } finally { rmSync(temporary, { force: true }); }
  }
  if (inventory === undefined && (reused !== directories || Object.keys(previous).length !== directories)) await writeNxAdmissionSnapshot(snapshotPath, { policy: snapshotPolicy, directories: current });
  if(profile)console.log("[DEBUG] Nx workspace catalog admission "+JSON.stringify({policyBytes:Buffer.byteLength(policyText)+Buffer.byteLength(ownershipText),manifests:admitted.length,directories,reused,inventory:inventory===undefined?"physical-walk":"git",policyReadMilliseconds:policyRead-profileStarted,policyParseHashMilliseconds:policyParsed-policyRead,snapshotReadMilliseconds:snapshotRead-inventoryRead,inventoryMilliseconds:inventoryRead-inventoryStarted,discoveryMilliseconds:discoveryFinished-snapshotRead,custodyMilliseconds:custodyFinished-discoveryFinished,publicationMilliseconds:performance.now()-custodyFinished,totalMilliseconds:performance.now()-profileStarted}));
}

/** 🏗️ Projects portable process controls into the selected Nx implementation. */
export function nxBuildEnvironmentV1(root:string,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv {
 const env:NodeJS.ProcessEnv={...environment,SEMIO_BUILD_TOOL:"nx",SEMIO_WORKSPACE_ROOT:root,NX_WORKSPACE_ROOT:root,NX_WORKSPACE_ROOT_PATH:root};
 env.NX_TUI=environment.SEMIO_BUILD_TERMINAL??environment.NX_TUI??"false";
 env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT=environment.SEMIO_BUILD_DYNAMIC_OUTPUT??environment.NX_TASKS_RUNNER_DYNAMIC_OUTPUT??"false";
 env.NX_VERBOSE_LOGGING=environment.SEMIO_BUILD_VERBOSE??environment.NX_VERBOSE_LOGGING??"false";
 env.NX_PARALLEL=environment.SEMIO_BUILD_PARALLEL??environment.NX_PARALLEL;
 if(environment.SEMIO_BUILD_GRAPH_DIGEST){env.NX_FORCE_REUSE_CACHED_GRAPH="false";delete env.SEMIO_NX_GRAPH_SEAL_REUSE;}
 return env;
}

/** 🧮️ Lets each watched build finish its graph while the daemon retains source watching. */
export function nxChildEnvironment(environment: NodeJS.ProcessEnv, args: readonly string[], watching: boolean): NodeJS.ProcessEnv {
  if (args[0] === "watch") return { ...environment, NX_DAEMON: "true" };
  return { ...environment, NX_DAEMON: watching || environment.NX_FILE_CHANGES !== undefined ? "false" : environment.NX_DAEMON ?? "true" };
}

/** 🧷️ Names the exact SDK before the physical namespace owner supplies every filesystem path. */
export function nxToolingEnvironmentV1(environment:NodeJS.ProcessEnv,toolingIdentity:string):NodeJS.ProcessEnv{
 if(!/^[a-f0-9]{64}$/u.test(toolingIdentity))throw Error("Original immutable Nx tooling identity required");return {...environment,SEMIO_NX_TOOLING_IDENTITY:toolingIdentity};
}

/** 🔗️ Retains each watcher binding with its concrete graph and socket locations. */
export type NxWatcherEnvironment = Readonly<Record<string, string | undefined>> & Readonly<{ NX_WORKSPACE_DATA_DIRECTORY: string; NX_SOCKET_DIR: string; NX_DAEMON_SOCKET_DIR: undefined }>;

/** 🧷️ Gives an activation binding its own watcher graph and short portable daemon socket path. */
export function nxWatcherEnvironment(root: string, environment: Record<string, string | undefined>, activation: string): NxWatcherEnvironment {
  const binding = Object.entries(environment).filter(([key]) => /^(?:S_|VITE_|PLAYGROUND_|SEMIO_(?:PLUGIN|RENDERER|BUILD_MODE|NX_TOOLING_IDENTITY|LOCALE|LOCKED_LOCALE|LOCKED_TERMINOLOGY)$)/.test(key)).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0);
  const identity = createHash("sha256").update(JSON.stringify([resolve(root).replaceAll("\\", "/"), activation, binding])).digest("hex").slice(0, 24);
  const storage=environment.SEMIO_NX_FIXED_STORAGE,graphStorage=environment.SEMIO_NX_GRAPH_STORAGE,socketStorage=environment.SEMIO_NX_SOCKET_STORAGE,tooling=environment.SEMIO_NX_TOOLING_IDENTITY;if(!storage||!graphStorage||!socketStorage||!tooling)throw Error("Original funded watcher namespace required");const graphIdentity=environment.SEMIO_BUILD_GRAPH_DIGEST??null,expected=boundedNxNamespaceV1(storage,graphStorage,tooling,root,graphIdentity,environment.SEMIO_NX_IMPLEMENTATION_IDENTITY!);if(environment.NX_WORKSPACE_DATA_DIRECTORY!==expected.environment.NX_WORKSPACE_DATA_DIRECTORY)throw Error("Original watcher base namespace refused");const namespace=boundedNxNamespaceV1(storage,graphStorage,tooling,root,graphIdentity,environment.SEMIO_NX_IMPLEMENTATION_IDENTITY!,identity),sockets=boundedNxSocketNamespaceV1(socketStorage,tooling,root,graphIdentity,environment.SEMIO_NX_IMPLEMENTATION_IDENTITY!,identity);
  return { ...environment,...namespace.environment,...sockets.environment,...(environment.SEMIO_NX_GRAPH_DIAGNOSTIC_DIRECTORY?{SEMIO_NX_GRAPH_DIAGNOSTIC_DIRECTORY:join(environment.SEMIO_NX_GRAPH_DIAGNOSTIC_DIRECTORY,"watchers",identity)}:{}),SEMIO_NX_WATCHER_IDENTITY:identity,NX_WORKSPACE_DATA_DIRECTORY:namespace.environment.NX_WORKSPACE_DATA_DIRECTORY!, NX_SOCKET_DIR:sockets.environment.NX_SOCKET_DIR!, NX_DAEMON_SOCKET_DIR: undefined };
}

/** ⏳️ Awaits Nx's readiness handshake while cold graph discovery reports progress and remains cancellable. */
export function waitForNxWatcher(watcher: ReturnType<typeof spawnNxProcess>): Promise<void> {
  return new Promise((accept, reject) => {
    let pending = "";
    const started = Date.now(), progress = setInterval(() => console.log(`[nx] Waiting for source watcher and project graph (${Math.floor((Date.now() - started) / 1000)}s); Ctrl+C cancels`), 15000);
    const finish = (error?: Error): void => {
      clearInterval(progress);
      watcher.removeListener("close", exited);
      watcher.removeListener("error", failed);
      watcher.stdout!.removeListener("data", received);
      if (error) reject(error); else accept();
    };
    const exited = (code: number | null, signal: string | null): void => finish(new Error(`Nx source watcher exited before readiness (${signal ?? code ?? "unknown status"})`));
    const failed = (error: Error): void => finish(error);
    const received = (chunk: Buffer): void => {
      pending = (pending + chunk.toString()).slice(-8192);
      if (pending.includes("watch process waiting...")) finish();
    };
    watcher.once("close", exited);
    watcher.once("error", failed);
    watcher.stdout!.on("data", received);
  });
}

/** 🔁️ Reconciles sources after both watcher readiness and the initial prerequisite graph. */
export function activateWhenNxWatcherReady(watcher: ReturnType<typeof spawnNxProcess>, prepared: Promise<void>, active: () => boolean, activate: () => void, failed: (error: Error) => void): void {
  void Promise.all([waitForNxWatcher(watcher), prepared]).then(() => { if (active()) activate(); }).catch(error => { if (active()) failed(error); });
}

/** 🛡️ Registers automatic source callbacks after the initial prerequisite graph has settled. */
export async function startNxWatcherAfterPreparation(prepared:Promise<void>,active:()=>boolean,start:()=>ReturnType<typeof spawnNxProcess>|Promise<ReturnType<typeof spawnNxProcess>|undefined>):Promise<ReturnType<typeof spawnNxProcess>|undefined> {
  await prepared;return active()?start():undefined;
}

/** ⏳️ Observes Nx starting the serving target after its prerequisite graph completes. */
export function waitForNxTargetStart(child: ReturnType<typeof spawnNxProcess>, target: string): Promise<void> {
  return new Promise((accept,reject) => {
    let pending="";
    const finish=(error?:Error):void => { child.stdout!.removeListener("data",received);child.removeListener("close",closed);child.removeListener("error",failed);if(error)reject(error);else accept(); };
    const received=(chunk:Buffer):void => { pending=(pending+chunk.toString()).replace(/\x1b\[[0-9;?]*[A-Za-z]/g,"").slice(-8192);const marker="> nx run "+target;if([" ","\r","\n"].some(separator=>pending.includes(marker+separator)))finish(); };
    const closed=(code:number|null):void => finish(new Error("Nx initial preparation ended before serving target "+target+" ("+code+")"));
    const failed=(error:Error):void => finish(error);
    child.stdout!.on("data",received);child.once("close",closed);child.once("error",failed);
  });
}

//#region 🔖️NxScript
/** 🌿️ Names the file a nested Nx run streams its complete output to; the caller's own stream then carries only a bounded summary. */
export const NX_OUTPUT_LOG_ENV = "SEMIO_NX_OUTPUT_LOG";

/** 🪵️ Streams every line of a nested Nx run to a log file and forwards a bounded summary: task starts, Nx banners and
 * the run summary, at most `forwardBytes` of them, then one heartbeat per `heartbeatMs`. An outer Nx task buffers all
 * output of its child in one string and dies at V8's 512 MiB string limit, so a release build that logs 950 MB (build #9)
 * must never reach it. @see ./📦️dependencies/🧫️fixtures/🔣️.json `spool` */
export class NxOutputSpoolV1 {
  static readonly FORWARDED = /^(?:> nx run |\s*NX\s|\s*Successfully ran|\s*Failed tasks|\s*Tasks not run|\s*Tasks stopped|\s*Run duration|\[budget\])/u;
  static readonly FORWARD_BYTES = 4 * 1024 * 1024;
  static readonly HEARTBEAT_MS = 60_000;
  lines = 0;
  bytes = 0;
  forwarded = 0;
  private forwardedBytes = 0;
  private nextHeartbeat: number;
  constructor(private readonly fd: number, private readonly path: string, private readonly forward: (line: string) => void, private readonly now: () => number = Date.now, private readonly forwardBytes = NxOutputSpoolV1.FORWARD_BYTES, private readonly heartbeatMs = NxOutputSpoolV1.HEARTBEAT_MS) {
    this.nextHeartbeat = now() + heartbeatMs;
  }
  line(text: string): void {
    this.lines++;
    this.bytes += writeSync(this.fd, `${text}\n`);
    const at = this.now();
    if (NxOutputSpoolV1.FORWARDED.test(text.replace(/\u001b\[[0-9;]*m/gu, "")) && this.forwardedBytes + text.length < this.forwardBytes) {
      this.forwardedBytes += text.length + 1;
      this.forwarded++;
      this.forward(text);
    } else if (at >= this.nextHeartbeat) {
      this.nextHeartbeat = at + this.heartbeatMs;
      this.forward(`[nx-output] ${this.lines} line(s), ${this.bytes} byte(s) in ${this.path}`);
    }
  }
  summary(): string { return `[nx-output] complete log: ${this.path} (${this.lines} line(s), ${this.bytes} byte(s)); ${this.forwarded} line(s) forwarded`; }
  static open(path: string, forward: (line: string) => void): NxOutputSpoolV1 {
    mkdirSync(dirname(path), { recursive: true });
    return new NxOutputSpoolV1(openSync(path, "a"), path, forward);
  }
  close(): void { closeSync(this.fd); }
}

export class NxScript extends Script {
  /** 📦️ The locked JavaScript environment every workspace script task imports from. On a fresh clone (no
   * `node_modules/nx`) it runs to completion before the requested graph: Nx schedules sibling setup tasks in parallel,
   * and without this every generator, `setup-git`, `cpp-setup` and `deps-browsers` started before `bun install` finished
   * (measured on a clean Linux container, ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP `📓️wp-z2.md`). */

  /** 🧿️ A watch invocation starts its configured daemon without deleting namespace failure state. */
  static async ensureDaemon(nxCli: string, root: string, env: NodeJS.ProcessEnv,control:ResourceLeaseControl): Promise<void> {
    const selected=["daemon","--start"],original=readScriptProcessEnvelope(JSON.parse(env[SCRIPT_PROCESS_INVOCATION_ENV]!),Date.now()),owned=nativeNxChildEnvironmentV1(root,issueSelectedNxCallerEnvelopeV1(root,original,selected),selected,env);process.stdout.write(await captureOwnedCommandV1("node",[nxCli,...selected],root,{env:owned,control}));
  }

  static ownedDescendants(roots: readonly number[], rows: readonly { pid: number; parent: number; command: string }[], daemonScript: string): number[] {
    const children = new Map<number, typeof rows[number][]>(), normalize = (path: string): string => path.replaceAll("\\", "/").toLowerCase();
    for (const row of rows) { const siblings = children.get(row.parent) ?? []; siblings.push(row); children.set(row.parent, siblings); }
    const seen = new Set(roots), pending = [...roots], owned: number[] = [], daemon = normalize(daemonScript);
    while (pending.length) for (const row of children.get(pending.pop()!) ?? []) {
      if (seen.has(row.pid) || normalize(row.command).includes(daemon)) continue;
      seen.add(row.pid); owned.push(row.pid); pending.push(row.pid);
    }
    return owned.sort((a, b) => a - b);
  }
  async run(segments: string[]): Promise<void> {return this.runIssued(segments,createIssuedResourceControlV1(this.invocation,"nx:invocation",0));}
  async runIssued(segments:string[],phaseControl:ResourceLeaseControl):Promise<void>{
    requireIssuedResourceControlInvocationV1(phaseControl,this.invocation);
    const{projectOwnedNxProcessEnvelopeV1}=await import("../../🏗️build/📥️caller/🟦️.ts");projectOwnedNxProcessEnvelopeV1(this.root,this.invocation,resolveNxInvocation(segments,{...process.env,SEMIO_BUILD_MODE:readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode},this.root).args);
    await nxBootstrapServices().prepareNxToolingStorageV1(this.root,this.invocation,phaseControl);
    if(segments.length===1&&segments[0]==="--version")return this.runInStorage(segments,phaseControl);
    const {prepareNxGraphCacheStorageV1}=await import("../💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts");await prepareNxGraphCacheStorageV1(this.root,this.invocation,phaseControl,0);
    return withRepositoryNxStorageV1(this.root,this.invocation,phaseControl,(bind,actualPhase)=>this.runInStorage(segments,actualPhase,bind));
  }
  private async runInStorage(segments:string[],phaseControl:ResourceLeaseControl,bind?:(toolingIdentity:string,graphIdentity:string|null,implementationIdentity:string)=>Promise<BoundedNxNamespaceV1>):Promise<void>{return nxBootstrapServices().withNxToolingHeadroomRecoveryV1(this.root,this.invocation,phaseControl,actualPhase=>withRepositoryToolingStoreV1(this.root,this.invocation,actualPhase,(store,storePhase)=>this.runWithToolingStore(segments,store,storePhase,bind)));}
  private async runWithToolingStore(segments:string[],store:RepositoryToolingStoreV1,phaseControl:ResourceLeaseControl,bind?:(toolingIdentity:string,graphIdentity:string|null,implementationIdentity:string)=>Promise<BoundedNxNamespaceV1>):Promise<void|ToolingHeadroomRefusalV1>{
    const graphConsumer=!segments.some(argument=>["--version","-v","--help","-h"].includes(argument))&&!["reset","daemon"].includes(segments[0]??"");
    if (graphConsumer&&!process.env.SEMIO_BUILD_GRAPH_PATH) {
      const {loadBuildGraphV1}=await import("../../🏗️build/🕸️graph/🟦️.ts"),{withRepositoryBuildGraphArtifactV1}=await import("../💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts");
      const {buildInvocationGraphSelectionV1}=await import("../../🏗️build/🕸️graph/🚪️admission/🟦️.ts"),baseline = await loadBuildGraphV1(this.root,this.invocation,phaseControl,{selection:buildInvocationGraphSelectionV1(segments)}),{withSelectedRustBuildGraphV1}=await import("../../🏗️build/🕸️graph/🦀️rust/🟦️.ts");return withSelectedRustBuildGraphV1(this.root,baseline,segments,this.invocation,phaseControl,async(prepared,rustPhase)=>{const graph=prepared.graph;
      const {withSelectedNextBuildGraphV1,nextCacheEnabledV1}=await import("../../../🖥️server/🎛️coordinator/🏗️builder/📥️inputs/🔌️nx/🟦️.ts");
      const useNext=nextCacheEnabledV1()?withSelectedNextBuildGraphV1:async<T>(_workspace:string,value:typeof graph,_args:readonly string[],_invocation:ScriptInvocation,actualPhase:ResourceLeaseControl,body:(value:typeof graph,phase:ResourceLeaseControl)=>Promise<T>)=>body(value,actualPhase);
      return useNext(this.root,graph,segments,this.invocation,rustPhase,(nextGraph,nextPhase)=>withRepositoryBuildGraphArtifactV1(this.root,nextGraph,this.invocation,nextPhase,async(pinned,artifactPhase)=>{const patch={...prepared.environment,SEMIO_BUILD_GRAPH_PATH:pinned.path,SEMIO_BUILD_GRAPH_DIGEST:pinned.digest,SEMIO_BUILD_REUSE_GRAPH:"true"},previous=Object.fromEntries(Object.keys(patch).map(key=>[key,process.env[key]]));Object.assign(process.env,patch);try{await this.runWithToolingStore(segments,store,artifactPhase,bind);}finally{for(const[key,value]of Object.entries(previous))if(value===undefined)delete process.env[key];else process.env[key]=value;}}));
      });
    }
    const {withSelectedRustNativeSnapshotV1}=await import("../../🏗️build/📥️inputs/🎯️selected/🦀️rust/🟦️.ts");return withSelectedRustNativeSnapshotV1(this.root,segments,this.invocation,phaseControl,async(environment,actualPhase)=>{const previous=Object.fromEntries(Object.keys(environment??{}).map(key=>[key,process.env[key]]));Object.assign(process.env,environment);try{return await this.runWithPreparedToolingStore(segments,store,actualPhase,bind,graphConsumer);}finally{for(const[key,value]of Object.entries(previous))if(value===undefined)delete process.env[key];else process.env[key]=value;}});
  }
  private async runWithPreparedToolingStore(segments:string[],store:RepositoryToolingStoreV1,phaseControl:ResourceLeaseControl,bind:((toolingIdentity:string,graphIdentity:string|null,implementationIdentity:string)=>Promise<BoundedNxNamespaceV1>)|undefined,graphConsumer:boolean):Promise<void|ToolingHeadroomRefusalV1>{
    const check=()=>checkResourceLeaseControl(phaseControl);check();
    const outputLog = process.env[NX_OUTPUT_LOG_ENV];
    delete process.env[NX_OUTPUT_LOG_ENV];
    const bootstrapStart=performance.now(),profileBootstrap=process.env.SEMIO_BUILD_PROFILE_INPUTS==="1";
    let admissionFinished=bootstrapStart,provisionFinished=bootstrapStart;
    let tooling: { cli: string; modulePath: string; bun: string; authorizer:string };
    {
      const originalSignal = phaseControl.signal;
      let stopped: NodeJS.Signals | undefined;
      const stop = (signal: NodeJS.Signals): void => { stopped ??= signal; };
      const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
      originalSignal.throwIfAborted();
      process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
      try {
        if(graphConsumer)await publishNxCollectionBoundary(this.root, this.invocation, directories => {check();if (directories === 1 || directories % 1024 === 0) console.log(`[nx] Admitting current real project owners (${directories} directories)`); });check();
        admissionFinished=performance.now();
        const api = nxBootstrapServices();
        const provisioned=await api.provisionNxTools(this.root,store,phaseControl);check();if(isToolingHeadroomRefusalV1(provisioned))return provisioned;tooling=provisioned;
        provisionFinished=performance.now();
        await api.activateNxTools(this.root,tooling,store,phaseControl);check();
        if(profileBootstrap)console.log(`[nx] Bootstrap phases admissionMs=${Math.round(admissionFinished-bootstrapStart)} provisioningMs=${Math.round(provisionFinished-admissionFinished)} activationMs=${Math.round(performance.now()-provisionFinished)}`);
      } catch (error) { if (!stopped) throw error; process.exitCode = stopped === "SIGINT" ? 130 : 143; return; }
      finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
    }
    const nxCli = tooling.cli;
    const invocation = resolveNxInvocation(segments,{...process.env,SEMIO_BUILD_MODE:readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode},this.root), children: ReturnType<typeof spawnNxProcess>[] = [], spooling: Promise<void>[] = [];
    const toolingIdentity=basename(dirname(tooling.modulePath));if(!/^[a-f0-9]{64}$/u.test(toolingIdentity))throw Error("Original loaded Nx tooling receipt required");
    return withNxToolingUseV1(nxToolingUseStorageV1(this.root),{identity:toolingIdentity,toolkitRoot:dirname(tooling.modulePath),invocation:this.invocation},phaseControl,async(_witness,actualPhase)=>{
    const check=()=>checkResourceLeaseControl(actualPhase);check();
    const implementationIdentity=createRequire(import.meta.url)("../../🟨️.mjs").nxBuildDependencyImplementationV1();
    const env = await nxCacheEnvironmentV1(this.root,pinnedNxRuntimeEnvironment(nxToolingEnvironmentV1(nxBuildEnvironmentV1(this.root,devToolingEnv({ ...invocation.env, SEMIO_BUILD_MODE:readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode,CARGO_NET_OFFLINE:String(readNativeCallerAuthorityV1(this.invocation.capabilities).network.offline), NX_ISOLATE_PLUGINS: process.env.NX_ISOLATE_PLUGINS ?? "false", NODE_PATH: tooling.modulePath, SEMIO_NX_TOOLING_ROOT: dirname(tooling.modulePath), SEMIO_NX_IMPLEMENTATION_IDENTITY: implementationIdentity, NX_WORKSPACE_DATA_DIRECTORY: invocation.env.NX_WORKSPACE_DATA_DIRECTORY || process.env.NX_WORKSPACE_DATA_DIRECTORY, NX_SOCKET_DIR: invocation.env.NX_SOCKET_DIR || process.env.NX_SOCKET_DIR, NX_DAEMON_SOCKET_DIR: undefined, npm_lifecycle_event: undefined, npm_lifecycle_script: undefined })),toolingIdentity), tooling.bun));
    if(bind)Object.assign(env,(await bind(toolingIdentity,env.SEMIO_BUILD_GRAPH_DIGEST??null,implementationIdentity)).environment);
    const{projectOwnedNxProcessEnvelopeV1}=await import("../../🏗️build/📥️caller/🟦️.ts"),carried=projectOwnedNxProcessEnvelopeV1(this.root,this.invocation,invocation.args);env[SCRIPT_PROCESS_INVOCATION_ENV]=scriptProcessEnvironment(carried,env)[SCRIPT_PROCESS_INVOCATION_ENV];
    let cancelled: NodeJS.Signals | undefined, cancellationDeadline = 0, watchFailure = 0, finishing = false;
    let sourceHandoff:Readonly<{environment:string;close():Promise<void>}>|undefined;
    let force: ReturnType<typeof setTimeout> | undefined, watcher: ReturnType<typeof spawnNxProcess> | undefined;
    const descendants = new Set<number>();
    const captureDescendants = (): void => {
      const windows = process.platform === "win32";
      const snapshot = stopNxProcessTree(windows ? "powershell.exe" : "ps", windows ? ["-NoLogo", "-NoProfile", "-NonInteractive", "-Command", "Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,CommandLine | ConvertTo-Json -Compress"] : ["-axo", "pid=,ppid=,command="], { encoding: "utf8", windowsHide: true, timeout: 5000 });
      if (snapshot.status !== 0) { console.error("Could not capture Nx descendants for cancellation"); return; }
      const rows = windows ? [JSON.parse(snapshot.stdout)].flat().filter((row) => row && Number.isSafeInteger(row.ProcessId) && row.ProcessId > 0 && Number.isSafeInteger(row.ParentProcessId)).map((row) => ({ pid: row.ProcessId, parent: row.ParentProcessId, command: typeof row.CommandLine === "string" ? row.CommandLine : "" })) : snapshot.stdout.trim().split("\n").flatMap((row) => { const match = row.trim().match(/^(\d+)\s+(\d+)\s+(.*)$/); return match ? [{ pid: Number(match[1]), parent: Number(match[2]), command: match[3] }] : []; });
      const daemon = createRequire(nxCli).resolve("nx/src/daemon/server/start.js");
      for (const pid of NxScript.ownedDescendants(children.flatMap((child) => child.pid ? [child.pid] : []), rows, daemon)) descendants.add(pid);
    };
    const kill = (child: ReturnType<typeof spawnNxProcess>, signal: NodeJS.Signals): void => {
      if (!child.pid) return;
      try { process.kill(-child.pid, signal); } catch {}
    };
    const killAll = (signal: NodeJS.Signals): void => {
      if (process.platform === "win32") {
        const pids = [...descendants, ...children.flatMap((child) => child.pid ? [child.pid] : [])];
        if (pids.length) stopNxProcessTree("taskkill", [...pids.flatMap((pid) => ["/pid", String(pid)]), "/f"], { stdio: "ignore", windowsHide: true });
        return;
      }
      for (const pid of descendants) try { process.kill(pid, signal); } catch {}
      children.forEach((child) => kill(child, signal));
    };
    const stop = (signal: NodeJS.Signals): void => {
      if (cancelled) return;
      cancelled = signal;
      cancellationDeadline = Date.now() + 5000;
      try { captureDescendants(); }
      catch (error) { console.error(`Could not inspect Nx descendants; stopping owned launch processes: ${error instanceof Error ? error.message : String(error)}`); }
      killAll(signal);
      force = setTimeout(() => killAll("SIGKILL"), 5000);
      force.unref();
    };
    const graphChanges = new WeakMap<ReturnType<typeof spawnNxProcess>, { changed: boolean; executing: boolean; tail: string }>();
    const launch = (args: string[], capture = false, environment = env): ReturnType<typeof spawnNxProcess> => {
      check();
      const original=readScriptProcessEnvelope(JSON.parse(environment[SCRIPT_PROCESS_INVOCATION_ENV]!),Date.now()),ownedEnvironment=nativeNxChildEnvironmentV1(this.root,issueSelectedNxCallerEnvelopeV1(this.root,original,args),args,environment);
      const spooled = Boolean(outputLog) && !capture && !invocation.watch;
      const monitored = !invocation.watch;
      const child = spawnNxProcess("node", [nxCli, ...args], { cwd: this.root, env: nxChildEnvironment(ownedEnvironment, args, Boolean(invocation.watch)), stdio: spooled || monitored ? ["inherit", "pipe", "pipe"] : capture ? ["inherit", "pipe", "inherit"] : "inherit", detached: process.platform !== "win32" });
      if (monitored) {
        const state = { changed: false, executing: false, tail: "" }; graphChanges.set(child, state);
        const started = Date.now(), progress = setInterval(() => { if (!state.executing) console.log(`[nx] Graph and hash preparation running elapsedMs=${Date.now() - started}`); }, 10000);
        progress.unref();
        child.once("close", () => clearInterval(progress));
        child.once("error", () => clearInterval(progress));
        for (const [input, output] of [[child.stdout!, process.stdout], [child.stderr!, process.stderr]] as const) input.on("data", chunk => {
          state.tail = (state.tail + String(chunk)).slice(-4096);
          state.executing ||= />\s+nx run\s/.test(state.tail) || state.tail.includes("[script]");
          state.changed ||= state.tail.includes("Dependency resolution authority changed during graph construction");
          if (!spooled) output.write(chunk);
        });
      }
      children.push(child);
      if (spooled) {
        const spool = NxOutputSpoolV1.open(outputLog!, (line) => process.stdout.write(`${line}\n`));
        const drained = [child.stdout!, child.stderr!].map((input) => new Promise<void>((accept) => { createInterface({ input, crlfDelay: Infinity }).on("line", (line) => spool.line(line)).once("close", accept); }));
        spooling.push(new Promise<void>((accept) => { child.once("close", () => { void Promise.all(drained).then(() => { process.stdout.write(`${spool.summary()}\n`); spool.close(); accept(); }); }); }));
      }
      return child;
    };
    const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM"), abort = (): void => stop("SIGTERM");
    actualPhase.signal.addEventListener("abort",abort,{once:true});if(actualPhase.signal.aborted)abort();
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", terminate);
    const requestedBudget = orchestratorBudgetMs();
    const remaining=checkResourceLeaseControl(actualPhase),originalBudget=scriptInvocationBudget(this.invocation,requestedBudget??0),budget=remaining===null?originalBudget:originalBudget===0?Math.floor(remaining):Math.min(originalBudget,Math.floor(remaining));
    const timeout = budget > 0 ? setTimeout(() => { console.error(`[budget] Nx exceeded ${budget}ms`); stop("SIGTERM"); }, budget) : undefined;
    try {
      if (graphConsumer&&!existsSync(join(this.root, "node_modules/nx/package.json")) && !invocation.args.includes(nxBootstrapJavascriptEnvironmentV1)) {
        const install = launch(["run", nxBootstrapJavascriptEnvironmentV1, "--output-style=stream"]);
        const installed = await new Promise<number>((accept, reject) => { install.once("error", reject); install.once("close", (code) => accept(code ?? 1)); });
        if (installed !== 0 || cancelled) { process.exitCode = cancelled ? cancelled === "SIGINT" ? 130 : 143 : installed; return; }
      }
      if (!cancelled) {
        if(graphConsumer){
        const graphSeal = await import("../🕸️graph/🔏️seal/🟦️.ts"), workspaceData = env.NX_WORKSPACE_DATA_DIRECTORY!, inheritedReuse = env.NX_FORCE_REUSE_CACHED_GRAPH === "true" && env.SEMIO_NX_GRAPH_SEAL_REUSE === undefined;
        const sealStarted=performance.now();
        delete env.SEMIO_NX_SELECTED_GRAPH_RECEIPT;
        let selected=false;
        if(!inheritedReuse&&!invocation.watch&&nxChildEnvironment(env,invocation.args,false).NX_DAEMON!=="false"){
          for(const key of ["NX_WORKSPACE_DATA_DIRECTORY","NX_PROJECT_GRAPH_CACHE_DIRECTORY","NX_CACHE_DIRECTORY","NX_NATIVE_FILE_CACHE_DIRECTORY","SEMIO_NX_FIXED_STORAGE","SEMIO_NX_SOCKET_STORAGE","TMPDIR","TMP","TEMP","NX_SOCKET_DIR","NX_DAEMON_SOCKET_DIR","NX_DAEMON"]){const value=key==="NX_DAEMON"?nxChildEnvironment(env,invocation.args,false).NX_DAEMON:env[key];if(value===undefined)delete process.env[key];else process.env[key]=value;}
          const frozen=await createRequire(nxCli)("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1();
          if(!frozen||frozen.toolingIdentity!==toolingIdentity)throw Error("Original immutable graph authorizers required");
          if(frozen.authorizers.selectedTaskSelectionV1(invocation.args,readBuildMode(env.SEMIO_BUILD_MODE??"prototype"))){
            const control=actualPhase,ports=await selectedGraphSdkPortsV1(this.root,nxCli,invocation.args,readBuildMode(env.SEMIO_BUILD_MODE),control,nxChildEnvironment(nativeNxChildEnvironmentV1(this.root,issueSelectedNxCallerEnvelopeV1(this.root,readScriptProcessEnvelope(JSON.parse(env[SCRIPT_PROCESS_INVOCATION_ENV]!),Date.now()),invocation.args),invocation.args,env),invocation.args,false)),proof=await frozen.authorizers.captureSelectedGraphReuseReceiptV1(this.root,workspaceData,ports,actualPhase.signal);check();
            if(proof){sourceHandoff=await writeSelectedGraphHandoffV1(this.root,workspaceData,proof,env,actualPhase.signal);env.SEMIO_NX_SELECTED_GRAPH_RECEIPT=sourceHandoff!.environment;selected=true;}
          }
        }
        const sourceMembership=selected||env.SEMIO_BUILD_GRAPH_DIGEST?undefined:await graphSeal.nxGraphSourceMembershipV2(this.root,actualPhase.signal).catch(error=>{if(profileBootstrap)console.log(`[nx] Graph membership fallback: ${error instanceof Error?error.message:String(error)}`);return undefined;});
        const sealed=selected||!env.SEMIO_BUILD_GRAPH_DIGEST&&!inheritedReuse&&!invocation.watch&&env.NX_DAEMON!=="true"&&await graphSeal.nxGraphSealHolds(this.root,workspaceData,actualPhase.signal,sourceMembership?.digest).catch(()=>false);
        if(profileBootstrap)console.log(`[nx] Graph observation selected=${selected} sealed=${sealed} elapsedMs=${Math.round(performance.now()-sealStarted)}`);
        if(!inheritedReuse){env.NX_FORCE_REUSE_CACHED_GRAPH=sealed?"true":"false";env.SEMIO_NX_GRAPH_SEAL_REUSE=sealed?"true":"false";}
        check();
        env.SEMIO_NX_GRAPH_SOURCE_MEMBERSHIP=sourceMembership?.digest;
        }
        const launchedAt = Date.now();
        const child=launch(invocation.args,Boolean(invocation.watch));
        const status=new Promise<number>((accept,reject) => { child.once("error",reject);child.once("close",code=>accept(code??1)); });void status.catch(()=>{});
        if(invocation.watch) {
          const watchTarget=invocation.watch;
          const prepared=invocation.watch.includes(":activate-") ? waitForNxTargetStart(child,invocation.args[1]!) : Promise.resolve();
          child.stdout!.on("data",chunk=>process.stdout.write(chunk));
          const watchEnvironment=nxWatcherEnvironment(this.root,env,invocation.watch);
          watcher=await startNxWatcherAfterPreparation(prepared,()=>!finishing&&!cancelled,async()=>{
            await NxScript.ensureDaemon(nxCli,this.root,watchEnvironment,actualPhase);check();
            if(finishing||cancelled)return undefined;
            const registered=launch(["watch","--all","--includeGlobalWorkspaceFiles","--verbose","--","bun","nx","run",watchTarget,"--output-style=stream"],true,watchEnvironment);
            return registered;
          });
          if(!watcher){process.exitCode=cancelled?cancelled==="SIGINT"?130:143:await status;return;}
          watcher.stdout!.on("data",chunk=>process.stdout.write(chunk));
          watcher.once("close",code=>{if(!finishing&&!cancelled){watchFailure=code||1;stop("SIGTERM");}});
          activateWhenNxWatcherReady(watcher,Promise.resolve(),()=>!finishing&&!cancelled,()=>{
            const activation=launch(["run",watchTarget,"--output-style=stream"]);
            activation.once("error",error=>{console.error(error.message);watchFailure=1;stop("SIGTERM");});
            activation.once("close",code=>{if(code&&!finishing&&!cancelled){watchFailure=code;stop("SIGTERM");}});
          },error=>{console.error(error.message);watchFailure=1;stop("SIGTERM");});
        }
        let code = await status, completed = child, attemptStart = launchedAt;
        for (let attempt = 1; code !== 0 && !cancelled && !invocation.watch && graphChanges.get(completed)?.changed && !graphChanges.get(completed)?.executing && attempt <= 3; attempt++) {
          check();
          console.log(`[nx] Source authority changed; admitting the current graph again (${attempt}/3)`);
          await publishNxCollectionBoundary(this.root,this.invocation,()=>check());check();
          await sourceHandoff?.close();sourceHandoff=undefined;delete env.SEMIO_NX_SELECTED_GRAPH_RECEIPT;
          env.NX_FORCE_REUSE_CACHED_GRAPH = "false";
          env.SEMIO_NX_GRAPH_SEAL_REUSE = "false";
          attemptStart = Date.now(); completed = launch(invocation.args);
          code = await new Promise<number>((accept, reject) => { completed.once("error", reject); completed.once("close", value => accept(value ?? 1)); });
        }
        await Promise.all(spooling);
        process.exitCode=watchFailure||(cancelled?cancelled==="SIGINT"?130:143:code);
      } else process.exitCode=cancelled==="SIGINT"?130:143;
    } catch (error) {
      if (cancelled) process.exitCode = cancelled === "SIGINT" ? 130 : 143;
      else throw error;
    } finally {
      await sourceHandoff?.close();
      finishing = true;
      if (invocation.watch && children.length && !cancelled) stop("SIGTERM");
      if (cancelled && process.platform !== "win32") {
        const alive = (child: ReturnType<typeof spawnNxProcess>): boolean => { try { if (child.pid) { process.kill(-child.pid, 0); return true; } } catch {} return false; };
        const descendantAlive = (pid: number): boolean => { try { process.kill(pid, 0); return true; } catch { return false; } };
        while ((children.some(alive) || [...descendants].some(descendantAlive)) && Date.now() < cancellationDeadline) await new Promise((accept) => setTimeout(accept, 25));
        killAll("SIGKILL");
        const settle = Date.now() + 1000;
        while ([...descendants].some(descendantAlive) && Date.now() < settle) await new Promise((accept) => setTimeout(accept, 25));
      }
      if (force) clearTimeout(force);
      if (timeout) clearTimeout(timeout);
      actualPhase.signal.removeEventListener("abort",abort);
      process.removeListener("SIGINT", interrupt);
      process.removeListener("SIGTERM", terminate);
    }
    });
  }
}

/** 🎚️ Selects one compiler mode before Nx hashes or builds its task graph. */
export function resolveNxInvocation(segments: string[], environment: NodeJS.ProcessEnv = process.env,root:string=WORKSPACE_ROOT): { args: string[]; env: NodeJS.ProcessEnv; watch?: string } {
  const selection = resolveBuildSelectionV1(root,segments);
  return resolveNxBuildMode(selection, environment);
}
//#endregion 🔖️NxScript


/** 🌳️ Runs a newly owned outer Nx invocation whose resources come from the current process environment, within the caller's remaining deadline. */
export async function runOwnedNxInvocationV1(root:string,args:readonly string[],parent:ScriptInvocation,phaseControl:ResourceLeaseControl):Promise<void>{
 const{projectOwnedNxProcessEnvelopeV1}=await import("../../🏗️build/📥️caller/🟦️.ts");projectOwnedNxProcessEnvelopeV1(root,parent,resolveNxInvocation([...args],{...process.env,SEMIO_BUILD_MODE:readNativeCallerAuthorityV1(parent.capabilities).original.buildMode},root).args);
 const priorExitCode=process.exitCode;process.exitCode=undefined;
 await new NxScript(root,root,parent).runIssued([...args],phaseControl);
 const status=process.exitCode;process.exitCode=priorExitCode;
 if(status!==undefined&&status!==0)throw new Error(`Owned Nx invocation failed (${status}): nx ${args.join(" ")}`);
}

if (import.meta.main) {
  const args=process.argv.slice(2);
    const encoded = process.env.SEMIO_SCRIPT_POLICY, capabilityWire = process.env.SEMIO_SCRIPT_CAPABILITIES, inherited = process.env[SCRIPT_PROCESS_INVOCATION_ENV];
    const incoming = inherited === undefined ? undefined : readScriptProcessEnvelope(JSON.parse(inherited), Date.now());
    const policy = encoded === undefined ? incoming?.policy ?? readScriptPolicy({ version: 1, owner: "repository:nx", maximumElapsedMilliseconds: orchestratorBudgetMs() ?? 0 }) : readScriptPolicy(JSON.parse(encoded));
    const capabilities = incoming ? capabilityWire===undefined?incoming.capabilities:JSON.parse(capabilityWire) : capabilityWire===undefined?createNxBootstrapCapabilitiesV1(WORKSPACE_ROOT,args,process.env):readNxBootstrapAuthoringCapabilitiesV1(JSON.parse(capabilityWire));
    const envelope = incoming ?? createScriptProcessEnvelope(policy, capabilities, Date.now());
    if (envelope.policy.version !== policy.version || envelope.policy.owner !== policy.owner || envelope.policy.maximumElapsedMilliseconds !== policy.maximumElapsedMilliseconds) throw Error("Original Nx process policy changed across handoff");
    if (JSON.stringify(envelope.capabilities) !== JSON.stringify(capabilities)) throw Error("Original Nx process capabilities changed across handoff");
    process.env[SCRIPT_PROCESS_INVOCATION_ENV] = scriptProcessEnvironment(envelope, process.env)[SCRIPT_PROCESS_INVOCATION_ENV];
  const native=await import("../../../🎛️dashboard/📦️installation/🟦️.ts"),selectedMode=resolveNxInvocation(args.slice(1)).env.SEMIO_BUILD_MODE,mode=readBuildMode((envelope.capabilities as {buildMode?:unknown}).buildMode??selectedMode??"prototype");
  if(selectedMode!==undefined&&mode!==selectedMode)throw Error("Original dashboard build mode changed across handoff");
  const invocation=args[0]==="nx"&&native.dashboardInstalled(WORKSPACE_ROOT,mode)?native.dashboardInvocation(args.slice(1)):undefined;
  if(invocation){try{await withScriptProcessEnvelope(envelope,async original=>{process.exitCode=await native.launchDashboard(WORKSPACE_ROOT,invocation,{invocation:original,workspace:WORKSPACE_ROOT,mode,network:readNativeCallerAuthorityV1(original.capabilities).network,control:scriptCompilerAdmissionControlV1(original,"dashboard:launch",0)});});}catch(error){console.error(error instanceof Error?error.message:String(error));process.exitCode=1;}}
  else{
    try { await withScriptProcessEnvelope(envelope, async original => {
      publishBootstrapSources(WORKSPACE_ROOT);
      await new ScriptRouter(WORKSPACE_ROOT).register("nx", NxScript).run(args, original);
    }); } finally { if (inherited === undefined) delete process.env[SCRIPT_PROCESS_INVOCATION_ENV]; else process.env[SCRIPT_PROCESS_INVOCATION_ENV] = inherited; }
  }
}
