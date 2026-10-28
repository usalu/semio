/** 📦️ Contract: a payload belongs to the nearest package owner exactly when the owner's authored export namespace (string, subpaths, conditions up to default, arrays) names a portable `./` target inside the payload directory and both carry the same package name; the corpus is language-agnostic and `🐍️.py` re-derives the verdicts. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {ownsPayload} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\"):path;
const adapt=<T extends {absDir:string}>(value:T):T=>({...value,absDir:platform(value.absDir)});
for(const row of corpus.cases)test("owns payload: "+row.name,()=>{expect(ownsPayload(adapt(row.owner as never),adapt(row.payload as never))).toBe(row.expected);});
test("the TypeScript facade is the same function",()=>{expect(typeof ownsPayload).toBe("function");});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
