import{appendCargoProfileConfigurationArgumentsV1}from"../../🏗️build/🎚️mode/⚙️cargo/🟦️.ts";
import {requireIssuedResourceControlV1,withIssuedResourceOperationV1} from "../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {checkResourceLeaseControl} from "../../🔒️leases/🟦️.ts";
import {withArtifactDescriptorSlotV1} from "../📤️publication/📥️sources/🟦️.ts";
import {observePhysicalFileV1} from "../../../📁️filesystem/🧾️observation/🟦️.ts";
import sourceSchema from "../📤️publication/📥️sources/🧬️schema/🔣️.json" with {type:"json"};
import {lstat} from "node:fs/promises";
import { acquireCompilerCapacityLeaseV1, compilerAdmissionBudgetV1, requestedCompilerAdmissionControlV1, type CompilerAdmissionControl } from "../../🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode,buildModePolicy, type BuildMode } from "../../🏗️build/🎚️mode/🟦️.ts";
import { acquireCargoBuildLeaseV1 } from "../🏗️native-build/🔒️lease/🟦️.ts";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { join, resolve,relative } from "node:path";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { collectArtifactFiles } from "../🗂️files/🟦️.ts";
import {captureOwnedCommandV1} from "../../🎛️owned-execution/🟦️.ts";
import { admitCargoSelectionV1, type CargoSourceBoundaryPolicyV1 } from "../🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import{withCargoCompilerInputsV1,type CargoCompilerInputsPortV1}from"../🏗️native-build/📥️inputs/🟦️.ts";

export type WasmPackWebPkg = {
  name: string;
  version?: string;
  files: readonly string[];
  main: string;
  module: string;
  types: string;
  sideEffects?: string[];
};

/** 🪪️ Identifies canonical mode artifacts from any package projection without relying on its link location. */
export type WasmPackageOwnerV1=Readonly<{version:1;sourceOwner:string;mode:BuildMode}>;

/** 🕸️ Binds the exact source owner, tools, mode, compiler storage and deadline. */
export type WasmBuildPolicyV1 = Readonly<{ version: 1; cwd: string; workspaceRoot: string; sourceBoundary: CargoSourceBoundaryPolicyV1; compilerConfiguration?: string; sourceOwner: string; mode: BuildMode; artifactDirectory: string; buildDirectory: string; leaseDirectory: string; budgetMs: number; packageName: string; searchPath: string; bindgen: Readonly<{ command: string; args: readonly string[] }>; wasmPack: Readonly<{ command: string; args: readonly string[] }> }>;
/** 📦️ Selects one owned browser package publication without discovering repository policy: `rsDir` owns the published package,
 * `componentDir` is the `🧩️component` cdylib crate wasm-bindgen links (a library crate is never a cdylib). */
export type WasmPackWebBuildOptions = Readonly<{ storage:NativeStorageReservationV1;consume:(capture:ProducedWasmPackageV1)=>Promise<void>;environment:Readonly<Record<string,string|undefined>>; control: CompilerAdmissionControl;withCargoInputs?:CargoCompilerInputsPortV1; rsDir: string; componentDir: string; logPrefix: string; pkg: WasmPackWebPkg; wasmBaseName: string; threads?: boolean; cargoFeatures?: readonly string[]; noDefaultFeatures?: boolean; cargoConfigurationSelectors: readonly string[] }>;
/** 🔌️ Executes an explicitly selected compiler with bounded cancellation; `admit` refuses the exact Cargo selection before it compiles. */
export type WasmBuildExecutionPortV1 = Readonly<{ run: (command: string, args: readonly string[], options: Readonly<{ cwd: string; environment: Readonly<Record<string,string|undefined>>; budgetMs: number; label: string; signal: AbortSignal;control:CompilerAdmissionControl }>) => Promise<void>; admit: (args: readonly string[], options: Readonly<{ cwd: string; environment: Readonly<Record<string,string|undefined>>; policy: WasmBuildPolicyV1; storage:NativeStorageReservationV1;control: CompilerAdmissionControl }>) => Promise<void> }>;

/** 📏️ Admits one wasm-bindgen Cargo selection through the one Source admission owner, keyed by the policy's repository root.
 * @see ../🏗️native-build/📥️source/🎟️admission/🟦️.ts */
export function admitWasmCargoSourcesV1(args: readonly string[], options: Readonly<{ cwd: string; environment: Readonly<Record<string,string|undefined>>; policy: WasmBuildPolicyV1; storage:NativeStorageReservationV1;control: CompilerAdmissionControl }>): Promise<void> {
  if(!options.storage)throw Error("Original enforcing WASM compiler store required");return admitCargoSelectionV1({ workspaceRoot: options.policy.workspaceRoot, cwd: options.cwd, args, environment: options.environment, sourceBoundary: options.policy.sourceBoundary,storage:options.storage }, options.control);
}
const wasmPolicySchema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", import.meta.url), "utf8"));
function admitWasmPolicy(value: unknown): WasmBuildPolicyV1 {
  const errors = validateJsonSchemaSubset(wasmPolicySchema, value);
  if (errors.length) throw Error(`Invalid WASM compiler policy: ${errors.join("; ")}`);
  return value as WasmBuildPolicyV1;
}
/** 📍️ Admits one portable package's declared compiler owner and mode-qualified canonical output. */
export function readWasmPackageOwnerV1(value:unknown):WasmPackageOwnerV1 {
 const errors=validateJsonSchemaSubset(wasmPolicySchema.definitions.PackageOwner,value);
 const owner=value as WasmPackageOwnerV1;
 if(errors.length)throw Error("Invalid WASM package owner");
 return owner;
}

/** 🔐️ Refuses missing compiler policy or a policy belonging to a different source owner. */
export function readWasmBuildPolicyV1(environment: Readonly<Record<string,string|undefined>>, cwd: string): WasmBuildPolicyV1 {
  if (!environment.SEMIO_WASM_BUILD_POLICY) throw Error("Explicit WASM compiler policy required");
  const policy = admitWasmPolicy(JSON.parse(environment.SEMIO_WASM_BUILD_POLICY));
  if (resolve(policy.cwd) !== resolve(cwd)) throw Error("WASM compiler policy belongs to a different owner");
  return policy;
}

/** 📂️ Validates one portable compiler output owner without executing a compiler. */
export function wasmOutputDirectory(rsDir: string, outputDirectory: string): string {
  if (!outputDirectory || /[/\\:*?"<>|\u0000]|[. ]$/u.test(outputDirectory)) throw new Error("WASM outputDirectory must be one portable literal directory name");
  return join(rsDir, outputDirectory);
}

/** 🎚️ Keeps wasm-pack profile flags separate from Cargo's explicit profile selection. */
export function wasmBuildArguments(profile: BuildMode): { pack: string[]; cargo: string[] } {profile=readBuildMode(profile);return{pack:["--profile",profile],cargo:["--profile",profile]};}

export interface WasmCompilerSelectionV1{readonly mode:BuildMode;readonly manifest:string;readonly configuration:string|null;readonly selectors:readonly string[];readonly threads:boolean;readonly features:readonly string[];readonly noDefaultFeatures:boolean}
/** 📐️ Binds both compiler entrypoints to one closed finite selection; this DATA issues no runtime authority. */
export function wasmCompilerSelectionArgumentsV1(value:unknown):Readonly<{cargo:readonly string[];wasmPackCargo:readonly string[]}>{
 if(validateJsonSchemaSubset(wasmPolicySchema.definitions.CompilerSelection,value,wasmPolicySchema).length)throw Error("Closed finite WASM compiler selection required");const row=value as WasmCompilerSelectionV1,flags=[...(row.configuration?["--config",row.configuration]:[]),...(row.noDefaultFeatures?["--no-default-features"]:[]),...row.features.flatMap(value=>["--features",value])],cargo=appendCargoProfileConfigurationArgumentsV1(["build","--locked","--lib","--manifest-path",row.manifest,"--profile",row.mode,"--target","wasm32-unknown-unknown",...(row.threads?["-Z","build-std=std,panic_abort"]:[]),...flags],row.mode,row.selectors),wasmPackCargo=appendCargoProfileConfigurationArgumentsV1(["--locked","--manifest-path",row.manifest,"--target","wasm32-unknown-unknown",...flags],row.mode,row.selectors);return Object.freeze({cargo,wasmPackCargo});
}

/** 📦️`wasm-pack build` for `--target web`, restores `pkg/package.json`, verifies wasm output. */
export async function buildWasmWebV1(opts: WasmPackWebBuildOptions, policy: WasmBuildPolicyV1, port: WasmBuildExecutionPortV1 = { run: async (command, args, options) => { await captureOwnedCommandV1(command,args,options.cwd,{env:options.environment,signal:options.signal,control:requestedCompilerAdmissionControlV1(options.control,options.budgetMs)}); }, admit: admitWasmCargoSourcesV1 }): Promise<void> {
  requireIssuedResourceControlV1(opts.control);requireNativeStorageReservationV1(opts.storage,opts.control);if(opts.storage.allocation.kind!=="compiler-workspace"||typeof opts.consume!=="function")throw Error("Actual compiler store and completed WASM consumer required");policy=admitWasmPolicy(policy);opts=Object.freeze({...opts,pkg:Object.freeze({...opts.pkg,files:Object.freeze([...opts.pkg.files])}),cargoConfigurationSelectors:Object.freeze([...opts.cargoConfigurationSelectors]),cargoFeatures:Object.freeze([...(opts.cargoFeatures??[])])});for(const path of[policy.artifactDirectory,policy.buildDirectory])requireNativeStoragePathV1(opts.storage.backing.root,path);if(relative(policy.workspaceRoot,resolve(opts.rsDir)).replaceAll("\\","/")!==policy.sourceOwner)throw Error("Exact declared WASM source owner required");if(opts.pkg.files.length>sourceSchema.properties.maximumFiles.const||opts.pkg.files.some(name=>validateJsonSchemaSubset(sourceSchema.definitions.SourceName,name).length))throw Error("Finite canonical declared package files required");
  const control = requestedCompilerAdmissionControlV1(opts.control, policy.budgetMs), signal = control.signal;
  if (resolve(policy.cwd) !== resolve(opts.rsDir)) throw Error("WASM policy belongs to a different owner");
  const { rsDir, componentDir, logPrefix, pkg, wasmBaseName, threads = false, cargoFeatures = [], noDefaultFeatures = false } = opts;
  const selection=wasmCompilerSelectionArgumentsV1({mode:policy.mode,manifest:join(componentDir,"Cargo.toml"),configuration:policy.compilerConfiguration??null,selectors:opts.cargoConfigurationSelectors,threads,features:cargoFeatures,noDefaultFeatures});
  const captureRoot = policy.artifactDirectory;
  mkdirSync(captureRoot, { recursive: true });
  const cargoOutput = mkdtempSync(join(captureRoot, "wasm-cargo-"));
  const packageOutput = mkdtempSync(join(captureRoot, "wasm-package-"));
  let primaryFailed=false;
  try {
    const pkgDir=packageOutput;
    const profile = buildModePolicy(policy.mode).cargoProfile;
    signal.throwIfAborted();
    const wasmPath = join(pkgDir, `${wasmBaseName}_bg.wasm`);
    const { pack: packProfileArgs } = wasmBuildArguments(profile);
    const profileOutDir = profile;
    const buildEnv:Record<string,string|undefined> = { ...opts.environment, PATH: policy.searchPath, CARGO_TARGET_DIR: cargoOutput, CARGO_BUILD_BUILD_DIR: policy.buildDirectory };
    const buildLabel = threads ? "cargo build (threaded) + wasm-bindgen" : "wasm-pack build";
    console.log(`[${logPrefix}] ${buildLabel} ${packProfileArgs.join(" ")} --target web --out-dir ${pkgDir} --out-name ${wasmBaseName} --no-pack`);
    const t0 = Date.now();
    await withCargoCompilerInputsV1({manifest:join(componentDir,"Cargo.toml"),cwd:componentDir,arguments:selection.cargo,environment:buildEnv,control},opts.withCargoInputs,async actual=>{
    let capacity: Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>> | undefined,lease: Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>> | undefined,bodyFailed=false;
    try{
    const signal=actual.control.signal;capacity=await acquireCompilerCapacityLeaseV1({request:{kind:"whole-host"},directory:policy.leaseDirectory,control:actual.control});lease=await acquireCargoBuildLeaseV1({directory:policy.leaseDirectory,buildDirectory:policy.buildDirectory,targetDirectory:cargoOutput,args:actual.arguments,signal:actual.control.signal,control:actual.control});checkResourceLeaseControl(actual.control);
    const environment:Record<string,string|undefined>={...actual.environment,SEMIO_COMPILER_CAPACITY_HELD:"true",SEMIO_COMPILER_PARALLELISM:String(capacity.parallelism)};if(!environment.MAKEFLAGS&&!environment.CARGO_MAKEFLAGS)environment.CARGO_BUILD_JOBS=String(capacity.parallelism);
    const invoke=async(command:string,args:readonly string[])=>{checkResourceLeaseControl(actual.control);await port.run(command,args,{cwd:componentDir,environment,budgetMs:compilerAdmissionBudgetV1(actual.control),label:logPrefix,signal:actual.control.signal,control:actual.control});checkResourceLeaseControl(actual.control);};
    await port.admit(actual.arguments,{cwd:componentDir,environment,policy,storage:opts.storage,control:actual.control});checkResourceLeaseControl(actual.control);
    if (threads) {
      const crateName = readFileSync(join(componentDir, "Cargo.toml"), "utf8").match(/^name\s*=\s*"([^"]+)"/m)?.[1];
      if (!crateName) {
        throw new Error(`[${logPrefix}] missing package name in ${join(componentDir, "Cargo.toml")}`);
      }
      const cargoWasm = join(cargoOutput, `wasm32-unknown-unknown/${profileOutDir}`, `${crateName.replace(/-/g, "_")}.wasm`);
      const threadedCargoArgs = actual.arguments;
      await invoke("cargo", threadedCargoArgs);
      if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
      await invoke(policy.bindgen.command, [...policy.bindgen.args, cargoWasm, "--out-dir", pkgDir, "--typescript", "--target", "web", "--out-name", wasmBaseName]);
    } else {
      const forwarded:string[]=[];for(let index=1;index<actual.arguments.length;index++){const argument=actual.arguments[index]!;if(argument==="--profile"){index++;continue;}if(argument!=="--lib")forwarded.push(argument);}
      const buildArgs = ["build", "--mode", "no-install", ...packProfileArgs, "--target", "web", "--out-dir", pkgDir, "--out-name", wasmBaseName, "--no-pack", ...(policy.mode === "prototype" ? ["--no-opt"] : []), "--", ...forwarded];
      await invoke(policy.wasmPack.command, [...policy.wasmPack.args, ...buildArgs]);
    }
    console.log(`[${logPrefix}] wasm build done in ${((Date.now() - t0) / 1000).toFixed(1)}s`);

    if (!existsSync(pkgDir)) mkdirSync(pkgDir, { recursive: true });
    const snippetFiles=[...(await collectArtifactFiles(pkgDir,signal)).keys()].filter(name=>name.startsWith("snippets/"));
    const pkgJson = {
      type: "module",
      version: pkg.version ?? "0.1.0",
      sideEffects: pkg.sideEffects ?? ["./snippets/*"],
      ...pkg,
      semioWasmOwner: {version:1,sourceOwner:policy.sourceOwner,mode:policy.mode},
      files: [...new Set([...pkg.files, ...snippetFiles])],
    };
    writeFileSync(join(pkgDir, "package.json"), `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

    if (existsSync(wasmPath)) {
      const sz = (statSync(wasmPath).size / (1024 * 1024)).toFixed(2);
      const files = await collectArtifactFiles(pkgDir, signal);
      await consumeProducedWasmPackageV1(files,readWasmPackageOwnerV1(pkgJson.semioWasmOwner),{...opts,control:actual.control},()=>{capacity?.release();capacity=undefined;});
      console.log(`[${logPrefix}] owned ${policy.mode}/${wasmBaseName}_bg.wasm consumed (${sz} MiB)`);
    } else {
      throw new Error(`[${logPrefix}] expected wasm output missing: ${wasmPath}`);
    }
    }catch(error){bodyFailed=true;throw error;}finally{
      let failed=false,failure:unknown;for(const cleanup of[()=>{const held=lease;lease=undefined;held?.release();},()=>{const held=capacity;capacity=undefined;held?.release();}])try{cleanup();}catch(error){if(!failed){failed=true;failure=error;}}if(!bodyFailed&&failed)throw failure;
    }
    });
  }catch(error){primaryFailed=true;throw error;}finally{
    let failed=false,failure:unknown;for(const cleanup of[()=>rmSync(cargoOutput,{recursive:true,force:true}),()=>rmSync(packageOutput,{recursive:true,force:true})])try{cleanup();}catch(error){if(!failed){failed=true;failure=error;}}if(!primaryFailed&&failed)throw failure;
  }
}



declare const wasmPackageBrand:unique symbol;
export type ProducedWasmPackageV1=Readonly<{readonly[wasmPackageBrand]:true}>;
export interface ProducedWasmPackageViewV1{readonly files:ReadonlyMap<string,string>;readonly owner:WasmPackageOwnerV1;readonly control:CompilerAdmissionControl;recheck():Promise<void>}
type PackageScope={live:boolean;accepting:boolean;view:ProducedWasmPackageViewV1;pending:Set<Promise<unknown>>;failures:unknown[]};
const producedPackages=new WeakMap<ProducedWasmPackageV1,PackageScope>();
/** 📥️ Enrolls a task only against an actually issued finite package while its compiler store remains held. */
export function withProducedWasmPackageTaskV1<T>(token:ProducedWasmPackageV1,body:(view:ProducedWasmPackageViewV1)=>Promise<T>):Promise<T>{const scope=producedPackages.get(token);if(!scope)throw Error("Actual paid WASM package required");const admitted=scope.live&&scope.accepting&&typeof body==="function"&&scope.pending.size<sourceSchema.properties.maximumPendingOperations.const,task=withIssuedResourceOperationV1(scope.view.control,async()=>{await Promise.resolve();if(!admitted)throw Error("Live accepted WASM package task required");await scope.view.recheck();const value=await body(Object.freeze({...scope.view,files:new Map(scope.view.files)}));await scope.view.recheck();return value;});if(admitted){scope.pending.add(task);void task.then(()=>scope.pending.delete(task),error=>{scope.pending.delete(task);scope.failures.push(error);});}return task;}
/** ✅️ Rechecks the actual retained package without exposing a compiler-state path after retirement. */
export function verifyProducedWasmPackageV1(token:ProducedWasmPackageV1):Promise<void>{return withProducedWasmPackageTaskV1(token,async view=>{checkResourceLeaseControl(view.control);await view.recheck();});}
async function consumeProducedWasmPackageV1(files:ReadonlyMap<string,string>,owner:WasmPackageOwnerV1,options:WasmPackWebBuildOptions,compilerStopped:()=>void):Promise<void>{
 const limits=sourceSchema.properties;requireIssuedResourceControlV1(options.control);requireNativeStorageReservationV1(options.storage,options.control);if(!files.size||files.size>limits.maximumFiles.const)throw Error("Finite completed WASM package required");const stamp=(row:{dev:bigint;ino:bigint;size:bigint;mode:bigint;mtimeNs:bigint;ctimeNs:bigint})=>[row.dev,row.ino,row.size,row.mode,row.mtimeNs,row.ctimeNs],facts=new Map<string,{path:string;sha256:string;byteLength:number;identity:readonly bigint[]}>(),observe=(maximum:number)=>({maxBytes:maximum,maxWork:Math.ceil(maximum/limits.chunkBytes.const)+16,chunkBytes:limits.chunkBytes.const,cancelled:()=>options.control.signal.aborted,remainingMs:()=>checkResourceLeaseControl(options.control)??Number.MAX_SAFE_INTEGER,onProgress:()=>checkResourceLeaseControl(options.control)});let bytes=0;
 for(const[name,path]of files){requireNativeStoragePathV1(options.storage.backing.root,path);const stat=await lstat(path,{bigint:true}),size=Number(stat.size);if(validateJsonSchemaSubset(sourceSchema.definitions.SourceName,name).length||!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1n||!Number.isSafeInteger(size)||size>limits.maximumBytes.const-bytes)throw Error("Finite single-link actual package file required");bytes+=size;const fact=await withArtifactDescriptorSlotV1(()=>observePhysicalFileV1(path,observe(size)));facts.set(name,{...fact,identity:stamp(stat)});}
 const recheck=async()=>{checkResourceLeaseControl(options.control);await options.storage.recheck();for(const fact of facts.values()){requireNativeStoragePathV1(options.storage.backing.root,fact.path);const stat=await lstat(fact.path,{bigint:true});if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1n||stamp(stat).some((value,index)=>value!==fact.identity[index]))throw Error("Actual WASM package identity changed");const actual=await withArtifactDescriptorSlotV1(()=>observePhysicalFileV1(fact.path,observe(fact.byteLength)));if(actual.sha256!==fact.sha256||actual.byteLength!==fact.byteLength)throw Error("Actual WASM package bytes changed");}},token=Object.freeze({})as ProducedWasmPackageV1,scope:PackageScope={live:true,accepting:true,view:Object.freeze({files:new Map(files),owner:Object.freeze({...owner}),control:options.control,recheck}),pending:new Set(),failures:[]};producedPackages.set(token,scope);let failed=false,failure:unknown;
 compilerStopped();try{try{await withIssuedResourceOperationV1(options.control,()=>options.consume(token));}catch(error){failed=true;failure=error;}scope.accepting=false;while(scope.pending.size)await Promise.allSettled([...scope.pending]);if(failed)throw failure;if(scope.failures.length)throw scope.failures[0];await recheck();}finally{scope.accepting=false;scope.live=false;}
}
