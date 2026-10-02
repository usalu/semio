import { readFileSync, mkdirSync, mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { join, isAbsolute } from "node:path";
import { StringDecoder } from "node:string_decoder";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import { TEST_LEVELS, isTestLevel, type TestLevel } from "../🎚️budget/🟦️.ts";
import { runBudgetedTestCommand } from "../🎛️execution/🟦️.ts";
import { startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";

/** 🦀️ Binds a Cargo invocation to its exact caller-owned execution policy. */
export type CargoTestPolicyV1 = Readonly<{version:1;manifestPath:string;targetDirectory:string;nextest:boolean;configPath:string;level:TestLevel;assertionBudgets:Readonly<Record<TestLevel,number>>;buildBudgetMs:number;assertionThreads:number;artifactDirectory:string;retainArtifacts:boolean;coverageEnabled:boolean;coveragePath:string|null;rustMinStack:string}>;
/** 📋️ Selects an exact manifest and package inventory without repository discovery. */
export type CargoTestRequestV1 = Readonly<{manifestPath:string;packages:readonly string[];cwd:string;extraArgs?:readonly string[];environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal}>;
/** 🎬️ Describes a compiler, assertion, or report invocation under an explicit budget. */
export type CargoTestStepV1 = Readonly<{phase:"build"|"assert"|"report";args:string[];budgetMs:number;capture:boolean}>;
const policySchema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8"));

/** 🔐️ Refuses absent or malformed execution policies before starting a compiler. */
export function readCargoTestPolicyV1(environment:Readonly<Record<string,string|undefined>>):CargoTestPolicyV1 {
  const serialized=environment.SEMIO_CARGO_TEST_POLICY;
  if(!serialized) throw Error("Explicit Cargo test policy required");
  const policy=admitPolicy(JSON.parse(serialized)),level=isTestLevel(environment.SEMIO_TEST_LEVEL)?environment.SEMIO_TEST_LEVEL:policy.level;
  return {...policy,level,coverageEnabled:environment.SEMIO_COVERAGE===undefined?policy.coverageEnabled:environment.SEMIO_COVERAGE==="1"};
}
function admitPolicy(value:unknown):CargoTestPolicyV1 { const errors=validateJsonSchemaSubset(policySchema,value);if(errors.length)throw Error(`Invalid Cargo test policy: ${errors.join("; ")}`);return value as CargoTestPolicyV1; }

/** 🧪️ Preserves build selection on compilation and runtime filters on metadata execution. */
export function partitionNextestExecutionFilters(args: readonly string[]): { buildArgs: string[]; executionArgs: string[]; libtestArgs: string[] } {
  const valuedFilters = new Set(["-E", "--filter-expr", "--partition", "--run-ignored"]);
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
    if (arg === "--ignore-default-filter" || arg === "--no-fail-fast" || (arg.startsWith("-E") && arg.length > 2)) {
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


/** 🧭️ Projects one exact Cargo request into bounded build, assertion and coverage operations. */
export function cargoTestPlanV1(request:CargoTestRequestV1, input:CargoTestPolicyV1, metadataPath:string):CargoTestStepV1[] {
  const policy=admitPolicy(input);
  if(request.manifestPath!==policy.manifestPath||!isAbsolute(policy.configPath)||policy.coveragePath!==null&&!isAbsolute(policy.coveragePath)||!isAbsolute(request.manifestPath)||!isAbsolute(request.cwd)||!isAbsolute(metadataPath)||request.packages.some(name=>!name || name.startsWith("-"))) throw Error("Cargo tests require exact absolute manifest, working directory and metadata paths");
  const packages=["--manifest-path",request.manifestPath,...request.packages.flatMap(name=>["-p",name])], split=partitionNextestExecutionFilters(request.extraArgs??[]), skip=TEST_LEVELS.slice(TEST_LEVELS.indexOf(policy.level)+1).flatMap(level=>["--skip",`${level}::`]), profile=["--config-file",policy.configPath,"--profile",policy.level];
  const step=(phase:CargoTestStepV1["phase"],args:string[],capture=false):CargoTestStepV1=>({phase,args,budgetMs:phase==="assert"?policy.assertionBudgets[policy.level]:policy.buildBudgetMs,capture});
  if(policy.coverageEnabled){
    if(!policy.coveragePath)throw Error("Coverage requires an explicit owner report path");
    const operation=policy.nextest?"nextest":"test", args=["--release","--no-report",...(policy.nextest?["--no-tests","fail",...profile]:[]),...packages,...split.buildArgs,...split.executionArgs,"--",...split.libtestArgs,...skip];
    return [step("build",policy.nextest?["llvm-cov",operation,"--no-run",...args]:["llvm-cov",operation,...args,"--list"]),step("assert",["llvm-cov",operation,"--no-clean",...args]),step("report",["llvm-cov","report","--release","--lcov",...packages,"--output-path",policy.coveragePath])];
  }
  if(policy.nextest) return [step("build",["nextest","list","--list-type","binaries-only","--message-format","json",...profile,...packages,...split.buildArgs],true),step("assert",["nextest","run","--binaries-metadata",metadataPath,"--no-tests","fail","--status-level","fail","--final-status-level","fail",...(policy.level==="fundamental"?["--test-threads",String(policy.assertionThreads)]:[]),...profile,"--manifest-path",request.manifestPath,...split.executionArgs,"--",...split.libtestArgs,...skip])];
  return [step("build",["build","--tests",...packages,...split.buildArgs]),step("assert",["test",...packages,...split.buildArgs,...split.executionArgs,"--",...split.libtestArgs,...skip])];
}

/** 🔌️ Supplies a caller-owned executable for the Cargo command grammar. */
export type CargoTestExecutionPortV1=Readonly<{command:string;args:readonly string[]}>;

/** 🏃️ Executes the admitted plan, retaining metadata only when its caller explicitly requests it. */
export async function runCargoTestsV1(request:CargoTestRequestV1,input:CargoTestPolicyV1,port:CargoTestExecutionPortV1={command:"cargo",args:[]}):Promise<void>{
  const policy=admitPolicy(input);
  if(!isAbsolute(policy.artifactDirectory)) throw Error("Cargo artifact directory must be absolute");
  mkdirSync(policy.artifactDirectory,{recursive:true});
  const directory=mkdtempSync(join(policy.artifactDirectory,"semio-nextest-")), metadata=join(directory,"binaries-metadata.json"), environment=request.environment??process.env, env={...environment,RUST_MIN_STACK:environment.RUST_MIN_STACK??policy.rustMinStack};
  try {for(const step of cargoTestPlanV1(request,policy,metadata)){
    const stop=startNativeProgress(`cargo:${step.phase}`), decoder=new StringDecoder("utf8");let output="";
    try {await runBudgetedTestCommand(port.command,[...port.args,...step.args],{cwd:request.cwd,env,signal:request.signal,budgetMs:step.budgetMs,throwOnFailure:true,...(step.capture?{captureStdout:{limitBytes:536870912,onChunk:(bytes:Uint8Array)=>{output+=decoder.write(Buffer.from(bytes));}}}:{})});if(step.capture)writeFileSync(metadata,output+decoder.end());}
    finally {stop();}
  }} finally {if(policy.retainArtifacts)console.error(`[TRACE] Nextest artifacts retained at ${directory}`);else rmSync(directory,{recursive:true,force:true});}
}

/** 🧹️ Projects exact owner Clippy selection while keeping warning policy out of compiler flags. */
export function cargoLintPlanV1(request:CargoTestRequestV1,input:CargoTestPolicyV1):CargoTestStepV1{
  const policy=admitPolicy(input);
  cargoTestPlanV1({...request,extraArgs:[]},policy,join(policy.artifactDirectory,"lint-metadata.json"));
  return {phase:"build",args:["clippy","--manifest-path",request.manifestPath,...request.packages.flatMap(name=>["-p",name]),"--all-targets",...(request.extraArgs??[]),"--","-D","warnings"],budgetMs:policy.buildBudgetMs,capture:false};
}

/** 🛠️ Executes one exact manifest's Clippy plan under its existing build budget. */
export async function runCargoLintV1(request:CargoTestRequestV1,policy:CargoTestPolicyV1,port:CargoTestExecutionPortV1={command:"cargo",args:[]}):Promise<void>{
  const step=cargoLintPlanV1(request,policy),stop=startNativeProgress("cargo:lint");
  try {await runBudgetedTestCommand(port.command,[...port.args,...step.args],{cwd:request.cwd,env:request.environment??process.env,signal:request.signal,budgetMs:step.budgetMs,throwOnFailure:true});}finally{stop();}
}
