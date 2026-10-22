/** 📦 Contract: the files the pinned tooling installer receives equal the authored cases in `./🔣️.json` and an independent Python derivation (manifest without first-party links, lock with content-addressed patches, patch files named by the sha256 of their bytes, one file per distinct content, authored order kept), the patch authority of the lock is compared exactly, and a lock that is not an object, changed patch authority, missing or invalid patch bytes are refused with the authored messages. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {nxToolingInstallFilesV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(cases:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({cases}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const lockOf=(row:any):unknown=>"lockRaw" in row?JSON.parse(row.lockRaw):row.lock;
const sourcesOf=(row:any)=>new Map<string,Uint8Array>(Object.entries(row.sources as Record<string,string>).map(([path,text])=>[path,Buffer.from(text)]));
const attempt=(action:()=>Map<string,Buffer>):{files?:[string,string][];error?:string}=>{try{return{files:[...action()].map(([name,bytes])=>[name,bytes.toString("utf8")] as [string,string])};}catch(error){return{error:(error as Error).message};}};
const patchClass=new Set(["Original tooling patch file required","Original tooling patch hunk required","Relative tooling patch file required","Tooling patch file repeated: x","Original tooling patch sections required"]);

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],sources:{a:1}}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{name:"x",sources:{}}]})).toBe(false);
});

test("the install image files equal the authored cases, patches are content addressed and refusals carry the authored message",()=>{
 const independent=corpus.cases.filter(row=>!(row as any).error||!patchClass.has((row as any).error));
 const answers=ask(independent.map(row=>({recipe:row.recipe,lock:lockOf(row),sources:row.sources})));
 for(const row of corpus.cases){
  const actual=attempt(()=>nxToolingInstallFilesV1(structuredClone(row.recipe) as never,lockOf(row),sourcesOf(row)));
  if((row as any).error){expect(actual.error,row.name).toBe((row as any).error);}
  else{
   expect(actual.error,row.name).toBeUndefined();
   const names=actual.files!.map(([name])=>name);
   expect(names.slice(0,2),row.name).toEqual(["package.json","bun.lock"]);
   for(const[name,text]of actual.files!.slice(2))expect(name,row.name).toBe("patches/"+createHash("sha256").update(text).digest("hex")+".patch");
   expect(new Set(names).size,row.name).toBe(names.length);
  }
  const index=independent.indexOf(row);
  if(index>=0){
   const answer=answers[index];
   if((row as any).error)expect(answer.error,row.name).toBe((row as any).error);
   else expect(actual.files,row.name).toEqual(answer.files);
  }
 }
});

test("the installer image does not mutate its inputs",()=>{
 const row=corpus.cases[1]!,recipe=structuredClone(row.recipe),lock=structuredClone(lockOf(row));
 nxToolingInstallFilesV1(recipe as never,lock,sourcesOf(row));
 expect(recipe).toEqual(row.recipe);expect(lock).toEqual(lockOf(row));
});
