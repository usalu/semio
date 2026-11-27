import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {randomBytes} from "node:crypto";
import {acquireQueuedResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {fileURLToPath} from "node:url";
import{loadProductFunctionsSyncV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
import ts from "typescript";
import {concurrentEditV1,retryOnConcurrentEditV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import cases from "./🔣️.json" with {type:"json"};
import caseSchema from "./🧬️.json" with {type:"json"};

const source=(relative:string):ts.SourceFile=>{const path=fileURLToPath(new URL(relative,import.meta.url));return ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);};
const fn=(file:ts.SourceFile,name:string):ts.FunctionDeclaration=>{const node=file.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text===name);if(!node)throw Error("missing "+name);return node;};
const calls=(node:ts.Node,name:string):ts.CallExpression[]=>{const out:ts.CallExpression[]=[];const visit=(child:ts.Node):void=>{if(ts.isCallExpression(child)){const callee=child.expression;if(ts.isIdentifier(callee)&&callee.text===name||ts.isPropertyAccessExpression(callee)&&callee.name.text===name)out.push(child);}ts.forEachChild(child,visit);};visit(node);return out;};

/** 🧪️ The exclusive registry-fetch lease ranks above the native storage mapping, so no journal-backed recheck may run while it is held. */
test("the exclusive registry fetch lease is held without any native storage recheck, which would request the lower-ranked image mapping",()=>{
 const ready=source("../../🟢️ready/💾️storage/🟦️.ts"),inputs=source("../../🟦️.ts");
 const once=fn(ready,"withRepositoryCargoRegistryReadyOnceV1"),custody=calls(once,"custody");
 expect(custody.length).toBe(1);
 const held={custodyWriter:calls(custody[0]!.arguments[0]!,"recheck").length,publish:calls(fn(ready,"publishReady"),"recheck").length,capture:calls(fn(ready,"captureReady"),"recheck").length};
 const owned=calls(fn(inputs,"withRepositoryCargoInputsOwnedV1"),"withRepositoryCargoRegistryReadyV1");
 expect(owned.length).toBe(1);
 const fetch=owned[0]!.arguments[2]!;
 expect({...held,fetch:calls(fetch,"recheck").length}).toEqual({custodyWriter:0,publish:0,capture:0,fetch:0});
});

/** 🪪️ Revalidates changed source before the single input handoff and preserves churn observed by or after the consumer. */
test("Cargo registry input is consumed once even when readiness moves afterward",async()=>{
 const db=new Database(":memory:");
 expect(new Ajv().validate(caseSchema,cases)).toBe(true);
 const oracle=db.query<{attempts:number;consumptions:number;settles:number},[string]>("SELECT 1 + (?1 = 'before') attempts, 1 consumptions, (?1 IN ('before', 'none')) settles");
 try{
  for(const row of cases){
   let attempts=0,consumptions=0;
   const churn=concurrentEditV1("generation","cargo-ready-fixture",row.change,"snapshot");
   const once=async(_options:unknown,_custody:unknown,_fetch:unknown,body:()=>Promise<string>)=>{
    attempts++;
    if(row.change==="before"&&attempts===1)throw churn;
    const result=await body();
    if(row.change==="after"&&attempts===1)throw churn;
    return result;
   };
   const retry:typeof retryOnConcurrentEditV1=(profile,body,options)=>retryOnConcurrentEditV1(profile,body,{...options,sleep:async()=>{},onRetry:()=>{}});
   const own=loadProductFunctionsSyncV1(new URL("../../🟢️ready/💾️storage/🟦️.ts",import.meta.url),["withRepositoryCargoRegistryReadyV1"],{retryOnConcurrentEditV1:retry,withRepositoryCargoRegistryReadyOnceV1:once}).withRepositoryCargoRegistryReadyV1;
   const effect=own({control:{signal:new AbortController().signal}},()=>{},()=>{},async()=>{
    consumptions++;
    if(row.change==="during")throw churn;
    return "published";
   });
   if(row.settles)expect(await effect,row.change).toBe("published");else await expect(effect,row.change).rejects.toBe(churn);
   expect<unknown>({attempts,consumptions,settles:Number(row.settles)},row.change).toEqual(oracle.get(row.change));
   expect({attempts,consumptions},row.change).toEqual({attempts:row.attempts,consumptions:row.consumptions});
  }
 }finally{db.close();}
});

const variable=(file:ts.SourceFile,name:string):ts.VariableDeclaration=>{let found:ts.VariableDeclaration|undefined;const visit=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.name.text===name)found=node;ts.forEachChild(node,visit);};visit(file);if(!found)throw Error("missing "+name);return found;};

/** 🔒️ A lock resolution (`cargo update`, `generate-lockfile`) runs its body under the registry and the shared compiler writer custody (image), so that body requests no Cargo build-directory custody (a lower-ranked lease); only the resolve command, which holds no writer around its body, takes it. */
test("the lock resolution body of the Cargo launcher takes no build lease under the compiler writer, and only the resolve body does",()=>{
 const launcher=source("../../../🚪️launcher/🟦️.ts"),lock=calls(launcher,"withRepositoryCargoResolutionV1");
 expect(lock.length).toBe(1);
 expect(lock[0]!.arguments[1]!.getText()).toBe("resolution");
 expect(calls(variable(launcher,"resolution"),"acquireCargoBuildLeaseV1").length).toBe(0);
 expect(calls(variable(launcher,"resolved"),"acquireCargoBuildLeaseV1").length).toBe(1);
 expect(calls(variable(launcher,"resolved"),"resolution").length).toBe(1);
 expect(calls(fn(source("../../🟦️.ts"),"withRepositoryCargoResolutionOwnedV1"),"acquireCargoBuildLeaseV1").length).toBe(0);
});

/** 🪶️ A lock resolution takes the registry as a shared cohort reader only, offline or online, and enters the compiler writer after it and leaves it before. */
test("the lock resolution holds the registry shared, never exclusive, around the compiler writer",async()=>{
 for(const args of [["fetch","--locked","--offline"],["fetch","--locked"]]){
  const trace:string[]=[],taken:Record<string,unknown>[]=[];
  const own=loadProductFunctionsSyncV1(new URL("../../🟦️.ts",import.meta.url),["withRepositoryCargoResolutionOwnedV1"],{
   admitRepositoryCargoInputsV1:async()=>({requests:[{args}],leaseOptions:{directory:"leases",resource:"cargo:registry:/home"}}),
   acquireQueuedResourceLease:async(options:Record<string,unknown>)=>{taken.push(options);trace.push(String(options.resource)+" "+String(options.mode));return{release:()=>{trace.push(String(options.resource)+" release");}};},
   withCompilerWorkspaceSharedWriteV1:async(_storage:unknown,_directory:string,_control:unknown,run:()=>Promise<unknown>)=>{trace.push("writer");try{return await run();}finally{trace.push("writer release");}},
   randomBytes:(size:number)=>Buffer.alloc(size)
  }).withRepositoryCargoResolutionOwnedV1 as (...values:unknown[])=>Promise<unknown>;
  expect(await own({storage:{recheck:async()=>{}},control:{}},{selections:[]},async()=>{trace.push("body");return"done";}),args.join(" ")).toBe("done");
  expect(trace,args.join(" ")).toEqual(["cargo:registry:/home shared","writer","body","writer release","cargo:registry:/home release"]);
  expect(taken[0]!.sharedCohort,args.join(" ")).toBe(true);
 }
});

/** ⏳️ Regression: a build or test that holds the registry as a reader for its whole run (a 42 minute `cargo test` did) never blocks a concurrent resolution, offline or online,. */
test("a long-running registry reader does not block a concurrent online or offline lock resolution",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"reg-")),resource="cargo:registry:/home",signal=new AbortController().signal;
 const reader=await acquireQueuedResourceLease({directory,resource,mode:"shared",sharedCohort:true,owner:"long-build",signal});
 try{
  for(const args of [["fetch","--locked","--offline"],["fetch","--locked"]]){
   const own=loadProductFunctionsSyncV1(new URL("../../🟦️.ts",import.meta.url),["withRepositoryCargoResolutionOwnedV1"],{
    admitRepositoryCargoInputsV1:async()=>({requests:[{args}],leaseOptions:{directory,resource,signal,control:undefined}}),
    acquireQueuedResourceLease,
    withCompilerWorkspaceSharedWriteV1:async(_storage:unknown,_directory:string,_control:unknown,run:()=>Promise<unknown>)=>run(),
    randomBytes
   }).withRepositoryCargoResolutionOwnedV1 as (...values:unknown[])=>Promise<unknown>;
   const started=Date.now();
   expect(await Promise.race([own({storage:{recheck:async()=>{}},control:{}},{selections:[]},async()=>"resolved"),new Promise(done=>setTimeout(()=>done("blocked"),5000))]),args.join(" ")).toBe("resolved");
   expect(Date.now()-started).toBeLessThan(5000);
  }
 }finally{reader.release();rmSync(directory,{recursive:true,force:true});}
});
