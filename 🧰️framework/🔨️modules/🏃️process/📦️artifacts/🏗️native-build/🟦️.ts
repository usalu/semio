import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import { stageArtifacts } from "../📤️publication/🟦️.ts";
import { terminateOwnedChildTree } from "../../🪓️termination/🟦️.ts";
import { startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";
import { chmodSync, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, readlinkSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve, sep, isAbsolute, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn, spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { homedir } from "node:os";
import { createInterface } from "node:readline";
import {lstat,mkdir,writeFile} from "node:fs/promises";
import {CurrentPhysicalOwnerV1,type CurrentPhysicalPortV1} from "../../../📁️filesystem/🧾️observation/📁️current/🟦️.ts";

import { acquireCargoBuildLeaseV1 } from "./🔒️lease/🟦️.ts";


/** ✍️ Re-signs a native executable on macOS, ad-hoc, and does nothing anywhere else.
 *
 * A Mach-O binary carries its code signature inside the file, and the kernel validates it against
 * the bytes on disk. A copied or rewritten executable whose signature no longer matches is not
 * rejected with a message — it is `SIGKILL`ed without one. Signing every distribution copy ad-hoc
 * (`--sign -`) is what keeps that from happening. This is *not* Developer ID signing or
 * notarization: a binary signed this way still trips Gatekeeper when it arrives from the internet
 * on someone else's Mac, which needs a signing identity this repository does not have. */
export function signExecutableForDistribution(binary: string): void {
  if (process.platform !== "darwin") return;
  const signed = spawnSync("codesign", ["--force", "--sign", "-", binary], { stdio: ["ignore", "inherit", "inherit"] });
  if (signed.status !== 0) throw new Error(`codesign refused ${binary} (status ${signed.status ?? "unknown"})`);
}

/** 📥️ Places an executable at `destination`: remove, copy, then sign — never an overwrite in place.
 *
 * Writing over a running or previously-signed Mach-O binary is the exact shape that produces a
 * silent `SIGKILL` on the next launch, so the old inode is unlinked first and the fresh copy is
 * signed afterwards. */
export function installExecutable(source: string, destination: string): void {
  mkdirSync(dirname(destination), { recursive: true });
  rmSync(destination, { force: true });
  copyFileSync(source, destination);
  chmodSync(destination, lstatSync(source).mode & 0o777);
  signExecutableForDistribution(destination);
}

/** 🚚️ Packages one already-built native executable as a versioned, checksummed local tarball.
 *
 * This is the last mile a release build was missing: a compiled binary sitting in a project's
 * `dist/` is not something an operator can be handed. It produces
 * `<output>/<name>-<version>-<platform>-<arch>.tar.gz` plus a `.sha256` next to it, and **uploads
 * nothing anywhere** — where the artifact goes afterwards is a deployment decision this repository
 * does not take. Returns the tarball path. */
export function packageNativeRelease(options: { readonly binary: string; readonly name: string; readonly version: string; readonly output: string; readonly platform?: string; readonly arch?: string }): string {
  const platform = options.platform ?? process.platform;
  const arch = options.arch ?? process.arch;
  if (!lstatSync(options.binary).isFile()) throw new Error(`no release binary at ${options.binary}`);
  mkdirSync(options.output, { recursive: true });
  const filename = `${options.name}-${options.version}-${platform}-${arch}.tar.gz`;
  const tarball = join(options.output, filename);
  const payload = mkdtempSync(join(options.output, "payload-"));
  try {
    installExecutable(options.binary, join(payload, `${options.name}-${options.version}`, options.name));
    rmSync(tarball, { force: true });
    const archived = spawnSync("tar", ["-czf", tarball, "-C", payload, `${options.name}-${options.version}`], { stdio: ["ignore", "inherit", "inherit"] });
    if (archived.status !== 0) throw new Error(`tar refused ${tarball} (status ${archived.status ?? "unknown"})`);
  } finally {
    rmSync(payload, { recursive: true, force: true });
  }
  const digest = createHash("sha256").update(readFileSync(tarball)).digest("hex");
  writeFileSync(`${tarball}.sha256`, `${digest}  ${filename}\n`);
  console.log(`[publish] ${tarball}\n[publish] sha256 ${digest}`);
  return tarball;
}

/** 🪢️ The process a Cargo build belongs to, by pid. A launcher that builds on someone's behalf (the
 * MCP gateway staging its own binary inside a client's `initialize`) names itself here, and the build
 * stops the moment that owner is gone — killed by its client, timed out by a gate — instead of
 * outliving it as an orphan that holds the shared build-dir locks. */
export const CARGO_BUILD_OWNER_PID_ENV = "SEMIO_BUILD_OWNER_PID";

/** 🪢️ Whether the owner named by {@link CARGO_BUILD_OWNER_PID_ENV} is still alive; `true` when none is named. */
export function cargoBuildOwnerAliveV1(env: Readonly<Record<string,string|undefined>> = process.env): boolean {
  const owner = Number(env[CARGO_BUILD_OWNER_PID_ENV] ?? "");
  if (!Number.isSafeInteger(owner) || owner <= 0) return true;
  try {
    process.kill(owner, 0);
    return true;
  } catch (error) {
    return (error as NodeJS.ErrnoException).code === "EPERM";
  }
}

/** 🧾️ Schema id of the sources record a staged Cargo executable can carry: every source file Cargo
 * compiled it from (its own dep-info) and when the build that produced it started. */
export const CARGO_BINARY_SOURCES_SCHEMA_V1 = "semio.cargo.binary-sources/v1";

/** 🧾️ One staged executable's sources record. */
export interface CargoBinarySourcesV1 {
  readonly schema: typeof CARGO_BINARY_SOURCES_SCHEMA_V1;
  readonly builtAtMs: number;
  readonly sources: readonly string[];
}

/** 📜️ The source files a Cargo dep-info file (`<artifact>.d`) names for its target, with Cargo's
 * `\ ` space escape undone. Only the first `target: deps` rule counts; the empty per-dependency
 * rules Cargo appends after it carry nothing. */
export function cargoDepInfoSourcesV1(text: string): readonly string[] {
  const rule = text.split(/\r?\n/u).find((line) => line.includes(": ")) ?? "";
  return rule
    .slice(rule.indexOf(": ") + 2)
    .split(/(?<!\\) /u)
    .filter((entry) => entry.length > 0)
    .map((entry) => entry.replace(/\\ /gu, " "));
}

/** 🧾️ Reads a sources record, answering `null` for anything that is not exactly one. */
export function parseCargoBinarySourcesV1(text: string): CargoBinarySourcesV1 | null {
  try {
    const value = JSON.parse(text) as { schema?: unknown; builtAtMs?: unknown; sources?: unknown };
    if (value.schema !== CARGO_BINARY_SOURCES_SCHEMA_V1 || typeof value.builtAtMs !== "number" || !Number.isFinite(value.builtAtMs) || !Array.isArray(value.sources) || value.sources.length === 0 || !value.sources.every((source) => typeof source === "string" && source.length > 0)) return null;
    return { schema: CARGO_BINARY_SOURCES_SCHEMA_V1, builtAtMs: value.builtAtMs, sources: value.sources as string[] };
  } catch {
    return null;
  }
}

/** 🕰️ Whether a staged executable is still what its sources build: every source it was compiled from
 * still exists and none was modified after its build started — Cargo's own freshness rule, over the
 * whole dependency closure rather than one crate. `modifiedAtMs` answers `null` for a missing file. A
 * touched but unchanged file reads as `changed`, which costs a no-op Cargo build and never serves a
 * stale binary. */
export function cargoBinarySourcesFreshnessV1(record: CargoBinarySourcesV1, modifiedAtMs: (path: string) => number | null): { readonly fresh: boolean; readonly changed: string | null } {
  for (const source of record.sources) {
    const modified = modifiedAtMs(source);
    if (modified === null || modified > record.builtAtMs) return { fresh: false, changed: source };
  }
  return { fresh: true, changed: null };
}

/** 🏗️ Supplies exact caller-owned compiler storage and execution limits. */
export type CargoArtifactBuildPolicyV1 = Readonly<{ version: 1; cwd: string; buildDirectory: string; leaseDirectory: string; captureDirectory: string; budgetMs: number }>;
const artifactPolicySchema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
function admitArtifactPolicy(value: unknown): CargoArtifactBuildPolicyV1 {
  const errors = validateJsonSchemaSubset(artifactPolicySchema, value);
  if (errors.length) throw Error(`Invalid Cargo artifact policy: ${errors.join("; ")}`);
  return value as CargoArtifactBuildPolicyV1;
}
/** 🔐️ Admits only an explicit compiler policy for the selected source owner. */
export function readCargoArtifactBuildPolicyV1(environment: Readonly<Record<string,string|undefined>>, cwd: string): CargoArtifactBuildPolicyV1 {
  if (!environment.SEMIO_CARGO_ARTIFACT_POLICY) throw Error("Explicit Cargo artifact policy required");
  const policy = admitArtifactPolicy(JSON.parse(environment.SEMIO_CARGO_ARTIFACT_POLICY));
  if (resolve(policy.cwd) !== resolve(cwd)) throw Error("Cargo artifact policy belongs to a different owner");
  return policy;
}

/** 📦️ Captures Cargo's declared deliverables, including link dependencies, without copying compiler state.
 * `sourcesRecord` names a {@link CargoBinarySourcesV1} file staged beside the selected executable, in
 * the same atomic publication, so a consumer can tell a stale executable from a fresh one without
 * running Cargo. Cargo's stderr is piped and forwarded, never inherited: Bun marks its own stderr
 * `O_NONBLOCK` once written, an inherited pipe shares that flag, and a Cargo burst (replayed warnings
 * of fresh units) then fails with `EAGAIN` as soon as a slow reader lets the 64 KiB pipe fill — the
 * build dies with its diagnostics cut mid-line (ticket 26/09/23 W4, `wp-w4/w4-nonblock-probe.ts`). */
export type CargoArtifactBuildOptionsV1 = { readonly command?: "build" | "rustc"; readonly output?: string; readonly sourcesRecord?: string; readonly validate?: (files: ReadonlyMap<string, string>) => void; readonly signal?: AbortSignal; readonly environment?: Readonly<Record<string,string|undefined>>; readonly commandPort?: Readonly<{ command: string; args: readonly string[] }> };

/** 🚦️ Holds the shared profile lease through compiler shutdown and artifact capture, including queue cancellation. */
export async function buildCargoArtifacts(manifest: string, args: string[], policy: CargoArtifactBuildPolicyV1, options: CargoArtifactBuildOptionsV1 = {}): Promise<void> {
  policy = admitArtifactPolicy(policy);
  if (!isAbsolute(manifest)) throw Error("Cargo artifact manifest must be absolute");
  const budget = policy.budgetMs;
  if (!Number.isFinite(budget) || budget < 0) throw Error("Invalid Cargo build budget");
  const controller = new AbortController();
  const abort = () => controller.abort();
  options.signal?.addEventListener("abort", abort, { once: true });
  process.once("SIGINT", abort);
  process.once("SIGTERM", abort);
  const observeOwner = () => { if (!cargoBuildOwnerAliveV1(options.environment ?? process.env)) controller.abort(); };
  observeOwner();
  const ownerWatch = setInterval(observeOwner, 1000);
  const expiry = budget > 0 ? setTimeout(abort, budget) : undefined;
  let lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined;
  try {
    if (options.signal?.aborted) abort();
    lease = await acquireCargoBuildLeaseV1({ directory: policy.leaseDirectory, buildDirectory: policy.buildDirectory, args, signal: controller.signal });
    controller.signal.throwIfAborted();
    await captureCargoArtifacts(manifest, args, policy, { ...options, signal: controller.signal });
  } finally {
    try { lease?.release(); }
    finally {
      clearInterval(ownerWatch);
      if (expiry) clearTimeout(expiry);
      options.signal?.removeEventListener("abort", abort);
      process.removeListener("SIGINT", abort);
      process.removeListener("SIGTERM", abort);
    }
  }
}

/** 🗂️ Observes actual directory names, entry kinds and raw link targets in portable UTF-8 order. */
export function cargoDirectoryEntriesV1(path: string): Array<[string, string, string | null]> | null {
  try { return readdirSync(path).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))).map(name => { const child = join(path, name), entry = lstatSync(child); return [name, entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other", entry.isSymbolicLink() ? readlinkSync(child) : null]; }); } catch { return null; }
}

/** 🧮️ Distinguishes compiler file bytes from canonical directory input rosters without guessing absent inputs. */
export function cargoInputDigestV1(path: string): { path: string; kind: "file" | "directory" | null; sha256: string | null } {
  try {
    const entry = lstatSync(path), kind = entry.isFile() ? "file" : entry.isDirectory() ? "directory" : null, entries = kind === "directory" ? cargoDirectoryEntriesV1(path) : null;
    return { path, kind, sha256: kind === "file" ? createHash("sha256").update(readFileSync(path)).digest("hex") : entries ? createHash("sha256").update(JSON.stringify(entries)).digest("hex") : null };
  } catch { return { path, kind: null, sha256: null }; }
}

const resourceReadSchema = JSON.parse(readFileSync(new URL("./📥️resources/🧬️schema/🔣️.json",import.meta.url),"utf8"));
/** 📋️ Retains actual completed Cargo compiler, dep-info and resource observations without changing build or staging selection. */
export type CargoCompilerUnitEvidenceV1=Readonly<{version:1;kind:"discovery";paths:readonly string[];producer:null}|{version:1;kind:"received";paths:readonly string[];producer:Readonly<{path:string;sha256:string;unit:number}>}>;
const unitEvidenceSchema=JSON.parse(readFileSync(new URL("./🧬️schema/🧾️unit-evidence/🔣️.json",import.meta.url),"utf8"));
/** 🧾️ Requires exact producer custody or explicitly funded compiler discovery. */
export function parseCargoCompilerUnitEvidenceV1(value:unknown):CargoCompilerUnitEvidenceV1{if(validateJsonSchemaSubset(unitEvidenceSchema,value).length)throw Error("Complete compiler unit evidence required");const row=value as CargoCompilerUnitEvidenceV1;if(row.paths.some(path=>!isAbsolute(path)||path.length>256)||row.kind==="discovery"&&(row.paths.length!==0||row.producer!==null)||row.kind==="received"&&(!row.paths.length||!row.producer||!isAbsolute(row.producer.path)||row.producer.path.length>256||!Number.isSafeInteger(row.producer.unit)))throw Error("Compiler unit evidence owner refused");return Object.freeze({...row,paths:Object.freeze([...row.paths]),producer:row.producer===null?null:Object.freeze({...row.producer})}) as CargoCompilerUnitEvidenceV1;}

export async function writeCompletedCargoInvocationProvenanceV1(receiptPath: string, invocation: { manifest: string; cwd: string; command: string; args: string[]; buildDirectory: string | null; builtAtMs: number; status: number; cancelled: boolean; units: any[]; buildScripts: any[] }, stagedPaths: ReadonlyMap<string, string>, cargoHome: string, physical: CurrentPhysicalPortV1): Promise<void> {
  if(!physical||!physical.control||!["read","digest","directoryEntries","checkpoint","recheck"].every(key=>typeof (physical as any)[key]==="function"))throw Error("Complete current physical provenance authority required");
  const policy=physical.control;if(![policy.maxBytes,policy.maxWork,policy.chunkBytes].every(Number.isSafeInteger)||policy.maxBytes<0||policy.maxWork<1||policy.chunkBytes<1||!["cancelled","remainingMs","onProgress"].every(key=>typeof(policy as any)[key]==="function"))throw Error("Finite current physical provenance control required");
  const path=(value:string):string=>{if(typeof value!=="string"||!isAbsolute(value)||value.length>256||value.includes("\0"))throw Error("Physical provenance path refused");return resolve(value);};
  path(receiptPath);path(cargoHome);path(invocation.cwd);path(invocation.manifest);if(invocation.buildDirectory)path(invocation.buildDirectory);
  const kinds=new Map<string,"file"|"directory"|null>(),bytes=new Map<string,Uint8Array|null>(),digests=new Map<string,string|null>(),rosters=new Map<string,readonly import("../../../📁️filesystem/🧾️observation/📁️current/🟦️.ts").CurrentPhysicalEntryV1[]|null>();
  const kind=async(value:string):Promise<"file"|"directory"|null>=>{const file=path(value);if(kinds.has(file))return kinds.get(file)!;await physical.checkpoint();let result:"file"|"directory"|null=null;try{const stat=await lstat(file);if(stat.isSymbolicLink())throw Error("Physical provenance input cannot be a symbolic link");result=stat.isFile()?"file":stat.isDirectory()?"directory":null;}catch(error){if(!["ENOENT","ENOTDIR"].includes((error as {code?:string}).code??""))throw error;await physical.digest(file);}await physical.checkpoint();kinds.set(file,result);return result;};
  const read=async(value:string):Promise<Uint8Array|null>=>{const file=path(value);if(bytes.has(file))return bytes.get(file)!;if(await kind(file)!=="file"){bytes.set(file,null);return null;}const actual=await physical.read(file);if(actual===undefined)throw Error("Physical provenance input disappeared");const valueBytes=typeof actual==="string"?Buffer.from(actual):actual;bytes.set(file,valueBytes);digests.set(file,createHash("sha256").update(valueBytes).digest("hex"));return valueBytes;};
  const text=async(value:string):Promise<string|null>=>{const valueBytes=await read(value);return valueBytes===null?null:Buffer.from(valueBytes).toString("utf8");};
  const entries=async(value:string)=>{const file=path(value);if(rosters.has(file))return rosters.get(file)!;if(await kind(file)!=="directory"){rosters.set(file,null);return null;}const rows=await physical.directoryEntries(file);if(!rows)throw Error("Physical provenance directory disappeared");rosters.set(file,rows);digests.set(file,createHash("sha256").update(JSON.stringify(rows.map(row=>[basename(row.path),row.kind,row.symlinkTarget]))).digest("hex"));return rows;};
  const digest=async(value:string):Promise<string|null>=>{const file=path(value);if(digests.has(file))return digests.get(file)!;const type=await kind(file);if(type==="directory")await entries(file);else if(type==="file"){const actual=await physical.digest(file);if(actual===undefined)throw Error("Physical provenance input disappeared");digests.set(file,actual);}else digests.set(file,null);return digests.get(file)!;};
  const sha256=async(value:string):Promise<string|null>=>await kind(value)==="file"?await digest(value):null;
  const input=async(value:string)=>({path:value,kind:await kind(value),sha256:await digest(value)});
  const resource=async(observation:any):Promise<any>=>{const actual=await input(observation.path);if(observation.kind==="read"){const errors=validateJsonSchemaSubset(resourceReadSchema,observation);const valueBytes=await read(observation.path),length=valueBytes?.byteLength??null;return{input:observation,bytes:length,sha256:actual.sha256,schemaErrors:errors,sourceExact:errors.length===0&&actual.kind==="file"&&length===observation.bytes&&actual.sha256===observation.sha256};}const rows=observation.kind==="directory"?await entries(observation.path):null;return{input:observation,sha256:actual.sha256,...(observation.kind==="copy"?{outputSha256:typeof observation.output==="string"?await sha256(observation.output):null}:{}),...(rows?{observedEntries:rows}: {})};};
  const extension=/\.(?:rlib|rmeta|so|dylib|dll|lib|wasm|exe)$/u,units:any[]=[];
  for(const original of invocation.units){
    await physical.checkpoint();const evidence=parseCargoCompilerUnitEvidenceV1(original.evidence),unit={...original},message=unit.message,candidates:string[]=[...new Set<string>((message.filenames??[]).flatMap((file:string)=>[file.replace(extension,"")+".d",join(dirname(file),basename(file).replace(/^lib/u,"").replace(extension,"")+".d")]))];
    delete unit.evidence;let producerUnit:any=null;if(evidence.kind==="received"){const source=await text(evidence.producer.path);if(source===null||await sha256(evidence.producer.path)!==evidence.producer.sha256)throw Error("Compiler producer receipt changed");const producer=JSON.parse(source);for(const field of["manifest","cwd","command","args","buildDirectory","builtAtMs","status","cancelled"])if(JSON.stringify(producer[field])!==JSON.stringify((invocation as any)[field]))throw Error("Compiler producer invocation differs");producerUnit=producer.units?.[evidence.producer.unit];if(!producerUnit||JSON.stringify(producerUnit.message)!==JSON.stringify(message)||JSON.stringify(producerUnit.depInfo?.map((row:any)=>row.path))!==JSON.stringify(evidence.paths))throw Error("Compiler producer unit custody differs");candidates.splice(0,candidates.length,...evidence.paths);}
    const sourceOwners=new Set<string>(),targetSource=resolve(message.target.src_path);for(const manifest of[invocation.manifest,message.manifest_path].filter((value):value is string=>typeof value==="string"))for(let owner=dirname(resolve(manifest));;){if(await kind(join(owner,"Cargo.toml"))==="file")sourceOwners.add(owner);const parent=dirname(owner);if(parent===owner)break;owner=parent;}
    const matchesSource=async(source:string):Promise<boolean>=>{for(const value of cargoDepInfoSourcesV1(source)){await physical.checkpoint();if(isAbsolute(value)){if(resolve(value)===targetSource)return true;continue;}for(const owner of sourceOwners){await physical.checkpoint();if(resolve(owner,value)===targetSource)return true;}}return false;};
    if(evidence.kind==="discovery"&&invocation.buildDirectory){const option=(name:string)=>invocation.args.flatMap((value,index)=>value===name?[invocation.args[index+1]!]:value.startsWith(name+"=")?[value.slice(name.length+1)]:[])[0],packageName=/#([^@]+)@/u.exec(message.package_id??"")?.[1];if(packageName){const target=option("--target"),owner=join(invocation.buildDirectory,...(target?[target]:[]),option("--profile")??(invocation.args.includes("--release")?"release":"debug"),"build",packageName),rows=await entries(owner);if(rows)for(const row of rows){const output=join(row.path,"out");if(await kind(output)!=="directory")continue;for(const file of message.filenames??[]){const internal=join(output,basename(file)),dep=internal.replace(extension,"")+".d",depText=await text(dep);if(!depText?.includes("# checksum:")||!await matchesSource(depText))continue;const selected=await sha256(file);if(selected===null||await sha256(internal)!==selected)continue;candidates.push(dep);}}}}
    const depInfo:any[]=[];for(const file of new Set(candidates)){const depText=await text(file);if(depText===null){if(evidence.kind==="received")throw Error("Compiler received candidate disappeared");continue;}if(evidence.kind==="received"&&producerUnit.depInfo.find((row:any)=>row.path===file)?.text!==depText)throw Error("Compiler received candidate bytes differ");const sources=cargoDepInfoSourcesV1(depText),relativeSources=sources.filter(source=>!isAbsolute(source)).map(source=>normalize(source)),owners=new Set<string>();for(const manifest of[invocation.manifest,message.manifest_path].filter((value):value is string=>typeof value==="string"))for(let owner=dirname(resolve(manifest));;){if(await kind(join(owner,"Cargo.toml"))==="file")owners.add(owner);const parent=dirname(owner);if(parent===owner)break;owner=parent;}const bases:string[]=[];for(const base of owners){if(!relativeSources.some(source=>resolve(base,source)===resolve(message.target.src_path)))continue;let valid=true;for(const source of sources){if(await kind(resolve(base,source))===null){valid=false;break;}}if(valid)bases.push(base);}depInfo.push({path:file,text:depText,baseDirectory:relativeSources.length===0?invocation.cwd:bases.length===1?bases[0]:null});}
    unit.depInfo=depInfo;unit.inputs=[];for(const file of new Set<string>(depInfo.flatMap(row=>cargoDepInfoSourcesV1(row.text).filter(source=>isAbsolute(source)||row.baseDirectory!==null).map(source=>resolve(row.baseDirectory??invocation.cwd,source)))))unit.inputs.push(await input(file));unit.observedAtMs=Date.now();unit.artifacts=[];for(const file of message.filenames??[]){const stagedPath=stagedPaths.get(file);unit.artifacts.push({path:file,sha256:await sha256(file),...(stagedPath?{stagedPath,stagedSha256:await sha256(stagedPath)}:{})});}if(producerUnit){if(JSON.stringify(unit.depInfo)!==JSON.stringify(producerUnit.depInfo)||JSON.stringify(unit.inputs)!==JSON.stringify(producerUnit.inputs)||JSON.stringify(unit.artifacts.map((row:any)=>[row.path,row.sha256]))!==JSON.stringify(producerUnit.artifacts.map((row:any)=>[row.path,row.sha256])))throw Error("Compiler received physical witnesses differ");}units.push(unit);
  }
  const buildResources:any[]=[];for(const build of invocation.buildScripts){const file=join(build.out_dir,"semio-runtime-resource-inputs.jsonl"),source=await text(file);if(source===null)continue;const resources:any[]=[];for(const line of source.split(/\r?\n/u).filter(Boolean))resources.push(await resource(JSON.parse(line)));buildResources.push({package_id:build.package_id,out_dir:build.out_dir,path:file,text:source,sha256:await sha256(file),observedAtMs:Date.now(),resources});}
  const compilerResourceRoot=invocation.buildDirectory?join(invocation.buildDirectory,"semio-compiler-resources"):null,compilerResources:any[]=[];
  const compilerPaths=compilerResourceRoot?[...new Set<string>(units.filter(unit=>!unit.message.target.kind?.includes("proc-macro")&&!unit.message.target.kind?.includes("custom-build")).flatMap(unit=>unit.inputs.map((row:any)=>row.path)))].filter(file=>basename(file)==="observation.json"&&relative(compilerResourceRoot,file).split(sep)[0]!==".."&&!isAbsolute(relative(compilerResourceRoot,file))):[];
  for(const file of compilerPaths){const source=await text(file);let observation:any;try{observation=source===null?null:JSON.parse(source);}catch{observation=null;}const bind=(identity:any,producer:boolean)=>{if(!identity||![identity.manifest,identity.source].every(value=>typeof value==="string"&&isAbsolute(value)))return null;const matches=units.filter(unit=>typeof unit.message.manifest_path==="string"&&resolve(unit.message.manifest_path)===resolve(identity.manifest)&&unit.inputs.some((row:any)=>row.kind==="file"&&resolve(row.path)===resolve(identity.source))&&(producer?unit.message.target.kind.includes("proc-macro"):typeof identity.crate==="string"&&unit.message.target.name.replaceAll("-","_")===identity.crate&&!unit.message.target.kind.includes("custom-build")&&!unit.message.target.kind.includes("proc-macro")));return matches.length===1?matches[0].message:null;},resources:any[]=[];for(const row of Array.isArray(observation?.resources)?observation.resources:[])resources.push(await resource(row));compilerResources.push({path:file,text:source,sha256:await sha256(file),observedAtMs:Date.now(),producer:observation?.producer??null,caller:observation?.caller??null,producerUnit:bind(observation?.producer,true),callerUnit:bind(observation?.caller,false),resources});}
  const manifests=new Set<string>([invocation.manifest,...units.map(unit=>unit.message.manifest_path).filter((value):value is string=>typeof value==="string")]),inputPaths=new Set<string>(manifests),visited=new Set<string>();for(let file of[invocation.cwd,...[...manifests].map(value=>dirname(value))])for(;;){if(visited.has(file))break;visited.add(file);for(const name of["Cargo.toml","Cargo.lock",".cargo/config.toml",".cargo/config","rust-toolchain.toml","rust-toolchain"])inputPaths.add(join(file,name));const parent=dirname(file);if(parent===file)break;file=parent;}for(const name of["config.toml","config"])inputPaths.add(join(cargoHome,name));const invocationInputs:any[]=[];for(const file of[...inputPaths].sort())invocationInputs.push({path:file,sha256:await sha256(file)});
  const source=JSON.stringify({version:1,...invocation,units,observedAtMs:Date.now(),invocationInputs,buildResources,compilerResourceRoot,compilerResources})+"\n";if(Buffer.byteLength(source)>policy.maxBytes)throw Error("Physical provenance publication budget");await physical.recheck();for(const file of new Set([receiptPath,...(invocation.buildDirectory?[join(invocation.buildDirectory,"semio-cargo-provenance",basename(receiptPath))]:[])])){path(file);await physical.checkpoint();await mkdir(dirname(file),{recursive:true});await physical.checkpoint();await writeFile(file,source);}

}

async function captureCargoArtifacts(manifest: string, args: string[], policy: CargoArtifactBuildPolicyV1, options: CargoArtifactBuildOptionsV1): Promise<void> {
  options.signal?.throwIfAborted();
  const path = resolve(manifest);
  const sourceRoot = dirname(path);
  const staging = resolve(sourceRoot, options.output ?? "dist/build");
  if (!staging.startsWith(sourceRoot + sep)) throw new Error("Cargo deliverables must belong to their source project");
  const captureParent = policy.captureDirectory;
  mkdirSync(captureParent, { recursive: true });
  const capture = mkdtempSync(join(captureParent, "cargo-artifacts-"));
  const owner = relative(policy.cwd, path).split(sep).join("/");
  const files = new Map<string, string>();
  const dependencies = new Map<string, string>();
  const provenanceRoot = (options.environment ?? process.env).SEMIO_TEST_ARTIFACT_DIR;
  const units: any[] = [], buildScripts: any[] = [], stagedNames = new Map<string, string>();
  let hasLibrary = false;
  let primaryExecutable: string | undefined;
  let cancelled = false;
  let forceKill: ReturnType<typeof setTimeout> | undefined;
  const delimiter = args.indexOf("--"),
    compilerArgs = delimiter < 0 ? [] : args.slice(delimiter),
    cargoArgs = delimiter < 0 ? args : args.slice(0, delimiter);
  const builtAtMs = Date.now();
  const commandPort = options.commandPort ?? { command: "cargo", args: [] };
  const child = spawn(commandPort.command, [...commandPort.args, options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], {
    cwd: policy.cwd,
    env: { ...(options.environment ?? process.env), SEMIO_COMPILER_RESOURCE_ROOT: join(policy.buildDirectory, "semio-compiler-resources"), CARGO_TARGET_DIR: join(capture, "target"), CARGO_BUILD_BUILD_DIR: policy.buildDirectory },
    detached: process.platform !== "win32",
    stdio: ["inherit", "pipe", "pipe"],
  });
  child.stderr!.pipe(process.stderr, { end: false });
  const cancel = (): void => {
    cancelled = true;
    terminateOwnedChildTree(child);
  };
  options.signal?.addEventListener("abort", cancel, { once: true });
  if (options.signal?.aborted) cancel();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const stopProgress = startNativeProgress(`artifact-rust:${owner}:build`);
  const status = new Promise<number>((accept) => {
    child.once("error", (error) => {
      console.error(error.message);
      accept(1);
    });
    child.once("close", (code) => accept(code ?? 1));
  });
  try {
    try {
      try {
        for await (const line of createInterface({ input: child.stdout!, crlfDelay: Infinity })) {
          let message;
          try {
            message = JSON.parse(line);
          } catch {
            process.stdout.write(line + "\n");
            continue;
          }
          if ((message.reason === "compiler-artifact" || message.reason === "build-script-executed") && (options.environment ?? process.env).SEMIO_TEST_ARTIFACT_DIR) process.stdout.write(line + "\n");
          if (message.reason === "build-script-executed") buildScripts.push(message);
          if (message.reason === "compiler-artifact") units.push({ message,evidence:{version:1,kind:"discovery",paths:[],producer:null} });
          if (message.reason !== "compiler-artifact" || message.target?.kind?.includes("custom-build")) continue;
          const packageUrl = message.package_id?.split("#")[0]?.replace(/^path\+/, "");
          const bin = args.indexOf("--bin"),
            example = args.indexOf("--example");
          const selected =
            bin >= 0
              ? message.target?.kind?.includes("bin") && message.target.name === args[bin + 1]
              : example >= 0
                ? message.target?.kind?.includes("example") && message.target.name === args[example + 1]
                : args.includes("--bins")
                  ? message.target?.kind?.includes("bin")
                  : true;
          const primary = selected && packageUrl !== undefined && packageUrl.startsWith("file:") && resolve(fileURLToPath(packageUrl)) === sourceRoot;
          if (primary && typeof message.executable === "string") primaryExecutable = message.executable;
          for (const file of message.filenames ?? []) {
            if (file.endsWith(".d")) continue;
            const library = primary && file.endsWith(".rmeta") ? message.filenames.find((candidate: string) => candidate.endsWith(".rlib")) : undefined;
            const name = (library ? library.replace(/\.rlib$/, ".rmeta") : file).split(/[\\/]/).at(-1)!;
            const key = primary ? name : /\.(rlib|rmeta|so|dylib|dll|lib)$/.test(file) ? `deps/${name}` : undefined;
            if (!key) continue;
            stagedNames.set(file, key);
            if (!primary) {
              dependencies.set(key, file);
              continue;
            }
            const captured = join(capture, key);
            mkdirSync(dirname(captured), { recursive: true });
            copyFileSync(file, captured);
            chmodSync(captured, lstatSync(file).mode & 0o777);
            files.set(key, captured);
            hasLibrary ||= file.endsWith(".rlib");
          }
        }
      } catch (error) {
        cancel();
        await status;
        throw error;
      }
      if ((await status) !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
    } finally {
      stopProgress();
      if (forceKill) clearTimeout(forceKill);
      options.signal?.removeEventListener("abort", cancel);
      process.removeListener("SIGINT", cancel);
      process.removeListener("SIGTERM", cancel);
    }
    if (files.size === 0) throw new Error(`Cargo emitted no final artifacts for ${owner}`);
    if (hasLibrary)
      for (const [name, file] of dependencies) {
        const captured = join(capture, name);
        mkdirSync(dirname(captured), { recursive: true });
        copyFileSync(file, captured);
        chmodSync(captured, lstatSync(file).mode & 0o777);
        files.set(name, captured);
      }
    if (options.sourcesRecord !== undefined) {
      if (primaryExecutable === undefined) throw new Error(`Cargo emitted no executable to record the sources of: ${owner}`);
      const record: CargoBinarySourcesV1 = { schema: CARGO_BINARY_SOURCES_SCHEMA_V1, builtAtMs, sources: cargoDepInfoSourcesV1(readFileSync(`${primaryExecutable.replace(/\.exe$/u, "")}.d`, "utf8")) };
      if (record.sources.length === 0) throw new Error(`Cargo dep-info names no sources for ${owner}`);
      const captured = join(capture, options.sourcesRecord);
      writeFileSync(captured, `${JSON.stringify(record)}\n`);
      files.set(options.sourcesRecord, captured);
    }
    options.validate?.(files);
    await stageArtifacts(staging, owner, files, { signal: options.signal, leaseDirectory: policy.leaseDirectory });
    console.log(`[nx-native] staged ${files.size} deliverables in ${relative(policy.cwd, staging).split(sep).join("/")}`);
  } finally {
    try {
      {
        if(!options.signal)throw Error("Cargo capture physical signal required");const observedAt=performance.now(),physical=new CurrentPhysicalOwnerV1(policy.cwd,{maxBytes:128*1024*1024,maxWork:65536,chunkBytes:1024*1024,cancelled:()=>options.signal!.aborted,remainingMs:()=>60000-(performance.now()-observedAt),onProgress:row=>console.log("[cargo-provenance] "+JSON.stringify(row))});
        await writeCompletedCargoInvocationProvenanceV1(join(provenanceRoot ?? join(policy.buildDirectory, "semio-cargo-provenance"), `cargo-unit-provenance-${basename(capture)}.json`), { manifest: path, cwd: policy.cwd, command: commandPort.command, args: [...commandPort.args, options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], buildDirectory: policy.buildDirectory, builtAtMs, status: await status, cancelled, units, buildScripts }, new Map([...stagedNames].filter(([, key]) => files.has(key)).map(([path, key]) => [path, join(staging, key)])), resolve((options.environment ?? process.env).CARGO_HOME??join(homedir(),".cargo")),physical);
      }
    } finally {
      rmSync(capture, { recursive: true, force: true });
    }
  }
}
