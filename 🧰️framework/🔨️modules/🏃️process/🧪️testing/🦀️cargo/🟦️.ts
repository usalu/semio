import {buildModePolicy,type BuildMode} from "../../🏗️build/🎚️mode/🟦️.ts";
import policySchema from "./🧬️schema/🔣️.json" with {type:"json"};
import { statSync, readFileSync, mkdirSync, mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { join, isAbsolute } from "node:path";
import { StringDecoder } from "node:string_decoder";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { TEST_LEVELS, isTestLevel, type TestLevel } from "../🎚️budget/🟦️.ts";
import { runBudgetedTestCommand } from "../🎛️execution/🟦️.ts";
import { startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";
import {requireNativeStorageReservationV1,type NativeStorageReservationV1} from "../../📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {requireNativeStoragePathV1} from "../../📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../🔒️leases/🟦️.ts";
import {captureCargoCompilationGraphV1,actualCargoCompilerPlanningPortV1,type CargoCompilerPlanningPortV1} from "../../📦️artifacts/🏗️native-build/🕸️units/📥️capture/🟦️.ts";
import {admitCargoSourceBoundariesV1,actualCargoSourceAdmissionPortV1,type CargoSourceBoundaryPolicyV1,type CargoSourceAdmissionPortV1} from "../../📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";
import {acquireCargoBuildLeaseV1,cargoBuildLeaseIdentityV1,type CargoBuildLeaseIdentityV1} from "../../📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import { runtimeNativeSystemV1 } from "../../../⚙️runtime/🧲️native/🟦️.ts";
import{withCargoCompilerInputsV1,type CargoCompilerInputsPortV1}from"../../📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts";

/** 🦀️ Binds a Cargo invocation to its exact caller-owned execution policy. */
export type CargoTestPolicyV1 = Readonly<{version:1;mode:BuildMode;manifestPath:string;targetDirectory:string;buildDirectory:string;leaseDirectory:string;nextest:boolean;configPath:string;level:TestLevel;assertionBudgets:Readonly<Record<TestLevel,number>>;buildBudgetMs:number;assertionThreads:number;artifactDirectory:string;retainArtifacts:boolean;coverageEnabled:boolean;coveragePath:string|null;rustMinStack:string}>;
/** 📋️ Selects an exact manifest and package inventory without repository discovery. */
export type CargoTestRequestV1 = Readonly<{manifestPath:string;packages:readonly string[];cwd:string;extraArgs?:readonly string[];environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal;remainingMilliseconds?:()=>number|null}>;
/** 🎬️ Describes a compiler, assertion, or report invocation under an explicit budget. */
export type CargoTestStepV1 = Readonly<{phase:"build"|"assert"|"report";args:string[];budgetMs:number;capture:boolean}>;

/** 🔐️ Holds dependency identity fixed after acquisition and Source admission on every compiling operation. */
export function cargoTestExecutionArgumentsV1(step:CargoTestStepV1):string[]{
 const compiling=step.phase==="build"||step.phase==="assert"&&(step.args[0]==="test"||step.args[0]==="llvm-cov"&&step.args[1]==="test");if(!compiling)return[...step.args];const separator=step.args.indexOf("--"),head=separator<0?step.args:step.args.slice(0,separator),tail=separator<0?[]:step.args.slice(separator);return[...head,...["--locked","--offline"].filter(flag=>!head.includes(flag)),...tail];
}

/** 🔐️ Refuses absent or malformed execution policies before starting a compiler. */
export function readCargoTestPolicyV1(environment:Readonly<Record<string,string|undefined>>):CargoTestPolicyV1 {
  const serialized=environment.SEMIO_CARGO_TEST_POLICY;
  if(!serialized) throw Error("Explicit Cargo test policy required");
  const policy=admitPolicy(JSON.parse(serialized)),level=isTestLevel(environment.SEMIO_TEST_LEVEL)?environment.SEMIO_TEST_LEVEL:policy.level;
  return {...policy,level,coverageEnabled:environment.SEMIO_COVERAGE===undefined?policy.coverageEnabled:environment.SEMIO_COVERAGE==="1"};
}
function admitPolicy(value:unknown):CargoTestPolicyV1 { const errors=validateJsonSchemaSubset(policySchema,value);if(errors.length)throw Error(`Invalid Cargo test policy: ${errors.join("; ")}`);return value as CargoTestPolicyV1; }

/** 🪟️ Resolves Windows' system-owned short name without relocating the caller's profile directory. */
async function nativeCoverageDirectoryV1(directory:string):Promise<string>{
 const {dlopen,ptr,FFIType}=runtimeNativeSystemV1(),library=dlopen("kernel32.dll",{GetShortPathNameW:{args:[FFIType.ptr,FFIType.ptr,FFIType.u32],returns:FFIType.u32}}),input=Buffer.from(directory+"\0","utf16le");
 try{const required=library.symbols.GetShortPathNameW(ptr(input),null,0);if(required===0)throw Error("Windows coverage directory has no native identity");const output=Buffer.alloc((required+1)*2),length=library.symbols.GetShortPathNameW(ptr(input),ptr(output),required+1);if(length===0||length>required)throw Error("Windows coverage directory identity changed");const native=output.subarray(0,length*2).toString("utf16le");const original=statSync(directory,{bigint:true}),aliased=statSync(native,{bigint:true});if(original.ino===0n||original.ino!==aliased.ino||original.dev!==aliased.dev)throw Error("Windows coverage directory custody changed");return native;}finally{library.close();}
}

/** 📊️ Keeps Unicode paths on UTF-8 hosts and admits an exact ASCII identity for Windows' profiling C runtime. */
export async function cargoCoverageRuntimeDirectoryV1(directory:string,platform:string=process.platform,native:(directory:string)=>Promise<string>=nativeCoverageDirectoryV1):Promise<string>{
 if(platform!=="win32"||/^[\x20-\x7e]+$/.test(directory))return directory;
 const identity=await native(directory);if(!/^[\x20-\x7e]+$/.test(identity))throw Error("Windows LLVM coverage requires an ASCII native directory identity");return identity;
}

/** 🧪️ Preserves build selection on compilation and runtime filters on metadata execution. */
export function partitionNextestExecutionFilters(args: readonly string[]): { buildArgs: string[]; executionArgs: string[]; libtestArgs: string[] } {
  const reporterOptions = new Set(["--status-level", "--final-status-level"]), seenReporters = new Set<string>();
  const valuedFilters = new Set(["-E", "--filter-expr", "--partition", "--run-ignored", "--test-threads", "--success-output", "--failure-output", ...reporterOptions]);
  const requiredBuildOptions = new Set([
    "-p",
    "--package",
    "--exclude",
    "--manifest-path",
    "--target",
    "--target-dir",
    "--features",
    "-F",
    "--jobs",
    "-j",
    "--build-jobs",
    "--cargo-profile",
    "--cargo-message-format",
    "--config",
    "-Z",
    "--color",
    "--profile",
    "-P",
    "--test",
    "--bin",
    "--bench",
    "--example",
    "--message-format",
    "-T",
    "--list-type",
    "--archive-file",
    "--archive-format",
    "--extract-to",
    "--cargo-metadata",
    "--workspace-remap",
    "--binaries-metadata",
    "--target-dir-remap",
    "--build-dir-remap",
    "--config-file",
    "--user-config-file",
    "--tool-config-file",
  ]);
  const optionalBuildOptions = new Set(["--timings"]);
  const joinedBuildOptions = ["-p", "-F", "-j", "-Z", "-P", "-T"];
  const buildArgs: string[] = [],
    executionArgs: string[] = [];
  const separator = args.indexOf("--");
  const cargoArgs = separator < 0 ? args : args.slice(0, separator);
  const libtestArgs = separator < 0 ? [] : args.slice(separator + 1);
  for (let index = 0; index < cargoArgs.length; index += 1) {
    const arg = cargoArgs[index]!;
    const key = arg.split("=", 1)[0]!;
    if (reporterOptions.has(key)) {
      if (seenReporters.has(key)) throw Error(`Nextest reporter ${key} is repeated`);
      seenReporters.add(key);
    }
    if (arg === "--ignore-default-filter" || arg === "--no-fail-fast" || arg === "--nocapture" || arg === "--no-capture" || (arg.startsWith("-E") && arg.length > 2)) {
      executionArgs.push(arg);
    } else if (valuedFilters.has(key)) {
      if (arg.includes("=")) {
        if (arg.endsWith("=")) throw new Error(`Nextest filter ${key} requires a value`);
        executionArgs.push(arg);
      } else {
        const value = cargoArgs[index + 1];
        if (value === undefined || value.length === 0 || value.startsWith("-")) throw new Error(`Nextest filter ${key} requires a value`);
        executionArgs.push(arg, value);
        index += 1;
      }
    } else {
      if (!arg.startsWith("-")) executionArgs.push(arg);
      else {
        buildArgs.push(arg);
        const inlineValue = arg.includes("=") || joinedBuildOptions.some((option) => arg.startsWith(option) && arg.length > option.length);
        const requiresValue = requiredBuildOptions.has(key);
        const allowsValue = optionalBuildOptions.has(key);
        if ((requiresValue || allowsValue) && arg.endsWith("=")) throw new Error(`Nextest build option ${key} requires a non-empty value`);
        if (!inlineValue && requiresValue) {
          const value = cargoArgs[index + 1];
          if (value !== undefined && value.length > 0 && !value.startsWith("-")) {
            buildArgs.push(value);
            index += 1;
          } else if (requiresValue) throw new Error(`Nextest build option ${key} requires a value`);
        }
      }
    }
  }
  return { buildArgs, executionArgs, libtestArgs };
}


function requireOriginalCompilerArguments(args:readonly string[]):void{
 const boundary=args.indexOf("--"),compiler=boundary<0?args:args.slice(0,boundary);
 if(compiler.some(arg=>arg==="-r"||arg==="--release"||arg==="--profile"||arg.startsWith("--profile=")||arg==="--cargo-profile"||arg.startsWith("--cargo-profile=")||arg.startsWith("-P")))throw Error("Cargo compiler profile belongs to the original caller mode");
 if(compiler.some(arg=>arg==="--target-dir"||arg.startsWith("--target-dir=")))throw Error("Cargo compiler target belongs to the original caller storage");
}

/** 🧭️ Projects one exact Cargo request into bounded build, assertion and coverage operations. */
export function cargoTestPlanV1(request:CargoTestRequestV1, input:CargoTestPolicyV1, metadataPath:string):CargoTestStepV1[] {
  const policy=admitPolicy(input);
  if(request.manifestPath!==policy.manifestPath||!isAbsolute(policy.configPath)||policy.coveragePath!==null&&!isAbsolute(policy.coveragePath)||!isAbsolute(request.manifestPath)||!isAbsolute(request.cwd)||!isAbsolute(metadataPath)||request.packages.some(name=>!name || name.startsWith("-"))) throw Error("Cargo tests require exact absolute manifest, working directory and metadata paths");
  requireOriginalCompilerArguments(request.extraArgs??[]);
  const compilerProfile=buildModePolicy(policy.mode).cargoProfile,compiler=[policy.nextest?"--cargo-profile":"--profile",compilerProfile];
  const packages=["--manifest-path",request.manifestPath,...request.packages.flatMap(name=>["-p",name])], split=partitionNextestExecutionFilters(request.extraArgs??[]), skip=TEST_LEVELS.slice(TEST_LEVELS.indexOf(policy.level)+1).flatMap(level=>["--skip",`${level}::`]), profile=["--config-file",policy.configPath,"--profile",policy.level];
  const threads=split.executionArgs.some(arg=>arg.split("=",1)[0]==="--test-threads")?[]:policy.level==="fundamental"?["--test-threads",String(policy.assertionThreads)]:[];
  const step=(phase:CargoTestStepV1["phase"],args:string[],capture=false):CargoTestStepV1=>({phase,args,budgetMs:phase==="assert"?policy.assertionBudgets[policy.level]:policy.buildBudgetMs,capture});
  if(policy.coverageEnabled){
    if(!policy.coveragePath)throw Error("Coverage requires an explicit owner report path");
    const report=step("report",["llvm-cov","report","--profile",compilerProfile,"--lcov",...packages,"--output-path",policy.coveragePath]);
    if(policy.nextest){
      const archive=metadataPath+".tar.zst",reporters=["--status-level","--final-status-level"].flatMap(option=>split.executionArgs.some(arg=>arg.split("=",1)[0]===option)?[]:[option,"fail"]);
      return [step("build",["llvm-cov","nextest-archive",...compiler,...profile,...packages,...split.buildArgs,"--archive-file",archive]),step("assert",["llvm-cov","nextest",...compiler,"--no-report","--archive-file",archive,"--extract-overwrite","--no-tests","fail",...reporters,...(policy.level==="fundamental"?["--test-threads",String(policy.assertionThreads)]:[]),...profile,"--manifest-path",request.manifestPath,...split.executionArgs,"--",...split.libtestArgs,...skip]),report];
    }
    return [step("build",["llvm-cov","test",...compiler,"--no-report",...packages,...split.buildArgs,"--","--list"]),step("assert",["llvm-cov","test",...compiler,"--no-report",...packages,...split.buildArgs,...split.executionArgs,"--",...split.libtestArgs,...skip]),report];
  }
  if(policy.nextest) return [step("build",["nextest","list",...compiler,"--list-type","binaries-only","--message-format","json",...profile,...packages,...split.buildArgs],true),step("assert",["nextest","run","--binaries-metadata",metadataPath,"--no-tests","fail",...["--status-level","--final-status-level"].flatMap(option=>split.executionArgs.some(arg=>arg.split("=",1)[0]===option)?[]:[option,"fail"]),...(policy.level==="fundamental"?["--test-threads",String(policy.assertionThreads)]:[]),...profile,"--manifest-path",request.manifestPath,...split.executionArgs,"--",...split.libtestArgs,...skip])];
  return [step("build",["test","--no-run",...compiler,...packages,...split.buildArgs]),step("assert",["test",...compiler,...packages,...split.buildArgs,...split.executionArgs,"--",...split.libtestArgs,...skip])];
}

/** 📥️ Shares the exact admitted compiler selection with dependency preparation before assertion filters are applied. */
export function cargoTestCompilerArgumentsV1(request:CargoTestRequestV1,input:CargoTestPolicyV1):string[]{
 const policy=admitPolicy(input);cargoTestPlanV1(request,policy,join(policy.artifactDirectory,"binaries-metadata.json"));const split=partitionNextestExecutionFilters(request.extraArgs??[]);
 return["test","--no-run","--profile",buildModePolicy(policy.mode).cargoProfile,"--manifest-path",request.manifestPath,...request.packages.flatMap(name=>["-p",name]),...split.buildArgs];
}

/** 🔌️ Supplies a caller-owned executable for the Cargo command grammar. */
export type CargoTestExecutionPortV1=Readonly<{command:string;args:readonly string[]}>;

/** 🪪️ Projects Cargo's artifact profile without confusing Nextest's assertion configuration. */
export function cargoTestBuildLeaseIdentityV1(request:CargoTestRequestV1,input:CargoTestPolicyV1):CargoBuildLeaseIdentityV1{
 const policy=admitPolicy(input),plan=cargoTestPlanV1(request,policy,join(policy.artifactDirectory,"binaries-metadata.json")),args=plan[0]!.args,compiler:string[]=[];
 for(let index=0;index<args.length&&args[index]!=="--";index++){
  const arg=args[index]!,option=policy.nextest?"--cargo-profile":"--profile";
  if(arg==="--release")compiler.push(arg);
  else if(arg===option){const profile=args[++index];if(!profile||profile.startsWith("-"))throw Error("Cargo profile requires a value");compiler.push("--profile",profile);}
  else if(arg.startsWith(option+"="))compiler.push("--profile",arg.slice(option.length+1));
 }
 return cargoBuildLeaseIdentityV1(policy.buildDirectory,compiler);
}

/** 🎟️ Retains issued physical storage and the original caller through compiler and assertion settlement. */
export interface CargoTestAdmissionV1 {readonly workspaceRoot:string;readonly control:ResourceLeaseControl;readonly storage:NativeStorageReservationV1;readonly sourceBoundary:CargoSourceBoundaryPolicyV1;readonly planningPort?:CargoCompilerPlanningPortV1;readonly sourcePort?:CargoSourceAdmissionPortV1;readonly withCargoInputs?:CargoCompilerInputsPortV1}

async function admitCargoExecutionV1(request:CargoTestRequestV1,policy:CargoTestPolicyV1,admission:CargoTestAdmissionV1,compilerArgs:readonly string[]):Promise<Readonly<Record<string,string|undefined>>>{
 const control=admission?.control,storage=admission?.storage;if(!control)throw Error("Original Cargo test admission required");checkResourceLeaseControl(control);requireNativeStorageReservationV1(storage,control);if(storage.allocation.kind!=="compiler-workspace")throw Error("Compiler workspace reservation required");await storage.recheck();
 for(const path of [policy.targetDirectory,policy.buildDirectory,policy.artifactDirectory,...(policy.coveragePath?[policy.coveragePath]:[])])requireNativeStoragePathV1(storage.backing.root,path);
 const original=request.environment??process.env,env:Record<string,string|undefined>={...original,CARGO_TARGET_DIR:policy.targetDirectory,CARGO_BUILD_BUILD_DIR:policy.buildDirectory,RUST_MIN_STACK:original.RUST_MIN_STACK??policy.rustMinStack};
 for(const name of ["CARGO_HOME","TMPDIR","TMP","TEMP"]){const path=env[name];if(!path)throw Error("Bounded original Cargo environment required: "+name);requireNativeStoragePathV1(storage.backing.root,path);}
 const graph=await captureCargoCompilationGraphV1({workspaceRoot:admission.workspaceRoot,cwd:request.cwd,program:"cargo",prefix:[],args:compilerArgs,environment:env,maximumOutputBytes:16777216},control,admission.planningPort??actualCargoCompilerPlanningPortV1());await admitCargoSourceBoundariesV1(graph,admission.workspaceRoot,admission.sourceBoundary,control,admission.sourcePort??actualCargoSourceAdmissionPortV1(env,request.cwd,storage));await storage.recheck();return env;
}

/** 🏃️ Executes only after actual backing and selected Source admission, under unchanged original authority. */
export async function runCargoTestsV1(request:CargoTestRequestV1,input:CargoTestPolicyV1,admission:CargoTestAdmissionV1,port:CargoTestExecutionPortV1={command:"cargo",args:[]}):Promise<void>{
 if(!admission?.control)return Promise.reject(Error("Original Cargo test admission required"));const policy=admitPolicy(input),compilerArgs=cargoTestCompilerArgumentsV1(request,policy);return withCargoCompilerInputsV1({manifest:request.manifestPath,cwd:request.cwd,arguments:compilerArgs,environment:request.environment??process.env,control:admission.control},admission.withCargoInputs,async actual=>{
 request={...request,environment:actual.environment};admission={...admission,control:actual.control};const env=await admitCargoExecutionV1(request,policy,admission,actual.arguments),control=actual.control;
 mkdirSync(policy.artifactDirectory,{recursive:true});const directory=mkdtempSync(join(policy.artifactDirectory,"semio-nextest-")),metadata=join(directory,"binaries-metadata.json");requireNativeStoragePathV1(admission.storage.backing.root,directory);
 let lease:Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>>|undefined,primary:{error:unknown}|undefined;
 try{const plan=cargoTestPlanV1(request,policy,metadata),identity=cargoBuildLeaseIdentityV1(policy.buildDirectory,actual.arguments);lease=await acquireCargoBuildLeaseV1({directory:policy.leaseDirectory,buildDirectory:identity.buildDirectory,targetDirectory:policy.targetDirectory,args:actual.arguments,signal:control.signal,control});
  for(const step of plan){checkResourceLeaseControl(control);await admission.storage.recheck();const stop=startNativeProgress(`cargo:${step.phase}`),decoder=new StringDecoder("utf8");let output="";
   try{await withCargoCompilerInputsV1({manifest:request.manifestPath,cwd:request.cwd,arguments:cargoTestExecutionArgumentsV1(step),environment:env,control},admission.withCargoInputs,async actual=>{const remaining=checkResourceLeaseControl(actual.control),budgetMs=Math.floor(remaining===null?step.budgetMs:step.budgetMs>0?Math.min(step.budgetMs,remaining):remaining);await runBudgetedTestCommand(port.command,[...port.args,...actual.arguments],{cwd:request.cwd,env:actual.environment,signal:actual.control.signal,budgetMs,throwOnFailure:true,...(step.capture?{captureStdout:{limitBytes:16777216,onChunk:(bytes:Uint8Array)=>{output+=decoder.write(Buffer.from(bytes));}}}:{})});checkResourceLeaseControl(actual.control);});if(step.capture)writeFileSync(metadata,output+decoder.end());}
   catch(error){if(step.capture)writeFileSync(join(directory,`${step.phase}-failure.stdout.txt`),output+decoder.end());throw error;}finally{stop();}
  }
 }catch(error){primary={error};throw error;}finally{let failure:{error:unknown}|undefined;for(const cleanup of[()=>lease?.release(),()=>{if(policy.retainArtifacts)console.error(`[TRACE] Nextest artifacts retained at ${directory}`);else rmSync(directory,{recursive:true,force:true});}])try{cleanup();}catch(error){failure??={error};}if(!primary&&failure)throw failure.error;}
 });
}

/** 🧹️ Projects exact owner Clippy selection while keeping warning policy out of compiler flags. */
export function cargoLintPlanV1(request:CargoTestRequestV1,input:CargoTestPolicyV1):CargoTestStepV1{
  const policy=admitPolicy(input);
  requireOriginalCompilerArguments(request.extraArgs??[]);
  cargoTestPlanV1({...request,extraArgs:[]},policy,join(policy.artifactDirectory,"lint-metadata.json"));
  return {phase:"build",args:["clippy","--profile",buildModePolicy(policy.mode).cargoProfile,"--manifest-path",request.manifestPath,...request.packages.flatMap(name=>["-p",name]),"--all-targets",...(request.extraArgs??[]),"--","-D","warnings"],budgetMs:policy.buildBudgetMs,capture:false};
}

/** 🛠️ Executes admitted Clippy inside the same original physical compiler workspace. */
export async function runCargoLintV1(request:CargoTestRequestV1,policy:CargoTestPolicyV1,admission:CargoTestAdmissionV1,port:CargoTestExecutionPortV1={command:"cargo",args:[]}):Promise<void>{
 if(!admission?.control)return Promise.reject(Error("Original Cargo test admission required"));const step=cargoLintPlanV1(request,policy);return withCargoCompilerInputsV1({manifest:request.manifestPath,cwd:request.cwd,arguments:cargoTestExecutionArgumentsV1(step),environment:request.environment??process.env,control:admission.control},admission.withCargoInputs,async actual=>{request={...request,environment:actual.environment};admission={...admission,control:actual.control};const env=await admitCargoExecutionV1(request,policy,admission,actual.arguments),control=actual.control,identity=cargoBuildLeaseIdentityV1(policy.buildDirectory,actual.arguments),stop=startNativeProgress("cargo:lint");let lease:Awaited<ReturnType<typeof acquireCargoBuildLeaseV1>>|undefined,primary:{error:unknown}|undefined;
 try{lease=await acquireCargoBuildLeaseV1({directory:policy.leaseDirectory,buildDirectory:identity.buildDirectory,targetDirectory:policy.targetDirectory,args:actual.arguments,signal:control.signal,control});await admission.storage.recheck();const remaining=checkResourceLeaseControl(control);await runBudgetedTestCommand(port.command,[...port.args,...actual.arguments],{cwd:request.cwd,env,signal:control.signal,budgetMs:Math.floor(remaining===null?step.budgetMs:step.budgetMs>0?Math.min(step.budgetMs,remaining):remaining),throwOnFailure:true});checkResourceLeaseControl(control);}catch(error){primary={error};throw error;}finally{let failure:{error:unknown}|undefined;for(const cleanup of[()=>lease?.release(),stop])try{cleanup();}catch(error){failure??={error};}if(!primary&&failure)throw failure.error;}});
}
