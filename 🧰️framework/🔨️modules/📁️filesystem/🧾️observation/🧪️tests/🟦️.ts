import assert from "node:assert/strict";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync,renameSync,symlinkSync,rmSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import {sha256} from "@noble/hashes/sha2.js";
import {observePhysicalFileV1,readPhysicalFileV1,FileObservationErrorV1,type FileObservationControlV1,type FileObservationProgressV1} from "../🟦️.ts";
import * as ts from "typescript";
import {transform} from "esbuild";
import * as fs from "node:fs/promises";
import {createHash} from "node:crypto";
import {resolve} from "node:path";
import type {CurrentPhysicalOwnerV1} from "../📁️current/🟦️.ts";

/** 🧫️ Joins portable file cases to Noble SHA-256, WebCrypto and JSON Schema outputs. */
export async function runFileObservationChecksV1():Promise<number>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")),ajv=new Ajv({strict:true});ajv.addSchema(schema);
 const validate=(kind:string,value:unknown)=>assert(ajv.getSchema(schema.$id+"#/definitions/"+kind)!(value));
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;assert(output);mkdirSync(output,{recursive:true});const directory=mkdtempSync(join(output,"physical-file-"));
 try{
  for(const row of fixture.cases){
   const path=join(directory,row.id+"-🌍️.bin"),bytes=new TextEncoder().encode(row.text.repeat(row.repeat)),progress:FileObservationProgressV1[]=[];let cancelled=row.cancelAt==="before",mutated=false;
   writeFileSync(path,bytes);
   if(row.mutation==="symlink"){
    renameSync(path,path+".source");
    try{symlinkSync(path+".source",path,"file");}catch(error){if(process.platform!=="win32"||(error as {code?:string}).code!=="EPERM")throw error;const target=path+".junction";mkdirSync(target);symlinkSync(target,path,"junction");}
   }
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
   assert(progress.every(step=>step.work<=row.maxWork&&step.bytes<=row.maxBytes));
  }
  const path=join(directory,"invalid-control");writeFileSync(path,"a");
  for(const limits of[{maxBytes:-1,maxWork:6,chunkBytes:1},{maxBytes:1,maxWork:0,chunkBytes:1},{maxBytes:1,maxWork:6,chunkBytes:0},{maxBytes:1,maxWork:6,chunkBytes:1048577}])await assert.rejects(()=>observePhysicalFileV1(path,{...limits,cancelled:()=>false,remainingMs:()=>60_000,onProgress:()=>{}}),(error:unknown)=>error instanceof FileObservationErrorV1&&error.code==="invalid-control");
  await assert.rejects(()=>observePhysicalFileV1(join(directory,"missing"),{maxBytes:1,maxWork:6,chunkBytes:1,cancelled:()=>false,remainingMs:()=>60_000,onProgress:()=>{}}),(error:unknown)=>error instanceof FileObservationErrorV1&&error.code==="io");
  return fixture.cases.length+9;
 }finally{rmSync(directory,{recursive:true,force:true});}
}

/** 🧵️ Executes the real current-file owner with measured system observers under two independent compilers. */
export async function runCurrentFileRecheckChecksV1():Promise<number>{
 const fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);assert(ajv.getSchema(schema.$id+"#/definitions/recheck")!(fixture.recheck));
 const file=join(import.meta.dir,"../📁️current/🟦️.ts"),source=readFileSync(file,"utf8"),parsed=ts.createSourceFile(file,source,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
 const body=parsed.statements.filter(statement=>!ts.isImportDeclaration(statement)).map(statement=>statement.getFullText(parsed)).join("\n").replace(/\bexport\s+/g,"");
 const implementations=[ts.transpileModule(body,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,(await transform(body,{loader:"ts",target:"es2022",format:"esm"})).code];
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;assert(output);mkdirSync(output,{recursive:true});
 for(const code of implementations){
  for(const scenario of ["success","mutation","cancel","deadline","budget"]){
   const root:string=mkdtempSync(join(output,"current-recheck-")),paths:string[]=[];let active=0,peak=0,rechecking=false,cancelled=false,entered=0,expired=false,starved=false;
   const observer:typeof observePhysicalFileV1=async(path,control)=>{
    active++;peak=Math.max(peak,active);if(rechecking){entered++;if(scenario==="cancel")cancelled=true;}
    try{await new Promise(done=>setTimeout(done,fixture.recheck.delayMs));return await observePhysicalFileV1(path,control);}finally{active--;}
   };
   const Owner=new Function("lstat","opendir","readlink","createHash","join","resolve","readPhysicalFileV1","observePhysicalFileV1",code+"\nreturn CurrentPhysicalOwnerV1;")(fs.lstat,fs.opendir,fs.readlink,createHash,join,resolve,readPhysicalFileV1,observer) as typeof CurrentPhysicalOwnerV1;
   const progress:FileObservationProgressV1[]=[],control:FileObservationControlV1={maxBytes:32*1024*1024,maxWork:65536,chunkBytes:1024,cancelled:()=>cancelled,remainingMs:()=>expired?0:60_000,onProgress:row=>{progress.push(row);if(starved)cancelled=true;}};
   try{
    const owner:CurrentPhysicalOwnerV1=new Owner(root,control);
    for(let index=0;index<fixture.recheck.files;index++){const path=join(root,index+".bin"),bytes=Buffer.from("Grüße "+index);paths.push(path);writeFileSync(path,bytes);assert.equal(await owner.digest(path),Buffer.from(sha256(bytes)).toString("hex"));}
    peak=0;rechecking=true;
    if(scenario==="mutation")writeFileSync(paths[0]!,"changed");
    if(scenario==="deadline")expired=true;
    if(scenario==="budget"){(control as {maxWork:number}).maxWork=progress.at(-1)!.work;starved=true;}
    if(scenario==="success"){await owner.recheck();assert(peak>1,"current-byte transfer checks must overlap independent files");assert(peak<=fixture.recheck.maximumConcurrency);assert.equal(entered,fixture.recheck.files);}
    else await assert.rejects(()=>owner.recheck());
    assert.equal(active,0,"every admitted observer must finish before returning");assert(peak<=fixture.recheck.maximumConcurrency);assert(progress.every(row=>row.bytes<=control.maxBytes&&row.work<=control.maxWork));
    console.log("[DEBUG] current-file recheck "+JSON.stringify({scenario,peak,entered,active}));
   }finally{rmSync(root,{recursive:true,force:true});}
  }
 }
 return implementations.length*5;
}
