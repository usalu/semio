/** 🌍️ Contract: the environment digest changes exactly when an edge of an unrelated project may change, and its rows prove it unchanged otherwise. Extractors agree with real parsers (JSON, Bun.TOML) and the digests with an independent Python model. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {lstat,readFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {deriveEnvironmentV1,environmentRowHoldsV1,extractEnvironmentFieldV1,type EnvironmentExtractorV1} from "../🟨️.mjs";

const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-environment-"));
const readers=(dir:string)=>({
 read:async(path:string)=>{try{return await readFile(join(dir,path),"utf8");}catch{return undefined;}},
 stat:async(path:string)=>{try{const status=await lstat(join(dir,path));return status.isFile()?{size:status.size,mtimeMs:status.mtimeMs}:undefined;}catch{return undefined;}},
});
const write=(dir:string,tree:Record<string,string>)=>{for(const[path,text]of Object.entries(tree)){mkdirSync(dirname(join(dir,path)),{recursive:true});writeFileSync(join(dir,path),text);}};
const derive=(dir:string)=>deriveEnvironmentV1({...readers(dir),projects:fixture.projects,workspaceManifests:fixture.workspaceManifests,lock:fixture.lock,taxonomy:fixture.taxonomy});

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

for(const row of fixture.cases)test("extractor "+row.extractor+": "+row.name,()=>{
 expect(extractEnvironmentFieldV1(row.extractor as EnvironmentExtractorV1,row.text??undefined)).toEqual(row.expected);
 if(row.extractor==="cargo-package"&&row.text!==null)expect(extractEnvironmentFieldV1("cargo-package",row.text)).toEqual({name:(Bun.TOML.parse(row.text) as {package?:{name?:string}}).package?.name??null});
});

test("an unknown extractor and an unknown row kind are refused",async()=>{
 expect(()=>extractEnvironmentFieldV1("other" as never,"{}")).toThrow();
 await expect(environmentRowHoldsV1({kind:"other",path:"x",fact:null},{read:async()=>undefined,stat:async()=>undefined})).rejects.toThrow();
});

test("the digest equals the independent model for every variant and the rows prove exactly the unchanged ones",async()=>{
 const base=fixture.digests[0]!,baseDir=scratch();
 try{
  write(baseDir,base.tree as Record<string,string>);const original=await derive(baseDir);
  expect(original.digest).toBe(base.digest);
  for(const variant of fixture.digests){
   const dir=scratch();
   try{
    write(dir,variant.tree as Record<string,string>);const derived=await derive(dir);expect(derived.digest,variant.name).toBe(variant.digest);
    const holds=await Promise.all(original.rows.map(row=>environmentRowHoldsV1(row,readers(dir))));
    if(variant.digest===base.digest)expect(holds.every(Boolean),variant.name).toBe(true);else expect(holds.some(value=>!value),variant.name).toBe(true);
   }finally{rmSync(dir,{recursive:true,force:true});}
  }
  expect(original.rows.every(row=>row.kind==="field"||row.kind==="digest")).toBe(true);
  expect(original.rows.filter(row=>row.kind==="digest").map(row=>row.path).sort()).toEqual(["Cargo.toml","bun.lock","tax.json"]);
 }finally{rmSync(baseDir,{recursive:true,force:true});}
});

test("a digest row takes the stamp shortcut only when size and time are unchanged",async()=>{
 const dir=scratch();
 try{
  write(dir,{"f":"abc"});const rows=(await deriveEnvironmentV1({...readers(dir),projects:[],workspaceManifests:[],lock:"f"})).rows,row=rows[0]!;
  expect(await environmentRowHoldsV1(row,readers(dir))).toBe(true);
  writeFileSync(join(dir,"f"),"abd");expect(await environmentRowHoldsV1(row,readers(dir))).toBe(false);
  writeFileSync(join(dir,"f"),"abc");expect(await environmentRowHoldsV1(row,readers(dir))).toBe(true);
  rmSync(join(dir,"f"));expect(await environmentRowHoldsV1(row,readers(dir))).toBe(false);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
