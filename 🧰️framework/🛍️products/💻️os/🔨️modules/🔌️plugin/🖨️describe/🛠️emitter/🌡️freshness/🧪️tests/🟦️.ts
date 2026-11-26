import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {mkdtempSync,mkdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {staleDescriptorEmitterCargoInputsV1,staleDescriptorEmitterInputsV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const ABC="ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",EMPTY="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

/** 🧪️ Known-answer SHA-256 vectors (FIPS 180-4) stand in for the recorded compiler receipt, so the oracle shares no hashing code with the implementation. */
async function withWorkspace<T>(body:(root:string,receipt:(rows:readonly [string,string][])=>string)=>Promise<T>):Promise<T>{
 const root=mkdtempSync(join(tmpdir(),"emitter-fresh-"));
 try{
  mkdirSync(join(root,"src"),{recursive:true});mkdirSync(join(root,".cache","registry"),{recursive:true});
  return await body(root,rows=>JSON.stringify({strings:rows.map(([path])=>path),inputs:rows.map(([,sha256],index)=>[index,"file",sha256])}));
 }finally{rmSync(root,{recursive:true,force:true});}
}

test("a receipt whose recorded hashes match the sources is fresh",async()=>{
 await withWorkspace(async(root,receipt)=>{
  writeFileSync(join(root,"src","a.rs"),"abc");writeFileSync(join(root,"src","b.rs"),"");
  expect(await staleDescriptorEmitterInputsV1(receipt([[join(root,"src","a.rs"),ABC],[join(root,"src","b.rs"),EMPTY]]),root)).toEqual({checked:2,stale:[]});
 });
});

test("an edited or deleted source is stale, in sorted relative posix form",async()=>{
 await withWorkspace(async(root,receipt)=>{
  writeFileSync(join(root,"src","a.rs"),"abd");writeFileSync(join(root,"src","c.rs"),"abc");
  const outcome=await staleDescriptorEmitterInputsV1(receipt([[join(root,"src","c.rs"),ABC],[join(root,"src","a.rs"),ABC],[join(root,"src","gone.rs"),ABC]]),root);
  expect(outcome).toEqual({checked:3,stale:["src/a.rs","src/gone.rs"]});
 });
});

test("dependency sources under .cache and files outside the workspace are not authored inputs",async()=>{
 await withWorkspace(async(root,receipt)=>{
  const outside=mkdtempSync(join(tmpdir(),"emitter-outside-"));
  try{expect(await staleDescriptorEmitterInputsV1(receipt([[join(root,".cache","registry","dep.rs"),ABC],[join(outside,"x.rs"),ABC]]),root)).toEqual({checked:0,stale:[]});}
  finally{rmSync(outside,{recursive:true,force:true});}
 });
});

test("a receipt without hashed inputs is refused",async()=>{
 await expect(staleDescriptorEmitterInputsV1("{}",tmpdir())).rejects.toThrow("without hashed inputs");
});

test("emitter reuse accepts unrelated lock edits and refuses changed consumed Cargo inputs",async()=>{
 await withWorkspace(async root=>{
  const lock=join(root,"Cargo.lock"),manifest=join(root,"emitter","Cargo.toml"),config=join(root,".cargo","config.toml");
  const cargoHome=join(root,".cache","registry"),homeConfig=join(cargoHome,"config.toml");
  const manifestText=`[package]\nname = "emitter"\nversion = "${fixture.versions.emitter}"\n`;
  const lockText=(dependency:string,unrelated:string)=>`version = 4\n\n[[package]]\nname = "emitter"\nversion = "${fixture.versions.emitter}"\ndependencies = [\n "dependency",\n]\n\n[[package]]\nname = "dependency"\nversion = "${dependency}"\n\n[[package]]\nname = "unrelated"\nversion = "${unrelated}"\n`;
  const sha=(bytes:string)=>createHash("sha256").update(bytes).digest("hex");
  const oldLock=lockText(fixture.versions.dependency,fixture.versions.unrelated),lockDigest=sha(oldLock);
  const retained=join(root,"retained");
  writeFileSync(retained,oldLock);
  mkdirSync(join(root,"emitter"));mkdirSync(join(root,".cargo"));
  const observation={manifest,args:["build"],units:[],invocationInputs:[
   {path:lock,sha256:lockDigest},{path:manifest,sha256:sha(manifestText)},{path:config,sha256:sha("original")},{path:homeConfig,sha256:sha("original")},
  ]};
  const files=new Map([["blobs/"+lockDigest,{path:retained}]]);
  const oracle=new Database(":memory:");
  try{
   const expected=oracle.query<{path:string},[string]>("select value as path from json_each(?) order by key");
   for(const row of fixture.cases){
    writeFileSync(lock,lockText(row.dependency,row.unrelated));
    writeFileSync(manifest,manifestText+(row.manifest==="edited"?"edition = \"2024\"\n":""));
    writeFileSync(config,row.config);
    writeFileSync(homeConfig,row.homeConfig);
    const pinned=join(root,"retired","Cargo.lock");
    const alternate={...observation,args:["build","--config","resolver.lockfile-path="+JSON.stringify(pinned)],invocationInputs:observation.invocationInputs.map(row=>row.path===lock?{...row,path:pinned}:row)};
    for(const selected of [observation,alternate]){
      const result=await staleDescriptorEmitterCargoInputsV1(selected,root,{origin:{cargoHome},files});
      expect(result.stale,row.description).toEqual(expected.all(JSON.stringify(row.stale)).map(row=>row.path));
    }
   }
  }finally{oracle.close();}
 });
});
