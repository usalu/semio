import {test,expect} from "bun:test";
import {mkdtempSync,mkdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {staleDescriptorEmitterInputsV1} from "../🟦️.ts";

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
