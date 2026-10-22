/** 🔢️ Contract: unified lines of code count JSON object keys, diff JSON by key units and count physical lines for everything else; the corpus is language-agnostic and `🐍️oracle.py` recounts with Python's `json` module. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {countJsonKeys,countUnifiedLocForFile,diffJsonUloc} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.keys)test("json keys "+JSON.stringify(row.text),()=>{expect(countJsonKeys(row.text)).toBe(row.expected);});
for(const[index,row]of corpus.diff.entries())test("json diff "+index+": "+JSON.stringify(row.before)+" -> "+JSON.stringify(row.after),()=>{expect(diffJsonUloc(row.before,row.after)).toEqual(row.expected);});
for(const row of corpus.loc)test("loc "+row.path+" "+JSON.stringify(row.data),()=>{expect(countUnifiedLocForFile(row.path,row.data)).toBe(row.expected);});
for(const row of corpus.locDefects)test.failing("loc defect "+row.path+" "+JSON.stringify(row.data),()=>{expect(countUnifiedLocForFile(row.path,row.data)).toBe(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
