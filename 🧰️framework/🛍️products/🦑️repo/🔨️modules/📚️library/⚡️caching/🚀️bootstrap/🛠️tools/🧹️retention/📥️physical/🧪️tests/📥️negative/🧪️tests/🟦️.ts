import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFile}from"node:fs/promises";
import{loadProductFunctionsV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
import corpus from"../🔣️.json";
import schema from"../🧬️schema.json";
const repository=await privateRepositoryWorkspaceV1();
test("retention negative operations leave accepted parent frames through strict received purpose results",async()=>{
 const source=await readFile(new URL("../../🟦️.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("retention.ts",source,ts.ScriptTarget.Latest,true),direct:string[]=[],db=new Database(":memory:");
 try{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const visit=(node:import("typescript").Node)=>{if(ts.isAwaitExpression(node)&&/^await expect\((?:owner|storeOwner)\.withRepositoryCacheStore(?:Queued)?MaintenanceV1/u.test(node.getText(ast)))direct.push(node.getText(ast));ts.forEachChild(node,visit);};visit(ast);db.exec("create table cases(purpose text primary key,waited integer)");for(const row of corpus.cases)db.query("insert into cases values(?,?)").run(row.purpose,Number(row.waited));expect(db.query("select count(*) cases,sum(waited) waits from cases").get()).toEqual({cases:8,waits:2});expect(direct).toEqual([]);
 }finally{db.close();}
});

import{readdirSync}from"node:fs";
import{observeRetentionNegativePurposeV1}from"../🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
test("retention negative receiver retains actual strict wire and rejected request status without physical effects",async()=>withBuildTestInvocationV1(async original=>{
 const workspace=await repository.scratch("tr-");expect(readdirSync(workspace)).toEqual([]);for(const purpose of["queued-invalid-0","queued-invalid-1","queued-invalid-2"])await observeRetentionNegativePurposeV1(original,workspace,purpose);expect(readdirSync(workspace)).toEqual([]);expect(original.control.signal.aborted).toBe(false);
}));

test("retention negative receiver closes partial readers while preserving undefined and frozen setup failures",async()=>withBuildTestInvocationV1(async original=>{
 const routing=await import("../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),db=new Database(":memory:");
 try{for(const kind of["undefined","frozen"]){let opened=0,closed=0,caught=false,actual:unknown;const primary=kind==="undefined"?undefined:Object.freeze(Error("Frozen setup primary")),ports={requirePurpose:()=>"queued-cancel",requireWorkspace:()=>{},checkScriptInvocation:routing.checkScriptInvocation,acknowledgement:()=>({}),ackPrefix:"",failurePrefix:"",console:{log(){}},AbortController,createIssuedResourceControlV1:routing.createIssuedResourceControlV1,repositoryCacheBudgetV1:()=>({partitions:{tools:{directory:"model"}}}),openRepositoryCachePartitionV1:async()=>{opened++;return{close:async()=>{closed++;throw Error("Secondary close failure");}};},leases:Object.defineProperty({},"acquireQueuedResourceLease",{get(){throw primary;}}),physical:{queuedMaintenance:{}},owner:{},delay:()=>{throw Error("No wait entered");},repositoryLeaseDirectoryV1:()=>"model"},run=(await loadProductFunctionsV1(new URL("../🟦️.ts",import.meta.url),["runRetentionNegativePurposeV1"],ports)).runRetentionNegativePurposeV1;try{await run(original,"explicit-storage-port-model","queued-cancel");}catch(error){caught=true;actual=error;}expect(caught).toBe(true);expect(actual).toBe(primary);expect(db.query("select ?1=1 and ?2=1 closed").get(opened,closed)).toEqual({closed:1});expect(original.control.signal.aborted).toBe(false);}
 }finally{db.close();}
}));
