import { acquireCompilerCapacityLeaseV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../🏗️build/🎟️capacity/🟦️.ts";
import { checkResourceLeaseControl, requireMaterializedPathV1 } from "../../🔒️leases/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { stageArtifacts } from "../📤️publication/🟦️.ts";
import { terminateOwnedChildTree } from "../../🪓️termination/🟦️.ts";
import { captureOwnedCommandV1, startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";
import {acquireCargoTargetRecipeV1,cargoTargetRecipeEnvironmentV1,type CargoTargetRecipeV1,type CargoTargetRecipeLeaseV1} from "./🎯️recipe/🟦️.ts";
import { chmodSync, closeSync, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readdirSync, readlinkSync, rmSync, writeFileSync, writeSync } from "node:fs";
import { basename, dirname, join, relative, resolve, sep, isAbsolute, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { spawn, spawnSync } from "node:child_process";
import { createHash, randomUUID } from "node:crypto";
import { homedir } from "node:os";
import { createInterface } from "node:readline";
import {copyFile,lstat,mkdir,open,rename,rm,type FileHandle} from "node:fs/promises";
import type { FileObservationControlV1 } from "../../../📁️filesystem/🧾️observation/🟦️.ts";
import {CURRENT_PHYSICAL_CONCURRENCY_V1,CurrentPhysicalFilesV1,CurrentPhysicalOwnerV1,mapBoundedV1,type CurrentPhysicalEntryV1,type CurrentPhysicalPortV1} from "../../../📁️filesystem/🧾️observation/📁️current/🟦️.ts";
import {
  CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1,
  CargoProvenanceStreamV1,
  cargoDepInfoChecksumsV1,
  cargoDepInfoSourcesV1,
  cargoProvenancePhysicalControlV1,
  cargoProvenanceProgressSinkV1,
  decodeCargoProvenanceV1,
  type CargoArtifactObservationV1,
  type CargoBuildResourcesV1,
  type CargoDepInfoV1,
  type CargoInputObservationV1,
  type CargoInvocationInputV1,
  type CargoProvenanceSinkV1,
  type CargoProvenanceV1,
  type CargoResourceWitnessV1,
  type CargoUnitObservationV1,
} from "./🧾️receipt/🟦️.ts";

export { cargoDepInfoChecksumsV1, cargoDepInfoSourcesV1, cargoProvenancePhysicalControlV1, cargoProvenanceProgressSinkV1 } from "./🧾️receipt/🟦️.ts";

import { buildModePolicy, type BuildMode } from "../../🏗️build/🎚️mode/🟦️.ts";
import { cargoRequestedProfileV1 } from "./🔒️lease/🟦️.ts";


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
export type CargoArtifactBuildPolicyV1 = Readonly<{ version: 1; mode: BuildMode; compilerConfiguration?: string; cwd: string; buildDirectory: string; leaseDirectory: string; captureDirectory: string; budgetMs: number }>;
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

/** ⚙️ Includes an owned Cargo configuration before compiler argument forwarding. */
export function cargoConfigurationArgumentsV1(args:readonly string[],configuration:string|undefined,buildDirectory:string):string[]{
 if(!isAbsolute(buildDirectory)||buildDirectory.includes("\0"))throw Error("Compiler build storage must be an absolute path");
 if(configuration!==undefined&&(!isAbsolute(configuration)||configuration.includes("\0")))throw Error("Compiler configuration must be an absolute path");
 const delimiter=args.indexOf("--"),head=delimiter<0?[...args]:args.slice(0,delimiter),tail=delimiter<0?[]:args.slice(delimiter);
 if(configuration!==undefined&&!head.some((arg,index)=>arg==="--config"&&head[index+1]===configuration||arg===`--config=${configuration}`))head.push("--config",configuration);
 return [...head,"--config",`build.build-dir=${JSON.stringify(buildDirectory)}`,...tail];
}

/** 🎚️ Declares the exact selected artifact profile and owned compiler configuration. */
export function cargoArtifactArgumentsV1(args:readonly string[],policy:Pick<CargoArtifactBuildPolicyV1,"mode"|"compilerConfiguration"|"buildDirectory">):string[]{
 const selectedProfile=cargoRequestedProfileV1(args),expected=buildModePolicy(policy.mode);
 if(selectedProfile&&selectedProfile!==expected.cargoProfile&&selectedProfile!==expected.wasmProfile&&!(policy.mode==="release"&&selectedProfile==="host-tool"))throw Error("Cargo profile conflicts with selected build mode");
 let selected=[...args];if(!selectedProfile){const delimiter=args.indexOf("--");selected=delimiter<0?[...args,"--profile",expected.cargoProfile]:[...args.slice(0,delimiter),"--profile",expected.cargoProfile,...args.slice(delimiter)];}
 return cargoConfigurationArgumentsV1(selected,policy.compilerConfiguration,policy.buildDirectory);
}

/** 📦️ Captures Cargo's declared deliverables, including link dependencies, without copying compiler state.
 * `sourcesRecord` names a {@link CargoBinarySourcesV1} file staged beside the selected executable, in
 * the same atomic publication, so a consumer can tell a stale executable from a fresh one without
 * running Cargo. Cargo's stderr is piped and forwarded, never inherited: Bun marks its own stderr
 * `O_NONBLOCK` once written, an inherited pipe shares that flag, and a Cargo burst (replayed warnings
 * of fresh units) then fails with `EAGAIN` as soon as a slow reader lets the 64 KiB pipe fill — the
 * build dies with its diagnostics cut mid-line (ticket 26/09/23 W4, `wp-w4/w4-nonblock-probe.ts`). */
import observationOutputSchema from "./📦️observation/🧬️schema/🔣️.json" with {type:"json"};
export type CargoObservationOutputV1=Readonly<{version:1;directory:string}>;
/** 🧾️ Admits a fresh immutable compiler observation under its explicit storage owner. */
export function cargoObservationOutputV1(value:unknown,buildDirectory:string):string{
 if(validateJsonSchemaSubset(observationOutputSchema,value).length)throw Error("Complete compiler observation output required");
 const directory=resolve((value as CargoObservationOutputV1).directory),owner=join(resolve(buildDirectory),"semio-cargo-observations");
 requireMaterializedPathV1(directory);
 if(!directory.startsWith(owner+sep)||existsSync(directory))throw Error("Compiler observation output must be fresh and belong to its compiler store");
 return directory;
}

export type CargoArtifactBuildOptionsV1 = { readonly control: CompilerAdmissionControl; readonly observationOutput?: CargoObservationOutputV1; readonly command?: "build" | "rustc"; readonly deliverables?: readonly string[]; readonly output?: string; readonly sourcesRecord?: string; readonly validate?: (files: ReadonlyMap<string, string>) => void; readonly environment?: Readonly<Record<string,string|undefined>>; readonly commandPort?: Readonly<{ command: string; args: readonly string[]; compilerIdentity:string; toolchainIdentity:string }>; readonly provenance?: (receiptPath: string) => void };

/** 🧭️ Observes the original compiler recipe while leaving source freshness validation to every Cargo run. */
async function cargoArtifactTargetRecipeV1(manifest:string,args:readonly string[],policy:CargoArtifactBuildPolicyV1,options:CargoArtifactBuildOptionsV1):Promise<Readonly<{recipe:CargoTargetRecipeV1;recheck:()=>Promise<void>}>>{
 const control=options.control,environment=options.environment??process.env,port=options.commandPort,compilerIdentity=port?.compilerIdentity??await captureOwnedCommandV1("cargo",["--version","--verbose"],policy.cwd,{control,env:environment}),toolchainIdentity=port?.toolchainIdentity??await captureOwnedCommandV1("rustc",["--version","--verbose"],policy.cwd,{control,env:environment}),physical=new CurrentPhysicalOwnerV1(policy.cwd,cargoProvenancePhysicalControlV1({cancelled:()=>control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(control),onProgress:cargoProvenanceProgressSinkV1(row=>console.log("[cargo-target-recipe] "+JSON.stringify(row)))})),paths=new Set<string>([manifest]);
 if(policy.compilerConfiguration)paths.add(policy.compilerConfiguration);for(let root of[policy.cwd,dirname(manifest)])for(;;){for(const name of["Cargo.toml","Cargo.lock",".cargo/config.toml",".cargo/config","rust-toolchain.toml","rust-toolchain"])paths.add(join(root,name));const parent=dirname(root);if(parent===root)break;root=parent;}const cargoHome=resolve(environment.CARGO_HOME??join(homedir(),".cargo"));for(const name of["config.toml","config"])paths.add(join(cargoHome,name));
 const inputs=[];for(const path of[...paths].sort()){checkResourceLeaseControl(control);inputs.push({path,sha256:await physical.digest(path)??null});await control.publish({resource:"cargo-target-recipe-inputs",mode:"exclusive",elapsedMs:0});await control.yieldContinuation();}const declared=cargoTargetRecipeEnvironmentV1(environment);
 return{recipe:{version:1,cwd:policy.cwd,manifest,command:port?.command??"cargo",args:[...(port?.args??[]),options.command??"build","--locked","--manifest-path",manifest,...args],buildDirectory:policy.buildDirectory,compilerIdentity:compilerIdentity.trim(),toolchainIdentity:toolchainIdentity.trim(),inputs,environment:declared},recheck:()=>physical.recheck()};
}

/** 🚦️ Retains exact recipe target and shared store custody through invocation capture and provenance under the original control. */
export async function buildCargoArtifacts(manifest: string, args: string[], policy: CargoArtifactBuildPolicyV1, options: CargoArtifactBuildOptionsV1): Promise<void> {
  policy = admitArtifactPolicy(policy);
  args = cargoArtifactArgumentsV1(args,policy);
  if(options.observationOutput&&options.output)throw Error("Compiler observations and source deliverables require separate output ownership");
  options = { ...options, ...(options.observationOutput?{}:{output: options.output ?? `dist/build/${policy.mode}`}) };
  if (!isAbsolute(manifest)) throw Error("Cargo artifact manifest must be absolute");
  const control = requestedCompilerAdmissionControlV1(options.control, policy.budgetMs);
  if (!cargoBuildOwnerAliveV1(options.environment ?? process.env)) throw Error("Original Cargo build owner exited");
  let capacity: Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>> | undefined, lease:CargoTargetRecipeLeaseV1|undefined;
  try {
    options = { ...options, control };
    const recipe=await cargoArtifactTargetRecipeV1(manifest,args,policy,options);
    lease=await acquireCargoTargetRecipeV1({directory:policy.leaseDirectory,recipe:recipe.recipe,control});
    capacity = await acquireCompilerCapacityLeaseV1({ directory: policy.leaseDirectory, control, environment: options.environment });
    const environment: Record<string, string | undefined> = { ...(options.environment ?? process.env), SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(capacity.parallelism) };
    if (!environment.MAKEFLAGS && !environment.CARGO_MAKEFLAGS) environment.CARGO_BUILD_JOBS = String(capacity.parallelism);
    options = { ...options, environment };
    await recipe.recheck();
    await captureCargoArtifacts(manifest, args, { ...policy, budgetMs: compilerAdmissionBudgetV1(control) }, options,lease.targetDirectory,() => { const allocated = capacity; capacity = undefined; allocated?.release(); });
  } finally { lease?.release(); capacity?.release(); }
}

/** 🪵️ Forwards a compiler's diagnostics live up to a line budget, keeps the complete stream in a log file and, after a failure, replays the newest lines.
 * A release build replays the cached warnings of every fresh unit into every component task (build #9: 945 MB), so an unbounded forward floods the task log and the outer Nx buffer. */
export class BoundedDiagnosticsV1 {
  lines = 0;
  suppressed = 0;
  private logged = 0;
  private readonly newest: string[] = [];
  constructor(private readonly log: number | null, private readonly forward: (line: string) => void, private readonly liveLines = 200, private readonly newestLines = 200, private readonly logLimitBytes = 128 * 1024 * 1024) {}
  line(text: string): void {
    this.lines++;
    if (this.log !== null && this.logged < this.logLimitBytes) this.logged += writeSync(this.log, `${text}\n`);
    if (this.lines <= this.liveLines) { this.forward(text); return; }
    this.suppressed++;
    this.newest.push(text);
    if (this.newest.length > this.newestLines) this.newest.shift();
  }
  finish(failed: boolean, logPath: string | null): string[] {
    const rows: string[] = [];
    if (this.suppressed > 0) rows.push(`[cargo] ${this.suppressed} diagnostic line(s) not shown${logPath ? `; complete log: ${logPath}` : ""}`);
    if (failed) rows.push(...this.newest);
    return rows;
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

/** 🗃️ The completed Cargo invocation a provenance receipt is written for, before any physical observation. */
export type CargoInvocationSourceV1 = Readonly<{ manifest: string; cwd: string; command: string; args: readonly string[]; buildDirectory: string | null; builtAtMs: number; status: number; cancelled: boolean; units: readonly any[]; buildScripts: readonly any[] }>;

const compilerArtifactExtension = /\.(?:rlib|rmeta|so|dylib|dll|lib|wasm|exe)$/u;
const receiptTemporaryName = (receipt: string): string => requireMaterializedPathV1(join(dirname(receipt), `.${randomUUID()}.tmp`));

class ReceiptFileSinkV1 implements CargoProvenanceSinkV1 {
  bytes = 0;
  constructor(private readonly handle: FileHandle, private readonly limit: number) {}
  async write(chunk: string): Promise<void> {
    const buffer = Buffer.from(chunk, "utf8");
    this.bytes += buffer.byteLength;
    if (this.bytes > this.limit) throw Error("Physical provenance publication budget");
    await this.handle.write(buffer);
  }
}

/** 📝️ One completed invocation's observation as a session stages it: the receipt waiting for the session's recheck and the records its leaves are derived from. */
export type CargoProvenanceStagedV1 = Readonly<{ receiptPath: string; units: readonly CargoUnitObservationV1[]; buildResources: readonly CargoBuildResourcesV1[] }>;

/** 🔐️ Where a provenance session takes its physical authority from: one port per observed invocation, settled (its structural observations rechecked) when that
 * invocation's receipt is staged, and one final recheck of every file the session read. */
export type CargoProvenanceCustodyV1 = Readonly<{ invocation: () => Readonly<{ port: CurrentPhysicalPortV1; settle: () => Promise<void> }>; recheck: () => Promise<void> }>;

/** 🚪️ The custody of exactly one port: every invocation observes through it and the port's own complete recheck closes the session. */
export function portProvenanceCustodyV1(port: CurrentPhysicalPortV1): CargoProvenanceCustodyV1 {
  return Object.freeze({ invocation: () => Object.freeze({ port, settle: async () => undefined }), recheck: () => port.recheck() });
}

/** 🤝️ The custody of one build session: every invocation observes through its own owner, all owners share one file custody, so a file is hashed once however
 * many invocations name it and rechecked once at the end, while each invocation's directory rosters and absent paths are rechecked as its receipt is staged. */
export function sharedProvenanceCustodyV1(root: string, control: () => FileObservationControlV1): CargoProvenanceCustodyV1 & Readonly<{ files: CurrentPhysicalFilesV1 }> {
  const files = new CurrentPhysicalFilesV1();
  return Object.freeze({ files, invocation: () => { const owner = new CurrentPhysicalOwnerV1(root, control(), files); return Object.freeze({ port: owner, settle: () => owner.recheckStructure() }); }, recheck: () => files.recheck(control) });
}

/** 🔭️ One invocation's view of the physical world: structural facts (kinds, rosters, directory digests, input rows) are its own, file facts are the session's. */
type ProvenanceObserverV1 = Readonly<{ port: CurrentPhysicalPortV1; path: (value: string) => string; kind: (value: string) => Promise<"file" | "directory" | null>; text: (value: string) => Promise<string | null>; length: (value: string) => Promise<number | null>; entries: (value: string) => Promise<readonly CurrentPhysicalEntryV1[] | null>; digest: (value: string) => Promise<string | null>; sha256: (value: string) => Promise<string | null>; input: (value: string) => Promise<CargoInputObservationV1>; resource: (observation: any) => Promise<CargoResourceWitnessV1>; fileDigest: (value: string) => string | undefined }>;

const COMPILER_RESOURCE_DIRECTORY_V1 = "semio-compiler-resources", COMPILER_RESOURCE_OBSERVATION_V1 = "observation.json", BUILD_RESOURCE_INPUTS_V1 = "semio-runtime-resource-inputs.jsonl";

/** 📄️ Whether provenance parses `file` as well as hashing it (a compiler resource observation or a build script's resource inputs), so its one read serves both. */
function parsedProvenanceFileV1(file: string): boolean {
  const name = basename(file);
  return name === BUILD_RESOURCE_INPUTS_V1 || (name === COMPILER_RESOURCE_OBSERVATION_V1 && basename(dirname(dirname(file))) === COMPILER_RESOURCE_DIRECTORY_V1);
}

/** 🗃️ One build session's physical provenance: every file is hashed once however many invocations name it, dep-info and resource texts are parsed once, and every
 * file is rechecked once when the session closes; each invocation's directory rosters and absent paths are rechecked when its receipt is staged. Each completed
 * invocation is observed into a private temporary receipt, and the receipts become visible only by atomic renames after the final recheck, so a refused or failed
 * session publishes none. Observation never retains file bodies: dep-info is kept as its digest plus the sources and checksums parsed from exactly those bytes,
 * resource captures as their digest, and every path, input digest and resource witness is interned once.
 * @see ./🧾️receipt/🟦️.ts
 * @see ./🧩️component-pass/🟦️.ts */
export class CargoProvenanceSessionV1 {
  private readonly fileDigests = new Map<string, string>();
  private readonly lengths = new Map<string, number>();
  private readonly texts = new Map<string, string>();
  private readonly reading = new Map<string, Promise<unknown>>();
  private readonly producers = new Map<string, CargoProvenanceV1>();
  private readonly staged: { readonly temporary: string; readonly destinations: readonly string[]; readonly physical: CurrentPhysicalPortV1 }[] = [];
  private state: "open" | "closed" | "failed" = "open";

  constructor(private readonly custody: CargoProvenanceCustodyV1) {
    if (!custody || typeof custody.invocation !== "function" || typeof custody.recheck !== "function") throw Error("Complete current physical provenance authority required");
  }

  private once<T>(key: string, read: () => Promise<T>): Promise<T> {
    const known = this.reading.get(key);
    if (known) return known as Promise<T>;
    const pending = read();
    this.reading.set(key, pending);
    void pending.catch(() => this.reading.delete(key));
    return pending;
  }

  private observer(port: CurrentPhysicalPortV1): ProvenanceObserverV1 {
    if (!port || !port.control || !["read", "digest", "measure", "directoryEntries", "checkpoint", "recheck"].every((key) => typeof (port as any)[key] === "function")) throw Error("Complete current physical provenance authority required");
    const policy = port.control;
    if (![policy.maxBytes, policy.maxWork, policy.chunkBytes].every(Number.isSafeInteger) || policy.maxBytes < 0 || policy.maxWork < 1 || policy.chunkBytes < 1 || !["cancelled", "remainingMs", "onProgress"].every((key) => typeof (policy as any)[key] === "function")) throw Error("Finite current physical provenance control required");
    const kinds = new Map<string, "file" | "directory" | null>(), rosters = new Map<string, readonly CurrentPhysicalEntryV1[] | null>(), directoryDigests = new Map<string, string | null>(), inputs = new Map<string, CargoInputObservationV1>();
    const path = (value: string): string => {
      if (typeof value !== "string" || !isAbsolute(value) || value.includes("\0")) throw Error("Physical provenance path refused");
      const file = resolve(value);
      if (file.length > 256) throw Error(`Physical provenance path exceeds 256 UTF-16 units (${file.length}): ${file}`);
      return file;
    };
    const kind = async (value: string): Promise<"file" | "directory" | null> => {
      const file = path(value);
      if (kinds.has(file)) return kinds.get(file)!;
      await port.checkpoint();
      let result: "file" | "directory" | null = null;
      try {
        const stat = await lstat(file);
        if (stat.isSymbolicLink()) throw Error("Physical provenance input cannot be a symbolic link");
        result = stat.isFile() ? "file" : stat.isDirectory() ? "directory" : null;
      } catch (error) {
        if (!["ENOENT", "ENOTDIR"].includes((error as { code?: string }).code ?? "")) throw error;
        await port.digest(file);
      }
      await port.checkpoint();
      kinds.set(file, result);
      return result;
    };
    const text = async (value: string): Promise<string | null> => {
      const file = path(value);
      if ((await kind(file)) !== "file") return null;
      const known = this.texts.get(file);
      if (known !== undefined) return known;
      return await this.once(`text:${file}`, async () => {
        const actual = await port.read(file);
        if (actual === undefined) throw Error("Physical provenance input disappeared");
        const body = typeof actual === "string" ? Buffer.from(actual) : actual;
        this.fileDigests.set(file, createHash("sha256").update(body).digest("hex"));
        this.lengths.set(file, body.byteLength);
        const decoded = Buffer.from(body).toString("utf8");
        this.texts.set(file, decoded);
        return decoded;
      });
    };
    const length = async (value: string): Promise<number | null> => {
      const file = path(value);
      if (!this.lengths.has(file)) await digest(file);
      return this.lengths.get(file) ?? null;
    };
    const entries = async (value: string): Promise<readonly CurrentPhysicalEntryV1[] | null> => {
      const file = path(value);
      if (rosters.has(file)) return rosters.get(file)!;
      if ((await kind(file)) !== "directory") {
        rosters.set(file, null);
        return null;
      }
      const rows = await port.directoryEntries(file);
      if (!rows) throw Error("Physical provenance directory disappeared");
      rosters.set(file, rows);
      directoryDigests.set(file, createHash("sha256").update(JSON.stringify(rows.map((row) => [basename(row.path), row.kind, row.symlinkTarget]))).digest("hex"));
      return rows;
    };
    const digest = async (value: string): Promise<string | null> => {
      const file = path(value);
      const type = await kind(file);
      if (type === "file") {
        const known = this.fileDigests.get(file);
        if (known !== undefined) return known;
        if (parsedProvenanceFileV1(file)) { await text(file); return this.fileDigests.get(file)!; }
        const pendingText = this.reading.get(`text:${file}`);
        if (pendingText) { await pendingText; return this.fileDigests.get(file)!; }
        return await this.once(`digest:${file}`, async () => {
          const actual = await port.measure(file);
          if (actual === undefined) throw Error("Physical provenance input disappeared");
          this.fileDigests.set(file, actual.sha256);
          this.lengths.set(file, actual.byteLength);
          return actual.sha256;
        });
      }
      if (type === "directory") {
        if (!directoryDigests.has(file)) await entries(file);
        return directoryDigests.get(file) ?? null;
      }
      return null;
    };
    const sha256 = async (value: string): Promise<string | null> => ((await kind(value)) === "file" ? await digest(value) : null);
    const input = async (value: string): Promise<CargoInputObservationV1> => {
      const file = path(value);
      const known = inputs.get(file);
      if (known) return known;
      const observed = { path: value, kind: await kind(value), sha256: await digest(value) };
      inputs.set(file, observed);
      return observed;
    };
    const resource = async (observation: any): Promise<CargoResourceWitnessV1> => {
      if (observation.kind === "read") {
        const errors = validateJsonSchemaSubset(resourceReadSchema, observation);
        const consumed = await length(observation.path);
        const actual = await input(observation.path);
        return { input: observation, bytes: consumed, sha256: actual.sha256, schemaErrors: errors, sourceExact: errors.length === 0 && actual.kind === "file" && consumed === observation.bytes && actual.sha256 === observation.sha256 };
      }
      const actual = await input(observation.path);
      const rows = observation.kind === "directory" ? await entries(observation.path) : null;
      return { input: observation, sha256: actual.sha256, ...(observation.kind === "copy" ? { outputSha256: typeof observation.output === "string" ? await sha256(observation.output) : null } : {}), ...(rows ? { observedEntries: rows } : {}) };
    };
    return Object.freeze({ port, path, kind, text, length, entries, digest, sha256, input, resource, fileDigest: (value: string) => this.fileDigests.get(path(value)) });
  }

  /** ⚡️ Hashes one unit's outputs, dep-info and dep-info sources the moment Cargo announces it, so the receipt written after the invocation finds every file
   * digest already known; directory rosters and every authoritative row are still observed when the invocation's receipt is written. */
  async prefetch(message: Readonly<{ filenames?: readonly string[] }>): Promise<void> {
    if (this.state !== "open") throw Error("Cargo provenance session is closed");
    const observe = this.observer(this.custody.invocation().port), files = [...new Set<string>(message.filenames ?? [])];
    await mapBoundedV1(files, CURRENT_PHYSICAL_CONCURRENCY_V1, (file) => observe.sha256(file));
    for (const file of files) {
      const dep = file.replace(compilerArtifactExtension, "") + ".d";
      const text = await observe.text(dep);
      if (text === null) continue;
      const base = dirname(dep);
      await mapBoundedV1(cargoDepInfoSourcesV1(text).filter((source) => isAbsolute(source)).map((source) => resolve(base, source)), CURRENT_PHYSICAL_CONCURRENCY_V1, (source) => observe.digest(source));
    }
  }

  /** 🔏️ Observes one completed invocation under the session's physical authority and stages its receipt (and its build-directory ledger copy) for publication
   * when the session closes. Resolves with the unit and build-resource records the receipt holds, the rows a caller derives provenance leaves from. */
  async write(receiptPath: string, invocation: CargoInvocationSourceV1, stagedPaths: ReadonlyMap<string, string>, cargoHome: string): Promise<CargoProvenanceStagedV1> {
    if (this.state !== "open") throw Error("Cargo provenance session is closed");
    const lease = this.custody.invocation(), observe = this.observer(lease.port), physical = lease.port, policy = physical.control, path = observe.path;
    path(receiptPath);
    path(cargoHome);
    path(invocation.cwd);
    path(invocation.manifest);
    if (invocation.buildDirectory) path(invocation.buildDirectory);
    const witnessed: { readonly message: any; readonly files: ReadonlySet<string>; readonly rows: readonly CargoInputObservationV1[] }[] = [];
    const observedUnits: CargoUnitObservationV1[] = [], observedResources: CargoBuildResourcesV1[] = [];
    const limit = Math.min(policy.maxBytes, CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1);
    const destinations = [...new Set([receiptPath, ...(invocation.buildDirectory ? [join(invocation.buildDirectory, "semio-cargo-provenance", basename(receiptPath))] : [])])];
    for (const file of destinations) { path(file); requireMaterializedPathV1(join(dirname(file), `.${"x".repeat(36)}.tmp`)); }
    const temporary = receiptTemporaryName(receiptPath);
    await physical.checkpoint();
    await mkdir(dirname(receiptPath), { recursive: true });
    await physical.checkpoint();
    const handle = await open(temporary, "wx");
    let staged = false;
    try {
      const sink = new ReceiptFileSinkV1(handle, limit);
      const stream = new CargoProvenanceStreamV1(sink);
      await stream.begin(invocation);
      await mapBoundedV1([...new Set<string>(invocation.units.flatMap((unit) => unit.message.filenames ?? []))], CURRENT_PHYSICAL_CONCURRENCY_V1, (file) => observe.sha256(file));
      for (const original of invocation.units) {
        await physical.checkpoint();
        const evidence = parseCargoCompilerUnitEvidenceV1(original.evidence);
        const message = original.message;
        const candidates: string[] = [...new Set<string>((message.filenames ?? []).flatMap((file: string) => [file.replace(compilerArtifactExtension, "") + ".d", join(dirname(file), basename(file).replace(/^lib/u, "").replace(compilerArtifactExtension, "") + ".d")]))];
        let producerUnit: CargoUnitObservationV1 | null = null;
        if (evidence.kind === "received") {
          const source = await observe.text(evidence.producer.path);
          if (source === null || (await observe.sha256(evidence.producer.path)) !== evidence.producer.sha256) throw Error("Compiler producer receipt changed");
          let producer = this.producers.get(evidence.producer.path);
          if (!producer) {
            producer = decodeCargoProvenanceV1(source);
            this.producers.set(evidence.producer.path, producer);
          }
          for (const field of ["manifest", "cwd", "command", "args", "buildDirectory", "builtAtMs", "status", "cancelled"] as const) if (JSON.stringify(producer[field]) !== JSON.stringify(invocation[field])) throw Error("Compiler producer invocation differs");
          producerUnit = producer.units[evidence.producer.unit] ?? null;
          if (!producerUnit || JSON.stringify(producerUnit.message) !== JSON.stringify(message) || JSON.stringify(producerUnit.depInfo.map((row) => row.path)) !== JSON.stringify(evidence.paths)) throw Error("Compiler producer unit custody differs");
          candidates.splice(0, candidates.length, ...evidence.paths);
        }
        const sourceOwners = new Set<string>();
        const targetSource = resolve(message.target.src_path);
        for (const manifest of [invocation.manifest, message.manifest_path].filter((value): value is string => typeof value === "string"))
          for (let owner = dirname(resolve(manifest)); ; ) {
            if ((await observe.kind(join(owner, "Cargo.toml"))) === "file") sourceOwners.add(owner);
            const parent = dirname(owner);
            if (parent === owner) break;
            owner = parent;
          }
        const matchesSource = async (source: string): Promise<boolean> => {
          for (const value of cargoDepInfoSourcesV1(source)) {
            await physical.checkpoint();
            if (isAbsolute(value)) {
              if (resolve(value) === targetSource) return true;
              continue;
            }
            for (const owner of sourceOwners) {
              await physical.checkpoint();
              if (resolve(owner, value) === targetSource) return true;
            }
          }
          return false;
        };
        if (evidence.kind === "discovery" && invocation.buildDirectory) {
          const option = (name: string) => invocation.args.flatMap((value, index) => (value === name ? [invocation.args[index + 1]!] : value.startsWith(name + "=") ? [value.slice(name.length + 1)] : []))[0];
          const packageName = /#([^@]+)@/u.exec(message.package_id ?? "")?.[1];
          if (packageName) {
            const target = option("--target");
            const owner = join(invocation.buildDirectory, ...(target ? [target] : []), option("--profile") ?? (invocation.args.includes("--release") ? "release" : "debug"), "build", packageName);
            const rows = await observe.entries(owner);
            if (rows)
              for (const row of rows) {
                const output = join(row.path, "out");
                if ((await observe.kind(output)) !== "directory") continue;
                for (const file of message.filenames ?? []) {
                  const internal = join(output, basename(file));
                  const dep = internal.replace(compilerArtifactExtension, "") + ".d";
                  const depText = await observe.text(dep);
                  if (!depText?.includes("# checksum:") || !(await matchesSource(depText))) continue;
                  const selected = await observe.sha256(file);
                  if (selected === null || (await observe.sha256(internal)) !== selected) continue;
                  candidates.push(dep);
                }
              }
          }
        }
        const depInfo: CargoDepInfoV1[] = [];
        for (const file of new Set(candidates)) {
          const depText = await observe.text(file);
          if (depText === null) {
            if (evidence.kind === "received") throw Error("Compiler received candidate disappeared");
            continue;
          }
          const depSha256 = observe.fileDigest(file)!;
          if (evidence.kind === "received" && producerUnit!.depInfo.find((row) => row.path === file)?.sha256 !== depSha256) throw Error("Compiler received candidate bytes differ");
          const sources = cargoDepInfoSourcesV1(depText);
          const relativeSources = sources.filter((source) => !isAbsolute(source)).map((source) => normalize(source));
          const bases: string[] = [];
          for (const base of sourceOwners) {
            if (!relativeSources.some((source) => resolve(base, source) === targetSource)) continue;
            let valid = true;
            for (const source of sources)
              if ((await observe.kind(resolve(base, source))) === null) {
                valid = false;
                break;
              }
            if (valid) bases.push(base);
          }
          depInfo.push({ path: file, sha256: depSha256, baseDirectory: relativeSources.length === 0 ? invocation.cwd : bases.length === 1 ? bases[0]! : null, sources, checksums: cargoDepInfoChecksumsV1(depText) });
        }
        const rows: CargoInputObservationV1[] = [];
        const sourceFiles = [...new Set<string>(depInfo.flatMap((row) => row.sources.filter((source) => isAbsolute(source) || row.baseDirectory !== null).map((source) => resolve(row.baseDirectory ?? invocation.cwd, source))))];
        await mapBoundedV1(sourceFiles, CURRENT_PHYSICAL_CONCURRENCY_V1, (file) => observe.input(file));
        for (const file of sourceFiles) rows.push(await observe.input(file));
        const observedAtMs = Date.now();
        const artifacts: CargoArtifactObservationV1[] = [];
        for (const file of message.filenames ?? []) {
          const stagedPath = stagedPaths.get(file);
          artifacts.push({ path: file, sha256: await observe.sha256(file), ...(stagedPath ? { stagedPath, stagedSha256: await observe.sha256(stagedPath) } : {}) });
        }
        const unit: CargoUnitObservationV1 = { message, depInfo, inputs: rows, observedAtMs, artifacts };
        if (producerUnit && (JSON.stringify(unit.depInfo) !== JSON.stringify(producerUnit.depInfo) || JSON.stringify(unit.inputs) !== JSON.stringify(producerUnit.inputs) || JSON.stringify(unit.artifacts.map((row) => [row.path, row.sha256])) !== JSON.stringify(producerUnit.artifacts.map((row) => [row.path, row.sha256])))) throw Error("Compiler received physical witnesses differ");
        await stream.unit(unit);
        observedUnits.push(unit);
        witnessed.push({ message, rows, files: new Set(rows.filter((row) => row.kind === "file").map((row) => resolve(row.path))) });
      }
      await stream.buildScripts(invocation.buildScripts);
      for (const build of invocation.buildScripts) {
        const file = join(build.out_dir, BUILD_RESOURCE_INPUTS_V1);
        const source = await observe.text(file);
        if (source === null) continue;
        const resources: CargoResourceWitnessV1[] = [];
        for (const line of source.split(/\r?\n/u).filter(Boolean)) resources.push(await observe.resource(JSON.parse(line)));
        const row: CargoBuildResourcesV1 = { package_id: build.package_id, out_dir: build.out_dir, path: file, sha256: (await observe.sha256(file))!, observedAtMs: Date.now(), resources };
        await stream.buildResource(row);
        observedResources.push(row);
      }
      const compilerResourceRoot = invocation.buildDirectory ? join(invocation.buildDirectory, COMPILER_RESOURCE_DIRECTORY_V1) : null;
      await stream.compilerResources(compilerResourceRoot);
      const compilerPaths = compilerResourceRoot
        ? [...new Set<string>(witnessed.filter((unit) => !unit.message.target.kind?.includes("proc-macro") && !unit.message.target.kind?.includes("custom-build")).flatMap((unit) => unit.rows.map((row) => row.path)))].filter((file) => basename(file) === COMPILER_RESOURCE_OBSERVATION_V1 && relative(compilerResourceRoot, file).split(sep)[0] !== ".." && !isAbsolute(relative(compilerResourceRoot, file)))
        : [];
      const bind = (identity: any, producer: boolean): Record<string, unknown> | null => {
        if (!identity || ![identity.manifest, identity.source].every((value) => typeof value === "string" && isAbsolute(value))) return null;
        const source = resolve(identity.source);
        const matches = witnessed.filter((unit) => typeof unit.message.manifest_path === "string" && resolve(unit.message.manifest_path) === resolve(identity.manifest) && unit.files.has(source) && (producer ? unit.message.target.kind.includes("proc-macro") : typeof identity.crate === "string" && unit.message.target.name.replaceAll("-", "_") === identity.crate && !unit.message.target.kind.includes("custom-build") && !unit.message.target.kind.includes("proc-macro")));
        return matches.length === 1 ? matches[0]!.message : null;
      };
      for (const file of compilerPaths) {
        const source = await observe.text(file);
        let observation: any;
        try {
          observation = source === null ? null : JSON.parse(source);
        } catch {
          observation = null;
        }
        const resources: CargoResourceWitnessV1[] = [];
        for (const row of Array.isArray(observation?.resources) ? observation.resources : []) resources.push(await observe.resource(row));
        await stream.compilerResource({ path: file, sha256: await observe.sha256(file), observedAtMs: Date.now(), producer: observation?.producer ?? null, caller: observation?.caller ?? null, producerUnit: bind(observation?.producer, true), callerUnit: bind(observation?.caller, false), resources });
      }
      const manifests = new Set<string>([invocation.manifest, ...witnessed.map((unit) => unit.message.manifest_path).filter((value): value is string => typeof value === "string")]);
      const inputPaths = new Set<string>(manifests);
      const visited = new Set<string>();
      for (let file of [invocation.cwd, ...[...manifests].map((value) => dirname(value))])
        for (;;) {
          if (visited.has(file)) break;
          visited.add(file);
          for (const name of ["Cargo.toml", "Cargo.lock", ".cargo/config.toml", ".cargo/config", "rust-toolchain.toml", "rust-toolchain"]) inputPaths.add(join(file, name));
          const parent = dirname(file);
          if (parent === file) break;
          file = parent;
        }
      for (const name of ["config.toml", "config"]) inputPaths.add(join(cargoHome, name));
      const invocationInputs: CargoInvocationInputV1[] = [];
      for (const file of [...inputPaths].sort()) invocationInputs.push({ path: file, sha256: await observe.sha256(file) });
      await stream.invocationInputs(invocationInputs);
      await stream.finish(Date.now());
      await physical.checkpoint();
      await handle.close();
      await lease.settle();
      await physical.checkpoint();
      this.staged.push({ temporary, destinations, physical });
      staged = true;
      return Object.freeze({ receiptPath, units: observedUnits, buildResources: observedResources });
    } finally {
      await handle.close().catch(() => undefined);
      if (!staged) await rm(temporary, { force: true });
    }
  }

  /** ✅️ Rechecks every input the session observed exactly once, then publishes every staged receipt and its ledger copies by atomic renames. */
  async close(): Promise<void> {
    if (this.state !== "open") throw Error("Cargo provenance session is closed");
    const copies: string[] = [];
    try {
      await this.custody.recheck();
      for (const row of this.staged) {
        await row.physical.checkpoint();
        for (const file of row.destinations.slice(1)) {
          await mkdir(dirname(file), { recursive: true });
          await row.physical.checkpoint();
          const copy = receiptTemporaryName(file);
          copies.push(copy);
          await copyFile(row.temporary, copy);
          await row.physical.checkpoint();
        }
      }
      let next = 0;
      for (const row of this.staged) {
        await row.physical.checkpoint();
        await rename(row.temporary, row.destinations[0]!);
        await row.physical.checkpoint();
        for (const file of row.destinations.slice(1)) { await rename(copies[next++]!, file); await row.physical.checkpoint(); }
      }
      this.state = "closed";
    } catch (error) {
      this.state = "failed";
      throw error;
    } finally {
      for (const copy of copies) await rm(copy, { force: true });
      if (this.state !== "closed") for (const row of this.staged) await rm(row.temporary, { force: true });
    }
  }

  /** 🗑️ Removes every staged temporary receipt of a session that will not close. */
  async discard(): Promise<void> {
    if (this.state === "closed") return;
    this.state = "failed";
    for (const row of this.staged) await rm(row.temporary, { force: true });
  }
}

/** 🔏️ Observes one completed Cargo invocation under complete current physical authority and publishes its receipt: a one-invocation {@link CargoProvenanceSessionV1}.
 * @see ./🧾️receipt/🟦️.ts
 */
export async function writeCompletedCargoInvocationProvenanceV1(receiptPath: string, invocation: CargoInvocationSourceV1, stagedPaths: ReadonlyMap<string, string>, cargoHome: string, physical: CurrentPhysicalPortV1): Promise<CargoProvenanceStagedV1> {
  const session = new CargoProvenanceSessionV1(portProvenanceCustodyV1(physical));
  try {
    const staged = await session.write(receiptPath, invocation, stagedPaths, cargoHome);
    await session.close();
    return staged;
  } finally {
    await session.discard();
  }
}

async function captureCargoArtifacts(manifest: string, args: string[], policy: CargoArtifactBuildPolicyV1, options: CargoArtifactBuildOptionsV1,targetDirectory:string, compilerStopped: () => void): Promise<void> {
  options.control.signal?.throwIfAborted();
  const path = resolve(manifest);
  const sourceRoot = dirname(path);
  const staging = options.observationOutput?cargoObservationOutputV1(options.observationOutput,policy.buildDirectory):resolve(sourceRoot, options.output ?? "dist/build");
  if (!options.observationOutput&&!staging.startsWith(sourceRoot + sep)) throw new Error("Cargo deliverables must belong to their source project");
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
  const granted = compilerAdmissionBudgetV1(options.control);
  const commandPort = options.commandPort ?? { command: "cargo", args: [] };
  const child = spawn(commandPort.command, [...commandPort.args, options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], {
    cwd: policy.cwd,
    env: { ...(options.environment ?? process.env), SEMIO_COMPILER_RESOURCE_ROOT: join(policy.buildDirectory, COMPILER_RESOURCE_DIRECTORY_V1), CARGO_TARGET_DIR:targetDirectory, CARGO_BUILD_BUILD_DIR: policy.buildDirectory },
    detached: process.platform !== "win32",
    stdio: ["inherit", "pipe", "pipe"],
  });
  const diagnosticsPath = join(policy.buildDirectory, "semio-cargo-logs", `${basename(capture)}.log`);
  mkdirSync(dirname(diagnosticsPath), { recursive: true });
  const diagnosticsLog = openSync(diagnosticsPath, "w");
  let diagnosticsOpen = true;
  const diagnostics = new BoundedDiagnosticsV1(diagnosticsLog, (line) => process.stderr.write(`${line}\n`));
  const diagnosticsDrained = new Promise<void>((accept) => { createInterface({ input: child.stderr!, crlfDelay: Infinity }).on("line", (line) => diagnostics.line(line)).once("close", accept); });
  const cancel = (): void => {
    cancelled = true;
    terminateOwnedChildTree(child);
  };
  options.control.signal?.addEventListener("abort", cancel, { once: true });
  if (options.control.signal?.aborted) cancel();
  const deadline = granted === 0 ? undefined : setTimeout(cancel, Math.min(granted, 2_147_483_647));
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
            if (primary && options.deliverables && !options.deliverables.some((extension) => file.endsWith(extension))) continue;
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
      const compilerStatus = await status;
      await diagnosticsDrained;
      diagnosticsOpen = false;
      closeSync(diagnosticsLog);
      for (const row of diagnostics.finish(compilerStatus !== 0 || cancelled, diagnosticsPath)) process.stderr.write(`${row}\n`);
      if (compilerStatus === 0 && !cancelled && diagnostics.suppressed === 0) rmSync(diagnosticsPath, { force: true });
      if (compilerStatus !== 0 || cancelled) throw new Error(`Cargo artifact build ${cancelled ? "cancelled" : "failed"}: ${owner}`);
      compilerStopped();
    } finally {
      if (diagnosticsOpen) { await diagnosticsDrained; closeSync(diagnosticsLog); }
      stopProgress();
      if (forceKill) clearTimeout(forceKill);
      options.control.signal?.removeEventListener("abort", cancel);
      if (deadline) clearTimeout(deadline);
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
    checkResourceLeaseControl(options.control);
    await stageArtifacts(staging, owner, files, { control: options.control, signal: options.control.signal, leaseDirectory: policy.leaseDirectory });
    checkResourceLeaseControl(options.control);
    console.log(`[nx-native] staged ${files.size} deliverables in ${relative(policy.cwd, staging).split(sep).join("/")}`);
  } finally {
    try {
      {
        const physical=new CurrentPhysicalOwnerV1(policy.cwd,cargoProvenancePhysicalControlV1({cancelled:()=>options.control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(options.control),onProgress:cargoProvenanceProgressSinkV1(row=>console.log("[cargo-provenance] "+JSON.stringify(row)))}));
        const provenanceReceipt = join(provenanceRoot ?? join(policy.buildDirectory, "semio-cargo-provenance"), `cargo-unit-provenance-${basename(capture)}.json`);
        await writeCompletedCargoInvocationProvenanceV1(provenanceReceipt, { manifest: path, cwd: policy.cwd, command: commandPort.command, args: [...commandPort.args, options.command ?? "build", "--locked", "--manifest-path", path, ...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], buildDirectory: policy.buildDirectory, builtAtMs, status: await status, cancelled, units, buildScripts }, new Map([...stagedNames].filter(([, key]) => files.has(key)).map(([path, key]) => [path, join(staging, key)])), resolve((options.environment ?? process.env).CARGO_HOME??join(homedir(),".cargo")),physical);
        options.provenance?.(provenanceReceipt);
      }
    } finally {
      rmSync(capture, { recursive: true, force: true });
    }
  }
}
