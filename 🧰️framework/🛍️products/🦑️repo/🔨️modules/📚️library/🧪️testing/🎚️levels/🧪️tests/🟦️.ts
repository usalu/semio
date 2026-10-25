/** 🎚️ Contract: each test toolchain gets cumulative level arguments — per-level timeouts, `-short` through quick, a skip or marker filter for every higher level — from one ordered level list and one budget table; the corpus is language-agnostic and `🐍️.py` derives the same argument vectors. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {bunTestLevelArgs,dotnetLevelArgs,goLevelTestArgs,playwrightTestTimeoutMs,pytestLevelArgs,vitestLevelArgs} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const withEnv=<T,>(values:Record<string,string|undefined>,action:()=>T):T=>{const saved:Record<string,string|undefined>={};for(const key of Object.keys(values)){saved[key]=process.env[key];if(values[key]===undefined)delete process.env[key];else process.env[key]=values[key];}try{return action();}finally{for(const[key,value]of Object.entries(saved))if(value===undefined)delete process.env[key];else process.env[key]=value;}};
const clean={SEMIO_TEST_BUDGET_MS:undefined,SEMIO_TEST_LEVEL:undefined};
for(const row of corpus.levels)test("level arguments of "+row.level,()=>{withEnv(clean,()=>{
 const level=row.level as "fundamental";expect(goLevelTestArgs(level)).toEqual(row.go);expect(vitestLevelArgs(level)).toEqual(row.vitest);expect(bunTestLevelArgs(level)).toEqual(row.bun);expect(pytestLevelArgs(level)).toEqual(row.pytest);expect(dotnetLevelArgs(level)).toEqual(row.dotnet);expect(playwrightTestTimeoutMs(level)).toBe(row.playwright);
});});
test("a budget override changes every timeout, rounding seconds up",()=>{withEnv({SEMIO_TEST_BUDGET_MS:corpus.override.budgetMs,SEMIO_TEST_LEVEL:undefined},()=>{expect(goLevelTestArgs("fundamental")).toEqual(corpus.override.go);expect(pytestLevelArgs("fundamental")).toEqual(corpus.override.pytest);expect(bunTestLevelArgs("fundamental")).toEqual(corpus.override.bun);expect(playwrightTestTimeoutMs("fundamental")).toBe(corpus.override.playwright);});});
test("the active level comes from the environment and an unknown value falls back to fundamental",()=>{
 withEnv({SEMIO_TEST_BUDGET_MS:undefined,SEMIO_TEST_LEVEL:corpus.default.env},()=>{expect(bunTestLevelArgs()).toEqual(corpus.default.expectedBun);});
 withEnv({SEMIO_TEST_BUDGET_MS:undefined,SEMIO_TEST_LEVEL:"nonsense"},()=>{expect(bunTestLevelArgs()).toEqual(corpus.invalidEnvDefaultsToFundamental);});
 withEnv(clean,()=>{expect(bunTestLevelArgs()).toEqual(corpus.invalidEnvDefaultsToFundamental);});
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
