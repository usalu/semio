/** 💰️ Contract: a crate budget is `<files>/<bytes>/<lines>` of positive safe integers, with a target below the hard compiler-ingress gate; the corpus is language-agnostic and `🐍️.py` parses the same strings. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {CRATE_HARD_BUDGET,CRATE_TARGET_BUDGET,parseCrateBudget} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
test("the target budget leaves headroom below the hard gate",()=>{expect(CRATE_TARGET_BUDGET).toEqual(corpus.constants.target);expect(CRATE_HARD_BUDGET).toEqual(corpus.constants.hard);for(const key of["files","bytes","lines"]as const)expect(CRATE_TARGET_BUDGET[key]).toBeLessThan(CRATE_HARD_BUDGET[key]);});
for(const row of corpus.ok)test("budget "+row.text,()=>{expect(parseCrateBudget(row.text)).toEqual(row.expected);});
for(const text of corpus.errors)test("budget refuses "+JSON.stringify(text),()=>{expect(()=>parseCrateBudget(text)).toThrow("a crate budget is written <files>/<bytes>/<lines> with positive integers, got "+JSON.stringify(text));});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
