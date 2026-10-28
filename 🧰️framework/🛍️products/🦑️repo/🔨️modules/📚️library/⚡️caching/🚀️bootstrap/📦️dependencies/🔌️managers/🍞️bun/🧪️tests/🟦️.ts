/** 🩺️ Contract: the Bun lock check answers which manifests a lock disagrees with and which disagreements make Bun refuse a frozen install; the corpus is language-agnostic, Ajv validates its shape, `🐍️.py` models the check independently, and the pinned Bun itself is the third-party oracle for every `frozen` verdict (a `frozen` finding must be refused by Bun, every refusal must leave at least one finding). */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {parseJsoncV1} from "../../📖️formats/🟦️.ts";
import {selectPackageManagerV1} from "../../📚️catalog/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "../../🍞️bun/🧪️tests/🧬️schema/🔣️.json";

type Case=(typeof corpus.cases)[number];
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),manager=selectPackageManagerV1("javascript"),manifestsOf=(row:Case)=>new Map(row.manifests as [string,unknown][]);
const reachable=await fetch("https://registry.npmjs.org/ms",{method:"HEAD",signal:AbortSignal.timeout(4000)}).then(response=>response.ok,()=>false);
const bunRefuses=(row:Case):boolean=>{
 const root=mkdtempSync(join(tmpdir(),"lock-check-"));
 try{
  for(const [path,document] of row.manifests as [string,unknown][]){const file=join(root,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,JSON.stringify(document));}
  writeFileSync(join(root,"bun.lock"),row.lock);
  return Bun.spawnSync([process.execPath,"install","--frozen-lockfile","--dry-run","--ignore-scripts","--cache-dir",join(root,".cache")],{cwd:root,stdout:"ignore",stderr:"ignore"}).exitCode!==0;
 }finally{rmSync(root,{recursive:true,force:true});}
};
test("the corpus matches its schema",()=>{const validate=new Ajv({strict:false,allErrors:true}).compile(schema);expect(validate(corpus)).toBe(true);expect(validate.errors).toBeNull();});
for(const row of corpus.cases)test("lock check: "+row.name,()=>{expect(JSON.parse(JSON.stringify(manager.checkLock(row.lock,manifestsOf(row))))).toEqual(row.expected);});
test("the lock text reader accepts what Bun writes and refuses what it cannot read",()=>{expect(parseJsoncV1('{"a":[1,],}')).toEqual({a:[1]});expect(()=>parseJsoncV1("{")).toThrow();});
test("a manager without a lock check refuses to answer",()=>{expect(()=>selectPackageManagerV1("javascript",{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:"npm"}).checkLock("{}",new Map())).toThrow("bootstrap.dependencies.lock-check-required");});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
for(const row of corpus.cases.filter(row=>row.bun!==null))test.if(process.versions.bun===corpus.bun.version&&(!row.bun!.network||reachable))("pinned Bun agrees: "+row.name,()=>{
 const refused=bunRefuses(row);
 expect(refused).toBe(row.bun!.refused);
 if((row.expected as {frozen:boolean}[]).some(drift=>drift.frozen))expect(refused).toBe(true);
 if(refused)expect(row.expected.length).toBeGreaterThan(0);
});
