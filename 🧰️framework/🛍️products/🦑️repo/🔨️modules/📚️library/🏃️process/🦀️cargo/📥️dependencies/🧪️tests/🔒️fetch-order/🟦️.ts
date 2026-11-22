import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
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
   expect({attempts,consumptions,settles:Number(row.settles)},row.change).toEqual(oracle.get(row.change));
   expect({attempts,consumptions},row.change).toEqual({attempts:row.attempts,consumptions:row.consumptions});
  }
 }finally{db.close();}
});
