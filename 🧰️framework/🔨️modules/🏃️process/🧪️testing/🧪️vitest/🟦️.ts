import { readFileSync } from "node:fs";
import { isAbsolute, join, resolve } from "node:path";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import { isTestLevel, TEST_LEVEL_BUDGET_MS } from "../🎚️budget/🟦️.ts";
import { runBudgetedTestCommand } from "../🎛️execution/🟦️.ts";

/** 🧪️ Binds a test runner, storage and runtime to the owning caller. */
export type VitestPolicyV1=Readonly<{version:1;cwd:string;toolPath:string;runtime:string;coverageRuntime:string;cacheRoot:string;coverageDirectory:string;budgetMs:number}>;
const schema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8"));
function admit(policy:VitestPolicyV1):VitestPolicyV1{
  if(validateJsonSchemaSubset(schema,policy).length||![policy.cwd,policy.toolPath,policy.cacheRoot,policy.coverageDirectory].every(isAbsolute))throw Error("Invalid Vitest owner policy");
  return policy;
}

/** 🔐️ Refuses missing storage/tool authority before loading a test config. */
export function readVitestPolicyV1(environment:Readonly<Record<string,string|undefined>>,ownerCwd?:string):VitestPolicyV1{
  if(!environment.SEMIO_VITEST_POLICY)throw Error("Explicit Vitest policy required");
  const policy=admit(JSON.parse(environment.SEMIO_VITEST_POLICY));
  if(ownerCwd!==undefined&&resolve(ownerCwd)!==policy.cwd)throw Error("Vitest policy belongs to another owner");
  const budgetMs=environment.SEMIO_TEST_BUDGET_MS!==undefined?Number(environment.SEMIO_TEST_BUDGET_MS):isTestLevel(environment.SEMIO_TEST_LEVEL)?TEST_LEVEL_BUDGET_MS[environment.SEMIO_TEST_LEVEL]:policy.budgetMs;
  return admit({...policy,budgetMs});
}

/** 📁️ Selects one safe cache leaf under the explicit caller's root. */
export function testCacheDirectoryV1(environment:Readonly<Record<string,string|undefined>>,id:string):string{
  if(!/^[A-Za-z0-9_-]+$/u.test(id))throw Error("Invalid test cache identity");
  return join(readVitestPolicyV1(environment).cacheRoot,id);
}

/** 📋️ Preserves owning config resolution, argument order and report semantics. */
export function vitestArgumentsV1(policy:VitestPolicyV1,args:readonly string[],config:string,coverage:boolean):string[]{
  admit(policy);if(!config)throw Error("Explicit Vitest config required");
  const ms=String(policy.budgetMs);
  return [policy.toolPath,"run","--config",resolve(policy.cwd,config),"--testTimeout",ms,"--hookTimeout",ms,"--teardownTimeout",ms,...(coverage?["--coverage.enabled","--coverage.provider=v8","--coverage.reporter=lcovonly",`--coverage.reportsDirectory=${policy.coverageDirectory}`]:[]),...args];
}

/** ⏱️ Runs the supplied test tool with cancellation and its exact wall-clock budget. */
export async function runVitestV1(policy:VitestPolicyV1,args:readonly string[],config:string,environment:Readonly<Record<string,string|undefined>>,signal?:AbortSignal):Promise<void>{
  const coverage=environment.SEMIO_COVERAGE==="1";
  await runBudgetedTestCommand(coverage?policy.coverageRuntime:policy.runtime,vitestArgumentsV1(policy,args,config,coverage),{cwd:policy.cwd,env:environment,budgetMs:policy.budgetMs,signal,throwOnFailure:true});
}
