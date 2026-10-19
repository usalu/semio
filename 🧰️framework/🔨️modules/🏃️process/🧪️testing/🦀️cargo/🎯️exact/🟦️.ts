import{readExactCargoExecutableV1,selectExactCargoLawsV1,requireExactCargoLawResultV1,requireExactCargoFingerprintV1,requireExactCargoTargetBoundaryV1,readExactCargoLawNamesV1}from"./📋️protocol/🟦️.ts";
import{readExactCargoLawOutputSelectionV1,type ExactCargoLawOutputSelectionV1}from"./📦️outputs/🎯️selection/🟦️.ts";
import exactOutputSchema from"./📦️outputs/🎯️selection/🧬️schema/🔣️.json";
import{requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1,withScriptInvocationSignalV1}from"../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {captureOwnedProcess,type OwnedProcessCaptureOptions,type OwnedProcessCaptureResult} from "../../../📥️capture/🟦️.ts";
import { closeSync, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { isAbsolute, join, resolve, sep } from "node:path";
import { createHash } from "node:crypto";
import policySchema from "./📋️policy/🧬️schema/🔣️.json" with { type: "json" };
import configuredPolicy from "./📋️policy/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { advanceScriptInvocation, checkScriptInvocation, type ScriptInvocation } from "../../../🧭️routing/📥️invocation/🟦️.ts";
import {requestedCompilerAdmissionControlV1,compilerAdmissionBudgetV1} from "../../../🏗️build/🎟️capacity/🟦️.ts";
import {acquireCompilerCapacityLeaseV1} from "../../../🏗️build/🎟️capacity/🟦️.ts";
import {acquireCargoBuildLeaseV1} from "../../../📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../../📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../../📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1,type CargoCompilerPlanningPortV1} from "../../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";
import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1,type CargoSourceBoundaryPolicyV1,type CargoSourceAdmissionPortV1} from "../../../📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import type {ResourceLeaseControl} from "../../../🔒️leases/🟦️.ts";

export type ExactCargoLawGroup = {
  package: string;
  target: { kind: "lib"; name?: string } | { kind: "test" | "bin"; name: string };
  laws: readonly string[];
  cargoArgs?: readonly string[];
};
export type ExactCargoLawStage = "build" | "list" | "native";
/** 🔌️ Owned process/fingerprint port permits deterministic hostile runner laws without compiling Cargo. */
export type ExactCargoLawPreparationOptionsV1=Omit<OwnedProcessCaptureOptions,"stdoutPath"|"stderrPath">&Readonly<{control:ResourceLeaseControl}>;
/** 🎟️ Carries an admitted exact Cargo command and its actual resource phase. */
export type ExactCargoLawInputsV1=Readonly<{arguments:readonly string[];environment:Readonly<Record<string,string|undefined>>;control:ResourceLeaseControl}>;
export type ExactCargoLawPort = {
  prepare?: (command:string,args:string[],options:ExactCargoLawPreparationOptionsV1)=>Promise<void>;
  withBuildInputs?: <T>(command:string,args:string[],options:ExactCargoLawPreparationOptionsV1,control:ResourceLeaseControl,body:(inputs:ExactCargoLawInputsV1)=>Promise<T>)=>Promise<T>;
  probe: (command: string, args: string[], options: OwnedProcessCaptureOptions) => Promise<OwnedProcessCaptureResult>;
  fingerprint: (path: string) => { path: string; sha256: string };
  nativeLibraryDirectory?: (options:OwnedProcessCaptureOptions)=>Promise<string>;
};
/** 📋️ Declares actual build, discovery, assertion and output authority. */
export type ExactCargoLawPolicyV1 = Readonly<{ version: 1; buildMilliseconds: number; listMilliseconds: number; lawMilliseconds: number; transport: Readonly<{ maximumBuildOutputBytes: number; maximumNativeOutputBytes: number }> }>;
/** 🔐️ Admits the complete authored execution policy independently of testing examples. */
export function readExactCargoLawPolicyV1(value: unknown): ExactCargoLawPolicyV1 {
  if (validateJsonSchemaSubset(policySchema, value).length) throw Error("Complete exact Cargo law policy required");
  return value as ExactCargoLawPolicyV1;
}
/** ⚙️ Reads the authored neutral execution configuration. */
export function configuredExactCargoLawPolicyV1(): ExactCargoLawPolicyV1 { return readExactCargoLawPolicyV1(configuredPolicy); }
export type ExactCargoLawOptions = {
  invocation: ScriptInvocation;
  control:ResourceLeaseControl;
  storage:NativeStorageReservationV1;
  leaseDirectory:string;
  sourceBoundary:CargoSourceBoundaryPolicyV1;
  planningPort?:CargoCompilerPlanningPortV1;
  sourcePort?:CargoSourceAdmissionPortV1;
  policy: ExactCargoLawPolicyV1;
  cwd: string;
  groups: readonly ExactCargoLawGroup[];
  manifestPaths: Readonly<Record<string, string>>;
  cargoTargetDir: string;
  cargoArgs?: readonly string[];
  env?: Readonly<Record<string, string | undefined>>;
  nativeEnv?: Readonly<Record<string, string | undefined>>;
  artifactDir?: string;
  cancelled?: () => boolean;
  progress?: (event: { stage: ExactCargoLawStage; package: string; law?: string; artifactDir: string }) => void | Promise<void>;
  consume?: (outputs:ExactCargoLawOutputsV1)=>Promise<void>;
};
export type ExactCargoLawReceipt = {
  package: string;
  target: ExactCargoLawGroup["target"];
  executable: string;
  sha256: string;
  laws: readonly string[];
  assertions: number;
  artifactDir: string;
  cargoTargetDir: string;
};

declare const exactCargoOutputBrand:unique symbol;
/** 📦️ Identifies actual completed law output only during its retained producing callback. */
export interface ExactCargoLawOutputsV1{readonly receipts:readonly Readonly<ExactCargoLawReceipt>[];readonly[exactCargoOutputBrand]:true}
/** 📁️ Borrows the producing directory and exact descendant phase for one completed receipt. */
export interface ExactCargoLawOutputTaskV1{readonly receipt:Readonly<ExactCargoLawReceipt>;readonly directory:string;readonly control:ResourceLeaseControl}
type ExactOutputIdentity={readonly dev:bigint;readonly ino:bigint};
type ExactOutputOwner={invocation:ScriptInvocation;control:ResourceLeaseControl;storage:NativeStorageReservationV1;receipts:readonly Readonly<ExactCargoLawReceipt>[];directories:ReadonlyMap<string,ExactOutputIdentity>;live:boolean;accepting:boolean;pending:Set<Promise<unknown>>;failures:unknown[]};
const exactOutputOwners=new WeakMap<ExactCargoLawOutputsV1,ExactOutputOwner>(),exactOutputPolicy=Object.freeze({...exactOutputSchema.definitions.Policy.const});
function actualExactOutputOwner(outputs:ExactCargoLawOutputsV1):ExactOutputOwner{const owner=exactOutputOwners.get(outputs);if(!owner?.live||!owner.accepting)throw Error("Actual live exact Cargo output scope required");requireIssuedResourceControlInvocationV1(owner.control,owner.invocation);requireNativeStorageReservationV1(owner.storage,owner.control);return owner;}
async function recheckExactOutput(owner:ExactOutputOwner,receipt:Readonly<ExactCargoLawReceipt>):Promise<void>{compilerAdmissionBudgetV1(owner.control);await owner.storage.recheck();compilerAdmissionBudgetV1(owner.control);requireNativeStoragePathV1(owner.storage.backing.root,receipt.artifactDir);const expected=owner.directories.get(receipt.artifactDir),state=lstatSync(receipt.artifactDir,{bigint:true});if(!expected||!state.isDirectory()||state.isSymbolicLink()||state.dev!==expected.dev||state.ino!==expected.ino)throw Error("Exact Cargo output directory identity changed");}
/** 🪢️ Enrolls a genuine output borrow before effects and fences it only after every accepted child drains. */
export function withExactCargoLawOutputV1<T>(outputs:ExactCargoLawOutputsV1,selection:ExactCargoLawOutputSelectionV1,body:(task:ExactCargoLawOutputTaskV1)=>Promise<T>):Promise<T>{try{const owner=actualExactOutputOwner(outputs),captured=readExactCargoLawOutputSelectionV1(selection),receipt=owner.receipts[captured.receiptIndex];if(!receipt||typeof body!=="function"||owner.pending.size>=exactOutputPolicy.maximumPendingTasks)throw Error("Finite exact Cargo output task required");const work=withIssuedResourceOperationV1(owner.control,async()=>{await recheckExactOutput(owner,receipt);let failed=false,failure:unknown,value:T|undefined;try{value=await withRequestedIssuedResourceControlV1(owner.control,0,control=>body(Object.freeze({receipt,directory:receipt.artifactDir,control})));}catch(error){failed=true;failure=error;}try{await recheckExactOutput(owner,receipt);}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;return value as T;});owner.pending.add(work);void work.then(()=>{owner.pending.delete(work);},error=>{owner.accepting=false;if(!owner.failures.length)owner.failures.push(error);owner.pending.delete(work);});return work;}catch(error){const failure=Promise.reject(error);void failure.catch(()=>{});return failure;}}
async function consumeExactOutputs(invocation:ScriptInvocation,control:ResourceLeaseControl,storage:NativeStorageReservationV1,receipts:readonly ExactCargoLawReceipt[],directories:ReadonlyMap<string,ExactOutputIdentity>,consume:(outputs:ExactCargoLawOutputsV1)=>Promise<void>):Promise<void>{await withRequestedIssuedResourceControlV1(control,0,async actualControl=>{const completed=Object.freeze(receipts.map(row=>Object.freeze({...row,target:Object.freeze({...row.target}),laws:Object.freeze([...row.laws])}))),outputs=Object.freeze({receipts:completed})as ExactCargoLawOutputsV1,owner:ExactOutputOwner={invocation,control:actualControl,storage,receipts:completed,directories,live:true,accepting:true,pending:new Set(),failures:[]};exactOutputOwners.set(outputs,owner);let failed=false,failure:unknown;try{try{await consume(outputs);}catch(error){failed=true;failure=error;}owner.accepting=false;while(owner.pending.size)await Promise.allSettled([...owner.pending]);if(!failed&&owner.failures.length){failed=true;failure=owner.failures[0];}if(failed)throw failure;for(const receipt of completed)await recheckExactOutput(owner,receipt);}finally{owner.accepting=false;owner.live=false;}});}

export const EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX = ".exact-cargo-laws-active-";
export const EXACT_CARGO_ACTIVE_LEASE_MANIFEST = "lease.json";
export const EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS = 120_000;

/** 🛡️ Recognizes fresh process-bound output for exact Cargo and managed verification owners. */
export function generatedOutputHasLiveLease(root: string): boolean {
  const stack = [{ path: root, depth: 0 }];
  while (stack.length > 0) {
    const current = stack.pop()!;
    let names: string[];
    try {
      names = readdirSync(current.path);
    } catch {
      continue;
    }
    for (const name of names) {
      const path = join(current.path, name);
      if (name === "state.json" && current.path.includes(`${sep}managed-verification${sep}`)) {
        try {
          const state = lstatSync(path);
          if (!state.isFile() || state.isSymbolicLink() || state.size > 65_536) continue;
          const manifest = JSON.parse(readFileSync(path, "utf8")) as { status?: unknown; pid?: unknown };
          if (!["scheduled", "running"].includes(String(manifest.status)) || !Number.isSafeInteger(manifest.pid) || Number(manifest.pid) < 1) continue;
          const output = manifest.status === "scheduled" ? state : lstatSync(join(current.path, "output.log"));
          if (!output.isFile() || output.isSymbolicLink() || Date.now() - output.mtimeMs > EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS || output.mtimeMs - Date.now() > 5_000) continue;
          try { process.kill(Number(manifest.pid), 0); return true; }
          catch (error) { if ((error as NodeJS.ErrnoException).code === "EPERM") return true; }
        } catch {}
        continue;
      }
      let state;
      try {
        state = lstatSync(path);
      } catch {
        continue;
      }
      if (!state.isDirectory() || state.isSymbolicLink()) continue;
      if (name.startsWith(EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX)) {
        const manifestPath = join(path, EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
        try {
          const manifestState = lstatSync(manifestPath);
          if (!manifestState.isFile() || manifestState.isSymbolicLink() || manifestState.size > 128 || Date.now() - manifestState.mtimeMs > EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS || manifestState.mtimeMs - Date.now() > 5_000) continue;
          const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { version?: unknown; pid?: unknown };
          if (manifest.version !== 1 || !Number.isSafeInteger(manifest.pid) || Number(manifest.pid) < 1) continue;
          try {
            process.kill(Number(manifest.pid), 0);
            return true;
          } catch (error) {
            if ((error as NodeJS.ErrnoException).code === "EPERM") return true;
          }
        } catch {
          continue;
        }
      } else if (current.depth < 4) stack.push({ path, depth: current.depth + 1 });
    }
  }
  return false;
}

/** 💓 Holds a fresh process-bound lease until the exact Cargo run reaches a terminal result. */
function beginExactCargoLease(artifactRoot: string): () => void {
  const leaseRoot = mkdtempSync(join(artifactRoot, EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX));
  const manifestPath = join(leaseRoot, EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
  const heartbeat = (): void => {
    try {
      writeFileSync(manifestPath, JSON.stringify({ version: 1, pid: process.pid }), { mode: 0o600 });
    } catch {}
  };
  heartbeat();
  const timer = setInterval(heartbeat, 10_000);
  timer.unref?.();
  return () => {
    clearInterval(timer);
    rmSync(leaseRoot, { recursive: true, force: true });
  };
}

/** 🚫️ Preserves the precise failing stage and actual child status independently of assertion parsing. */
export class ExactCargoLawError extends Error {
  constructor(
    readonly stage: ExactCargoLawStage,
    readonly status: number | null,
    readonly signal: string | null,
    readonly artifactDir: string,
    detail: string,
  ) {
    super(`exact Cargo law ${stage} failed: status=${status} signal=${signal ?? "none"} artifacts=${artifactDir}; ${detail}`);
  }
}

/** 🔬️ Fingerprints one retained executable descriptor with bounded streaming and cancellation. */
export function exactExecutableFingerprint(path: string, control: Readonly<{ cancelled?: () => boolean; progress?: (completed: number, total: number) => void }> = {}): { path: string; sha256: string; byteLength: number } {
  const check = () => {
    if (control.cancelled?.()) throw new Error("Executable fingerprint cancelled");
  };
  check();
  if (!isAbsolute(path) || lstatSync(path).isSymbolicLink()) throw new Error("Executable must be one absolute regular file");
  const canonical = realpathSync(path);
  const descriptor = openSync(canonical, "r");
  try {
    const before = fstatSync(descriptor);
    const same = (other: typeof before) => other.isFile() && !other.isSymbolicLink() && other.dev === before.dev && other.ino === before.ino && other.size === before.size && other.mtimeMs === before.mtimeMs && other.ctimeMs === before.ctimeMs;
    if (!before.isFile() || !same(lstatSync(canonical)) || before.size <= 0 || before.size > 8 * 1024 ** 3 || (process.platform !== "win32" && (before.mode & 0o111) === 0)) throw new Error("Executable size or type denied");
    const digest = createHash("sha256");
    const buffer = Buffer.alloc(64 * 1024);
    let count = 0;
    while (true) {
      check();
      const length = readSync(descriptor, buffer);
      if (length === 0) break;
      digest.update(buffer.subarray(0, length));
      count += length;
      if (count > before.size) throw new Error("Executable changed while hashing");
      control.progress?.(count, before.size);
    }
    const after = fstatSync(descriptor);
    if (count !== before.size || !same(after) || !same(lstatSync(canonical))) throw new Error("Executable changed while hashing");
    return { path: canonical, sha256: digest.digest("hex"), byteLength: count };
  } finally {
    closeSync(descriptor);
  }
}

/** 🧰️ Discovers the compiler's actual native standard libraries under the original process invocation. */
export async function originalNativeLibraryDirectory(options:OwnedProcessCaptureOptions):Promise<string>{
 const result=await captureOwnedProcess(options.env?.RUSTC??"rustc",["--print","target-libdir"],options);
 const directory=result.stdout.trim();
 if(result.status!==0||result.signal!==null||result.reason&&result.reason!=="exit"||!isAbsolute(directory)||directory.includes("\n")||directory.length>4096)throw Error("Original native Rust standard library discovery failed");
 const canonical=realpathSync(directory);if(!lstatSync(canonical).isDirectory())throw Error("Original native Rust library owner is not a directory");return canonical;
}
/** 🔗️ Preserves original environment fields and adds only the actual compiler's native library owners. */
export function originalNativeLibraryEnvironment(env:Readonly<Record<string,string|undefined>>,directory:string,platform:string=process.platform):Record<string,string|undefined>{
 const key=platform==="win32"?(Object.keys(env).find(key=>key.toLowerCase()==="path")??"PATH"):platform==="darwin"?"DYLD_LIBRARY_PATH":"LD_LIBRARY_PATH",separator=platform==="win32"?";":":";
 return {...env,[key]:[directory,resolve(directory,"../../.."),env[key]].filter(value=>value!==undefined&&value!=="").join(separator)};
}
const originalExactCargoPort:ExactCargoLawPort={probe:captureOwnedProcess,fingerprint:exactExecutableFingerprint,nativeLibraryDirectory:originalNativeLibraryDirectory};

/** 🧪️ Compiles each explicit target once and executes only its hash-bound, exact-listed native laws. */
export async function runExactCargoLaws(options: ExactCargoLawOptions, port: ExactCargoLawPort = originalExactCargoPort): Promise<readonly ExactCargoLawReceipt[]> {
  const invocation=options.invocation,control=options.control;requireIssuedResourceControlInvocationV1(control,invocation);
  const consumer=Object.getOwnPropertyDescriptor(options,"consume");if(consumer&&(!("value"in consumer)||typeof consumer.value!=="function"&&consumer.value!==undefined))throw Error("Own exact Cargo output consumer required");const consume=consumer?.value as ExactCargoLawOptions["consume"];
  port=Object.freeze({prepare:port.prepare,withBuildInputs:port.withBuildInputs,probe:port.probe,fingerprint:port.fingerprint,nativeLibraryDirectory:port.nativeLibraryDirectory});
  return withIssuedResourceOperationV1(control,async()=>{
  const policy = readExactCargoLawPolicyV1(options.policy);
  checkScriptInvocation(invocation);
  if(consume&&(port.probe!==captureOwnedProcess||port.fingerprint!==exactExecutableFingerprint||port.nativeLibraryDirectory!==originalNativeLibraryDirectory))throw Error("Actual exact Cargo process producers required for output consumption");
  const storage=requireNativeStorageReservationV1(options.storage,control);if(storage.allocation.kind!=="compiler-workspace")throw Error("Original exact Cargo compiler storage required");await storage.recheck();
  if (invocation.policy.maximumElapsedMilliseconds > 0 && policy.buildMilliseconds === 0) throw Error("Finite original owner requires positive build authority");
  const configuredEnv = options.env ?? process.env;
  const artifactRoot = options.artifactDir ?? configuredEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("Exact Cargo laws require an absolute artifactDir or SEMIO_TEST_ARTIFACT_DIR");
  const cargoTargetDir = options.cargoTargetDir;
  if (!cargoTargetDir || !isAbsolute(cargoTargetDir)) throw new Error("Explicit absolute Cargo target directory required");
  requireExactCargoTargetBoundaryV1(options.cwd,cargoTargetDir);
  const env:Record<string,string|undefined> = { ...configuredEnv, CARGO_TARGET_DIR: cargoTargetDir };
  const nativeEnv: Record<string, string | undefined> = { ...env, ...options.nativeEnv, CARGO_TARGET_DIR: cargoTargetDir };
  delete nativeEnv.RUST_TEST_NOCAPTURE;
  for(const path of [artifactRoot,cargoTargetDir,env.CARGO_HOME,env.CARGO_BUILD_BUILD_DIR]){if(!path)throw Error("Original bounded exact Cargo directories required");requireNativeStoragePathV1(storage.backing.root,path);}
  for(const name of ["TMPDIR","TMP","TEMP","SEMIO_TEST_ARTIFACT_DIR"] as const)for(const environment of [env,nativeEnv])if(environment[name])requireNativeStoragePathV1(storage.backing.root,environment[name]!);
  if (!isAbsolute(options.cwd) || !options.groups.length || options.groups.length > 64) throw new Error("Exact Cargo laws require a bounded nonempty target list and absolute cwd");
  const groupKeys = options.groups.map((group) => JSON.stringify([group.package, group.target.kind, group.target.name ?? ""]));
  if (new Set(groupKeys).size !== groupKeys.length) throw new Error("Exact Cargo groups must combine laws for the same package/target");
  for (const group of options.groups) {
    if (!isAbsolute(options.manifestPaths[group.package] ?? "")) throw new Error("Explicit absolute Cargo manifest required for each package");
    if(!group.package)throw Error("Exact Cargo package required");
    readExactCargoLawNamesV1(group.laws);
  }
  let endLease: (() => void) | undefined;
  let runRoot: string | undefined;
  try {
    const cancelled = () => control.signal.aborted || options.cancelled?.() === true;
    const receipts: ExactCargoLawReceipt[] = [];
    const outputDirectories=new Map<string,ExactOutputIdentity>();
    for (const [index, group] of options.groups.entries()) {
      let groupRoot = artifactRoot;
      let stage: ExactCargoLawStage = "build";
      let last: OwnedProcessCaptureResult = { status: null, signal: null, stdout: "", stderr: "" };
      const fail = (detail: string): never => {
        throw new ExactCargoLawError(stage, last.status, last.signal, groupRoot, detail);
      };
      const checkpoint = (): void => {
        compilerAdmissionBudgetV1(control);
        if (cancelled()) fail("cancelled");
        checkScriptInvocation(invocation);
      };
      const capture = async (next: ExactCargoLawStage, command: string, args: string[], budget: number, name: string): Promise<OwnedProcessCaptureResult> => {
        stage = next;
        const phaseControl=requestedCompilerAdmissionControlV1(control,budget);
        checkpoint();
        await storage.recheck();
        const preparation={control:phaseControl,cwd:options.cwd,env,budgetMs:compilerAdmissionBudgetV1(phaseControl),maxOutputBytes:policy.transport.maximumBuildOutputBytes,invocation:invocation};
        if(next==="build")await port.prepare?.(command,args,preparation);
        const execute=async(consumption:ExactCargoLawInputsV1)=>{
        const args=[...consumption.arguments],env=consumption.environment,consumedControl=consumption.control;requireIssuedResourceControlRefinementV1(phaseControl,consumedControl);compilerAdmissionBudgetV1(consumedControl);
        if(next==="build"){const graph=await captureCargoCompilationGraphV1({workspaceRoot:options.cwd,cwd:options.cwd,program:command,prefix:[],args,environment:env,maximumOutputBytes:16777216},consumedControl,options.planningPort??actualCargoCompilerPlanningPortV1());compilerAdmissionBudgetV1(consumedControl);await admitCargoSourceBoundariesV1(graph,options.cwd,options.sourceBoundary,consumedControl,options.sourcePort??actualCargoSourceAdmissionPortV1(env,options.cwd,storage));compilerAdmissionBudgetV1(consumedControl);await storage.recheck();compilerAdmissionBudgetV1(consumedControl);}
        await advanceScriptInvocation(invocation, "exact-cargo:" + stage + ":" + group.package, "running");compilerAdmissionBudgetV1(consumedControl);
        await options.progress?.({ stage, package: group.package, ...(next === "native" ? { law: args[0] } : {}), artifactDir: groupRoot });compilerAdmissionBudgetV1(consumedControl);
        checkpoint();
        compilerAdmissionBudgetV1(consumedControl);
        if (!runRoot) {
          mkdirSync(artifactRoot, { recursive: true });
          endLease = beginExactCargoLease(artifactRoot);
          runRoot = mkdtempSync(join(artifactRoot, "exact-cargo-laws-"));
        }
        const selectedGroupRoot = join(runRoot, String(index).padStart(2, "0"));
        if (groupRoot !== selectedGroupRoot) { mkdirSync(selectedGroupRoot); groupRoot = selectedGroupRoot;const identity=lstatSync(groupRoot,{bigint:true});if(!identity.isDirectory()||identity.isSymbolicLink())throw Error("Fresh exact Cargo group directory required");outputDirectories.set(groupRoot,{dev:identity.dev,ino:identity.ino}); }
        const stdoutPath = join(groupRoot, `${name}.stdout`);
        const stderrPath = join(groupRoot, `${name}.stderr`);
        if(next==="build"){const lease=await acquireCargoBuildLeaseV1({directory:options.leaseDirectory,buildDirectory:env.CARGO_BUILD_BUILD_DIR!,targetDirectory:cargoTargetDir,args,signal:consumedControl.signal,control:consumedControl});let leaseFailed=false;try{compilerAdmissionBudgetV1(consumedControl);const capacity=await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:options.leaseDirectory,control:consumedControl,environment:env});let capacityFailed=false;try{compilerAdmissionBudgetV1(consumedControl);last=await withScriptInvocationSignalV1(invocation,consumedControl.signal,invocation=>port.probe(command,args,{cwd:options.cwd,env:{...env,...(!env.MAKEFLAGS&&!env.CARGO_MAKEFLAGS?{CARGO_BUILD_JOBS:String(capacity.parallelism)}:{})},budgetMs:compilerAdmissionBudgetV1(consumedControl),maxOutputBytes:policy.transport.maximumBuildOutputBytes,stdoutPath,stderrPath,invocation}));compilerAdmissionBudgetV1(consumedControl);}catch(error){capacityFailed=true;throw error;}finally{try{capacity.release();}catch(error){if(!capacityFailed)throw error;}}}catch(error){leaseFailed=true;throw error;}finally{try{lease.release();}catch(error){if(!leaseFailed)throw error;}}}else last = await withScriptInvocationSignalV1(invocation,consumedControl.signal,invocation=>port.probe(command, args, { cwd: options.cwd, env:{...nativeEnv,SEMIO_TEST_ARTIFACT_DIR:groupRoot}, budgetMs: compilerAdmissionBudgetV1(consumedControl), maxOutputBytes: policy.transport.maximumNativeOutputBytes, stdoutPath, stderrPath, invocation }));
        compilerAdmissionBudgetV1(consumedControl);
        await storage.recheck();compilerAdmissionBudgetV1(consumedControl);
        if (!existsSync(stdoutPath)) writeFileSync(stdoutPath, last.stdout, { flag: "wx", mode: 0o600 });
        if (!existsSync(stderrPath)) writeFileSync(stderrPath, last.stderr, { flag: "wx", mode: 0o600 });
        writeFileSync(join(groupRoot, `${name}.json`), JSON.stringify({ command, args, cargoTargetDir, status: last.status, signal: last.signal, reason: last.reason ?? "exit" }), { flag: "wx", mode: 0o600 });
        checkpoint();compilerAdmissionBudgetV1(consumedControl);
        return last;
        };
        return next==="build"&&port.withBuildInputs?await port.withBuildInputs(command,args,preparation,phaseControl,execute):await execute({arguments:args,environment:env,control:phaseControl});
      };
      const target = group.target.kind === "lib" ? ["--lib"] : [`--${group.target.kind}`, group.target.name];
      const cargoArgs = [...(options.cargoArgs ?? []), ...(group.cargoArgs ?? [])];
      if (
        cargoArgs.some(
          (arg) =>
            ["--", "--test", "--bin", "--lib", "-p", "--package", "--no-run", "--message-format", "--target-dir", "--manifest-path"].includes(arg) || ["--message-format=", "--target-dir=", "--manifest-path="].some((prefix) => arg.startsWith(prefix)),
        )
      )
        fail("Cargo target/control arguments are helper-owned");
      const built = await capture(
        "build",
        "cargo",
        ["test", "--manifest-path", options.manifestPaths[group.package]!, "-p", group.package, ...target, ...cargoArgs, "--no-run", "--message-format=json"],
        policy.buildMilliseconds,
        "build",
      );
      const protocol=<T>(read:()=>T):T=>{try{return read();}catch(error){return fail(error instanceof Error?error.message:String(error));}};
      const executable=protocol(()=>readExactCargoExecutableV1(built,group.package,group.target));
      const fingerprint = (): { path: string; sha256: string } => {
        try {
          return port.fingerprint(executable);
        } catch (error) {
          return fail(String(error));
        }
      };
      const initial = fingerprint();
      const verify = (): void => {
        checkpoint();
        const current = fingerprint();
        protocol(()=>requireExactCargoFingerprintV1(initial,current));
      };
      protocol(()=>requireExactCargoFingerprintV1(initial,initial));
      writeFileSync(join(groupRoot, "executable.json"), JSON.stringify({ package: group.package, target: group.target, ...initial }), { flag: "wx", mode: 0o600 });
      verify();
      if(port.nativeLibraryDirectory){
        stage="list";checkpoint();const libraryControl=requestedCompilerAdmissionControlV1(control,policy.listMilliseconds);
        const directory=await withScriptInvocationSignalV1(invocation,libraryControl.signal,invocation=>port.nativeLibraryDirectory!({cwd:options.cwd,env,budgetMs:compilerAdmissionBudgetV1(libraryControl),maxOutputBytes:policy.transport.maximumNativeOutputBytes,stdoutPath:join(groupRoot,"toolchain-runtime.stdout"),stderrPath:join(groupRoot,"toolchain-runtime.stderr"),invocation}));compilerAdmissionBudgetV1(libraryControl);checkpoint();Object.assign(nativeEnv,originalNativeLibraryEnvironment(nativeEnv,directory));
        writeFileSync(join(groupRoot,"toolchain-runtime.json"),JSON.stringify({directory,cargoTargetDir}),{flag:"wx",mode:0o600});
      }
      const listed = await capture("list", initial.path, ["--list"], policy.listMilliseconds, "list");
      verify();
      const laws=protocol(()=>selectExactCargoLawsV1(listed,group.laws));
      for (const [lawIndex, law] of laws.entries()) {
        verify();
        const result = await capture("native", initial.path, [law, "--exact", "--include-ignored", "--test-threads=1", "--show-output"], policy.lawMilliseconds, `law-${lawIndex}`);
        verify();
        protocol(()=>requireExactCargoLawResultV1(result,law));
      }
      const receipt = { package: group.package, target: group.target, executable: initial.path, sha256: initial.sha256, laws, assertions: laws.length, artifactDir: groupRoot, cargoTargetDir };
      writeFileSync(join(groupRoot, "receipt.json"), JSON.stringify(receipt), { flag: "wx", mode: 0o600 });
      receipts.push(receipt);
    }
    if(consume)await consumeExactOutputs(invocation,control,storage,receipts,outputDirectories,consume);
    return receipts;
  } finally {
    endLease?.();
  }
  });
}
