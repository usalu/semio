#!/usr/bin/env bun
import {nativeNxChildEnvironmentV1,createNativeNxCallerCapabilitiesV1,issueSelectedNxCallerEnvelopeV1} from "../📦️artifacts/📋️native-orchestration/📥️caller/🟦️.ts";
import {resolveBuildSelectionV1} from "../../🏗️build/🧭️invocation/🟦️.ts";
import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { createScriptProcessEnvelope, readScriptProcessEnvelope, scriptProcessEnvironment, SCRIPT_PROCESS_INVOCATION_ENV, withScriptProcessEnvelope } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { spawn as spawnNxProcess, spawnSync as stopNxProcessTree } from "node:child_process";
import { closeSync, existsSync, mkdirSync, openSync, readFileSync, rmSync, writeSync } from "node:fs";
import { lstat, mkdir, readFile, readdir, rename, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { createHash, randomUUID } from "node:crypto";
import { createInterface } from "node:readline";
import { tmpdir } from "node:os";
import { basename, dirname, isAbsolute, join, resolve, relative, sep, win32, posix } from "node:path";
import { Script, ScriptRouter, readScriptPolicy, scriptInvocationBudget, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { orchestratorBudgetMs } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { devToolingEnv, semioBuildParallelFlag } from "../../🏃️process/🌿️environment/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";

import { gitCollectionManifests, moduleCollectionAncestry } from "./📥️admission/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
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

/** 🌱️ Publishes the declared bootstrap sources — generated files Bun or the repository tooling must find before Nx runs
 * and a fresh clone lacks — through their owners' dependency-free publishers, required lazily so the Nx bootstrap closure
 * stays system-only. @see ./🌱️sources/🔣️.json */
export function publishBootstrapSources(root: string): void {
  const declaration = join(import.meta.dirname, "🌱️sources", "🔣️.json");
  if (!existsSync(declaration)) return;
  const { sources } = JSON.parse(readFileSync(declaration, "utf8")) as { sources: readonly { module: string; export: string }[] };
  for (const source of sources) createRequire(import.meta.url)(join(root, source.module))[source.export](root);
}

/** 📂️ One directory the admission walk descended into: its identity stamp, the child directories it queues and the admitted manifest names it holds. */
type NxAdmissionDirectory = { readonly dev: number; readonly ino: number; readonly mtimeMs: number; readonly ctimeMs: number; readonly children: readonly string[]; readonly manifests: readonly string[] };

/** 🗄️ The admission snapshot persisted between invocations, keyed by the policy that produced it. */
type NxAdmissionSnapshot = { readonly policy: string; readonly directories: Readonly<Record<string, NxAdmissionDirectory>> };

/** ⏱️ Directory stamps younger than this at snapshot time are re-read next time: a later entry change within one coarse timestamp tick would otherwise keep the same mtime (racy-git rule). */
const NX_ADMISSION_RACY_MS = 2000;

/** 🚦️ Concurrent directory reads in flight during the admission walk. */
const NX_ADMISSION_CONCURRENCY = 64;

/** 🗂️ Where one workspace's admission snapshot lives: inside the excluded repository cache, so writing it never perturbs the walk. */
function nxAdmissionSnapshotPath(root: string): string {
  return join(root, ".🧬semio", "🦑️repo", "⚡️cache", "nx-admission", `${createHash("sha256").update(root).digest("hex").slice(0, 16)}.json`);
}

/** 📖️ Reads the previous admission snapshot for `policy`, or none when absent, unreadable or produced by another policy. */
async function readNxAdmissionSnapshot(path: string, policy: string): Promise<NxAdmissionSnapshot["directories"]> {
  try {
    const snapshot = JSON.parse(await readFile(path, "utf8")) as NxAdmissionSnapshot;
    return snapshot.policy === policy && snapshot.directories && typeof snapshot.directories === "object" ? snapshot.directories : {};
  } catch { return {}; }
}

/** 💾️ Atomically replaces the admission snapshot; a concurrent invocation's write wins whole or loses whole. */
async function writeNxAdmissionSnapshot(path: string, snapshot: NxAdmissionSnapshot): Promise<void> {
  await mkdir(dirname(path), { recursive: true });
  const temporary = `${path}.${process.pid}.${randomUUID()}.tmp`;
  await writeFile(temporary, JSON.stringify(snapshot));
  await rename(temporary, path);
}

/** 🧵️ Runs `visit` over a growing work list with at most `limit` visits in flight; the first failure stops scheduling and rejects. */
function boundedWalk<T>(initial: readonly T[], limit: number, visit: (item: T, enqueue: (next: T) => void) => Promise<void>): Promise<void> {
  return new Promise((accept, reject) => {
    const pending = [...initial];
    let active = 0, failed = false;
    const pump = (): void => {
      while (!failed && active < limit && pending.length) {
        active++;
        visit(pending.pop()!, (next) => { pending.push(next); }).then(() => { active--; if (!failed) { if (pending.length) pump(); else if (active === 0) accept(); } }, (error: unknown) => { if (!failed) { failed = true; reject(error); } });
      }
      if (!failed && active === 0 && pending.length === 0) accept();
    };
    pump();
  });
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
export async function publishNxCollectionBoundary(root: string, signal?: AbortSignal, progress?: (directories: number) => void): Promise<void> {
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
  const snapshotPath = nxAdmissionSnapshotPath(root), snapshotPolicy = createHash("sha256").update(policyText).update("\0").update(ownershipText).digest("hex");
  const previous = await readNxAdmissionSnapshot(snapshotPath, snapshotPolicy), current: Record<string, NxAdmissionDirectory> = {}, startedMs = Date.now();
  const insideCollection = (parts: readonly string[]): boolean => moduleCollectionAncestry(parts, ownership.collections).inside;
  let directories = 0, reused = 0;
  const inventory = await gitCollectionManifests(root, policy, ownership.collections, signal);
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
  await verifyNxAdmissionDirectoriesV1(root,visited,admitted,current,signal);
  for (const file of admitted) {
    signal?.throwIfAborted();
    await real(dirname(join(root, file.path)));
    const now = await lstat(join(root, file.path));
    if (!now.isFile() || now.isSymbolicLink() || now.dev !== file.device || now.ino !== file.inode) throw new Error("Nx current admission requires unchanged real manifest custody");
  }
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
    const temporary = join(dirname(snapshotPath), `ignore-${process.pid}-${randomUUID()}.tmp`);
    await mkdir(dirname(temporary), { recursive: true });
    try {
      await writeFile(temporary, result);
      signal?.throwIfAborted();
      if (await original() !== before) throw new Error("Nx ignore authority changed before atomic publication");
      await rename(temporary, target);
    } finally { rmSync(temporary, { force: true }); }
  }
  if (inventory === undefined && (reused !== directories || Object.keys(previous).length !== directories)) await writeNxAdmissionSnapshot(snapshotPath, { policy: snapshotPolicy, directories: current });
}

/** 🏗️ Projects portable process controls into the selected Nx implementation. */
export function nxBuildEnvironmentV1(root:string,environment:NodeJS.ProcessEnv):NodeJS.ProcessEnv {
 const env:NodeJS.ProcessEnv={...environment,SEMIO_BUILD_TOOL:"nx",SEMIO_WORKSPACE_ROOT:root,NX_WORKSPACE_ROOT:root};
 env.NX_TUI=environment.SEMIO_BUILD_TERMINAL??environment.NX_TUI??"false";
 env.NX_TASKS_RUNNER_DYNAMIC_OUTPUT=environment.SEMIO_BUILD_DYNAMIC_OUTPUT??environment.NX_TASKS_RUNNER_DYNAMIC_OUTPUT??"false";
 env.NX_VERBOSE_LOGGING=environment.SEMIO_BUILD_VERBOSE??environment.NX_VERBOSE_LOGGING??"false";
 env.NX_PARALLEL=environment.SEMIO_BUILD_PARALLEL??environment.NX_PARALLEL;
 if(environment.SEMIO_BUILD_REUSE_GRAPH==="true")env.NX_FORCE_REUSE_CACHED_GRAPH="true";
 return env;
}

/** 🧮️ Lets each watched build finish its graph while the daemon retains source watching. */
export function nxChildEnvironment(environment: NodeJS.ProcessEnv, args: readonly string[], watching: boolean): NodeJS.ProcessEnv {
  if (args[0] === "watch") return { ...environment, NX_DAEMON: "true" };
  return { ...environment, NX_DAEMON: watching || environment.NX_FILE_CHANGES !== undefined ? "false" : environment.NX_DAEMON ?? "true" };
}

/** 🧷️ Pins the finite daemon's graph and socket to its actual immutable tooling and workspace. */
export function nxFiniteEnvironment(root:string,environment:NodeJS.ProcessEnv,toolingIdentity:string,platform:NodeJS.Platform=process.platform,temporary=tmpdir()):NodeJS.ProcessEnv{
 if(!/^[a-f0-9]{64}$/u.test(toolingIdentity))throw Error("Original immutable Nx tooling identity required");const paths=platform==="win32"?win32:posix,identity=createHash("sha256").update(JSON.stringify([paths.resolve(root).replaceAll("\\","/"),toolingIdentity])).digest("hex").slice(0,24),socketRoot=platform!=="win32"&&Buffer.byteLength(temporary)>50?"/tmp":temporary;
 return {...environment,SEMIO_NX_TOOLING_IDENTITY:toolingIdentity,NX_WORKSPACE_DATA_DIRECTORY:environment.NX_WORKSPACE_DATA_DIRECTORY??paths.join(root,".nx","workspace-data","tooling",identity),NX_SOCKET_DIR:environment.NX_SOCKET_DIR??paths.join(socketRoot,"semio-nx-"+identity),NX_DAEMON_SOCKET_DIR:undefined};
}

/** 🔗️ Retains each watcher binding with its concrete graph and socket locations. */
export type NxWatcherEnvironment = Readonly<Record<string, string | undefined>> & Readonly<{ NX_WORKSPACE_DATA_DIRECTORY: string; NX_SOCKET_DIR: string; NX_DAEMON_SOCKET_DIR: undefined }>;

/** 🧷️ Gives an activation binding its own watcher graph and short portable daemon socket path. */
export function nxWatcherEnvironment(root: string, environment: Record<string, string | undefined>, activation: string): NxWatcherEnvironment {
  const binding = Object.entries(environment).filter(([key]) => /^(?:S_|VITE_|PLAYGROUND_|SEMIO_(?:PLUGIN|RENDERER|BUILD_MODE|NX_TOOLING_IDENTITY|LOCALE|LOCKED_LOCALE|LOCKED_TERMINOLOGY)$)/.test(key)).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0);
  const identity = createHash("sha256").update(JSON.stringify([resolve(root).replaceAll("\\", "/"), activation, binding])).digest("hex").slice(0, 24);
  const temporary = tmpdir(), socketRoot = process.platform !== "win32" && Buffer.byteLength(temporary) > 50 ? "/tmp" : temporary;
  return { ...environment, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx", "workspace-data", "watchers", identity), NX_SOCKET_DIR: join(socketRoot, `semio-nx-${identity}`), NX_DAEMON_SOCKET_DIR: undefined };
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
  static readonly javascriptEnvironment = "workspace:deps-javascript";

  /** 🧿️ A watch invocation starts its configured daemon without deleting namespace failure state. */
  static async ensureDaemon(nxCli: string, root: string, env: NodeJS.ProcessEnv,control:ResourceLeaseControl): Promise<void> {
    process.stdout.write(await captureOwnedCommandV1("node",[nxCli,"daemon","--start"],root,{env,control}));
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
  async run(segments: string[]): Promise<void> {
    const outputLog = process.env[NX_OUTPUT_LOG_ENV];
    delete process.env[NX_OUTPUT_LOG_ENV];
    const bootstrapStart=performance.now(),profileBootstrap=process.env.SEMIO_NX_PROFILE_INPUTS==="1";
    let admissionFinished=bootstrapStart,provisionFinished=bootstrapStart;
    let tooling: { cli: string; modulePath: string; bun: string };
    {
      const originalSignal = this.invocation.control.signal;
      let stopped: NodeJS.Signals | undefined;
      const stop = (signal: NodeJS.Signals): void => { stopped ??= signal; };
      const interrupt = (): void => stop("SIGINT"), terminate = (): void => stop("SIGTERM");
      originalSignal.throwIfAborted();
      process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
      try {
        await publishNxCollectionBoundary(this.root, this.invocation.control.signal, directories => { if (directories === 1 || directories % 1024 === 0) console.log(`[nx] Admitting current real project owners (${directories} directories)`); });
        admissionFinished=performance.now();
        const api = nxBootstrapServices();
        tooling = await api.provisionNxTools(this.root, this.invocation.control.signal);
        provisionFinished=performance.now();
        await api.activateNxTools(this.root, tooling, this.invocation.control.signal);
        if(profileBootstrap)console.log(`[nx] Bootstrap phases admissionMs=${Math.round(admissionFinished-bootstrapStart)} provisioningMs=${Math.round(provisionFinished-admissionFinished)} activationMs=${Math.round(performance.now()-provisionFinished)}`);
      } catch (error) { if (!stopped) throw error; process.exitCode = stopped === "SIGINT" ? 130 : 143; return; }
      finally { process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", terminate); }
    }
    const nxCli = tooling.cli;
    const invocation = resolveNxInvocation(segments), children: ReturnType<typeof spawnNxProcess>[] = [], spooling: Promise<void>[] = [];
    const toolingIdentity=basename(dirname(tooling.modulePath));if(!/^[a-f0-9]{64}$/u.test(toolingIdentity))throw Error("Original loaded Nx tooling receipt required");
    return withNxToolingUseV1(nxToolingUseStorageV1(this.root),{identity:toolingIdentity,toolkitRoot:dirname(tooling.modulePath),invocation:this.invocation},async()=>{
    const env = await nxCacheEnvironmentV1(this.root,pinnedNxRuntimeEnvironment(nxFiniteEnvironment(this.root,nxBuildEnvironmentV1(this.root,devToolingEnv({ ...invocation.env, NX_ISOLATE_PLUGINS: process.env.NX_ISOLATE_PLUGINS ?? "false", NODE_PATH: tooling.modulePath, NX_WORKSPACE_DATA_DIRECTORY: invocation.env.NX_WORKSPACE_DATA_DIRECTORY || process.env.NX_WORKSPACE_DATA_DIRECTORY, NX_SOCKET_DIR: invocation.env.NX_SOCKET_DIR || process.env.NX_SOCKET_DIR, NX_DAEMON_SOCKET_DIR: undefined, npm_lifecycle_event: undefined, npm_lifecycle_script: undefined })),toolingIdentity), tooling.bun));
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
    this.invocation.control.signal.addEventListener("abort", abort, { once: true });
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", terminate);
    const requestedBudget = orchestratorBudgetMs();
    const budget = scriptInvocationBudget(this.invocation, requestedBudget ?? 0);
    const timeout = budget > 0 ? setTimeout(() => { console.error(`[budget] Nx exceeded ${budget}ms`); stop("SIGTERM"); }, budget) : undefined;
    try {
      if (!existsSync(join(this.root, "node_modules/nx/package.json")) && !invocation.args.includes(NxScript.javascriptEnvironment)) {
        const install = launch(["run", NxScript.javascriptEnvironment, "--output-style=stream"]);
        const installed = await new Promise<number>((accept, reject) => { install.once("error", reject); install.once("close", (code) => accept(code ?? 1)); });
        if (installed !== 0 || cancelled) { process.exitCode = cancelled ? cancelled === "SIGINT" ? 130 : 143 : installed; return; }
      }
      if (!cancelled) {
        const graphSeal = await import("../🕸️graph/🔏️seal/🟦️.ts"), workspaceData = env.NX_WORKSPACE_DATA_DIRECTORY!, inheritedReuse = env.NX_FORCE_REUSE_CACHED_GRAPH === "true" && env.SEMIO_NX_GRAPH_SEAL_REUSE === undefined;
        const sealStarted=performance.now();
        delete env.SEMIO_NX_SELECTED_GRAPH_RECEIPT;
        let selected=false;
        if(!inheritedReuse&&!invocation.watch&&nxChildEnvironment(env,invocation.args,false).NX_DAEMON!=="false"){
          for(const key of ["NX_WORKSPACE_DATA_DIRECTORY","NX_SOCKET_DIR","NX_DAEMON_SOCKET_DIR","NX_DAEMON"]){const value=key==="NX_DAEMON"?nxChildEnvironment(env,invocation.args,false).NX_DAEMON:env[key];if(value===undefined)delete process.env[key];else process.env[key]=value;}
          const frozen=await createRequire(nxCli)("nx/src/utils/workspace-context").loadSemioGraphAuthorizersV1();
          if(!frozen||frozen.toolingIdentity!==toolingIdentity)throw Error("Original immutable graph authorizers required");
          if(frozen.authorizers.selectedTaskSelectionV1(invocation.args,readBuildMode(env.SEMIO_BUILD_MODE??"prototype"))){
            const control=scriptCompilerAdmissionControlV1(this.invocation,"nx:selected-graph-observation",0),ports=await frozen.authorizers.selectedGraphSdkPortsV1(this.root,nxCli,invocation.args,readBuildMode(env.SEMIO_BUILD_MODE),control,nxChildEnvironment(nativeNxChildEnvironmentV1(this.root,issueSelectedNxCallerEnvelopeV1(this.root,readScriptProcessEnvelope(JSON.parse(env[SCRIPT_PROCESS_INVOCATION_ENV]!),Date.now()),invocation.args),invocation.args,env),invocation.args,false)),proof=await frozen.authorizers.captureSelectedGraphReuseReceiptV1(this.root,workspaceData,ports,this.invocation.control.signal);
            if(proof){sourceHandoff=await frozen.authorizers.writeSelectedGraphHandoffV1(workspaceData,proof,this.invocation.control.signal);env.SEMIO_NX_SELECTED_GRAPH_RECEIPT=sourceHandoff!.environment;selected=true;}
          }
        }
        const sourceMembership=selected?undefined:await graphSeal.nxGraphSourceMembershipV2(this.root,this.invocation.control.signal).catch(error=>{if(profileBootstrap)console.log(`[nx] Graph membership fallback: ${error instanceof Error?error.message:String(error)}`);return undefined;});
        const sealed=selected||!inheritedReuse&&!invocation.watch&&env.NX_DAEMON!=="true"&&await graphSeal.nxGraphSealHolds(this.root,workspaceData,this.invocation.control.signal,sourceMembership?.digest).catch(()=>false);
        if(profileBootstrap)console.log(`[nx] Graph observation selected=${selected} sealed=${sealed} elapsedMs=${Math.round(performance.now()-sealStarted)}`);
        if(!inheritedReuse){env.NX_FORCE_REUSE_CACHED_GRAPH=sealed?"true":"false";env.SEMIO_NX_GRAPH_SEAL_REUSE=sealed?"true":"false";}
        this.invocation.control.signal.throwIfAborted();
        env.SEMIO_NX_GRAPH_SOURCE_MEMBERSHIP=sourceMembership?.digest;
        const launchedAt = Date.now();
        const child=launch(invocation.args,Boolean(invocation.watch));
        const status=new Promise<number>((accept,reject) => { child.once("error",reject);child.once("close",code=>accept(code??1)); });void status.catch(()=>{});
        if(invocation.watch) {
          const watchTarget=invocation.watch;
          const prepared=invocation.watch.includes(":activate-") ? waitForNxTargetStart(child,invocation.args[1]!) : Promise.resolve();
          child.stdout!.on("data",chunk=>process.stdout.write(chunk));
          const watchEnvironment=nxWatcherEnvironment(this.root,env,invocation.watch);
          watcher=await startNxWatcherAfterPreparation(prepared,()=>!finishing&&!cancelled,async()=>{
            await NxScript.ensureDaemon(nxCli,this.root,watchEnvironment,scriptCompilerAdmissionControlV1(this.invocation,"nx:watch-daemon-start",0));
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
          this.invocation.control.signal.throwIfAborted();
          console.log(`[nx] Source authority changed; admitting the current graph again (${attempt}/3)`);
          await publishNxCollectionBoundary(this.root, this.invocation.control.signal);
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
      this.invocation.control.signal.removeEventListener("abort", abort);
      process.removeListener("SIGINT", interrupt);
      process.removeListener("SIGTERM", terminate);
    }
    });
  }
}

/** 🎚️ Selects one compiler mode before Nx hashes or builds its task graph. */
export function resolveNxInvocation(segments: string[], environment: NodeJS.ProcessEnv = process.env): { args: string[]; env: NodeJS.ProcessEnv; watch?: string } {
  const selection = resolveBuildSelectionV1(WORKSPACE_ROOT,segments);
  return resolveNxBuildMode(selection, environment);
}
//#endregion 🔖️NxScript


/** 🧭️ Declares the actual resolved user, provision and watcher launches before the original envelope exists. */
export function createNxBootstrapCapabilitiesV1(root:string,args:readonly string[],environment:NodeJS.ProcessEnv):ReturnType<typeof createNativeNxCallerCapabilitiesV1>{
 if(args[0]!=="nx")throw Error("Original bootstrap Nx request required");
 const invocation=resolveNxInvocation(args.slice(1),environment),launches=[invocation.args,["run",NxScript.javascriptEnvironment,"--output-style=stream"]];
 if(invocation.watch)launches.push(["watch","--all","--includeGlobalWorkspaceFiles","--verbose","--","bun","nx","run",invocation.watch,"--output-style=stream"],["run",invocation.watch,"--output-style=stream"]);
 return createNativeNxCallerCapabilitiesV1(root,args,invocation.args,{...environment,...invocation.env},launches);
}

/** 🌳️ Runs a newly owned outer Nx invocation whose resources come from the current process environment, within the caller's remaining deadline. */
export async function runOwnedNxInvocationV1(root:string,args:readonly string[],parent:ScriptInvocation):Promise<void>{
 const remaining=parent.control.remainingMilliseconds(),now=Date.now(),inherited=process.env[SCRIPT_PROCESS_INVOCATION_ENV];
 const envelope=readScriptProcessEnvelope({version:1,policy:parent.policy,deadlineEpochMilliseconds:remaining===null?null:now+remaining,capabilities:createNxBootstrapCapabilitiesV1(root,["nx",...args],process.env)},now);
 process.env[SCRIPT_PROCESS_INVOCATION_ENV]=scriptProcessEnvironment(envelope,process.env)[SCRIPT_PROCESS_INVOCATION_ENV];
 const priorExitCode=process.exitCode;
 process.exitCode=undefined;
 try{await withScriptProcessEnvelope(envelope,original=>new ScriptRouter(root).register("nx",NxScript).run(["nx",...args],original));}
 finally{if(inherited===undefined)delete process.env[SCRIPT_PROCESS_INVOCATION_ENV];else process.env[SCRIPT_PROCESS_INVOCATION_ENV]=inherited;}
 const status=process.exitCode;
 process.exitCode=priorExitCode;
 if(status!==undefined&&status!==0)throw new Error(`Owned Nx invocation failed (${status}): nx ${args.join(" ")}`);
}

if (import.meta.main) {
  const args=process.argv.slice(2);
    const encoded = process.env.SEMIO_SCRIPT_POLICY, capabilityWire = process.env.SEMIO_SCRIPT_CAPABILITIES, inherited = process.env[SCRIPT_PROCESS_INVOCATION_ENV];
    const incoming = inherited === undefined ? undefined : readScriptProcessEnvelope(JSON.parse(inherited), Date.now());
    const policy = encoded === undefined ? incoming?.policy ?? readScriptPolicy({ version: 1, owner: "repository:nx", maximumElapsedMilliseconds: orchestratorBudgetMs() ?? 0 }) : readScriptPolicy(JSON.parse(encoded));
    const capabilities = capabilityWire === undefined ? incoming?.capabilities ?? createNxBootstrapCapabilitiesV1(WORKSPACE_ROOT,args,process.env) : JSON.parse(capabilityWire);
    const envelope = incoming ?? createScriptProcessEnvelope(policy, capabilities, Date.now());
    if (envelope.policy.version !== policy.version || envelope.policy.owner !== policy.owner || envelope.policy.maximumElapsedMilliseconds !== policy.maximumElapsedMilliseconds) throw Error("Original Nx process policy changed across handoff");
    if (JSON.stringify(envelope.capabilities) !== JSON.stringify(capabilities)) throw Error("Original Nx process capabilities changed across handoff");
    process.env[SCRIPT_PROCESS_INVOCATION_ENV] = scriptProcessEnvironment(envelope, process.env)[SCRIPT_PROCESS_INVOCATION_ENV];
  const native=await import("../../../🎛️dashboard/📦️installation/🟦️.ts"),selectedMode=resolveNxInvocation(args.slice(1)).env.SEMIO_BUILD_MODE,mode=readBuildMode((envelope.capabilities as {buildMode?:unknown}).buildMode??selectedMode??"prototype");
  if(selectedMode!==undefined&&mode!==selectedMode)throw Error("Original dashboard build mode changed across handoff");
  const invocation=args[0]==="nx"&&native.dashboardInstalled(WORKSPACE_ROOT,mode)?native.dashboardInvocation(args.slice(1)):undefined;
  if(invocation){try{await withScriptProcessEnvelope(envelope,async original=>{process.exitCode=await native.launchDashboard(WORKSPACE_ROOT,invocation,{mode,control:scriptCompilerAdmissionControlV1(original,"dashboard:launch",0)});});}catch(error){console.error(error instanceof Error?error.message:String(error));process.exitCode=1;}}
  else{
    try { await withScriptProcessEnvelope(envelope, async original => {
      publishBootstrapSources(WORKSPACE_ROOT);
      await new ScriptRouter(WORKSPACE_ROOT).register("nx", NxScript).run(args, original);
    }); } finally { if (inherited === undefined) delete process.env[SCRIPT_PROCESS_INVOCATION_ENV]; else process.env[SCRIPT_PROCESS_INVOCATION_ENV] = inherited; }
  }
}
