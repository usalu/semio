/** 🧰️ Contract: tree helpers find the common directory of files, list every item of a source including those of inline modules, and widen a removal over the doc comments above and the trailing blanks and newline; the corpus is language-agnostic and `🐍️.py` recomputes the common directory. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {rustOuterDocStart} from "../../../🌳️module-tree/🟦️.ts";
import {allItems,lcaDirectory,removeRange} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.lca)test("common directory of "+JSON.stringify(row.files),()=>{expect(lcaDirectory(row.files)).toBe(row.expected);});
for(const row of corpus.items)test("all items of "+JSON.stringify(row.text),()=>{expect(allItems(row.text).map(item=>item.name).filter(name=>name!==null)).toEqual(row.names);});
for(const row of corpus.removals)test("removal range of "+JSON.stringify(row.needle)+" in "+JSON.stringify(row.text),()=>{const start=row.text.indexOf(row.needle),range=removeRange(row.text,start,start+row.needle.length);expect(row.text.slice(0,range.start)+row.text.slice(range.end)).toBe(row.text.slice(0,rustOuterDocStart(row.text,start))+row.expected);expect(range.end).toBeGreaterThanOrEqual(start+row.needle.length);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
