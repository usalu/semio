/** 🎮️ Contract: the workspace cleanup command accepts exactly a closed mode plus an optional dry marker (and passes the rest of `taxonomy` and `test` through) and rejects everything else before touching a resource; the corpus is language-agnostic and `🐍️.py` parses the same argument vectors. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {readWorkspaceCleanupEntryV1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.accepted)test("accepts "+JSON.stringify(row.segments),()=>{const entry=readWorkspaceCleanupEntryV1(row.segments);expect<unknown>({mode:entry.mode,dry:entry.dry,arguments:[...entry.arguments]}).toEqual({mode:row.mode,dry:row.dry,arguments:row.arguments});expect(Object.isFrozen(entry)).toBe(true);expect(Object.isFrozen(entry.arguments)).toBe(true);});
for(const row of corpus.rejected)test("rejects "+JSON.stringify(row.segments),()=>{expect(()=>readWorkspaceCleanupEntryV1(row.segments)).toThrow("Closed workspace cleanup command required");});
for(const[index,value]of corpus.nonArrays.entries())test("rejects the non-array input "+index,()=>{expect(()=>readWorkspaceCleanupEntryV1(value as never)).toThrow("Closed workspace cleanup command required");});
test("an array holding a non-string is rejected",()=>{expect(()=>readWorkspaceCleanupEntryV1(["dry",1] as never)).toThrow("Closed workspace cleanup command required");});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
