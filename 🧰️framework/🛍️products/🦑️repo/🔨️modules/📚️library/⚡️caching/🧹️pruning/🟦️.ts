import{forEachMetadataBatchV1,MetadataContinuationBarrierV1,metadataBatchSizeV1}from"./📦️metadata/🟦️.ts";
import{CacheScanTimingRecorderV1,formatCacheScanTimingV1,cacheScanTimingPrefixV1,type CacheScanTimingPhaseV1}from"./⏱️measurement/🟦️.ts";
import { lstat, readdir, rmdir, unlink } from "node:fs/promises";
import type {Stats} from "node:fs";
import recencySchema from "./⏱️recency/🧬️schema/🔣️.json" with { type: "json" };
import{fitsRecencySourceBytesV1,type RecencyFileV1}from"./⏱️recency/🟦️.ts";
/** 🔬️ Identifies one deterministic maximum from the same physical measurement, without custody authority. */
export type CacheRecencySourceV1=Readonly<RecencyFileV1&{field:"atimeNs"|"mtimeNs"}>;
const recencyPolicy=recencySchema.definitions.ProvenancePolicy.const;
type MeasuredStat=Pick<Stats,"isDirectory"|"isFile"|"isSymbolicLink"|"size"|"atimeMs"|"mtimeMs">&{facts?:Omit<RecencyFileV1,"path">};
const milliseconds=(ns:bigint):number=>Number(ns/1000000000n)*1000+Number(ns%1000000000n)/1000000;
const firstSource=(left:CacheRecencySourceV1|undefined,right:CacheRecencySourceV1|undefined):CacheRecencySourceV1|undefined=>!left?right:!right?left:left.path<right.path||left.path===right.path&&left.field<right.field?left:right;
const measuredSource=(root:string,path:string,stat:MeasuredStat,recencyMs:number):CacheRecencySourceV1|undefined=>{if(!stat.facts)return;const name=relative(root,path).split(sep).join("/");if(name.length>256||Buffer.byteLength(name)>recencyPolicy.maximumSourcePathBytes)return;return{path:name,...stat.facts,field:stat.atimeMs===recencyMs?"atimeNs":"mtimeNs"};};
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import {acquireResourceLease,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {acquireCargoStoreMaintenanceLeaseV1,acquireCargoProfileMaintenanceLeaseV1,acquireCargoStoreLeaseV1,cargoBuildLeaseIdentityV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";

export type CacheUnitKind = "cargo-build" | "cargo-incremental" | "cargo-incremental-session" | "cargo-target-file" | "directory";
/** 📦️ One independently deletable slice of a cache area: a compilation unit, an incremental crate dir, a stale finalized incremental session, an uplifted file, or a whole scratch directory. */
export interface CacheUnit {
  readonly path: string;
  readonly bytes: number;
  readonly recencyMs: number;
  readonly lockHeld: boolean;
  readonly kind: CacheUnitKind;
  readonly recencySource?: CacheRecencySourceV1;
}
export interface CacheAreaInput {
  readonly name: string;
  readonly budgetBytes: number | null;
  readonly unusedAgeMs: number;
  readonly guardAgeMs?: number;
  readonly units: readonly CacheUnit[];
}
export interface AreaPlan {
  readonly name: string;
  readonly totalBytes: number;
  readonly unitCount: number;
  readonly ageDeletions: readonly CacheUnit[];
  readonly budgetDeletions: readonly CacheUnit[];
  readonly retainedBytes: number;
  readonly guardedOverBudgetBytes: number;
}
export interface PrunePlan {
  readonly generatedAtMs: number;
  readonly areas: readonly AreaPlan[];
}

/** 🛡️ Rescans and prunes a shared compiler store only after all admitted profiles release it. */
export async function withCargoCacheMaintenanceV1<T>(options:Readonly<{buildDirectory:string;relatedDirectories?:readonly string[];leaseDirectory:string;control:ResourceLeaseControl;wait:boolean}>,run:()=>Promise<T>):Promise<T>{
  checkResourceLeaseControl(options.control);if(typeof options.wait!=="boolean")throw Error("Explicit Cargo maintenance wait policy required");const signal=options.control.signal,prune=await acquireResourceLease({directory:options.leaseDirectory,resource:"cache-prune",mode:"exclusive",signal,control:options.control,...(!options.wait?{timeoutMs:0}:{})});
  const stores:Awaited<ReturnType<typeof acquireCargoStoreLeaseV1>>[]=[];
  try{for(const buildDirectory of [...new Set([options.buildDirectory,...(options.relatedDirectories??[])].map(path=>cargoBuildLeaseIdentityV1(path,[]).buildDirectory))].sort())stores.push(await(options.wait?acquireCargoStoreLeaseV1({directory:options.leaseDirectory,buildDirectory,mode:"exclusive",signal,control:options.control}):acquireCargoStoreMaintenanceLeaseV1({directory:options.leaseDirectory,buildDirectory,signal,control:options.control})));checkResourceLeaseControl(options.control);const result=await run();checkResourceLeaseControl(options.control);return result;}
  finally{for(const store of stores.reverse())store.release();prune.release();}
}

/**
 * 📐️ Pure age-then-budget eviction over an abstract cache tree; no filesystem access, so it is identically
 * testable in every language. Age deletions run first; if an area still exceeds its budget, the remaining
 * units are evicted oldest-first, but never past the guard age (`recencyMs` within the area's own
 * `guardAgeMs`, falling back to the shared `guardAgeMs`, of `nowMs`) and never while `lockHeld` — those
 * bytes are reported as `guardedOverBudgetBytes` instead of deleted.
 */
export function planCachePrune(areas: readonly CacheAreaInput[], nowMs: number, guardAgeMs: number): PrunePlan {
  const areaPlans = areas.map((area): AreaPlan => {
    const totalBytes = area.units.reduce((sum, unit) => sum + unit.bytes, 0);
    const ageCutoff = nowMs - area.unusedAgeMs;
    const ageDeletions = area.units.filter((unit) => !unit.lockHeld && unit.recencyMs <= ageCutoff);
    const ageDeletedPaths = new Set(ageDeletions.map((unit) => unit.path));
    const remaining = area.units.filter((unit) => !ageDeletedPaths.has(unit.path));
    let remainingBytes = remaining.reduce((sum, unit) => sum + unit.bytes, 0);
    const budgetDeletions: CacheUnit[] = [];
    let guardedOverBudgetBytes = 0;
    if (area.budgetBytes !== null && remainingBytes > area.budgetBytes) {
      const guardCutoff = nowMs - (area.guardAgeMs ?? guardAgeMs);
      for (const unit of [...remaining].sort((left, right) => left.recencyMs - right.recencyMs || (left.path < right.path ? -1 : left.path > right.path ? 1 : 0))) {
        if (remainingBytes <= area.budgetBytes) break;
        if (unit.lockHeld || unit.recencyMs > guardCutoff) { guardedOverBudgetBytes += unit.bytes; continue; }
        budgetDeletions.push(unit);
        remainingBytes -= unit.bytes;
      }
    }
    const deletedBytes = ageDeletions.reduce((sum, unit) => sum + unit.bytes, 0) + budgetDeletions.reduce((sum, unit) => sum + unit.bytes, 0);
    return { name: area.name, totalBytes, unitCount: area.units.length, ageDeletions, budgetDeletions, retainedBytes: totalBytes - deletedBytes, guardedOverBudgetBytes };
  });
  return { generatedAtMs: nowMs, areas: areaPlans };
}

/** 🔢️ Human-readable binary byte count for report text output. */
export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KiB", "MiB", "GiB", "TiB"];
  let value = bytes, index = -1;
  do { value /= 1024; index++; } while (value >= 1024 && index < units.length - 1);
  return `${value.toFixed(2)} ${units[index]}`;
}

class CacheWalk {
  private readonly continuation=new MetadataContinuationBarrierV1();
  private readonly started = Date.now();
  private sourceBytes=0;
  private readonly timing?:CacheScanTimingRecorderV1;
  readonly control: ResourceLeaseControl;
  readonly resource: string;
  readonly mode: "shared" | "exclusive";
  readonly diagnostics: boolean;
  constructor(control: ResourceLeaseControl, resource: string, mode: "shared" | "exclusive",diagnostics=false,timing?:Readonly<{phase:CacheScanTimingPhaseV1;scanner:"build"|"incremental"}>) { this.control=control;this.resource=resource;this.mode=mode;this.diagnostics=diagnostics;checkResourceLeaseControl(control);if(typeof diagnostics!=="boolean")throw Error("Explicit read-only recency diagnostic flag required");if(timing)this.timing=new CacheScanTimingRecorderV1(resource.slice("cache-scan:".length),timing.phase,timing.scanner); }
  stage(value:"discovery"|"measurement"|"enumeration"):void{this.timing?.enter(value);}
  report(complete:boolean,units:number):void{if(this.timing)try{console.log(cacheScanTimingPrefixV1+formatCacheScanTimingV1(this.timing.finish(complete?"complete":"failed",units)));}catch{}}
  async checkpoint(): Promise<void> {
    checkResourceLeaseControl(this.control);
    const pending=this.continuation.checkpoint(async()=>{
      const started=this.timing?.begin("continuation");try{await this.control.publish({resource:this.resource,mode:this.mode,elapsedMs:Date.now()-this.started});checkResourceLeaseControl(this.control);
      await this.control.yieldContinuation();checkResourceLeaseControl(this.control);}finally{if(started!==undefined)this.timing!.end("continuation",started);}
    });if(pending)await pending;checkResourceLeaseControl(this.control);
  }
  async stat(path: string): Promise<Stats | undefined> {
    await this.checkpoint();const started=this.timing?.begin("stat");try { const value=await lstat(path);checkResourceLeaseControl(this.control);return value; } catch(error) { if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error; }finally{if(started!==undefined)this.timing!.end("stat",started);}
  }
  async measurement(path:string):Promise<MeasuredStat|undefined>{
    if(!this.diagnostics)return this.stat(path);await this.checkpoint();const started=this.timing?.begin("measurement");try{const value=await lstat(path,{bigint:true});checkResourceLeaseControl(this.control);return{isDirectory:()=>value.isDirectory(),isFile:()=>value.isFile(),isSymbolicLink:()=>value.isSymbolicLink(),size:Number(value.size),atimeMs:milliseconds(value.atimeNs),mtimeMs:milliseconds(value.mtimeNs),facts:{device:String(value.dev),inode:String(value.ino),bytes:String(value.size),atimeNs:String(value.atimeNs),mtimeNs:String(value.mtimeNs),ctimeNs:String(value.ctimeNs)}};}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}finally{if(started!==undefined)this.timing!.end("measurement",started);}
  }
  retain(source:CacheRecencySourceV1|undefined):CacheRecencySourceV1|undefined{if(!source||source.path.length>256||Buffer.byteLength(source.path)>recencyPolicy.maximumSourcePathBytes)return;const bytes=Buffer.byteLength(JSON.stringify(source));if(!fitsRecencySourceBytesV1(this.sourceBytes,bytes))return;this.sourceBytes+=bytes;return Object.freeze({...source});}
  async entries(path: string): Promise<string[]> {
    await this.checkpoint();const started=this.timing?.begin("entries");try { const value=await readdir(path);checkResourceLeaseControl(this.control);return value; } catch(error) { if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;return []; }finally{if(started!==undefined)this.timing!.end("entries",started);}
  }
}

/**
 * 📏️ Recursively measures one deletable unit's total bytes and most-recent access/modification time.
 * Recency comes only from file timestamps, never directory timestamps — a directory's own mtime changes
 * on unrelated churn (a sibling created or renamed) and would otherwise mask a genuinely stale unit.
 */
async function measureUnit(root: string, walk: CacheWalk): Promise<{ bytes: number; recencyMs: number;recencySource?:CacheRecencySourceV1 }> {
  let bytes = 0, recencyMs = 0,recencySource:CacheRecencySourceV1|undefined;
  const pending = [root];
  while(pending.length){
    const batch:string[]=[];while(pending.length&&batch.length<metadataBatchSizeV1)batch.push(pending.pop()!);
    await forEachMetadataBatchV1(batch,path=>walk.measurement(path),async(path,stat)=>{
    if (!stat || stat.isSymbolicLink()) return;
    if (stat.isDirectory()) { for (const child of await walk.entries(path)) pending.push(join(path, child)); return; }
    if (!stat.isFile()) return;
    const current=Math.max(stat.atimeMs,stat.mtimeMs),source=measuredSource(root,path,stat,current);if(current>recencyMs)recencySource=source;else if(current===recencyMs)recencySource=firstSource(recencySource,source);
    recencyMs = Math.max(recencyMs, stat.atimeMs, stat.mtimeMs);
    bytes += stat.size;
    });
  }
  return { bytes, recencyMs,...(recencySource?{recencySource}:{}) };
}

const posix = (path: string): string => path.split(sep).join("/");

/** 🔎️ Recursively finds every directory literally named `named` under `root`, at any depth (an optional target-triple/profile level in between is handled without guessing). */
async function findNamedDirs(root: string, named: string, walk: CacheWalk): Promise<string[]> {
  const found: string[] = [];
  const pending=[{dir:root,depth:0}];
  while(pending.length){
    const {dir,depth}=pending.pop()!,rootStat=await walk.stat(dir);
    if(depth>8||!rootStat?.isDirectory()||rootStat.isSymbolicLink())continue;
    await forEachMetadataBatchV1(await walk.entries(dir),name=>walk.stat(join(dir,name)),(name,stat)=>{
      const path = join(dir, name);
      if (!stat?.isDirectory() || stat.isSymbolicLink()) return;
      if (name === named) { found.push(path); return; }
      pending.push({dir:path,depth:depth+1});
    });
  }
  return found;
}

/** 🗃️ Walks Cargo's shared build-dir for compiled units: `.../build/<package>/<unit-hash>/`. https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-dir */
export async function scanCargoBuildUnits(buildDir: string, control: ResourceLeaseControl, onUnit?: (unit: CacheUnit) => void,diagnostics=false,timing?:CacheScanTimingPhaseV1): Promise<CacheUnit[]> {
  const walk=new CacheWalk(control,"cache-scan:"+resolve(buildDir),"shared",diagnostics,timing===undefined?undefined:{phase:timing,scanner:"build"});
  const units: CacheUnit[] = [];let complete=false;
  try{const roots=await findNamedDirs(buildDir,"build",walk);walk.stage("enumeration");for (const buildRoot of roots) {
    for (const pkg of await walk.entries(buildRoot)) {
      const packagePath=join(buildRoot,pkg),packageStat=await walk.stat(packagePath);if(!packageStat?.isDirectory()||packageStat.isSymbolicLink())continue;
      for (const hash of await walk.entries(packagePath)) {
      const unitPath = join(buildRoot, pkg, hash);
      const unitStat=await walk.stat(unitPath);
      if (!unitStat?.isDirectory() || unitStat.isSymbolicLink()) continue;
      walk.stage("measurement");const measured = await measureUnit(unitPath,walk);walk.stage("enumeration");const recencySource=walk.retain(measured.recencySource);
      const unit: CacheUnit = { path: posix(relative(buildDir, unitPath)), bytes: measured.bytes, recencyMs: measured.recencyMs, lockHeld: false, kind: "cargo-build",...(recencySource?{recencySource}:{}) };
      units.push(unit);
      onUnit?.(unit);
      checkResourceLeaseControl(control);
      }
    }
  }
  complete=true;return units;}finally{walk.report(complete,units.length);}
}

const sessionLockName = (sessionDirName: string): string => `${sessionDirName.split("-").slice(0, 3).join("-")}.lock`;

/** ➕️ Measures one incremental session (its directory plus its sibling `.lock` file, which rustc keeps alongside even after finalizing). */
async function measureSession(crateDir: string, sessionName: string, walk: CacheWalk): Promise<{ bytes: number; recencyMs: number;recencySource?:CacheRecencySourceV1 }> {
  const measured = await measureUnit(join(crateDir, sessionName),walk);
  let lockBytes = 0, lockRecencyMs = 0;
  const lockPath=join(crateDir,sessionLockName(sessionName)),lockStat=await walk.measurement(lockPath);let source=measured.recencySource;if(lockStat?.isFile()&&!lockStat.isSymbolicLink()){lockBytes=lockStat.size;lockRecencyMs=Math.max(lockStat.atimeMs,lockStat.mtimeMs);const next=measuredSource(join(crateDir,sessionName),lockPath,lockStat,lockRecencyMs);if(lockRecencyMs>measured.recencyMs)source=next;else if(lockRecencyMs===measured.recencyMs)source=firstSource(source,next);}
  return { bytes: measured.bytes + lockBytes, recencyMs: Math.max(measured.recencyMs, lockRecencyMs),...(source?{recencySource:source}:{}) };
}

export interface CargoIncrementalScan {
  readonly units: readonly CacheUnit[];
  readonly staleSessions: readonly CacheUnit[];
}

/**
 * 🔨️ Walks Cargo's shared build-dir for incremental crate dirs: `.../incremental/<crate>-<hash>/`, each holding
 * one session per compile as `s-<timestamp>-<id>-working` (in progress) or a finalized `s-<timestamp>-<id>-<svh>`,
 * with a `s-<timestamp>-<id>.lock` file alongside. https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html
 *
 * `units` is one `cargo-incremental` unit per crate dir, sized to only its newest finalized session plus any
 * in-progress `-working` session (`lockHeld: true` whenever a `-working` session exists — a crate dir must
 * never be deleted while one is present, however old it looks, since a crashed build can leave it stale for a
 * long time without meaning it is safe to remove). `staleSessions` lists every finalized session rustc itself
 * no longer needs (everything but the newest), as independent `cargo-incremental-session` units for
 * unconditional compaction, regardless of the crate dir's own age or lock state.
 */
export async function scanCargoIncrementalUnits(buildDir: string, control: ResourceLeaseControl, onUnit?: (unit: CacheUnit) => void,diagnostics=false,timing?:CacheScanTimingPhaseV1): Promise<CargoIncrementalScan> {
  const walk=new CacheWalk(control,"cache-scan:"+resolve(buildDir),"shared",diagnostics,timing===undefined?undefined:{phase:timing,scanner:"incremental"});
  const units: CacheUnit[] = [];
  const staleSessions: CacheUnit[] = [];let complete=false;
  try{const roots=await findNamedDirs(buildDir,"incremental",walk);walk.stage("enumeration");for (const incrementalRoot of roots) {
    for (const crate of await walk.entries(incrementalRoot)) {
      const crateDir = join(incrementalRoot, crate);
      const crateStat=await walk.stat(crateDir);
      if (!crateStat?.isDirectory() || crateStat.isSymbolicLink()) continue;
      const measured:{name:string;working:boolean;bytes:number;recencyMs:number;recencySource?:CacheRecencySourceV1}[]=[];
      for(const name of await walk.entries(crateDir)){
        if(!name.startsWith("s-")||name.endsWith(".lock"))continue;const stat=await walk.stat(join(crateDir,name));if(!stat?.isDirectory()||stat.isSymbolicLink())continue;
        walk.stage("measurement");const session=await measureSession(crateDir,name,walk);walk.stage("enumeration");const recencySource=walk.retain(session.recencySource);measured.push({name,working:name.endsWith("-working"),bytes:session.bytes,recencyMs:session.recencyMs,...(recencySource?{recencySource}:{})});
      }
      const working = measured.filter((session) => session.working);
      const finalized = measured.filter((session) => !session.working).sort((left, right) => right.recencyMs - left.recencyMs);
      const [newest, ...stale] = finalized;
      for (const session of stale) {
        const recencySource=walk.retain(session.recencySource);
        const unit: CacheUnit = { path: posix(relative(buildDir, join(crateDir, session.name))), bytes: session.bytes, recencyMs: session.recencyMs, lockHeld: false, kind: "cargo-incremental-session",...(recencySource?{recencySource}:{}) };
        staleSessions.push(unit);
        onUnit?.(unit);
      }
      const crateBytes = (newest?.bytes ?? 0) + working.reduce((sum, session) => sum + session.bytes, 0);
      const crateRecencyMs = Math.max(newest?.recencyMs ?? 0, ...working.map((session) => session.recencyMs), 0);
      let maximumSource:CacheRecencySourceV1|undefined;for(const session of [...(newest?[newest]:[]),...working])if(session.recencyMs===crateRecencyMs&&session.recencySource)maximumSource=firstSource(maximumSource,{...session.recencySource,path:posix(join(session.name,session.recencySource.path))});const recencySource=walk.retain(maximumSource);
      const unit: CacheUnit = { path: posix(relative(buildDir, crateDir)), bytes: crateBytes, recencyMs: crateRecencyMs, lockHeld: working.length > 0, kind: "cargo-incremental",...(recencySource?{recencySource}:{}) };
      units.push(unit);
      onUnit?.(unit);
      checkResourceLeaseControl(control);
    }
  }
  complete=true;return { units, staleSessions };}finally{walk.report(complete,units.length+staleSessions.length);}
}

/** 🔒️ Compacts only superseded finalized sessions after admitting their exact compiler profile. */
export async function compactCargoIncrementalSessionsV1(options:Readonly<{buildDirectory:string;leaseDirectory:string;profile:string;control:ResourceLeaseControl;dryRun:boolean;onUnit?:(unit:CacheUnit)=>void}>):Promise<Readonly<{profile:string;paths:readonly string[];bytes:number;dryRun:boolean}>>{
 checkResourceLeaseControl(options.control);const signal=options.control.signal;
 const identity=cargoBuildLeaseIdentityV1(options.buildDirectory,["--profile",options.profile]),prune=await acquireResourceLease({directory:options.leaseDirectory,resource:"cache-prune",mode:"exclusive",signal,control:options.control});
 let compiler:Awaited<ReturnType<typeof acquireCargoProfileMaintenanceLeaseV1>>|undefined;
 try{
  compiler=await acquireCargoProfileMaintenanceLeaseV1({directory:options.leaseDirectory,buildDirectory:identity.buildDirectory,args:["--profile",identity.profile],signal,control:options.control});
  const scan=await scanCargoIncrementalUnits(identity.buildDirectory,options.control);checkResourceLeaseControl(options.control);const working=new Set(scan.units.filter(unit=>unit.lockHeld).map(unit=>unit.path));
  const candidates=scan.staleSessions.filter(unit=>{const parts=unit.path.split("/"),index=parts.indexOf("incremental");return index>0&&parts[index-1]===identity.profile&&!working.has(parts.slice(0,-1).join("/"));}).sort((left,right)=>left.path.localeCompare(right.path));
  let bytes=0;const paths:string[]=[];
  for(const unit of candidates){checkResourceLeaseControl(options.control);options.onUnit?.(unit);checkResourceLeaseControl(options.control);if(!options.dryRun)await deleteUnit(identity.buildDirectory,unit,options.control);paths.push(unit.path);bytes+=unit.bytes;}
  return {profile:identity.profile,paths,bytes,dryRun:options.dryRun};
 }finally{compiler?.release();prune.release();}
}

const CARGO_SENTINEL_NAMES = new Set(["CACHEDIR.TAG"]);
const isCargoSentinel = (name: string): boolean => CARGO_SENTINEL_NAMES.has(name) || name.startsWith(".cargo-");

/** 🎯️ Walks Cargo's uplifted target-dir; under the new build-dir layout it holds only final deliverable files, so each file is its own unit. */
export async function scanCargoTargetUnits(targetDir: string, control: ResourceLeaseControl, onUnit?: (unit: CacheUnit) => void): Promise<CacheUnit[]> {
  const walk=new CacheWalk(control,"cache-scan:"+resolve(targetDir),"shared");
  const units: CacheUnit[] = [];
  const pending=[targetDir];
  while(pending.length){
    const dir=pending.pop()!,directoryStat=await walk.stat(dir);if(!directoryStat?.isDirectory()||directoryStat.isSymbolicLink())continue;
    for (const name of await walk.entries(dir)) {
      const path = join(dir, name);
      const stat=await walk.stat(path);
      if (!stat || stat.isSymbolicLink()) continue;
      if (stat.isDirectory()) { pending.push(path); continue; }
      if (!stat.isFile() || isCargoSentinel(name)) continue;
      const unit: CacheUnit = { path: posix(relative(targetDir, path)), bytes: stat.size, recencyMs: Math.max(stat.atimeMs, stat.mtimeMs), lockHeld: false, kind: "cargo-target-file" };
      units.push(unit);
      onUnit?.(unit);
      checkResourceLeaseControl(control);
    }
  }
  return units;
}

/** 📂️ Walks one cache-root area whose units are its immediate child directories (Vite consumer caches, stray agent scratch dirs). */
export async function scanDirectoryUnits(areaRoot: string, control: ResourceLeaseControl, exclude: ReadonlySet<string> = new Set(), onUnit?: (unit: CacheUnit) => void): Promise<CacheUnit[]> {
  const walk=new CacheWalk(control,"cache-scan:"+resolve(areaRoot),"shared"),rootStat=await walk.stat(areaRoot);
  if(!rootStat?.isDirectory()||rootStat.isSymbolicLink())return [];
  const units: CacheUnit[] = [];
  for (const name of await walk.entries(areaRoot)) {
    if (exclude.has(name)) continue;
    const path = join(areaRoot, name);
    const stat=await walk.stat(path);
    if (!stat?.isDirectory() || stat.isSymbolicLink()) continue;
    const measured = await measureUnit(path,walk);
    const unit: CacheUnit = { path: name, bytes: measured.bytes, recencyMs: measured.recencyMs, lockHeld: false, kind: "directory" };
    units.push(unit);
    onUnit?.(unit);
    checkResourceLeaseControl(control);
  }
  return units;
}

/** 🗑️ Deletes one previously scanned unit: a flat target-dir file prunes now-empty parent directories back to the area root; a stale incremental session also removes its sibling `.lock` file. */
export async function deleteUnit(areaRoot: string, unit: CacheUnit, control: ResourceLeaseControl): Promise<void> {
  const walk=new CacheWalk(control,"cache-delete:"+resolve(areaRoot),"exclusive"),root=resolve(areaRoot),parts=unit.path.split("/");
  if(!unit.path||isAbsolute(unit.path)||parts.some(part=>!part||part==="."||part===".."||part.includes("\\")||part.includes("\0")))throw Error("Cache unit must remain inside its area");
  const rootStat=await walk.stat(root);if(!rootStat)return;const device=rootStat.dev;requireCacheFilesystemDeviceV1(device,device);const absolute=join(root,...parts);let ancestor=root;
  for(const part of ["",...parts.slice(0,-1)]){ancestor=part?join(ancestor,part):ancestor;const stat=await walk.stat(ancestor);if(!stat)return;requireCacheFilesystemDeviceV1(device,stat.dev);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Cache unit parent must remain a physical directory");}
  const removeFile=async(path:string):Promise<void>=>{await walk.checkpoint();try{await unlink(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}checkResourceLeaseControl(control);};
  if (unit.kind === "cargo-target-file") {
    const stat=await walk.stat(absolute);if(!stat)return;requireCacheFilesystemDeviceV1(device,stat.dev);if(!stat.isFile()||stat.isSymbolicLink())throw Error("Cargo target unit must remain a physical file");await removeFile(absolute);
    let parent = dirname(absolute);
    for (;;) {
      if (parent === root || relative(root, parent).startsWith("..")) break;
      if ((await walk.entries(parent)).length > 0) break;
      await walk.checkpoint();try { await rmdir(parent); } catch(error) { if(!["ENOENT","ENOTEMPTY","EEXIST"].includes((error as NodeJS.ErrnoException).code??""))throw error;break; }
      parent = dirname(parent);
    }
    return;
  }
  const pending:{path:string;visited:boolean}[]=[{path:absolute,visited:false}];
  while(pending.length){
    const {path,visited}=pending.pop()!,stat=await walk.stat(path);if(!stat)continue;requireCacheFilesystemDeviceV1(device,stat.dev);
    if(stat.isSymbolicLink()){if(path===absolute)throw Error("Cache unit must remain a physical directory");await removeFile(path);continue;}
    if(!stat.isDirectory()){await removeFile(path);continue;}
    if(visited){await walk.checkpoint();try{await rmdir(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}checkResourceLeaseControl(control);continue;}
    pending.push({path,visited:true});for(const child of await walk.entries(path))pending.push({path:join(path,child),visited:false});
  }
  if(unit.kind==="cargo-incremental-session")await removeFile(join(dirname(absolute),sessionLockName(basename(absolute))));
}

function requireCacheFilesystemDeviceV1(root:number,entry:number):void{if(!Number.isSafeInteger(root)||root<0||!Number.isSafeInteger(entry)||entry<0||entry!==root)throw Error("Cache deletion must remain on its original filesystem");}
