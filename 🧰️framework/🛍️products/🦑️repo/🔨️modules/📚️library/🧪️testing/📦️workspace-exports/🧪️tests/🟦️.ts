/** 📦️ Contract: the workspace test aliases are built from explicit public runtime exports only — an exact package name, portable `./` keys and targets, the first of node, import and default in object order, no wildcards, parents or node_modules; the corpus is language-agnostic, `resolve.exports` is the independent third-party oracle for the selected targets and `🐍️oracle.py` re-derives the selection. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {exports as resolveExports} from "resolve.exports";
import {publicRuntimePackageExportsV1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.ok)test("exports: "+row.name,()=>{const found=publicRuntimePackageExportsV1(row.manifest);expect(found).toEqual(row.expected);if(row.expected.length>0)expect(Object.isFrozen(found)).toBe(true);for(const entry of found)expect(Object.isFrozen(entry)).toBe(true);});
for(const row of corpus.errorDefects)test.failing("exports defect: "+row.name,()=>{expect(()=>publicRuntimePackageExportsV1(row.manifest)).toThrow(row.message);});
for(const row of corpus.errors)test("exports refuse "+row.name,()=>{expect(()=>publicRuntimePackageExportsV1(row.manifest)).toThrow(row.message);});
test("resolve.exports resolves every selected export to the same target",()=>{
 for(const row of corpus.ok){
  const manifest=row.manifest as {name:string};
  for(const entry of row.expected){const key=entry.specifier===manifest.name?".":"."+entry.specifier.slice(manifest.name.length);expect<unknown>(resolveExports(manifest as never,key,{conditions:["node","import"]})?.[0],row.name+" "+entry.specifier).toBe(entry.target);}
 }
});
test("resolve.exports finds no runtime target where the selection omits an export",()=>{
 for(const row of corpus.ok.filter(item=>item.expected.length===0&&(item.manifest as {exports?:unknown}).exports!=null)){
  let resolved:string[]|void=undefined;try{resolved=resolveExports(row.manifest as never,".",{conditions:["node","import"]});}catch{resolved=undefined;}expect(resolved===undefined||resolved.length===0,row.name).toBe(true);
 }
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
