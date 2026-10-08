import assert from "node:assert/strict";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,renameSync,symlinkSync,rmSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {sha256} from "@noble/hashes/sha2.js";
import {observePhysicalFileV1,readPhysicalFileV1,FileObservationErrorV1,type FileObservationControlV1,type FileObservationProgressV1} from "../🟦️.ts";

/** 🧫️ Joins portable file cases to Noble SHA-256, WebCrypto and JSON Schema outputs. */
export async function runFileObservationChecksV1():Promise<number>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")),ajv=new Ajv({strict:true});ajv.addSchema(schema);
 const validate=(kind:string,value:unknown)=>assert(ajv.getSchema(schema.$id+"#/definitions/"+kind)!(value));
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;assert(output);mkdirSync(output,{recursive:true});const directory=mkdtempSync(join(output,"physical-file-"));
 try{
  for(const row of fixture.cases){
   const path=join(directory,row.id+"-🌍️.bin"),bytes=new TextEncoder().encode(row.text.repeat(row.repeat)),progress:FileObservationProgressV1[]=[];let cancelled=row.cancelAt==="before",mutated=false;
   writeFileSync(path,bytes);
   if(row.mutation==="symlink"){renameSync(path,path+".source");symlinkSync(path+".source",path,"file");}
   if(row.mutation==="directory"){rmSync(path);mkdirSync(path);}
   const limits={maxBytes:row.maxBytes,maxWork:row.maxWork,chunkBytes:row.chunkBytes};validate("limits",limits);
   const control:FileObservationControlV1={...limits,cancelled:()=>cancelled,remainingMs:()=>row.expire?0:60_000,onProgress:step=>{
    validate("progress",step);progress.push({...step});if(row.cancelAt==="read"&&step.phase==="read")cancelled=true;
    if(step.phase==="read"&&!mutated&&(row.mutation==="replace"||row.mutation==="rewrite")){mutated=true;if(row.mutation==="replace"){renameSync(path,path+".original");writeFileSync(path,bytes);}else writeFileSync(path,"changed");}
   }};
   let actual:unknown;
   try{const claim=await observePhysicalFileV1(path,control);validate("claim",claim);actual={byteLength:claim.byteLength,sha256:claim.sha256,work:progress.at(-1)!.work};}
   catch(error){assert(error instanceof FileObservationErrorV1);validate("refusal",error.code);actual={refusal:error.code};}
   if(row.expected.refusal)assert.deepEqual(actual,row.expected,row.id);
   else{
    const independent=Buffer.from(sha256(bytes)).toString("hex"),web=Buffer.from(await crypto.subtle.digest("SHA-256",bytes)).toString("hex"),expected={byteLength:bytes.length,sha256:independent,work:Math.ceil(bytes.length/row.chunkBytes)+5};
    assert.equal(independent,web);assert.deepEqual(expected,row.expected);assert.deepEqual(actual,expected,row.id);assert.equal(progress.at(-1)!.phase,"complete");
    const retained=await readPhysicalFileV1(path,control);validate("retained",{claim:retained.claim,bytes:Array.from(retained.bytes)});assert.equal(retained.claim.sha256,independent);assert.deepEqual(retained.bytes,bytes);retained.bytes.fill(0);assert.deepEqual(readFileSync(path),Buffer.from(bytes));
   }
   assert(progress.every(step=>step.work<=row.maxWork&&step.bytes<=row.maxBytes));console.log("[DEBUG] physical-file "+JSON.stringify({id:row.id,actual}));
  }
  const path=join(directory,"invalid-control");writeFileSync(path,"a");
  for(const limits of[{maxBytes:-1,maxWork:6,chunkBytes:1},{maxBytes:1,maxWork:0,chunkBytes:1},{maxBytes:1,maxWork:6,chunkBytes:0},{maxBytes:1,maxWork:6,chunkBytes:1048577}])await assert.rejects(()=>observePhysicalFileV1(path,{...limits,cancelled:()=>false,remainingMs:()=>60_000,onProgress:()=>{}}),(error:unknown)=>error instanceof FileObservationErrorV1&&error.code==="invalid-control");
  console.log(`[DEBUG] physical-file laws=${fixture.cases.length+8}; Noble/WebCrypto=4; exact retained reads=4; custody mutations=2`);return fixture.cases.length+8;
 }finally{rmSync(directory,{recursive:true,force:true});}
}
