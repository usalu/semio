/** 📜️ Contract: the commit bundle body parser reads emoji scope lines, `🎆️YY🌙️MM☀️DD` day sections and emoji bullets, ignores comments, metrics and sign-offs, and refuses a stray subject, footer, path or malformed bullet with a fixed message; the corpus is language-agnostic and `🐍️.py` is a line-state machine over the same text. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {commitBundleBodyError,parseCommitBundleBody} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
test("a full bundle body is parsed into scopes, days and bullets",()=>{expect(parseCommitBundleBody(corpus.valid.text)).toEqual(corpus.valid.expected);});
for(const row of corpus.errors)test("parse refuses "+row.name,()=>{expect(()=>parseCommitBundleBody(row.text)).toThrow(row.message);});
for(const row of corpus.defects)test.failing("parse defect: "+row.name,()=>{if((row as {expected?:unknown}).expected!==undefined)expect<unknown>(parseCommitBundleBody(row.text)).toEqual((row as {expected:unknown}).expected);else expect(()=>parseCommitBundleBody(row.text)).toThrow(row.message);});
for(const row of corpus.bodyErrors)test("body error: "+row.name,()=>{const error=commitBundleBodyError(row.text);if(row.message===null)expect(error).toBeNull();else expect(error).toContain(row.message);});
test("every parsed bundle has a date and every day a bullet",()=>{for(const bundle of parseCommitBundleBody(corpus.valid.text)){expect(bundle.dates.length).toBeGreaterThan(0);for(const date of bundle.dates)expect(date.bullets.length).toBeGreaterThan(0);}});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
