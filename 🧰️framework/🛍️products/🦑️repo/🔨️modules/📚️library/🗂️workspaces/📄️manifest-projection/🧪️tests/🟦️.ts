/** 📄️ Contract: the root package document is parsed as an object whose `workspaces` is a string array and re-rendered with two-space indentation, every other field in its original order and a trailing newline; the corpus is language-agnostic and `🐍️.py` renders the same documents with Python's `json`. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {parseWorkspaceRootDocument,renderWorkspaceRootDocument} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.parse)test("parse: "+row.name,()=>{const document=parseWorkspaceRootDocument(row.source);expect(document.workspaces??[]).toEqual(row.workspaces);expect(Object.keys(document)).toEqual(Object.keys(JSON.parse(row.source)));});
for(const row of corpus.rejects)test("parse refuses "+row.name,()=>{if(row.message==="")expect(()=>parseWorkspaceRootDocument(row.source)).toThrow();else expect(()=>parseWorkspaceRootDocument(row.source)).toThrow(row.message);});
for(const[index,row]of corpus.render.entries())test("render "+index,()=>{const rendered=renderWorkspaceRootDocument(parseWorkspaceRootDocument(row.source),row.workspaces);expect(rendered).toBe(row.expected);expect(JSON.parse(rendered).workspaces).toEqual(row.workspaces);});
test("rendering does not mutate the parsed document",()=>{const document=parseWorkspaceRootDocument('{"workspaces":["a"]}');renderWorkspaceRootDocument(document,["b"]);expect(document.workspaces).toEqual(["a"]);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
